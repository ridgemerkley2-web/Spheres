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
function supply(extra={}){return {title:'Production supply plan',status:'Some programmes need attention',stage:'national',detail:'Review funded production, refits and shared warehouse stock.',
  metrics:[{label:'Production and refit programmes',value:2},{label:'Planning date',value:'11 Feb 1990'}],
  resources:[{commodity:'steel',name:'Steel',unit:'kt',remaining:1.25,planned_day:.04,stock:.03,shortfall:.01,detail:'Shared stock is not reserved.'},{commodity:'gas',name:'Natural gas',unit:'bcf',remaining:.0000456,planned_day:.00000047,stock:null,shortfall:null}],
  warnings:['Future deliveries are not counted as stock.'],actions:[{label:'Review Steel supply',navigate:{action:'resources',commodity:'steel'}}],...extra};}
function service(extra={}){return {title:'Fleet service readiness',status:'Review fleet support',detail:'Delivered vehicles, refits and incoming deliveries remain separate.',
  metrics:[{label:'Available vehicles',value:17},{label:'In refit',value:2},{label:'Incoming deliveries',value:3},{label:'Annual maintenance need',value:'$420k'}],
  roles:[{label:'Protected mobility',value:'0.742 capability',detail:'At currently recorded military support.'},{label:'Observation',value:null,detail:'This reading has no estimate.'}],
  warnings:['Inherited equipment is included in the arsenal.'],actions:[{label:'Review military support',navigate:{action:'budget',ministry:'defense',department:2}}],...extra};}
function retirement(extra={}){return action({label:'Review retirement',command:{kind:'equipment_retire',revision:'fleet-ifv',quantity:1},inputs:[{key:'quantity',label:'Vehicles to retire',type:'number',value:1,min:1,max:3,step:1}],...extra});}
function retirementQuote(extra={}){return quote({detail:'Retire the selected available vehicles.',costs:[],timing:[],metrics:[],requirements:[],
  service_effects:service({title:'Fleet after retirement',status:'Permanent removal',detail:'These vehicles leave service immediately.',
    metrics:[{label:'Available vehicles',value:'17 → 16'},{label:'Annual maintenance need',value:'$420k → $396k'}],roles:[{label:'Protected mobility',value:'0.742 → 0.719',detail:'At unchanged recorded support.'}],
    warnings:['Retirement is permanent. There is no refund or recovered material.','Lower maintenance need does not reduce the military budget or release cash.'],actions:[]}),
  actions:[{label:'Confirm retirement',command:{kind:'equipment_retire',revision:'fleet-ifv',quantity:1},requires_preview:false}],...extra});}
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
  for(const text of ['From concept to service','Research','Designer','Library','Development','Production','In service','Individual specifications','Interactive ground and aviation model','Observation','1.3','One-time work','$30m','$4m','30 days'])assert(html.includes(text),text);
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

function aviationSnapshot(){
  const data=snapshot(),{build}=require('../../spheres-web/ui/equipment-mesh.js');
  for(const [id,name] of [['air_light_attack','Light attack aircraft'],['air_tactical_strike','Tactical strike aircraft']]){
    const defaults=build({platform:id}).specification;
    const slots=Object.entries(defaults.components).map(([slot,component])=>({id:slot,name:slot.replace('air_','').replaceAll('_',' '),required:true,components:[component,...(slot==='air_payload'?['air_payload_guided']:[])]}));
    data.platforms.push({id,name,family:'aviation',family_name:'Tactical aviation',slots,default_spec:defaults});
    data.presets.push({id,name,spec:defaults});
    for(const slot of slots)for(const component of slot.components)if(!data.components.some(c=>c.id===component))data.components.push({id:component,name:component.replaceAll('_',' '),slot:slot.id,known:true});
  }
  return data;
}
test('aviation family uses supplied defaults, clears ground slots and groups all eight independent specifications',async()=>{
  const c=fixture(),data=aviationSnapshot();loaded(c,data);c.eq.draft.source_revision=7;c.equipmentRender();
  c.mount.querySelector('[data-equipment-family="aviation"]').onclick();await tick();
  assert.equal(c.eq.draft.platform,'air_light_attack');assert.equal(c.eq.draft.source_revision,undefined);assert.deepEqual(plain(c.eq.draft.components),data.platforms[1].default_spec.components);
  assert.equal(c.mount.querySelectorAll('[data-equipment-slot]').length,8);for(const id of ['airframe','airmission','airsystems'])assert(c.mount.querySelector(`[data-equipment-detail="spec:${id}"]`));
  assert.doesNotMatch(c.mount.innerHTML,/Turret &amp; armament|Turret & armament|Interactive ground vehicle model/);assert.match(c.mount.innerHTML,/Ground &amp; aviation/);
  const platform=c.mount.querySelector('#equipmentPlatform');platform.value='air_tactical_strike';platform.onchange();await tick();
  assert.equal(c.eq.draft.platform,'air_tactical_strike');assert.equal(Object.keys(c.eq.draft.components).length,8);assert.equal(c.eq.draft.components.armament,undefined);assert.equal(c.calls.length,0);
});
test('aircraft part inspection focuses its group and payload edits submit all eight slots to a pure fresh review',async()=>{
  const c=fixture(),data=aviationSnapshot();loaded(c,data);c.equipmentChangePlatform('air_light_attack');await tick();
  c.equipmentRender();const model=c.modelMounts[0],requests=c.requests.length,select=c.mount.querySelector('[data-equipment-slot="air_fuel"]');
  c.mount['onequipment-part-select']({detail:{slot:'air_fuel',part:'air_fuel / internal fuel access panels',label:'Internal fuel access panels'}});
  assert.equal(select.parentDetail.open,true);assert.equal(c.document.activeElement,select);assert.equal(model.selectedParts.at(-1),'air_fuel');assert.equal(c.requests.length,requests);
  const pending=[];c.api=(...args)=>new Promise(resolve=>pending.push({args,resolve}));
  let payload=c.mount.querySelector('[data-equipment-slot="air_payload"]');payload.value='air_payload_guided';payload.onchange();
  assert.equal(c.eq.preview,null);assert.equal(c.eq.previewLoading,true);assert.equal(pending[0].args[0],'/api/equipment-preview');assert.equal(pending[0].args[1].components.air_payload,'air_payload_guided');assert.equal(Object.keys(pending[0].args[1].components).length,8);
  payload=c.mount.querySelector('[data-equipment-slot="air_payload"]');payload.value='air_payload_unguided';payload.onchange();
  pending[0].resolve(preview({detail:'Stale guided payload quote'}));await tick();assert.equal(c.eq.preview,null);assert.doesNotMatch(c.mount.innerHTML,/Stale guided payload quote/);
  pending[1].resolve(preview({detail:'Current unguided aircraft review',metrics:[{label:'Sustained sorties',value:'Server-calculated output'}]}));await tick();assert.match(c.mount.innerHTML,/Current unguided aircraft review/);assert.match(c.mount.innerHTML,/Server-calculated output/);assert.equal(c.calls.length,0);assert.equal(c.modelMounts.length,1);
});

test('aviation readiness precedes maintenance, preserves server loadout readings and navigates through captured actions',()=>{
  const c=fixture(),aviation=service({title:'Tactical aviation readiness',status:'Mission stores needed',roles_title:'Aircraft and mission loadouts',roles:[{label:'Strike <model>',value:'3 available · 2.75 supported aircraft equivalents',detail:'Four stores per sortie · supported strike effectiveness from the server.'}],actions:[{label:'Prepare aircraft mission stores',navigate:{action:'equipment',tab:'ammunition'}}]});
  loaded(c,snapshot({aviation,maintenance:service({title:'Maintenance funding'})}));c.equipmentSelectTab('service');
  assert(c.mount.innerHTML.indexOf('Tactical aviation readiness')<c.mount.innerHTML.indexOf('Maintenance funding'));
  for(const text of ['Aircraft and mission loadouts','Strike &lt;model&gt;','2.75 supported aircraft equivalents','Four stores per sortie'])assert(c.mount.innerHTML.includes(text),text);
  assert.equal(c.calls.length,0);const go=c.mount.querySelector('[data-equipment-action="aviation.actions.0"]');assert(go);go.onclick();assert.deepEqual(c.calls,[{action:'equipment',tab:'ammunition'}]);
  c.eq.stale=true;go.onclick();assert.equal(c.calls.length,1);
  c.eq.data.aviation=null;c.eq.stale=false;c.equipmentRender();assert.doesNotMatch(c.mount.innerHTML,/Tactical aviation readiness/);assert.match(c.mount.innerHTML,/Maintenance funding/);
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
  c.LOGI={open:false};c.stock={open:false};c.gov={open:false};c.tech={open:false,byId:new Map([['radar',0]]),data:[{id:'radar',domain:'Computing'}]};
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

test('reviewed retirement reaches the real command bridge and reports immediate fleet changes',async()=>{
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.draft.name='Unfinished design';c.queued=[{kind:'tax',value:20}];
  const current={session_id:'one',player:'USA',receipt:'retired'};let adopted=0;
  c.api=async(...args)=>{c.requests.push(plain(args));return current;};
  c.adopt=async(state,history)=>{assert.equal(state,current);assert.equal(history,false);adopted++;c.S=state;c.equipmentOnStateChanged();};
  const command={kind:'equipment_retire',revision:'fleet-ifv',quantity:2},result=await c.equipmentCommand(command);
  assert.deepEqual(c.requests.filter(row=>row[0]==='/api/command'),[['/api/command',{commands:[command]}]]);assert.equal(adopted,1);
  assert(c.requests.some(row=>row[0]==='/api/equipment?session_id=one'),'Adopting the order refreshes the service overview');
  assert.match(result.message,/Vehicles retired/);assert.doesNotMatch(result.message,/programme will/);
  assert.equal(c.eq.draft.name,'Unfinished design');assert.deepEqual(c.queued,[{kind:'tax',value:20}]);
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

test('production supply precedes project cards and shows server quantities with separate physical units',()=>{
  const c=fixture(),s=supply(),data=snapshot({supply:s,production:[{id:12,name:'Sentinel batch',status:'Producing',supply:supply({stage:'production',title:'Programme supply readiness'})}]});loaded(c,data);const before=plain(data);c.equipmentSelectTab('production');
  const html=c.mount.innerHTML;assert(html.indexOf('Production supply plan')<html.indexOf('Build the model you approved'));
  for(const text of ['Planning date','11 Feb 1990','Remaining work','Next work','Next work gap','National stock','Stock for this review','0.0000456','0.00000047','<small>kt</small>','<small>bcf</small>','does not reserve materials'])assert(html.includes(text),text);
  assert.doesNotMatch(html,/kt\/day|bcf\/day|1\.25 kt\/day|Next-day shortfall/);assert.match(html,/<td>—<\/td>/);assert.deepEqual(plain(data),before);assert.equal(c.requests.length,0);assert.equal(c.calls.length,0);
});

test('per-project supply expands and keeps its exact resource target without placing an order',()=>{
  const c=fixture();loaded(c,snapshot({production:[{id:12,name:'Sentinel batch',status:'Tooling',supply:supply({stage:'tooling',title:'Programme supply readiness',detail:'Tooling uses funding first; fabrication materials follow.'})}]}));c.equipmentSelectTab('production');
  const details=c.mount.querySelector('[data-equipment-detail="supply:production.0.supply"]');assert(details);assert.equal(details.open,false);details.open=true;c.equipmentRender();assert.equal(c.mount.querySelector('[data-equipment-detail="supply:production.0.supply"]').open,true);
  assert.match(c.mount.innerHTML,/Tooling uses funding first/);const link=c.mount.querySelector('[data-equipment-supply-action="production.0.supply.actions.0"]');assert.equal(link.disabled,false);assert.equal(link.onclick(),true);assert.deepEqual(c.calls,[{action:'resources',commodity:'steel'}]);assert.equal(c.requests.length,0);
});

test('optional supply data preserves old responses and explicit empty stages without inventing requirements',()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('production');assert.doesNotMatch(c.mount.innerHTML,/eq-supply-table|Production supply plan|Next work gap/);assert.match(c.mount.innerHTML,/No equipment production is listed/);
  c.eq.data.supply=supply({status:'No production commitments',resources:[],warnings:[],detail:'Certified designs can be scheduled from the Library.'});c.equipmentRender();assert.match(c.mount.innerHTML,/No production commitments/);assert.match(c.mount.innerHTML,/Certified designs can be scheduled/);assert.doesNotMatch(c.mount.innerHTML,/eq-supply-table/);
  const development=c.equipmentSupplyHtml(supply({stage:'development',resources:[],detail:'Development uses funding and engineering time, with no raw-material recipe.'}),'supply');assert.match(development,/no raw-material recipe/);assert.doesNotMatch(development,/eq-supply-table|0 kt|Materials ready/);
});

test('supply navigation rejects stale data, replaced readings, pending orders and command-shaped actions',()=>{
  for(const invalidate of [c=>c.eq.stale=true,c=>c.eq.data=snapshot({supply:supply()}),c=>c.S={session_id:'two',player:'CAN'},c=>c.COMMAND_CHANNEL.pending={}]){
    const c=fixture();loaded(c,snapshot({supply:supply()}));c.equipmentSelectTab('production');const link=c.mount.querySelector('[data-equipment-supply-action="supply.actions.0"]');invalidate(c);assert.equal(link.onclick(),false);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
  }
  const c=fixture();loaded(c,snapshot({supply:supply({actions:[{label:'Buy automatically',command:{kind:'resource_buy'},navigate:{action:'resources',commodity:'steel'}},{label:'Open stock',navigate:{action:'resources',commodity:'gas'}}]})}));c.equipmentSelectTab('production');assert.doesNotMatch(c.mount.innerHTML,/Buy automatically/);assert.equal(c.equipmentSupplyNavigate('supply.actions.0','data',c.eq.data,c.S),false);assert.equal(c.mount.querySelector('[data-equipment-supply-action="supply.actions.1"]').onclick(),true);assert.deepEqual(c.calls,[{action:'resources',commodity:'gas'}]);
});

test('order supply appears before confirmation and invalid orders may review resources without submitting',async()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('library');c.api=async()=>quote({valid:false,supply:supply({stage:'unavailable',title:'Supply before you commit',status:'Order not ready'}),blockers:['Select a working plant.']});c.mount.querySelector('[data-equipment-action="designs.0.actions.0"]').onclick();await tick();
  const html=c.mount.innerHTML;assert(html.indexOf('Supply before you commit')<html.indexOf('data-equipment-intent="0"'));assert.equal(c.mount.querySelector('[data-equipment-intent="0"]').disabled,true);
  const link=c.mount.querySelector('[data-equipment-supply-action="supply.actions.0"]');assert.equal(link.dataset.equipmentScope,'intent');assert.equal(link.disabled,false);assert.equal(link.onclick(),true);assert.deepEqual(c.calls,[{action:'resources',commodity:'steel'}]);assert.equal(c.eq.review.command.kind,'equipment_produce');assert.equal(c.eq.draft.name,'Balanced');
});

test('quoted supply links require the captured quote and current order settings even when campaign data is fresh',async()=>{
  for(const mode of ['loading','error','edit','replacement','dismiss']){
    const c=fixture();loaded(c);c.equipmentSelectTab('library');c.api=async()=>quote({supply:supply({title:'Supply before you commit'})});c.mount.querySelector('[data-equipment-action="designs.0.actions.0"]').onclick();await tick();const link=c.mount.querySelector('[data-equipment-supply-action="supply.actions.0"]');assert.equal(link.disabled,false);
    if(mode==='loading')c.eq.review.loading=true;else if(mode==='error')c.eq.review.error='Review failed';else if(mode==='edit')c.eq.review.command.quantity=9;else if(mode==='replacement')c.eq.review.quote=quote({supply:supply()});else c.eq.review=null;
    assert.equal(link.onclick(),false,mode);assert.equal(c.calls.length,0,mode);if(mode!=='dismiss'){c.equipmentRender();if(mode!=='replacement')assert.equal(c.mount.querySelector('[data-equipment-supply-action="supply.actions.0"]').disabled,true,mode);}
  }
});

test('supply text and attributes escape unsafe server content while small physical requirements remain visible',()=>{
  const c=fixture();loaded(c);const unsafe='\"><img src=x onerror=alert(1)>',s=supply({title:unsafe,status:unsafe,detail:unsafe,warnings:[unsafe],resources:[{commodity:unsafe,name:unsafe,unit:unsafe,detail:unsafe,remaining:.000000000003456,planned_day:0,stock:Infinity,shortfall:NaN}],actions:[{label:unsafe,reason:unsafe,navigate:{action:'resources'}}]});const html=c.equipmentSupplyHtml(s,'supply');
  assert.doesNotMatch(html,/<img|<script|title=""><img/);assert.match(html,/&lt;img/);assert.match(html,/0\.00000000000346/);assert.match(html,/<td>0<\/td>/);assert.match(html,/<td>—<\/td>/);assert.equal(c.calls.length,0);
});

test('commodity navigation reaches the exact resource and preserves unsaved designs while refusing pending navigation',async()=>{
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.draft.name='Uncommitted supply design';c.openStock=id=>c.calls.push(['resources',id]);
  assert.equal(await c.equipmentNavigate({action:'resources',commodity:'steel'}),true);assert.deepEqual(c.calls.at(-1),['resources','steel']);assert.equal(c.eq.draft.name,'Uncommitted supply design');assert.equal(c.requests.length,0);
  const count=c.calls.length;c.COMMAND_CHANNEL.pending={};assert.equal(await c.equipmentNavigate({action:'resources',commodity:'gas'}),false);assert.equal(c.calls.length,count);
});

test('material tables contain horizontal scrolling and retain readable narrow controls',()=>{
  assert.match(css,/\.eq-supply-scroll\s*\{[^}]*max-width:100%[^}]*overflow-x:auto[^}]*overscroll-behavior-x:contain/);
  assert.match(css,/\.eq-supply-table\s*\{[^}]*min-width:590px/);assert.match(css,/@media\(max-width:700px\)[^\n]*\.eq-supply-actions button[^}]*min-height:42px/);
  assert.match(css,/\.eq-supply-scroll:focus-visible/);const c=fixture();loaded(c);assert.match(c.equipmentSupplyHtml(supply(),'supply'),/role="region" aria-label="Production material requirements" tabindex="0"/);
});

