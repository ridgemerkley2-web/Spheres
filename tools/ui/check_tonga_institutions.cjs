// Tonga civilian offices use served identities and the existing government review channel.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const ui=require('../../spheres-web/ui/government-ui.js');
const copy=value=>JSON.parse(JSON.stringify(value));
const freeze=value=>{if(value&&typeof value==='object'){Object.freeze(value);Object.values(value).forEach(freeze);}return value;};
const source='https://parliament.gov.to/en/about-parliament/how-parliament-works';
function person(id,name,fiction=false){return {id,name,sources:[source],portrait:{url:'/art/people/'+id+'.png',credit:'Reviewed character artwork',from:fiction?'2026-09-08':'1990-01-01',to:fiction?'2036-01-01':'1991-01-01'},...(fiction?{fiction:{origin:'fictional_successor',fictional_biography:'An invented civilian character.',eligible_from:'2026-09-08'}}:{})};}
function fixture(){
 const king=person('taufaahau_tupou_iv',"Taufa'ahau Tupou IV"),pm=person('fatafehi_tuipelehake',"Prince Fatafehi Tu'ipelehake");
 const data={nation:'Tonga',nation_name:'Tonga',mine:true,on:true,electoral:false,leader:{name:king.name,office:'King',since:'1965-12-16'},actions:[],party_leadership:{nation:'Tonga',enabled:true,parties:[],executive_person:king}};
 const action=(type,label,extras={},refusal=null)=>{
  const view={command:{kind:'tonga_institutions',action:{type,...extras}},price_pc:5,refusal};
  data.actions.push({command:copy(view.command),price:5,refusal,label,category:'institutions',detail:'Requires a campaign decision.'});return view;
 };
 const people=[['fictional_to_lesieli_fotu','Lesieli Fotu','peoples_representative'],['fictional_to_sitani_lolohea','Sitani Lolohea','prime_minister'],['fictional_to_pisila_tukuafu','Pisila Tukuafu','nonelected_minister'],['fictional_to_kalolo_matalehu','Kalolo Matalehu','party_organizer']];
 const future=people.map(([id,name,role])=>({person:person(id,name,true),role,eligible_by_date:true,restriction:role==='party_organizer'?'Reference only: no party office, seat, or executive permission.':'Requires an institutional event.',actions:role==='prime_minister'?[action('recommend_prime_minister','Recommend Sitani Lolohea',{person_id:id},'The candidate must first hold an elected seat.')]:role==='nonelected_minister'?[action('appoint_minister','Appoint Pisila Tukuafu',{person_id:id})]:[]}));
 data.party_leadership.institutional_leadership={nation:'Tonga',date:'2030-01-01',version:1,active:true,reformed:false,monarch:{...data.leader,heir:{name:'Crown Prince Tupouto\u02bba',office:'Crown Prince'}},prime_minister:{person:pm,appointment:{reason:'opening_reference',selected_on:'1990-01-01'}},people_representatives:[],nonelected_ministers:[],peoples_seats:null,nobles_seats:null,unnamed_peoples_seats:null,proposed_model_seats:{peoples:17,nobles:9},nobles_eligibility:'Hereditary eligibility; no invented candidates.',recommendation_due:true,future_preview:future,
  historical_bindings:[{id:'historic-pm',role_id:'to_pm',person_id:pm.id,person:pm,portrait_reference_date:'1990-01-01',portrait_date_is_tenure:false,holder:{name:pm.name,attested_period:{from:'1990-01-01',through:'1990-12-31'},note:'Recorded at year precision.',uncertainty:'Exact retirement date unresolved.',sources:[source]}}],
  actions:{reform:action('reform','Adopt civilian Assembly reform'),hold_election:action('hold_election','Hold an institutional election',{},'First adopt Assembly reform.'),vacancies:[]},institution_sources:[source],gameplay_assumption:'Elections and appointments require an explicit campaign event.',notice:'The separate premier never replaces the monarch.',historical_reference_through:'2026-09-07',fictional_from:'2026-09-08'};
 return data;
}
const board=data=>data.party_leadership.institutional_leadership;
const show=(data=fixture(),state={})=>ui.render(data,{tab:'institutions',...state});
const reviewButtons=html=>[...html.matchAll(/<button\b[^>]*data-gov-review="\d+"[^>]*>[^]*?<\/button>/g)].map(match=>match[0]);
const card=(html,id)=>html.match(new RegExp('<article class="gov-ui-institution-person" data-gov-institution-person="'+id+'"[^]*?</article>'))?.[0]||'';

