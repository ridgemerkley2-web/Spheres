// Small VM verification of the exact outside candidate helper. No browser,
// server, game archive or production state is loaded. It models the relevant
// server-side last-interceptor removal from local Playwright 1.58.2.
'use strict';
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),crypto=require('node:crypto');
const staging=__dirname,repo=path.resolve(staging,'../integration');
const source=path.join(repo,'tools/ui/ci-supplier-imports.cjs'),candidate=path.join(staging,'ci-supplier-imports.cleanup-order.cjs'),patch=path.join(staging,'supplier-preview-cleanup-order.patch');
const output=process.argv[2]||path.join(staging,'supplier-preview-cleanup-order-vm-evidence.json');
const hash=value=>crypto.createHash('sha256').update(value).digest('hex');
const extract=file=>{const text=fs.readFileSync(file,'utf8'),match=text.match(/async function withHeldSupplierPreview\(page,trigger,whileHeld\)\{[\s\S]*?\n\}/);assert(match);return {file,text,helper:match[0],sha256:hash(text),helper_sha256:hash(match[0])};};
const original=extract(source),fixed=extract(candidate);
const bindings=JSON.parse(fs.readFileSync(path.join(staging,'supplier-preview-cleanup-order-bindings.json'),'utf8'));
assert.equal(original.sha256,bindings.source_sha256);assert.equal(fixed.sha256,bindings.candidate_sha256);assert.equal(hash(fs.readFileSync(patch)),bindings.patch_sha256);
const pw=path.join(repo,'tools/ui/node_modules/playwright-core');
const evidence={passed:false,started_utc:new Date().toISOString(),source:{path:source,sha256:original.sha256,helper_sha256:original.helper_sha256},candidate:{path:candidate,sha256:fixed.sha256,helper_sha256:fixed.helper_sha256},patch_sha256:bindings.patch_sha256,verifier_sha256:hash(fs.readFileSync(__filename)),playwright_version:JSON.parse(fs.readFileSync(path.join(pw,'package.json'),'utf8')).version,playwright_sources:['lib/client/page.js','lib/server/dispatchers/pageDispatcher.js','lib/server/page.js','lib/server/network.js'].map(name=>({name,sha256:hash(fs.readFileSync(path.join(pw,name)))})),tests:[]};
const write=()=>{fs.writeFileSync(output,JSON.stringify(evidence,null,2)+'\n');fs.writeFileSync(output.replace(/\.json$/,'')+'.log',evidence.tests.map(row=>(row.passed?'PASS ':'FAIL ')+row.name+(row.error?' '+row.error:'')).join('\n')+'\n');};
const watchdog=setTimeout(()=>{evidence.failure='VM verification exceeded 10 real seconds';write();console.error(evidence.failure);process.exit(1);},10000);
const deferred=()=>{let resolve;const promise=new Promise(done=>resolve=done);return {promise,resolve};};
const observe=promise=>promise.then(value=>({ok:true,value}),error=>({ok:false,error}));
async function flush(){for(let i=0;i<30;i++)await Promise.resolve();}

