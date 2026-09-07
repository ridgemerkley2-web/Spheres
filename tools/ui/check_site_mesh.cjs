// Exercise the staged construction-site geometry the province and construction
// views draw. The contract this file defends is not "the meshes are pretty": it
// is that a stage is a pure function of the work the server has recorded, that
// nothing in the generator can advance a building by being looked at, that every
// one of the thirteen project kinds has something correct to show, and — added
// with the detail pass — that the SHADING is right: curved surfaces are smooth,
// flat surfaces are not, and no surface is lit from behind.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const cp=require('node:child_process');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const file=path.resolve(__dirname,'../../spheres-web/ui/site-mesh.js');
const site=require(file);
const source=fs.readFileSync(file,'utf8');
const STAGES=['site','foundation','frame','enclosed','complete'];

// BUDGET, and what moved. LOD1 is unchanged and is the hard one: sites are drawn
// as baked sprites on the globe overlay and there can be many, so 100..800
// stands. Measured worst case today is 710 (machinery_works/complete/L5) and the
// detail pass added nothing to the coarse path — every addition below is behind
// `d0.fine`, and that is deliberate, because the far mesh had under a hundred
// triangles of headroom.
//
// LOD0's ceiling was RAISED from 12,000 to 40,000 on the owner's brief ("way
// more detailed with realistic mesh"). The roadmap's own section 4 calls the
// first band "initial budgets to validate on the user's machine, not measured
// performance promises", and the old ceiling was what capped this art: the whole
// finished arms plant fitted in 6,356 triangles. The floor moved the other way,
// 2,000 -> 6,000, which is a TIGHTENING: nothing in this file can now fall back
// to the plain massing it used to be and still pass. Measured range today is
// 7,658 (research_center/site/L1) to 36,788 (arms_plant/complete/L5), so the
// ceiling keeps roughly nine per cent of headroom for the next pass.
const NEAR_MIN=6000,NEAR_MAX=40000,FAR_MIN=100,FAR_MAX=800;

// The smoothing pass folds a corner's neighbours in only when they are within
// the crease limit of its own face. That is 0.35 in site-mesh.js, and it makes
// the floor below a THEOREM rather than a measurement: every folded face is
// within the limit of the corner's own, so the normalised sum cannot fall under
// it. Measured minimum across every kind, stage, level and LOD is 0.6146, so
// this bar has room and still goes red the moment a vertex normal is allowed to
// point behind the triangle carrying it.
const CREASE=0.35;

const digest=m=>crypto.createHash('sha256')
  .update(Buffer.from(m.positions.buffer))
  .update(Buffer.from(m.normals.buffer))
  .update(Buffer.from(m.colors.buffer)).digest('hex');
const monotone=values=>values.every((value,i)=>i===0||value>=values[i-1]);

