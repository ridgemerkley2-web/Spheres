// Exercise the logistics and civilian prop kit. The contract this file defends
// is not "the props are pretty": it is that the same inputs produce the same
// bytes in this process and in a fresh one, that every piece is inside the
// budget it will actually be drawn at, that the named ranges cover the buffer
// so a click can be resolved, that the shading is sound, that a tractor and a
// trailer really do couple at one documented height, and that nothing in the
// kit claims a vehicle or a capacity the game does not have.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const cp=require('node:child_process');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const file=path.resolve(__dirname,'../../spheres-web/ui/prop-mesh.js');
const prop=require(file);
const source=fs.readFileSync(file,'utf8');

// Budgets, quoted once. Near is the inspection mesh, map is what a yard scene
// draws forty of. A FLOOR as well as a ceiling: a prop under the floor is not
// cheap, it is unfinished, and the pallet is a shared part rather than a piece
// precisely because it could not honestly reach the floor.
const NEAR_MIN=300,NEAR_MAX=2500,MAP_MIN=30,MAP_MAX=200,YARD_MAX=8000;
// The smoothing crease, copied from the module. A vertex normal is an average
// of the faces within this limit OF ITS OWN FACE, so no vertex normal may ever
// point further from its triangle than this.
const CREASE=0.35;

const digest=m=>crypto.createHash('sha256')
  .update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.normals.buffer))
  .update(Buffer.from(m.colors.buffer)).digest('hex');
const faceNormal=(m,i)=>{
  const ax=m.positions[i],ay=m.positions[i+1],az=m.positions[i+2];
  const ux=m.positions[i+3]-ax,uy=m.positions[i+4]-ay,uz=m.positions[i+5]-az;
  const vx=m.positions[i+6]-ax,vy=m.positions[i+7]-ay,vz=m.positions[i+8]-az;
  const n=[uy*vz-uz*vy,uz*vx-ux*vz,ux*vy-uy*vx],len=Math.hypot(...n);
  return {n,len};
};

// Every mesh must satisfy this, at every piece, LOD and variant, and the yard
// and the coupled rig must satisfy it too. Written once because a defect that
// only appears on the map mesh of the nineteenth piece is exactly the defect
// nobody finds by looking.
function validate(mesh,label,budget){
  assert(mesh.positions instanceof Float32Array,`${label}: positions buffer`);
  assert(mesh.normals instanceof Float32Array,`${label}: normals buffer`);
  assert(mesh.colors instanceof Float32Array,`${label}: colors buffer`);
  assert.equal(mesh.positions.length,mesh.normals.length,`${label}: normal alignment`);
  assert.equal(mesh.positions.length,mesh.colors.length,`${label}: color alignment`);
  assert.equal(mesh.positions.length,mesh.triangleCount*9,`${label}: complete triangles`);
  if(budget){
    assert(mesh.triangleCount>=budget[0]&&mesh.triangleCount<=budget[1],
      `${label}: ${mesh.triangleCount} triangles outside ${budget[0]}..${budget[1]}`);
  }
  for(let i=0;i<mesh.positions.length;i+=3){
    for(let j=0;j<3;j++){
      assert(Number.isFinite(mesh.positions[i+j]),`${label}: finite position at ${i+j}`);
      assert(Number.isFinite(mesh.normals[i+j]),`${label}: finite normal at ${i+j}`);
      assert(Number.isFinite(mesh.colors[i+j]),`${label}: finite colour at ${i+j}`);
      assert(mesh.colors[i+j]>=0&&mesh.colors[i+j]<=1,`${label}: colour ${mesh.colors[i+j]} out of 0..1 at ${i+j}`);
      assert(mesh.positions[i+j]>=mesh.bounds.min[j]-1e-5&&mesh.positions[i+j]<=mesh.bounds.max[j]+1e-5,
        `${label}: vertex outside stated bounds on axis ${j}`);
    }
    assert(Math.abs(Math.hypot(mesh.normals[i],mesh.normals[i+1],mesh.normals[i+2])-1)<1e-5,
      `${label}: unit normal at ${i}`);
  }
  for(let i=0;i<mesh.positions.length;i+=9){
    const {n,len}=faceNormal(mesh,i);
    assert(len>1e-9,`${label}: nondegenerate triangle ${i/9}`);
    // Smoothing may turn a vertex normal away from its face, but never past
    // the crease limit and never behind the triangle. That is the property the
    // averaging is written to guarantee and it is asserted, not assumed.
    for(let k=0;k<3;k++){
      const dot=n.reduce((sum,value,j)=>sum+(value/len)*mesh.normals[i+k*3+j],0);
      assert(dot>=CREASE-1e-4,`${label}: vertex normal ${dot.toFixed(3)} behind its face at triangle ${i/9}`);
    }
  }
  // Ground contact. A prop is placed on terrain by its base and nothing else,
  // so this is exact and not a tolerance.
  assert.equal(mesh.bounds.min[1],0,`${label}: ground contact at Y=0`);
  assert(mesh.bounds.max[1]>0.5,`${label}: has height`);
  // Named ranges: contiguous, non-overlapping, covering every vertex, with a
  // group and a label the picker can print.
  let previous=0;const names=new Set();
  for(const part of mesh.parts){
    assert(!names.has(part.name),`${label}: duplicate part ${part.name}`);names.add(part.name);
    assert.equal(part.first,previous,`${label}: part ${part.name} is not contiguous`);
    assert(part.count>0&&part.count%3===0,`${label}: whole triangles in ${part.name}`);
    assert(part.group&&part.label,`${label}: part semantics on ${part.name}`);
    previous=part.first+part.count;
  }
  assert.equal(previous,mesh.positions.length/3,`${label}: parts cover every vertex`);
  assert(mesh.parts.length>=2,`${label}: only ${mesh.parts.length} named parts`);
}

