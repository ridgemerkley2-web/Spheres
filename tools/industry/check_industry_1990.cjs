'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const c = require('./collect_industry_1990.cjs');

test('mapping covers exactly the canonical 137 IDs with explicit former-federation gaps', () => {
  const root=path.resolve(__dirname,'../..');
  const embedded=fs.readFileSync(path.join(root,'spheres-sim/src/data/embedded.rs'),'utf8');
  const ids=[...embedded.matchAll(/file: "(data\/nations\/[^\"]+)"/g)].map(m=>JSON.parse(fs.readFileSync(path.join(root,'spheres-sim',m[1]),'utf8')).id);
  assert.equal(ids.length,137);assert.deepEqual(new Set(ids),new Set(Object.keys(c.ISO)));
  assert.deepEqual(Object.entries(c.ISO).filter(([,iso])=>iso==='-').map(([id])=>id),['USSR','Yugoslavia','Czechoslovakia']);
  assert.equal(c.ISO.Zaire,'COD');assert.equal(c.ISO.Congo,'COG');
});
test('missing, suppressed and non-finite observations never coerce to zero', () => {
  for(const value of [null,undefined,false,true,'',' ','..','NaN','Infinity',NaN,Infinity,{}])assert.equal(c.finiteObservation(value),null);
  assert.equal(c.finiteObservation(0),0);assert.equal(c.finiteObservation('0'),0);
  assert.equal(c.finiteObservation('1.25e4'),12500);
});
test('national share requires unique matching-year current-USD numerator and denominator', () => {
  const gdp=5963144000000,mva=1043089112786;
  const good={data:[{p:'1990',c:'GdpCud',v:gdp},{p:'1990',c:'MvaCud',v:mva}]};
  assert.deepEqual(c.nationalAccounts(good),{gdp,mva,share:mva/gdp});
  assert.equal(c.nationalAccounts({data:[...good.data,good.data[1]]}).share,null);
  assert.equal(c.nationalAccounts({data:good.data.map(r=>({...r,p:'1991'}))}).share,null);
  assert.equal(c.nationalAccounts({data:[{p:'1990',c:'GdpCud',v:gdp},{p:'1990',c:'MvaCud',v:null}]}).share,null);
  assert.equal(c.nationalAccounts({data:[{p:'1990',c:'GdpCud',v:1},{p:'1990',c:'MvaCud',v:2}]}).share,null);
  assert.equal(c.nationalAccounts({data:[{p:'1990',c:'GdpCud',v:1},{p:'1990',c:'MvaCud',v:0}]}).share,0);
});
test('metadata ISO mapping is exact and refuses ambiguous country codes', () => {
  assert.equal(c.countryCode({countries:[{iso3:'USA',c:'840'}]},'USA'),'840');
  assert.equal(c.countryCode(JSON.stringify({countries:[{iso3:'USA',c:'840'}]}),'USA'),'840');
  assert.equal(c.countryCode({countries:[{iso3:'USA',c:'840'},{iso3:'USA',c:'999'}]},'USA'),null);
  assert.equal(c.countryCode({countries:[{iso3:'RUS',c:'643'}]},'-'),null);
  const sudan={countries:[{iso3:'SDN',c:'729',lang:{en:'Sudan'}},{iso3:'NA ',c:'736',lang:{en:'Former Sudan'}}]};
  assert.equal(c.countryCode(sudan,'SDN','Sudan'),'736');
  assert.equal(c.countryCode({countries:[sudan.countries[0]]},'SDN','Sudan'),null);
});
function complete(){return {data:Array.from({length:23},(_,i)=>({p:'1990',a:String(i+15),v:1})),ym:[],am:[]};}
test('five model groups form one nonoverlapping partition of ISIC 15 through 37', () => {
  const codes=Object.values(c.PARTITION).flat();assert.equal(codes.length,23);
  assert.deepEqual([...codes].sort(),Array.from({length:23},(_,i)=>String(i+15)));
  const mix=c.composition(complete());assert(mix.weights);
  assert.equal(mix.weights.food_textiles,5/23);assert.equal(mix.weights.materials,8/23);
  assert.equal(mix.weights.chemicals,1/23);assert.equal(mix.weights.machinery_electronics,7/23);assert.equal(mix.weights.other,2/23);
});
test('approved partial profiles label imputation without replacing unknown source values', () => {
  const partial=complete();partial.data.pop();
  const parsed=c.composition(partial);assert.equal(parsed.quality,'partial_1990_model');assert.equal(parsed.observations['37'],null);assert.equal(parsed.observations['15'],1);
  assert.deepEqual(parsed.model.imputed_value_added,{'37':1});
  const sparse=complete();sparse.data.splice(17);assert.equal(c.composition(sparse).weights,null);
  const aggregate=complete();aggregate.data[0].cn='Includes activity 16';
  assert.equal(c.composition(aggregate).weights,null);assert(c.composition(aggregate).notes.length>0);
  const scoped=complete();scoped.ym=[{note:'Supplier: national statistical office'}];assert(c.composition(scoped).weights);assert(c.composition(scoped).notes.length);
  const duplicate=complete();duplicate.data.push({...duplicate.data[0]});assert.equal(c.composition(duplicate).weights,null);
});
test('same-group repeated parent/child notes are counted once; USD companion is not a note', () => {
  const p=complete();p.data.pop();p.data.forEach(r=>r.u=999);
  for(const[parent,child]of [['18','19'],['29','30'],['31','32'],['34','35']]){
    const a=p.data.find(r=>r.a===parent),b=p.data.find(r=>r.a===child);
    a.v=10;delete b.v;a.c=b.c=parent+'A';a.cn=b.cn=parent+' includes '+child;
  }
  const mix=c.composition(p);assert.equal(mix.quality,'partial_1990_model');assert.equal(mix.model.observed_divisions,18);
  assert.deepEqual(mix.model.covered_by_parent,['19','30','32','35']);assert.deepEqual(mix.model.imputed_value_added,{'37':1});
  assert.equal(mix.weights.food_textiles,13/55);assert.equal(mix.observations['19'],null);
  const cross=complete();cross.data[0].cn='15 includes 24';assert.equal(c.composition(cross).weights,null);
  const overlap=complete();overlap.data[0].cn='15 includes 16';overlap.data[2].cn='17 includes 16';assert.equal(c.composition(overlap).weights,null);
});
test('observed zero is distinct from missing; all-zero or negative mix cannot normalize', () => {
  const some=complete();some.data[0].v=0;assert.equal(c.composition(some).observations['15'],0);assert(c.composition(some).weights);
  const zero=complete();zero.data.forEach(r=>r.v=0);assert.equal(c.composition(zero).weights,null);
  const negative=complete();negative.data[0].v=-1;assert.equal(c.composition(negative).weights,null);
});
test('represented coverage includes resolved child divisions, not just numeric parents', () => {
  const p=complete();p.data.pop();
  for(const[parent,children]of [['15',['16','17','18','19']],['29',['30','31']]]){
    const note=parent+' includes '+children.join(' and ');
    const a=p.data.find(r=>r.a===parent);a.v=10;a.c='aggregate';a.cn=note;
    for(const child of children){const b=p.data.find(r=>r.a===child);delete b.v;b.c='aggregate';b.cn=note;}
  }
  const mix=c.composition(p);assert.equal(mix.model.observed_divisions,16);assert.equal(mix.model.covered_divisions,22);
  assert.equal(mix.quality,'partial_1990_model');assert.deepEqual(mix.model.imputed_value_added,{'37':1});
});
test('current artifact coverage and sourced ratios reconcile without changing source nulls', () => {
  const root=path.resolve(__dirname,'../..');
  const artifact=JSON.parse(fs.readFileSync(path.join(root,'spheres-sim/data/industry_1990.json'),'utf8'));
  const ids=Object.keys(c.ISO);c.validate(artifact,ids);
  const entries=Object.entries(artifact.countries);
  assert.equal(artifact.meta.coverage.source_shares,entries.filter(([,r])=>r.share_quality==='sourced_1990').length);
  assert.equal(artifact.meta.coverage.source_mixes+artifact.meta.coverage.partial_model_mixes+artifact.meta.coverage.fallback_mixes,137);
  for(const[,r]of entries){
    if(r.share_quality==='sourced_1990')assert.equal(r.source_mva_share,r.source_mva_usd/r.source_gdp_usd);
    for(const code of Object.keys(r.source.sector_mix_model?.imputed_value_added??{}))assert.equal(r.source_sector_value_added[code],null);
  }
  for(const id of ['USSR','Yugoslavia','Czechoslovakia']){assert.equal(artifact.countries[id].source_mva_share,null);assert.equal(artifact.countries[id].share_quality,'model_fallback');}
});
test('final artifact retains a nonuniform non-fallback sector mix', () => {
  const root=path.resolve(__dirname,'../..');
  const artifact=JSON.parse(fs.readFileSync(path.join(root,'spheres-sim/data/industry_1990.json'),'utf8'));
  assert(Object.values(artifact.countries).some(row=>{
    const weights=Object.values(row.sector_weights);
    return row.mix_quality!=='model_fallback' && Math.max(...weights)-Math.min(...weights)>1e-12;
  }),'A share-only output must not overwrite the final sector-composition artifact');
});
