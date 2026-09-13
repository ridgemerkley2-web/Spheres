const fs=require('node:fs');
const input=process.argv[2],needle=process.argv[3]||'/guidance-ui.css';
const log=JSON.parse(fs.readFileSync(input,'utf8'));
const names=Object.fromEntries(Object.entries(log.constants.logEventTypes).map(([name,id])=>[id,name]));
const phase=Object.fromEntries(Object.entries(log.constants.logEventPhase).map(([name,id])=>[id,name]));
const ids=/^#\d+$/.test(needle)?new Set([Number(needle.slice(1))]):new Set(log.events.filter(e=>e.params?.url?.includes(needle)&&e.source.type===(log.constants.logSourceType?.URL_REQUEST??1)).map(e=>e.source.id));
const events=log.events.filter(e=>ids.has(e.source.id)).map(e=>({...e,type_name:names[e.type],phase_name:phase[e.phase]}));
console.log(JSON.stringify({input,needle,source_ids:[...ids],events},null,2));
