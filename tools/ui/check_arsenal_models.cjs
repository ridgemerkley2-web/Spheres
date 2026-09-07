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

test('arsenal recipes are deterministic across fresh loads and reuse cached meshes',()=>{
  delete require.cache[require.resolve(file)];const fresh=require(file);
  assert.deepEqual(fresh.ids(),models.ids());
  for(const id of models.ids()){
    const a=models.build(id),b=fresh.build(id);assert.equal(models.build(id),a,`${id}: reuse cache`);
    for(const field of ['positions','normals','colors','min','max','size','centre'])assert.deepEqual(a[field],b[field],`${id}: deterministic ${field}`);
  }
});

test('OBJ exports retain every triangle and reference existing vertices and face normals',()=>{
  for(const id of models.ids()){
    const mesh=models.build(id),obj=models.toOBJ(id),lines=obj.trim().split('\n');
    const vertices=lines.filter(line=>line.startsWith('v ')),normals=lines.filter(line=>line.startsWith('vn ')),faces=lines.filter(line=>line.startsWith('f '));
    assert.equal(vertices.length,mesh.count,`${id}: OBJ vertices`);assert.equal(normals.length,mesh.count/3,`${id}: OBJ normals`);assert.equal(faces.length,mesh.count/3,`${id}: OBJ faces`);
    assert(lines.includes(`o ${id}`));
    for(const line of [...vertices,...normals])assert(line.split(/\s+/).slice(1).map(Number).every(Number.isFinite),`${id}: finite OBJ attributes`);
    const used=new Set();for(const line of faces){const corners=line.slice(2).split(' ');assert.equal(corners.length,3);for(const corner of corners){const match=/^(\d+)\/\/(\d+)$/.exec(corner);assert(match,`${id}: valid OBJ corner`);const vertex=Number(match[1]),normal=Number(match[2]);assert(vertex>=1&&vertex<=vertices.length);assert(normal>=1&&normal<=normals.length);used.add(vertex);}}
    assert.equal(used.size,vertices.length,`${id}: all vertices exported into faces`);
  }
});

test('unknown catalog ids fail cleanly or resolve only through an explicit known class',()=>{
  assert.equal(models.has('unknown-kit'),false);assert.equal(models.meta('unknown-kit'),null);assert.equal(models.build('unknown-kit'),null);assert.equal(models.toOBJ('unknown-kit'),null);
  for(const cls of new Set(models.ids().map(id=>models.meta(id).cls))){const mesh=models.build('unknown-kit',cls);assert(mesh,`${cls}: class fallback`);assert(models.has(mesh.id));assert.equal(mesh.cls.toLowerCase(),cls.toLowerCase());}
});
