// Run the shipped equipment room against a small DOM and authoritative server fixtures.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const base=path.resolve(__dirname,'../..');
const source=fs.readFileSync(path.join(base,'spheres-web/ui/equipment-ui.js'),'utf8');
const css=fs.readFileSync(path.join(base,'spheres-web/ui/equipment-ui.css'),'utf8');
const page=fs.readFileSync(path.join(base,'spheres-web/ui/index.html'),'utf8').replace(/\r\n/g,'\n');
const economy=fs.readFileSync(path.join(base,'spheres-web/ui/province-economy-ui.js'),'utf8');
const money=economy.slice(economy.indexOf('function economyMoney('),economy.indexOf('\n}',economy.indexOf('function economyMoney('))+2);
const plain=value=>JSON.parse(JSON.stringify(value));
const tick=()=>new Promise(resolve=>setImmediate(resolve));
const decode=value=>String(value).replace(/&quot;/g,'"').replace(/&#39;/g,"'").replace(/&lt;/g,'<').replace(/&gt;/g,'>').replace(/&amp;/g,'&');
const camel=value=>value.replace(/-([a-z])/g,(_,letter)=>letter.toUpperCase());
function spec(){return {platform:'tank_medium',components:{mobility:'diesel',protection:'steel',armament:'gun',sensors:'optical',communications:'radio'}};}
function action(extra={}){return {label:'Plan production',command:{kind:'equipment_produce',revision:7,district:'US-CA',quantity:1,daily_budget_mn:2},requires_preview:true,
  inputs:[{key:'quantity',label:'Vehicles',type:'number',value:1,min:1,max:20,step:1},{key:'district',label:'Production site',type:'select',value:'US-CA',options:[{value:'US-CA',label:'California'},{value:'US-TX',label:'Texas'}]},
    {key:'daily_budget_mn',label:'Daily limit',type:'number',unit:'$m/day',value:2,min:0,step:.1}],...extra};}
function snapshot(extra={}){
  const parts=[['mobility','diesel','Diesel powerpack'],['protection','steel','Baseline protection'],['armament','gun','Main armament'],['sensors','optical','Day optics'],['communications','radio','Tactical radio']].map(([slot,id,name])=>({id,name,slot,known:true,description:`${name} for the tank family.`}));
  parts.push({id:'thermal',name:'Thermal observation',slot:'sensors',known:false,description:'Observation in poor visibility.',reason:'Research vehicle thermal observation.',tech:{id:'land_thermal',name:'Vehicle thermal observation',domain:'Aerospace'},tradeoffs:['More support demand','Better observation']});
  const slots=['mobility','protection','armament','sensors','communications'].map(id=>({id,name:id[0].toUpperCase()+id.slice(1),required:true,components:parts.filter(row=>row.slot===id).map(row=>row.id)}));
  return {session_id:'one',nation:'USA',name:'United States',date:'11 Feb 1990',enabled:true,reason:null,platforms:[{id:'tank_medium',name:'Medium tank platform',description:'A general-purpose inherited chassis.',slots}],components:parts,
    presets:[{id:'affordable',name:'Affordable',description:'A straightforward fit.',...spec()},{id:'balanced',name:'Balanced',description:'A shared configuration.',default:true,...spec()}],
    designs:[{id:7,name:'Sentinel 90',status:'certified',spec:spec(),detail:'The approved configuration.',metrics:[{label:'Observation',value:1.4,unit:'rating'}],costs:[{label:'Unit cost',amount_bn:.004,period:'Per complete vehicle'}],actions:[action()]}],
    development:[{id:9,name:'Sentinel 90B',status:'Developing',detail:'Testing a named revision.',progress:.06235,metrics:[{label:'Remaining work',value:6,unit:'days',period:'At current funding'}],costs:[{label:'Last paid work',amount_bn:.00000075,period:'Settled 10 Feb 1990'}],receipt_label:'10 Feb 1990',actions:[{label:'Pause development',command:{kind:'equipment_pause',project:9,paused:true},enabled:true}]}],
    production:[],lots:[{id:5,name:'Sentinel 90',status:'In service',detail:'Delivered, available stock.',metrics:[{label:'Available vehicles',value:3,unit:'vehicles'},{label:'Platform age',value:12,unit:'months'}],actions:[action({label:'Review refit',command:{kind:'equipment_refit',lot:5,target:8,quantity:1},inputs:[{key:'quantity',label:'Vehicles to refit',type:'number',value:1,min:1,max:3}]})]}],
    research:[{id:'thermal',name:'Vehicle thermal observation',status:'Research available',detail:'A component unlock, not instant fleet strength.',metrics:[{label:'Research remaining',value:4,unit:'points'}],actions:[{label:'Review research',navigate:{action:'research',id:'land_thermal',domain:'Aerospace'}}]}],
    funding:{metrics:[{label:'Development authorization',value:'$4m',period:'Currently available'},{label:'Procurement authorization',value:'$20m',period:'Currently available'}]},actions:[],note:'Quotes depend on current funding and supply.',...extra};
}
function preview(extra={}){return {session_id:'one',nation:'USA',valid:true,detail:'A compatible tank configuration.',blockers:[],metrics:[{label:'Observation',value:1.3,unit:'rating'},{label:'Mobility',value:1.1,unit:'rating'}],
  costs:[{label:'Development',amount_bn:.03,period:'One-time work'},{label:'Unit cost',amount_bn:.004,period:'Per complete vehicle'},{label:'Maintenance',amount_bn:.00001,period:'Per vehicle per year'}],timing:[{label:'Minimum development',value:'30 days'}],requirements:['Subject to the chosen production site.'],
  actions:[{label:'Save draft',command:{kind:'equipment_save',name:'Balanced',...spec()}},{label:'Fund development',command:{kind:'equipment_develop',name:'Balanced',...spec(),daily_budget_mn:1},requires_preview:true,inputs:[{key:'daily_budget_mn',label:'Daily development limit',type:'number',unit:'$m/day',value:1,min:0}]}],...extra};}
function quote(extra={}){return preview({detail:'Three complete vehicles at the selected site.',costs:[{label:'Batch cost',amount_bn:.017,period:'One production batch'},{label:'Daily limit',amount_bn:.003,period:'Maximum per day'}],metrics:[{label:'Vehicles requested',value:3,unit:'complete vehicles'}],timing:[{label:'First delivery',value:'45 days if funded and supplied'}],actions:[{label:'Confirm production',command:{kind:'equipment_produce',revision:7,district:'US-TX',quantity:3,daily_budget_mn:3,quote_token:'server-quote'},requires_preview:false}],...extra});}
function fixture(){
  const calls=[],requests=[],modelMounts=[],doc={activeElement:null},scroller={scrollTop:0};let html='';
  function parse(tag,attrs,body=''){
    const attributes={},dataset={};for(const m of attrs.matchAll(/([\w-]+)="([^"]*)"/g)){attributes[m[1]]=decode(m[2]);if(m[1].startsWith('data-'))dataset[camel(m[1].slice(5))]=decode(m[2]);}
    for(const m of attrs.matchAll(/\b(data-[\w-]+)(?=\s|$)/g)){attributes[m[1]]='';dataset[camel(m[1].slice(5))]='';}
    const node={tagName:tag.toUpperCase(),attributes,dataset,id:attributes.id,disabled:/\bdisabled(?:\s|$)/.test(attrs),open:/\bopen(?:\s|$)/.test(attrs),value:attributes.value??'',selectionStart:0,selectionEnd:0,listeners:[],
      focus(){if(!this.disabled)doc.activeElement=this;},setSelectionRange(a,b){this.selectionStart=a;this.selectionEnd=b;},addEventListener(type,fn){this.listeners.push(type);this['on'+type]=fn;},closest(){return this.parentDetail||null;},scrollIntoView(options){this.lastScroll=options;}};
    if(tag==='select'){const options=[...body.matchAll(/<option([^>]*)>/g)];const selected=options.find(m=>/\bselected(?:\s|$)/.test(m[1]))||options[0];if(selected)node.value=decode(/value="([^"]*)"/.exec(selected[1])?.[1]||'');}
    return node;
  }
  const mount={dataset:{equipmentSession:'one'},attributes:{},parentElement:scroller,nodes:[],listeners:[],addEventListener(type,fn){this.listeners.push(type);this['on'+type]=fn;},
    setAttribute(key,value){this.attributes[key]=value;},get innerHTML(){return html;},set innerHTML(value){html=value;doc.activeElement=null;this.nodes=[];
      for(const m of value.matchAll(/<(button|select|h1|h2|summary|details)\b([^>]*)>([\s\S]*?)<\/\1>/g)){
        const parent=parse(m[1],m[2],m[3]);this.nodes.push(parent);if(m[1]==='details'){for(const child of m[3].matchAll(/<(button|select|summary)\b([^>]*)>([\s\S]*?)<\/\1>/g)){const node=parse(child[1],child[2],child[3]);node.parentDetail=parent;this.nodes.push(node);}}
      }
      for(const m of value.matchAll(/<input\b([^>]*)>/g))this.nodes.push(parse('input',m[1]));
      for(const m of value.matchAll(/<article\b([^>]*)>/g))this.nodes.push(parse('article',m[1]));
      if(value.includes('data-equipment-model')){
        const children=this.nodes.filter(node=>Object.keys(node.dataset).some(key=>key.startsWith('model'))),status={textContent:'Preparing the 3D model…'};
        const host={tagName:'DIV',attributes:{'data-equipment-model':''},dataset:{equipmentModel:''},children,status,
          remove(){mount.nodes=mount.nodes.filter(node=>node!==this&&!this.children.includes(node));},
          replaceWith(next){this.remove();mount.nodes.push(next,...next.children);},
          querySelector(selector){return selector==='[data-model-status]'?this.status:null;},querySelectorAll(selector){return selector==='button'?this.children:[];}};
        this.nodes.push(host);
      }
    },querySelectorAll(selector){return this.nodes.filter(node=>{if(selector.startsWith('#'))return node.id===selector.slice(1);const m=/^(?:([\w]+))?\[([\w-]+)(?:="([^"]*)")?\]$/.exec(selector);return m&&(!m[1]||node.tagName===m[1].toUpperCase())&&Object.hasOwn(node.attributes,m[2])&&(m[3]===undefined||node.attributes[m[2]]===m[3]);});},querySelector(selector){return this.querySelectorAll(selector)[0]||null;},
  };
  doc.querySelector=selector=>selector==='#equipmentRoot'?mount:mount.querySelector(selector);
  const c=vm.createContext({console,window:{EquipmentModel:{mount(host,spec){const controller={host,initial:plain(spec),updates:[],resizes:0,disposals:0,selectedParts:[],selectPart(slot){this.selectedParts.push(slot);},update(next){this.updates.push(plain(next));},resize(){++this.resizes;},dispose(){++this.disposals;}};modelMounts.push(controller);return controller;}}},document:doc,S:{session_id:'one',player:'USA'},CAB:{tab:'overview',busy:false},PROD:{busy:false},COMP:{busy:false},SESSION:{busy:false},COMMAND_CHANNEL:{busy:false,pending:null},advancing:false,pendingAdvance:null,
    equipmentIsOpen:()=>true,equipmentScroller:()=>scroller,openEquipmentDrawer:()=>calls.push('open'),closeEquipmentDrawer:()=>{calls.push('close');c.equipmentClose();},
    equipmentNavigate:action=>calls.push(plain(action)),equipmentCommand:async command=>{calls.push(plain(command));return {message:'Accepted'};},
    api:async(...args)=>{requests.push(plain(args));return args[0].startsWith('/api/equipment?')?snapshot():args[1]?.command?quote():preview();},
  });
  vm.runInContext(money+'\n'+source,c);c.eq=vm.runInContext('EQUIP',c);Object.assign(c.eq,{open:true,session:'one',nation:'USA'});Object.assign(c,{calls,requests,modelMounts,mount,scroller});return c;
}
function loaded(c,data=snapshot()){Object.assign(c.eq,{data,state:c.S,loading:false,stale:false,error:''});c.equipmentDefaultDraft();Object.assign(c.eq,{preview:preview(),previewState:c.S,previewKey:c.equipmentDraftKey(),previewLoading:false,previewError:''});return data;}

test('designer starts from the server balanced preset and renders supplied ratings and costs without recalculating',()=>{
  const c=fixture(),data=loaded(c),before=plain(data),html=c.equipmentContentHtml();assert.equal(c.eq.preset,'balanced');assert.deepEqual(plain(c.eq.draft.components),spec().components);
  for(const text of ['From concept to service','Research','Designer','Library','Development','Production','In service','Individual specifications','Interactive ground vehicle model','Observation','1.3','One-time work','$30m','$4m','30 days'])assert(html.includes(text),text);
  assert.deepEqual(plain(data),before);assert.equal(c.requests.length,0);assert.equal(c.calls.length,0);
});

test('starting from a legacy model uses its supplied separate specifications and preserves the original',()=>{
  const c=fixture();loaded(c);const original={id:'old-1',name:'Legacy',spec:spec(),editable_spec:{...spec(),components:{...spec().components,tracks:'wide',turret:'large',ammunition:'mixed'}}};
  const before=plain(original);c.equipmentSetDraft(original);assert.deepEqual(plain(c.eq.draft.components),original.editable_spec.components);assert.deepEqual(plain(original),before);
  assert.match(c.eq.message,/original revision remains unchanged/);assert.equal(c.calls.length,0);
});

test('independent new specifications are submitted separately and grouped for review',async()=>{
  const c=fixture(),data=snapshot();for(const [id,group] of [['tracks','tracks'],['suspension','suspension'],['turret','turret'],['ammunition','ammunition'],['fire_control','fire_control']]){
    data.platforms[0].slots.push({id,name:id,required:true,components:[id+'_base',id+'_upgrade']});
    for(const suffix of ['base','upgrade'])data.components.push({id:id+'_'+suffix,name:id+' '+suffix,family:group,known:true});
    data.presets[1].components[id]=id+'_base';
  }
  loaded(c,data);c.equipmentRender();const select=c.mount.querySelector('[data-equipment-slot="ammunition"]');select.value='ammunition_upgrade';select.onchange();await tick();
  const sent=c.requests.at(-1)[1];assert.equal(sent.components.ammunition,'ammunition_upgrade');assert.equal(sent.components.turret,'turret_base');assert.equal(sent.components.tracks,'tracks_base');
  assert(c.mount.querySelector('[data-equipment-detail="spec:weapon"]'));assert(c.mount.querySelector('[data-equipment-detail="spec:drive"]'));assert.equal(c.calls.length,0);
});

test('3D preview retains one live host and camera across pricing responses, typing, and component changes',async()=>{
  const c=fixture();loaded(c);c.equipmentRender();const model=c.modelMounts[0],host=model.host;
  assert.deepEqual(model.initial,{...spec(),name:'Balanced'});assert.equal(model.updates.length,0);
  c.equipmentRender();c.equipmentRender();assert.equal(c.modelMounts.length,1);assert.equal(c.mount.querySelector('[data-equipment-model]'),host);
  const name=c.mount.querySelector('#equipmentName');name.value='Sentinel 91';name.oninput();await tick();
  assert.equal(c.modelMounts.length,1);assert.equal(model.updates.at(-1).name,'Sentinel 91');assert.equal(c.mount.querySelector('[data-equipment-model]'),host);
  c.eq.draft.components.sensors='thermal';c.equipmentDraftChanged();await tick();
  assert.equal(c.modelMounts.length,1);assert.equal(model.updates.at(-1).components.sensors,'thermal');assert.equal(model.disposals,0);
  assert.equal(c.calls.length,0);assert(c.requests.every(row=>row[0]==='/api/equipment-preview'));
});

test('model controls retain focus across a repaint and expose keyboard, camera, finish, and download actions',()=>{
  const c=fixture();loaded(c);c.equipmentRender();const host=c.modelMounts[0].host,control=c.mount.querySelector('[data-model-view="side"]');control.focus();
  c.equipmentRender();assert.equal(c.document.activeElement,control);assert.equal(c.mount.querySelector('[data-equipment-model]'),host);
  for(const key of ['hero','front','side','rear','top'])assert(c.mount.querySelector(`[data-model-view="${key}"]`));
  for(const key of ['left','right','up','down'])assert(c.mount.querySelector(`[data-model-rotate="${key}"]`));
  for(const key of ['olive','sand','winter'])assert(c.mount.querySelector(`[data-model-finish="${key}"]`));
  for(const key of ['model-reset','model-turntable','model-export'])assert(c.mount.querySelector(`[data-${key}]`));
  assert.match(c.mount.innerHTML,/Paint changes appearance only/);assert.doesNotMatch(c.mount.innerHTML,/Illustrative silhouette|Stylized tank/);
});

test('leaving the designer and closing release the model exactly once while returning remounts it',()=>{
  const c=fixture();loaded(c);c.equipmentRender();const first=c.modelMounts[0];
  c.equipmentSelectTab('library');assert.equal(first.disposals,1);assert.equal(c.mount.querySelector('[data-equipment-model]'),null);
  c.equipmentRender();assert.equal(first.disposals,1);
  c.equipmentSelectTab('designer');assert.equal(c.modelMounts.length,2);assert.notEqual(c.modelMounts[1].host,first.host);
  c.equipmentClose();c.equipmentClose();assert.equal(c.modelMounts[1].disposals,1);assert.equal(c.eq.draft.name,'Balanced');
});

test('same campaign state refresh keeps the preview but switching nations disposes before the new response arrives',async()=>{
  const c=fixture();loaded(c);c.equipmentRender();const first=c.modelMounts[0];
  c.S={session_id:'one',player:'USA'};c.equipmentOnStateChanged();await tick();assert.equal(first.disposals,0);assert.equal(c.modelMounts.length,1);
  let resolve;c.api=()=>new Promise(r=>resolve=r);c.S={session_id:'two',player:'Canada'};c.equipmentOnStateChanged();
  assert.equal(first.disposals,1);assert.equal(c.mount.querySelector('[data-equipment-model]'),null);
  resolve(snapshot({session_id:'two',nation:'Canada'}));await tick();assert.equal(c.modelMounts.length,2);assert.notEqual(c.modelMounts[1].host,first.host);
});

test('an unavailable or failed renderer leaves the configuration and priced review usable',()=>{
  for(const failure of ['missing','throws']){
    const c=fixture();loaded(c);c.window.EquipmentModel=failure==='missing'?null:{mount(){throw new Error('No graphics context');}};c.equipmentRender();
    const status=c.mount.querySelector('[data-equipment-model]').status.textContent;assert.match(status,/3D preview/);assert.match(status,/choices and design review remain available/);
    assert(c.mount.querySelector('#equipmentName'));assert.equal(c.mount.querySelector('[data-equipment-action="actions.0"]').disabled,false);assert.equal(c.calls.length,0);
  }
});

test('component choice permits a future draft but invalidates old effects and uses a pure server review',async()=>{
  const c=fixture();loaded(c);c.eq.advanced=true;c.equipmentRender();let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};
  const slot=c.mount.querySelector('[data-equipment-slot="sensors"]');slot.value='thermal';slot.focus();slot.onchange();
  assert.equal(c.eq.draft.components.sensors,'thermal');assert.equal(c.eq.preview,null);assert.equal(c.eq.previewLoading,true);assert.equal(c.calls.length,0);
  assert.deepEqual(c.requests[0],['/api/equipment-preview',{session_id:'one',name:'Balanced',platform:'tank_medium',components:{...spec().components,sensors:'thermal'}}]);
  assert.match(c.mount.innerHTML,/Research vehicle thermal observation/);assert.match(c.mount.innerHTML,/Review required research/);
  resolve(preview({valid:false,blockers:['Vehicle thermal observation is not known.'],actions:[{label:'Save future draft',command:{kind:'equipment_save'}}]}));await tick();
  assert.match(c.mount.innerHTML,/Resolve the requirements/);assert.match(c.mount.innerHTML,/Vehicle thermal observation is not known/);assert.doesNotMatch(c.mount.innerHTML,/Fund development/);assert.equal(c.equipmentPreviewCurrent(),true);
});

test('name and search edits preserve caret and never create orders or discard the draft on tab navigation',async()=>{
  const c=fixture();loaded(c);c.equipmentRender();const name=c.mount.querySelector('#equipmentName');name.value='Sentinel 90B';name.selectionStart=3;name.selectionEnd=6;name.focus();name.oninput();await tick();
  assert.equal(c.eq.draft.name,'Sentinel 90B');assert.equal(c.document.activeElement,c.mount.querySelector('#equipmentName'));assert.equal(c.document.activeElement.selectionStart,3);assert.equal(c.document.activeElement.selectionEnd,6);
  c.equipmentSelectTab('library');const search=c.mount.querySelector('#equipmentSearch');search.value='Sentinel';search.selectionStart=4;search.selectionEnd=4;search.focus();search.oninput();assert.equal(c.document.activeElement,c.mount.querySelector('#equipmentSearch'));
  c.equipmentSelectTab('designer');assert.equal(c.eq.draft.name,'Sentinel 90B');assert.equal(c.calls.length,0);
});

test('development, manufacturing and service show dated receipts, preserved platform age and real funding labels',()=>{
  const c=fixture();loaded(c);c.eq.tab='development';let html=c.equipmentContentHtml();
  for(const text of ['Current funding','Development authorization','6.2% complete','$750','Settled 10 Feb 1990','Recorded · 10 Feb 1990'])assert(html.includes(text),text);
  c.eq.tab='production';html=c.equipmentContentHtml();assert.match(html,/No equipment production is listed/);assert.match(html,/certified design/);
  c.eq.tab='service';html=c.equipmentContentHtml();assert.match(html,/Available vehicles/);assert.match(html,/Platform age/);assert.match(html,/12/);assert.match(html,/Review refit/);
});

test('unknown supplied metrics and costs remain unknown; all unsafe names, notes and attributes are escaped',()=>{
  const c=fixture();loaded(c);assert.match(c.equipmentMetric({label:'Age',value:null}),/—/);assert.match(c.equipmentCosts([{label:'Cost',amount_bn:null}]),/—/);assert.equal(c.equipmentMoney(.00000075),'$750');
  c.eq.draft.name='<script>';c.eq.preview.metrics=[{label:'<svg>',value:'<img>',period:'<iframe>'}];c.eq.preview.costs=[{label:'<input>',amount_bn:null}];const html=c.equipmentContentHtml();
  assert.doesNotMatch(html,/<script|<iframe|<img|<svg>/);assert.match(html,/&lt;script&gt;/);assert.match(html,/&lt;iframe&gt;/);
});

test('research links navigate to exact technology without acquiring it or changing allocations',()=>{
  const c=fixture();loaded(c);c.eq.draft.components.sensors='thermal';c.eq.advanced=true;c.equipmentRender();
  c.mount.querySelector('[data-equipment-research="thermal"]').onclick();assert.deepEqual(c.calls,[{action:'research',id:'land_thermal',name:'Vehicle thermal observation',domain:'Aerospace'}]);assert.equal(c.requests.length,0);
  c.equipmentSelectTab('research');c.mount.querySelector('[data-equipment-branch="optics"]').onclick();const button=c.mount.querySelector('[data-equipment-action="research.0.actions.0"]');assert.equal(button.onclick(),true);assert.equal(c.calls[1].id,'land_thermal');
});

test('save and programme actions require an explicit review confirmation and send exactly the supplied command',async()=>{
  const c=fixture();loaded(c);c.equipmentRender();const before=plain(c.eq.preview.actions[0].command);
  c.mount.querySelector('[data-equipment-action="actions.0"]').onclick();assert.equal(c.calls.length,0);assert.match(c.mount.innerHTML,/equipmentActionReviewTitle/);
  assert.equal(await c.equipmentConfirm(),true);assert.deepEqual(c.calls[0],before);assert.equal(c.calls.length,1);await tick();
});

test('quantity, site and daily funding changes request a new quote before an exact committed order',async()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('library');c.mount.querySelector('[data-equipment-action="designs.0.actions.0"]').onclick();await tick();
  assert.equal(c.calls.length,0);assert.equal(c.requests[0][1].command.quantity,1);assert.equal(c.equipmentReviewQuoteCurrent(),true);
  let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};
  const input=c.mount.querySelector('[data-equipment-order-input="quantity"]');input.value='3';input.focus();input.oninput();
  assert.equal(c.eq.review.command.quantity,3);assert.equal(c.equipmentReviewQuoteCurrent(),false);assert.equal(await c.equipmentConfirm(0),false);assert.equal(c.calls.length,0);
  resolve(quote());await tick();const site=c.mount.querySelector('[data-equipment-order-input="district"]');site.value='US-TX';site.onchange();resolve(quote());await tick();
  const funding=c.mount.querySelector('[data-equipment-order-input="daily_budget_mn"]');funding.value='3';funding.oninput();resolve(quote());await tick();
  assert.equal(c.requests.at(-1)[1].command.daily_budget_mn,3,'Display units stay as supplied input units; backend owns money conversion');
  assert.match(c.mount.innerHTML,/\$17m/);assert.match(c.mount.innerHTML,/45 days if funded and supplied/);
  const final=plain(c.eq.review.quote.actions[0].command);c.api=async path=>path.startsWith('/api/equipment?')?snapshot():preview();assert.equal(await c.equipmentConfirm(0),true);assert.deepEqual(c.calls,[final]);
});

test('late order quotes cannot re-enable a commit after input edits, tab navigation or closing',async()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('library');const resolvers=[];c.api=()=>new Promise(resolve=>resolvers.push(resolve));c.mount.querySelector('[data-equipment-action="designs.0.actions.0"]').onclick();
  let input=c.mount.querySelector('[data-equipment-order-input="quantity"]');input.value='4';input.oninput();assert.equal(resolvers.length,2);
  resolvers[1](quote({detail:'Latest selected quantity'}));await tick();resolvers[0](quote({detail:'Stale quantity'}));await tick();assert.equal(c.eq.review.quote.detail,'Latest selected quantity');
  input=c.mount.querySelector('[data-equipment-order-input="quantity"]');input.value='5';input.oninput();c.equipmentSelectTab('service');resolvers[2](quote());await tick();assert.equal(c.eq.review,null);assert.equal(await c.equipmentConfirm(0),false);
  c.equipmentClose();assert.equal(c.equipmentCurrent(),false);assert.equal(c.calls.length,0);
});

