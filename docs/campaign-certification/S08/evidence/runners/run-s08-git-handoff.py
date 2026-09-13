"""Push the completed S08 commit normally; preserve a verified bundle on failure."""
import datetime, hashlib, json, os, pathlib, subprocess

base = pathlib.Path(__file__).resolve().parent
repo = base / 'integration'
env = dict(os.environ, GIT_TERMINAL_PROMPT='0')
prefix = ['git', '-c', 'core.longpaths=true', '-c', 'credential.interactive=never', '-C', str(repo)]

def git(*args):
    p = subprocess.run(prefix + list(args), env=env, capture_output=True, text=True, encoding='utf-8', errors='replace')
    return {'command': list(args), 'exit_code': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}

status = git('status', '--porcelain')
assert status['exit_code'] == 0 and not status['stdout'].strip(), 'Commit all reviewed S08 publication files first'
branch = git('branch', '--show-current')
assert branch['exit_code'] == 0 and branch['stdout'].strip() == 'codex/campaign-certification'
head_result = git('rev-parse', 'HEAD')
assert head_result['exit_code'] == 0
head = head_result['stdout'].strip()
manifest = json.loads((repo / 'docs/campaign-certification/S08/manifest.json').read_text(encoding='utf-8'))
assert manifest['status'] == 'complete', 'S08 must be published before transport'
stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
output = base / f'S08-git-disposition-{stamp}.json'
assert not output.exists()
record = {'head': head, 'branch': branch['stdout'].strip(), 'started_utc': stamp, 'remote_verified': False, 'bundle': None}
record['push'] = git('push', '--porcelain', 'origin', 'HEAD:refs/heads/codex/campaign-certification')
if record['push']['exit_code'] == 0:
    record['remote_check'] = git('ls-remote', '--heads', 'origin', 'refs/heads/codex/campaign-certification')
    record['remote_verified'] = record['remote_check']['exit_code'] == 0 and record['remote_check']['stdout'].split() == [head, 'refs/heads/codex/campaign-certification']
if not record['remote_verified']:
    bundle = base / f'S08-campaign-certification-{head[:12]}-{stamp}.bundle'
    assert not bundle.exists()
    record['bundle_create'] = git('bundle', 'create', str(bundle), 'refs/heads/codex/campaign-certification')
    if record['bundle_create']['exit_code'] == 0:
        record['bundle_verify'] = git('bundle', 'verify', str(bundle))
        record['bundle_heads'] = git('bundle', 'list-heads', str(bundle))
        verified = record['bundle_verify']['exit_code'] == 0 and record['bundle_heads']['exit_code'] == 0 and record['bundle_heads']['stdout'].split() == [head, 'refs/heads/codex/campaign-certification']
        with bundle.open('rb') as stream:
            digest = hashlib.file_digest(stream, 'sha256').hexdigest()
        record['bundle'] = {'path': str(bundle), 'bytes': bundle.stat().st_size, 'sha256': digest, 'verified': verified}
record['finished_utc'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
record['head_after'] = git('rev-parse', 'HEAD')['stdout'].strip()
record['clean_after'] = git('status', '--porcelain')['stdout'].strip() == ''
record['local_handoff_verified'] = record['head_after'] == head and record['clean_after'] and (record['remote_verified'] or bool(record['bundle'] and record['bundle']['verified']))
with output.open('x', encoding='utf-8') as stream:
    json.dump(record, stream, indent=2); stream.write('\n')
print(json.dumps({'record': str(output), **record}, indent=2), flush=True)
raise SystemExit(0 if record['local_handoff_verified'] else 1)