test('service overview precedes modernization and displays supplied fleet counts and role capabilities',()=>{
  const c=fixture(),data=snapshot({service:service()});loaded(c,data);const before=plain(data);c.equipmentSelectTab('service');
  const html=c.mount.innerHTML;assert(html.indexOf('Fleet service readiness')<html.indexOf('Suggested modernization'));assert(html.indexOf('Fleet service readiness')<html.indexOf('Equipment in service'));
  for(const text of ['Available vehicles','In refit','Incoming deliveries','Annual maintenance need','$420k','0.742 capability','At currently recorded military support.','Inherited equipment is included'])assert(html.includes(text),text);
  assert.match(html,/<dt>Observation<\/dt><dd>—<small>/);assert.match(html,/<dl><div><dt>Protected mobility/);assert.deepEqual(plain(data),before);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
});

test('service navigation preserves a local design and refuses stale readings or command-shaped shortcuts',()=>{
  const c=fixture();loaded(c,snapshot({service:service({actions:[{label:'Issue order',command:{kind:'equipment_retire',revision:'fleet-ifv',quantity:1},navigate:{action:'budget',ministry:'defense',department:2}},{label:'Review military support',navigate:{action:'budget',ministry:'defense',department:2}}]})}));c.eq.draft.name='Uncommitted Sentinel';c.equipmentSelectTab('service');
  assert.doesNotMatch(c.mount.innerHTML,/Issue order/);assert.equal(c.equipmentServiceNavigate('service.actions.0',c.eq.data,c.S),false);assert.equal(c.mount.querySelector('[data-equipment-service-action="service.actions.1"]').onclick(),true);assert.deepEqual(c.calls,[{action:'budget',ministry:'defense',department:2}]);assert.equal(c.eq.draft.name,'Uncommitted Sentinel');assert.equal(c.requests.length,0);
  for(const change of [c=>c.eq.stale=true,c=>c.eq.data=snapshot({service:service()}),c=>c.eq.loading=true,c=>c.S={session_id:'new',player:'CAN'},c=>c.COMMAND_CHANNEL.pending={},c=>c.eq.data.service.actions[0].enabled=false,c=>c.eq.data.service.actions[0].available=false]){
    const c=fixture();loaded(c,snapshot({service:service()}));c.equipmentSelectTab('service');const button=c.mount.querySelector('[data-equipment-service-action="service.actions.0"]');change(c);assert.equal(button.onclick(),false);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
  }
});

test('retirement effects and permanent loss are reviewed before any command and quantity edits require a new captured quote',async()=>{
  const c=fixture(),data=snapshot();data.service=service();data.lots[0].actions=[retirement()];loaded(c,data);c.eq.draft.name='My unfinished design';c.equipmentSelectTab('service');
  c.api=async(...args)=>{c.requests.push(plain(args));return retirementQuote();};c.mount.querySelector('[data-equipment-action="lots.0.actions.0"]').onclick();await tick();
  assert.equal(c.calls.length,0);assert.equal(c.requests[0][0],'/api/equipment-preview');assert.deepEqual(c.requests[0][1].command,{kind:'equipment_retire',revision:'fleet-ifv',quantity:1});
  const html=c.mount.innerHTML;assert(html.indexOf('Fleet after retirement')<html.indexOf('data-equipment-intent="0"'));for(const text of ['Permanent removal','17 → 16','$420k → $396k','0.742 → 0.719','At unchanged recorded support.','There is no refund','does not reduce the military budget or release cash'])assert(html.includes(text),text);
  const oldConfirm=c.mount.querySelector('[data-equipment-intent="0"]');let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};const input=c.mount.querySelector('[data-equipment-order-input="quantity"]');input.value='2';input.oninput();
  assert.equal(c.requests.at(-1)[1].command.quantity,2);assert.equal(c.equipmentReviewQuoteCurrent(),false);assert.equal(await oldConfirm.onclick(),false);assert.equal(c.calls.length,0);
  const next=retirementQuote({service_effects:service({title:'Fleet after retirement',metrics:[{label:'Server maintenance estimate',value:'$381k'}],roles:[],actions:[]}),actions:[{label:'Confirm retirement',command:{kind:'equipment_retire',revision:'fleet-ifv',quantity:2,quote_token:'reviewed-two'}}]});resolve(next);await tick();
  assert.match(c.mount.innerHTML,/\$381k/);assert.equal(await oldConfirm.onclick(),false,'A retained confirmation cannot accept a replacement quote');assert.equal(c.calls.length,0);const confirm=c.mount.querySelector('[data-equipment-intent="0"]');
  c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await confirm.onclick(),true);assert.deepEqual(c.calls,[next.actions[0].command]);assert.equal(c.eq.draft.name,'My unfinished design');assert.equal(c.eq.tab,'service');await tick();
});

test('invalid retirement reviews keep effects visible but cannot remove vehicles',async()=>{
  const c=fixture(),data=snapshot();data.lots[0].actions=[retirement()];loaded(c,data);c.equipmentSelectTab('service');c.api=async()=>retirementQuote({valid:false,blockers:['Only one available vehicle can be retired.']});c.mount.querySelector('[data-equipment-action="lots.0.actions.0"]').onclick();await tick();
  assert.match(c.mount.innerHTML,/Only one available vehicle can be retired/);assert.match(c.mount.innerHTML,/Fleet after retirement/);assert.equal(c.mount.querySelector('[data-equipment-intent="0"]').disabled,true);assert.equal(await c.mount.querySelector('[data-equipment-intent="0"]').onclick(),false);assert.equal(c.calls.length,0);
});

test('retirement confirmation from a dismissed review cannot confirm another current review',async()=>{
  const c=fixture(),data=snapshot();data.lots[0].actions=[retirement()];loaded(c,data);c.equipmentSelectTab('service');c.api=async()=>retirementQuote();c.mount.querySelector('[data-equipment-action="lots.0.actions.0"]').onclick();await tick();const oldConfirm=c.mount.querySelector('[data-equipment-intent="0"]');
  c.mount.querySelector('[data-equipment-dismiss]').onclick();c.mount.querySelector('[data-equipment-action="lots.0.actions.0"]').onclick();await tick();assert.equal(c.equipmentReviewQuoteCurrent(),true);assert.equal(await oldConfirm.onclick(),false);assert.equal(c.calls.length,0);
});

test('fleet shortcuts switch only recognized equipment tabs while preserving the open room and unsaved design',async()=>{
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.draft.name='Unfinished fleet successor';c.eq.tab='service';c.equipmentRender();const draft=plain(c.eq.draft);
  for(const tab of ['production','library','designer']){assert.equal(await c.equipmentNavigate({action:'equipment',tab}),true);assert.equal(c.eq.tab,tab);assert.equal(c.room.hidden,false);assert.deepEqual(plain(c.eq.draft),draft);}
  const before=c.eq.tab;assert.equal(await c.equipmentNavigate({action:'equipment',tab:'unknown'}),false);assert.equal(c.eq.tab,before);assert.equal(c.room.hidden,false);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
  c.COMMAND_CHANNEL.pending={};assert.equal(await c.equipmentNavigate({action:'equipment',tab:'production'}),false);assert.equal(c.eq.tab,before);assert.deepEqual(plain(c.eq.draft),draft);
});

test('service overview and effects escape labels, role detail and attributes while optional responses stay usable',()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('service');assert.doesNotMatch(c.mount.innerHTML,/eq-service-heading|Fleet service readiness/);assert.match(c.mount.innerHTML,/Equipment in service/);
  const unsafe='"><img src=x onerror=alert(1)>',value=service({title:unsafe,status:unsafe,detail:unsafe,metrics:[{label:unsafe,value:unsafe}],roles:[{label:unsafe,value:unsafe,detail:unsafe}],warnings:[unsafe],actions:[{label:unsafe,reason:unsafe,navigate:{action:'budget',ministry:'defense',department:2}}]});
  for(const effects of [false,true]){const html=c.equipmentServiceHtml(value,effects);assert.doesNotMatch(html,/<img|<script|title=""><img/);assert.match(html,/&lt;img/);if(effects)assert.doesNotMatch(html,/data-equipment-service-action/);}
  assert.equal(c.equipmentServiceHtml(null),'');assert.equal(c.equipmentServiceHtml(undefined,true),'');assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
});

test('maintenance, supply and target confirmations use the real command bridge with accurate outcomes',async()=>{
  for(const [command,message] of [[{kind:'equipment_maintenance',daily_budget_mn:.5},/Maintenance plan saved/],[{kind:'equipment_supply',horizon_days:90,spending_cap_mn:2},/Material purchase completed/],[{kind:'equipment_target',revision:'fleet-ifv',quantity:0},/Fleet target saved/]]){
    const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.draft.name='My unfinished vehicle';
    c.api=async(...args)=>{c.requests.push(plain(args));return {session_id:'one',player:'USA'};};c.adopt=async state=>{c.S=state;};
    const result=await c.equipmentCommand(command);assert.deepEqual(c.requests,[['/api/command',{commands:[command]}]]);assert.match(result.message,message);assert.doesNotMatch(result.message,/programme for progress/);assert.equal(c.eq.draft.name,'My unfinished vehicle');
  }
});

