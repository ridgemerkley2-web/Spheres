#!/usr/bin/env node
'use strict';
// Source collector only: never writes nation GDP, physical assets, inventories, or saves.
// No dependencies. Node >=18. Public documented UNIDO API, no browser/bypass.
// Offline rebuild: node tools/industry/collect_industry_1990.cjs --offline
// Fetch: node tools/industry/collect_industry_1990.cjs --fetch --share-only
// Mix follow-up: omit --share-only; all successful raw requests are cached.
// --max-countries N bounds this run; --refresh replaces cached responses explicitly.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const assert = require('node:assert/strict');
const ROOT = path.resolve(__dirname, '../..');
const API = 'https://stat.unido.org/portal/dataset/';
const META_NA = API + 'getDataset/NATIONAL_ACCOUNTS';
const META_IND = API + 'getDataset/INDSTAT/3';
const PARTITION = Object.freeze({
  food_textiles: ['15','16','17','18','19'],
  materials: ['20','21','22','23','25','26','27','28'],
  chemicals: ['24'],
  machinery_electronics: ['29','30','31','32','33','34','35'],
  other: ['36','37'],
});
const FALLBACK_MIX = Object.freeze(Object.fromEntries(Object.keys(PARTITION).map(k => [k, 0.2])));
// Canonical IDs, not display names. Former federations deliberately have no modern proxy.
const ISO = Object.fromEntries(`
USA USA;USSR -;China CHN;Japan JPN;Germany DEU;UK GBR;France FRA;Italy ITA;
India IND;Pakistan PAK;Iraq IRQ;Kuwait KWT;SaudiArabia SAU;Iran IRN;SouthKorea KOR;
Poland POL;Brazil BRA;Indonesia IDN;Egypt EGY;Israel ISR;Turkey TUR;Nigeria NGA;
Vietnam VNM;Yugoslavia -;Spain ESP;Netherlands NLD;Belgium BEL;Sweden SWE;
Switzerland CHE;Austria AUT;Portugal PRT;Greece GRC;Denmark DNK;Norway NOR;
Finland FIN;Ireland IRL;Czechoslovakia -;Hungary HUN;Romania ROU;Bulgaria BGR;
Albania ALB;Argentina ARG;Mexico MEX;Chile CHL;Colombia COL;Venezuela VEN;
Peru PER;Cuba CUB;Bolivia BOL;Ecuador ECU;Uruguay URY;Syria SYR;Jordan JOR;
Lebanon LBN;UAE ARE;Qatar QAT;Oman OMN;Yemen YEM;Bahrain BHR;Algeria DZA;
Morocco MAR;Tunisia TUN;Libya LBY;Sudan SDN;SouthAfrica ZAF;Ethiopia ETH;
Kenya KEN;Ghana GHA;Zaire COD;Angola AGO;Zimbabwe ZWE;Tanzania TZA;Uganda UGA;
Senegal SEN;Cameroon CMR;Bangladesh BGD;SriLanka LKA;Nepal NPL;Afghanistan AFG;
Myanmar MMR;NorthKorea PRK;Taiwan TWN;Mongolia MNG;Thailand THA;Malaysia MYS;
Singapore SGP;Philippines PHL;Cambodia KHM;Laos LAO;Canada CAN;Australia AUS;
NewZealand NZL;DominicanRepublic DOM;Haiti HTI;Jamaica JAM;TrinidadTobago TTO;
Bahamas BHS;Chad TCD;CentralAfricanRepublic CAF;Congo COG;Gabon GAB;
EquatorialGuinea GNQ;SaoTome STP;Guatemala GTM;Honduras HND;ElSalvador SLV;
Nicaragua NIC;CostaRica CRI;Panama PAN;Belize BLZ;Madagascar MDG;Mauritius MUS;
Seychelles SYC;Comoros COM;CapeVerde CPV;Fiji FJI;SolomonIslands SLB;Vanuatu VUT;
Samoa WSM;Tonga TON;Brunei BRN;PapuaNewGuinea PNG;Bhutan BTN;Maldives MDV;
Iceland ISL;Luxembourg LUX;Malta MLT;Cyprus CYP;Mozambique MOZ;Zambia ZMB;
Malawi MWI;Botswana BWA;Lesotho LSO;Swaziland SWZ;Paraguay PRY;Guyana GUY;Suriname SUR
`.trim().split(';').map(s => s.trim().split(/\s+/)));

