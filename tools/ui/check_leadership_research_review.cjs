const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),{webcrypto,createHash}=require('node:crypto');
const review=require('./leadership-research-review.js');
const root=path.resolve(__dirname,'../..'),read=p=>JSON.parse(fs.readFileSync(path.join(root,p),'utf8'));
const index=read('docs/campaign-certification/C01/research-index.json');
const countries=read('docs/campaign-certification/C01/countries.json');
const clone=value=>JSON.parse(JSON.stringify(value));
const row=index.countries.find(c=>c.nation==='Tonga'),packet=read(row.packet);
const deferred=()=>{let resolve,reject;const promise=new Promise((a,b)=>{resolve=a;reject=b;});return {resolve,reject,promise};};

test('all country identities remain selectable and unknown coverage is not zero',()=>{
  const before=clone(countries),rows=review.validateIndex(index,countries);
  assert.equal(rows.length,160);assert.equal(rows.filter(c=>c.certified_case).length,9);
  assert.equal(rows.filter(c=>c.discovery).length,index.counts.country_packets);
  assert.ok(rows.some(c=>c.party_rows===0&&c.discovery));
  assert.equal(rows.find(c=>c.id==='UK').discovery,null);assert.deepEqual(countries,before);
});
test('every indexed country packet can be reviewed without changing source observations',()=>{
  for(const row of index.countries){const packet=read(row.packet),before=clone(packet),board=review.preparePacket(packet,row);
    assert.equal(board.entries.length,row.organization_observations+row.institution_observations);assert.deepEqual(packet,before);
  }
});
test('wrong country, stale counts and false completed coverage cannot be presented',()=>{
  assert.throws(()=>review.preparePacket(packet,{...row,nation:'Russia'}),/does not match/);
  assert.throws(()=>review.preparePacket(packet,{...row,organization_observations:999}),/counts differ/);
  for(const flag of ['c01_complete','g2_prerequisite_satisfied','runtime_roster_modified'])assert.throws(()=>review.validateIndex({...index,[flag]:true},countries),/partial discovery/);
  assert.throws(()=>review.validateIndex({...index,countries:[...index.countries,index.countries[0]]},countries),/coverage/);
});
test('holder claims resolve only through a source cited by their separate office',()=>{
  const p=clone(packet),role=p.institutions.flatMap(i=>i.roles).find(r=>r.holder_claims.length);
  role.holder_claims=['does-not-exist'];assert.throws(()=>review.preparePacket(p,row),/holder claim/);
  const altered=clone(packet);altered.organizations[0].claim_ids=['does-not-exist'];assert.throws(()=>review.preparePacket(altered,row),/another source/);
});
test('search includes sourced people and preserves organization versus institution filters',()=>{
  const board=review.preparePacket(packet,row);
  const people=review.selectEntries(board,"Tu'i'onetoa");assert.ok(people.length);
  assert.ok(review.selectEntries(board,'','institutions').every(e=>e.category==='institutions'));
  assert.ok(review.selectEntries(board,'','organizations').every(e=>e.category==='organizations'));
  assert.ok(review.selectEntries(board,'','roles').every(e=>e.roles.length));
  assert.equal(review.selectEntries(board,'a totally absent string').length,0);
  assert.throws(()=>review.selectEntries(board,'','future-appointed'),/Unknown/);
});
test('date wording retains unknown, month precision and observation versus interval',()=>{
  assert.equal(review.dateText(null),'Not established');
  assert.equal(review.dateText({kind:'month',value:'1990-07'}),'1990-07 (month precision)');
  assert.match(review.observationDate({attested_on:'2021-05-28'}),/^Observed on/);
  assert.match(review.observationDate({name:'Example'}),/not established/);
});

test('society testimony dates are displayed as observations without inventing officer terms',()=>{
  const party=packet.organizations.find(e=>e.id==='to_peoples_party'),before=clone(party);
  for(const role of party.roles.filter(r=>r.kind==='other')){
    const holder=role.holder_claims[0],label=review.observationDate(holder);
    assert.match(label,/Observed between 2022-04-19 and 2022-04-21/);
    assert.match(label,/office term not established/);
    assert.doesNotMatch(label,/Reported interval/);
  }
  assert.deepEqual(party,before);
  assert.match(review.observationDate({attested_period:{from:'2022-04-19',through:null}}),/and Not established; office term not established/);
  assert.match(review.observationDate({attested_period:{from:'2022-04-19',through:'2022-04-21'},from:'2021-01-01',until:null}),/reported office interval: 2021-01-01 → Not established/);
});
test('source navigation rejects script URLs, credentials and filesystem traversal',()=>{
  assert.equal(review.publicURL('https://example.org/report'),'https://example.org/report');
  for(const value of ['javascript:alert(1)','file:///secret','https://name:pass@example.org','http://example.org'])assert.equal(review.publicURL(value),null);
  assert.equal(review.sourcePath(row.packet),'../../'+row.packet);
  for(const value of ['../secret','docs/campaign-certification/C01/research/../secret.json','docs/campaign-certification/C01/research/%2e%2e.json','https://example.org/packet.json'])assert.equal(review.sourcePath(value),null);
});
test('packet loader verifies exact bytes and fails closed on changed source',async()=>{
  const bytes=new TextEncoder().encode(JSON.stringify(packet)),descriptor={path:row.packet,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};
  const fetcher=async url=>{assert.equal(url,'../../'+row.packet);return {ok:true,arrayBuffer:async()=>bytes.buffer};};
  assert.deepEqual(await review.readJSON(row.packet,descriptor,undefined,fetcher,webcrypto),packet);
  await assert.rejects(review.readJSON(row.packet,{...descriptor,sha256:'0'.repeat(64)},undefined,fetcher,webcrypto),/Source files changed/);
  await assert.rejects(review.readJSON(row.packet,undefined,undefined,fetcher,webcrypto),/Missing file identity/);
  await assert.rejects(review.readJSON(row.packet,descriptor,undefined,fetcher,{}),/localhost or HTTPS/);
});
test('late country success cannot replace the latest selected country',async()=>{
  const first=deferred(),second=deferred(),accepted=[],errors=[],signals=[];
  const choose=review.latestLoader((id,signal)=>{signals.push(signal);return id==='first'?first.promise:second.promise;},(...v)=>accepted.push(v),(...v)=>errors.push(v));
  const a=choose('first'),b=choose('second');assert.equal(signals[0].aborted,true);
  second.resolve('new country');await b;first.resolve('old country');await a;
  assert.deepEqual(accepted,[['second','new country']]);assert.deepEqual(errors,[]);
});
test('late country error cannot erase the latest data but current errors surface',async()=>{
  const first=deferred(),second=deferred(),accepted=[],errors=[];
  const choose=review.latestLoader(id=>id==='first'?first.promise:second.promise,(...v)=>accepted.push(v),(...v)=>errors.push(v));
  const a=choose('first'),b=choose('second');second.resolve(null);await b;first.reject(Error('stale'));await a;
  assert.deepEqual(accepted,[['second',null]]);assert.deepEqual(errors,[]);
  const fail=review.latestLoader(async()=>{throw Error('current');},()=>assert.fail(),(_,error)=>errors.push(error.message));await fail('current');assert.deepEqual(errors,['current']);
});
