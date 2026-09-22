// S18 art bench: actual WebGL, all CP1 meshes, controls and portable downloads.
const {chromium}=require('playwright'),assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path'),http=require('node:http'),crypto=require('node:crypto');
const root=path.resolve(__dirname,'../..');
(async()=>{
  const output=path.resolve(process.env.SPHERES_ART_OUTPUT||path.join(root,'../evidence/S18-art'));
  fs.mkdirSync(output,{recursive:true});const out=fs.mkdtempSync(path.join(output,'inspection-'));
  const server=http.createServer((req,res)=>{const file=path.resolve(root,'.'+decodeURIComponent(new URL(req.url,'http://localhost').pathname));if(!file.startsWith(root+path.sep)){res.writeHead(403).end();return;}fs.readFile(file,(error,data)=>{if(error){res.writeHead(404).end();return;}res.setHeader('Content-Type',({'.html':'text/html','.css':'text/css','.js':'text/javascript','.jpg':'image/jpeg','.png':'image/png'})[path.extname(file)]||'application/octet-stream');res.end(data);});});
  await new Promise(r=>server.listen(0,'127.0.0.1',r));
  let browser;const result={passed:false,aircraft:[],screenshots:[],errors:[]};
  try{
    browser=await chromium.launch({headless:true,channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome'});
    const page=await browser.newPage({viewport:{width:1440,height:1100},reducedMotion:'reduce'});page.on('pageerror',e=>result.errors.push(e.message));
    await page.goto(`http://127.0.0.1:${server.address().port}/tools/arsenal/flight-command.html`);
    assert.equal(await page.locator('[data-mission]').count(),0,'No demonstration mission actions remain');
    assert(!/Northfield|Falcon Squadron|8 ready/.test(await page.locator('body').innerText()));
    for(const platform of ['air_light_attack','air_fighter','air_tactical_strike']){
      await page.selectOption('#platform',platform);
      await page.waitForFunction(()=>document.querySelector('#model-host canvas')&&document.querySelector('#mesh-count').textContent.includes('triangles'));
      const triangles=Number((await page.locator('#mesh-count').textContent()).split(' triangles')[0].replaceAll(',',''));assert(triangles>=100000);
      for(const view of ['exterior','cockpit','engine','intake']){
        if(view==='exterior')await page.locator('[data-model-reset]').click();
        else await page.locator(view==='cockpit'?'[data-model-focus="air_avionics"]':view==='engine'?'[data-model-focus="air_engine"]:not([data-model-focus-region])':'[data-model-focus-region="front"]').click();
        await page.locator('#model-host').scrollIntoViewIfNeeded();await page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));
        const name=`${platform}-${view}.png`;await page.locator('#model-host').screenshot({path:path.join(out,name)});result.screenshots.push(name);
      }
      const downloadPromise=page.waitForEvent('download');await page.locator('#download').click();const download=await downloadPromise;assert.equal(await download.failure(),null);
      const chunks=[];for await(const chunk of await download.createReadStream())chunks.push(chunk);const bytes=Buffer.concat(chunks);assert.equal(bytes.readUInt32LE(0),0x46546c67);assert.equal(bytes.readUInt32LE(8),bytes.length);
      const json=JSON.parse(bytes.subarray(20,20+bytes.readUInt32LE(12)).toString()),extras=json.meshes[0].extras;
      assert.equal(extras.triangleCount,triangles);assert.equal(extras.specification.platform,platform);assert.equal(new Set(extras.parts.map(p=>p.slot)).size,8);assert(extras.surfaces.length);
      const lods=[];for(const lod of ['1','2']){await page.selectOption('#detail',lod);const n=Number((await page.locator('#mesh-count').textContent()).split(' triangles')[0].replaceAll(',',''));assert(n<(lod==='1'?18000:2500));lods.push(n);}
      await page.selectOption('#detail','0');await page.selectOption('[data-slot="air_fuel"]','air_fuel_extended');
      const choices=await page.locator('#part option').evaluateAll(xs=>xs.map(x=>x.value).filter(Boolean));for(const slot of ['air_engine','air_wing','air_radar','air_avionics','air_countermeasures','air_hardpoints','air_payload','air_fuel']){const part=choices.find(x=>x.startsWith(slot+' /'));assert(part);await page.selectOption('#part',part);assert.equal(await page.inputValue('#part'),part);}
      result.aircraft.push({platform,triangles,lods,glb_bytes:bytes.length,glb_sha256:crypto.createHash('sha256').update(bytes).digest('hex'),selectable_slots:8});
    }
    await page.setViewportSize({width:390,height:844});await page.locator('[data-model-reset]').click();await page.locator('#model-host').scrollIntoViewIfNeeded();
    assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth));await page.screenshot({path:path.join(out,'workshop-narrow.png')});result.screenshots.push('workshop-narrow.png');
    assert.deepEqual(result.errors,[]);result.passed=true;
  }catch(error){result.failure=String(error.stack||error);throw error;}
  finally{fs.writeFileSync(path.join(out,'result.json'),JSON.stringify(result,null,2)+'\n');if(browser)await browser.close();await new Promise(r=>server.close(r));console.log(path.join(out,'result.json'));}
})().catch(e=>{console.error(e);process.exitCode=1;});
