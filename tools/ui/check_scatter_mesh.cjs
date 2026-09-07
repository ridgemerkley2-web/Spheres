// Regression anchor for the terrain scatter kit, `spheres-web/ui/scatter-mesh.js`.
//
// WHAT THIS DEFENDS. Roadmap section G asks for biome trees, scrub, grass,
// rocks, cliffs, snow edges, shores, river banks and field patches, and section
// 4 prices them. The two ways to get that wrong are to ship geometry the
// renderer chokes on, and to ship a landscape that is a different landscape
// after a reload. Everything below is aimed at one or the other. The budgets
// are section 4's; the determinism bar is iron rule 1 restated for art, and it
// matters more here than anywhere else in the art, because a scatter field is
// stored as a SEED. If the seed does not rebuild the same field, the field
// cannot be saved.
//
// IT READS THE SOURCE AS WELL AS THE MESHES. A wall-clock call or an entropy
// source does not necessarily show up as a failing digest inside one process,
// and it is exactly the kind of thing that survives review.
//
// AND IT WATCHES ITSELF GO RED. Iron rule 5: a test that cannot fail is worse
// than no test. `SABOTAGE` and `SABOTAGE_WIDE` at the foot of this file are the
// ledger — seventeen deliberate defects, each patched into the source, each run
// through the same bars, each required to fail. The eight in `SABOTAGE` must
// fail on the NAMED assertion, not merely somewhere, so a sabotage cannot
// quietly start passing for the wrong reason.
//
// Writing it that way earned its keep twice. It found that a mesh could float
// while its own bounding box claimed it was seated, which is why ground contact
// is now measured off the buffer; and it found that the per-corner facing bar
// CANNOT see a globally flipped winding, because normals here are derived from
// the geometry rather than authored, so a flipped face flips its normal with it
// and the two still agree. The signed-volume bar exists because of that, and
// without the ledger nobody would have known it was needed.
//
// A verification pass on 2026-09-06 ran its own sabotages against this file and
// found three more holes: a family budget could be widened to admit anything
// (only the tree row was pinned), two biomes could collapse onto one silhouette
// (the only claim made was that eight kinds are family 'tree'), and foliage
// could become a standing sheet. All three are now bars, and all three are in
// the ledger. The one hole left open is named at the foot of the card test,
// with the measurement that says why it cannot be closed cheaply.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const file=path.resolve(__dirname,'../../spheres-web/ui/scatter-mesh.js');
const kit=require(file);
const source=fs.readFileSync(file,'utf8');

const CREASE=0.35;
const digest=m=>crypto.createHash('sha256').update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.normals.buffer)).update(Buffer.from(m.colors.buffer)).digest('hex');
const kinds=kit.kinds();

// Everything the renderer assumes about a buffer before it draws it, plus the
// contract the roadmap put on top. Written once because thirty kinds, nine
// biomes and two levels all have to pass it, and a check only the headline
// asset passes is not a check.
function validate(mod,mesh,label){
  assert.ok(mesh,`${label}: built`);
  // Cross-realm: a module loaded into a vm context builds its buffers with THAT
  // realm's Float32Array, so `instanceof` is false there even when the buffer is
  // perfectly good. The sabotage ledger below runs everything through this same
  // validator, so the type test has to be one that survives the boundary.
  const typed=(buf,name)=>assert.ok(ArrayBuffer.isView(buf)&&buf.constructor.name==='Float32Array',`${label}: ${name} buffer`);
  typed(mesh.positions,'positions');typed(mesh.normals,'normals');typed(mesh.colors,'colors');
  assert.equal(mesh.positions.length,mesh.normals.length,`${label}: normal alignment`);
  assert.equal(mesh.positions.length,mesh.colors.length,`${label}: colour alignment`);
  assert.equal(mesh.positions.length,mesh.triangleCount*9,`${label}: complete triangles`);
  assert.ok(mesh.triangleCount>0,`${label}: non-empty`);
  for(let i=0;i<mesh.positions.length;i+=3){
    for(let k=0;k<3;k+=1){
      assert.ok(Number.isFinite(mesh.positions[i+k]),`${label}: finite position at ${i+k}`);
      assert.ok(Number.isFinite(mesh.normals[i+k]),`${label}: finite normal at ${i+k}`);
      assert.ok(mesh.colors[i+k]>=0&&mesh.colors[i+k]<=1,`${label}: colour in 0..1 at ${i+k}`);
      // Vertices inside the bounding box is not asserted here: the extents
      // block below asserts the stronger thing, that the box IS the buffer's
      // own extents to a micron on all six faces.
    }
    assert.ok(Math.abs(Math.hypot(mesh.normals[i],mesh.normals[i+1],mesh.normals[i+2])-1)<1e-5,`${label}: unit normal at ${i}`);
  }
  // A degenerate triangle is invisible, shades black and exports as a hole. The
  // generator drops them at emit — a blade tapering to a point produces two per
  // blade by construction — so any survivor here is a real defect.
  //
  // The facing bar is CREASE and not 0.999, and that is the whole point of the
  // smoothing pass: a smoothed corner deliberately does not point along its own
  // face. What it may never do is point BEHIND it, and the crease limit proves
  // it cannot — every face folded into a corner is within CREASE of that
  // corner's own face, so the normalised sum keeps dot >= CREASE with it. That
  // is the property that catches an inverted or mirrored normal, and it is the
  // strongest statement that is true of both shading paths.
  for(let i=0;i<mesh.positions.length;i+=9){
    const a=mesh.positions.subarray(i,i+3),b=mesh.positions.subarray(i+3,i+6),c=mesh.positions.subarray(i+6,i+9);
    const u=[b[0]-a[0],b[1]-a[1],b[2]-a[2]],v=[c[0]-a[0],c[1]-a[1],c[2]-a[2]];
    const n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]];
    const area=Math.hypot(n[0],n[1],n[2]);
    assert.ok(area>1e-9,`${label}: non-degenerate triangle ${i/9}`);
    for(let k=0;k<3;k+=1){
      const o=i+k*3;
      const facing=(n[0]*mesh.normals[o]+n[1]*mesh.normals[o+1]+n[2]*mesh.normals[o+2])/area;
      assert.ok(facing>=CREASE-1e-6,`${label}: normal follows winding at ${i/9} (${facing.toFixed(3)})`);
    }
  }
  // Ground contact, measured off the BUFFER and not off the reported bounds.
  // Reading the bounds alone would have been satisfied by a mesh that floated
  // while its bounding box claimed otherwise, which is precisely what the
  // sabotage ledger's first entry produces — and a bounding box that disagrees
  // with its own geometry also breaks frustum culling, so both halves are
  // asserted: the extents ARE the buffer's extents, and the floor is zero.
  const lo=[Infinity,Infinity,Infinity],hi=[-Infinity,-Infinity,-Infinity];
  for(let i=0;i<mesh.positions.length;i+=3)
    for(let k=0;k<3;k+=1){
      if(mesh.positions[i+k]<lo[k])lo[k]=mesh.positions[i+k];
      if(mesh.positions[i+k]>hi[k])hi[k]=mesh.positions[i+k];
    }
  assert.equal(lo[1],0,`${label}: ground contact at Y=0`);
  for(let k=0;k<3;k+=1){
    assert.ok(Math.abs(lo[k]-mesh.bounds.min[k])<1e-6,`${label}: bounds.min[${k}] is the real minimum`);
    assert.ok(Math.abs(hi[k]-mesh.bounds.max[k])<1e-6,`${label}: bounds.max[${k}] is the real maximum`);
  }
  assert.equal(mesh.bounds.min[1],0,`${label}: ground contact at Y=0 in the reported bounds`);
  assert.ok(mesh.bounds.max[1]>0.05,`${label}: has height`);
  assert.ok(mesh.size.every(v=>v>0),`${label}: visible volume`);
  // Budgets, both ends. The floor matters as much as the ceiling — a piece that
  // quietly fell to four triangles has stopped being the thing it claims to be.
  const budget=mesh.budget;
  assert.ok(Array.isArray(budget),`${label}: carries its budget`);
  assert.ok(mesh.triangleCount>=budget[0]&&mesh.triangleCount<=budget[1],
    `${label}: ${mesh.triangleCount} triangles outside budget ${budget[0]}..${budget[1]}`);
  // Semantic ranges must tile the buffer exactly once and in order, or picking
  // a tree selects the rock beside it.
  let cursor=0;const names=new Set();
  for(const part of mesh.parts){
    assert.ok(!names.has(part.name),`${label}: unique part ${part.name}`);names.add(part.name);
    assert.equal(part.first,cursor,`${label}: part ${part.name} starts where the last ended`);
    assert.ok(part.count>0&&part.count%3===0,`${label}: part ${part.name} is whole triangles`);
    assert.ok(part.group&&part.label,`${label}: part semantics on ${part.name}`);
    cursor=part.first+part.count;
  }
  assert.equal(cursor,mesh.positions.length/3,`${label}: parts cover every vertex`);
  assert.ok(mesh.parts.length>=1,`${label}: named parts`);
  // Truthfulness, roadmap section 3. Representative scenery has to say so, in
  // the string a caller would actually print.
  assert.ok(typeof mesh.description==='string'&&mesh.description.length>60,`${label}: described`);
  assert.ok(/[Rr]epresentative/.test(mesh.description),`${label}: description says representative`);
  assert.ok(/not a survey/.test(mesh.description),`${label}: description refuses to be a survey`);
  assert.ok(/grants no/.test(mesh.description),`${label}: description grants nothing`);
}

