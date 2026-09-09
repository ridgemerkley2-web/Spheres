// Structural geometry regressions: exercise the actual recipes in a plain VM.
const {test}=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),crypto=require('node:crypto');
const source=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/arsenal-models.js'),'utf8');
const A=require('../../spheres-web/ui/arsenal-models.js');
const affected=new Set(['air_gen2','air_gen3','air_gen4','f15e','f22','ea18g','f35a','sixthgen','nav_patrol','nav_escort','nav_blue']);
const baseline=[
  {"id":"inf_light","hash":"eedd67608ab70daca07c22fbb4fc4b214ee60d7e4d9d397168086722be73887f"},
  {"id":"inf_mech","hash":"aecd22fe3c04bd8207080e43aff5253e9f9b98994bdbaee884c0d1e7a1843614"},
  {"id":"arm_gen2","hash":"689e6a27d568edadd126d02270a16dbf0c9da3d9ef8ba24d76b0782f8c9f1485"},
  {"id":"arm_gen3","hash":"819b7be463a5603b0df592e1acbde375df19a13efceba01ff97f392c6c89af08"},
  {"id":"trophy","hash":"e5d6cd4599d57c14dd60e33a81c9e7d00a97bd3f3ff20e51755fedf9ef11ada2"},
  {"id":"air_gen2","hash":"eb98416e6374f40169ebedd9fadc20792fda9ab15a1eaf05073b67130557d8b8"},
  {"id":"air_gen3","hash":"462bafa01255def7bd25df768f71365ebd207b6e818d93ee76d6d6629f50a9c9"},
  {"id":"air_gen4","hash":"49aab2cb213ec11f5c4d7e6936312ef99a65327d7402d70b141c41ea6a92ac4c"},
  {"id":"f15e","hash":"51f9ebf2dc4cd27f74a0a03b2b54baa926bf9a6a25bd0161d7c9f9ab52b14bc3"},
  {"id":"f117","hash":"f8460276bde848f4253dac604f3437b0bcb13096d7c3727f8761a61da71a1544"},
  {"id":"e3","hash":"71c4bca767163c82d298e428f34c35d81691c7c9bfda47f2272f0f4e51152afe"},
  {"id":"b2","hash":"aa0053ed960d7343921ba96ed3166019306954f70a898d16e65c2691c88da30c"},
  {"id":"predator","hash":"a47d491ac4ec3511baa8f9a9120e7c33163152df6d9f8c04d7d35daa6173c29b"},
  {"id":"mq1b","hash":"96da85d87119b86806b58bd98d5b10ce73f439dd625cbe3629d7a378a43b6b05"},
  {"id":"f22","hash":"ed1adabd21fcc09ca16583e13880150b032f1060a0a67655b81463c3b388fe39"},
  {"id":"ea18g","hash":"11f6c1615e7efd1a54e698169d31bcfc3112dd25c7d971a77532eb2f9b19f1ba"},
  {"id":"rq170","hash":"547e21a63a793fc84f5221624b4d5cbd0f450992ed30ee1894833930963ca6ea"},
  {"id":"f35a","hash":"0a2f00e92a615c6cf79718a309ce0c55109c28d72c55a069e0535f1de4669a00"},
  {"id":"cca","hash":"2e7c891d25b175bd929e494b56ad0d7e09c1d4d0181e15236434be7ddd0f40db"},
  {"id":"sixthgen","hash":"2ae7566d7d4b6e1f7f99f9351a3cb74979518e0f4cd57a3f128f1fbda886a708"},
  {"id":"aesa","hash":"8f29a94595610c3660fc3ee2897285c5eac362e319454862b9e971746b0d4f4e"},
  {"id":"nav_patrol","hash":"36462b881f4d9f4d1bae48a1e3205dc09d2766c547153cdaf9ad6924149f677c"},
  {"id":"nav_escort","hash":"5e5a260cfc8baef59a43b872e45b02a3d2b5b0e523540e55143e68ba77f701b1"},
  {"id":"nav_blue","hash":"c183aeb2ac5973bff7a6f8980c25b6bdac2e2ea6f965b297a278a7fd2d02cd92"},
  {"id":"la_ssn","hash":"31fd9d44bcd70a54fe796eedc416fbb40b8913b1d7ce9cc2b3ff3a790b5e1777"},
  {"id":"aip_ssk","hash":"6c52275a40d90d62ba896fa86c8483c219a85520b49aedc10d69f9de94364b31"},
  {"id":"laws","hash":"59e32a0c27cdb954aad86058ff968a770a350997e156f1ff72b37aafad8ae4a6"},
  {"id":"msl_sam","hash":"ea137e662e3571d4d4a15fa027ad7eb8a6f334f50557a252ebf690c3f649e690"},
  {"id":"msl_brm","hash":"e01b32f59a5f83215d1b99358baf846f9dceaaf950826017fd02deaf80bc781c"},
  {"id":"msl_deterrent","hash":"38c6181b4a792513efac57a3fd9b8f735327661d90e28fc66fffffa689612931"},
  {"id":"paveway","hash":"b3c865573211a2b9793b1db0c38cffaccab5861d7c8e080e3dfacff5719d7dce"},
  {"id":"tomahawk","hash":"7d40e327e113ebe5a3f5b15d44fd190187a36f753aeb19f920b32350c231b040"},
  {"id":"patriot","hash":"5ec4996abb41529b25cb3605d2ee27930fb56a7a301bb948662855c244114fe8"},
  {"id":"jdam","hash":"fafd8b169129c6665ebe812493e57f7ab3bc6e97ca4b586b7ca3e452578e0b3b"},
  {"id":"gbi","hash":"1dc93287e965f018ecbf4d4ba6a457ff602bfce67e9d3cc02c4fc5531da43b36"},
  {"id":"x51","hash":"c39262ce83f8673a0e0f3c5c0354bed3011480165cc1aa73a9fb2eb28eb180ec"},
  {"id":"hgv","hash":"3f4200a50d0f2378378e7637a65ae1b850d4d7bb63b982329b3eb6db1d2cce0c"},
  {"id":"owa","hash":"db9d41ce98c3a70d2df02953d151548a9d1103e88f7c835fdda81f1c8321b74f"},
  {"id":"raven","hash":"0d6f1d59327c4aec2f91ebff27952fef38e37fdaf0e0b317d724c4359a2cf33a"},
  {"id":"switchblade","hash":"cedc7592bc826f6aad5268f47167fc251a1ce9296f8c15cc2b86bcf1235ceac8"},
  {"id":"cuas","hash":"58ab6ff68cca63f7060a3e156e875e4b820d719e8e0a1bb5d1e7b023e583f121"},
  {"id":"link16","hash":"87e67526ca2c7838f52a0a420726a5e6ae903881fb250efd68cdc51068d04343"},
  {"id":"c4isr","hash":"178936ea5850f2b2a82e4c457a1f06d7fa2584755583f7e02f57e2e53484ac64"},
  {"id":"atr","hash":"57cdf0955ef44b31c1718579d99efbbad15b6fe8f6be7a9856efdfeeb0fb2376"},
  {"id":"spc_recon","hash":"5a37b259e7740e732528eff5b7667575f74183970085cae3e1ff3a24babb28e3"},
  {"id":"kh11","hash":"b5be96882acd1af0b08eb37b37ca61d37ea0b9490d18beb523d852eb9ec6ce05"}
];
const hash=g=>{const h=crypto.createHash('sha256');for(const a of [g.positions,g.normals,g.colors])h.update(Buffer.from(a.buffer,a.byteOffset,a.byteLength));return h.digest('hex');};
const context={module:{exports:{}}};
const marker='Mesh, primitives: { ring';assert.equal(source.split(marker).length,2);
vm.runInNewContext(source.replace(marker,'realism: { fairStations, hullShell, weatherDeck, glazingPanel, houseSkin, seaBoat, canopyGlass }, '+marker),context);
const P=context.module.exports, palette=P.palette;
function build(name,args,lod='near'){const m=new P.Mesh(lod);P.realism[name](m,...args);return m.finish();}
function vertices(g){const a=[];for(let i=0;i<g.positions.length;i+=3)a.push(Array.from(g.positions.slice(i,i+3)));return a;}
function close(a,b,why){assert(Math.abs(a-b)<1e-5,why+': '+a+' vs '+b);}
function hits(g,axis,u,v){
  const other=[0,1,2].filter(n=>n!==axis),result=[],p=g.positions;
  for(let i=0;i<p.length;i+=9){
    const a=Array.from(p.slice(i,i+3)),b=Array.from(p.slice(i+3,i+6)),c=Array.from(p.slice(i+6,i+9));
    const x=other[0],y=other[1],det=(b[y]-c[y])*(a[x]-c[x])+(c[x]-b[x])*(a[y]-c[y]);
    if(Math.abs(det)<1e-13)continue;
    const r=((b[y]-c[y])*(u-c[x])+(c[x]-b[x])*(v-c[y]))/det;
    const s=((c[y]-a[y])*(u-c[x])+(a[x]-c[x])*(v-c[y]))/det,t=1-r-s;
    if(r>=-1e-7&&s>=-1e-7&&t>=-1e-7)result.push({at:r*a[axis]+s*b[axis]+t*c[axis],color:Array.from(g.colors.slice(i,i+3))});
  }return result.sort((a,b)=>b.at-a.at);
}
function sameColor(a,b){return a.every((n,i)=>Math.abs(n-b[i])<1e-6);}