test('an invalid order quote or unknown number never falls back to a zero-cost accepted order',async()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('library');c.api=async()=>quote({valid:false,blockers:['Choose a positive vehicle quantity.']});c.mount.querySelector('[data-equipment-action="designs.0.actions.0"]').onclick();await tick();
  assert.equal(await c.equipmentConfirm(0),false);assert.match(c.mount.innerHTML,/Choose a positive vehicle quantity/);assert.equal(c.mount.querySelector('[data-equipment-intent="0"]').disabled,true);
  const input=c.mount.querySelector('[data-equipment-order-input="quantity"]');input.value='';input.oninput();assert.equal(c.eq.review.command.quantity,null);assert.equal(c.calls.length,0);
});

test('stale state, pending orders and replacement payloads block captured buttons and direct confirmations',()=>{
  for(const change of [c=>c.eq.stale=true,c=>c.eq.loading=true,c=>c.eq.error='failed',c=>c.advancing=true,c=>c.pendingAdvance={},c=>c.COMMAND_CHANNEL.busy=true,c=>c.COMMAND_CHANNEL.pending={},c=>c.SESSION.busy=true,c=>c.PROD.busy=true,c=>c.eq.open=false,c=>c.S={session_id:'one',player:'USA'}]){
    const c=fixture();loaded(c);c.equipmentRender();const button=c.mount.querySelector('[data-equipment-action="actions.0"]');change(c);assert.equal(button.onclick(),false);assert.equal(c.calls.length,0);
  }
  const c=fixture();loaded(c);c.equipmentSelectTab('library');const button=c.mount.querySelector('[data-equipment-action="designs.0.actions.0"]');c.eq.data=snapshot();assert.equal(button.onclick(),false);
});

