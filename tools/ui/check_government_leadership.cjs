// Historical reference browsing never owns commands or campaign succession.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path');
const vm=require('node:vm');
const {render}=require('../../spheres-web/ui/government-ui.js');
const copy=value=>JSON.parse(JSON.stringify(value));
const freeze=value=>{if(value&&typeof value==='object'){Object.freeze(value);Object.values(value).forEach(freeze);}return value;};
const day=value=>({kind:'day',value}),open={kind:'open'},unknown={kind:'unknown'};
const source='https://members.parliament.uk/member/693/career';
const person=(id,name,extra={})=>({id,name,sources:[source],...extra});
const record=(p,extra={})=>({person:p,term:{id:p.id+'_term',person:p.id,role:'Party leader',kind:'leader',from:day('1983-10-02'),until:day('1992-07-18'),sources:[source],...extra}});
function fixture(){
 const kinnock=person('neil_kinnock','Neil Kinnock',{born:{kind:'year',value:'1942'},age_label:'47 years at this date',portrait:{url:'/art/party-leaders/neil-kinnock-1990-v1.webp',credit:'Original painted character artwork',method:'AI-assisted illustration',from:'1990',to:'1999',source_url:source}});
 const salmond=person('alex_salmond','Alex Salmond'),wigley=person('dafydd_wigley','Dafydd Wigley');
 const board={enabled:true,nation:'UK',date:'1990-02-11',campaign_date:'1990-02-11',reference_from:'1990-01-01',reference_through:'2026-09-07',parties:[
  {party_id:'uk_lab',party_name:'Labour Party',kind:'party',coverage:'partial',sources:[source],components:[],campaign:[{person:kinnock,role:'Campaign party leader',since_label:'11 February 1990',reason:'Selected when the campaign began',historical_reference_continues:true}],historical:[record(kinnock)],eligible:[record(kinnock)],gaps:[{from:unknown,until:unknown,reason:'The later transition date needs review.'}]},
  {party_id:'uk_nat',party_name:'Scottish National Party and Plaid Cymru',kind:'coalition',coverage:'partial',identity_note:'Two separate parties share one game row; there is no combined leader.',sources:[source],components:[{id:'snp',name:'Scottish National Party'},{id:'plaid_cymru',name:'Plaid Cymru'}],campaign:[{person:salmond,component:'snp',historical_reference_continues:false},{person:wigley,component:'plaid_cymru',historical_reference_continues:true}],historical:[record(salmond,{component:'snp'}),record(wigley,{component:'plaid_cymru',from:{kind:'year',value:'1991'},until:{kind:'month',value:'2000-08'}})],eligible:[record(salmond,{component:'snp'}),record(wigley,{component:'plaid_cymru'})],gaps:[]},
  {party_id:'uk_tiny',party_name:'Tiny coalition partner',kind:'party',coverage:'gap',sources:[],components:[],campaign:[],historical:[],eligible:[],support:0,seats:0,reason:'No researched leadership roster for this simulation party.',gaps:[{from:day('1990-01-01'),until:open,reason:'Leadership research is pending.'}]}
 ]};
 return {nation:'UK',nation_name:'United Kingdom',mine:true,on:true,electoral:true,leader:{name:'Campaign prime minister'},actions:[],party_leadership:board};
}
const show=(data=fixture(),state={})=>render(data,{tab:'leadership',...state});
const reference=(data,date='1990-01-01')=>({...copy(data.party_leadership),date,enabled:false});

