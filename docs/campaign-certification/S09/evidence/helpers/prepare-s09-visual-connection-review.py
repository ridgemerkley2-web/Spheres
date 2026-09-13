import pathlib
base=pathlib.Path(__file__).resolve().parent
source=(base/'run-s09-visual-review.cjs').read_text()
source=source.replace("const expected='94d2c094b2c69a70613af2258acbf49e10d9e871';", "const expected=process.env.SPHERES_EXPECTED_REVISION;assert.match(expected||'',/^[a-f0-9]{40}$/,'Set exact SPHERES_EXPECTED_REVISION');")
source=source.replace('evidence/S09-visual-review', 'evidence/S09-visual-review-connection-fix')
source=source.replace("const {chromium}=require(path.join(repo,'tools/ui/node_modules/playwright'));", "const {chromium}=require(path.join(repo,'tools/ui/node_modules/playwright'));\nconst {PNG}=require(path.join(repo,'tools/ui/node_modules/playwright-core/lib/utilsBundle.js'));")
source=source.replace("if(selector)await page.locator(selector).scrollIntoViewIfNeeded();const filename", "if(selector)await page.locator(selector).scrollIntoViewIfNeeded();await page.waitForTimeout(350);const filename")
marker=' const dimension=async(label)=>'
assert source.count(marker)==1
helper=r''' const settledCanvas=async(label)=>{
  const canvas=page.locator('[data-equipment-model] canvas');await canvas.waitFor({state:'visible'});await canvas.scrollIntoViewIfNeeded();
  result.canvas_samples ||= {};
  const samples=result.canvas_samples[label]=[];
  for(let attempt=1;attempt<=3;attempt++){
   // Observe the browser's composited pixels after resize/visibility observers
   // have had time to draw. Never request or alter the WebGL context.
   await page.waitForTimeout(attempt===1?350:600);
   const filename=label+'-canvas-'+attempt+'.png',buffer=await canvas.screenshot({timeout:5000});fs.writeFileSync(path.join(out,filename),buffer);
   const png=PNG.sync.read(buffer),colors=new Set();let nonBlack=0,total=0;
   for(let y=Math.floor(png.height*.1);y<Math.ceil(png.height*.9);y+=2)for(let x=Math.floor(png.width*.1);x<Math.ceil(png.width*.9);x+=2){const i=(y*png.width+x)*4,r=png.data[i],g=png.data[i+1],b=png.data[i+2];total++;if(Math.max(r,g,b)>8&&png.data[i+3]>0)nonBlack++;colors.add((r>>3)*1024+(g>>3)*32+(b>>3));}
   const sample={attempt,filename,width:png.width,height:png.height,non_black_fraction:nonBlack/total,quantized_colors:colors.size,status:await page.locator('[data-model-status]').innerText()};sample.nonblank=sample.non_black_fraction>.05&&sample.quantized_colors>16;samples.push(sample);write();
   if(sample.nonblank)return true;
  }
  return false;
 };
'''
source=source.replace(marker,helper+marker)
source=source.replace("stage='designer desktop';", "await page.waitForTimeout(5500);stage='designer desktop';")
source=source.replace("await shot('desktop-model','[data-equipment-model]');", "result.desktop_canvas_nonblank=await settledCanvas('desktop');await shot('desktop-model','[data-equipment-model]');")
source=source.replace("await shot('mobile-model','[data-equipment-model]');", "result.mobile_canvas_nonblank=await settledCanvas('mobile');await shot('mobile-model','[data-equipment-model]');if(!result.mobile_canvas_nonblank){await page.locator('[data-model-reset]').click();result.mobile_after_reset_nonblank=await settledCanvas('mobile-after-reset');await shot('mobile-model-after-reset','[data-equipment-model]');}")
(base/'run-s09-visual-connection-review.cjs').write_text(source,encoding='utf-8')
print(base/'run-s09-visual-connection-review.cjs')