test('the kit covers the piece list the assignment names, in both directions',()=>{
  const keys=prop.pieces();
  assert.equal(keys.length,25,'twenty-five pieces');
  assert.equal(new Set(keys).size,keys.length,'piece keys are unique');
  for(const wanted of ['tractor_unit','trailer_box','trailer_flatbed','trailer_tanker','rigid_box_van',
    'service_van','bus','car','forklift','fuel_bowser','container_20ft','container_40ft','container_stack',
    'pallet_stack','crate_stack','yard_crane','gantry_crane','locomotive','wagon_container','wagon_tank',
    'wagon_hopper','storage_tank','silo_group','conveyor','light_mast'])
    assert(keys.includes(wanted),`the kit builds ${wanted}`);
  for(const key of keys){
    const meta=prop.meta(key);
    assert(meta&&meta.name&&meta.blurb&&meta.kind,`${key}: described`);
    assert.equal(meta.key,key);
  }
  assert.equal(prop.meta('not_a_prop'),null);
  assert.deepEqual(prop.budgets.near,[NEAR_MIN,NEAR_MAX]);
  assert.deepEqual(prop.budgets.map,[MAP_MIN,MAP_MAX]);
  assert.equal(prop.budgets.yard,YARD_MAX);
});

test('every piece builds at both LODs and at several variants, inside its budget',()=>{
  for(const key of prop.pieces()){
    for(const lod of [0,1]){
      const budget=lod?[MAP_MIN,MAP_MAX]:[NEAR_MIN,NEAR_MAX];
      for(const variant of [0,1,7,999]){
        const mesh=prop.build(key,{lod,variant});
        validate(mesh,`${key}/lod${lod}/v${variant}`,budget);
        assert.equal(mesh.piece,key);assert.equal(mesh.lod,lod);assert.equal(mesh.variant,variant);
      }
    }
    // The map mesh is a cheaper view of the same prop, never a dearer one.
    assert(prop.build(key,{lod:1}).triangleCount<prop.build(key,{lod:0}).triangleCount,
      `${key}: the map mesh must be cheaper than the near mesh`);
    // Both words and both numbers mean the coarse level, so a caller borrowing
    // site-mesh.js's or town-mesh.js's vocabulary gets the mesh it asked for
    // rather than silently getting the expensive one.
    const map=digest(prop.build(key,{lod:1}));
    for(const spelling of [1,2,'map','far','lod1','lod2'])
      assert.equal(digest(prop.build(key,{lod:spelling})),map,`${key}: lod ${spelling} is the map mesh`);
    for(const spelling of [0,undefined,null,'near','nonsense'])
      assert.equal(digest(prop.build(key,{lod:spelling})),digest(prop.build(key,{lod:0})),
        `${key}: lod ${spelling} is the near mesh`);
  }
});