test('the eleven air/naval changes and explicitly pinned later mechanised revision retain their bounded scope',()=>{
  assert.deepEqual(A.ids(),baseline.map(r=>r.id));
  for(const row of baseline){const value=hash(A.build(row.id));
    if(row.id==='inf_mech')assert.equal(value,'5ece92ffe4420f168940c6c39b30683e6caed67cd5affba378bd7a1e05b47a88','later formation pass: see check_arsenal_inf_mech.cjs and its archived pre-pass buffers');
    else if(affected.has(row.id))assert.notEqual(value,row.hash,row.id+' must receive the structural pass');
    else assert.equal(value,row.hash,row.id+' is outside this pass');
  }
});

test('ship forebody waterlines narrow below the unchanged deck while amidships stays fixed',()=>{
  const o={len:100,beam:10,draft:3,freeboard:4,sheer:1};
  const st=[[0,.76,.26,.05],[.5,1,0,0],[.9,.68,.52,.34],[1,.06,1.12,.95]];
  const near=vertices(build('hullShell',[o,st])),far=vertices(build('hullShell',[o,st],'far'));
  const width=(verts,z,y)=>Math.max(...verts.filter(p=>Math.abs(p[2]-z)<1e-5&&Math.abs(p[1]-y)<1e-5).map(p=>Math.abs(p[0])));
  close(width(near,50,0),width(far,50,0),'midship waterline');
  close(width(near,90,4.52),width(far,90,4.52),'foredeck edge');
  assert(width(near,90,0)<width(far,90,0)*.85,'forebody narrows meaningfully at waterline');
  assert(width(near,90,4.52)/width(near,90,0)>1.15,'bow has real flare through its section');
});

