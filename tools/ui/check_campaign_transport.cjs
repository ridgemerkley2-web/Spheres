const {test}=require('node:test');const assert=require('node:assert/strict');
const transport=require('../../spheres-web/ui/campaign-transport.js');
function fixture(request,existing){const map=existing||new Map();let seq=0;return {map,client:transport.create({request,session:()=>"campaign-1",identity:()=>({client_id:"browser-1",request_seq:++seq}),storage:{getItem:k=>map.get(k),setItem:(k,v)=>map.set(k,v),removeItem:k=>map.delete(k)}})};}
test('immediate response loss/reload/retry charges exactly once and preserves original command',async()=>{
  const receipts=new Map();let charges=0,lose=true;
  const server=async p=>{const key=p.client_id+":"+p.request_seq;if(!receipts.has(key)){charges++;receipts.set(key,p);}if(lose){lose=false;throw Error('lost committed response');}return {session_id:p.session_id,errors:[]};};
  const f=fixture(server),body={commands:[{kind:"improve",target:"France"}]};
  await assert.rejects(f.client.send(body),/lost committed/);body.commands[0].target="Japan";
  assert.equal(f.client.pending.commands[0].target,"France");
  const reload=fixture(server,f.map);await reload.client.retry();assert.equal(charges,1);assert.equal(reload.client.pending,null);
});
test('a second click while in flight cannot submit a second action',async()=>{
  let finish,calls=0;const f=fixture(p=>{calls++;return new Promise(resolve=>finish=()=>resolve({session_id:p.session_id}));});
  const result=f.client.send({commands:[{kind:'improve',target:'France'}]});
  await assert.rejects(f.client.send({commands:[{kind:'improve',target:'France'}]}),/pending order/);
  finish();await result;assert.equal(calls,1);
});
test('certain malformed rejection releases write lane; unknown outcome and stale campaign do not',async()=>{
  const f=fixture(async()=>{const e=Error('malformed');e.notApplied=true;throw e;});await assert.rejects(f.client.send({commands:[]}));assert.equal(f.client.pending,null);
  const g=fixture(async()=>{throw Error('offline')});await assert.rejects(g.client.send({commands:[]}));assert.ok(g.client.pending);
  const old=transport.create({request:()=>assert.fail('must not replay old campaign'),session:()=>"replacement",storage:{getItem:k=>g.map.get(k)}});
  await assert.rejects(old.retry(),/another campaign/);assert.ok(old.pending);old.clear();assert.equal(old.pending,null);
});
test('external commerce receipt identity is preserved by the common mutation channel',async()=>{
  let sent;const f=fixture(async p=>{sent=p;return {session_id:p.session_id};});
  await f.client.send({session_id:'campaign-1',client_id:'exchange-1',request_seq:45,commands:[{kind:'goods_trade'}]});
  assert.equal(sent.client_id,'exchange-1');assert.equal(sent.request_seq,45);
});
function point(t,reset=false){return {t:[t],labels:[`${t+1} Jan 1990`],oil:[20],epoch:1,cursor:t,reset,session_id:'campaign-1',order:['USA'],available:{USA:'United States',France:'France'},nations:{USA:{name:'United States',t0:0,gdp:[10+t],growth:[1],inflation:[2],debt:[3],stability:[4],mil:[5]}}};}
test('history deltas append exact data and compaction resets replace old indices',()=>{
  const a=point(0,true),b=point(1);const joined=transport.mergeHistory(a,b);
  assert.deepEqual(joined.nations.USA.gdp,[10,11]);assert.deepEqual(a.nations.USA.gdp,[10]);
  const compact=point(2,true);compact.epoch=2;assert.deepEqual(transport.mergeHistory(joined,compact).t,[2]);
});
test('history only asks for chosen series; a new selection gets a complete window',async()=>{
  const paths=[];let day=0;const client=transport.historyClient(async path=>{paths.push(path);return point(day++,paths.length!==2);});
  await client.read(['USA'],{session_id:'campaign-1'});await client.read(['USA'],{session_id:'campaign-1'});await client.read(['France','USA'],{session_id:'campaign-1'});
  assert.equal(paths[0],'/api/history?nations=USA');assert.match(paths[1],/epoch=1&after=0/);assert.equal(paths[2],'/api/history?nations=France,USA');
});
test('save names cannot express paths',()=>{assert.equal(transport.slotName('France-1990'),'France-1990');for(const bad of ['../save','C:\\save','a/b','',"a".repeat(65)])assert.equal(transport.slotName(bad),null);});