test('geometry is deterministic in process, across a fresh load, and in a separate process',()=>{
  const options={lod:0,variant:11},snapshot=JSON.stringify(options);
  delete require.cache[require.resolve(file)];const fresh=require(file);
  assert.deepEqual(fresh.pieces(),prop.pieces());
  for(const key of prop.pieces()){
    const a=prop.build(key,options),b=fresh.build(key,options),c=prop.build(key,{...options});
    assert.equal(JSON.stringify(options),snapshot,`${key}: build does not mutate its options`);
    for(const field of ['positions','normals','colors'])
      assert.deepEqual(a[field],b[field],`${key}: deterministic ${field} across a fresh load`);
    assert.equal(digest(a),digest(c),`${key}: a repeat build is identical`);
    assert.deepEqual(a.bounds,b.bounds);assert.deepEqual(a.parts,b.parts);
  }
  // A SEPARATE PROCESS, because that is the claim the iron rule actually makes
  // and a module-level cache would satisfy everything above without it.
  const script=`const p=require(${JSON.stringify(file)});const c=require("node:crypto").createHash("sha256");`
    +`for(const k of p.pieces())for(const lod of [0,1]){const m=p.build(k,{lod,variant:11});`
    +`c.update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.normals.buffer))`
    +`.update(Buffer.from(m.colors.buffer));}`
    +`c.update(Buffer.from(p.yard().positions.buffer));process.stdout.write(c.digest("hex"));`;
  const here=crypto.createHash('sha256');
  for(const key of prop.pieces())for(const lod of [0,1]){
    const m=prop.build(key,{lod,variant:11});
    here.update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.normals.buffer))
      .update(Buffer.from(m.colors.buffer));
  }
  here.update(Buffer.from(prop.yard().positions.buffer));
  const child=cp.spawnSync(process.execPath,['-e',script],{encoding:'utf8'});
  assert.equal(child.status,0,`child process failed: ${child.stderr}`);
  assert.equal(child.stdout,here.digest('hex'),'a separate process builds byte-identical geometry');
});

test('variety comes from the variant and from nothing else',()=>{
  for(const key of prop.pieces()){
    const seen=new Map();
    for(let v=0;v<8;v+=1)seen.set(v,digest(prop.build(key,{lod:0,variant:v})));
    // Two different variants of the same piece are allowed to be identical —
    // most of this kit is unpainted steel and only some pieces take a livery —
    // but a variant must never be unstable between calls.
    for(let v=0;v<8;v+=1)
      assert.equal(digest(prop.build(key,{lod:0,variant:v})),seen.get(v),`${key}: variant ${v} is stable`);
    // Out-of-range and malformed variants resolve to something, deterministically.
    for(const bad of [Number.NaN,Infinity,-0,'3',null,undefined,{}])
      assert.equal(digest(prop.build(key,{lod:0,variant:bad})),seen.get(0),`${key}: variant ${String(bad)} falls back to 0`);
    assert.equal(digest(prop.build(key,{lod:0,variant:-7})),digest(prop.build(key,{lod:0,variant:7})),
      `${key}: a negative variant is the same draw as its magnitude`);
  }
  // At least some of the kit does vary, or the hash is decorative.
  const varies=prop.pieces().filter(key=>digest(prop.build(key,{variant:0}))!==digest(prop.build(key,{variant:3})));
  assert(varies.length>=6,`only ${varies.length} pieces respond to the variant at all`);
});

