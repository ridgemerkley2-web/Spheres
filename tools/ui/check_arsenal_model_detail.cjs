// Close-up mechanical detail contracts. No browser, server, or external assets.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const vm=require('node:vm');
const crypto=require('node:crypto');
const source=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/arsenal-models.js'),'utf8');
const models=require('../../spheres-web/ui/arsenal-models.js');
// Captured before this pass. These are compatibility fixtures, not output
// snapshots to regenerate when an unrelated recipe changes.
const baseline=[
  {"id":"inf_light","cls":"Infantry","span":4,"near":8118,"far":1224,"min":[-2.0057735443115234,0,-1.7991869449615479],"max":[1.9600000381469727,1.725000023841858,1.8337513208389282],"farHash":"c27321a7414258ee7d8d0931cbb95676cc74da5685d2b3eb4890749f8e021d5d"},
  {"id":"inf_mech","cls":"Infantry","span":6.7,"near":11108,"far":632,"min":[-1.6887860298156738,-0.05352136120200157,-0.10000000149011612],"max":[2.3899717330932617,2.824749708175659,6.800000190734863],"farHash":"a90be9635e44b7ced548a6d78ceeebdd5e83b27de3a96651cb772165dd08b681"},
  {"id":"arm_gen2","cls":"Armour","span":9,"near":8499,"far":1039,"min":[-1.9528440237045288,-0.0016008041566237807,-0.1289999932050705],"max":[1.9528440237045288,3.1445295810699463,7.945099830627441],"farHash":"663c0e5d027a452b66dd7d91f5b087c63a9bad35b7cc080bc26943b38f37452b"},
  {"id":"arm_gen3","cls":"Armour","span":11,"near":9407,"far":1081,"min":[-2.1857519149780273,-0.021448228508234024,-0.15800000727176666],"max":[2.1857519149780273,3.5752575397491455,10.133299827575684],"farHash":"a6090ad48b4d31f8da485b94577da4b381be0769a1bafca7fa4e27552bed81d4"},
  {"id":"trophy","cls":"Armour","span":11,"near":10147,"far":1153,"min":[-2.1857519149780273,-0.021448228508234024,-0.15800000727176666],"max":[2.1857519149780273,3.5752575397491455,10.133299827575684],"farHash":"696a86db9b16aa643e38a8965441e4477ab7b6fbeea56d40ad92b931cf5d9aa3"},
  {"id":"air_gen2","cls":"Air","span":15,"near":4796,"far":544,"min":[-3.5733745098114014,-0.9660429954528809,-0.30000001192092896],"max":[3.5733745098114014,3.2274999618530273,15],"farHash":"06ebdaf2dfa4de84997ee7469b85a8b60b19032939953649fa084df788c11c9b"},
  {"id":"air_gen3","cls":"Air","span":17.7,"near":5300,"far":820,"min":[-5.825577259063721,-1.3664065599441528,-0.3540000021457672],"max":[5.825577259063721,4.014999866485596,17.700000762939453],"farHash":"ca2520951a5e903bfb38c03711f3b05cf5982c5840c1a3e3ed7ff469441f1767"},
  {"id":"air_gen4","cls":"Air","span":19.4,"near":6124,"far":840,"min":[-6.525000095367432,-1.3028205633163452,-0.5988113880157471],"max":[6.525000095367432,4.07657527923584,19.399999618530273],"farHash":"fcecab25f061fed8c80bd970bf85116c898bd9e9c340f4a4b7ec1614c101e102"},
  {"id":"f15e","cls":"Air","span":19.4,"near":6632,"far":1004,"min":[-6.525000095367432,-1.4299999475479126,-0.5988113880157471],"max":[6.525000095367432,4.07657527923584,19.399999618530273],"farHash":"c1487cd9f2d3c75d46307ce91735907ce2a3b96e41a7152191a882f994317ba6"},
  {"id":"f117","cls":"Air","span":20.1,"near":4504,"far":952,"min":[-8.199999809265137,-1.2599999904632568,-0.4399999976158142],"max":[8.199999809265137,2.9791431427001953,20.35355567932129],"farHash":"a2e6ec631be4ebe167d6db455139444245ae031d81fadf81f2da23c21112f57b"},
  {"id":"e3","cls":"Air","span":44.4,"near":6040,"far":1004,"min":[-22.09551429748535,-3.3499999046325684,-0.11721044033765793],"max":[22.09551429748535,9.49734115600586,46.599998474121094],"farHash":"ac0852109a250f54544c6bd16ea3f22d14f12114e57baba779ec2831513aa3dc"},
  {"id":"b2","cls":"Air","span":52.4,"near":4094,"far":982,"min":[-26.200000762939453,-1.2699999809265137,-0.671999990940094],"max":[26.200000762939453,2.2100000381469727,21],"farHash":"8648e4c4e4afd289e4dc397fd811d107930bd72ec332260ab1017d4cbefa861a"},
  {"id":"predator","cls":"Air","span":14.8,"near":4036,"far":470,"min":[-7.296511650085449,-1.4471865892410278,-0.14712150394916534],"max":[7.296511650085449,0.8223999738693237,8.220000267028809],"farHash":"bc4c3d926e1f56dad881dc26c463932b3bb9c035b1c1355b1eacf9a2fe5ae2ca"},
  {"id":"mq1b","cls":"Air","span":16.8,"near":4756,"far":1070,"min":[-7.296511650085449,-1.4471865892410278,-0.14712150394916534],"max":[7.296511650085449,0.8223999738693237,8.220000267028809],"farHash":"b37d2fd6fa701fd0beec690aaf2b0470a24fa0951fb199451b39e9583b234ce6"},
  {"id":"f22","cls":"Air","span":13.56,"near":4528,"far":1220,"min":[-6.78000020980835,-1.2772986888885498,-0.800000011920929],"max":[6.78000020980835,3.5915842056274414,18.920000076293945],"farHash":"6f2f494b7b7af52a7dfe7d4116cac140007644b6a6d03172104e01292ad4141c"},
  {"id":"ea18g","cls":"Air","span":13.62,"near":5836,"far":840,"min":[-6.809999942779541,-1.7729798555374146,-0.4989469647407532],"max":[6.809999942779541,3.7254109382629395,18.309999465942383],"farHash":"46c606e26b5b9fdc56249f920f177ee638dd24cd030816ff67ad2ed83782c9aa"},
  {"id":"rq170","cls":"Air","span":20,"near":4030,"far":782,"min":[-10,-0.45500001311302185,-0.8100000023841858],"max":[10,0.878870964050293,4.5],"farHash":"86172a1e311304246de892385bbe45e1ec7ebd4f3542ef30fa270dbadf15394f"},
  {"id":"f35a","cls":"Air","span":10.7,"near":4452,"far":1186,"min":[-5.349999904632568,-1.3423558473587036,-0.4990817606449127],"max":[5.349999904632568,3.3471970558166504,15.699999809265137],"farHash":"1244c537f9f119ea4bb13ef2a4ff2eac1ba1e1c5b3b066b0f2015bc216973e74"},
  {"id":"cca","cls":"Air","span":8,"near":4284,"far":750,"min":[-4,-0.7486724257469177,-0.18000000715255737],"max":[4,1.4606629610061646,9],"farHash":"505927651af5d96afbfa3552b5daf5bba33bb697f3d84ac6c505ebc3ca73298e"},
  {"id":"sixthgen","cls":"Air","span":17,"near":4036,"far":1056,"min":[-8.5,-1.5002223253250122,-1],"max":[8.5,1.565999984741211,22],"farHash":"e882fb006d0a87fe0cc74bf9430e506df7b9ad615ce4fcdcb50fd9e7b430c027"},
  {"id":"aesa","cls":"Air","span":1.4,"near":5148,"far":708,"min":[-0.6200000047683716,0,-0.9610368609428406],"max":[0.6200000047683716,1.3742079734802246,0.5],"farHash":"48300b4d8f0b3962af7366512116fd3cf6cd7533e1346ff5216826108bc68c6c"},
  {"id":"nav_patrol","cls":"Naval","span":35,"near":6712,"far":766,"min":[-3.7264862060546875,-2.0199999809265137,-0.6499999761581421],"max":[3.726485013961792,11.772000312805176,35.0764274597168],"farHash":"293637a031a750e2df1829eca4909306ca115762e8543084433c308c694fefb1"},
  {"id":"nav_escort","cls":"Naval","span":133,"near":9948,"far":1146,"min":[-9.038630485534668,-5.159999847412109,-0.8050000071525574],"max":[9.038630485534668,29.464000701904297,133.13206481933594],"farHash":"2a3736fa856a2ad8c48979e5c9fa90050158e80673a67ba62cb88f3ed62434e9"},
  {"id":"nav_blue","cls":"Naval","span":333,"near":11162,"far":1266,"min":[-103.22740936279297,-12.220000267028809,-67.18456268310547],"max":[95.22740936279297,51.220001220703125,333.3443908691406],"farHash":"9cb1bea229cc115a8e1b88501d9dc8e0504c17ad05d294fa717f5346c0c933ca"},
  {"id":"la_ssn","cls":"Naval","span":110,"near":4658,"far":572,"min":[-7,-7,0],"max":[7,13.527999877929688,110],"farHash":"e78a92475c520ead9021dd0b70972acb47ab354bd95b26f7571710725b42f1e7"},
  {"id":"aip_ssk","cls":"Naval","span":56,"near":4346,"far":572,"min":[-5.703999996185303,-3.1426968574523926,0],"max":[5.703999996185303,8.03600025177002,56],"farHash":"d4be1b78b7fd0a4137663783218f267a4f049c16a4f2ae07454f0582e7005a4f"},
  {"id":"laws","cls":"Naval","span":3.4,"near":4372,"far":616,"min":[-1.3200000524520874,-5.817072218907302e-17,-1.483585000038147],"max":[1.3200000524520874,2.441494941711426,3.096583366394043],"farHash":"2d70626ad6e24bc1375674886694e20f663db095e42529bc5ebfd91f57593f8d"},
  {"id":"msl_sam","cls":"Missile","span":13.1,"near":7824,"far":680,"min":[-1.968000054359436,-0.024800000712275505,-0.024206504225730896],"max":[1.968000054359436,9.315406799316406,13.196000099182129],"farHash":"883e81b9e4d4039977f3da9a158267ce0a4e49bb2935122fddf3706eaa614ff4"},
  {"id":"msl_brm","cls":"Missile","span":13.4,"near":7030,"far":1003,"min":[-1.8450000286102295,-0.026399999856948853,-1.399999976158142],"max":[1.8450000286102295,12.461636543273926,13.489999771118164],"farHash":"9a8492791542cb1cc6d732734b18f9ed79ab579fe9dd9546c35aa8c9b8d760b3"},
  {"id":"msl_deterrent","cls":"Missile","span":21,"near":4546,"far":980,"min":[-6.300000190734863,-2.4000000953674316,-6.265488147735596],"max":[8.130249977111816,22.200000762939453,6.265488147735596],"farHash":"6403fa11fefb0ed2a4f28d3ea9a629f9bca9543142ff5be11b67dcf7482def4b"},
  {"id":"paveway","cls":"Missile","span":4.4,"near":4216,"far":1134,"min":[-0.8232499957084656,-0.8232499957084656,-0.15839999914169312],"max":[0.8232499957084656,0.8232499957084656,4.481599807739258],"farHash":"fd91d8ac55a651389a37def82083e40828c1c79ae92f268e73aadc13185e5be6"},
  {"id":"tomahawk","cls":"Missile","span":6.25,"near":4484,"far":1231,"min":[-1.6100000143051147,-0.5339999794960022,-0.00800000037997961],"max":[1.6100000143051147,0.5339999794960022,6.25],"farHash":"13edc6a824aaee2661056f69e788830a319ca6dd49afa016b6d9c90f023e03a4"},
  {"id":"patriot","cls":"Missile","span":12,"near":4292,"far":836,"min":[-1.8700000047683716,-0.06499999761581421,-0.17895954847335815],"max":[1.8700000047683716,6.5270843505859375,9.5],"farHash":"77b62e2221061af166542590ecc808de0edb72623781705e3b5ecd5fd844461e"},
  {"id":"jdam","cls":"Missile","span":3.84,"near":4848,"far":968,"min":[-0.5697982311248779,-0.6546000242233276,-0.41600000858306885],"max":[0.5697982311248779,0.33713212609291077,3.847599983215332],"farHash":"8a5d2e4ea3cef030a2bf9a0aab9af7e8f82b3be6af2c13b79545f617fd1b9809"},
  {"id":"gbi","cls":"Missile","span":16.8,"near":4106,"far":865,"min":[-2.4000000953674316,-0.949999988079071,-2.3897619247436523],"max":[2.4000000953674316,17.780000686645508,2.3897619247436523],"farHash":"fb860be03b871dd0e727525a0c9bda8d1183a4410e0ce07c9e7265deb7c73ee4"},
  {"id":"x51","cls":"Missile","span":7.6,"near":4448,"far":759,"min":[-0.7350391745567322,-0.7850391864776611,-3.4200000762939453],"max":[0.7350391745567322,0.9599999785423279,4.300000190734863],"farHash":"ddaf32f52ac4c938d76438f149ca1783df4c50b79e72374140771f9639c40100"},
  {"id":"hgv","cls":"Missile","span":5,"near":4102,"far":386,"min":[-1.3032039403915405,-0.5699999928474426,-0.7006799578666687],"max":[1.3032039403915405,0.5199999809265137,5.019999980926514],"farHash":"8784543ed45d0d537a72f5a225b220d5570b84e19161d0814a6713f9577c28a5"},
  {"id":"owa","cls":"Missile","span":2.5,"near":4126,"far":560,"min":[-1.3472537994384766,-0.5131950974464417,-0.2750000059604645],"max":[1.3472537994384766,0.42679306864738464,3.6424999237060547],"farHash":"f85902c9f6c65ce6dd8e63fcab8d71006c694eafa896d783ccbcdb5c7bfd354a"},
  {"id":"raven","cls":"Infantry","span":1.4,"near":4680,"far":460,"min":[-0.6650000214576721,-0.1759459674358368,-0.08299999684095383],"max":[0.6650000214576721,0.06800000369548798,0.9904000163078308],"farHash":"0b188490081c6e9289875d37100e22443a7aaf4cfeb8a5b7fcf809df1dec91de"},
  {"id":"switchblade","cls":"Infantry","span":1.2,"near":4162,"far":469,"min":[-0.30275529623031616,-0.07123646140098572,-0.06811581552028656],"max":[0.463040828704834,0.16839027404785156,0.5861714482307434],"farHash":"2b6504da14bad8ba231478e4ebf0b9d5c890080b733ea7a25039823dc985ac1e"},
  {"id":"cuas","cls":"Infantry","span":6.4,"near":6938,"far":1226,"min":[-1.3675999641418457,-0.03500000014901161,0],"max":[1.3675999641418457,2.72758412361145,5.437743663787842],"farHash":"40a4fbdf7cada62ca88545bcbf67ac74d08b815e0e3fe17a9e7d0c9e8216a832"},
  {"id":"link16","cls":"Infantry","span":2.2,"near":4132,"far":364,"min":[-0.8999999761581421,0,-0.699999988079071],"max":[0.9139180779457092,1.152832269668579,0.699999988079071],"farHash":"822f9d23eee5d983b0c46e7464eaedfc9177e8074440b3fc7b76f7b6f4d667dd"},
  {"id":"c4isr","cls":"Infantry","span":7.4,"near":5880,"far":504,"min":[-1.600000023841858,-0.02199999988079071,-0.05999999865889549],"max":[1.537500023841858,4.90749979019165,7.474999904632568],"farHash":"35976cd866340956146084cf02cb775a3cdfceb2f268a5ee927f4404c293e2d4"},
  {"id":"atr","cls":"Infantry","span":0.8,"near":4674,"far":558,"min":[-0.30000001192092896,0,-0.6399999856948853],"max":[0.30000001192092896,0.7300000190734863,0.30000001192092896],"farHash":"5e5d6ef35a35fbb6c36151423ba7b304e1f025e5d0d192ef604ca26b3b15002f"},
  {"id":"spc_recon","cls":"Space","span":12,"near":11032,"far":1354,"min":[-7.388054847717285,-3.285382032394409,-8.739998817443848],"max":[7.290287494659424,3.2829437255859375,2.583423614501953],"farHash":"00cd2132d5b4f2cf6263b60ec3a6609368142acd147e94d74938424299ae6179"},
  {"id":"kh11","cls":"Space","span":15,"near":9898,"far":876,"min":[-7.695000171661377,-2.5268304347991943,-5.389999866485596],"max":[7.695000171661377,2.982767105102539,9.805000305175781],"farHash":"f73cfbf9000bfa7067070401c6cd37c2bef42ef7b4c157379f7a138dddab86cf"}
];
const hash=(mesh)=>{const h=crypto.createHash('sha256');for(const a of [mesh.positions,mesh.normals,mesh.colors])h.update(Buffer.from(a.buffer,a.byteOffset,a.byteLength));return h.digest('hex');};
function inspected(){
  const context={module:{exports:{}},console};
  const marker='Mesh, primitives: { ring';
  assert.equal(source.split(marker).length,2,'private helper probe has one anchor');
  vm.runInNewContext(source.replace(marker,'details: { socketFastener, trackPin, nozzle, intakeDuct, rotorFace, jointBand, louvre, railRun }, '+marker),context);
  return context.module.exports;
}
const probes=inspected();
function detail(name,o,lod='near') {const m=new probes.Mesh(lod);probes.details[name](m,o);return m.finish();}
function vertices(mesh) {const out=[];for(let i=0;i<mesh.positions.length;i+=3)out.push(Array.from(mesh.positions.slice(i,i+3)));return out;}
function firstAxialHit(mesh){
  let nearest=Infinity;const p=mesh.positions;
  for(let i=0;i<p.length;i+=9){
    const a=[p[i],p[i+1],p[i+2]],b=[p[i+3],p[i+4],p[i+5]],c=[p[i+6],p[i+7],p[i+8]];
    const det=(b[1]-c[1])*(a[0]-c[0])+(c[0]-b[0])*(a[1]-c[1]);
    if(Math.abs(det)<1e-14)continue;
    const u=((b[1]-c[1])*(-c[0])+(c[0]-b[0])*(-c[1]))/det;
    const v=((c[1]-a[1])*(-c[0])+(a[0]-c[0])*(-c[1]))/det,w=1-u-v;
    if(u>=-1e-6&&v>=-1e-6&&w>=-1e-6){const z=u*a[2]+v*b[2]+w*c[2];if(z>=0)nearest=Math.min(nearest,z);}
  }return nearest;
}

