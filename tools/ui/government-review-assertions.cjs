// S10.c browser-only assertions. These read the served UI; they never change
// native state, inject styles, hide overlays or horizontally scroll content.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const quoted=value=>JSON.stringify(String(value));

// Serialized into the page by Playwright; keep this function self-contained.
function visibleTextReading(element,{scope,obstructions}){
  const errors=[],box=r=>({left:r.left,top:r.top,right:r.right,bottom:r.bottom,width:r.width,height:r.height});
  const rect=element.getBoundingClientRect(),viewport={left:0,top:0,right:document.documentElement.clientWidth,bottom:innerHeight};
  const inside=(a,b)=>a.left>=b.left-1&&a.right<=b.right+1&&a.top>=b.top-1&&a.bottom<=b.bottom+1;
  const intersects=(a,b)=>Math.min(a.right,b.right)-Math.max(a.left,b.left)>1&&Math.min(a.bottom,b.bottom)-Math.max(a.top,b.top)>1;
  if(rect.width<=0||rect.height<=0)errors.push('Element has no visible area');
  if(!inside(rect,viewport))errors.push('Element is outside the viewport');
  const ancestors=[];
  for(let node=element;node;node=node.parentElement){
    const style=getComputedStyle(node),bounds=node.getBoundingClientRect();
    ancestors.push({tag:node.tagName,id:node.id,scrollLeft:node.scrollLeft,scrollWidth:node.scrollWidth,clientWidth:node.clientWidth});
    if(Math.abs(node.scrollLeft)>1)errors.push('Horizontal scroll used on '+(node.id||node.tagName));
    if(node.clientWidth>0&&node.scrollWidth>node.clientWidth+1)errors.push('Horizontal overflow on '+(node.id||node.tagName));
    if(node!==element&&/(auto|scroll|hidden|clip)/.test(style.overflowY)&&!inside(rect,{left:-Infinity,right:Infinity,top:bounds.top+node.clientTop,bottom:bounds.top+node.clientTop+node.clientHeight}))errors.push('Vertically clipped by '+(node.id||node.tagName));
    if(node.matches(scope))break;
  }
  const range=document.createRange();range.selectNodeContents(element);
  const textRects=Array.from(range.getClientRects()).filter(r=>r.width>0&&r.height>0);
  if(!textRects.length)errors.push('No rendered text');
  for(const r of textRects){
    if(!inside(r,viewport)||!inside(r,rect))errors.push('Text is clipped or extends outside its element');
    for(const fraction of [0.2,0.5,0.8]){
      const x=r.left+r.width*fraction,y=r.top+r.height/2,hit=document.elementFromPoint(x,y);
      if(!hit||!(hit===element||element.contains(hit)))errors.push('Text is obscured by '+(hit?.id||hit?.className||hit?.tagName||'outside viewport'));
    }
    for(const obstruction of document.querySelectorAll(obstructions)){
      if(obstruction===element||obstruction.contains(element))continue;
      const style=getComputedStyle(obstruction);
      if(style.display!=='none'&&style.visibility!=='hidden'&&intersects(r,obstruction.getBoundingClientRect()))errors.push('Text overlaps header/navigation '+(obstruction.id||obstruction.className));
    }
  }
  return {text:element.textContent,rect:box(rect),text_rects:textRects.map(box),ancestors,errors,viewport};
}
async function visibleText(locator,settings,label){
  assert.equal(await locator.count(),1,label+' must have exactly one node');
  assert(await locator.isVisible(),label+' is hidden');
  const reading=await locator.evaluate(visibleTextReading,settings);
  assert.deepEqual(reading.errors,[],label+': '+JSON.stringify(reading));return reading;
}
const govSettings={scope:'#govScreen',obstructions:'#govScreen > .tbar, #govScreen .gov-ui-tabs'};
const agencySettings={scope:'#agencyPanel',obstructions:'#agencyPanel > header, #agencyStatus'};
async function exactChanges(page,kind,quote){
  assert(['government','agency'].includes(kind));assert(Array.isArray(quote.changes)&&quote.changes.length,'A real native quote must contain effects');
  const root=kind==='government'?'#govReview':'#agencyReview',attr=kind==='government'?'data-gov-change':'data-agency-change';
  const cards=page.locator(root+' ['+attr+']');assert.equal(await cards.count(),quote.changes.length,'All native effects must be rendered');
  for(let index=0;index<quote.changes.length;index++){
    const row=quote.changes[index],card=cards.nth(index);assert.equal(await card.getAttribute(attr),String(index));
    assert.equal(await card.locator('dt').textContent(),String(row.label||row.key),'Exact native effect label '+index);
    for(const side of ['before','after']){
      // These native preview APIs return already-formatted strings. Fail on a
      // contract change instead of rebuilding or rounding the quoted values.
      assert.equal(typeof row[side],'string','Native '+side+' must be a display string');
      assert.equal(await card.locator('[data-change-'+side+']').textContent(),row[side],'Exact native '+side+' '+index);
      assert.equal(await card.locator('[data-change-'+side+']').locator('..').locator('span').textContent(),side==='before'?'Before':'After');
    }
  }
  return cards;
}
async function governmentNavigation(page){
  const nav=page.locator('#govScreen .gov-ui-tabs');
  await nav.evaluate(e=>e.scrollIntoView({block:'center',inline:'nearest',behavior:'instant'}));
  const buttons=nav.locator('[role="tab"]'),labels=[];assert(await buttons.count()>=4);
  for(const button of await buttons.all()){
    const reading=await visibleText(button,govSettings,'Government navigation label');
    assert(reading.text.trim());labels.push({id:await button.getAttribute('id'),...reading});
  }
  return {viewport:page.viewportSize(),labels};
}
async function mobileChanges(page,kind,quote,onWidth){
  const readings=[],original=page.viewportSize();
  for(const width of [390,320]){
    await page.setViewportSize({width,height:844});
    const navigation=kind==='government'?await governmentNavigation(page):undefined;
    const cards=await exactChanges(page,kind,quote),rows=[],settings=kind==='government'?govSettings:agencySettings;
    for(let index=0;index<quote.changes.length;index++){
      const card=cards.nth(index);
      await card.evaluate(e=>e.scrollIntoView({block:'center',inline:'nearest',behavior:'instant'}));
      const before=await visibleText(card.locator('[data-change-before]'),settings,kind+' Before '+index+' at '+width);
      const after=await visibleText(card.locator('[data-change-after]'),settings,kind+' After '+index+' at '+width);
      rows.push({index,label:quote.changes[index].label||quote.changes[index].key,before,after});
    }
    readings.push({viewport:page.viewportSize(),navigation,rows});
    // Capture a checked card instead of scrolling the entire long review back
    // underneath its sticky header just to take a screenshot.
    await cards.first().evaluate(e=>e.scrollIntoView({block:'center',inline:'nearest',behavior:'instant'}));
    if(onWidth)await onWidth(width);
  }
  await page.setViewportSize(original);return readings;
}
async function governmentNotice(page){
  // Do not repair focus or scrolling from the test: inspect the final state
  // left by the ordinary confirmation and its final UI render.
  await page.waitForFunction(()=>!gov.busy&&document.activeElement===document.getElementById('govNotice'));
  const reading=await visibleText(page.locator('#govNotice'),govSettings,'Settled government result notice');
  assert(reading.text.trim());return {viewport:page.viewportSize(),focused:true,...reading};
}
async function tongaPortrait(page,{url,root,expected,git,board}){
  const person=board.party_leadership?.executive_person;assert.equal(person?.id,'taufaahau_tupou_iv');
  const manifestFile='spheres-web/data/person_portraits.json',committed=git(['show',expected+':'+manifestFile]);
  const manifest=JSON.parse(committed.toString('utf8'));assert.deepEqual(JSON.parse(fs.readFileSync(path.join(root,manifestFile),'utf8')),manifest);
  const matches=manifest.people[person.id].portraits.filter(p=>p.from<='1990-01-01'&&p.to>'1990-01-01');assert.equal(matches.length,1);
  const art=matches[0];assert.equal(art.identity_source.person_id,person.id);assert.equal(art.style,'cartoon');assert.equal(art.status,'illustrated-likeness');
  assert.equal(art.asset,'spheres-web/ui/person-portraits/taufaahau-tupou-iv-cartoon-1990-v1.png');
  const expectedUrl='/art/people/'+path.basename(art.asset);assert.equal(person.portrait?.url,expectedUrl);
  assert.equal(person.portrait.from,art.from);assert.equal(person.portrait.to,art.to);
  const img=page.locator('#gov-panel-overview img[src='+quoted(expectedUrl)+']');assert.equal(await img.count(),1);
  assert.equal(await page.locator('#gov-panel-overview .gov-ui-art-pending').count(),0,'Opening executive must not retain the TT placeholder');
  await img.scrollIntoViewIfNeeded();await img.evaluate(async image=>{await image.decode();});
  const image=await img.evaluate(i=>({url:new URL(i.currentSrc).pathname,complete:i.complete,width:i.naturalWidth,height:i.naturalHeight,alt:i.alt}));
  assert.equal(image.url,expectedUrl);assert(image.complete);assert.equal(image.width,art.width);assert.equal(image.height,art.height);assert(image.alt.includes(person.name));
  const local=fs.readFileSync(path.join(root,art.asset)),blob=git(['show',expected+':'+art.asset]);assert(local.equals(blob));assert.equal(hash(local),art.sha256);
  const response=await page.request.get(url+expectedUrl);let served;
  try{assert(response.ok());assert.match(response.headers()['content-type']||'',/^image\/png/);served=await response.body();assert(served.equals(local));}finally{await response.dispose();}
  return {person_id:person.id,manifest_sha256:hash(committed),asset:art.asset,art_window:{from:art.from,to:art.to},served_sha256:hash(served),committed_sha256:hash(blob),checkout_sha256:hash(local),served_bytes:served.length,image};
}
module.exports={exactChanges,mobileChanges,governmentNavigation,governmentNotice,tongaPortrait};
