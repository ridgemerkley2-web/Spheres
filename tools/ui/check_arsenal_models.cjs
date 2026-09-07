// Exercise the same catalog geometry used by manufacturing cards and OBJ downloads.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const path=require('node:path');
const file=path.resolve(__dirname,'../../spheres-web/ui/arsenal-models.js');
const models=require(file);

test('each arsenal model has finite render buffers, unit normals and usable spatial bounds',()=>{
  const ids=models.ids();assert(ids.length>0);assert.equal(new Set(ids).size,ids.length);
  for(const id of ids){
    const mesh=models.build(id),meta=models.meta(id);assert(mesh,`${id}: missing geometry`);assert.equal(mesh.id,id);assert.equal(mesh.name,meta.name);assert.equal(mesh.cls,meta.cls);
    assert(Number.isInteger(mesh.count)&&mesh.count>0&&mesh.count%3===0,`${id}: complete triangles`);
    for(const field of ['positions','normals','colors']){assert.equal(mesh[field].length,mesh.count*3,`${id}: ${field} vertex alignment`);assert(mesh[field].every(Number.isFinite),`${id}: finite ${field}`);}
    assert(mesh.colors.every(value=>value>=0&&value<=1),`${id}: color range`);
    for(let i=0;i<mesh.normals.length;i+=3)assert(Math.abs(Math.hypot(...mesh.normals.subarray(i,i+3))-1)<1e-5,`${id}: unit normal at ${i}`);
    for(const field of ['min','max','size','centre'])assert(mesh[field].length===3&&mesh[field].every(Number.isFinite),`${id}: finite ${field}`);
    assert(mesh.size.every(value=>value>0),`${id}: visible volume`);
    for(let i=0;i<mesh.positions.length;i++)assert(mesh.positions[i]>=mesh.min[i%3]&&mesh.positions[i]<=mesh.max[i%3],`${id}: vertex within bounds`);
  }
});

