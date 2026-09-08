/* Campaign storage, command recovery and selected history presentation. The
   importable state machines live in campaign-transport.js. */
let campaignConfirmation=null;
function campaignConfirm(message,{title="Confirm campaign action",confirmLabel="Confirm"}={}){
  // A second invocation never inherits consent from the dialog already open.
  if(campaignConfirmation)return Promise.resolve(false);
  if(typeof clockPause==="function")clockPause();
  const opener=document.activeElement,state=typeof S==="undefined"?null:S;
  const dialog=document.createElement("dialog");
  dialog.id="campaignConfirmDialog";dialog.className="decision-dialog";
  dialog.setAttribute("aria-labelledby","campaignConfirmTitle");
  dialog.setAttribute("aria-describedby","campaignConfirmMessage");
  dialog.innerHTML='<header><h2 id="campaignConfirmTitle"></h2></header><div class="decision-body"><p id="campaignConfirmMessage" style="white-space:pre-line"></p><div class="decision-actions"><button type="button" id="campaignConfirmCancel" autofocus>Cancel</button><button type="button" id="campaignConfirmAccept"></button></div></div>';
  dialog.querySelector("#campaignConfirmTitle").textContent=title;
  dialog.querySelector("#campaignConfirmMessage").textContent=message;
  dialog.querySelector("#campaignConfirmAccept").textContent=confirmLabel;
  document.body.append(dialog);campaignConfirmation=dialog;
  return new Promise(resolve=>{
    let settled=false;
    const finish=accepted=>{
      if(settled)return;settled=true;
      // A day or campaign adopted during the prompt invalidates its context.
      if(accepted && typeof S!=="undefined" && S!==state){accepted=false;banner("The campaign changed while confirmation was open. Review the current state and choose the action again.");}
      campaignConfirmation=null;
      if(dialog.open)dialog.close();dialog.remove();
      if(opener?.isConnected && !opener.disabled)opener.focus({preventScroll:true});
      resolve(accepted);
    };
    dialog.querySelector("#campaignConfirmCancel").onclick=()=>finish(false);
    dialog.querySelector("#campaignConfirmAccept").onclick=()=>finish(true);
    dialog.addEventListener("keydown",event=>event.stopPropagation());
    dialog.addEventListener("cancel",event=>{event.preventDefault();finish(false);});
    dialog.addEventListener("close",()=>finish(false));
    try{dialog.showModal();dialog.querySelector("#campaignConfirmCancel").focus({preventScroll:true});}
    catch(error){finish(false);banner("Could not open confirmation. The action was cancelled. "+error.message);}
  });
}
let commandBusyButton=null;
let commandWasBusy=false,commandRecoveryKey=null,commandRecoveryReturnFocus=null;
function campaignModalOpen(){
  if(campaignConfirmation)return false;
  return document.querySelector("dialog[open]") || (typeof arcadeTopRoom==="function" && arcadeTopRoom());
}
function revealCommandRecovery(){
  const pending=COMMAND_CHANNEL.pending;
  if(!pending||COMMAND_CHANNEL.busy||campaignConfirmation)return;
  commandRecoveryKey=`${pending.session_id}:${pending.client_id}:${pending.request_seq}`;
  // A native dialog makes body-level controls inert regardless of z-index.
  // Use each room's normal close path so its opener, forms and queued intent
  // have the same lifecycle as a manual close. The frozen receipt stays put.
  document.querySelectorAll("dialog[open]").forEach(dialog=>dialog.close());
  if(typeof closeGlobalMenus==="function")closeGlobalMenus();
  if(typeof COMP!=="undefined" && COMP.open)closeCompetition();
  if(typeof keysCardIsOpen==="function" && keysCardIsOpen())setKeysCard(false);
  if(typeof closeGameDrawers==="function")closeGameDrawers();
  if(typeof techMenuIsOpen==="function" && techMenuIsOpen())closeTechMenu();
  if(typeof tech!=="undefined" && tech.open)closeTech();
  if(typeof stock!=="undefined" && stock.open)closeStock();
  if(typeof PROD!=="undefined" && PROD.open)closeProduction();
  if(typeof LOGI!=="undefined" && LOGI.open)closeLogistics();
  if(typeof dominationIsOpen==="function" && dominationIsOpen())closeDomination();
  if(document.getElementById("sheet")?.style.display==="block" && typeof closeSheet==="function")closeSheet();
  const current=document.activeElement;
  if(current && current.id!=="retryCommandBtn" && current.id!=="reviewCommandBtn")commandRecoveryReturnFocus=current;
  const focus=()=>{if(COMMAND_CHANNEL.pending && !COMMAND_CHANNEL.busy && !campaignModalOpen())document.getElementById("retryCommandBtn")?.focus({preventScroll:true});};
  focus();setTimeout(focus,0); // Native close events may restore their opener later.
}
function finishCommandRecovery(){
  if(COMMAND_CHANNEL.pending)return;
  const target=commandRecoveryReturnFocus;commandRecoveryReturnFocus=null;
  if(target?.isConnected && !target.disabled && target.getClientRects().length)target.focus({preventScroll:true});
}
function syncClockControls() {
  const toggle=document.getElementById("playPauseBtn");
  if(!toggle)return;
  // Pause only retires the local timer chain. A running clock must remain
  // stoppable while a day is in flight; Play still respects recovery locks.
  const running=typeof clock!=="undefined"&&clock.running;
  const pendingTurn=typeof pendingAdvance!=="undefined"&&!!pendingAdvance;
  const turnInFlight=typeof advancing!=="undefined"&&advancing;
  const channel=typeof COMMAND_CHANNEL==="undefined"?null:COMMAND_CHANNEL;
  const sessionBusy=typeof SESSION!=="undefined"&&SESSION.busy;
  toggle.disabled=!running&&(!!channel?.pending||!!channel?.busy||
    (pendingTurn&&!turnInFlight)||!!sessionBusy);
}
function syncCommandControls() {
  const pending=COMMAND_CHANNEL.pending,busy=COMMAND_CHANNEL.busy;
  const uncertain=!!pending&&!busy&&commandWasBusy;commandWasBusy=busy;
  if(!pending)commandRecoveryKey=null;
  if(pending && typeof clock!=="undefined" && clock.running)clockPause();
  const box=document.getElementById("pendingCommand");
  if(box){box.hidden=!pending;document.getElementById("pendingCommandText").textContent=busy
    ?"Sending your order… Another action will wait for its result."
    :"An order's outcome needs confirmation. Check its receipt, or review the current campaign before issuing another action.";}
  for(const id of ["retryCommandBtn","reviewCommandBtn"]){const b=document.getElementById(id);if(b)b.disabled=busy;}
  if(busy && !commandBusyButton && document.activeElement?.tagName==="BUTTON"){
    commandBusyButton=document.activeElement;commandBusyButton.disabled=true;
  }
  if(!busy && commandBusyButton){if(commandBusyButton.isConnected)commandBusyButton.disabled=false;commandBusyButton=null;}
  for(const id of ["saveBtn","stepBtn","loadBtn","newCampaignBtn","saveNamedBtn","loadBackupBtn"]){
    const b=document.getElementById(id);if(b)b.disabled=!!pending||busy||advancing||!!pendingAdvance||SESSION.busy;
  }
  document.querySelectorAll("[data-step]").forEach(b=>b.disabled=advancing||!!pendingAdvance||!!pending);
  const enact=document.getElementById("cabinetEnact");if(enact)enact.disabled=CAB.busy||advancing||!!pendingAdvance||!!pending;
  const save=document.getElementById("saveBtn");if(save){const slot=SESSION.slot||"default";save.textContent="Save · "+slot;save.title="Save the current campaign and history to "+slot;}
  syncClockControls();
  if(typeof renderMainMenuState === "function")renderMainMenuState();
  if(typeof companiesRender === "function")companiesRender();
  if(uncertain || (pending&&!busy&&commandRecoveryKey&&campaignModalOpen()))revealCommandRecovery();
}
async function retryCampaignCommand() {
  if(!S)await continueCampaign();
  if(!S)return;
  try {const result=await COMMAND_CHANNEL.retry();
    if(typeof COMP!=="undefined" && COMP.pending){COMP.pending=null;competitionPendingStore();}
    await adopt(result,false);banner(result.errors?.length?result.errors.join(" "):"Order confirmed. It was applied once.");
  }catch(error){banner(error.message);}finally{syncCommandControls();finishCommandRecovery();}
}
async function reviewCampaignCommand() {
  if(COMMAND_CHANNEL.busy)return;
  const receipt=COMMAND_CHANNEL.pending;if(!receipt)return;
  try {const state=await api("/api/state");
    if(!await campaignConfirm("Review the current campaign and dismiss this order's pending notice? This does not replay or undo the order. Check its event log and accounts before issuing a new action.",{title:"Review pending order",confirmLabel:"Dismiss notice"}))return;
    if(COMMAND_CHANNEL.busy||COMMAND_CHANNEL.pending!==receipt)return;
    COMMAND_CHANNEL.clear();
    if(typeof COMP!=="undefined"){COMP.pending=null;competitionPendingStore();}
    if(state.player)await enterCampaign(state,state.session_id!==S?.session_id);
    else {SESSION.live=state;S=null;renderSessionActions();}
    banner("Current state restored. The earlier order was not replayed or undone.");
  }catch(error){banner(error.message);}finally{syncCommandControls();finishCommandRecovery();}
}
async function refreshSaveSlots() {
  const select=document.getElementById("saveSlots");if(!select)return;
  const previous=select.value||SESSION.slot||"default";
  try {const result=await api("/api/saves");SESSION.saves=result.slots;select.replaceChildren();
    for(const entry of result.slots){const option=document.createElement("option");option.value=entry.slot;
      option.textContent=`${entry.slot} · ${entry.player||"campaign"} · ${entry.date||"legacy date"}${entry.readable?"":" · damaged; try backup"}`;
      option.dataset.backup=String(entry.backup);select.append(option);}
    if(!result.slots.length){const option=document.createElement("option");option.value="default";option.textContent="No saved campaign yet";select.append(option);}
    if([...select.options].some(o=>o.value===previous))select.value=previous;
    document.getElementById("saveSlotStatus").textContent=result.autosave;
  }catch(error){document.getElementById("saveSlotStatus").textContent="Could not read save slots: "+error.message;}
  if(typeof renderMainMenuState === "function")renderMainMenuState();
}
async function saveNamedCampaign() {
  const input=document.getElementById("saveName"),slot=CampaignTransport.slotName(input.value);
  if(!slot){banner("Choose a name of 1–64 letters, numbers, hyphens or underscores.");input.focus();return;}
  if(slot.startsWith("auto-")){banner("Names beginning auto- are reserved for rotating autosaves.");return;}
  if(advancing||pendingAdvance||COMMAND_CHANNEL.pending){banner("Confirm the pending action before saving.");return;}
  if(!S?.player){banner("Continue the live campaign before saving it.");return;}
  const button=document.getElementById("saveNamedBtn");button.disabled=true;
  try {await api("/api/save",{slot});SESSION.slot=slot;await refreshSaveSlots();document.getElementById("saveSlots").value=slot;banner(`Saved ${S.date} with its history to ${slot}.`);}
  catch(error){banner("Save failed: "+error.message);}finally{syncCommandControls();}
}
const HISTORY_VIEW={key:null,pending:false};
function historySelection() {
  const ids=[];
  if(selected)ids.push(selected);
  if(ui.tab==="charts") {
    if(CHRONICLE.view==="compare")ids.push(...ui.picked);
    else ids.push(CHRONICLE.nation||S?.player);
  }
  return [...new Set(ids.filter(Boolean))];
}
async function refreshCampaignHistory(state,force=false) {
  const ids=historySelection();if(!ids.length)return;
  const key=JSON.stringify([state.session_id,state.date,ids.slice().sort()]);
  if(HISTORY_VIEW.key===key && !force)return;
  HISTORY_VIEW.key=key;HISTORY_VIEW.pending=true;
  try {const result=await HISTORY_READER.read(ids,state);if(!result||S!==state||HISTORY_VIEW.key!==key)return;
    HIST=result;return true;
  }catch(error){if(HISTORY_VIEW.key===key)HIST=null;banner("History could not load: "+error.message+". Reopen History to retry.");return false;}
  finally{if(HISTORY_VIEW.key===key)HISTORY_VIEW.pending=false;}
}
function requestVisibleHistory() {
  if(!S)return;
  const state=S,key=JSON.stringify([state.session_id,state.date,historySelection().slice().sort()]);
  if(HISTORY_VIEW.key===key)return;
  refreshCampaignHistory(state).then(loaded=>{
    if(!loaded||S!==state)return;
    if(ui.tab==="charts")renderCharts();
    if(selected)openNation(selected,true);
  });
}
let dispatchLimit=500,dispatchBusy=false;
async function moreCampaignDispatches(){
  if(dispatchBusy||!S)return;
  const state=S;dispatchBusy=true;
  try{
    if(dispatchLimit>=S.log.length && S.log.length<(S.dispatch_count||0)){
      const page=await api(`/api/events?before=${S.dispatch_count-S.log.length}&limit=500`);
      if(S!==state||page.session_id!==S.session_id)return;
      S.log.push(...page.events);
    }
    dispatchLimit+=500;renderLog();
  }catch(error){banner("Could not load older dispatches: "+error.message);}
  finally{dispatchBusy=false;}
}
function installCampaignControls() {
  document.getElementById("retryCommandBtn").onclick=retryCampaignCommand;
  document.getElementById("reviewCommandBtn").onclick=reviewCampaignCommand;
  document.getElementById("saveNamedBtn").onclick=saveNamedCampaign;
  document.getElementById("loadBackupBtn").onclick=()=>loadCampaign(true);
  refreshSaveSlots();syncCommandControls();
  if(COMMAND_CHANNEL.pending&&!COMMAND_CHANNEL.busy)revealCommandRecovery();
}