/// The sweep both the live module and every sabotaged copy is put through. Kept
/// to one seed and one biome per kind so a twelve-entry sabotage ledger stays
/// affordable; the wider sweeps below run against the live module only.
function sweep(mod,tag){
  for(const kind of mod.kinds()){
    for(const lod of [0,1]){
      validate(mod,mod.piece(kind,{lod,seed:5,index:3}),`${tag}${kind}/lod${lod}`);
    }
  }
}

test('the kit covers every biome and family roadmap section G names',()=>{
  const wanted=['temperate_broadleaf','temperate_conifer','boreal','mediterranean','arid','tropical','alpine','tundra'];
  for(const biome of wanted)assert.ok(kit.biomeInfo(biome),`${biome}: covered`);
  assert.equal(kit.biomeInfo('not_a_biome'),null);
  // Every biome needs a silhouette of its own to be read by. Tundra is the one
  // that legitimately has no tree — that is what tundra means — so it answers
  // with a shrub, and the check says so rather than pretending otherwise.
  for(const biome of kit.biomes()){
    const info=kit.biomeInfo(biome);
    assert.ok(info.label&&info.note,`${biome}: described`);
    assert.ok(info.tree||info.shrub,`${biome}: has a woody silhouette`);
    assert.ok(info.grass&&info.rock,`${biome}: has ground cover and stone`);
    assert.ok(info.mix.length>=4,`${biome}: mix has something to choose from`);
    for(const [kind,weight] of info.mix){
      assert.ok(kit.kindInfo(kind),`${biome}: mix names a real kind (${kind})`);
      assert.ok(weight>0,`${biome}: ${kind} carries weight`);
      assert.notEqual(kit.kindInfo(kind).family,'dressing',`${biome}: ${kind} is scatterable`);
    }
  }
  assert.equal(kit.biomeInfo('tundra').tree,null,'tundra has no tree, and says so');
  // Eight distinguishable woody silhouettes. A palm, a pine, an oak and a
  // saguaro have to be four shapes, which starts with four builders' worth of
  // parts and four different heights.
  for(const kind of ['oak','pine','spruce','holm_oak','saguaro','palm','krummholz','acacia'])
    assert.equal(kit.kindInfo(kind).family,'tree',`${kind}: is a tree`);
  for(const kind of ['cliff_face','snow_edge','shore_sand','shore_shingle','shore_shelf','river_bank','field_patch','hedgerow'])
    assert.equal(kit.kindInfo(kind).family,'dressing',`${kind}: is terrain dressing`);
  assert.equal(kit.kindInfo('not_a_kind'),null);
});

// An orthographic silhouette of one piece, rasterised into a bitmap. This is
// what a player sees against the sky with the shading taken away, which is the
// point: the claim "you can tell the biomes apart" must not be rescued by
// colour, and only the outline is allowed to argue for it.
const SIL_RES=96,SIL_YAWS=6;
function silhouette(mesh,yaw,window){
  const R=SIL_RES,buf=new Uint8Array(R*R),c=Math.cos(yaw),s=Math.sin(yaw);
  const p=mesh.positions,n=p.length/9,k=R/window;
  for(let t=0;t<n;t+=1){
    const X=[],Y=[];
    for(let j=0;j<3;j+=1){
      const i=(t*3+j)*3;
      X.push(R/2+(p[i]*c-p[i+2]*s)*k);Y.push(R-1-p[i+1]*k);   // ground on the bottom row
    }
    const x0=Math.max(0,Math.floor(Math.min(...X))),x1=Math.min(R-1,Math.ceil(Math.max(...X)));
    const y0=Math.max(0,Math.floor(Math.min(...Y))),y1=Math.min(R-1,Math.ceil(Math.max(...Y)));
    const d=(X[1]-X[0])*(Y[2]-Y[0])-(X[2]-X[0])*(Y[1]-Y[0]);
    if(Math.abs(d)<1e-12)continue;
    for(let py=y0;py<=y1;py+=1)for(let px=x0;px<=x1;px+=1){
      const fx=px+0.5,fy=py+0.5;
      const w0=((X[1]-fx)*(Y[2]-fy)-(X[2]-fx)*(Y[1]-fy))/d;
      const w1=((X[2]-fx)*(Y[0]-fy)-(X[0]-fx)*(Y[2]-fy))/d;
      if(w0>=-1e-9&&w1>=-1e-9&&1-w0-w1>=-1e-9)buf[py*R+px]=1;
    }
  }
  return buf;
}
const overlap=(a,b)=>{let i=0,u=0;for(let k=0;k<a.length;k+=1){if(a[k]|b[k])u+=1;if(a[k]&b[k])i+=1;}return u?i/u:1;};
/// The most confusable orientation pairing of two biomes' signature species,
/// at TRUE world scale — every yaw of one against every yaw of the other, worst
/// case kept. Worst case is the honest reading: a player is not promised a
/// flattering angle.
function worstConfusion(mod,lod){
  const wanted=['temperate_broadleaf','temperate_conifer','boreal','mediterranean','arid','tropical','alpine','tundra'];
  const meshes={};let tallest=0;
  for(const biome of wanted){
    const info=mod.biomeInfo(biome);
    meshes[biome]=mod.piece(info.tree||info.shrub,{biome,lod,seed:4});
    if(meshes[biome].bounds.max[1]>tallest)tallest=meshes[biome].bounds.max[1];
  }
  const views={};
  for(const biome of wanted){
    views[biome]=[];
    for(let y=0;y<SIL_YAWS;y+=1)views[biome].push(silhouette(meshes[biome],y*Math.PI*2/SIL_YAWS,tallest*1.05));
  }
  let worst=0,pair='';
  for(let i=0;i<wanted.length;i+=1)for(let j=i+1;j<wanted.length;j+=1){
    let best=0;
    for(const a of views[wanted[i]])for(const b of views[wanted[j]])best=Math.max(best,overlap(a,b));
    if(best>worst){worst=best;pair=`${wanted[i]} vs ${wanted[j]}`;}
  }
  return {worst,pair};
}