// ADDED, REPLACING NOTHING. A NORMAL ITS OWN FACE POINTS AWAY FROM IS NOT
// SHADING, IT IS NOISE — and until the smoothing pass arrived nothing here
// could tell the difference, because every triangle in the file was flat and so
// every vertex normal WAS its face normal and this held for free. The moment
// normals were averaged it stopped holding, silently: 971 triangles across the
// Air class came out carrying a normal in their own face's back hemisphere,
// the worst 172 degrees out, wherever a smoothing group closed round a fold of
// nearly half a turn — a wing trailing edge, and the leading edge of any panel
// swept back further than its own chord. That draws as a dark seam down the two
// lines of a wing a card actually reads, and every existing assertion stayed
// green through it: the normals were finite, they were unit, they were
// deterministic, and they were wrong. Two clauses, both universal, both true of
// all six classes as they stand:
//   - every triangle encloses area. One that does not draws no pixel and still
//     costs its place in the budget; 192 of them shipped in the Air pass. The
//     smallest REAL triangle in the deck is 1.4e-7 m2 — the TIP rib of the
//     Switchblade's folding propeller, re-measured after the Infantry pass,
//     which undercut by a factor of seven the 9.7e-7 the Missile pass recorded
//     here (the ROOT of the OWA drone's propeller blade), which had itself
//     undercut the Air pass's 1.4e-6. A propeller blade has been the floor
//     three passes running, for the same reason each time: it is the one part
//     in the deck with a real aerofoil section and a chord measured in
//     centimetres, so its cosine-spaced leading-edge points land a tenth of a
//     millimetre apart. Thirty-four triangles in that model sit in the same
//     decade and none below it. The floor below is a cross-product magnitude
//     of 1e-12, an area of 5e-13 m2, so it still sits five and a half orders of
//     magnitude clear of anything authored. RE-MEASURE THIS RATHER THAN CARRY
//     IT FORWARD: it is the whole of the margin, and every detail pass so far
//     has shrunk it. Re-measured after the Naval pass and NOT moved: the
//     Switchblade's tip rib still holds it at 1.4e-7 m2, and the smallest the
//     Naval class brought is 3.2e-6 — the crown ring of a mushroom ventilator
//     on the patrol craft's forecastle, a 130 mm dome cut into nine segments
//     and four rings, twenty-three times larger than the deck's floor-holder
//     and six and a half orders of magnitude clear of the 5e-13 bar itself.
//     This is the first pass in four that did NOT shrink this number, and the
//     reason is worth knowing before the next one is written: nothing in the
//     Naval class has a chord measured in centimetres. It has propellers, and
//     they are lofted aerofoils like every other blade here, but a warship's
//     screw blade is a third of a metre across where a Switchblade's is thirty
//     millimetres — so the floor is set by what a model IS, not by how much
//     detail a pass added. Re-measured after the ARMOUR pass and not moved
//     either, and by the widest margin any pass has left: the Switchblade's tip
//     rib still holds it at 1.4e-7 m2 and the smallest triangle the three tanks
//     brought is 6.0e-5 — a track link's guide horn, 430 times the deck's
//     floor-holder and eight orders of magnitude clear of the 5e-13 bar. The
//     reason is the same one the Naval pass found: an armoured vehicle's
//     smallest authored feature is a bolt head or a horn measured in
//     centimetres, and there is no aerofoil anywhere in the class.
//     RE-MEASURED AFTER THE SPACE PASS, the last class, and not moved: the
//     Switchblade's tip rib still holds the deck at 1.4e-7 m2, and the smallest
//     the two spacecraft bring is 3.3e-6 — a solar cell's chamfer corner on the
//     constellation's lead craft, twenty-four times the deck's floor-holder and
//     six and a half orders of magnitude clear of the 5e-13 bar. Two passes in
//     six have shrunk this number and four have not, and the reading is the one
//     the Naval pass wrote down: the floor is set by what a model IS. A solar
//     array's smallest authored feature is a cell some centimetres across; only
//     a lofted aerofoil with a chord in centimetres goes below that, and there
//     is no aerofoil in space.
//     RE-MEASURED AFTER THE P0 DETAIL PASS — the one that took the deck from
//     272,491 catalogue triangles to 392,407 across all six classes at once —
//     AND NOT MOVED. The Switchblade's tip rib still holds the deck at 1.4e-7
//     m2, and the class floors are now Air 8.2e-7 (the CCA's bay round, a
//     lofted 50 mm-radius body), Missile 9.7e-7 (the OWA's propeller root),
//     Naval 1.4e-6, Space 3.3e-6 and Armour 6.0e-5. Two passes in seven have
//     shrunk this number and five have not, and the reading is unchanged: the
//     floor is set by what a model IS. This pass added roughly 120,000
//     triangles of fairings, hinges, flange bolts, grating, tiles and deck
//     clutter and did not go near it, because none of that is an aerofoil.
//   - every vertex normal shares a half-space with the face carrying it. This
//     asks for the sign and claims nothing more, deliberately: the tightest
//     corner in the deck sits at 0.0017 — msl_brm's, tighter than the 0.0037
//     the Air pass measured here — so any margin above zero would be a number
//     invented rather than measured, and the next pass would only have to
//     move it again. Re-measured after the Infantry pass and NOT moved:
//     msl_brm still holds it, and the tightest the Infantry class brought is
//     0.0049, the fold where the C4ISR shelter's waveguide flange meets its
//     run. Nothing in that pass came near the bar. Re-measured after the Naval
//     pass and STILL not moved, though that pass got there the hard way:
//     msl_brm holds it at 0.0017 and the tightest the Naval class brought is
//     0.0199, the aft tip of the fillet fairing an SSN's sail into its casing.
//     It reached that from 0.0005 — the WORST corner in the deck at one point
//     in the pass, and a number this bar could not have caught, because 0.0005
//     is a normal ninety degrees off its own face and this asks only for the
//     sign. The three
//     causes were all structural rather than local, all invisible to every
//     assertion in this file, and all worth naming so the next class does not
//     re-import them: a smoothed cylinder closed by `tube`'s own caps (the cap
//     is one flat face sharing every rim vertex with the wall, and on anything
//     shorter than it is wide the area weighting lets it swing the rim's normal
//     round onto the cap); a hull band lofted as a CLOSED ring (the panel that
//     closes it is a full-beam lid at the waterline, never drawn, inside the
//     same smoothing group as the sides it then drags over); and a fan whose
//     hub sat in a run of collinear contour points — which shipped zero-area
//     triangles too, and those this bar DID catch, which is how it was found.
//     The count of corners wearing a normal more than seventy-eight degrees off
//     their own face fell by an order of magnitude across the twelve naval
//     meshes on those fixes, at no cost in triangles at all: the same geometry,
//     with the flat faces kept out of the smoothing groups the curved ones
//     belong to.
//     RE-MEASURED ON THE VERIFICATION PASS, WHICH FOUND TWO MORE OF THE SAME
//     TWO CAUSES STILL STANDING and closed them the same way, again at zero
//     triangle cost — 324 such corners became 312, and the worst corner on each
//     of the two surface ships moved from about eighty-eight degrees to
//     eighty-five. Both were exactly the shapes named above, which is the point
//     of naming them: the ship's boat was a CLOSED-RING loft whose fourth panel
//     is a full-beam lid across the gunwales, dragging both topsides (0.0329 on
//     nav_patrol, 0.0253 on nav_escort — the worst corners in either model),
//     and the frigate's two funnel uptake caps were a CAPPED `tube` inside a
//     smoothing group, 1.4 m across and 0.6 m tall, at 0.0693. A sweep for the
//     second pattern is one paren-balanced scan for a `loft` or `tube` call
//     lexically inside a `.soft(` block without an explicit `false` in the caps
//     slot; it now returns NOTHING in the naval kit and forty-six sites
//     in Air, Missile and Infantry, which is where the next pass should start.
//     THE ARMOUR PASS IMPORTED NEITHER PATTERN, and how is worth more than
//     that: the three tanks were authored against this list rather than checked
//     against it afterwards. Every `loft` and `tube` inside one of their
//     `.soft(` blocks passes `false` in the caps slot and is shut by a
//     `discCap` or a bare `fan` emitted OUTSIDE the group, and the one
//     closed-ring loft in the class — the cast turret of the second-generation
//     tank, whose closing panel is the full-width ring floor the model never
//     draws — hands that panel to `loft`'s own `hard` argument, which is the
//     same escape the aerofoil trailing edge uses. So the tightest corner the
//     class brings is 0.4809 with NO corner anywhere under 0.2; msl_brm still
//     holds the bar at 0.0017 and it is again NOT moved.
//     BOTH AVOIDANCES WERE MEASURED BY RE-INTRODUCING THE DEFECT, because a
//     claim about shading that nothing can fail is worth nothing. Capping ONE
//     tube inside a smoothing group — the return roller, 3 per side per tank —
//     takes the class from 0.4809 to 0.0529 and puts 216 corners under 0.2.
//     Dropping the cast turret's `hard` argument, so its ring floor smooths
//     with the flanks it shares a rim with, takes it to 0.0026: within a factor
//     of one and a half of the worst corner in the entire deck, off one
//     argument. NEITHER BREAK REDS THIS TEST, and that is the honest reading of
//     what this clause is — it asks for the sign, so it catches an INVERTED
//     normal and a degenerate face and it does not catch a merely bad one. The
//     bar stays at zero for the reason given above; the guard against these two
//     is that they are named here and re-measured each pass.
//     THE SPACE PASS IMPORTED NEITHER PATTERN EITHER, and it was the class most
//     exposed to the first of them: a spacecraft is a stack of short cans, and a
//     can shorter than it is wide is the exact shape whose cap can swing the
//     rim's normal round onto itself. Every `loft` and `tube` inside a `.soft(`
//     block in the space kit passes `false` in the caps slot and is shut by a
//     `discCap` emitted OUTSIDE the group — drive canisters, pedestals, antenna
//     hubs, thruster bells, star-tracker baffles, hinge pins, the separation
//     ring and the telescope's own barrel, twenty-two sites, no exceptions. The
//     separation ring also had to be authored in INCREASING z and then hung off
//     the aft face rather than lofted downward, which is the trap `motorBell`
//     sidesteps with a half turn: a band lofted in decreasing z winds inward and
//     ships a ring of inverted normals — WHICH THIS CLAUSE DOES NOT CATCH. That
//     sentence used to end "which THIS clause does catch"; the verification pass
//     checked it the way this file checks everything, by breaking the geometry,
//     and it is false. See the next test, which was written because it is false.
//     The class
//     brings a tightest corner of 0.2170 — the coarse constellation's antenna
//     rim — with NO corner anywhere under 0.2, the second class after Armour to
//     manage that; msl_brm still holds the deck at 0.0017 and the bar is again
//     NOT moved. Re-measured, not carried: the count of corners under 0.2 across
//     the whole deck at both levels stands at 3,768, and none of them is in
//     Space or Armour. The sweep itself was re-run rather than assumed — 205
//     `loft`/`tube` calls sit inside a `.soft(` block across the file and 47 of
//     them do not pass an explicit `false` in the caps slot. RE-RUN AGAIN ON THE
//     VERIFICATION PASS at the same 205 and 47, and one attribution corrected:
//     they are not "every one in Air, Missile or Infantry". Four sit in helpers
//     shared wider than that — three in `roadWheel` and one in `truckKit`,
//     reached from Missile and Infantry — and the one at the top of `bodyLoft`
//     is reached from Naval as well, by `submarine`. With all six classes raised
//     there is no next
//     pass for that list to be handed to, so it is a standing item now and not
//     a forward pointer: it is where a Naval-style shading repair would go if
//     one is ever wanted, and it costs no triangles either way.
//     RE-MEASURED AFTER THE P0 DETAIL PASS, WHICH RE-IMPORTED THE FIRST
//     PATTERN TWICE AND WAS CAUGHT BY MEASURING RATHER THAN BY THIS BAR. Both
//     were the same shape the Naval pass named — a can shorter than it is wide
//     closed by `loft`'s own caps INSIDE a `.soft(` block — and both were added
//     by this pass, which is the point of re-measuring instead of carrying the
//     number forward:
//       - `jet`'s anti-collision beacon, a dome 0.23 m across and 0.08 m tall
//         on the spine of every aircraft in the class. It took the DECK's worst
//         corner from 0.0017 to 0.00046 — a normal ninety degrees off its own
//         face — and all nine tests in this file stayed green through it.
//       - the rolled camouflage net in `armour`'s bustle rack, 0.3 W long and
//         0.22 H across. It took the Armour class from 0.4809, which is the
//         number recorded above as "NO corner anywhere under 0.2", to 0.0309
//         with twelve corners under 0.2.
//     Both were repaired the way the Naval pass repaired its four: `false` in
//     the caps slot and the fans emitted OUTSIDE the smoothing group, at a cost
//     of nothing but the two fans that were already being drawn. Re-measured
//     after the repair, every class is back at the value recorded above, to
//     four figures — Infantry 0.0049, Armour 0.4809, Air 0.0037, Naval 0.0199,
//     Missile 0.0017, Space 0.2170 — and msl_brm still holds the deck at
//     0.0017, so the bar is again NOT moved. What DID move is the population:
//     corners under 0.2 across both levels stand at 4,106 against 3,768, on a
//     deck that grew by 44%, and none of them is in Space or Armour.
//     THE SWEEP WAS RE-RUN RATHER THAN ASSUMED and it grew with the deck: 335
//     `loft`/`tube` calls now sit inside a `.soft(` block (was 205) and 48 of
//     them do not pass an explicit `false` in the caps slot (was 47). The
//     detail pass added 130 such calls and exactly one of them to the second
//     list, which is the discipline the Armour and Space passes established —
//     author against this list rather than check against it afterwards.
//     This clause is also what catches an INVERTED normal, which nothing
//     in this file caught before — unit, finite and deterministic are all true
//     of a normal pointing exactly backwards.
test('no triangle is degenerate and no vertex normal contradicts the face it sits on',()=>{
  for(const id of models.ids())for(const mesh of [models.build(id),models.build(id,null,{lod:'far'})]){
    const p=mesh.positions,n=mesh.normals;
    for(let t=0;t<mesh.count/3;t+=1){
      const i=t*9;
      const ux=p[i+3]-p[i],uy=p[i+4]-p[i+1],uz=p[i+5]-p[i+2];
      const vx=p[i+6]-p[i],vy=p[i+7]-p[i+1],vz=p[i+8]-p[i+2];
      const fx=uy*vz-uz*vy,fy=uz*vx-ux*vz,fz=ux*vy-uy*vx;
      assert(Math.hypot(fx,fy,fz)>1e-12,`${id} (${mesh.lod}): triangle ${t} encloses no area`);
      for(let c=0;c<3;c+=1){const j=i+c*3;
        assert(n[j]*fx+n[j+1]*fy+n[j+2]*fz>0,`${id} (${mesh.lod}): triangle ${t} corner ${c} carries a normal its own face points away from`);}
    }
  }
});

