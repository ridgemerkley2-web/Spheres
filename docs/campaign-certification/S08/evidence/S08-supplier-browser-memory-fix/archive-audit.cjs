// Harness-only bounded save audit. Each Python child owns exactly one parsed
// native world; the browser parent retains small facts and file descriptors.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const cp=require('node:child_process'),crypto=require('node:crypto');
const worker=path.join(__dirname,'archive-worker.py');
let output,selection={},sequence=0,record=()=>{};
function fileHash(file){
  const digest=crypto.createHash('sha256'),buffer=Buffer.allocUnsafe(1024*1024),fd=fs.openSync(file,'r');
  try{for(;;){const n=fs.readSync(fd,buffer,0,buffer.length,null);if(!n)break;digest.update(buffer.subarray(0,n));}}finally{fs.closeSync(fd);}
  return digest.digest('hex');
}
function configure(directory,telemetry){output=path.join(directory,'archive-audit');fs.mkdirSync(output);record=telemetry;}
function select(value){selection={...value};}
function child(args){
  const result=cp.spawnSync(process.env.SPHERES_AUDIT_PYTHON||'python',[worker,...args],{
    windowsHide:true,encoding:'utf8',maxBuffer:4*1024*1024+65536,timeout:120000});
  if(result.error)throw result.error;
  assert.equal(result.status,0,'Archive audit child failed: '+String(result.stderr).slice(0,4000));
  return JSON.parse(result.stdout);
}
function inspect(input,buyer,ignorePlan=false,requested=selection){
  assert(output,'Archive evidence directory was not configured');
  const stem=String(++sequence).padStart(3,'0')+'-'+path.basename(input,'.json')+(ignorePlan?'-except-maintenance-plan':'');
  const target=path.join(output,stem+'.canonical'),args=[input,target,buyer];
  for(const key of ['seller','company','product','revision'])if(requested[key]!=null)args.push('--'+key,String(requested[key]));
  if(ignorePlan)args.push('--ignore-maintenance-plan');
  record('archive-audit-start',{input,ignore_plan:ignorePlan});
  const result=child(args);
  assert.equal(result.input.path.toLowerCase(),path.resolve(input).toLowerCase());
  assert.equal(result.requested.buyer,buyer);
  fs.writeFileSync(path.join(output,stem+'.json'),JSON.stringify(result,null,2)+'\n',{flag:'wx'});
  record('archive-audit-complete',{input,input_sha256:result.input.sha256,canonical_bytes:result.canonical.bytes});
  return result;
}
function compare(left,right,message){
  assert.deepEqual(left.canonical.ignored_paths,right.canonical.ignored_paths,'Comparison omitted different native fields');
  const a=fs.openSync(left.canonical.path,'r'),b=fs.openSync(right.canonical.path,'r');
  const ab=Buffer.allocUnsafe(1024*1024),bb=Buffer.allocUnsafe(1024*1024);
  let count=0,equal=true;
  try{for(;;){const an=fs.readSync(a,ab,0,ab.length,null),bn=fs.readSync(b,bb,0,bb.length,null);
    if(an!==bn||!ab.subarray(0,an).equals(bb.subarray(0,bn))){equal=false;break;}
    count+=an;if(!an)break;
  }}finally{fs.closeSync(a);fs.closeSync(b);}
  const proof={message,equal,bytes_compared:count,left:left.canonical,right:right.canonical};
  if(!equal)proof.first_difference=child(['--compare',left.canonical.path,right.canonical.path]);
  record('exact-native-comparison',proof);
  assert(equal,message+'; bounded first difference: '+JSON.stringify(proof.first_difference));
}
function compareExceptMaintenance(left,right,message){
  const before=inspect(left.input.path,left.requested.buyer,true,left.requested);
  const after=inspect(right.input.path,right.requested.buyer,true,right.requested);
  assert.equal(before.input.sha256,left.input.sha256,'Earlier raw archive changed');
  assert.equal(after.input.sha256,right.input.sha256,'Later raw archive changed');
  compare(before,after,message);
}
module.exports={configure,select,inspect,compare,compareExceptMaintenance,fileHash};