function fixture(helper=fixed.helper,options={}){
  const timers=new Map(),trace=[],pageHandlers=new Map(),inflight=new Set();let next=0;
  const unrelatedPage=()=>{},unrelatedContext=()=>{};
  const contextHandlers=new Map([['unrelated-context',unrelatedContext]]);
  if(options.unrelatedPage)pageHandlers.set('unrelated-page',unrelatedPage);
  const fn=vm.runInNewContext(helper+'\nwithHeldSupplierPreview',{
    setTimeout(callback,ms){assert.equal(ms,30000);const id=++next;timers.set(id,callback);return id;},clearTimeout(id){timers.delete(id);}
  });
  const page={handler:null,removed:0,
    async route(pattern,handler){assert.equal(pattern,'**/api/equipment-preview');this.handler=handler;pageHandlers.set(pattern,handler);trace.push('registered');},
    async unroute(pattern,handler){
      assert.equal(pattern,'**/api/equipment-preview');assert.equal(handler,this.handler);assert.equal(pageHandlers.get(pattern),handler);
      pageHandlers.delete(pattern);this.removed++;trace.push('unroute');
      // Local PW: removing its last page URL pattern removes the server page
      // interceptor. Every held current route then falls back immediately.
      if(!pageHandlers.size)for(const route of inflight)route.serverContinueOnRemove();
      if(options.unrouteError)throw options.unrouteError;
    }
  };
  function route(config={}){
    const calls={parse:0,fetch:0,fulfill:0,fulfilled:0,dispose:0,continue:0,serverContinue:0};let serverHandled=false;
    const response={async dispose(){calls.dispose++;trace.push('dispose:start');if(config.disposeGate)await config.disposeGate.promise;if(config.disposeError)throw config.disposeError;trace.push('dispose:done');}};
    const result={calls,
      request(){return {postDataJSON(){calls.parse++;if(config.parseError)throw config.parseError;return {command:{kind:config.kind||'company_import_purchase'}};}};},
      async fetch(fetchOptions){assert.equal(fetchOptions.timeout,30000);calls.fetch++;inflight.add(result);trace.push('fetch');if(config.fetchError)throw config.fetchError;return response;},
      serverContinueOnRemove(){if(!serverHandled){calls.serverContinue++;serverHandled=true;inflight.delete(result);trace.push('server:continued-on-remove');}},
      async fulfill(value){
        assert.equal(value.response,response);calls.fulfill++;trace.push('fulfill:requested');
        // A protocol call does not synchronously reach the server. This yield
        // lets old cleanup remove the interceptor before its server handling.
        await Promise.resolve();
        if(serverHandled)throw Error('route.fulfill: Route is already handled!');
        if(config.fulfillError)throw config.fulfillError;
        serverHandled=true;if(config.fulfillGate)await config.fulfillGate.promise;
        calls.fulfilled++;inflight.delete(result);trace.push('fulfill:done');
      },
      async continue(){calls.continue++;trace.push('continue');if(config.continueError)throw config.continueError;}
    };
    return result;
  }
  const clean=()=>{assert.equal(page.removed,1);assert.equal(contextHandlers.get('unrelated-context'),unrelatedContext);assert.equal(contextHandlers.size,1);assert.equal(pageHandlers.size,options.unrelatedPage?1:0);if(options.unrelatedPage)assert.equal(pageHandlers.get('unrelated-page'),unrelatedPage);assert.equal(timers.size,0,'Manual timeout must clear');};
  const expire=()=>{assert.equal(timers.size,1);const [id,callback]=timers.entries().next().value;timers.delete(id);callback();};
  return {fn,page,route,timers,trace,clean,expire};
}
async function test(name,run){const row={name,passed:false};evidence.tests.push(row);try{await run(row);row.passed=true;}catch(error){row.error=String(error.stack||error).slice(0,6000);throw error;}}