test('curved surfaces are smoothed, flat ones are not, and hard edges stay hard',()=>{
  // A box-only prop must come out perfectly flat. The container is all plate,
  // rail, corrugation and casting, so if anything in it is smoothed the crease
  // limit or the grouping is wrong.
  const box=prop.build('container_20ft',{lod:0});
  for(let i=0;i<box.positions.length;i+=9){
    const {n,len}=faceNormal(box,i);
    for(let k=0;k<3;k++){
      const dot=n.reduce((sum,value,j)=>sum+(value/len)*box.normals[i+k*3+j],0);
      assert(dot>0.9999,`container: flat plate must keep its face normal, got ${dot.toFixed(4)}`);
    }
  }
  // A prop with a barrel, wheels and dished ends must have real smoothing in
  // it — and must still keep flat faces elsewhere, or "smooth everything" has
  // been quietly substituted for "smooth what curves".
  for(const key of ['trailer_tanker','storage_tank','car','wagon_tank']){
    const mesh=prop.build(key,{lod:0});
    let smoothed=0,flat=0;
    for(let i=0;i<mesh.positions.length;i+=9){
      const {n,len}=faceNormal(mesh,i);
      for(let k=0;k<3;k++){
        const dot=n.reduce((sum,value,j)=>sum+(value/len)*mesh.normals[i+k*3+j],0);
        if(dot>0.9999)flat++;else smoothed++;
      }
    }
    assert(smoothed>60,`${key}: only ${smoothed} smoothed vertices — curvature is not being shaded`);
    assert(flat>smoothed,`${key}: ${flat} flat vertices against ${smoothed} smoothed — plate is being welded`);
    // A hard edge survives: somewhere in the mesh, one position carries two
    // different normals. Without this, "smoothed" and "welded into a blob" pass
    // the same test.
    const atPoint=new Map();let split=0;
    for(let i=0;i<mesh.positions.length;i+=3){
      const key3=[0,1,2].map(j=>Math.round(mesh.positions[i+j]*8192)).join(',');
      const nrm=[0,1,2].map(j=>Math.round(mesh.normals[i+j]*512)).join(',');
      const held=atPoint.get(key3);
      if(held===undefined)atPoint.set(key3,nrm);else if(held!==nrm)split+=1;
    }
    assert(split>20,`${key}: only ${split} split normals — hard edges are being smoothed away`);
  }
});

test('a tractor and a trailer couple at one documented height and the rig still stands on the ground',()=>{
  const head=prop.build('tractor_unit',{lod:0});
  assert.deepEqual(Object.keys(head.sockets),['fifth_wheel']);
  assert.equal(head.sockets.fifth_wheel[1],prop.couplingHeight,'the fifth wheel is at the coupling height');
  for(const key of ['trailer_box','trailer_flatbed','trailer_tanker']){
    const tail=prop.build(key,{lod:0});
    assert.deepEqual(Object.keys(tail.sockets),['kingpin']);
    assert.equal(tail.sockets.kingpin[1],prop.couplingHeight,`${key}: the kingpin is at the coupling height`);
    assert.equal(tail.sockets.kingpin[0],0,`${key}: the kingpin is on the centreline`);
    const rig=prop.rig('tractor_unit',key,{lod:0});
    validate(rig,`rig/${key}`,[NEAR_MIN,NEAR_MAX*2]);
    // The coupling is a translation in Z ALONE. A fifth wheel and a kingpin at
    // different heights is exactly how a coupled vehicle ends up hovering, and
    // it is invisible in a screenshot taken from the side.
    assert.equal(rig.coupling.offset[0],0,`${key}: no sideways offset in the coupling`);
    assert.equal(rig.coupling.offset[1],0,`${key}: no height offset in the coupling`);
    assert(rig.coupling.offset[2]<0,`${key}: the trailer sits behind the tractor`);
    assert.equal(rig.bounds.min[1],0,`${key}: the coupled rig touches the ground`);
    assert(rig.bounds.max[2]-rig.bounds.min[2]>14,`${key}: the rig is an articulated vehicle length`);
    // The rig is exactly its two halves: the tractor as built, plus the trailer
    // as built COUPLED and translated. Nothing is re-authored for the assembly.
    const coupled=prop.build(key,{lod:0,coupled:true});
    assert.equal(rig.triangleCount,head.triangleCount+coupled.triangleCount,`${key}: the rig is its two halves`);
    assert.equal(rig.parts.length,head.parts.length+coupled.parts.length,`${key}: part ranges carry across`);
    assert(rig.parts.every(part=>part.group==='tractor'||part.group==='trailer'),`${key}: rig parts are attributed`);
    for(let i=0;i<coupled.positions.length;i+=3){
      const at=head.positions.length+i;
      assert(Math.abs(rig.positions[at]-coupled.positions[i])<1e-4,`${key}: trailer geometry is reused as-is`);
      assert(Math.abs(rig.positions[at+2]-(coupled.positions[i+2]+rig.coupling.offset[2]))<1e-4,
        `${key}: the trailer is placed by the coupling offset`);
    }
    // Landing legs are down when the trailer is parked and up when it is under
    // a tractor. Drawing one state for both is a small lie that makes a yard
    // read as a diagram.
    assert.notEqual(digest(coupled),digest(prop.build(key,{lod:0})),`${key}: a coupled trailer is posed differently`);
    assert.equal(coupled.bounds.min[1],0,`${key}: a coupled trailer still stands on its wheels`);
    assert(coupled.description.includes('landing legs raised'),`${key}: the description says the legs are up`);
  }
  // Only the pieces that couple carry sockets.
  for(const key of prop.pieces()){
    const sockets=Object.keys(prop.build(key,{lod:1}).sockets);
    const expected=key==='tractor_unit'?1:key.startsWith('trailer_')?1:0;
    assert.equal(sockets.length,expected,`${key}: ${sockets.length} sockets`);
  }
  // Nonsense arguments give a usable vehicle rather than a throw.
  assert.equal(prop.rig('not_a_tractor','not_a_trailer').tractor,'tractor_unit');
  assert.equal(prop.rig(null,null).trailer,'trailer_box');
  // A tractor is not a trailer: coupling one to a piece with no kingpin returns
  // the tractor rather than inventing a joint.
  assert.equal(prop.rig('tractor_unit','container_20ft').piece,'tractor_unit');
});

