// Regression anchor for the terrain scatter PLACEMENT engine,
// `spheres-web/ui/world-scatter.js`.
//
// WHAT THIS DEFENDS. Roadmap section G asks for terrain-following placement
// that excludes water, excessive slope, important labels and transport
// corridors, on a zoom ladder that shows detail only at the close end. All of
// that is asserted below against samplers this file invents and therefore
// controls completely — an island, a cliff, a corridor — because a bar read
// against the real elevation raster proves only that the raster was loaded.
//
// THE BAR THAT MATTERS MOST is STABILITY UNDER PAN, and it is the reason this
// module exists at all rather than the kit's own local `scatter()`. A map is
// panned. If an instance's position, species or facing depends on the extent it
// was asked for, then every pan re-rolls the landscape, and a landscape that
// re-rolls cannot be saved as a seed — which is iron rule 1 restated for art.
// `barPan` plans a view, plans it again shifted by a fraction of a cell and
// again by seven and a bit cells, and demands that every instance present in
// both is identical field for field. It was watched going red against a
// deliberately extent-dependent generator before it was believed: that is
// `SABOTAGE` entry 1, and it shifts positions by three tenths of a metre.
//
// THE SECOND-ORDER VERSION of the same property is `barLadder`. Instances must
// not move when the ZOOM changes either — detail arrives by instances fading in
// where they always were, never by the field re-rolling denser. The ladder's
// strides and slot counts are chosen so a coarse rung is a strict subset of a
// fine one, and the bar asserts exactly that, at every rung, against the
// closest.
//
// AND IT WATCHES ITSELF GO RED. Iron rule 5: a test that cannot fail is worse
// than no test. `SABOTAGE` at the foot of this file is the ledger — ten
// deliberate defects, each patched into the source as a single-line edit, each
// run through the same sweep, each required to fail ON ITS NAMED BAR rather
// than merely somewhere, so a sabotage cannot quietly start passing for the
// wrong reason.
//
// THE VOCABULARY IS PINNED TO THE LIVE KIT. world-scatter.js carries a copy of
// scatter-mesh.js's biome mixes and footprints, because it must load with no
// loader in the browser. A copy rots. Every name, every weight and every radius
// is asserted against the real module here, so a rename or a reweight there
// fails in this file rather than putting a palm in Finland at run time.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const file=path.resolve(__dirname,'../../spheres-web/ui/world-scatter.js');
const kit=require(file);
const source=fs.readFileSync(file,'utf8');
const mesh=require(path.resolve(__dirname,'../../spheres-web/ui/scatter-mesh.js'));

// --------------------------------------------------------------- the fixture
// A quarter degree of the Ile-de-France, at a zoom well inside the city rung.
// Small enough that a whole sweep is milliseconds, large enough that the pan
// bar has thousands of instances to compare.
const VIEW={west:2,east:2.25,south:48,north:48.25};
const ZOOM=120;
/// An instance's IDENTITY: which world cell, which slot in it. Everything else
/// about the instance is a claim that must survive a pan, a zoom and a budget.
const id=i=>i.cell+"/"+i.slot;
const shifted=(view,d)=>({west:view.west+d,east:view.east+d,south:view.south+d,north:view.north+d});

// Synthetic terrain. Every one of these is a closed form so the bars can
// recompute what the module should have decided rather than trusting it.
/// Gentle enough that no slope is ever refused: 440 m of relief per degree is a
/// gradient of 0.006, a hundredth of the limit.
const rolling=(lon,lat)=>120+40*Math.sin(lon*11)+30*Math.cos(lat*9);
/// One island in the middle of the fixture and open sea around it.
const ISLE={lon:2.125,lat:48.125,r:0.08};
const island=(lon,lat)=>Math.hypot(lon-ISLE.lon,lat-ISLE.lat)<ISLE.r;
/// A cliff: 3000 m of rise over five hundredths of a degree of longitude, which
/// at 48 degrees is 3724 m of run — a gradient of 0.806, half again over the
/// 0.55 limit. Flat at 100 m to the west of it and 3100 m to the east.
const RAMP={x0:2.10,x1:2.15,rise:3000};
const cliff=(lon)=>{const t=(lon-RAMP.x0)/(RAMP.x1-RAMP.x0);return 100+RAMP.rise*(t<0?0:t>1?1:t);};
/// A cover index that sweeps the whole 0..1 range across the fixture, so the
/// biome table is exercised rather than pinned to one branch.
const cover=(lon)=>.5+.45*Math.sin(lon*13);
/// The corridor the host refuses: a box through the middle of the fixture,
/// standing in for a road, a rail line or a label's keep-out.
const BOX={west:2.08,east:2.16,south:48.08,north:48.16};
const inBox=(lon,lat)=>lon>BOX.west&&lon<BOX.east&&lat>BOX.south&&lat<BOX.north;