test('maintenance and target planning show authoritative figures without issuing orders on navigation',async()=>{
  const c=fixture(),data=snapshot({maintenance:service({title:'Maintenance and readiness',roles_title:'What affects readiness',metrics:[{label:'Last invoice paid',value:'$41.327k / day'}],actions:[action({label:'Review maintenance limit',command:{kind:'equipment_maintenance',daily_budget_mn:.1},inputs:[{key:'daily_budget_mn',label:'Maximum maintenance payment',type:'number',value:.1,min:0}]})]}),targets:[{id:'target:fleet-ifv',name:'Fleet IFV target',status:'More vehicles needed',metrics:[{label:'Still to order',value:7}],actions:[action({label:'Review fleet target',command:{kind:'equipment_target',revision:'fleet-ifv',quantity:0},inputs:[{key:'quantity',label:'Desired vehicles',type:'number',value:0,min:0,step:1}]})]}]});
  loaded(c,data);c.equipmentSelectTab('service');assert.match(c.mount.innerHTML,/What affects readiness/);assert.match(c.mount.innerHTML,/\$41.327k/);assert.match(c.mount.innerHTML,/Fleet IFV target/);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,1);
  c.api=async(...args)=>{c.requests.push(plain(args));return quote({actions:[],metrics:[]});};c.mount.querySelector('[data-equipment-action="targets.0.actions.0"]').onclick();await tick();assert.equal(c.requests[0][1].command.quantity,0);assert.equal(c.calls.length,0);
});

test('supply horizon keeps its numeric type and editing the cash cap invalidates the previous purchase review',async()=>{
  const c=fixture(),purchase=action({label:'Review material purchase',command:{kind:'equipment_supply',horizon_days:30,spending_cap_mn:1},inputs:[{key:'horizon_days',label:'Supply horizon',type:'select',value:30,options:[{value:30,label:'30 days'},{value:90,label:'90 days'}]},{key:'spending_cap_mn',label:'Maximum purchase spending',type:'number',value:1,min:.000001}]}),data=snapshot({replenishment:service({title:'Purchase missing materials',actions:[purchase]})});
  loaded(c,data);c.equipmentSelectTab('production');assert.equal(c.requests.length,0);
  c.api=async(...args)=>{c.requests.push(plain(args));return quote({actions:[{label:'Confirm material purchase',command:plain(args[1].command)}]});};c.mount.querySelector('[data-equipment-action="replenishment.actions.0"]').onclick();await tick();
  const horizon=c.mount.querySelector('[data-equipment-order-input="horizon_days"]');horizon.value='90';horizon.onchange();await tick();assert.equal(c.requests.at(-1)[1].command.horizon_days,90);
  const old=c.mount.querySelector('[data-equipment-intent="0"]');let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};const cap=c.mount.querySelector('[data-equipment-order-input="spending_cap_mn"]');cap.value='0.25';cap.oninput();
  assert.equal(c.requests.at(-1)[1].command.spending_cap_mn,.25);assert.equal(await old.onclick(),false);assert.equal(c.calls.length,0);
  const command=plain(c.requests.at(-1)[1].command);resolve(quote({actions:[{label:'Confirm material purchase',command}]}));await tick();assert.equal(await old.onclick(),false);c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.mount.querySelector('[data-equipment-intent="0"]').onclick(),true);assert.deepEqual(c.calls,[command]);
});

function ammunitionOrder(extra={}){return action({label:'Review cannon ammunition batch',command:{kind:'equipment_ammo_order',family:'cannon',district:'US-CA',quantity:100,daily_budget_mn:.25},inputs:[{key:'quantity',label:'Rounds to manufacture',type:'number',value:100,min:1,max:10000,step:1},{key:'district',label:'Ammunition plant',type:'select',value:'US-CA',options:[{value:'US-CA',label:'California'},{value:'US-TX',label:'Texas'}]},{key:'daily_budget_mn',label:'Daily funding limit',type:'number',value:.25,min:0}],...extra});}
function ammunition(extra={}){return {overview:service({title:'Physical ammunition reserve',status:'Planning available',detail:'Only completed ammunition is available to the fleet.',metrics:[{label:'Available rounds',value:2475},{label:'Funding department',value:'Defense · Ammunition'}],roles:[],warnings:['A scheduled batch is not available stock.'],actions:[{label:'Review ammunition activation',command:{kind:'equipment_ammo_activate'},requires_preview:true}]}),families:[{id:'autocannon',name:'Autocannon rounds',status:'Stock available',metrics:[{label:'Available rounds',value:2400}],actions:[]},{id:'cannon',name:'Cannon rounds',status:'Supply needed',detail:'Compatible with the selected cannon family.',metrics:[{label:'Available rounds',value:75}],actions:[ammunitionOrder()]}],orders:[{id:'ammo:7',name:'Cannon batch · California',status:'Working',detail:'Funded fabrication is progressing.',progress:.1234,receipt_label:'10 Feb 1990',metrics:[{label:'Completed rounds',value:12}],actions:[{label:'Pause ammunition batch',command:{kind:'equipment_ammo_pause',project:7,paused:true}}]}],...extra};}
function ammunitionReserve(extra={}){return action({label:'Review reserve target',command:{kind:'equipment_ammo_reserve',family:'cannon',target_rounds:5000,district:'US-CA',daily_budget_mn:.125,automatic:false},inputs:[{key:'target_rounds',label:'Reserve target',type:'number',value:5000,min:1,step:1},{key:'district',label:'Production site',type:'select',value:'US-CA',options:[{value:'US-CA',label:'California'},{value:'US-TX',label:'Texas'}]},{key:'daily_budget_mn',label:'Daily funding ceiling',type:'number',value:.125,min:0},{key:'automatic',label:'Replenishment',type:'select',value:false,options:[{value:false,label:'Manual target'},{value:true,label:'Automatic replenishment'}]}],...extra});}
function ammunitionReserves(){return [{id:'reserve:autocannon',name:'Autocannon reserve',status:'Manual target',metrics:[{label:'Reserve target',value:2000}],actions:[]},{id:'reserve:cannon',name:'Cannon reserve',status:'Eligible to replenish',detail:'California is the preferred ammunition production site.',metrics:[{label:'Reserve target',value:5000},{label:'Remaining gap',value:1337}],costs:[{label:'Daily funding ceiling',amount_bn:.000125,period:'Maximum per day'}],receipt_label:'11 Feb 1990',blockers:['Shared funding remains subject to fleet servicing.'],actions:[ammunitionReserve(),{label:'Remove reserve plan',command:{kind:'equipment_ammo_reserve_clear',family:'cannon'},requires_preview:true}]}];}

test('ammunition is a separate keyboard workflow with server figures, action paths and one search',async()=>{
  const c=fixture(),data=snapshot({ammunition:ammunition()}),before=plain(data);loaded(c,data);c.eq.draft.name='Unfinished vehicle';c.equipmentSelectTab('production');
  c.mount.querySelector('[data-equipment-tab="production"]').onkeydown({key:'ArrowRight',preventDefault(){}});assert.equal(c.eq.tab,'ammunition');assert.equal(c.document.activeElement.dataset.equipmentFocus,'tab:ammunition');
  const html=c.mount.innerHTML;assert(html.indexOf('Physical ammunition reserve')<html.indexOf('Ammunition families'));assert(html.indexOf('Ammunition families')<html.indexOf('Ammunition production'));
  for(const text of ['2,475','Defense · Ammunition','Autocannon rounds','Cannon rounds','Completed rounds','12.3% complete','Recorded · 10 Feb 1990'])assert(html.includes(text),text);
  for(const path of ['ammunition.overview.actions.0','ammunition.families.1.actions.0','ammunition.orders.0.actions.0'])assert(c.mount.querySelector(`[data-equipment-action="${path}"]`),path);
  assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,1);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);assert.deepEqual(plain(data),before);
  const search=c.mount.querySelector('#equipmentSearch');search.value='Supply needed';search.oninput();assert.doesNotMatch(c.mount.innerHTML,/Autocannon rounds/);assert(c.mount.querySelector('[data-equipment-action="ammunition.families.1.actions.0"]'));assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,1);
  c.api=async(...args)=>{c.requests.push(plain(args));return quote({actions:[]});};c.mount.querySelector('[data-equipment-action="ammunition.families.1.actions.0"]').onclick();await tick();assert.deepEqual(c.requests[0][1].command,ammunitionOrder().command);assert.equal(c.calls.length,0);
  c.equipmentSelectTab('library');assert.equal(c.eq.query,'');assert.match(c.mount.innerHTML,/Your equipment library/);assert.equal(c.eq.draft.name,'Unfinished vehicle');
});

test('ammunition order edits require a new captured quote before submitting the server command',async()=>{
  const c=fixture(),data=snapshot({ammunition:ammunition({orders:[]})});loaded(c,data);c.equipmentSelectTab('ammunition');c.eq.draft.name='Saved locally only';
  const ammoQuote=command=>quote({detail:'The proposed rounds require paid work and raw inputs.',metrics:[],costs:[{label:'Ammunition fabrication',amount_bn:.00137,period:'One batch'}],timing:[{label:'Minimum work',value:'9 funded days'}],requirements:['Iron and copper must be available.'],service_effects:service({title:'Reserve after this batch completes',metrics:[{label:'Cannon rounds',value:'75 → 175'}],roles:[],warnings:[],actions:[]}),actions:[{label:'Confirm ammunition batch',command}]});
  c.api=async(...args)=>{c.requests.push(plain(args));return ammoQuote(plain(args[1].command));};c.mount.querySelector('[data-equipment-action="ammunition.families.1.actions.0"]').onclick();await tick();
  for(const text of ['Reserve after this batch completes','75 → 175','$1.37m','9 funded days','Iron and copper'])assert(c.mount.innerHTML.includes(text),text);assert.equal(c.calls.length,0);
  const old=c.mount.querySelector('[data-equipment-intent="0"]');let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};const input=c.mount.querySelector('[data-equipment-order-input="quantity"]');input.value='250';input.oninput();
  assert.equal(c.requests.at(-1)[1].command.quantity,250);assert.equal(await old.onclick(),false);assert.equal(c.calls.length,0);
  const reviewed={...plain(c.requests.at(-1)[1].command),quote_token:'fresh-ammunition'};resolve(ammoQuote(reviewed));await tick();assert.equal(await old.onclick(),false);assert.equal(c.calls.length,0);
  c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.mount.querySelector('[data-equipment-intent="0"]').onclick(),true);assert.deepEqual(c.calls,[reviewed]);assert.equal(c.eq.draft.name,'Saved locally only');assert.equal(c.eq.tab,'ammunition');await tick();
});

test('ammunition activation and batch controls remain reviewed and reject stale board actions',async()=>{
  const c=fixture(),data=snapshot({ammunition:ammunition()});loaded(c,data);c.equipmentSelectTab('ammunition');c.api=async(...args)=>{c.requests.push(plain(args));return quote({actions:[]});};
  c.mount.querySelector('[data-equipment-action="ammunition.overview.actions.0"]').onclick();await tick();assert.deepEqual(c.requests[0][1].command,{kind:'equipment_ammo_activate'});assert.equal(c.calls.length,0);
  c.mount.querySelector('[data-equipment-dismiss]').onclick();const pause=c.mount.querySelector('[data-equipment-action="ammunition.orders.0.actions.0"]');pause.onclick();assert.equal(c.calls.length,0);assert.deepEqual(plain(c.eq.review.command),{kind:'equipment_ammo_pause',project:7,paused:true});
  c.mount.querySelector('[data-equipment-dismiss]').onclick();c.eq.stale=true;assert.equal(pause.onclick(),false);assert.equal(c.calls.length,0);
});

test('all ammunition bridge commands report their own result and preserve unrelated drafts and queues',async()=>{
  const cases=[[{kind:'equipment_ammo_order',family:'cannon',district:'US-CA',quantity:100,daily_budget_mn:.25},/Ammunition batch scheduled/],[{kind:'equipment_ammo_activate'},/Physical ammunition management activated/],[{kind:'equipment_ammo_funding',project:7,daily_budget_mn:.1},/Ammunition funding limit saved/],[{kind:'equipment_ammo_pause',project:7,paused:true},/Ammunition batch paused/],[{kind:'equipment_ammo_pause',project:7,paused:false},/Ammunition batch resumed/],[{kind:'equipment_ammo_cancel',project:7},/Remaining ammunition work cancelled/]];
  for(const [command,message] of cases){const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.draft.name='Unfinished ammunition carrier';c.queued=[{kind:'tax',value:20}];c.api=async(...args)=>{c.requests.push(plain(args));return {session_id:'one',player:'USA'};};c.adopt=async state=>{c.S=state;};const result=await c.equipmentCommand(command);assert.deepEqual(c.requests,[['/api/command',{commands:[command]}]]);assert.match(result.message,message);assert.equal(c.eq.draft.name,'Unfinished ammunition carrier');assert.deepEqual(c.queued,[{kind:'tax',value:20}]);}
  const c=shellFixture();loaded(c,snapshot({ammunition:ammunition()}));c.room.hidden=false;assert.equal(await c.equipmentNavigate({action:'equipment',tab:'ammunition'}),true);assert.equal(c.eq.tab,'ammunition');assert.equal(c.room.hidden,false);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
});