// The normal the WINDING gives a triangle, recomputed here rather than read from
// the buffer, because the buffer is the thing under test.
function faceNormal(mesh,tri){
  const p=mesh.positions,i=tri*9;
  const ux=p[i+3]-p[i],uy=p[i+4]-p[i+1],uz=p[i+5]-p[i+2];
  const vx=p[i+6]-p[i],vy=p[i+7]-p[i+1],vz=p[i+8]-p[i+2];
  const n=[uy*vz-uz*vy,uz*vx-ux*vz,ux*vy-uy*vx];
  const len=Math.hypot(...n);
  return [n[0]/len,n[1]/len,n[2]/len];
}
// A triangle is FLAT when its three vertices carry one normal between them, and
// SMOOTH when they do not. That is the only distinction the renderer can see and
// it is the one these checks are written against.
function isFlat(mesh,tri){
  const n=mesh.normals,i=tri*9;
  return n[i]===n[i+3]&&n[i+1]===n[i+4]&&n[i+2]===n[i+5]
    &&n[i]===n[i+6]&&n[i+1]===n[i+7]&&n[i+2]===n[i+8];
}
function partRange(mesh,fragment){
  const part=mesh.parts.find(entry=>entry.name.includes(fragment));
  return part?{first:part.first/3,count:part.count/3,name:part.name}:null;
}
function smoothShare(mesh,part){
  let smooth=0;
  for(let t=part.first;t<part.first+part.count;t+=1)if(!isFlat(mesh,t))smooth+=1;
  return smooth/part.count;
}
// The share of a part's faces whose normal is a DIAGONAL — two components of
// real size. A box has six axis-aligned faces and scores zero; a chamfered
// section scores on every bevel, a profiled sheet on every web. It is how a
// check can tell "this was modelled" from "this is a cuboid" without looking.
function bevelShare(mesh,part){
  let bevelled=0;
  for(let t=part.first;t<part.first+part.count;t+=1){
    const n=faceNormal(mesh,t).map(Math.abs).sort((a,b)=>b-a);
    if(n[1]>0.25)bevelled+=1;
  }
  return bevelled/part.count;
}

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
  // SHADING, and what this replaced. The old bar asked that ONE vertex of each
  // triangle carry exactly the triangle's own face normal, which is the same as
  // banning smooth shading outright — and it was the reason every one of the
  // 44,000 triangles on this model was flat. It is replaced by three bars that
  // are each stronger in the direction that matters:
  //   1. all THREE corners are checked, not the first one;
  //   2. a corner's normal may never point behind its own face, which is the
  //      property the old assertion was really defending;
  //   3. a FLAT triangle must still carry the exact face normal, so the
  //      smoothing pass cannot quietly perturb plate, panelling or concrete.
  let smooth=0;
  for(let t=0;t<mesh.triangleCount;t+=1){
    const i=t*9;
    const a=mesh.positions.subarray(i,i+3),b=mesh.positions.subarray(i+3,i+6),c=mesh.positions.subarray(i+6,i+9);
    const u=b.map((v,j)=>v-a[j]),v=c.map((value,j)=>value-a[j]);
    const n=[u[1]*v[2]-u[2]*v[1],u[2]*v[0]-u[0]*v[2],u[0]*v[1]-u[1]*v[0]],area=Math.hypot(...n);
    assert(area>1e-9,`${label}: nondegenerate triangle ${t}`);
    const flat=isFlat(mesh,t);
    if(!flat)smooth+=1;
    for(let corner=0;corner<3;corner+=1){
      const o=i+corner*3;
      const dot=n.reduce((sum,value,j)=>sum+(value/area)*mesh.normals[o+j],0);
      assert(dot>CREASE-1e-6,`${label}: normal points behind its face at ${t}.${corner} (${dot})`);
      if(flat)assert(dot>1-1e-5,`${label}: flat triangle ${t} does not carry its face normal (${dot})`);
    }
  }
  assert.equal(mesh.shading.smoothTriangles,smooth,`${label}: shading report disagrees with the buffer`);
  assert.equal(mesh.shading.flatTriangles,mesh.triangleCount-smooth,`${label}: flat count`);
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
  assert(mesh.parts.length>=(lod?4:10),`${label}: only ${mesh.parts.length} named parts`);
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