const full=extra=>Object.assign({zoom:ZOOM,seed:'spheres',budget:100000,
  land:()=>true,height:rolling,vegetation:cover},extra);

// ------------------------------------------------------------------ the bars
// Written as functions of the module so the sabotage ledger can run the exact
// same sweep against a deliberately broken copy. Every message is distinctive,
// because the ledger asserts WHICH bar caught each defect.

function barContract(mod,tag){
  const p=mod.plan(VIEW,full());
  for(const key of ['items','cells','culled','budget'])
    assert.ok(Object.prototype.hasOwnProperty.call(p,key),`${tag}plan is missing ${key}`);
  assert.ok(Array.isArray(p.items),`${tag}items is not a list`);
  assert.ok(p.items.length>0,`${tag}the fixture emitted nothing at all`);
  assert.equal(typeof p.cells,'number',`${tag}cells is not a count`);
  for(const reason of ['water','slope','excluded','noHeight','budget'])
    assert.equal(typeof p.culled[reason],'number',`${tag}culled.${reason} is not a count`);
  for(const i of p.items){
    for(const f of ['lon','lat','elevation','yaw','scale'])
      assert.ok(Number.isFinite(i[f]),`${tag}${f} is not finite`);
    assert.ok(i.lon>=-180&&i.lon<180,`${tag}longitude ${i.lon} is outside the world`);
    assert.ok(i.lat>=-90&&i.lat<=90,`${tag}latitude ${i.lat} is outside the world`);
    assert.ok(i.yaw>=0&&i.yaw<Math.PI*2,`${tag}yaw ${i.yaw} is not one turn`);
    assert.ok(i.scale>=.72&&i.scale<=1.34,`${tag}scale ${i.scale} is outside the kit's range`);
    assert.ok(i.lod===0||i.lod===1,`${tag}lod ${i.lod} is neither near nor far`);
    assert.equal(typeof i.kind,'string',`${tag}kind is not a name`);
    assert.equal(typeof i.biome,'string',`${tag}biome is not a name`);
    // A cell that intersects the extent is emitted WHOLE, so an item may lie up
    // to one cell outside it. That skirt is documented and it is bounded: an
    // item further out than one cell means the walk is reading the wrong cells.
    const skirt=mod.cellSize;
    assert.ok(i.lon>=VIEW.west-skirt&&i.lon<=VIEW.east+skirt,
      `${tag}an instance at ${i.lon} is more than one cell outside the extent`);
    assert.ok(i.lat>=VIEW.south-skirt&&i.lat<=VIEW.north+skirt,
      `${tag}an instance at ${i.lat} is more than one cell outside the extent`);
  }
}

function barDeterminism(mod,tag){
  const a=mod.plan(VIEW,full()),b=mod.plan(VIEW,full());
  assert.equal(JSON.stringify(a.items),JSON.stringify(b.items),
    `${tag}two identical calls in one process disagreed`);
  const other=mod.plan(VIEW,full({seed:'a different world'}));
  assert.notEqual(JSON.stringify(other.items),JSON.stringify(a.items),
    `${tag}the seed does nothing`);
}

