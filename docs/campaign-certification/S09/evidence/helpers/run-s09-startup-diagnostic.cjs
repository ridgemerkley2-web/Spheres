const fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process'),net=require('node:net'),assert=require('node:assert/strict');
const base=__dirname,repo=path.join(base,'integration'),{chromium}=require(path.join(repo,'tools/ui/node_modules/playwright'));
const output=path.join(base,'evidence/S09-startup-diagnostic');fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'chrome-')),run=path.join(out,'server');fs.mkdirSync(run);
const start=performance.now(),elapsed=()=>Math.round((performance.now()-start)*100)/100;
const e={kind:'startup diagnosis; not qualification',expected_revision:'94d2c094b2c6',started_utc:new Date().toISOString(),events:[],requests:{},errors:[],console:[],post_paths:[]};
const write=(name,value)=>fs.writeFileSync(path.join(out,name),JSON.stringify(value,null,2)+'\n');
const event=(name,data={})=>{const row={ms:elapsed(),name,...data};e.events.push(row);fs.appendFileSync(path.join(out,'events.jsonl'),JSON.stringify(row)+'\n');};
const bounded=(promise,ms,label)=>Promise.race([promise,new Promise((_,reject)=>{const t=setTimeout(()=>reject(new Error(label+' exceeded '+ms+'ms')),ms);t.unref();})]);
async function main(){
 const reservation=net.createServer();await new Promise(r=>reservation.listen(0,'127.0.0.1',r));const port=reservation.address().port;await new Promise(r=>reservation.close(r));const url='http://127.0.0.1:'+port;e.url=url;
 const server=cp.spawn(path.join(base,'integration-target/release/spheres-web.exe'),['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});e.owned_server_pid=server.pid;
 const log=fs.createWriteStream(path.join(out,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);let browser,page,cdp,profiling=false;console.log(JSON.stringify({out,url,pid:server.pid}));
 try{
  for(let i=0;;i++){assert.equal(server.exitCode,null);try{const r=await fetch(url+'/api/build',{signal:AbortSignal.timeout(1000)});if(r.ok){e.build=await r.json();break;}}catch(error){if(i>99)throw error;}await new Promise(r=>setTimeout(r,100));}assert.equal(e.build.revision,e.expected_revision);
  browser=await chromium.launch({headless:true,channel:'chrome',timeout:30000});e.browser=browser.version();page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});page.setDefaultTimeout(30000);
  page.on('pageerror',error=>{e.errors.push(error.message);event('pageerror',{message:error.message});});page.on('console',message=>{const row={ms:elapsed(),type:message.type(),text:message.text(),location:message.location()};e.console.push(row);});page.on('crash',()=>event('page-crash'));page.on('close',()=>event('page-close'));page.on('request',request=>{if(request.method()==='POST')e.post_paths.push(new URL(request.url()).pathname);});
  cdp=await page.context().newCDPSession(page);for(const name of ['Page.enable','Network.enable','Runtime.enable','Profiler.enable'])await bounded(cdp.send(name),5000,name);await cdp.send('Page.setLifecycleEventsEnabled',{enabled:true});
  for(const name of ['Page.domContentEventFired','Page.loadEventFired','Page.frameStartedLoading','Page.frameStoppedLoading','Page.frameNavigated','Page.lifecycleEvent','Inspector.targetCrashed'])cdp.on(name,data=>event(name,data));
  cdp.on('Runtime.exceptionThrown',data=>event('Runtime.exceptionThrown',data));
  cdp.on('Network.requestWillBeSent',data=>{e.requests[data.requestId]={ms:elapsed(),url:data.request.url,method:data.request.method,type:data.type,initiator:data.initiator,documentURL:data.documentURL};});
  cdp.on('Network.responseReceived',data=>{Object.assign(e.requests[data.requestId]||=( {}),{response_ms:elapsed(),status:data.response.status,mimeType:data.response.mimeType,fromDiskCache:data.response.fromDiskCache,timing:data.response.timing});});
  cdp.on('Network.loadingFinished',data=>{Object.assign(e.requests[data.requestId]||=( {}),{finished_ms:elapsed(),encodedDataLength:data.encodedDataLength});});
  cdp.on('Network.loadingFailed',data=>{Object.assign(e.requests[data.requestId]||=( {}),{failed_ms:elapsed(),errorText:data.errorText,canceled:data.canceled,blockedReason:data.blockedReason});});
  await cdp.send('Profiler.setSamplingInterval',{interval:1000});await cdp.send('Profiler.start');profiling=true;event('navigate-start');
  try{await page.goto(url,{waitUntil:'commit',timeout:30000});event('navigate-commit',{url:page.url()});await page.waitForLoadState('domcontentloaded',{timeout:30000});event('domcontentloaded-observed',{url:page.url()});e.domcontentloaded=true;}catch(error){e.navigation_failure={message:error.message,url:page.url()};event('navigation-timeout',e.navigation_failure);e.domcontentloaded=false;}
  e.current_url=page.url();
  try{const {profile}=await bounded(cdp.send('Profiler.stop'),5000,'Profiler.stop');profiling=false;write('cpu-profile.cpuprofile',profile);const ids=new Map(profile.nodes.map(n=>[n.id,n]));const counts=new Map();for(const id of profile.samples||[])counts.set(id,(counts.get(id)||0)+1);e.cpu_top_samples=[...counts].sort((a,b)=>b[1]-a[1]).slice(0,30).map(([id,samples])=>({samples,callFrame:ids.get(id)?.callFrame,hitCount:ids.get(id)?.hitCount}));event('cpu-profile-saved',{nodes:profile.nodes.length,samples:profile.samples?.length});}catch(error){e.cpu_profile_error=error.message;}
  for(const [name,filename]of [['Performance.getMetrics','performance-metrics.json'],['Page.getFrameTree','frame-tree.json']])try{write(filename,await bounded(cdp.send(name),3000,name));}catch(error){e.errors.push(error.message);}
  e.pending_requests=Object.values(e.requests).filter(r=>!r.finished_ms&&!r.failed_ms);
  if(e.domcontentloaded){try{await bounded(page.screenshot({path:path.join(out,'initial-page.png')}),5000,'initial screenshot');}catch(error){e.screenshot_error=error.message;}}
  console.log(JSON.stringify({out,domcontentloaded:e.domcontentloaded,pending:e.pending_requests.map(r=>r.url),errors:e.errors,cpu_top_samples:e.cpu_top_samples?.slice(0,12)}));
 }catch(error){e.failure={message:error.message,stack:error.stack};console.log(JSON.stringify({out,failure:error.message}));process.exitCode=1;}
 finally{if(profiling&&cdp)try{await bounded(cdp.send('Profiler.stop'),2000,'final profiler stop');}catch{}if(browser)try{await bounded(browser.close(),10000,'browser.close');}catch(error){e.cleanup_error=error.message;}if(server.exitCode===null){server.kill();await Promise.race([new Promise(r=>server.once('exit',r)),new Promise(r=>setTimeout(r,5000))]);}log.end();e.owned_server_exit={code:server.exitCode,signal:server.signalCode};e.finished_utc=new Date().toISOString();write('result.json',e);}
}
main().catch(error=>{console.error(error);process.exitCode=1;});