test('all 46 IDs retain their catalogue metadata, exact near envelope and exact far buffers',()=>{
  assert.deepEqual(models.ids(),baseline.map(r=>r.id));
  for(const row of baseline){const near=models.build(row.id),far=models.build(row.id,null,{lod:'far'});
    assert.equal(near.span,row.span,row.id+' span');assert.equal(near.cls,row.cls,row.id+' class');
    assert.deepEqual(near.min,row.min,row.id+' minimum');assert.deepEqual(near.max,row.max,row.id+' maximum');
    assert.equal(far.count/3,row.far,row.id+' far triangle count');
    assert.equal(hash(far),row.farHash,row.id+' far buffers changed');
  }
});

test('every catalogue recipe gains close-up geometry within the 16k card and 40MiB total buffer budget',()=>{
  let nearBytes=0,farBytes=0;
  for(const row of baseline){const n=models.build(row.id),f=models.build(row.id,null,{lod:'far'});
    assert(n.count/3>row.near,row.id+' needs actual added close-up geometry');
    assert(n.count/3<=16000,row.id+' exceeds card budget');
    assert(f.count/3<=1500,row.id+' exceeds map budget');
    for(const a of [n.positions,n.normals,n.colors])nearBytes+=a.byteLength;
    for(const a of [f.positions,f.normals,f.colors])farBytes+=a.byteLength;
  }
  assert(nearBytes+farBytes<40*1024*1024,'all 92 CPU attribute-buffer sets remain under 40MiB');
});