// A SEPARATE PROCESS, which the same-process reload above cannot stand in for.
// The smoothing pass gathers corners into a Map keyed by position and sums the
// face normals it finds there, so its output depends on insertion order and on
// float addition order — both fine, both deterministic, and both exactly the
// kind of thing that would stop being deterministic if somebody reached for a
// Set of objects or an unordered key. Iron rule 1 says determinism is sacred;
// this is the arm of it the detail pass could plausibly have broken.
test('two node processes build byte-identical geometry, normals included',()=>{
  const probe=`const site=require(${JSON.stringify(file)});const crypto=require('node:crypto');`
    +`const h=crypto.createHash('sha256');`
    +`for(const key of site.kinds())for(const stage of ${JSON.stringify(STAGES)})for(const lod of [0,1])for(const level of [1,4]){`
    +`const m=site.build(key,stage,{lod,level,variant:7,status:'slowed'});`
    +`h.update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.normals.buffer)).update(Buffer.from(m.colors.buffer));}`
    +`process.stdout.write(h.digest('hex'));`;
  const child=cp.execFileSync(process.execPath,['-e',probe],{encoding:'utf8'});
  const local=crypto.createHash('sha256');
  for(const key of site.kinds())for(const stage of STAGES)for(const lod of [0,1])for(const level of [1,4]){
    const m=site.build(key,stage,{lod,level,variant:7,status:'slowed'});
    local.update(Buffer.from(m.positions.buffer)).update(Buffer.from(m.normals.buffer)).update(Buffer.from(m.colors.buffer));
  }
  assert.equal(child,local.digest('hex'),'a second node process disagrees about the geometry');
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

// SMOOTH SHADING, which is the largest single thing this pass bought and costs
// no triangles at all. Before it, all 6,356 triangles of the finished arms plant
// carried their own face normal and every drum, tank, bollard, pipe, spoil heap
// and crane chord read as a faceted prism. The bars below are floors measured
// across every kind, stage and level and set with margin; they exist so that
// removing the smoothing pass, or dropping a primitive out of its smoothing
// group, goes red instead of just looking slightly worse.
test('curved surfaces are smooth-shaded and flat ones are not',()=>{
  // Measured minima across all 13 kinds x 5 stages x 5 levels at LOD0, with the
  // bar set roughly a fifth below each: crane 82.7%, reinforcement 66.9%,
  // bollards 79.5%, batching skid 64.4%, stacked materials 59.7%, flue stack
  // 50.0%, lighting masts 40.7%. These parts are mixtures — a crane has base
  // plates and a counterweight as well as chords — so the bar is not 100%.
  const curved=[['plant / tower crane','frame',0.70],['reinforcement and formwork','foundation',0.55],
    ['bollards and gate island','complete',0.65],['batching skid','foundation',0.52],
    ['site / stacked materials','site',0.48],['lighting masts','frame',0.32]];
  for(const [fragment,stage,floor] of curved){
    for(const key of site.kinds()){
      for(const level of [1,5]){
        const mesh=site.build(key,stage,{lod:0,level});
        const part=partRange(mesh,fragment);
        assert(part,`${key}/${stage}: ${fragment} present`);
        const share=smoothShare(mesh,part);
        assert(share>=floor,`${key}/${stage}/L${level}: ${part.name} only ${(share*100).toFixed(1)}% smooth, wanted ${floor*100}%`);
      }
    }
  }
  // And the other half of the claim: plate, sheeting, panelling and concrete
  // never enter a smoothing group, so their normals are exactly their faces'.
  // A crease limit that drifted wide enough to weld a wall panel to its
  // neighbour would show up here and nowhere else.
  const flat=['site / formation platform','envelope / wall cladding','envelope / roof sheeting and ridge',
    'structure / ground slab','yard / marked bays and kerbs','yard / permanent perimeter'];
  for(const key of site.kinds()){
    for(const stage of ['frame','complete']){
      const mesh=site.build(key,stage,{lod:0,level:2});
      for(const fragment of flat){
        const part=partRange(mesh,fragment);
        if(!part)continue;
        assert.equal(smoothShare(mesh,part),0,`${key}/${stage}: ${part.name} should be flat throughout`);
      }
    }
  }
  // Measured floor is 15.66% (warehouse/complete/L5). A site is mostly flat
  // plate and concrete and should be; this bar is only here so the pass cannot
  // be switched off wholesale without a red test.
  for(const key of site.kinds()){
    for(const stage of STAGES){
      const mesh=site.build(key,stage,{lod:0,level:3});
      const share=mesh.shading.smoothTriangles/mesh.triangleCount;
      assert(share>0.12,`${key}/${stage}: only ${(share*100).toFixed(1)}% of triangles are smooth-shaded`);
    }
  }
});

// WINDING, and why this is now asserted. The detail pass found four surfaces
// wound inside out — the formation platform every site stands on, the floor of
// every excavation, the base of every spoil heap, and BOTH SLOPES OF EVERY ROOF,
// where the weather sheet faced the floor and the dark inner lining faced the
// sky. None of them could be caught by the old bar, which only compared a vertex
// normal against its own triangle and is satisfied by any consistent winding.
// The rule below is the smallest one that catches all four: on a surface that
// exists to be walked on or rained on, the HIGHEST near-horizontal facet must
// face up. Checked against the pre-repair geometry it goes red on all four.
test('surfaces you stand on and surfaces it rains on face the sky',()=>{
  const tops=['formation platform','ground slab','structure / slab','roof sheeting','envelope / massing',
    'apron','hardstand','marked bays','dig and spoil','access road','excavation'];
  let checked=0;
  for(const key of site.kinds()){
    for(const stage of STAGES){
      for(const lod of [0,1]){
        for(const level of [1,4]){
          const mesh=site.build(key,stage,{lod,level});
          for(const fragment of tops){
            for(const part of mesh.parts.filter(entry=>entry.name.includes(fragment))){
              let best=null,bestY=-Infinity;
              for(let t=part.first/3;t<part.first/3+part.count/3;t+=1){
                const n=faceNormal(mesh,t);
                if(Math.abs(n[1])<0.965)continue;          // only the near-level facets
                const i=t*9,p=mesh.positions;
                const y=(p[i+1]+p[i+4]+p[i+7])/3;
                if(y>bestY){bestY=y;best=n;}
              }
              if(!best)continue;
              checked+=1;
              assert(best[1]>0,`${key}/${stage}/lod${lod}/L${level}: ${part.name} is lit from underneath`);
            }
          }
        }
      }
    }
  }
  assert(checked>800,`only ${checked} top surfaces were found to check`);
  // Heaps get their own line, because the rule above cannot see them: a tall
  // spoil pile's sides are 60 degrees off level and the near-level filter drops
  // them. Every one of them WAS inside out — `mound` listed its side triangles
  // the wrong way round and the whole cone faced inward, base included, which
  // also let the smoothing pass weld the base to the sides. A cone has `seg`
  // side facets looking up and out and `seg - 2` base facets looking down, so
  // more of a heap faces the sky than faces the ground, and it stops being true
  // the moment the winding flips.
  for(const key of site.kinds()){
    for(const [stage,fragment] of [['foundation','spoil heaps'],['site','topsoil bund'],['complete','apron and verge']]){
      const mesh=site.build(key,stage,{lod:0,level:2});
      const part=partRange(mesh,fragment);
      assert(part,`${key}/${stage}: ${fragment} present`);
      let up=0,down=0;
      for(let t=part.first;t<part.first+part.count;t+=1){
        const n=faceNormal(mesh,t);
        if(n[1]>0.05)up+=1;else if(n[1]<-0.05)down+=1;
      }
      assert(up>down,`${key}/${stage}: ${part.name} has ${up} facets facing the sky and ${down} facing the ground — inside out?`);
    }
  }
});

// A CHAMFER ON EVERY HARD EDGE was the second half of the brief, and it is what
// separates a machined object from a folded piece of card in a renderer with no
// textures. A box has six axis-aligned faces; a bevel, a folded flashing and a
// profiled sheet all have faces that point diagonally. Measured minima are 24.0%
// on portal frames, 29.7% on wall cladding and 12.6% on the permanent perimeter.
test('structure and envelope are modelled rather than boxed',()=>{
  const bevelled=[['portal frames','frame',0.15],['envelope / wall cladding','complete',0.18],
    ['yard / permanent perimeter','complete',0.08]];
  // Level one, so `partRange` lands on the lead block. A delivered wing is
  // drawn a step plainer on purpose — wider rib pitch, same flashings — and
  // holding it to the lead block's bar would be asking for detail the file
  // deliberately does not spend there.
  for(const [fragment,stage,floor] of bevelled){
    for(const key of site.kinds()){
      const mesh=site.build(key,stage,{lod:0,level:1});
      const part=partRange(mesh,fragment);
      assert(part,`${key}/${stage}: ${fragment} present`);
      const share=bevelShare(mesh,part);
      assert(share>=floor,`${key}/${stage}: ${part.name} is ${(share*100).toFixed(1)}% bevelled, wanted ${floor*100}% — is it back to plain boxes?`);
    }
  }
  // Floors on the pieces the brief named, measured on arms_plant at LOD0 level
  // one and set about a fifth under: crane 3,294, reinforcement 2,904, portal
  // frames 3,612, perimeter 3,792, scaffold 1,048, roof plant 984, louvres
  // 1,024. They are here so a future edit cannot quietly return any of these to
  // the boxes they were.
  const floors=[['plant / tower crane','frame',2400],['reinforcement and formwork','foundation',2200],
    ['portal frames','frame',2800],['yard / permanent perimeter','complete',2900],
    ['gable scaffold','enclosed',800],['roof plant and access walkway','complete',700],
    ['wall louvres','complete',760],['accommodation and welfare','frame',900]];
  for(const [fragment,stage,floor] of floors){
    const mesh=site.build('arms_plant',stage,{lod:0,level:1});
    const part=partRange(mesh,fragment);
    assert(part,`arms_plant/${stage}: ${fragment} present`);
    assert(part.count>=floor,`arms_plant/${stage}: ${part.name} is only ${part.count} triangles, wanted ${floor}`);
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
    'dock canopy','test and service hardstand','gatehouse','substation','permanent perimeter',
    'roof plant and access walkway','wall louvres','roof lights','weighbridge'])
    assert(plant.parts.some(part=>part.name.includes(wanted)),`arms plant has ${wanted}`);
  for(const [stage,wanted] of [['site','perimeter hoarding'],['site','accommodation'],['site','stacked materials'],
    ['site','setting out'],
    ['foundation','excavation'],['foundation','reinforcement and formwork'],['foundation','spoil heaps'],
    ['foundation','batching skid'],
    ['frame','portal frames'],['frame','tower crane'],['frame','lighting masts'],['frame','ground slab'],
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
  assert.equal(sandboxed.shading.smoothTriangles,site.build('arms_plant','frame',{lod:0}).shading.smoothTriangles);
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
