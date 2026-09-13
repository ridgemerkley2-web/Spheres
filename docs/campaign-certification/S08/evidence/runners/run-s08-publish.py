"""Prepare S08 closeout from collected evidence; --apply writes only reviewed test/docs files.

Run only after qualification, drafts and collection are frozen. Even a dry run
hashes the collected package. This helper neither commits nor pushes Git.
"""
import argparse
import copy
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

BASE = Path(__file__).resolve().parent
PIN = 'd770592aeead51f1313d507edd26b02d75a69bba'
GAME_SHA = 'c5852894069c797c1cbf6b15a302f73f85a0f0b4a4edfee63eb1ca6b367f8435'
ORIGINAL_SHA = 'de8a13611ce1617133710b252253cd82e41d6a4b74d39799f7d29eb2b97ce7f1'
PATCHED_SHA = 'cd9c5ab5783d6ca6bf71aa913842436e851311fa00ce4b6a33881a25f3bdc33a'
PLAN_SHA = '8d2cb23fe996ba9dc35da0aed5859072a83c2e61b55d4881cf047bc1ac195a28'
HARNESS = 'tools/ui/ci-supplier-imports.cjs'
HARNESS_ATTRIBUTES = 'tools/ui/.gitattributes'
MD_PATHWAY = 'docs/CERTIFIED_CAMPAIGN_PATHWAY.md'
JSON_PATHWAY = 'docs/planning/campaign-pathway.json'
S08 = 'docs/campaign-certification/S08'
COMPLETED_DATE = '2026-09-12'  # America/Los_Angeles; qualification UTC is September 13.


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def sha(path):
    with Path(path).open('rb') as handle:
        return hashlib.file_digest(handle, 'sha256').hexdigest()


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))


def encoded(value):
    return (json.dumps(value, indent=2, ensure_ascii=False) + '\n').encode('utf-8')


def git(repo, *args):
    return subprocess.check_output(['git', '-c', 'core.longpaths=true', '-C', str(repo), *args])


def source_path(value):
    path = Path(value)
    return path.resolve() if path.is_absolute() else (BASE / path).resolve()


def checked_child(parent, relative):
    path = (parent / relative).resolve()
    require(path.is_relative_to(parent.resolve()), 'Evidence path escapes package: ' + str(relative))
    return path


def check_scope(repo):
    require(git(repo, 'rev-parse', 'HEAD').decode().strip() == PIN, 'Runtime HEAD changed')
    changed = git(repo, 'diff', '--name-only', '-z', PIN).decode().split('\0')
    changed += git(repo, 'ls-files', '--others', '--exclude-standard', '-z').decode().split('\0')
    allowed = lambda p: p in {HARNESS, HARNESS_ATTRIBUTES, MD_PATHWAY, JSON_PATHWAY} or p.startswith(S08 + '/')
    require(all(not p or allowed(p) for p in changed), 'Unexpected runtime/source changes: ' + repr([p for p in changed if p and not allowed(p)]))


