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
// stands. Measured worst case today is 756 (infrastructure/complete/L5), and
// the twelve compositions PAID for their map silhouettes rather than adding to
// the far mesh: the coarse perimeter went from a line of posts to a line (168
// triangles), the coarse lighting mast lost its lamp housings, the coarse
// scaffold went to one lift on one bay, and mesh guarding, pallet racking and
// handrails now draw nothing at map range at all. Every one of those is a thing
// that is under a pixel there. The floor moved 100 -> 164 measured
// (starter_industry/site/L1).
//
// LOD0's ceiling has been RAISED TWICE, both times on the owner's brief and
// both times recorded rather than quietly widened. 12,000 -> 40,000 was the
// first detail pass ("way more detailed with realistic mesh"); 40,000 -> 54,000
// is the second ("way more detailed" again, on plant and process: pipework
// runs, ducting, stacks, gantries, roof plant, external stairs and walkways,
// groundworks, and yard life). The roadmap's own section 4 calls the first band
// "initial budgets to validate on the user's machine, not measured performance
// promises", and it is the ceiling and not the art that has been the binding
// constraint here from the start. THIS IS A DECISION, NOT A MEASUREMENT, and
// docs/art/BUDGET_DECISION.md is where it is argued; the numbers below are what
// this file actually costs so the record can be kept honest.
//
// THE FLOOR MOVED FURTHER THAN THE CEILING, PROPORTIONALLY, and that is the
// half of this that is a TIGHTENING: 2,000 -> 6,000 -> 11,000. Nothing in this
// file can now fall back to the massing it used to be and still pass, and the
// second pass cannot be deleted from the early stages either — the cheapest
// mesh in the whole table is a level-one site establishment and it is 11,312.
//
// Measured range today is 11,312 (starter_industry/site/L1) to 50,102
// (generation/complete/L5): a machine hall, a conversion block, heat rejection,
// consumables handling, a tank farm, a takeoff gantry, a flue stack, a yard
// pipe bridge, roof ducting and vent stacks, an external stair, a substation
// and a marked-out yard, on the biggest pad this file draws. That leaves seven
// per cent of headroom under the ceiling and three per cent over the floor.
//
// AND THE MAP MESH DID NOT MOVE AT ALL. Every function the second pass added
// returns on its first line unless `d0.fine`, the far range is the 164..756 it
// was, and `the map mesh pays for none of the close-range detail` below pins
// the far triangle total to the exact number so it cannot drift by accident.
const NEAR_MIN=11000,NEAR_MAX=54000,FAR_MIN=100,FAR_MAX=800;

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
    ['site / stacked materials','site',0.48],['lighting masts','frame',0.32],
    // The second detail pass, on the same terms. Measured minima across all 13
    // kinds and levels: drainage 60.4%, earthmoving plant 43.8%, external stair
    // 43.6%, external pipework 42.6%, extract ducting 29.2% (at hand-over — at
    // the enclosing stage it is honestly all folded plate and no round section
    // at all, which is what a bare duct route IS), bunded store 37.6%,
    // substation 20.6%, pipe bridge 22.3%. Bars set roughly a fifth under.
    ['drainage runs and manholes','foundation',0.45],['plant / earthmoving plant','site',0.34],
    ['external stair and roof access','frame',0.34],['external pipework and cable ladder','enclosed',0.33],
    ['extract ducting and vent stacks','complete',0.22],['bunded store and gas cage','complete',0.29],
    ['packaged substation and cable route','frame',0.16],['yard pipe bridge','complete',0.17]];
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
    'structure / ground slab','yard / marked bays and kerbs','yard / permanent perimeter',
    // Paint is plate. The walkway, the hatching, the arrows, the roundel and
    // the bay-number boards are the one place in this file where the ONLY thing
    // being drawn is a flat surface with a different tone, and if the smoothing
    // pass ever reached them they would stop reading as markings at all.
    'yard / hardstanding markings and walkway'];
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
    'apron','hardstand','marked bays','dig and spoil','access road','excavation',
    // Added with the second pass. The trench floor and the yard markings are
    // both surfaces made of `deck` and `plate` calls, and both are exactly the
    // kind of hand-listed horizontal quad that came out inside out last time.
    'drainage runs','hardstanding markings'];
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