test('ammunition empty states, filtering and server text stay usable and escaped',()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('ammunition');assert.match(c.mount.innerHTML,/Ammunition information is not available/);assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,0);
  loaded(c,snapshot({ammunition:ammunition({families:[],orders:[]})}));c.equipmentRender();assert.match(c.mount.innerHTML,/No ammunition families are listed/);assert.match(c.mount.innerHTML,/No ammunition batches are scheduled/);assert.match(c.mount.innerHTML,/No reserve plans saved/);assert.match(c.mount.innerHTML,/Choose an ammunition family below and set a reserve target/);assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,0);
  const unsafe='"><img src=x onerror=alert(1)>',record={id:unsafe,name:unsafe,detail:unsafe,status:unsafe,metrics:[{label:unsafe,value:unsafe}],actions:[{label:unsafe,reason:unsafe,command:{kind:'equipment_ammo_cancel',project:7}}]};loaded(c,snapshot({ammunition:ammunition({overview:service({title:unsafe,roles:[],actions:[]}),reserves:[record],families:[record],orders:[record]})}));c.equipmentRender();assert.doesNotMatch(c.mount.innerHTML,/<img src=x|title=""><img/);assert.match(c.mount.innerHTML,/&lt;img/);
  const search=c.mount.querySelector('#equipmentSearch');search.value='missing family';search.oninput();assert.match(c.mount.innerHTML,/No matches/);assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,1);c.mount.querySelector('[data-equipment-clear]').onclick();assert.equal(c.eq.query,'');assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
});

test('ammunition reserve cards share search counts and retain authoritative values and original action indexes',async()=>{
  const c=fixture(),data=snapshot({ammunition:ammunition({reserves:ammunitionReserves()})}),before=plain(data);loaded(c,data);c.equipmentSelectTab('ammunition');
  const html=c.mount.innerHTML;assert(html.indexOf('Physical ammunition reserve')<html.indexOf('Reserve plans'));assert(html.indexOf('Reserve plans')<html.indexOf('Ammunition families'));
  for(const text of ['5 of 5 shown','1,337','$125k','Recorded · 11 Feb 1990','Shared funding remains subject to fleet servicing.'])assert(html.includes(text),text);
  assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,1);const search=c.mount.querySelector('#equipmentSearch');search.value='Eligible to replenish';search.oninput();
  assert.match(c.mount.innerHTML,/1 of 5 shown/);assert.doesNotMatch(c.mount.innerHTML,/Autocannon reserve/);assert(c.mount.querySelector('[data-equipment-action="ammunition.reserves.1.actions.0"]'));assert(c.mount.querySelector('[data-equipment-action="ammunition.reserves.1.actions.1"]'));assert.equal(c.mount.querySelector('[data-equipment-action="ammunition.reserves.0.actions.0"]'),null);
  assert.deepEqual(plain(data),before);assert.equal(c.requests.length,0);assert.equal(c.calls.length,0);
  c.api=async(...args)=>{c.requests.push(plain(args));return quote({actions:[]});};c.mount.querySelector('[data-equipment-action="ammunition.reserves.1.actions.0"]').onclick();await tick();assert.deepEqual(c.requests[0][1].command,ammunitionReserve().command);assert.equal(c.calls.length,0);
});

test('reserve target and Boolean automation edits invalidate captured quotes before exact confirmation',async()=>{
  const c=fixture(),data=snapshot({ammunition:ammunition({reserves:ammunitionReserves()})});loaded(c,data);c.equipmentSelectTab('ammunition');c.eq.draft.name='Local vehicle draft';
  const reserveQuote=command=>quote({detail:'Only the reviewed reserve policy will be saved.',costs:[{label:'Daily funding ceiling',amount_bn:.000125,period:'Maximum per day'}],metrics:[],requirements:['Replenishment uses available funding after fleet servicing.'],timing:[{label:'First review',value:'Next simulation day'}],service_effects:service({title:'Reserve policy after confirmation',metrics:[{label:'Automatic replenishment',value:command.automatic?'Enabled':'Disabled'}],roles:[],actions:[]}),actions:[{label:'Confirm reserve plan',command}]});
  c.api=async(...args)=>{c.requests.push(plain(args));return reserveQuote(plain(args[1].command));};c.mount.querySelector('[data-equipment-action="ammunition.reserves.1.actions.0"]').onclick();await tick();
  assert.equal(c.requests.at(-1)[1].command.automatic,false);assert.match(c.mount.innerHTML,/Reserve policy after confirmation/);assert.match(c.mount.innerHTML,/Next simulation day/);assert.match(c.mount.innerHTML,/Replenishment uses available funding after fleet servicing/);
  const captured=c.mount.querySelector('[data-equipment-intent="0"]');let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};
  let automatic=c.mount.querySelector('[data-equipment-order-input="automatic"]');automatic.value='true';automatic.onchange();assert.equal(c.requests.at(-1)[1].command.automatic,true);assert.equal(await captured.onclick(),false);assert.equal(c.calls.length,0);
  resolve(reserveQuote(plain(c.requests.at(-1)[1].command)));await tick();assert.equal(await captured.onclick(),false);
  automatic=c.mount.querySelector('[data-equipment-order-input="automatic"]');automatic.value='false';automatic.onchange();assert.equal(c.requests.at(-1)[1].command.automatic,false);resolve(reserveQuote(plain(c.requests.at(-1)[1].command)));await tick();
  const target=c.mount.querySelector('[data-equipment-order-input="target_rounds"]');target.value='7200';target.oninput();assert.equal(c.requests.at(-1)[1].command.target_rounds,7200);assert.equal(await c.equipmentConfirm(0),false);
  const reviewed={...plain(c.requests.at(-1)[1].command),quote_token:'reviewed-reserve'};resolve(reserveQuote(reviewed));await tick();assert.equal(await captured.onclick(),false);
  c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.mount.querySelector('[data-equipment-intent="0"]').onclick(),true);assert.deepEqual(c.calls,[reviewed]);assert.equal(c.eq.draft.name,'Local vehicle draft');await tick();
});

test('reserve removal requires its own fresh quote and stale reserve cards cannot send commands',async()=>{
  const c=fixture(),data=snapshot({ammunition:ammunition({reserves:ammunitionReserves()})});loaded(c,data);c.equipmentSelectTab('ammunition');const clear={kind:'equipment_ammo_reserve_clear',family:'cannon'};
  c.api=async(...args)=>{c.requests.push(plain(args));return quote({detail:'Remove the reserve policy.',actions:[{label:'Confirm removal',command:clear}],service_effects:service({title:'After removing this plan',metrics:[{label:'Reserve policy',value:'Removed'}],roles:[],actions:[]})});};
  const button=c.mount.querySelector('[data-equipment-action="ammunition.reserves.1.actions.1"]');button.onclick();await tick();assert.deepEqual(c.requests[0][1].command,clear);assert.equal(c.calls.length,0);assert.match(c.mount.innerHTML,/After removing this plan/);
  const captured=c.mount.querySelector('[data-equipment-intent="0"]');c.eq.stale=true;assert.equal(await captured.onclick(),false);assert.equal(button.onclick(),false);assert.equal(c.calls.length,0);
  loaded(c,data);c.equipmentRender();c.mount.querySelector('[data-equipment-action="ammunition.reserves.1.actions.1"]').onclick();await tick();c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.equipmentConfirm(0),true);assert.deepEqual(c.calls,[clear]);await tick();
});

test('reserve bridge distinguishes manual, automatic and removal outcomes and returns to ammunition only on success',async()=>{
  const reserve=ammunitionReserve().command,cases=[[reserve,/target saved for manual replenishment/],[{...reserve,automatic:true},/automatic preference saved/],[{kind:'equipment_ammo_reserve_clear',family:'cannon'},/reserve plan removed/]];
  for(const [command,message] of cases){const c=shellFixture();loaded(c,snapshot({ammunition:ammunition()}));c.room.hidden=false;c.eq.draft.name='Unsaved carrier';c.eq.tab='production';c.queued=[{kind:'tax',value:20}];c.api=async(...args)=>{c.requests.push(plain(args));return {session_id:'one',player:'USA'};};c.adopt=async state=>{c.S=state;};const result=await c.equipmentCommand(command);assert.deepEqual(c.requests,[['/api/command',{commands:[command]}]]);assert.match(result.message,message);assert.equal(c.eq.tab,'ammunition');assert.equal(c.eq.draft.name,'Unsaved carrier');assert.deepEqual(c.queued,[{kind:'tax',value:20}]);}
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.tab='production';c.api=async()=>({errors:['The selected site is unavailable.']});await assert.rejects(c.equipmentCommand(reserve),/selected site is unavailable/);assert.equal(c.eq.tab,'production');assert.equal(c.eq.message,'');
});

function supplyPolicyAction(){return action({label:'Review material purchasing policy',command:{kind:'equipment_supply_policy',automatic:false,horizon_days:30,spending_cap_mn:1,cash_floor_mn:0,review_interval_days:7},inputs:[
  {key:'automatic',label:'Automatic purchasing',type:'select',value:false,options:[{value:false,label:'Disabled'},{value:true,label:'Enabled'}]},
  {key:'horizon_days',label:'Supply horizon',type:'select',value:30,options:[{value:30,label:'30 days'},{value:90,label:'90 days'},{value:365,label:'365 days'}]},
  {key:'spending_cap_mn',label:'Spending cap per review',type:'number',value:1,min:0,unit:'$m'},
  {key:'cash_floor_mn',label:'Retained cash floor',type:'number',value:0,min:0,unit:'$m'},
  {key:'review_interval_days',label:'Review interval',type:'select',value:7,options:[{value:1,label:'Daily'},{value:7,label:'Every 7 days'},{value:30,label:'Every 30 days'}]},
]});}
function supplyAutomation(extra={}){return {overview:service({title:'Automatic military material purchasing',status:'Disabled',detail:'Review the purchasing limits before authorizing future purchases.',metrics:[{label:'Next policy review',value:'18 Feb 1990'}],roles:[],actions:[supplyPolicyAction(),{label:'Clear material purchasing policy',command:{kind:'equipment_supply_policy_clear'},requires_preview:true}]}),reviews:Array.from({length:6},(_,index)=>({id:`supply-review:${6-index}`,name:`Policy receipt ${6-index}`,status:index===1?'Deferred':'Purchased',detail:index===1?'Retained cash is at its chosen floor.':'Paid shipments are moving toward the national warehouse.',metrics:[{label:'Quoted deficit',value:1337+index,unit:'t'}],costs:[{label:'Recorded spending',amount_bn:.000125,period:'This review'}],receipt_label:`${11-index} Feb 1990`,actions:[{label:'Review shipment record',navigate:{action:'resources',commodity:'iron'}}]})),...extra};}
function supplyPolicyQuote(command,extra={}){return quote({detail:'Save only the reviewed material purchasing policy.',metrics:[{label:'Future automatic reviews',value:command.automatic?'Enabled':'Disabled'}],costs:[{label:'Saved spending cap',amount_bn:.000375,period:'Per review'}],timing:[{label:'First eligible review',value:'12 Feb 1990'}],requirements:['Purchased materials must arrive before production can consume them.'],service_effects:service({title:'Material supply policy after confirmation',metrics:[{label:'Retained cash floor',value:'$2.25m'}],roles:[],actions:[]}),actions:[{label:command.kind==='equipment_supply_policy_clear'?'Confirm policy removal':'Confirm purchasing policy',command}],...extra});}

test('material purchasing policy precedes manual purchases and shows three recent receipts with preserved history paths',()=>{
  const c=fixture(),data=snapshot({supply_automation:supplyAutomation(),replenishment:service({title:'Manual material purchase',actions:[]})}),before=plain(data);loaded(c,data);c.equipmentSelectTab('production');
  const html=c.mount.innerHTML;assert(html.indexOf('Automatic military material purchasing')<html.indexOf('Manual material purchase'));assert(html.indexOf('Recent purchasing reviews')<html.indexOf('Manual material purchase'));
  const history=c.mount.querySelector('details[data-equipment-detail="supply-automation-history"]');assert(history);assert.equal(history.open,false);assert.match(html,/Show 3 earlier purchasing reviews/);
  const beforeHistory=html.slice(0,html.indexOf('data-equipment-detail="supply-automation-history"'));for(const number of [6,5,4])assert(beforeHistory.includes(`Policy receipt ${number}`));assert(!beforeHistory.includes('Policy receipt 3'));assert.match(html,/Retained cash is at its chosen floor/);assert.match(html,/1,337/);assert.match(html,/\$125k/);
  assert(c.mount.querySelector('[data-equipment-action="supply_automation.overview.actions.0"]'));assert(c.mount.querySelector('[data-equipment-action="supply_automation.reviews.4.actions.0"]'));assert.deepEqual(plain(data),before);assert.equal(c.requests.length,0);assert.equal(c.calls.length,0);
  history.open=true;history.ontoggle();c.equipmentRender();assert.equal(c.mount.querySelector('details[data-equipment-detail="supply-automation-history"]').open,true);c.mount.querySelector('[data-equipment-action="supply_automation.reviews.4.actions.0"]').onclick();assert.deepEqual(c.calls,[{action:'resources',commodity:'iron'}]);assert.equal(c.requests.length,0);
});

