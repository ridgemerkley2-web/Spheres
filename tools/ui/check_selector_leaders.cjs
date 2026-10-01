// Exercise the actual game host: starting and saved campaigns have distinct leaders.
const {test}=require('node:test'),assert=require('node:assert/strict');
const fs=require('node:fs'),path=require('node:path'),vm=require('node:vm');
const page=fs.readFileSync(path.join(__dirname,'../../spheres-web/ui/index.html'),'utf8');
function source(name){
  const start=new RegExp(`^(?:async )?function ${name}\\(`,'m').exec(page);assert(start,name);
  const end=page.indexOf('\n}',start.index);assert(end>start.index);return page.slice(start.index,end+2);
}
function fixture(){
  const context=vm.createContext({Date});
  vm.runInContext(['campaignLeaderDate','figureFor','figureInitials','loadFigurePortrait','dominationFigureDecor'].map(source).join('\n'),context);
  return context;
}
function nation(name='Opening leader',date='1990-01-01',extra={}){
  return {id:'France',name:'France',campaign_leader:{date,person_id:'francois_mitterrand',name,office:'President',identity_status:'linked_person',
    portrait:{url:'/art/people/francois-mitterrand-cartoon-1990-v1.png',from:'1990-01-01',to:'1991-01-01',method:'generated',style:'cartoon',status:'illustrated-likeness',credit:'Actual source credit'},...extra}};
}
test('opening roster and live state resolve only their served leaders on their own dates',()=>{
  const c=fixture(),opening=nation(),live=nation('Campaign successor','1996-04-02',{person_id:'another_person',portrait:null});
  assert.equal(c.figureFor(opening,'1990-01-01').figure,'Opening leader');
  assert.equal(c.figureFor(live,'1996-04-02').figure,'Campaign successor');
  assert.equal(c.figureFor(live,'1990-01-01').figure,'Leader not recorded');
  assert.equal(c.figureFor(opening,'1996-04-02').portrait,null);
});
test('missing, unlinked and institutional holders never borrow artwork',()=>{
  const c=fixture();
  assert.equal(c.figureFor({id:'France',leader_art:{asset:'historical-icon.png'}},'1990-01-01').portrait,null);
  const unlinked=nation('New campaign name','1990-01-01',{identity_status:'unlinked_person',person_id:null});
  assert.equal(c.figureFor(unlinked,'1990-01-01').figure,'New campaign name');
  assert.equal(c.figureFor(unlinked,'1990-01-01').portrait,null);
  assert.equal(c.figureFor(nation(null,'1990-01-01',{identity_status:'institutional',person_id:null}),'1990-01-01').figure,'Campaign leadership');
  assert.equal(c.figureFor(undefined,'1990-01-01').figure,'Leader not recorded');
});
test('portrait needs an exact linked identity, approved cartoon method and local people URL',()=>{
  const c=fixture();assert(c.figureFor(nation(),'1990-01-01').portrait);
  for(const extra of [{person_id:null},{identity_status:'unavailable'}])assert.equal(c.figureFor(nation('Name','1990-01-01',extra),'1990-01-01').portrait,null);
  for(const patch of [{url:'/art/portraits/France-icon.png'},{url:'https://example.org/a.png'},{url:'//example.org/a.png'},{url:'/art/people/../icon.png'},{style:'photo'},{method:'archival'},{status:'pending'},{from:''},{from:'1990-02-30'},{to:'someday'}]){
    const n=nation();Object.assign(n.campaign_leader.portrait,patch);assert.equal(c.figureFor(n,'1990-01-01').portrait,null,JSON.stringify(patch));
  }
});
test('appearance intervals are half open and do not authorize another age',()=>{
  const c=fixture();
  for(const date of ['1989-12-31','1991-01-01','2035-01-01'])assert.equal(c.figureFor(nation('Name',date),date).portrait,null,date);
  const fictional=nation('Fictional successor','2030-01-01',{person_id:'fictional_v1_france_01',origin:'fictional_successor'});
  Object.assign(fictional.campaign_leader.portrait,{from:'2026-09-08',to:'2036-01-01',status:'fictional-character'});
  const result=c.figureFor(fictional,'2030-01-01');assert.equal(result.figure,'Fictional successor');assert.equal(result.fictional,true);
  fictional.campaign_leader.portrait=null;
  assert.equal(c.figureFor(fictional,'2030-01-01').fictional,true,'fiction is an identity property even when art is missing');
});
test('date parsing uses authoritative board components and rejects invalid dates',()=>{
  const c=fixture();assert.equal(c.campaignLeaderDate({year:1990,month:1,day:1,date:'January 1990'}),'1990-01-01');
  assert.equal(c.campaignLeaderDate({date:'1992-02-29'}),'1992-02-29');
  for(const board of [{date:'1991-02-29'},{year:1990,month:13,day:1},{year:1990,month:2,day:31},{},null])assert.equal(c.campaignLeaderDate(board),'');
});
test('changing a leader clears old image and credit, and failed loads retain initials',()=>{
  const c=fixture(),classes=new Set(),attributes={},img={dataset:{},classList:{remove:v=>classes.delete(v),add:v=>classes.add(v),toggle:(v,on)=>on?classes.add(v):classes.delete(v)},removeAttribute(k){delete attributes[k];if(k==='src')delete this.src;if(k==='title')delete this.title;},setAttribute(k,v){attributes[k]=v;}};
  c.loadFigurePortrait(img,c.figureFor(nation(),'1990-01-01'),true);assert.match(img.src,/^\/art\/people\//);assert.equal(img.title,'Actual source credit');
  const oldLoad=img.onload;oldLoad();assert(classes.has('ready'));img.onerror();assert(!classes.has('ready'));
  c.loadFigurePortrait(img,c.figureFor(nation('Replacement','1990-01-01',{portrait:null}),'1990-01-01'),false);
  oldLoad();assert(!classes.has('ready'));assert.equal(img.src,undefined);assert.equal(img.title,undefined);assert.equal(img.onload,null);
});
test('War Room decoration uses live country leader rather than opening roster or national icon',()=>{
  const c=fixture();c.S={player:'France',year:1990,month:1,day:1,nations:[nation()]};c.logisticsEscAttr=s=>String(s).replaceAll('"','&quot;');
  assert.match(c.dominationFigureDecor({}),/art\/people\/francois/);
  c.S.nations[0].campaign_leader=nation('New leader','1991-01-01',{portrait:null}).campaign_leader;c.S.year=1991;
  assert.doesNotMatch(c.dominationFigureDecor({}),/<img/);assert.match(c.dominationFigureDecor({}),/flag-France/);
});
test('game selector does not fetch national representative figures or describe them as leaders',()=>{
  assert.doesNotMatch(source('buildSetup'),/nation-figures|setupFigures/);
  assert.doesNotMatch(page,/setupFigures|Historical avatar|A figure from this nation's history/);
  assert.match(source('renderShowcase'),/figureFor\(n, setupDate\)/);
  assert.match(source('renderPick'),/figureFor\(n, setupDate\)/);
  assert.match(source('renderDominationIdentity'),/figureFor\(nation, campaignLeaderDate\(S\)\)/);
});