test('reads and previews validate session identity and rejected previews cannot expose a legal action',async()=>{
  const c=fixture();assert.equal(await c.equipmentFetch(),true);await tick();assert.equal(c.requests[0][0],'/api/equipment?session_id=one');assert.equal(c.requests[1][0],'/api/equipment-preview');assert.equal(c.requests[1][1].session_id,'one');assert.equal(c.eq.state,c.S);
  c.api=async()=>preview({session_id:'other'});assert.equal(await c.equipmentFetchPreview(),false);assert.match(c.eq.previewError,/did not match/);assert.equal(c.equipmentPreviewCurrent(),false);
  c.api=async()=>snapshot({nation:'Canada'});assert.equal(await c.equipmentFetch(),false);assert.match(c.eq.error,/did not match/);assert.equal(c.equipmentCurrent(),false);
});

test('new campaign removes old drafts and holdings immediately while same-campaign refresh preserves work',async()=>{
  const c=fixture();loaded(c);c.eq.draft.name='My unsaved tank';c.equipmentRender();c.S={session_id:'one',player:'USA'};c.equipmentOnStateChanged();await tick();assert.equal(c.eq.draft.name,'My unsaved tank');
  let resolve;c.api=()=>new Promise(r=>resolve=r);c.S={session_id:'two',player:'Canada'};c.equipmentOnStateChanged();assert.equal(c.eq.draft,null);assert.equal(c.eq.data,null);assert.equal(c.eq.session,'two');assert.doesNotMatch(c.mount.innerHTML,/My unsaved tank|Sentinel 90/);
  resolve(snapshot({session_id:'two',nation:'Canada',name:'Canada'}));await tick();assert.equal(c.eq.data.name,'Canada');
});

