// Exercise the ordinary host callbacks with synthetic UI state; no campaign grants.
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const page=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/index.html'),'utf8');
const start=page.indexOf('function bindForceAllocations('),end=page.indexOf('\n}',start)+2;
assert(start>=0&&end>start);
const source=page.slice(start,end),command={kind:'force_allocation',conflict:7,share_bp:500};
function fixture(){
  const calls=[],container={isConnected:true,querySelectorAll:()=>[]};let send,navigate;
  const c=vm.createContext({S:{session_id:'one',player:'France'},COMMAND_CHANNEL:{busy:false,pending:false},advancing:false,pendingAdvance:false,
    window:{MilitaryOperationsUI:{bind:(_,s,n)=>{send=s;navigate=n;}}},clockPause:()=>calls.push(['pause']),
    api:async(...args)=>{calls.push(['api',...args]);return {errors:[]};},adopt:async(...args)=>calls.push(['adopt',...args]),
    openEquipment:async options=>{calls.push(['equipment',JSON.parse(JSON.stringify(options))]);return true;}
  });
  vm.runInContext(source,c);c.bindForceAllocations(container);return {c,calls,container,send:cmd=>send(cmd),navigate:tab=>navigate(tab)};
}
test('old, detached and busy allocation forms cannot dispatch or open equipment',async()=>{
  for(const mode of ['world','detached','busy','pending','advance','advance-pending']){
    const f=fixture();
    if(mode==='world')f.c.S={session_id:'two',player:'Japan'};
    if(mode==='detached')f.container.isConnected=false;
    if(mode==='busy')f.c.COMMAND_CHANNEL.busy=true;
    if(mode==='pending')f.c.COMMAND_CHANNEL.pending={};
    if(mode==='advance')f.c.advancing=true;
    if(mode==='advance-pending')f.c.pendingAdvance={};
    await assert.rejects(f.send(command),/campaign or current order changed/);
    await assert.rejects(f.navigate('service'),/campaign or current order changed/);
    assert.deepEqual(f.calls,[],mode);
  }
});
test('one confirmed allocation adopts its response; uncertain or stale responses do not',async()=>{
  const good=fixture();await good.send(command);assert.equal(good.calls.filter(c=>c[0]==='api').length,1);assert.equal(good.calls.filter(c=>c[0]==='adopt').length,1);
  for(const mode of ['pending','error','replaced']){
    const f=fixture();f.c.api=async()=>{f.calls.push(['api']);if(mode==='replaced')f.c.S={session_id:'two'};return {command_pending:mode==='pending',errors:mode==='error'?['Conflict ended.']:[]};};
    await assert.rejects(f.send(command),/awaits confirmation|campaign changed|Conflict ended/);
    assert.deepEqual(f.calls,[['api']]);
  }
});
test('valid equipment shortcuts pause and open the exact existing tab without a command',async()=>{
  for(const tab of ['service','ammunition','companies']){
    const f=fixture();await f.navigate(tab);assert.deepEqual(f.calls,[['pause'],['equipment',{tab}]]);
  }
  const f=fixture();await assert.rejects(f.navigate('market'),/unavailable/);assert.deepEqual(f.calls,[]);
});
test('editing the ordinary host retains its required CRLF line endings',()=>{
  assert.equal(page.split('\n').length,page.split('\r\n').length);
});
