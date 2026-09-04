// Run from any directory: node --test tools/ui/check_programs.cjs
// Executes the actual programme helpers and renderers. No browser, server,
// package install, save file or copied simulation funding model is involved.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');

const root = path.resolve(__dirname, '../..');
const page = fs.readFileSync(path.join(root, 'spheres-web/ui/index.html'), 'utf8');
const source = fs.readFileSync(path.join(root, 'spheres-web/ui/programs-ui.js'), 'utf8');
const plain = value => JSON.parse(JSON.stringify(value));

function pageFunction(name) {
  const match = new RegExp(`^(?:async\\s+)?function\\s+${name}\\(`, 'm').exec(page);
  assert(match, `The real page must declare ${name}`);
  const firstLine = page.slice(match.index, page.indexOf('\n', match.index));
  if (/\}\s*$/.test(firstLine)) return firstLine;
  const end = page.indexOf('\n}', match.index);
  assert(end >= 0, `${name} must retain its top-level closing brace`);
  return page.slice(match.index, end + 2);
}

function ministrySource() {
  const start = page.indexOf('const MINISTRIES = [');
  const end = page.indexOf('\n];', start);
  assert(start >= 0 && end > start, 'The real ministry presentation list must exist');
  return page.slice(start, end + 3);
}

function fixture() {
  const calls = [], moneyCalls = [];
  const context = vm.createContext({
    calls,
    fmt: { money(value) { moneyCalls.push(value); return `$${value}bn`; } },
    noteQueued: () => calls.push('note-queued'),
    paintCabinetDraft: () => calls.push('paint-draft'),
    openProduction: () => calls.push('open-production'),
    productionFetch: async () => calls.push('fetch-production'),
    renderProductionPanel: () => calls.push('render-production'),
    closeGameDrawers: () => calls.push('close-drawers'),
    openStock: () => calls.push('open-stock'),
    cabinetIsOpen: () => false,
  });
  // Use the shipped shared escaper as well as the shipped programme renderer.
  // A fixture that silently provides a stronger escaper would hide attribute
  // injection defects in the real page.
  const escaper = page.match(/^const escText\s*=.*;\s*$/m);
  assert(escaper, 'The page must expose its real shared text escaper');
  vm.runInContext(`
    ${ministrySource()}
    ${escaper[0]}
    let queued = [];
    let S = {player:'USA',date:'1 Jan 1991',year:1991,programs:{
      enabled:false,due:false,
      departments:Array.from({length:10},()=>[2000,2000,2000,2000,2000]),
      ministryrows:MINISTRIES.map((m,index)=>({
        index,name:m.name,editable:index===5,annual_bn:100+index,
        daily_bn:0.25+index/100,available_bn:9+index,spent_ytd_bn:2+index,
        departments:Array.from({length:5},(_,i)=>({
          index:i,name:'Department '+(i+1),kind:index===5?'capital':'operating',
          description:'A served department description.',annual_bn:20+i,
          daily_bn:0.05+i/100,available_bn:2+i,spent_ytd_bn:1+i
        }))
      })),
      investment_choices:Array.from({length:10},(_,i)=>({
        id:'choice-'+i,department:Math.floor(i/2),name:'Investment '+i,
        description:'Build a real facility.',effect:'A served physical payoff.',
        icon:'◈',tag:'Industry investment',enabled:true,
        project_kind:i===4?null:'civilian_industry',total_days:120,pc_cost:3
      })),
      industry:{sites:[],note:'The served industrial ledger.'}
    }};
    const m={id:'USA',name:'United States',annual_budget:{fiscal_year:1990}};
    MINISTRIES.forEach(spec=>{m.annual_budget[spec.id]=0.02;});
    const PROD={mode:'manufacture',view:'queue',pickKind:null};
    function me(){return m;}
    ${pageFunction('annualBudgetOf')}
    ${pageFunction('annualPoliticalCost')}
    ${pageFunction('fmtQ')}
    ${source}
  `, context, { filename: 'actual-programs-ui.js' });
  context.moneyCalls = moneyCalls;
  return context;
}
const run = (context, code) => vm.runInContext(code, context, { timeout: 2000 });
const json = (context, code) => plain(run(context, code));