// WHICH KINDS HAVE HAD THEIR ART PASS, as a list rather than as a rule. It
// used to be `key !== 'arms_plant'`, which was true exactly while twelve of the
// thirteen were plain massing and stopped being true the moment any one of them
// was finished. The list below is the whole claim this file makes about that,
// and the count is asserted rather than described: a kind cannot be quietly
// marked done, a kind that is NOT done cannot be quietly left out of the
// number, and if any are left the failure message names them.
const PLACEHOLDER_KINDS=[];

test('the placeholder count is asserted, not described',()=>{
  const declared=site.kinds().filter(key=>site.meta(key).placeholder).sort();
  assert.deepEqual(declared,PLACEHOLDER_KINDS.slice().sort(),
    `placeholder set disagrees with PLACEHOLDER_KINDS: ${declared.length} of ${site.kinds().length} kinds are flagged (${declared.join(', ')||'none'})`);
  assert.equal(declared.length,PLACEHOLDER_KINDS.length,
    `${declared.length} of ${site.kinds().length} kinds are still placeholders`);
  for(const key of site.kinds()){
    const placeholder=PLACEHOLDER_KINDS.includes(key);
    // meta() and build() have to agree, or a card and a picker would disagree.
    assert.equal(site.meta(key).placeholder,placeholder,`${key}: meta placeholder flag`);
    for(const stage of STAGES){
      const mesh=site.build(key,stage);
      assert.equal(mesh.placeholder,placeholder,`${key}/${stage}: build placeholder flag`);
      // A placeholder must SAY so on its own card, and a finished kind must not
      // be allowed to keep the excuse.
      assert.equal(mesh.description.includes('placeholder'),placeholder,`${key}/${stage}: description and flag disagree`);
    }
  }
});

test('the arms plant is realised and every kind declares what it is not',()=>{
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
    assert(mesh.description.includes('not a real'),`${key}: description refuses to claim a real facility`);
    assert(mesh.description.includes('no capability of its own'),`${key}: description grants nothing`);
    assert(mesh.description.includes(site.meta(key).blurb),`${key}: the card says what the thing is`);
  }
  // `generation` is the one kind the roadmap explicitly fences off: technology
  // variants are a later pass, so this one must not name a technology and then
  // stand behind it. Everything a player can read - the card, the blurb and
  // every part name the picker shows - is checked, at every stage and every
  // level, because that is where such a claim would actually be made.
  const NAMED_TECH=/reactor|nuclear|coal|lignite|gas turbine|ccgt|combined cycle|photovoltaic|solar farm|wind turbine|hydro|geothermal|biomass/i;
  for(const stage of STAGES){
    for(const level of [1,5]){
      for(const lod of [0,1]){
        const mesh=site.build('generation',stage,{lod,level});
        assert(!NAMED_TECH.test(mesh.description),`generation/${stage}: the card names a technology`);
        for(const part of mesh.parts)assert(!NAMED_TECH.test(part.name),`generation/${stage}: part "${part.name}" names a technology`);
      }
    }
    assert(site.build('generation',stage).description.includes('generic'),`generation/${stage}: says it is generic`);
  }
  assert(!NAMED_TECH.test(site.meta('generation').blurb),'the generation blurb names a technology');
});

