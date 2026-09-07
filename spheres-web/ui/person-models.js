/* Original physical cartoon likeness studies. Geometry is authored per person;
   identity and appearance dates come exclusively from the explicit catalogue. */
(function(root, factory) {
  if (typeof module === "object" && module.exports) module.exports = factory(require("./arsenal-models.js"), require("../data/person_models.json"));
  else root.PersonModels = factory(root.ArsenalModels, root.PersonModelData);
})(typeof globalThis !== "undefined" ? globalThis : this, function(art, data) {
  "use strict";
  const records = new Map((data?.characters || []).map(p => [p.id,p]));
  const TAU=Math.PI*2, clamp=(x,a=0,b=1)=>Math.max(a,Math.min(b,x));
  const rgb=hex=>[0,2,4].map(i=>parseInt(hex.slice(i,i+2),16)/255);
  const tint=(c,s)=>c.map(v=>clamp(v*s));
  const mix=(a,b,t)=>a.map((v,i)=>v+(b[i]-v)*t);
  const add=(a,b)=>a.map((v,i)=>v+b[i]), sub=(a,b)=>a.map((v,i)=>v-b[i]);
  const mul=(a,s)=>a.map(v=>v*s);
  const cross=(a,b)=>[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]];
  const unit=a=>mul(a,1/(Math.hypot(...a)||1));
  const bell=(x,c,w)=>Math.exp(-(((x-c)/w)**2));
  // Catmull-Rom interpolation gives the sculpted outlines continuous tangents.
  function spline(points,t) {
    const f=clamp(t)*(points.length-1),i=Math.min(points.length-2,Math.floor(f)),u=f-i;
    const a=points[Math.max(0,i-1)],b=points[i],c=points[i+1],d=points[Math.min(points.length-1,i+2)];
    return b.map((v,k)=>.5*((2*v)+(-a[k]+c[k])*u+(2*a[k]-5*v+4*c[k]-d[k])*u*u+(-a[k]+3*v-3*c[k]+d[k])*u*u*u));
  }
  function build(id) {
    const record=records.get(id);if(!record||!art)return null;
    const p=record.profile,m=new art.Mesh(),parts=[];
    // Allocate the same ~100k envelope across different wardrobes and hairstyles.
    const sampling=p.pearls?.74:p.hair_style==="receding"?.80:.775;
    const skin=rgb(p.skin),hair=rgb(p.hair),suit=rgb(p.suit),shirt=rgb(p.shirt),tie=rgb(p.tie);
    const dark=rgb("252831"),gold=rgb("d0ab67"),white=rgb("eee9dd"),lip=rgb(p.pearls?"b26c6d":"b37e6c");
    const part=(name,fn)=>{const first=m.pos.length/3;fn();parts.push({name,first,count:m.pos.length/3-first});};
    const triangle=(a,b,c,ca,cb=ca,cc=ca)=>{
      // Sphere poles and collapsed patch tips have one triangle, never filler faces.
      if(Math.hypot(...cross(sub(b,a),sub(c,a)))<1e-10)return;
      const first=m.col.length;m.tri(a,b,c,ca);
      for(let i=0;i<3;i++){m.col[first+i]=ca[i];m.col[first+3+i]=cb[i];m.col[first+6+i]=cc[i];}
    };
    const surface=(nu,nv,point,color)=>m.soft(()=>{
      nu=Math.max(4,Math.round(nu*sampling));nv=Math.max(3,Math.round(nv*sampling));
      const rows=Array.from({length:nv+1},(_,j)=>Array.from({length:nu+1},(_,i)=>{
        const q=point(i/nu,j/nv);return {q,c:typeof color==="function"?color(q,i/nu,j/nv):color};
      }));
      for(let j=0;j<nv;j++)for(let i=0;i<nu;i++){
        const a=rows[j][i],b=rows[j][i+1],c=rows[j+1][i+1],d=rows[j+1][i];
        triangle(a.q,b.q,c.q,a.c,b.c,c.c);triangle(a.q,c.q,d.q,a.c,c.c,d.c);
      }
    });
    const ball=(x,y,z,rx,ry,rz,col,nu=24,nv=16)=>surface(nu,nv,(u,v)=>{
      const a=u*TAU,b=v*Math.PI;return [x+Math.sin(a)*Math.sin(b)*rx,y-Math.cos(b)*ry,z+Math.cos(a)*Math.sin(b)*rz];
    },col);
    // A continuous swept tube: smooth joints, tapered ends and no beads at knots.
    const curve=(points,r,col,steps=24,sides=8)=>{
      const at=typeof points==="function"?points:t=>spline(points,t),radius=t=>typeof r==="function"?r(t):r;
      let lastV=-1,q,a,b,rad;
      surface(sides,steps,(u,v)=>{
        // Every point in a cross-section shares its path evaluation and frame.
        if(v!==lastV){
          lastV=v;q=at(v);rad=radius(v);
          const t=unit(sub(at(Math.min(1,v+.0001)),at(Math.max(0,v-.0001))));
          // Transport the preceding frame instead of flipping reference axes
          // midway through a strand, eyelid, finger or handbag handle.
          const projected=a?sub(a,mul(t,a.reduce((sum,c,i)=>sum+c*t[i],0))):null;
          a=projected&&Math.hypot(...projected)>.001?unit(projected):unit(cross(Math.abs(t[2])>.92?[0,1,0]:[0,0,1],t));b=cross(t,a);
        }
        const ang=u*TAU;
        return add(q,mul(add(mul(a,Math.cos(ang)),mul(b,Math.sin(ang))),rad));
      },col);
    };
    const loft=(sections,col,nu=48,nv=32,deform=null)=>{
      const at=t=>spline(sections,t);
      surface(nu,nv,(u,v)=>{
        const [y,rx,rz,z=0]=at(v),a=u*TAU;
        const q=[Math.sin(a)*rx,y,z+Math.cos(a)*rz];return deform?deform(q,a,v):q;
      },col);
      // End discs use a centre fan; no zero-area triangles along the axis.
      for(const end of [0,1])m.soft(()=>{
        const [y,rx,rz,z=0]=at(end),c=[0,y,z];
        for(let i=0;i<nu;i++){
          const a=i/nu*TAU,b=(i+1)/nu*TAU,q=[Math.sin(a)*rx,y,z+Math.cos(a)*rz],r=[Math.sin(b)*rx,y,z+Math.cos(b)*rz];
          if(end)triangle(c,q,r,col);else triangle(c,r,q,col);
        }
      });
    };
    // A raised cloth panel with a rolled perimeter and a subtly convex face.
    const panel=(outline,col,bulge=.012,project=null)=>{
      const center=outline.reduce((a,b)=>add(a,mul(b,1/outline.length)),[0,0,0]);
      surface(outline.length*12,8,(u,v)=>{
        const f=u*outline.length,i=Math.min(outline.length-1,Math.floor(f)),t=f-i;
        const edge=mix(outline[i],outline[(i+1)%outline.length],t),q=mix(center,edge,v);
        q[2]=(project?project(q):q[2])+bulge*Math.sin(Math.PI*v)+.006;return q;
      },col);
      const edge=[...outline,outline[0]];
      curve(project?t=>{const q=spline(edge,t);q[2]=project(q)+.006;return q;}:edge,.0045,tint(col,.93),outline.length*10,6);
    };
    const jacketSections=[[1.83,.447,.25],[1.87,.49,.289],[2.05,.502,.309],[2.37,.469,.30],[2.69,.56,.316],[2.94,.602,.285],[3.075,.49,.221],[3.17,.209,.161]];
    const jacketLookup=Array.from({length:513},(_,i)=>spline(jacketSections,i/512));
    const coatFront=(x,y,offset)=>{
      let a=0,b=512;while(b-a>1){const i=(a+b)>>1;if(jacketLookup[i][0]<y)a=i;else b=i;}
      const [,rx,rz]=mix(jacketLookup[a],jacketLookup[b],clamp((y-jacketLookup[a][0])/(jacketLookup[b][0]-jacketLookup[a][0])));
      return rz*Math.sqrt(Math.max(0,1-(x/rx)**2))+offset;
    };
    const clothPanel=(outline,col,bulge=.012,offset=.023)=>panel(outline,col,bulge,q=>coatFront(q[0],q[1],offset));
    part("Shoes and legs",()=>{
      for(const side of [-1,1]) {
        const x=side*.255;
        ball(x,.075,.12,.221,.07,.385,tint(dark,.67),36,12);
        ball(x,.17,.12,.218,.12,.369,dark,40,20);
        curve([[x-.19,.105,.27],[x,.106,.485],[x+.19,.105,.27]],.008,tint(dark,1.5),28,6);
        if(p.outfit==="skirt") {
          m.save().move(x,0,0);
          loft([[.2,.108,.105],[.38,.112,.118],[.62,.14,.14],[.84,.158,.145],[1.23,.142,.15]],tint(skin,.96),40,32);m.restore();
          curve([[x-.17,.21,.18],[x,.25,.1],[x+.17,.21,.18]],.014,dark,24,8);
        } else {
          m.save().move(x,0,0);
          loft([[.26,.192,.177],[.33,.211,.207],[.47,.195,.196],[.99,.18,.215],[1.52,.224,.243],[1.96,.26,.26]],tint(suit,.88),48,42,(q,a,v)=>{
            const fold=.007*Math.sin(v*42+a*2)*bell(v,.08,.12)+.005*Math.sin(v*37-a*2)*bell(v,.46,.12);
            q[0]+=Math.sin(a)*fold;q[2]+=Math.cos(a)*fold+.007*bell(Math.sin(a),0,.12)*Math.max(0,Math.cos(a));return q;
          });m.restore();
          for(let k=0;k<4;k++)curve([[x-.052,.254+k*.009,.13+k*.033],[x+.052,.254+k*.009,.13+k*.033]],.006,tint(dark,1.5),8,6);
          curve([[x-.16,.185,.36],[x,.223,.365],[x+.16,.185,.36]],.005,tint(dark,1.4),24,6);
        }
      }
      if(p.outfit==="skirt"){
        loft([[1.07,.548,.295],[1.11,.558,.311],[1.32,.536,.318],[1.7,.478,.299],[2.1,.463,.285]],tint(suit,.91),64,44,(q,a,v)=>{
          q[2]+=.005*Math.cos(a*8)*Math.sin(v*Math.PI);return q;
        });
        curve([[-.38,1.105,.229],[0,1.105,.315],[.38,1.105,.229]],.005,tint(suit,.8),36,6);
      }
    });
    part("Tailored jacket",()=>{
      loft(jacketSections,suit,64,48,(q,a,v)=>{
        const fold=.008*Math.sin(a*6+v*20)*bell(v,.38,.13);q[0]+=Math.sin(a)*fold;q[2]+=Math.cos(a)*fold;return q;
      });
      for(const s of [-1,1]){
        m.save().move(s*.595,2.91,0).rotZ(s*8);
        loft([[-1.04,.146,.156],[-.98,.159,.166],[-.75,.168,.18],[-.49,.185,.195],[-.28,.207,.202],[0,.221,.21],[.12,.13,.128]],suit,48,40,(q,a,v)=>{
          const folds=.012*Math.sin(v*49+a*2)*bell(v,.46,.15);q[0]+=Math.sin(a)*folds;q[2]+=Math.cos(a)*folds;return q;
        });m.restore();
        m.save().move(s*.74,0,.019);loft([[1.82,.141,.146],[1.88,.145,.15],[1.935,.143,.148]],shirt,32,6);m.restore();
        for(let b=0;b<3;b++)ball(s*.841,1.98+b*.055,.108,.018,.018,.014,tint(dark,1.1),12,8);
        clothPanel([[s*.21,2.27,.32],[s*.431,2.27,.256],[s*.437,2.22,.258],[s*.21,2.22,.32]],tint(suit,1.035),.004);
      }
      surface(40,32,(u,v)=>{const y=2.44+v*.73,x=(u*2-1)*(.009+v*.191);return [x,y,coatFront(x,y,.014)];},shirt);
      for(const s of [-1,1]){
        clothPanel([[s*.19,3.15,.107],[s*.394,2.964,.238],[s*.327,2.887,.29],[s*.426,2.906,.234],[s*.223,2.62,.32],[s*.024,2.425,.344]],tint(suit,1.1),.016);
        clothPanel([[s*.038,3.187,.166],[s*.195,3.15,.107],[s*.133,2.965,.304],[s*.071,3.07,.262]],white,.008,.03);
      }
      curve([[.02,2.43,.342],[.054,2.22,.33],[.048,1.88,.3]],.004,tint(suit,.75),28,6);
      for(const y of [2.35,2.09]){
        ball(.055,y,.327,.027,.027,.011,p.pearls?gold:dark,24,12);
        if(!p.pearls)for(const x of [.049,.061])ball(x,y,.338,.0028,.0028,.0016,shirt,8,6);
      }
      if(!p.pearls){
        clothPanel([[-.041,3.06,.268],[-.065,3.016,.291],[0,2.946,.317],[.064,3.016,.291],[.04,3.06,.268]],tie,.012,.037);
        clothPanel([[-.035,2.96,.307],[-.077,2.607,.339],[0,2.533,.346],[.077,2.607,.339],[.035,2.96,.307]],tie,.009,.029);
        for(let i=0;i<5;i++)curve(t=>{const x=-.045+t*.086,y=2.7+i*.041-t*.026;return [x,y,coatFront(x,y,.044)];},.0025,tint(tie,1.27),10,6);
      }
      clothPanel([[-.432,2.744,.276],[-.4,2.823,.28],[-.358,2.78,.304],[-.334,2.816,.308],[-.298,2.75,.322]],white,.006);
      curve([[-.438,2.738,.276],[-.301,2.743,.326]],.006,tint(suit,.78),16,6);
      ball(.319,2.954,.33,.022,.024,.009,gold,20,12);
    });
    part("Hands and fingers",()=>{
      for(const s of [-1,1]){
        m.save().move(s*.758,1.77,.028).rotZ(s*6);
        ball(0,-.005,.012,.124,.13,.089,skin,32,22);
        for(let i=0;i<4;i++){
          const x=(i-1.5)*.061,len=[.166,.196,.18,.139][i];
          curve([[x,-.049,.005],[x,-.127,.024],[x+.006,-.073-len,.05],[x+.003,-.084-len,.095]],t=>.029*Math.pow(Math.sin(Math.PI*(.06+t*.94)),.28),skin,30,12);
          ball(x+.003,-.063-len,.111,.019,.025,.005,mix(skin,white,.26),16,10);
          curve([[x-.02,-.113,.065],[x,-.117,.07],[x+.02,-.113,.065]],.0025,tint(skin,.8),12,6);
        }
        curve([[-s*.1,.046,.011],[-s*.158,-.027,.064],[-s*.156,-.113,.114],[-s*.127,-.143,.118]],t=>.041*Math.pow(Math.sin(Math.PI*(.12+.88*t)),.33),skin,32,14);
        ball(-s*.133,-.123,.15,.022,.027,.006,mix(skin,white,.24),16,10);
        m.restore();
      }
    });
    const w=p.head_width,j=p.jaw_width;
    const headSections=[[3.34,.06,.105,.025],[3.385,j*.47,.259,.023],[3.45,j*.85,.345,.015],[3.58,j,.412,0],[3.78,w*.935,.465,0],[4,w,.477,-.018],[4.23,w*.965,.46,-.04],[4.415,w*.765,.353,-.05],[4.53,w*.42,.2,-.05],[4.575,0,0,-.05]];
    // Surface displacement blends the nose, orbital hollows and cheekbones into
    // the same continuous skin mesh. The face is not a pile of primitive balls.
    const faceRelief=(x,y)=>{
      const eyes=p.pearls?.047:.038;
      return p.cheek_fullness*(bell(x,-w*.51,.2)+bell(x,w*.51,.2))*bell(y,3.837,.115)
        -.032*(bell(x,-w*.43,.135)+bell(x,w*.43,.135))*bell(y,4.01,.073)
        +eyes*(bell(x,-w*.43,.17)+bell(x,w*.43,.17))*bell(y,4.12,.077)
        +p.nose*.39*bell(x,0,p.nose_bridge_width)*bell(y,3.995,.185)
        +p.nose*.75*bell(x,0,p.nose_tip_width)*bell(y,3.866,.071)
        +.044*(bell(x,-.088,.045)+bell(x,.088,.045))*bell(y,3.827,.048)
        +.027*bell(x,0,.177)*bell(y,3.673,.088)+.022*bell(x,0,.18)*bell(y,3.469,.065);
    };
    // Find the same interpolated skin surface by height for attached facial details.
    const headLookup=Array.from({length:1025},(_,i)=>spline(headSections,i/1024));
    const headAtY=y=>{
      let a=0,b=1024;while(b-a>1){const i=(a+b)>>1;if(headLookup[i][0]<y)a=i;else b=i;}
      return mix(headLookup[a],headLookup[b],clamp((y-headLookup[a][0])/(headLookup[b][0]-headLookup[a][0])));
    };
    const front=(x,y,offset=0)=>{
      const [,rx,rz,z]=headAtY(y),c=Math.sqrt(Math.max(0,1-(x/rx)**2));
      return z+rz*c+faceRelief(x,y)*c**5+offset;
    };
    const faceLine=(points,r,col,steps=32)=>curve(t=>{
      const [x,y,d=.003]=spline(points,t);return [x,y,front(x,y,d)];
    },r,col,steps,8);
    part("Face and ears",()=>{
      loft([[3.08,.181,.153],[3.19,.184,.159],[3.34,.176,.169],[3.44,.165,.172]],skin,48,24);
      surface(144,104,(u,v)=>{
        const [y,rx,rz,z]=spline(headSections,v),a=u*TAU,x=Math.sin(a)*rx,c=Math.cos(a);
        return [x,y,z+c*rz+Math.max(0,c)**5*faceRelief(x,y)];
      },q=>{
        const [x,y,z]=q,blush=(bell(x,-w*.54,.18)+bell(x,w*.54,.18))*bell(y,3.82,.11)*clamp(z*3);
        return mix(skin,rgb("ca8979"),blush*.2);
      });
      for(const s of [-1,1]){
        m.save().move(s*w*.99,3.915,-.045).rotY(s*55);
        ball(0,0,0,.095,.183,.072,skin,32,26);
        ball(s*.014,.007,.054,.049,.118,.021,mix(skin,rgb("b47063"),.4),24,20);
        curve([[0,-.146,.054],[s*.065,-.07,.068],[s*.073,.061,.05],[s*.02,.145,.032],[-s*.025,.102,.049]],.014,skin,40,10);
        curve([[s*.01,-.09,.074],[s*.041,.008,.077],[s*.01,.075,.074]],.01,tint(skin,1.04),24,8);
        ball(-s*.016,-.039,.079,.021,.029,.018,skin,18,12);
        if(p.pearls)ball(0,-.164,.063,.037,.043,.034,white,24,16);
        m.restore();
      }
    });
    part("Eyes and expression",()=>{
      for(const s of [-1,1]){
        const x=s*w*p.eye_spacing,ey=4.012,ew=p.eye_width,eh=p.eye_height;
        // An almond patch sits inside the socket. Both lids meet at its corners.
        const eye=(u,v)=>{
          const xx=(u*2-1)*ew,shape=Math.pow(Math.max(0,1-(xx/ew)**2),.72);
          const y=ey+((v*2-1)*eh+(s*xx)*.04)*shape;
          return [x+xx,y,front(x+xx,y,.008)+.018*shape*Math.sin(v*Math.PI)];
        };
        surface(40,20,eye,white);
        const iz=front(x,ey,.031),iris=rgb(p.eyes),ir=Math.min(.039,eh*.9);
        ball(x-s*.008,ey,iz,ir*.97,ir,.006,iris,40,16);
        // Radial iris fibres are coloured geometry; they remain in GLB exports.
        surface(64,5,(u,v)=>{const a=u*TAU,r=ir*(.41+v*.51);return [x-s*.008+Math.sin(a)*r,ey+Math.cos(a)*r,iz+.0065-v*.003];},(q,u,v)=>tint(iris,.7+.36*Math.sin(u*TAU*23)**2+.2*v));
        ball(x-s*.008,ey,iz+.008,ir*.44,ir*.51,.004,dark,24,14);
        ball(x-s*(.008+ir*.3),ey+ir*.38,iz+.012,ir*.18,ir*.18,.002,white,12,8);
        for(const upper of [false,true]){
          const pts=Array.from({length:17},(_,i)=>eye(i/16,upper?1:0));
          curve(pts,t=>.004+(upper?.01:.006)*Math.sin(t*Math.PI),upper?tint(skin,.94):skin,40,10);
        }
        faceLine([[x-ew,4.075],[x-ew*.4,4.096],[x+ew*.6,4.088],[x+ew,4.067]],.0035,tint(skin,.83),32);
        faceLine([[x-ew*.8,3.942],[x,3.934],[x+ew*.9,3.952]],.0026,tint(skin,.82),30);
        const browY=p.pearls?4.141:4.139;
        faceLine([[x-ew,browY-.02],[x-ew*.2,browY+.012],[x+ew*.6,browY+.005],[x+ew*1.07,browY-.02]],t=>.003+p.brow*.38*Math.sin(t*Math.PI),tint(hair,.7),40);
        for(let i=0;i<10;i++){
          const xx=x-ew*.84+i*ew*.175,yy=browY+.012*Math.cos((i-4)/7);
          faceLine([[xx,yy-.006,.008],[xx+s*.012,yy+.009,.007]],.0019,tint(hair,.94),6);
        }
        // Small recessed nostril slits and restrained nasolabial folds.
        faceLine([[s*.054,3.814,.004],[s*.08,3.81,.004],[s*.099,3.823,.003]],.005,mix(skin,dark,.28),24);
        faceLine([[s*.139,3.806,.003],[s*.172,3.75,.003],[s*.196,3.695,.003]],.0027,tint(skin,.83),28);
      }
      const mouthY=x=>3.648+p.smile*(Math.abs(x)/p.mouth_width)**1.8;
      for(const upper of [true,false])surface(64,12,(u,v)=>{
        const x=(u*2-1)*p.mouth_width,fade=Math.sin(u*Math.PI),mid=mouthY(x);
        const bow=.014*bell(Math.abs(x),.041,.025)+.01;
        const y=mid+(upper?bow:-.028)*p.lip_fullness*Math.sin(v*Math.PI/2)*fade;
        return [x,y,front(x,y,.004+Math.sin(v*Math.PI)*.012*fade)];
      },upper?lip:mix(lip,skin,.19));
      faceLine(Array.from({length:9},(_,i)=>{const x=(i/8*2-1)*p.mouth_width;return [x,mouthY(x),.006];}),t=>.0015+.002*Math.sin(t*Math.PI),mix(lip,dark,.35),40);
      faceLine([[-.021,3.746],[0,3.735],[.021,3.746]],.002,tint(skin,.87),24);
      for(let i=0;i<3;i++)faceLine([[-.22,4.255+i*.056],[0,4.266+i*.052],[.21,4.254+i*.057]],t=>.0008+.0008*Math.sin(t*Math.PI),tint(skin,.9),40);
    });
    part("Sculpted hair",()=>{
      const bouffant=p.hair_style==="bouffant",bald=p.hair_style==="receding",sidepart=p.hair_style==="sidepart";
      // A fitted scalp shell with a person-specific hairline and sculpted flow.
      const hairPoint=(u,v)=>{
        const a=bald?1.12+u*(TAU-2.24):u*TAU,frontness=(Math.cos(a)+1)/2;
        const temples=!bouffant&&!bald?(sidepart?.18:.3)*bell(Math.abs(Math.sin(a)),.66,.21)*frontness**6:0;
        const edge=1.94-.83*frontness-temples+(sidepart?.1*Math.sin(a):0)+(bouffant?.13*Math.sin(a*2+.4)*frontness**4:0);
        const start=.72+.35*frontness;
        const theta=bald?start+v*(edge-start):v*edge;
        const roll=bouffant?.04*Math.sin(a*5+theta*3)*Math.sin(theta)**2:.003*Math.sin(a*14+theta*5);
        const wave=bouffant?.009*Math.cos(a*13-theta*7):.0025*Math.cos(a*25+theta*(sidepart?9:15));
        const skinY=4.071+Math.cos(theta)*.504;
        const [,rx,rz,z]=headAtY(skinY);
        const edgeFade=bald?clamp(Math.min(u,1-u,v,1-v)*18):clamp((1-v)*18);
        const thickness=.003+(bouffant?.132:.027)*edgeFade;
        const volume=bouffant?.038*Math.sin(theta)*Math.cos(a+theta):(!bald?.016*Math.sin(theta)*Math.sin(a-theta*2):0);
        const sweptRoll=bouffant?frontness**6*bell(theta,.91+.12*Math.sin(a),.31):0;
        const relief=(roll+wave)*edgeFade*Math.sin(theta)**2;
        return [Math.sin(a)*(rx+Math.sin(theta)*(thickness+relief)),skinY+Math.cos(theta)*(thickness+relief)+volume+.105*sweptRoll,z+Math.cos(a)*(rz+Math.sin(theta)*(thickness+relief))+.07*sweptRoll];
      };
      surface(bald?112:144,bald?64:72,(u,v)=>hairPoint(1-u,v),(q,u,v)=>{
        const stripe=Math.sin(u*TAU*(bouffant?13:25)+v*11);
        return tint(hair,.94+.035*stripe+.035*clamp((q[1]-4.2)*2));
      });
      const strands=bouffant?64:bald?60:72;
      for(let i=0;i<strands;i++){
        const u=(i+.5)/strands;
        const pts=t=>{
          const v=.14+t*.84,flow=(bouffant?.039:sidepart?.027:.046)*Math.sin(v*Math.PI);
          const q=hairPoint(bald?clamp(u+flow):(u+flow)%1,v);return add(q,mul(unit(sub(q,[0,4.071,-.055])),.006));
        };
        curve(pts,t=>(bouffant?.0035:.0023)*Math.sin(Math.PI*t)**.5,tint(hair,i%3===0?1.08:.99),32,6);
      }
      if(sidepart){
        curve(Array.from({length:20},(_,i)=>{const q=hairPoint(.085+i*.0014,.18+i/19*.8);q[1]+=.004;return q;}),.005,tint(hair,.57),40,6);
      }
    });
    part("Personal accessories",()=>{
      if(p.pearls)for(let i=0;i<21;i++){
        const a=Math.PI*i/20,x=Math.cos(a)*.258,y=3.128-Math.sin(a)*.235;ball(x,y,coatFront(x,y,.04),.026,.027,.025,white,24,16);
      }
      if(p.handbag){
        m.save().move(-.955,0,.12);
        loft([[1.007,.197,.082],[1.037,.239,.105],[1.23,.235,.112],[1.472,.199,.078],[1.495,.18,.071]],rgb("303137"),48,28);
        panel([[-.187,1.46,.079],[-.212,1.39,.097],[0,1.326,.12],[.212,1.39,.097],[.187,1.46,.079]],rgb("37383e"),.01);
        curve([[-.171,1.475,0],[-.16,1.69,0],[0,1.79,0],[.16,1.69,0],[.171,1.475,0]],.024,dark,48,10);
        curve([[-.194,1.39,.105],[0,1.333,.13],[.194,1.39,.105]],.0035,gold,36,6);
        ball(0,1.372,.145,.032,.023,.009,gold,24,16);
        for(const s of [-1,1])ball(s*.167,1.472,.024,.026,.028,.015,gold,20,12);
        m.restore();
      }
    });
    // Authored head proportions apply to skin, eyes, ears and hair together.
    // Transform before finish so the normals are recomputed for the final sculpt.
    for(let i=1;i<m.pos.length;i+=3)if(m.pos[i]>3.3)m.pos[i]=3.3+(m.pos[i]-3.3)*p.face_height;
    const mesh=m.finish();
    mesh.id="person:"+id;mesh.parts=parts;mesh.triangleCount=mesh.positions.length/9;
    mesh.bounds={min:mesh.min,max:mesh.max};mesh.assetKind="character";
    mesh.portraitPivot=[0,3.3+.735*p.face_height,.03];
    mesh.description=record.credit;
    mesh.specification={person_id:record.person_id,appearance_from:record.from,appearance_until:record.to,status:record.status,sources:record.sources,detail_tier:"100k",geometry_revision:2};
    return mesh;
  }
  return {build,ids:()=>[...records.keys()],meta:id=>records.get(id)||null};
});
