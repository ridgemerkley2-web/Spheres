'use strict';
const fs=require('node:fs'),path=require('node:path'),{verifyEvidence}=require('./worldwide-startup-lib.cjs');
if(require.main===module){const directory=path.resolve(process.argv[2]||'');if(!process.argv[2])throw new Error('Supply the retained run directory');const result=JSON.parse(fs.readFileSync(path.join(directory,'result.json'),'utf8'));console.log(JSON.stringify(verifyEvidence(result,directory),null,2));}
