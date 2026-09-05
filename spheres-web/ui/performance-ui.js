/* Optional local observations; never simulation state or remote telemetry. */
const PerformanceSample={
  enabled:false,started:null,render:[],input:[],
  record(kind,ms){if(Number.isFinite(ms)&&ms>=0&&ms<60000){this[kind].push(ms);if(this[kind].length>256)this[kind].shift();}},
  measureRender(draw){if(!this.enabled)return draw();const start=performance.now();try{return draw();}finally{this.record("render",performance.now()-start);}},
  summary(values){const a=values.slice().sort((a,b)=>a-b),n=a.length;return {samples:n,median_ms:n?a[Math.floor(n/2)]:null,p95_ms:n?a[Math.ceil(n*.95)-1]:null,max_ms:n?a[n-1]:null};},
  report(){return {build:S?.build,date:S?.date,started:this.started,viewport:{width:innerWidth,height:innerHeight},synchronous_state_render:this.summary(this.render),input_to_following_frame:this.summary(this.input),method:"Local opt-in sample, capped at 256 observations. Render times only the synchronous state redraw. Trusted click/key input is observed through two animation frames, an upper-bound paint opportunity rather than exact display latency. Dialog controls excluded; server/network and asynchronous room fetches are not included. No external transmission."};}
};
document.addEventListener("click",observePerformanceInput,true);
document.addEventListener("keydown",observePerformanceInput,true);
function observePerformanceInput(event){
  if(!PerformanceSample.enabled||!event.isTrusted||event.target?.closest("dialog"))return;
  const start=event.timeStamp;
  requestAnimationFrame(()=>requestAnimationFrame(()=>{if(PerformanceSample.enabled)PerformanceSample.record("input",performance.now()-start);}));
}
function openPerformanceSample(){
  clockPause();document.querySelectorAll("dialog[open]").forEach(d=>d.close());
  let panel=document.getElementById("performancePanel");
  if(!panel){panel=document.createElement("dialog");panel.id="performancePanel";panel.className="decision-dialog";panel.setAttribute("aria-label","Local performance sample");panel.addEventListener("keydown",e=>e.stopPropagation());document.body.append(panel);}
  panel.innerHTML='<header><h2>Local performance sample</h2><button id="performanceClose">Close</button></header><div class="decision-body"><p>Start recording, advance several days and open or close game screens. Return here to inspect timing. These observations stay on your device.</p><div class="decision-actions"><button id="performanceStart">Start fresh sample</button><button id="performanceStop">Stop and show results</button><button id="performanceRefresh">Refresh results</button></div><pre id="performanceResults" style="white-space:pre-wrap;overflow-wrap:anywhere"></pre></div>';
  const draw=()=>panel.querySelector("#performanceResults").textContent=JSON.stringify(PerformanceSample.report(),null,2);
  panel.querySelector("#performanceClose").onclick=()=>panel.close();
  panel.querySelector("#performanceStart").onclick=()=>{PerformanceSample.render=[];PerformanceSample.input=[];PerformanceSample.started=new Date().toISOString();PerformanceSample.enabled=true;panel.close();};
  panel.querySelector("#performanceStop").onclick=()=>{PerformanceSample.enabled=false;draw();};
  panel.querySelector("#performanceRefresh").onclick=draw;
  panel.showModal();draw();panel.querySelector("#performanceClose").focus();
}
