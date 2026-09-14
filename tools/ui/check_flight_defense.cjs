// Execute the shipped map card and overlay against small read-only fixtures.
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const page=fs.readFileSync(path.resolve(__dirname,'../../spheres-web/ui/index.html'),'utf8').replace(/\r\n/g,'\n');
function source(name){const start=page.indexOf('function '+name+'(');assert(start>=0);const lineEnd=page.indexOf('\n',start);if(page.slice(start,lineEnd).trim().endsWith('}'))return page.slice(start,lineEnd);return page.slice(start,page.indexOf('\n}',start)+2);}
function fixture(missionKind='defend_skies'){
  const calls=[],drawn=[],nodes=[];
  function node(tag){const n={tag,children:[],textContent:'',append(...children){this.children.push(...children);},setAttribute(){},remove(){const i=nodes.indexOf(this);if(i>=0)nodes.splice(i,1);}};nodes.push(n);return n;}
  const stage=node('stage'),state={player:'France',date:'12 Feb 1990'},row={missionKind,base:'FR-A',name:'Fighter base',lon:2,lat:48,capacity:12,occupied:4,rangeKm:900,target:{district:'FR-B',name:'Friendly <province>',lon:3,lat:49}};
  const c={S:state,FLIGHT_MAP:{state,row},DINDEX:{},window:{},document:{getElementById:id=>nodes.find(n=>n.id===id),querySelector:selector=>selector==='#pane-map .globe-stage'?stage:null,createElement:node},GLOBE:{render:()=>calls.push('render')},openEquipment:options=>calls.push(options),clamp:(v,a,b)=>Math.max(a,Math.min(b,v)),slerpGeo:(a,b,t)=>a.map((x,i)=>x+(b[i]-x)*t),roundedRect(){}};
  vm.createContext(c);vm.runInContext(['flightMapGeo','flightMapCurrent','renderFlightMapCard','flightMapRadiusPoints','drawFlightMapOverlay'].map(source).join('\n'),c);
  const ctx={save(){},restore(){},beginPath(){},moveTo(){},lineTo(){},stroke(){},setLineDash(){},arc(){},fill(){},fillText:text=>drawn.push(text),measureText:text=>({width:text.length*6})};
  return {c,calls,drawn,ctx,stage,text:n=>[n.textContent,...n.children.map(x=>[x.textContent,...x.children.map(y=>y.textContent)].join(' '))].join(' ')};
}
test('s16 map explains a defended province and draws a defense marker instead of a strike cross',()=>{
  const f=fixture();f.c.renderFlightMapCard();const card=f.c.document.getElementById('flightMapCard'),text=f.text(card);
  assert.match(text,/Defense area: Friendly <province>/);assert.match(text,/900 km patrol radius/);assert.match(text,/Support army and Strike target missions/);assert.match(text,/does not attack ground forces/);assert.doesNotMatch(text,/Target:/);
  f.c.drawFlightMapOverlay(f.ctx,{ratio:1},{projectGeo:(lon,lat)=>[lon,lat],labelBoxes:[]});assert(f.drawn.includes('D'));assert(f.drawn.includes('Defend · Friendly <province>'));assert(!f.drawn.includes('×'));assert.equal(f.calls.length,0);
  card.children.at(-1).children[0].onclick();assert.equal(f.calls[0].tab,'flight');
});
test('s16 map retains existing strike wording and refuses stale defense geometry',()=>{
  const f=fixture('strike_target');f.c.renderFlightMapCard();assert.match(f.text(f.c.document.getElementById('flightMapCard')),/Target: Friendly <province>/);
  f.c.drawFlightMapOverlay(f.ctx,{ratio:1},{projectGeo:(lon,lat)=>[lon,lat],labelBoxes:[]});assert(f.drawn.includes('×'));assert(!f.drawn.includes('D'));
  f.c.FLIGHT_MAP.row.missionKind='defend_skies';f.c.S={player:'France',date:'13 Feb 1990'};f.drawn.length=0;f.c.renderFlightMapCard();assert.match(f.text(f.c.document.getElementById('flightMapCard')),/campaign reading changed/);f.c.drawFlightMapOverlay(f.ctx,{ratio:1},{projectGeo:()=>assert.fail('Stale geometry must not project')});assert.deepEqual(f.drawn,[]);assert.deepEqual(f.calls,[]);
});