/// THE ONE. Two shifts: a fraction of a cell, which does not change which cells
/// are walked and so catches a generator that reads the extent directly, and
/// seven and a bit cells, which does change them and so catches a generator
/// that lays instances out from the corner of whatever it was asked for.
function barPan(mod,tag){
  const o=full();
  const a=mod.plan(VIEW,o);
  // A third of a cell, which does not change WHICH cells are walked; seven and
  // a bit, which does; and a third of the extent, which leaves less than half
  // of it in common and so proves the bar is not comparing a view with itself.
  for(const cells of [0.37,7.37,21.5]){
    const d=mod.cellSize*cells;
    const b=mod.plan(shifted(VIEW,d),o);
    const seen=new Map(a.items.map(i=>[id(i),i]));
    let overlap=0;
    for(const i of b.items){
      const j=seen.get(id(i));
      if(!j)continue;
      overlap+=1;
      for(const f of ['lon','lat','elevation','kind','biome','yaw','scale','lod','seed'])
        assert.equal(i[f],j[f],`${tag}a pan of ${cells} cells moved the ${f} of instance ${id(i)}`);
    }
    assert.ok(overlap>500,`${tag}a pan of ${cells} cells left only ${overlap} instances in common, too few to prove anything`);
  }
}

/// The same property against ZOOM. A coarse rung must be a strict subset of a
/// fine one: instances appear, they never move and they never change species.
function barLadder(mod,tag){
  const rungs=[32,56,96,192];
  const near=new Map(mod.plan(VIEW,full({zoom:192})).items.map(i=>[id(i),i]));
  let prev=-1;
  for(const z of rungs){
    const p=mod.plan(VIEW,full({zoom:z}));
    assert.ok(p.items.length>=prev,`${tag}zoom ${z} emitted fewer instances than the rung below it`);
    prev=p.items.length;
    for(const i of p.items){
      const j=near.get(id(i));
      assert.ok(j,`${tag}instance ${id(i)} exists at zoom ${z} but not at city zoom`);
      for(const f of ['lon','lat','kind','biome','yaw','scale'])
        assert.equal(i[f],j[f],`${tag}zooming to ${z} moved the ${f} of instance ${id(i)}`);
    }
  }
}

function barBudget(mod,tag){
  const o=full({budget:0});
  for(const n of [0,1,10,250,1500]){
    const p=mod.plan(VIEW,Object.assign({},o,{budget:n}));
    assert.ok(p.items.length<=n,`${tag}a budget of ${n} returned ${p.items.length} instances`);
    assert.equal(p.budget,n,`${tag}the plan does not report the budget it was given`);
  }
  // The cull is STABLE: a smaller budget is a strict PREFIX of a larger one, so
  // the same instances survive every frame instead of the draw changing.
  const big=mod.plan(VIEW,Object.assign({},o,{budget:2000}));
  const small=mod.plan(VIEW,Object.assign({},o,{budget:200}));
  assert.equal(small.items.length,200,`${tag}the smaller budget did not fill`);
  assert.ok(big.items.length>small.items.length,`${tag}the larger budget bought nothing`);
  for(let i=0;i<small.items.length;i+=1)
    assert.equal(id(small.items[i]),id(big.items[i]),
      `${tag}shrinking the budget drew a different field at position ${i}`);
  assert.ok(big.culled.budget>0,`${tag}nothing was reported as culled for budget`);
}

function barWater(mod,tag){
  const p=mod.plan(VIEW,full({land:island}));
  assert.ok(p.items.length>50,`${tag}the island emitted almost nothing`);
  for(const i of p.items)
    assert.ok(island(i.lon,i.lat),`${tag}an instance stands in water at ${i.lon}, ${i.lat}`);
  assert.ok(p.culled.water>0,`${tag}nothing was refused for water`);
  const drowned=mod.plan(VIEW,full({land:()=>false}));
  assert.equal(drowned.items.length,0,`${tag}an all-sea extent still emitted instances`);
}

function barSlope(mod,tag){
  const p=mod.plan(VIEW,full({height:cliff}));
  for(const i of p.items){
    const s=mod.slopeAt(cliff,i.lon,i.lat);
    assert.ok(s<=mod.slopeLimit+1e-9,`${tag}an instance stands on a slope of ${s}`);
  }
  assert.ok(p.culled.slope>0,`${tag}nothing was refused for slope`);
  const face=p.items.filter(i=>i.lon>RAMP.x0+.005&&i.lon<RAMP.x1-.005);
  assert.equal(face.length,0,`${tag}${face.length} instances stand on the cliff face`);
  const flat=mod.plan(VIEW,full({height:()=>50}));
  assert.ok(flat.items.length>p.items.length,`${tag}the cliff refused nothing that flat ground kept`);
  // And the arithmetic itself: a degree of longitude is shorter than a degree
  // of latitude away from the equator, so the same rise over the same degree
  // offset is a STEEPER slope east-west. A generator that used one constant for
  // both would read this as equal.
  const east=mod.slopeAt((lon)=>lon*1000,0,60);
  const north=mod.slopeAt((lon,lat)=>lat*1000,0,60);
  assert.ok(east>north*1.9,`${tag}the degree-to-metre conversion does not vary with latitude`);
}