test('reuse is measured, and every shared part builder serves more than one piece',()=>{
  const before=prop.pieces().map(key=>digest(prop.build(key,{lod:0})));
  const reuse=prop.reuse();
  assert.equal(reuse.pieceCount,prop.pieces().length);
  assert(reuse.builderCount>=15,`only ${reuse.builderCount} shared builders`);
  assert.equal(reuse.sharedByMoreThanOne,reuse.builderCount,
    'a builder used by exactly one piece is not a shared part, it is that piece');
  for(const name of reuse.builders){
    assert(reuse.byBuilder[name].length>=2,`${name}: serves only ${reuse.byBuilder[name].join(',')}`);
    for(const key of reuse.byBuilder[name])
      assert(reuse.pieces[key].includes(name),`${name}/${key}: the two directions of the matrix disagree`);
  }
  // The parts that carry the argument for a kit rather than a pile of meshes.
  assert(reuse.byBuilder.roadWheel.length>=10,'the wheel is shared across the road fleet');
  assert(reuse.byBuilder.semiChassis.length>=3,'one trailer chassis under the box, flat and tanker');
  assert(reuse.byBuilder.isoContainer.length>=3,'one container in the boxes and the stack');
  assert(reuse.byBuilder.tankBarrel.length>=3,'one barrel on the road and on the rails');
  assert(reuse.byBuilder.railBogie.length>=4,'one bogie under the locomotive and every wagon');
  assert(reuse.byBuilder.roadCab.length>=3,'one cab on the tractor, the rigid and the bowser');
  // Every piece uses at least one shared part except the ones that honestly
  // have none, and the trace never touches the geometry it measures.
  const orphans=prop.pieces().filter(key=>reuse.pieces[key].length===0);
  assert.deepEqual(orphans,[],`pieces sharing nothing: ${orphans.join(',')}`);
  // THE TRACE MUST NOT TOUCH THE GEOMETRY IT MEASURES. The counts in the report
  // come from traced builds; these come from untraced ones, and they have to
  // agree, or the reuse matrix is a measurement of a different kit.
  prop.pieces().forEach((key,i)=>{
    assert.deepEqual(reuse.triangles[key],
      [prop.build(key,{lod:0}).triangleCount,prop.build(key,{lod:1}).triangleCount],
      `${key}: the traced build differs from the untraced one`);
    assert.equal(digest(prop.build(key,{lod:0})),before[i],`${key}: the trace left a residue`);
  });
  assert.deepEqual(prop.reuse(),reuse,'the reuse report is itself deterministic');
});

