import subprocess
from pathlib import Path

root = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
revision = 'c98d1d5668870e184c320a3e03f477bd7f89a12c'
args = ['git', 'diff', '--binary', revision + '^', revision, '--', '.',
        ':(exclude)docs/campaign-certification/C01/research-index.json']
patch = subprocess.check_output(args, cwd=root)
out = Path(r'D:\spheres-offload\codex-next-20260928\combined-intakes-34-37-validation')
with (out / 'japan-c98-scoped.patch').open('xb') as f:
    f.write(patch)
assert patch.count(b'diff --git ') == 4
assert b'diff --git a/docs/campaign-certification/C01/research-index.json' not in patch
subprocess.run(['git', 'apply', '--index', '--3way'], input=patch, cwd=root, check=True)
subprocess.run(['git', 'commit', '-m', 'Apply retained Japan SDP wording correction without stale aggregate index'], cwd=root, check=True)