test('the eight biomes are told apart by outline alone, not by colour',()=>{
  // Roadmap section 3 asks for regional variety read off the map, and section G
  // puts biome trees at the top of terrain detailing. That claim is only worth
  // making if it is measured, so it is: rasterise each biome's signature woody
  // species — its tree, or its shrub where the biome legitimately has no tree —
  // and take the intersection over union of the two most confusable views.
  //
  // MEASURED 2026-09-06 at 96 px over six yaws, worst pair of the 28:
  //   near LOD 0.293, far LOD 0.558, both temperate_conifer vs boreal.
  // A collapsed pair reads 1.000 — verified by cloning the oak's spec onto the
  // holm oak, which until this bar existed left the whole suite green. The bar
  // is 0.75: clear of the measured 0.558 by a wide margin, and it catches a
  // pair sharing three quarters of its outline, which is confusable on a map.
  //
  // TWO HONEST CAVEATS, both measured rather than argued.
  // 1. Pine and spruce are the close pair, at 0.558 far. They are two conifers
  //    and they are meant to be similar; what separates them is the pine's bare
  //    lower trunk (clear 0.36 of height against the spruce's 0.20) and the
  //    spruce's narrower taper. If that gap ever closes, this bar moves first.
  // 2. This bar reads TRUE world scale, which is how a map draws them. Asked
  //    with height normalised away, oak and holm oak read 0.70 near and 0.73
  //    far: they are the same rounded broadleaf dome at two sizes, separated on
  //    the map by height (19.0 m against 11.9 m) and by canopy colour, and NOT
  //    by shape. A caller that normalises tree height for a map icon would lose
  //    that pair, and this bar would not see it, because this bar deliberately
  //    reads what the map actually draws.
  for(const lod of [0,1]){
    const {worst,pair}=worstConfusion(kit,lod);
    assert.ok(worst<0.75,`lod${lod}: ${pair} share ${(100*worst).toFixed(1)}% of their silhouette`);
  }
});

test('every kind builds sound geometry at both levels, in every biome',()=>{
  sweep(kit,'');
  // The palette is a biome's, not a kind's: a boulder in the tundra is not the
  // boulder in the desert, and the geometry must survive being asked for either.
  for(const kind of kinds){
    for(const biome of kit.biomes()){
      const near=kit.piece(kind,{biome,seed:11}),far=kit.piece(kind,{biome,seed:11,lod:'map'});
      assert.equal(near.biome,biome);assert.equal(far.lod,1);
      assert.equal(near.bounds.min[1],0,`${kind}/${biome}: seated`);
      assert.ok(near.triangleCount>=near.budget[0]&&near.triangleCount<=near.budget[1],`${kind}/${biome}: near budget`);
      assert.ok(far.triangleCount>=far.budget[0]&&far.triangleCount<=far.budget[1],`${kind}/${biome}: far budget`);
      assert.ok(far.triangleCount<near.triangleCount,`${kind}/${biome}: the map mesh is actually cheaper`);
    }
  }
});

test('the tree budget is roadmap section 4, and a thousand trees are priced from the meshes',()=>{
  const budgets=kit.budgets();
  assert.deepEqual(budgets.tree.near,[100,800],'section 4: tree near 100..800');
  assert.deepEqual(budgets.tree.far,[20,100],'section 4: tree far 20..100');
  for(const family of ['shrub','grass','rock']){
    assert.ok(budgets[family].near[1]<budgets.tree.near[1],`${family}: cheaper than a tree near`);
    assert.ok(budgets[family].far[1]<=budgets.tree.far[1]+20,`${family}: cheaper than a tree far`);
  }
  const cost=kit.treeCost(1000);
  assert.equal(cost.instances,1000);
  assert.equal(cost.kinds,8);
  assert.equal(cost.nearTotal,cost.nearMedian*1000);
  assert.equal(cost.farTotal,cost.farMedian*1000);
  assert.ok(cost.nearMedian>=100&&cost.nearMedian<=800,'median tree inside the near budget');
  assert.ok(cost.farMedian>=20&&cost.farMedian<=100,'median tree inside the far budget');
  // The far LOD is the one that decides whether a map can afford a forest, so
  // the saving has to be a real order of magnitude and not a rounding.
  assert.ok(cost.nearMedian/cost.farMedian>4,'the map tree is at least four times cheaper');
  assert.ok(/not a frame-time promise/.test(cost.note),'the price refuses to be a performance claim');
  // EVERY family band pinned to a literal, not just the tree. The tree row is
  // roadmap section 4 verbatim; the other four are this kit's own. They are
  // pinned so that widening one is a visible edit to a test rather than a
  // silent drift in a table nobody rereads — iron rule 5, whose whole point is
  // that the repair for a piece that busts its budget is a cheaper piece and
  // never a wider budget.
  //
  // Added 2026-09-06 after a verification pass measured the hole: widening the
  // DRESSING band to [1, 900000] at both levels left the entire suite green,
  // because the loop below only ever asked the other families to be cheaper
  // than a tree, and never asked dressing anything at all.
  assert.deepEqual(budgets.shrub,{near:[30,420],far:[6,100]},'shrub band pinned');
  assert.deepEqual(budgets.grass,{near:[16,240],far:[4,60]},'grass band pinned');
  assert.deepEqual(budgets.rock,{near:[30,520],far:[6,120]},'rock band pinned');
  assert.deepEqual(budgets.dressing,{near:[80,1800],far:[16,360]},'dressing band pinned');
  assert.deepEqual(Object.keys(budgets).sort(),['dressing','grass','rock','shrub','tree'],
    'five families, and no sixth slipped in unbudgeted');
});

test('a scattered kind costs a fixed number of triangles, so a thousand of them is a number',()=>{
  // Only sizes, angles and colours move with the seed; the structure does not.
  // That is a deliberate constraint and it is what turns a budget into
  // arithmetic. Dressing tiles are exempt and say so: a field patch's crop
  // changes how many plough ridges it has.
  for(const kind of kit.scatterable){
    assert.equal(kit.kindInfo(kind).fixedCost,true,`${kind}: declares a fixed cost`);
    for(const lod of [0,1]){
      const want=kit.cost(kind,lod);
      for(const seed of [0,1,7,'abc',99991]){
        for(const biome of ['tundra','tropical']){
          assert.equal(kit.piece(kind,{seed,biome,lod}).triangleCount,want,
            `${kind}/lod${lod}/${seed}/${biome}: count moved`);
        }
      }
    }
  }
  for(const kind of kit.tileable.concat(['field_patch'])){
    assert.equal(kit.kindInfo(kind).fixedCost,false,`${kind}: does not claim a fixed cost`);
    for(const lod of [0,1]){
      for(const seed of [0,3,'x']){
        for(const index of [0,1,2,9]){
          const mesh=kit.piece(kind,{seed,index,lod});
          assert.ok(mesh.triangleCount>=mesh.budget[0]&&mesh.triangleCount<=mesh.budget[1],
            `${kind}/lod${lod}/${seed}/${index}: ${mesh.triangleCount} outside ${mesh.budget.join('..')}`);
        }
      }
    }
  }
});

