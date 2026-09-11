// Inventories original compiled exporter output only. Never runs or edits a game.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const option=key=>{const i=process.argv.indexOf(key);return i<0?null:process.argv[i+1];};
const output=option('--root');if(!output||!path.isAbsolute(output))throw Error('Supply --root with an absolute disposable export directory.');
const root=path.resolve(output),layout=JSON.parse(fs.readFileSync(path.join(__dirname,'active-fixture-layout.json'),'utf8'));
const hash=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const manifestFile=path.join(root,'active-fixture-manifest.json');
const listed=layout.fixtures.map(row=>{const bytes=fs.readFileSync(path.join(root,row.file));const value=JSON.parse(bytes);if(!value||typeof value!=='object'||(!value.world&&!value.nations))throw Error('Missing native saved world '+row.file);return {...row,bytes:bytes.length,sha256:hash(bytes)};});
if(process.argv.includes('--verify')){
  const manifest=JSON.parse(fs.readFileSync(manifestFile,'utf8'));
  if(manifest.source_revision!==layout.source_revision||JSON.stringify(manifest.fixtures)!==JSON.stringify(listed))throw Error('Original fixture bytes or coverage differ from the recorded manifest.');
  for(const source of manifest.source_binaries){if(hash(fs.readFileSync(source.path))!==source.sha256)throw Error('Original source binary changed: '+source.path);}
  console.log('Verified all 28 original fixtures and 3 recorded source binaries; no writes.');
}else{
  if(fs.existsSync(manifestFile))throw Error('The fixture manifest already exists. Use --verify; do not overwrite provenance.');
  if(option('--source-revision')!==layout.source_revision)throw Error('Require --source-revision '+layout.source_revision+' from the original pinned build evidence.');
  const source_binaries=['companies','ammunition','refits'].map(name=>{const file=option('--'+name+'-binary');if(!file||!path.isAbsolute(file))throw Error('Supply exact original --'+name+'-binary path.');const bytes=fs.readFileSync(file);return {name,path:file,bytes:bytes.length,sha256:hash(bytes)};});
  const manifest={format:'spheres-active-fixture-matrix',version:1,source_revision:layout.source_revision,source_binaries,exporters:layout.exporters,scope:layout.scope,fixtures:listed};
  fs.writeFileSync(manifestFile,JSON.stringify(manifest,null,2)+'\n',{flag:'wx'});
  console.log(JSON.stringify({manifest:manifestFile,sha256:hash(fs.readFileSync(manifestFile)),count:listed.length,source_binaries},null,2));
}