test('the weather deck has a crown through the quarter beam, not two planar roof slopes',()=>{
  const o={len:20,beam:10,freeboard:2,camber:.2};
  const g=vertices(build('weatherDeck',[o,[[0,1,0,0],[1,1,0,0]],.955,2,palette.navyDeck]));
  const quarter=g.filter(p=>Math.abs(Math.abs(p[0])-2.3875)<1e-5);
  assert(quarter.length>0,'intermediate crown station');quarter.forEach(p=>close(p[1],2.15,'parabolic quarter-beam height'));
  g.filter(p=>Math.abs(p[0])<1e-6).forEach(p=>close(p[1],2.2,'centre datum'));
  g.filter(p=>Math.abs(Math.abs(p[0])-4.775)<1e-5).forEach(p=>close(p[1],2,'deck-edge datum'));
});

test('bridge panes are the first visible surfaces through framed openings on side and end walls',()=>{
  const o={w:10,z0:0,z1:30,y:0,h:6,tumble:.9,col:palette.navy,glass:palette.glass};
  const g=build('houseSkin',[o]);
  const y=4.01,z=3+17.4/8;
  const side=hits(g,0,y,z);assert(side.length>0);assert(sameColor(side[0].color,palette.glass),'solid hull skin must not cover the side pane');
  const end=hits(g,2,1.7,y);assert(end.length>0);assert(sameColor(end[0].color,palette.glass),'solid end cap must not cover the front pane');
  assert(end[0].at<30&&end[0].at>29.8,'front glass sits behind the structural surround');
  const panel=build('glazingPanel',[[[1,0,0],[1,2,0],[1,2,2],[1,0,2]],.1,palette.navy,palette.glass]);
  close(hits(panel,0,1,1)[0].at,.9,'recessed pane depth');
  close(hits(panel,0,.05,1)[0].at,1,'frame remains at the wall surface');
});

