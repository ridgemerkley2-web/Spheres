/* Original, physical character meshes. Person identity and appearance eras are
   explicit data, never inferred from a country, party colour or random seed. */
(function(root, factory) {
  if (typeof module === "object" && module.exports) module.exports = factory(require("./arsenal-models.js"), require("../data/person_models.json"));
  else root.PersonModels = factory(root.ArsenalModels, root.PersonModelData);
})(typeof globalThis !== "undefined" ? globalThis : this, function(art, data) {
  "use strict";
  const records = new Map((data?.characters || []).map(p => [p.id,p]));
  const rgb = hex => [0,2,4].map(i => parseInt(hex.slice(i,i+2),16)/255);
  const tint = (c,s) => c.map(v => Math.max(0,Math.min(1,v*s)));
  function build(id) {
    const record = records.get(id);
    if (!record || !art) return null;
    const p=record.profile, m=new art.Mesh(), parts=[];
    const skin=rgb(p.skin), hair=rgb(p.hair), suit=rgb(p.suit), shirt=rgb(p.shirt), tie=rgb(p.tie);
    const dark=rgb("272732"), gold=rgb("d9b577"), white=rgb("f8f0db");
    const part=(name,fn)=>{const first=m.pos.length/3;fn();parts.push({name,first,count:m.pos.length/3-first});};
    const ball=(x,y,z,rx,ry,rz,col,angle=0)=>{
      const small=Math.max(rx,ry,rz)<.07;
      m.save().move(x,y,z).rotZ(angle).scale(rx,ry,rz).soft(()=>m.ball(1,small?10:20,small?6:14,col)).restore();
    };
    // Rounded sections around a vertical axis, with exact authored silhouettes.
    const loft=(sections,col)=>m.soft(()=>{
      const n=32, rings=sections.map(([y,rx,rz,z=0])=>Array.from({length:n},(_,i)=>{
        const a=i/n*Math.PI*2;return [Math.cos(a)*rx,y,z+Math.sin(a)*rz];
      }));
      for(let j=0;j<rings.length-1;j++)for(let i=0;i<n;i++){
        const k=(i+1)%n;m.quad(rings[j][i],rings[j+1][i],rings[j+1][k],rings[j][k],col);
      }
      m.fan(rings[0],col);m.fan(rings[rings.length-1].slice().reverse(),col);
    });
    const line=(pts,r,col)=>{
      for(let i=0;i<pts.length-1;i++) {
        const a=pts[i],b=pts[i+1],dx=b[0]-a[0],dy=b[1]-a[1],dz=b[2]-a[2],len=Math.hypot(dx,dy,dz);
        m.save().move(...a).rotY(Math.atan2(dx,dz)*180/Math.PI).rotX(-Math.atan2(dy,Math.hypot(dx,dz))*180/Math.PI).soft(()=>m.tube(r,r,len,8,col)).restore();
      }
      pts.forEach(a=>ball(...a,r,r,r,col));
    };
    part("Shoes and legs",()=>{
      for(const side of [-1,1]) {
        const x=side*0.26;
        ball(x,0.14,0.11,0.23,0.15,0.4,dark);
        if(p.outfit==="skirt") {
          ball(x,0.76,0,0.15,0.65,0.16,tint(skin,0.92));
          line([[x,0.22,-0.13],[x,0.2,0.34]],0.036,dark);
        } else {
          m.save().move(x,0,0);loft([[0.26,0.2,0.19],[0.38,0.21,0.22],[1.05,0.19,0.22],[1.75,0.27,0.27],[2.02,0.25,0.25]],tint(suit,0.83));m.restore();
          line([[x,0.4,0.217],[x,0.96,0.224],[x,1.55,0.25]],0.01,tint(suit,1.18));
        }
      }
      if(p.outfit==="skirt")loft([[1.08,0.55,0.3],[1.16,0.56,0.32],[1.7,0.49,0.31],[2.17,0.46,0.29]],tint(suit,0.9));
    });
    part("Tailored jacket",()=>{
      loft([[1.8,0.44,0.25],[1.89,0.5,0.3],[2.32,0.48,0.3],[2.76,0.59,0.32],[3.01,0.62,0.25],[3.15,0.36,0.19],[3.17,0.21,0.17]],suit);
      for(const s of [-1,1]) {
        m.save().move(s*.59,2.9,0).rotZ(s*8);
        loft([[-1.02,.163,.175],[-.84,.175,.184],[-.52,.19,.195],[-.18,.219,.22],[0,.224,.217],[.12,.14,.14],[.14,.07,.1]],suit);
        m.restore();
        ball(s*0.75,1.88,0.055,0.165,0.12,0.17,shirt);
        ball(s*0.77,1.69,0.06,0.16,0.23,0.13,skin,s*7);
        ball(s*0.65,1.72,0.14,0.07,0.135,0.075,skin,s*-20);
        // Subtle knuckle grooves stay in geometry even in the exported file.
        for(let i=0;i<3;i++)line([[s*(0.71+i*0.052),1.65,0.176],[s*(0.715+i*0.052),1.57,0.163]],0.006,tint(skin,0.8));
      }
      m.tri([-.26,3.09,.348],[0,2.48,.348],[.26,3.09,.348],shirt);
      for(const s of [-1,1]) {
        m.quad([s*.28,3.08,.359],[s*.45,2.89,.335],[s*.23,2.69,.359],[s*.035,2.48,.359],tint(suit,1.18));
        m.tri([s*.07,3.16,.3],[s*.27,3.08,.366],[s*.16,2.91,.366],white);
        line([[s*.28,2.2,.309],[s*.43,2.2,.295]],.013,tint(suit,.72));
      }
      line([[0,2.45,.32],[0,1.92,.3]],.009,tint(suit,.78));
      for(const y of [2.35,2.08])ball(.055,y,.325,.035,.035,.018,p.pearls?gold:dark);
      if(!p.pearls) {
        ball(0,3.02,.37,.085,.082,.035,tie);
        m.quad([-.044,2.98,.38],[.044,2.98,.38],[.091,2.61,.38],[-.091,2.61,.38],tie);
        m.tri([-.091,2.61,.38],[.091,2.61,.38],[0,2.51,.38],tie);
      }
      // Folded pocket square and small lapel pin.
      m.tri([-.44,2.74,.307],[-.37,2.81,.305],[-.3,2.75,.327],white);
      ball(.33,2.87,.29,.031,.031,.014,gold);
    });
    part("Face and ears",()=>{
      ball(0,3.23,0,.2,.26,.19,skin);
      const w=p.head_width,j=p.jaw_width;
      loft([[3.35,.13,.16,.05],[3.42,j*.7,.3,.025],[3.53,j,.42,0],[3.73,w*.92,.48,0],[3.96,w,.5,0],[4.21,w*.98,.47,-.025],[4.43,w*.79,.36,-.035],[4.53,w*.42,.22,-.035],[4.57,.055,.07,-.035]],skin);
      for(const s of [-1,1]){
        ball(s*w*.98,3.91,-.015,.115,.19,.11,skin);
        ball(s*w*1.035,3.9,.066,.054,.1,.035,tint(skin,.8));
      }
      // Eyelids surround small convex eyes, with inset irises and highlights.
      for(const s of [-1,1]) {
        const x=s*w*.44,z=.454;
        ball(x,4.005,z,.147,.093,.045,tint(skin,.84),s*-5);
        ball(x,4.0,z+.027,.133,.075,.036,white,s*-5);
        ball(x-s*.009,4.003,z+.06,.055,.06,.018,rgb(p.eyes));
        ball(x-s*.009,4.003,z+.076,.029,.036,.009,dark);
        ball(x-s*.024,4.026,z+.083,.012,.013,.005,white);
        line([[x-s*.13,4.09,.46],[x,4.127,.463],[x+s*.14,4.096,.442]],p.brow,tint(hair,.73));
        line([[x-s*.12,3.896,.451],[x,3.876,.465],[x+s*.1,3.889,.447]],.007,tint(skin,.86));
      }
      ball(0,3.95,.474,.078,.17,.068,skin);
      ball(0,3.855,.46+p.nose*.47,.115,.084,p.nose*.49,skin);
      for(const s of [-1,1])ball(s*.075,3.817,.526,.029,.016,.036,tint(skin,.63));
      const lips=rgb(p.pearls?"b87873":"af7c69"),cy=3.64;
      line([[-.18,cy+p.smile,.413],[-.09,cy-.004,.462],[0,cy,.475],[.09,cy-.004,.462],[.18,cy+p.smile,.413]],.014,lips);
      ball(0,3.606,.442,.123,.025,.022,tint(skin,1.025));
      if(p.pearls)for(const s of [-1,1])ball(s*w*1.01,3.73,.075,.046,.053,.041,white);
    });
    part("Sculpted hair",()=>{
      const w=p.head_width;
      if(p.hair_style==="bouffant") {
        ball(0,4.41,-.11,w*1.07,.36,.5,hair);
        ball(-w*.7,4.29,-.18,.26,.34,.35,hair,18);
        ball(w*.7,4.31,-.17,.26,.35,.34,hair,-18);
        ball(-.19,4.49,.23,.45,.205,.28,tint(hair,1.12),-14);
        ball(.3,4.41,.26,.28,.18,.23,hair,24);
        for(let i=0;i<6;i++) {
          const x=-.43+i*.15;
          line([[x,4.56,.27],[x+.045,4.64,.12],[x+.05,4.63,-.04]],.011,tint(hair,1.2));
        }
      } else {
        // Receding styles leave the top of the actual head visible.
        ball(0,4.12,-.375,w*.94,.42,.19,hair);
        for(const s of [-1,1])ball(s*w*.89,4.21,-.15,.13,.23,.3,hair,s*10);
        if(p.hair_style!=="receding") {
          ball(-.15,4.48,-.025,w*.8,.15,.38,hair,-9);
          ball(.32,4.41,.02,.23,.18,.34,tint(hair,.92),21);
          for(let i=0;i<7;i++) {
            const x=-.46+i*.12;
            line([[x,4.47,.28],[x+.075,4.59,.035],[x+.13,4.5,-.25]],.009,tint(hair,1.18));
          }
        } else {
          for(const s of [-1,1])for(let i=0;i<3;i++)line([[s*w*.85,4.25-i*.08,.1],[s*w*.96,4.23-i*.08,-.07]],.01,tint(hair,1.12));
        }
      }
    });
    part("Personal accessories",()=>{
      if(p.pearls)for(let i=0;i<15;i++) {
        const a=Math.PI*i/14;
        ball(Math.cos(a)*.255,3.13-Math.sin(a)*.24,.39+Math.sin(a)*.018,.033,.034,.03,white);
      }
      if(p.handbag) {
        ball(-.95,1.28,.13,.26,.3,.12,rgb("31333a"));
        line([[-1.13,1.49,.14],[-1.1,1.71,.14],[-.91,1.82,.14],[-.75,1.67,.14],[-.76,1.49,.14]],.036,dark);
        line([[-1.13,1.4,.228],[-.78,1.4,.228]],.01,gold);
        ball(-.955,1.4,.25,.04,.03,.015,gold);
      }
    });
    const mesh=m.finish();
    mesh.id="person:"+id; mesh.parts=parts; mesh.triangleCount=mesh.positions.length/9;
    mesh.bounds={min:mesh.min,max:mesh.max};
    mesh.assetKind="character";
    mesh.description=record.credit;
    mesh.specification={person_id:record.person_id,appearance_from:record.from,appearance_until:record.to,status:record.status,sources:record.sources};
    return mesh;
  }
  return {build, ids:()=>[...records.keys()], meta:id=>records.get(id)||null};
});
