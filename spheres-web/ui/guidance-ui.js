/* Optional learning and read-only advice. Simulation commands stay in their existing review screens. */
(function(root,factory){
  const api=factory(typeof module==='object'&&module.exports?require('./tutorial-model.js'):root.TutorialModel,
    typeof module==='object'&&module.exports?require('./advisor-model.js'):root.AdvisorModel);
  if(typeof module==='object'&&module.exports)module.exports=api;else root.GuidanceUI=api;
})(typeof globalThis!=='undefined'?globalThis:this,function(Tutorial,Advisor){
  'use strict';
  const STORAGE_KEY='spheres.guidance.tutorial.v1';
  const esc=value=>String(value??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
  const areas={all:'All advisors',economy:'Economy',government:'Government',research:'Research',military:'Military',diplomacy:'Diplomacy'};
  const art={economy:'treasury',government:'cabinet',research:'research',military:'military',diplomacy:'diplomacy'};
  const lessonArt={home:'campaign',budget:'treasury',construction:'production',industry:'production',government:'cabinet',research:'research',equipment:'military',world:'diplomacy',campaign:'history'};
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
  function createSession(adapter){
    let progress=Tutorial.normalize(null),storageNotice='';
    try{if(!adapter.storage)throw new Error('No browser storage');const raw=adapter.storage.getItem(STORAGE_KEY);if(raw)progress=Tutorial.normalize(JSON.parse(raw));}
    catch(_){storageNotice='Progress is kept for this visit. Browser storage is unavailable.';}
    let view='tutorial',filter='all',query='',status='idle',notice='',cards=[],baseline=null,baselineKey=null,snapshot=null,seq=0;
    let dismissed=new Set(),listeners=new Set();
    const state=()=>({view,filter,query,status,notice,storageNotice,progress,cards:cards.filter(c=>!dismissed.has(c.id)),hiddenCount:cards.filter(c=>dismissed.has(c.id)).length,
      snapshot,canNavigate:!!adapter.getState()?.player&&adapter.canNavigate()});
    const emit=()=>listeners.forEach(listener=>listener(state()));
    function save(){try{adapter.storage?.setItem(STORAGE_KEY,JSON.stringify(progress));}catch(_){storageNotice='Progress is kept for this visit. Browser storage is unavailable.';}}
    const identity=value=>value?JSON.stringify([value.session_id,value.player,value.t,value.date]):null;
    function current(){return !!baseline&&adapter.getState()===baseline&&identity(adapter.getState())===baselineKey;}
    async function refresh(){
      const ticket=++seq;baseline=adapter.getState();baselineKey=identity(baseline);snapshot=null;cards=[];notice='';
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
        snapshot=served;cards=Advisor.evaluate(served,data.production);status='ready';emit();
      }catch(error){if(ticket!==seq)return;status='error';snapshot=null;cards=[];notice=error?.message||'Advice could not be loaded. Retry when the game is available.';emit();}
    }
    function changed(){
      if(baseline&&!current()){
        ++seq;baseline=null;baselineKey=null;snapshot=null;cards=[];dismissed.clear();status='stale';
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
    return {
      state,refresh,changed,follow,
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
  function render(model,asset=key=>`/art/areas/${key}-v1.webp`){
    const p=model.progress,l=lessonFor(p);
    const nav=`<nav class="guidance-tabs" aria-label="Guidance sections">${[['tutorial','Learn to play'],['advisors','Your advisors'],['glossary','Field guide']].map(([key,label])=>`<button type="button" data-guidance-view="${key}" aria-pressed="${model.view===key}">${label}</button>`).join('')}</nav>`;
    const notice=`${model.notice?`<p class="guidance-notice" role="status">${esc(model.notice)}</p>`:''}${model.storageNotice?`<p class="guidance-muted">${esc(model.storageNotice)}</p>`:''}`;
    let body='';
    if(model.view==='tutorial'){
      body=`<div class="guidance-intro"><p class="guidance-kicker">A country, one decision at a time</p><h2>Your first steps in Spheres</h2><p>Learn the connections between money, people and power. Read at your own pace, open the real screens, then return here.</p></div>
      <div class="guidance-progress"><progress max="${Tutorial.lessons.length}" value="${p.done.length}" aria-label="Lessons completed"></progress><span>${p.done.length} completed · ${p.skipped.length} skipped · ${Tutorial.lessons.length} lessons</span></div>
      <div class="guidance-learning"><nav class="guidance-lessons" aria-label="Tutorial lessons">${Tutorial.lessons.map((item,i)=>`<button type="button" data-guidance-lesson="${item.id}" ${l?.id===item.id?'aria-current="step"':''}><span class="guidance-step-number" aria-hidden="true">${p.done.includes(item.id)?'✓':p.skipped.includes(item.id)?'–':i+1}</span><span><strong>${esc(item.title)}</strong><small>${p.done.includes(item.id)?'Completed':p.skipped.includes(item.id)?'Skipped':esc(item.area)}</small></span></button>`).join('')}</nav>
      <article class="guidance-lesson">${l?`${picture(lessonArt[l.action.kind]||'campaign',asset)}<div class="guidance-lesson-copy"><p class="guidance-kicker">Lesson ${Tutorial.lessons.indexOf(l)+1} / ${Tutorial.lessons.length} · ${esc(l.area)}</p><h3 tabindex="-1" data-guidance-heading>${esc(l.title)}</h3><p>${esc(l.summary)}</p><ol>${l.steps.map(step=>`<li>${esc(step)}</li>`).join('')}</ol><div class="guidance-look"><strong>Look for</strong><p>${esc(l.lookFor)}</p></div><div class="guidance-actions"><button class="guidance-primary" type="button" data-guidance-open-lesson ${model.canNavigate?'':'disabled'}>${esc(l.actionLabel)} <span aria-hidden="true">↗</span></button><button type="button" data-guidance-complete>${p.done.includes(l.id)?'Read again · next lesson':'I understand · next lesson'}</button><button class="guidance-text-button" type="button" data-guidance-skip>Skip for now</button></div>${!model.canNavigate?'<p class="guidance-muted">You can read now. Continue or start a campaign to explore its screens.</p>':''}<p class="guidance-muted">Completion records what you’ve read. Orders and time remain under your control.</p></div>`:`<div class="guidance-finished">${picture('campaign',asset)}<h3 tabindex="-1" data-guidance-heading>${p.done.length===Tutorial.lessons.length?'You’ve finished the introduction.':'You’ve reached the end of the tour.'}</h3><p>${p.done.length===Tutorial.lessons.length?'Keep your advisors close as your country develops.':'You can return to the skipped lessons whenever you want.'}</p><button type="button" data-guidance-view="advisors">Read your country briefing →</button></div>`}</article></div>
      <div class="guidance-footer"><span>Learning progress is remembered in this browser across campaigns.</span><button class="guidance-text-button" type="button" data-guidance-restart>Restart lessons</button></div>`;
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
    const launcher=doc.createElement('button');launcher.type='button';launcher.id='guidanceLauncher';launcher.className='guidance-launcher';launcher.hidden=true;
    launcher.onclick=()=>open(returnToTutorial?'tutorial':'advisors');doc.body.append(launcher);
    function updateLauncher(){
      const m=session.state(),l=lessonFor(m.progress);launcher.hidden=!adapter.canNavigate();
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
          if(b.hasAttribute('data-guidance-view')){session.setView(b.dataset.guidanceView);if(b.dataset.guidanceView==='advisors'&&m.status!=='ready')session.refresh();}
          else if(b.hasAttribute('data-guidance-lesson'))session.learn({type:'select',id:b.dataset.guidanceLesson});
          else if(b.hasAttribute('data-guidance-open-lesson')&&l){returnToTutorial=true;session.learn({type:'start'});session.follow(l.action);}
          else if(b.hasAttribute('data-guidance-complete')&&l){session.learn({type:'complete',id:l.id});box.querySelector('[data-guidance-heading]')?.focus();}
          else if(b.hasAttribute('data-guidance-skip')&&l){session.learn({type:'skip',id:l.id});box.querySelector('[data-guidance-heading]')?.focus();}
          else if(b.hasAttribute('data-guidance-restart'))session.learn({type:'restart'});
          else if(b.hasAttribute('data-guidance-refresh'))session.refresh();
          else if(b.hasAttribute('data-guidance-hide'))session.dismiss(b.dataset.guidanceHide);
          else if(b.hasAttribute('data-guidance-restore'))session.restore();
          else if(b.hasAttribute('data-guidance-follow')){const card=m.cards.find(c=>c.id===b.dataset.guidanceFollow);if(card){returnToTutorial=false;session.follow(card.action,true);}}
        });
        box.addEventListener('change',e=>{if(e.target.hasAttribute('data-guidance-filter'))session.setFilter(e.target.value);});
        box.addEventListener('input',e=>{if(e.target.hasAttribute('data-guidance-search'))session.setQuery(e.target.value);});
        doc.body.append(box);
      }
      if(!box.open){focusBack=doc.activeElement;box.showModal();}
      session.setView(view);paint();box.querySelector('[data-guidance-close]').focus();
      if(view==='advisors')session.refresh();
    }
    session.subscribe(paint);
    return {open,close,changed:session.changed,session};
  }
  return {STORAGE_KEY,glossary,createSession,render,mount};
});