test('boarding boats have open wells, raised seats and rounded inflatable collars',()=>{
  const o={x:0,y:0,z:0,len:6,w:2};
  const near=build('seaBoat',[o]),far=build('seaBoat',[o],'far');
  const well=hits(near,1,0,2.04),lid=hits(far,1,0,2.04);
  assert(well.length&&lid.length);assert(well[0].at<.5,'ray reaches the cockpit sole');
  assert(lid[0].at>.9,'far model retains its old inexpensive lid');
  assert(hits(near,1,0,1.5)[0].at>well[0].at+.25,'bench stands over the open well');
  assert(hits(near,1,.9,1.2)[0].at>1,'inflatable collar is a rounded volume above the gunwale');
});

test('fighter fairing preserves authored frames and extrema without padding straight barrels',()=>{
  const st=[[0,.1,.2],[.25,.8,.6],[.5,1,1],[.75,1,1],[1,.6,.4]],saved=JSON.stringify(st);
  const result=JSON.parse(JSON.stringify(P.realism.fairStations(st)));
  assert.equal(JSON.stringify(st),saved,'input is immutable');
  st.forEach(s=>assert.deepEqual(result.find(p=>p[0]===s[0]),s));
  assert(result.length>st.length,'curved transitions gain authored intermediate shape');
  let nonlinear=false;
  result.forEach(p=>{if(st.some(s=>s[0]===p[0]))return;
    const i=st.findIndex((s,k)=>k+1<st.length&&p[0]>s[0]&&p[0]<st[k+1][0]);
    for(const axis of [1,2]){assert(p[axis]>=Math.min(st[i][axis],st[i+1][axis])&&p[axis]<=Math.max(st[i][axis],st[i+1][axis]));
      if(Math.abs(p[axis]-(st[i][axis]+st[i+1][axis])/2)>.0055)nonlinear=true;}
  });assert(nonlinear,'the extra frame must actually fair the skin');
  const straight=[[0,.2,.2],[.5,.6,.6],[1,1,1]];
  assert.deepEqual(JSON.parse(JSON.stringify(P.realism.fairStations(straight))),straight,'a straight taper does not receive empty subdivisions');
});

test('fighter canopy glass sits above a flat sill and has a separate raked forward screen',()=>{
  const g=build('canopyGlass',[{x:0,y:0,z:0,w:2,h:1,len:4}]);
  let glass=0,screen=0;
  for(let i=0;i<g.positions.length;i+=3){const c=Array.from(g.colors.slice(i,i+3));
    // Loft panels apply deterministic lighting tints; the glass chromaticity
    // remains stable through those tints, unlike the neutral frame material.
    const blue=Math.abs(c[0]/c[2]-palette.canopy[0]/palette.canopy[2])<1e-5
      &&Math.abs(c[1]/c[2]-palette.canopy[1]/palette.canopy[2])<1e-5;
    if(blue){glass++;assert(g.positions[i+1]>=-1e-6,'glazing cannot bulge under the cockpit sill');
      if(g.positions[i+2]>=3.2-1e-5){screen++;
        assert(Math.abs(g.positions[i+2]-3.2)<1e-5||Math.abs(g.positions[i+2]-4)<1e-5,'raked screen is a separate panel run');}}
  }assert(glass>0&&screen>0,'both hood and forward screen are present');
});