test('fasteners have a recessed socket with an inward wall instead of a cap across the opening',()=>{
  const m=detail('socketFastener',{r:1,h:1,col:probes.palette.metal});
  const p=vertices(m);assert(p.some(v=>Math.abs(v[2]-.62)<1e-6),'recessed floor');
  assert(Math.abs(firstAxialHit(m)-.62)<1e-6,'the centre ray reaches the socket floor');
  let inward=0;
  for(let i=0;i<m.positions.length;i+=3){const x=m.positions[i],y=m.positions[i+1],z=m.positions[i+2];
    if(Math.abs(Math.hypot(x,y)-.25)<1e-5&&z>.62&&x*m.normals[i]+y*m.normals[i+1]<-.1)inward++;
  }
  assert(inward>=12,'socket walls point into the recess');
  assert(p.every(v=>Math.hypot(v[0],v[1])<=1.000001&&v[2]>=0&&v[2]<=1));
  assert.equal(detail('socketFastener',{r:1,h:1},'far').count,0);
});

test('track hinges bridge the belt width and retain hard planar end caps under smooth pin sides',()=>{
  const m=detail('trackPin',{w:2,r:.1,z:.2});
  assert(Math.abs(m.min[0]+1)<1e-6&&Math.abs(m.max[0]-1)<1e-6,'pin spans the shoe');
  assert(m.min[1]>=-.100001&&m.max[1]<=.100001,'pin stays inside hinge radius');
  let hardCaps=0,smoothSides=0;
  for(let i=0;i<m.normals.length;i+=3){const n=Array.from(m.normals.slice(i,i+3));
    if(Math.abs(n[0])>.999999)hardCaps++;
    else if(Math.abs(n[0])<1e-6&&Math.hypot(n[1],n[2])>.999999)smoothSides++;
  }
  assert(hardCaps>=24&&smoothSides>=36,'caps must not bend the radial side normals');
  assert.equal(detail('trackPin',{w:2,r:.1,z:.2},'far').count,0);
});

