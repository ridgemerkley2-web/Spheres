/* Optional local observations; never simulation state or remote telemetry. */
function performanceContext(){const s=typeof S==="undefined"?null:S;return {session_id:s?.session_id??null,player:s?.player??null,date:s?.date??null};}
const PerformanceSample={
  enabled:false,started:null,stopped:null,stopReason:null,generation:0,visibilityEpoch:0,run:0,frame:null,lastFrame:null,build:null,buildError:null,
  context:null,render:[],input:[],frames:[],dropped:{render:0,input:0,frames:0},
  record(kind,ms){if(!["render","input","frames"].includes(kind)||!Number.isFinite(ms)||ms<0)return;
    this[kind].push(ms);if(this[kind].length>256){this[kind].shift();this.dropped[kind]++;}},
  current(generation){const c=performanceContext();return this.enabled&&this.generation===generation&&c.session_id===this.context?.session_id&&c.player===this.context?.player;},
  start(){
    this.stop("restarted");this.render=[];this.input=[];this.frames=[];this.dropped={render:0,input:0,frames:0};
    this.context=performanceContext();this.started=new Date().toISOString();this.stopped=null;this.stopReason=null;this.enabled=true;this.lastFrame=null;
    this.build=null;this.buildError=null;const run=++this.run,generation=this.generation;
    if(typeof api==="function")Promise.resolve().then(()=>api("/api/build")).then(b=>{if(this.run===run)this.build={version:b.version,revision:b.revision,branch:b.branch,built_at_unix_seconds:b.built_at_unix_seconds};}).catch(()=>{if(this.run===run)this.buildError="Build identity unavailable; retry with a new sample.";});
    else this.buildError="Build identity unavailable in this context.";
    const sample=now=>{
      if(!this.enabled||this.generation!==generation)return;
      if(!this.current(generation)){this.stop("campaign changed");return;}
      if(document.hidden)this.lastFrame=null;
      else {if(this.lastFrame!==null)this.record("frames",now-this.lastFrame);this.lastFrame=now;}
      this.frame=requestAnimationFrame(sample);
    };
    this.frame=requestAnimationFrame(sample);
  },
  stop(reason="stopped by player"){this.enabled=false;++this.generation;if(this.frame!==null)cancelAnimationFrame(this.frame);this.frame=null;this.lastFrame=null;if(this.started){this.stopped=new Date().toISOString();this.stopReason=reason;}},
  measureRender(draw){if(!this.enabled)return draw();const start=performance.now(),generation=this.generation;try{return draw();}finally{if(this.current(generation))this.record("render",performance.now()-start);}},
  summary(values){const a=values.slice().sort((a,b)=>a-b),n=a.length;return {samples:n,median_ms:n?a[Math.floor(n/2)]:null,p95_ms:n?a[Math.ceil(n*.95)-1]:null,max_ms:n?a[n-1]:null};},
  report(){return {build:this.build,build_error:this.buildError,recording:this.enabled,started:this.started,stopped:this.stopped,stop_reason:this.stopReason,start_context:this.context,current_context:performanceContext(),viewport:{width:innerWidth,height:innerHeight},
    synchronous_state_render:this.summary(this.render),input_to_following_frame:this.summary(this.input),visible_frame_intervals:this.summary(this.frames),discarded_older_observations:{...this.dropped},
    observations_ms:{synchronous_state_render:this.render.slice(),input_to_following_frame:this.input.slice(),visible_frame_intervals:this.frames.slice()},
    method:"Local opt-in sample; latest 256 observations per metric. Frame intervals are visible-page requestAnimationFrame cadence, not a GPU or map FPS measurement. Hidden-tab intervals are excluded. State render measures synchronous redraw only; trusted click/key input ends after two animation frames (a paint opportunity, not exact display latency). Dialog input is excluded. Server/network and asynchronous room loads are not measured. Sampling stops on campaign identity change. Reports remain local unless you choose to share the downloaded file."};}
};
document.addEventListener("click",observePerformanceInput,true);
document.addEventListener("keydown",observePerformanceInput,true);
document.addEventListener("visibilitychange",()=>{PerformanceSample.lastFrame=null;++PerformanceSample.visibilityEpoch;});
function observePerformanceInput(event){
  if(!PerformanceSample.enabled||document.hidden||!event.isTrusted||event.target?.closest("dialog"))return;
  const start=event.timeStamp,generation=PerformanceSample.generation,visibilityEpoch=PerformanceSample.visibilityEpoch;
  requestAnimationFrame(()=>requestAnimationFrame(()=>{if(PerformanceSample.current(generation)&&PerformanceSample.visibilityEpoch===visibilityEpoch&&!document.hidden)PerformanceSample.record("input",performance.now()-start);}));
}
function openPerformanceSample(){
  clockPause();document.querySelectorAll("dialog[open]").forEach(d=>d.close());
  let panel=document.getElementById("performancePanel");
  if(!panel){panel=document.createElement("dialog");panel.id="performancePanel";panel.className="decision-dialog";panel.setAttribute("aria-label","Local performance sample");panel.addEventListener("keydown",e=>e.stopPropagation());document.body.append(panel);}
  panel.innerHTML='<header><h2>Local performance sample</h2><button id="performanceClose">Close</button></header><div class="decision-body"><p>Start recording, advance several days and open or close game screens. Return here to inspect timing. Closing this panel keeps recording; Stop ends the sample. These observations stay on your device.</p><div class="decision-actions"><button id="performanceStart">Start fresh sample</button><button id="performanceStop">Stop and show results</button><button id="performanceRefresh">Refresh results</button><button id="performanceDownload">Download timing report</button></div><pre id="performanceResults" style="white-space:pre-wrap;overflow-wrap:anywhere"></pre></div>';
  const draw=()=>panel.querySelector("#performanceResults").textContent=JSON.stringify(PerformanceSample.report(),null,2);
  panel.querySelector("#performanceClose").onclick=()=>panel.close();
  panel.querySelector("#performanceStart").onclick=()=>{PerformanceSample.start();panel.close();};
  panel.querySelector("#performanceStop").onclick=()=>{PerformanceSample.stop();draw();};
  panel.querySelector("#performanceRefresh").onclick=draw;
  panel.querySelector("#performanceDownload").onclick=()=>{const url=URL.createObjectURL(new Blob([JSON.stringify(PerformanceSample.report(),null,2)+"\n"],{type:"application/json"}));const a=document.createElement("a");a.href=url;a.download="spheres-performance-sample.json";a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);};
  panel.showModal();draw();panel.querySelector("#performanceClose").focus();
}