test('optional material purchasing data keeps old payloads usable, escapes receipt text and avoids duplicated ammunition history',()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('production');assert.doesNotMatch(c.mount.innerHTML,/Recent purchasing reviews/);assert.match(c.mount.innerHTML,/Build the model you approved/);
  loaded(c,snapshot({supply_automation:supplyAutomation({reviews:[]})}));c.equipmentRender();assert.match(c.mount.innerHTML,/No purchasing reviews recorded yet/);assert.equal(c.mount.querySelector('details[data-equipment-detail="supply-automation-history"]'),null);
  const unsafe='"><img src=x onerror=alert(1)>',automation=supplyAutomation({overview:service({title:unsafe,detail:unsafe,roles:[],actions:[]}),reviews:[{id:unsafe,name:unsafe,status:unsafe,detail:unsafe,receipt_label:unsafe,metrics:[{label:unsafe,value:unsafe}],actions:[{label:unsafe,navigate:{action:'resources',commodity:'iron'}}]}]});
  loaded(c,snapshot({supply_automation:automation,ammunition:ammunition({overview:service({title:'Ammunition supply',roles:[],actions:[{label:'Review material purchasing',navigate:{action:'equipment',tab:'production'}}]})})}));c.equipmentRender();assert.doesNotMatch(c.mount.innerHTML,/<img src=x/);assert.match(c.mount.innerHTML,/&lt;img/);c.equipmentSelectTab('ammunition');assert.doesNotMatch(c.mount.innerHTML,/Recent purchasing reviews/);c.mount.querySelector('[data-equipment-action="ammunition.overview.actions.0"]').onclick();assert.deepEqual(c.calls,[{action:'equipment',tab:'production'}]);assert.equal(c.requests.length,0);
});

test('material purchasing authorization uses exact edited Boolean and numeric values only after a fresh quote',async()=>{
  const c=fixture(),data=snapshot({supply_automation:supplyAutomation({reviews:[]})});loaded(c,data);c.equipmentSelectTab('production');c.eq.draft.name='Unfinished vehicle';
  c.api=async(...args)=>{c.requests.push(plain(args));return supplyPolicyQuote(plain(args[1].command));};c.mount.querySelector('[data-equipment-action="supply_automation.overview.actions.0"]').onclick();await tick();assert.deepEqual(c.requests.at(-1)[1].command,supplyPolicyAction().command);assert.equal(c.calls.length,0);
  const captured=c.mount.querySelector('[data-equipment-intent="0"]');let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};const automatic=c.mount.querySelector('[data-equipment-order-input="automatic"]');automatic.value='true';automatic.onchange();assert.equal(c.requests.at(-1)[1].command.automatic,true);assert.equal(await captured.onclick(),false);assert.equal(await c.equipmentConfirm(0),false);resolve(supplyPolicyQuote(plain(c.requests.at(-1)[1].command)));await tick();
  c.api=async(...args)=>{c.requests.push(plain(args));return supplyPolicyQuote(plain(args[1].command));};
  for(const [key,value,event] of [['horizon_days','90','onchange'],['spending_cap_mn','.375','oninput'],['cash_floor_mn','2.25','oninput'],['review_interval_days','30','onchange']]){const input=c.mount.querySelector(`[data-equipment-order-input="${key}"]`);input.value=value;input[event]();assert.equal(await captured.onclick(),false);await tick();}
  const expected={kind:'equipment_supply_policy',automatic:true,horizon_days:90,spending_cap_mn:.375,cash_floor_mn:2.25,review_interval_days:30};assert.deepEqual(c.requests.at(-1)[1].command,expected);assert.match(c.mount.innerHTML,/Material supply policy after confirmation/);assert.match(c.mount.innerHTML,/\$2.25m/);assert.match(c.mount.innerHTML,/12 Feb 1990/);assert.match(c.mount.innerHTML,/Purchased materials must arrive/);assert.equal(c.calls.length,0);
  c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.mount.querySelector('[data-equipment-intent="0"]').onclick(),true);assert.deepEqual(c.calls,[expected]);assert.equal(c.eq.draft.name,'Unfinished vehicle');await tick();
});

test('material purchasing reviews reject stale or pending confirmations and invalid authorization quotes',async()=>{
  for(const invalidate of [c=>c.eq.stale=true,c=>c.eq.data=snapshot(),c=>c.COMMAND_CHANNEL.pending={},c=>c.advancing=true]){const c=fixture();loaded(c,snapshot({supply_automation:supplyAutomation()}));c.equipmentSelectTab('production');c.api=async(...args)=>supplyPolicyQuote(plain(args[1].command));const button=c.mount.querySelector('[data-equipment-action="supply_automation.overview.actions.0"]');button.onclick();await tick();const captured=c.mount.querySelector('[data-equipment-intent="0"]');invalidate(c);assert.equal(await captured.onclick(),false);assert.equal(button.onclick(),false);assert.equal(c.calls.length,0);}
  const c=fixture();loaded(c,snapshot({supply_automation:supplyAutomation()}));c.equipmentSelectTab('production');c.api=async(...args)=>supplyPolicyQuote(plain(args[1].command),{valid:false,blockers:['The purchasing cash account is unavailable.']});c.mount.querySelector('[data-equipment-action="supply_automation.overview.actions.0"]').onclick();await tick();assert.match(c.mount.innerHTML,/purchasing cash account is unavailable/);assert.equal(await c.equipmentConfirm(0),false);assert.equal(c.calls.length,0);
});

test('clearing the material purchasing policy requires a fresh explicit review',async()=>{
  const c=fixture(),data=snapshot({supply_automation:supplyAutomation()});loaded(c,data);c.equipmentSelectTab('production');const command={kind:'equipment_supply_policy_clear'};c.api=async(...args)=>{c.requests.push(plain(args));return supplyPolicyQuote(plain(args[1].command),{detail:'Remove future purchasing authorization. Existing shipments remain.'});};
  c.mount.querySelector('[data-equipment-action="supply_automation.overview.actions.1"]').onclick();await tick();assert.deepEqual(c.requests[0][1].command,command);assert.equal(c.calls.length,0);assert.match(c.mount.innerHTML,/Existing shipments remain/);const captured=c.mount.querySelector('[data-equipment-intent="0"]');c.mount.querySelector('[data-equipment-dismiss]').onclick();assert.equal(await captured.onclick(),false);assert.equal(c.calls.length,0);
  c.mount.querySelector('[data-equipment-action="supply_automation.overview.actions.1"]').onclick();await tick();c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.equipmentConfirm(0),true);assert.deepEqual(c.calls,[command]);await tick();
});

test('material purchasing bridge distinguishes disabled, enabled and cleared policies and returns to Production',async()=>{
  const command=supplyPolicyAction().command,cases=[[command,/automatic purchases disabled/],[{...command,automatic:true},/Automatic material purchasing enabled/],[{kind:'equipment_supply_policy_clear'},/Material purchasing policy cleared/]];
  for(const [order,message] of cases){const c=shellFixture();loaded(c,snapshot({supply_automation:supplyAutomation()}));c.room.hidden=false;c.eq.tab='ammunition';c.eq.draft.name='Local tank';c.queued=[{kind:'tax',value:20}];c.api=async(...args)=>{c.requests.push(plain(args));return {session_id:'one',player:'USA'};};c.adopt=async state=>{c.S=state;};const result=await c.equipmentCommand(order);assert.deepEqual(c.requests,[['/api/command',{commands:[order]}]]);assert.match(result.message,message);assert.equal(c.eq.tab,'production');assert.equal(c.eq.draft.name,'Local tank');assert.deepEqual(c.queued,[{kind:'tax',value:20}]);}
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.tab='ammunition';c.api=async()=>({errors:['Cash floor must be nonnegative.']});await assert.rejects(c.equipmentCommand(command),/Cash floor must be nonnegative/);assert.equal(c.eq.tab,'ammunition');
});

function establishCompanyAction(){return {label:'Establish a manufacturer',command:{kind:'company_establish',name:'National Armor',district:'US-TX',capital_mn:50},requires_preview:true,inputs:[
  {key:'name',label:'Company name',type:'text',value:'National Armor',maxlength:80,detail:'A player-established state manufacturer.'},
  {key:'district',label:'Arms Plant',type:'select',value:'US-TX',options:[{value:'US-TX',label:'Texas',detail:'One available Arms Plant slot.'},{value:'US-CA',label:'California',detail:'Shared facilities must be available.'}]},
  {key:'capital_mn',label:'Initial working capital',type:'number',unit:'$m',value:50,min:1,step:1,detail:'Paid once into the separate company account.'}
]};}
function companyPurchaseAction(quantity=2){return {label:'Review purchase',command:{kind:'company_purchase',company:1,product:3,quantity},requires_preview:true,inputs:[{key:'quantity',label:'Vehicles to buy',type:'number',value:quantity,min:1,max:8,step:1}]};}
function companies(extra={}){return {overview:service({title:'Domestic manufacturers',status:'One state company',detail:'Manufacturers develop your exact revision and build their own stock.',roles:[],metrics:[{label:'Company cash',value:'$24m'},{label:'Available stock',value:8},{label:'Paid deliveries',value:2}],actions:[establishCompanyAction()]}),
  firms:[{id:'firm:1',name:'National Armor',status:'Operating',detail:'State owned · Texas Arms Plant',metrics:[{label:'Company working capital',value:'$24m'}],costs:[{label:'Initial paid capitalization',amount_bn:.05}],requirements:['The facility slot is reserved for company work.'],actions:[{label:'Add working capital',command:{kind:'company_capitalize',company:1,amount_mn:10},requires_preview:true,inputs:[{key:'amount_mn',label:'Working capital transfer',type:'number',value:10,min:1}]}]}],
  products:[{id:'product:1:3',name:'Sentinel 90',supplier_name:'National Armor',status:'In stock',phase:'stock',detail:'Certified revision. These unsold vehicles are company property.',source_revision:7,spec:{name:'Sentinel 90',...spec()},availability:{ready_stock:8,unit_price_bn:.0045,delivery_days:7,fleet_need:18,maintenance_bn_per_year:.000016,restock:'5 more vehicles in 20 days if supplied.'},milestones:[{label:'Certification',value:'8 Feb 1990',detail:'Engineering and trials complete.'},{label:'First stock',value:'11 Feb 1990'}],actions:[companyPurchaseAction()]},
    {id:'product:1:4',name:'Sentinel Scout',supplier_name:'National Armor',status:'In development',phase:'development',detail:'A separate frozen specification.',progress:.5,spec:{name:'Sentinel Scout',...spec(),components:{...spec().components,engine:'engine-light'}},availability:{ready_stock:0,unit_price_bn:null,restock:'Awaiting certification and funded company tooling.'},milestones:[{label:'Engineering and trials',value:'30 days remaining',detail:'At the current daily development ceiling.'}],blockers:['First stock depends on certification and company working capital.'],actions:[]}],
  deliveries:[{id:'delivery:1',name:'Sentinel 90 · 2 vehicles',status:'In transit',phase:'delivery',detail:'Purchased stock awaits arrival before it becomes usable.',receipt_label:'11 Feb 1990 · $9m paid',metrics:[{label:'Arrival',value:'18 Feb 1990'}],actions:[]}],...extra};}
function companyQuote(command,extra={}){return quote({detail:'The government buys only the selected finished vehicles. Arrival adds them to service.',metrics:[{label:'Vehicles purchased',value:command.quantity??0},{label:'Company stock after purchase',value:command.quantity==null?8:8-command.quantity}],costs:[{label:'Purchase total',amount_bn:.009},{label:'Maintenance after delivery',amount_bn:.000032,period:'Per year'}],timing:[{label:'Delivery',value:'18 Feb 1990'}],service_effects:service({title:'Fleet after delivery',metrics:[{label:'Available tanks',value:'10 → 12'},{label:'Remaining fleet need',value:'18 → 16'}],roles:[],actions:[]}),actions:[{label:'Confirm purchase',command:{...command,quote:'company-quote'},enabled:true}],...extra});}

test('companies explain separate ownership and show server-priced stock, delivery and fleet need without issuing orders',()=>{
  const c=fixture(),data=snapshot({companies:companies()});loaded(c,data);const before=plain(data);c.equipmentSelectTab('companies');
  for(const text of ['Companies &amp; Procurement','Design it. Commission it. Buy it.','Fund engineering &amp; trials','Company cash &amp; facilities','Available to buy','Price per vehicle','$4.5m','Remaining fleet need','After owned stock and paid deliveries','$16k/year','18 Feb 1990','Company inventory remains the supplier’s property','does not authorize spending','Existing public development and production'])assert(c.mount.innerHTML.includes(text),text);
  assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,1);assert.deepEqual(plain(data),before);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
  const search=c.mount.querySelector('#equipmentSearch');search.value='Scout';search.oninput();assert.match(c.mount.innerHTML,/1 of 4 shown/);assert.equal(c.mount.querySelector('[data-equipment-action="companies.products.0.actions.0"]'),null);assert.match(c.mount.innerHTML,/30 days remaining/);assert.match(c.mount.innerHTML,/First stock depends on certification/);
});

test('company model inspection uses frozen specifications while preserving an unfinished design and one live renderer',()=>{
  const c=fixture();loaded(c,snapshot({companies:companies()}));c.eq.draft.name='My unfinished tank';const draft=plain(c.eq.draft);c.equipmentRender();const controller=c.modelMounts[0];
  c.equipmentSelectTab('companies');assert.equal(c.modelMounts.length,1);assert.equal(controller.disposals,0);assert.equal(controller.updates.at(-1).name,'Sentinel 90');assert.match(c.mount.innerHTML,/Specifications of this exact model/);
  assert.equal(c.mount.querySelector('[data-equipment-company-model="1"]').onclick(),true);assert.equal(controller.updates.at(-1).components.engine,'engine-light');assert.deepEqual(plain(c.eq.draft),draft);assert.equal(c.eq.companyProduct,'product:1:4');
  c.equipmentRender();assert.equal(c.modelMounts.length,1);c.equipmentSelectTab('designer');assert.deepEqual(controller.updates.at(-1),draft);c.equipmentSelectTab('library');assert.equal(controller.disposals,1);assert.equal(c.calls.length,0);
});

