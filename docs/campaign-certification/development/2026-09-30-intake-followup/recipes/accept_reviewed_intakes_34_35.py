import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
OUT = Path(r'D:\spheres-offload\codex-next-20260928\combined-intakes-34-37-validation')
SPECS = {
    '34': ('brazil', '255a66c0317fca81fcbac1461b986aa2b029162d',
        '75 of 76 submitted original responses match exactly. The remaining registry response is accepted only as a separately identified current response supporting the two directly reviewed claims. The original response pin, missing-body limitation and unknown cause of change remain explicit. All 127 claims and 12 discrete holder observations were reviewed. Earlier SOURCE-17 repairs remain intact.'),
    '35': ('ussr', '6582e01ca1a0fc8a497e15cab186ee53eeb7f8e5',
        'All 16 recorded responses match exactly. Acceptance covers 36 claims and eight dated holder observations attributed to the reproduced facsimiles. Private and nonprofit hosting does not establish government-host or paper-original authentication. The SOURCE-26 repair remains intact. No inferred office boundaries, sole-leader claims or game-party mapping are accepted.'),
    '36': ('tonga', '7caba83835ab5385a9e9b581f89920a5d717328b',
        'All 39 recorded original responses match exactly. Acceptance covers 53 reviewed claims and ten holder observations, after removing seven unsupported explanations for acting service. Existing dated appointment, oath, resignation and death distinctions remain intact, together with Speakers C01-24, C04 preparation and all 168 earlier sources. Tongan-language interpretations are explicitly bounded rather than certified translations.'),
    '37': ('france', '73d748ef541d4d1970a07b9050115cf52bb2ecee',
        'All 31 original responses and two decoded bodies match exactly. All 48 claims remain dated source events. Sixteen named observations of ten prime ministers are retained; twelve inferred effective starts and fourteen inferred effective ends were removed under the existing evidence contract. The accepted scope covers this first ten-person batch through March 2014, with later prime ministers and effective-term research still open. The unchanged importer generated France from the supplement; all accepted C01-23 presidential records and 635 organization observations remain intact.'),
}

def pin(path, relative):
    raw = path.read_bytes()
    return {'path': relative, 'bytes': len(raw), 'sha256': hashlib.sha256(raw).hexdigest()}

results = []
selected = sys.argv[1:] or ['34', '35']
for number in selected:
    country, revision, scope = SPECS[number]
    folder = ROOT / f'docs/campaign-certification/C01/integrations/CLAUDE-C01-{number}'
    manifest_path = folder / 'manifest.json'
    manifest_raw = manifest_path.read_bytes()
    manifest = json.loads(manifest_raw)
    key = 'files' if 'files' in manifest else 'evidence'
    for row in manifest[key]:
        assert pin(folder / row['path'], row['path']) == row, row['path']
    relative = f'docs/campaign-certification/C01/research/{country}.json'
    reviewed = json.loads(subprocess.check_output(['git', 'show', f'{revision}:{relative}'], cwd=ROOT))
    assert json.loads((ROOT / relative).read_bytes()) == reviewed, relative
    for name, raw in [('original-review-README.md', (folder / 'README.md').read_bytes()),
                      ('original-review-manifest.json', manifest_raw)]:
        with (folder / name).open('xb') as f:
            f.write(raw)
    text = f'''# Integration acceptance — CLAUDE-C01-{number}

**Decision: accepted as bounded research intake.**

The coordinating Codex review accepts the exact corrected country data reviewed
at `{revision}`. The integrated JSON was compared in full with that reviewed
revision. The original independent review and manifest are preserved verbatim
as [original-review-README.md](original-review-README.md) and
[original-review-manifest.json](original-review-manifest.json).

{scope}

The independent review's original tests, failures, retrieval attempts and
limitations remain unchanged. Combined-branch checks are recorded separately.
The wrapper manifest pins both original records and every current receipt file.
No runtime character installation, portrait rights, complete chronology,
country certification or parent C01/C06/S23 acceptance follows.
'''
    (folder / 'README.md').write_text(text, encoding='utf-8', newline='\n')
    manifest['integration_decision'] = 'accepted_bounded_research_intake'
    manifest['original_review_manifest'] = 'original-review-manifest.json'
    manifest[key] = [pin(p, p.relative_to(folder).as_posix()) for p in sorted(folder.rglob('*'))
                     if p.is_file() and p != manifest_path]
    manifest_path.write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')
    results.append({'packet': number, 'country_matches_reviewed_revision': revision,
                    'original_payloads_verified': len(json.loads(manifest_raw)[key]),
                    'current_payloads': len(manifest[key]), 'scope': scope})
(OUT / ('root-acceptance-' + '-'.join(selected) + '.json')).write_text(json.dumps(results, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
print(json.dumps(results, indent=2, ensure_ascii=False))