// Read one real HTML start tag without treating a > inside an attribute value
// as its end. Parsing its attribute names catches a label that broke out of a
// quoted attribute, rather than merely searching for the word "onfocus".
function startTag(html, name) {
  const match = new RegExp(`<${name}\\b`, 'i').exec(html);
  assert(match, `Expected a rendered <${name}>`);
  let quote = null;
  for (let i = match.index + match[0].length; i < html.length; i++) {
    const ch = html[i];
    if (quote) { if (ch === quote) quote = null; }
    else if (ch === '"' || ch === "'") quote = ch;
    else if (ch === '>') return html.slice(match.index, i + 1);
  }
  assert.fail(`The <${name}> start tag was not closed`);
}
function attributes(tag) {
  const body = tag.replace(/^<[^\s>]+/, '').replace(/>$/, '');
  const out = new Map();
  const pattern = /([^\s=<>/'"]+)(?:\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s"'=<>`]+)))?/g;
  for (const match of body.matchAll(pattern)) {
    const key = match[1].toLowerCase();
    assert(!out.has(key), `Duplicate ${key} attribute in ${tag}`);
    out.set(key, match[2] ?? match[3] ?? match[4] ?? '');
  }
  return out;
}
function decode(value) {
  return value.replace(/&(?:quot|apos|lt|gt|amp|#\d+|#x[\da-f]+);/gi, entity => {
    const named = {'&quot;':'"','&apos;':"'",'&lt;':'<','&gt;':'>','&amp;':'&'};
    if (named[entity.toLowerCase()] !== undefined) return named[entity.toLowerCase()];
    return String.fromCodePoint(entity[2].toLowerCase() === 'x' ? parseInt(entity.slice(3,-1),16) : parseInt(entity.slice(2,-1),10));
  });
}

test('allocation moves conserve exactly 10000 basis points across many rows and bounds', () => {
  const c = fixture();
  let seed = 0x5137;
  const random = () => { seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0; return seed; };
  const rows = [
    [2000,2000,2000,2000,2000], [10000,0,0,0,0], [0,0,0,0,10000],
    [1,1,1,1,9996], [9996,1,1,1,1], [3333,3333,3334,0,0],
  ];
  for (let i=0; i<250; i++) {
    const cuts = [0,10000,...Array.from({length:4},()=>random()%10001)].sort((a,b)=>a-b);
    rows.push(cuts.slice(1).map((x,index)=>x-cuts[index]));
  }
  const values = [-100000,-0.5,0,1,499.4,499.5,2500,9999,9999.5,10000,100000,Number.MAX_VALUE];
  let checked = 0;
  for (const input of rows) {
    const row = Object.freeze(input.slice()), before = row.slice();
    for (let selected=0; selected<5; selected++) for (const value of values) {
      const out = c.programRedistribute(row,selected,value);
      assert(out, `A valid allocation was refused: ${row}, ${selected}, ${value}`);
      assert.equal(out.length,5);
      assert(out.every(n=>Number.isInteger(n) && n>=0 && n<=10000));
      assert.equal(out.reduce((a,b)=>a+b,0),10000,'no rounding drift may create or remove funding');
      assert.equal(out[selected],Math.min(10000,Math.max(0,Math.round(value))));
      assert.deepEqual(row,before,'redistribution must not mutate the live or queued row');
      checked++;
    }
  }
  assert.equal(checked,15360);
});

test('rounding residue and zero-weight siblings are allocated deterministically', () => {
  const c=fixture();
  assert.deepEqual(plain(c.programRedistribute([10000,0,0,0,0],0,1)),[1,2500,2500,2500,2499]);
  assert.deepEqual(plain(c.programRedistribute([3333,3333,3334,0,0],3,2501)),[2500,2499,2500,2501,0]);
  let row=[2000,2000,2000,2000,2000];
  for(let i=0;i<1000;i++) {
    const next=c.programRedistribute(row,i%5,(i*977)%10001);
    assert.deepEqual(plain(next),plain(c.programRedistribute(row,i%5,(i*977)%10001)));
    assert.equal(next.reduce((a,b)=>a+b,0),10000);
    row=next;
  }
});

test('invalid allocations, sparse rows, indices and nonnumeric requested values are refused', () => {
  const c=fixture(), good=[2000,2000,2000,2000,2000];
  const sparse=Array(5); sparse[0]=10000;
  for(const row of [null,undefined,{},'2000,2000,2000,2000,2000',[],[10000],[2500,2500,2500,2500],
    [...good,0],[1999,2000,2000,2000,2000],[-1,2500,2500,2500,2501],
    [2000.5,1999.5,2000,2000,2000],['2000',2000,2000,2000,2000],
    [NaN,2000,2000,2000,2000],[Infinity,0,0,0,0],sparse]) {
    assert.equal(c.programRedistribute(row,0,1000),null,`Invalid row accepted: ${String(row)}`);
  }
  for(const index of [-1,5,0.5,NaN,Infinity,'0',null,undefined]) assert.equal(c.programRedistribute(good,index,1000),null);
  for(const value of [NaN,Infinity,-Infinity,'1000',null,undefined,false,{},[]]) assert.equal(c.programRedistribute(good,0,value),null);
});

test('reading a programme draft deeply copies rows and prioritizes the queued programme plan', () => {
  const c=fixture();
  run(c,'const first=programPlanOf(); first[0][0]=9000;');
  assert.equal(run(c,'S.programs.departments[0][0]'),2000);
  run(c,`queued=[{kind:'program_budget',departments:Array.from({length:10},()=>[1000,2000,3000,2000,2000])}];
    const second=programPlanOf(); second[0][0]=9999; second.push([10000,0,0,0,0]);`);
  assert.equal(run(c,'queued[0].departments[0][0]'),1000);
  assert.equal(run(c,'queued[0].departments.length'),10);
  assert.deepEqual(json(c,'programPlanOf()[0]'),[1000,2000,3000,2000,2000]);
  assert.equal(run(c,'S.programs.departments[0][0]'),2000);
  run(c,'queued=[]; S.programs=null;');
  assert.equal(run(c,'programPlanOf()'),null);
});

test('programme command retains all ten annual ministry amounts and detaches its child rows', () => {
  const c=fixture();
  run(c,`queued=[{kind:'tax',value:.3},{...annualBudgetOf(m),fiscal_year:1990,industry:.045}];
    const command=programBudgetCommand(m);`);
  const command=json(c,'command');
  assert.equal(command.kind,'program_budget'); assert.equal(command.fiscal_year,1991);
  assert.equal(command.industry,.045); assert.equal(command.health,.02);
  assert.equal(command.departments.length,10);
  assert.equal(Object.keys(command).length,13,'ten parents, kind, year and the 10x5 child matrix');
  run(c,'command.departments[5][0]=10000; command.industry=.1;');
  assert.equal(run(c,'S.programs.departments[5][0]'),2000);
  assert.equal(run(c,'queued[1].industry'),.045);
  run(c,'S.programs=null; queued=[];');
  assert.equal(run(c,'programBudgetCommand(m).kind'),'annual_budget','older state keeps the existing annual command');
});

test('a funding preview is used only for the exact player, date and draft that requested it', () => {
  const c=fixture();
  run(c,'PG.preview={marker:"preview"}; PG.key=programDraftKey(m);');
  assert.equal(run(c,'programView(m).marker'),'preview');
  run(c,'S.date="2 Jan 1991";'); assert.equal(run(c,'programView(m)===S.programs'),true);
  run(c,'PG.key=programDraftKey(m); S.player="France";'); assert.equal(run(c,'programView(m)===S.programs'),true);
  run(c,'PG.key=programDraftKey(m); queued=[{...annualBudgetOf(m),industry:.03}];');
  assert.equal(run(c,'programView(m)===S.programs'),true);
});

test('invalidation discards an old response even when the new request has the same date and draft', async () => {
  const c=fixture(), requests=[];
  c.api=(url,command)=>new Promise(resolve=>requests.push({url,command:plain(command),resolve}));
  run(c,'PG.error="Previous error"; PG.failedKey="previous failed request";');
  const oldRequest=c.refreshProgramPreview(run(c,'m'));
  assert.equal(requests.length,1); assert.equal(run(c,'PG.pending'),true);
  const oldKey=run(c,'programDraftKey(m)'), oldSequence=run(c,'PG.seq');

  c.invalidateProgramPreview();
  assert.equal(run(c,'PG.preview'),null); assert.equal(run(c,'PG.key'),null);
  assert.equal(run(c,'PG.error'),''); assert.equal(run(c,'PG.failedKey'),null);
  assert.equal(run(c,'PG.requestKey'),null); assert.equal(run(c,'PG.pending'),false);
  assert(run(c,'PG.seq')>oldSequence,'an in-flight response must lose its request generation');
  run(c,'S.programs={...S.programs,marker:"updated same-day ledger"};');
  assert.equal(run(c,'programDraftKey(m)'),oldKey,'this regression deliberately keeps the old cache key');

  const newRequest=c.refreshProgramPreview(run(c,'m'));
  assert.equal(requests.length,2);
  assert.equal(requests[0].url,'/api/program-preview');
  assert.deepEqual(requests[0].command,requests[1].command);
  const newSequence=run(c,'PG.seq');
  requests[0].resolve({marker:'outdated response',political_cost:99});
  await oldRequest;
  assert.equal(run(c,'PG.preview'),null,'the old response must never enter the cache');
  assert.equal(run(c,'programView(m).marker'),'updated same-day ledger');
  assert.equal(run(c,'PG.pending'),true,'the old finally block must not finish the newer request');
  assert.equal(run(c,'PG.seq'),newSequence);

  requests[1].resolve({marker:'fresh response',political_cost:7});
  await newRequest;
  assert.equal(run(c,'programView(m).marker'),'fresh response');
  assert.equal(run(c,'annualPoliticalCost(m,annualBudgetOf(m))'),7);
  assert.equal(run(c,'PG.pending'),false);
});

test('a failed draft B preview cannot relabel draft A money or political cost as current', async () => {
  const c=fixture();
  let requests=0;
  c.api=async()=>{ requests++; return {marker:'draft A',annual_authorized_bn:100,political_cost:4}; };
  await c.refreshProgramPreview(run(c,'m'));
  const keyA=run(c,'PG.key');
  assert.equal(run(c,'programView(m).marker'),'draft A');
  run(c,'queued=[{...annualBudgetOf(m),industry:.03}];');
  const keyB=run(c,'programDraftKey(m)');
  assert.notEqual(keyA,keyB);
  c.api=async()=>{ requests++; throw new Error('Funding preview unavailable'); };
  await c.refreshProgramPreview(run(c,'m'));
  assert.match(run(c,'PG.error'),/Funding preview unavailable/);
  assert.match(run(c,'PG.error'),/enacted ledger.*draft is kept/,'the fallback must explain that draft amounts were not computed');
  assert.notEqual(run(c,'PG.key'),keyB,'only a successful matching response may claim the current key');
  assert.equal(run(c,'PG.failedKey'),keyB,'failed requests need their own retry-loop guard');
  assert.equal(run(c,'programView(m)===S.programs'),true);
  assert(Number.isNaN(run(c,'annualPoliticalCost(m,annualBudgetOf(m))')),'old political cost must not be shown for the rejected draft');
  assert.equal(run(c,'PG.pending'),false);
  await c.refreshProgramPreview(run(c,'m'));
  assert.equal(requests,2,'rendering after an error must not cause an automatic request loop');

  c.api=async()=>{ requests++; return {marker:'draft B',annual_authorized_bn:200,political_cost:9}; };
  await c.refreshProgramPreview(run(c,'m'),true);
  assert.equal(requests,3,'an explicit retry must bypass the failed-request guard');
  assert.equal(run(c,'programView(m).marker'),'draft B');
  assert.equal(run(c,'annualPoliticalCost(m,annualBudgetOf(m))'),9);
  assert.equal(run(c,'PG.error'),'');
});

test('the real adopt path invalidates same-day programme previews before rendering new state', async () => {
  const c=fixture(), adoptSource=pageFunction('adopt');
  assert.match(adoptSource,/invalidateProgramPreview\s*\(/,'every adopted server state must invalidate the funding preview');
  vm.runInContext(adoptSource,c);
  run(c,`const STOCKW={}, LOGI={open:false}, MANU={};
    let HIST; const ui={picked:['USA']}, selectedDistrict=null;
    function render(){calls.push(programView(m).marker);}
    S.programs.marker='old live ledger';
    PG.preview={...S.programs,marker:'old preview'}; PG.key=programDraftKey(m);
    PG.error='Old error'; PG.failedKey=PG.key;
    const nextState={...S,programs:{...S.programs,marker:'new live ledger'}};`);
  const keyBefore=run(c,'programDraftKey(m)');
  await c.adopt(run(c,'nextState'),false);
  assert.equal(run(c,'programDraftKey(m)'),keyBefore,'the adopted state has the same player, day and budget');
  assert.equal(run(c,'PG.preview'),null); assert.equal(run(c,'PG.key'),null);
  assert.equal(run(c,'PG.error'),''); assert.equal(run(c,'PG.failedKey'),null);
  assert.deepEqual(c.calls,['new live ledger'],'the first render after adoption must see the new ledger');
  assert.equal(run(c,'programView(m)===S.programs'),true);
});

test('money formatting preserves zero and refuses missing or nonfinite amounts', () => {
  const c=fixture();
  assert.equal(c.programMoney(0),'$0bn'); assert.equal(c.programMoney(.125),'$0.125bn');
  for(const value of [null,undefined,NaN,Infinity,-Infinity,'0',false,{}]) assert.equal(c.programMoney(value),'—');
  assert.deepEqual(c.moneyCalls,[0,.125],'unknown money must not be passed to the number formatter as zero');
});

test('automatic departments show a meter while editable departments expose a labeled basis-point control', () => {
  const c=fixture();
  const automatic=run(c,'programDepartmentCard(S.programs.ministryrows[0].departments[0],S.programs.ministryrows[0],programPlanOf()[0])');
  assert.match(automatic,/<progress\b[^>]*value="2000"[^>]*max="10000"/);
  assert.doesNotMatch(automatic,/<input\b|data-pg-share=|data-pg-department=/);
  assert.match(automatic,/Managed daily/); assert.match(automatic,/\$20bn/); assert.match(automatic,/\$0\.05bn/);
  const editable=run(c,'programDepartmentCard(S.programs.ministryrows[5].departments[0],S.programs.ministryrows[5],programPlanOf()[5])');
  const input=attributes(startTag(editable,'input'));
  assert.equal(input.get('type'),'range'); assert.equal(input.get('min'),'0'); assert.equal(input.get('max'),'10000');
  assert.equal(input.get('step'),'1'); assert.equal(input.get('value'),'2000');
  assert.equal(input.get('aria-valuetext'),'20.0 percent');
  assert.match(editable,/for="pgShare-5-0"/); assert.match(editable,/data-pg-department="0"/);
  assert.match(editable,/aria-pressed="true"/); assert.doesNotMatch(editable,/<progress\b/);
  for(const button of editable.matchAll(/<button\b[^>]*>[\s\S]*?<\/button>/g)) assert.doesNotMatch(button[0],/<input\b|<button\b[\s\S]*<button\b/,'controls must not be nested inside selection buttons');
});

test('all redistributed basis points remain exactly representable by the rendered range controls', () => {
  const c=fixture(), row=plain(c.programRedistribute([2000,2000,2000,2000,2000],0,2500));
  assert.deepEqual(row,[2500,1875,1875,1875,1875]);
  c.redistributedRow=row;
  for(let index=0;index<5;index++) {
    const html=run(c,`programDepartmentCard(S.programs.ministryrows[5].departments[${index}],S.programs.ministryrows[5],redistributedRow)`);
    const input=attributes(startTag(html,'input'));
    assert.equal(input.get('step'),'1','native controls must not round a valid single-basis-point allocation');
    assert.equal(+input.get('value'),row[index]);
    assert.equal((+input.get('value')-+input.get('min'))%+input.get('step'),0);
    assert.equal(input.get('aria-valuetext'),`${(row[index]/100).toFixed(1)} percent`);
  }
});

test('investment controls require the live enacted plan, not an optimistic preview', () => {
  const c=fixture();
  run(c,'PG.preview=JSON.parse(JSON.stringify(S.programs)); PG.preview.enabled=true; PG.key=programDraftKey(m);');
  let html=run(c,'programBoardHtml(m,5)');
  const buttons=[...html.matchAll(/<button\b[^>]*data-pg-invest="[^"]*"[^>]*>/g)].map(m=>m[0]);
  assert.equal(buttons.length,2,'exactly the selected department’s pair is shown');
  assert(buttons.every(tag=>/\sdisabled(?:\s|>)/.test(tag)));
  assert.match(html,/Enact your department budget first/);
  run(c,'S.programs.enabled=true; S.programs.due=false; PG.key=null;');
  html=run(c,'programBoardHtml(m,5)');
  assert([...html.matchAll(/<button\b[^>]*data-pg-invest="[^"]*"[^>]*>/g)].every(m=>!m[0].includes('disabled')));
  run(c,'S.programs.due=true;');
  html=run(c,'programBoardHtml(m,5)');
  assert([...html.matchAll(/<button\b[^>]*data-pg-invest="[^"]*"[^>]*>/g)].every(m=>m[0].includes('disabled')));
  const blocked=c.programInvestmentHtml({id:'blocked',name:'Locked',description:'Wait',effect:'None yet',enabled:false,reason:'Needs a real grid.',project_kind:'power_grid'},true);
  assert.match(startTag(blocked,'button'),/\sdisabled(?:\s|>)/); assert.match(blocked,/Needs a real grid/);
});

test('pre-enact investment calls are inert even if invoked without pressing the disabled button', async () => {
  const c=fixture();
  await c.openProgramInvestment('choice-0'); assert.deepEqual(c.calls,[]);
  run(c,'S.programs.enabled=true; S.programs.due=true;');
  await c.openProgramInvestment('choice-0'); assert.deepEqual(c.calls,[]);
  run(c,'S.programs.due=false; S.programs.investment_choices[0].enabled=false;');
  await c.openProgramInvestment('choice-0'); await c.openProgramInvestment('missing'); assert.deepEqual(c.calls,[]);
  run(c,'S.programs.investment_choices[0].enabled=true;');
  await c.openProgramInvestment('choice-0');
  assert.deepEqual(c.calls,['open-production','render-production']);
  assert.equal(run(c,'PROD.pickKind'),'civilian_industry'); assert.equal(run(c,'PROD.view'),'provinces');
});

test('malicious department labels cannot escape text or quoted accessibility attributes', () => {
  const c=fixture(), name='Power " autofocus onfocus="alert(1)"><script>boom</script>&';
  const rowName='Industry " onclick="alert(2)';
  c.hostileName=name; c.hostileRowName=rowName;
  run(c,'S.programs.ministryrows[5].name=hostileRowName; S.programs.ministryrows[5].departments[0].name=hostileName;');
  const html=run(c,'programDepartmentCard(S.programs.ministryrows[5].departments[0],S.programs.ministryrows[5],programPlanOf()[5])');
  assert.doesNotMatch(html,/<script\b|<img\b/);
  assert.match(html,/&lt;script>/); assert.match(html,/&amp;/);
  const input=attributes(startTag(html,'input'));
  assert.deepEqual([...input.keys()].sort(),['id','type','min','max','step','value','data-pg-share','aria-label','aria-valuetext'].sort(),
    'a hostile label must remain one attribute value, not create autofocus or event handlers');
  assert.equal(decode(input.get('aria-label')),`${name} share of ${rowName}`);
});

test('investment identifiers and board labels remain inert quoted text', () => {
  const c=fixture(), id='mine" onclick="alert(1)', label='Industry " autofocus onfocus="alert(2)';
  const html=c.programInvestmentHtml({id,name:'<img src=x onerror=alert(3)>',description:'<script>bad</script>',effect:'A & B',enabled:true,project_kind:'power_grid'},true);
  assert.doesNotMatch(html,/<script\b|<img\b/); assert.match(html,/A &amp; B/);
  const button=attributes(startTag(html,'button'));
  assert.deepEqual([...button.keys()].sort(),['type','data-pg-invest'].sort());
  assert.equal(decode(button.get('data-pg-invest')),id);
  c.hostileLabel=label; run(c,'S.programs.ministryrows[5].name=hostileLabel;');
  const section=attributes(startTag(run(c,'programBoardHtml(m,5)'),'section'));
  assert.deepEqual([...section.keys()].sort(),['class','id','aria-label','aria-busy'].sort());
  assert.equal(decode(section.get('aria-label')),`${label} departments`);
});

test('board status distinguishes drafts and pending previews without altering live spending', () => {
  const c=fixture(), before=json(c,'S.programs');
  run(c,'queued=[programBudgetCommand(m)];');
  assert.match(run(c,'programBoardHtml(m,5)'),/Draft allocations · not enacted/);
  run(c,'PG.pending=true;'); assert.match(run(c,'programBoardHtml(m,5)'),/Updating the server/);
  run(c,'PG.error="<img src=x onerror=alert(1)>";');
  const error=run(c,'programBoardHtml(m,5)'); assert.doesNotMatch(error,/<img src=x/); assert.match(error,/&lt;img src=x/);
  assert.deepEqual(json(c,'S.programs'),before,'rendering a draft must not write the live financial ledger');
  assert.equal(run(c,'programBoardHtml(m,99)'),'', 'a missing ministry must not fabricate controls');
  run(c,'S.programs=null;'); assert.equal(run(c,'programBoardHtml(m,5)'),'');
});

test('industry rendering uses the served goods, power and site snapshot without inventing returns', () => {
  const c=fixture();
  const data={
    note:'Industrial goods are not an extra GDP reward.',
    goods:{intermediates:42,capital_goods:7},capacity_each:1000,
    power_capacity_daily:9,power_used_daily:3,
    sites:[{district:'US-CA',kind:'machinery_works',level:2,status:'working',
      reason:'The line has its inputs.',output_daily:1.25,power_used_daily:.75,cash_spent_daily_bn:.004}]
  };
  const before=plain(data), html=c.programIndustryHtml(data);
  for(const text of ['Intermediate packs','Capital-goods packs','Storage / goods type',
    'Industrial power used / capacity','42','7','1,000','3 / 9','machinery works · US-CA',
    'The line has its inputs.','1.25 packs','$0.004bn','Output / day','Work cost / day']) {
    assert(html.includes(text),`Missing served industry reading ${text}`);
  }
  assert.doesNotMatch(html,/NaN|undefined|Infinity/);
  assert.deepEqual(data,before,'rendering must not change industrial inventory or operations');
  const hostile=c.programIndustryHtml({...data,sites:[{...data.sites[0],
    district:'<img src=x onerror=alert(1)>',kind:'<script>bad</script>',
    status:'<svg onload=alert(2)>',reason:'<iframe>bad</iframe>'}]});
  assert.doesNotMatch(hostile,/<img\b|<script\b|<svg\b|<iframe\b/);
  assert.match(hostile,/&lt;img/);
  assert.equal(c.programIndustryHtml(null),'');
  assert.match(c.programIndustryHtml({...data,sites:[]}),/Completed sites and their actual operating status will appear here/);
});