test('a yard of forty props at map detail stays inside the scene budget',()=>{
  const yard=prop.yard();
  assert.equal(yard.lod,1,'the yard defaults to the map mesh, which is the question the budget asks');
  assert.equal(yard.propCount,40);
  assert.equal(prop.yardRoster().length,40);
  validate(yard,'yard/map',null);
  assert(yard.triangleCount<=YARD_MAX,`the yard costs ${yard.triangleCount} triangles, over ${YARD_MAX}`);
  assert.equal(yard.parts.length,40,'one named range per placed prop');
  // The roster has to be a yard and not forty copies of the cheapest prop, or
  // the measurement means nothing.
  const kinds=new Set(prop.yardRoster().map(slot=>slot.piece));
  assert(kinds.size>=15,`the yard uses only ${kinds.size} distinct pieces`);
  for(const slot of prop.yardRoster()){
    assert(prop.pieces().includes(slot.piece),`${slot.piece}: on the roster but not in the kit`);
    assert(Number.isInteger(slot.yaw)&&slot.yaw>=0&&slot.yaw<4,`${slot.piece}: cardinal yaw only`);
  }
  // The sum of the placed props IS the scene: placement adds no geometry.
  const sum=prop.yardRoster().reduce((total,slot,i)=>total+prop.build(slot.piece,{lod:1,variant:i}).triangleCount,0);
  assert.equal(yard.triangleCount,sum,'the yard is exactly the props placed in it');
  // Cardinal yaw keeps normals exactly unit through the placement.
  for(let i=0;i<yard.normals.length;i+=3)
    assert(Math.abs(Math.hypot(yard.normals[i],yard.normals[i+1],yard.normals[i+2])-1)<1e-5,'placed normals stay unit');
  assert.equal(digest(prop.yard()),digest(prop.yard()),'the yard is deterministic');
  // The near yard is legal geometry too, and honestly dearer.
  const near=prop.yard({lod:0});
  validate(near,'yard/near',null);
  assert(near.triangleCount>yard.triangleCount*4,'the near yard costs what near detail costs');
});

// The widest |x| reached by each named part, and by the mesh as a whole. Written
// per PART rather than per mesh because "this vehicle is too wide" is not an
// actionable failure and "its mudguards are too wide" is.
const halfWidths=mesh=>mesh.parts.map(part=>{
  let half=0;
  for(let v=part.first;v<part.first+part.count;v+=1)half=Math.max(half,Math.abs(mesh.positions[v*3]));
  return {name:part.name,group:part.group,half};
});
// The |x| span of whatever is touching the ground behind `zMax`. At Y=0 the
// only geometry back there is tyre, so this measures the track and nothing
// else — the z cut is what keeps a parked trailer's landing-leg feet, which
// also stand on Y=0, out of the reading.
const trackAtGround=(mesh,match,zMax)=>{
  let inner=Infinity,outer=0;
  for(const part of mesh.parts){
    if(!match.test(part.name))continue;
    for(let v=part.first;v<part.first+part.count;v+=1){
      if(mesh.positions[v*3+1]>0.01||mesh.positions[v*3+2]>zMax)continue;
      const x=Math.abs(mesh.positions[v*3]);
      if(x<inner)inner=x;if(x>outer)outer=x;
    }
  }
  assert(outer>0,'the track probe found no ground contact to measure');
  return {inner,outer};
};

