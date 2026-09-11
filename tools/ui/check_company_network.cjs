// Deliberately synthetic UI controls; actual company books are exercised by native tests.
const {test}=require('node:test');
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const root=path.resolve(__dirname,'../..');
const source=fs.readFileSync(path.join(root,'spheres-web/ui/companies-ui.js'),'utf8');
const page=fs.readFileSync(path.join(root,'spheres-web/ui/index.html'),'utf8');
const plain=value=>JSON.parse(JSON.stringify(value));
const deferred=()=>{let resolve;return {promise:new Promise(r=>resolve=r),resolve:value=>resolve(value)};};
function fixture(){return {session_id:'synthetic-session',nation:'France',name:'France',enabled:true,
  directory:[{reference:'supplier:1',kind:'supplier',id:1,name:'Maker <script>',sector:'defense'},{reference:'contractor:1',kind:'contractor',id:1,name:'Civil specialists',sector:'construction'}],
  suppliers:{firms:[{id:1,name:'Maker <script>',metrics:[{label:'Company cash',value:'$1m'}],costs:[]}],products:[{id:'1:1',company:1,name:'Test vehicle',availability:{ready_stock:3,unit_price_bn:.001}}]},
  contractors:{companies:[{id:1,name:'Civil specialists',nation:'France',sector:'construction',capacity:2,experience:0,work_bonus:.1,input_saving:.05,fee_rate:.03,total_fees_bn:.02}],assignments:[],targets:[{sector:'construction',label:'A public project',target:{kind:'construction',project:7},current_company_id:null}]},
  operations:{enabled:true,companies:[{company:1,reason:'Needs advanced components',company_cash_available_bn:.01,locked_working_capital_bn:.003,public_refit_escrow_bn:.002,readiness:{slot_available:true,staffing_fraction:.75,work_fraction:.5,power_available:2,power_required:3},next_packet:{company_cash_required_bn:.001,public_development_required_bn:0,inputs:[{name:'Advanced components',unit:'parts',required:3,available:1,missing:2}]},last_receipt:null}]}};}
function harness(){
  const calls=[],pending=[];
  const c=vm.createContext({S:{session_id:'synthetic-session',player:'France'},CAB:{tab:'companies'},COMMAND_CHANNEL:{},SESSION:{},window:{},
    document:{querySelector:()=>null,activeElement:null},cabinetIsOpen:()=>true,calls,
    api:(...args)=>{calls.push(args);const d=deferred();pending.push(d);return d.promise;},
    adopt:async result=>{calls.push(['adopt',result]);c.S=result;},openEquipment:options=>{calls.push(['equipment',options]);return true;},
  });
  vm.runInContext(source,c);c.desk=vm.runInContext('CDESK',c);Object.assign(c.desk,{open:true,data:fixture(),state:c.S,session:c.S.session_id,nation:c.S.player,stale:false});
  c.pending=pending;c.companiesFetch=async()=>false;return c;
}
const assignment={kind:'assign_sector_contractor',company:1,target:{kind:'construction',project:7}};
function quote(command=assignment){return {session_id:'synthetic-session',nation:'France',valid:true,title:'Review service assignment',note:'Future funded work only',quote:{work_rate:1.1,input_rate:.95,fee_rate:.03,fee_basis:'Fee on delivered work'},command:{...command,quote:'exact-reviewed-token'}};}

test('qualified company identities keep supplier property and contractor fees separate',()=>{
  const c=harness(),before=JSON.stringify(c.desk.data),rows=c.companyRows();
  assert.deepEqual(plain(rows.map(r=>r.reference)),['supplier:1','contractor:1']);
  const supplier=c.companyCardHtml(rows[0]),contractor=c.companyCardHtml(rows[1]);
  assert(supplier.includes('Maker &lt;script&gt;'));assert(!supplier.includes('<script>'));
  assert(supplier.includes('State-owned equipment manufacturer'));assert(!supplier.includes('Earned service fees'));
  assert(contractor.includes('Earned service fees'));assert(contractor.includes('not a cash balance'));assert(!contractor.includes('Spendable company cash'));
  assert.equal(JSON.stringify(c.desk.data),before);
  c.desk.data.directory.push({reference:'supplier:1',kind:'contractor',id:1});assert.equal(c.companyRows().length,2,'mismatched namespace is not resolved');
});

test('role, sector and available-work filters use the correct company account',()=>{
  const c=harness(),[supplier,contractor]=c.companyRows();c.desk.onlyAvailable=true;
  assert(c.companyMatches(supplier));assert(c.companyMatches(contractor));
  c.desk.data.suppliers.products[0].availability.ready_stock=0;assert(!c.companyMatches(supplier));assert(c.companyMatches(contractor));
  c.desk.onlyAvailable=false;c.desk.role='supplier';assert(c.companyMatches(supplier));assert(!c.companyMatches(contractor));
  c.desk.role='all';c.desk.sector='construction';assert(!c.companyMatches(supplier));assert(c.companyMatches(contractor));
});

test('supplier operating readings preserve unknowns, shortage quantities and actual receipt distinction',()=>{
  const c=harness(),row=c.companyRows()[0];let html=c.companyOperatingHtml(row);
  assert(html.includes('Needs advanced components'));assert(html.includes('75%'));assert(html.includes('50%'));
  assert(html.includes('No completed operating receipt'));assert(html.includes('company-funded purchases'));assert(html.includes('Public stock'));
  c.desk.data.operations.companies[0].readiness.staffing_fraction=null;
  c.desk.data.operations.companies[0].last_receipt={date:'1990-01-04',work_days:.5,units:0,cash_paid_bn:.001};
  html=c.companyOperatingHtml(row);assert(html.includes('1990-01-04'));assert(html.includes('0.5 work days'));assert(html.includes('>—</dd>'));
});