function barExclude(mod,tag){
  const radii=[];
  const exclude=(at,r)=>{radii.push(r);return inBox(at[0],at[1]);};
  const p=mod.plan(VIEW,full({exclude}));
  for(const i of p.items)
    assert.ok(!inBox(i.lon,i.lat),`${tag}an instance stands inside the refused corridor`);
  assert.ok(p.culled.excluded>0,`${tag}nothing was refused by the host`);
  assert.ok(radii.length>0,`${tag}the exclusion callback was never asked`);
  for(const r of radii)
    assert.ok(r>0&&r<.001,`${tag}the exclusion radius ${r} is not a small positive count of degrees`);
  // The radius must be the kind's own footprint, not one constant: a grass tuft
  // asks for less room than an oak.
  assert.ok(Math.max(...radii)>Math.min(...radii)*3,
    `${tag}every kind asked the exclusion callback for the same room`);
}

function barZoom(mod,tag){
  for(const z of [0,1,8,20,31,mod.minZoom-1e-9]){
    const p=mod.plan(VIEW,full({zoom:z}));
    assert.equal(p.items.length,0,
      `${tag}zoom ${z} emitted ${p.items.length} instances below the detail threshold`);
  }
  const on=mod.plan(VIEW,full({zoom:mod.minZoom}));
  assert.ok(on.items.length>0,`${tag}the threshold zoom itself emitted nothing`);
}

function barTerrain(mod,tag){
  const p=mod.plan(VIEW,full({budget:2000}));
  assert.ok(p.items.length>0,`${tag}nothing to stand on the terrain`);
  for(const i of p.items)
    assert.equal(i.elevation,rolling(i.lon,i.lat),
      `${tag}an instance floats: it carries ${i.elevation} where the terrain is ${rolling(i.lon,i.lat)}`);
  const nothing=mod.plan(VIEW,full({height:()=>null}));
  assert.equal(nothing.items.length,0,`${tag}an extent with no elevation data still emitted instances`);
  assert.ok(nothing.culled.noHeight>0,`${tag}missing elevation was not reported`);
}

function barBiome(mod,tag){
  const c=mod.classify;
  const table=[
    [.80,5,50,'tropical','wet equator'],
    [.30,5,50,'savanna','open equator'],
    [.02,25,300,'arid','bare subtropics'],
    [.60,46,3000,'alpine','above the treeline'],
    [.80,0,4500,'alpine','above the tropical treeline'],
    [.50,75,200,'tundra','the polar lowland, which is not alpine'],
    [.10,58,150,'tundra','bare subpolar'],
    [.50,60,200,'boreal','vegetated subpolar'],
    [.30,35,100,'mediterranean','dry warm temperate'],
    [.60,50,100,'temperate_conifer','the pine belt'],
    [.60,45,1200,'temperate_conifer','temperate upland'],
    [.60,45,100,'temperate_broadleaf','temperate lowland'],
    [null,45,100,'temperate_broadleaf','with no cover at all'],
    [null,5,50,'savanna','the tropics with no cover at all'],
  ];
  for(const [veg,lat,elev,want,why] of table)
    assert.equal(c(veg,lat,elev),want,`${tag}${why} should classify as ${want}`);
  for(const lat of [-60,-20,0,20,60])
    assert.equal(c(.4,lat,100),c(.4,-lat,100),`${tag}the classification is not symmetric about the equator`);
  // And the plan must actually USE the table it documents.
  const p=mod.plan(VIEW,full({budget:1500}));
  for(const i of p.items)
    assert.equal(i.biome,c(cover(i.lon),i.lat,i.elevation),
      `${tag}the plan's biome disagrees with the classification it documents`);
}

