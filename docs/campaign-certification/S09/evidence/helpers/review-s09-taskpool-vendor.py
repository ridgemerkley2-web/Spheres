import hashlib,json,pathlib,difflib
base=pathlib.Path(__file__).resolve().parent
vendor=base/'integration/spheres-web/vendor/tiny_http'
registry=pathlib.Path(r'C:\Users\ridge\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tiny_http-0.12.0')
manifest=json.loads((vendor/'UPSTREAM.json').read_text())
digest=lambda b:hashlib.sha256(b).hexdigest()
rows=[]
for row in manifest['files']:
    relative=row['path']; original=(registry/relative).read_bytes();copied=(vendor/relative).read_bytes()
    rows.append({'path':relative,'upstream_hash_matches':digest(original)==row['upstream_sha256'],'copy_identical':copied==original})
changed=[r['path'] for r in rows if not r['copy_identical']]
up=(registry/'src/util/task_pool.rs').read_text().replace('\r\n','\n')
patched=(vendor/'src/util/task_pool.rs').read_text().replace('\r\n','\n')
fixture=(base/'evidence/S09-startup-taskpool-regression/pool-6v_p3vas/corrected.rs').read_text().replace('\r\n','\n')
test_marker='#[cfg(test)]'
assert test_marker in patched and test_marker in fixture
review={'kind':'read-only vendored dependency review','candidate':'e5cfd1a0f18e4114073185b6b05352d236462610','upstream_files_checked':len(rows),'rows':rows,'changed_upstream_files':changed,'exact_executed_regression_test':patched.split(test_marker,1)[1]==fixture.split(test_marker,1)[1],'workspace_test_compiles_shipped_source':(base/'integration/spheres-web/tests/http_connections.rs').read_text().find('../vendor/tiny_http/src/util/task_pool.rs')>=0,'runtime_diff':''.join(difflib.unified_diff(up.splitlines(True),patched.split(test_marker,1)[0].rstrip().splitlines(True),fromfile='upstream',tofile='vendored-runtime'))}
review['passed']=all(r['upstream_hash_matches'] for r in rows) and changed==['src/util/task_pool.rs'] and review['exact_executed_regression_test'] and review['workspace_test_compiles_shipped_source']
out=base/'evidence/S09-taskpool-vendor-review.json';out.write_text(json.dumps(review,indent=2)+'\n')
print(json.dumps({k:review[k] for k in ['passed','upstream_files_checked','changed_upstream_files','exact_executed_regression_test','workspace_test_compiles_shipped_source']}));assert review['passed']