test('company establishment name, facility and paid capital remain editable and are sent only after a fresh review',async()=>{
  const c=fixture(),data=snapshot({companies:companies({firms:[],products:[],deliveries:[]})});loaded(c,data);c.equipmentSelectTab('companies');assert.match(c.mount.innerHTML,/Establish your first manufacturer/);assert.match(c.mount.innerHTML,/paid working capital and rights to an available Arms Plant/);
  c.api=async(...args)=>{c.requests.push(plain(args));return companyQuote(plain(args[1].command),{detail:'The capitalization is a paid transfer into separate company cash.'});};c.mount.querySelector('[data-equipment-action="companies.overview.actions.0"]').onclick();await tick();assert.equal(c.calls.length,0);
  const oldConfirm=c.mount.querySelector('[data-equipment-intent="0"]'),name=c.mount.querySelector('[data-equipment-order-input="name"]');assert.equal(name.attributes.type,'text');assert.equal(name.attributes.maxlength,'80');name.value='Armor & Mobility 1990';name.oninput();assert.equal(c.requests.at(-1)[1].command.name,'Armor & Mobility 1990');assert.equal(await oldConfirm.onclick(),false);await tick();
  const site=c.mount.querySelector('[data-equipment-order-input="district"]');site.value='US-CA';site.onchange();await tick();assert.match(c.mount.innerHTML,/Shared facilities must be available/);const capital=c.mount.querySelector('[data-equipment-order-input="capital_mn"]');capital.value='37.5';capital.oninput();await tick();
  const expected={kind:'company_establish',name:'Armor & Mobility 1990',district:'US-CA',capital_mn:37.5};assert.deepEqual(c.requests.at(-1)[1].command,expected);assert.match(c.mount.innerHTML,/paid transfer into separate company cash/);assert.equal(c.calls.length,0);c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.equipmentConfirm(0),true);assert.deepEqual(c.calls,[{...expected,quote:'company-quote'}]);await tick();
});

test('manufacturer selection keeps numeric identity and quotes development separately from stock purchases',async()=>{
  const c=fixture();loaded(c,snapshot({companies:companies()}));const action={label:'Choose manufacturer and develop',command:{kind:'company_develop',company:1,name:'Sentinel',...spec(),daily_budget_mn:1,stock_target:8},requires_preview:true,inputs:[{key:'company',label:'Manufacturer',type:'select',value:1,options:[{value:1,label:'National Armor',detail:'Domestic tank development.'},{value:2,label:'Other supplier',enabled:false,reason:'No available facility rights.'}]},{key:'daily_budget_mn',label:'Development ceiling',type:'number',value:1,min:0},{key:'stock_target',label:'Company stock target',type:'number',value:8,min:0,max:24,step:1}]};c.eq.preview=preview({actions:[action]});c.equipmentRender();
  c.api=async(...args)=>{c.requests.push(plain(args));return companyQuote(plain(args[1].command),{detail:'Development fees fund engineering and trials. Saleable vehicles are bought separately.'});};c.mount.querySelector('[data-equipment-action="actions.0"]').onclick();await tick();assert.equal(c.requests[0][1].command.company,1);assert.equal(typeof c.requests[0][1].command.company,'number');assert.match(c.mount.innerHTML,/No available facility rights/);assert.match(c.mount.innerHTML,/Saleable vehicles are bought separately/);
  const supplier=c.mount.querySelector('[data-equipment-order-input="company"]');supplier.value='2';supplier.onchange();assert.equal(c.requests.length,1);const stock=c.mount.querySelector('[data-equipment-order-input="stock_target"]');stock.value='12';stock.oninput();await tick();assert.equal(c.requests.at(-1)[1].command.stock_target,12);assert.equal(c.calls.length,0);
});

test('company purchase quantity changes invalidate the old sale quote and show delivery and upkeep before confirmation',async()=>{
  const c=fixture(),data=snapshot({companies:companies()});loaded(c,data);c.equipmentSelectTab('companies');c.api=async(...args)=>{c.requests.push(plain(args));return companyQuote(plain(args[1].command));};c.mount.querySelector('[data-equipment-action="companies.products.0.actions.0"]').onclick();await tick();
  const captured=c.mount.querySelector('[data-equipment-intent="0"]');assert.match(c.mount.innerHTML,/Fleet after delivery/);assert.match(c.mount.innerHTML,/10 → 12/);assert.match(c.mount.innerHTML,/Maintenance after delivery/);assert.match(c.mount.innerHTML,/18 Feb 1990/);assert.equal(c.calls.length,0);
  let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};const quantity=c.mount.querySelector('[data-equipment-order-input="quantity"]');assert.equal(quantity.attributes.max,'8');quantity.value='5';quantity.oninput();assert.equal(await captured.onclick(),false);assert.equal(await c.equipmentConfirm(0),false);const command=plain(c.requests.at(-1)[1].command);resolve(companyQuote(command,{costs:[{label:'Purchase total',amount_bn:.0225}]}));await tick();assert.match(c.mount.innerHTML,/\$22.5m/);assert.deepEqual(command,{kind:'company_purchase',company:1,product:3,quantity:5});c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.equipmentConfirm(0),true);assert.deepEqual(c.calls,[{...command,quote:'company-quote'}]);await tick();
});

test('company purchases cannot commit insufficient stock or funding and old inputs cannot alter another review',async()=>{
  const c=fixture();loaded(c,snapshot({companies:companies()}));c.equipmentSelectTab('companies');c.api=async(...args)=>companyQuote(plain(args[1].command),{valid:false,blockers:['Only 1 finished vehicle remains.','Defense procurement authority is insufficient.']});c.mount.querySelector('[data-equipment-action="companies.products.0.actions.0"]').onclick();await tick();assert.match(c.mount.innerHTML,/Only 1 finished vehicle remains/);assert.match(c.mount.innerHTML,/procurement authority is insufficient/);assert.equal(await c.equipmentConfirm(0),false);
  const oldInput=c.mount.querySelector('[data-equipment-order-input="quantity"]');c.mount.querySelector('[data-equipment-dismiss]').onclick();c.mount.querySelector('[data-equipment-action="companies.products.0.actions.0"]').onclick();await tick();oldInput.value='7';oldInput.oninput();assert.equal(c.eq.review.command.quantity,2);assert.equal(c.calls.length,0);
});

test('company buttons and reviewed purchase reject replaced state, old campaigns and pending orders',async()=>{
  for(const invalidate of [c=>c.eq.stale=true,c=>c.eq.data=snapshot(),c=>c.S={session_id:'two',player:'FRA'},c=>c.COMMAND_CHANNEL.pending={},c=>c.advancing=true]){const c=fixture();loaded(c,snapshot({companies:companies()}));c.equipmentSelectTab('companies');c.api=async(...args)=>companyQuote(plain(args[1].command));const purchase=c.mount.querySelector('[data-equipment-action="companies.products.0.actions.0"]'),inspect=c.mount.querySelector('[data-equipment-company-model="1"]');purchase.onclick();await tick();const confirm=c.mount.querySelector('[data-equipment-intent="0"]');invalidate(c);assert.equal(await confirm.onclick(),false);assert.equal(purchase.onclick(),false);assert.equal(inspect.onclick(),false);assert.equal(c.calls.length,0);}
});

test('company renderers escape all supplier, specification, milestone and offer text and tolerate old payloads',()=>{
  const c=fixture();loaded(c);c.equipmentSelectTab('companies');assert.match(c.mount.innerHTML,/Company information is not available/);const unsafe='"><img src=x onerror=alert(1)>',row={id:unsafe,name:unsafe,status:unsafe,supplier_name:unsafe,phase:unsafe,detail:unsafe,spec:{platform:'mbt',components:{[unsafe]:unsafe}},availability:{ready_stock:null,unit_price_bn:null,restock:unsafe},milestones:[{label:unsafe,value:unsafe,detail:unsafe}],actions:[{label:unsafe,command:{kind:'company_purchase'}}]};loaded(c,snapshot({companies:companies({products:[row]})}));c.equipmentRender();assert.doesNotMatch(c.mount.innerHTML,/<img src=x/);assert.match(c.mount.innerHTML,/&lt;img/);assert.match(c.mount.innerHTML,/data-company-phase="other"/);assert.doesNotMatch(c.mount.innerHTML,/Price per vehicle/);assert.equal(c.calls.length,0);
});

test('company command bridge uses the existing transaction channel and returns to procurement only after success',async()=>{
  const cases=[['company_establish',/Manufacturer established/],['company_capitalize',/Company funding recorded/],['company_develop',/Development contract commissioned/],['company_purchase',/Paid|paid vehicles/],['company_funding',/Development funding limit saved/],['company_inventory',/places no government purchase/],['company_cancel',/cancellation recorded/]];
  for(const [kind,message] of cases){const c=shellFixture();loaded(c,snapshot({companies:companies()}));c.room.hidden=false;c.eq.tab='designer';c.eq.draft.name='Local draft';c.queued=[{kind:'tax',value:20}];c.api=async(...args)=>{c.requests.push(plain(args));return {session_id:'one',player:'USA'};};c.adopt=async state=>{c.S=state;};const command={kind,company:1,product:3,quantity:2,quote:'kept-verbatim'},result=await c.equipmentCommand(command);assert.deepEqual(c.requests,[['/api/command',{commands:[command]}]]);assert.match(result.message,message);assert.equal(c.eq.tab,'companies');assert.equal(c.eq.draft.name,'Local draft');assert.deepEqual(c.queued,[{kind:'tax',value:20}]);}
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.tab='designer';c.api=async()=>({errors:['Stock changed. Review a fresh purchase quote.']});await assert.rejects(c.equipmentCommand({kind:'company_purchase'}),/Stock changed/);assert.equal(c.eq.tab,'designer');
});

test('companies navigation stays in the equipment room, preserves drafts and has visible entry points',async()=>{
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.draft.name='Tank draft';assert.equal(await c.equipmentNavigate({action:'equipment',tab:'companies'}),true);assert.equal(c.eq.tab,'companies');assert.equal(c.eq.draft.name,'Tank draft');assert.equal(c.room.hidden,false);assert.equal(c.calls.length,0);assert.match(page,/data-equipment-companies><b>Companies &amp; Procurement/);assert.match(css,/eq-company-intro[\s\S]*production-v1\.webp/);
});

test('a tank draft survives the no-manufacturer route and returns to the same editable specifications',()=>{
  const c=shellFixture();loaded(c,snapshot({companies:companies({firms:[],products:[],deliveries:[]})}));c.room.hidden=false;c.eq.draft.name='Unfinished national tank';c.eq.draft.components.engine='unsaved-engine';c.eq.preview=preview({actions:[{label:'Establish a tank manufacturer',navigate:{action:'equipment',tab:'companies'},detail:'Save this draft, then establish a funded contractor.'}]});c.eq.previewKey=c.equipmentDraftKey();const draft=plain(c.eq.draft);c.equipmentRender();
  c.mount.querySelector('[data-equipment-action="actions.0"]').onclick();assert.equal(c.eq.tab,'companies');assert.match(c.mount.innerHTML,/Establish your first manufacturer/);assert.deepEqual(plain(c.eq.draft),draft);assert.equal(c.requests.length,0);assert.equal(c.calls.length,0);c.equipmentSelectTab('designer');assert.equal(c.mount.querySelector('#equipmentName').value,draft.name);assert.deepEqual(plain(c.eq.draft.components),draft.components);
});

test('establishment with no eligible plant shows an explained unavailable select and keeps the server refusal visible',async()=>{
  const c=fixture(),action=establishCompanyAction();action.command.district='';action.inputs[1]={...action.inputs[1],value:'',options:[]};loaded(c,snapshot({companies:companies({overview:service({title:'Domestic manufacturers',actions:[action]}),firms:[],products:[],deliveries:[]})}));c.equipmentSelectTab('companies');c.api=async(...args)=>companyQuote(plain(args[1].command),{valid:false,blockers:['Build a completed Arms Plant with an available slot first.']});c.mount.querySelector('[data-equipment-action="companies.overview.actions.0"]').onclick();await tick();const select=c.mount.querySelector('[data-equipment-order-input="district"]');assert.equal(select.disabled,true);assert.equal(select.attributes['aria-describedby'],'equipment-order-note-district');assert.match(c.mount.innerHTML,/No available choices/);assert.match(c.mount.innerHTML,/No choices are available in this reading/);assert.match(c.mount.innerHTML,/Build a completed Arms Plant/);assert.equal(await c.equipmentConfirm(0),false);assert.equal(c.calls.length,0);
});

function mixedCompanySnapshot(){
  const data=aviationSnapshot(),board=companies(),{build}=require('../../spheres-web/ui/equipment-mesh.js'),ground=build({platform:'ground_apc'}).specification;
  data.platforms.push({id:'ground_apc',name:'Armored personnel carrier',family:'ground_apc',slots:Object.keys(ground.components).map(id=>({id,name:id.replaceAll('_',' ')})),default_spec:ground});
  for(const id of Object.values(ground.components))if(!data.components.some(c=>c.id===id))data.components.push({id,name:id.replaceAll('_',' '),known:true});
  board.products.push({id:'product:1:10',name:'Guardian APC',supplier_name:'National Armor',family:'ground',unit_label:'vehicle',status:'Building company stock',phase:'manufacturing',spec:{...ground,name:'Guardian APC'},availability:{ready_stock:0,unit_price_bn:.0027},actions:[]});
  for(const [id,platform,name,price] of [[11,'air_light_attack','Swift light attack',.012],[12,'air_tactical_strike','Falcon tactical strike',.032]]){
    const spec={...data.platforms.find(p=>p.id===platform).default_spec,name};
    board.products.push({id:`product:1:${id}`,name,supplier_name:'National Armor',family:'aircraft',unit_label:'aircraft',platform_name:data.platforms.find(p=>p.id===platform).name,source_revision:`USA-design-${id}`,status:'In stock',phase:'stock',spec,availability:{ready_stock:2,unit_price_bn:price,delivery_days:7,maintenance_bn_per_year:.00021,fleet_need:4},actions:[{...companyPurchaseAction(1),command:{kind:'company_purchase',company:1,product:id,quantity:1}}]});
  }
  board.deliveries[0].name='Existing ground delivery';data.companies=board;return data;
}