function decode(value) {
  // Some portal deployments JSON-encode an already-JSON response.
  if (typeof value === 'string' && /^[\[{]/.test(value.trim())) {
    try { return decode(JSON.parse(value)); } catch { return value; }
  }
  return value;
}
function* objects(value) {
  value = decode(value);
  if (!value || typeof value !== 'object') return;
  if (!Array.isArray(value)) yield value;
  for (const child of Object.values(value)) yield* objects(child);
}
function label(row) { return row?.lang?.en ?? row?.name ?? row?.label ?? ''; }
function countryCode(meta, iso3, canonicalId=null) {
  if (!iso3 || iso3 === '-') return null;
  // 1990 Sudan includes the territory of both later states. Resolve the
  // historical reporter by its exact metadata label, never today's SDN proxy.
  const matches = [...objects(meta)].filter(x => x.c != null && (canonicalId==='Sudan'?label(x)==='Former Sudan':x.iso3===iso3));
  const codes = [...new Set(matches.map(x => String(x.c)))];
  return codes.length === 1 ? codes[0] : null;
}
function datasetId(meta) {
  meta = decode(meta);
  const id = meta?.id ?? meta?.datasetId;
  if (!Number.isInteger(Number(id)) || Number(id) <= 0) throw Error('Metadata has no unambiguous runtime dataset ID');
  return Number(id);
}
function assertVariable(meta, code, pattern) {
  const matches = [...objects(meta)].filter(x => String(x.c) === code && pattern.test(label(x)));
  if (!matches.length) throw Error(`Metadata does not confirm variable ${code} with expected units`);
  return code;
}
function finiteObservation(value) {
  // Missing, suppression strings, booleans, and blanks are never zero.
  if (typeof value === 'number') return Number.isFinite(value) ? value : null;
  if (typeof value !== 'string' || !/^[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?$/.test(value.trim())) return null;
  const n = Number(value);
  return Number.isFinite(n) ? n : null;
}
function rows(payload) { const p = decode(payload); return Array.isArray(p?.data) ? p.data : []; }
function nationalAccounts(payload) {
  const observations = {};
  for (const variable of ['GdpCud', 'MvaCud']) {
    const matches = rows(payload).filter(r => String(r.p) === '1990' && r.c === variable);
    observations[variable] = matches.length === 1 ? finiteObservation(matches[0].v) : null;
  }
  const gdp = observations.GdpCud, mva = observations.MvaCud;
  const share = gdp != null && gdp > 0 && mva != null && mva >= 0 && mva < gdp ? mva / gdp : null;
  return { gdp, mva, share };
}
function footnotes(payload) {
  const p = decode(payload) || {};
  const result = [];
  for (const key of ['ym', 'vm', 'am', 'notes', 'footnotes']) {
    if (p[key] != null && (!Array.isArray(p[key]) || p[key].length)) result.push({ scope: key, value: p[key] });
  }
  for (const r of rows(p)) {
    // `u` is an alternate monetary value, not a footnote or overlap flag.
    const extra = Object.fromEntries(Object.entries(r).filter(([k]) => !['p','a','v','u'].includes(k)));
    if (Object.keys(extra).length) result.push({ activity: r.a ?? null, values: extra });
  }
  return result;
}
function composition(payload) {
  const raw = rows(payload).filter(r => String(r.p) === '1990');
  const expected = Object.values(PARTITION).flat();
  const groupOf = Object.fromEntries(Object.entries(PARTITION).flatMap(([g,codes])=>codes.map(c=>[c,g])));
  const duplicateCodes = expected.filter(c=>raw.filter(r=>String(r.a)===c).length>1);
  const observations = Object.fromEntries(expected.map(code => {
    const match = raw.filter(r => String(r.a) === code);
    return [code, match.length === 1 ? finiteObservation(match[0].v) : null];
  }));
  const notes = footnotes(payload);
  const annotations = [...new Set(raw.filter(r=>(r.c!=null&&String(r.c).trim())||(r.cn!=null&&String(r.cn).trim())).map(r=>String(r.cn??'').trim()))];
  const aggregates=[]; const covered=new Set(); let problem=null;
  // Portal repeats the same annotation on parent AND included child rows.
  // Only this explicit two-digit same-game-sector grammar is resolved.
  for(const note of annotations) {
    const match=note.match(/^(\d{2})\s+includes\s+(\d{2}(?:\s*(?:,|and|&)\s*\d{2})*)\.?$/i);
    if(!match){problem='Unrecognized row aggregation annotation';break;}
    const parent=match[1],children=match[2].match(/\d{2}/g);
    if(!groupOf[parent]||!(observations[parent]>0)||children.some(c=>c===parent||groupOf[c]!==groupOf[parent]||covered.has(c))){problem='Cross-sector, conflicting, or unvalued aggregation';break;}
    aggregates.push({parent,includes:children,note});children.forEach(c=>covered.add(c));
  }
  if(aggregates.some(a=>covered.has(a.parent)))problem='Nested aggregation is not resolved';
  if(duplicateCodes.length)problem='Duplicate activity observations';
  if(Object.values(observations).some(v=>v!==null&&v<0))problem='Negative source value added is not imputed';
  const observedCount=expected.filter(c=>observations[c]!==null&&observations[c]>=0).length;
  const missing=expected.filter(c=>observations[c]===null&&!covered.has(c));
  const aggregateParents=new Set(aggregates.map(a=>a.parent));
  const positiveUncombined=expected.filter(c=>!covered.has(c)&&!aggregateParents.has(c)&&observations[c]>0).map(c=>observations[c]).sort((a,b)=>a-b);
  const mid=Math.floor(positiveUncombined.length/2);
  const median=positiveUncombined.length?(positiveUncombined.length%2?positiveUncombined[mid]:(positiveUncombined[mid-1]+positiveUncombined[mid])/2):null;
  const representedCount=expected.length-missing.length;
  if(missing.length && (representedCount<18||missing.length>5||median===null))problem='Too little non-overlapping represented coverage for the approved partial-profile model';
  const modeled={...observations};
  const imputed={};
  if(!problem)for(const c of missing){modeled[c]=median;imputed[c]=median;}
  const sums=Object.fromEntries(Object.entries(PARTITION).map(([g,codes])=>[g,codes.filter(c=>!covered.has(c)).reduce((s,c)=>s+(modeled[c]??0),0)]));
  const total=Object.values(sums).reduce((a,b)=>a+b,0);
  if(!(total>0))problem='No positive non-overlapping value-added total';
  const weights=problem?null:Object.fromEntries(Object.entries(sums).map(([g,value])=>[g,value/total]));
  const quality=weights?(missing.length||aggregates.length?'partial_1990_model':'indstat_1990'):'model_fallback';
  return {observations,weights,notes,quality,model:{observed_divisions:observedCount,covered_divisions:representedCount,covered_by_parent:[...covered].sort(),aggregates,missing_divisions:missing,imputed_value_added:imputed,imputation_basis:missing.length&&!problem?'Within-country median of positive uncombined observed division value added; GAME MODEL estimate, not a historical observation.':null},reason:problem?problem+'; source observations retained, generic GAME MODEL profile used.':null};
}
function roster() {
  const text = fs.readFileSync(path.join(ROOT,'spheres-sim/src/data/embedded.rs'),'utf8');
  const files = [...text.matchAll(/file: "(data\/nations\/[^\"]+)"/g)].map(m => 'spheres-sim/' + m[1]);
  const list = files.map(file => ({...JSON.parse(fs.readFileSync(path.join(ROOT,file),'utf8')), file}));
  assert.equal(list.length,137); assert.equal(new Set(list.map(n => n.id)).size,137);
  assert.deepEqual(new Set(list.map(n=>n.id)),new Set(Object.keys(ISO)));
  return list;
}
function sha(value) { return crypto.createHash('sha256').update(value).digest('hex'); }
function json(value) { return JSON.stringify(value,null,2)+'\n'; }
async function collect(options) {
  const nations = roster();
  fs.mkdirSync(options.cache,{recursive:true});
  const refs = new Map();
  async function request(url, body) {
    const method = body ? 'POST' : 'GET';
    const requestKey = sha(JSON.stringify({url,method,body:body??null}));
    const cachePath = path.join(options.cache,requestKey+'.json');
    let envelope;
    if (!options.refresh && fs.existsSync(cachePath)) envelope = JSON.parse(fs.readFileSync(cachePath,'utf8'));
    else if (!options.fetch) throw Error('not_cached');
    else {
      const retrieved = new Date().toISOString();
      try {
        const response = await fetch(url,{method,headers:body?{'Content-Type':'application/json'}:{},body:body?JSON.stringify(body):undefined,signal:AbortSignal.timeout(15000)});
        const raw = await response.text();
        if (!response.ok) throw Error('HTTP '+response.status);
        const payload = decode(JSON.parse(raw));
        envelope = {ok:true,url,method,request:body??null,retrieved_utc:retrieved,response_sha256:sha(raw),raw_response:raw,payload};
      } catch(error) { envelope={ok:false,url,method,request:body??null,retrieved_utc:retrieved,error:String(error.message)}; }
      fs.writeFileSync(cachePath,json(envelope));
    }
    refs.set(requestKey,{cache_key:requestKey,url,method,request:body??null,retrieved_utc:envelope.retrieved_utc,response_sha256:envelope.response_sha256??null,ok:envelope.ok,error:envelope.error??null});
    if (!envelope.ok) throw Error(envelope.error);
    return {payload:decode(envelope.payload),cache_key:requestKey};
  }
  let na=null,ind=null,naError=null,indError=null;
  try { na=await request(META_NA); datasetId(na.payload); assertVariable(na.payload,'GdpCud',/GDP.*current USD/i); assertVariable(na.payload,'MvaCud',/MVA.*current USD/i); } catch(e) { naError=e.message;na=null; }
  if (!options.shareOnly) try { ind=await request(META_IND); datasetId(ind.payload); } catch(e) { indError=e.message;ind=null; }
  let indVariable=null;
  if(ind) {
    // A within-country mix cancels a common monetary unit. Do not claim USD:
    // INDSTAT metadata currently labels this series simply "Value added".
    const vars=[...objects(ind.payload)].filter(x => /^value added$/i.test(label(x)) && x.type==='M' && x.c!=null);
    const codes=[...new Set(vars.map(x=>String(x.c)))];
    if(codes.length===1) indVariable=codes[0]; else indError='No unique metadata-confirmed monetary value-added variable';
  }
  const output={}; let next=0,finished=0;
  async function worker() {
    for(;;) {
      const index=next++; if(index>=nations.length)return;
      const nation=nations[index],iso=ISO[nation.id],historical=iso==='-';
      const source={provider:'UNIDO',country_iso3:historical?null:iso,national_accounts_metadata_url:META_NA,indstat_metadata_url:META_IND,national_accounts_country_code:na?countryCode(na.payload,iso,nation.id):null,indstat_country_code:ind?countryCode(ind.payload,iso,nation.id):null,national_accounts_reporter:nation.id==='Sudan'?'Former Sudan':null,national_accounts_variables:['GdpCud','MvaCud'],indstat_variable:indVariable,national_accounts_cache_key:null,indstat_cache_key:null,footnotes:[]};
      const notes=['GAME CAPACITY ESTIMATE, not a literal factory or establishment census. Source GDP is retained for ratio provenance only; it does not replace the calibrated game GDP.'];
      let observed={gdp:null,mva:null,share:null},mix={observations:Object.fromEntries(Object.values(PARTITION).flat().map(c=>[c,null])),weights:null,notes:[],reason:null};
      const allowed=index<options.maxCountries && (!options.only || options.only.includes(nation.id));
      if(historical) notes.push('Historical federation: no modern successor proxy or summed successor series is used. National share and industry mix remain explicit MODEL fallbacks.');
      if(!allowed) notes.push('Not queried in this bounded collection pass.');
      if(!historical && allowed) {
        if(na && source.national_accounts_country_code) {
          try {
            const r=await request(API+'getDataWithoutActivities',{datasetId:datasetId(na.payload),countryCode:source.national_accounts_country_code,variableCodes:['GdpCud','MvaCud'],periods:['1990'],fullPrecision:true});
            source.national_accounts_cache_key=r.cache_key;observed=nationalAccounts(r.payload);
            source.national_accounts_observations=rows(r.payload);
            source.national_accounts_notes={ym:r.payload.ym??[],vm:r.payload.vm??[]};
          }catch(e){notes.push('National Accounts request unavailable: '+e.message);}
        }else notes.push('National Accounts metadata/country unavailable: '+(naError??'no unambiguous ISO3 match'));
        if(ind && indVariable && source.indstat_country_code) {
          try {
            const r=await request(API+'getData',{datasetId:datasetId(ind.payload),countryCode:source.indstat_country_code,variableCode:indVariable,activityCodes:Object.values(PARTITION).flat().sort(),periods:['1990'],fullPrecision:true});
            source.indstat_cache_key=r.cache_key;mix=composition(r.payload);source.footnotes=mix.notes;source.sector_mix_model=mix.model;
          }catch(e){notes.push('INDSTAT request unavailable: '+e.message);}
        }else notes.push('INDSTAT composition unavailable: '+(options.shareOnly?'share-only collection pass':indError??'no unambiguous ISO3 match'));
      }
      if(['Germany','Yemen','Ethiopia'].includes(nation.id)) notes.push('Boundary qualification: this is the reporter\'s retrospective annual 1990 series, not proof of 1 January 1990 territorial coverage. No successor summation is performed.');
      if(nation.id==='Sudan')notes.push('Historical boundary: metadata-resolved Former Sudan reporter is used for 1990 whole Sudan. Modern SDN is only the canonical ISO alias; if Former Sudan is absent in a dataset that source remains unknown, without substituting current Sudan.');
      if(nation.id==='Zaire')notes.push('Canonical Zaire is mapped to ISO COD (Democratic Republic of the Congo), not COG (Republic of the Congo).');
      if(observed.share===null)notes.push('No valid same-year current-USD MVA/GDP ratio was retrieved; source nulls stay unknown. Manufacturing share 0.20 is the existing generic GAME MODEL fallback.');
      if(!mix.weights)notes.push(mix.reason??'Five equal sector weights of 0.20 are a generic GAME MODEL fallback, not observed sector shares.');
      if(mix.quality==='partial_1990_model')notes.push('Partial 1990 GAME MODEL sector profile: same-group aggregate parents counted once; any explicitly listed missing divisions use the within-country median positive uncombined value. Raw source nulls remain unknown.');
      output[nation.id]={source_gdp_usd:observed.gdp,source_mva_usd:observed.mva,source_mva_share:observed.share,manufacturing_share:observed.share??0.2,share_quality:observed.share!==null?'sourced_1990':'model_fallback',sector_weights:mix.weights??{...FALLBACK_MIX},mix_quality:mix.quality??'model_fallback',source_sector_value_added:mix.observations,source,notes};
      finished++;if(finished%10===0 || finished===nations.length)console.error(`Collected ${finished}/${nations.length}`);
    }
  }
  await Promise.all(Array.from({length:4},worker));
  // Canonical roster order, never request completion order.
  const countries=Object.fromEntries(nations.map(n=>[n.id,output[n.id]]));
  const sourceRequests=[...refs.values()].sort((a,b)=>a.cache_key.localeCompare(b.cache_key));
  const artifact={schema_version:1,meta:{reference_year:1990,status:'source_backed_game_estimates_with_explicit_fallbacks',generator:'tools/industry/collect_industry_1990.cjs',canonical_roster:'spheres-sim/src/data/embedded.rs',count:nations.length,quantity_basis:'Same-year current-USD manufacturing value added / GDP; dimensionless share applied to existing calibrated game GDP. GAME CAPACITY ESTIMATES, not literal factory counts.',source_api_documentation:'https://stat.unido.org/unido-statistics-portal-api',sector_classification:'ISIC Revision 3 two-digit manufacturing groups 15 through 37',sector_partition:PARTITION,composition_policy:'Complete nonnegative 23-group same-country monetary value added is normalized; common currency cancels. Exact same-game-group NN includes NN annotations count the observed parent once and exclude covered children. Cross-group or unresolved aggregates and duplicate activities reject the mix. Partial profiles require at least18 represented divisions (observed or covered by a resolved parent) and at most5 uncovered missing divisions; only those missing model values use the within-country median of positive uncombined observations. Raw missing source values remain null. Such profiles are partial_1990_model, not historical observations. Metadata supplier/scope notes are retained without rejecting otherwise usable rows. Petroleum/refining group23 belongs to game materials, not chemicals24.',fallback:{manufacturing_share:0.2,sector_weights:FALLBACK_MIX,description:'Approved generic GAME MODEL fallback; not historical observations.'},coverage:{source_shares:Object.values(countries).filter(c=>c.share_quality==='sourced_1990').length,source_mixes:Object.values(countries).filter(c=>c.mix_quality==='indstat_1990').length,partial_model_mixes:Object.values(countries).filter(c=>c.mix_quality==='partial_1990_model').length,fallback_mixes:Object.values(countries).filter(c=>c.mix_quality==='model_fallback').length,historical_federation_fallbacks:['USSR','Yugoslavia','Czechoslovakia']},source_requests:sourceRequests},countries};
  validate(artifact,nations.map(n=>n.id));
  fs.mkdirSync(path.dirname(options.output),{recursive:true});fs.writeFileSync(options.output,json(artifact));
  console.log(JSON.stringify({output:options.output,countries:nations.length,...artifact.meta.coverage}));
  return artifact;
}
function validate(artifact,ids) {
  assert.deepEqual(Object.keys(artifact.countries),ids);
  const codes=Object.values(PARTITION).flat();assert.equal(codes.length,23);assert.equal(new Set(codes).size,23);
  for(const[id,r]of Object.entries(artifact.countries)) {
    assert(Number.isFinite(r.manufacturing_share)&&r.manufacturing_share>=0&&r.manufacturing_share<1,id);
    assert.deepEqual(Object.keys(r.sector_weights),Object.keys(PARTITION));
    assert(Object.values(r.sector_weights).every(x=>Number.isFinite(x)&&x>=0),id);
    assert(Math.abs(Object.values(r.sector_weights).reduce((a,b)=>a+b,0)-1)<1e-12,id);
    if(r.share_quality==='sourced_1990')assert.equal(r.manufacturing_share,r.source_mva_usd/r.source_gdp_usd);
    else assert.equal(r.manufacturing_share,0.2);
  }
}
function options(argv) {
  const value=(flag,fallback)=>{const i=argv.indexOf(flag);return i<0?fallback:argv[i+1];};
  const maxCountries=Number(value('--max-countries',137));if(!Number.isInteger(maxCountries)||maxCountries<0||maxCountries>137)throw Error('--max-countries must be 0..137');
  return {fetch:argv.includes('--fetch'),refresh:argv.includes('--refresh'),shareOnly:argv.includes('--share-only'),maxCountries,only:value('--only','').split(',').filter(Boolean).length?value('--only','').split(','):null,cache:path.resolve(value('--cache-dir',path.join(ROOT,'../../artifacts/industry-1990-cache'))),output:path.resolve(value('--output',path.join(ROOT,'spheres-sim/data/industry_1990.json')))};
}
module.exports={ISO,PARTITION,finiteObservation,nationalAccounts,composition,countryCode,validate,collect,options};
if(require.main===module)collect(options(process.argv.slice(2))).catch(e=>{console.error(e);process.exitCode=1;});
