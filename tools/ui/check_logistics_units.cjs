// Exercise the shipped freight formatter and cargo card, not a copied display.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const page=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8');
function fn(name){
  const match=new RegExp(`^function ${name}\\(`,'m').exec(page);
  assert(match,`Shipped function ${name} exists`);
  const lineEnd=page.indexOf('\n',match.index),line=page.slice(match.index,lineEnd);
  return /}\s*$/.test(line)?line:page.slice(match.index,page.indexOf('\n}',match.index)+2);
}
function fixture(){
  const c=vm.createContext({LOGI:{selected:null},RESOURCES:{},stockRows:()=>[],nationById:()=>null,
    escText:value=>String(value??''),rglyph:()=>'',stockHue:()=>'',MONTHS:[],clamp:(n,a,b)=>Math.min(b,Math.max(a,n))});
  vm.runInContext(['fmtQ','logisticsQuantity','logisticsNumber','logisticsCalendar','logisticsMonthLabel',
    'logisticsRoute','logisticsLaneId','logisticsEndpoint','logisticsEndpointName','logisticsCommodity',
    'logisticsCommodityName','logisticsEscAttr','logisticsCargoHtml'].map(fn).join('\n'),c);
  return c;
}
test('small physical freight reads as tonnes or kilograms without changing its quantity',()=>{
  const c=fixture();
  for(const [value,unit,expected] of [[4e-6,'kt','4 kg'],[.05,'kt','50 t'],[.5,'kt','500 t'],[2,'kt','2 kt'],
    [.004,'t','4 kg'],[.125,'kg','0.125 kg'],[0,'kt','0 kt'],[.004,'bcf','0.004 bcf'],[null,'kt','—'],[NaN,'kt','—']]){
    assert.equal(c.logisticsQuantity(value,unit),expected,`${value} ${unit}`);
  }
});
test('a small cargo keeps its real due day and actual arrival ahead of legacy month estimates',()=>{
  const c=fixture(),cargo={id:'cargo:7',from:'USA',to:'France',commodity:'rare_earths',commodity_name:'Rare earths',
    quantity:4e-6,unit:'kt',due_day:{label:'31 Dec 1991'},due_month:{label:'Jan 1992'}};
  const before=JSON.stringify(cargo),moving=c.logisticsCargoHtml(cargo,false);
  assert.match(moving,/4 kg/);assert.match(moving,/Due<b>31 Dec 1991/);
  assert.doesNotMatch(moving,/Jan 1992|0 kt/);assert.equal(JSON.stringify(cargo),before);
  const arrived=c.logisticsCargoHtml({...cargo,arrived_day:{label:'3 Jan 1992'},arrived_month:{label:'Jan 1992'}},true);
  assert.match(arrived,/Arrived<b>3 Jan 1992/);assert.doesNotMatch(arrived,/31 Dec 1991/);
});
