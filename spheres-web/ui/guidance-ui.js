/* Optional learning and read-only advice. Simulation commands stay in their existing review screens. */
(function(root,factory){
  const api=factory(typeof module==='object'&&module.exports?require('./tutorial-model.js'):root.TutorialModel,
    typeof module==='object'&&module.exports?require('./advisor-model.js'):root.AdvisorModel);
  if(typeof module==='object'&&module.exports)module.exports=api;else root.GuidanceUI=api;
})(typeof globalThis!=='undefined'?globalThis:this,function(Tutorial,Advisor){
  'use strict';
  const STORAGE_KEY='spheres.guidance.tutorial.v1',RECEIPTS_KEY='spheres.guidance.receipts.v1',RECEIPT_LIMIT=20,RESTORED='Campaign and its history restored.';
  const esc=value=>String(value??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
  const obj=v=>v!==null&&typeof v==='object'&&!Array.isArray(v);
  const txt=v=>typeof v==='string'&&v.trim().length>0,str=v=>txt(v)&&v.length<=200;
  const count=v=>Number.isSafeInteger(v)&&v>=0;
  // Host and stored records are read as own data properties only; getters never run.
  const own=(value,key)=>{const d=Object.getOwnPropertyDescriptor(value,key);return d&&Object.hasOwn(d,'value')?d.value:undefined;};
  function dayIndex(year,month,day){
    if(!Number.isInteger(year)||year<1990||year>9999||!Number.isInteger(month)||month<1||month>12||!Number.isInteger(day)||day<1||day>31)return null;
    const time=Date.UTC(year,month-1,day),date=new Date(time);
    return date.getUTCMonth()===month-1&&date.getUTCDate()===day?(time-Date.UTC(1990,0,1))/86400000:null;
  }
  const areas={all:'All advisors',economy:'Economy',government:'Government',research:'Research',military:'Military',diplomacy:'Diplomacy'};
  const art={economy:'treasury',government:'cabinet',research:'research',military:'military',diplomacy:'diplomacy'};
  const lessonArt={home:'campaign',budget:'treasury',construction:'production',industry:'production',government:'cabinet',research:'research',equipment:'military',air:'military',world:'diplomacy',campaign:'history'};
  const stepLabels={achieved:'Achieved in this campaign',progress:'In progress',not_yet:'Not yet',unknown:'Unknown'};
  const stepMarks={achieved:'✓',progress:'◐',not_yet:'○',unknown:'?'};
  const milestoneLabels={done:'Done',pending:'Not yet',unknown:'Unknown'};
  const glossary=[
    ['Treasury','The government’s cash balance. GDP measures economic output; it is not a spendable account. Open the budget and cash-flow views to inspect the country’s actual books.'],
    ['Annual budget','The government’s ministry funding plan for a financial year. Review the authorization, daily spending and renewal status. Enacting a plan uses the screen’s stated political cost and time advance.'],
    ['Construction funding','A daily financial limit shared by building projects. Money pays for work as it progresses. A higher limit does not remove a project’s minimum lead time.'],
    ['Project effects','The construction preview separates the proposed province and country effects, costs and conditions. These are estimates to review before starting a project.'],
    ['Operating industry','A completed facility still needs its operating budget and inputs. Construction completion alone does not guarantee full output.'],
    ['Research focus','The technology being investigated in a research domain. Funding, prerequisites and calendar gates determine when progress and completion are possible.'],
    ['Equipment model','A design assembled from compatible researched components. Compare the model’s specifications and tradeoffs before submitting it for development.'],
    ['Development and procurement','A company develops a submitted design before equipment becomes available. Review company stock, purchase cost and delivery; a saved design is not equipment already in your army.'],
    ['Political capital','The political resource used by certain decisions. Review each action’s current cost and requirements on its confirmation screen.'],
    ['Party leader and national leader','Leading a political party is a separate role from leading the country. The Government page identifies the office held and distinguishes campaign leadership from historical reference.'],
    ['Stability and discontent','Different readings of the country’s political condition. Review their displayed contributors and government warnings before choosing a response.'],
    ['Orders and time','Some decisions apply through a preview and explicit confirmation; others are queued for a time advance. Read the button’s effect. The tutorial never issues orders or advances a day for you.']
  ];
  const picture=(key,asset)=>`<img class="guidance-art" src="${esc(asset(key))}" alt="" width="1536" height="1024" loading="lazy">`;
  function lessonFor(progress){return Tutorial.lessons.find(l=>l.id===progress.current)||null;}
  function saveRow(v){
    if(!obj(v))return null;
    const row={session_id:own(v,'session_id'),player:own(v,'player'),slot:own(v,'slot'),date:own(v,'date'),dispatches:own(v,'dispatches')??null};
    return str(row.session_id)&&str(row.player)&&str(row.slot)&&str(row.date)&&(row.dispatches===null||count(row.dispatches))?row:null;
  }
  function loadRow(v){
    if(!obj(v))return null;
    const row={session_id:own(v,'session_id'),player:own(v,'player'),slot:own(v,'slot')??null,backup:own(v,'backup'),date:own(v,'date'),dispatch_count:own(v,'dispatch_count')??null};
    return str(row.session_id)&&str(row.player)&&(row.slot===null||str(row.slot))&&typeof row.backup==='boolean'&&str(row.date)
      &&(row.dispatch_count===null||count(row.dispatch_count))?row:null;
  }
  function recent(value,parse){
    const out=[];if(!Array.isArray(value))return out;
    // Bounded recovery: only the newest RECEIPT_LIMIT stored entries are considered; malformed rows are dropped.
    for(let i=Math.max(0,value.length-RECEIPT_LIMIT);i<value.length;i++){const row=parse(own(value,String(i)));if(row)out.push(row);}
    return out;
  }
  // A missing record means nothing is recorded in this browser. ok:false means a record exists that this version
  // cannot read (corrupt, oversize or another version): it vouches for nothing and is never overwritten.
  function readReceipts(raw){
    if(raw===null||raw===undefined)return {ok:true,saves:[],loads:[]};
    const bad={ok:false,saves:[],loads:[]};
    if(typeof raw!=='string'||raw.length>100000)return bad;
    try{const data=JSON.parse(raw),saves=obj(data)?own(data,'saves'):null,loads=obj(data)?own(data,'loads'):null;
      return obj(data)&&own(data,'version')===1&&Array.isArray(saves)&&Array.isArray(loads)?{ok:true,saves:recent(saves,saveRow),loads:recent(loads,loadRow)}:bad;}
    catch(_){return bad;}
  }
  /** Validate one native /api/save or /api/load response captured by the host. Anything else is ignored. */
  function hostReceipt(input){
    if(!obj(input))return null;
    const path=own(input,'path'),body=own(input,'body'),data=own(input,'data'),state=own(input,'state');
    if(!obj(body)||!obj(data))return null;
    if(path==='/api/save'){
      const slot=own(data,'slot'),date=own(data,'date'),session=own(body,'session_id'),dispatches=own(data,'dispatches');
      if(!obj(state)||own(data,'ok')!==true||!str(slot)||slot!==own(body,'slot')||!str(date)||date!==own(state,'date')||!str(session)||session!==own(state,'session_id'))return null;
      const row=saveRow({session_id:session,player:own(state,'player'),slot,date,dispatches:count(dispatches)?dispatches:null});
      return row&&{kind:'saves',row};
    }
    if(path==='/api/load'){
      const session=own(data,'session_id'),slot=own(body,'slot'),dispatches=own(data,'dispatch_count');
      if(!str(session)||obj(state)&&own(state,'session_id')===session||own(data,'storage_notice')!==RESTORED)return null;
      const row=loadRow({session_id:session,player:own(data,'player'),slot:str(slot)?slot:null,backup:own(body,'backup')===true,date:own(data,'date'),dispatch_count:count(dispatches)?dispatches:null});
      return row&&{kind:'loads',row};
    }
    return null;
  }
  function createSession(adapter){
    let progress=Tutorial.normalize(null),receipts={saves:[],loads:[]};const volatile={progress:false,receipts:false};
    // known: the latest read of the receipt store succeeded, so an empty kind really means none recorded here.
    // unreadable: the stored record could not be read; earlier saves and loads are unknown and the record is left as it is.
    let known=false,unreadable=false;
    try{if(!adapter.storage)throw new Error('No browser storage');const raw=adapter.storage.getItem(STORAGE_KEY);if(raw)progress=Tutorial.normalize(JSON.parse(raw));}
    catch(_){volatile.progress=true;}
    const join=(a,b)=>{const seen=new Set(),out=[];for(const r of [...a,...b]){const k=JSON.stringify(r);if(!seen.has(k)){seen.add(k);out.push(r);}}return out.slice(-RECEIPT_LIMIT);};
    // Stored rows merge with this visit's before every write and every reading, so another tab's receipts are neither lost nor ignored.
    function sync(){
      try{if(!adapter.storage)throw new Error('No browser storage');const stored=readReceipts(adapter.storage.getItem(RECEIPTS_KEY));
        known=stored.ok;unreadable=!stored.ok;receipts={saves:join(stored.saves,receipts.saves),loads:join(stored.loads,receipts.loads)};}
      catch(_){known=false;volatile.receipts=true;}
    }
    sync();
    const storageNotice=()=>[volatile.progress||volatile.receipts?`${volatile.progress?volatile.receipts?'Progress and save receipts are':'Progress is':'Save receipts are'} kept for this visit. Browser storage is unavailable.`:'',
      unreadable?'Save receipts stored in this browser could not be read, so earlier saves and loads are unknown. That record is left unchanged; new receipts are kept for this visit.':''].filter(Boolean).join(' ');
    let view='tutorial',filter='all',query='',status='idle',notice='',cards=[],baseline=null,baselineKey=null,snapshot=null,seq=0;
    let reading=null,route=null,verified=null,routeStale=false;
    let dismissed=new Set(),listeners=new Set();
    const state=()=>({view,filter,query,status,notice,storageNotice:storageNotice(),progress,cards:cards.filter(c=>!dismissed.has(c.id)),hiddenCount:cards.filter(c=>dismissed.has(c.id)).length,
      snapshot,route,routeStale,routeChecked:verified?.date??null,routeAvailable:typeof Advisor?.recognize==='function',canNavigate:!!adapter.getState()?.player&&adapter.canNavigate()});
    const emit=()=>listeners.forEach(listener=>listener(state()));
    function save(){try{adapter.storage?.setItem(STORAGE_KEY,JSON.stringify(progress));}catch(_){volatile.progress=true;}}
    // Writes only over a record just read; each reason for skipping (no storage, failed read, unreadable record) already has its notice.
    function keep(){if(!known)return;try{adapter.storage.setItem(RECEIPTS_KEY,JSON.stringify({version:1,saves:receipts.saves,loads:receipts.loads}));}catch(_){volatile.receipts=true;}}
    const identity=value=>value?JSON.stringify([value.session_id,value.player,value.t,value.date]):null;
    function current(){return !!baseline&&adapter.getState()===baseline&&identity(adapter.getState())===baselineKey;}
    // Without a readable store an empty kind proves nothing: it is unknown (null) unless this visit captured one for this session.
    const held=()=>{const session=snapshot?.session_id,rows=k=>known||receipts[k].some(r=>r.session_id===session)?receipts[k].map(r=>({...r})):null;
      return {saves:rows('saves'),loads:rows('loads')};};
    function reevaluate(){if(status==='ready'&&current()&&reading){cards=Advisor.evaluate(snapshot,reading.production,reading.outcomes,held());route=recognize(reading);}emit();}
    function recognize(data){
      if(typeof Advisor?.recognize!=='function')return null;
      try{const result=Advisor.recognize(data,held());if(obj(result)&&Array.isArray(result.steps))return result;}catch(_){}
      return {status:'unknown',reason:'Campaign results could not be interpreted from this reading.',as_of:null,steps:[]};
    }
    // A verified route survives only as a labelled earlier reading of the same session and player.
    function holdRoute(next,stale){
      if(!route)return;
      if(!verified||!obj(next)||next.session_id!==verified.session_id||next.player!==verified.player){route=null;verified=null;routeStale=false;}
      else if(stale)routeStale=true;
    }
    async function refresh(){
      const ticket=++seq;sync();baseline=adapter.getState();baselineKey=identity(baseline);snapshot=null;reading=null;cards=[];notice='';holdRoute(baseline,true);
      if(!baseline?.player||!adapter.canNavigate()){status='idle';emit();return;}
      const requested=baseline;status='loading';emit();
      try{
        const data=await adapter.readSnapshot(requested);
        if(ticket!==seq)return;
        if(!current())throw new Error('The campaign changed while advice was loading. Refresh advice to review it again.');
        const served=data?.state;
        if(!served||served.session_id!==requested.session_id||served.player!==requested.player||served.t!==requested.t||served.date!==requested.date){
          throw new Error('The live campaign has moved ahead or changed. Open Campaigns and Continue to read its current state.');
        }
        // Outcomes must describe this player on this exact day; otherwise they are unknown and advice still loads.
        const day=dayIndex(served.year,served.month,served.day),found=obj(data.outcomes)?data.outcomes:null;
        const outcomes=found&&found.nation===served.player&&day!==null&&found.as_of_day===day?found:null;
        reading={...data,outcomes};snapshot=served;cards=Advisor.evaluate(served,data.production,outcomes,held());
        route=recognize(reading);verified=route?{session_id:served.session_id,player:served.player,date:served.date}:null;routeStale=false;
        status='ready';emit();
      }catch(error){if(ticket!==seq)return;status='error';snapshot=null;reading=null;cards=[];holdRoute(adapter.getState(),true);notice=error?.message||'Advice could not be loaded. Retry when the game is available.';emit();}
    }
    function changed(){
      holdRoute(adapter.getState(),!(status==='ready'&&current()));
      if(baseline&&!current()){
        ++seq;baseline=null;baselineKey=null;snapshot=null;reading=null;cards=[];dismissed.clear();status='stale';
        notice='The world has changed. Refresh advice for the current day and country.';emit();
      }else emit();
    }
    function follow(action,advice=false){
      if(!adapter.canNavigate()||!adapter.getState()?.player){notice='Continue or start a campaign before opening a game screen.';emit();return false;}
      if(advice&&(status!=='ready'||!current())){notice='Refresh advice before following this recommendation.';emit();return false;}
      if(adapter.busy()){notice='Finish the pending turn or confirmation before opening this review.';emit();return false;}
      if(!action||typeof action.kind!=='string'||!adapter.supports(action.kind)){notice='This review screen is unavailable in this build.';emit();return false;}
      try{if(adapter.navigate(action)===false)throw new Error('This screen could not be opened. Resolve the pending action and try again.');return true;}
      catch(error){notice=error?.message||'This screen could not be opened.';emit();return false;}
    }
    function receipt(input){
      let found=null;try{found=hostReceipt(input);}catch(_){}
      if(!found)return false;
      sync();const key=JSON.stringify(found.row),list=receipts[found.kind].filter(row=>JSON.stringify(row)!==key);
      list.push(found.row);receipts={...receipts,[found.kind]:list.slice(-RECEIPT_LIMIT)};keep();
      reevaluate();return true;
    }
    return {
      state,refresh,changed,follow,receipt,
      /** Another tab changed the receipt store: merge it and re-read only the current verified reading. */
      reread(){sync();reevaluate();},
      subscribe(fn){listeners.add(fn);return()=>listeners.delete(fn);},
      setView(next){view=['tutorial','advisors','glossary'].includes(next)?next:'tutorial';notice='';emit();},
      setFilter(next){filter=Object.hasOwn(areas,next)?next:'all';emit();},
      setQuery(next){query=String(next||'').slice(0,200);emit();},
      learn(event){progress=Tutorial.advance(progress,event);save();notice='';emit();},
      dismiss(id){if(cards.some(c=>c.id===id))dismissed.add(id);emit();},
      restore(){dismissed.clear();emit();},
      cancel(){++seq;if(status==='loading'){status='idle';baseline=null;}},
      report(message){notice=String(message);emit();}
    };
  }
  function readingOf(step,p){
    const ids=(Array.isArray(step.lessons)?step.lessons:[]).filter(id=>Tutorial.lessons.some(l=>l.id===id));
    const read=ids.filter(id=>p.done.includes(id)).length,part=ids.length>1&&read?` · ${read} of ${ids.length} read`:'';
    if(!ids.length)return ['none','No lesson'];
    if(read===ids.length)return ['read','Read'];
    return ids.some(id=>p.skipped.includes(id))?['skipped',`Skipped${part}`]:['unread',`Not read${part}`];
  }
  function routePanel(model){
    const p=model.progress,route=obj(model.route)?model.route:null,stale=!!route&&model.routeStale===true,loading=model.status==='loading';
    const known=!!route&&route.status==='ready',live=model.status==='ready'&&!stale&&!!model.canNavigate,checked=model.routeChecked||'an earlier reading';
    const line=model.routeAvailable===false?'Campaign results are unavailable in this build. The lessons remain available.'
      :loading?'Reading this campaign’s dated records…'
      :stale?'These results are from an earlier reading of this campaign. Refresh before following a step.'
      :!model.canNavigate?'Continue or start a campaign to check its results. You can read the lessons now.'
      :route&&!known?`Campaign results are unknown: ${txt(route.reason)?route.reason:'this reading is incomplete.'}`
      :route?`Checked ${checked}. Only dated records from this campaign count.`
      :model.status==='error'?'Campaign results could not be read. Refresh to try again.'
      :'Refresh to check which steps this campaign has achieved.';
    const steps=route&&Array.isArray(route.steps)?route.steps.filter(obj).slice(0,6):[];
    const items=steps.map((step,i)=>{
      const status=known&&Object.hasOwn(stepLabels,step.status)?step.status:'unknown',[readKey,readText]=readingOf(step,p);
      const campaign=stale?`Last checked ${checked}: ${stepLabels[status]}`:status==='achieved'&&model.routeChecked?`${stepLabels.achieved} · as of ${model.routeChecked}`:stepLabels[status];
      const milestones=(Array.isArray(step.milestones)?step.milestones:[]).filter(obj).slice(0,8).map(m=>{
        const s=known&&Object.hasOwn(milestoneLabels,m.status)?m.status:'unknown';
        return `<li class="guidance-milestone--${s}"><span class="guidance-milestone-state">${milestoneLabels[s]}</span><span>${esc(m.label||'Milestone')}${txt(m.date)?` · <time datetime="${esc(m.date)}">${esc(m.date)}</time>`:''}${txt(m.detail)?`<small>${esc(m.detail)}</small>`:''}</span></li>`;
      }).join('');
      const o=obj(step.obstacle)?step.obstacle:null,action=o&&obj(o.action)&&typeof o.action.kind==='string';
      const obstacle=o?`<div class="guidance-obstacle"><p class="guidance-kicker">Current obstacle</p><strong>${esc(o.title||'Review this step')}</strong>${txt(o.reason)?`<p>${esc(o.reason)}</p>`:''}${action?`<button type="button" data-guidance-route-step="${esc(step.id)}" aria-label="${esc(o.actionLabel||'Review')}: ${esc(step.title||'this step')}" ${live?'':'disabled'}>${esc(o.actionLabel||'Review')} <span aria-hidden="true">↗</span></button>`:''}</div>`:'';
      return `<li class="guidance-route-step guidance-route-step--${status}" data-guidance-route-card="${esc(step.id)}"><h4><span class="guidance-route-number" aria-hidden="true">${i+1}</span>${esc(step.title||'Step')}</h4><p class="guidance-chips"><span class="guidance-chip guidance-chip--${readKey}"><span class="guidance-chip-name">Reading:</span> ${esc(readText)}</span><span class="guidance-chip guidance-chip--${stale?'stale':status}"><span aria-hidden="true">${stepMarks[status]}</span> <span class="guidance-chip-name">Campaign:</span> ${esc(campaign)}</span></p>${step.summary?`<p class="guidance-route-summary">${esc(step.summary)}</p>`:''}${milestones?`<ul class="guidance-milestones" aria-label="Milestones: ${esc(step.title||'this step')}">${milestones}</ul>`:''}${obstacle}</li>`;
    }).join('');
    return `<section class="guidance-route" aria-labelledby="guidanceRouteTitle" aria-busy="${loading}"><div class="guidance-route-head"><div><p class="guidance-kicker">Optional route · campaign results</p><h3 id="guidanceRouteTitle">Your first hour</h3></div><button type="button" data-guidance-refresh ${loading||!model.canNavigate?'disabled':''}>${loading?'Reading your campaign…':stale?`Last checked ${esc(checked)} — refresh`:'Refresh results'}</button></div>
      <p class="guidance-route-note">Reading a lesson never completes a step. A step counts only when this campaign’s own dated records show it.</p><p class="guidance-route-line${stale?' guidance-route-line--stale':''}" role="status">${esc(line)}</p>${items?`<ol class="guidance-route-steps">${items}</ol>`:''}</section>`;
  }
  // One plain line beside lesson progress; the route panel itself follows the lessons.
  function routeSummary(model){
    const route=obj(model.route)?model.route:null;if(!route)return '';
    const steps=Array.isArray(route.steps)?route.steps.filter(obj).slice(0,6):[],known=route.status==='ready'&&steps.length>0;
    const head=model.routeStale===true?`Campaign results (last checked ${model.routeChecked||'earlier'})`:'Campaign results';
    return `<p class="guidance-results-line">${esc(`${head}: ${known?`${steps.filter(s=>s.status==='achieved').length} of ${steps.length} achieved`:'unknown'} · see Your first hour below`)}</p>`;
  }
  function render(model,asset=key=>`/art/areas/${key}-v1.webp`){
    const p=model.progress,l=lessonFor(p);
    const nav=`<nav class="guidance-tabs" aria-label="Guidance sections">${[['tutorial','Learn to play'],['advisors','Your advisors'],['glossary','Field guide']].map(([key,label])=>`<button type="button" data-guidance-view="${key}" aria-pressed="${model.view===key}">${label}</button>`).join('')}</nav>`;
    const notice=`${model.notice?`<p class="guidance-notice" role="status">${esc(model.notice)}</p>`:''}${model.storageNotice?`<p class="guidance-muted">${esc(model.storageNotice)}</p>`:''}`;
    let body='';
    if(model.view==='tutorial'){
      body=`<div class="guidance-intro"><p class="guidance-kicker">A country, one decision at a time</p><h2>Your first steps in Spheres</h2><p>Learn the connections between money, people and power. Read at your own pace, open the real screens, then return here.</p></div>
      <div class="guidance-progress"><progress max="${Tutorial.lessons.length}" value="${p.done.length}" aria-label="Lessons read"></progress><span>${p.done.length} read · ${p.skipped.length} skipped · ${Tutorial.lessons.length} lessons</span></div>${routeSummary(model)}
      <div class="guidance-learning"><nav class="guidance-lessons" aria-label="Tutorial lessons">${Tutorial.lessons.map((item,i)=>`<button type="button" data-guidance-lesson="${item.id}" ${l?.id===item.id?'aria-current="step"':''}><span class="guidance-step-number" aria-hidden="true">${p.done.includes(item.id)?'✓':p.skipped.includes(item.id)?'–':i+1}</span><span><strong>${esc(item.title)}</strong><small>${p.done.includes(item.id)?'Read':p.skipped.includes(item.id)?'Skipped':esc(item.area)}</small></span></button>`).join('')}</nav>
      <article class="guidance-lesson">${l?`${picture(lessonArt[l.action.kind]||'campaign',asset)}<div class="guidance-lesson-copy"><p class="guidance-kicker">Lesson ${Tutorial.lessons.indexOf(l)+1} / ${Tutorial.lessons.length} · ${esc(l.area)}</p><h3 tabindex="-1" data-guidance-heading>${esc(l.title)}</h3><p>${esc(l.summary)}</p><ol>${l.steps.map(step=>`<li>${esc(step)}</li>`).join('')}</ol><div class="guidance-look"><strong>Look for</strong><p>${esc(l.lookFor)}</p></div><div class="guidance-actions"><button class="guidance-primary" type="button" data-guidance-open-lesson ${model.canNavigate?'':'disabled'}>${esc(l.actionLabel)} <span aria-hidden="true">↗</span></button><button type="button" data-guidance-complete>${p.done.includes(l.id)?'Read again · next lesson':'I understand · next lesson'}</button><button class="guidance-text-button" type="button" data-guidance-skip>Skip for now</button></div>${!model.canNavigate?'<p class="guidance-muted">You can read now. Continue or start a campaign to explore its screens.</p>':''}<p class="guidance-muted">Completion records what you’ve read. Orders and time remain under your control.</p></div>`:`<div class="guidance-finished">${picture('campaign',asset)}<h3 tabindex="-1" data-guidance-heading>${p.done.length===Tutorial.lessons.length?'You’ve finished the introduction.':'You’ve reached the end of the tour.'}</h3><p>${p.done.length===Tutorial.lessons.length?'Keep your advisors close as your country develops.':'You can return to the skipped lessons whenever you want.'}</p><button type="button" data-guidance-view="advisors">Read your country briefing →</button></div>`}</article></div>
      ${routePanel(model)}
      <div class="guidance-footer"><span>Reading progress is remembered in this browser across campaigns and never counts as a campaign result. Campaign results come only from this campaign’s dated records.</span><button class="guidance-text-button" type="button" data-guidance-restart>Restart lessons</button></div>`;
    }else if(model.view==='advisors'){
      const shown=model.cards.filter(c=>model.filter==='all'||c.area===model.filter);
      body=`<div class="guidance-intro"><p class="guidance-kicker">Your advisory council</p><h2>What needs your attention?</h2><p>${model.snapshot?`${esc(model.snapshot.player_name||model.snapshot.player)} · ${esc(model.snapshot.date)}. Recommendations reflect this reading of the campaign.`:'Get a briefing from your current country’s budget, government, research and world situation.'}</p></div>
      <div class="guidance-toolbar"><label>Advisor <select data-guidance-filter>${Object.entries(areas).map(([key,name])=>`<option value="${key}" ${model.filter===key?'selected':''}>${name}</option>`).join('')}</select></label><button type="button" data-guidance-refresh ${model.status==='loading'||!model.canNavigate?'disabled':''}>${model.status==='loading'?'Reading your country…':'Refresh advice'}</button>${model.hiddenCount?`<button type="button" data-guidance-restore>Show ${model.hiddenCount} hidden</button>`:''}</div>
      <p class="guidance-muted">Review each reason and tradeoff. Advisors open decision screens; you choose the orders.</p>
      <div class="guidance-cards" aria-busy="${model.status==='loading'}">${shown.map(card=>`<article class="guidance-advice" data-guidance-card="${esc(card.id)}">${picture(art[card.area]||'campaign',asset)}<div class="guidance-advice-copy"><p class="guidance-kicker">${esc(areas[card.area]||'Advisor')} · <span class="guidance-priority guidance-priority--${card.priority==='attention'?'attention':'normal'}">${esc(card.priority)}</span></p><h3>${esc(card.title)}</h3><p>${esc(card.reason)}</p>${card.evidence?.length?`<details><summary>Why this recommendation?</summary><ul>${card.evidence.map(e=>`<li>${esc(e)}</li>`).join('')}</ul></details>`:''}${card.caution?`<p class="guidance-caution"><strong>Before deciding</strong> ${esc(card.caution)}</p>`:''}<div class="guidance-actions"><button class="guidance-primary" type="button" data-guidance-follow="${esc(card.id)}">${esc(card.actionLabel)} ↗</button><button class="guidance-text-button" type="button" data-guidance-hide="${esc(card.id)}" aria-label="Hide for now: ${esc(card.title)}">Hide for now</button></div></div></article>`).join('')||`<div class="guidance-empty"><h3>${model.status==='loading'?'Preparing your briefing':model.status==='error'?'The briefing is unavailable':model.status==='stale'?'A fresh briefing is needed':model.status==='ready'?'No recommendations in this view':'Your advisors are ready when you are'}</h3><p>${model.status==='ready'?'Try another advisor or restore hidden recommendations. Missing data is never treated as a clean bill of health.':model.status==='loading'?'Checking one consistent snapshot of the game.':model.canNavigate?'Use Refresh advice to inspect current conditions.':'Continue or start a campaign, then open Your advisors.'}</p></div>`}</div>`;
    }else{
      const terms=glossary.filter(row=>row.join(' ').toLocaleLowerCase().includes(model.query.toLocaleLowerCase()));
      body=`<div class="guidance-intro"><p class="guidance-kicker">Keep the essentials close</p><h2>A field guide to your country</h2><p>Plain-language explanations of the systems you’ll use most.</p></div><label class="guidance-search">Find a concept<input type="search" data-guidance-search value="${esc(model.query)}" placeholder="Funding, party leader, equipment…" autocomplete="off"></label><p role="status">${terms.length} concepts</p><dl class="guidance-glossary">${terms.map(([term,definition])=>`<div><dt>${esc(term)}</dt><dd>${esc(definition)}</dd></div>`).join('')||'<div><dt>No matching concepts</dt><dd>Try a shorter search or browse the tutorial.</dd></div>'}</dl>`;
    }
    return `${nav}<div class="guidance-body">${notice}${body}</div>`;
  }
  function mount(adapter){
    const doc=adapter.document||document;let box=null,focusBack=null,returnToTutorial=false;
    const session=createSession({...adapter,navigate(action){close(false);try{const result=adapter.navigate(action);if(result===false)open(session.state().view);updateLauncher();return result;}catch(error){open(session.state().view);throw error;}}});
    const live=()=>!!adapter.getState()?.player&&adapter.canNavigate();
    const launcher=doc.createElement('button');launcher.type='button';launcher.id='guidanceLauncher';launcher.className='guidance-launcher';launcher.hidden=true;
    launcher.onclick=()=>open(returnToTutorial?'tutorial':'advisors');doc.body.append(launcher);
    function updateLauncher(){
      const m=session.state(),l=lessonFor(m.progress);
      // Hosts with their own guidance navigation can omit the floating entry.
      // Standalone hosts retain it, including the return-to-tutorial shortcut.
      launcher.hidden=adapter.launcher===false||!adapter.canNavigate();
      launcher.textContent=returnToTutorial&&l?`↖ Tutorial · ${Tutorial.lessons.indexOf(l)+1}/${Tutorial.lessons.length}`:'✦ Guidance · F1';
      launcher.setAttribute('aria-label',returnToTutorial&&l?`Return to tutorial: ${l.title}`:'Open tutorial and advisors');
    }
    function paint(){
      updateLauncher();if(!box?.open)return;
      const previous=doc.activeElement,mark=previous?.getAttribute('data-guidance-focus');
      const selection=previous?.selectionStart;
      box.querySelector('.guidance-content').innerHTML=render(session.state(),adapter.asset);
      const content=box.querySelector('.guidance-content');
      content.querySelectorAll('button,input,select,summary').forEach((el,i)=>{
        const attr=Array.from(el.attributes).find(a=>a.name.startsWith('data-guidance-')&&a.name!=='data-guidance-focus');
        const key=attr?`${attr.name}:${attr.value}`:`${el.tagName}:${el.closest('[data-guidance-card]')?.getAttribute('data-guidance-card')||i}`;
        el.setAttribute('data-guidance-focus',key);
      });
      if(mark!=null){
        const next=Array.from(content.querySelectorAll('[data-guidance-focus]')).find(el=>el.getAttribute('data-guidance-focus')===mark&&!el.disabled);
        const target=next||box.querySelector('[data-guidance-close]');target.focus({preventScroll:true});
        if(next&&typeof selection==='number'&&next.type==='search')next.setSelectionRange?.(selection,selection);
      }else if(previous&&!previous.isConnected)box.querySelector('[data-guidance-close]').focus({preventScroll:true});
    }
    function close(restore=true){
      if(!box?.open)return;session.cancel();box.close();
      if(restore&&focusBack?.isConnected&&!focusBack.closest('[hidden],[inert]'))focusBack.focus({preventScroll:true});
    }
    function open(view='tutorial'){
      adapter.pause();
      if(!box){
        box=doc.createElement('dialog');box.id='guidanceDialog';box.className='guidance-dialog';box.setAttribute('aria-labelledby','guidanceTitle');
        box.innerHTML='<header class="guidance-header"><div><span>SPHERES / COMMAND SCHOOL</span><h1 id="guidanceTitle">Learn. Plan. Lead.</h1></div><button type="button" data-guidance-close aria-label="Close tutorial and advisors">Close</button></header><div class="guidance-content"></div>';
        box.addEventListener('keydown',event=>event.stopPropagation());
        box.addEventListener('cancel',event=>{event.preventDefault();close();});
        box.addEventListener('close',()=>{if(!box.open)session.cancel();});
        box.querySelector('[data-guidance-close]').onclick=()=>close();
        box.addEventListener('click',event=>{
          const b=event.target.closest('button');if(!b)return;
          const m=session.state(),l=lessonFor(m.progress);
          if(b.hasAttribute('data-guidance-view')){const next=b.dataset.guidanceView;session.setView(next);if((next==='advisors'||next==='tutorial'&&live())&&m.status!=='ready'&&m.status!=='loading')session.refresh();}
          else if(b.hasAttribute('data-guidance-lesson'))session.learn({type:'select',id:b.dataset.guidanceLesson});
          else if(b.hasAttribute('data-guidance-open-lesson')&&l){returnToTutorial=true;session.learn({type:'start'});session.follow(l.action);}
          else if(b.hasAttribute('data-guidance-complete')&&l){session.learn({type:'complete',id:l.id});box.querySelector('[data-guidance-heading]')?.focus();}
          else if(b.hasAttribute('data-guidance-skip')&&l){session.learn({type:'skip',id:l.id});box.querySelector('[data-guidance-heading]')?.focus();}
          else if(b.hasAttribute('data-guidance-restart'))session.learn({type:'restart'});
          else if(b.hasAttribute('data-guidance-refresh'))session.refresh();
          else if(b.hasAttribute('data-guidance-hide'))session.dismiss(b.dataset.guidanceHide);
          else if(b.hasAttribute('data-guidance-restore'))session.restore();
          else if(b.hasAttribute('data-guidance-follow')){const card=m.cards.find(c=>c.id===b.dataset.guidanceFollow);if(card){returnToTutorial=false;session.follow(card.action,true);}}
          else if(b.hasAttribute('data-guidance-route-step')){const step=(Array.isArray(m.route?.steps)?m.route.steps:[]).find(s=>obj(s)&&s.id===b.dataset.guidanceRouteStep);if(obj(step?.obstacle)){returnToTutorial=true;session.follow(step.obstacle.action,true);}}
        });
        box.addEventListener('change',e=>{if(e.target.hasAttribute('data-guidance-filter'))session.setFilter(e.target.value);});
        box.addEventListener('input',e=>{if(e.target.hasAttribute('data-guidance-search'))session.setQuery(e.target.value);});
        doc.body.append(box);
      }
      if(!box.open){focusBack=doc.activeElement;box.showModal();}
      session.setView(view);paint();box.querySelector('[data-guidance-close]').focus();
      // Reads only: opening guidance has already paused the clock.
      if(view==='advisors'||view==='tutorial'&&live())session.refresh();
    }
    session.subscribe(paint);
    // A save or load recorded by another tab counts here without a reload (key null: the store was cleared).
    const win=Object.hasOwn(adapter,'window')?adapter.window:typeof window!=='undefined'?window:null;
    win?.addEventListener?.('storage',event=>{if(event?.key===RECEIPTS_KEY||event?.key===null)session.reread();});
    return {open,close,changed:session.changed,receipt:session.receipt,session};
  }
  return {STORAGE_KEY,RECEIPTS_KEY,glossary,createSession,render,mount};
});
