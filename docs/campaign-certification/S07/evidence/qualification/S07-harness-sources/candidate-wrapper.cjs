// Compile the ignored candidate at its original filename so relative imports,
// repository root, Playwright resolution and embedded-asset checks stay exact.
const fs=require('node:fs'),path=require('node:path'),Module=require('node:module');
const lane=process.argv[2],names={tonga:'ci-construction.cjs',browser:'ci-browser.cjs'};
if(!names[lane])throw Error('Choose tonga or browser');
const repo=path.resolve(__dirname,'../..'),filename=path.join(repo,'tools/ui',names[lane]);
if(lane==='browser'){
  // Same installed-browser fallback as the existing s05-installed-browser.cjs.
  const scoped=Module.createRequire(filename),{chromium}=scoped('playwright');
  const launch=chromium.launch.bind(chromium);
  chromium.launch=options=>launch({...options,channel:process.env.SPHERES_BROWSER_CHANNEL||'chrome'});
}
const candidate=new Module(filename,module);
candidate.filename=filename;
candidate.paths=Module._nodeModulePaths(path.dirname(filename));
candidate._compile(fs.readFileSync(path.join(__dirname,names[lane]),'utf8'),filename);