test('an old design quote loses to the latest draft and cannot survive closing',async()=>{
  const c=fixture();loaded(c);const resolvers=[];c.api=()=>new Promise(r=>resolvers.push(r));const first=c.equipmentFetchPreview();c.eq.draft.name='Latest';c.equipmentDraftChanged();
  resolvers[1](preview({detail:'Latest review'}));await tick();resolvers[0](preview({detail:'Old review'}));await first;assert.equal(c.eq.preview.detail,'Latest review');
  const pending=c.equipmentFetchPreview();c.equipmentClose();resolvers[2](preview());assert.equal(await pending,false);assert.equal(c.equipmentPreviewCurrent(),false);
});

test('refresh failures remain explicit and retryable without losing a draft or creating a polling loop',async()=>{
  const c=fixture();loaded(c);c.eq.draft.name='Keep this draft';c.api=async()=>{throw new Error('Offline');};assert.equal(await c.equipmentFetch(),false);assert.match(c.mount.innerHTML,/Offline/);assert.equal(c.eq.draft.name,'Keep this draft');
  c.equipmentBind();assert.equal(c.eq.loading,false);c.api=async path=>path.startsWith('/api/equipment?')?snapshot():preview();c.mount.querySelector('[data-equipment-refresh]').onclick();await tick();assert.equal(c.eq.error,'');assert.equal(c.eq.draft.name,'Keep this draft');
});