test('leadership gets an accessible active tab when the API supplies its read model',()=>{
 const html=show();assert.match(html,/id="gov-tab-leadership" role="tab"[^>]*aria-selected="true" tabindex="0"/);assert.match(html,/id="gov-panel-leadership" role="tabpanel" aria-labelledby="gov-tab-leadership"/);
 assert.equal((html.match(/role="tab"/g)||[]).length,5);
});
test('older government payloads keep their existing four sections',()=>{
 const data=fixture();delete data.party_leadership;const html=render(data,{tab:'politics'});assert.equal((html.match(/role="tab"/g)||[]).length,4);assert.match(html,/data-gov-section="politics"/);
});
test('every party is represented, including a zero-share small partner and explicit gaps',()=>{
 const html=show();assert.equal((html.match(/data-gov-leadership-party=/g)||[]).length,3);assert.match(html,/Tiny coalition partner/);assert.match(html,/3 of 3 parties shown/);assert.match(html,/No researched leadership roster/);assert.match(html,/Research needed/);
});
test('combined game rows retain each component leader instead of inventing a joint person',()=>{
 const html=show();assert.match(html,/Two separate parties share one game row/);assert.match(html,/data-gov-person="alex_salmond"/);assert.match(html,/data-gov-person="dafydd_wigley"/);assert.match(html,/<p class="gov-ui-kicker">Scottish National Party<\/p>/);assert.match(html,/<p class="gov-ui-kicker">Plaid Cymru<\/p>/);
});
test('campaign holders preserve supplied identities, dates, roles and divergence',()=>{
 const html=show();assert.match(html,/Campaign party leader · Since 11 February 1990/);assert.match(html,/Selected when the campaign began/);assert.match(html,/Your campaign has continued beyond this historical term/);assert.match(html,/Historical reference at the campaign date/);
});
test('reference browsing displays reference identities without leaking current campaign holders',()=>{
 const data=fixture(),ref=reference(data);ref.parties[0].historical=[record(person('historical','Historical-only leader'))];ref.parties[0].eligible=[];ref.parties[0].campaign=[{person:person('current','DO NOT SHOW CAMPAIGN HOLDER')}];
 const html=show(data,{leadershipMode:'reference',leadershipDate:ref.date,leadershipReference:{data:ref}});
 assert.match(html,/Historical-only leader/);assert.doesNotMatch(html,/DO NOT SHOW CAMPAIGN HOLDER|Since 11 February 1990/);assert.match(html,/Browsing a date does not change your campaign/);
});
test('the year list covers 1990–2026 while exact dates stop at the verified cutoff',()=>{
 const html=show(fixture(),{leadershipMode:'reference',leadershipDate:'2026-09-07'});assert.equal((html.match(/<option value=/g)||[]).length,37);assert.match(html,/<option value="2026" selected>2026/);assert.match(html,/min="1990-01-01" max="2026-09-07" value="2026-09-07"/);assert.doesNotMatch(html,/2027/);
});
test('reference controls expose distinct host hooks and never an order or confirmation',()=>{
 const html=show(fixture(),{leadershipMode:'reference'});for(const hook of ['mode','year','date','search'])assert.ok(html.includes('data-gov-leadership-'+hook));assert.doesNotMatch(html,/data-gov-confirm=|data-gov-review=|data-command|onclick=|<form/);
});
test('loading reference hides a previous successful response',()=>{
 const data=fixture(),ref=reference(data);ref.parties[0].party_name='STALE REFERENCE';const html=show(data,{leadershipMode:'reference',leadershipReference:{loading:true,data:ref}});assert.match(html,/Loading the historical reference/);assert.doesNotMatch(html,/STALE REFERENCE|data-gov-leadership-party=/);
});
test('failed reference hides stale data and retries the reference only',()=>{
 const data=fixture(),ref=reference(data);ref.parties[0].party_name='STALE REFERENCE';const html=show(data,{leadershipMode:'reference',leadershipReference:{error:'Reference request failed',data:ref}});assert.match(html,/Reference request failed/);assert.match(html,/data-gov-leadership-retry/);assert.doesNotMatch(html,/STALE REFERENCE/);
});
test('stale nation and date responses are not presented as the selected reference',()=>{
 for(const altered of [{nation:'USA'},{date:'2001-01-01'}]){
  const data=fixture(),ref={...reference(data),...altered};ref.parties[0].party_name='MISMATCHED';const html=show(data,{leadershipMode:'reference',leadershipDate:'1990-01-01',leadershipReference:{data:ref}});assert.match(html,/Loading the historical reference/);assert.doesNotMatch(html,/MISMATCHED/);
 }
});
test('campaign mode ignores an unrelated pending or failed historical response',()=>{
 const html=show(fixture(),{leadershipMode:'campaign',leadershipReference:{loading:true,error:'HISTORICAL ERROR'}});assert.match(html,/Neil Kinnock/);assert.doesNotMatch(html,/HISTORICAL ERROR|Loading the historical reference/);
});
test('campaigns with the rule off label historical identities as references',()=>{
 const data=fixture();data.party_leadership.enabled=false;const html=show(data);assert.match(html,/Historical party succession is off/);assert.match(html,/not saved campaign appointments/);assert.doesNotMatch(html,/Since 11 February 1990/);
});
test('unknown dates remain unknown; month and year precision stay visible',()=>{
 const data=fixture();data.party_leadership.parties[0].eligible[0].term.until=unknown;const html=show(data);assert.match(html,/1991 · year recorded/);assert.match(html,/2000-08 · month recorded/);assert.match(html,/Date not established/);assert.doesNotMatch(html,/1991-01-01|2000-08-01/);
});
test('acting and co-leadership are preserved with separate person identities',()=>{
 const data=fixture(),p=data.party_leadership.parties[0];p.historical=[record(person('davey','Ed Davey'),{kind:'co_leader',role:'Acting co-leader'}),record(person('brinton','Sal Brinton'),{kind:'co_leader',role:'Acting co-leader'}),record(person('beckett','Margaret Beckett'),{kind:'acting'})];
 const html=show(data);assert.match(html,/Ed Davey/);assert.match(html,/Sal Brinton/);assert.match(html,/Acting co-leader/);assert.match(html,/Acting leader/);
});
test('candidate evidence is a disclosure, not a promised election outcome',()=>{
 const html=show();assert.match(html,/data-gov-detail="leader-candidates:uk_lab"/);assert.match(html,/does not schedule an election winner/);assert.match(html,/No verified eligible candidate is supplied/);assert.doesNotMatch(html,/Appoint Neil|Elect Neil|Hire leader/);
});
test('known people without art keep names and explicit avatar placeholders',()=>{
 const html=show();assert.match(html,/Alex Salmond/);assert.match(html,/Avatar not yet available/);assert.match(html,/class="gov-ui-person-art gov-ui-art-pending"/);
});
test('character artwork is uncropped, compact and explicitly labelled',()=>{
 const html=show();assert.match(html,/alt="Character avatar of Neil Kinnock"/);assert.match(html,/width="240" height="360"/);assert.match(html,/Appearance reference: 1990 to 1999/);
 const css=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/government-ui.css'),'utf8');assert.match(css,/\.gov-ui-person-art img\s*\{[^}]*object-fit:contain/);assert.match(css,/\.gov-ui-person-art\s*\{[^}]*min-width:90px/);
});
test('untrusted image URLs cannot become avatar requests',()=>{
 for(const url of ['javascript:alert(1)','https://example.com/face.png','//example.com/a.png','/art/../private/a.png','/art/a.svg','/art/a.png" onerror="evil']){
  const data=fixture();data.party_leadership.parties[0].campaign[0].person.portrait.url=url;const html=show(data);assert.doesNotMatch(html,/onerror=|src="https:|src="javascript:|src="\/\//);assert.match(html,/Avatar not yet available/);
 }
});
test('source disclosures keep HTTPS evidence and reject executable or local links',()=>{
 const data=fixture();data.party_leadership.parties[0].sources.push('javascript:alert(1)','file:///secret','http://insecure.example','https://example.org/evidence?x=1&y=2');const html=show(data);
 assert.match(html,/href="https:\/\/example.org\/evidence\?x=1&amp;y=2" target="_blank" rel="noopener noreferrer"/);assert.doesNotMatch(html,/href="javascript:|href="file:|href="http:/);assert.match(html,/opens in a new tab/);
});
test('search includes people, native names and small component parties',()=>{
 const data=fixture();data.party_leadership.parties[0].campaign[0].person.native='NATIVE NEEDLE';
 assert.match(show(data,{leadershipQuery:'native needle'}),/1 of 3 parties shown/);
 const component=show(data,{leadershipQuery:'plaid'});assert.match(component,/1 of 3 parties shown/);assert.match(component,/Dafydd Wigley/);assert.doesNotMatch(component,/data-gov-leadership-party="uk_lab"/);
});
test('search does not mutate dates, terms, candidates or caller state',()=>{
 const data=freeze(fixture()),state=freeze({tab:'leadership',leadershipQuery:'salmond'}),before=JSON.stringify(data);render(data,state);assert.equal(JSON.stringify(data),before);
});
test('person ages are served labels rather than a browser-derived number',()=>{
 const html=show();assert.match(html,/47 years at this date/);assert.match(html,/Born: 1942 · year recorded/);
 const data=fixture();delete data.party_leadership.parties[0].campaign[0].person.age_label;assert.doesNotMatch(show(data),/\d+ years at this date/);
});
test('foreign views retain the same read-only leadership browsing controls',()=>{
 const data=fixture();data.mine=false;const html=show(data,{leadershipMode:'reference',leadershipReference:{data:reference(data)}});assert.match(html,/data-gov-leadership-year/);assert.match(html,/Neil Kinnock/);assert.doesNotMatch(html,/data-gov-confirm=|data-gov-review=/);
});
test('missing leadership payload and backend errors remain explicit',()=>{
 assert.match(show({...fixture(),party_leadership:null}),/No party leadership reading was supplied/);
 const data=fixture();data.party_leadership={error:'Catalogue invalid'};assert.match(show(data),/Catalogue invalid/);
});
test('unknown person references do not fabricate a name, face or biography',()=>{
 const data=fixture();data.party_leadership.parties[0].campaign=[{person:null,person_id:'unknown'}];const html=show(data);assert.match(html,/Identity not established/);assert.match(html,/does not name a verified person/);assert.doesNotMatch(html,/Character avatar of unknown/);
});
test('party, person, component, image credits and search text are HTML escaped',()=>{
 const data=fixture(),attack='<img src=x onerror="evil">';data.party_leadership.parties[0].party_name=attack;data.party_leadership.parties[0].campaign[0].person.name=attack;data.party_leadership.parties[0].campaign[0].person.portrait.credit=attack;data.party_leadership.parties[1].components[0].name=attack;
 const html=show(data)+show(data,{leadershipQuery:attack});assert.match(html,/&lt;img src=x onerror=&quot;evil&quot;&gt;/);assert.doesNotMatch(html,/<img src=x|onerror="evil"/);
});
test('empty and filtered-out catalogues explain the distinction',()=>{
 const data=fixture();data.party_leadership.parties=[];assert.match(show(data),/No party records supplied/);assert.match(show(fixture(),{leadershipQuery:'absent'}),/No matching parties or people/);
});
test('avatar attribution distinguishes generated artwork from a licensed reference image',()=>{
 const data=fixture(),art=data.party_leadership.parties[0].campaign[0].person.portrait;
 Object.assign(art,{license:'generated',source_credit:'Photographer Example, 1990',source_license:'CC BY 2.0',source_license_url:'https://creativecommons.org/licenses/by/2.0/',era_note:'Cartoon interpretation of the sourced 1990 appearance.'});
 const html=show(data);assert.match(html,/Reference image: Photographer Example, 1990/);assert.match(html,/Reference image license: <a href="https:\/\/creativecommons.org\/licenses\/by\/2.0\/"/);assert.match(html,/Cartoon interpretation of the sourced 1990 appearance/);assert.doesNotMatch(html,/Artwork license: generated/);
});
test('a supplied artwork license gets its own safe link and unsafe license URLs stay plain text',()=>{
 const data=fixture(),art=data.party_leadership.parties[0].campaign[0].person.portrait;
 Object.assign(art,{license:'CC BY 4.0',license_url:'https://creativecommons.org/licenses/by/4.0/',source_license:'Reference terms',source_license_url:'javascript:alert(1)'});
 const html=show(data);assert.match(html,/Artwork license: <a href="https:\/\/creativecommons.org\/licenses\/by\/4.0\/"/);assert.match(html,/Reference image license: Reference terms/);assert.doesNotMatch(html,/href="javascript:/);
});
test('uncertain historical people are disclosed separately from verified holders and eligible candidates',()=>{
 const data=fixture(),p=data.party_leadership.parties[2];p.uncertain_historical=[{...record(person('uncertain','Possibly Serving Person'),{from:{kind:'month',value:'1990-02'},until:unknown}),reason:'The two sources disagree on the exact handover day.'}];
 const html=show(data);assert.match(html,/data-gov-detail="leader-uncertain:uk_tiny"/);assert.match(html,/Possibly Serving Person/);assert.match(html,/do not establish an officeholder or an eligible candidate/);assert.match(html,/The two sources disagree/);assert.match(html,/No campaign leader recorded/);assert.match(html,/No verified eligible candidate is supplied/);
});
test('a person known only through an uncertain date can still be found without making them eligible',()=>{
 const data=fixture();data.party_leadership.parties[2].uncertain_historical=[record(person('uncertain','Distinct Uncertain Name'))];
 const html=show(data,{leadershipQuery:'distinct uncertain'});assert.match(html,/1 of 3 parties shown/);assert.match(html,/Date uncertainty/);assert.match(html,/No verified eligible candidate is supplied/);
});
test('leadership browsing does not expose a retained decision confirmation from another section',()=>{
 const data=fixture();data.actions=[{label:'Earlier decision',price:8,command:{kind:'expel'},refusal:null}];
 const state={review:{index:0,data:{valid:true,title:'DO NOT DISPLAY OLD REVIEW',price_pc:8,command:{kind:'expel'}}}};
 for(const leadershipMode of ['campaign','reference']){
  const html=show(data,{...state,leadershipMode,leadershipReference:{data:reference(data)}});assert.doesNotMatch(html,/data-gov-confirm=|DO NOT DISPLAY OLD REVIEW|id="govReview"/);
 }
 assert.match(render(data,{...state,tab:'decisions'}),/data-gov-confirm="0"/);
});

const page=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8');
function hostFunction(name){
 const match=new RegExp('(?:async )?function '+name+'\\(').exec(page);assert(match,name);
 return page.slice(match.index,page.indexOf('\n}',match.index)+2);
}
function hostFixture(){
 const calls=[],pending=[];
 const c=vm.createContext({gov:{open:true,nation:'UK',tab:'leadership',seq:0,leadershipMode:'reference',leadershipDate:'1990-01-01',data:fixture(),expandedDetails:new Set()},S:{year:1990,month:2,day:11,nations:[]},renders:0,governmentReads:0,
  renderGovernment(){c.renders++;},govFetch(){c.governmentReads++;},api(url,body){calls.push({url,body});return new Promise((resolve,reject)=>pending.push({resolve,reject}));},
  $(){return {style:{},focus(){}};},document:{activeElement:null},encodeURIComponent});
 vm.runInContext(['govLeadershipBounds','govLeadershipDefaultDate','govLeadershipValidDate','govSetLeadershipMode','govSetLeadershipDate','govLeadershipFetch','govSelectTab','closeGovernment'].map(hostFunction).join('\n'),c);
 // govShow is one line, unlike the multi-line functions extracted above.
 vm.runInContext(page.match(/^function govShow\(id\).*$/m)[0],c);
 return {c,calls,pending,result:(date='1990-01-01',nation='UK')=>({nation,date,enabled:false,parties:[]})};
}
test('actual historical host uses a GET read without a command, session receipt or world adoption',async()=>{
 const f=hostFixture(),world=f.c.S,before=JSON.stringify(world),request=f.c.govLeadershipFetch();
 assert.equal(f.calls[0].url,'/api/party-leadership?nation=UK&date=1990-01-01');assert.equal(f.calls[0].body,undefined);
 f.pending[0].resolve(f.result());await request;assert.equal(f.c.S,world);assert.equal(JSON.stringify(world),before);assert.equal(f.c.gov.leadershipReference.loading,false);assert.equal(f.c.gov.leadershipReference.data.date,'1990-01-01');
});
test('out-of-order historical responses cannot overwrite a later selected date',async()=>{
 const f=hostFixture(),first=f.c.govLeadershipFetch();f.c.gov.leadershipDate='1991-01-01';const second=f.c.govLeadershipFetch();
 f.pending[1].resolve(f.result('1991-01-01'));await second;f.pending[0].resolve(f.result());await first;
 assert.equal(f.c.gov.leadershipReference.data.date,'1991-01-01');assert.equal(f.c.renders,3);
});
test('same-date retries use request identity so an older error cannot replace new data',async()=>{
 const f=hostFixture(),first=f.c.govLeadershipFetch(),second=f.c.govLeadershipFetch();f.pending[1].resolve(f.result());await second;f.pending[0].reject(Error('OLD ERROR'));await first;assert.equal(f.c.gov.leadershipReference.error,'');assert.equal(f.c.gov.leadershipReference.data.date,'1990-01-01');
});
test('closing Government invalidates an in-flight reference without a late redraw',async()=>{
 const f=hostFixture(),request=f.c.govLeadershipFetch();f.c.closeGovernment();const renders=f.c.renders;f.pending[0].resolve(f.result());await request;assert.equal(f.c.gov.leadershipReference,null);assert.equal(f.c.renders,renders);
});
test('switching nation clears the reference and its search before an old response returns',async()=>{
 const f=hostFixture(),request=f.c.govLeadershipFetch();f.c.gov.leadershipQuery='old query';f.c.govShow('USA');f.pending[0].resolve(f.result());await request;assert.equal(f.c.gov.nation,'USA');assert.equal(f.c.gov.leadershipReference,null);assert.equal(f.c.gov.leadershipQuery,'');assert.equal(f.c.gov.leadershipMode,'campaign');
});
test('leaving Leadership invalidates pending references without clearing other tab searches',async()=>{
 const f=hostFixture(),request=f.c.govLeadershipFetch();f.c.gov.queries={decisions:'budget'};f.c.govSelectTab('decisions');f.pending[0].resolve(f.result());await request;assert.equal(f.c.gov.leadershipReference,null);assert.equal(f.c.gov.query,'budget');
});
test('switching to campaign mode rejects the old reference result',async()=>{
 const f=hostFixture(),request=f.c.govLeadershipFetch();f.c.govSetLeadershipMode('campaign');f.pending[0].resolve(f.result());await request;assert.equal(f.c.gov.leadershipMode,'campaign');assert.equal(f.c.gov.leadershipReference.data,null);
});
test('reference host reports API errors and rejects wrong nation/date success payloads',async()=>{
 for(const result of [{error:'Date unavailable'},{nation:'USA',date:'1990-01-01'},{nation:'UK',date:'2000-01-01'}]){
  const f=hostFixture(),request=f.c.govLeadershipFetch();f.pending[0].resolve(result);await request;assert.equal(f.c.gov.leadershipReference.data,null);assert(f.c.gov.leadershipReference.error);assert.equal(f.c.gov.leadershipReference.loading,false);
 }
});
test('invalid and unresearched calendar dates never reach the API',()=>{
 for(const date of ['1990-02-30','1991-02-29','1989-12-31','2026-09-08','2026-12-31','junk']){
  const f=hostFixture();f.c.govSetLeadershipDate(date);assert.equal(f.calls.length,0,date);assert.match(f.c.gov.leadershipReference.error,/valid date/);
 }
 const f=hostFixture();assert.equal(f.c.govLeadershipValidDate('1992-02-29'),true);
});
test('opening historical mode defaults to the campaign date bounded by researched coverage',()=>{
 for(const [date,expected] of [['1989-12-01','1990-01-01'],['1996-04-02','1996-04-02'],['2035-01-01','2026-09-07']]){
  const f=hostFixture();f.c.gov.data.party_leadership.date=date;f.c.gov.data.party_leadership.campaign_date=date;f.c.gov.leadershipDate=null;f.c.govSetLeadershipMode('reference');assert.equal(f.c.gov.leadershipDate,expected);
 }
});
test('historical cache can be reused only for the same nation and date',()=>{
 const f=hostFixture();f.c.gov.leadershipReference={nation:'UK',date:'1990-01-01',data:f.result(),loading:false,error:''};f.c.govSetLeadershipMode('reference');assert.equal(f.calls.length,0);
 f.c.gov.leadershipReference.nation='USA';f.c.govSetLeadershipMode('reference');assert.equal(f.calls.length,1);
});
test('selector art credit no longer invents a verified portrait source',()=>{
 const code=hostFunction('loadFigurePortrait');assert.match(code,/identity_source_asset \|\| leaderArt.identity_source_wikidata \|\| "authored historical identity"/);assert.doesNotMatch(code,/"verified portrait"/);
});
test('index host preserves its CRLF line endings and every inline script parses',()=>{
 const bytes=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8');assert.equal((bytes.match(/\n/g)||[]).length,(bytes.match(/\r\n/g)||[]).length);
 let count=0;for(const match of bytes.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script\s*>/gi)){
  if(/\bsrc\s*=/.test(match[1])||/type\s*=\s*["']application\//i.test(match[1]))continue;
  new vm.Script(match[2],{filename:'index-inline-'+(++count)});
 }
 assert(count>0);
});
function hostDomFixture(){
 const f=hostFixture(),c=f.c;
 f.tabIds=['overview','decisions','politics','leadership','watch'];f.disclosureVisible=true;
 c.gov.leadershipMode='campaign';c.gov.expandedNation='UK';c.gov.expandedDetails=new Set();
 c.S.nations=[{id:'UK',name:'United Kingdom'},{id:'USA',name:'United States'}];
 const doc={activeElement:null,getElementById(id){return id==='govPick'?pick:f.tabs.find(tab=>tab.id===id)||null;}};
 function element(attrs={},dataset={}){
  return {id:attrs.id,tagName:attrs.tagName||'BUTTON',dataset,attrs,style:{},value:'',selectionStart:null,selectionEnd:null,
   hasAttribute(name){return Object.hasOwn(this.attrs,name);},getAttribute(name){return this.attrs[name]??null;},
   focus(){doc.activeElement=this;},scrollIntoView(){},closest(){return null;},querySelector(){return null;},
   setSelectionRange(start,end,direction){this.restoredRange=[start,end,direction];}};
 }
 function rebuild(){
  f.renderedDisclosureVisible=f.disclosureVisible;
  f.tabs=f.tabIds.map(id=>element({id:'gov-tab-'+id,role:'tab','data-gov-tab':id},{govTab:id}));
  f.search=element({tagName:'INPUT','data-gov-leadership-search':''});f.search.value=c.gov.leadershipQuery||'';
  f.detail=element({tagName:'DETAILS','data-gov-detail':'leader-test'},{govDetail:'leader-test'});f.detail.open=false;
  f.summary=element({tagName:'SUMMARY'});f.summary.closest=()=>f.detail;f.detail.querySelector=()=>f.summary;
 }
 const pick=element({id:'govPick',tagName:'SELECT'}),sub={innerHTML:'',contains:node=>node===pick};
 const body={scrollTop:123,contains:node=>f.tabs.includes(node)||node===f.search||node===f.detail||node===f.summary,
  querySelectorAll(selector){
   if(selector==='[data-gov-tab]'||selector==='[data-gov-tab][role=tab]')return f.tabs;
   if(selector==='[data-gov-leadership-search]')return [f.search];
   if(selector==='details[data-gov-detail]')return f.renderedDisclosureVisible?[f.detail]:[];
   return [];
  },set innerHTML(_html){rebuild();}};
 rebuild();c.document=doc;c.window={GovernmentUI:{render}};c.escText=String;c.governmentActionBlocked=()=>false;
 c.$=selector=>selector==='#govBody'?body:selector==='#govSub'?sub:selector==='#govPick'?pick:doc.getElementById(selector.slice(1));
 vm.runInContext(hostFunction('renderGovernment'),c);
 f.body=body;f.doc=doc;return f;
}
test('actual tab keyboard routing follows rendered tabs, including five-tab and legacy four-tab layouts',()=>{
 const f=hostDomFixture(),c=f.c;const key=(name)=>f.tabs.find(t=>t.dataset.govTab===c.gov.tab).onkeydown({key:name,preventDefault(){}});
 c.gov.tab='politics';c.renderGovernment();key('ArrowRight');assert.equal(c.gov.tab,'leadership');assert.equal(f.doc.activeElement.id,'gov-tab-leadership');
 key('ArrowRight');assert.equal(c.gov.tab,'watch');key('ArrowRight');assert.equal(c.gov.tab,'overview');key('End');assert.equal(c.gov.tab,'watch');key('Home');assert.equal(c.gov.tab,'overview');
 f.tabIds=['overview','decisions','politics','watch'];c.gov.tab='politics';c.renderGovernment();key('ArrowRight');assert.equal(c.gov.tab,'watch');key('ArrowRight');assert.equal(c.gov.tab,'overview');
 assert.equal(f.calls.length,0);
});
test('actual redraw preserves leadership search selection and filtered disclosures, scoped to the nation',()=>{
 const f=hostDomFixture(),c=f.c;c.gov.tab='leadership';c.renderGovernment();
 f.search.value='Dafydd Wigley';f.search.selectionStart=2;f.search.selectionEnd=8;f.search.selectionDirection='backward';f.doc.activeElement=f.search;f.detail.open=true;
 f.search.oninput();assert.equal(c.gov.leadershipQuery,'Dafydd Wigley');assert.equal(f.doc.activeElement,f.search);assert.deepEqual(f.search.restoredRange,[2,8,'backward']);assert.equal(f.detail.open,true);assert.equal(f.body.scrollTop,123);
 f.disclosureVisible=false;c.renderGovernment();f.disclosureVisible=true;c.renderGovernment();assert.equal(f.detail.open,true);
 c.gov.nation='USA';c.renderGovernment();assert.equal(f.detail.open,false);
});