test('geometry is deterministic in-process and across a fresh load',()=>{
  delete require.cache[require.resolve(file)];const fresh=require(file);
  assert.deepEqual(fresh.kinds(),kinds);
  for(const kind of kinds){
    for(const lod of [0,1]){
      const options={seed:'lough neagh',biome:'boreal',lod,index:6},snapshot=JSON.stringify(options);
      const a=kit.piece(kind,options),b=fresh.piece(kind,options),c=kit.piece(kind,{...options});
      assert.equal(JSON.stringify(options),snapshot,`${kind}: build does not mutate its options`);
      for(const field of ['positions','normals','colors'])
        assert.deepEqual(a[field],b[field],`${kind}/lod${lod}: deterministic ${field}`);
      assert.equal(digest(a),digest(c),`${kind}/lod${lod}: repeat build is identical`);
      assert.deepEqual(a.bounds,b.bounds);assert.deepEqual(a.parts,b.parts);
    }
  }
  // And a different seed must actually be a different specimen, or the seed is
  // decoration. Two oaks from two seeds share a species and nothing else.
  for(const kind of kinds){
    assert.notEqual(digest(kit.piece(kind,{seed:1})),digest(kit.piece(kind,{seed:2})),`${kind}: the seed does something`);
  }
});

test('curved surfaces are smoothed, flat ones are not, and hard edges stay hard',()=>{
  // Smoothing costs zero triangles and is the largest gain available to a
  // renderer with no textures, so its absence is a regression worth catching.
  // The two arms are equally important: a trunk that stopped being smoothed
  // reads as a faceted prism, and a boulder that STARTED being smoothed reads
  // as a potato.
  const bent=(name,partName)=>{
    const mesh=kit.piece(name,{seed:9});
    const part=partName?mesh.parts.find(p=>p.name===partName):null;
    if(partName)assert.ok(part,`${name}: has a ${partName} part`);
    const from=part?part.first*3:0,to=part?(part.first+part.count)*3:mesh.positions.length;
    let smoothed=0,corners=0;
    for(let i=from;i<to;i+=9){
      const a=mesh.positions.subarray(i,i+3),b=mesh.positions.subarray(i+3,i+6),c=mesh.positions.subarray(i+6,i+9);
      const u=[b[0]-a[0],b[1]-a[1],b[2]-a[2]],v=[c[0]-a[0],c[1]-a[1],c[2]-a[2]];
      const n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]];
      const area=Math.hypot(n[0],n[1],n[2]);
      for(let k=0;k<3;k+=1){
        const o=i+k*3;corners+=1;
        if((n[0]*mesh.normals[o]+n[1]*mesh.normals[o+1]+n[2]*mesh.normals[o+2])/area<0.999)smoothed+=1;
      }
      }
    return {mesh,smoothed,corners};
  };
  // Asked PART BY PART, because a conifer is both: its trunk is a round stem
  // and its branch tiers are flat plates, and averaging the two into one figure
  // for the whole tree would let either arm rot without the number moving. This
  // is the bar that caught the spruce reading 11% smoothed and looking fine.
  for(const [kind,part] of [['oak','trunk'],['oak','canopy'],['spruce','trunk'],['pine','trunk'],
    ['palm','stem'],['palm','crown shaft'],['saguaro','column'],['saguaro','arms'],['holm_oak','canopy'],['maquis','foliage']]){
    const {smoothed,corners}=bent(kind,part);
    assert.ok(smoothed>corners*0.5,`${kind}/${part}: only ${smoothed} of ${corners} corners are smoothed`);
  }
  for(const kind of ['boulder','scree_patch','cliff_face','grass_tuft']){
    const {smoothed}=bent(kind);
    assert.equal(smoothed,0,`${kind}: is faceted on purpose and must stay flat shaded`);
  }
  // A palm frond is a smoothed rib carrying flat leaflets, so the part as a
  // whole is mostly flat and correctly so — a leaflet is a folded blade and a
  // fold is an edge. The rib is covered by the stem and crown-shaft bars above.
  const leaf=bent('palm','fronds');
  assert.ok(leaf.smoothed>0,'palm: the frond ribs are still round');
  assert.ok(leaf.smoothed<leaf.corners*0.5,'palm: the leaflets are still folded blades, not tubes');
  // A branch plate is flat on purpose too, and a tier that started smoothing
  // would weld nine branch tips into one skirt.
  for(const kind of ['spruce','pine','krummholz']){
    const {smoothed}=bent(kind,'branch tiers');
    assert.equal(smoothed,0,`${kind}: branch plates stay flat`);
  }
});