test('tabs support roving keyboard focus and details, scroll and refresh focus survive repaint',async()=>{
  const c=fixture();loaded(c);c.equipmentRender();c.scroller.scrollTop=470;c.mount.querySelector('details[data-equipment-detail="spec:electronics"]').open=true;c.equipmentRememberView();c.equipmentRender();assert(c.eq.details.has('spec:electronics'));assert.equal(c.scroller.scrollTop,470);
  const tab=c.mount.querySelector('[data-equipment-tab="designer"]');let prevented=false;tab.onkeydown({key:'ArrowRight',preventDefault(){prevented=true;}});assert(prevented);assert.equal(c.eq.tab,'library');assert.equal(c.document.activeElement.dataset.equipmentFocus,'tab:library');
  let resolve;c.api=()=>new Promise(r=>resolve=r);const refresh=c.mount.querySelector('[data-equipment-refresh]');refresh.focus();refresh.onclick();assert.equal(c.eq.focus.key,'refresh');resolve(snapshot());await tick();assert.equal(c.document.activeElement.dataset.equipmentFocus,'refresh');
});

test('explicit opening uses root visibility, respects requested tabs and makes one snapshot read',async()=>{
  const c=fixture();c.eq.open=false;await c.window.openEquipment({tab:'library'});await tick();assert.equal(c.eq.tab,'library');assert.equal(c.calls[0],'open');assert.equal(c.requests.filter(row=>row[0].startsWith('/api/equipment?')).length,1);
  c.equipmentBind();assert.equal(c.requests.filter(row=>row[0].startsWith('/api/equipment?')).length,1);c.equipmentClose();const count=c.requests.length;c.equipmentOnStateChanged();assert.equal(c.requests.length,count);
});

test('published assets parse and retain scoped responsive controls without an alternative command channel',()=>{
  new vm.Script(source);assert.match(css,/\.equipment-room/);assert.match(css,/@media\(max-width:700px\)/);assert.match(css,/focus-visible/);assert.match(css,/min-height:42px/);
  assert.doesNotMatch(source,/\/api\/command|\/api\/advance|fetch\(/);assert.match(source,/equipmentCommand\(equipmentCopy\(action.command\)\)/);
});

test('order review reveals itself from a long catalogue and retains its raw-input requirements through requotes',async()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('library');c.scroller.scrollTop=980;
  c.api=async()=>quote({requirements:['Raw input Steel: 1.250 kt','Funding authority is shared with other projects.']});
  const button=c.mount.querySelector('[data-equipment-action="designs.0.actions.0"]');button.focus();button.onclick();await tick();
  assert.equal(c.scroller.scrollTop,0);assert.equal(c.document.activeElement.id,'equipmentActionReviewTitle');
  assert.match(c.mount.innerHTML,/Inputs and conditions/);assert.match(c.mount.innerHTML,/Raw input Steel: 1.250 kt/);
  const detail=c.mount.querySelector('details[data-equipment-detail="order-requirements"]');detail.open=true;detail.ontoggle();
  const input=c.mount.querySelector('[data-equipment-order-input="quantity"]');input.value='2';input.oninput();await tick();
  assert.equal(c.mount.querySelector('details[data-equipment-detail="order-requirements"]').open,true);
  assert.equal(c.calls.length,0);
});

test('active equipment campaigns expose supplied funding and plant navigation without inventing orders',()=>{
  const c=fixture(),nav={action:'budget',ministry:'defense',department:4};loaded(c,snapshot({actions:[{label:'Development funding',navigate:nav},{label:'Build an arms plant',navigate:{action:'construction',kind:'arms_plant'}}]}));c.equipmentRender();
  assert.match(c.mount.innerHTML,/Funding and facilities/);assert.match(c.mount.innerHTML,/Build an arms plant/);
  c.mount.querySelectorAll('[data-equipment-action="actions.0"]').find(node=>node.dataset.equipmentScope==='data').onclick();
  assert.deepEqual(c.calls,[nav]);assert.equal(c.requests.length,0);
});

function shellSource(name){
  const match=new RegExp(`^(?:async\\s+)?function ${name}\\(`,'m').exec(page);assert(match,`Page contains ${name}`);
  const line=page.slice(match.index,page.indexOf('\n',match.index));return /\}\s*$/.test(line)?line:page.slice(match.index,page.indexOf('\n}',match.index)+2);
}
function shellFixture(){
  const c=fixture(),app={inert:false};
  const launch={isConnected:true,getClientRects:()=>[1],closest:()=>null,focus(){c.document.activeElement=this;}},fallback={focus(){c.document.activeElement=this;}};
  const room={id:'equipmentRoom',hidden:true,scrollTop:0,attributes:{},setAttribute(key,value){this.attributes[key]=value;},focus(){c.document.activeElement=this;},
    set innerHTML(value){c.mount.innerHTML=value;},get innerHTML(){return c.mount.innerHTML;},querySelectorAll:selector=>c.mount.querySelectorAll(selector)};
  const nodes={'#equipmentRoom':room,'#equipmentRoot':c.mount,'#app':app,'#techBtn':fallback};c.$=selector=>nodes[selector]||null;
  c.document.activeElement=launch;c.document.querySelector=selector=>nodes[selector]||c.mount.querySelector(selector);c.mount.parentElement=room;
  c.LOGI={open:false};c.stock={open:false};c.tech={open:false,byId:new Map([['radar',0]]),data:[{id:'radar',domain:'Computing'}]};
  for(const name of ['closeGlobalMenus','closeTechMenu','closeSheet','closeGameDrawers','closeLogistics','closeProduction','closeStock','closeTech','closeDomination','setKeysCard'])c[name]=()=>c.calls.push(name);
  c.dominationIsOpen=()=>false;c.keysCardIsOpen=()=>false;c.openConstruction=value=>c.calls.push(['construction',plain(value)]);c.openProduction=()=>c.calls.push('manufacture');c.openStock=()=>c.calls.push('resources');
  c.cashFlowNavigate=value=>{c.calls.push(['budget',plain(value)]);return true;};c.openTech=async domain=>{c.calls.push(['tech',domain]);c.tech.open=true;};
  c.setTechView=(...args)=>c.calls.push(['tech-view',...args]);c.techNodeClick=index=>c.calls.push(['tech-node',index]);
  vm.runInContext('const EQUIPMENT_ROOM={lastFocus:null};\n'+['equipmentIsOpen','equipmentScroller','openEquipmentDrawer','closeEquipmentDrawer','equipmentExternalPending','equipmentCommand','equipmentNavigate'].map(shellSource).join('\n'),c);
  return Object.assign(c,{room,app,launch,fallback});
}