test('road vehicles stay inside a 1990 road envelope and a trailer never oversails its tractor',()=>{
  // WHY THESE TWO NUMBERS. EC Directive 85/3/EEC put the goods-vehicle width
  // limit at 2.50 m, and that is the limit that was in force in 1990; the
  // 2.55 m of 96/53/EC (2.60 m for refrigerated bodies) arrived later in the
  // decade. This kit's widest STRUCTURE is a rub rail at 2.58 m, so the bar is
  // 2.60 m — representative scenery may round a few centimetres, and this test
  // exists because the trailer bogie once stood at 3.20 m, which is not
  // rounding. MIRRORS are the one thing allowed outside the body line, and only
  // on something with a cab to hang them from: a 1990 truck's flat mirrors on
  // tubular arms genuinely take a 2.44 m cab out to about 3.00 m.
  const STRUCTURE=2.60,WITH_MIRRORS=3.05;
  // A trailer has no cab and therefore no mirrors, so it gets the body bar.
  const mirrored=new Set(['tractor_unit','rigid_box_van','service_van','bus','car','fuel_bowser','forklift']);
  for(const key of ['tractor_unit','trailer_box','trailer_flatbed','trailer_tanker','rigid_box_van',
    'service_van','bus','car','fuel_bowser']){
    const limit=mirrored.has(key)?WITH_MIRRORS:STRUCTURE;
    const mesh=prop.build(key,{lod:0});
    const width=mesh.bounds.max[0]-mesh.bounds.min[0];
    const widest=halfWidths(mesh).sort((a,b)=>b.half-a.half)[0];
    assert(width<=limit,`${key}: ${width.toFixed(2)} m wide, over ${limit} m, at "${widest.name}"`);
    // Symmetry, because a road vehicle that is 2.5 m wide but sits 0.3 m off
    // its own centreline is just as wrong and the width alone will not say so.
    assert(Math.abs(mesh.bounds.max[0]+mesh.bounds.min[0])<0.02,`${key}: not centred on its own centreline`);
    // The running gear is never the widest thing on a vehicle with a body over
    // it. The tanker is the honest exception the roadmap's "silhouettes stay
    // distinct" asks for: its barrel really is narrower than its frame.
    if(key!=='trailer_tanker'&&key!=='forklift')
      assert(!/^running gear/.test(widest.name),`${key}: the widest part is "${widest.name}"`);
  }
  // ONE ROAD STANDARD ACROSS THE KIT. The tractor and the trailer it pulls put
  // their twin tyres on the same lines, because they are the same vehicle under
  // the same rules. This is the assertion that pins the defect at its root
  // rather than at its symptom: the bogie was passed halfW + 0.16 while the
  // unit was passed halfW, and the overall width was only how that showed up.
  const unit=trackAtGround(prop.build('tractor_unit',{lod:0}),/drive axles/,-1);
  for(const key of ['trailer_box','trailer_flatbed','trailer_tanker']){
    const bogie=trackAtGround(prop.build(key,{lod:0}),/^running gear/,-1);
    assert(Math.abs(bogie.outer-unit.outer)<0.02,
      `${key}: bogie tyres reach ${bogie.outer.toFixed(3)} m from the centreline, the tractor's ${unit.outer.toFixed(3)} m`);
    assert(Math.abs(bogie.inner-unit.inner)<0.02,`${key}: bogie track is not the tractor's track`);
  }
  // AND ON THE COUPLED VEHICLE the widest point belongs to the tractor. That is
  // not a styling preference: the mirrors have to see past the trailer, so a
  // trailer wider than them is a trailer the driver cannot see down the side of.
  for(const key of ['trailer_box','trailer_flatbed','trailer_tanker']){
    const rig=prop.rig('tractor_unit',key,{lod:0});
    let head=0,tail=0;
    for(const part of halfWidths(rig))if(part.group==='tractor')head=Math.max(head,part.half);
    else tail=Math.max(tail,part.half);
    assert(tail<head,`${key}: the trailer reaches ${tail.toFixed(2)} m and the tractor's mirrors only ${head.toFixed(2)} m`);
    assert(rig.bounds.max[0]-rig.bounds.min[0]<=WITH_MIRRORS,`${key}: the coupled rig is over ${WITH_MIRRORS} m wide`);
  }
  // The containers are the one place in the kit where the outside world fixes
  // the numbers, so they are checked against it rather than against a taste.
  // ISO 668 series 1: 2.438 m wide, 2.591 m high, 6.058 m and 12.192 m long.
  for(const [key,length] of [['container_20ft',6.058],['container_40ft',12.192]]){
    const box=prop.build(key,{lod:0});
    assert(Math.abs(box.bounds.max[0]-box.bounds.min[0]-2.438)<0.06,`${key}: not an ISO width`);
    assert(Math.abs(box.bounds.max[1]-2.591)<0.06,`${key}: not an ISO height`);
    assert(Math.abs(box.bounds.max[2]-box.bounds.min[2]-length)<0.09,`${key}: not an ISO length`);
  }
});