test('company family filters use supplied categories and keep accounts and paid deliveries visible',()=>{
  const c=fixture(),data=mixedCompanySnapshot();loaded(c,data);const before=plain(data);c.equipmentSelectTab('companies');assert.match(c.mount.innerHTML,/Ground &amp; aircraft procurement/);assert.equal(c.mount.querySelectorAll('[data-equipment-company-family]').length,3);assert.match(c.mount.innerHTML,/7 of 7 shown/);
  c.mount.querySelector('[data-equipment-company-family="aircraft"]').onclick();assert.equal(c.eq.companyFamily,'aircraft');assert.match(c.mount.innerHTML,/4 of 7 shown/);assert.match(c.mount.innerHTML,/Swift light attack/);assert.match(c.mount.innerHTML,/Falcon tactical strike/);assert.doesNotMatch(c.mount.innerHTML,/Guardian APC|Sentinel Scout/);assert.match(c.mount.innerHTML,/Existing ground delivery/);assert.match(c.mount.innerHTML,/Company working capital/);assert.match(c.mount.innerHTML,/Price per aircraft/);assert.match(c.mount.innerHTML,/Maintenance per aircraft/);assert.equal(c.mount.querySelector('[data-equipment-company-family="aircraft"]').attributes['aria-pressed'],'true');assert.equal(c.document.activeElement.dataset.equipmentFocus,'company-family:aircraft');
  c.mount.querySelector('[data-equipment-company-family="ground"]').onclick();assert.match(c.mount.innerHTML,/5 of 7 shown/);assert.match(c.mount.innerHTML,/Guardian APC/);assert.doesNotMatch(c.mount.innerHTML,/Swift light attack|Falcon tactical strike/);assert.deepEqual(plain(data),before);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
});

test('mixed supplier 3D inspection retains exact aircraft and specialist specs without touching the local tank draft',()=>{
  const c=fixture(),data=mixedCompanySnapshot();loaded(c,data);c.eq.draft.name='Unfinished tank';const draft=plain(c.eq.draft);c.equipmentRender();const controller=c.modelMounts[0];c.equipmentSelectTab('companies');c.equipmentSetCompanyFamily('aircraft');const light=controller.updates.at(-1);assert.equal(light.platform,'air_light_attack');assert.equal(Object.keys(light.components).length,8);assert.equal(light.components.armament,undefined);assert.equal(light.components.turret,undefined);
  c.mount.querySelector('[data-equipment-company-model="4"]').onclick();assert.equal(controller.updates.at(-1).platform,'air_tactical_strike');assert.deepEqual(controller.updates.at(-1).components,plain(data.companies.products[4].spec.components));assert.equal(c.modelMounts.length,1);assert.match(c.mount.innerHTML,/USA-design-12/);
  c.equipmentSetCompanyFamily('ground');c.mount.querySelector('[data-equipment-company-model="2"]').onclick();assert.equal(controller.updates.at(-1).platform,'ground_apc');assert.equal(Object.keys(controller.updates.at(-1).components).length,13);assert.equal(controller.updates.at(-1).components.air_payload,undefined);assert.deepEqual(plain(c.eq.draft),draft);c.equipmentSelectTab('designer');assert.deepEqual(controller.updates.at(-1),draft);assert.equal(c.calls.length,0);
});

test('filtered aircraft purchases retain original action identity and require a new review after changing model family',async()=>{
  const c=fixture(),data=mixedCompanySnapshot();loaded(c,data);c.equipmentSelectTab('companies');c.equipmentSetCompanyFamily('aircraft');c.api=async(...args)=>{c.requests.push(plain(args));return companyQuote(plain(args[1].command),{metrics:[{label:'Exact model',value:'Swift light attack'}],costs:[{label:'Total aircraft purchase',amount_bn:.012}]});};const purchase=c.mount.querySelector('[data-equipment-action="companies.products.3.actions.0"]');assert(purchase);purchase.onclick();await tick();assert.deepEqual(c.requests[0][1].command,{kind:'company_purchase',company:1,product:11,quantity:1});assert.match(c.mount.innerHTML,/Total aircraft purchase/);const old=c.mount.querySelector('[data-equipment-intent="0"]');c.equipmentSetCompanyFamily('ground');assert.equal(c.eq.review,null);assert.equal(await old.onclick(),false);assert.equal(c.calls.length,0);
  c.equipmentSetCompanyFamily('aircraft');c.mount.querySelector('[data-equipment-action="companies.products.4.actions.0"]').onclick();await tick();const command={kind:'company_purchase',company:1,product:12,quantity:1,quote:'company-quote'};c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.equipmentConfirm(0),true);assert.deepEqual(c.calls,[command]);await tick();
});

test('company search combines with family filtering while empty results remove a previous 3D model and preserve the draft',()=>{
  const c=fixture();loaded(c,mixedCompanySnapshot());c.eq.draft.name='Local configuration';const draft=plain(c.eq.draft);c.equipmentSelectTab('companies');c.equipmentSetCompanyFamily('aircraft');const controller=c.modelMounts[0],search=c.mount.querySelector('#equipmentSearch');search.value='Light attack aircraft';search.oninput();assert.match(c.mount.innerHTML,/1 of 7 shown/);assert(c.mount.querySelector('[data-equipment-action="companies.products.3.actions.0"]'));assert.equal(c.mount.querySelector('[data-equipment-action="companies.products.4.actions.0"]'),null);
  c.mount.querySelector('#equipmentSearch').value='no such model';c.mount.querySelector('#equipmentSearch').oninput();assert.match(c.mount.innerHTML,/No matches/);assert.equal(c.mount.querySelector('[data-equipment-model]'),null);assert.equal(controller.disposals,1);c.mount.querySelector('[data-equipment-clear]').onclick();assert.equal(c.eq.companyFamily,'aircraft');assert.equal(c.modelMounts.length,2);assert.deepEqual(plain(c.eq.draft),draft);assert.equal(c.calls.length,0);
});

test('family fallback requires known catalogue metadata and campaign changes reset the filter and selected model',()=>{
  const c=fixture(),data=mixedCompanySnapshot();delete data.companies.products[3].family;delete data.companies.products[4].family;data.companies.products.push({id:'unknown',name:'Aircraft with no supported platform',spec:{platform:'air_future_unknown',components:{}},actions:[]});loaded(c,data);c.equipmentSelectTab('companies');assert.equal(c.equipmentCompanyFamily(data.companies.products[3]),'aircraft');assert.equal(c.equipmentCompanyFamily(data.companies.products[0]),'ground');assert.equal(c.equipmentCompanyFamily(data.companies.products.at(-1)),null);
  c.equipmentSetCompanyFamily('aircraft');assert.doesNotMatch(c.mount.innerHTML,/Aircraft with no supported platform/);assert.match(c.mount.innerHTML,/Swift light attack/);assert.equal(c.equipmentSetCompanyFamily('__proto__'),false);assert.equal(c.eq.companyFamily,'aircraft');c.equipmentSetCompanyFamily('all');assert.match(c.mount.innerHTML,/Aircraft with no supported platform/);c.eq.companyProduct='product:1:12';c.eq.companyFamily='aircraft';c.S={session_id:'two',player:'FRA'};c.equipmentResetCampaign();assert.equal(c.eq.companyFamily,'all');assert.equal(c.eq.companyProduct,null);assert.equal(c.eq.data,null);assert.equal(c.calls.length,0);
});

function companyAmmoPurchase(product=8,quantity=20){return {label:'Review finished ammunition purchase',command:{kind:'company_ammo_purchase',company:1,product,quantity},requires_preview:true,inputs:[{key:'quantity',label:'Rounds to buy',type:'number',value:quantity,min:1,max:100,step:1}]};}
function supplierAmmunitionMarket(){return {overview:service({title:'Domestic ammunition suppliers',status:'Supply available',detail:'Company working capital pays for finished stock. Reserve targets place no purchases.',metrics:[{label:'Purchase authority after protected upkeep',value:'$83k'}],roles:[],warnings:[],actions:[{label:'Commission ammunition supply',command:{kind:'company_ammo_supply',company:1,family:'cannon',stock_target:200},requires_preview:true,inputs:[{key:'stock_target',label:'Company stock target',type:'number',value:200,min:1,max:10000,step:1}]}]}),offers:[
  {id:'ammo-product:1:8',name:'Supplier cannon rounds',supplier_name:'National Armor',family:'ammunition',ammo_family:'cannon',unit_label:'round',phase:'stock',status:'Ready to buy',detail:'Covers a reserve gap for Sentinel 90.',metrics:[{label:'Available company stock',value:100,unit:'rounds'},{label:'National magazine',value:75,unit:'rounds'},{label:'Paid incoming',value:23,unit:'rounds'},{label:'Remaining reserve gap',value:102,unit:'rounds'}],costs:[{label:'Price per round',amount_bn:.00000083}],requirements:['Compatible models: Sentinel 90.'],actions:[companyAmmoPurchase()]},
  {id:'ammo-product:1:9',name:'Supplier unguided bombs',supplier_name:'National Armor',family:'ammunition',ammo_family:'air_bomb_unguided',unit_label:'store',phase:'stock',status:'Ready to buy',detail:'Required by Swift light attack aircraft.',metrics:[{label:'Available company stock',value:4,unit:'stores'},{label:'National magazine',value:0,unit:'stores'},{label:'Remaining reserve gap',value:7,unit:'stores'}],costs:[{label:'Price per store',amount_bn:.0000041}],requirements:['Compatible models: Swift light attack.'],actions:[{...companyAmmoPurchase(9,2),inputs:[{key:'quantity',label:'Stores to buy',type:'number',value:2,min:1,max:4,step:1}]}]}
],deliveries:[{id:'ammo-delivery:1:5',name:'Paid cannon shipment',supplier_name:'National Armor',family:'ammunition',ammo_family:'cannon',unit_label:'round',phase:'delivery',status:'Awaiting settlement',detail:'Paid ownership is retained. Shipping begins after settlement.',metrics:[{label:'Purchased rounds',value:23,unit:'rounds'},{label:'Available in magazine',value:0,unit:'rounds'}],actions:[]}]};}
function supplierAmmoSnapshot(){const data=mixedCompanySnapshot(),market=supplierAmmunitionMarket();data.ammunition=ammunition({supplier_market:market});data.companies.products.push(...plain(market.offers));data.companies.deliveries.push(...plain(market.deliveries));return data;}
function supplierAmmoQuote(command,extra={}){return quote({detail:'Buy completed supplier stock. Only arrival adds physical ammunition.',costs:[{label:'Reviewed ammunition purchase',amount_bn:.0000166}],metrics:[{label:'Purchased rounds',value:command.quantity},{label:'Company stock after sale',value:100-command.quantity}],timing:[{label:'Delivery after settlement',value:'7 accessible days'}],requirements:['Fleet maintenance has priority over this purchase.'],service_effects:service({title:'Reserve after delivery',metrics:[{label:'Remaining reserve gap',value:82,unit:'rounds'}],roles:[],warnings:['This purchase creates no vehicle and activates no ammunition system.'],actions:[]}),actions:[{label:'Confirm stock purchase',command:{...command,quote:'exact-ammo-stock-quote'}}],...extra});}

test('ammunition supplier shelf shows authoritative stock, reserve reasons and arrivals ahead of preserved public batches',()=>{
  const c=fixture(),data=supplierAmmoSnapshot(),before=plain(data);loaded(c,data);c.eq.draft.name='Unfinished strike model';c.equipmentSelectTab('ammunition');const html=c.mount.innerHTML;
  assert(html.indexOf('Buy finished ammunition')<html.indexOf('Manufacturer stock'));assert(html.indexOf('Manufacturer stock')<html.indexOf('Reserve plans'));assert(html.indexOf('Purchased ammunition & arrivals')<html.indexOf('Ammunition production'));
  for(const text of ['6 of 6 shown','Domestic ammunition suppliers','$83k','Supplier cannon rounds','102','Paid incoming','23','Compatible models: Sentinel 90','Supplier unguided bombs','Swift light attack','Price per store','$4.1k','Paid cannon shipment','Awaiting settlement','Existing public batches retain their funding'])assert(html.includes(text),text);
  for(const path of ['ammunition.supplier_market.overview.actions.0','ammunition.supplier_market.offers.0.actions.0','ammunition.supplier_market.offers.1.actions.0','ammunition.orders.0.actions.0'])assert(c.mount.querySelector(`[data-equipment-action="${path}"]`),path);
  assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,1);assert.equal(c.mount.querySelector('[data-equipment-model]'),null);assert.doesNotMatch(html,/Maintenance per round|Price per vehicle|data-equipment-company-model/);assert.deepEqual(plain(data),before);assert.equal(c.eq.draft.name,'Unfinished strike model');assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
});