class Collection:
    def __init__(self, path, expected_sha):
        self.path, self.root = path.resolve(), path.resolve().parent
        self.digest = sha(path)
        require(re.fullmatch('[0-9a-f]{64}', expected_sha or '') and self.digest == expected_sha,
                'Explicit collector inventory SHA does not match')
        require((self.root / 'inventory.sha256').read_text(encoding='ascii').strip() == self.digest + '  inventory.json',
                'Collector checksum file differs')
        self.data = read(path)
        require(self.data.get('format') == 'spheres-s08-evidence-inventory' and self.data.get('candidate') == PIN,
                'Wrong evidence inventory/runtime')
        require(self.data.get('status') == 'evidence_collected_only', 'Unexpected collector status')
        self.rows = self.data['source_files']
        require(self.rows and self.data['stored_files'], 'Empty collection')
        stored_names = set()
        for item in self.data['stored_files']:
            require(item['file'] not in stored_names, 'Duplicate stored path')
            stored_names.add(item['file'])
            actual = checked_child(self.root, item['file'])
            require(actual.is_file() and actual.stat().st_size == item['bytes'] and sha(actual) == item['sha256'],
                    'Stored evidence changed: ' + str(actual))
        for digest, item in self.data['compressed_objects'].items():
            require(item.get('reconstruction_verified') is True and item['original_sha256'] == digest and item['parts'],
                    'Unverified compressed evidence')
            for part in item['parts']:
                require(part['file'] in stored_names, 'Missing compressed part')
        attrs = self.root.parent / '.gitattributes'
        require(sha(attrs) == self.data['byte_preservation_attributes']['sha256']
                and b'evidence/** -text' in attrs.read_bytes(), 'Evidence byte preservation rule missing')

    def row(self, original, digest=None, size=None):
        wanted = source_path(original)
        matches = [r for r in self.rows if source_path(r['original_path']) == wanted
                   and (digest is None or r['sha256'] == digest) and (size is None or r['bytes'] == size)]
        require(matches, 'Proof absent from collected source_files: ' + str(original))
        # Actual browser result paths are preferred over duplicate automatic-root copies.
        matches.sort(key=lambda r: (r['storage']['kind'] != 'raw', not r['logical_path'].startswith('browser/'), r['logical_path']))
        return matches[0]

    def mapping(self, row):
        if row['storage']['kind'] == 'raw':
            actual = checked_child(self.root, row['storage']['file'])
            require(actual.is_file() and sha(actual) == row['sha256'], 'Raw proof bytes differ')
            return {'collected_path': 'evidence/' + row['storage']['file'], 'collected_logical_path': row['logical_path']}
        item = self.data['compressed_objects'][row['storage']['object']]
        require(item['original_sha256'] == row['sha256'] and item['original_bytes'] == row['bytes'], 'Compressed proof identity differs')
        return {'collected_path': 'evidence/inventory.json', 'collected_logical_path': row['logical_path'],
                'collected_archives': ['evidence/' + p['file'] for p in item['parts']],
                'restore': 'python evidence/restore-record.py evidence/inventory.json "' + row['logical_path'] + '" NEW_OUTPUT'}

    def verified_json(self, original, digest=None):
        path = source_path(original)
        digest = digest or sha(path)
        row = self.row(path, digest, path.stat().st_size)
        require(sha(path) == row['sha256'], 'Original proof changed since collection')
        self.mapping(row)
        return read(path), row


def map_readme(text, collection):
    # Only explicit evidence/source paths are remapped; prose, model names and hashes remain literal.
    def replace(match):
        value = match.group(1)
        if value == 'manifest-publication-d770.json':
            return '[manifest.json](manifest.json)'
        if not (value.startswith(('evidence/', 's08-staging/', 'integration/artifacts/')) or value == 'S08-review-launch.json'):
            return match.group(0)
        mapped = collection.mapping(collection.row(value))['collected_path']
        return '[' + value.replace('\\', '/') + '](' + mapped + ')'
    return re.sub(r'`([^`\n]+)`', replace, text)


def closeout_readme(text, inventory_sha):
    replacements = {
        '**S08 acceptance evidence passed; publication remains pending. Execution stops before S09.** Runtime qualification uses clean candidate `d770592aeead51f1313d507edd26b02d75a69bba`. The full supplier journey, preservation audit and isolated review launch are complete; evidence collection and the final test/documentation commit remain pending. Paths below are relative to the external `campaign-certification` directory and must be remapped to the verified collection before publication. Proof hashes are recorded in `manifest-publication-d770.json`.':
        '**S08 complete · 12 September 2026. Execution stopped before S09.** Runtime qualification uses clean candidate `d770592aeead51f1313d507edd26b02d75a69bba`. The full supplier journey, preservation audit, isolated review launch and collected evidence passed their recorded checks. Proof links below resolve into the retained collection; hashes and scope are recorded in [manifest.json](manifest.json). The verified [evidence inventory](evidence/inventory.json) has SHA-256 `' + inventory_sha + '`.',
        'The final test/documentation commit must apply these exact executed bytes; its revision is pending.':
        'The accompanying test/documentation change includes these exact executed bytes and preserves them with `ci-supplier-imports.cjs -text`. Its containing commit is resolved after commit creation with `git log -1 --format=%H -- docs/campaign-certification/S08/manifest.json`; the runtime qualification remains on d770592. This document does not invent its own future commit hash.',
        'Remaining publication gates are collector inventory/reconstruction, exact harness plus final documentation commit, and verified Git push or bundle disposition. Update both pathway representations consistently only after publication acceptance. S09 remains Planned, and no later campaign, content or release certificate is granted by S08.':
        'The collector verified archive reconstruction, and publication verified the inventory and every stored evidence file. Both pathway representations mark S08 complete and execution stopped; S09 remains Planned. Git push or bundle disposition is recorded separately after the containing test/documentation commit exists. No transport success or later campaign, content or release certificate is granted by this record.',
        '**http://127.0.0.1:7846/**': '[the isolated S08 review](http://127.0.0.1:7846/)',
    }
    for old, new in replacements.items():
        require(text.count(old) == 1, 'Publication README template changed: ' + old[:90])
        text = text.replace(old, new)
    return text