test('Tonga gets its own accessible institutional tab; other governments keep their existing tabs',()=>{
 const html=show();assert.match(html,/id="gov-tab-institutions" role="tab"[^>]*aria-selected="true"/);assert.match(html,/id="gov-panel-institutions" role="tabpanel" aria-labelledby="gov-tab-institutions"/);
 assert.equal((html.match(/role="tab"/g)||[]).length,6);
 const data=fixture();data.nation='UK';assert.doesNotMatch(show(data),/gov-tab-institutions|data-gov-institution-person=/);
});
test('monarch remains the hero while opening Prime Minister has a separate portrait and explicit reference label',()=>{
 const data=fixture(),html=show(data),hero=html.match(/<header class="gov-ui-hero[^]*?<\/header>/)[0];
 assert.match(hero,/Taufa&#39;ahau Tupou IV/);assert.doesNotMatch(hero,/Fatafehi/);
 assert.match(html,/alt="Cartoon avatar of Taufa&#39;ahau Tupou IV"/);assert.match(html,/alt="Cartoon avatar of Prince Fatafehi Tu&#39;ipelehake"/);
 assert.match(html,/Opening Prime Minister reference/);assert.match(html,/retained in your campaign until an explicit change/);
 const overview=ui.render(data,{tab:'overview'});assert.match(overview,/Separate civilian government/);assert.match(overview,/data-gov-tab="institutions"/);assert.match(overview,/Opening Prime Minister reference/);
});
test('active civilian appointments retain supplied dates, identities and vacant-office commands',()=>{
 const data=fixture(),b=board(data),pm=b.future_preview[1].person,minister=b.future_preview[2].person,rep=b.future_preview[0].person;
 b.prime_minister={person:pm,appointment:{reason:'assembly_recommendation',selected_on:'2030-02-03'}};
 b.people_representatives=[{person:rep,appointment:{reason:'election',selected_on:'2030-02-01'}}];b.nonelected_ministers=[{person:minister,appointment:{reason:'royal_appointment',selected_on:'2030-02-04'}}];
 const command={kind:'tonga_institutions',action:{type:'vacate',office:'prime_minister',person_id:pm.id}};
 b.actions.vacancies=[{command,price_pc:0,refusal:null}];data.actions.push({command:copy(command),price:0,label:'Vacate Sitani Lolohea as Prime Minister'});
 const html=show(data);assert.match(html,/Campaign appointment · 2030-02-03 · assembly recommendation/);assert.match(html,/2030-02-01/);assert.match(html,/2030-02-04/);assert.match(html,/Vacate Sitani Lolohea as Prime Minister/);
 assert.doesNotMatch(html.match(/<header class="gov-ui-hero[^]*?<\/header>/)[0],/Sitani Lolohea/);
});
test('inactive Crown institutions do not relabel a changed regime head as King',()=>{
 const data=fixture(),b=board(data);b.active=false;b.monarch={name:'Campaign president',office:'President'};data.leader=b.monarch;data.party_leadership.executive_person=person('campaign_president','Campaign president');
 const html=show(data);assert.match(html,/Crown institutions are inactive/);assert.match(html,/Current head of state/);assert.match(card(html,'campaign_president'),/<p class="gov-ui-kicker">President<\/p>/);
});

test('unadopted Assembly seats are a proposed model, not historical campaign counts',()=>{
 const data=fixture(),b=board(data);b.peoples_seats=null;b.nobles_seats=null;b.unnamed_peoples_seats=null;b.proposed_model_seats={peoples:17,nobles:9};
 const html=show(data);assert.match(html,/<dt>People’s seats<\/dt><dd>—<\/dd>/);assert.match(html,/Reform model: 17 people’s seats and 9 nobles’ seats/);
 b.reformed=true;b.peoples_seats=17;b.nobles_seats=9;b.unnamed_peoples_seats=15;
 const adopted=show(data);assert.match(adopted,/<dt>People’s seats<\/dt><dd>17<\/dd>/);assert.doesNotMatch(adopted,/Reform model:/);
});
test('all four fictional people use supplied portraits; Kalolo has no appointment controls',()=>{
 const data=fixture(),html=show(data);for(const entry of board(data).future_preview){assert.match(html,new RegExp('alt="Fictional cartoon avatar of '+entry.person.name+'"'));}
 assert.match(html,/4 of 4 fictional people shown/);assert.match(html,/Political organizer · reference only/);
 const kalolo=card(html,'fictional_to_kalolo_matalehu');assert.match(kalolo,/Reference only · no appointment action/);assert.doesNotMatch(kalolo,/data-gov-review=/);
 // Even an inconsistent candidate view cannot put an appointment control on this reference card.
 board(data).future_preview[3].actions=[copy(board(data).future_preview[2].actions[0])];assert.doesNotMatch(card(show(data),'fictional_to_kalolo_matalehu'),/data-gov-review=/);
});
test('date availability never promises appointment and backend refusals remain visible',()=>{
 const data=fixture();board(data).future_preview.forEach(entry=>entry.eligible_by_date=false);
 const html=show(data);assert.match(html,/Preview only at this date/);assert.match(html,/The candidate must first hold an elected seat\./);assert.match(html,/First adopt Assembly reform\./);
 const sitani=card(html,'fictional_to_sitani_lolohea');assert.match(sitani,/<button[^>]*data-gov-review="0"[^>]*disabled/);
 assert.doesNotMatch(html,/Guaranteed appointment|will win an election|automatically appointed/i);
});
test('foreign and pending views disable every institutional action',()=>{
 for(const state of [{busy:true},{blocked:true},{loading:true},{review:{loading:true}}]){
  const buttons=reviewButtons(show(fixture(),state));assert(buttons.length);assert(buttons.every(button=>/\bdisabled\b/.test(button)));
 }
 const data=fixture();data.mine=false;const html=show(data);assert.match(html,/Foreign government · view only/);assert(reviewButtons(html).every(button=>/\bdisabled\b/.test(button)));
});
test('unknown command or nested-target mismatch does not create an action button',()=>{
 const data=fixture();board(data).future_preview[2].actions[0].command.action.person_id='different_person';
 const pisila=card(show(data),'fictional_to_pisila_tukuafu');assert.doesNotMatch(pisila,/data-gov-review=/);assert.match(pisila,/not available in the current government reading/);
});
test('historical cards preserve uncertainty and appearance context without creating appointments',()=>{
 const html=show();assert.match(html,/Historical reference · no campaign appointment/);assert.match(html,/Attested within 1990-01-01 to 1990-12-31/);assert.match(html,/Exact retirement date unresolved/);assert.match(html,/appearance context does not establish an office tenure/);
 const historical=html.slice(html.indexOf('Sourced office references'));assert.doesNotMatch(historical,/data-gov-review=/);
});
test('search filters both casts while leaving serving officials visible and data unchanged',()=>{
 const data=freeze(fixture()),before=JSON.stringify(data),html=show(data,{query:'pisila'});assert.equal(JSON.stringify(data),before);assert.match(html,/1 of 4 fictional people shown/);assert.match(html,/Pisila Tukuafu/);assert.doesNotMatch(html,/Lesieli Fotu|Kalolo Matalehu/);assert.match(html,/Opening Prime Minister reference/);
});
test('institutional names, reasons, biographies and evidence are escaped; unsafe portraits and links are refused',()=>{
 const data=fixture(),attack='<img src=x onerror="evil">',b=board(data);
 b.future_preview[2].person.name=attack;b.future_preview[2].person.fiction.fictional_biography=attack;b.future_preview[2].restriction=attack;b.future_preview[2].person.portrait.url='javascript:evil()';
 b.actions.reform.refusal=attack;b.historical_bindings[0].holder.note=attack;b.institution_sources=['javascript:evil()',source];b.notice=attack;
 const html=show(data);assert.match(html,/&lt;img src=x onerror=&quot;evil&quot;&gt;/);assert.match(html,/Fictional avatar pending/);assert.doesNotMatch(html,/<img src=x|onerror="evil"|href="javascript:|src="javascript:/);
});
test('nested command comparison ignores key order but preserves action type and exact person',()=>{
 const command={kind:'tonga_institutions',action:{type:'recommend_prime_minister',person_id:'fictional_to_sitani_lolohea'}};
 assert.equal(ui.commandKey(command),ui.commandKey({action:{person_id:'fictional_to_sitani_lolohea',type:'recommend_prime_minister'},kind:'tonga_institutions'}));
 assert.notEqual(ui.commandKey(command),ui.commandKey({kind:'tonga_institutions',action:{type:'recommend_prime_minister',person_id:'fictional_to_kalolo_matalehu'}}));
});

const page=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8');
function hostFunction(name){const match=page.match(new RegExp('^(?:async )?function '+name+'\\([^]*?^\\}','m'));assert(match,name);return match[0];}
function hostFixture(){
 const data=fixture(),calls=[],world={session_id:'tonga-test',player:'Tonga'},index=1;
 const c=vm.createContext({window:{GovernmentUI:ui},gov:{open:true,nation:'Tonga',data,dataState:world,tab:'institutions',busy:false,review:null,queries:{}},S:world,SESSION:{busy:false},advancing:false,pendingAdvance:null,COMMAND_CHANNEL:{busy:false,pending:null},calls,adopted:[],
  renderGovernment(){},govFetch(){},$(){return {focus(){},scrollIntoView(){},textContent:''};},banner(){},clearTimeout(){},
  campaignConfirm(){throw Error('Reviewed actions must not open another confirmation dialog');},
  async api(url,body){calls.push({url,body:copy(body)});return url==='/api/government/preview'?{session_id:world.session_id,valid:true,review_token:'review-token',command:copy(body.command),price_pc:5,changes:[]}:{session_id:world.session_id};},
  async adopt(value){c.adopted.push(value);},
 });
 vm.runInContext(['governmentActionBlocked','govSelectTab','govReview','govAct'].map(hostFunction).join('\n'),c);return {c,calls,index,data};
}
test('institution control uses standard preview and confirmed command receipt with exact nested payload',async()=>{
 const f=hostFixture();await f.c.govReview(f.index);assert.equal(f.calls.length,1);assert.equal(f.calls[0].url,'/api/government/preview');assert.equal(f.c.gov.tab,'decisions');
 assert.equal(f.c.adopted.length,0);const review=f.c.gov.review;await f.c.govAct(f.data.actions[f.index],review);
 assert.equal(f.calls.length,2);assert.equal(f.calls[1].url,'/api/command');assert.deepEqual(f.calls[1].body,{commands:[f.data.actions[f.index].command],review_token:'review-token',review_kind:'government'});assert.equal(f.c.adopted.length,1);
});
test('host rejects a preview that changes only the nested person or action type',async()=>{
 for(const change of [command=>command.action.person_id='fictional_to_kalolo_matalehu',command=>command.action.type='vacate']){
  const f=hostFixture();f.c.api=async(url,body)=>{const command=copy(body.command);change(command);return {session_id:f.c.S.session_id,valid:true,review_token:'token',command,price_pc:5};};
  await f.c.govReview(f.index);assert.match(f.c.gov.review.error,/no longer matches/);assert.equal(f.c.gov.review.data,null);
 }
});
test('host accepts reordered nested command keys and never bypasses foreign, refused or pending guards',async()=>{
 const f=hostFixture();f.c.api=async(url,body)=>({session_id:f.c.S.session_id,valid:true,review_token:'token',price_pc:5,command:{action:{person_id:body.command.action.person_id,type:body.command.action.type},kind:body.command.kind}});
 await f.c.govReview(f.index);assert.equal(f.c.gov.review.error,'');assert(f.c.gov.review.data.valid);
 for(const change of [g=>g.data.mine=false,g=>g.c.COMMAND_CHANNEL.pending={},g=>g.data.actions[g.index].refusal='Cannot appoint']){
  const g=hostFixture();change(g);await g.c.govReview(g.index);assert.equal(g.calls.length,0);
 }
});
