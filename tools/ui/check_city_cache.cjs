// Exercise the actual campaign CityMesh upload path and every allocation peak.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const ui=path.resolve(__dirname,'../../spheres-web/ui');
const page=fs.readFileSync(path.join(ui,'index.html'),'utf8');
const start=page.indexOf('function cityBuffers('),end=page.indexOf('\n}',start)+2;
assert(start>=0&&end>start);
const kit=require(path.join(ui,'city-mesh.js')),layer=require(path.join(ui,'city-layer.js'));
const records={window:{}};vm.runInNewContext(fs.readFileSync(path.join(ui,'cities.js'),'utf8'),records);
const cities=records.window.CITIES.slice().sort((a,b)=>b.pop-a.pop).slice(0,6);
function fixture({small=false}={}) {
  let current,live=0,peak=0,uploads=0,deleted=0,vaoDeleted=0;
  const sizes=new Map();
  const gl={ARRAY_BUFFER:1,STATIC_DRAW:2,FLOAT:3,createVertexArray:()=>({}),bindVertexArray(){},deleteVertexArray(){vaoDeleted++;},
    createBuffer:()=>({}),bindBuffer(target,buffer){current=buffer;},bufferData(target,data){
      live+=data.byteLength-(sizes.get(current)||0);sizes.set(current,data.byteLength);peak=Math.max(peak,live);uploads++;
    },deleteBuffer(buffer){assert(sizes.has(buffer),'no double deletion');live-=sizes.get(buffer);sizes.delete(buffer);deleted++;},
    enableVertexAttribArray(){},vertexAttribPointer(){}};
  const c={CityMesh:kit,CityLayer:{...layer,waterFootprint:()=>()=>false},Globe3D:{project:(x,y)=>[x,y]},
    GLR:{cityCache:new Map(),cityWater:()=>false,surface:{ready:true,sampleHeight:()=>0}},
    CITY_LAYER:{maxSpan:181,cellPx:4.5,cache:8,cacheTriangles:800000},DEFERRED:Symbol('deferred'),
    WaterDetail:{riverMask:()=>()=>false},RIVERS:{rivers:[]},mapDetailStyle:()=>({})};
  if(small)c.CityMesh={...kit,build:city=>kit.build(city,{maxSpan:9,waterSurface:false})};
  vm.createContext(c);vm.runInContext(page.slice(start,end),c);
  const view={terrainExaggeration:1,zoom:900};
  return {c,gl,view,stats:()=>({live,peak,uploads,deleted,vaoDeleted}),
    build(index,{city=cities[index%cities.length],span=181,defer=false}={}) {
      return c.cityBuffers(gl,{index,city,screenPx:span*4.5},view,defer);
    }};
}

test('main city cache never exceeds its unchanged payload ceiling, including transient uploads',()=>{
  const f=fixture();let supplied=0;
  for(let i=0;i<cities.length;i++) {
    const entry=f.build(i);assert(entry);supplied+=entry.tris;
    const held=[...f.c.GLR.cityCache.values()].reduce((sum,row)=>sum+row.tris,0);
    assert.equal(f.stats().live,held*108);
    assert(f.stats().peak<=f.c.CITY_LAYER.cacheTriangles*108,'reserve before uploading, not after');
  }
  assert(supplied>f.c.CITY_LAYER.cacheTriangles,'exercise eviction with real shipped meshes');
  assert(f.stats().deleted>0);assert.equal(f.stats().deleted,3*f.stats().vaoDeleted);
});

test('main city LRU caps entry count and reuse/reseating never duplicate attributes',()=>{
  const f=fixture({small:true});
  for(let i=0;i<8;i++)f.build(i);
  const kept=f.build(0),before=f.stats();
  assert.equal(f.build(0),kept);assert.equal(f.stats().uploads,before.uploads);
  f.build(8);assert.equal(f.c.GLR.cityCache.size,8);assert(f.c.GLR.cityCache.has(0));assert(!f.c.GLR.cityCache.has(1));
  const stored=f.stats();f.view.terrainExaggeration=2;
  assert.equal(f.build(0),kept);assert.equal(f.stats().uploads,stored.uploads+1,'only positions are reseated');
  assert.equal(f.stats().live,stored.live);
});

test('deferred cities neither allocate nor evict; larger replacement frees previous buffers',()=>{
  const f=fixture({small:true});const original=f.build(0,{span:40}),before=f.stats();
  assert.equal(f.build(1,{defer:true}),f.c.DEFERRED);
  assert.equal(f.build(0,{span:100,defer:true}),original);assert.deepEqual(f.stats(),before);
  assert.notEqual(f.build(0,{span:100}),original);assert.equal(f.stats().deleted,3);assert.equal(f.stats().vaoDeleted,1);
});

test('an oversized future city is refused before allocation or eviction',()=>{
  const f=fixture({small:true});f.build(0);const before=f.stats();
  f.c.CityLayer.place=()=>({triangleCount:f.c.CITY_LAYER.cacheTriangles+1});
  assert.equal(f.build(1),null);assert.deepEqual(f.stats(),before);assert.equal(f.c.GLR.cityCache.size,1);
});

test('Cities Off stops the real 3D city path before selection or allocation',()=>{
  const from=page.indexOf('function drawCityLayer('),to=page.indexOf('\n}',from)+2;
  const c={ui:{mapDetails:{cities:false}}};
  vm.createContext(c);vm.runInContext(page.slice(from,to),c);
  assert.equal(c.drawCityLayer({},1920,1080),0,'disabled path must not access the renderer or mesh builder');
});
