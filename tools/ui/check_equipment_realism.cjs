'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const {build}=require('../../spheres-web/ui/equipment-mesh.js'),{raycast}=require('../../spheres-web/ui/equipment-model.js');
const armor=[.32,.37,.25],paint=[.37,.41,.36],roof=[.35*1.08,.40*1.08,.27*1.08];
function points(mesh,name,color){const part=mesh.parts.find(p=>p.name===name);assert(part,name);const out=[];
  for(let i=part.first*3;i<(part.first+part.count)*3;i+=3)if(!color||color.every((v,k)=>Math.abs(mesh.colors[i+k]-v)<1e-6))out.push({p:Array.from(mesh.positions.subarray(i,i+3)),n:Array.from(mesh.normals.subarray(i,i+3))});return out;
}
const ground=['ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense'];
test('tank nose armor stays below its deck instead of bulging over it as a cylindrical casting',()=>{
  for(const platform of ['tank_standard','tank_heavy']){
    const mesh=build({platform}),name='chassis / sloped lower hull',upper=points(mesh,name,[.35,.4,.27]);
    const plated=points(mesh,name,armor);assert(plated.length>40&&upper.length>20);
    const deck=Math.max(...upper.map(v=>v.p[1])),bow=Math.max(...plated.map(v=>v.p[1]));
    assert(bow<deck-.10,`${platform} bow ${bow} must remain below deck ${deck}`);
  }
});
test('specialist frontal armor is composed of flat folded plates instead of a curved cross-hull drum',()=>{
  for(const platform of ground){
    const mesh=build({platform}),part=mesh.parts.find(p=>p.name==='protection / specialist sloped hull');let inspected=0;
    for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){
      if(!armor.every((v,k)=>Math.abs(mesh.colors[i+k]-v)<1e-6))continue;
      const p=mesh.positions,a=[p[i+3]-p[i],p[i+4]-p[i+1],p[i+5]-p[i+2]],b=[p[i+6]-p[i],p[i+7]-p[i+1],p[i+8]-p[i+2]],n=[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]],len=Math.hypot(...n);
      for(let j=0;j<9;j+=3)assert(n.every((v,k)=>Math.abs(v/len-mesh.normals[i+j+k])<1e-3),`${platform} armor plate has a planar normal`);inspected++;
    }
    // Two folded octagonal side bands contribute sixteen triangles each;
    // their independently shaded end caps are deliberately excluded here.
    assert(inspected>=32,`${platform} must expose both folded armor bands`);
  }
});

