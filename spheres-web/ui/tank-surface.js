/* Original armoured-vehicle presentation materials. The TankSurface API name
 * remains compatible with the tank viewer. Authored material classes survive
 * palette shading; no RGB guessing, network requests, simulation state or RNG.
 * Surface occupancy gives a bounded approximate contact bake, not ray tracing.
 * Finished RGB is ordinary vertex colour and survives the existing GLB export. */
(function(root,factory){const api=factory();if(typeof module==='object'&&module.exports)module.exports=api;else root.TankSurface=api;})(typeof globalThis!=='undefined'?globalThis:this,function(){
  'use strict';
  const classes=Object.freeze({paint:0,steel:1,track:2,rubber:3,glass:4,canvas:5,cable:6,lamp:7,unknown:255});
  const finishes=Object.freeze(Object.fromEntries(Object.entries({olive:[.270,.310,.205],sand:[.420,.365,.245],winter:[.500,.525,.490],woodland:[.235,.290,.195]}).map(([key,value])=>[key,Object.freeze(value)])));
  const wearLevels=Object.freeze({factory:0,service:.42,field:1});
  const cache=new WeakMap(),clamp=(v,a,b)=>Math.max(a,Math.min(b,v));
  const smooth=(a,b,x)=>{const t=clamp((x-a)/(b-a),0,1);return t*t*(3-2*t);};
  const mix=(a,b,t)=>a+(b-a)*t;
  const fract=x=>x-Math.floor(x);
  const hash=(x,y,z)=>{x=fract(x*.1031);y=fract(y*.1031);z=fract(z*.1031);const d=x*(y+33.33)+y*(z+33.33)+z*(x+33.33);return fract((x+y+2*d)*(z+d));};
  function noise(x,y,z){const ix=Math.floor(x),iy=Math.floor(y),iz=Math.floor(z),fx=smooth(0,1,x-ix),fy=smooth(0,1,y-iy),fz=smooth(0,1,z-iz);return mix(mix(mix(hash(ix,iy,iz),hash(ix+1,iy,iz),fx),mix(hash(ix,iy+1,iz),hash(ix+1,iy+1,iz),fx),fy),mix(mix(hash(ix,iy,iz+1),hash(ix+1,iy,iz+1),fx),mix(hash(ix,iy+1,iz+1),hash(ix+1,iy+1,iz+1),fx),fy),fz);}
  function supports(mesh,platform=mesh?.specification?.platform){return ['tank_standard','tank_heavy','tank_light','tank_destroyer','ground_ifv','ground_apc','ground_recon','ground_artillery','ground_air_defense'].includes(platform)&&mesh?.materialClasses instanceof Uint8Array&&mesh.materialClasses.length===mesh.positions?.length/3;}
  function validate(mesh){
    if(!mesh||!mesh.positions||!mesh.normals||!mesh.colors||!mesh.materialClasses||!mesh.positions.length||mesh.positions.length%9||mesh.positions.length>2700000||mesh.normals.length!==mesh.positions.length||mesh.colors.length!==mesh.positions.length||mesh.materialClasses.length!==mesh.positions.length/3)throw new TypeError('Tank surface requires matching triangle and authored material buffers.');
    for(const key of ['positions','normals','colors'])if(!mesh[key].every(Number.isFinite))throw new TypeError('Tank surface buffers must be finite.');
    if(!mesh.materialClasses.every(c=>Number.isInteger(c)&&(c>=0&&c<=7||c===255)))throw new TypeError('Unknown tank material class.');
  }
  function prepare(mesh){
    const existing=cache.get(mesh);if(existing)return existing;validate(mesh);
    const p=mesh.positions,n=mesh.normals,count=p.length/3,lo=[Infinity,Infinity,Infinity],hi=[-Infinity,-Infinity,-Infinity];
    for(let i=0;i<p.length;i++) {const k=i%3;lo[k]=Math.min(lo[k],p[i]);hi[k]=Math.max(hi[k],p[i]);}
    const span=hi.map((v,k)=>v-lo[k]);if(span.some(v=>!Number.isFinite(v)||v>1000))throw new RangeError('Tank surface bounds exceed the supported physical scale.');
    const cell=Math.max(.085,Math.max(...span)/112),origin=lo.map(v=>v-cell*3),dims=span.map(v=>Math.ceil(v/cell)+7),stride=dims[0]*dims[1],occupied=new Uint8Array(stride*dims[2]);
    const index=(x,y,z)=>{const a=Math.floor((x-origin[0])/cell),b=Math.floor((y-origin[1])/cell),c=Math.floor((z-origin[2])/cell);return a<0||b<0||c<0||a>=dims[0]||b>=dims[1]||c>=dims[2]?-1:a+b*dims[0]+c*stride;};
    // Rasterise triangle surfaces at sub-voxel spacing. The triangle and grid
    // limits bound work even for corrupt/untrusted external geometry. Tiny
    // fittings need one sample; broad armour plates receive a connected grid.
    const rasterBudget=2400000,stepsByTriangle=new Uint8Array(p.length/9);let requested=0;
    for(let i=0;i<p.length;i+=9){const edge=Math.max(Math.hypot(p[i]-p[i+3],p[i+1]-p[i+4],p[i+2]-p[i+5]),Math.hypot(p[i]-p[i+6],p[i+1]-p[i+7],p[i+2]-p[i+8]),Math.hypot(p[i+3]-p[i+6],p[i+4]-p[i+7],p[i+5]-p[i+8]));const steps=Math.min(192,Math.max(1,Math.ceil(edge/(cell*.70))));stepsByTriangle[i/9]=steps;requested+=(steps+1)*(steps+2)/2+1;}
    while(requested>rasterBudget){requested=0;for(let i=0;i<stepsByTriangle.length;i++){const steps=stepsByTriangle[i]=Math.max(1,Math.floor(stepsByTriangle[i]*.85));requested+=(steps+1)*(steps+2)/2+1;}}
    let samples=0;
    for(let i=0;i<p.length;i+=9){const ax=p[i],ay=p[i+1],az=p[i+2],bx=p[i+3]-ax,by=p[i+4]-ay,bz=p[i+5]-az,cx=p[i+6]-ax,cy=p[i+7]-ay,cz=p[i+8]-az;
      const steps=stepsByTriangle[i/9],center=index(ax+(bx+cx)/3,ay+(by+cy)/3,az+(bz+cz)/3);if(center>=0)occupied[center]=1;samples++;
      for(let u=0;u<=steps;u++)for(let v=0;v<=steps-u;v++){const at=index(ax+(bx*u+cx*v)/steps,ay+(by*u+cy*v)/steps,az+(bz*u+cz*v)/steps);if(at>=0)occupied[at]=1;samples++;}
    }
    const ao=new Float32Array(count),parameters=new Float32Array(count*4),seen=new Map();
    const materialParameters=[[.84,0],[.58,.90],[.67,.86],[.93,0],[.17,0],[.96,0],[.73,.70],[.28,0]];
    for(let i=0;i<count;i++){
      const at=i*3,x=p[at],y=p[at+1],z=p[at+2],length=Math.hypot(n[at],n[at+1],n[at+2])||1,nx=n[at]/length,ny=n[at+1]/length,nz=n[at+2]/length;
      // Deduplicate split vertices without merging opposite sides of a plate.
      const key=[Math.round(x/cell*5),Math.round(y/cell*5),Math.round(z/cell*5),Math.round(nx*12),Math.round(ny*12),Math.round(nz*12)].join(',');
      let value=seen.get(key);
      if(value===undefined){
        const tangent=Math.abs(ny)<.9?[-nz,0,nx]:[ny,-nx,0],tl=Math.hypot(...tangent)||1,tx=tangent[0]/tl,ty=tangent[1]/tl,tz=tangent[2]/tl,bx=ny*tz-nz*ty,by=nz*tx-nx*tz,bz=nx*ty-ny*tx;
        let blocked=0;
        for(let ray=0;ray<5;ray++){
          const u=ray===1?.72:ray===2?-.72:0,v=ray===3?.72:ray===4?-.72:0,dl=Math.hypot(1,u,v),dx=(nx+u*tx+v*bx)/dl,dy=(ny+u*ty+v*by)/dl,dz=(nz+u*tz+v*bz)/dl;
          // Start outside the raster shell so a flat plate cannot shadow itself.
          for(const distance of [1.9,2.7,3.5,4.3,5.1,5.9]){const idx=index(x+nx*cell*.65+dx*cell*distance,y+ny*cell*.65+dy*cell*distance,z+nz*cell*.65+dz*cell*distance);if(idx>=0&&occupied[idx]){blocked+=distance<3?.95:distance<4?.65:.35;break;}}
        }
        value=1-blocked/5*.30;seen.set(key,value);
      }
      ao[i]=value;const kind=mesh.materialClasses[i],preset=materialParameters[kind]||[.85,0];parameters.set([preset[0],preset[1],kind,value],i*4);
    }
    const result=Object.freeze({ao,parameters,min:lo,max:hi,stats:Object.freeze({vertices:count,uniqueSamples:seen.size,voxelCells:occupied.length,rasterSamples:samples,cellSize:cell})});cache.set(mesh,result);return result;
  }
  function bake(mesh,options={}){
    options=options&&typeof options==='object'?options:{};
    const data=prepare(mesh),finish=Object.hasOwn(finishes,options.finish)?options.finish:'olive',tone=finishes[finish],wear=Object.hasOwn(wearLevels,options.wear)?wearLevels[options.wear]:wearLevels.service;
    const out=new Float32Array(mesh.colors.length),p=mesh.positions,n=mesh.normals,c=mesh.colors,height=data.max[1]-data.min[1],baseOnly=options.baseOnly===true;
    for(let i=0;i<mesh.materialClasses.length;i++){
      const at=i*3,kind=mesh.materialClasses[i],x=p[at],y=p[at+1]-data.min[1],z=p[at+2],up=clamp(n[at+1],0,1),broad=noise(x*.90+11,y*.90+4,z*.90-9),detail=noise(x*5.4,y*5.4,z*5.4),source=c[at]*.25+c[at+1]*.65+c[at+2]*.10;
      let color=[c[at],c[at+1],c[at+2]],dust=0;
      if(kind===classes.paint){
        // Preserve only subtle authored plane variation: old edge/cap colours
        // must not turn an entire lid into bright cream after a sand repaint.
        const variation=clamp(1+(source-.32)*.20,.965,1.035);
        color=tone.map(v=>v*variation);
        if(!baseOnly)color=color.map(v=>v*(1+(broad-.5)*.035));
        if(finish==='woodland'&&!baseOnly){
          // Continuous metre-scale field, not triangle-index or part-index
          // alternation. Colours are sampled here so exports retain the finish.
          const patch=noise(x*.70+17,y*.70+1,z*.70-8)*.72+noise(x*1.55-8,y*1.55+13,z*1.55+5)*.28,dark=smooth(.575,.590,patch),brown=smooth(.370,.385,patch)*(1-smooth(.475,.490,patch));
          for(let k=0;k<3;k++)color[k]=mix(mix(color[k],[.255,.205,.140][k],brown*.96),[.120,.138,.105][k],dark*.98);
        }
        const low=1-smooth(.10,Math.min(1.50,Math.max(.8,height*.45)),y);
        dust=wear*(low*(.14+.19*broad)+up*(.045+.055*detail));
      }else if(kind===classes.steel||kind===classes.track||kind===classes.cable){
        const bright=clamp(source/.42,.30,1),tone=kind===classes.track?[.18,.18,.165]:kind===classes.cable?[.22,.225,.205]:[.28,.29,.27];
        color=tone.map(v=>v*(.78+bright*.22));
        // Track shoes pick up dry earth; exposed top contacts remain legible.
        dust=wear*(kind===classes.track?.30:.075)*(.4+.6*broad)*(1-up*.35);
      }else if(kind===classes.rubber){color=[.080,.087,.078].map(v=>v*clamp(source/.064,.65,1.12));dust=wear*.15*(.45+.55*broad);}
      else if(kind===classes.glass){color=[.045,.100,.108].map(v=>v*clamp(source/.30,.70,1.12));}
      else if(kind===classes.canvas){color=[.215,.225,.155].map(v=>v*(baseOnly?1:.94+.12*broad));dust=wear*.09*(.5+broad*.5);}
      if(baseOnly)dust=0;
      const occlusion=baseOnly||kind===classes.glass||kind===classes.lamp?1:data.ao[i];
      for(let k=0;k<3;k++)out[at+k]=clamp(mix(color[k],[.315,.282,.215][k],dust)*occlusion,0,1);
    }
    return out;
  }
  // Dedicated authored response. No RGB inference and no blue-sheet normal
  // map projected onto tank armour. Shared MilitarySurface remains unchanged.
  const glsl=`
uniform float uTankCamo;uniform float uTankWear;uniform float uTankHeight;
float tankHash(vec3 p){p=fract(p*.1031);p+=dot(p,p.yzx+33.33);return fract((p.x+p.y)*p.z);}
float tankNoise(vec3 p){vec3 i=floor(p),f=fract(p);f=f*f*(3.-2.*f);
 return mix(mix(mix(tankHash(i),tankHash(i+vec3(1,0,0)),f.x),mix(tankHash(i+vec3(0,1,0)),tankHash(i+vec3(1,1,0)),f.x),f.y),mix(mix(tankHash(i+vec3(0,0,1)),tankHash(i+vec3(1,0,1)),f.x),mix(tankHash(i+vec3(0,1,1)),tankHash(i+vec3(1,1,1)),f.x),f.y),f.z);}
vec3 tankFinishColor(vec3 rgb,vec3 p,vec3 n,vec4 surface){
 float kind=floor(surface.z+.5),broad=tankNoise(p*.90+vec3(11.,4.,-9.)),detail=tankNoise(p*5.4),up=clamp(n.y,0.,1.),dust=0.;
 if(kind<.5){rgb*=1.+(broad-.5)*.035;
  if(uTankCamo>.5){float patch=tankNoise(p*.70+vec3(17.,1.,-8.))*.72+tankNoise(p*1.55+vec3(-8.,13.,5.))*.28,dark=smoothstep(.575,.590,patch),brown=smoothstep(.370,.385,patch)*(1.-smoothstep(.475,.490,patch));rgb=mix(mix(rgb,vec3(.255,.205,.140),brown*.96),vec3(.120,.138,.105),dark*.98);}
  float low=1.-smoothstep(.10,min(1.50,max(.8,uTankHeight*.45)),p.y);dust=uTankWear*(low*(.14+.19*broad)+up*(.045+.055*detail));
 }else if(kind<2.5||abs(kind-6.)<.1){dust=uTankWear*(abs(kind-2.)<.1?.30:.075)*(.4+.6*broad)*(1.-up*.35);}
 else if(abs(kind-3.)<.1)dust=uTankWear*.15*(.45+.55*broad);
 else if(abs(kind-5.)<.1){rgb*=.94+.12*broad;dust=uTankWear*.09*(.5+broad*.5);}
 float ao=abs(kind-4.)<.1||abs(kind-7.)<.1?1.:surface.w;
 return mix(rgb,vec3(.315,.282,.215),dust)*ao;
}
vec3 tankFresnel(float cosine,vec3 f0){return f0+(1.-f0)*pow(1.-clamp(cosine,0.,1.),5.);}
vec3 tankBRDF(vec3 base,vec3 n,vec3 v,vec3 l,float rough,float metal,vec3 radiance){
 vec3 h=normalize(v+l);float nv=max(dot(n,v),.001),nl=max(dot(n,l),0.),nh=max(dot(n,h),0.);
 float a=rough*rough,a2=a*a,d=nh*nh*(a2-1.)+1.,distribution=a2/max(3.14159265*d*d,.00001);
 float k=(rough+1.)*(rough+1.)*.125,geometry=nv/(nv*(1.-k)+k)*nl/max(nl*(1.-k)+k,.001);
 vec3 f=tankFresnel(max(dot(h,v),0.),mix(vec3(.04),base,metal));
 return ((1.-f)*(1.-metal)*base/3.14159265+distribution*geometry*f/max(4.*nv*nl,.001))*radiance*nl;
}
vec3 tankLighting(vec3 rgb,vec3 normal,vec3 p,vec3 view,float visibility,vec4 surface){
 vec3 n=normalize(normal),v=normalize(view);if(dot(n,v)<0.)n=-n;
 float kind=floor(surface.z+.5),glass=abs(kind-4.)<.1?1.:abs(kind-7.)<.1?.3:0.;
 float rough=clamp(surface.x+uTankWear*.025,.14,.98),metal=clamp(surface.y,0.,1.);rgb=tankFinishColor(rgb,p,normal,surface);
 vec3 base=pow(max(rgb,vec3(.002)),vec3(2.2));
 vec3 lit=tankBRDF(base,n,v,normalize(vec3(-.55,.85,.65)),rough,metal,vec3(4.2,4.12,3.96))*visibility;
 lit+=tankBRDF(base,n,v,normalize(vec3(.7,.35,-.55)),rough,metal,vec3(1.35,1.42,1.50));
 float sky=smoothstep(-.55,.85,n.y);lit+=base*mix(vec3(.34,.355,.375),vec3(.62,.65,.69),sky)*(1.-metal*.50);
 vec3 r=reflect(-v,n);float strip=pow(max(dot(r,normalize(vec3(.15,.70,.70))),0.),mix(9.,65.,1.-rough));
 vec3 fresnel=tankFresnel(max(dot(n,v),0.),mix(vec3(.04),base,metal));
 lit+=fresnel*(.035+metal*(.20+sky*.25)+strip*mix(.12,.90,max(metal,glass)))*(1.-rough*.65);
 lit+=glass*vec3(.007,.012,.013)*sky;
 lit*=1.30;lit=clamp((lit*(2.51*lit+.03))/(lit*(2.43*lit+.59)+.14),0.,1.);
 return pow(lit,vec3(1./2.2));
}`;
  return Object.freeze({version:1,classes,finishes,wearLevels,supports,prepare,bake,glsl});
});