test('aircraft exhaust opens aft with visible recessed internals, while compressors are explicitly selected',()=>{
  const nozzle=detail('nozzle',{r:1,len:1});
  assert(firstAxialHit(nozzle)>.65&&firstAxialHit(nozzle)<.9,'open aft bore reaches the internal flameholder');
  const options={w:1,h:1,len:1,col:probes.palette.grey};
  const plain=detail('intakeDuct',options),fan=detail('intakeDuct',{...options,compressor:true});
  assert(fan.count>plain.count,'an authored conventional intake has compressor blades');
  assert.equal(hash(detail('intakeDuct',options,'far')),hash(detail('intakeDuct',{...options,compressor:true},'far')));
  // Instrument authoring choices without extending the public runtime API.
  const calls=[],context={module:{exports:{}},record:o=>calls.push(o)};
  vm.runInNewContext(source.replace('function rotorFace(m, o) {','function rotorFace(m, o) { if(!m.far) record(o);'),context);
  for(const id of ['f22','f35a','sixthgen','x51']){calls.length=0;context.module.exports.build(id);
    assert(calls.every(o=>o.open),id+' must not expose a compressor through a hidden S-duct or ramjet');}
  calls.length=0;context.module.exports.build('e3');assert.equal(calls.filter(o=>!o.open).length,4,'four AWACS nacelle compressors');
});

test('joint grooves replace a continuous band surface without changing its mounting envelope',()=>{
  const m=detail('jointBand',{r:1,w:.2,k:1.1,seg:16,z:0,col:probes.palette.metal});
  const p=vertices(m),at=(z)=>p.filter(v=>Math.abs(v[2]-z)<1e-6).map(v=>Math.hypot(v[0],v[1]));
  assert(at(.009).every(r=>Math.abs(r-1.035)<1e-5),'recessed centre gasket');
  assert(at(.056).every(r=>Math.abs(r-1.1)<1e-5),'retaining lip');
  assert(Math.abs(m.min[2]+.1)<1e-6&&Math.abs(m.max[2]-.1)<1e-6);
});

test('the mechanised AFV engine grille contains a spread bank, not coincident copies',()=>{
  const calls=[],context={module:{exports:{}},record:o=>calls.push(o)};
  vm.runInNewContext(source.replace('function louvre(m, o) {','function louvre(m, o) { if(!m.far) record(o);'),context);
  context.module.exports.build('inf_mech');assert.equal(calls.length,1);
  assert(Math.abs(calls[0].y1-calls[0].y0)>.5,'engine bank must have a real span');
});