test('tank headlamps have compact neutral faces carried by the upper deck brackets',()=>{
  for(const platform of ['tank_standard','tank_heavy']){
    const mesh=build({platform}),name='chassis / glacis applique, splash guard and spare track links';
    assert.equal(points(mesh,name,[.20,.44,.48]).length,0,'Headlamps do not reuse cyan sight optics');
    const faces=points(mesh,name,[.39*1.35,.42*1.35,.38*1.35]);assert(faces.length>=60);
    const deck=Math.max(...points(mesh,'chassis / sloped lower hull',[.35,.40,.27]).map(v=>v.p[1]));
    const ys=faces.map(v=>v.p[1]);assert(Math.max(...ys)<deck+.12&&Math.min(...ys)>deck-.03,'Lens stays in the compact deck-mounted housing');
    const brackets=points(mesh,name,[.20,.22,.20]).filter(v=>Math.abs(v.p[1]-(deck-.05))<1e-5);
    assert(brackets.length>=12,'Both mounting brackets extend below the deck top for a positive attachment');
  }
});
test('rotating turret roofs taper into a narrow gun opening while keeping a broad crew compartment',()=>{
  for(const turret of ['turret_standard','turret_compact','turret_heavy','turret_autoload']){
    const mesh=build({platform:'tank_standard',components:{turret}}),vertices=points(mesh,'turret / ring and faceted armor shell',roof).filter(v=>v.n[1]>.999);
    assert(vertices.length>20);const front=Math.max(...vertices.map(v=>v.p[2])),wide=Math.max(...vertices.map(v=>Math.abs(v.p[0]))),mouth=Math.max(...vertices.filter(v=>Math.abs(v.p[2]-front)<1e-5).map(v=>Math.abs(v.p[0])));
    assert(mouth/wide<.33,`${turret}: roof front ${mouth} vs broad side ${wide}`);assert(wide>.7);
  }
});
test('casemate armor has an intermediate shoulder within the redesigned tank envelope',()=>{
  const mesh=build({platform:'tank_destroyer',components:{turret:'turret_casemate'}}),vertices=points(mesh,'turret / ring and faceted armor shell',[.35,.40,.27]);
  const shoulder=vertices.filter(v=>Math.abs(v.p[2]-1.51)<1e-5&&v.p[1]>1.8&&v.p[1]<2.1);assert(shoulder.length>=4);
  assert(vertices.some(v=>v.p[1]>2.15&&Math.abs(v.p[2]-1.05)<1e-5),'Roof anchor remains above the shoulder');
});
test('aircraft root fairings rise off the wing into the fuselage instead of using flat join plates',()=>{
  for(const platform of ['air_light_attack','air_tactical_strike']){
    const strike=platform==='air_tactical_strike',mesh=build({platform}),w=strike?.77:.58,y=strike?2.10:1.72,z=strike?.15:-.1,shift=mesh.bounds.max[1]-(y+(strike?2:1.65));
    const fairing=points(mesh,'air_wing / airframe and wing roots',paint).filter(v=>Math.abs(v.p[0])>w*1.04&&Math.abs(v.p[2]-(z-.10))<.05);
    assert(fairing.length>20);assert(Math.max(...fairing.map(v=>v.p[1]))>y+shift+.16);
    assert(Math.min(...fairing.map(v=>v.p[1]))<y+shift-.11,'Fairing has a joined lower surface too');
  }
});
test('forebody shapes contain curved intermediate sections rather than only linearly joined endpoints',()=>{
  for(const platform of ['air_light_attack','air_tactical_strike']){
    const strike=platform==='air_tactical_strike',mesh=build({platform}),y=strike?2.10:1.72;
    const vertices=points(mesh,'air_wing / airframe and wing roots',paint).filter(v=>Math.abs(v.p[0])<1e-5&&v.p[1]>y&&v.p[2]>2.05);
    const byZ=new Map(vertices.map(v=>[v.p[2].toFixed(5),v.p[1]]));assert(byZ.size>=6,`${platform} curved forebody has ${byZ.size} sections`);
    const line=[...byZ].map(([z,h])=>[Number(z),h]).sort((a,b)=>a[0]-b[0]);
    for(let i=1;i<line.length;i++)assert(line[i][1]<=line[i-1][1]+1e-5,'Nose taper does not bulge between authored endpoints');
  }
});
test('every engine choice exposes two actual inlet throats with unobstructed depth to the fan',()=>{
  for(const platform of ['air_light_attack','air_tactical_strike'])for(const engine of ['air_engine_economical','air_engine_efficient','air_engine_twin']){
    const strike=platform==='air_tactical_strike';if(!strike&&engine==='air_engine_twin')continue;
    const mesh=build({platform,components:{air_engine:engine}}),twin=engine==='air_engine_twin',w=strike?.77:.58,y=strike?2.10:1.72,r=(twin?.54:engine==='air_engine_efficient'?.52:.43)*(twin?.66:.65),x=w+r+.045,z=(strike?.15:-.1)+.95;
    const shift=mesh.bounds.max[1]-(y+(strike?2:1.65));
    for(const side of [-1,1]){
      const hit=raycast(mesh,[side*x,y-.15+shift,20],[0,0,-1]);assert(hit,`${platform} ${engine} inlet is visible`);assert.equal(hit.part.slot,'air_engine');
      const depth=z-hit.point[2];assert(depth>.45&&depth<.72,`${platform} ${engine}: throat depth ${depth}; no casing cap may block the mouth`);
    }
    assert.equal(mesh.parts.filter(p=>p.slot==='air_engine').length,twin?2:1,'Intake ducts do not invent extra engines or picking IDs');
  }
});
test('changed profiles remain inside existing shipping geometry allowances',()=>{
  for(const platform of ground)assert(build({platform}).triangleCount*108+32768<5000000,platform);
  for(const platform of ['tank_standard','tank_heavy','tank_light','tank_destroyer'])assert(build({platform,components:{suspension:'suspension_hydro',turret:'turret_heavy'}}).triangleCount*108+32768<12000000,platform);
  for(const platform of ['air_light_attack','air_tactical_strike'])assert(build({platform}).triangleCount<40000,platform);
});
test('affected armor, duct and curved shell configurations retain the full existing normal contract',()=>{
  const file=path.join(__dirname,'check_equipment_mesh.cjs'),src=fs.readFileSync(file,'utf8'),context={assert};
  vm.runInNewContext(src.slice(src.indexOf('function normalsContract('),src.indexOf('function validate('))+';this.verify=normalsContract;',context,{filename:file});
  for(const turret of ['turret_standard','turret_compact','turret_heavy','turret_autoload','turret_casemate'])context.verify(build({platform:'tank_standard',components:{turret}}));
  for(const platform of ground)context.verify(build({platform}));
  for(const platform of ['air_light_attack','air_tactical_strike'])for(const engine of ['air_engine_economical','air_engine_efficient','air_engine_twin'])context.verify(build({platform,components:{air_engine:engine}}));
});