async function main(){
  await test('frozen original deterministically reproduces last-interceptor double handling',async row=>{
    const f=fixture(original.helper),r=f.route();let request;
    const result=await observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=> 'new date'));
    assert(!result.ok);assert.match(String(result.error),/Route is already handled!/);await request;
    assert.equal(r.calls.serverContinue,1);assert.equal(r.calls.fulfilled,0);assert.equal(r.calls.dispose,1);f.clean();row.trace=f.trace;row.expected_original_failure=String(result.error);
  });
  await test('candidate finishes held response before exact unroute and continues later matches',async row=>{
    const f=fixture(),first=f.route(),later=f.route();let request;
    const result=await observe(f.fn(f.page,async()=>{request=f.page.handler(first);},async()=>{
      assert.equal(first.calls.fetch,1);assert.equal(first.calls.fulfill,0);assert.equal(first.calls.dispose,0);
      await f.page.handler(later);assert.equal(later.calls.continue,1);assert.equal(later.calls.parse,0);assert.equal(later.calls.fetch,0);return '1995-10-29';
    }));
    assert(result.ok);assert.equal(result.value,'1995-10-29');await request;assert.equal(first.calls.fulfilled,1);assert.equal(first.calls.serverContinue,0);assert.equal(first.calls.dispose,1);
    assert(f.trace.indexOf('unroute')>f.trace.indexOf('dispose:done'));f.clean();row.trace=f.trace;
  });
  await test('pending fulfill and pending dispose both keep this interceptor registered',async row=>{
    const f=fixture(),fulfillGate=deferred(),disposeGate=deferred(),r=f.route({fulfillGate,disposeGate});let request;
    const pending=observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{}));await flush();
    assert.equal(r.calls.fulfill,1);assert.equal(r.calls.fulfilled,0);assert.equal(f.page.removed,0);
    fulfillGate.resolve();await flush();assert.equal(r.calls.fulfilled,1);assert.equal(r.calls.dispose,1);assert.equal(f.page.removed,0);
    disposeGate.resolve();assert((await pending).ok);await request;f.clean();row.trace=f.trace;
  });
  await test('unrelated page and context handlers retain exact identity',async()=>{
    const f=fixture(fixed.helper,{unrelatedPage:true}),r=f.route();let request;
    assert((await observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{}))).ok);await request;f.clean();
  });
  await test('fetch failure propagates and does not run the date callback',async()=>{
    const f=fixture(),error=Error('sentinel fetch failure'),r=f.route({fetchError:error});let request,days=0;
    const result=await observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{days++;}));
    assert(!result.ok);assert.equal(result.error,error);await request;assert.equal(days,0);assert.equal(r.calls.dispose,0);f.clean();
  });
  await test('no interception retains the explicit 30-second failure and cleanup',async()=>{
    const f=fixture();let days=0;const pending=observe(f.fn(f.page,async()=>{},async()=>{days++;}));await flush();f.expire();
    const result=await pending;assert(!result.ok);assert.match(String(result.error),/timeout after 30000 ms: waiting for the fetched preview/);assert.equal(days,0);f.clean();
  });
  for(const key of ['fulfillError','disposeError'])await test(key+' remains visible and disposes exactly once',async()=>{
    const f=fixture(),error=Error('sentinel '+key),r=f.route({[key]:error});let request;
    const result=await observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{}));
    assert(!result.ok);assert.equal(result.error,error);await request;assert.equal(r.calls.fulfill,1);assert.equal(r.calls.dispose,1);f.clean();
  });
  await test('failed visible day still releases the hold and preserves both error causes',async()=>{
    const f=fixture(),dayError=Error('sentinel date change failure'),disposeError=Error('sentinel dispose failure'),r=f.route({disposeError});let request;
    const result=await observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{throw dayError;}));
    assert(!result.ok);assert.equal(result.error.errors.length,2);assert(result.error.errors.includes(dayError));assert(result.error.errors.includes(disposeError));await request;assert.equal(r.calls.fulfilled,1);f.clean();
  });
  await test('request parse failure is forwarded without a missing-interception wait',async()=>{
    const f=fixture(),error=Error('sentinel parse failure'),r=f.route({parseError:error});let request,days=0;
    const result=await observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{days++;}));
    assert(!result.ok);assert.equal(result.error,error);await request;assert.equal(days,0);assert.equal(r.calls.fetch,0);f.clean();
  });
  await test('nonmatching continue errors are not swallowed or claimed as interception',async()=>{
    const f=fixture(),error=Error('sentinel continue failure'),other=f.route({kind:'equipment_maintenance',continueError:error}),r=f.route();let request;
    const result=await observe(f.fn(f.page,async()=>{await assert.rejects(f.page.handler(other),cause=>cause===error);request=f.page.handler(r);},async()=>{}));
    assert(result.ok);await request;assert.equal(other.calls.continue,1);assert.equal(other.calls.fetch,0);f.clean();
  });
  await test('completion timeout still removes only this handler and fails explicitly',async()=>{
    const f=fixture(),fulfillGate=deferred(),r=f.route({fulfillGate});let request;
    const pending=observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{}));await flush();assert.equal(f.page.removed,0);f.expire();
    const result=await pending;assert(!result.ok);assert.match(String(result.error),/timeout after 30000 ms: finishing the held preview/);
    fulfillGate.resolve();await request;f.clean();
  });
  await test('unroute errors remain visible after successful response settlement',async()=>{
    const error=Error('sentinel unroute failure'),f=fixture(fixed.helper,{unrouteError:error}),r=f.route();let request;
    const result=await observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{}));
    assert(!result.ok);assert.equal(result.error,error);await request;assert.equal(r.calls.fulfilled,1);assert.equal(r.calls.dispose,1);f.clean();
  });
  evidence.passed=true;
}
main().catch(error=>{evidence.failure=String(error.stack||error).slice(0,6000);process.exitCode=1;}).finally(()=>{
  clearTimeout(watchdog);evidence.finished_utc=new Date().toISOString();write();console.log(JSON.stringify({passed:evidence.passed,tests:evidence.tests.length,evidence:output}));
});
