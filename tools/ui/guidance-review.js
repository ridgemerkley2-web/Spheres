'use strict';
// Deliberately invented review scenarios, never a campaign API or saved world.
const sampleNation={id:'ExampleRepublic',name:'Example Republic',alive:true,treasury:4,annual_budget:{due:false}};
let sampleState, sampleProduction, scenario='opening';const memory=new Map();
function scenarioData(){
  const state={session_id:'sample-guidance-1',player:'ExampleRepublic',player_name:'Example Republic',date:'1 Jan 1990',year:1990,month:1,day:1,t:0,simulation_cadence:'daily',nations:[structuredClone(sampleNation),{id:'Neighbor',name:'Neighboring Republic',alive:true}],programs:{due:false},wars:[],research:{nation:'Example Republic',monthly:1,domains:[{name:'Energy',domain:'Energy',project:null,options:[{id:'example-energy',name:'Sample energy technology',year:1990}]}]}};
  const production={nation:'ExampleRepublic',mode:'province_projects',construction_budget:{enrolled:true,daily_budget_bn:.01,available_bn:.01},queue:[],mine_queue:[],completed:[],catalog:[],provinces:[],suggestions:{as_of_day:0,items:[]}};
  if(scenario==='opening')state.programs.due=true;
  if(scenario==='construction'){
    production.construction_budget.daily_budget_bn=0;
    production.queue=[{id:1,name:'Sample industrial workshop',status:'paused',progress:.35,reason:'The daily construction funding limit is zero.',province:{id:'sample-province',name:'Central province'}}];
  }
  if(scenario==='diplomacy')state.agency={offers:[{id:1,title:'A proposed diplomatic agreement',from:'Neighbor',from_name:'Neighboring Republic',expires:'1990-01-01',days_remaining:0,consequence:'The full agreement and its costs are available in the decision preview.'}]};
  if(scenario==='military'){
    state.nations[0].takeover={coup:{open:true,half_armed:true,armed:false,reason:'Review the reported political conditions before choosing a response.'}};
    state.wars=[{id:1,theatre_name:'Sample theatre',class:'limited conflict',posture:[{id:'ExampleRepublic'}]}];
    state.operations={enabled:true,deployments:[{nation:'ExampleRepublic',conflict:1,requested:8,deployed:5}]};
  }
  if(scenario==='empty'){state.research=null;return {state,production:null};}
  return {state,production};
}
function resetSample(){const data=scenarioData();sampleState=data.state;sampleProduction=data.production;}
resetSample();
const review=GuidanceUI.mount({
  getState:()=>sampleState,canNavigate:()=>true,pause:()=>{},busy:()=>false,
  supports:()=>true,storage:{getItem:key=>memory.get(key),setItem:(key,value)=>memory.set(key,value)},
  asset:key=>`../../spheres-web/ui/area-art/${key}-v1.webp`,
  readSnapshot:async()=>{if(scenario==='offline')throw new Error('Sample: the game connection is unavailable. You can retry the briefing and continue reading the tutorial.');return {state:structuredClone(sampleState),production:structuredClone(sampleProduction)};},
  navigate(action){const target=document.getElementById('destination');target.replaceChildren();const heading=document.createElement('h2');heading.textContent='Review destination: '+action.kind;const note=document.createElement('p');note.textContent='In the game this opens the existing review screen. This isolated page records the destination only, so it cannot issue an order or change a campaign.';target.append(heading,note);target.scrollIntoView({block:'center'});}
});
document.getElementById('learn').onclick=()=>review.open('tutorial');
document.getElementById('advice').onclick=()=>review.open('advisors');
document.getElementById('guide').onclick=()=>review.open('glossary');
document.getElementById('scenario').onchange=e=>{scenario=e.target.value;resetSample();review.changed();review.open('advisors');};
document.getElementById('change').onclick=()=>{sampleState={...sampleState,date:'2 Jan 1990',t:1,day:2};if(sampleProduction)sampleProduction={...sampleProduction,suggestions:{...sampleProduction.suggestions,as_of_day:1}};review.changed();review.open('advisors');};
review.open('tutorial');
