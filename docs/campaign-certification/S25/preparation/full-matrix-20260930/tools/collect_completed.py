"""Collect all actually downloaded cells once; never invent missing evidence."""
import concurrent.futures,json,pathlib,subprocess,sys
BASE=pathlib.Path(__file__).resolve().parent
META=BASE/'final-collection-20260930-01'
rows=json.loads((META/'download-results.json').read_text())['cells']
LOGS=META/'collection-logs';LOGS.mkdir(exist_ok=False)
def work(row):
    cell=row['cell']
    if not row.get('artifact_verified'): return {'cell':cell,'collected':False,'reason':'No verified artifact available'}
    previous=BASE/'cell-receipts'/cell/'collection.json'
    if previous.exists():
        r=json.loads(previous.read_text());assert r['extraction_complete'] and r['source_zip_unchanged']
        return {'cell':cell,'collected':True,'previous_receipt':str(previous),'exit_code':r['exit_code']}
    args=[sys.executable,'-B','-X','utf8',str(BASE/'collect_cell.py'),'collect','--cell',cell,'--artifact-id',str(row['artifact_id']),'--job-id',str(row['job_id']),'--artifact-metadata',str(META/'metadata'/(cell+'.artifact.json')),'--job-metadata',str(META/'metadata'/(cell+'.job.json')),'--run-metadata',str(META/'run.json'),'--zip',str(META/'downloads'/(cell+'.zip'))]
    with (LOGS/(cell+'.log')).open('xb') as f: r=subprocess.run(args,stdout=f,stderr=subprocess.STDOUT)
    result={'cell':cell,'collected':True,'exit_code':r.returncode};print(json.dumps(result),flush=True);return result
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:results=list(pool.map(work,rows))
with (META/'collection-results.json').open('x') as f:json.dump({'qualification':False,'full_matrix_passed':False,'cells':results},f,indent=2)
