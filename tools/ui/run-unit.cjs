const fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process');
const root=path.resolve(__dirname,'../..');
const tests=fs.readdirSync(__dirname).filter(n=>/^check_.*\.cjs$/.test(n)&&!n.includes('_browser')).sort().map(n=>path.join(__dirname,n));
const result=cp.spawnSync(process.execPath,['--test',...tests],{cwd:root,stdio:'inherit'});
process.exit(result.status??1);