test('ammunition supplier search keeps original offer paths and combines supplier, family and paid-delivery records',async()=>{
  const c=fixture(),data=supplierAmmoSnapshot();loaded(c,data);c.equipmentSelectTab('ammunition');let search=c.mount.querySelector('#equipmentSearch');search.value='National Armor';search.oninput();assert.match(c.mount.innerHTML,/3 of 6 shown/);assert.match(c.mount.innerHTML,/Paid cannon shipment/);assert.doesNotMatch(c.mount.innerHTML,/Cannon batch · California/);
  search=c.mount.querySelector('#equipmentSearch');search.value='air_bomb_unguided';search.oninput();assert.match(c.mount.innerHTML,/1 of 6 shown/);assert.equal(c.mount.querySelector('[data-equipment-action="ammunition.supplier_market.offers.0.actions.0"]'),null);const buy=c.mount.querySelector('[data-equipment-action="ammunition.supplier_market.offers.1.actions.0"]');assert(buy);
  c.api=async(...args)=>{c.requests.push(plain(args));return supplierAmmoQuote(plain(args[1].command));};buy.onclick();await tick();assert.deepEqual(c.requests[0][1].command,{kind:'company_ammo_purchase',company:1,product:9,quantity:2});assert.equal(c.calls.length,0);assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,1);c.mount.querySelector('[data-equipment-dismiss]').onclick();c.mount.querySelector('[data-equipment-clear]').onclick();assert.match(c.mount.innerHTML,/6 of 6 shown/);
});

test('reviewed ammunition stock quantity invalidates previous quotes and confirms only the exact fresh supplier command',async()=>{
  const c=fixture(),data=supplierAmmoSnapshot();loaded(c,data);c.eq.draft.name='Local draft retained';c.equipmentSelectTab('ammunition');c.api=async(...args)=>{c.requests.push(plain(args));return supplierAmmoQuote(plain(args[1].command));};c.mount.querySelector('[data-equipment-action="ammunition.supplier_market.offers.0.actions.0"]').onclick();await tick();
  for(const text of ['$16.6k','Reserve after delivery','82','7 accessible days','maintenance has priority','creates no vehicle'])assert(c.mount.innerHTML.includes(text),text);assert.equal(c.calls.length,0);const old=c.mount.querySelector('[data-equipment-intent="0"]');let resolve;c.api=(...args)=>{c.requests.push(plain(args));return new Promise(r=>resolve=r);};const quantity=c.mount.querySelector('[data-equipment-order-input="quantity"]');quantity.value='40';quantity.oninput();assert.equal(await old.onclick(),false);assert.equal(await c.equipmentConfirm(0),false);
  const requested={kind:'company_ammo_purchase',company:1,product:8,quantity:40};assert.deepEqual(c.requests.at(-1)[1].command,requested);resolve(supplierAmmoQuote(requested,{costs:[{label:'Reviewed ammunition purchase',amount_bn:.0000332}]}));await tick();assert.equal(await old.onclick(),false);assert.match(c.mount.innerHTML,/\$33.2k/);c.api=async path=>path.startsWith('/api/equipment?')?data:preview();assert.equal(await c.equipmentConfirm(0),true);assert.deepEqual(c.calls,[{...requested,quote:'exact-ammo-stock-quote'}]);assert.equal(c.eq.draft.name,'Local draft retained');await tick();
});

test('supplier ammunition commissioning and failed purchases use standard review guards without replacing old public batch controls',async()=>{
  const c=fixture(),data=supplierAmmoSnapshot();loaded(c,data);c.equipmentSelectTab('ammunition');c.api=async(...args)=>{c.requests.push(plain(args));return supplierAmmoQuote(plain(args[1].command));};c.mount.querySelector('[data-equipment-action="ammunition.supplier_market.overview.actions.0"]').onclick();await tick();const target=c.mount.querySelector('[data-equipment-order-input="stock_target"]');target.value='320';target.oninput();await tick();assert.deepEqual(c.requests.at(-1)[1].command,{kind:'company_ammo_supply',company:1,family:'cannon',stock_target:320});assert.equal(c.calls.length,0);c.mount.querySelector('[data-equipment-dismiss]').onclick();
  c.api=async(...args)=>supplierAmmoQuote(plain(args[1].command),{valid:false,blockers:['Only four rounds remain in company stock.','Purchase funding is protected for fleet upkeep.']});c.mount.querySelector('[data-equipment-action="ammunition.supplier_market.offers.0.actions.0"]').onclick();await tick();assert.match(c.mount.innerHTML,/protected for fleet upkeep/);assert.equal(await c.equipmentConfirm(0),false);c.mount.querySelector('[data-equipment-dismiss]').onclick();const old=c.mount.querySelector('[data-equipment-action="ammunition.supplier_market.offers.0.actions.0"]');c.S={session_id:'two',player:'FRA'};assert.equal(old.onclick(),false);assert.equal(c.calls.length,0);
  const next=fixture();loaded(next,data);next.equipmentSelectTab('ammunition');next.mount.querySelector('[data-equipment-action="ammunition.orders.0.actions.0"]').onclick();assert.deepEqual(plain(next.eq.review.command),{kind:'equipment_ammo_pause',project:7,paused:true});assert.equal(next.calls.length,0);
});

test('company ammunition filtering never fabricates a vehicle preview and rejects captured hidden vehicle controls',()=>{
  const c=fixture(),data=supplierAmmoSnapshot();data.companies.products.at(-1).spec=plain(data.companies.products[3].spec);loaded(c,data);c.eq.draft.name='Untouched aircraft draft';const draft=plain(c.eq.draft);c.equipmentSelectTab('companies');const controller=c.modelMounts[0],oldInspect=c.mount.querySelector('[data-equipment-company-model="0"]');assert.equal(c.mount.querySelectorAll('[data-equipment-company-family]').length,4);assert.match(c.mount.innerHTML,/All stock/);assert.equal(c.mount.querySelector('[data-equipment-company-model="6"]'),null);
  c.equipmentSetCompanyFamily('ammunition');assert.match(c.mount.innerHTML,/Supplier cannon rounds|Supplier unguided bombs/);assert.match(c.mount.innerHTML,/Company working capital/);assert.match(c.mount.innerHTML,/Existing ground delivery/);assert.match(c.mount.innerHTML,/Paid cannon shipment/);assert.equal(c.equipmentCompanyProduct(),null);assert.equal(c.mount.querySelector('[data-equipment-model]'),null);assert.equal(controller.disposals,1);assert.equal(oldInspect.onclick(),false);assert.doesNotMatch(c.mount.innerHTML,/Inspect 3D model|Maintenance per store/);
  c.equipmentSetCompanyFamily('aircraft');assert.match(c.mount.innerHTML,/Swift light attack/);assert.doesNotMatch(c.mount.innerHTML,/Supplier unguided bombs/);assert.equal(c.modelMounts.at(-1).initial.platform,'air_light_attack');assert.deepEqual(plain(c.eq.draft),draft);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
});

test('supplier ammunition empty and unavailable records retain explanations, escape text and invent no prices or model units',()=>{
  const c=fixture(),market=supplierAmmunitionMarket();market.offers=[];market.deliveries=[];loaded(c,snapshot({ammunition:ammunition({supplier_market:market,families:[],orders:[]})}));c.equipmentSelectTab('ammunition');assert.match(c.mount.innerHTML,/No manufacturer stock listed/);assert.match(c.mount.innerHTML,/No purchased ammunition deliveries/);assert.match(c.mount.innerHTML,/arrange company supply/);assert.equal(c.mount.querySelectorAll('#equipmentSearch').length,0);
  const unsafe='"><img src=x onerror=alert(1)>';market.offers=[{id:unsafe,name:unsafe,supplier_name:unsafe,family:'ammunition',ammo_family:unsafe,unit_label:'round',status:'Blocked',phase:'blocked',detail:unsafe,metrics:[{label:'Available rounds',value:null}],requirements:[unsafe],blockers:['No certified compatible weapon is available.'],actions:[]}];loaded(c,snapshot({ammunition:ammunition({supplier_market:market,families:[],orders:[]})}));c.equipmentRender();assert.doesNotMatch(c.mount.innerHTML,/<img src=x|Price per vehicle|Maintenance per vehicle/);assert.match(c.mount.innerHTML,/&lt;img/);assert.match(c.mount.innerHTML,/Available rounds<\/dt><dd>—/);assert.match(c.mount.innerHTML,/No certified compatible weapon/);assert.equal(c.calls.length,0);assert.equal(c.requests.length,0);
});

test('all company ammunition bridge commands preserve payloads and drafts and return to the ammunition shelf only on success',async()=>{
  const cases=[[{kind:'company_ammo_supply',company:1,family:'cannon',stock_target:200,quote:'reviewed-supply'},/Ammunition supply commissioned/],[{kind:'company_ammo_inventory',company:1,product:8,stock_target:0},/places no government purchase/],[{kind:'company_ammo_purchase',company:1,product:9,quantity:2,quote:'reviewed-stores'},/only after delivery/]];
  for(const [command,message] of cases){const c=shellFixture();loaded(c,supplierAmmoSnapshot());c.room.hidden=false;c.eq.tab='companies';c.eq.draft.name='Local aircraft';c.queued=[{kind:'tax',value:20}];c.api=async(...args)=>{c.requests.push(plain(args));return {session_id:'one',player:'USA'};};c.adopt=async state=>{c.S=state;};const result=await c.equipmentCommand(command);assert.deepEqual(c.requests,[['/api/command',{commands:[command]}]]);assert.match(result.message,message);assert.equal(c.eq.tab,'ammunition');assert.equal(c.eq.draft.name,'Local aircraft');assert.deepEqual(c.queued,[{kind:'tax',value:20}]);}
  const c=shellFixture();loaded(c);c.room.hidden=false;c.eq.tab='companies';c.api=async()=>({errors:['Company ammunition stock changed. Review again.']});await assert.rejects(c.equipmentCommand(cases[2][0]),/stock changed/);assert.equal(c.eq.tab,'companies');
});

test('converted ammunition reserve plans follow served manual controls while existing public batches remain separately controllable',async()=>{
  const c=fixture(),data=supplierAmmoSnapshot(),reserve=ammunitionReserve();reserve.inputs[3].options=[{value:false,label:'Manual target'},{value:true,label:'Automatic public manufacture',enabled:false,reason:'This family uses reviewed manufacturer stock purchases.'}];data.ammunition.reserves=[{id:'reserve:cannon',name:'Cannon reserve target',status:'Company supply · manual purchases',detail:'Target retained. Future automatic public batches stopped for this family.',metrics:[{label:'Reserve target',value:5000,unit:'rounds'},{label:'Paid incoming',value:23,unit:'rounds'}],actions:[reserve]}];data.ammunition.families[1].actions=[companyAmmoPurchase()];const before=plain(data);loaded(c,data);c.equipmentSelectTab('ammunition');assert.match(c.mount.innerHTML,/Future automatic public batches stopped/);assert.match(c.mount.innerHTML,/Targets place no company purchase/);assert(c.mount.querySelector('[data-equipment-action="ammunition.orders.0.actions.0"]'));
  c.api=async(...args)=>{c.requests.push(plain(args));return quote({actions:[],metrics:[],costs:[],timing:[]});};c.mount.querySelector('[data-equipment-action="ammunition.reserves.0.actions.0"]').onclick();await tick();const automatic=c.mount.querySelector('[data-equipment-order-input="automatic"]');automatic.value='true';automatic.onchange();assert.equal(c.requests.length,1);assert.equal(c.requests[0][1].command.automatic,false);assert.match(c.mount.innerHTML,/reviewed manufacturer stock purchases/);assert.deepEqual(plain(data),before);assert.equal(c.calls.length,0);
});

test('a supplier reserve preserves the actual suspended automatic preference and confirms without claiming replenishment was enabled',async()=>{
  const c=shellFixture(),data=supplierAmmoSnapshot(),reserve=ammunitionReserve();reserve.command.automatic=true;reserve.inputs[3].value=true;reserve.inputs[3].options=[{value:false,label:'Manual target · review each order'},{value:true,label:'Saved automatic preference · suspended during company supply'}];reserve.detail='The automatic public-production preference is suspended while this company supplies the family.';data.ammunition.reserves=[{id:'reserve:cannon',name:'Cannon reserve',status:'Reviewed company purchases',metrics:[{label:'Automatic review from',value:'Suspended · company purchases require review'}],actions:[reserve]}];loaded(c,data);c.room.hidden=false;c.eq.draft.name='Retained supplier test draft';c.equipmentSelectTab('ammunition');
  c.api=async(...args)=>{c.requests.push(plain(args));if(args[0]==='/api/equipment-preview')return quote({detail:reserve.detail,metrics:[],costs:[],timing:[{label:'Automatic review begins',value:'Suspended · company purchases require review'}],actions:[{label:'Save supplier reserve target',command:plain(args[1].command)}]});if(args[0]==='/api/command')return {session_id:'one',player:'USA'};return data;};c.adopt=async state=>{c.S=state;};
  c.mount.querySelector('[data-equipment-action="ammunition.reserves.0.actions.0"]').onclick();await tick();assert.equal(c.requests[0][1].command.automatic,true);assert.equal(c.mount.querySelector('[data-equipment-order-input="automatic"]').value,'true');const target=c.mount.querySelector('[data-equipment-order-input="target_rounds"]');target.value='6000';target.oninput();await tick();assert.equal(c.requests.at(-1)[1].command.automatic,true);assert.match(c.mount.innerHTML,/suspended during company supply/);assert.equal(await c.equipmentConfirm(0),true);await tick();
  const commands=c.requests.filter(([path])=>path==='/api/command');assert.equal(commands.length,1);assert.deepEqual(commands[0][1].commands,[{...reserve.command,target_rounds:6000}]);assert.match(c.eq.message,/automatic preference saved/);assert.doesNotMatch(c.eq.message,/replenishment enabled|automatic purchasing enabled/);assert.equal(c.eq.draft.name,'Retained supplier test draft');assert.equal(c.eq.tab,'ammunition');
});
