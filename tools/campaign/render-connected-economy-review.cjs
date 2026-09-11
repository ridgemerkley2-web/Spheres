/* Render actual native read-model fixtures with the production presentation.
   Usage: node tools/campaign/render-connected-economy-review.cjs input.json output.html
   This review has no API calls and cannot change a campaign. */
const fs=require('node:fs'),path=require('node:path');
const [input,output]=process.argv.slice(2);
if(!input||!output)throw new Error('Supply a native S02 fixture JSON and output HTML path.');
const root=path.resolve(__dirname,'../..');
const fixture=JSON.parse(fs.readFileSync(input,'utf8'));
const page=fs.readFileSync(path.join(root,'spheres-web/ui/index.html'),'utf8');
const styles=[...page.matchAll(/<style[^>]*>([\s\S]*?)<\/style>/g)].map(match=>match[1]).join('\n');
const css=fs.readFileSync(path.join(root,'spheres-web/ui/cash-flow-ui.css'),'utf8');
const presentation=fs.readFileSync(path.join(root,'spheres-web/ui/fiscal-recovery-ui.js'),'utf8');
const embedded=JSON.stringify(fixture).replace(/</g,'\\u003c');
const html=`<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>Spheres · Connected economy review</title><style>${styles}\n${css}
body {overflow:auto;height:auto;min-height:100vh;display:block;padding:24px;}
.s02-review {max-width:1120px;margin:0 auto;}
.s02-review header {display:block;position:static;height:auto;padding:20px;margin-bottom:24px;}
.s02-review nav {display:flex;gap:8px;flex-wrap:wrap;margin:16px 0;}
.s02-review nav button {padding:10px 14px;}
.s02-review #status {min-height:24px;}
@media(max-width:500px){body{padding:12px}.s02-review .cf-balances{grid-template-columns:1fr 1fr}.s02-review .cf-priority-grid{grid-template-columns:1fr}}
</style></head><body><main class="s02-review"><header><p class="cf-kicker">Spheres · S02 integration review</p><h1>People, industry and public finances</h1><p>Read-only simulation snapshots. Review buttons demonstrate navigation; they do not change a campaign.</p><nav aria-label="Snapshot"><button type="button" data-state="legacy">Before upgrade</button><button type="button" data-state="fresh">Connected economy</button><button type="button" data-state="after_three_days">After three days</button></nav><p id="status" role="status"></p></header><div id="reading"></div></main><script>${presentation}</script><script>
const fixture=${embedded};let unbind;
function show(key){if(unbind)unbind();const view=fixture[key];document.getElementById('reading').innerHTML=FiscalRecoveryUI.renderConnected(view);document.getElementById('status').textContent='Snapshot: '+key.replaceAll('_',' ');document.querySelectorAll('[data-state]').forEach(button=>button.setAttribute('aria-pressed',String(button.dataset.state===key)));unbind=FiscalRecoveryUI.bind(document.getElementById('reading'),{isCurrent:()=>true,isBusy:()=>false,canUpgrade:view?.upgrade?.available===true,onEnable:()=>document.getElementById('status').textContent='Review: '+view.upgrade.effect,onPolicy:policy=>document.getElementById('status').textContent='Review training focus: '+(view.population?.policies?.find(item=>item.policy===policy)?.name||policy),onNavigate:action=>document.getElementById('status').textContent='Navigation preview: '+action});}
document.querySelectorAll('[data-state]').forEach(button=>button.addEventListener('click',()=>show(button.dataset.state)));show('after_three_days');
</script></body></html>`;
fs.mkdirSync(path.dirname(path.resolve(output)),{recursive:true});fs.writeFileSync(output,html);
console.log(path.resolve(output));
