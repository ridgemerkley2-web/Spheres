// Executes the shipped operations desk and request lifecycle, without a copied economy model.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const base=path.resolve(__dirname,'../..');
const source=fs.readFileSync(path.join(base,'spheres-web/ui/industry-ui.js'),'utf8');
const css=fs.readFileSync(path.join(base,'spheres-web/ui/industry-ui.css'),'utf8');
const economy=fs.readFileSync(path.join(base,'spheres-web/ui/province-economy-ui.js'),'utf8');
const money=economy.slice(economy.indexOf('function economyMoney('),economy.indexOf('\n}',economy.indexOf('function economyMoney('))+2);
const plain=value=>JSON.parse(JSON.stringify(value));
const decode=value=>value.replace(/&quot;/g,'"').replace(/&#39;/g,"'").replace(/&lt;/g,'<').replace(/&gt;/g,'>').replace(/&amp;/g,'&');
function control(extra={}) { return {disabled:false,listeners:[],addEventListener(type,callback){this.listeners.push(type);this['on'+type]=callback;},...extra}; }
function fixture(){
  const calls=[],requests=[],doc={activeElement:null},scroller={scrollTop:0};
  let html='';
  const mount={dataset:{industrySession:'one'},attributes:{},actions:[],filters:[],details:[],controls:{},
    setAttribute(key,value){this.attributes[key]=value;},
    get innerHTML(){return html;},
    set innerHTML(value){
      html=value;
      doc.activeElement=null;
      this.actions=[...value.matchAll(/<button\b([^>]*?)data-industry-action="([^"]+)"([^>]*)>/g)].map(m=>control({dataset:{industryAction:decode(m[2])},disabled:/\bdisabled\b/.test(m[3])}));
      this.filters=[...value.matchAll(/data-industry-filter="([^"]+)"/g)].map(m=>control({dataset:{industryFilter:m[1]},focus(){if(!this.disabled)doc.activeElement=this;}}));
      this.details=[...value.matchAll(/<details data-industry-detail="([^"]+)"([^>]*)>/g)].map(m=>({dataset:{industryDetail:decode(m[1])},open:/\bopen\b/.test(m[2])}));
      this.controls={};
      const input=/id="industrySearch"[^>]*value="([^"]*)"/.exec(value);
      if(input){const search=control({id:'industrySearch',value:decode(input[1]),selectionStart:0,selectionEnd:0,
        focus(){doc.activeElement=this;},setSelectionRange(a,b){this.selectionStart=a;this.selectionEnd=b;}});this.controls['#industrySearch']=search;}
      for(const key of ['clear','reset','refresh','retry']){
        const match=new RegExp(`data-industry-${key}([^>]*)>`).exec(value);
        if(match)this.controls['[data-industry-'+key+']']=control({dataset:{['industry'+key[0].toUpperCase()+key.slice(1)]:''},disabled:/\bdisabled\b/.test(match[1]),focus(){if(!this.disabled)doc.activeElement=this;}});
      }
    },
    querySelector(selector){const filter=/^\[data-industry-filter="([^"]+)"\]$/.exec(selector);return filter?this.filters.find(button=>button.dataset.industryFilter===filter[1])||null:this.controls[selector]||null;},
    querySelectorAll(selector){return selector==='[data-industry-action]'?this.actions:selector==='[data-industry-filter]'?this.filters:selector==='details[data-industry-detail]'?this.details:[];},
  };
  doc.querySelector=selector=>selector==='#industryRoot'?mount:selector==='#left'?scroller:null;
  const c=vm.createContext({console,window:{},document:doc,S:{session_id:'one',player:'USA'},CAB:{tab:'industry',busy:false},
    COMMAND_CHANNEL:{busy:false,pending:null},SESSION:{busy:false},PROD:{busy:false},COMP:{busy:false},advancing:false,pendingAdvance:null,
    cabinetIsOpen:()=>true,openIndustryCabinet(){calls.push('open-cabinet');c.CAB.tab='industry';},
    industryNavigate:action=>calls.push(plain(action)),api:async(...args)=>{requests.push(args);return reading();},
  });
  vm.runInContext(money+'\n'+source,c);
  c.desk=vm.runInContext('IDESK',c);Object.assign(c.desk,{open:true,session:'one',nation:'USA'});
  c.mount=mount;c.scroller=scroller;c.requests=requests;c.calls=calls;return c;
}
function productive(extra={}) {return {id:'site:US-CA:processing_plant',district:'US-CA',district_name:'California',kind:'processing_plant',name:'Materials Processing',effect:'Converts real raw inputs into intermediate packs.',level:1,capacity_micros:null,productive:true,status:'running',attention:false,reason:null,
  has_receipt:true,receipt_day:3,receipt_label:'1 Jan 1990',output_daily:2.5,output_unit:'intermediate packs',cash_spent_daily_bn:.00003,power_used_daily:2.5,
  actions:[{action:'province',label:'Inspect province',district:'US-CA'},{action:'budget',label:'Review operating funds',ministry:'industry',department:2},{action:'resources',label:'Inspect raw input supplies'},{action:'trade',good:'intermediates',label:'Inspect goods'},{action:'construction',district:'US-CA',label:'Review province construction'}],...extra};}