test('closed shapes are wound outward, and ground faces up',()=>{
  // A boulder is a genuinely closed mesh, so its signed volume is the honest
  // test for an inverted winding — the defect that makes a rock render inside
  // out and disappear under backface culling. A canopy is closed too, part by
  // part, so it gets the same test.
  const volume=(mesh,first,count)=>{
    let v=0;const s=(first||0)*3,e=count==null?mesh.positions.length:(first+count)*3;
    for(let i=s;i<e;i+=9){
      const a=mesh.positions.subarray(i,i+3),b=mesh.positions.subarray(i+3,i+6),c=mesh.positions.subarray(i+6,i+9);
      v+=(a[0]*(b[1]*c[2]-b[2]*c[1])-a[1]*(b[0]*c[2]-b[2]*c[0])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6;
    }
    return v;
  };
  for(const lod of [0,1]){
    const rock=kit.piece('boulder',{seed:4,lod});
    assert.ok(volume(rock)>0.05,`boulder/lod${lod}: closed and wound outward (${volume(rock).toFixed(3)})`);
    for(const kind of ['oak','holm_oak','acacia']){
      const tree=kit.piece(kind,{seed:4,lod});
      const canopy=tree.parts.find(p=>p.name==='canopy');
      assert.ok(canopy,`${kind}: has a canopy part`);
      assert.ok(volume(tree,canopy.first,canopy.count)>1,`${kind}/lod${lod}: canopy wound outward`);
    }
  }
  const meanY=(mesh)=>{let y=0;for(let i=1;i<mesh.normals.length;i+=3)y+=mesh.normals[i];return y/(mesh.normals.length/3);};
  for(const kind of ['shore_sand','shore_shingle','shore_shelf','field_patch','snow_edge','river_bank'])
    assert.ok(meanY(kit.piece(kind,{seed:4}))>0.15,`${kind}: ground faces up`);
  // The cliff faces out of the cliff rather than into it, which is the same
  // defect on a vertical surface.
  const cliff=kit.piece('cliff_face',{seed:4});
  const face=cliff.parts.find(p=>p.name==='rock face');
  let z=0;for(let i=face.first*3+2;i<(face.first+face.count)*3;i+=3)z+=cliff.normals[i];
  assert.ok(z/face.count<-0.5,'cliff face points out of the cliff');
});

test('a tile run is seamless: tile n right edge is tile n+1 left edge, vertex for vertex',()=>{
  // The whole reason `strip` hashes a GLOBAL boundary index rather than a local
  // one. A run of cliff or shore that does not close at the seam shows a crack
  // straight through to the skybox, and it is invisible in a single-tile
  // preview — which is exactly why it belongs in a test and not in an eyeball.
  const width=kit.tileWidth;
  const edge=(mesh,x)=>{
    const out=[];
    for(let i=0;i<mesh.positions.length;i+=3)
      if(Math.abs(mesh.positions[i]-x)<1e-6)out.push(mesh.positions[i+1].toFixed(5)+','+mesh.positions[i+2].toFixed(5));
    return Array.from(new Set(out)).sort();
  };
  assert.ok(kit.tileable.length>=6,'six tiling kinds');
  for(const kind of kit.tileable){
    assert.equal(kit.kindInfo(kind).tile,width,`${kind}: declares the shared tile width`);
    for(const lod of [0,1]){
      for(const index of [0,3,17,4096]){
        const a=kit.piece(kind,{seed:'coast',index,lod}),b=kit.piece(kind,{seed:'coast',index:index+1,lod});
        const right=edge(a,width),left=edge(b,0);
        assert.ok(right.length>=2,`${kind}/lod${lod}: has a seam profile`);
        assert.deepEqual(right,left,`${kind}/lod${lod}/tile${index}: seam does not close`);
        // And two tiles in a run are not the same tile, or the run repeats.
        assert.notEqual(digest(a),digest(b),`${kind}/lod${lod}/tile${index}: the run varies`);
      }
      // A different seed is a different stretch of coast entirely.
      assert.notEqual(digest(kit.piece(kind,{seed:'a',index:3,lod})),digest(kit.piece(kind,{seed:'b',index:3,lod})),
        `${kind}/lod${lod}: the seed changes the run`);
    }
  }
});

test('scatter places deterministically, and two seeds are visibly different arrangements',()=>{
  const area={width:90,depth:90};
  const a=kit.scatter(1,'temperate_broadleaf',120,area);
  const again=kit.scatter(1,'temperate_broadleaf',120,area);
  const b=kit.scatter(2,'temperate_broadleaf',120,area);
  assert.equal(JSON.stringify(a.instances),JSON.stringify(again.instances),'same seed, same arrangement');
  assert.equal(a.placed,120);assert.ok(a.instances.every(i=>Number.isFinite(i.x)&&Number.isFinite(i.z)));
  // Everything lands inside the area it was asked for.
  for(const i of a.instances){
    assert.ok(i.x>=0&&i.x<=area.width,`instance ${i.cell} inside width`);
    assert.ok(i.z>=0&&i.z<=area.depth,`instance ${i.cell} inside depth`);
    assert.ok(kit.kindInfo(i.kind),`instance ${i.cell} names a real kind`);
    assert.ok(i.scale>0.5&&i.scale<1.5&&i.yaw>=0&&i.yaw<Math.PI*2,`instance ${i.cell} plausibly posed`);
  }
  // DIFFERENT, and measured rather than asserted by eye. Bars set from the
  // measurement on this arrangement: 97.5% of cells move more than half a metre
  // and 70.8% draw a different kind, so 80% and 40% are comfortably clear of
  // the truth while still catching a seed that has stopped reaching placement
  // at all (which reads as 0% on both).
  const byCell=new Map(b.instances.map(i=>[i.cell,i]));
  let moved=0,differentKind=0,compared=0;
  for(const i of a.instances){
    const j=byCell.get(i.cell);if(!j)continue;compared+=1;
    if(Math.hypot(i.x-j.x,i.z-j.z)>0.5)moved+=1;
    if(i.kind!==j.kind)differentKind+=1;
  }
  assert.ok(compared>80,'enough cells to compare');
  assert.ok(moved/compared>0.8,`only ${(100*moved/compared).toFixed(1)}% of instances moved between seeds`);
  assert.ok(differentKind/compared>0.4,`only ${(100*differentKind/compared).toFixed(1)}% of instances changed kind`);
  assert.notDeepEqual(a.histogram,b.histogram,'the two seeds do not even draw the same mix');
  // Every biome scatters, and the plan prices itself from the measured meshes.
  for(const biome of kit.biomes()){
    const plan=kit.scatter(9,biome,200,{width:120,depth:120});
    assert.ok(plan.placed>150,`${biome}: ${plan.placed} of 200 placed`);
    assert.equal(plan.biome,biome);
    let near=0,far=0;
    for(const kind of Object.keys(plan.histogram)){
      near+=kit.cost(kind,0)*plan.histogram[kind];far+=kit.cost(kind,1)*plan.histogram[kind];
    }
    assert.equal(plan.triangles.near,near,`${biome}: near price is the sum of its pieces`);
    assert.equal(plan.triangles.far,far,`${biome}: far price is the sum of its pieces`);
    assert.ok(plan.triangles.far*3<plan.triangles.near,`${biome}: the map field is much cheaper`);
    assert.ok(/not a survey/.test(plan.description),`${biome}: the plan refuses to be a survey`);
  }
});

test('scatter refuses the ground the caller told it to refuse, and says what it refused',()=>{
  // Roadmap section G: placement excludes water, excessive slope, labels and
  // transport corridors. All four live outside this module, so the module takes
  // them as data and REPORTS what it dropped rather than silently thinning.
  const area={width:100,depth:100};
  const clear=kit.scatter(4,'boreal',200,area);
  const cut=kit.scatter(4,'boreal',200,area,{exclude:[{x:50,z:50,r:30}]});
  assert.ok(cut.refused.excluded>40,`${cut.refused.excluded} instances refused by the exclusion`);
  assert.equal(cut.placed+cut.refused.excluded+cut.refused.slope+cut.refused.spacing,200,'every instance is accounted for');
  for(const i of cut.instances)assert.ok(Math.hypot(i.x-50,i.z-50)>30,'nothing inside the exclusion circle');
  const boxed=kit.scatter(4,'boreal',200,area,{exclude:[{x0:0,x1:100,z0:40,z1:60}]});
  for(const i of boxed.instances)assert.ok(i.z<40||i.z>60,'nothing inside the exclusion rectangle');
  // Slope. A 0.6 gradient against a 0.4 limit refuses everything, and refuses it
  // as slope rather than quietly as an exclusion.
  const steep=kit.scatter(4,'boreal',200,area,{ground:()=>0,slopeLimit:0.4});
  assert.equal(steep.placed+steep.refused.spacing,200,'flat ground places');
  const cliff=kit.scatter(4,'boreal',200,area,{ground:(x)=>x*0.6,slopeLimit:0.4});
  assert.equal(cliff.placed,0,'nothing stands on a 0.6 gradient under a 0.4 limit');
  assert.equal(cliff.refused.slope,200,'and it is refused as slope');
  // Terrain height is the caller's too: an instance sits where the sampler says.
  const rolling=kit.scatter(4,'boreal',60,area,{ground:(x,z)=>Math.sin(x/20)*3});
  for(const i of rolling.instances)assert.ok(Math.abs(i.y-Math.sin(i.x/20)*3)<1e-9,'instance sits on the sampled ground');
  assert.ok(clear.placed>=cut.placed,'an exclusion never adds instances');
  assert.equal(kit.scatter(4,'not_a_biome',10,area).biome,'temperate_broadleaf','an unknown biome falls back');
  assert.equal(kit.scatter(4,'boreal',0,area).placed,0);
});

test('a scatter field builds as one mesh, deterministically, from a small variant cache',()=>{
  const field=kit.scatterMesh(4,'tropical',60,{width:80,depth:80},{lod:1});
  const twin=kit.scatterMesh(4,'tropical',60,{width:80,depth:80},{lod:1});
  assert.equal(digest(field),digest(twin),'the same field twice');
  assert.equal(field.bounds.min[1],0,'the field is seated');
  assert.ok(field.instances>40,'the field holds its instances');
  let cursor=0;
  for(const part of field.parts){
    assert.equal(part.first,cursor,`field part ${part.name} is contiguous`);
    assert.ok(/\d+ instances/.test(part.name),'a field part counts its instances');
    cursor=part.first+part.count;
  }
  assert.equal(cursor,field.positions.length/3,'field parts cover every vertex');
  for(let i=0;i<field.colors.length;i+=1)assert.ok(field.colors[i]>=0&&field.colors[i]<=1,'field colour in range');
  for(let i=0;i<field.normals.length;i+=3)
    assert.ok(Math.abs(Math.hypot(field.normals[i],field.normals[i+1],field.normals[i+2])-1)<1e-5,'field unit normal');
  assert.notEqual(digest(field),digest(kit.scatterMesh(5,'tropical',60,{width:80,depth:80},{lod:1})),'a different seed is a different field');
});

test('unknown and malformed input resolves to a stable, honest default',()=>{
  const fallback=kit.piece('boulder',{seed:0});
  for(const input of ['not_a_kind','__proto__','',null,undefined,{},0]){
    const mesh=kit.piece(input,{seed:0});
    assert.equal(mesh.kind,'boulder',`${String(input)}: falls back`);
    assert.equal(digest(mesh),digest(fallback));
  }
  for(const options of [undefined,null,'nonsense',{lod:'far'},{seed:Number.NaN},{index:Number.NaN},{biome:'__proto__'}])
    validate(kit,kit.piece('oak',options),`options ${JSON.stringify(options)}`);
  assert.equal(kit.piece('oak',{lod:'map'}).lod,1);
  assert.equal(kit.piece('oak',{lod:'far'}).lod,1);
  assert.equal(kit.piece('oak',{lod:'near'}).lod,0);
  assert.equal(kit.cost('not_a_kind',0),kit.cost('boulder',0));
  assert.equal(kit.scatter(1,'boreal',-5,{width:10,depth:10}).placed,0);
  assert.equal(kit.scatter(1,'boreal',5,null).area.width,100,'a missing area gets a default');
});

/// PCA slab thickness of a vertex range, in metres: the standard deviation
/// along the range's thinnest principal axis. A sheet reads ~0; anything with
/// real body does not. Closed-form symmetric 3x3 eigenvalues, so it costs
/// nothing to run over every part of every kind.
function slabThickness(pos,first,count){
  let cx=0,cy=0,cz=0;
  for(let i=first;i<first+count;i+=1){cx+=pos[i*3];cy+=pos[i*3+1];cz+=pos[i*3+2];}
  cx/=count;cy/=count;cz/=count;
  const C=[[0,0,0],[0,0,0],[0,0,0]];
  for(let i=first;i<first+count;i+=1){
    const d=[pos[i*3]-cx,pos[i*3+1]-cy,pos[i*3+2]-cz];
    for(let a=0;a<3;a+=1)for(let b=0;b<3;b+=1)C[a][b]+=d[a]*d[b];
  }
  for(let a=0;a<3;a+=1)for(let b=0;b<3;b+=1)C[a][b]/=count;
  const p1=C[0][1]**2+C[0][2]**2+C[1][2]**2;
  if(p1===0)return Math.sqrt(Math.max(0,Math.min(C[0][0],C[1][1],C[2][2])));
  const q=(C[0][0]+C[1][1]+C[2][2])/3;
  const p=Math.sqrt(((C[0][0]-q)**2+(C[1][1]-q)**2+(C[2][2]-q)**2+2*p1)/6);
  const B=[[0,0,0],[0,0,0],[0,0,0]];
  for(let a=0;a<3;a+=1)for(let b=0;b<3;b+=1)B[a][b]=(C[a][b]-(a===b?q:0))/p;
  const det=B[0][0]*(B[1][1]*B[2][2]-B[1][2]*B[2][1])-B[0][1]*(B[1][0]*B[2][2]-B[1][2]*B[2][0])+B[0][2]*(B[1][0]*B[2][1]-B[1][1]*B[2][0]);
  const phi=Math.acos(Math.max(-1,Math.min(1,det/2)))/3;
  const e1=q+2*p*Math.cos(phi),e3=q+2*p*Math.cos(phi+2*Math.PI/3);
  return Math.sqrt(Math.max(0,Math.min(e1,e3,3*q-e1-e3)));
}

test('no alpha and no billboard cards: every leaf mass is geometry',()=>{
  // Roadmap section 4 says avoid alpha overdraw, and this renderer has no
  // textures and no UVs to hang a cut-out on in the first place. Three ways in,
  // three bars.
  //
  // 1. NO ALPHA CHANNEL. Colour is exactly three floats a vertex — a fourth
  //    would be the first step toward a cut-out sheet — and the mesh carries no
  //    UV or texture field for one to arrive on later.
  for(const kind of kinds){
    for(const lod of [0,1]){
      const mesh=kit.piece(kind,{lod,seed:11});
      assert.equal(mesh.colors.length,mesh.positions.length,`${kind}/lod${lod}: RGB only, no alpha`);
      for(const field of ['uv','uvs','alpha','texture','textures','opacity'])
        assert.ok(!(field in mesh),`${kind}/lod${lod}: carries no ${field}`);
    }
  }
  //    The source bar here is deliberately narrower than the wall-clock one
  //    below. Banning the bare words would ban the module's own header, which
  //    says in plain English that there are no UVs, no alpha and no cut-out
  //    sheets in this renderer — and a bar that punishes a file for documenting
  //    its own limits teaches the next session to delete the documentation. So
  //    this bans the IMPLEMENTATION shapes: the identifier, never the noun.
  for(const pattern of [/\buvs?\s*[:=]/i,/\balpha\s*[:=]/i,/\bopacity\s*[:=]/i,
    /\bbillboard\s*[:=(]/i,/\bimpostor\s*[:=(]/i,/\bsprite\s*[:=(]/i])
    assert.ok(!pattern.test(source),`scatter-mesh.js must not contain ${pattern}`);
  // 2. NOTHING DOUBLE SIDED. A card has to be visible from behind, which in a
  //    renderer with backface culling and no two-sided material means emitting
  //    the same triangle twice, wound both ways. Measured on the live kit:
  //    zero duplicate vertex-triples across all thirty kinds at both levels.
  for(const kind of kinds){
    for(const lod of [0,1]){
      const mesh=kit.piece(kind,{lod,seed:11}),p=mesh.positions,seen=new Set();
      for(let t=0;t<p.length/9;t+=1){
        const key=[0,1,2].map((j)=>{
          const i=(t*3+j)*3;
          return `${p[i].toFixed(4)},${p[i+1].toFixed(4)},${p[i+2].toFixed(4)}`;
        }).sort().join(';');
        assert.ok(!seen.has(key),`${kind}/lod${lod}: triangle ${t} is a duplicate face`);
        seen.add(key);
      }
    }
  }
  // 3. NO STANDING SHEET. A part with no thickness at all is a sheet, and a
  //    sheet is only legitimate here when it is the ground: a field surface, a
  //    headland track, a scree apron. Measured on the live kit, the only parts
  //    under a centimetre of slab thickness are exactly those three, and all of
  //    them face straight up (mean normal.y = 1.000). A standing sheet is a
  //    billboard, and there are none.
  for(const kind of kinds){
    for(const lod of [0,1]){
      const mesh=kit.piece(kind,{lod,seed:11});
      for(const part of mesh.parts){
        if(slabThickness(mesh.positions,part.first,part.count)>=0.01)continue;
        let y=0;
        for(let i=part.first;i<part.first+part.count;i+=1)y+=mesh.normals[i*3+1];
        assert.ok(y/part.count>0.5,
          `${kind}/lod${lod}: part "${part.name}" is a sheet standing on edge (mean normal.y ${(y/part.count).toFixed(3)}) — that is a billboard`);
      }
    }
  }
  // WHAT THIS DOES NOT CATCH, measured rather than assumed: cards buried inside
  // a part that also holds solid geometry. The obvious tighter bar — the share
  // of a mesh's area sitting in near-vertical exactly-coplanar groups — was
  // tried and rejected, because a saguaro's fluted column reads 78% on it while
  // a fully carded oak reads 94%. The bands overlap, so that bar would red on
  // honest geometry, and a bar that reds on honest geometry gets widened until
  // it means nothing. These three are the ones that separate cleanly.
});

test('no wall clock, no frame counter and no second entropy source can reach this geometry',()=>{
  // Iron rule 1. A scatter field is saved as a seed, so a clock or an entropy
  // source in this file would make a saved landscape unreproducible. The
  // cheapest way to keep that true is for the vocabulary not to exist at all.
  for(const pattern of [/\bDate\b/,/\bnow\s*\(/,/performance/i,/hrtime/,/requestAnimationFrame/i,
    /\bsetTimeout\b/,/\bsetInterval\b/,/Math\s*\.\s*random/,/\brandom\b/i,/getTime/,/\bcrypto\b/i])
    assert.ok(!pattern.test(source),`scatter-mesh.js must not contain ${pattern}`);
  // And no DOM, so the geometry stays checkable outside a browser.
  for(const pattern of [/\bdocument\b/,/\bwindow\b/,/canvas/i,/WebGL/i,/require\s*\(/,/\bimport\b/])
    assert.ok(!pattern.test(source),`scatter-mesh.js must not contain ${pattern}`);
});

test('the browser global exports the same contract with no dependencies',()=>{
  const context=vm.createContext({});
  vm.runInContext(source,context);
  for(const fn of ['piece','scatter','scatterMesh','kinds','kindInfo','biomes','biomeInfo','budgets','cost','treeCost'])
    assert.equal(typeof context.ScatterMesh[fn],'function',`ScatterMesh.${fn}`);
  assert.equal(context.ScatterMesh.kinds().join(','),kinds.join(','));
  assert.equal(context.ScatterMesh.piece('oak',{seed:3}).triangleCount,kit.piece('oak',{seed:3}).triangleCount);
  assert.equal(digest(context.ScatterMesh.piece('spruce',{seed:3})),digest(kit.piece('spruce',{seed:3})),
    'the browser path builds byte-identical geometry to the node path');
});

// ---------------------------------------------------------------------------
// Iron rule 5, made mechanical. Every bar above was watched going red against a
// deliberately broken generator, and this is the record of it: the defect, the
// edit that produces it, and the assertion that must catch it. If a future
// change makes one of these sabotages pass, the assertion it names has stopped
// working and the ledger says which one.
const SABOTAGE=[
  {defect:'the mesh stops being seated on the ground',expect:'ground contact at Y=0',
    edits:[['for (let i = 1; i < positions.length; i += 3) positions[i] -= ground;',
      'for (let i = 1; i < positions.length; i += 3) positions[i] -= ground - 0.05;']]},
  {defect:'colour escapes 0..1',expect:'colour in 0..1',
    edits:[['function clamp01(v) { return v < 0 ? 0 : v > 1 ? 1 : v; }','function clamp01(v) { return v * 1.6; }']]},
  {defect:'normals stop being unit length',expect:'unit normal',
    edits:[['normals[i + k * 3] = nx; normals[i + k * 3 + 1] = ny; normals[i + k * 3 + 2] = nz;',
      'normals[i + k * 3] = nx * 1.5; normals[i + k * 3 + 1] = ny * 1.5; normals[i + k * 3 + 2] = nz * 1.5;']]},
  {defect:'a position arrives as NaN',expect:'finite position',
    edits:[['    const ground = min[1];','    const ground = min[1] + Number.NaN;']]},
  {defect:'every triangle collapses and the mesh comes back empty',expect:'non-empty',
    edits:[['function askSpan(seed, salt, lo, hi) { return lo + (hi - lo) * askUnit(seed, salt); }',
      'function askSpan(seed, salt, lo, hi) { return lo + (hi - lo) * askUnit(seed, salt) * Math.sqrt(-1); }']]},
  {defect:'a part range no longer starts where the last one ended',expect:'starts where the last ended',
    edits:[['        name, first, count,','        name, first: first + 3, count,']]},
  {defect:'degenerate triangles are emitted instead of dropped',expect:'non-degenerate triangle',
    edits:[['    if (!(Math.hypot(nx, ny, nz) > 1e-9)) return this;','    if (false) return this;']]},
  {defect:'the far tree blows its triangle budget',expect:'outside budget',
    edits:[['    const forks = d ? spec.forks : 2;','    const forks = d ? spec.forks : 12;'],
      ['    const seg = d ? 8 : 4;','    const seg = d ? 8 : 20;']]},
];
// These three are checked against their own bars rather than the shared
// validator, because they break a property no per-vertex sweep can see.
const SABOTAGE_WIDE=[
  {defect:'geometry drifts between two builds in one process',
    check:(mod)=>assert.deepEqual(mod.piece('oak',{seed:3}).positions,mod.piece('oak',{seed:3}).positions),
    edits:[['  "use strict";','  "use strict";\n  let drift = 0;'],
      ['function askUnit(seed, salt) { return ask(seed, salt) / 4294967296; }',
        'function askUnit(seed, salt) { drift += 1; return (ask(seed, salt) / 4294967296 + drift * 1e-6) % 1; }']]},
  {defect:'a band is wound the wrong way round',
    // This one CANNOT be caught by the per-corner facing bar, and finding that
    // out is why the ledger exists. Normals here are derived from the geometry
    // in `finish` rather than authored, so flipping a quad flips its normal with
    // it and the two still agree — the mesh renders inside out and every
    // per-vertex test stays green. The signed volume is what sees it.
    check:(mod)=>{
      const mesh=mod.piece('boulder',{seed:4});
      let v=0;
      for(let i=0;i<mesh.positions.length;i+=9){
        const a=mesh.positions.subarray(i,i+3),b=mesh.positions.subarray(i+3,i+6),c=mesh.positions.subarray(i+6,i+9);
        v+=(a[0]*(b[1]*c[2]-b[2]*c[1])-a[1]*(b[0]*c[2]-b[2]*c[0])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6;
      }
      assert.ok(v>0.05,`boulder volume ${v.toFixed(3)}`);
    },
    edits:[['        this.quad(lower[k], upper[k], upper[j], lower[j], col);',
      '        this.quad(lower[k], lower[j], upper[j], upper[k], col);']]},
  {defect:'a tile stops matching the tile beside it',
    check:(mod)=>{
      const width=mod.tileWidth;
      const edge=(mesh,x)=>{const out=[];
        for(let i=0;i<mesh.positions.length;i+=3)
          if(Math.abs(mesh.positions[i]-x)<1e-6)out.push(mesh.positions[i+1].toFixed(5)+','+mesh.positions[i+2].toFixed(5));
        return Array.from(new Set(out)).sort();};
      assert.deepEqual(edge(mod.piece('cliff_face',{seed:'c',index:3}),width),edge(mod.piece('cliff_face',{seed:'c',index:4}),0));
    },
    edits:[['      const gj = index * cols + j, col = [];','      const gj = index * cols + j + index, col = [];']]},
  {defect:'the scatter seed stops reaching the placement',
    check:(mod)=>{
      const a=mod.scatter(1,'temperate_broadleaf',120,{width:90,depth:90});
      const b=mod.scatter(2,'temperate_broadleaf',120,{width:90,depth:90});
      const byCell=new Map(b.instances.map(i=>[i.cell,i]));
      let moved=0,compared=0;
      for(const i of a.instances){const j=byCell.get(i.cell);if(!j)continue;compared+=1;
        if(Math.hypot(i.x-j.x,i.z-j.z)>0.5)moved+=1;}
      assert.ok(moved/compared>0.8,`only ${(100*moved/compared).toFixed(1)}% moved`);
    },
    edits:[['      const x = box.x0 + (cx + 0.5 + askSigned(base, i * 4 + 2, 0.44)) * cw;',
      '      const x = box.x0 + (cx + 0.5 + askSigned(0, i * 4 + 2, 0.44)) * cw;'],
      ['      const z = box.z0 + (cz + 0.5 + askSigned(base, i * 4 + 3, 0.44)) * cd;',
        '      const z = box.z0 + (cz + 0.5 + askSigned(0, i * 4 + 3, 0.44)) * cd;']]},
  {defect:'smoothing stops happening',
    check:(mod)=>{
      const mesh=mod.piece('oak',{seed:9});
      const part=mesh.parts.find(p=>p.name==='canopy');
      let smoothed=0,corners=0;
      for(let i=part.first*3;i<(part.first+part.count)*3;i+=9){
        const a=mesh.positions.subarray(i,i+3),b=mesh.positions.subarray(i+3,i+6),c=mesh.positions.subarray(i+6,i+9);
        const u=[b[0]-a[0],b[1]-a[1],b[2]-a[2]],v=[c[0]-a[0],c[1]-a[1],c[2]-a[2]];
        const n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]];
        const area=Math.hypot(n[0],n[1],n[2]);
        for(let k=0;k<3;k+=1){const o=i+k*3;corners+=1;
          if((n[0]*mesh.normals[o]+n[1]*mesh.normals[o+1]+n[2]*mesh.normals[o+2])/area<0.999)smoothed+=1;}
      }
      assert.ok(smoothed>corners*0.5,`only ${smoothed} of ${corners} corners smoothed`);
    },
    edits:[['          if (ox * fx + oy * fy + oz * fz < CREASE) continue;','          continue;']]},
  // The three below were added 2026-09-06 by a verification pass that ran its
  // own sabotages against this file and found three defects it did not catch.
  // Each is recorded with the reading that proved the hole.
  {defect:'a family budget is widened instead of the piece being made to fit',
    // Widening the dressing band to [1, 900000] at both levels left all fifteen
    // tests green: the tree row was pinned to section 4, but dressing was asked
    // nothing at all, and shrub/grass/rock only asked to be cheaper than a tree.
    check:(mod)=>assert.deepEqual(mod.budgets().dressing,{near:[80,1800],far:[16,360]}),
    edits:[['    dressing: { near: [80, 1800], far: [16, 360] },','    dressing: { near: [1, 900000], far: [1, 900000] },']]},
  {defect:'two biomes collapse onto one silhouette',
    // Copying the oak's spec onto the holm oak made temperate and mediterranean
    // the same tree at the same size, and every bar stayed green: the only
    // silhouette claim in the file was that eight named kinds are family 'tree'.
    // Cloned, the pair reads 1.000 against a measured live worst of 0.293.
    check:(mod)=>{
      const {worst,pair}=worstConfusion(mod,0);
      assert.ok(worst<0.75,`${pair} share ${(100*worst).toFixed(1)}% of their silhouette`);
    },
    edits:[['      lobes: 5, trunkR: 0.062, lean: 0.16, gnarl: 0.34, roots: true, leafShift: -0.12,',
      '      lobes: 6, trunkR: 0.048, lean: 0.10, gnarl: 0.22, roots: true,'],
      ['      h: [6, 9.5], clear: 0.20, forks: 3, spread: 0.72, crown: 0.60, flat: 0.94,',
        '      h: [10, 16], clear: 0.36, forks: 4, spread: 0.60, crown: 0.56, flat: 0.80,']]},
  {defect:'foliage becomes a flat sheet standing on edge — a billboard',
    // Section 4 warns against alpha overdraw. Nothing in the file said a leaf
    // mass had to have thickness, so a canopy could be replaced by coplanar
    // vertical quads unnoticed. Verified: with `blob` emitting a single quad in
    // one plane, hazel scrub's foliage reads 0 m of slab thickness with a mean
    // normal.y of 0, against a live 0.427 m.
    check:(mod)=>{
      const mesh=mod.piece('hazel_scrub',{seed:11});
      const part=mesh.parts.find((p)=>p.name==='foliage');
      assert.ok(part,'hazel scrub has a foliage part');
      if(slabThickness(mesh.positions,part.first,part.count)>=0.01)return;
      let y=0;
      for(let i=part.first;i<part.first+part.count;i+=1)y+=mesh.normals[i*3+1];
      assert.ok(y/part.count>0.5,`foliage is a sheet on edge (mean normal.y ${(y/part.count).toFixed(3)})`);
    },
    edits:[['  Mesh.prototype.blob = function (cx, cy, cz, rx, ry, rz, seg, stacks, seed, rough, col, colTop, droop) {',
      '  Mesh.prototype.blob = function (cx, cy, cz, rx, ry, rz, seg, stacks, seed, rough, col, colTop, droop) {\n    return this.quad([cx - rx, cy - ry, 0], [cx + rx, cy - ry, 0], [cx + rx, cy + ry, 0], [cx - rx, cy + ry, 0], col);']]},
  {defect:'every face is emitted twice, wound both ways, so the sheets read from behind',
    check:(mod)=>{
      const mesh=mod.piece('oak',{seed:11}),p=mesh.positions,seen=new Set();
      for(let t=0;t<p.length/9;t+=1){
        const key=[0,1,2].map((j)=>{
          const i=(t*3+j)*3;
          return `${p[i].toFixed(4)},${p[i+1].toFixed(4)},${p[i+2].toFixed(4)}`;
        }).sort().join(';');
        assert.ok(!seen.has(key),`triangle ${t} is a duplicate face`);
        seen.add(key);
      }
    },
    // Single-line anchors only in this ledger: the module is CRLF, so a
    // multi-line anchor written with \n never matches and the sabotage silently
    // becomes a stale-anchor failure instead of the bar it meant to exercise.
    edits:[['    return this.tri(a, b, c, col).tri(a, c, d, col);',
      '    return this.tri(a, b, c, col).tri(a, c, b, col).tri(a, c, d, col).tri(a, d, c, col);']]},
];

function sabotaged(edits){
  let broken=source;
  for(const [from,to] of edits){
    assert.ok(broken.includes(from),`sabotage anchor is stale: ${from.slice(0,60)}`);
    broken=broken.replace(from,to);
  }
  const context=vm.createContext({});
  vm.runInContext(broken,context);
  return context.ScatterMesh;
}

test('every bar above goes red against a deliberately broken generator',()=>{
  for(const entry of SABOTAGE){
    const mod=sabotaged(entry.edits);
    let caught=null;
    try{sweep(mod,'sabotaged ');}catch(err){caught=err;}
    assert.ok(caught,`sabotage passed unnoticed: ${entry.defect}`);
    assert.ok(String(caught.message).includes(entry.expect),
      `sabotage "${entry.defect}" failed on the wrong bar: ${caught.message}`);
  }
  for(const entry of SABOTAGE_WIDE){
    const mod=sabotaged(entry.edits);
    let caught=null;
    try{entry.check(mod);}catch(err){caught=err;}
    assert.ok(caught,`sabotage passed unnoticed: ${entry.defect}`);
  }
  // And the ledger only means anything if the same sweep is green on the real
  // source, which the tests above already assert — this restates it so a reader
  // of this block alone can see both halves.
  sweep(kit,'live ');
});
