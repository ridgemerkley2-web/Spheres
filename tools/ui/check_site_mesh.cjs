// Exercise the staged construction-site geometry the province and construction
// views draw. The contract this file defends is not "the meshes are pretty": it
// is that a stage is a pure function of the work the server has recorded, that
// nothing in the generator can advance a building by being looked at, and that
// every one of the thirteen project kinds has something correct to show.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const file=path.resolve(__dirname,'../../spheres-web/ui/site-mesh.js');
const site=require(file);
const source=fs.readFileSync(file,'utf8');
const STAGES=['site','foundation','frame','enclosed','complete'];
const NEAR_MIN=2000,NEAR_MAX=12000,FAR_MIN=100,FAR_MAX=800;
const digest=m=>crypto.createHash('sha256').update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.colors.buffer)).digest('hex');
const monotone=values=>values.every((value,i)=>i===0||value>=values[i-1]);

// Every mesh must satisfy this, at every kind, stage, level, status and LOD.
// Written once because a defect that only shows up on the fourth stage of the
// eleventh kind is exactly the defect nobody looks for by hand.
function validate(mesh,label,lod){
  assert(mesh.positions instanceof Float32Array,`${label}: positions buffer`);
  assert(mesh.normals instanceof Float32Array,`${label}: normals buffer`);
  assert(mesh.colors instanceof Float32Array,`${label}: colors buffer`);
  assert.equal(mesh.positions.length,mesh.normals.length,`${label}: normal alignment`);
  assert.equal(mesh.positions.length,mesh.colors.length,`${label}: color alignment`);
  assert.equal(mesh.positions.length,mesh.triangleCount*9,`${label}: complete triangles`);
  const min=lod?FAR_MIN:NEAR_MIN,max=lod?FAR_MAX:NEAR_MAX;
  assert(mesh.triangleCount>=min&&mesh.triangleCount<=max,`${label}: ${mesh.triangleCount} triangles outside ${min}..${max}`);
  for(let i=0;i<mesh.positions.length;i+=3){
    for(let j=0;j<3;j++){
      assert(Number.isFinite(mesh.positions[i+j]),`${label}: finite position at ${i+j}`);
      assert(Number.isFinite(mesh.normals[i+j]),`${label}: finite normal at ${i+j}`);
      assert(mesh.colors[i+j]>=0&&mesh.colors[i+j]<=1,`${label}: color in range at ${i+j}`);
      assert(mesh.positions[i+j]>=mesh.bounds.min[j]-1e-5&&mesh.positions[i+j]<=mesh.bounds.max[j]+1e-5,`${label}: vertex within bounds`);
    }
    assert(Math.abs(Math.hypot(...mesh.normals.subarray(i,i+3))-1)<1e-5,`${label}: unit normal at ${i}`);
  }
  for(let i=0;i<mesh.positions.length;i+=9){
    const a=mesh.positions.subarray(i,i+3),b=mesh.positions.subarray(i+3,i+6),c=mesh.positions.subarray(i+6,i+9);
    const u=b.map((v,j)=>v-a[j]),v=c.map((value,j)=>value-a[j]);
    const n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]],area=Math.hypot(...n);
    assert(area>1e-9,`${label}: nondegenerate triangle ${i/9}`);
    assert(n.reduce((sum,value,j)=>sum+value/area*mesh.normals[i+j],0)>0.999,`${label}: normal follows winding at ${i/9}`);
  }
  // Ground contact. A site is placed on terrain by its base and nothing else,
  // so this is exact and not a tolerance.
  assert.equal(mesh.bounds.min[1],0,`${label}: ground contact at Y=0`);
  assert(mesh.bounds.max[1]>3,`${label}: visible height`);
  for(const axis of [0,2])assert(mesh.bounds.max[axis]-mesh.bounds.min[axis]>20,`${label}: compound footprint on axis ${axis}`);
  let previous=0;const names=new Set();
  for(const part of mesh.parts){
    assert(!names.has(part.name),`${label}: unique part ${part.name}`);names.add(part.name);
    assert.equal(part.first,previous,`${label}: contiguous part ${part.name}`);
    assert(part.count>0&&part.count%3===0,`${label}: whole triangles in ${part.name}`);
    assert(part.group&&part.label,`${label}: part semantics on ${part.name}`);
    previous=part.first+part.count;
  }
  assert.equal(previous,mesh.positions.length/3,`${label}: parts cover every triangle`);
  assert(mesh.parts.length>=(lod?4:9),`${label}: only ${mesh.parts.length} named parts`);
}