function supporting(extra={}) {return productive({id:'site:US-CA:power_grid',name:'Power Grid',kind:'power_grid',productive:false,status:'ready',effect:'Carries power within this province.',reason:'Carries power within this province.',has_receipt:false,receipt_day:null,receipt_label:null,output_daily:null,output_unit:null,cash_spent_daily_bn:null,power_used_daily:null,actions:[{action:'province',district:'US-CA',label:'Inspect province'}],...extra});}
function reading(extra={}){return {session_id:'one',nation:'USA',name:'United States',date:'2 Jan 1990',as_of_day:4,enabled:true,daily:true,settlement:{day:3,label:'1 Jan 1990'},summary:{facility_count:2,attention_count:0,queued_count:0},
  goods:[{good:'intermediates',name:'Intermediate packs',stock:12.75,capacity:250},{good:'capital_goods',name:'Capital-goods packs',stock:2,capacity:250}],power:{capacity_daily:10,used_daily:2.5},sites:[productive(),supporting()],queue:[],note:'Output and spending are dated receipts. Capacity is not output.',...extra};}
function loaded(c,data=reading()){Object.assign(c.desk,{data,state:c.S,stale:false,loading:false,error:''});return data;}

test('industry renders served inventories and modeled power separately from dated operations',()=>{
  const c=fixture(),data=loaded(c),before=plain(data),html=c.industryContentHtml();
  for(const text of ['From construction to production','Current date · 2 Jan 1990','Latest industry settlement · 1 Jan 1990','12.75 packs in stock','2 packs in stock','250 packs of storage for this good','10 units / day','2.5 intermediate packs','$30k','Recorded · 1 Jan 1990'])assert(html.includes(text),text);
  assert.doesNotMatch(html,/GDP bonus|Guaranteed|2\.5 packs in stock/);
  assert.deepEqual(plain(data),before);assert.equal(c.requests.length,0);
});

test('unknown operation receipts never become zero output or zero spending',()=>{
  const c=fixture();loaded(c);
  const site=productive({has_receipt:false,output_daily:null,cash_spent_daily_bn:null,power_used_daily:null,status:'awaiting_settlement'});
  let html=c.industrySiteHtml(site);assert.match(html,/Awaiting first operation/);assert.match(html,/spending will appear after work is recorded/);
  assert.doesNotMatch(html,/\$0|0 intermediate packs|Recorded operating spending|Recorded power use/);
  site.output_daily=999;site.cash_spent_daily_bn=500;assert.doesNotMatch(c.industrySiteHtml(site),/999|\$500/,'Receipt flag is authoritative');
  site.has_receipt=true;site.output_daily=0;site.cash_spent_daily_bn=0;site.power_used_daily=0;
  html=c.industrySiteHtml(site);assert.match(html,/0 intermediate packs/);assert.match(html,/\$0/);
});

test('support facilities explain their effect once and are never zero-output alarms',()=>{
  const c=fixture();loaded(c);const html=c.industrySiteHtml(supporting());
  assert.match(html,/Supporting/);assert.equal(html.split('Carries power within this province.').length-1,1);
  assert.doesNotMatch(html,/Awaiting first operation|Recorded output|0 packs|\$0|needs-attention/);
  assert.match(html,/Open province/);
});