def update_pathways(repo):
    data = read(repo / JSON_PATHWAY)
    prior = copy.deepcopy(data)
    require(data['status'] == 'in_progress' and data['last_completed_session'] == 'S07' and data['next_session'] == 'S08', 'Unexpected pathway starting state')
    entries = data['sessions']
    s08 = next(s for s in entries if s['id'] == 'S08')
    require(s08['status'] == 'in_progress' and next(s for s in entries if s['id'] == 'S09')['status'] == 'planned', 'S08/S09 status changed')
    s08.update(status='complete', completed_date=COMPLETED_DATE, evidence='../campaign-certification/S08/README.md')
    data.update(date=COMPLETED_DATE, last_completed_session='S08', next_session='S09')
    require(data['execution']['authorized_through'] == 'S08' and data['execution']['stop_boundary'] == 'S08', 'Authorization boundary changed')
    data['execution']['status'] = 'stopped'
    data['execution']['next_session_requires_instruction'] = True
    require(data['gate_decisions'] == prior['gate_decisions'] and set(data['gate_decisions']) == {'G1'}, 'Gate scope changed')
    require([s for s in entries if s['id'] != 'S08'] == [s for s in prior['sessions'] if s['id'] != 'S08'], 'Another session changed')
    text = (repo / MD_PATHWAY).read_text(encoding='utf-8-sig')
    replacements = {
        'S01–S07 complete; S08 in progress.': 'S01–S08 complete; execution stopped before S09.',
        'S01–S07 are complete; S08 is in progress; S09–S30 remain planned. Execution will stop after S08.': 'S01–S08 are complete; S09–S30 remain planned. Execution stopped after S08.',
        'A subsequent “Next” authorized S08, which is in progress; execution will stop before S09.': 'A subsequent “Next” authorized S08, which completed on 12 September 2026 on its recorded runtime and evidence; execution stopped before S09.',
    }
    for old, new in replacements.items():
        require(text.count(old) == 1, 'Pathway prose no longer matches: ' + old)
        text = text.replace(old, new)
    start, end = text.index('<a id="s08"></a>'), text.index('<a id="s09"></a>')
    section = text[start:end]
    require(section.count('**Status:** In progress') == 1 and section.count('- [ ]') == 3, 'Unexpected S08 marker/checklist')
    section = section.replace('**Status:** In progress', '**Status:** Complete · **Completed:** 12 September 2026').replace('- [ ]', '- [x]')
    section = section.rstrip() + '\n\nEvidence: [S08 qualification and retained limitations](campaign-certification/S08/README.md).\n\n'
    text = text[:start] + section + text[end:]
    require('#### S09' in text and '**Status:** Planned' in text[text.index('<a id="s09"></a>'):text.index('<a id="s10"></a>')], 'S09 changed')
    return text.encode('utf-8'), encoded(data)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo', type=Path, default=BASE / 'integration')
    parser.add_argument('--readme', type=Path, default=BASE / 's08-staging/README-publication-d770.md')
    parser.add_argument('--manifest', type=Path, default=BASE / 's08-staging/manifest-publication-d770.json')
    parser.add_argument('--inventory-sha256', required=True)
    parser.add_argument('--outer', type=Path, default=BASE / 'evidence/S08-supplier-browser-corrected-route-pool-2/runner-result.json')
    parser.add_argument('--preservation', type=Path, default=BASE / 'evidence/S08-preservation-after.json')
    parser.add_argument('--launch', type=Path, default=BASE / 'S08-review-launch.json')
    parser.add_argument('--report', type=Path, default=BASE / 'S08-publication-result.json')
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    repo = args.repo.resolve()
    check_scope(repo)
    require(not args.report.exists(), 'Publication report already exists')
    collection = Collection(repo / S08 / 'evidence/inventory.json', args.inventory_sha256)
    draft, _ = collection.verified_json(args.manifest)
    collection.row(args.readme, sha(args.readme), args.readme.stat().st_size)
    require(draft['runtime_candidate'] == PIN and draft['session'] == 'S08', 'Wrong publication draft')
    require(draft['corrected_supplier_browser'].get('passed') is True, 'Final draft has not accepted corrected supplier browser')
    require(draft['proofs']['performance_plan']['sha256'] == PLAN_SHA, 'Original acceptance plan changed')
    for proof in draft['proofs'].values():
        row = collection.row(proof['original_path'], proof['sha256'], proof['bytes'])
        proof.update(collection.mapping(row))
    outer, outer_row = collection.verified_json(args.outer)
    require(outer.get('passed') is True and outer.get('exit_code') == 0 and outer.get('integrity_passed') is True
            and outer.get('candidate') == PIN, 'Final corrected wrapper did not pass')
    for phase in ['source_before', 'source_after']:
        require(outer[phase]['revision'] == PIN and outer[phase]['status_porcelain'] == '', 'Browser source not clean/pinned')
    overlay = outer['harness_overlay']
    require(overlay['original_checkout_sha256'] == ORIGINAL_SHA and overlay['patched_sha256'] == PATCHED_SHA
            and overlay['executed_sha256'] == PATCHED_SHA, 'Browser did not execute exact reviewed correction')
    require(outer['binaries']['game']['sha256'] == GAME_SHA, 'Browser game binary changed')
    require(outer['overlay_execution']['record']['evaluated_sha256'] == PATCHED_SHA, 'Overlay execution identity differs')
    inner, inner_row = collection.verified_json(outer['inner_result']['path'], outer['inner_result']['sha256'])
    require(inner.get('passed') is True and inner['build']['revision'] == PIN and inner['build']['binary_sha256'] == GAME_SHA,
            'Actual full browser proof did not pass on qualified runtime')
    require(inner.get('errors') == [] and inner.get('maintenance', {}).get('receipt'), 'Final browser maintenance/errors proof missing')
    executed = source_path(overlay['executed_snapshot'])
    collection.row(executed, PATCHED_SHA, executed.stat().st_size)
    require(sha(executed) == PATCHED_SHA and sha(repo / HARNESS) == ORIGINAL_SHA, 'Exact harness bytes changed before publication')
    preservation, preservation_row = collection.verified_json(args.preservation)
    require(preservation.get('passed') is True and len(preservation['files']) == 8 and len(preservation['worktrees']) == 2
            and all(r.get('unchanged') is True for r in preservation['files'] + preservation['worktrees']), 'Protected originals not preserved')
    launch, launch_row = collection.verified_json(args.launch)
    require(launch['runtime_revision'] == PIN and launch['executable_sha256'] == GAME_SHA and launch['build']['revision'] == PIN[:12]
            and launch['player'] == 'Tonga' and len(launch['copies']) == 5 and re.fullmatch(r'http://127\.0\.0\.1:\d+', launch['url']), 'Review launch identity/scope differs')
    for item in launch['qualification']:
        collection.row(item['path'], item['sha256'])
    for item in launch['copies']:
        collection.row(item['source'], item['sha256'])
    require(any(source_path(p['path']) == source_path(outer['inner_result']['path']) for p in launch['qualification']), 'Review not bound to final corrected browser')
    readme = map_readme(closeout_readme(args.readme.read_text(encoding='utf-8-sig'), collection.digest), collection)
    require('Complete supplier browser acceptance remains pending' not in readme and 'full browser acceptance is pending' not in readme,
            'README still claims supplier acceptance pending; finalize draft before publication')
    draft.update(format='spheres-s08-qualification', status='complete', publication_status='collected_and_verified',
                 completed_date=COMPLETED_DATE, source_path_base='collected_path is relative to this manifest; original_path remains external provenance')
    draft['final_test_documentation_commit'] = {
        'resolution': 'containing_git_commit',
        'command': 'git log -1 --format=%H -- docs/campaign-certification/S08/manifest.json',
        'scope': 'Resolve after the reviewed test/docs commit is created; runtime qualification remains on ' + PIN,
    }
    draft['evidence_inventory'] = {'path': 'evidence/inventory.json', 'sha256': collection.digest,
        'stored_files_verified': len(collection.data['stored_files']), 'reconstruction_verified_by_collector': True}
    gates = draft['remaining_publication_gates']
    gates['corrected_supplier_browser'] = {'status': 'passed', **collection.mapping(outer_row), 'inner': collection.mapping(inner_row)}
    gates['collector_inventory'] = {'status': 'passed', **draft['evidence_inventory']}
    gates['final_preservation'].update(status='passed', **collection.mapping(preservation_row), eight_protected_saves=True, protected_source_heads=2)
    gates['isolated_review_launch'].update(status='passed_at_recorded_launch', **collection.mapping(launch_row), url=launch['url'], runtime_revision=PIN, binary_sha256=GAME_SHA)
    gates['final_test_docs_commit'] = {'status': 'resolve_containing_git_commit', 'exact_executed_patch_verified': True}
    gates['git_disposition'] = {'status': 'external_transport_record_after_commit', 'scope': 'No push or bundle success is claimed by this manifest.'}
    gates['pathways'] = {'status': 'updated', 'last_completed_session': 'S08', 'next_session': 'S09', 'execution': 'stopped'}
    md, pathway = update_pathways(repo)
    attributes_path = repo / HARNESS_ATTRIBUTES
    attributes = attributes_path.read_bytes() if attributes_path.exists() else b''
    require(not any(b'ci-supplier-imports.cjs' in line and line.strip() != b'ci-supplier-imports.cjs -text'
                    for line in attributes.splitlines()), 'Conflicting harness attributes require review')
    if b'ci-supplier-imports.cjs -text' not in attributes.splitlines():
        attributes += (b'\n' if attributes and not attributes.endswith(b'\n') else b'') + b'ci-supplier-imports.cjs -text\n'
    outputs = {repo / HARNESS: executed.read_bytes(), repo / S08 / 'README.md': readme.encode('utf-8'),
               repo / S08 / 'manifest.json': encoded(draft), repo / MD_PATHWAY: md, repo / JSON_PATHWAY: pathway,
               attributes_path: attributes}
    check_scope(repo)
    record = {'candidate': PIN, 'mode': 'apply' if args.apply else 'verified_dry_run', 'inventory_sha256': collection.digest,
              'prepared_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
              'outputs': [{'path': p.relative_to(repo).as_posix(), 'bytes': len(b), 'sha256': hashlib.sha256(b).hexdigest()} for p, b in outputs.items()],
              'committed': False, 'pushed': False}
    if args.apply:
        # All verification and content preparation precede repository writes.
        for path, content in outputs.items():
            path.parent.mkdir(parents=True, exist_ok=True)
            with tempfile.NamedTemporaryFile(dir=path.parent, prefix='.s08-publish-', delete=False) as handle:
                temporary = Path(handle.name)
                handle.write(content)
            os.replace(temporary, path)
        check_scope(repo)
        require(sha(repo / HARNESS) == PATCHED_SHA, 'Published harness differs from executed bytes')
        for item in record['outputs']:
            require(sha(repo / item['path']) == item['sha256'], 'Published output differs')
        with args.report.open('xb') as handle:
            handle.write(encoded(record))
    print(json.dumps(record, indent=2))


if __name__ == '__main__':
    main()