test('nothing in the kit claims a real vehicle, a measured dimension or a capability',()=>{
  for(const key of prop.pieces()){
    const mesh=prop.build(key,{lod:0});
    assert(mesh.description.includes('Representative scenery'),`${key}: says what it is`);
    assert(mesh.description.includes('not a licensed, named or measured vehicle'),`${key}: refuses to be a real vehicle`);
    assert(mesh.description.includes('no cargo, capacity or capability'),`${key}: grants nothing`);
    assert(mesh.description.includes('1990'),`${key}: names its era baseline`);
  }
  assert(prop.rig('tractor_unit','trailer_tanker').description.includes('carries no load'),
    'the rig refuses to carry a load the simulation has not recorded');
  assert(prop.yard().description.includes('no throughput, storage or capacity'),
    'the yard refuses to be a terminal capacity');
  // The pieces most likely to be read as a capacity say so in their own words.
  assert(prop.meta('fuel_bowser').blurb.includes('holds no fuel'));
  assert(prop.meta('storage_tank').blurb.includes('stores nothing'));
  assert(prop.meta('gantry_crane').blurb.includes('not a port capacity'));
  assert(prop.meta('wagon_container').blurb.includes('carrying no container'));
  // And nowhere does the source name a manufacturer or a model. Cheap to keep
  // true, expensive to discover later in a screenshot.
  for(const marque of [/mercedes/i,/scania/i,/volvo/i,/\bman\s+truck/i,/maersk/i,/hanjin/i,/evergreen/i,
    /caterpillar/i,/\bkomatsu\b/i,/liebherr/i,/kalmar/i,/\bford\b/i,/leyland/i,/kenworth/i])
    assert(!marque.test(source),`prop-mesh.js must not name ${marque}`);
});

test('no wall clock, no frame counter and no second RNG can reach this geometry',()=>{
  // Iron rule 1, kept the cheapest way there is: the vocabulary does not exist
  // in the file. A prop left on screen for an hour is the same prop, and two
  // machines building the same yard build the same bytes.
  for(const pattern of [/\bDate\b/,/\bnow\s*\(/,/performance/i,/hrtime/,/requestAnimationFrame/i,
    /\bsetTimeout\b/,/\bsetInterval\b/,/Math\s*\.\s*random/,/\brandom\b/i,/getTime/,/\bMath\.trunc\(Math\./])
    assert(!pattern.test(source),`prop-mesh.js must not contain ${pattern}`);
  // And no DOM, so the geometry stays checkable outside a browser.
  for(const pattern of [/\bdocument\b/,/\bwindow\b/,/canvas/i,/WebGL/i,/require\s*\(/,/\bimport\b/,/\bfetch\b/])
    assert(!pattern.test(source),`prop-mesh.js must not contain ${pattern}`);
  // Iteration order can decide geometry as surely as a clock can. The only Map
  // in the file is the smoothing bucket and it is looked up, never walked.
  assert(!/for\s*\(\s*const\s*\[[^\]]*\]\s*of\s*buckets/.test(source),'the smoothing map must not be iterated');
  assert(!/buckets\.(keys|values|entries|forEach)/.test(source),'the smoothing map must not be iterated');
});

test('the browser global exports the same contract with no dependencies',()=>{
  const context=vm.createContext({});
  vm.runInContext(source,context);
  assert.equal(typeof context.PropMesh.build,'function');
  assert.equal(typeof context.PropMesh.pieces,'function');
  assert.equal(typeof context.PropMesh.rig,'function');
  assert.equal(typeof context.PropMesh.yard,'function');
  assert.equal(typeof context.PropMesh.reuse,'function');
  assert.equal(context.PropMesh.pieces().join(','),prop.pieces().join(','));
  const sandboxed=context.PropMesh.build('locomotive',{lod:0,variant:4});
  assert.equal(sandboxed.triangleCount,prop.build('locomotive',{lod:0,variant:4}).triangleCount);
  assert.equal(context.PropMesh.yard().triangleCount,prop.yard().triangleCount);
  // The module surface is frozen, so a page cannot monkey-patch a prop into
  // saying something the checks have not read.
  assert(Object.isFrozen(prop),'the exported api is frozen');
});

test('unknown and malformed input resolves to a stable, honest default',()=>{
  const fallback=prop.build('container_20ft',{lod:0});
  for(const input of ['not_a_piece','__proto__','toString','',null,undefined,0,{},[]]){
    const mesh=prop.build(input,{lod:0});
    assert.equal(mesh.piece,'container_20ft',`${String(input)}: falls back to a known piece`);
    assert.equal(digest(mesh),digest(fallback),`${String(input)}: the fallback is one mesh`);
    assert.equal(mesh.requestedPiece,input,'the request is reported back unchanged');
  }
  for(const options of [undefined,null,'nonsense',0,{lod:{}},{variant:[]},{coupled:'yes'}])
    validate(prop.build('trailer_box',options),`options ${JSON.stringify(options)}`,[NEAR_MIN,NEAR_MAX]);
  for(const options of [undefined,{},{lod:'map'},{variant:3}])
    validate(prop.yard(options),`yard options ${JSON.stringify(options)}`,null);
});