// WHAT EACH KIND IS, taken from roadmap section E and asserted as geometry.
// Triangle counts cannot express this and neither can a screenshot: the named
// parts can, because a part is the unit the picker and the card already speak
// in. Each row is [stage, fragment] pairs that MUST be present, and they are
// the pieces that make that kind that kind rather than a shed with a different
// paint code.
const COMPOSITION={
  infrastructure:[['site','carriageway formation'],['foundation','abutments and pier'],
    ['frame','deck beams'],['enclosed','deck slab and approaches'],
    ['complete','parapets, barrier and sign gantry'],['foundation','utility duct trench'],
    ['complete','duct route reinstated'],['complete','precast and culvert stockpile']],
  civilian_industry:[['frame','portal frames and haunches (module 2)'],
    ['frame','portal frames and haunches'],['frame','module bases and movement joints'],
    ['enclosed','high-level module links'],['complete','service block'],
    ['complete','external plant compound']],
  power_grid:[['site','earth grid and stone surfacing'],['foundation','equipment plinths and cable trench'],
    ['frame','busbar gantries'],['enclosed','circuit breakers and disconnectors'],
    ['enclosed','transformer bays'],['complete','line termination tower']],
  research_center:[['frame','framed floors and columns'],['frame','glazed link and entrance'],
    ['enclosed','spandrel panels and strip glazing'],['enclosed','roof deck and parapet'],
    ['enclosed','fume extract and roof plant'],['complete','courtyard and planting']],
  arms_plant:[['frame','portal frames'],['enclosed','loading dock and levellers'],
    ['complete','test and service hardstand'],['complete','gatehouse']],
  machinery_works:[['frame','overhead travelling crane'],['frame','portal frames and haunches (fitting bay)'],
    ['enclosed','stillages and swarf skips'],['enclosed','compressor house'],
    ['complete','outdoor crane rail']],
  generation:[['frame','heat rejection bank'],['frame','consumables handling gallery'],
    ['frame','portal frames and haunches (conversion block)'],['enclosed','tank farm'],
    ['complete','takeoff gantry'],['complete','flue stack']],
  processing_plant:[['foundation','vessel bases and bund slab'],['frame','process tower and access decks'],
    ['enclosed','process vessels'],['enclosed','horizontal drum and pumps'],
    ['frame','pipe rack'],['complete','road tanker loading bay']],
  freight_terminal:[['site','rail formation and sidings'],['foundation','rail formation and sidings'],
    ['frame','transfer gantry'],['enclosed','container stacks'],
    ['enclosed','transfer canopy and doors'],['complete','trailer park and running lanes']],
  warehouse:[['frame','sprinkler tank and pump house'],['enclosed','pallet racking'],
    ['enclosed','dock elevation and canopy'],['complete','trailer standing']],
  automation:[['site','floor broken out for machine bases'],['foundation','machine bases and isolation pads'],
    ['frame','machine cell line'],['frame','guarding and interlocks'],
    ['enclosed','control cabinets and cable tray'],['enclosed','compressed air and chiller skid']],
  efficiency:[['foundation','skid plinth and trestle bases'],['frame','heat recovery skid and economiser'],
    ['frame','insulated pipe run'],['enclosed','wall cladding (over-clad to host)'],
    ['enclosed','roof insulation boards'],['enclosed','thermal buffer vessel'],
    ['enclosed','controls kiosk and panel line']],
  starter_industry:[['frame','blockwork store'],['enclosed','office pod and canopy'],
    ['enclosed','open stock rack and compressor'],['complete','open lean-to']],
};

test('every kind is composed of what section E says it is',()=>{
  assert.deepEqual(Object.keys(COMPOSITION).slice().sort(),site.kinds().slice().sort(),
    'COMPOSITION covers exactly the thirteen project kinds');
  for(const key of site.kinds()){
    for(const [stage,fragment] of COMPOSITION[key]){
      const mesh=site.build(key,stage,{lod:0});
      assert(mesh.parts.some(part=>part.name.includes(fragment)),
        `${key}/${stage}: composition is missing "${fragment}"`);
      // and it has to still be there once the site is handed over, unless it is
      // a stage-only piece of the works.
      if(stage!=='complete'&&!/broken out|bases and|plinth and|formation and sidings|duct trench/.test(fragment)){
        const done=site.build(key,'complete',{lod:0});
        assert(done.parts.some(part=>part.name.includes(fragment)),
          `${key}: "${fragment}" went missing between ${stage} and hand-over`);
      }
    }
  }
  // NO TWO KINDS ARE THE SAME COMPOSITION. Every kind must own at least three
  // part names that no other kind draws at any stage — which a colour change,
  // a different pad size or a different prop list cannot satisfy.
  const owned=new Map();
  for(const key of site.kinds()){
    const names=new Set();
    for(const stage of STAGES)for(const part of site.build(key,stage,{lod:0}).parts)names.add(part.name);
    owned.set(key,names);
  }
  for(const key of site.kinds()){
    const mine=[...owned.get(key)].filter(name=>{
      for(const other of site.kinds())if(other!==key&&owned.get(other).has(name))return false;
      return true;
    });
    assert(mine.length>=3,`${key}: only ${mine.length} part names are its own (${mine.join(', ')}) — is it a shed with a different colour?`);
  }
});