test('research receipts keep acquisition-cost research units distinct from actual cash spending and packs',()=>{
  const c=fixture();loaded(c);const site=supporting({id:'research:US-CA',name:'Research Center',kind:'research_center',status:'running',has_receipt:true,receipt_label:'1 Jan 1990',cash_spent_daily_bn:.0001,
    research:{technology_name:'Industrial Robotics',prototype_credit:.002,goods_used:{intermediates:1,capital_goods:.2},day:3},actions:[{action:'research',label:'Inspect research projects'}]});
  const html=c.industrySiteHtml(site);assert.match(html,/Prototype credit/);assert.match(html,/0\.002 research units/);assert.match(html,/Recorded operating spending/);assert.match(html,/\$100k/);assert.match(html,/Industrial Robotics/);assert.match(html,/Review research/);
  assert.doesNotMatch(html,/\$2m|Recorded output|intermediate packs|Awaiting first operation/);assert.equal(c.industryProducing(site),true);
  site.research.prototype_credit=null;assert.match(c.industrySiteHtml(site),/Prototype credit<\/dt><dd>—<\/dd>/);
});

test('site search supports province names, codes and facility names with server attention semantics',()=>{
  const c=fixture();loaded(c,reading({sites:[productive({district_name:'São Paulo',district:'BR-SP'}),supporting(),productive({id:'blocked',name:'Machinery Works',status:'blocked',attention:true,output_daily:0}),productive({id:'awaiting',name:'Tiny Workshop',has_receipt:false,status:'paused',attention:false,output_daily:null})]}));
  c.desk.query='SAO materials';assert.equal(c.industryVisibleSites().length,1);
  c.desk.query='br-sp';assert.equal(c.industryVisibleSites()[0].district,'BR-SP');
  c.desk.query='';c.desk.filter='attention';assert.deepEqual(plain(c.industryVisibleSites()).map(s=>s.id),['blocked']);
  c.desk.filter='supporting';assert.equal(c.industryVisibleSites()[0].kind,'power_grid');
  c.desk.filter='producing';assert.equal(c.industryVisibleSites().length,1);
  c.desk.query='no matching name';assert.match(c.industryContentHtml(),/No facilities match these filters/);assert.match(c.industryContentHtml(),/Clear search and filters/);
});

test('empty completed industry bridges exact queued projects and inherited economic context',()=>{
  const c=fixture();loaded(c,reading({sites:[],queue:[{id:8,name:'Starter Industry',district:'US-CA',district_name:'California',progress:.125,eta_days:88},{id:'mine:US-NV:iron',name:'Iron Mine',district:'US-NV',district_name:'Nevada',progress:0,eta_days:null}]}));
  const html=c.industryContentHtml();
  for(const text of ['No completed facilities here yet','Your production base is taking shape','Starter Industry','12.5% complete','About 88 days remaining','Iron Mine','0% complete','Completion estimate unavailable','See the wider national economy'])assert(html.includes(text),text);
  assert.match(html,/&quot;project&quot;:8/);assert.match(html,/&quot;project&quot;:&quot;mine:US-NV:iron&quot;/);
  assert.doesNotMatch(html,/Recorded output|0 intermediate packs|facility filters/i);
});

test('construction bridge uses the same one-decimal progress and visible tiny-work rule as Construction',()=>{
  const c=fixture();
  for(const [fraction,text] of [[.06235,'6.2% complete'],[.00001,'<0.1% complete'],[0,'0% complete'],[1,'100% complete'],[null,'Progress unavailable']])assert.equal(c.industryProgressText(fraction),text);
  loaded(c,reading({sites:[],queue:[{id:8,name:'Workshop',district:'US-CA',progress:.00001}]}));
  assert.match(c.industryContentHtml(),/&lt;0\.1% complete/);assert.match(c.industryContentHtml(),/Completed mines remain in Resources/);
});

test('all user-visible strings and action attributes are escaped; unknown metrics stay unknown',()=>{
  const c=fixture();loaded(c,reading({name:'<img>',date:'<script>',sites:[productive({name:'<svg>',reason:'<iframe>',district_name:'" onclick="bad()',output_daily:null,cash_spent_daily_bn:null,actions:[{action:'province',district:'" onclick="bad()<img>',label:'<svg>'}]})],goods:[{name:'<script>',stock:null,capacity:null}],power:{capacity_daily:null,used_daily:null},note:'<iframe>'}));
  const html=c.industryContentHtml();assert.doesNotMatch(html,/<script|<svg|<iframe|<img/);assert.match(html,/&lt;iframe&gt;/);assert.match(html,/— packs in stock/);assert.doesNotMatch(html,/\$0/);
  assert.equal(c.industryButton({action:'start_project',label:'Order'}),'');
});