function barVocabulary(mod,tag){
  // Compared as TEXT, not with deepStrictEqual. The sabotage ledger runs this
  // same sweep against a module loaded into a vm context, whose arrays carry
  // that realm's Array.prototype — deepStrictEqual checks prototypes and would
  // fail every sabotage here for a reason that has nothing to do with the
  // defect being exercised. The same lesson is written up in
  // check_scatter_mesh.cjs, and it cost a wrong reading there first.
  assert.equal(mod.biomes().slice().sort().join(','),mesh.biomes().slice().sort().join(','),
    `${tag}the biome vocabulary has drifted from scatter-mesh`);
  for(const b of mesh.biomes())
    assert.equal(JSON.stringify(mod.mixFor(b)),JSON.stringify(mesh.biomeInfo(b).mix),
      `${tag}the ${b} mix has drifted from scatter-mesh`);
  for(const k of mod.kinds()){
    const info=mesh.kindInfo(k);
    assert.ok(info,`${tag}kind ${k} does not exist in scatter-mesh`);
    assert.equal(mod.radiusFor(k),info.radius,`${tag}the footprint of ${k} has drifted from scatter-mesh`);
    assert.ok(mesh.scatterable.includes(k),`${tag}kind ${k} is not one scatter-mesh will scatter`);
  }
  const p=mod.plan(VIEW,full({budget:4000}));
  const kinds=new Set(),biomes=new Set();
  for(const i of p.items){
    assert.ok(mesh.kindInfo(i.kind),`${tag}emitted kind ${i.kind} does not exist in scatter-mesh`);
    assert.ok(mesh.biomeInfo(i.biome),`${tag}emitted biome ${i.biome} does not exist in scatter-mesh`);
    kinds.add(i.kind);biomes.add(i.biome);
  }
  assert.ok(kinds.size>=4,`${tag}the fixture only produced ${kinds.size} kinds`);
  assert.ok(biomes.size>=2,`${tag}the cover sweep only produced ${biomes.size} biomes`);
}

/// Every sampler is optional and every missing one must degrade to a documented
/// default. A plan that throws because the host has no cover raster yet is a
/// blank screen; a plan that says what it could not check is a working one.
function barDegrades(mod,tag){
  assert.doesNotThrow(()=>mod.plan(VIEW,{zoom:ZOOM}),`${tag}a plan with no samplers threw`);
  const bare=mod.plan(VIEW,{zoom:ZOOM,seed:'bare',budget:200});
  assert.equal(bare.assumed.land,true,`${tag}a missing land test was not reported`);
  assert.equal(bare.assumed.height,true,`${tag}a missing elevation sampler was not reported`);
  assert.equal(bare.assumed.vegetation,true,`${tag}a missing cover sampler was not reported`);
  assert.ok(bare.items.length>0,`${tag}a plan with no samplers emitted nothing`);
  for(const i of bare.items)
    assert.equal(i.elevation,0,`${tag}an elevation appeared without an elevation sampler`);
  assert.ok(/zonation/.test(bare.description),`${tag}the description does not admit the cover fallback`);
  const withCover=mod.plan(VIEW,{zoom:ZOOM,seed:'bare',budget:200,vegetation:cover});
  assert.ok(!/zonation/.test(withCover.description),`${tag}the cover caveat is printed even with cover`);
  for(const bad of [null,undefined,{},{west:'a',east:1,south:0,north:1},
    {west:0,east:0,south:0,north:0},{west:NaN,east:1,south:0,north:1}]){
    let p=null;
    assert.doesNotThrow(()=>{p=mod.plan(bad,{zoom:ZOOM});},`${tag}a malformed extent threw`);
    assert.equal(p.items.length,0,`${tag}a malformed extent emitted instances`);
  }
  // Samplers that answer with nothing are answers, not failures.
  const blind=mod.plan(VIEW,{zoom:ZOOM,seed:'blind',budget:200,
    height:()=>NaN,vegetation:()=>NaN,land:()=>true});
  assert.equal(blind.items.length,0,`${tag}an elevation sampler with no data still placed instances`);
  // And the description always says what the plan is not.
  const p=mod.plan(VIEW,full({budget:100}));
  assert.ok(/not a land-cover measurement/.test(p.description),
    `${tag}the description does not refuse to be a land-cover claim`);
  assert.ok(/grants no forest/.test(p.description),
    `${tag}the description does not refuse to grant anything`);
}