test('the five stages of every kind are structurally different, not just heavier',()=>{
  for(const key of site.kinds()){
    for(const lod of [0,1]){
      const sets=STAGES.map(stage=>new Set(site.build(key,stage,{lod,level:2}).parts.map(part=>part.name)));
      for(let i=0;i+1<sets.length;i+=1){
        // The SET OF NAMED PARTS has to change, not the triangle count. A stage
        // that only grew is a stage that drew the same objects bigger, and the
        // five states this file promises are five different states of a site.
        const gained=[...sets[i+1]].filter(name=>!sets[i].has(name));
        const lost=[...sets[i]].filter(name=>!sets[i+1].has(name));
        // Two names at inspection range, one at map range. The map mesh has
        // fewer objects to change because most of a composition draws nothing
        // there, but a stage that changes NO named part is the previous stage
        // drawn heavier, which is the failure this is for.
        const floor=lod?1:2;
        assert(gained.length+lost.length>=floor,
          `${key}/lod${lod}: ${STAGES[i]} -> ${STAGES[i+1]} changes ${gained.length+lost.length} part names — the two stages are the same scene`);
      }
      for(let i=0;i<sets.length;i+=1){
        for(let j=i+1;j<sets.length;j+=1){
          const same=[...sets[i]].length===[...sets[j]].length&&[...sets[i]].every(name=>sets[j].has(name));
          assert(!same,`${key}/lod${lod}: ${STAGES[i]} and ${STAGES[j]} draw the same set of parts`);
        }
      }
    }
    // AND THE KIND'S OWN WORK HAS TO MOVE, not just the shared stage kit. The
    // hoarding coming down and the fence going up would satisfy the rule above
    // on its own, which would let a composition ignore the stage entirely and
    // still pass. `own` is the parts no other kind draws, so this is the
    // permanent work of THIS facility and nothing else.
    const shared=new Set();
    for(const other of site.kinds()){
      if(other===key)continue;
      for(const stage of STAGES)for(const part of site.build(other,stage,{lod:0,level:2}).parts)shared.add(part.name);
    }
    const own=STAGES.map(stage=>site.build(key,stage,{lod:0,level:2}).parts
      .map(part=>part.name).filter(name=>!shared.has(name)).sort().join('|'));
    // FROM THE FRAME STAGE ON. Site establishment and foundations are the two
    // stages roadmap section E asks to be built from ONE SHARED KIT - hoarding,
    // huts, materials, dig, footings - and on most of these kinds what differs
    // there is honestly the footprint and the pad and not a different set of
    // objects. From the frame stage the permanent work of the facility is
    // standing, and from there each kind's own work has to move at every step
    // or the composition is not reading the stage at all.
    for(let i=2;i+1<own.length;i+=1){
      assert(own[i]!==own[i+1],
        `${key}: its own work is identical at ${STAGES[i]} and ${STAGES[i+1]} — the composition is not reading the stage`);
    }
    // And by the commissioning stage every kind must have work of its own
    // standing. Not at the frame stage: a steel portal frame is the same object
    // on an assembly hall as on a warehouse, and pretending otherwise would be
    // asking the art to lie about how buildings are put up.
    assert(own[3].length>0,`${key}: nothing of its own is standing by the commissioning stage`);
  }
});

// The bounding box of one named part, in model space.
function partBox(mesh,fragment){
  const part=mesh.parts.find(entry=>entry.name.includes(fragment));
  if(!part)return null;
  const box={min:[Infinity,Infinity,Infinity],max:[-Infinity,-Infinity,-Infinity]};
  for(let i=part.first*3;i<(part.first+part.count)*3;i+=3){
    for(let a=0;a<3;a+=1){
      const v=mesh.positions[i+a];
      if(v<box.min[a])box.min[a]=v;
      if(v>box.max[a])box.max[a]=v;
    }
  }
  return box;
}

