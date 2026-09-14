// Synthetic proof records exercise the shipped browser qualification guards.
// They do not substitute campaign responses or claim a gameplay result.
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const driver=fs.readFileSync(path.join(__dirname,'ci-ground-operations.cjs'),'utf8');
const first=driver.indexOf('function verifyOutcomes('),last=driver.indexOf('\nasync function groundReading(',first);assert(first>=0&&last>first);
const clone=x=>JSON.parse(JSON.stringify(x));
const context=vm.createContext({assert,copy:clone,near:(a,b,label)=>assert(Math.abs(a-b)<1e-9,label)});
vm.runInContext(driver.slice(first,last),context);
function fixture(){
  const required={company:1,refit:10,vehicle_delivery:20,ammunition_delivery:21,target_revision:'night-ifv',retired_revision:'tank',retired_quantity:1,conflicts:[7,8]};
  const base={models:[],holdings:[{design_id:'ifv',units:3,refit_reserved:0},{design_id:'tank',units:5,refit_reserved:0}],last_report:null,ammunition:{consumed:{shells:0}},ledgers:{company:{id:1,nation:'France'},refit:null,vehicle_delivery:null,ammunition_delivery:null}};
  const stage=(id,day,ground)=>({id,as_of_day:day,ground:clone(ground)}),stages=[stage('loaded',100,base)];
  const refit={id:10,product:2,source_revision:'ifv',target_revision:'night-ifv',quantity:1,completed_units:0,cancelled_units:0,cancelled_day:null,booked_day:100,settled_day:null,closed_day:null,escrow_bn:.01};
  let g=clone(base);g.holdings[0].refit_reserved=1;g.ledgers.refit=refit;stages.push(stage('refit',100,g));
  const delivery={id:20,company:1,buyer:'France',product:2,quantity:1,revision_id:'night-ifv',purchased_day:100,settled_day:null,delivered_day:null,total_price_bn:.1,status:'awaiting_settlement'};
  g.ledgers.vehicle_delivery=delivery;stages.push(stage('purchase',100,g));
  g.ledgers.ammunition_delivery={...clone(delivery),id:21,product:3,quantity:500};stages.push(stage('resupply',100,g));
  g.holdings[1].units=4;g.last_report={day:101,conflicts:[7,8],revisions:[{revision_id:'tank',lost:1,opening_delivered:5,remaining_delivered:4,opening_available:5,remaining_available:4,opening_reserved:0,remaining_reserved:0}]};g.ammunition={consumed:{shells:10},last_consumption:{day:101}};stages.push(stage('combat',101,g));
  g.holdings[0]={design_id:'ifv',units:2,refit_reserved:0};g.holdings.push({design_id:'night-ifv',units:2,refit_reserved:0});g.ledgers.refit={...clone(refit),completed_units:1,settled_day:101,closed_day:104,escrow_bn:0};
  for(const key of ['vehicle_delivery','ammunition_delivery'])g.ledgers[key]={...g.ledgers[key],settled_day:101,delivered_day:103,status:'delivered'};stages.push(stage('service',104,g));
  g.holdings[1].units=3;stages.push(stage('retire',104,g));
  const command=(stage,c)=>({stage,payload:{commands:[c]}}),commands=[
    command('refit',{kind:'company_refit',company:1,product:2,source:'ifv',quantity:1}),command('purchase',{kind:'company_purchase',company:1,product:2,quantity:1}),
    command('resupply',{kind:'company_ammo_purchase',company:1,product:3,quantity:500}),command('retire',{kind:'equipment_retire',revision:'tank',quantity:1})];
  return {e:{stages,commands},manifest:{required_outcomes:required,days_advanced:4}};
}
test('only new paid arrivals, completed refit, actual losses and retirement qualify',()=>{
  const {e,manifest}=fixture(),result=context.verifyOutcomes(e,manifest);assert.equal(result.returned_target_units,2);assert.equal(result.ammunition_consumed_delta,10);assert.equal(result.retirement.quantity,1);
});
test('pre-authored contracts and merely purchased inventory cannot satisfy completion',()=>{
  for(const key of ['refit','vehicle_delivery','ammunition_delivery']){const {e,manifest}=fixture();e.stages[0].ground.ledgers[key]=clone(e.stages.at(-1).ground.ledgers[key]);assert.throws(()=>context.verifyOutcomes(e,manifest),/not pre-authored/);}
  for(const key of ['vehicle_delivery','ammunition_delivery']){const {e,manifest}=fixture();e.stages.at(-1).ground.ledgers[key].delivered_day=null;assert.throws(()=>context.verifyOutcomes(e,manifest),/arrival.*advanced interval/);}
  const {e,manifest}=fixture();e.stages.at(-1).ground.ledgers.refit.completed_units=0;assert.throws(()=>context.verifyOutcomes(e,manifest));
});
test('wrong contracts, retained reservations and invented target vehicles fail',()=>{
  for(const mode of ['company','product','reservation','target']){const {e,manifest}=fixture(),g=e.stages.at(-1).ground;if(mode==='company')g.ledgers.vehicle_delivery.company=99;if(mode==='product')g.ledgers.refit.product=99;if(mode==='reservation')g.holdings[0].refit_reserved=1;if(mode==='target')g.holdings.at(-1).units=1;assert.throws(()=>context.verifyOutcomes(e,manifest),undefined,mode);}
});
test('no-loss, wrong-front, free-ammunition and falsified retirement records fail',()=>{
  for(const mode of ['loss','front','ammunition','retirement','receipt']){const {e,manifest}=fixture();
    if(mode==='loss')for(const s of e.stages)if(s.ground.last_report)s.ground.last_report.revisions[0].lost=0;
    if(mode==='front')for(const s of e.stages)if(s.ground.last_report)s.ground.last_report.conflicts=[7];
    if(mode==='ammunition')e.stages.at(-1).ground.ammunition.consumed.shells=0;
    if(mode==='retirement')e.stages.at(-1).ground.holdings[1].units=4;
    if(mode==='receipt')e.stages.at(-1).ground.last_report.revisions[0].remaining_reserved=1;
    assert.throws(()=>context.verifyOutcomes(e,manifest),undefined,mode);
  }
});