// ADDED BY THE SPACE VERIFICATION PASS, REPLACING NOTHING, and it exists
// because the paragraph above told a lie about the clause it sits on. The claim
// was that a band lofted in decreasing z ships inverted normals "which THIS
// clause does catch". It was checked the way this file checks everything —
// `spaceBus`'s separation ring was re-authored in decreasing z, the deck's hash
// moved, and every one of that ring's 64 faces turned to face the axis instead
// of away from it. ALL EIGHT TESTS STAYED GREEN.
//
// The reason is structural and bounds what that clause can ever be asked for.
// `finish` derives the vertex normal FROM the winding, and then refuses to hand
// a corner a normal its own face points away from. So when a surface is wound
// inside-out its vertex normals and its face normals invert TOGETHER and their
// dot product stays positive. A mesh compared only against itself cannot tell
// you which way it faces. That clause catches an externally corrupted normal
// buffer and a zero-area triangle — both re-proved on this pass by breaking
// them — and it cannot catch winding. Saying it can is worse than saying
// nothing, because the next author reads it and stops guarding by hand.
//
// THE OBVIOUS UNIVERSAL REPLACEMENT IS NOT AVAILABLE, and that was measured
// rather than assumed. Signed volume about a model's own centre is already
// NEGATIVE on two Missile models as they stand — msl_deterrent at -0.067 of its
// bounding box, gbi at -0.057 — so a `volume > 0` clause would ship red on
// models this pass may not touch, which is a finding for whoever owns Missile
// and not a licence to widen anything here. Per-loft winding is not assertable
// either: 1,593 of the deck's 7,120 lofted section pairs come out facing
// inward, and most are RIGHT to. A telescope bore, an intake duct, a primary
// mirror and a wheel well are all surfaces you are meant to see the inside of.
//
// So what is asserted is THE CONVENTION ITSELF, at the primitive, where it is a
// fact about two lines of code instead of a fact about forty-six models: `loft`
// is direction-sensitive, and every model in this file is authored to loft
// forward and turn the part. `motorBell`'s half turn and the separation ring's
// hung aft face are both that convention being kept. This goes red if anyone
// "fixes" `loft` to be direction-agnostic — which would silently turn every
// intentional bore, duct and mirror in the deck inside-out, a far larger hazard
// than the one ring that prompted it.
test('loft winds outward only for an increasing z run, which is the convention every model here is authored to',()=>{
  const {Mesh,primitives:{ring}}=models;
  // A probe wall is a plain 12-sided cylinder about the local z axis, so
  // "outward" is unambiguous: the face normal agrees with the radius or it does
  // not. No model geometry is involved and nothing is cached.
  const facing=(draw)=>{
    const m=new Mesh('near');draw(m);
    const g=m.finish(),p=g.positions,n=g.normals;let out=0,inward=0;
    for(let i=0;i<p.length;i+=9){
      const cx=(p[i]+p[i+3]+p[i+6])/3,cy=(p[i+1]+p[i+4]+p[i+7])/3,r=Math.hypot(cx,cy);
      assert(r>1e-9,'the probe wall is off-axis by construction');
      if((n[i]*cx+n[i+1]*cy)/r>0)out+=1;else inward+=1;
    }
    return {out,inward,total:g.count/3};
  };
  const wall=(zs)=>(m)=>m.loft(zs.map(z=>({z,pts:ring(1,1,12)})),[0.5,0.5,0.5],false);
  const up=facing(wall([0,1,2])),down=facing(wall([0,-1,-2]));
  assert.equal(up.total,down.total,'the same wall either way round');
  assert(up.total>0,'the probe wall drew something');
  assert.equal(up.inward,0,`an increasing z run must wind outward (${up.inward} of ${up.total} faces point at the axis)`);
  assert.equal(down.out,0,`a decreasing z run winds INWARD and nothing downstream will tell you (${down.out} of ${down.total} faces point away from the axis)`);
  // `tube` is the same call with the run written as a length, so a negative
  // length is the same trap wearing different clothes.
  const back=facing((m)=>m.tube(1,1,-2,12,[0.5,0.5,0.5],false));
  assert.equal(back.out,0,`a tube of negative length winds inward too (${back.out} of ${back.total} faces point away from the axis)`);
  // And the escape every model actually uses: turn the part rather than loft it
  // backwards. A half turn about x re-points a forward loft aft AND flips the
  // determinant, so `tri` reverses the winding and the surface still faces out.
  const turned=facing((m)=>{m.save().rotX(180);wall([0,1,2])(m);m.restore();});
  assert.equal(turned.inward,0,`a forward loft under a half turn must still face out (${turned.inward} of ${turned.total} faces point at the axis)`);
});

