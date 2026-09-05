#!/usr/bin/env node
'use strict';
// Frozen historical shares, never current live data in a running simulation.
// --fetch populates a cache; offline rebuild is the default. CC BY 4.0 WDI.
const fs=require('node:fs'),path=require('node:path'),assert=require('node:assert/strict');
const {ISO}=require('./collect_industry_1990.cjs');
const ROOT=path.resolve(__dirname,'../..'),CACHE=path.join(__dirname,'sector-source-cache');
const SERIES=['NV.AGR.TOTL.ZS','NV.IND.TOTL.ZS','NV.SRV.TOTL.ZS'];
const peers=()=>Object.fromEntries(Object.keys(ISO).map(id=>{const n=JSON.parse(fs.readFileSync(path.join(ROOT,'spheres-sim/data/nations',id+'.json'),'utf8'));return [id,n.economy.gdp_bn*1000/n.economy.population_m];}));
async function main(){
  fs.mkdirSync(CACHE,{recursive:true});const observations={};
  for(const series of SERIES){const file=path.join(CACHE,series+'.json');
    if(process.argv.includes('--fetch')){const url=`https://api.worldbank.org/v2/country/all/indicator/${series}?date=1988:1992&format=json&per_page=20000`;const r=await fetch(url);if(!r.ok)throw Error(`${url}: ${r.status}`);fs.writeFileSync(file,JSON.stringify({url,retrieved:'2026-09-04',response:await r.json()},null,2)+'\n');}
    const data=JSON.parse(fs.readFileSync(file,'utf8'));for(const r of data.response[1]||[]){if(!r.countryiso3code||r.value==null||!Number.isFinite(r.value)||r.value<0||r.value>100)continue;const key=r.countryiso3code;observations[key]??={};(observations[key][series]??=[]).push({year:Number(r.date),value:r.value/100});}
  }
  const choose=(iso,s)=>[...(observations[iso]?.[s]||[])].sort((a,b)=>Math.abs(a.year-1990)-Math.abs(b.year-1990)||a.year-b.year)[0];
  const industry=JSON.parse(fs.readFileSync(path.join(ROOT,'spheres-sim/data/industry_1990.json'),'utf8')).countries,pc=peers(),countries={};
  for(const [id,base] of Object.entries(industry)){
    const iso=ISO[id],observed=Object.fromEntries(SERIES.map(s=>[s,choose(iso,s)||null]));const fallback={};
    const values=SERIES.map(s=>{if(observed[s])return observed[s].value;
      const candidates=Object.keys(industry).filter(other=>choose(ISO[other],s)).sort((a,b)=>Math.abs(Math.log(pc[a]/pc[id]))-Math.abs(Math.log(pc[b]/pc[id]))||a.localeCompare(b)).slice(0,5);
      const v=candidates.map(other=>choose(ISO[other],s).value).sort((a,b)=>a-b);fallback[s]={method:'median of five nearest opening game GDP-per-capita countries with observations',peers:candidates};return v[Math.floor(v.length/2)];});
    // WDI sector sums can differ from GDP due to net product taxes/statistical
    // differences. Preserve the existing UNIDO manufacturing share exactly;
    // normalize the remaining agriculture/nonmanufacturing industry/services.
    const m=base.manufacturing_share;const residual=[values[0],Math.max(0,values[1]-m),values[2]],sum=residual.reduce((a,b)=>a+b,0),scale=(1-m)/sum;
    const [ag,nonman,sv]=residual.map(x=>x*scale);
    // Transparent game split, not observed sub-sector accounts.
    const shares=[ag,nonman*6/17,m,nonman*4/17,nonman*7/17,sv*10/55,sv*30/55,0];shares[7]=1-shares.slice(0,7).reduce((a,b)=>a+b,0);
    assert(shares.every(x=>Number.isFinite(x)&&x>=0));assert(Math.abs(shares.reduce((a,b)=>a+b,0)-1)<1e-12);
    countries[id]={shares,source:JSON.stringify({provider:'World Bank WDI',license:'CC BY 4.0',iso,observations:observed,fallback,manufacturing:'existing frozen UNIDO/game profile',normalization:'Preserve manufacturing; normalize the other broad sectors into remaining GDP.',subsector_model:'Nonmanufacturing industry extraction/utilities/construction 6:4:7; services transport/market/public 10:30:15. These splits are modeled, not observed.',urls:SERIES.map(s=>'https://data.worldbank.org/indicator/'+s)},null,2),quality:Object.values(observed).every(Boolean)?(Object.values(observed).every(v=>v.year===1990)?'sourced_1990_broad_sectors':'nearest_1988_1992_broad_sectors'):'explicit_income_peer_fallback'};
  }
  const out={schema_version:1,countries};fs.writeFileSync(path.join(ROOT,'spheres-sim/data/sectors_1990.json'),JSON.stringify(out,null,2)+'\n');
  const counts={};for(const r of Object.values(countries))counts[r.quality]=(counts[r.quality]||0)+1;console.log(JSON.stringify(counts));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
