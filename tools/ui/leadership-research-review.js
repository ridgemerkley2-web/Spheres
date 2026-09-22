/* Read-only C01 discovery explorer. This module never calls the game API. */
(function(root,factory){
  'use strict';
  const api=factory();
  if(typeof module==='object'&&module.exports)module.exports=api;
  else {root.LeadershipResearchReview=api;api.start(document).catch(error=>{
    const status=document.getElementById('atlas-status');status.textContent='Research unavailable: '+error.message;status.setAttribute('role','alert');
    document.getElementById('atlas-country-content').setAttribute('aria-busy','false');
  });}
})(typeof window!=='undefined'?window:globalThis,function(){
  'use strict';
  const CUTOFF='2026-09-07',PAGE_SIZE=25,BASE='../../';
  const INDEX='docs/campaign-certification/C01/research-index.json';
  const COUNTRIES='docs/campaign-certification/C01/countries.json';
  function require(ok,message){if(!ok)throw Error(message);}
  function publicURL(value){try{const url=new URL(value);return url.protocol==='https:'&&!url.username&&!url.password?url.href:null;}catch{return null;}}
  function sourcePath(value){return typeof value==='string'&&/^docs\/campaign-certification\/C01\/(?:countries\.json|research\/[a-z0-9-]+\.json)$/.test(value)?BASE+value:null;}
  function validateIndex(index,countries){
    require(index.format==='spheres-c01-research-intake/v1'&&index.research_cutoff===CUTOFF,'Unsupported research register.');
    require(index.c01_complete===false&&index.g2_prerequisite_satisfied===false&&index.runtime_roster_modified===false,'This view requires the partial discovery register.');
    require(Array.isArray(countries)&&Array.isArray(index.countries)&&Array.isArray(index.source_files)&&Array.isArray(index.work_orders),'Incomplete research register.');
    const ids=new Set(countries.map(c=>c.id));require(ids.size===countries.length,'Duplicate country identity.');
    const packets=new Set();
    for(const row of index.countries){
      require(ids.has(row.nation)&&!packets.has(row.nation)&&row.country_census_complete===false,'Mismatched country coverage.');packets.add(row.nation);
      require(sourcePath(row.packet)&&index.source_files.filter(s=>s.path===row.packet).length===1,'Missing packet provenance.');
    }
    require(index.counts.country_packets===packets.size&&index.counts.countries_without_new_discovery_packet===countries.length-packets.size,'Register country counts differ.');
    return countries.map(c=>({...c,discovery:index.countries.find(p=>p.nation===c.id)||null}));
  }
  function preparePacket(packet,row){
    require(packet.version===1&&packet.nation===row.nation&&packet.research_cutoff===CUTOFF,'Country packet does not match the selected reference.');
    require(packet.coverage?.status==='partial_primary_source_inventory'&&packet.coverage.unresolved?.length,'Missing partial-coverage disclosure.');
    require(Array.isArray(packet.organizations)&&Array.isArray(packet.institutions)&&Array.isArray(packet.sources),'Incomplete country packet.');
    require(packet.organizations.length===row.organization_observations&&packet.institutions.length===row.institution_observations,'Packet counts differ from the register.');
    const sources=new Map(),claims=new Map(),ids=new Set();
    for(const source of packet.sources){
      require(source.id&&!sources.has(source.id)&&publicURL(source.url)&&Array.isArray(source.claims),'Invalid source reference.');sources.set(source.id,source);
      for(const claim of source.claims){require(claim.id&&!claims.has(claim.id)&&typeof claim.text==='string','Invalid claim reference.');claims.set(claim.id,{...claim,source_id:source.id});}
    }
    function references(value){
      require(Array.isArray(value.sources)&&value.sources.length&&value.sources.every(id=>sources.has(id)),'Unknown source reference.');
      require(Array.isArray(value.claim_ids)&&value.claim_ids.length&&value.claim_ids.every(id=>claims.has(id)&&value.sources.includes(claims.get(id).source_id)),'Claim belongs to another source.');
    }
    const entries=[];
    for(const category of ['organizations','institutions'])for(const entry of packet[category]){
      require(entry.id&&!ids.has(entry.id)&&typeof entry.name==='string'&&Array.isArray(entry.roles)&&Array.isArray(entry.represented_party_ids),'Invalid organization observation.');ids.add(entry.id);references(entry);
      for(const role of entry.roles){references(role);for(const holder of role.holder_claims||[]){
        if(typeof holder==='string')require(claims.has(holder)&&role.sources.includes(claims.get(holder).source_id),'Unknown holder claim.');else references(holder);
      }}
      const strings=[entry.name,entry.id,entry.kind,...entry.roles.flatMap(r=>[r.title,...(r.holder_claims||[]).map(h=>typeof h==='string'?claims.get(h).text:h.name)]),...entry.claim_ids.map(id=>claims.get(id).text)];
      entries.push({...entry,category,searchText:strings.join(' ').toLocaleLowerCase()});
    }
    return {packet,sources,claims,entries};
  }
  function selectEntries(board,query='',kind='all'){
    require(['all','organizations','institutions','roles'].includes(kind),'Unknown observation filter.');
    const text=query.trim().toLocaleLowerCase();return board.entries.filter(e=>(!text||e.searchText.includes(text))&&(kind==='all'||kind==='roles'&&e.roles.length>0||e.category===kind));
  }
  function dateText(value){
    if(value==null)return 'Not established';if(typeof value==='string')return value;
    if(typeof value==='object'&&value.kind)return value.value?`${value.value} (${value.kind} precision)`:'Not established';
    return 'Not established';
  }
  function observationDate(holder){
    if(holder.attested_on)return 'Observed on '+dateText(holder.attested_on);
    if(holder.attested_period){
      const period=holder.attested_period,observed='Observed between '+dateText(period.from)+' and '+dateText(period.through);
      const term=holder.from||holder.until?'reported office interval: '+dateText(holder.from)+' → '+dateText(holder.until):'office term not established';
      return observed+'; '+term;
    }
    if(holder.from||holder.until||holder.through)return 'Reported interval: '+dateText(holder.from)+' → '+dateText(holder.until||holder.through);
    return 'Exact office interval not established';
  }
  async function readJSON(path,descriptor,signal,fetcher=globalThis.fetch,crypto=globalThis.crypto){
    const href=path===INDEX?BASE+path:sourcePath(path);require(href,'Unsupported research file path.');
    if(path!==INDEX)require(descriptor?.path===path&&Number.isInteger(descriptor.bytes)&&/^[a-f0-9]{64}$/.test(descriptor.sha256),'Missing file identity.');
    const response=await fetcher(href,{cache:'no-store',signal});require(response.ok,'Cannot load the research file ('+response.status+').');
    const bytes=await response.arrayBuffer();
    if(descriptor){
      require(crypto?.subtle,'Open this review on localhost or HTTPS to verify its source files.');
      const hash=Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',bytes)),v=>v.toString(16).padStart(2,'0')).join('');
      require(bytes.byteLength===descriptor.bytes&&hash===descriptor.sha256,'Source files changed. Rebuild the research register before reviewing this country.');
    }
    return JSON.parse(new TextDecoder('utf-8',{fatal:true}).decode(bytes));
  }
  // A superseded request may finish even after abort. Only the latest request
  // can publish content or errors; every selection clears the previous result.
  function latestLoader(load,accept,reject){
    let serial=0,controller;
    return async value=>{const request=++serial;controller?.abort();controller=new AbortController();
      try{const result=await load(value,controller.signal);if(request===serial)accept(value,result);}
      catch(error){if(request===serial&&error.name!=='AbortError')reject(value,error);}
    };
  }
  async function start(doc){
    const el=id=>doc.getElementById(id),make=(tag,text,cls)=>{const n=doc.createElement(tag);if(text!==undefined)n.textContent=text;if(cls)n.className=cls;return n;};
    const para=text=>make('p',text),list=items=>{const ul=make('ul');items.forEach(item=>ul.append(make('li',item)));return ul;};
    const link=(title,url)=>{const a=make('a',title);a.href=url;a.target='_blank';a.rel='noopener noreferrer';return a;};
    const index=await readJSON(INDEX),countryDescriptor=index.source_files?.find(s=>s.path===COUNTRIES);
    const countries=validateIndex(index,await readJSON(COUNTRIES,countryDescriptor));
    const byId=new Map(countries.map(c=>[c.id,c]));
    let board=null,limit=PAGE_SIZE;
    const cert=make('optgroup');cert.label='Campaign certification cases';const world=make('optgroup');world.label='Rest of the world';
    for(const country of countries){const option=make('option',country.name+(country.discovery?' · Partial research':' · Intake needed'));option.value=country.id;(country.certified_case?cert:world).append(option);}
    el('atlas-country').replaceChildren(cert,world);el('atlas-country').disabled=false;
    el('atlas-status').textContent=`${index.counts.country_packets} country packets · ${index.counts.exhaustive_country_censuses} exhaustive censuses complete · Historical cutoff ${CUTOFF}`;
    function renderSummary(country){
      const row=country.discovery,heading=make('div',undefined,'country-heading'),title=make('div');
      title.append(make('h2',country.name),para(country.start_1990?'1990 starting identity':'Successor identity · separate historical coverage'));
      heading.append(title,make('span',row?'Partial source inventory':'Discovery intake needed','scope-badge'));
      const facts=make('div',undefined,'facts');
      const values=[[country.party_rows,'Existing game party rows'],[row?.organization_observations??'Unknown','Organization observations'],[row?.institution_observations??'Unknown','Institution observations']];
      for(const [value,label] of values){const fact=make('div',undefined,'fact');fact.append(make('strong',String(value)),make('span',label));facts.append(fact);}
      el('atlas-summary').replaceChildren(heading,facts);
      const note=make('div',undefined,'notice');note.append(para('The full organization census remains open. Missing research does not mean a country has no parties, and source observations do not establish complete leadership histories.'));
      el('atlas-gaps').replaceChildren(note);
    }
    function sourceDetails(entry,entryBoard){
      const wrapper=make('div');wrapper.append(make('h3','Evidence and attribution'));
      const selectedIds=new Set(entry.claim_ids);
      for(const role of entry.roles){role.claim_ids.forEach(id=>selectedIds.add(id));for(const holder of role.holder_claims||[])typeof holder==='string'?selectedIds.add(holder):holder.claim_ids.forEach(id=>selectedIds.add(id));}
      const cited=new Set([...selectedIds].map(id=>entryBoard.claims.get(id).source_id));
      for(const id of cited){const source=entryBoard.sources.get(id),details=make('details',undefined,'source');details.append(make('summary',source.title));
        details.append(link('Read source · '+source.publisher,publicURL(source.url)),make('p',`Published: ${source.published_date||'Date not recorded'} · Accessed: ${source.accessed_date}`,'source-meta'));
        if(source.document_date)details.append(para('Document date: '+source.document_date));
        if(source.access_method)details.append(para('How the source was read: '+source.access_method.replaceAll('_',' ')));
        if(source.scope_note)details.append(para('Source scope: '+source.scope_note));
        if(source.rights_note)details.append(para(source.rights_note));
        for(const cid of selectedIds){const claim=entryBoard.claims.get(cid);if(claim.source_id!==id)continue;
          const body=make('div');body.append(make('span',cid,'claim-id'),para(claim.text));
          if(claim.period)body.append(para('Claim period: '+dateText(claim.period.from)+' → '+dateText(claim.period.through||claim.period.until)));
          if(claim.attested_on)body.append(para('Observed on '+dateText(claim.attested_on)));
          if(claim.uncertainty)body.append(para('Qualification: '+claim.uncertainty));
          if(claim.locator)body.append(para('Source location: '+JSON.stringify(claim.locator)));
          details.append(body);
        }wrapper.append(details);
      }return wrapper;
    }
    function entryCard(entry){
      const entryBoard=board;
      const details=make('details',undefined,'entry');details.dataset.researchId=entry.id;
      const summary=make('summary'),title=make('div');title.append(make('strong',entry.name),make('small',`${entry.category==='institutions'?'Institution':'Organization'} · ${entry.kind.replaceAll('_',' ')} · ${entry.roles.length} recorded offices`));summary.append(title);details.append(summary);
      // Build expanded evidence only on demand; large election registers stay usable.
      details.addEventListener('toggle',()=>{if(!details.isConnected||!details.open||details.querySelector('.entry-body'))return;
        const body=make('div',undefined,'entry-body');body.append(make('h3','Identity and coverage'));
        body.append(para(entry.identity_note||entry.note||'Identity observed in the cited source. Broader identity reconciliation remains open.'));
        body.append(para('Lifecycle: '+(entry.lifecycle?.status||'unknown').replaceAll('_',' ')+'. Start: '+dateText(entry.lifecycle?.from)+'. End: '+dateText(entry.lifecycle?.until)+'.'));
        if(entry.lifecycle?.note)body.append(para(entry.lifecycle.note));
        body.append(para(entry.represented_party_ids.length?'Reviewed catalogue references: '+entry.represented_party_ids.join(', ')+'. A reference does not grant an office.':'Game identity mapping: unresolved. This is not a declaration that the organization is absent from the game.'));
        body.append(make('h3','Offices and holder observations'));
        if(!entry.roles.length)body.append(para('No office-holder observation has been recorded in this packet. Party, parliamentary and national offices require separate research.'));
        for(const role of entry.roles){const box=make('div',undefined,'role');box.append(make('h4',role.title),para(role.kind.replaceAll('_',' ')));
          if(!(role.holder_claims||[]).length)box.append(para('Holder history is still needed.'));
          for(const holder of role.holder_claims||[]){if(typeof holder==='string')box.append(para(entryBoard.claims.get(holder).text));else {box.append(para(holder.name+' · '+observationDate(holder)));if(holder.note)box.append(para(holder.note));if(holder.uncertainty&&holder.uncertainty!==holder.note)box.append(para(holder.uncertainty));}}
          box.append(para('This sourced role is separate from any saved campaign appointment.'));body.append(box);
        }
        body.append(make('h3','Research still needed'),list(entry.coverage?.unresolved||['Complete source and date review.']),sourceDetails(entry,entryBoard));
        details.append(body);
      });return details;
    }
    function renderEntries(append=false){
      if(!board)return;const selected=selectEntries(board,el('atlas-search').value,el('atlas-kind').value);
      const shown=selected.slice(0,limit);
      if(append)el('atlas-entries').append(...shown.slice(el('atlas-entries').children.length).map(entryCard));
      else el('atlas-entries').replaceChildren(...shown.map(entryCard));
      if(!selected.length)el('atlas-entries').append(make('p','No observations match these filters.','empty'));
      el('atlas-results').textContent=`${shown.length} of ${selected.length} matching observations · ${board.entries.length} in this packet`;
      el('atlas-more').hidden=shown.length===selected.length;
    }
    const choose=latestLoader(async(country,signal)=>country.discovery?preparePacket(await readJSON(country.discovery.packet,index.source_files.find(s=>s.path===country.discovery.packet),signal),country.discovery):null,(country,result)=>{
      board=result;el('atlas-country-content').setAttribute('aria-busy','false');
      if(!board){el('atlas-results').textContent='No discovery packet yet. Start with the country’s organization and office census; existing game rows are not an exhaustive list.';return;}
      const gaps=make('details',undefined,'notice');gaps.append(make('summary','Open questions for '+country.name),list(board.packet.coverage.unresolved));el('atlas-gaps').append(gaps);
      el('atlas-search').disabled=false;el('atlas-kind').disabled=false;renderEntries();
    },(country,error)=>{board=null;el('atlas-country-content').setAttribute('aria-busy','false');el('atlas-results').textContent='Country research could not be loaded.';const p=make('p','Could not review '+country.name+': '+error.message,'error');p.setAttribute('role','alert');el('atlas-entries').replaceChildren(p);});
    function changeCountry(){
      const country=byId.get(el('atlas-country').value);require(country,'Unknown country selection.');
      board=null;limit=PAGE_SIZE;el('atlas-search').value='';el('atlas-kind').value='all';el('atlas-search').disabled=true;el('atlas-kind').disabled=true;
      el('atlas-entries').replaceChildren();el('atlas-more').hidden=true;el('atlas-results').textContent='Checking this country’s research…';el('atlas-country-content').setAttribute('aria-busy','true');renderSummary(country);
      const url=new URL(doc.defaultView.location.href);url.searchParams.set('country',country.id);doc.defaultView.history.replaceState(null,'',url);return choose(country);
    }
    el('atlas-country').addEventListener('change',changeCountry);
    el('atlas-search').addEventListener('input',()=>{limit=PAGE_SIZE;renderEntries();});el('atlas-kind').addEventListener('change',()=>{limit=PAGE_SIZE;renderEntries();});
    el('atlas-more').addEventListener('click',()=>{const before=el('atlas-entries').children.length;limit+=PAGE_SIZE;renderEntries(true);if(el('atlas-more').hidden)el('atlas-entries').children[before]?.querySelector('summary')?.focus();});
    const wanted=new URL(doc.defaultView.location.href).searchParams.get('country');el('atlas-country').value=byId.has(wanted)?wanted:'France';await changeCountry();
  }
  return {CUTOFF,PAGE_SIZE,publicURL,sourcePath,validateIndex,preparePacket,selectEntries,dateText,observationDate,readJSON,latestLoader,start};
});
