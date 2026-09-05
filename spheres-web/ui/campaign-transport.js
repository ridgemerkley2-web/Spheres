/* Importable transport and history state. No world rules or DOM assumptions. */
(function(root,factory){const api=factory();if(typeof module==="object"&&module.exports)module.exports=api;else root.CampaignTransport=api;})(globalThis,function(){
  "use strict";
  const clone=value=>JSON.parse(JSON.stringify(value));
  function create(options) {
    let pending=null,busy=false;
    const store=options.storage;
    try {const old=JSON.parse(store?.getItem("spheres.pending-command")||"null");if(old?.session_id&&Array.isArray(old.commands))pending=old;} catch(_){}
    function changed(){try{if(pending)store?.setItem("spheres.pending-command",JSON.stringify(pending));else store?.removeItem("spheres.pending-command");}catch(_){}options.changed?.();}
    function conflict(message){const e=new Error(message);e.requiresReview=true;return e;}
    async function execute(){
      if(busy)throw new Error("An order is already in flight. Wait for its result.");
      if(!pending)throw new Error("There is no pending order to check.");
      if(pending.session_id!==options.session())throw conflict("This order belongs to another campaign/server session. Review current state; it will not be replayed here.");
      busy=true;changed();
      try {
        const result=await options.request(clone(pending));
        if(result?.session_id!==pending.session_id)throw conflict("The campaign changed while this order was processed. Review current state.");
        pending=null;changed();return result;
      }catch(error){
        // Only an explicit pre-mutation rejection establishes no effect.
        // Lost responses keep the exact receipt, including across reload.
        if(error.notApplied===true)pending=null;
        changed();throw error;
      }finally{busy=false;changed();}
    }
    return {
      get pending(){return pending;},get busy(){return busy;},
      async send(body){
        if(pending){
          const same=body.client_id===pending.client_id&&body.request_seq===pending.request_seq&&body.session_id===pending.session_id
            &&JSON.stringify(body.commands)===JSON.stringify(pending.commands);
          if(!same)throw new Error("Check or review the pending order before issuing another command.");
          return execute();
        }
        if(!options.session())throw new Error("Continue a campaign before issuing orders.");
        pending=clone({...body,session_id:body.session_id||options.session(),...(body.client_id?{}:options.identity())});
        changed();return execute();
      },
      retry:execute,
      clear(){if(busy)throw new Error("Wait for the in-flight order before reviewing it.");pending=null;changed();},
    };
  }
  const metrics=["gdp","growth","inflation","debt","stability","mil"];
  function mergeHistory(previous,delta){
    if(!previous||delta.reset||previous.epoch!==delta.epoch)return clone(delta);
    const out=clone(previous),offset=out.t.length;
    out.t.push(...delta.t);out.labels.push(...delta.labels);out.oil.push(...delta.oil);
    for(const [id,series] of Object.entries(delta.nations||{})) {
      const old=out.nations[id];
      if(!old){out.nations[id]={...clone(series),t0:offset+series.t0};continue;}
      for(const metric of metrics){
        const list=old[metric],gap=offset+series.t0-old.t0-list.length;
        // Match the full archive's legacy bridge for a returning nation;
        // trailing dead nations still receive no fabricated observations.
        if(gap>0)list.push(...Array(gap).fill(list.at(-1)));
        list.push(...series[metric]);
      }
    }
    out.order=[...new Set([...out.order,...delta.order])];
    out.cursor=delta.cursor;out.available=delta.available;out.retention=delta.retention;
    return out;
  }
  function historyClient(request){
    let cache=null,key="",session=null,sequence=0;
    return {
      reset(){cache=null;key="";session=null;++sequence;},
      async read(ids,state){
        const nextKey=[...new Set(ids.filter(Boolean))].sort().join(",");
        const full=key!==nextKey||session!==state.session_id||!cache;
        const query=`nations=${nextKey}`+(full?"":`&epoch=${cache.epoch}&after=${cache.cursor}`);
        const seq=++sequence;
        const delta=await request("/api/history?"+query);
        if(seq!==sequence)return null;
        if(delta.session_id!==state.session_id)throw new Error("The campaign changed while its history was loading.");
        cache=mergeHistory(full?null:cache,delta);key=nextKey;session=state.session_id;return cache;
      },
    };
  }
  function slotName(value){const s=String(value||"").trim();return /^[A-Za-z0-9_-]{1,64}$/.test(s)?s:null;}
  return {create,mergeHistory,historyClient,slotName};
});