test('the two retrofits are installed in a host that already stands',()=>{
  // Roadmap section E, twice over: "Automation and efficiency are retrofit
  // kits, not fictional new standalone buildings", and "do not pretend
  // invisible software needs a giant external box". Three things follow, and
  // all three are geometry rather than a flag.
  for(const key of ['automation','efficiency']){
    assert.equal(site.meta(key).retrofit,true,`${key}: declared a retrofit`);
    let first=null;
    for(const stage of STAGES){
      const mesh=site.build(key,stage,{lod:0});
      const host=partBox(mesh,'existing host facility');
      assert(host,`${key}/${stage}: the host facility stands throughout`);
      // 1. THE HOST IS NOT BUILT BY THIS PROJECT. It is the same object at every
      //    stage, VERTEX FOR VERTEX: it was there before the job started. A
      //    bounding box is not enough — a wall that grows under a roof taller
      //    than it leaves the box alone — so this is the actual geometry.
      const part=mesh.parts.find(entry=>entry.name.includes('existing host facility'));
      const bytes=crypto.createHash('sha256')
        .update(Buffer.from(mesh.positions.slice(part.first*3,(part.first+part.count)*3).buffer)).digest('hex');
      if(first)assert.equal(bytes,first,`${key}/${stage}: the host is not the same geometry it was at the previous stage — it is being built, not retrofitted`);
      else first=bytes;
      // 2. THE NEW WORK IS ATTACHED TO IT. The retrofit's own envelope has to
      //    touch the host, not stand in the same field as it.
      const own=partBox(mesh,'wall cladding')||partBox(mesh,'portal frames');
      if(own){
        const gapX=Math.max(own.min[0]-host.max[0],host.min[0]-own.max[0]);
        const gapZ=Math.max(own.min[2]-host.max[2],host.min[2]-own.max[2]);
        assert(gapX<=0.6&&gapZ<=0.6,
          `${key}/${stage}: the new structure stands ${Math.max(gapX,gapZ).toFixed(2)} m clear of the host — that is a new building, not a retrofit`);
      }
      // 3. IT IS SMALLER THAN THE HOST. A retrofit that out-masses the facility
      //    it is fitted to is a facility.
      const hostArea=(host.max[0]-host.min[0])*(host.max[2]-host.min[2]);
      for(const part of mesh.parts){
        if(!/^structure \//.test(part.name))continue;
        if(part.name.includes('existing host'))continue;
        if(part.label.startsWith('delivered wing'))continue;
        const box=partBox(mesh,part.name);
        const area=(box.max[0]-box.min[0])*(box.max[2]-box.min[2]);
        assert(area<hostArea,`${key}/${stage}: "${part.name}" covers ${area.toFixed(0)} m2 against the host's ${hostArea.toFixed(0)} m2`);
        assert(box.max[1]<=host.max[1]+0.5,`${key}/${stage}: "${part.name}" stands taller than the host it is fitted to`);
      }
    }
    // And the controls are a kiosk and a panel line, not a building.
    const done=site.build(key,'complete',{lod:0});
    const controls=done.parts.find(part=>/cabinet|kiosk|panel line/.test(part.name));
    assert(controls,`${key}: the control content is drawn`);
    const box=partBox(done,controls.name);
    assert((box.max[0]-box.min[0])*(box.max[2]-box.min[2])<40,
      `${key}: "${controls.name}" is a building, and invisible software does not need one`);
  }
});

test('delivered wings stand on the fill they were built on',()=>{
  // The wings used to march off the lead block's flank at thirteen metres a
  // level, which put wing four sixty metres out and hanging over nothing. The
  // old check could not see it: the wings were themselves what made the
  // bounding box grow, so `bounds.max[0]` still went up every level.
  for(const key of site.kinds()){
    for(const level of [2,3,4,5]){
      for(const lod of [0,1]){
        const mesh=site.build(key,'complete',{level,lod});
        const halfW=mesh.compound.width/2,halfD=mesh.compound.depth/2;
        const wings=mesh.parts.filter(part=>/delivered wing|\(wing \d\)/.test(part.name));
        assert(wings.length>=level-1,`${key}/L${level}/lod${lod}: ${wings.length} wing parts`);
        for(const part of wings){
          const box=partBox(mesh,part.name);
          assert(box.min[0]>=-halfW-0.05&&box.max[0]<=halfW+0.05,
            `${key}/L${level}/lod${lod}: ${part.name} runs off the platform in x (${box.min[0].toFixed(1)}..${box.max[0].toFixed(1)} against +-${halfW})`);
          assert(box.min[2]>=-halfD-0.05&&box.max[2]<=halfD+0.05,
            `${key}/L${level}/lod${lod}: ${part.name} runs off the platform in z (${box.min[2].toFixed(1)}..${box.max[2].toFixed(1)} against +-${halfD})`);
        }
      }
    }
  }
});


// THE SECOND DETAIL PASS, asserted as PARTS AND STAGES rather than as a triangle
// count. A budget range says the file got bigger; it cannot say that what was
// added is a drainage trench at the stage a drainage trench is dug, and it is
// exactly the claim this pass makes that has to be checkable: the brief asked
// for detail that reads on a 1124x102 strip AND for the five stages to stay
// clearly distinguishable, and the second of those is the one a triangle count
// will happily let you break.
//
// Each row is [part name, the stages it may stand at, a triangle floor]. THE
// STAGE LIST IS EXHAUSTIVE IN BOTH DIRECTIONS: the part must be there at every
// stage in it, on every kind, and must be ABSENT at every stage that is not —
// so an early site cannot quietly grow finished pipework and a handed-over
// facility cannot keep its skips. The floors are measured minima across all 13
// kinds at levels one and five, set about a fifth under, and they are here so a
// future edit cannot return any of these to the boxes they could have been.
const DETAIL=[
  ['earthworks / drainage runs and manholes',['site','foundation'],1000],
  ['plant / earthmoving plant',['site','foundation'],1600],
  ['site / skips and waste segregation',['site','foundation','frame','enclosed'],640],
  ['services / external stair and roof access',['frame','enclosed','complete'],1450],
  ['services / packaged substation and cable route',['frame','enclosed','complete'],580],
  ['services / external pipework and cable ladder',['enclosed','complete'],1300],
  ['services / extract ducting and vent stacks',['enclosed','complete'],400],
  ['services / yard pipe bridge',['enclosed','complete'],1400],
  ['yard / hardstanding markings and walkway',['complete'],260],
  ['yard / bunded store and gas cage',['complete'],1100],
];

test('the second detail pass stands on every kind, at the stages it belongs to and at no others',()=>{
  for(const [name,stages,floor] of DETAIL){
    assert(stages.every(stage=>STAGES.includes(stage)),`${name}: names a stage that does not exist`);
    for(const key of site.kinds()){
      for(const stage of STAGES){
        for(const level of [1,5]){
          const mesh=site.build(key,stage,{lod:0,level});
          const part=mesh.parts.find(entry=>entry.name===name);
          if(!stages.includes(stage)){
            assert(!part,`${key}/${stage}/L${level}: "${name}" is standing at a stage it does not belong to`);
            continue;
          }
          assert(part,`${key}/${stage}/L${level}: "${name}" is missing`);
          assert(part.count%3===0);
          assert(part.count/3>=floor,
            `${key}/${stage}/L${level}: "${name}" is only ${part.count/3} triangles against a floor of ${floor} — has it gone back to boxes?`);
        }
      }
    }
  }
  // And the pass is a pass, not a rename: ten named objects that were not there
  // before have to be there now, on all thirteen kinds, across the five stages.
  for(const key of site.kinds()){
    const seen=new Set();
    for(const stage of STAGES)for(const part of site.build(key,stage,{lod:0}).parts)seen.add(part.name);
    for(const [name] of DETAIL)assert(seen.has(name),`${key}: never draws "${name}" at any stage`);
  }
});

test('the map mesh pays for none of the close-range detail',()=>{
  // The far LOD is what the globe overlay pays for and it is the tight budget:
  // the worst kind sits 44 triangles under a hard 800. Every function the
  // second pass added returns on its first line unless `d0.fine`, and this is
  // the bar that keeps it true — by name, so a coarse path cannot be added
  // later without saying so, and by total, so it cannot drift a triangle at a
  // time either.
  let far=0;
  for(const key of site.kinds()){
    for(const stage of STAGES){
      for(let level=1;level<=site.maxLevel;level+=1){
        const mesh=site.build(key,stage,{lod:1,level});
        far+=mesh.triangleCount;
        for(const [name] of DETAIL){
          assert(!mesh.parts.some(part=>part.name===name),
            `${key}/${stage}/L${level}: the map mesh is drawing "${name}", which is close-range detail`);
        }
      }
    }
  }
  assert.equal(far,142776,
    `the far mesh now costs ${far} triangles over the 325 kind/stage/level combinations instead of 142776 — the map budget is the one that is actually tight, so a change here has to be a decision and not a side effect`);
});

test('the formation platform is the thing that touches the ground',()=>{
  // `finish` re-seats the lowest vertex of the model on Y=0, which means
  // `bounds.min[1] === 0` is true whatever happens and CANNOT catch a bucket, a
  // track shoe or a trench floor dropped below the platform. What such a thing
  // does instead is silently LIFT the whole site by however far it went under —
  // the platform, the building, the fence and every object on the pad — and
  // nothing else in this file would notice. The site stands on its platform, so
  // the platform is what has to be on the ground.
  for(const key of site.kinds()){
    for(const stage of STAGES){
      for(const lod of [0,1]){
        for(const level of [1,5]){
          for(const status of site.statuses){
            const mesh=site.build(key,stage,{lod,level,status});
            const part=mesh.parts.find(entry=>entry.name.includes('formation platform'));
            assert(part,`${key}/${stage}/lod${lod}: formation platform present`);
            let low=Infinity;
            for(let i=part.first*3;i<(part.first+part.count)*3;i+=3){
              if(mesh.positions[i+1]<low)low=mesh.positions[i+1];
            }
            assert(low<1e-5,
              `${key}/${stage}/lod${lod}/L${level}/${status}: the platform is floating ${low.toFixed(3)} m off the ground — something under it is holding the model up`);
          }
        }
      }
    }
  }
});

test('earthmoving plant is parked when the work stops, not deleted',()=>{
  // The same contract the tower crane is already held to, one stage earlier.
  // The crane covers the frame and enclosing stages; the two stages before them
  // had no plant at all until this pass and so had nothing that could read as
  // stopped except a board at the gate. A machine holding its boom over the dig
  // on a paused site is the exact lie this file exists to refuse.
  for(const key of site.kinds()){
    for(const stage of ['site','foundation']){
      const working=site.build(key,stage,{status:'building'});
      const stopped=site.build(key,stage,{status:'paused'});
      const find=mesh=>mesh.parts.find(part=>part.name==='plant / earthmoving plant');
      const a=find(working),b=find(stopped);
      assert(a&&b,`${key}/${stage}: earthmoving plant at both statuses`);
      // A stop is a POSE. Same triangles either way, so it can move neither a
      // budget nor a monotonicity run.
      assert.equal(a.count,b.count,`${key}/${stage}: a parked machine costs a different number of triangles`);
      const span=(mesh,part)=>Array.from(mesh.positions.slice(part.first*3,(part.first+part.count)*3));
      assert.notDeepEqual(span(working,a),span(stopped,b),
        `${key}/${stage}: the machine stands in exactly the same pose whether or not anybody is paying for it`);
      // Not merely "different": the boom is DOWN. The highest point of the
      // plant has to drop, which a slew angle or a moved track cannot fake.
      const top=(mesh,part)=>{
        let y=-Infinity;
        for(let i=part.first*3;i<(part.first+part.count)*3;i+=3)if(mesh.positions[i+1]>y)y=mesh.positions[i+1];
        return y;
      };
      assert(top(stopped,b)<top(working,a)-0.5,
        `${key}/${stage}: the stopped machine is still holding its boom up (${top(stopped,b).toFixed(2)} against ${top(working,a).toFixed(2)})`);
    }
    // And a slowed site is still a working site, here as everywhere else.
    assert.equal(digest(site.build(key,'foundation',{status:'building'})),
      digest(site.build(key,'foundation',{status:'slowed'})),`${key}: slowed earthworks is still earthworks`);
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