/// The antimeridian. west > east is an extent through the seam, and a world
/// cell has one longitude however the extent that found it was written.
function barSeam(mod,tag){
  const p=mod.plan({west:179.9,east:-179.9,south:0,north:.1},
    {zoom:ZOOM,seed:'seam',budget:5000,land:()=>true,height:rolling});
  assert.ok(p.items.length>0,`${tag}an extent across the seam emitted nothing`);
  for(const i of p.items)
    assert.ok(i.lon>=-180&&i.lon<180,`${tag}longitude ${i.lon} is outside the world`);
  assert.ok(p.items.some(i=>i.lon>179),`${tag}the seam extent found nothing east of 179`);
  assert.ok(p.items.some(i=>i.lon<-179),`${tag}the seam extent found nothing west of -179`);
}

function sweep(mod,tag){
  barContract(mod,tag);
  barDeterminism(mod,tag);
  barPan(mod,tag);
  barLadder(mod,tag);
  barBudget(mod,tag);
  barWater(mod,tag);
  barSlope(mod,tag);
  barExclude(mod,tag);
  barZoom(mod,tag);
  barTerrain(mod,tag);
  barBiome(mod,tag);
  barVocabulary(mod,tag);
  barDegrades(mod,tag);
  barSeam(mod,tag);
}

// ----------------------------------------------------------------- the tests
test('the plan contract holds and every instance is inside the extent it was asked for',()=>{
  barContract(kit,'');
});

test('the same extent and seed plan the same field twice, and in a fresh load',()=>{
  barDeterminism(kit,'');
  // A FRESH LOAD, not a second call: module-level state that leaks into a plan
  // survives a repeated call inside one process and does not survive this.
  delete require.cache[require.resolve(file)];
  const again=require(file);
  assert.notEqual(again,kit,'the module was not actually reloaded');
  assert.equal(JSON.stringify(again.plan(VIEW,full()).items),
    JSON.stringify(kit.plan(VIEW,full()).items),
    'a freshly loaded module planned a different field');
});

test('an instance survives a pan byte-identical: position, kind, yaw and scale',()=>{
  barPan(kit,'');
});

test('an instance survives a zoom too: a coarse rung is a subset of a fine one',()=>{
  barLadder(kit,'');
  // The ladder is a function of zoom alone, and its shape is documented.
  assert.ok(kit.ladder.length>0,'there is no ladder');
  for(let i=1;i<kit.ladder.length;i+=1){
    assert.ok(kit.ladder[i].zoom<kit.ladder[i-1].zoom,'the ladder is not ordered');
    assert.ok(kit.ladder[i].stride>=kit.ladder[i-1].stride,'a wider view walks more cells');
    assert.ok(kit.ladder[i].slots<=kit.ladder[i-1].slots,'a wider view offers more slots');
  }
  assert.equal(kit.ladder[kit.ladder.length-1].zoom,kit.minZoom,
    'the bottom of the ladder is not the stated threshold');
});

test('the budget is a hard cap and the cull is a stable prefix',()=>{
  barBudget(kit,'');
});

test('water, steep ground and the host exclusion are all actually refused',()=>{
  barWater(kit,'');
  barSlope(kit,'');
  barExclude(kit,'');
});

test('nothing at all is emitted above the zoom-out threshold',()=>{
  barZoom(kit,'');
});

test('every instance carries the elevation the terrain sampler gave it',()=>{
  barTerrain(kit,'');
});

test('the biome is the documented classification of cover, latitude and elevation',()=>{
  barBiome(kit,'');
});

test('every kind and biome exists in the live scatter-mesh, with its weights and footprints',()=>{
  barVocabulary(kit,'');
});

test('a missing sampler degrades to a stated default rather than throwing',()=>{
  barDegrades(kit,'');
});

test('an extent across the antimeridian works and normalises its longitudes',()=>{
  barSeam(kit,'');
});