test('navigation sends only exact existing navigation targets, never commands',()=>{
  const c=fixture();loaded(c);
  const action={action:'budget',ministry:'industry',department:2,command:'start_project',amount:999,label:'Fund operations'};
  assert.equal(c.industryNavigateCurrent(action),true);assert.deepEqual(c.calls,[{action:'budget',ministry:'industry',department:2}]);
  assert.equal(c.industryNavigateCurrent({action:'start_project'}),false);assert.equal(c.industryNavigateCurrent({action:'resources',enabled:false}),false);assert.equal(c.requests.length,0);
});

test('stale, loading, changed-world and pending-order guards reject navigation',()=>{
  for(const change of [c=>c.desk.stale=true,c=>c.desk.loading=true,c=>c.desk.error='failed',c=>c.advancing=true,c=>c.pendingAdvance={},c=>c.COMMAND_CHANNEL.pending={},c=>c.COMMAND_CHANNEL.busy=true,c=>c.SESSION.busy=true,c=>c.PROD.busy=true,c=>c.CAB.busy=true,c=>c.CAB.tab='budget',c=>c.desk.open=false,c=>c.S={session_id:'one',player:'USA'}]){
    const c=fixture();loaded(c);change(c);assert.equal(c.industryNavigateCurrent({action:'resources'}),false);assert.equal(c.calls.length,0);
    assert.match(c.industryButton({action:'resources'},'Find raw inputs'),/disabled/);
  }
});

test('a bound action cannot operate a replacement payload even on the same campaign state',()=>{
  const c=fixture();loaded(c);c.industryRender();const button=c.mount.actions.find(b=>JSON.parse(b.dataset.industryAction).action==='province');
  c.desk.data=reading();assert.equal(button.onclick(),false);assert.equal(c.calls.length,0);
  c.industryRender();assert.equal(c.mount.actions.find(b=>JSON.parse(b.dataset.industryAction).action==='province').onclick(),true);assert.deepEqual(c.calls,[{action:'province',district:'US-CA'}]);
});

test('GET industry is session-bound, validates identity and binds accepted readings to the exact state',async()=>{
  const c=fixture();assert.equal(await c.industryFetch(),true);
  assert.deepEqual(c.requests,[['/api/industry?session_id=one']]);assert.equal(c.desk.state,c.S);assert.equal(c.desk.stale,false);assert.equal(c.industryCurrent(),true);
  for(const bad of [reading({session_id:'other'}),reading({nation:'Canada'}),reading({sites:null})]){
    c.api=async()=>bad;assert.equal(await c.industryFetch(true),false);assert.match(c.desk.error,/did not match this campaign/);assert.equal(c.industryCurrent(),false);
  }
});

test('late reads are rejected after state adoption, close, tab switch and newer request',async()=>{
  for(const change of [c=>c.S={session_id:'one',player:'USA'},c=>c.industryClose(),c=>c.CAB.tab='budget']){
    const c=fixture();let resolve;c.api=()=>new Promise(r=>resolve=r);const pending=c.industryFetch();change(c);resolve(reading());
    assert.equal(await pending,false);assert.equal(c.desk.data,null);
  }
  const c=fixture(),resolvers=[];c.api=()=>new Promise(r=>resolvers.push(r));
  const first=c.industryFetch();c.industryOnStateChanged();assert.equal(resolvers.length,2);
  resolvers[1](reading({note:'new'}));await new Promise(r=>setImmediate(r));
  resolvers[0](reading({note:'old'}));await first;assert.equal(c.desk.data.note,'new');
});

test('new campaign adoption immediately clears old nation data and view without waiting for a response',async()=>{
  const c=fixture();loaded(c);c.desk.query='California';c.desk.filter='attention';c.desk.details.add('old');c.scroller.scrollTop=900;c.industryRender();
  c.S={session_id:'two',player:'Canada'};let resolve;c.api=()=>new Promise(r=>resolve=r);c.industryOnStateChanged();
  assert.equal(c.desk.data,null);assert.equal(c.desk.session,'two');assert.equal(c.desk.nation,'Canada');assert.equal(c.desk.query,'');assert.equal(c.desk.filter,'all');assert.equal(c.desk.details.size,0);
  assert.doesNotMatch(c.mount.innerHTML,/California|United States/);assert.equal(c.desk.scroll,0);
  resolve(reading({session_id:'two',nation:'Canada',name:'Canada'}));await new Promise(r=>setImmediate(r));assert.equal(c.desk.data.name,'Canada');
});