test('the kind table is exactly PROJECT_KINDS, in both directions',()=>{
  const rust=fs.readFileSync(path.resolve(__dirname,'../../spheres-sim/src/production.rs'),'utf8');
  const block=rust.slice(rust.indexOf('impl ProjectKind {'),rust.indexOf('pub fn parse'));
  const keys=[...block.matchAll(/=>\s*"([a-z_]+)"/g)].map(match=>match[1]);
  assert.equal(keys.length,13,'thirteen project kinds in production.rs');
  assert.deepEqual(site.kinds().slice().sort(),keys.slice().sort());
  for(const key of keys)assert(site.meta(key),`${key}: covered by the art`);
  assert.equal(site.meta('not_a_project_kind'),null);
});

// The province level is swept here rather than left at its default, because
// the compound grows with it and the map budget is the thing that grows into.
// This test passed at level one while machinery_works spent 830 of its 800 map
// triangles at level three and above, almost half of them on perimeter fence
// posts smaller than a pixel at that zoom. A budget asked at one level is a
// budget that cannot fail.
test('every kind builds at every stage and every level, at both detail levels, inside its budget',()=>{
  for(const key of site.kinds()){
    for(const stage of STAGES){
      for(const lod of [0,1]){
        for(let level=1;level<=site.maxLevel;level+=1){
          const mesh=site.build(key,stage,{lod,level});
          validate(mesh,`${key}/${stage}/lod${lod}/L${level}`,lod);
          assert.equal(mesh.kind,key);assert.equal(mesh.stage,stage);assert.equal(mesh.lod,lod);assert.equal(mesh.level,level);
        }
      }
    }
  }
});

test('stage tables are ordered, cover 0..1 and describe the same five states for every kind',()=>{
  for(const key of site.kinds()){
    const rows=site.stages(key);
    assert.deepEqual(rows.map(row=>row.key),STAGES);
    assert.equal(rows[0].from,0);assert.equal(rows[4].from,1);
    rows.forEach((row,i)=>{
      assert.equal(row.index,i);
      assert(row.name&&row.work,`${key}: stage ${row.key} is described`);
      if(i)assert(row.from>rows[i-1].from,`${key}: thresholds ascend`);
      assert.equal(row.to,i<4?rows[i+1].from:1);
    });
  }
});

test('the stage is a pure function of recorded server progress',()=>{
  for(const key of site.kinds()){
    const rows=site.stages(key);
    for(const row of rows){
      // Two different progress readings inside one band are the same site.
      const low=row.from+(row.key==='complete'?0:1e-4),high=row.key==='complete'?1:row.to-1e-4;
      const a=site.build(key,low),b=site.build(key,high);
      assert.equal(a.stage,row.key,`${key}: ${low} is ${row.key}`);
      assert.equal(b.stage,row.key,`${key}: ${high} is ${row.key}`);
      assert.equal(digest(a),digest(b),`${key}: ${row.key} does not drift inside its band`);
      // The project record the browser is already served resolves identically.
      const record=site.build(key,{progress_days:high*400,total_days:400,status:'building'});
      assert.equal(digest(record),digest(b),`${key}: progress_days/total_days agrees with the fraction`);
      if(row.index)assert.notEqual(digest(a),digest(site.build(key,row.from-1e-4)),`${key}: ${row.key} threshold changes the site`);
    }
    // Completion is the hand-off state and nothing short of full progress
    // reaches it: a finished project leaves the queue, it does not linger at 99%.
    assert.equal(site.build(key,0.999).stage,'enclosed');
    assert.equal(site.build(key,1).stage,'complete');
    assert.equal(site.build(key,4).stage,'complete');
    assert.equal(site.build(key,-3).stage,'site');
  }
});

test('geometry is deterministic across fresh loads and unaffected by anything outside its inputs',()=>{
  delete require.cache[require.resolve(file)];const fresh=require(file);
  assert.deepEqual(fresh.kinds(),site.kinds());
  for(const key of site.kinds()){
    for(const stage of STAGES){
      const options={lod:0,level:3,status:'slowed',variant:11},snapshot=JSON.stringify(options);
      const a=site.build(key,stage,options),b=fresh.build(key,stage,options),c=site.build(key,stage,{...options});
      assert.equal(JSON.stringify(options),snapshot,`${key}: build does not mutate its options`);
      for(const field of ['positions','normals','colors'])assert.deepEqual(a[field],b[field],`${key}/${stage}: deterministic ${field}`);
      assert.equal(digest(a),digest(c),`${key}/${stage}: repeat build is identical`);
      assert.deepEqual(a.bounds,b.bounds);assert.deepEqual(a.parts,b.parts);
    }
  }
});