test('the cell grid is a whole-degree power of two and the stride lattice wraps',()=>{
  // Both of these are load-bearing and neither is obvious from the number.
  // A cell index has to be an exact integer at every longitude, and the
  // coarsening lattice has to divide the globe or the antimeridian grows a seam
  // of double or missing density.
  assert.equal(kit.cellsPerDegree&(kit.cellsPerDegree-1),0,'the cell size is not a power of two per degree');
  assert.equal(kit.gridSize[0],360*kit.cellsPerDegree,'the global grid is not whole degrees wide');
  assert.equal(kit.gridSize[1],180*kit.cellsPerDegree,'the global grid is not whole degrees tall');
  for(const stride of [1,2,4,8,16,32,64,128,256,512,1024]){
    assert.equal(kit.gridSize[0]%stride,0,`stride ${stride} does not divide the world in longitude`);
    assert.equal(kit.gridSize[1]%stride,0,`stride ${stride} does not divide the world in latitude`);
  }
  assert.ok(Math.abs(kit.cellSize*110540-431.8)<1,'the cell is not the documented 432 m of latitude');
});

test('a view too wide for the walk guard thins instead of stalling, and thins stably',()=>{
  // The guard is derived from the SPAN, which a pan does not change, so a wide
  // view still cannot re-roll under the camera.
  const wide={west:2,east:22,south:20,north:40};
  const o={zoom:192,seed:'wide',budget:3000,land:()=>true,height:rolling,vegetation:cover};
  const a=kit.plan(wide,o);
  assert.ok(a.cells<=kit.maxCells,`the guard walked ${a.cells} cells`);
  assert.ok(a.stride>1,'the guard did not coarsen a twenty-degree view');
  assert.equal(a.items.length,3000,'the wide view did not fill its budget');
  const d=kit.cellSize*3.3;
  const b=kit.plan({west:wide.west+d,east:wide.east+d,south:wide.south+d,north:wide.north+d},o);
  assert.equal(b.stride,a.stride,'a pan changed the guard');
  const seen=new Map(a.items.map(i=>[id(i),i]));
  let overlap=0;
  for(const i of b.items){
    const j=seen.get(id(i));
    if(!j)continue;
    overlap+=1;
    assert.equal(i.lon,j.lon,'a pan moved an instance in a guarded view');
    assert.equal(i.kind,j.kind,'a pan changed a species in a guarded view');
  }
  assert.ok(overlap>100,`a guarded pan left only ${overlap} instances in common`);
});

