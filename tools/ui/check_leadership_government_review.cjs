const {test}=require('node:test'),assert=require('node:assert/strict');
const {presentation,assetPath}=require('./leadership-government-review.js');
const {render}=require('../../spheres-web/ui/government-ui.js');
const clone=value=>JSON.parse(JSON.stringify(value));
function fixture(){
 const real={id:'historical',name:'Historical person'},fiction={id:'fictional',name:'Invented person',fiction:{origin:'fictional_successor'}};
 const party={party_id:'example',party_name:'Example party',coverage:'partial',campaign:[{person:real}],historical:[{person:real,term:{role:'Party chair'}}],future_preview:[{person:fiction,eligible:false}],future_candidates:[{person:fiction,eligible:true}]};
 const board={nation:'Japan',enabled:true,date:'1990-01-01',campaign_date:'1990-01-01',reference_from:'1990-01-01',reference_through:'2026-09-07',executive_person:real,parties:[party],future_policy:{from:'2026-09-08'}};
 return {nation:'Japan',reference_1990:board,future_reference_2030:{...clone(board),date:'2030-01-01'}};
}
test('historical review removes campaign and future records without mutating native snapshot',()=>{
 const native=fixture(),before=clone(native),{data,state}=presentation(native,'history');
 assert.equal(state.leadershipMode,'reference');assert.equal(data.party_leadership.enabled,false);
 assert.equal(data.party_leadership.executive_person,null);const p=data.party_leadership.parties[0];
 assert.deepEqual([p.campaign,p.future_candidates,p.future_preview],[[],[],[]]);
 assert.equal(p.historical[0].person.name,'Historical person');assert.deepEqual(native,before);
});
test('future review keeps fictional source data but cannot masquerade as a campaign appointment',()=>{
 const {data,state}=presentation(fixture(),'future');assert.equal(data.party_leadership.enabled,false);
 assert.equal(data.party_leadership.date,'2030-01-01');assert.equal(data.party_leadership.campaign_date,'1990-01-01');
 assert.deepEqual(data.party_leadership.parties[0].campaign,[]);
 const html=render(data,state);assert.match(html,/Fictional successor/);assert.match(html,/Future cast preview/);
 assert.doesNotMatch(html,/Eligible for future campaign succession/);
});
test('generated campaign view remains a labeled sample with original bindings',()=>{
 const {data,state}=presentation(fixture(),'campaign','Example');
 assert.equal(data.party_leadership.enabled,true);assert.equal(state.leadershipQuery,'Example');
 assert.match(data.briefing.date_label,/Sample starting campaign/);assert.equal(data.mine,false);assert.deepEqual(data.actions,[]);
 assert.equal(data.party_leadership.parties[0].campaign[0].person.id,'historical');
 assert.throws(()=>presentation(fixture(),'unrecognized'));
});
test('static art route mapper is confined to explicit repo directories',()=>{
 assert.equal(assetPath('/art/people/example-1990.png'),'../../spheres-web/ui/person-portraits/example-1990.png');
 assert.equal(assetPath('/art/government/council-v1.png'),'../../spheres-web/ui/government-art/council-v1.png');
 for(const bad of ['/art/people/../secret.png','/art/people/%2e%2e/secret.png','/art/people/a/b.png','https://example.com/a.png','javascript:bad()','/art/people/a.png?x=1']) assert.equal(assetPath(bad),null);
});
