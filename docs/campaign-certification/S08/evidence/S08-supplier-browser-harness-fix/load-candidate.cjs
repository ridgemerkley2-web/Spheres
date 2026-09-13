const fs=require('node:fs'),path=require('node:path'),Module=require('node:module');
const original="C:\\Users\\ridge\\Documents\\Codex\\2026-09-05\\pick-up-the-spheres-game-on\\work\\campaign-certification\\integration\\tools\\ui\\ci-supplier-imports.cjs";
const candidate=path.join(__dirname,'ci-supplier-imports.cjs');
const loaded=new Module(original,module);loaded.filename=original;loaded.paths=Module._nodeModulePaths(path.dirname(original));
loaded._compile(fs.readFileSync(candidate,'utf8'),original);