test('arsenal recipes are deterministic across fresh loads and reuse cached meshes',()=>{
  delete require.cache[require.resolve(file)];const fresh=require(file);
  assert.deepEqual(fresh.ids(),models.ids());
  for(const id of models.ids()){
    const a=models.build(id),b=fresh.build(id);assert.equal(models.build(id),a,`${id}: reuse cache`);
    for(const field of ['positions','normals','colors','min','max','size','centre'])assert.deepEqual(a[field],b[field],`${id}: deterministic ${field}`);
  }
});

// The export carries ONE NORMAL PER VERTEX, not one per face. It used to carry
// one per face, which was right while every triangle in the file was flat; now
// that fuselages, wings and nozzles carry averaged normals, a per-face export
// would drop the smooth shading on the way out. Every clause below is at least
// as strict as the one it replaces: three times as many `vn` lines are demanded
// rather than a third as many, each corner must name ITS OWN normal instead of
// any normal in range, every normal must be referenced as well as every vertex,
// and the exported numbers are now compared against the mesh they claim to be —
// which the per-face version never did for a single value.
test('OBJ exports retain every triangle and carry the mesh position, colour and per-vertex normal of each corner',()=>{
  for(const id of models.ids()){
    const mesh=models.build(id),obj=models.toOBJ(id),lines=obj.trim().split('\n');
    const vertices=lines.filter(line=>line.startsWith('v ')),normals=lines.filter(line=>line.startsWith('vn ')),faces=lines.filter(line=>line.startsWith('f '));
    assert.equal(vertices.length,mesh.count,`${id}: OBJ vertices`);assert.equal(normals.length,mesh.count,`${id}: OBJ normals, one per vertex`);assert.equal(faces.length,mesh.count/3,`${id}: OBJ faces`);
    assert(lines.includes(`o ${id}`));
    for(const line of [...vertices,...normals])assert(line.split(/\s+/).slice(1).map(Number).every(Number.isFinite),`${id}: finite OBJ attributes`);
    for(let v=0;v<mesh.count;v+=1){
      const got=vertices[v].split(/\s+/).slice(1).map(Number),nrm=normals[v].split(/\s+/).slice(1).map(Number);
      assert.equal(got.length,6,`${id}: vertex ${v} carries position and colour`);
      for(let k=0;k<3;k+=1){
        assert.equal(got[k],Number(mesh.positions[v*3+k].toFixed(4)),`${id}: vertex ${v} position ${k}`);
        assert.equal(got[3+k],Number(mesh.colors[v*3+k].toFixed(3)),`${id}: vertex ${v} colour ${k}`);
        assert.equal(nrm[k],Number(mesh.normals[v*3+k].toFixed(4)),`${id}: vertex ${v} normal ${k}`);
      }
    }
    const usedV=new Set(),usedN=new Set();
    for(let f=0;f<faces.length;f+=1){const corners=faces[f].slice(2).split(' ');assert.equal(corners.length,3);
      for(let c=0;c<3;c+=1){const match=/^(\d+)\/\/(\d+)$/.exec(corners[c]);assert(match,`${id}: valid OBJ corner`);const vertex=Number(match[1]),normal=Number(match[2]);
        assert.equal(vertex,f*3+c+1,`${id}: face ${f} corner ${c} names its own vertex`);
        assert.equal(normal,vertex,`${id}: face ${f} corner ${c} names its own normal`);
        usedV.add(vertex);usedN.add(normal);}}
    assert.equal(usedV.size,vertices.length,`${id}: all vertices exported into faces`);
    assert.equal(usedN.size,normals.length,`${id}: all normals exported into faces`);
  }
});

