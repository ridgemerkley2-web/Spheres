// Tiny synthetic save shapes exercise only the harness, not a native campaign.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const audit=require(path.resolve(__dirname,'../../integration/tools/ui/supplier-archive-audit.cjs'));
const root=fs.mkdtempSync(path.join(__dirname,'helper-probes-')),events=[];
audit.configure(root,(event,detail)=>events.push({event,...detail}));
const source=maintenance=>({world:{nations:[{id:'Tonga',tech:{level:1},treasury:-0,
  equipment:{revisions:{},learned:[],...(maintenance?{maintenance_plan:maintenance}:{})},
  arsenal:{held:[{design_id:null,units:50},{design_id:'test-import',units:2}]}}],
  companies:{firms:[],imports:{contracts:[]}}}});
function save(name,value){const file=path.join(root,name+'.json');fs.writeFileSync(file,JSON.stringify(value));return file;}
const before=audit.inspect(save('before',source()),'Tonga');
const after=audit.inspect(save('after',source({daily_limit_bn:.000002,from_day:123})),'Tonga');
assert.equal(before.buyer.held_by_revision['test-import'],2);
assert.equal(Object.hasOwn(before.buyer.held_by_revision,'null'),false);
audit.compareExceptMaintenance(before,after,'Only the reviewed plan changes');
assert.throws(()=>audit.compare(before,after,'The full worlds differ'),/bounded first difference/);
const altered=source({daily_limit_bn:.000002,from_day:123});altered.world.nations[0].treasury=5;
const other=audit.inspect(save('unrelated-cash-change',altered),'Tonga');
assert.throws(()=>audit.compareExceptMaintenance(before,other,'Cash must not change'),/treasury/);
audit.compare(before,before,'Unmodified canonical bytes compare exactly');
fs.writeFileSync(path.join(root,'result.json'),JSON.stringify({passed:true,synthetic_harness_only:true,
  checks:['null legacy holding retained outside custom totals','exact full-world mismatch detected',
    'only selected maintenance plan excluded','unrelated cash change still fails with a bounded path',
    'equal canonical files checked byte-for-byte'],events},null,2)+'\n');
console.log('Archive audit helper probes passed: '+root);
