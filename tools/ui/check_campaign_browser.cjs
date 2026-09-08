// Real browser coverage of the actual module with an explicitly synthetic DTO.
// This tests interaction/layout, not simulation outcomes or the web API.
// Run: node tools/ui/check_campaign_browser.cjs
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const {chromium}=require('playwright');
const root=path.resolve(__dirname,'../..'),out=path.join(root,'artifacts/browser-ci/campaign-module');
const operation={enabled:true,conflict:7,order:{conflict:7,nation:'Iraq',target:'IQ-BA',approach:'advance',reserve_bp:1500,air:'none',naval:'none'},
  targets:[{id:'IQ-BA',name:'Basra',held:false},{id:'IQ-NI',name:'Nineveh',held:true}],fielded:12.8,reserve:3.2,garrison:.8,in_transit:1.25,readiness:.63,preparation:.42,
  supply:{coverage:.51,status:'Strained',reason:'The supply crossing is contested. Local reserves are sustaining the operation.',eta_days:3},
  enemy:{low:4,high:8,confidence:.6,observed_day:10,observed_label:'10 Jan 1990'},
  forecast:{advance_low:.003,advance_high:.008,loss_low:.02,loss_high:.04,reason:'Readiness is limiting the advance. Hold to recover before committing reserves.',period:'per day'},
  last_report:{day:11,day_label:'11 Jan 1990',advanced:.006,retreated:0,losses:.025,readiness:.63,summary:'The advance reached the crossing. The defender committed reserves and held the far bank.'}};
const peace={aim:{kind:'recover',districts:['IQ-BA']},aim_options:[{kind:'expel',label:'Expel an invader',description:'Restore the legal border.'},
  {kind:'recover',label:'Recover territory',description:'Recover the named district.'},{kind:'concession',label:'Compel a concession',description:'Seek limited terms.'},
  {kind:'government_change',label:'Government change',description:'Seek a political opening.'}],targets:[{id:'IQ-BA',name:'Basra'}],can_propose:true,
  prices:{aim:3,proposal:0,response:0},limits:{reparations_min_bp:1,reparations_max_bp:200,garrison_max_bp:5000},proposals:[{id:2,from_name:'Kuwait',incoming:true,summary:'Ceasefire: restore legal borders.',days_left:12,can_respond:true}],
  garrison:{share_bp:1000,policy:'restraint'},occupation:[{district:'IQ-BA',name:'Basra',resistance:.31,coverage:.48,days:8}]};
(async()=>{
  fs.mkdirSync(out,{recursive:true});const browser=await chromium.launch({headless:true});
  try{
    const page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'}),errors=[];
    page.on('pageerror',error=>errors.push(error.message));
    await page.setContent('<!doctype html><html><head><meta name="viewport" content="width=device-width, initial-scale=1"><title>Campaign module verification</title></head><body style="margin:0;background:#111d1b"><main id="board" style="max-width:960px;margin:0 auto;padding:12px"></main></body></html>');
    await page.addStyleTag({path:path.join(root,'spheres-web/ui/campaign-operations-ui.css')});
    await page.addScriptTag({path:path.join(root,'spheres-web/ui/campaign-operations-ui.js')});
    await page.evaluate(({operation,peace})=>{
      window.__orders=[];window.__failure=null;
      document.querySelector('#board').innerHTML=CampaignOperationsUI.html({operation,peace,theatre_name:'Persian Gulf'});
      CampaignOperationsUI.bind(document.querySelector('#board'),async order=>{if(window.__failure)throw Error(window.__failure);window.__orders.push(order);return {errors:[]};});
    },{operation,peace});
    const order=page.locator('[data-campaign-command="operation"]');
    await order.getByText('Breakthrough',{exact:true}).click();
    await order.getByLabel('Objective district',{exact:true}).selectOption('IQ-NI');
    await order.getByLabel('Keep in reserve (%)',{exact:true}).fill('12.34');
    await order.getByLabel('Air mission',{exact:true}).selectOption('reconnaissance');
    await order.getByLabel('Naval mission',{exact:true}).selectOption('escort');
    await order.getByRole('button',{name:'Apply operation · free',exact:true}).click();
    await order.getByRole('status').filter({hasText:'Operation order applied.'}).waitFor();
    assert.deepEqual(await page.evaluate(()=>window.__orders[0]),{kind:'operation',conflict:7,target:'IQ-NI',approach:'breakthrough',reserve_bp:1234,air:'reconnaissance',naval:'escort'});
    await page.evaluate(()=>window.__failure='Basing access was withdrawn.');await order.getByRole('button').click();
    await order.getByRole('status').filter({hasText:'Basing access was withdrawn.'}).waitFor();
    assert.equal(await page.evaluate(()=>window.__orders.length),1);assert.equal(await order.getByRole('button').isEnabled(),true);
    await page.evaluate(()=>window.__failure=null);
    const aim=page.locator('[data-campaign-command="set_aim"]');
    await aim.getByLabel('War aim',{exact:true}).selectOption('expel');assert(await aim.getByLabel('Territory to recover',{exact:true}).isHidden());
    await aim.getByRole('button').click();
    const response=page.locator('[data-campaign-command="respond"]');await response.getByRole('button',{name:'Reject offer',exact:true}).click();
    await page.getByText('Prepare a peace proposal',{exact:true}).click();
    const proposal=page.locator('[data-campaign-command="propose"]');
    await proposal.getByLabel('Proposed peace terms',{exact:true}).selectOption('cede');
    await proposal.getByLabel('Demanded district',{exact:true}).selectOption('IQ-BA');await proposal.getByRole('button').click();
    await page.getByText('Occupation and garrison policy',{exact:true}).click();
    const garrison=page.locator('[data-campaign-command="garrison"]');await garrison.getByLabel('Theatre force for garrisons (%)',{exact:true}).fill('25');
    await garrison.getByLabel('Occupation policy',{exact:true}).selectOption('reconstruction');await garrison.getByRole('button').click();
    const submitted=await page.evaluate(()=>window.__orders);
    assert.deepEqual(submitted.slice(1),[
      {kind:'war_diplomacy',order:{kind:'set_aim',conflict:7,aim:{kind:'expel'}}},
      {kind:'war_diplomacy',order:{kind:'respond',offer:2,accept:false}},
      {kind:'war_diplomacy',order:{kind:'propose',conflict:7,terms:{kind:'cede',districts:['IQ-BA']}}},
      {kind:'war_diplomacy',order:{kind:'garrison',conflict:7,share_bp:2500,policy:'reconstruction'}}
    ]);
    for(const [name,width,height] of [['desktop',1440,1000],['tablet',768,1024],['mobile',390,844]]){
      await page.setViewportSize({width,height});
      assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),`${name} page overflows horizontally`);
      assert(await page.evaluate(()=>Array.from(document.querySelectorAll('.campaign-command')).every(element=>element.scrollWidth<=element.clientWidth+1)),`${name} board overflows`);
      await page.screenshot({path:path.join(out,`${name}.png`),fullPage:true});
    }
    assert.deepEqual(errors,[]);
    fs.writeFileSync(path.join(out,'result.json'),JSON.stringify({passed:true,fixture:'synthetic DTO; actual shipped renderer/handlers',viewports:[1440,768,390],operation_submission:true,server_refusal:true,peace_and_garrison_forms:true,no_horizontal_overflow:true},null,2));
    console.log('Campaign module: browser controls, refusal recovery and desktop/tablet/mobile layout passed.');
  }finally{await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