// The coarse variant, and the promise that asking for it did not move the
// signature. `build(id, cls)` must still hand back the detailed mesh from the
// same cache slot it always used; the coarse one arrives only through the third
// argument, must be a legal mesh by every rule the detailed one obeys, and must
// actually be cheaper than the mesh it stands in for.
test('the far level of detail is optional, cheaper, deterministic and never displaces the detailed mesh',()=>{
  delete require.cache[require.resolve(file)];const fresh=require(file);
  for(const id of models.ids()){
    const near=models.build(id),far=models.build(id,null,{lod:'far'});
    assert(far,`${id}: far mesh`);assert.equal(far.id,id);assert.equal(far.name,near.name);assert.equal(far.cls,near.cls);
    assert.equal(near.lod,'near',`${id}: default stays detailed`);assert.equal(far.lod,'far');
    assert.notEqual(far,near,`${id}: far is its own mesh`);
    assert.equal(models.build(id),near,`${id}: the detailed slot survives a far build`);
    assert.equal(models.build(id,null,{lod:'far'}),far,`${id}: far reuses its cache`);
    assert(Number.isInteger(far.count)&&far.count>0&&far.count%3===0,`${id}: far complete triangles`);
    assert(far.count<=near.count,`${id}: far is not heavier than near`);
    // The coarse variant is for a map pin, so it has a ceiling of its own, and
    // a model too heavy to BE a map pin has to offer one. Both clauses are
    // general: they hold for every class in the deck and they are what stops a
    // future detail pass from shipping a catalogue mesh with no map level
    // behind it.
    //
    // THE FLOOR HAS MOVED UP HERE, WHERE THE PREVIOUS PASS SAID IT BELONGED.
    // It used to live only in the budget test below, scoped to the classes that
    // had had their pass, and the reason was named: Space had not, its two
    // models averaged 542 triangles at both levels, and a floor asserted here
    // would have been a red test about work nobody had done. Space has now had
    // its pass — 542 to 10,465 average, with map forms at 1,354 and 876 — so
    // every one of the deck's 46 models is inside 300..1500 coarse and the
    // caveat is spent. Asserted UNIVERSALLY rather than per budgeted class,
    // which is strictly the stronger claim: it binds a model added tomorrow in
    // a class this file has never heard of, and the old scoping did not.
    assert(far.count/3>=300,`${id}: far is too coarse to be a model (${far.count/3} triangles)`);
    assert(far.count/3<=1500,`${id}: far fits a map pin (${far.count/3} triangles)`);
    if(near.count/3>1500)assert(far.count<near.count,`${id}: a catalogue mesh over 1500 triangles needs a real coarse variant`);
    for(const field of ['positions','normals','colors']){assert.equal(far[field].length,far.count*3,`${id}: far ${field} alignment`);assert(far[field].every(Number.isFinite),`${id}: finite far ${field}`);}
    assert(far.colors.every(value=>value>=0&&value<=1),`${id}: far colour range`);
    for(let i=0;i<far.normals.length;i+=3)assert(Math.abs(Math.hypot(...far.normals.subarray(i,i+3))-1)<1e-5,`${id}: far unit normal at ${i}`);
    assert(far.size.every(value=>value>0),`${id}: far visible volume`);
    for(let i=0;i<far.positions.length;i++)assert(far.positions[i]>=far.min[i%3]&&far.positions[i]<=far.max[i%3],`${id}: far vertex within bounds`);
    const other=fresh.build(id,null,{lod:'far'});
    for(const field of ['positions','normals','colors','min','max','size','centre'])assert.deepEqual(far[field],other[field],`${id}: deterministic far ${field}`);
  }
});

