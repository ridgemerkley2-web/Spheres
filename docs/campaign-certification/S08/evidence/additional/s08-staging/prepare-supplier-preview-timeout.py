import difflib
from pathlib import Path

base = Path(__file__).resolve().parent.parent
source = base / 'integration/tools/ui/ci-supplier-imports.cjs'
before = source.read_text(encoding='utf-8')
anchor = "async function currentQuote(page){"
helper = '''// Hold one actual supplier response across a visible day. Manual promises need
// their own limits: Playwright's locator timeout does not cover awaiting them.
async function withHeldSupplierPreview(page,trigger,whileHeld){
  const pattern='**/api/equipment-preview',timeoutMs=30000;
  let release,intercepted,handled,started=false;
  const seen=new Promise(resolve=>intercepted=resolve),hold=new Promise(resolve=>release=resolve),completed=new Promise(resolve=>handled=resolve);
  const wait=async(promise,label)=>{let timer;try{return await Promise.race([promise,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('Supplier stale-review timeout after '+timeoutMs+' ms: '+label)),timeoutMs);})]);}finally{clearTimeout(timer);}};
  const handler=async route=>{
    if(started)return route.continue();
    let kind;try{kind=route.request().postDataJSON()?.command?.kind;}
    catch(error){started=true;intercepted({ok:false,error});handled([error]);return;}
    if(kind!=='company_import_purchase')return route.continue();
    started=true;let response;const failures=[];
    try{response=await route.fetch({timeout:timeoutMs});intercepted({ok:true});await hold;await route.fulfill({response});}
    catch(error){failures.push(error);intercepted({ok:false,error});}
    finally{if(response)try{await response.dispose();}catch(error){failures.push(error);}handled(failures);}
  };
  const failures=[];let result;
  try{
    await page.route(pattern,handler);await trigger();
    const arrival=await wait(seen,'waiting for the fetched preview');if(!arrival.ok)throw arrival.error;
    result=await whileHeld();
  }catch(error){failures.push(error);}
  finally{
    release();
    // Removing this handler leaves unrelated interceptors intact and lets an
    // already running handler finish. Await its fetch/fulfill/dispose outcome.
    try{await page.unroute(pattern,handler);}catch(error){failures.push(error);}
    if(started)try{failures.push(...await wait(completed,'finishing the held preview'));}catch(error){failures.push(error);}
  }
  const errors=[...new Set(failures)];
  if(errors.length)throw errors.length===1?errors[0]:new AggregateError(errors,'Supplier stale-review request and cleanup failed: '+errors.map(error=>String(error).slice(0,2000)).join(' | '));
  return result;
}
'''
old = '''    let release,intercepted,handled;const seen=new Promise(r=>intercepted=r),heldResponse=new Promise(r=>release=r),completedResponse=new Promise(r=>handled=r);
    await page.route('**/api/equipment-preview',async route=>{
      if(route.request().postDataJSON()?.command?.kind!=='company_import_purchase')return route.continue();
      const response=await route.fetch();try{intercepted();await heldResponse;await route.fulfill({response});}finally{await response.dispose();handled();}
    });
    const current=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data))),oi=current.companies.market.offers.findIndex(o=>o.id===offer.id),ai=current.companies.market.offers[oi].actions.findIndex(a=>a.command?.kind==='company_import_purchase');
    await page.locator('[data-equipment-action='+q('companies.market.offers.'+oi+'.actions.'+ai)+']').click();await seen;
    const oldDate=(await state(page,url)).date;await closePanels(page);e.advances.push(await day(page,url));release();await completedResponse;await page.unroute('**/api/equipment-preview');
'''
new = '''    const oldDate=await withHeldSupplierPreview(page,async()=>{
      const current=await page.evaluate(()=>JSON.parse(JSON.stringify(EQUIP.data))),oi=current.companies.market.offers.findIndex(o=>o.id===offer.id),ai=current.companies.market.offers[oi].actions.findIndex(a=>a.command?.kind==='company_import_purchase');
      await page.locator('[data-equipment-action='+q('companies.market.offers.'+oi+'.actions.'+ai)+']').click();
    },async()=>{
      const date=(await state(page,url)).date;await closePanels(page);e.advances.push(await day(page,url));return date;
    });
'''
assert before.count(anchor) == 1
assert before.count(old) == 1
after = before.replace(anchor, helper + anchor).replace(old, new)
patch = ''.join(difflib.unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True), fromfile='a/tools/ui/ci-supplier-imports.cjs', tofile='b/tools/ui/ci-supplier-imports.cjs'))
(base / 's08-staging/supplier-preview-timeout.patch').write_text(patch, encoding='utf-8', newline='\n')
print('Wrote only the outside-repo supplier-preview-timeout.patch; repository source unchanged.')