test('supplier deep links cannot resolve a service contractor with the same numeric ID',()=>{
  const c=harness();assert.equal(c.companiesOpenSupplier('contractor:1','designer'),false);
  assert.equal(c.companiesOpenSupplier('supplier:1','designer'),true);
  assert.deepEqual(plain(c.calls[0]),['equipment',{tab:'designer',company:1}]);
  c.desk.stale=true;assert.equal(c.companiesOpenSupplier('supplier:1','companies'),false);
});

test('assignment review makes no purchase and confirmation sends the exact returned quote once',async()=>{
  const c=harness();const reviewing=c.companiesReview(assignment);assert.equal(c.calls.length,1);assert.equal(c.calls[0][0],'/api/companies-preview');
  assert.equal(await c.companiesReview(assignment),false,'double clicks cannot start parallel reviews');
  c.pending[0].resolve(quote());assert.equal(await reviewing,true);assert.equal(c.calls.length,1);
  const confirm=c.companiesConfirm();assert.equal(await c.companiesConfirm(),false);
  assert.deepEqual(plain(c.calls[1]),['/api/command',{commands:[quote().command]}]);
  c.pending[1].resolve({session_id:'synthetic-session',player:'France',errors:[]});assert.equal(await confirm,true);
  assert.equal(c.calls.filter(row=>row[0]==='adopt').length,1);
});

test('a replaced campaign, intervening state or pending command invalidates open review',async()=>{
  for(const change of [c=>{c.S={session_id:'new',player:'Japan'};},c=>{c.S={...c.S};},c=>{c.COMMAND_CHANNEL.pending={};}]){
    const c=harness(),reviewing=c.companiesReview(assignment);c.pending[0].resolve(quote());await reviewing;
    change(c);assert.equal(await c.companiesConfirm(),false);assert.equal(c.calls.length,1);
  }
});

test('wrong action or campaign in a quote cannot become a command',async()=>{
  for(const change of [q=>q.command.company=2,q=>q.command.kind='company_purchase',q=>q.session_id='other',q=>q.nation='Japan',q=>q.command.quote='']){
    const c=harness(),reviewing=c.companiesReview(assignment),q=quote();change(q);c.pending[0].resolve(q);
    assert.equal(await reviewing,false);assert.equal(await c.companiesConfirm(),false);assert.equal(c.calls.length,1);
  }
});

test('refused quotes and refused commands do not adopt or automatically retry',async()=>{
  const c=harness(),reviewing=c.companiesReview(assignment);c.pending[0].resolve({...quote(),valid:false,reason:'Facility changed'});await reviewing;
  assert.equal(await c.companiesConfirm(),false);assert(c.companyReviewHtml().includes('Facility changed'));
  const d=harness(),next=d.companiesReview(assignment);d.pending[0].resolve(quote());await next;
  const confirming=d.companiesConfirm();d.pending[1].resolve({errors:['The quote expired.']});assert.equal(await confirming,false);
  assert.equal(d.calls.length,2);assert.equal(d.desk.message,'The quote expired.');
});

test('company-network adoption uses the same explicit review and command lane',async()=>{
  const c=harness(),reviewing=c.companiesReview({kind:'enable_companies'});
  c.pending[0].resolve({...quote({kind:'enable_companies'}),quote:null});await reviewing;
  const confirming=c.companiesConfirm();c.pending[1].resolve({session_id:'synthetic-session',player:'France',errors:[]});assert.equal(await confirming,true);
  assert.equal(c.calls.filter(row=>row[0]==='/api/command').length,1);
});

test('late directory or command responses cannot replace another campaign',async()=>{
  const c=harness(),reviewing=c.companiesReview(assignment);c.pending[0].resolve(quote());await reviewing;
  const confirming=c.companiesConfirm();c.S={session_id:'new-session',player:'Japan'};
  c.pending[1].resolve({session_id:'synthetic-session',player:'France',errors:[]});assert.equal(await confirming,false);
  assert.equal(c.calls.filter(row=>row[0]==='adopt').length,0);
});

test('designer review honors selected manufacturer and refuses unavailable deep links',()=>{
  const source=fs.readFileSync(path.join(root,'spheres-web/ui/equipment-ui.js'),'utf8');
  const c=vm.createContext({window:{},S:{session_id:'test',player:'France'},document:{querySelector:()=>null}});vm.runInContext(source,c);
  const equip=vm.runInContext('EQUIP',c),action={enabled:true,requires_preview:true,command:{kind:'company_develop',company:1},inputs:[{key:'company',value:1,options:[{value:1,enabled:true},{value:2,enabled:true}]}]};
  const data={actions:[action]};Object.assign(equip,{open:true,state:c.S,data,stale:false,companyFirm:2});
  c.equipmentRender=()=>{};c.equipmentRememberView=()=>{};c.equipmentFetchOrderPreview=()=>{};
  assert.equal(c.equipmentInvoke('actions.0','data',data,c.S),true);assert.equal(equip.review.command.company,2);
  equip.review=null;action.inputs[0].options[1].enabled=false;
  assert.equal(c.equipmentInvoke('actions.0','data',data,c.S),false);assert.equal(equip.review,null);
  equip.companyFirm=3;assert.equal(c.equipmentInvoke('actions.0','data',data,c.S),false);
});

test('company panel is reachable and all shipped inline JavaScript remains parseable',()=>{
  assert(page.includes('id="cab-tab-companies"'));assert(page.includes('id="cabinet-companies"'));
  assert(page.includes('src="/companies-ui.js"'));assert(page.includes('href="/companies.css"'));
  for(const script of page.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/g))if(!/\btype\s*=\s*["'](?:application\/json|importmap)/.test(script[1]))new vm.Script(script[2]);
});