// ADDED BY THE ARMOUR VERIFICATION PASS, REPLACING NOTHING, and it exists
// because a real defect walked past every clause above. `trophy` is the deck's
// Trophy Active Protection: a Third-Generation Armour with the suite that is
// the entire point of the kit bolted to it. Its CATALOGUE mesh had that suite
// and its MAP mesh did not — the greeble block was gated whole, which is the
// right rule almost everywhere and was the wrong one here. The measured
// consequence: trophy's far mesh came back position- and normal-identical to
// arm_gen3's, float for float, 9,729 floats of each with zero differences, and
// the only thing left separating the two pins was 2,250 colour floats. Tan
// paint. Every assertion in this file stayed green through it, and each was
// right to: the mesh was finite, unit-normalled, in bounds, deterministic,
// cheaper than its near form and inside the map band. Nothing here was asking
// whether it was still the model it claimed to be.
//
// So this asks the one question none of them did. It is a universal clause and
// it holds for all six classes as they stand — 46 models, 46 distinct
// position+normal buffers at BOTH levels of detail. Note what it is keyed on:
// shape, not colour. Two kits that differ only in paint are not two kits, and a
// deck where a coarse pass has quietly collapsed two ids into one silhouette is
// exactly the failure a per-model budget cannot see, because both models are
// individually inside every band. Colour is deliberately excluded from the
// hash — a recolour that saved a real difference in geometry would pass this,
// as it should, and a recolour that saved nothing else will not.
test('no two models collapse into the same shape at either level of detail',()=>{
  const crypto=require('node:crypto');
  for(const opts of [undefined,{lod:'far'}]){
    const seen=new Map();
    for(const id of models.ids()){
      const mesh=models.build(id,null,opts);
      const shape=crypto.createHash('sha256')
        .update(Buffer.from(Float32Array.from(mesh.positions).buffer))
        .update(Buffer.from(Float32Array.from(mesh.normals).buffer)).digest('hex');
      assert(!seen.has(shape),`${id} (${mesh.lod}) is the same shape as ${seen.get(shape)} — two ids, one model`);
      seen.set(shape,id);
    }
  }
});