const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const page=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/index.html'),'utf8');
function pageFunction(name){const hit=new RegExp(`^(?:async\\s+)?function ${name}\\(`,'m').exec(page);assert(hit);return page.slice(hit.index,page.indexOf('\n}',hit.index)+2);}
test('the real immediate API wraps legacy panel failures and retries the same committed receipt',async()=>{
  let lost=true,charges=0;const receipts=new Set(),sent=[];
  const c=vm.createContext({CampaignTransport:transport,banner(){},fetch:async(url,options)=>{
    const p=JSON.parse(options.body);sent.push(p);const key=p.client_id+':'+p.request_seq;
    if(!receipts.has(key)){receipts.add(key);charges++;}
    if(lost){lost=false;throw Error('lost after commit');}
    return {ok:true,text:async()=>JSON.stringify({session_id:'campaign-1',errors:[]})};
  }});
  vm.runInContext(`let S={session_id:'campaign-1',date:'1 Jan 1990'},advancing=false,pendingAdvance=null;
    ${pageFunction('api')}
    const COMMAND_CHANNEL=CampaignTransport.create({request:p=>api('/api/command',p,true),session:()=>S.session_id,identity:()=>({client_id:'page-test',request_seq:1})});`,c);
  const failed=await vm.runInContext(`api('/api/command',{commands:[{kind:'improve',target:'France'}]})`,c);
  assert.equal(failed.command_pending,true);assert.equal(failed.date,'1 Jan 1990');assert.equal(failed.errors.length,1);
  await vm.runInContext(`api('/api/command',{commands:[{kind:'improve',target:'Japan'}]})`,c);
  assert.equal(sent.length,1,'a different action cannot overtake an unconfirmed receipt');
  await vm.runInContext('COMMAND_CHANNEL.retry()',c);assert.equal(charges,1);assert.deepEqual(sent[1],sent[0]);
});
test('a failed visible history read has no render or retry loop',async()=>{
  let requests=0,draws=0;
  const c=vm.createContext({document:{},banner(){},renderCharts(){draws++;},openNation(){draws++;}});
  const presentation=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/campaign-ui.js'),'utf8');
  vm.runInContext(`let S={session_id:'one',date:'1 Jan 1990',player:'USA'},HIST=null,selected=null;
    const ui={tab:'charts',picked:[]},CHRONICLE={view:'sheet',nation:'USA'};
    const HISTORY_READER={read:async()=>{throw Error('offline');}};`+presentation,c);
  c.countRead=()=>requests++;
  vm.runInContext('HISTORY_READER.read=async()=>{countRead();throw Error("offline");};requestVisibleHistory();',c);
  await new Promise(resolve=>setImmediate(resolve));
  vm.runInContext('requestVisibleHistory()',c);await new Promise(resolve=>setImmediate(resolve));
  assert.equal(requests,1);assert.equal(draws,0);
});
test('campaign and chronicle classic scripts parse alongside the actual page without duplicate globals',()=>{
  const files=['campaign-transport.js','campaign-ui.js','chronicle-data.js','chronicle-ui.js'];
  const scripts=files.map(name=>fs.readFileSync(path.join(__dirname,'../../spheres-web/ui',name),'utf8'));
  const inline=[...page.matchAll(/<script>([\s\S]*?)<\/script>/g)].map(m=>m[1]);
  new vm.Script([...scripts,...inline].join('\n'));
});
