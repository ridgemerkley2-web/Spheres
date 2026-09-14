const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict'),crypto=require('node:crypto');
const base=__dirname,repo=path.join(base,'integration'),out=path.join(base,'evidence/S11-final2-header-review/attempt-2'),url='http://127.0.0.1:7856';
const {chromium}=require(require.resolve('playwright',{paths:[path.join(repo,'tools/ui')]})),hash=v=>crypto.createHash('sha256').update(v).digest('hex');
async function main(){
 fs.mkdirSync(out);
 const proof={visual_diagnostic_only:true,passed:false,url,requests:[],screenshots:[],readings:[],errors:[]};let browser;
 try{
  browser=await chromium.launch({headless:true,channel:'chrome'});const page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
  page.on('request',r=>{if(r.method()==='POST')proof.requests.push({method:r.method(),path:new URL(r.url()).pathname});});page.on('pageerror',e=>proof.errors.push(e.message));
  const get=async p=>{const r=await page.request.get(url+p);try{assert(r.ok());return await r.json();}finally{await r.dispose();}};
  proof.build=await get('/api/build');assert.equal(proof.build.revision,'d9afd1604219');
  const native=await get('/api/state');assert.equal(native.player,'France');proof.before_state_sha256=hash(JSON.stringify(native));
  fs.writeFileSync(path.join(out,'before-state.json'),JSON.stringify(native));
  await page.goto(url,{waitUntil:'domcontentloaded'});await page.locator('#continueBtn').waitFor({state:'visible'});await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});await page.locator('[data-drawer="intelDrawer"]').click();
  const head=page.locator('#warsCard .military-operations > header');await head.waitFor({state:'visible'});
  for(const width of [1440,320]){
   await page.setViewportSize({width,height:width===1440?1000:844});await head.scrollIntoViewIfNeeded();
   const reading=await head.evaluate(e=>{const span=e.querySelector('span'),title=e.querySelector('h3'),rect=r=>({x:r.x,y:r.y,width:r.width,height:r.height,top:r.top,bottom:r.bottom,left:r.left,right:r.right});return {display:getComputedStyle(e).display,client_width:e.clientWidth,scroll_width:e.scrollWidth,header:rect(e.getBoundingClientRect()),eyebrow:rect(span.getBoundingClientRect()),title:rect(title.getBoundingClientRect()),text:e.innerText};});
   assert.equal(reading.display,'block');assert(reading.title.top>=reading.eyebrow.bottom,'Header title must be below the eyebrow');assert(reading.scroll_width<=reading.client_width+1);assert(reading.header.left>=0&&reading.header.right<=width);proof.readings.push({width,...reading});
   const file='header-'+width+'.png';await page.screenshot({path:path.join(out,file)});proof.screenshots.push({file,sha256:hash(fs.readFileSync(path.join(out,file)))});
  }
  const final=await get('/api/state');proof.after_state_sha256=hash(JSON.stringify(final));assert.deepEqual(final,native);fs.writeFileSync(path.join(out,'after-state.json'),JSON.stringify(final));
  assert.deepEqual(proof.requests.filter(r=>['/api/command','/api/advance','/api/save','/api/load'].includes(r.path)),[]);assert(proof.requests.every(r=>r.path==='/api/program-preview'),'Only the existing read-only programme preview POST may occur');assert.deepEqual(proof.errors,[]);proof.passed=true;
 }catch(e){proof.failure=String(e.stack||e);throw e;}finally{fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(proof,null,2));if(browser)await browser.close();}
 console.log(JSON.stringify({passed:true,result:path.join(out,'result.json')}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
