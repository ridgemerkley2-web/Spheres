// The downloadable GLBs contain the same triangles as the interactive avatars.
const fs=require('node:fs'),path=require('node:path');
const models=require('../../spheres-web/ui/person-models.js');
const {glb}=require('../../spheres-web/ui/equipment-export.js');
const dir=path.resolve(__dirname,'../../spheres-web/ui/person-models');
if(!process.argv.includes('--check'))fs.mkdirSync(dir,{recursive:true});
for(const id of models.ids()){
  const mesh=models.build(id),bytes=Buffer.from(glb(mesh,models.meta(id).name));
  const file=path.join(dir,id+'.glb');
  if(process.argv.includes('--check')){
    if(!fs.existsSync(file)||!fs.readFileSync(file).equals(bytes))throw new Error('Rebuild character model: '+id);
  }else fs.writeFileSync(file,bytes);
  console.log(id+': '+mesh.triangleCount+' triangles, '+bytes.length+' bytes');
}
