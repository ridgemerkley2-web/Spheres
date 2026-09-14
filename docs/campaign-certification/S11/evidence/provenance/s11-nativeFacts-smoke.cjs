const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm'),assert=require('node:assert/strict'),cp=require('node:child_process');
const base=__dirname,source=fs.readFileSync(path.join(base,'s11-nativeFacts-patch.cjs'),'utf8');
const fixture=path.join(base,'evidence','S11-final-native-fixture'),manifest=JSON.parse(fs.readFileSync(path.join(fixture,'manifest.json')));
const context=vm.createContext({cp,assert,process,requiredOutcomes:manifest.required_outcomes});
vm.runInContext(source.slice(source.indexOf('function nativeFacts('),source.indexOf('\nfunction inspect(')),context);
const files=[path.join(fixture,'before.json'),path.join(base,'evidence/S11-browser-final-browser/france-h7r1uO/server/captures/s11-audit-loaded.json'),path.join(fixture,manifest.expected_stages.retire)];
const results=files.map(file=>{const facts=context.nativeFacts(file);return {file,holdings:facts.holdings.length,ledgers:facts.ledgers,envelope:facts.envelope};});
assert.equal(results[0].envelope.sha256,results[1].envelope.sha256);assert.equal(results[2].ledgers.refit.completed_units,1);
fs.writeFileSync(path.join(base,'s11-nativeFacts-smoke-result.json'),JSON.stringify({passed:true,results},null,2));console.log(JSON.stringify({passed:true,files:files.length,wrapper_counts:results.map(x=>x.envelope.wrapper_count)}));