test('no wall clock, no entropy source and no browser can reach this placement',()=>{
  // Iron rule 1. A scatter field is stored as a seed and a set of extents; a
  // clock or an entropy source here would make a saved landscape unreproducible
  // and would not necessarily show up as a failing digest inside one process.
  // The cheapest way to keep it true is for the vocabulary not to exist at all.
  for(const pattern of [/Math\s*\.\s*random/i,/\brandom\b/i,/\bDate\b/,/performance/i,
    /\bhrtime\b/,/\bnow\s*\(/,/requestAnimationFrame/i,/\bsetTimeout\b/,/\bsetInterval\b/,
    /getTime/,/\bcrypto\b/i])
    assert.ok(!pattern.test(source),`world-scatter.js must not contain ${pattern}`);
  // And no host, no loader and no graphics context, so a plan is checkable
  // outside a browser and the module loads from a plain script tag.
  for(const pattern of [/\bdocument\b/,/\bwindow\b/i,/\bcanvas\b/i,/WebGL/i,
    /require\s*\(/,/\bimport\b/,/\bfetch\b/i])
    assert.ok(!pattern.test(source),`world-scatter.js must not contain ${pattern}`);
});

test('the browser global exports the same contract and plans the identical field',()=>{
  const context=vm.createContext({});
  vm.runInContext(source,context);
  const global=context.WorldScatter;
  for(const f of ['plan','classify','rung','kinds','biomes','mixFor','radiusFor','slopeAt','treeline'])
    assert.equal(typeof global[f],'function',`WorldScatter.${f}`);
  assert.equal(JSON.stringify(global.plan(VIEW,full()).items),
    JSON.stringify(kit.plan(VIEW,full()).items),
    'the browser path plans a different field to the node path');
});

// ---------------------------------------------------------------------------
// THE SABOTAGE LEDGER. Ten deliberate defects, each a SINGLE-LINE edit — the
// module is checked out CRLF, so a multi-line anchor written with \n never
// matches and the sabotage silently becomes a stale-anchor failure instead of
// the bar it meant to exercise. Each must fail on the NAMED bar.
const SABOTAGE=[
  {defect:'placement reads the extent it was asked for, shifting instances by a third of a metre',
    expect:'moved the lon of instance',
    edits:[['        const lon = (gx + askSpan(slotSeed, 1, 0.06, 0.94)) * CELL - 180;',
      '        const lon = (gx + askSpan(slotSeed, 1, 0.06, 0.94)) * CELL - 180 + (west - Math.floor(west)) * 1e-4;']]},
  {defect:'the budget overruns by five',
    expect:'returned',
    edits:[['        if (items.length >= budget) break;',
      '        if (items.length >= budget + 5) break;']]},
  {defect:'the cull order depends on the budget, so shrinking it redraws the field',
    expect:'drew a different field at position',
    edits:[['          const rank = ask(mix32(cellSeed ^ Math.imul(s + 1, 0x9e3779b1)), 0);',
      '          const rank = ask(mix32(cellSeed ^ Math.imul(s + 1, 0x9e3779b1)) ^ (budget | 0), 0);']]},
  {defect:'water is counted but not refused',
    expect:'stands in water at',
    edits:[['        if (land && !land(lon, lat)) { culled.water += 1; continue; }',
      '        if (land && !land(lon, lat)) { culled.water += 1; }']]},
  {defect:'steep ground is counted but not refused',
    expect:'stands on a slope of',
    edits:[['            if (Math.hypot(dx, dy) > slopeLimit) { culled.slope += 1; continue; }',
      '            if (Math.hypot(dx, dy) > slopeLimit) { culled.slope += 1; }']]},
  {defect:'the host exclusion is asked and then ignored',
    expect:'stands inside the refused corridor',
    edits:[['          if (exclude([lon, lat], wide)) { culled.excluded += 1; continue; }',
      '          if (exclude([lon, lat], wide)) { culled.excluded += 1; }']]},
  {defect:'the zoom-out threshold is removed, so a continent gets a sprinkle',
    expect:'below the detail threshold',
    edits:[['  const MIN_ZOOM = 32;','  const MIN_ZOOM = 0;']]},
  {defect:'the instance stops following the terrain and sits at sea level',
    expect:'floats: it carries',
    edits:[['          elevation = h;','          elevation = 0;']]},
  {defect:'the alpine rule is disabled, so a mountain grows lowland pine',
    expect:'above the treeline should classify as alpine',
    edits:[['    if (e > treeline(y)) return "alpine";','    if (false) return "alpine";']]},
  {defect:'a kind is renamed away from the one scatter-mesh builds',
    expect:'mix has drifted from scatter-mesh',
    edits:[['    temperate_broadleaf: [["oak", 30], ["hazel_scrub", 18], ["grass_tuft", 38], ["boulder", 9], ["rock_cluster", 5]],',
      '    temperate_broadleaf: [["oaks", 30], ["hazel_scrub", 18], ["grass_tuft", 38], ["boulder", 9], ["rock_cluster", 5]],']]},
];

function sabotaged(edits){
  let broken=source;
  for(const [from,to] of edits){
    assert.equal(broken.split(from).length-1,1,`sabotage anchor is stale or not unique: ${from.slice(0,60)}`);
    broken=broken.replace(from,to);
  }
  const context=vm.createContext({});
  vm.runInContext(broken,context);
  return context.WorldScatter;
}

test('every bar above goes red against a deliberately broken placement engine',()=>{
  for(const entry of SABOTAGE){
    const mod=sabotaged(entry.edits);
    let caught=null;
    try{sweep(mod,'sabotaged ');}catch(err){caught=err;}
    assert.ok(caught,`sabotage passed unnoticed: ${entry.defect}`);
    assert.ok(String(caught.message).includes(entry.expect),
      `sabotage "${entry.defect}" failed on the wrong bar: ${caught.message}`);
  }
  // And the ledger only means anything if the same sweep is green on the real
  // source. The tests above already assert that one bar at a time; this restates
  // it so a reader of this block alone can see both halves.
  sweep(kit,'live ');
});
