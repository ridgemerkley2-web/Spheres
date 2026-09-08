// Actual server, browser command channel, daily simulation and save/load.
// Only the opposing-player acceptance branch uses a fixture: a real saved
// campaign with its player seat changed, preserving every simulation value.
// SPHERES_BINARY selects the exact build; optional SPHERES_SAVE_DIR and
// SPHERES_PORT select this script's disposable parent directory and port.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),net=require('node:net'),cp=require('node:child_process');
const {chromium}=require('playwright');
const root=path.resolve(__dirname,'../..'),out=path.resolve(process.env.SPHERES_SAVE_DIR||path.join(root,'artifacts/browser-ci/campaign-integration'));
async function availablePort(){const server=net.createServer();await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));const value=server.address().port;await new Promise(resolve=>server.close(resolve));return value;}
(async()=>{
  fs.mkdirSync(out,{recursive:true});const run=fs.mkdtempSync(path.join(out,'run-'));
  const port=process.env.SPHERES_PORT?Number(process.env.SPHERES_PORT):await availablePort(),url=`http://127.0.0.1:${port}`;
  const binary=process.env.SPHERES_BINARY||path.resolve(process.env.CARGO_TARGET_DIR||path.join(root,'target'),'release','spheres-web'+(process.platform==='win32'?'.exe':''));
  assert(fs.existsSync(binary),`Build the web server first: ${binary}`);
  const server=cp.spawn(binary,['--port',String(port),'--no-open'],{cwd:run,windowsHide:true,stdio:['ignore','pipe','pipe']});
  const log=fs.createWriteStream(path.join(run,'server.log'));server.stdout.pipe(log);server.stderr.pipe(log);
  let browser,page;const errors=[];
  try{
    for(let attempt=0;;attempt++){
      try{if((await fetch(url+'/api/state')).ok)break;}catch(_){}
      if(server.exitCode!==null)throw Error(`Server exited: ${server.exitCode}`);
      if(attempt>=300)throw Error('Server did not start.');await new Promise(resolve=>setTimeout(resolve,100));
    }
    browser=await chromium.launch({headless:true});page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
    page.setDefaultTimeout(30000);page.on('pageerror',error=>errors.push(error.message));
    const read=async()=>{const response=await page.request.get(url+'/api/state');assert(response.ok());return response.json();};
    const send=async command=>page.evaluate(async command=>{const response=await api('/api/command',{commands:[command]});await adopt(response,false);return response;},command);
    const accepted=async command=>{const response=await send(command);assert.deepEqual(response.errors||[],[],`Refused ${JSON.stringify(command)}: ${JSON.stringify(response.errors)}`);return response;};
    const loadSlot=async slot=>{const response=await page.request.post(url+'/api/load',{data:{slot}});assert(response.ok(),await response.text());await page.reload();await page.locator('#continueBtn').click();await page.locator('#app').waitFor({state:'visible'});return read();};
    const open=async conflict=>{await page.evaluate(id=>openConflict(id),conflict);await page.locator('#sheet .campaign-command').waitFor({state:'visible'});};
    await page.goto(url);await page.locator('#campaignHome').waitFor({state:'visible'});
    await page.waitForFunction(()=>!!SESSION.live);
    await page.locator('#newCampaignBtn').click();await page.locator('#nationPick [aria-label^="Iraq;"]').click();await page.locator('#startBtn').click();await page.locator('#app').waitFor({state:'visible'});
    let state=await read();assert.equal(state.player,'Iraq');assert.equal(state.simulation_cadence,'daily');
    const build=state.build;fs.writeFileSync(path.join(run,'initial.json'),JSON.stringify({date:state.date,build,player:state.player,political_capital:state.nations.find(n=>n.id==='Iraq').political_capital},null,2));
    state=await accepted({kind:'war',target:'Iran'});
    const war=state.wars.find(w=>w.attacker_id==='Iraq'&&w.defender_id==='Iran');assert(war,'Declared conflict missing');const cid=war.id;
    assert(war.operation?.enabled,'The browser did not enable operational campaigns');
    assert(war.operation.targets.length,'No operational districts were available');
    const target=war.operation.targets[0].id;
    await open(cid);const form=page.locator('#sheet [data-campaign-command="operation"]');
    await form.getByText('Probe',{exact:true}).click();await form.getByLabel('Objective district',{exact:true}).selectOption(target);
    await form.getByLabel('Keep in reserve (%)',{exact:true}).fill('12.34');await form.getByLabel('Air mission',{exact:true}).selectOption('reconnaissance');
    await form.getByRole('button',{name:'Apply operation · free',exact:true}).click();
    await page.waitForFunction(id=>S.wars.find(w=>w.id===id)?.operation?.order.reserve_bp===1234,cid);
    state=await read();let live=state.wars.find(w=>w.id===cid);assert.equal(live.operation.order.approach,'probe');assert.equal(live.operation.order.target,target);
    assert.equal(live.operation.order.nation,'Iraq');
    // A forged caller cannot redirect an operation to another participant.
    state=await accepted({kind:'operation',conflict:cid,nation:'Iran',target,approach:'advance',reserve_bp:1500,air:'none',naval:'none'});
    assert.equal(state.wars.find(w=>w.id===cid).operation.order.nation,'Iraq');
    const beforeRefusal=state.wars.find(w=>w.id===cid).operation.order;
    state=await send({kind:'operation',conflict:cid,target:'not-a-district',approach:'advance',reserve_bp:1500,air:'none',naval:'none'});
    assert(state.errors?.length,'Invalid target was silently accepted');assert.deepEqual(state.wars.find(w=>w.id===cid).operation.order,beforeRefusal);
    state=await accepted({kind:'war_diplomacy',order:{kind:'set_aim',conflict:cid,aim:{kind:'concession'}}});
    assert.equal(state.wars.find(w=>w.id===cid).peace.aim.kind,'concession');
    state=await accepted({kind:'war_diplomacy',order:{kind:'garrison',conflict:cid,share_bp:1000,policy:'restraint'}});
    assert.equal(state.wars.find(w=>w.id===cid).peace.garrison.share_bp,1000);
    const priorDay=state.day;
    for(let day=0;day<3;day++)state=await page.evaluate(async()=>{const response=await api('/api/advance',{days:1});await adopt(response,false);return response;});
    assert(state.day>priorDay,'Daily steps did not advance');live=state.wars.find(w=>w.id===cid);assert(live?.operation?.last_report,'Actual daily resolution did not produce a field report');
    const enemy=live.posture.find(n=>n.id==='Iran');assert.equal(enemy.resolve,null);assert.equal(enemy.committed,null);assert.equal(enemy.munitions,null);
    await open(cid);
    // Apply another order through the real module after a live tick replaced
    // the conflict sheet, exercising rebinding and nullable enemy fields.
    await page.locator('#sheet [data-campaign-command="operation"]').getByText('Hold',{exact:true}).click();
    await page.locator('#sheet [data-campaign-command="operation"]').getByRole('button').click();
    await page.waitForFunction(id=>S.wars.find(w=>w.id===id)?.operation?.order.approach==='hold',cid);
    state=await accepted({kind:'war_diplomacy',order:{kind:'propose',conflict:cid,terms:{kind:'ceasefire'}}});
    live=state.wars.find(w=>w.id===cid);assert(live.peace.proposals.length);const offer=live.peace.proposals[0].id;
    const saveResponse=await page.request.post(url+'/api/save',{data:{slot:'campaign-integration'}});assert(saveResponse.ok());const saved=await saveResponse.json();
    const beforeLoad=live.operation.order;
    state=await loadSlot('campaign-integration');live=state.wars.find(w=>w.id===cid);assert.deepEqual(live.operation.order,beforeLoad);assert(live.peace.proposals.some(p=>p.id===offer));
    await open(cid);
    for(const [name,width,height] of [['desktop',1440,1000],['mobile',390,844]]){
      await page.setViewportSize({width,height});assert(await page.locator('#sheet .campaign-command').evaluate(node=>node.scrollWidth<=node.clientWidth+1),`${name} operation board overflow`);
      await page.locator('#sheet .campaign-heading').evaluate(node=>{const sheet=document.querySelector('#sheet');sheet.scrollTop+=node.getBoundingClientRect().top-sheet.getBoundingClientRect().top-30;});
      await page.screenshot({path:path.join(run,`${name}.png`),fullPage:true});
    }
    // The public API has one human seat. Import a fork of the real save with
    // only that seat changed to exercise the other government's consent UI.
    const savedPath=path.resolve(run,saved.path);assert(savedPath.startsWith(run+path.sep),'Test save escaped its disposable directory');
    const archive=JSON.parse(fs.readFileSync(savedPath,'utf8'));
    const world=archive.world?.format==='spheres-equipment-save'?archive.world.world:archive.world;
    assert(world&&world.player==='Iraq','Unexpected campaign save shape');world.player='Iran';archive.player='Iran';
    const alternate=path.join(run,'saves/campaign-second-seat.json');fs.writeFileSync(alternate,JSON.stringify(archive));
    state=await loadSlot('campaign-second-seat');assert.equal(state.player,'Iran');await open(cid);
    let offerForm=page.locator(`#sheet [data-campaign-command="respond"][data-offer="${offer}"]`);
    await offerForm.getByRole('button',{name:'Reject offer',exact:true}).click();
    await page.waitForFunction(({cid,offer})=>!S.wars.find(w=>w.id===cid)?.peace.proposals.some(p=>p.id===offer),{cid,offer});
    assert((await read()).wars.some(w=>w.id===cid),'Rejecting peace ended the conflict');
    state=await loadSlot('campaign-second-seat');await open(cid);offerForm=page.locator(`#sheet [data-campaign-command="respond"][data-offer="${offer}"]`);
    await offerForm.getByRole('button',{name:'Accept terms · free',exact:true}).click();
    await page.waitForFunction(cid=>!S.wars.some(w=>w.id===cid),cid);
    assert(!(await read()).wars.some(w=>w.id===cid),'Accepted bilateral ceasefire remained active');
    assert.deepEqual(errors,[],'Browser runtime errors');
    const result={passed:true,build,binary,run,new_game:'Iraq 1990; normal declare-war command against Iran',actual_operations:true,invalid_target_refused:true,actor_bound:true,
      actual_daily_reports:true,exact_enemy_fields_hidden:true,save_reload_preserves_order_and_offer:true,opposing_consent:'real save fork; player seat only changed to Iran',reject_and_accept:true,viewports:[1440,390],console_errors:errors};
    fs.writeFileSync(path.join(run,'result.json'),JSON.stringify(result,null,2));console.log(JSON.stringify(result,null,2));
  }catch(error){if(page)await page.screenshot({path:path.join(run,'failure.png'),fullPage:true}).catch(()=>{});fs.writeFileSync(path.join(run,'failure.json'),JSON.stringify({message:error.message,console_errors:errors},null,2));throw error;}
  finally{if(browser)await browser.close();server.kill();log.end();}
})().catch(error=>{console.error(error);process.exitCode=1;});
