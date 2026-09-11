// Run the unchanged committed CI assertions using Playwright's supported
// installed-Chrome channel when its downloaded headless shell is unavailable.
const path=require('node:path'),{createRequire}=require('node:module');
const ci=path.join(__dirname,'integration/tools/ui/ci-browser.cjs');
const scoped=createRequire(ci),{chromium}=scoped('playwright');
const launch=chromium.launch.bind(chromium);
chromium.launch=options=>launch({...options,channel:'chrome'});
require(ci);