test('refresh failure keeps prior dated reading disabled and exposes a real retry',async()=>{
  const c=fixture();loaded(c);c.api=async()=>{throw new Error('Connection lost');};
  assert.equal(await c.industryFetch(true),false);assert.match(c.mount.innerHTML,/Industry could not be refreshed/);assert.match(c.mount.innerHTML,/Connection lost/);assert.match(c.mount.innerHTML,/previous reading/);assert.match(c.mount.innerHTML,/Retry Industry/);assert.equal(c.industryCurrent(),false);
  c.api=async()=>reading();c.mount.querySelector('[data-industry-retry]').onclick();await new Promise(r=>setImmediate(r));
  assert.equal(c.desk.error,'');assert.equal(c.industryCurrent(),true);
});

test('search caret, scroll and expanded facility role survive repaint while background tabs cannot overwrite them',()=>{
  const c=fixture();loaded(c);c.industryRender();c.scroller.scrollTop=420;
  const search=c.mount.querySelector('#industrySearch');search.value='Materials';search.selectionStart=3;search.selectionEnd=5;search.focus();search.oninput();
  const next=c.mount.querySelector('#industrySearch');assert.equal(c.desk.query,'Materials');assert.equal(c.document.activeElement,next);assert.equal(next.selectionStart,3);assert.equal(next.selectionEnd,5);assert.equal(c.scroller.scrollTop,420);
  c.mount.details[0].open=true;c.mount.details[0].ontoggle();c.industryRender();assert.equal(c.mount.details[0].open,true);
  c.industryClose();c.CAB.tab='budget';c.scroller.scrollTop=20;c.industryPanelHtml();assert.equal(c.desk.scroll,420);
});

test('filter selection keeps keyboard focus on its replacement button after each repaint',()=>{
  const c=fixture();loaded(c);c.industryRender();c.scroller.scrollTop=360;
  for(const key of ['attention','supporting','all']){
    const button=c.mount.querySelector(`[data-industry-filter="${key}"]`);button.focus();button.onclick();
    const replacement=c.mount.querySelector(`[data-industry-filter="${key}"]`);
    assert.notEqual(replacement,button);assert.equal(c.document.activeElement,replacement);assert.equal(c.desk.filter,key);assert.equal(c.scroller.scrollTop,360);
  }
});

test('refresh focus returns after success or failure and repeated binding adds no duplicate handlers',async()=>{
  for(const failed of [false,true]){
    const c=fixture();loaded(c);c.industryRender();c.industryBind(false);c.industryBind(false);
    const button=c.mount.querySelector('[data-industry-refresh]'),clear=c.mount.querySelector('[data-industry-clear]');
    assert.equal(button.listeners.length,0);assert.equal(clear.listeners.length,0);
    let resolve,reject,reads=0;c.api=()=>{reads++;return new Promise((a,b)=>{resolve=a;reject=b;});};
    button.focus();button.onclick();assert.equal(reads,1);assert.equal(c.desk.loading,true);
    assert.equal(c.mount.querySelector('[data-industry-refresh]').disabled,true);assert.equal(c.document.activeElement,null);
    assert.equal(c.desk.focus.control,'refresh','Disabled refresh retains a focus target until settlement');
    if(failed)reject(new Error('Offline'));else resolve(reading());
    await new Promise(r=>setImmediate(r));
    const replacement=c.mount.querySelector('[data-industry-refresh]');assert.notEqual(replacement,button);
    assert.equal(replacement.disabled,false);assert.equal(c.document.activeElement,replacement);assert.equal(c.desk.focus,null);
  }
});

test('opening the page uses the root Cabinet entry, preserves same-campaign filters and makes one read',async()=>{
  const c=fixture();c.desk.open=false;c.CAB.tab='overview';c.desk.query='Grid';c.desk.filter='supporting';
  assert.equal(c.window.openIndustry(),true);await new Promise(r=>setImmediate(r));
  assert.equal(c.CAB.tab,'industry');assert.equal(c.desk.query,'Grid');assert.equal(c.desk.filter,'supporting');assert.equal(c.requests.length,1);
  c.industryBind();assert.equal(c.requests.length,1);
  c.industryClose();const count=c.requests.length;c.industryOnStateChanged();assert.equal(c.requests.length,count,'Closed state changes do not fetch');
});

test('the shipped standalone script parses and the desk styles are scoped and responsive',()=>{
  new vm.Script(source);assert.match(css,/\.industry-desk/);assert.match(css,/@media\(max-width:640px\)/);assert.match(css,/min-height:44px/);
  assert.doesNotMatch(source,/\/api\/command|start_project|start_industry_module|develop_resource/);
});
