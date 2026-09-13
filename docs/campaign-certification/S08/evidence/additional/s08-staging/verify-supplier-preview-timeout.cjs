// Bounded test of the actual harness helper. No browser, server, or archive.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),crypto=require('node:crypto');
const source=path.resolve(__dirname,'../integration/tools/ui/ci-supplier-imports.cjs');
const patch=path.join(__dirname,'supplier-preview-timeout.patch');
const output=process.argv[2]||path.join(__dirname,'supplier-preview-timeout-vm-evidence.json');
const sourceText=fs.readFileSync(source,'utf8'),match=sourceText.match(/async function withHeldSupplierPreview\(page,trigger,whileHeld\)\{[\s\S]*?\n\}/);
assert(match,'Apply the prepared helper patch before running this verification');
const helper=match[0],hash=value=>crypto.createHash('sha256').update(value).digest('hex');
const evidence={passed:false,started_utc:new Date().toISOString(),source,source_sha256:hash(sourceText),helper_sha256:hash(helper),patch_sha256:hash(fs.readFileSync(patch)),tests:[]};
const write=()=>{fs.writeFileSync(output,JSON.stringify(evidence,null,2)+'\n');fs.writeFileSync(output.replace(/\.json$/,'')+'.log',evidence.tests.map(test=>(test.passed?'PASS ':'FAIL ')+test.name+(test.error?' '+test.error:'')).join('\n')+'\n');};
const watchdog=setTimeout(()=>{evidence.failure='Bounded VM verification exceeded 10 real seconds';write();console.error(evidence.failure);process.exit(1);},10000);

function fixture(){
  const timers=new Map(),trace=[],handlers=new Map();let next=0;
  const unrelated=()=>{};handlers.set('unrelated',unrelated);
  const fn=vm.runInNewContext(helper+'\nwithHeldSupplierPreview',{
    setTimeout(callback,ms){assert.equal(ms,30000);const id=++next;timers.set(id,callback);return id;},
    clearTimeout(id){timers.delete(id);}
  });
  const page={handler:null,removed:0,
    async route(pattern,handler){assert.equal(pattern,'**/api/equipment-preview');this.handler=handler;handlers.set(pattern,handler);trace.push('registered');},
    async unroute(pattern,handler){assert.equal(pattern,'**/api/equipment-preview');assert.equal(handler,this.handler,'Only this exact handler may be removed');assert.equal(handlers.get(pattern),handler);handlers.delete(pattern);this.removed++;trace.push('removed');}
  };
  function route(options={}){
    const calls={parse:0,fetch:0,fulfill:0,dispose:0,continue:0};
    const response={async dispose(){calls.dispose++;trace.push('dispose');if(options.disposeError)throw options.disposeError;}};
    return {calls,
      request(){return {postDataJSON(){calls.parse++;if(options.parseError)throw options.parseError;return {command:{kind:options.kind||'company_import_purchase'}};}};},
      async fetch(optionsFetch){assert.equal(optionsFetch.timeout,30000);calls.fetch++;trace.push('fetch');if(options.fetchError)throw options.fetchError;return response;},
      async fulfill(value){assert.equal(value.response,response);calls.fulfill++;trace.push('fulfill');if(options.fulfillError)throw options.fulfillError;},
      async continue(){calls.continue++;trace.push('continue');if(options.continueError)throw options.continueError;}
    };
  }
  const observe=promise=>promise.then(value=>({ok:true,value}),error=>({ok:false,error}));
  const clean=()=>{assert.equal(page.removed,1);assert.equal(handlers.size,1);assert.equal(handlers.get('unrelated'),unrelated);assert.equal(timers.size,0,'Every manual timeout must be cleared');};
  return {fn,page,route,timers,trace,observe,clean};
}
async function flush(){for(let i=0;i<20;i++)await Promise.resolve();}
async function test(name,run){const row={name,passed:false};evidence.tests.push(row);try{await run();row.passed=true;}catch(error){row.error=String(error.stack||error).slice(0,6000);throw error;}}

