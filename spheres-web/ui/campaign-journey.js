/* Campaign overview. Rules, outcomes, goals and archive entries come from the
   server. Mutations use the shared receipt channel and reviewed decision flow. */
(function(root){
  "use strict";
  const esc = v => String(v ?? "").replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
  const number = v => Number.isFinite(v) ? v.toLocaleString('en-US',{maximumFractionDigits:2}) : '—';
  const title = (aims,aim) => aims?.offers?.find(o=>o.aim===aim)?.title || aim;
  function overview(data){
    const s=data.summary;
    return `<section class="journey-hero"><p class="journey-kicker">${esc(s.nation_name)} · ${esc(s.date)}</p><h3>${esc(s.title)}</h3><p>${esc(s.detail)}</p><p class="journey-mode">${s.observing?'Observer continuation · ':''}${s.beyond_2035?'Open-ended sandbox':`Campaign horizon · ${esc(s.horizon)}`}</p></section>
      ${s.paused_reason?`<p class="journey-notice">${esc(s.paused_reason)}</p>`:''}
      <div class="journey-actions">${s.actions.map((a,i)=>`<button type="button" data-journey-action="${i}">${esc(a.label)}</button>`).join('')}<button type="button" data-journey-tab="goals">Review campaign aims</button><button type="button" data-journey-tab="history">Read your national record</button></div>
      <div class="journey-grid">${data.metrics.map(m=>`<article><h4>${esc(m.label)}</h4><p class="journey-value">${number(m.after)} <small>${esc(m.unit)}</small></p><p>From ${number(m.before)} on ${esc(m.from)}<br>Latest record: ${esc(m.to)}</p></article>`).join('')}</div>
      <p class="journey-note">${esc(data.history_note)}</p>
      ${s.transitions.length?`<section><h3>Countries you have governed</h3><ol>${s.transitions.map(t=>`<li>${esc(t.date)} · ${esc(t.from)} → ${esc(t.to)}</li>`).join('')}</ol></section>`:''}`;
  }
  function goals(data){
    const a=data.aims;if(!a)return '<p>Choose a country to pursue campaign aims.</p>';
    const g=a.active,e=a.evaluation,done=g?.completed_day!=null;
    const active=g?`<article><p class="journey-kicker">${done?'Achieved':'Active aim'}</p><h3>${esc(title(a,g.aim))}</h3><p>Current ${number(e.value)} / fixed target ${number(g.target)} ${esc(e.metric)}</p><progress max="1" value="${done?1:e.progress}" aria-label="Campaign target progress"></progress><p>${g.held_days} / ${g.hold_days} consecutive qualifying days.</p>${done?'<p>This achievement is retained even if conditions change.</p>':`<p>Breaking a condition resets the qualifying-day count.</p><ul>${e.blockers.map(b=>`<li>${esc(b)}</li>`).join('')||'<li>All conditions met today. Keep them met for the remaining days.</li>'}</ul>`}${data.summary.alive?`<button type="button" data-journey-sandbox>Review ${done?'next aim':'setting this aim aside'}</button>`:''}</article>`:'';
    return `<p>${esc(a.note)}</p>${active}${!g&&data.summary.alive?`<div class="journey-grid">${a.offers.map(o=>`<article><h3>${esc(o.title)}</h3><p>${esc(o.description)}</p>${o.unavailable?`<p>${esc(o.unavailable)}</p>`:''}<button type="button" data-journey-aim="${esc(o.aim)}" ${o.unavailable?'disabled':''}>Review this aim</button></article>`).join('')}</div>`:''}<button type="button" data-journey-military>Inspect military agendas &amp; sovereignty</button><section><h3>Recorded aims</h3>${data.legacy_aims.length?`<ul>${data.legacy_aims.slice().reverse().map(r=>`<li>${esc(r.goal.nation)} · ${esc(title(a,r.goal.aim))} · ${esc(r.outcome)} (${r.goal.held_days} qualifying days)</li>`).join('')}</ul>`:'<p>No closed aims recorded yet.</p>'}</section>`;
  }
  function events(data){return `<p>${data.total} matching dispatches across your current and former governments. Newest first.</p><ol class="journey-events">${data.events.map(e=>`<li><p class="journey-kicker">${esc(e.date)} · ${esc(e.cat)}</p><p>${esc(e.text)}</p></li>`).join('')||'<li>No recorded dispatches match this filter.</li>'}</ol>${data.before>0?'<button type="button" id="journeyOlder">Load older dispatches</button>':''}`;}
  const apiModel={overview,goals,events};
  if(typeof module!=='undefined')module.exports=apiModel;
  if(typeof document==='undefined')return;
  const state={tab:'overview',data:null,source:null,request:0,busy:false,category:'',opener:null};
  function panel(){
    let el=document.getElementById('campaignJourney');if(el)return el;
    el=document.createElement('dialog');el.id='campaignJourney';el.setAttribute('aria-labelledby','journeyTitle');
    el.innerHTML='<header><div><p class="journey-kicker">Your campaign</p><h2 id="journeyTitle">Goals &amp; national record</h2></div><button type="button" id="journeyClose">Back to map</button></header><nav aria-label="Campaign sections"><button type="button" data-journey-tab="overview">Overview</button><button type="button" data-journey-tab="goals">Goals</button><button type="button" data-journey-tab="history">History</button></nav><p id="journeyStatus" role="status" tabindex="-1"></p><div id="journeyBody"></div>';
    el.addEventListener('keydown',e=>e.stopPropagation());
    el.addEventListener('close',()=>{state.request++;state.data=null;state.source=null;if(state.opener?.isConnected)state.opener.focus({preventScroll:true});});
    el.querySelector('#journeyClose').onclick=()=>el.close();document.body.append(el);return el;
  }
  function status(text){panel().querySelector('#journeyStatus').textContent=text;}
  function paint(){
    const el=panel(),d=state.data;if(!d?.summary)return;
    const active=document.activeElement,focusId=active?.id,focusTab=active?.dataset?.journeyTab;
    el.querySelector('#journeyBody').innerHTML=state.tab==='overview'?overview(d):state.tab==='goals'?goals(d):`<label class="journey-filter">Dispatch category <select id="journeyCategory"><option value="">All categories</option>${['economy','politics','war','diplomacy','other'].map(c=>`<option value="${c}" ${state.category===c?'selected':''}>${c==='politics'?'Government':c==='war'?'Military':c}</option>`).join('')}</select></label>${events(d)}`;
    el.querySelectorAll('[data-journey-tab]').forEach(b=>{b.setAttribute('aria-pressed',String(b.dataset.journeyTab===state.tab));b.onclick=()=>{state.tab=b.dataset.journeyTab;paint();el.querySelector('nav [data-journey-tab="'+state.tab+'"]').focus();};});
    el.querySelectorAll('[data-journey-action]').forEach(b=>{b.disabled=state.busy||agencyActionBlocked();b.onclick=()=>act(d.summary.actions[Number(b.dataset.journeyAction)]);});
    el.querySelectorAll('[data-journey-aim]').forEach(b=>b.onclick=()=>{el.close();agencyReview({kind:'choose_campaign_aim',aim:b.dataset.journeyAim});});
    const sandbox=el.querySelector('[data-journey-sandbox]');if(sandbox)sandbox.onclick=()=>{el.close();agencyReview({kind:'continue_sandbox'});};
    const military=el.querySelector('[data-journey-military]');if(military)military.onclick=()=>{el.close();openDomination();};
    const filter=el.querySelector('#journeyCategory');if(filter)filter.onchange=()=>{state.category=filter.value;load(false);};
    const older=el.querySelector('#journeyOlder');if(older)older.onclick=()=>load(true);
    if(focusId&&el.querySelector('#'+focusId))el.querySelector('#'+focusId).focus({preventScroll:true});
    else if(focusTab)el.querySelector('nav [data-journey-tab="'+focusTab+'"]').focus({preventScroll:true});
  }
  async function load(append=false){
    const el=panel(),world=S,request=++state.request;
    status('Loading campaign record…');
    el.querySelectorAll('#journeyOlder,#journeyCategory,[data-journey-action]').forEach(b=>b.disabled=true);
    try {
      const data=await api('/api/campaign-journey?session_id='+encodeURIComponent(world.session_id)+'&category='+state.category+(append?'&before='+state.data.before:''));
      if(request!==state.request||!el.open||S!==world||data.session_id!==world.session_id)return;
      if(append)data.events=[...state.data.events,...data.events];
      state.data=data;state.source=world;paint();status('');
    } catch(error){if(request===state.request&&el.open){status(error.message);const b=document.createElement('button');b.type='button';b.textContent='Retry loading record';b.onclick=()=>load(append);el.querySelector('#journeyStatus').append(' ',b);}}
  }
  async function act(action){
    if(state.busy||agencyActionBlocked()||state.source!==S)return;
    const world=S;
    if(!await campaignConfirm(action.command.action==='successor'?'Control will move to the successor as it exists today. Your former country’s history stays recorded. No troops, money or goal rewards are created.':action.command.action==='observe'?'Your government’s chapter has ended. Continue watching the surviving world with no government orders.':'Continue after the settled 2035 endpoint in open-ended sandbox play. Later play is outside the CP1 date range.',{title:action.label,confirmLabel:action.label}))return;
    if(S!==world||!panel().open)return;
    state.busy=true;paint();status('Recording continuation…');
    try {
      const response=await api('/api/command',{commands:[action.command],session_id:world.session_id});
      if(response.errors?.length)throw new Error(response.errors.join(' '));
      if(S!==world||response.session_id!==world.session_id)return;
      await adopt(response,false);
      if(panel().open)await load();
    }catch(error){status(error.message);}
    finally{state.busy=false;paint();}
  }
  root.openCampaignJourney=function(){
    if(!S?.player)return;clockPause();state.opener=document.activeElement;
    const el=panel();if(!el.open)el.showModal();el.querySelector('#journeyClose').focus();load();
  };
  root.syncCampaignJourney=function(world){
    const b=document.getElementById('campaignJourneyBtn'),s=world?.campaign_journey;
    if(b){b.disabled=!s;b.classList.toggle('watch',!!s?.paused_reason);b.querySelector('small').textContent=s?.paused_reason?'review continuation':s?.status==='aim_achieved'?'aim achieved':'goals & history';b.onclick=root.openCampaignJourney;}
    const el=document.getElementById('campaignJourney');
    if(el?.open&&state.source!==world){state.data=null;state.request++;el.querySelector('#journeyBody').replaceChildren();if(world?.player)load();else el.close();}
  };
})(globalThis);
