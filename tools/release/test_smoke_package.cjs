'use strict';
// Pure orchestration guards; these tests do not claim any native/browser run.
const {test}=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),os=require('node:os'),path=require('node:path'),crypto=require('node:crypto');
const smoke=require('./smoke_package.cjs');
test('browser allowlist admits only the single local server origin',()=>{
  const origin='http://127.0.0.1:12345';assert(smoke.allowedRequest(origin+'/api/state',origin));
  for(const url of ['https://example.com/a','http://127.0.0.1:12346/a','http://localhost:12345/a','http://user@127.0.0.1:12345/a','https://127.0.0.1:12345/a','file:///tmp/a','data:text/plain,x','not a URL'])assert.equal(smoke.allowedRequest(url,origin),false,url);
});
test('owned-path guard rejects root, siblings, absolute escapes and traversal',()=>{
  const root=path.resolve('owned');assert.equal(smoke.inside(root,path.join(root,'child')),path.join(root,'child'));
  for(const candidate of [root,path.resolve('elsewhere'),path.join(root,'../outside')])assert.throws(()=>smoke.inside(root,candidate));
});
test('archive comparison retains every native byte except terminal timestamp',t=>{
  const root=fs.mkdtempSync(path.join(os.tmpdir(),'spheres-smoke-test-'));t.after(()=>fs.rmSync(root,{recursive:true}));
  const file=path.join(root,'save.json'),prefix='{"format":"spheres-campaign","version":1,"world":{"cash":-0.0},"history":[],"log":[],"journey":{},"saved_date":"1990-01-01","player":"France"';
  fs.writeFileSync(file,prefix+',"saved_unix":1}');const first=smoke.archive(file);
  fs.writeFileSync(file,prefix+',"saved_unix":2}');assert.equal(smoke.archive(file).normalized,first.normalized);
  fs.writeFileSync(file,prefix.replace('-0.0','0.0')+',"saved_unix":2}');assert.notEqual(smoke.archive(file).normalized,first.normalized);
  fs.writeFileSync(file,prefix+',"saved_unix":1,"later":1}');assert.throws(()=>smoke.archive(file));
});
test('extracted executable identity refuses source binaries, wrong revisions and changed payloads',t=>{
  const out=fs.mkdtempSync(path.join(os.tmpdir(),'spheres-extraction-test-'));t.after(()=>fs.rmSync(out,{recursive:true}));
  const root=path.join(out,'extracted','package');fs.mkdirSync(root,{recursive:true});
  const rev='a'.repeat(40),platform=process.platform==='win32'?'windows':process.platform;
  const name=platform==='windows'?'spheres-web.exe':'spheres-web',bytes=Buffer.from('synthetic executable fixture');fs.writeFileSync(path.join(root,name),bytes);
  const report={format:'spheres-package-verification/v1',passed:true,extracted_root:root,manifest:{format:'spheres-release/v1',revision:rev,short_revision:rev.slice(0,12),platform,
    native_build_info:{full_revision:rev,revision:rev.slice(0,12)},executable:{path:name,bytes:bytes.length,sha256:crypto.createHash('sha256').update(bytes).digest('hex')}}};
  assert.equal(smoke.extractionIdentity(report,rev,out).binary,path.join(root,name));
  for(const mutate of [r=>r.passed=false,r=>r.manifest.revision='b'.repeat(40),r=>r.extracted_root=out,r=>r.manifest.executable.path='../outside']){
    const changed=structuredClone(report);mutate(changed);assert.throws(()=>smoke.extractionIdentity(changed,rev,out));
  }
  fs.appendFileSync(path.join(root,name),'changed');assert.throws(()=>smoke.extractionIdentity(report,rev,out));
});
test('asset check rejects error text and tiny placeholder images',()=>{
  assert.throws(()=>smoke.imageKind(Buffer.from('Not found'),'png'));
  assert.throws(()=>smoke.imageKind(Buffer.alloc(2048),'webp'));
  const png=Buffer.alloc(2048);Buffer.from([137,80,78,71,13,10,26,10]).copy(png);assert.doesNotThrow(()=>smoke.imageKind(png,'png'));
});
