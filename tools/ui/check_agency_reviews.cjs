// Execute the shipped host against an explicit dialog/DOM surface, as the
// other campaign-host tests do. These are interaction tests, not browser proof.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const source=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/agency-ui.js'),'utf8');
const copy=x=>JSON.parse(JSON.stringify(x));
function fixture(){
  const all=node=>[node,...node.children.flatMap(all)];
  let document;
  class Element {
    constructor(tag='div'){this.tagName=tag.toUpperCase();this.children=[];this.attrs={};this.dataset={};this.listeners={};this.disabled=false;this.open=false;this._html='';this.textContent='';this.classList={toggle(){}};}
    setAttribute(key,value){this.attrs[key]=String(value);if(key==='id')this.id=String(value);if(key==='disabled')this.disabled=true;if(key.startsWith('data-'))this.dataset[key.slice(5).replace(/-([a-z])/g,(_,c)=>c.toUpperCase())]=String(value);}
    addEventListener(name,fn){(this.listeners[name]??=[]).push(fn);}
    emit(name){for(const fn of this.listeners[name]||[])fn({target:this,stopPropagation(){},preventDefault(){}});}
    appendChild(child){this.children.push(child);return child;}
    prepend(child){this.children.unshift(child);}
    focus(){document.activeElement=this;}
    scrollIntoView(options){this.lastScroll=options;}
    showModal(){this.open=true;}
    close(){this.open=false;this.emit('close');}
    cancel(){this.emit('cancel');this.close();}
    getBoundingClientRect(){return {left:0,top:0,right:800,bottom:600};}
    set innerHTML(value){
      this._html=value;this.children=[];
      for(const m of value.matchAll(/<(button|select|form|h3|p|div|section|article)\b([^>]*)>/g)){
        const child=new Element(m[1]);
        for(const a of m[2].matchAll(/([\w-]+)(?:="([^"]*)")?/g))child.setAttribute(a[1],a[2]||'');
        this.children.push(child);
      }
    }
    get innerHTML(){return this._html;}
    querySelectorAll(selector){return all(this).slice(1).filter(node=>selector.split(',').some(s=>{
      s=s.trim();if(s[0]==='#')return node.id===s.slice(1);
      if(s[0]==='['){const m=s.match(/^\[([^=\]]+)(?:="([^"]*)")?\]$/);return m&&m[1] in node.attrs&&(m[2]===undefined||node.attrs[m[1]]===m[2]);}
      return node.tagName.toLowerCase()===s;
    }));}
    querySelector(selector){return this.querySelectorAll(selector)[0]||null;}
  }
  document={body:new Element('body'),activeElement:null,createElement:tag=>new Element(tag),getElementById:id=>all(document.body).find(n=>n.id===id)||null};
  for(const id of ['agencyBtn','agencyCount']){const e=new Element('button');e.setAttribute('id',id);document.body.appendChild(e);}
  const state={session_id:'campaign-one',player:'France',date:'1 Jan 1990',agency:{offers:[{id:4,from_name:'Japan',expires:'1990-01-15',days_remaining:14,title:'Trade treaty',consequence:'A native proposal.'}],expiry_rule:'Reply by the recorded date.',policy:{defense_pacts:'review',trade_treaties:'review',calls_to_arms:'review'},monetary:{kind:'pegged',rate:.04},break_peg_pc:15,automatic_bank:true,history:[]}};
  const requests=[],adoptions=[];
  const c=vm.createContext({document,S:state,clockPause(){},AreaArt:{html:()=>'<img src="/art/diplomacy.png" alt="">'},FormData:class{constructor(form){return Object.entries(form.values);}}});
  c.window=c;c.api=async(route,payload)=>{requests.push({route,payload:copy(payload)});throw Error('Unexpected API call: '+route);};
  c.adopt=async value=>{adoptions.push(copy(value));c.S=value;c.renderAgency(value);};
  vm.runInContext(source,c);
  return {c,document,requests,adoptions,state,review:()=>vm.runInContext('AGENCY.review',c),busy:()=>vm.runInContext('AGENCY.busy',c),node:id=>document.getElementById(id),html:()=>all(document.body).map(n=>n._html+' '+n.textContent).join('\n'),
    api(fn){c.api=async(route,payload)=>{requests.push({route,payload:payload===undefined?null:copy(payload)});return fn(route,payload);};}};
}
const command={kind:'respond_diplomacy',offer:4,accept:true};
function quote(cmd=command,extra={}){return {session_id:'campaign-one',nation:'France',valid:true,reason:null,title:'Accept Japan’s proposal',date_label:'1 Jan 1990',description:'A reviewed diplomatic commitment.',changes:[{label:'Political capital',before:'40.0',after:'30.0',detail:'Native immediate cost.'}],warnings:['Future requests retain their own deadlines.'],command:copy(cmd),review_token:'native-token',...extra};}
const deferred=()=>{let resolve,reject;const promise=new Promise((a,b)=>{resolve=a;reject=b;});return {promise,resolve,reject};};

function commitments(){return {
  note:'Saved commitments only.',upkeep:{next_step_label:'$8.200m per day at today’s conditions',annual_label:'$3.000bn',note:'Native estimate; no payment.'},
  defense_pacts:[{partner:'Kuwait',partner_name:'<Kuwait>',since:'1990-01',status_label:'Saved defense pact',upkeep:{next_step_label:'$4.100m per day at today’s conditions'},warnings:['<Native warning>'],pending_offer_ids:[4]}],
  trade_agreements:[{partner:'Japan',partner_name:'Japan',status_label:'Saved trade agreement',depth:.425,dependency:0,partner_dependency:.0196,warnings:[]}],
  conflicts:[{conflict_id:7,theatre:'<Gulf>',started:'1989-12',side_label:'Defending coalition',rung:2,rung_name:'sanctions',shooting_label:'Below the shooting threshold',allies:[{name:'France',alive:true},{name:'<Kuwait>',alive:true}],opponents:[{name:'Iraq',alive:true}]}]
};}
test('saved commitments render native facts without mutations, invented dates or actions',()=>{
  const f=fixture();f.state.agency.commitments=commitments();const before=copy(f.state);f.c.openAgency();f.c.renderAgency(f.state);
  const html=f.node('agencyBody').innerHTML;
  for(const text of ['1990-01','$4.100m','42.5%','0.0%','2.0%','1989-12','Defending coalition','2 · sanctions','Below the shooting threshold']) assert(html.includes(text),text);
  assert.match(html,/&lt;Kuwait&gt;/);assert.match(html,/&lt;Gulf&gt;/);assert.match(html,/&lt;Native warning&gt;/);assert.doesNotMatch(html,/<Kuwait>|<Gulf>|<Native warning>/);
  assert(html.indexOf('id="agencyInbox"')<html.indexOf('id="agencyCommitments"'));
  assert(html.indexOf('id="agencyCommitments"')<html.indexOf('Standing diplomatic policy'));
  assert.deepEqual(f.state,before);assert.equal(f.requests.length,0);
});
test('unavailable commitment data is distinct from an empty set or zero cost',()=>{
  const f=fixture();f.c.openAgency();assert.match(f.html(),/Commitment details are not available/);assert.doesNotMatch(f.html(),/No saved defense pacts/);
  f.state.agency.commitments={defense_pacts:[],trade_agreements:[],conflicts:[],upkeep:{next_step_label:'$0 per day at today’s conditions',annual_label:'$0'}};f.c.renderAgency(f.state);
  assert.match(f.html(),/No saved defense pacts/);assert.match(f.html(),/No saved trade agreements/);assert.match(f.html(),/\$0 per day/);
});
test('View request focuses the exact inbox entry without choosing an answer or fetching a preview',()=>{
  const f=fixture();f.state.agency.commitments=commitments();f.c.openAgency();
  const button=f.node('agencyBody').querySelector('[data-agency-commitment-reply]');button.onclick();
  const offer=f.node('agencyBody').querySelector('[data-agency-offer="4"]');
  assert.equal(f.document.activeElement,offer);assert.equal(offer.attrs.tabindex,'-1');assert.equal(offer.lastScroll.block,'nearest');
  assert.equal(f.review(),null);assert.equal(f.requests.length,0);
  f.state.agency.offers=[];f.document.activeElement=null;button.onclick();assert.equal(f.document.activeElement,null);
});
test('Inspect conflict uses the current wars view and only navigates to a saved conflict',()=>{
  const f=fixture(),opened=[];f.c.openConflict=id=>opened.push(id);f.state.agency.commitments=commitments();f.state.wars=[{id:7}];f.c.openAgency();
  const button=f.node('agencyBody').querySelector('[data-agency-open-conflict]');button.onclick();assert.deepEqual(opened,[7]);assert.equal(f.node('agencyPanel').open,false);assert.equal(f.requests.length,0);
  f.c.openAgency();f.state.wars=[];f.node('agencyBody').querySelector('[data-agency-open-conflict]').onclick();assert.deepEqual(opened,[7]);assert.equal(f.node('agencyPanel').open,true);
});
test('call review displays matched native coalitions and escapes every context field',async()=>{
  const f=fixture(),context={requester_name:'<Kuwait>',guaranteed:true,requested_rung:2,requested_rung_name:'sanctions',status_label:'Review the native effects before answering.',note:'<Native posture note>',conflict:{theatre:'<Gulf>',started:'1990-01',defenders:[{name:'<Kuwait>',alive:true}],opponents:[{name:'Iraq',alive:true}]}};
  f.api(()=>quote(command,{call_context:context}));await f.c.agencyReview(command);
  const html=f.node('agencyReview').innerHTML;
  assert.match(html,/What you are being asked to join/);assert.match(html,/Defense pact request/);assert.match(html,/2 · sanctions/);assert.match(html,/Defending coalition/);assert.match(html,/Opposing coalition/);
  assert.match(html,/&lt;Kuwait&gt;/);assert.match(html,/&lt;Native posture note&gt;/);assert.doesNotMatch(html,/<Gulf>|<Kuwait>/);assert.equal(f.requests.length,1);
});
test('an unmatched call never borrows the coalitions of a reused current conflict ID',async()=>{
  const f=fixture();f.state.wars=[{id:7,theatre:'Wrong theatre',side_b:['Wrong defender']}];
  f.api(()=>quote(command,{valid:false,review_token:null,changes:[],call_context:{conflict_id:7,requester_name:'Kuwait',requested_rung:2,requested_rung_name:'sanctions',status_label:'The original conflict has ended or changed identity.',blocked:'Native refusal.',conflict:null,note:'Native context only.'}}));
  await f.c.agencyReview(command);assert.match(f.html(),/original conflict has ended/);assert.match(f.html(),/Native refusal/);assert.doesNotMatch(f.html(),/Wrong theatre|Wrong defender/);
  assert.equal(f.node('agencyReview').querySelector('[data-agency-confirm]').disabled,true);await f.c.agencyConfirm(f.review());assert.equal(f.requests.length,1);
});

test('inbox opens a dated native review and cancellation sends no command',async()=>{
  const f=fixture();f.api(async(route,payload)=>{assert.equal(route,'/api/decisions/preview');assert.deepEqual(copy(payload),{session_id:'campaign-one',nation:'France',command});return quote();});
  f.c.openAgency();await f.node('agencyBody').querySelector('[data-agency-accept]').onclick();
  assert.match(f.html(),/1 Jan 1990/);assert.match(f.html(),/Native immediate cost/);assert.match(f.html(),/art\/diplomacy\.png/);
  const panel=f.node('agencyReview');assert(panel);assert.equal(panel.querySelector('[data-agency-confirm]').disabled,false);
  panel.querySelector('[data-agency-review-cancel]').onclick();assert.equal(f.review(),null);assert.equal(f.requests.length,1);assert.equal(f.adoptions.length,0);
  assert.match(f.node('agencyStatus').textContent,/No decision sent/);
});

test('one confirmation posts the exact nested command and native token once',async()=>{
  const f=fixture(),pending=deferred(),cmd={kind:'set_diplomatic_policy',policy:{calls_to_arms:'decline',trade_treaties:'accept',defense_pacts:'review'}};
  f.api((route,payload)=>route.endsWith('/preview')?quote(cmd):pending.promise);
  await f.c.agencyReview(cmd);const review=f.review();cmd.policy.trade_treaties='decline';
  const first=f.c.agencyConfirm(review);await f.c.agencyConfirm(review);
  assert.equal(f.busy(),true);assert.equal(f.requests.filter(x=>x.route==='/api/command').length,1);
  assert.deepEqual(f.requests[1].payload,{commands:[{kind:'set_diplomatic_policy',policy:{calls_to_arms:'decline',trade_treaties:'accept',defense_pacts:'review'}}],review_token:'native-token',review_kind:'decisions'});
  pending.resolve({...f.state,date:'1 Jan 1990',errors:[]});await first;
  assert.equal(f.busy(),false);assert.equal(f.review(),null);assert.equal(f.adoptions.length,1);assert.match(f.node('agencyStatus').textContent,/Decision recorded/);
  assert.equal(f.document.activeElement,f.node('agencyStatus'));assert.equal(f.node('agencyStatus').lastScroll.block,'nearest');
});

test('late success and failure cannot revive a cancelled or reopened dialog',async()=>{
  for(const how of ['cancel','close','failure']){
    const f=fixture(),pending=deferred();f.api(()=>pending.promise);
    const request=f.c.agencyReview(command);assert(f.review().loading);
    if(how==='cancel')f.node('agencyPanel').cancel();else f.node('agencyPanel').close();
    f.c.openAgency();if(how==='failure')pending.reject(Error('Old failure'));else pending.resolve(quote());await request;
    assert.equal(f.review(),null);assert(!f.node('agencyReview'));assert.equal(f.adoptions.length,0);assert.doesNotMatch(f.html(),/Old failure|Confirm decision/);
  }
});

test('a newer review wins and stale world or session replacement cannot confirm',async()=>{
  const f=fixture(),old=deferred(),second={kind:'break_currency_peg'};
  f.api((route,payload)=>payload.command.kind===command.kind?old.promise:quote(second));
  const first=f.c.agencyReview(command);await f.c.agencyReview(second);const current=f.review();old.resolve(quote());await first;
  assert.equal(f.review(),current);assert.deepEqual(copy(current.command),second);
  f.c.S={...f.state,date:'2 Jan 1990'};f.c.renderAgency(f.c.S);await f.c.agencyConfirm(current);
  assert.equal(f.review(),null);assert.equal(f.requests.length,2);assert.match(f.node('agencyStatus').textContent,/campaign changed/);
  await f.c.agencyReview(second);const another=f.review();f.c.S={...f.c.S,session_id:'new-session'};await f.c.agencyConfirm(another);assert.equal(f.requests.length,3);
});

test('mismatched command, nation, session or missing token never enables confirmation',async()=>{
  for(const extra of [{command:{...command,accept:false}},{nation:'Japan'},{session_id:'old-session'},{review_token:''}]){
    const f=fixture();f.api(()=>quote(command,extra));await f.c.agencyReview(command);
    assert(f.review().error);assert.equal(f.node('agencyReview').querySelector('[data-agency-confirm]').disabled,true);
    await f.c.agencyConfirm(f.review());assert.equal(f.requests.length,1);
  }
});

test('refused or locally altered reviewed commands do not spend and display native reasons',async()=>{
  const f=fixture();f.api(()=>quote(command,{valid:false,reason:'Reply deadline expired.',review_token:null}));await f.c.agencyReview(command);
  assert.match(f.html(),/Reply deadline expired/);await f.c.agencyConfirm(f.review());assert.equal(f.requests.length,1);
  f.api(()=>quote());await f.c.agencyReview(command);f.review().command.accept=false;
  await f.c.agencyConfirm(f.review());assert.equal(f.requests.length,2);
});

test('server stale-review refusal adopts current state and demands a fresh review',async()=>{
  const f=fixture();f.api((route)=>route.endsWith('/preview')?quote():route==='/api/command'?({...f.state,errors:['The campaign changed; review again.'],command_pending:false}):({...f.state,date:'2 Jan 1990'}));
  await f.c.agencyReview(command);await f.c.agencyConfirm(f.review());
  assert.equal(f.adoptions.length,1);assert.equal(f.c.S.date,'2 Jan 1990');assert.equal(f.review(),null);
  assert.match(f.node('agencyStatus').textContent,/review again/);assert.equal(f.requests.filter(x=>x.route==='/api/command').length,1);
  assert.deepEqual(f.requests.map(x=>x.route),['/api/decisions/preview','/api/command','/api/state']);
});

test('command errors refresh by reading state without retrying the order',async()=>{
  const f=fixture();f.api((route)=>{if(route.endsWith('/preview'))return quote();if(route==='/api/command')throw Error('Receipt unavailable');return {...f.state,date:'2 Jan 1990'};});
  await f.c.agencyReview(command);await f.c.agencyConfirm(f.review());
  assert.deepEqual(f.requests.map(x=>x.route),['/api/decisions/preview','/api/command','/api/state']);
  assert.equal(f.adoptions.length,1);assert.equal(f.review(),null);assert.equal(f.busy(),false);assert.match(f.node('agencyStatus').textContent,/Receipt unavailable/);
});

test('an uncertain command envelope leaves the shared receipt available for recovery',async()=>{
  const f=fixture(),receipt={request_seq:9};f.c.COMMAND_CHANNEL={busy:false,pending:null};
  f.api(route=>{
    if(route.endsWith('/preview'))return quote();
    if(route==='/api/command'){f.c.COMMAND_CHANNEL.pending=receipt;return {...f.state,errors:['Recover the pending receipt.'],command_pending:true};}
    return {...f.state,date:'2 Jan 1990'};
  });
  await f.c.agencyReview(command);await f.c.agencyConfirm(f.review());
  assert.equal(f.c.COMMAND_CHANNEL.pending,receipt);assert.equal(f.review(),null);
  await f.c.agencyReview(command);assert.equal(f.requests.filter(x=>x.route==='/api/command').length,1);
  assert.equal(f.requests.length,3);assert.match(f.node('agencyStatus').textContent,/Recover the pending receipt/);
});

test('a command response from an older closed dialog cannot replace a newer world',async()=>{
  const f=fixture(),pending=deferred();f.api(route=>route.endsWith('/preview')?quote():pending.promise);
  await f.c.agencyReview(command);const sending=f.c.agencyConfirm(f.review());f.node('agencyPanel').close();
  const newer={...f.state,date:'3 Jan 1990'};f.c.S=newer;f.c.renderAgency(newer);f.c.openAgency();
  pending.resolve({...f.state,date:'2 Jan 1990',errors:[]});await sending;
  assert.equal(f.c.S,newer);assert.equal(f.adoptions.length,0);assert.equal(f.review(),null);assert.equal(f.busy(),false);
});

test('nested key order is immaterial, unsupported commands and unsafe labels stay inert',async()=>{
  const f=fixture(),cmd={kind:'set_diplomatic_policy',policy:{calls_to_arms:'review',trade_treaties:'review',defense_pacts:'review'}};
  f.api(()=>quote({policy:{defense_pacts:'review',trade_treaties:'review',calls_to_arms:'review'},kind:cmd.kind},{title:'<script>bad()</script>',changes:[{label:'<img>',before:'<svg>',after:'safe',detail:'<iframe>'}]}));
  await f.c.agencyReview({kind:'war',target:'Japan'});assert.equal(f.requests.length,0);
  await f.c.agencyReview(cmd);assert.equal(f.node('agencyReview').querySelector('[data-agency-confirm]').disabled,false);
  const html=f.node('agencyReview').innerHTML;assert.match(html,/&lt;script&gt;/);assert.doesNotMatch(html,/<script>|<svg>|<iframe>/);
});

test('standing policy, currency and aim controls all review before any order',async()=>{
  const f=fixture(),bodies=[];f.api((route,payload)=>{assert.equal(route,'/api/decisions/preview');bodies.push(copy(payload.command));return quote(payload.command);});
  f.c.openAgency();const form=f.node('agencyPolicy');
  form.values={defense_pacts:'decline',trade_treaties:'accept',calls_to_arms:'review'};
  form.onsubmit({target:form,preventDefault(){}});await new Promise(resolve=>setImmediate(resolve));
  assert.deepEqual(bodies[0],{kind:'set_diplomatic_policy',policy:form.values});
  await f.node('agencyBreakPeg').onclick();assert.equal(bodies[1].kind,'break_currency_peg');
  const changed={...f.state,campaign_aims:{note:'Native aims.',active:null,history:[],offers:[{aim:'prosperity',title:'Prosperity',description:'A native target.'}]}};
  f.c.S=changed;f.c.renderAgency(changed);await f.node('agencyBody').querySelector('[data-campaign-aim]').onclick();
  assert.deepEqual(bodies[2],{kind:'choose_campaign_aim',aim:'prosperity'});
  assert.equal(f.requests.filter(x=>x.route==='/api/command').length,0);
});

test('existing pending receipts block new or already-reviewed orders',async()=>{
  const f=fixture();f.api(()=>quote());await f.c.agencyReview(command);const held=f.review();
  f.c.COMMAND_CHANNEL={busy:false,pending:{request_seq:1}};f.c.renderAgency(f.c.S);
  assert.equal(f.node('agencyReview').querySelector('[data-agency-confirm]').disabled,true);
  await f.c.agencyConfirm(held);await f.c.agencyReview(command);
  assert.equal(f.requests.length,1);assert.equal(f.adoptions.length,0);
});

test('dated native history names remain escaped and original request identity is retained',()=>{
  const f=fixture();f.state.agency.history=[{offer:{id:4},outcome:'Accepted',date_label:'2 Jan 1990',from_name:'<Japan>',title:'<Treaty>'}];
  f.c.openAgency();const html=f.node('agencyBody').innerHTML;
  assert.match(html,/<time>2 Jan 1990<\/time>/);assert.match(html,/&lt;Japan&gt; · &lt;Treaty&gt; · Request #4: Accepted/);assert.doesNotMatch(html,/<Japan>|<Treaty>/);
});

test('pending requests precede optional aims and sort by the native deadline without mutating the inbox',()=>{
  const f=fixture(),offer=f.state.agency.offers[0];
  f.state.agency.offers=[{...offer,id:8,days_remaining:14},{...offer,id:6,days_remaining:3,expires:'1990-01-04'},{...offer,id:5,days_remaining:3,expires:'1990-01-04'}];
  f.state.campaign_aims={note:'Optional aims.',active:null,history:[],offers:[]};
  const before=copy(f.state);f.c.openAgency();f.c.renderAgency(f.state);
  assert.deepEqual(f.state,before);
  const body=f.node('agencyBody'),ids=body.querySelectorAll('[data-agency-offer]').map(n=>n.dataset.agencyOffer);
  assert.deepEqual(ids,['5','6','8']);assert.equal(body.children[0].id,'agencyInbox');
  assert.equal(body.children.at(-1).className,'agency-aims');
  assert.match(body.innerHTML,/Nearest deadline: <strong>1990-01-04<\/strong>/);
  assert.match(body.innerHTML,/Review acceptance/);assert.match(body.innerHTML,/Review decline/);
  assert.match(body.innerHTML,/Opening a review sends no decision/);assert.equal(f.requests.length,0);
});

test('reply dates are exclusive, the final day is singular, and closed windows never say today',()=>{
  const f=fixture(),offer=f.state.agency.offers[0];
  f.state.agency.expiry_rule='Unanswered requests decline on the recorded deadline.';
  f.state.agency.offers=[{...offer,id:4,days_remaining:1},{...offer,id:5,days_remaining:0}];f.c.openAgency();
  const html=f.node('agencyBody').innerHTML;
  assert.match(html,/Reply before <time>1990-01-15<\/time>/);assert.match(html,/1 day remaining/);
  assert.match(html,/Reply window closed/);assert.doesNotMatch(html,/Reply by|0 days remaining|1 days remaining|Reply today/);
});

test('overdue requests retain exact deadline order when native days remaining is clamped to zero',()=>{
  const f=fixture(),offer=f.state.agency.offers[0];
  f.state.agency.offers=[{...offer,id:1,days_remaining:0,expires:'1990-01-19'},{...offer,id:2,days_remaining:0,expires:'1990-01-15'}];
  const before=copy(f.state);f.c.openAgency();
  assert.deepEqual(f.node('agencyBody').querySelectorAll('[data-agency-offer]').map(n=>n.dataset.agencyOffer),['2','1']);
  assert.match(f.node('agencyBody').innerHTML,/Nearest deadline: <strong>1990-01-15<\/strong> · Reply window closed/);
  assert.deepEqual(f.state,before);
});

test('blocked acceptance retains a decline review and both request and quoted identities remain escaped',async()=>{
  const f=fixture();Object.assign(f.state.agency.offers[0],{from_name:'<Japan>',title:'<Trade treaty>',accept_blocked:'<Native refusal>',consequence:'<Native consequence>'});
  const decline={...command,accept:false};
  f.api((route,payload)=>{assert.equal(route,'/api/decisions/preview');assert.deepEqual(copy(payload.command),decline);return quote(decline,{title:'Decline <Japan> · <Trade treaty>',description:'Reply before <1990-01-15>.'});});
  f.c.openAgency();const body=f.node('agencyBody');
  assert.equal(body.querySelector('[data-agency-accept]').disabled,true);assert.equal(body.querySelector('[data-agency-decline]').disabled,false);
  assert.match(body.innerHTML,/&lt;Japan&gt; · &lt;Trade treaty&gt;/);assert.match(body.innerHTML,/&lt;Native refusal&gt;/);
  await body.querySelector('[data-agency-decline]').onclick();
  assert.match(f.node('agencyReview').innerHTML,/Decline &lt;Japan&gt; · &lt;Trade treaty&gt;/);
  assert.match(f.node('agencyReview').innerHTML,/Reply before &lt;1990-01-15&gt;/);
  assert.doesNotMatch(f.html(),/<Japan>|<Trade treaty>|<Native refusal>/);
  assert.equal(f.requests.filter(r=>r.route==='/api/command').length,0);
});
