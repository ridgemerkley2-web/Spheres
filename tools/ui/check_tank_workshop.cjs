'use strict';
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),{createRequire}=require('node:module');
const workshop=fs.readFileSync(path.join(__dirname,'../arsenal/tank-inspection.js'),'utf8');
function harness(){
  // Reuse the real controller's existing no-WebGL DOM fixture. The workshop
  // uses the production mesh generator and GLB exporter, not command stubs.
  const file=path.join(__dirname,'check_equipment_model.cjs'),tests=fs.readFileSync(file,'utf8');
  const scope={require:createRequire(file),assert,vm,source:fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/equipment-model.js'),'utf8'),bounds:{min:[-2,0,-4],max:[2,4,6]}};
  vm.runInNewContext(tests.slice(tests.indexOf('function fixture('),tests.indexOf("test('material readiness"))+';this.make=fixture;',scope);
  const f=scope.make({noGraphics:true}),nodes={},timers=[],created=[],revoked=[],anchors=[];let clickFails=false;
  const create=f.doc.createElement;
  f.doc.createElement=tag=>{const n=create(tag);n.tagName=tag.toUpperCase();n.append=(...children)=>children.forEach(c=>n.appendChild(c));
    if(tag==='a'){anchors.push(n);n.click=()=>{assert.equal(n.parentNode,f.doc.body,'Download anchor is attached before clicking');if(clickFails)throw Error('Blocked download');};}
    return n;};
  f.doc.createTextNode=text=>({textContent:text});
  for(const id of ['vehicle','lod','part','finish','wear','components','stats','reset-spec','export'])nodes[id]=f.doc.createElement(['components','stats'].includes(id)?'div':id.includes('spec')||id==='export'?'button':'select');
  Object.assign(nodes.vehicle,{value:'tank_standard'});nodes.lod.value='0';nodes.part.value='';nodes.finish.value='olive';nodes.wear.value='service';nodes.modelHost=f.host;
  f.doc.getElementById=id=>{assert(nodes[id],'Known workshop element '+id);return nodes[id];};
  const emit=f.host.dispatchEvent.bind(f.host);f.host.dispatchEvent=e=>{emit(e);f.host.listeners.get(e.type)?.(e);};
  f.ctx.EquipmentMesh=require('../../spheres-web/ui/equipment-mesh.js');f.ctx.EquipmentExport=require('../../spheres-web/ui/equipment-export.js');
  f.ctx.EquipmentModel={mount:(host,spec)=>{assert.equal(host,f.host);f.controller.update(spec);return f.controller;}};
  f.ctx.Option=function(text,value){const o=f.doc.createElement('option');o.textContent=text;o.value=value;return o;};
  f.ctx.URL={createObjectURL:blob=>{assert(blob instanceof Blob);const url='blob:test-'+created.length;created.push({url,blob});return url;},revokeObjectURL:url=>revoked.push(url)};
  f.ctx.Blob=Blob;f.ctx.setTimeout=fn=>{timers.push(fn);return timers.length;};
  vm.runInNewContext(workshop,f.ctx,{filename:'tank-inspection.js'});
  const change=(node,value)=>{node.value=value;node.listeners.get('change')({type:'change'});};
  const component=name=>{const label=nodes.components.children.find(n=>n.children[0].textContent===name);assert(label,name);return label.children.find(n=>n.tagName==='SELECT');};
  return {...f,nodes,timers,created,revoked,anchors,change,component,click:id=>nodes[id].listeners.get('click')(),failClick:value=>{clickFails=value;}};
}

test('workshop selection follows a part across LOD and component edits and clears on explicit preset or vehicle reset',()=>{
  const h=harness();try{
    h.change(h.nodes.part,'armament / standard weapon, mantlet and barrel');
    h.change(h.nodes.lod,'2');assert.equal(h.nodes.part.value,'armament / standard weapon, mantlet and barrel');
    h.change(h.component('Main gun'),'gun_120');assert.equal(h.nodes.part.value,'armament / heavy weapon, enlarged mantlet and sleeved barrel','Same armament slot follows its new part name');
    h.click('reset-spec');assert.equal(h.nodes.part.value,'');assert.equal(h.component('Main gun').value,'gun_105');assert.equal(h.nodes.lod.value,'2','Preset reset does not discard inspection detail preference');
    h.change(h.nodes.part,'turret / ring and faceted armor shell');h.change(h.nodes.vehicle,'tank_light');assert.equal(h.nodes.part.value,'');assert.equal(h.component('Turret').value,'turret_compact');assert.equal(h.component('Main gun').value,'gun_90');
  }finally{h.controller.dispose();}
});

test('the actual controller emits a part name string and the workshop accepts only current real parts',()=>{
  const h=harness();try{
    const selected=h.controller.selectPart('armament',true);assert(selected);assert.equal(typeof h.host.dispatched.at(-1).detail.part,'string');assert.equal(h.nodes.part.value,selected.name);
    h.host.dispatchEvent({type:'equipment-part-select',detail:{part:{name:'invalid'}}});assert.equal(h.nodes.part.value,selected.name);
    h.host.dispatchEvent({type:'equipment-part-select',detail:{part:'missing part'}});assert.equal(h.nodes.part.value,selected.name);
  }finally{h.controller.dispose();}
});

test('workshop export failures preserve the design and always clean up an attached anchor and created URL',()=>{
  const h=harness(),original=h.controller.exportGlb;try{
    h.change(h.nodes.part,'armament / standard weapon, mantlet and barrel');
    h.controller.exportGlb=()=>{throw Error('Export failed');};assert.doesNotThrow(()=>h.click('export'));assert.match(h.status.textContent,/could not be exported/);assert.equal(h.created.length,0);
    h.controller.exportGlb=()=>null;h.click('export');assert.match(h.status.textContent,/No tank model/);assert.equal(h.created.length,0);
    h.controller.exportGlb=original;h.failClick(true);assert.doesNotThrow(()=>h.click('export'));assert.match(h.status.textContent,/could not be exported/);assert.equal(h.anchors.at(-1).isConnected,false);assert.equal(h.timers.length,1);assert.deepEqual(h.revoked,[]);h.timers.shift()();assert.equal(h.revoked[0],h.created[0].url);
    h.failClick(false);h.click('export');assert.match(h.status.textContent,/download started/);assert.equal(h.anchors.at(-1).download,'spheres-tank_standard-olive.glb');assert.equal(h.anchors.at(-1).isConnected,false);h.timers.shift()();assert.equal(h.revoked.length,2);assert.equal(h.nodes.part.value,'armament / standard weapon, mantlet and barrel','Download errors do not discard the inspected part');
  }finally{h.controller.dispose();}
});