test('fullscreen bridge mounts outside the inert game, closes other rooms, and preserves the draft on return',async()=>{
  const c=shellFixture();loaded(c);c.eq.draft.name='Unsaved local revision';
  await c.openEquipment({tab:'designer'});await tick();assert.equal(c.room.hidden,false);assert.equal(c.room.attributes['aria-hidden'],'false');assert.equal(c.app.inert,true);
  assert(c.calls.includes('closeGameDrawers'));assert(c.calls.includes('closeTechMenu'));assert.equal(c.document.activeElement.id,'equipmentTitle');
  const count=c.calls.length;c.openEquipmentDrawer();assert.equal(c.calls.length,count,'Reopening the mounted shell cannot close its own lifecycle');
  c.closeEquipmentDrawer();assert.equal(c.eq.open,false);assert.equal(c.eq.draft.name,'Unsaved local revision');assert.equal(c.room.hidden,true);assert.equal(c.app.inert,false);assert.equal(c.document.activeElement,c.launch);
  c.document.activeElement=c.launch;await c.openEquipment({tab:'library'});await tick();assert.equal(c.eq.draft.name,'Unsaved local revision');assert.equal(c.eq.tab,'library');
  c.launch.isConnected=false;c.closeEquipmentDrawer();assert.equal(c.document.activeElement,c.fallback);
});

test('shell commands use the existing channel and adopted response while preserving local drafts and policy queues',async()=>{
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.draft.name='Still editing';c.queued=[{kind:'tax',value:20}];
  const current={session_id:'one',player:'USA',receipt:'server'};c.api=async(...args)=>{c.requests.push(plain(args));return current;};
  c.adopt=async(state,history)=>{assert.equal(state,current);assert.equal(history,false);c.S=state;c.equipmentOnStateChanged();};
  const command={kind:'equipment_save',name:'Reviewed',...spec()};await c.equipmentCommand(command);
  assert.deepEqual(c.requests[0],['/api/command',{commands:[command]}]);assert.equal(c.eq.draft.name,'Still editing');assert.deepEqual(c.queued,[{kind:'tax',value:20}]);
  const before=c.requests.length;c.COMMAND_CHANNEL.pending={};await assert.rejects(c.equipmentCommand(command),/current order/);assert.equal(c.requests.length,before);
  c.COMMAND_CHANNEL.pending=null;await assert.rejects(c.equipmentCommand({kind:'set_tax'}),/reviewed equipment order/);
});

test('refused, uncertain and cross-campaign command responses never adopt or announce a successful order',async()=>{
  for(const response of [{errors:['Insufficient authorization']},{command_pending:true},null]){
    const c=shellFixture();c.adopt=()=>assert.fail('Rejected or cross-campaign response was adopted');
    c.api=async()=>{if(response===null)c.S={session_id:'two',player:'CAN'};return response||{session_id:'one'};};
    await assert.rejects(c.equipmentCommand({kind:'equipment_cancel',project:1}),response===null?/campaign changed/:response.command_pending?/awaiting confirmation/:/Insufficient authorization/);
  }
});

test('research navigation keeps equipment-only integrations local and pins a genuine prerequisite in its real domain',async()=>{
  const c=shellFixture();loaded(c);c.room.hidden=false;c.equipmentRender();c.calls.length=0;
  await c.equipmentNavigate({action:'research',equipment:true,id:'thermal'});assert.equal(c.eq.tab,'research');assert.equal(c.room.hidden,false);assert.equal(c.document.activeElement.dataset.equipmentRecord,'thermal');assert.equal(c.calls.length,0);
  await c.equipmentNavigate({action:'research',id:'radar',domain:'Aerospace'});assert.equal(c.room.hidden,true);assert.deepEqual(c.calls,[['tech','Aerospace'],['tech-view','Computing',true],['tech-node',0]]);
});

test('funding and plant shortcuts retain exact backend targets and never issue equipment commands',async()=>{
  const c=shellFixture();loaded(c);c.room.hidden=false;await c.equipmentNavigate({action:'budget',ministry:'defense',department:4});assert.deepEqual(c.calls.at(-1),['budget',{action:'budget',ministry:'defense',department:4}]);
  c.room.hidden=false;c.eq.open=true;await c.equipmentNavigate({action:'construction',kind:'arms_plant',district:'US-TX'});assert.deepEqual(c.calls.at(-1),['construction',{kind:'arms_plant',province:'US-TX'}]);assert.equal(c.requests.length,0);
  c.COMMAND_CHANNEL.busy=true;const before=c.calls.length;assert.equal(await c.equipmentNavigate({action:'budget'}),false);assert.equal(c.calls.length,before);
});

test('fullscreen equipment consumes world shortcuts, permits native input, pauses a running clock and closes on Escape',()=>{
  const c=shellFixture(),start=page.indexOf('document.addEventListener("keydown", (e) => {\n  const room = arcadeTopRoom();');
  assert(start>=0);const end=page.indexOf('\n});',start);let handler;c.document.addEventListener=(name,fn)=>{if(name==='keydown')handler=fn;};
  c.arcadeTopRoom=()=>c.room;c.arcadeTrapTab=()=>c.calls.push('trap');c.clock={running:false};c.typing=()=>false;c.clockPause=()=>{c.clock.running=false;c.calls.push('pause');};
  vm.runInContext(page.slice(start,end+4),c);c.room.hidden=false;let prevented=false;const event=key=>({key,target:{closest:()=>null},preventDefault(){prevented=true;}});
  handler(event('2'));handler(event('Enter'));assert.equal(c.calls.length,0);assert.equal(prevented,false,'Native buttons keep Enter');
  handler(event('Tab'));assert.deepEqual(c.calls,['trap']);c.clock.running=true;handler(event(' '));assert.equal(c.calls.at(-1),'pause');assert(prevented);
  handler(event('Escape'));assert.equal(c.room.hidden,true);
});

test('raw-input navigation opens Resources without orders or loss of an unsaved equipment draft',async()=>{
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.draft.name='Unsaved supply review';
  assert.equal(await c.equipmentNavigate({action:'resources'}),true);assert.equal(c.room.hidden,true);assert.equal(c.eq.open,false);
  assert.equal(c.calls.at(-1),'resources');assert.equal(c.eq.draft.name,'Unsaved supply review');assert.equal(c.requests.length,0);
  c.pendingAdvance={};const count=c.calls.length;assert.equal(await c.equipmentNavigate({action:'resources'}),false);assert.equal(c.calls.length,count);
});