test('later stages are never less built than earlier ones',()=>{
  for(const key of site.kinds()){
    for(const lod of [0,1]){
      for(const status of site.statuses){
        for(const level of [1,3,5]){
          const counts=STAGES.map(stage=>site.build(key,stage,{lod,status,level}).triangleCount);
          assert(monotone(counts),`${key}/lod${lod}/${status}/L${level}: ${counts.join(' ')} goes backwards`);
        }
      }
    }
  }
});

test('paused and blocked sites stop work honestly, and nothing else does',()=>{
  for(const key of site.kinds()){
    for(const stage of ['frame','enclosed']){
      const working=site.build(key,stage,{status:'building'}),slowed=site.build(key,stage,{status:'slowed'});
      const paused=site.build(key,stage,{status:'paused'}),blocked=site.build(key,stage,{status:'blocked'});
      assert.equal(digest(working),digest(slowed),`${key}: slowed work is still work`);
      assert.equal(working.overlay,null);assert.equal(working.active,true);
      for(const stopped of [paused,blocked]){
        validate(stopped,`${key}/${stage}/${stopped.status}`,0);
        assert.equal(stopped.active,false);
        assert.equal(stopped.overlay.status,stopped.status);
        assert(stopped.overlay.label.includes('stopped'),`${key}: overlay says work is stopped`);
        assert.notEqual(digest(stopped),digest(working),`${key}: stopped plant is posed differently`);
        assert(stopped.parts.some(part=>part.name==='site / work stopped notice'),`${key}: stop notice on site`);
        assert(stopped.description.includes('stopped'));
        // Not merely "the mesh differs" — a stop notice alone would satisfy
        // that while the crane carried on working behind it. The crane's own
        // triangles have to be posed differently, and the load has to be gone.
        const crane=name=>{
          const mesh=name==='working'?working:stopped;
          const part=mesh.parts.find(entry=>entry.name.includes('tower crane'));
          assert(part,`${key}: crane present at ${stage}`);
          return mesh.positions.slice(part.first*3,(part.first+part.count)*3);
        };
        const busy=crane('working'),parked=crane('stopped');
        assert.notDeepEqual(Array.from(busy),Array.from(parked),`${key}: parked crane is posed differently`);
        assert(parked.length<busy.length,`${key}: a parked crane carries no load`);
      }
      assert.notEqual(digest(paused),digest(blocked),`${key}: paused and blocked are different states`);
      // The status is a pose, not progress: it can never move the stage.
      for(const mesh of [working,paused,blocked])assert.equal(mesh.stage,stage);
    }
    // A record carrying its own status is read from the record.
    const fromRecord=site.build(key,{progress_days:200,total_days:400,status:'blocked'});
    assert.equal(fromRecord.status,'blocked');assert.equal(fromRecord.active,false);
  }
});

test('a level upgrade extends the compound instead of cloning the building',()=>{
  for(const key of site.kinds()){
    const built=[1,2,3,4,5].map(level=>site.build(key,'complete',{level}));
    built.forEach((mesh,i)=>{
      validate(mesh,`${key}/complete/L${i+1}`,0);
      assert.equal(mesh.level,i+1);
      const wings=mesh.parts.filter(part=>part.label.startsWith('delivered wing'));
      assert.equal(wings.length,i,`${key}: level ${i+1} stands on ${i} delivered wings`);
      if(i){
        assert(mesh.triangleCount>built[i-1].triangleCount,`${key}: level ${i+1} adds structure`);
        assert(mesh.bounds.max[0]>built[i-1].bounds.max[0],`${key}: level ${i+1} extends the compound`);
      }
    });
    // MAX_PROVINCE_LEVEL is five in production.rs and this clamps to it rather
    // than growing a sixth wing off the end of the platform.
    assert.equal(site.build(key,'complete',{level:9}).level,site.maxLevel);
    assert.equal(site.build(key,'complete',{level:0}).level,1);
  }
});

