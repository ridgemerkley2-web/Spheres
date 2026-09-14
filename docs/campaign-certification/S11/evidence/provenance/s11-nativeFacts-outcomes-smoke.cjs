const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),assert=require('node:assert/strict'),cp=require('node:child_process');
const base=__dirname,source=fs.readFileSync(path.join(base,'s11-nativeFacts-patch.cjs'),'utf8');
const fixture=path.join(base,'evidence/S11-final-native-fixture'),manifest=JSON.parse(fs.readFileSync(path.join(fixture,'manifest.json'))),output=path.join(base,'evidence/S11-browser-reader-debug');
const run=path.join(output,fs.readdirSync(output).find(x=>x.startsWith('france-'))),e=JSON.parse(fs.readFileSync(path.join(run,'result.json')));
const context=vm.createContext({cp,assert,process,requiredOutcomes:manifest.required_outcomes,copy:x=>JSON.parse(JSON.stringify(x)),near:(a,b,label)=>assert(Math.abs(a-b)<1e-9,label)});
for(const [start,end] of [['function nativeFacts(','\nfunction inspect('],['function verifyOutcomes(','\nasync function groundReading(']]){const first=source.indexOf(start),last=source.indexOf(end,first);vm.runInContext(source.slice(first,last),context);}
for(const stage of e.stages){const s=JSON.parse(fs.readFileSync(path.join(run,stage.id+'-state.json')));stage.as_of_day=(Date.UTC(s.year,s.month-1,s.day)-Date.UTC(1990,0,1))/86400000;}
for(const stage of [e.stages[0],e.stages.at(-1)])assert.equal(stage.as_of_day,context.nativeFacts(stage.archive.path).as_of_day);
const outcomes=context.verifyOutcomes(e,manifest);fs.writeFileSync(path.join(base,'s11-nativeFacts-outcomes-smoke-result.json'),JSON.stringify({passed:true,diagnostic_only:true,original_result:path.join(run,'result.json'),outcomes},null,2));console.log(JSON.stringify({passed:true,stages:e.stages.length,days:outcomes.days_advanced,returned_units:outcomes.returned_target_units,ammunition_consumed_delta:outcomes.ammunition_consumed_delta}));
