"""Prepare an outside-only, exact one-block supplier harness correction."""
from pathlib import Path
import difflib
import hashlib
import json

staging = Path(__file__).resolve().parent
source = staging.parent / 'integration/tools/ui/ci-supplier-imports.cjs'
raw = source.read_bytes()
text = raw.decode('utf-8')
old = """    // Removing this handler leaves unrelated interceptors intact and lets an
    // already running handler finish. Await its fetch/fulfill/dispose outcome.
    try{await page.unroute(pattern,handler);}catch(error){failures.push(error);}
    if(started)try{failures.push(...await wait(completed,'finishing the held preview'));}catch(error){failures.push(error);}
"""
new = """    // Finish this response before removing the interceptor: Playwright may
    // otherwise continue the in-flight route while fulfill is still pending.
    if(started)try{failures.push(...await wait(completed,'finishing the held preview'));}catch(error){failures.push(error);}
    try{await page.unroute(pattern,handler);}catch(error){failures.push(error);}
"""
if text.count(old) != 1:
    old = old.replace('\n', '\r\n')
    new = new.replace('\n', '\r\n')
assert text.count(old) == 1, 'The frozen cleanup block must match exactly (LF or CRLF).'
replacement = text.replace(old, new, 1)
candidate = staging / 'ci-supplier-imports.cleanup-order.cjs'
patch = staging / 'supplier-preview-cleanup-order.patch'
candidate.write_bytes(replacement.encode('utf-8'))
patch.write_bytes(''.join(difflib.unified_diff(text.replace('\r\n', '\n').splitlines(True), replacement.replace('\r\n', '\n').splitlines(True), fromfile='a/tools/ui/ci-supplier-imports.cjs', tofile='b/tools/ui/ci-supplier-imports.cjs')).encode('utf-8'))
sha = lambda data: hashlib.sha256(data).hexdigest()
bindings = {'source': str(source), 'source_sha256': sha(raw), 'candidate': str(candidate), 'candidate_sha256': sha(candidate.read_bytes()), 'patch': str(patch), 'patch_sha256': sha(patch.read_bytes()), 'scope': 'Exactly one finally cleanup block; production and repository files untouched.'}
(staging / 'supplier-preview-cleanup-order-bindings.json').write_text(json.dumps(bindings, indent=2) + '\n', encoding='utf-8')
print(json.dumps(bindings, indent=2))