test('the published shell includes reachable research and manufacturing entrances plus session and modal hooks',()=>{
  for(const asset of ['equipment-mesh.js','equipment-export.js','equipment-model.js','equipment-ui.js','equipment-ui.css'])assert(page.includes('/'+asset));
  assert(page.indexOf('/equipment-mesh.js')<page.indexOf('/equipment-model.js'));
  assert(page.indexOf('/equipment-export.js')<page.indexOf('/equipment-model.js'));
  assert(page.indexOf('/equipment-model.js')<page.indexOf('/equipment-ui.js'));
  assert.match(page,/<div id="equipmentRoom"[^>]*role="dialog"[^>]*hidden>/);assert.match(css,/#equipmentRoom\[hidden\]/);
  assert.match(shellSource('arcadeTopRoom'),/equipmentIsOpen\(\)/);assert.match(shellSource('adopt'),/equipmentOnStateChanged\(\)/);assert.match(shellSource('resetCampaignUi'),/equipmentClose\(\)/);assert.match(shellSource('closeGameDrawers'),/closeEquipmentDrawer\(\)/);
  assert.match(shellSource('renderTechMenu'),/domain\?\.project\?\.equipment/);assert.match(shellSource('renderTechMenu'),/data-equipment-bureau/);
  assert.match(shellSource('manufacturingCatalogHtml'),/data-equipment-open/);assert.match(shellSource('manufacturingLinesHtml'),/data-equipment-open/);assert.match(shellSource('wireManufacturingPanel'),/openEquipment\(\)/);
  assert.match(page,/<button[^>]*onclick="openEquipment\(\)"[^>]*>Equipment designer<\/button>/);
  for(const script of page.matchAll(/<script(?:\s[^>]*)?>([\s\S]*?)<\/script>/g))if(script[1].trim())new vm.Script(script[1]);
});

test('legacy manufacturing holdings distinguish complete vehicles from legacy equipment equivalents',()=>{
  const c=vm.createContext({window:{},manufacturingHoldings:()=>[{name:'New model',units:3,unit_label:'vehicles',book_value_bn:.01,condition:1}],manufacturingOrders:()=>[{name:'Inherited procurement',units:4,unit_label:'legacy equivalents',value_bn:.03,due_date:'Tomorrow'}],fmtQ:String,escText:decode,manufacturingBn:n=>String(n)});
  vm.runInContext(shellSource('manufacturingKit3d')+'\n'+shellSource('manufacturingLedgerHtml'),c);const html=c.manufacturingLedgerHtml();assert.match(html,/3 vehicles/);assert.match(html,/4 legacy equivalents/);assert.doesNotMatch(html,/3 units|4 units/);
});

test('research branches expose real prerequisites, unlocks and four states with independent filters',()=>{
  const c=fixture(),research=[
    {id:'basic',name:'Basic chassis',branch:'chassis',state:'known',unlocks:['Tracked chassis'],prerequisites:[]},
    {id:'improved',name:'Improved chassis',branch:'chassis',state:'available',prerequisites:[{id:'basic',name:'Basic chassis',known:true,equipment:true}],unlocks:['Light chassis'],actions:[{label:'Review integration',command:{kind:'equipment_research',component:'improved'}}]},
    {id:'future',name:'Protected chassis',branch:'chassis',state:'locked',prerequisites:[{id:'improved',name:'Improved chassis',known:false,equipment:true}],unlock_components:[{id:'future_hull',name:'Reinforced hull',slot:'protection'}]},
    {id:'active',name:'Vehicle suspension',branch:'chassis',state:'researching',progress:.25,prerequisites:[],unlocks:['Hydropneumatic suspension']},
    {id:'digital',name:'Digital fire control',branch:'optics',state:'available',prerequisites:[{id:'basic',name:'Basic chassis',known:true,equipment:true},{id:'computer',name:'Computers',known:false,domain:'Computing'}],unlocks:['Digital sights']}
  ];loaded(c,snapshot({research}));c.equipmentSelectTab('research');
  for(const state of ['known','available','locked','researching'])assert.match(c.mount.innerHTML,new RegExp(`data-equipment-research-state="${state}"`));
  assert.deepEqual(c.mount.querySelectorAll('[data-equipment-branch]').map(button=>button.dataset.equipmentBranch),['chassis','engines','weapons','armor','optics','communications']);assert.match(c.mount.innerHTML,/Stage 3/);assert.match(c.mount.innerHTML,/Reinforced hull/);assert.match(c.mount.innerHTML,/25% complete/);
  c.mount.querySelector('[data-equipment-research-filter="locked"]').onclick();assert.match(c.mount.innerHTML,/Protected chassis/);assert.doesNotMatch(c.mount.innerHTML,/>Vehicle suspension<|>Basic chassis<\/h3>/);
  const search=c.mount.querySelector('#equipmentSearch');search.value='missing';search.oninput();assert.match(c.mount.innerHTML,/No technologies match this view/);
  c.mount.querySelector('[data-equipment-research-reset]').onclick();assert.equal(c.eq.query,'');assert.equal(c.eq.researchState,'all');
  c.mount.querySelector('[data-equipment-branch="optics"]').onclick();assert.match(c.mount.innerHTML,/Digital sights/);
  c.mount.querySelector('[data-equipment-prerequisite="4:0"]').onclick();assert.equal(c.eq.researchBranch,'chassis');assert.equal(c.document.activeElement.dataset.equipmentRecord,'basic');
  c.mount.querySelector('[data-equipment-branch="optics"]').onclick();c.mount.querySelector('[data-equipment-prerequisite="4:1"]').onclick();assert.deepEqual(c.calls,[{action:'research',id:'computer',domain:'Computing',name:'Computers'}]);assert.equal(c.requests.length,0);
});

test('research review buttons and prerequisite links retain campaign freshness guards after filtering',()=>{
  const c=fixture(),research=[{id:'entry',name:'Integration',branch:'chassis',state:'available',prerequisites:[{id:'computer',name:'Computers',known:false,domain:'Computing'}],actions:[{label:'Review integration',command:{kind:'equipment_research',component:'entry'}}]}];loaded(c,snapshot({research}));c.equipmentSelectTab('research');
  const action=c.mount.querySelector('[data-equipment-action="research.0.actions.0"]'),prerequisite=c.mount.querySelector('[data-equipment-prerequisite="0:0"]');
  c.eq.stale=true;assert.equal(action.onclick(),false);assert.equal(prerequisite.onclick(),false);assert.equal(c.calls.length,0);assert.equal(c.eq.review,null);
});

test('research connectors represent only actual visible prerequisite edges',()=>{
  const c=fixture(),rows=[{id:'base',prerequisites:[]},{id:'upgrade',prerequisites:[{id:'base'},{id:'external'}]},{id:'branch',prerequisites:[{id:'base'}]},{id:'unrelated',prerequisites:[]}];
  assert.deepEqual(plain(c.equipmentResearchEdges(rows)),[{from:'base',to:'upgrade'},{from:'base',to:'branch'}]);
  assert.deepEqual(plain(c.equipmentResearchEdges(rows.slice(1))),[]);
  loaded(c,snapshot({research:[{id:'base',branch:'chassis',name:'Known foundation',state:'known',progress:0}]}));c.eq.tab='research';assert.doesNotMatch(c.equipmentResearchHtml(),/0% complete/);
});

test('vehicle family changes preserve compatible parts and discard incompatible slots and source revisions',async()=>{
  const c=fixture(),data=snapshot(),ground={id:'ground_apc',name:'Armored personnel carrier',slots:[{id:'mobility',name:'Engine',components:['diesel']},{id:'wheels',name:'Wheels',components:['road_wheels']},{id:'troop_compartment',name:'Troop compartment',components:['troops']}],default_spec:{platform:'ground_apc',components:{mobility:'diesel',wheels:'road_wheels',troop_compartment:'troops'}}};
  data.platforms.push(ground);data.components.push({id:'road_wheels',name:'Road wheels',known:true},{id:'troops',name:'Protected troop seats',known:true});data.presets.push({id:'apc',name:'Personnel carrier starter',spec:ground.default_spec});loaded(c,data);c.eq.draft.components.tracks='old_tracks';c.eq.draft.source_revision=7;c.equipmentRender();
  assert.equal(c.mount.querySelectorAll('[data-equipment-preset]').length,2);assert.doesNotMatch(c.mount.innerHTML,/Personnel carrier starter/);
  c.mount.querySelector('[data-equipment-family="ground_apc"]').onclick();await tick();assert.equal(c.eq.draft.platform,'ground_apc');assert.deepEqual(plain(c.eq.draft.components),ground.default_spec.components);assert.equal(c.eq.draft.source_revision,undefined);assert.equal(c.mount.querySelectorAll('[data-equipment-preset]').length,1);assert.match(c.mount.innerHTML,/Personnel carrier starter/);assert.match(c.mount.innerHTML,/Troop compartment/);assert.equal(c.calls.length,0);
});

test('platform changes update untouched starting names while retaining typed and saved names',async()=>{
  const c=fixture(),data=snapshot(),ifv={id:'ground_ifv',name:'Infantry fighting vehicle',slots:data.platforms[0].slots,default_spec:{...spec(),platform:'ground_ifv'}};
  data.platforms.push(ifv);data.presets.push({id:'ground_ifv',name:ifv.name,spec:ifv.default_spec});loaded(c,data);c.equipmentRender();
  assert.equal(c.eq.draft.name,'Balanced');c.mount.querySelector('[data-equipment-family="ground_ifv"]').onclick();await tick();
  assert.equal(c.eq.draft.name,'Infantry fighting vehicle');assert.equal(c.requests.at(-1)[1].name,'Infantry fighting vehicle');
  c.equipmentChangePlatform('tank_medium');await tick();assert.equal(c.eq.draft.name,'Affordable');
  const input=c.mount.querySelector('#equipmentName');input.value='Vanguard 91';input.oninput();await tick();c.equipmentChangePlatform('ground_ifv');await tick();assert.equal(c.eq.draft.name,'Vanguard 91');
  c.equipmentSetDraft(data.designs[0]);await tick();c.equipmentChangePlatform('ground_ifv');await tick();assert.equal(c.eq.draft.name,'Sentinel 90');assert.equal(c.calls.length,0);
});

test('fractional ratings display at most three decimals while preserving precise server comparison data',()=>{
  const c=fixture();loaded(c);const comparison={available:true,id:'preset:balanced',name:'Baseline',same_role:true,rows:[{key:'protected_mobility',label:'Protected mobility',before:.407134,after:.51948,delta:.112346,unit:'rating',better_when:'higher'}],changes:[]};c.eq.preview.comparison=comparison;const original=plain(comparison);
  assert.match(c.equipmentMetric({label:'Protected mobility',value:.51948}),/>0\.519</);assert.equal(c.equipmentNumber(1.23456789),'1.235');assert.equal(c.equipmentNumber(12),'12');
  const html=c.equipmentComparisonHtml();for(const value of ['0.407 rating','0.519 rating','+0.112 rating'])assert(html.includes(value),value);assert.deepEqual(plain(comparison),original);assert.equal(c.equipmentComparisonValue(.00000075,'bn'),'$750');
});

test('comparison uses supplied numbers, directional tradeoffs and component changes with a persistent baseline',async()=>{
  const c=fixture(),options=[{id:'preset:balanced',name:'Balanced original',kind:'preset',platform:'tank_medium'},{id:7,name:'Sentinel revision 7',kind:'revision',platform:'tank_medium'}];loaded(c,snapshot({comparison_options:options}));
  c.eq.preview.comparison={available:true,id:'preset:balanced',name:'Balanced original',same_role:true,rows:[
    {key:'armor',label:'Protection',before:1,after:1.4,delta:.4,unit:'rating',better_when:'higher'},
    {key:'maintenance',label:'Annual maintenance',before:.00001,after:.000015,delta:.000005,unit:'bn',better_when:'lower'},
    {key:'development',label:'Development time',before:30,after:20,delta:-10,unit:'days',better_when:'lower'},
    {key:'unknown',label:'Unreported',before:null,after:null,delta:null,unit:'rating',better_when:'neutral'}],changes:[{slot:'protection',label:'Armor',before:'Steel',after:'Composite'}]};c.equipmentRender();
  for(const text of ['Balanced original','1.4 rating','+0.4 rating','$15k','+$5k','-10 days','Improvement','Tradeoff','Composite','—'])assert(c.mount.innerHTML.includes(text),text);
  assert.equal(c.eq.comparisonId,'preset:balanced');const select=c.mount.querySelector('#equipmentComparison');select.value='7';select.onchange();await tick();assert.equal(c.requests.at(-1)[1].comparison_id,7);
  const name=c.mount.querySelector('#equipmentName');name.value='New title';name.oninput();await tick();assert.equal(c.eq.preset,null);assert.equal(c.eq.comparisonId,7);assert.equal(c.requests.at(-1)[1].comparison_id,7);assert.equal(c.calls.length,0);
});

test('late comparison responses cannot price a different baseline or enable its old actions',async()=>{
  const c=fixture();loaded(c,snapshot({comparison_options:[{id:'preset:balanced',name:'Balanced',platform:'tank_medium'},{id:7,name:'Older revision',platform:'tank_medium'}]}));c.equipmentRender();const pending=[];c.api=(...args)=>new Promise(resolve=>pending.push({args,resolve}));
  let control=c.mount.querySelector('#equipmentComparison');control.value='7';control.onchange();control=c.mount.querySelector('#equipmentComparison');control.value='preset:balanced';control.onchange();
  pending[0].resolve(preview({detail:'Old baseline response'}));await tick();assert.equal(c.eq.preview,null);assert.equal(c.eq.previewLoading,true);
  pending[1].resolve(preview({detail:'Current baseline response'}));await tick();assert.match(c.mount.innerHTML,/Current baseline response/);assert.doesNotMatch(c.mount.innerHTML,/Old baseline response/);assert.equal(c.equipmentPreviewCurrent(),true);
});

test('modernization gives reasons and tradeoffs and opens a local proposal without issuing an order',async()=>{
  const c=fixture(),proposal={id:'suggestion:7',name:'Sentinel upgrade',status:'Opportunity',detail:'Upgrade the inherited revision.',reason:'The fleet lacks thermal observation.',tradeoff:'Higher maintenance and time in refit.',source_revision:7,spec:{...spec(),components:{...spec().components,sensors:'thermal'}},metrics:[{label:'Vehicles affected',value:3}],actions:[action({label:'Review fleet refit'})]};loaded(c,snapshot({modernization:[proposal],comparison_options:[{id:7,name:'Sentinel 90',platform:'tank_medium'}]}));c.equipmentSelectTab('service');
  for(const text of ['Suggested modernization','Why this helps','The fleet lacks thermal observation.','The tradeoff','Higher maintenance','Review fleet refit'])assert(c.mount.innerHTML.includes(text),text);
  const source=plain(proposal);assert.equal(c.mount.querySelector('[data-equipment-modernization="0"]').onclick(),true);await tick();assert.equal(c.eq.tab,'designer');assert.equal(c.eq.draft.source_revision,7);assert.equal(c.eq.draft.components.sensors,'thermal');assert.equal(c.eq.comparisonId,7);assert.deepEqual(plain(proposal),source);assert.equal(c.calls.length,0);
});

test('model part events open and focus their specification without repricing or remounting the camera',()=>{
  const c=fixture();loaded(c);c.equipmentRender();const model=c.modelMounts[0],before=c.mount.innerHTML,original=plain(c.eq.draft),select=c.mount.querySelector('[data-equipment-slot="sensors"]'),group=select.parentDetail;
  assert.equal(group.open,false);assert(c.mount.querySelector('[data-model-part]'));c.mount['onequipment-part-select']({detail:{slot:'sensors',part:'thermal_sight',label:'Thermal sight'}});
  assert.equal(group.open,true);assert(c.eq.details.has(group.dataset.equipmentDetail));assert.equal(c.document.activeElement,select);assert.equal(c.eq.selectedSlot,'sensors');assert.equal(model.selectedParts.at(-1),'sensors');assert.deepEqual(plain(c.eq.draft),original);assert.equal(c.mount.innerHTML,before);assert.equal(c.modelMounts.length,1);assert.equal(c.requests.length,0);assert.equal(c.calls.length,0);
  assert.equal(c.equipmentRevealSpecification('not_a_slot'),false);c.equipmentRender();assert.equal(c.mount.listeners.filter(type=>type==='equipment-part-select').length,1);assert(c.mount.querySelector('[data-equipment-detail="spec:electronics"]').open);
});
