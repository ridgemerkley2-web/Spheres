#!/usr/bin/env node
'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const filename=path.resolve(__dirname,'../../spheres-web/ui/equipment-mesh.js');
const source=fs.readFileSync(filename,'utf8'),context={module:{exports:{}}};
assert(source.includes('return Object.freeze({ build });'));
vm.runInNewContext(source.replace('return Object.freeze({ build });',
  'return {createBuilder, inspectionTrackConnector, LEVELS};'),context,{filename});
const {createBuilder,inspectionTrackConnector,LEVELS}=context.module.exports;
const sub=(a,b)=>a.map((v,i)=>v-b[i]),dot=(a,b)=>a.reduce((s,v,i)=>s+v*b[i],0);
const cross=(a,b)=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
const norm=a=>a.map(v=>v/Math.hypot(...a));
function fixture(pitch,width,angle,sealed){
  const b=createBuilder(LEVELS[0],6),cylinder=b.cylinder;
  // The reference retains both caps of the same solid pin. Only production's
  // cap choice differs; all triangle positions, materials and normals survive.
  if(sealed)b.cylinder=(a,z,r,color,segments,endRadius)=>cylinder(a,z,r,color,segments,endRadius,true);
  inspectionTrackConnector(b,[0,0,0],[0,Math.sin(angle),Math.cos(angle)],
    [0,Math.cos(angle),-Math.sin(angle)],width,pitch);
  return b.finish('track connector fixture');
}
function triangles(mesh){const out=[];for(let i=0;i<mesh.positions.length;i+=9)out.push([
  ...mesh.positions.subarray(i,i+9),...mesh.normals.subarray(i,i+9),
  ...mesh.colors.subarray(i,i+9),...mesh.materialClasses.subarray(i/3,i/3+3)].join(','));return out;}
function firstHit(mesh,origin,direction){
  let nearest=Infinity;const p=mesh.positions;
  for(let i=0;i<p.length;i+=9){
    const a=Array.from(p.subarray(i,i+3)),u=sub(Array.from(p.subarray(i+3,i+6)),a),v=sub(Array.from(p.subarray(i+6,i+9)),a);
    const h=cross(direction,v),det=dot(u,h);if(Math.abs(det)<1e-11)continue;
    const offset=sub(origin,a),s=dot(offset,h)/det;if(s< -1e-7||s>1+1e-7)continue;
    const q=cross(offset,u),t=dot(direction,q)/det;if(t< -1e-7||s+t>1+1e-7)continue;
    const distance=dot(v,q)/det;if(distance>1e-8)nearest=Math.min(nearest,distance);
  }
  return nearest;
}
test('inspection connector removes only twelve buried cap triangles and preserves every exterior triangle attribute',()=>{
  for(const pitch of [.16,.20,.25,.30])for(const width of [.38,.60,.82])for(const angle of [0,.31,Math.PI/2,Math.PI]){
    const old=fixture(pitch,width,angle,true),current=fixture(pitch,width,angle,false);
    assert.equal(old.triangleCount-current.triangleCount,12);
    const remaining=new Map();for(const triangle of triangles(old))remaining.set(triangle,(remaining.get(triangle)||0)+1);
    for(const triangle of triangles(current)){assert(remaining.get(triangle)>0,'Every retained triangle keeps its positions, normals, colors and material');remaining.set(triangle,remaining.get(triangle)-1);}
    assert.equal([...remaining.values()].reduce((a,b)=>a+b,0),12);
    assert.deepEqual(current.bounds,old.bounds);
    // The actual pin cap fits strictly inside its block at every tested pitch,
    // including a smaller pitch than any current production tracked platform.
    assert(.027<.061/2&&.027<pitch*.44/2&&.027<.074/2);
  }
});
test('all connector faces remain identical from external oblique rays on both mirrored pins',()=>{
  let checked=0,hits=0;
  for(const angle of [0,.43,Math.PI/2,Math.PI])for(const pitch of [.16,.25]){
    const a=fixture(pitch,.60,angle,true),b=fixture(pitch,.60,angle,false);
    const center=a.bounds.min.map((v,i)=>(v+a.bounds.max[i])/2);
    for(const camera of [[1,.4,1],[-1,.4,1],[1,-.4,-1],[-1,-.4,-1],[0,1,0],[0,-1,0],[1,0,0],[-1,0,0]]){
      const direction=norm(camera.map(v=>-v)),right=norm(cross(direction,Math.abs(direction[1])>.9?[0,0,1]:[0,1,0])),up=cross(right,direction);
      for(let x=-12;x<=12;x++)for(let y=-6;y<=6;y++){
        const origin=center.map((v,i)=>v-direction[i]*2+right[i]*x*.031+up[i]*y*.022);
        const before=firstHit(a,origin,direction),after=firstHit(b,origin,direction);
        assert(before===after||Math.abs(before-after)<1e-7,`External surface moved at angle ${angle}, camera ${camera}, ray ${x}/${y}`);
        checked++;if(Number.isFinite(before))hits++;
      }
    }
  }
  assert.equal(checked,20800);assert(hits>500,'The ray grid must hit both pins and their connector blocks');
});
test('selective cylinder cap control preserves legacy boolean callers and the exposed pin head',()=>{
  const build=caps=>{const b=createBuilder(LEVELS[0],6);b.cylinder([0,0,0],[1,0,0],.1,[.4,.4,.4],6,.1,caps);return b.finish('cap fixture');};
  const closed=build(true),open=build(false),pin=build({start:false});
  assert.equal(closed.triangleCount,24);assert.equal(open.triangleCount,12);assert.equal(pin.triangleCount,18);
  const head=[];for(let i=0;i<pin.positions.length;i+=9)if([0,3,6].every(j=>pin.positions[i+j]===1))head.push(i);
  assert.equal(head.length,6,'The outside head retains its complete six-triangle cap');
});
test('every omitted production pin cap is strictly enclosed by a solid connector box at all track widths',()=>{
  const records={boxes:[],caps:[]},probe={module:{exports:{}},records};
  const instrumented=source.replace('function box(center, size, color, basis) {',
    'function box(center, size, color, basis) { records.boxes.push({center,size,basis:basis||[[1,0,0],[0,1,0],[0,0,1]]});')
    .replace('function cylinder(a, b, radius, color, segments = 24, radiusEnd = radius, caps = true) {',
    'function cylinder(a, b, radius, color, segments = 24, radiusEnd = radius, caps = true) { if(caps&&caps.start===false)records.caps.push({a,b,radius});');
  vm.runInNewContext(instrumented,probe,{filename});let inspected=0;
  for(const platform of ['tank_standard','tank_heavy','tank_light','tank_destroyer','ground_ifv','ground_artillery','ground_air_defense'])for(const tracks of ['tracks_standard','tracks_wide','tracks_padded']){
    records.boxes=[];records.caps=[];probe.module.exports.build({platform,components:{tracks}});
    assert(records.caps.length>250,`${platform}/${tracks} must inspect production pin geometry`);
    for(const cap of records.caps){
      const axis=norm(sub(cap.b,cap.a));
      assert(records.boxes.some(box=>box.basis.every((u,i)=>{
        const projectedCenter=Math.abs(dot(sub(cap.a,box.center),u));
        const projectedRadius=cap.radius*Math.sqrt(Math.max(0,1-dot(axis,u)**2));
        return projectedCenter+projectedRadius<box.size[i]/2-1e-6;
      })),`${platform}/${tracks} cap at ${cap.a} must be strictly inside a solid box`);
      inspected++;
    }
  }
  assert(inspected>10000);
});
