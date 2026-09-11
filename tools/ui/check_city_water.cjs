// Focused W6 regressions: actual bundled water data, no server or external asset.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),zlib=require('node:zlib');
const root=path.resolve(__dirname,'../..'),ui=path.join(root,'spheres-web/ui');
const assets=ui;
const layer=require(path.join(ui,'city-layer.js')),kit=require(path.join(ui,'city-mesh.js'));
const c=vm.createContext({window:{},console});
for(const [base,file] of [[assets,'rivers.js'],[assets,'cities.js'],[assets,'globe3d.js'],[ui,'water-detail.js']])
  vm.runInContext(fs.readFileSync(path.join(base,file),'utf8'),c,{filename:file});
const water=c.window.WaterDetail,project=c.window.Globe3D.project;
function grayPNG(file){
  const bytes=fs.readFileSync(file),parts=[];let w,h,depth,type;
  for(let p=8;p<bytes.length;){const n=bytes.readUInt32BE(p),tag=bytes.toString('ascii',p+4,p+8),data=bytes.subarray(p+8,p+8+n);
    if(tag==='IHDR'){w=data.readUInt32BE(0);h=data.readUInt32BE(4);depth=data[8];type=data[9];assert.equal(data[12],0);}
    if(tag==='IDAT')parts.push(data);p+=n+12;}
  assert.equal(depth,8);assert.equal(type,0,'Bundled masks must remain L8');
  const raw=zlib.inflateSync(Buffer.concat(parts)),out=new Uint8Array(w*h);assert.equal(raw.length,(w+1)*h);
  const paeth=(a,b,c)=>{const p=a+b-c,pa=Math.abs(p-a),pb=Math.abs(p-b),pc=Math.abs(p-c);return pa<=pb&&pa<=pc?a:pb<=pc?b:c;};
  for(let y=0;y<h;y++){const filter=raw[y*(w+1)];assert(filter<=4);for(let x=0;x<w;x++){
    const at=y*w+x,a=x?out[at-1]:0,b=y?out[at-w]:0,d=x&&y?out[at-w-1]:0;
    out[at]=(raw[y*(w+1)+1+x]+[0,a,b,Math.floor((a+b)/2),paeth(a,b,d)][filter])&255;
  }}return{w,h,data:out};
}
const coast=grayPNG(path.join(assets,'coast.png')),lake=grayPNG(path.join(assets,'lake.png'));
const worldH=project(0,-58)[1];
const rendered=layer.waterSampler(coast.data,lake.data,coast.w,coast.h,2400,worldH);
const style=zoom=>({riverOpacity:.82,riverWidth:.34*Math.min(1,10/zoom)});
function recording(){const calls=[];return {calls,save(){},restore(){},stroke(){calls.push({width:this.lineWidth,color:this.strokeStyle});}};}
function fixture(name){
  const city=c.window.CITIES.find(row=>row.name===name);assert(city,name);
  const half=kit.extentMetres(city.pop)*.6,metres=kit.metresPerDegree(city.lat);
  const pts=[-1,1].flatMap(dx=>[-1,1].map(dy=>project(city.lon+dx*half/metres.lon,city.lat+dy*half/metres.lat)));
  const bounds=[Math.min(...pts.map(p=>p[0])),Math.min(...pts.map(p=>p[1])),Math.max(...pts.map(p=>p[0])),Math.max(...pts.map(p=>p[1]))];
  const rivers=water.riverMask(c.window.RIVERS.rivers,bounds,style(750),750);
  const mask=layer.waterFootprint(city,project,rendered,rivers);
  return{city,rivers,mask,metres};
}
test('terrain river channels and their casings continue shrinking through zoom1500',()=>{
  const course=[{n:'Fixture',d:'M0 0L200 0'}];let previous=Infinity;
  for(const zoom of [12,48,192,750,1500]){
    const ctx=recording();water.paint(ctx,course,style(zoom),{terrainView:true,zoom,path:s=>s});
    assert.equal(ctx.calls.length,2);const outer=ctx.calls[0].width;
    assert(outer<previous);assert(outer<=6.4/zoom,'Fixed map-unit casing overwhelms near view');previous=outer;
  }
});
test('rendered coast decode and lake encode filtering own water even above sea level',()=>{
  const land=layer.waterSampler(new Uint8Array([0,255,255,255]),new Uint8Array([255,255,0,255]),2,2,2,2);
  assert.equal(land(.5,.5),true);assert.equal(land(1.5,.5),false);
  assert.equal(land(.5,1.5),true);assert.equal(land(1.5,1.5),false);
  assert.equal(land(-1,0),true);
  // Coast filters decoded distances, not encoded bytes: these straddle the
  // inherited -.15 shoreline threshold differently under the wrong ordering.
  const coastOnly=layer.waterSampler(new Uint8Array([100,150]),new Uint8Array([255,255]),2,1,2,1);
  assert.equal(coastOnly(.75,.5),true);assert.equal(coastOnly(1.25,.5),false);
});
test('Paris keeps dry blocks but no generated lot intersects the displayed Seine corridor',()=>{
  const {city,rivers,mask,metres}=fixture('Paris');
  const plan=kit.plan(city,{maxSpan:181,water:mask});
  assert(plan.counts.water>0);assert(plan.counts.plot>100);
  for(let j=0;j<plan.span;j++)for(let i=0;i<plan.span;i++){
    if(plan.cls[j*plan.span+i]!==kit.classes.PLOT)continue;
    for(const lot of kit.lots(plan,i,j)){
      const x=(lot.x0+lot.x1)/2,z=(lot.z0+lot.z1)/2;
      const p=project(city.lon+x/metres.lon,city.lat-z/metres.lat);
      assert.equal(rivers(p[0],p[1]),false,'A building center entered the bundled river');
      for(const xx of [lot.x0,lot.x1])for(const zz of [lot.z0,lot.z1]){
        const q=project(city.lon+xx/metres.lon,city.lat-zz/metres.lat);
        assert.equal(rivers(q[0],q[1]),false,'A building corner crossed the bundled river');
      }
    }
  }
});
test('Singapore blocks follow rendered ocean and globe mesh adds no second water plate',()=>{
  const {city,mask,metres}=fixture('Singapore');
  const dry=kit.plan(city,{maxSpan:81}),wet=kit.plan(city,{maxSpan:81,water:mask});
  assert(wet.counts.water>0);assert(wet.counts.plot>0,'Keep dry island blocks when the rendered mask supports them');
  assert(wet.counts.plot<dry.counts.plot);
  for(let j=0;j<wet.span;j++)for(let i=0;i<wet.span;i++)if(wet.cls[j*wet.span+i]===kit.classes.PLOT){
    const p=project(city.lon+(i-wet.half)*wet.cell/metres.lon,city.lat-(j-wet.half)*wet.cell/metres.lat);
    assert.equal(rendered(p[0],p[1]),false,'A procedural block stands in rendered ocean/lake');
  }
  const mesh=kit.build(city,{maxSpan:81,water:mask,waterSurface:false});
  assert(mesh.triangleCount>0);assert.equal(mesh.assumed.land,false);
  const card=kit.build(city,{maxSpan:81,water:mask});
  assert(card.parts.some(p=>p.kind==='water'),'The standalone default still draws its water plate');
  assert(card.triangleCount>mesh.triangleCount);
  assert(!mesh.parts.some(p=>p.kind==='water'),'The globe must draw its actual water, not a city water plane');
});
test('default card geometry remains exactly unchanged when optional water support is unused',()=>{
  const city=c.window.CITIES.find(row=>row.name==='Paris');
  const before=kit.build(city,{lod:'map'}),after=kit.build(city,{lod:'map',water:null,waterSurface:true});
  assert.deepEqual(after.positions,before.positions);assert.deepEqual(after.colors,before.colors);
});
