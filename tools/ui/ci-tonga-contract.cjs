// Shared assertions for real native browser evidence and adversarial harness tests.
const assert=require('node:assert/strict');
const KING='taufaahau_tupou_iv',OPENING_PM='fatafehi_tuipelehake';
const IDS=['fictional_to_lesieli_fotu','fictional_to_sitani_lolohea','fictional_to_pisila_tukuafu','fictional_to_kalolo_matalehu'];
const CASES={'tonga-reference-cutoff':'2026-09-07','tonga-first-fiction':'2026-09-08','tonga-last-fiction':'2035-12-31','tonga-after-endpoint':'2036-01-01'};
function fixtureManifest(value,revision){
  assert.equal(value.version,1);assert.equal(value.fixture,'tonga-authored-date-browser');assert.equal(value.source_revision,revision);assert.equal(value.days_advanced,0);
  assert.match(value.scope,/not historical chronology/);assert.equal(value.cases.length,4);
  assert.deepEqual(value.cases.map(c=>c.name).sort(),Object.keys(CASES).sort());
  for(const c of value.cases){assert.equal(c.file,c.name+'.json');assert.equal(c.date,CASES[c.name]);assert.equal(c.player,'Tonga');assert.equal(c.days_advanced,0);assert.equal(c.institutions.date,c.date);}
}
function board(data,date){
  assert.equal(data.nation,'Tonga');assert.equal(data.mine,true);const b=data.party_leadership?.institutional_leadership;
  assert.equal(b?.nation,'Tonga');assert.equal(b.date,date);assert.equal(b.active,true);
  assert.equal(data.party_leadership.executive_person?.id,KING,'Civilian appointment must not replace the Crown identity');
  assert.equal(data.leader.office,'King');assert.equal(data.leader.name,b.monarch.name);assert(data.leader.name);
  assert.equal(b.historical_reference_through,'2026-09-07');assert.equal(b.fictional_from,'2026-09-08');assert.equal(b.fictional_until_exclusive,'2036-01-01');
  assert.deepEqual(b.future_preview.map(c=>c.person.id).sort(),[...IDS].sort());
  for(const c of b.future_preview){assert.equal(c.person.fiction.origin,'fictional_successor');assert.equal(c.eligible_by_date,date>='2026-09-08'&&date<'2036-01-01');}
  const kalolo=b.future_preview.find(c=>c.person.id===IDS[3]);assert.equal(kalolo.role,'party_organizer');assert.deepEqual(kalolo.actions,[]);
  assert(!data.actions.some(a=>a.command?.action?.person_id===IDS[3]),'Kalolo must not acquire an appointment command');
  return b;
}
function unappointed(b){assert.equal(b.reformed,false);assert.deepEqual(b.people_representatives,[]);assert.deepEqual(b.nonelected_ministers,[]);assert.equal(b.prime_minister?.person.id,OPENING_PM);assert.equal(b.prime_minister.appointment.reason,'opening_reference');assert.equal(b.prime_minister.appointment.selected_on,null);}
function appointed(b){assert.equal(b.reformed,true);assert.deepEqual(b.people_representatives.map(e=>e.person.id).sort(),IDS.slice(0,2));assert.equal(b.prime_minister?.person.id,IDS[1]);assert.equal(b.prime_minister.appointment.reason,'assembly_recommendation_and_royal_appointment');assert.deepEqual(b.nonelected_ministers.map(e=>e.person.id),[IDS[2]]);}
function actionIndex(data,type,extra={}){const rows=data.actions.map((a,i)=>({a,i})).filter(({a})=>a.command?.kind==='tonga_institutions'&&a.command.action?.type===type&&Object.entries(extra).every(([k,v])=>a.command.action[k]===v));assert.equal(rows.length,1,'Exactly one authoritative action must match '+type);return rows[0].i;}
function archiveText(text){
  // storage::Campaign serializes saved_unix as its final top-level member.
  // Keep every other original byte, including native u64 values JS cannot round-trip.
  assert(text.startsWith('{"format":"spheres-campaign","version":1,'),'Expected the native compact campaign envelope');
  const suffix=/,"saved_unix":[0-9]+}$/;assert(suffix.test(text),'Native final top-level timestamp missing or envelope format changed');
  return text.replace(suffix,',"saved_unix":0}');
}
module.exports={KING,OPENING_PM,IDS,CASES,fixtureManifest,board,unappointed,appointed,actionIndex,archiveText};