async function main(){
  await test('success holds the real first response and continues later matches',async()=>{
    const f=fixture(),first=f.route(),later=f.route();let request,dayFinished=false;
    const result=await f.observe(f.fn(f.page,async()=>{request=f.page.handler(first);},async()=>{
      assert.equal(first.calls.fetch,1);assert.equal(first.calls.fulfill,0);assert.equal(first.calls.dispose,0);
      await f.page.handler(later);assert.equal(later.calls.continue,1);assert.equal(later.calls.parse,0);assert.equal(later.calls.fetch,0);
      dayFinished=true;return '1995-10-29';
    }));
    assert(result.ok);assert.equal(result.value,'1995-10-29');assert(dayFinished);await request;
    assert.equal(first.calls.fulfill,1);assert.equal(first.calls.dispose,1);assert(f.trace.indexOf('fulfill')>f.trace.indexOf('fetch'));f.clean();
  });
  await test('fetch failure reaches the awaited result without calling the day callback',async()=>{
    const f=fixture(),error=Error('sentinel fetch failure'),r=f.route({fetchError:error});let request,calls=0;
    const result=await f.observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{calls++;}));
    assert(!result.ok);assert.equal(result.error,error);await request;assert.equal(calls,0);assert.equal(r.calls.dispose,0);f.clean();
  });
  await test('no interception fails explicitly using the 30-second fake timeout',async()=>{
    const f=fixture();let calls=0;const result=f.observe(f.fn(f.page,async()=>{},async()=>{calls++;}));
    await flush();assert.equal(f.timers.size,1);const [id,callback]=f.timers.entries().next().value;f.timers.delete(id);callback();
    const outcome=await result;assert(!outcome.ok);assert.match(String(outcome.error),/timeout after 30000 ms: waiting for the fetched preview/);assert.equal(calls,0);f.clean();
  });
  for(const failure of ['fulfillError','disposeError'])await test(failure+' reaches the awaited result and still disposes once',async()=>{
    const f=fixture(),error=Error('sentinel '+failure),r=f.route({[failure]:error});let request;
    const result=await f.observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{}));
    assert(!result.ok);assert.equal(result.error,error);await request;assert.equal(r.calls.fulfill,1);assert.equal(r.calls.dispose,1);f.clean();
  });
  await test('failed day callback releases the hold and keeps both cleanup causes visible',async()=>{
    const f=fixture(),dayError=Error('sentinel visible-day failure'),disposeError=Error('sentinel dispose failure'),r=f.route({disposeError});let request;
    const result=await f.observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{throw dayError;}));
    assert(!result.ok);assert.match(String(result.error),/sentinel visible-day failure/);assert.match(String(result.error),/sentinel dispose failure/);
    assert.equal(result.error.errors.length,2);assert(result.error.errors.includes(dayError));assert(result.error.errors.includes(disposeError));
    await request;assert.equal(r.calls.fulfill,1);assert.equal(r.calls.dispose,1);f.clean();
  });
  await test('request parsing failure is forwarded without waiting for interception',async()=>{
    const f=fixture(),error=Error('sentinel request parse failure'),r=f.route({parseError:error});let request,calls=0;
    const result=await f.observe(f.fn(f.page,async()=>{request=f.page.handler(r);},async()=>{calls++;}));
    assert(!result.ok);assert.equal(result.error,error);await request;assert.equal(calls,0);assert.equal(r.calls.fetch,0);f.clean();
  });
  await test('nonmatching continue failures are not swallowed or treated as interception',async()=>{
    const f=fixture(),error=Error('sentinel nonmatching continue failure'),other=f.route({kind:'equipment_maintenance',continueError:error}),target=f.route();let request;
    const result=await f.observe(f.fn(f.page,async()=>{
      await assert.rejects(f.page.handler(other),cause=>cause===error);request=f.page.handler(target);
    },async()=>{}));
    assert(result.ok);assert.equal(other.calls.continue,1);assert.equal(other.calls.fetch,0);assert.equal(target.calls.fetch,1);await request;f.clean();
  });
  evidence.passed=true;
}
main().catch(error=>{evidence.failure=String(error.stack||error).slice(0,6000);process.exitCode=1;}).finally(()=>{
  clearTimeout(watchdog);evidence.finished_utc=new Date().toISOString();write();console.log(JSON.stringify({passed:evidence.passed,tests:evidence.tests.length,evidence:output}));
});
