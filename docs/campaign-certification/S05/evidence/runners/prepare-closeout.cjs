// Generate four staged documentation files only. Never edits the repository.
// Run after filling final-record.json: node prepare-closeout.cjs final-record.json [repository]
const fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process'),assert=require('node:assert/strict');
const candidate='db9d17c8d726aa102aa143ceb3599009c558ffee';
const recordPath=path.resolve(process.argv[2]||path.join(__dirname,'final-record.json'));
const repository=path.resolve(process.argv[3]||path.join(__dirname,'../../integration'));
const record=JSON.parse(fs.readFileSync(recordPath,'utf8'));
assert.equal(record.runtime_candidate,candidate,'Update the templates and record together if runtime changes');
assert.equal(record.qualification_gates_passed,true,'Do not complete S04/S05 while any required runtime gate is failing or unverified');
assert.equal(record.decisions?.S04,'complete','S04 must have an explicit completed gate decision');
assert.equal(record.decisions?.S05,'complete','S05 must have an explicit completed gate decision');
assert(['earned','withheld'].includes(record.decisions?.G1),'G1 needs a separate explicit decision');
assert.match(record.completed_date||'',/^\d{4}-\d{2}-\d{2}$/);
assert(record.gate_reason?.trim(),'Record the reason for the G1 decision');
assert(record.evidence_index?.trim(),'Record the final source-bound evidence index');
const git=cp.spawnSync('git',['rev-parse','HEAD'],{cwd:repository,encoding:'utf8',windowsHide:true});
assert.equal(git.status,0,git.stderr);assert.equal(git.stdout.trim(),candidate,'Generate from the reviewed runtime commit before its documentation commit');
const replacements={...record.replacements,S04_STATUS:'Complete',S05_STATUS:'Complete'};
const required=new Set();
const rendered=[];
for(const id of ['S04','S05']){
  const relative=`docs/campaign-certification/${id}/README.md`;
  const template=fs.readFileSync(path.join(__dirname,relative),'utf8');
  for(const match of template.matchAll(/\{\{([A-Z0-9_]+)\}\}/g))required.add(match[1]);
  const result=template.replace(/\{\{([A-Z0-9_]+)\}\}/g,(_,key)=>{
    const value=replacements[key];assert(typeof value==='string'&&value.trim()&&value.trim()!=='PENDING',`Fill ${key} from the actual final evidence`);return value;
  });
  rendered.push([relative,result]);
}
const metadataPath='docs/planning/campaign-pathway.json';
const metadataText=fs.readFileSync(path.join(repository,metadataPath),'utf8');
const metadata=JSON.parse(metadataText);
assert.equal(metadata.sessions.find(s=>s.id==='S06')?.status,'planned','S06 must remain unstarted');
for(const id of ['S04','S05']){
  const session=metadata.sessions.find(s=>s.id===id);assert(session,`Missing ${id}`);
  session.status='complete';session.completed_date=record.completed_date;
  session.evidence=`../campaign-certification/${id}/README.md`;session.runtime_candidate=candidate;
}
metadata.last_completed_session='S05';metadata.next_session='S06';
metadata.execution={...(metadata.execution||{}),authorized_through:'S05',stop_boundary:'S05',status:'stopped',
  stopped_after:'S05',next_session_requires_instruction:true};
metadata.gate_decisions={...(metadata.gate_decisions||{}),G1:{status:record.decisions.G1,date:record.completed_date,
  candidate,evidence:record.evidence_index,reason:record.gate_reason}};
const integration=metadata.phases.find(p=>p.id==='integration');assert(integration);
integration.gate=record.decisions.G1==='earned'?'G1 · Unified playset ✓':'G1 · Decision withheld';
rendered.push([metadataPath,JSON.stringify(metadata,null,2)+'\n']);
const pathwayPath='docs/CERTIFIED_CAMPAIGN_PATHWAY.md';
let pathway=fs.readFileSync(path.join(repository,pathwayPath),'utf8').replace(/\r\n/g,'\n');
const header=/^\*\*Approved pathway[^\n]+\*\*$/m;assert(header.test(pathway));
pathway=pathway.replace(header,'**Approved pathway · 10 September 2026 · S01–S05 complete; execution stopped after S05.**');
const oldSummary='S01–S03 are complete; S04 is in progress; S05–S30 remain planned.';
assert(pathway.includes(oldSummary),'Review the pathway summary before replacing it');
pathway=pathway.replace(oldSummary,'S01–S05 are complete; S06–S30 remain planned. Execution is stopped after S05.');
for(const id of ['S04','S05']){
  const marker=`<a id="${id.toLowerCase()}"></a>`;const start=pathway.indexOf(marker);assert(start>=0);
  const end=pathway.indexOf('\n<a id=',start+marker.length);assert(end>start);
  const old=pathway.slice(start,end);assert(/\*\*Status:\*\* (?:Planned|In progress|Complete)/.test(old));
  let next=old.replace(/\*\*Status:\*\* (?:Planned|In progress|Complete)/,'**Status:** Complete').replace(/^- \[ \]/gm,'- [x]');
  const evidence=`Evidence: [${id} source-bound qualification](campaign-certification/${id}/README.md).`;
  const boundary=next.indexOf('\n### ');
  if(!next.includes(evidence))next=boundary>=0?next.slice(0,boundary).trimEnd()+'\n\n'+evidence+'\n\n'+next.slice(boundary):next.trimEnd()+'\n\n'+evidence+'\n';
  pathway=pathway.slice(0,start)+next+pathway.slice(end);
}
const stop='\n## Authorized execution boundary\n\nS04 and S05 are complete on the recorded runtime and evidence. '+
  `G1 decision: **${record.decisions.G1}**. ${record.gate_reason} `+
  'Execution stops after S05. S06 remains planned and requires a new instruction; no later campaign, content or release certificate is awarded.\n';
pathway+=stop;rendered.push([pathwayPath,pathway]);
const output=path.join(__dirname,'ready');
for(const [relative,text] of rendered){
  const target=path.join(output,relative);fs.mkdirSync(path.dirname(target),{recursive:true});
  const prior=fs.readFileSync(path.join(repository,relative),'utf8');
  const newline=prior.includes('\r\n')?'\r\n':'\n';
  fs.writeFileSync(target,text.replace(/\r\n/g,'\n').replace(/\n/g,newline));
}
fs.writeFileSync(path.join(output,'closeout-manifest.json'),JSON.stringify({runtime_candidate:candidate,
  record:recordPath,repository,decisions:record.decisions,completed_date:record.completed_date,
  files:rendered.map(([file])=>file),required_fields:[...required].sort(),
  source_edited:false,execution_stop_boundary:'S05',next_session_status:'planned'},null,2)+'\n');
console.log(JSON.stringify({prepared:output,files:rendered.map(([file])=>file),repository_edited:false},null,2));