test('the arms plant is realised and the other twelve are declared placeholders',()=>{
  const plant=site.build('arms_plant','complete',{lod:0});
  validate(plant,'arms_plant/complete',0);
  assert.equal(plant.placeholder,false);
  for(const wanted of ['portal frames','wall cladding','roof sheeting','assembly doors','loading dock',
    'dock canopy','test and service hardstand','gatehouse','substation','permanent perimeter'])
    assert(plant.parts.some(part=>part.name.includes(wanted)),`arms plant has ${wanted}`);
  for(const [stage,wanted] of [['site','perimeter hoarding'],['site','accommodation'],['site','stacked materials'],
    ['foundation','excavation'],['foundation','reinforcement and formwork'],['foundation','spoil heaps'],
    ['frame','portal frames'],['frame','tower crane'],['frame','lighting masts'],
    ['enclosed','wall cladding'],['enclosed','scaffold'],['enclosed','tower crane']]){
    const mesh=site.build('arms_plant',stage,{lod:0});
    assert(mesh.parts.some(part=>part.name.includes(wanted)),`arms plant ${stage} has ${wanted}`);
  }
  // Site plant belongs to the job and leaves with it.
  const done=site.build('arms_plant','complete',{lod:0});
  for(const gone of ['tower crane','hoarding','spoil heaps','scaffold'])
    assert(!done.parts.some(part=>part.name.includes(gone)),`completed plant has no ${gone}`);
  for(const key of site.kinds()){
    const mesh=site.build(key,'complete');
    assert.equal(mesh.placeholder,key!=='arms_plant',`${key}: placeholder flag`);
    assert(mesh.description.includes('not a real'),`${key}: description refuses to claim a real facility`);
    assert(mesh.description.includes('no capability of its own'),`${key}: description grants nothing`);
    if(mesh.placeholder)assert(mesh.description.includes('placeholder'),`${key}: says it is a placeholder`);
  }
  // Retrofits are installed into a facility that already exists; they are not a
  // new standalone building (roadmap section E).
  for(const key of ['automation','efficiency']){
    assert.equal(site.meta(key).retrofit,true);
    for(const stage of STAGES)assert(site.build(key,stage).parts.some(part=>part.name.includes('existing host facility')),`${key}/${stage}: host stands throughout`);
  }
});

test('no wall clock, no frame counter and no second RNG can reach this geometry',()=>{
  // Roadmap section E: elapsed time never completes a building. The cheapest way
  // to keep that true is for the vocabulary not to exist in the file at all.
  for(const pattern of [/\bDate\b/,/\bnow\s*\(/,/performance/i,/hrtime/,/requestAnimationFrame/i,
    /\bsetTimeout\b/,/\bsetInterval\b/,/Math\s*\.\s*random/,/\brandom\b/i,/getTime/,/\bDate\b/])
    assert(!pattern.test(source),`site-mesh.js must not contain ${pattern}`);
  // And no DOM, so the geometry stays checkable outside a browser.
  for(const pattern of [/\bdocument\b/,/\bwindow\b/,/canvas/i,/WebGL/i,/require\s*\(/,/\bimport\b/])
    assert(!pattern.test(source),`site-mesh.js must not contain ${pattern}`);
});

test('the browser global exports the same contract with no dependencies',()=>{
  const context=vm.createContext({});
  vm.runInContext(source,context);
  assert.equal(typeof context.SiteMesh.build,'function');
  assert.equal(typeof context.SiteMesh.stages,'function');
  assert.equal(typeof context.SiteMesh.kinds,'function');
  assert.equal(context.SiteMesh.kinds().join(','),site.kinds().join(','));
  const sandboxed=context.SiteMesh.build('arms_plant','frame',{lod:0});
  assert.equal(sandboxed.triangleCount,site.build('arms_plant','frame',{lod:0}).triangleCount);
});

test('unknown and malformed input resolves to a stable, honest default',()=>{
  const fallback=site.build('civilian_industry','site');
  for(const input of ['not_a_kind','__proto__','',null,undefined,{}]){
    const mesh=site.build(input,'site');
    assert.equal(mesh.kind,'civilian_industry',`${String(input)}: falls back`);
    assert.equal(digest(mesh),digest(fallback));
  }
  assert.equal(site.build('arms_plant','not_a_stage').stage,'site');
  assert.equal(site.build('arms_plant',{status:'nonsense'}).status,'building');
  assert.equal(site.build('arms_plant',Number.NaN).stage,'site');
  for(const options of [undefined,null,'nonsense',{lod:'far'},{level:Number.NaN},{variant:Number.NaN}])
    validate(site.build('arms_plant','complete',options),`options ${JSON.stringify(options)}`,options&&options.lod==='far'?1:0);
});
