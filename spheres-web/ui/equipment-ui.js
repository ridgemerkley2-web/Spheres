/* Equipment is a server-priced workflow. A local draft may be incomplete;
   only a fresh, reviewed server action can create work or change inventory. */
const EQUIP={open:false,data:null,state:null,session:null,nation:null,seq:0,loading:false,stale:true,error:"",busy:false,message:"",
  tab:"designer",draft:null,preset:null,draftSeq:0,preview:null,previewState:null,previewSeq:0,previewLoading:false,previewError:"",previewKey:null,
  query:"",researchBranch:"chassis",researchState:"all",comparisonId:null,automaticName:null,selectedSlot:null,advanced:false,details:new Set(),focus:null,scroll:0,request:null,review:null};
const EQUIPMENT_TABS=[["research","Research"],["designer","Designer"],["library","Library"],["development","Development"],["production","Production"],["service","In service"]];
const EQUIPMENT_BRANCHES=[["chassis","Chassis","The foundation of every vehicle"],["engines","Engines & running gear","Power, traction and mobility"],["weapons","Weapons","Armament, ammunition and firepower"],["armor","Armor & protection","Survivability and active defenses"],["optics","Optics & fire control","Find, track and engage targets"],["communications","Communications","Connect the force"]];
const EQUIPMENT_RESEARCH_STATES=[["all","All states"],["available","Available"],["researching","In progress"],["known","Known"],["locked","Locked"]];
// This host survives pricing and form repaints, retaining its camera and GPU resources.
const EQUIPMENT_VIEWER={host:null,controller:null,key:null};
const EQUIPMENT_TREE={observer:null};
function equipmentText(value){return String(value??"").replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;").replace(/'/g,"&#39;");}
function equipmentNumber(value){return Number.isFinite(value)?value.toLocaleString("en-US",{maximumFractionDigits:3}):"—";}
function equipmentMoney(value){return Number.isFinite(value)?`${value<0?"-":""}${economyMoney(Math.abs(value))}`:"—";}
function equipmentCopy(value){return value==null?value:JSON.parse(JSON.stringify(value));}
function equipmentRows(key){return Array.isArray(EQUIP.data?.[key])?EQUIP.data[key]:[];}
function equipmentActive(){return EQUIP.open&&typeof S!=="undefined"&&!!S?.player&&(typeof equipmentIsOpen!=="function"||equipmentIsOpen());}
function equipmentPending(){return EQUIP.busy||(typeof advancing!=="undefined"&&advancing)||(typeof pendingAdvance!=="undefined"&&!!pendingAdvance)
  ||(typeof COMMAND_CHANNEL!=="undefined"&&!!(COMMAND_CHANNEL.busy||COMMAND_CHANNEL.pending))||(typeof SESSION!=="undefined"&&SESSION.busy)
  ||(typeof CAB!=="undefined"&&CAB.busy)||(typeof PROD!=="undefined"&&PROD.busy)||(typeof COMP!=="undefined"&&!!(COMP.busy||COMP.pending));}
function equipmentCurrent(data=EQUIP.data,state=EQUIP.state){return equipmentActive()&&!!data&&data===EQUIP.data&&state===EQUIP.state&&state===S&&!EQUIP.loading&&!EQUIP.stale&&!EQUIP.error&&!equipmentPending();}
function equipmentDraftKey(){return JSON.stringify([EQUIP.draft,EQUIP.comparisonId]);}
function equipmentPreviewCurrent(preview=EQUIP.preview,state=EQUIP.previewState){return equipmentCurrent()&&!!preview&&preview===EQUIP.preview&&state===S&&state===EQUIP.previewState
  &&!EQUIP.previewLoading&&!EQUIP.previewError&&EQUIP.previewKey===equipmentDraftKey();}
function equipmentMetric(row){
  const value=typeof row.value==="string"?row.value:equipmentNumber(row.value);
  return `<div><dt>${equipmentText(row.label)}</dt><dd>${equipmentText(value)}${row.unit?` <span>${equipmentText(row.unit)}</span>`:""}</dd>${row.period?`<small>${equipmentText(row.period)}</small>`:""}${row.change?`<small>${equipmentText(row.change)}</small>`:""}</div>`;
}
function equipmentMetrics(rows){return Array.isArray(rows)&&rows.length?`<dl class="eq-metrics">${rows.map(equipmentMetric).join("")}</dl>`:"";}
function equipmentCosts(rows){return Array.isArray(rows)&&rows.length?`<dl class="eq-costs">${rows.map(row=>`<div><dt>${equipmentText(row.label)}</dt><dd>${equipmentText(equipmentMoney(row.amount_bn))}</dd>${row.period?`<small>${equipmentText(row.period)}</small>`:""}</div>`).join("")}</dl>`:"";}
function equipmentProgress(value){if(!Number.isFinite(value))return "";const fraction=Math.max(0,Math.min(1,value)),percent=Math.round(fraction*1000)/10;
  return `<progress class="eq-progress" value="${fraction}" max="1" aria-label="Completed work"></progress><p class="eq-field-note">${fraction>0&&percent===0?"&lt;0.1":equipmentNumber(percent)}% complete</p>`;}
function equipmentActionButton(action,index,scope="data",primary=false){
  if(!action||(!action.command&&!action.navigate))return "";
  const current=scope==="preview"?equipmentPreviewCurrent():equipmentCurrent();
  const enabled=current&&action.enabled!==false&&action.available!==false;
  return `<button type="button" ${primary?'class="eq-primary" ':""}data-equipment-action="${equipmentText(index)}" data-equipment-scope="${scope}" ${enabled?"":"disabled"}${action.reason?` title="${equipmentText(action.reason)}"`:""}>${equipmentText(action.label||"Review")}</button>`;
}
function equipmentActions(rows,path,scope="data"){return Array.isArray(rows)?rows.map((action,index)=>equipmentActionButton(action,`${path}.${index}`,scope,index===0)).join(""):"";}
function equipmentActionAt(path,scope){
  let value=scope==="intent"?EQUIP.review?.quote:scope==="preview"?EQUIP.preview:EQUIP.data;
  for(const key of String(path).split(".")){if(!value||["__proto__","prototype","constructor"].includes(key))return null;value=value[key];}
  return value&&typeof value==="object"?value:null;
}
function equipmentBlockers(rows){return Array.isArray(rows)&&rows.length?`<ul class="eq-blockers">${rows.map(row=>`<li>${equipmentText(typeof row==="string"?row:row.reason||row.detail||row.label)}</li>`).join("")}</ul>`:"";}
function equipmentRequirements(rows,key="order-requirements"){return Array.isArray(rows)&&rows.length?`<details class="eq-details" data-equipment-detail="${key}" ${EQUIP.details.has(key)?"open":""}><summary>Inputs and conditions</summary><ul>${rows.map(row=>`<li>${equipmentText(typeof row==="string"?row:row.detail||row.label)}</li>`).join("")}</ul></details>`:"";}
function equipmentComponent(id){return equipmentRows("components").find(row=>row.id===id);}
function equipmentPlatform(){return equipmentRows("platforms").find(row=>row.id===EQUIP.draft?.platform);}
function equipmentFamily(platform){return platform?.family||(/^tank_|^mbt/.test(platform?.id||"")?"tanks":platform?.id||"ground");}
function equipmentFamilyLabel(platform){return platform?.family_name||platform?.family_label||(equipmentFamily(platform)==="tanks"?"Tanks":platform?.name||"Ground vehicles");}
function equipmentBaseline(row,preset=false){const wanted=preset?`preset:${row.id}`:row.source_revision??row.revision_id??row.id;return equipmentRows("comparison_options").find(option=>String(option.id)===String(wanted))?.id??null;}
function equipmentFamilyHtml(){
  const seen=new Set(),platform=equipmentPlatform(),families=equipmentRows("platforms").filter(row=>{const key=equipmentFamily(row);if(seen.has(key))return false;seen.add(key);return true;});
  return families.length>1?`<div class="eq-family-picker" role="group" aria-label="Vehicle family">${families.map(row=>`<button type="button" data-equipment-family="${equipmentText(equipmentFamily(row))}" data-equipment-focus="family:${equipmentText(equipmentFamily(row))}" aria-pressed="${equipmentFamily(row)===equipmentFamily(platform)}" ${equipmentPending()?"disabled":""}>${equipmentText(equipmentFamilyLabel(row))}</button>`).join("")}</div>`:"";
}
function equipmentChangePlatform(id){
  const platform=equipmentRows("platforms").find(row=>row.id===id);if(!platform||!EQUIP.draft||equipmentPending())return false;
  const preset=equipmentRows("presets").find(row=>(row.editable_spec||row.spec||row).platform===id),base=platform.default_spec||preset?.editable_spec||preset?.spec||preset;
  const components={};for(const slot of platform.slots||[]){const ids=(slot.components||[]).map(row=>typeof row==="string"?row:row.id),old=EQUIP.draft.components?.[slot.id],fallback=base?.components?.[slot.id];if(old&&ids.includes(old))components[slot.id]=old;else if(fallback&&ids.includes(fallback))components[slot.id]=fallback;}
  const automatic=EQUIP.automaticName!=null&&EQUIP.draft.name===EQUIP.automaticName,name=automatic?(preset?.name||platform.name||"New vehicle model"):EQUIP.draft.name;
  EQUIP.automaticName=automatic?name:null;EQUIP.draft={name,platform:id,components};EQUIP.preset=null;EQUIP.comparisonId=preset?equipmentBaseline(preset,true):null;EQUIP.selectedSlot=null;equipmentDraftChanged();return true;
}
function equipmentModelHtml(){return `<div class="eq-model-viewer" data-equipment-model>
  <div class="eq-model-canvas" data-model-canvas></div>
  <p class="eq-model-status" data-model-status role="status">Preparing the 3D model…</p>
  <div class="eq-model-controls"><div class="eq-model-views" role="group" aria-label="Model viewpoint">${[["hero","Perspective"],["front","Front"],["side","Side"],["rear","Rear"],["top","Top"]].map(([key,label])=>`<button type="button" data-model-view="${key}" data-equipment-focus="model-view:${key}" aria-pressed="${key==="hero"}">${label}</button>`).join("")}</div>
  <div class="eq-model-tools"><div class="eq-model-orbit" role="group" aria-label="Rotate model">${[["left","↶","Rotate left"],["right","↷","Rotate right"],["up","↑","Tilt up"],["down","↓","Tilt down"]].map(([key,label,title])=>`<button type="button" data-model-rotate="${key}" data-equipment-focus="model-rotate:${key}" aria-label="${title}" title="${title}">${label}</button>`).join("")}</div>
  <div class="eq-model-zoom" role="group" aria-label="Model zoom"><button type="button" data-model-zoom="out" data-equipment-focus="model-zoom:out" aria-label="Zoom out" title="Zoom out">−</button><button type="button" data-model-zoom="in" data-equipment-focus="model-zoom:in" aria-label="Zoom in" title="Zoom in">+</button></div>
  <button type="button" data-model-reset data-equipment-focus="model-reset">Reset view</button><button type="button" data-model-turntable data-equipment-focus="model-turntable" aria-pressed="false">Auto-rotate</button></div>
  <label class="eq-model-part"><span>Inspect a visible part</span><select data-model-part data-equipment-focus="model-part"><option value="">Choose a part on the model</option></select></label>
  <div class="eq-model-footer"><div class="eq-model-finishes" role="group" aria-label="Model paint finish">${[["olive","Olive"],["sand","Sand"],["winter","Winter"]].map(([key,label])=>`<button type="button" data-model-finish="${key}" data-equipment-focus="model-finish:${key}" aria-pressed="${key==="olive"}"><span aria-hidden="true"></span>${label}</button>`).join("")}</div><button type="button" data-model-export data-equipment-focus="model-export">Download 3D model</button></div></div>
  <p class="eq-model-help">Select a part to open its specification. Drag to orbit · Scroll to zoom · Arrow keys adjust the view.</p></div>`;}
function equipmentDisposeModel(){
  const controller=EQUIPMENT_VIEWER.controller;EQUIPMENT_VIEWER.controller=null;EQUIPMENT_VIEWER.key=null;EQUIPMENT_VIEWER.host=null;
  controller?.dispose();
}
function equipmentSyncModel(root){
  const placeholder=root.querySelector("[data-equipment-model]");
  if(!placeholder||!EQUIP.draft||EQUIP.tab!=="designer"){equipmentDisposeModel();return;}
  if(EQUIPMENT_VIEWER.host&&placeholder!==EQUIPMENT_VIEWER.host)placeholder.replaceWith(EQUIPMENT_VIEWER.host);
  else EQUIPMENT_VIEWER.host=placeholder;
  const host=EQUIPMENT_VIEWER.host,spec={platform:EQUIP.draft.platform,components:equipmentCopy(EQUIP.draft.components||{}),name:EQUIP.draft.name},key=JSON.stringify(spec);
  if(!EQUIPMENT_VIEWER.controller){
    try{EQUIPMENT_VIEWER.controller=window.EquipmentModel?.mount(host,spec)||null;if(EQUIPMENT_VIEWER.controller)EQUIPMENT_VIEWER.key=key;}
    catch(error){const status=host.querySelector("[data-model-status]");if(status)status.textContent="The 3D preview could not load. Your component choices and design review remain available.";}
    if(!window.EquipmentModel){const status=host.querySelector("[data-model-status]");if(status)status.textContent="The 3D preview is unavailable. Your component choices and design review remain available.";}
  }
  if(EQUIPMENT_VIEWER.controller&&key!==EQUIPMENT_VIEWER.key){EQUIPMENT_VIEWER.controller.update(spec);EQUIPMENT_VIEWER.key=key;}
  EQUIPMENT_VIEWER.controller?.resize();
}
function equipmentPresetHtml(){const family=equipmentFamily(equipmentPlatform()),rows=equipmentRows("presets").filter(row=>equipmentFamily(equipmentRows("platforms").find(platform=>platform.id===(row.editable_spec||row.spec||row).platform))===family);return rows.length?`<div class="eq-presets" aria-label="Starting configurations">${rows.map(row=>`<button type="button" class="eq-preset" data-equipment-preset="${equipmentText(row.id)}" aria-pressed="${EQUIP.preset===row.id}" ${equipmentPending()?"disabled":""}><strong>${equipmentText(row.name)}</strong><small>${equipmentText(row.description||row.detail)}</small></button>`).join("")}</div>`:'<p class="eq-message">This family has no starting configuration. Choose its individual specifications below.</p>';}
function equipmentSlotHtml(slot){
  const selected=EQUIP.draft?.components?.[slot.id],component=equipmentComponent(selected);
  const ids=Array.isArray(slot.components)?slot.components:equipmentRows("components").filter(row=>row.slot===slot.id||row.family===slot.id).map(row=>row.id);
  const options=ids.map(id=>equipmentComponent(typeof id==="string"?id:id.id)).filter(Boolean);
  return `<div class="eq-slot${EQUIP.selectedSlot===slot.id?" eq-slot-selected":""}" data-equipment-specification="${equipmentText(slot.id)}"><label><span>${equipmentText(slot.name||slot.id)}${slot.required?" · required":""}</span><select data-equipment-slot="${equipmentText(slot.id)}" data-equipment-focus="slot:${equipmentText(slot.id)}" ${equipmentPending()?"disabled":""}><option value="">${slot.required?"Choose a system":"No optional system"}</option>${options.map(row=>`<option value="${equipmentText(row.id)}" ${row.id===selected?"selected":""}>${equipmentText(row.name)}${row.known===false?" · research needed":""}</option>`).join("")}</select></label>
    ${component?`<p class="eq-field-note">${equipmentText(component.description||component.detail)}</p>${component.reason?`<p class="eq-field-note">${equipmentText(component.reason)}</p>`:""}${Array.isArray(component.tradeoffs)&&component.tradeoffs.length?`<p class="eq-field-note">${component.tradeoffs.map(equipmentText).join(" · ")}</p>`:""}${component.tech?`<div class="eq-actions"><button type="button" data-equipment-research="${equipmentText(component.id)}" ${equipmentCurrent()?"":"disabled"}>${component.known===false?"Review required research":"View related research"}</button></div>`:""}`:""}</div>`;
}
function equipmentSpecificationGroups(slots){
  const groups=[['drive','Engine & running gear',['mobility','transmission','tracks','wheels','suspension']],['weapon','Turret & armament',['turret','armament','ammunition','artillery_loader']],['armor','Protection & troop compartment',['protection','active_protection','troop_compartment']],['electronics','Observation & control',['sensors','fire_control','communications','recon_package','radar']]];
  const assigned=new Set(groups.flatMap(g=>g[2]));
  return groups.map(([id,name,keys])=>{const rows=slots.filter(slot=>keys.includes(slot.id));return rows.length?`<details class="eq-spec-group" data-equipment-detail="spec:${id}" ${EQUIP.details.has(`spec:${id}`)?'open':''}><summary>${name}<small>${rows.length} specifications</small></summary>${rows.map(equipmentSlotHtml).join('')}</details>`:'';}).join('')+slots.filter(slot=>!assigned.has(slot.id)).map(equipmentSlotHtml).join('');
}
function equipmentPreviewHtml(){
  const preview=EQUIP.preview;
  let status=EQUIP.previewLoading?'<p class="eq-message" role="status">Updating this design’s effects…</p>':EQUIP.previewError?`<div class="eq-message error" role="alert"><p>${equipmentText(EQUIP.previewError)}</p><button type="button" data-equipment-preview-retry data-equipment-focus="preview-retry">Retry design review</button></div>`:"";
  if(!preview)return `<section class="eq-panel eq-preview" aria-labelledby="equipmentReviewTitle"><h3 id="equipmentReviewTitle">Review before committing</h3>${status||'<p>Your component choices will be priced and checked here.</p>'}</section>`;
  return `<section class="eq-panel eq-preview" aria-labelledby="equipmentReviewTitle"><p class="eq-eyebrow">Design review</p><h3 id="equipmentReviewTitle">${preview.valid?"Ready for the next step":"Resolve the requirements"}</h3>${status}${preview.detail?`<p>${equipmentText(preview.detail)}</p>`:""}${equipmentBlockers(preview.blockers)}${equipmentMetrics(preview.metrics)}${equipmentCosts(preview.costs)}
    ${Array.isArray(preview.timing)&&preview.timing.length?`<dl class="eq-costs">${preview.timing.map(row=>`<div><dt>${equipmentText(row.label)}</dt><dd>${equipmentText(row.value??"—")}</dd></div>`).join("")}</dl>`:""}
    ${Array.isArray(preview.requirements)&&preview.requirements.length?`<details class="eq-details" data-equipment-detail="requirements" ${EQUIP.details.has("requirements")?"open":""}><summary>Requirements and assumptions</summary><ul>${preview.requirements.map(row=>`<li>${equipmentText(typeof row==="string"?row:row.detail||row.label)}</li>`).join("")}</ul></details>`:""}
    <div class="eq-actions">${equipmentActions(preview.actions,"actions","preview")}</div></section>`;
}
function equipmentComparisonValue(value,unit){if(!Number.isFinite(value))return "—";return unit==="bn"?equipmentMoney(value):`${equipmentNumber(value)}${unit?` ${unit}`:""}`;}
function equipmentComparisonHtml(){
  const options=equipmentRows("comparison_options"),comparison=EQUIP.preview?.comparison;
  if(!options.length&&!comparison)return "";
  const selected=EQUIP.comparisonId??comparison?.id;
  const chooser=options.length?`<label class="eq-comparison-picker"><span>Compare this design with</span><select id="equipmentComparison" data-equipment-focus="comparison" ${equipmentPending()?"disabled":""}><option value="" ${selected==null?"selected":""}>Suggested starting configuration</option>${options.map(row=>`<option value="${equipmentText(row.id)}" ${String(row.id)===String(selected)?"selected":""}>${equipmentText(row.name)}${row.kind?` · ${equipmentText(row.kind)}`:""}</option>`).join("")}</select></label>`:"";
  const rows=Array.isArray(comparison?.rows)?comparison.rows:[],changes=Array.isArray(comparison?.changes)?comparison.changes:[];
  return `<section class="eq-panel eq-comparison" aria-labelledby="equipmentComparisonTitle"><div class="eq-section-heading"><div><p class="eq-eyebrow">Upgrade analysis</p><h2 id="equipmentComparisonTitle">What changes with this model?</h2><p>${equipmentText(comparison?.detail||"Choose a starting configuration or a saved revision to compare capability, expense and development time.")}</p></div>${chooser}</div>
    ${EQUIP.previewLoading?'<p class="eq-message" role="status">Updating the comparison…</p>':!comparison?.available?`<p class="eq-field-note">${equipmentText(comparison?.detail||"A fresh design review will supply this comparison.")}</p>`:`${comparison.same_role===false?'<p class="eq-message">These vehicles serve different roles. Consider the role ratings as well as their cost.</p>':""}<div class="eq-comparison-scroll" tabindex="0" role="region" aria-label="Design comparison ratings and costs"><table class="eq-comparison-table"><caption>${equipmentText(comparison.name||"Starting configuration")} → ${equipmentText(EQUIP.draft?.name||"Current design")}</caption><thead><tr><th scope="col">Specification</th><th scope="col">Before</th><th scope="col">Your design</th><th scope="col">Change</th></tr></thead><tbody>${rows.map(row=>{const direction=Number.isFinite(row.delta)&&row.delta!==0&&row.better_when!=="neutral"?(row.delta>0===(row.better_when==="higher")?"gain":"tradeoff"):"neutral";return `<tr><th scope="row">${equipmentText(row.label||row.key)}</th><td>${equipmentText(equipmentComparisonValue(row.before,row.unit))}</td><td>${equipmentText(equipmentComparisonValue(row.after,row.unit))}</td><td class="eq-delta-${direction}">${Number.isFinite(row.delta)&&row.delta>0?"+":""}${equipmentText(equipmentComparisonValue(row.delta,row.unit))}${direction!=="neutral"?`<small>${direction==="gain"?"Improvement":"Tradeoff"}</small>`:""}</td></tr>`;}).join("")}</tbody></table></div>${changes.length?`<div class="eq-component-changes"><h3>Component changes</h3><ul>${changes.map(row=>`<li><strong>${equipmentText(row.label||row.slot)}</strong><span>${equipmentText(row.before||"None")} <span aria-label="changes to">→</span> ${equipmentText(row.after||"None")}</span></li>`).join("")}</ul></div>`:'<p class="eq-field-note">The component configuration matches this comparison model.</p>'}`}</section>`;
}
function equipmentDesignerHtml(){
  if(!EQUIP.draft)return `<div class="eq-empty"><h2>No available vehicle configurations</h2><p>${equipmentText(EQUIP.data?.reason||"The current catalogue has no starting platform. Review research or return to the library.")}</p></div>`;
  const platform=equipmentPlatform(),slots=Array.isArray(platform?.slots)?platform.slots:[];
  return `<div class="eq-section-heading eq-designer-heading"><div><h2>Make a model that fits your force</h2><p>Choose a vehicle family and starting configuration, then inspect and customize every system.</p></div></div>${equipmentFamilyHtml()}${equipmentPresetHtml()}
    <div class="eq-design-grid"><section class="eq-panel" aria-labelledby="equipmentConfigurationTitle"><h3 id="equipmentConfigurationTitle">Configuration</h3><div class="eq-fields"><label><span>Model name</span><input id="equipmentName" data-equipment-focus="name" maxlength="80" value="${equipmentText(EQUIP.draft.name)}" placeholder="Name your model" ${equipmentPending()?"disabled":""}></label>
    <label><span>Vehicle platform</span><select id="equipmentPlatform" data-equipment-focus="platform" ${equipmentPending()?"disabled":""}>${equipmentRows("platforms").filter(row=>equipmentFamily(row)===equipmentFamily(platform)).map(row=>`<option value="${equipmentText(row.id)}" ${row.id===EQUIP.draft.platform?"selected":""}>${equipmentText(row.name)}</option>`).join("")}</select></label></div>
    ${platform?.description?`<p class="eq-field-note">${equipmentText(platform.description)}</p>`:""}<div class="eq-specifications"><h4>Individual specifications</h4><p class="eq-field-note">Open a group to choose each part. Review compatibility and total costs alongside the model.</p>${equipmentSpecificationGroups(slots)}</div></section>
    <section class="eq-visual" aria-label="Interactive ground vehicle model"><div class="eq-visual-heading"><div><p class="eq-eyebrow">3D model · Live configuration</p><h2>${equipmentText(EQUIP.draft.name||"Unnamed model")}</h2></div><span class="eq-status">${equipmentText(platform?.name||"Ground vehicle")}</span></div>${equipmentModelHtml()}<div class="eq-labels">${slots.map(slot=>{const part=equipmentComponent(EQUIP.draft.components?.[slot.id]);return part?`<button type="button" data-equipment-inspect="${equipmentText(slot.id)}">${equipmentText(part.name)}</button>`:"";}).join("")}</div><p class="eq-visual-note">Exterior specifications change the model’s visible equipment. Internal ammunition loads affect ratings and export metadata. Paint changes appearance only. Review the game ratings and costs before placing an order.</p></section>${equipmentPreviewHtml()}</div>${equipmentComparisonHtml()}`;
}
function equipmentRecordHtml(row,index,key){
  return `<article class="eq-card" data-equipment-record="${equipmentText(row.id)}" tabindex="-1"><div class="eq-card-header"><h3>${equipmentText(row.name||row.id)}</h3>${row.status?`<span class="eq-status">${equipmentText(row.status)}</span>`:""}</div>${row.detail?`<p>${equipmentText(row.detail)}</p>`:""}${equipmentProgress(row.progress)}${equipmentMetrics(row.metrics)}${equipmentCosts(row.costs)}
    ${row.receipt_label?`<p class="eq-receipt">Recorded · ${equipmentText(row.receipt_label)}</p>`:row.awaiting_receipt?'<p class="eq-receipt">Awaiting the first recorded operation</p>':""}${equipmentBlockers(row.blockers)}
    <div class="eq-actions">${key==="designs"&&row.spec?`<button type="button" data-equipment-edit="${equipmentText(row.id)}" ${equipmentPending()?"disabled":""}>Use as a starting point</button>`:""}${equipmentActions(row.actions,`${key}.${index}.actions`)}</div></article>`;
}
function equipmentCollectionHtml(key,title,description,empty){
  const rows=equipmentRows(key),query=EQUIP.query.trim().toLocaleLowerCase("en-US");
  const filtered=rows.map((row,index)=>({row,index})).filter(({row})=>!query||`${row.name||""} ${row.status||""} ${row.detail||""}`.toLocaleLowerCase("en-US").includes(query));
  return `<div class="eq-section-heading"><div><h2>${equipmentText(title)}</h2><p>${equipmentText(description)}</p></div>${key==="designs"?'<button type="button" class="eq-primary" data-equipment-tab="designer">Design a vehicle</button>':""}</div>
    ${rows.length?`<div class="eq-toolbar"><label><span>Find a model or programme</span><input id="equipmentSearch" data-equipment-focus="search" value="${equipmentText(EQUIP.query)}" placeholder="Search by name or status"></label><p>${filtered.length} of ${rows.length} shown</p></div>`:""}
    ${filtered.length?`<div class="eq-card-grid">${filtered.map(({row,index})=>equipmentRecordHtml(row,index,key)).join("")}</div>`:`<div class="eq-empty"><h3>${query?"No matches":"Nothing here yet"}</h3><p>${equipmentText(query?"Try another name or clear your search.":empty)}</p><div class="eq-actions">${query?'<button type="button" data-equipment-clear>Clear search</button>':'<button type="button" data-equipment-tab="designer">Open the designer</button>'}</div></div>`}`;
}
function equipmentResearchState(row){return EQUIPMENT_RESEARCH_STATES.some(([id])=>id===row.state&&id!=="all")?row.state:/known|complete/i.test(row.status||"")?"known":/progress|researching/i.test(row.status||"")?"researching":/locked|require/i.test(row.status||"")?"locked":"available";}
function equipmentResearchBranch(row){return EQUIPMENT_BRANCHES.some(([id])=>id===row.branch)?row.branch:"optics";}
function equipmentResearchDepth(row,rows,seen=new Set()){
  if(seen.has(row.id))return 0;const next=new Set(seen);next.add(row.id);
  const parents=(row.prerequisites||[]).map(prerequisite=>rows.find(candidate=>String(candidate.id)===String(prerequisite.id)&&equipmentResearchBranch(candidate)===equipmentResearchBranch(row))).filter(Boolean);
  return parents.length?1+Math.max(...parents.map(parent=>equipmentResearchDepth(parent,rows,next))):0;
}
function equipmentResearchEdges(rows){const ids=new Set(rows.map(row=>String(row.id)));return rows.flatMap(row=>(row.prerequisites||[]).filter(parent=>ids.has(String(parent.id))&&String(parent.id)!==String(row.id)).map(parent=>({from:String(parent.id),to:String(row.id)})));}
function equipmentSyncResearchTree(root){
  EQUIPMENT_TREE.observer?.disconnect();EQUIPMENT_TREE.observer=null;
  const tree=root.querySelector("[data-equipment-tree]"),lines=root.querySelector("[data-equipment-tree-lines]");if(!tree?.getBoundingClientRect||!lines)return;
  const nodes=[...tree.querySelectorAll("[data-equipment-record]")],visible=new Map(nodes.map(node=>[node.dataset.equipmentRecord,node])),rows=equipmentRows("research").filter(row=>visible.has(String(row.id))),edges=equipmentResearchEdges(rows);
  const draw=()=>{const bounds=tree.getBoundingClientRect(),rects=new Map(nodes.map(node=>[node.dataset.equipmentRecord,node.getBoundingClientRect()]));lines.setAttribute("width",Math.max(tree.clientWidth,...[...rects.values()].map(rect=>rect.right-bounds.left+tree.scrollLeft+4)));lines.setAttribute("height",Math.max(tree.clientHeight,...[...rects.values()].map(rect=>rect.bottom-bounds.top+tree.scrollTop+4)));lines.innerHTML=edges.map(edge=>{const from=rects.get(edge.from),to=rects.get(edge.to);if(to.left<=from.left)return "";const x1=from.right-bounds.left+tree.scrollLeft,y1=from.top-bounds.top+tree.scrollTop+Math.min(60,from.height/2),x2=to.left-bounds.left+tree.scrollLeft,y2=to.top-bounds.top+tree.scrollTop+Math.min(60,to.height/2),mid=(x1+x2)/2;return `<path d="M${x1} ${y1} C${mid} ${y1} ${mid} ${y2} ${x2} ${y2}"/>`;}).join("");};
  draw();if(typeof ResizeObserver!=="undefined"){EQUIPMENT_TREE.observer=new ResizeObserver(draw);EQUIPMENT_TREE.observer.observe(tree);nodes.forEach(node=>EQUIPMENT_TREE.observer.observe(node));}
}
function equipmentResearchNodeHtml(row,index,depth){
  const state=equipmentResearchState(row),stateName=EQUIPMENT_RESEARCH_STATES.find(([id])=>id===state)?.[1],branch=equipmentResearchBranch(row),prerequisites=Array.isArray(row.prerequisites)?row.prerequisites:[];
  const unlocks=Array.isArray(row.unlocks)&&row.unlocks.length?row.unlocks:(row.unlock_components||[]).map(component=>component.name||component.id);
  const unlockItem=unlock=>`<li>${equipmentText(typeof unlock==="string"?unlock:unlock.name||unlock.label)}</li>`,unlockKey=`unlocks:${row.id}`;
  const unlockList=`<ul>${unlocks.slice(0,5).map(unlockItem).join("")}</ul>${unlocks.length>5?`<details class="eq-details" data-equipment-detail="${equipmentText(unlockKey)}" ${EQUIP.details.has(unlockKey)?"open":""}><summary>${unlocks.length-5} more available systems</summary><ul>${unlocks.slice(5).map(unlockItem).join("")}</ul></details>`:""}`;
  return `<article class="eq-research-node" data-equipment-research-state="${state}" data-equipment-record="${equipmentText(row.id)}" data-equipment-focus="research:${equipmentText(row.id)}" tabindex="-1"><div class="eq-research-illustration"><img src="/art/components/${branch}-v1.webp" alt="" loading="lazy" width="640" height="360"><span class="eq-state-badge">${equipmentText(stateName)}</span></div><div class="eq-research-node-body"><p class="eq-eyebrow">${depth?`Technology stage ${depth+1}`:"Foundation"}</p><h3>${equipmentText(row.name||row.id)}</h3>${row.detail?`<p>${equipmentText(row.detail)}</p>`:""}${state==="researching"?equipmentProgress(row.progress):""}${equipmentMetrics(row.metrics)}
    <div class="eq-prerequisites"><h4>${prerequisites.length?"Requires":"Entry point"}</h4>${prerequisites.length?`<ul>${prerequisites.map((prerequisite,prerequisiteIndex)=>`<li><span aria-label="${prerequisite.known?"Known":"Not yet known"}" class="eq-prerequisite-state">${prerequisite.known?"✓":"○"}</span><button type="button" data-equipment-prerequisite="${index}:${prerequisiteIndex}" ${equipmentCurrent()?"":"disabled"}>${equipmentText(prerequisite.name||prerequisite.id)}</button></li>`).join("")}</ul>`:'<p>No prerequisite technology listed.</p>'}</div>
    ${unlocks.length?`<div class="eq-unlocks"><h4>Unlocks for your designs</h4>${unlockList}</div>`:""}${equipmentBlockers(row.blockers)}<div class="eq-actions">${equipmentActions(row.actions,`research.${index}.actions`)}</div></div></article>`;
}
function equipmentResearchHtml(){
  const rows=equipmentRows("research"),query=EQUIP.query.trim().toLocaleLowerCase("en-US"),branch=EQUIPMENT_BRANCHES.find(([id])=>id===EQUIP.researchBranch)||EQUIPMENT_BRANCHES[0];
  const filtered=rows.map((row,index)=>({row,index,depth:equipmentResearchDepth(row,rows)})).filter(({row})=>equipmentResearchBranch(row)===branch[0]&&(EQUIP.researchState==="all"||equipmentResearchState(row)===EQUIP.researchState)&&(!query||[row.name,row.detail,...(row.unlocks||[]),...(row.unlock_components||[]).map(component=>component.name),...(row.prerequisites||[]).map(prerequisite=>prerequisite.name)].join(" ").toLocaleLowerCase("en-US").includes(query)));
  const depths=[...new Set(filtered.map(item=>item.depth))].sort((a,b)=>a-b);
  return `<div class="eq-section-heading"><div><p class="eq-eyebrow">Military technology</p><h2>Build the systems. Shape the force.</h2><p>Follow component prerequisites across six branches. Research unlocks reusable parts; your vehicle designs combine them into a model.</p></div></div>
    <nav class="eq-research-branches" aria-label="Military research branches">${EQUIPMENT_BRANCHES.map(([id,label,detail])=>{const count=rows.filter(row=>equipmentResearchBranch(row)===id),known=count.filter(row=>equipmentResearchState(row)==="known").length;return `<button type="button" data-equipment-branch="${id}" data-equipment-focus="branch:${id}" aria-pressed="${branch[0]===id}" title="${detail}"><img src="/art/components/${id}-v1.webp" alt="" width="320" height="180"><span><strong>${label}</strong><small>${known} / ${count.length} known</small></span></button>`;}).join("")}</nav>
    <div class="eq-research-tools"><div class="eq-research-filters" role="group" aria-label="Research state">${EQUIPMENT_RESEARCH_STATES.map(([id,label])=>`<button type="button" data-equipment-research-filter="${id}" data-equipment-focus="research-filter:${id}" aria-pressed="${EQUIP.researchState===id}">${label}</button>`).join("")}</div><label><span>Search this branch</span><input id="equipmentSearch" data-equipment-focus="search" value="${equipmentText(EQUIP.query)}" placeholder="Technology, component or prerequisite"></label></div>
    <div class="eq-section-heading eq-tree-heading"><div><h2>${branch[1]}</h2><p>${branch[2]}. Select a prerequisite to follow its research path.</p></div><p role="status">${filtered.length} ${filtered.length===1?"technology":"technologies"} shown</p></div>
    ${filtered.length?`<div class="eq-research-tree" data-equipment-tree role="region" aria-label="${branch[1]} prerequisite tree" tabindex="0"><svg class="eq-tree-lines" data-equipment-tree-lines aria-hidden="true"></svg>${depths.map(depth=>`<section class="eq-research-tier" aria-label="Technology stage ${depth+1}"><p class="eq-tier-label">${depth?`Stage ${depth+1}`:"Foundations"}<span aria-hidden="true">${depth?"→":"◇"}</span></p>${filtered.filter(item=>item.depth===depth).map(({row,index})=>equipmentResearchNodeHtml(row,index,depth)).join("")}</section>`).join("")}</div>`:`<div class="eq-empty"><h3>${rows.length?"No technologies match this view":"No component research is listed"}</h3><p>${rows.length?"Choose another branch or clear the filters to see its research path.":"The designer still shows the requirements for each component in this campaign."}</p><div class="eq-actions"><button type="button" data-equipment-research-reset>Clear research filters</button><button type="button" data-equipment-tab="designer">Open the designer</button></div></div>`}`;
}
function equipmentModernizationHtml(){
  const rows=equipmentRows("modernization");
  return `<section class="eq-modernization" aria-labelledby="equipmentModernizationTitle"><div class="eq-section-heading"><div><p class="eq-eyebrow">Fleet planning</p><h2 id="equipmentModernizationTitle">Suggested modernization</h2><p>Review why a change is useful, what it costs, and which equipment it would affect.</p></div></div>${rows.length?`<div class="eq-card-grid">${rows.map((row,index)=>`<article class="eq-card eq-modernization-card"><div class="eq-card-header"><h3>${equipmentText(row.name)}</h3>${row.status?`<span class="eq-status">${equipmentText(row.status)}</span>`:""}</div>${row.detail?`<p>${equipmentText(row.detail)}</p>`:""}<div class="eq-recommendation-reason"><h4>Why this helps</h4><p>${equipmentText(row.reason||"Review the supplied capability and cost effects.")}</p></div>${row.tradeoff?`<div class="eq-recommendation-tradeoff"><h4>The tradeoff</h4><p>${equipmentText(row.tradeoff)}</p></div>`:""}${equipmentMetrics(row.metrics)}${equipmentCosts(row.costs)}${equipmentBlockers(row.blockers)}<div class="eq-actions">${row.spec?`<button type="button" data-equipment-modernization="${index}" ${equipmentCurrent()?"":"disabled"}>Explore this design</button>`:""}${equipmentActions(row.actions,`modernization.${index}.actions`)}</div></article>`).join("")}</div>`:'<div class="eq-empty"><h3>No modernization opportunities identified</h3><p>As your research and fleet develop, suggested upgrades and compatible refits will appear here. You can explore a new vehicle in the designer at any time.</p><button type="button" data-equipment-tab="designer">Explore a vehicle design</button></div>'}</section>`;
}
function equipmentReviewHtml(){
  const review=EQUIP.review;if(!review)return "";
  const inputs=(Array.isArray(review.action.inputs)?review.action.inputs:[]).map(field=>{
    const value=review.command[field.key]??"",disabled=equipmentPending()?"disabled":"";
    return `<label><span>${equipmentText(field.label||field.key)}${field.unit?` · ${equipmentText(field.unit)}`:""}</span>${field.type==="select"?`<select data-equipment-order-input="${equipmentText(field.key)}" data-equipment-focus="order:${equipmentText(field.key)}" ${disabled}>${(Array.isArray(field.options)?field.options:[]).map(option=>`<option value="${equipmentText(option.value)}" ${String(option.value)===String(value)?"selected":""} ${option.enabled===false?"disabled":""}>${equipmentText(option.label||option.value)}${option.reason?` · ${equipmentText(option.reason)}`:""}</option>`).join("")}</select>`:`<input type="number" data-equipment-order-input="${equipmentText(field.key)}" data-equipment-focus="order:${equipmentText(field.key)}" value="${equipmentText(value)}" ${Number.isFinite(field.min)?`min="${field.min}"`:""} ${Number.isFinite(field.max)?`max="${field.max}"`:""} step="${Number.isFinite(field.step)?field.step:"any"}" ${disabled}>`}</label>`;
  }).join("");
  const quoted=review.action.requires_preview===true,quote=review.quote;
  const body=quoted?`${review.loading?'<p class="eq-message" role="status">Checking these order settings…</p>':""}${review.error?`<div class="eq-message error" role="alert"><p>${equipmentText(review.error)}</p><button type="button" data-equipment-order-retry>Retry order review</button></div>`:""}${quote?`${equipmentBlockers(quote.blockers)}${equipmentMetrics(quote.metrics)}${equipmentCosts(quote.costs)}${Array.isArray(quote.timing)?`<dl class="eq-costs">${quote.timing.map(row=>`<div><dt>${equipmentText(row.label)}</dt><dd>${equipmentText(row.value??"—")}</dd></div>`).join("")}</dl>`:""}${quote.detail?`<p>${equipmentText(quote.detail)}</p>`:""}${equipmentRequirements(quote.requirements)}`:""}`:`${equipmentMetrics(review.action.metrics)}${equipmentCosts(review.action.costs)}`;
  const actions=quoted?(quote?.actions||[]).map((action,index)=>`<button type="button" ${index===0?'class="eq-primary" ':""}data-equipment-intent="${index}" ${equipmentReviewQuoteCurrent()&&action.enabled!==false&&(!action.command||quote.valid)?"":"disabled"}>${equipmentText(action.label||"Confirm")}</button>`).join(""):`<button type="button" class="eq-primary" data-equipment-confirm ${equipmentReviewCurrent()?"":"disabled"}>${equipmentText(review.action.confirm_label||review.action.label||"Confirm")}</button>`;
  return `<section class="eq-review" role="region" aria-labelledby="equipmentActionReviewTitle"><h2 id="equipmentActionReviewTitle" tabindex="-1" data-equipment-focus="review">${equipmentText(review.action.label||"Review this action")}</h2><p>${equipmentText(review.action.detail||"Review the configuration, funding and effects before confirming.")}</p>${inputs?`<div class="eq-order-inputs eq-fields">${inputs}</div>`:""}${body}${review.action.reason?`<p>${equipmentText(review.action.reason)}</p>`:""}<div class="eq-actions">${actions}<button type="button" data-equipment-dismiss>Keep reviewing</button></div></section>`;
}
function equipmentContentHtml(){
  const data=EQUIP.data,context=data?`${data.name||data.nation||"Your nation"} · ${data.date||"Current campaign"}`:"Preparing the equipment bureau";
  const header=`<header class="eq-header"><div><p class="eq-eyebrow">Military industry · Ground vehicles</p><h1 id="equipmentTitle" tabindex="-1" data-equipment-focus="title">From concept to service</h1><p>${equipmentText(context)}</p></div><div class="eq-header-actions"><button type="button" data-equipment-refresh data-equipment-focus="refresh" ${EQUIP.loading||equipmentPending()?"disabled":""}>Refresh reading</button><button type="button" data-equipment-close aria-label="Close equipment designer">Back to map</button></div></header>
    <div class="eq-process" aria-label="Equipment lifecycle"><span>Research</span> → <span>Design</span> → <span>Development</span> → <span>Production</span> → <span>Service & refit</span></div>
    <nav class="eq-tabs" role="tablist" aria-label="Equipment workflow">${EQUIPMENT_TABS.map(([key,label])=>`<button type="button" role="tab" data-equipment-tab="${key}" data-equipment-focus="tab:${key}" aria-selected="${EQUIP.tab===key}" tabindex="${EQUIP.tab===key?0:-1}">${label}</button>`).join("")}</nav>`;
  const status=EQUIP.error?`<div class="eq-message error" role="alert"><strong>The equipment reading could not be refreshed.</strong><p>${equipmentText(EQUIP.error)}</p><div class="eq-actions"><button type="button" data-equipment-refresh data-equipment-focus="retry" ${equipmentPending()?"disabled":""}>Retry equipment</button></div></div>`:EQUIP.loading||EQUIP.stale?`<p class="eq-message" role="status">${data?"Updating the equipment reading…":"Loading platforms, components and programmes…"}</p>`:equipmentPending()?'<p class="eq-message" role="status">An order or turn is being resolved. New actions will be available when it finishes.</p>':"";
  if(!data)return header+status;
  const message=EQUIP.message?`<p class="eq-message" role="status">${equipmentText(EQUIP.message)}</p>`:"";
  const unavailable=data.enabled===false?`<div class="eq-message"><strong>Equipment design is not active in this campaign.</strong><p>${equipmentText(data.reason)}</p><div class="eq-actions">${equipmentActions(data.actions,"actions")}</div></div>`:"";
  let body="";
  if(EQUIP.tab==="designer")body=equipmentDesignerHtml();
  else if(EQUIP.tab==="research")body=equipmentResearchHtml();
  else if(EQUIP.tab==="library")body=equipmentCollectionHtml("designs","Your equipment library","Keep useful models, compare revisions, and develop a configuration when it fits your plans.","Save a draft from the designer to start your equipment library.");
  else if(EQUIP.tab==="development")body=equipmentCollectionHtml("development","Development and trials","Fund a specific revision. Pausing preserves its completed work; certification makes it eligible for production.","No development programme is listed. Review a design and fund its development when you are ready.");
  else if(EQUIP.tab==="production")body=equipmentCollectionHtml("production","Build the model you approved","Follow the exact revision through tooling, manufacture and delivery. Funded work and fielded vehicles are separate stages.","No equipment production is listed. A certified design can be assigned to an eligible production site.");
  else body=equipmentModernizationHtml()+equipmentCollectionHtml("lots","Equipment in service","Inspect delivered models and compatible refits. A refit withdraws real equipment until the work is complete.","No designer-built equipment has entered service. Your inherited equipment remains in the existing arsenal.");
  const funding=["development","production","service"].includes(EQUIP.tab)&&Array.isArray(data.funding?.metrics)?`<section class="eq-panel eq-funding"><h3>Current funding</h3>${equipmentMetrics(data.funding.metrics)}${data.funding.note?`<p class="eq-field-note">${equipmentText(data.funding.note)}</p>`:""}</section>`:"";
  return `${header}${status}${message}${unavailable}${equipmentReviewHtml()}${funding}<div role="tabpanel" aria-label="${equipmentText(EQUIPMENT_TABS.find(row=>row[0]===EQUIP.tab)?.[1])}">${body}</div>${data.enabled!==false&&data.actions?.length?`<section class="eq-next"><h3>Funding and facilities</h3><div class="eq-actions">${equipmentActions(data.actions,"actions")}</div></section>`:""}${data.note?`<p class="eq-field-note">${equipmentText(data.note)}</p>`:""}`;
}
function equipmentRememberView(){
  if(!EQUIP.open)return;const root=document.querySelector("#equipmentRoot");if(!root||root.dataset.equipmentSession!==String(EQUIP.session??""))return;
  const scroller=typeof equipmentScroller==="function"?equipmentScroller():root.parentElement;if(scroller)EQUIP.scroll=scroller.scrollTop;
  root.querySelectorAll("details[data-equipment-detail]").forEach(detail=>{const key=detail.dataset.equipmentDetail;if(key==="components")EQUIP.advanced=detail.open;else if(detail.open)EQUIP.details.add(key);else EQUIP.details.delete(key);});
  const active=document.activeElement,key=active?.dataset?.equipmentFocus;
  if(key)EQUIP.focus={key,start:active.selectionStart,end:active.selectionEnd};
}
function equipmentPanelHtml(){equipmentRememberView();return `<div id="equipmentRoot" class="equipment-room" data-equipment-tab-view="${EQUIP.tab}" data-equipment-session="${equipmentText(EQUIP.session)}" aria-busy="${EQUIP.loading}">${equipmentContentHtml()}</div>`;}
function equipmentRender(){if(!equipmentActive())return;const root=document.querySelector("#equipmentRoot");if(!root)return;equipmentRememberView();EQUIPMENT_VIEWER.host?.remove();root.innerHTML=equipmentContentHtml();root.dataset.equipmentSession=String(EQUIP.session??"");root.dataset.equipmentTabView=EQUIP.tab;root.setAttribute("aria-busy",String(EQUIP.loading||EQUIP.previewLoading));equipmentBind(false);}
function equipmentResetCampaign(){
  if(EQUIP.session===S?.session_id&&EQUIP.nation===S?.player)return;
  equipmentDisposeModel();
  Object.assign(EQUIP,{session:S?.session_id,nation:S?.player,data:null,state:null,loading:false,stale:true,error:"",busy:false,message:"",draft:null,preset:null,comparisonId:null,automaticName:null,selectedSlot:null,preview:null,previewState:null,previewLoading:false,previewError:"",previewKey:null,query:"",researchBranch:"chassis",researchState:"all",advanced:false,focus:null,scroll:0,request:null,review:null});EQUIP.details.clear();++EQUIP.seq;++EQUIP.previewSeq;++EQUIP.draftSeq;
}
function equipmentSetDraft(row,{preset=false}={}){
  const spec=row?.editable_spec||row?.spec||row;if(!spec?.platform)return false;
  EQUIP.draft={name:row.name||"New vehicle model",platform:spec.platform,components:equipmentCopy(spec.components||{})};
  if(!preset&&(row.source_revision!=null||row.id!=null))EQUIP.draft.source_revision=row.source_revision??row.revision_id??row.id;
  EQUIP.comparisonId=equipmentBaseline(row,preset);EQUIP.automaticName=preset?EQUIP.draft.name:null;EQUIP.selectedSlot=null;
  EQUIP.message=row.editable_spec&&JSON.stringify(row.editable_spec)!==JSON.stringify(row.spec)?"This creates a new design with separate specifications and a fresh cost review. The original revision remains unchanged.":"";EQUIP.preset=preset?row.id:null;EQUIP.tab="designer";equipmentDraftChanged();return true;
}
function equipmentDefaultDraft(){
  if(EQUIP.draft)return;
  const presets=equipmentRows("presets"),row=presets.find(item=>item.default)||presets.find(item=>/balanced/i.test(item.id)||/balanced/i.test(item.name))||presets[0];
  if(row){const spec=row.editable_spec||row.spec||row;EQUIP.draft={name:row.name||"New vehicle model",platform:spec.platform,components:equipmentCopy(spec.components||{})};EQUIP.preset=row.id;EQUIP.comparisonId=equipmentBaseline(row,true);EQUIP.automaticName=EQUIP.draft.name;}
  else if(equipmentRows("platforms")[0]){const platform=equipmentRows("platforms")[0];EQUIP.draft={name:platform.name||"New vehicle model",platform:platform.id,components:equipmentCopy(platform.default_spec?.components||{})};EQUIP.automaticName=EQUIP.draft.name;}
}
function equipmentDraftChanged(){++EQUIP.draftSeq;++EQUIP.previewSeq;EQUIP.preview=null;EQUIP.previewState=null;EQUIP.previewLoading=false;EQUIP.previewError="";EQUIP.previewKey=null;EQUIP.review=null;equipmentRender();equipmentFetchPreview();}
async function equipmentFetch(){
  if(!equipmentActive()||EQUIP.loading)return false;equipmentResetCampaign();const state=S,seq=++EQUIP.seq;EQUIP.loading=true;EQUIP.stale=true;EQUIP.error="";EQUIP.review=null;equipmentRender();
  try{const data=await api(`/api/equipment?session_id=${encodeURIComponent(state.session_id)}`);if(seq!==EQUIP.seq||state!==S||!equipmentActive())return false;
    if(!data||data.session_id!==state.session_id||data.nation!==state.player||!Array.isArray(data.platforms)||!Array.isArray(data.components)||!Array.isArray(data.designs))throw new Error("The equipment reading did not match this campaign. Refresh to read the current equipment.");
    EQUIP.data=data;EQUIP.state=state;EQUIP.stale=false;equipmentDefaultDraft();
    if(EQUIP.request){const request=EQUIP.request;EQUIP.request=null;const row=equipmentRows(request.preset?"presets":"designs").find(item=>String(item.id)===String(request.preset??request.design??request.revision));if(row){const spec=row.editable_spec||row.spec||row;EQUIP.draft={name:row.name,platform:spec.platform,components:equipmentCopy(spec.components||{})};if(!request.preset)EQUIP.draft.source_revision=row.revision_id??row.id;EQUIP.preset=request.preset??null;EQUIP.comparisonId=equipmentBaseline(row,!!request.preset);EQUIP.automaticName=request.preset?EQUIP.draft.name:null;EQUIP.selectedSlot=null;}}
    return true;
  }catch(error){if(seq===EQUIP.seq&&state===S&&equipmentActive()){EQUIP.error=error.message||"The equipment service could not be reached.";EQUIP.stale=true;}return false;}
  finally{if(seq===EQUIP.seq){EQUIP.loading=false;equipmentRender();if(!EQUIP.stale&&EQUIP.draft)equipmentFetchPreview();}}
}
async function equipmentFetchPreview(){
  if(!equipmentCurrent()||!EQUIP.draft)return false;
  const state=S,seq=++EQUIP.previewSeq,key=equipmentDraftKey(),draft=equipmentCopy(EQUIP.draft);if(EQUIP.comparisonId!=null)draft.comparison_id=EQUIP.comparisonId;EQUIP.previewLoading=true;EQUIP.previewError="";EQUIP.review=null;equipmentRender();
  try{const data=await api("/api/equipment-preview",{session_id:state.session_id,...draft});
    if(seq!==EQUIP.previewSeq||state!==S||!equipmentActive()||key!==equipmentDraftKey())return false;
    if(!data||data.session_id!==state.session_id||data.nation!==state.player||typeof data.valid!=="boolean"||!Array.isArray(data.actions))throw new Error("This design review did not match the current campaign. Request a fresh review.");
    EQUIP.preview=data;EQUIP.previewState=state;EQUIP.previewKey=key;return true;
  }catch(error){if(seq===EQUIP.previewSeq&&state===S&&equipmentActive()){EQUIP.previewError=error.message||"The design could not be reviewed.";EQUIP.previewKey=null;}return false;}
  finally{if(seq===EQUIP.previewSeq){EQUIP.previewLoading=false;equipmentRender();}}
}
function equipmentReviewCurrent(){const r=EQUIP.review;return !!r&&(r.scope==="preview"?equipmentPreviewCurrent(r.data,r.state):equipmentCurrent(r.data,r.state))&&r.action===equipmentActionAt(r.path,r.scope)&&r.action.enabled!==false&&r.action.available!==false;}
function equipmentReviewQuoteCurrent(){const r=EQUIP.review;return equipmentReviewCurrent()&&!!r.quote&&!r.loading&&!r.error&&r.quoteState===S&&r.quoteKey===JSON.stringify(r.command);}
function equipmentInvoke(path,scope,data,state){
  if(!(scope==="preview"?equipmentPreviewCurrent(data,state):equipmentCurrent(data,state)))return false;
  const action=equipmentActionAt(path,scope);if(!action||action.enabled===false||action.available===false)return false;
  if(action.navigate){equipmentNavigate(equipmentCopy(action.navigate));return true;}
  if(!action.command||typeof action.command!=="object")return false;
  const command=equipmentCopy(action.command);
  for(const field of Array.isArray(action.inputs)?action.inputs:[])if(!["__proto__","prototype","constructor"].includes(field.key)&&field.value!==undefined)command[field.key]=field.value;
  EQUIP.review={path,scope,data,state,action,command,quote:null,quoteState:null,quoteKey:null,seq:0,loading:false,error:""};equipmentRememberView();EQUIP.focus={key:"review"};const scroller=typeof equipmentScroller==="function"?equipmentScroller():document.querySelector("#equipmentRoot")?.parentElement;if(scroller)scroller.scrollTop=0;EQUIP.scroll=0;equipmentRender();document.querySelector("#equipmentActionReviewTitle")?.focus({preventScroll:true});if(action.requires_preview)equipmentFetchOrderPreview();return true;
}
async function equipmentFetchOrderPreview(){
  if(!equipmentReviewCurrent())return false;const review=EQUIP.review,state=S,seq=++review.seq,key=JSON.stringify(review.command);review.loading=true;review.error="";review.quoteKey=null;equipmentRender();
  try{const quote=await api("/api/equipment-preview",{session_id:state.session_id,command:equipmentCopy(review.command)});
    if(review!==EQUIP.review||seq!==review.seq||state!==S||!equipmentActive()||key!==JSON.stringify(review.command))return false;
    if(!quote||quote.session_id!==state.session_id||quote.nation!==state.player||typeof quote.valid!=="boolean"||!Array.isArray(quote.actions))throw new Error("The order review did not match this campaign. Review the current settings again.");
    review.quote=quote;review.quoteState=state;review.quoteKey=key;return true;
  }catch(error){if(review===EQUIP.review&&seq===review.seq&&state===S&&equipmentActive()){review.error=error.message||"The order could not be reviewed.";review.quoteKey=null;}return false;}
  finally{if(review===EQUIP.review&&seq===review.seq){review.loading=false;equipmentRender();}}
}
async function equipmentConfirm(index=null){
  if(!equipmentReviewCurrent())return false;const r=EQUIP.review;
  const action=r.action.requires_preview?r.quote?.actions?.[index]:r.action;
  if(r.action.requires_preview&&!equipmentReviewQuoteCurrent())return false;
  if(!action||action.enabled===false||action.available===false)return false;
  if(action.navigate){equipmentNavigate(equipmentCopy(action.navigate));return true;}
  if(!action.command||(r.action.requires_preview&&!r.quote.valid))return false;
  const session=S.session_id;EQUIP.busy=true;EQUIP.message="";equipmentRender();
  try{const result=await equipmentCommand(equipmentCopy(action.command));if(session!==S?.session_id||!equipmentActive())return false;
    if(result?.errors?.length)throw new Error(result.errors.join(" "));if(result===false)throw new Error("The action was not accepted. Refresh and review its requirements.");
    EQUIP.message=result?.message||"Order received. The updated programme will show its progress and spending.";EQUIP.review=null;return true;
  }catch(error){if(session===S?.session_id&&equipmentActive())EQUIP.message=error.message||"The order could not be confirmed.";return false;}
  finally{if(session===S?.session_id){EQUIP.busy=false;EQUIP.stale=true;EQUIP.previewKey=null;equipmentRender();if(equipmentActive())equipmentFetch();}}
}
function equipmentSelectTab(tab,focus=true){if(!EQUIPMENT_TABS.some(row=>row[0]===tab))return false;equipmentRememberView();EQUIP.tab=tab;EQUIP.query="";EQUIP.review=null;EQUIP.scroll=0;if(focus)EQUIP.focus={key:`tab:${tab}`};const root=document.querySelector("#equipmentRoot"),scroller=typeof equipmentScroller==="function"?equipmentScroller():root?.parentElement;if(scroller)scroller.scrollTop=0;equipmentRender();return true;}
function equipmentRevealRecord(id){if(id==null||!equipmentActive())return;if(EQUIP.tab==="research"){const row=equipmentRows("research").find(item=>String(item.id)===String(id));if(row&&(EQUIP.researchBranch!==equipmentResearchBranch(row)||EQUIP.query||EQUIP.researchState!=="all")){EQUIP.researchBranch=equipmentResearchBranch(row);EQUIP.query="";EQUIP.researchState="all";equipmentRender();}}const root=document.querySelector("#equipmentRoot"),card=[...(root?.querySelectorAll("[data-equipment-record]")||[])].find(row=>row.dataset.equipmentRecord===String(id));if(card){card.focus({preventScroll:true});card.scrollIntoView?.({block:"center"});equipmentRememberView();}}
function equipmentRevealSpecification(slot){
  if(!equipmentActive()||EQUIP.tab!=="designer"||!(equipmentPlatform()?.slots||[]).some(row=>row.id===slot))return false;
  const root=document.querySelector("#equipmentRoot"),select=[...(root?.querySelectorAll("[data-equipment-slot]")||[])].find(node=>node.dataset.equipmentSlot===slot);if(!select||select.disabled)return false;
  const group=select.closest?.("details[data-equipment-detail]");if(group){group.open=true;EQUIP.details.add(group.dataset.equipmentDetail);}
  EQUIP.selectedSlot=slot;root.querySelectorAll("[data-equipment-specification]").forEach(node=>node.classList?.toggle("eq-slot-selected",node.dataset.equipmentSpecification===slot));
  EQUIPMENT_VIEWER.controller?.selectPart?.(slot);select.focus({preventScroll:true});select.scrollIntoView?.({block:"center",behavior:"auto"});equipmentRememberView();return true;
}
function equipmentFollowPrerequisite(path,data,state){
  if(!equipmentCurrent(data,state))return false;const [rowIndex,prerequisiteIndex]=String(path).split(":").map(Number),prerequisite=equipmentRows("research")[rowIndex]?.prerequisites?.[prerequisiteIndex];if(!prerequisite)return false;
  const row=equipmentRows("research").find(candidate=>String(candidate.id)===String(prerequisite.id));
  if(row){EQUIP.researchBranch=equipmentResearchBranch(row);EQUIP.researchState="all";EQUIP.query="";equipmentRender();equipmentRevealRecord(row.id);return true;}
  if(prerequisite.id&&prerequisite.domain){equipmentNavigate({action:"research",id:prerequisite.id,domain:prerequisite.domain,name:prerequisite.name});return true;}return false;
}
function equipmentBind(fetchIfNeeded=true){
  if(!equipmentActive())return;const root=document.querySelector("#equipmentRoot");if(!root)return;const data=EQUIP.data,state=EQUIP.state,preview=EQUIP.preview,previewState=EQUIP.previewState;
  equipmentSyncModel(root);
  equipmentSyncResearchTree(root);
  if(!root.equipmentPartListener&&root.addEventListener){root.equipmentPartListener=event=>equipmentRevealSpecification(event.detail?.slot);root.addEventListener("equipment-part-select",root.equipmentPartListener);}
  root.querySelectorAll("[data-equipment-inspect]").forEach(button=>button.onclick=()=>equipmentRevealSpecification(button.dataset.equipmentInspect));
  root.querySelectorAll("[data-equipment-action]").forEach(button=>{const scope=button.dataset.equipmentScope;button.onclick=()=>equipmentInvoke(button.dataset.equipmentAction,scope,scope==="preview"?preview:data,scope==="preview"?previewState:state);});
  root.querySelectorAll("[data-equipment-tab]").forEach(button=>{button.onclick=()=>equipmentSelectTab(button.dataset.equipmentTab);button.onkeydown=event=>{if(!["ArrowLeft","ArrowRight","Home","End"].includes(event.key))return;event.preventDefault();let index=EQUIPMENT_TABS.findIndex(row=>row[0]===button.dataset.equipmentTab);index=event.key==="Home"?0:event.key==="End"?EQUIPMENT_TABS.length-1:(index+(event.key==="ArrowRight"?1:-1)+EQUIPMENT_TABS.length)%EQUIPMENT_TABS.length;equipmentSelectTab(EQUIPMENT_TABS[index][0]);};});
  root.querySelectorAll("[data-equipment-refresh]").forEach(button=>button.onclick=()=>{if(!equipmentPending()){EQUIP.focus={key:button.dataset.equipmentFocus};equipmentFetch();}});
  const close=root.querySelector("[data-equipment-close]");if(close)close.onclick=()=>{if(typeof closeEquipmentDrawer==="function")closeEquipmentDrawer();else equipmentClose();};
  root.querySelectorAll("[data-equipment-preset]").forEach(button=>button.onclick=()=>{if(equipmentPending())return;const row=equipmentRows("presets").find(item=>item.id===button.dataset.equipmentPreset);if(row)equipmentSetDraft(row,{preset:true});});
  root.querySelectorAll("[data-equipment-edit]").forEach(button=>button.onclick=()=>{if(equipmentPending())return;const row=equipmentRows("designs").find(item=>String(item.id)===button.dataset.equipmentEdit);if(row)equipmentSetDraft(row);});
  root.querySelectorAll("[data-equipment-family]").forEach(button=>button.onclick=()=>{const platform=equipmentRows("platforms").find(row=>equipmentFamily(row)===button.dataset.equipmentFamily);if(platform)equipmentChangePlatform(platform.id);});
  root.querySelectorAll("[data-equipment-modernization]").forEach(button=>button.onclick=()=>{if(!equipmentCurrent(data,state))return false;const row=equipmentRows("modernization")[Number(button.dataset.equipmentModernization)];if(!row?.spec)return false;const draft={name:row.name,spec:row.spec};if(row.source_revision!=null)draft.source_revision=row.source_revision;equipmentSetDraft(draft);return true;});
  const name=root.querySelector("#equipmentName");if(name)name.oninput=()=>{if(equipmentPending())return;EQUIP.draft.name=name.value;EQUIP.automaticName=null;EQUIP.preset=null;equipmentDraftChanged();};
  const platform=root.querySelector("#equipmentPlatform");if(platform)platform.onchange=()=>equipmentChangePlatform(platform.value);
  const comparison=root.querySelector("#equipmentComparison");if(comparison)comparison.onchange=()=>{if(equipmentPending())return;const option=equipmentRows("comparison_options").find(row=>String(row.id)===comparison.value);if(comparison.value&&!option)return;EQUIP.comparisonId=option?.id??null;equipmentDraftChanged();};
  root.querySelectorAll("[data-equipment-slot]").forEach(select=>select.onchange=()=>{if(equipmentPending())return;const id=select.dataset.equipmentSlot;if(select.value)EQUIP.draft.components[id]=select.value;else delete EQUIP.draft.components[id];EQUIP.preset=null;EQUIP.selectedSlot=id;equipmentDraftChanged();EQUIPMENT_VIEWER.controller?.selectPart?.(id);});
  root.querySelectorAll("[data-equipment-research]").forEach(button=>button.onclick=()=>{if(!equipmentCurrent(data,state))return;const component=equipmentComponent(button.dataset.equipmentResearch);if(component?.tech)equipmentNavigate({action:"research",...equipmentCopy(component.tech)});});
  const search=root.querySelector("#equipmentSearch");if(search)search.oninput=()=>{EQUIP.query=search.value;equipmentRender();};
  const clear=root.querySelector("[data-equipment-clear]");if(clear)clear.onclick=()=>{EQUIP.query="";EQUIP.focus={key:"search"};equipmentRender();};
  root.querySelectorAll("[data-equipment-branch]").forEach(button=>button.onclick=()=>{if(!EQUIPMENT_BRANCHES.some(([id])=>id===button.dataset.equipmentBranch))return;EQUIP.researchBranch=button.dataset.equipmentBranch;EQUIP.review=null;equipmentRender();});
  root.querySelectorAll("[data-equipment-research-filter]").forEach(button=>button.onclick=()=>{if(!EQUIPMENT_RESEARCH_STATES.some(([id])=>id===button.dataset.equipmentResearchFilter))return;EQUIP.researchState=button.dataset.equipmentResearchFilter;equipmentRender();});
  root.querySelectorAll("[data-equipment-prerequisite]").forEach(button=>button.onclick=()=>equipmentFollowPrerequisite(button.dataset.equipmentPrerequisite,data,state));
  const resetResearch=root.querySelector("[data-equipment-research-reset]");if(resetResearch)resetResearch.onclick=()=>{EQUIP.query="";EQUIP.researchState="all";equipmentRender();};
  const retry=root.querySelector("[data-equipment-preview-retry]");if(retry)retry.onclick=()=>equipmentFetchPreview();
  const confirm=root.querySelector("[data-equipment-confirm]");if(confirm)confirm.onclick=()=>equipmentConfirm();
  root.querySelectorAll("[data-equipment-intent]").forEach(button=>button.onclick=()=>equipmentConfirm(Number(button.dataset.equipmentIntent)));
  root.querySelectorAll("[data-equipment-order-input]").forEach(input=>{const change=()=>{if(!equipmentReviewCurrent())return;const field=(EQUIP.review.action.inputs||[]).find(row=>row.key===input.dataset.equipmentOrderInput);if(!field||["__proto__","prototype","constructor"].includes(field.key))return;
    if(field.type==="select"){const option=(field.options||[]).find(row=>String(row.value)===input.value);if(!option||option.enabled===false)return;EQUIP.review.command[field.key]=option.value;}
    else EQUIP.review.command[field.key]=input.value.trim()===""?null:Number(input.value);
    EQUIP.review.quoteKey=null;equipmentFetchOrderPreview();};if(input.tagName==="SELECT")input.onchange=change;else input.oninput=change;});
  const orderRetry=root.querySelector("[data-equipment-order-retry]");if(orderRetry)orderRetry.onclick=()=>equipmentFetchOrderPreview();
  const dismiss=root.querySelector("[data-equipment-dismiss]");if(dismiss)dismiss.onclick=()=>{EQUIP.review=null;equipmentRender();};
  root.querySelectorAll("details[data-equipment-detail]").forEach(detail=>detail.ontoggle=()=>{const key=detail.dataset.equipmentDetail;if(key==="components")EQUIP.advanced=detail.open;else if(detail.open)EQUIP.details.add(key);else EQUIP.details.delete(key);});
  const scroller=typeof equipmentScroller==="function"?equipmentScroller():root.parentElement;if(scroller)scroller.scrollTop=EQUIP.scroll;
  if(EQUIP.focus){const saved=EQUIP.focus,control=[...root.querySelectorAll("[data-equipment-focus]")].find(item=>item.dataset.equipmentFocus===saved.key)||(saved.key==="retry"?root.querySelector("[data-equipment-refresh]"):null);if(control&&!control.disabled){control.focus({preventScroll:true});if(typeof control.setSelectionRange==="function"&&Number.isFinite(saved.start))control.setSelectionRange(saved.start,saved.end??saved.start);EQUIP.focus=null;}}
  if(fetchIfNeeded&&EQUIP.stale&&!EQUIP.loading&&!EQUIP.error)equipmentFetch();
}
function equipmentOnStateChanged(){equipmentResetCampaign();EQUIP.state=null;EQUIP.stale=true;EQUIP.error="";EQUIP.loading=false;EQUIP.previewLoading=false;EQUIP.previewKey=null;EQUIP.review=null;++EQUIP.seq;++EQUIP.previewSeq;if(equipmentActive()){equipmentRender();equipmentFetch();}}
function equipmentClose(){equipmentRememberView();equipmentDisposeModel();EQUIPMENT_TREE.observer?.disconnect();EQUIPMENT_TREE.observer=null;EQUIP.open=false;EQUIP.loading=false;EQUIP.previewLoading=false;EQUIP.stale=true;EQUIP.previewKey=null;EQUIP.review=null;EQUIP.focus=null;++EQUIP.seq;++EQUIP.previewSeq;}
async function openEquipment(options={}){
  if(typeof S==="undefined"||!S?.player)return false;equipmentResetCampaign();EQUIP.open=true;
  if(EQUIPMENT_TABS.some(row=>row[0]===options.tab))EQUIP.tab=options.tab;
  if(options.design!=null||options.revision!=null||options.preset!=null){EQUIP.request=options;EQUIP.tab="designer";}
  if(typeof openEquipmentDrawer==="function")openEquipmentDrawer();EQUIP.focus={key:"title"};equipmentRender();
  if(!EQUIP.loading)await equipmentFetch();return true;
}
window.openEquipment=openEquipment;