// The detail budget, and WHY IT IS SCOPED TO THE CLASSES THAT HAVE HAD THEIR
// PASS. The roadmap's section 4 asks a catalogue preview for 4,000-12,000
// triangles and a map form for 300-1,500; the whole deck averaged 452 against
// that, which is what the P0 art pass exists to fix. Air and Missile have been
// raised and are held to the budget here — Missile joined on the pass that gave
// the launchers running gear, the rounds their joint bands, raceways, filleted
// fins and nozzle throats, and the glide bodies their thermal courses; its
// eleven models went from a 359-triangle average at HEAD (3,944 triangles
// across the eleven) to 4,911. INFANTRY JOINS ON THE PASS THAT GAVE THE
// FIGURES BODIES — smoothed trunks, limbs with joints, plate carriers, load
// pouches and a modelled weapon — the mechanised formation a track built out
// of links with guide horns and dished road wheels instead of a painted band,
// the two air vehicles aerofoil sections and gimballed cameras, and the two
// abstract fits an honest equipment rack with unit handles, card slots,
// circular connectors and a bolt line. Its eight models went from a
// 407-triangle average (3,252 triangles across the eight) to 6,212 (49,692).
// NAVAL JOINS ON THE PASS THAT GAVE THE HULLS LINES — a turn of bilge and a
// smoothed shell in two bands that share a waterline, a cambered deck inside a
// bulwark and its guard rails, plating strakes down the side, superstructure
// that steps back tier by tier with window bands and doors, lattice masts,
// gun mounts with a bore drilled down the barrel, launcher cells that are lids
// rather than decals, and shafts, A-brackets, skewed screws and rudders under
// the counter; the two boats a casing with a hard edge, a faired sail, free
// flood holes, launch-tube hatches and a seven-bladed screw; and the laser
// mount the fasteners, louvres and plumbing that separate a weapon from a
// searchlight. Its six models went from a 533-triangle average (3,200
// triangles across the six) to 6,866 (41,198). The task group is still three
// ships in company inside that one budget and not one fictional super-ship,
// which is the constraint that shaped its lean lines table.
// ARMOUR JOINS ON THE PASS THAT GAVE THE THREE TANKS RUNNING GEAR, and that is
// where nearly all of its triangles went because that is where the eye goes: at
// card size the wheel line is found before the gun is, and these three carried
// a painted band with nine bare cylinders under it, which reads as a crate on
// blocks. Each now has a belt built out of shoes with a grouser pad outboard
// and a guide horn inboard, a torsion arm and a bump stop at every station, a
// TOOTHED sprocket at the drive end and a SPOKED idler at the other (drawing
// both ends the same is the tell that one wheel was drawn twice rather than a
// vehicle designed), and the return rollers holding the top run up. Over that:
// an eleven-point folded hull section in place of a four-point wedge, glacis
// applique laid on the measured plate angle with spare track links and a bolted
// nose beam, segmented skirts on hinge lugs hung from a shelf that walks the
// track's own height rather than guessing it, deck grilles, a driver's hatch
// with his periscopes, and a gun lofted with a fume extractor, thermal sleeve
// straps and a bore drilled back to a dark disc. The two generations are told
// apart the way the real ones are: the second's turret is a CASTING and is the
// one shell in the class that is smoothed, the third's is welded plate and
// stays faceted, and Trophy is the third with framed radar faces and trainable
// launchers round it. Its three models went from a 788-triangle average (2,364
// across the three) to 9,351 (28,053).
// SPACE JOINS ON THE PASS THAT CLOSES THE LIST, and it is the last class in the
// deck: all six are now graded and `graded` is every id there is. Its two models
// had a gold cuboid, four blue plates and a twelve-segment paraboloid between
// them — 542 triangles apiece, every triangle flat, and no coarse variant at
// all because there was nothing in them left to coarsen. What they have now is
// what a spacecraft is made of: an eight-sided bus of equipment panels wrapped
// in blanket that is QUILTED rather than painted, arrays built bay by bay out of
// individual cells with the interconnect gap between them as real shadow, and
// the three moving joints drawn as joints — a solar-array drive canister the
// wing rotates on, an antenna that trains in azimuth on a bearing ring and
// elevates on a yoke, and a boom that unfolds about a hinge with a trunnion
// either side of it. The antenna gained the half of a reflector a card actually
// sees: a stiffening rim, ribs and a hoop across the back, a subreflector on its
// tripod and a feed looking up into it. And the electro-optical satellite's
// telescope is a telescope — a scalloped sunshade, metering hoops, baffle vanes
// down a BORE THAT IS NOT CAPPED, the secondary on its four-vane spider and the
// primary at the bottom of it, which is the one thing on that model a
// three-quarter view is guaranteed to look straight into. Its two models went
// from a 542-triangle average (1,084 across the two, and the same 1,084 coarse)
// to 10,465 (20,930), with map forms of 1,354 and 876.
// The constellation is still THREE craft in company inside that one budget and
// not one fictional super-satellite, which is the constraint that shaped it: the
// two flying behind the lead ship are drawn at 0.42 and 0.34 scale, where a
// blanket panel is two centimetres across on a twelve-metre model, so they carry
// the structure and not the quilting and the triangles that buys go into the
// ship the card is about.
// The list is now closed, and closing it is the point: every class in the deck
// is held to the roadmap's band, so the only way to ship a 452-triangle model
// again is to widen the band — which is the move this file exists to forbid.
//
// EXCEPT THAT IT WAS NOT THE ONLY WAY, AND THE AUDIT PASS MEASURED THE OTHER
// ONE. Everything above this line is true and none of it was enforced: `BUDGETED`
// is a hand-kept list of strings, the test walks only the ids whose class is on
// it, and nothing anywhere asked the list to cover the deck. So the cheap way
// past the band was never to widen it — it was to delete six characters. Broken
// deliberately, the way iron rule 5 requires a bar to be checked: drop 'Space'
// from the set and build both spacecraft at coarse detail, and they ship their
// CATALOGUE mesh at 1,354 and 876 triangles — back under the 452-average this
// whole pass exists to end — with all nine tests in this file green and this
// test still announcing in its own name that every class holds its budget. That
// is the exact failure the scoping was invented to avoid while classes were
// still mid-pass, left standing after the last one landed.
//
// So the coverage is asserted rather than commented. The list stays, because
// while it matches the deck it is a readable statement of which classes have
// been through a pass; it just can no longer be quietly shortened, and a kit
// added tomorrow in a seventh class reds this until someone decides what band
// it belongs in. Strictly an ADDED clause: every id the old version walked, the
// new one still walks, under the same two bands.
//
// THE P0 DETAIL PASS RAISED ALL SIX CLASSES AT ONCE, and the numbers above are
// what it started from. Deck total 272,491 catalogue triangles to 392,407, a
// 5,924 average to 8,531; the map form moved 39,508 to 39,564, which is 0.14%
// and is the whole point — everything added is behind an `m.far` gate or an
// `m.lod` pair, so the level that scales when a hundred pins are on a map did
// not move. Per class, catalogue average before to after: Air 4,912 to 9,064,
// Missile 4,911 to 7,215, Infantry 6,212 to 7,261, Naval 6,866 to 9,600,
// Armour 9,351 to 10,071, Space 10,465 to 11,064. The near band is now
// 5,112..11,502 against 4,000..12,000 and the far band 364..1,354 against
// 300..1,500.
//
// WHAT THE CEILING MEANS FOR THE NEXT PASS, stated plainly because it is the
// binding constraint and not an opinion: 12,000 is a per-model CAP, the deck
// averaged 5,924 before this pass, so the largest multiple any pass can ever
// deliver from that start is 2.03x and only by putting every one of the 46
// models within a hair of the cap. 1.44x is what was reachable while every
// added triangle was a real fitting rather than a finer ring. Anyone asked for
// "2-4x" from here is being asked to widen this band, which is the move this
// file exists to forbid — so the answer is to say so, not to ship it.
const BUDGETED=new Set(['Air','Missile','Infantry','Naval','Armour','Space']);
test('every model in the deck holds the catalogue and map budgets, and no class can opt out',()=>{
  const graded=models.ids().filter(id=>BUDGETED.has(models.meta(id).cls));
  assert(graded.length>0,'at least one class is budgeted');
  assert.deepEqual(graded,models.ids(),
    `the budget must cover the whole deck; ungraded: ${models.ids().filter(id=>!BUDGETED.has(models.meta(id).cls)).map(id=>`${id} (${models.meta(id).cls})`).join(', ')}`);
  for(const id of graded){
    const near=models.build(id).count/3,far=models.build(id,null,{lod:'far'}).count/3;
    assert(near>=4000&&near<=12000,`${id}: catalogue preview is ${near} triangles, outside 4000..12000`);
    assert(far>=300&&far<=1500,`${id}: map form is ${far} triangles, outside 300..1500`);
  }
});

test('unknown catalog ids fail cleanly or resolve only through an explicit known class',()=>{
  assert.equal(models.has('unknown-kit'),false);assert.equal(models.meta('unknown-kit'),null);assert.equal(models.build('unknown-kit'),null);assert.equal(models.toOBJ('unknown-kit'),null);
  for(const cls of new Set(models.ids().map(id=>models.meta(id).cls))){const mesh=models.build('unknown-kit',cls);assert(mesh,`${cls}: class fallback`);assert(models.has(mesh.id));assert.equal(mesh.cls.toLowerCase(),cls.toLowerCase());}
});
