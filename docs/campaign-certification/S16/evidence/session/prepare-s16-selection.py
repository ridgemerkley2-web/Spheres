"""Build the S16 retention selection; no collection or publication runs here.

Usage: python prepare-s16-selection.py CONFIG.json [REVIEW-LAUNCH.json]
       [--output SELECTION.json] [--extra FILE_OR_DIRECTORY ...] [--overwrite]

Writes only the selected outside-repository JSON file when explicitly run.
Final proofs must already pass. Earlier S16 attempts remain nonqualifying
context. The active review directory contributes only its immutable copied
save and native audit; its stdout/stderr and live campaign files are omitted.
"""
import argparse
import ast
import hashlib
import json
import os
import pathlib
import re

BASE = pathlib.Path(__file__).resolve().parent
REPO = BASE / 'integration'
EVIDENCE = BASE / 'evidence'
DEST = REPO / 'docs/campaign-certification/S16/evidence'
LANES = ('web', 'sim', 'integration', 'node', 'binary', 'fixture')
EXCLUDED_SUFFIXES = {'.exe', '.dll'}


def need(condition, message):
    if not condition:
        raise ValueError(message)


def path(value, parent=BASE):
    value = str(value)
    if os.name == 'nt' and re.match(r'^/mnt/[a-z]/', value):
        value = value[5].upper() + ':/' + value[7:]
    supplied = pathlib.Path(value)
    supplied = supplied if supplied.is_absolute() else parent / supplied
    need(not supplied.is_symlink(), 'Linked evidence is not eligible: ' + str(supplied))
    return supplied.resolve()


def read(file):
    return json.loads(file.read_text(encoding='utf-8-sig'))


def sha(file):
    with file.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def exact(row):
    p = path(row.get('path', row.get('file')))
    need(p.is_file() and sha(p) == row['sha256'], 'Missing or changed referenced evidence: ' + str(p))
    need('bytes' not in row or p.stat().st_size == row['bytes'], 'Changed referenced evidence size: ' + str(p))
    return p


def is_s16(name):
    return re.search(r'(?:^|[-_])s16(?:[-_.]|$)', name, re.I) is not None


def is_review_directory(p):
    return p.is_dir() and re.match(r'^review[-_]s16(?:[-_]|$)', p.name, re.I) is not None


def required_sources(publisher):
    # Read literal source paths without importing/executing the publisher.
    tree = ast.parse(publisher.read_text(encoding='utf-8-sig'), filename=str(publisher))
    strings = {}
    for node in tree.body:
        if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
            if isinstance(node.value, ast.Constant) and isinstance(node.value.value, str):
                strings[node.targets[0].id] = node.value.value
    found = [node.value for node in ast.walk(tree) if isinstance(node, ast.Assign)
             and any(isinstance(t, ast.Name) and t.id == 'sources' for t in node.targets)]
    need(len(found) == 1 and isinstance(found[0], ast.List), 'Publisher source-list schema changed')
    result = []
    for node in found[0].elts:
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            result.append(node.value)
        elif isinstance(node, ast.Name) and node.id in strings:
            result.append(strings[node.id])
        else:
            raise ValueError('Publisher source list is no longer literal')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('config', type=pathlib.Path)
    parser.add_argument('review_launch', type=pathlib.Path, nargs='?')
    parser.add_argument('--output', type=pathlib.Path)
    parser.add_argument('--extra', type=pathlib.Path, action='append', default=[])
    parser.add_argument('--overwrite', action='store_true')
    args = parser.parse_args()
    config_file = path(args.config.resolve())
    cfg = read(config_file)
    pin = cfg['candidate_revision']
    need(isinstance(pin, str) and re.fullmatch('[a-f0-9]{40}', pin), 'Set the actual final runtime revision')
    label = cfg.get('proof_label', 'final1')
    need(isinstance(label, str) and re.fullmatch('[A-Za-z0-9_-]+', label), 'Invalid final proof label')
    output = path(args.output.resolve() if args.output else cfg.get('selection', 'S16-evidence-selection.json'))
    need(output == path(cfg.get('selection', 'S16-evidence-selection.json')), 'Output must match the publication config selection path')
    need(not output.is_relative_to(REPO) and output.parent.is_dir(), 'Selection output must have an existing parent outside the repository')
    need(not output.exists() or args.overwrite, 'Selection already exists; inspect it or explicitly use --overwrite')
    need(output != config_file and output != pathlib.Path(__file__).resolve(), 'Selection would replace an input helper/config')
    selected, excluded, missing_context = {}, {}, set()
    review_file = path(args.review_launch.resolve() if args.review_launch else cfg.get('review_launch', 'S16-review-launch.json'))
    need(review_file == path(cfg.get('review_launch', 'S16-review-launch.json')), 'Review record must match the publication config path')
    review = read(review_file)
    need(review['runtime_revision'] == pin, 'Review is not on the final runtime')
    review_dir = path(review['directory'])
    allowed_review_files = {path(review['save_copy']['copy']), review_dir / 'native-verification/copied-save.json'}
    review_audit = read(review_dir / 'native-verification/copied-save.json')
    allowed_review_files.add(exact(review_audit['canonical']))

    def logical(p):
        if p.is_relative_to(REPO):
            return 'source/' + p.relative_to(REPO).as_posix()
        if p.is_relative_to(EVIDENCE):
            return 'attempts/' + p.relative_to(EVIDENCE).as_posix()
        if p.is_relative_to(BASE):
            return 'session/' + p.relative_to(BASE).as_posix()
        parent_id = hashlib.sha256(str(p.parent).casefold().encode('utf-8')).hexdigest()[:16]
        return 'external/' + parent_id + '/' + p.name

    def add_file(file, required=True):
        p = path(file)
        if p == output:
            need(not required, 'Selection output would replace a required input: ' + str(p))
            return  # Its final entry is added after all inputs are resolved.
        if not p.exists():
            need(not required, 'Missing required source: ' + str(p))
            missing_context.add(str(p))
            return
        need(p.is_file(), 'Expected an evidence file: ' + str(p))
        if p.suffix.lower() in EXCLUDED_SUFFIXES:
            need(not required, 'A required source must not be a packaged binary: ' + str(p))
            excluded[str(p)] = 'compiled binary'
            return
        if p.is_relative_to(DEST):
            raise ValueError('Existing retained output cannot be selected as an input: ' + str(p))
        if p.is_relative_to(review_dir) and p not in allowed_review_files:
            need(not required, 'Mutable review output is not eligible: ' + str(p))
            excluded[str(p)] = 'active review output'
            return
        selected.setdefault(str(p).casefold(), {'source': str(p), 'name': logical(p)})

    def add_tree(directory, required=True):
        p = path(directory)
        if not p.exists():
            need(not required, 'Missing required directory: ' + str(p))
            missing_context.add(str(p))
            return
        need(p.is_dir() and p not in (BASE, REPO, EVIDENCE, DEST), 'Refusing an overly broad or invalid directory input: ' + str(p))
        if p == review_dir or is_review_directory(p):
            excluded[str(p)] = 'live review directory; only explicit immutable review files retained'
            return
        count = 0
        for child in sorted(p.rglob('*')):
            need(not child.is_symlink(), 'Selected evidence tree contains a link: ' + str(child))
            if child.is_file():
                add_file(child, required=False)
                count += 1
        need(not required or count > 0, 'Required evidence directory is empty: ' + str(p))

    def add(file_or_directory, required=True):
        p = path(file_or_directory)
        if p.is_dir():
            add_tree(p, required)
        else:
            add_file(p, required)

    windows = cfg.get('windows', {lane: {'proof': f'evidence/S16-{label}-{lane}.json',
                                        'log': f'evidence/S16-{label}-{lane}.log'} for lane in LANES})
    need(set(windows) == set(LANES), 'All final Windows lanes must be selected')
    for lane in LANES:
        proof_file, log_file = path(windows[lane]['proof']), path(windows[lane]['log'])
        proof = read(proof_file)
        need(proof['passed'] is True and proof['revision'] == pin and proof['lane'] == lane,
             'Final Windows proof is not complete: ' + lane)
        need(sha(log_file) == proof['log_sha256'], 'Final Windows log changed: ' + lane)
        add_file(proof_file)
        add_file(log_file)

    linux_file = path(cfg.get('linux_result', f'evidence/S16-{label}-linux/result.json'))
    linux = read(linux_file)
    need(linux['passed'] is True and linux['revision'] == pin and linux['status'] == 'passed', 'Final Linux qualification is incomplete')
    add_tree(linux_file.parent)
    fixture_file = path(cfg.get('fixture_manifest', f'evidence/S16-fixture-{label}/manifest.json'))
    need(read(fixture_file)['compiled_revision'] in (pin, pin[:12]), 'Final native fixture is not on the runtime pin')
    add_tree(fixture_file.parent)

    browser_proof_file = path(cfg.get('browser_proof', f'evidence/S16-{label}-browser.json'))
    browser_log_file = path(cfg.get('browser_log', f'evidence/S16-{label}-browser.log'))
    proof = read(browser_proof_file)
    need(proof['passed'] is True and proof.get('runtime_revision', proof.get('revision')) == pin,
         'Final browser proof is incomplete or has the wrong runtime')
    need(sha(browser_log_file) == proof['log_sha256'], 'Final browser log changed')
    emitted = []
    for line in browser_log_file.read_text(encoding='utf-8-sig').splitlines():
        try:
            row = json.loads(line)
        except ValueError:
            continue
        if isinstance(row, dict) and row.get('passed') is True and isinstance(row.get('result'), str):
            emitted.append(path(row['result'], REPO))
    need(len(emitted) == 1, 'Expected one final browser result in its complete wrapper log')
    browser_file = emitted[0]
    need('browser_result' not in cfg or browser_file == path(cfg['browser_result']), 'Configured browser result differs')
    if 'result' in proof:
        need(exact(proof['result']) == browser_file, 'Final browser result digest differs')
    browser = read(browser_file)
    need(browser['passed'] is True and browser['errors'] == [], 'Final browser result is incomplete')
    need({exact(r) for r in review['evidence']} == {path(windows['binary']['proof']), browser_file}, 'Review is linked to different final proofs')
    add_file(browser_proof_file)
    add_file(browser_log_file)
    add_file(browser_file)
    add_tree(browser_file.parent)

    # Include every S16 attempt under the established evidence root. This is a
    # bounded top-level name selection, not a repository/workspace-wide search.
    attempts = sorted(p for p in EVIDENCE.iterdir() if is_s16(p.name))
    for p in attempts:
        add(p, required=False)
        if p.is_file() and p.suffix.lower() == '.json' and p.stat().st_size < 4 * 1024 * 1024:
            try:
                row = read(p)
            except (ValueError, UnicodeError):
                continue
            if isinstance(row, dict) and isinstance(row.get('fixture'), str):
                # A failed export may leave a partial fixture at another exact
                # path, or may fail before creating that directory at all.
                add(path(row['fixture']), required=False)

    # Session helpers, preserved runner versions, patches and draft/context
    # records are bounded to S16 names. Live review directories are skipped.
    for p in sorted(BASE.iterdir()):
        if is_s16(p.name):
            add(p, required=False)
    for p in (pathlib.Path(__file__).resolve(), BASE / 'collect-s16-evidence.py', BASE / 'publish-s16-closure.py',
              BASE / 'run-s16-check.py', path(cfg.get('linux_runner', 'run-s16-linux.py')),
              BASE / 'launch-s16-review.py', config_file, review_file):
        add_file(p)
    if 'browser_runner' in cfg:
        add_file(path(cfg['browser_runner']))
    for rel in required_sources(BASE / 'publish-s16-closure.py'):
        add_file(REPO / rel)
    for rel in ('docs/campaign-certification/S10/final/manifest.json',
                'docs/campaign-certification/S11/manifest.json', 'docs/campaign-certification/S15/manifest.json'):
        add_file(REPO / rel)
    add_file(path(cfg.get('preservation', 'evidence/S16-preservation-final.json')))
    add_file(EVIDENCE / 'S08-preservation-before.json')
    add_file(path(review['save_copy']['source']))
    for p in allowed_review_files:
        add_file(p)
    for p in cfg.get('extra_evidence', []) + cfg.get('selection_extra_sources', []) + [x.resolve() for x in args.extra]:
        add(path(p))

    # A flat file-only selection avoids overlapping directory roots and keeps
    # exclusions explicit. Its own final file is a normal retained input.
    selected[str(output).casefold()] = {'source': str(output), 'name': logical(output)}
    items = sorted(selected.values(), key=lambda x: x['name'].casefold())
    raw = (json.dumps(items, indent=2, ensure_ascii=False) + '\n').encode('utf-8')
    logical_names, storage_names = {'inventory.json'}, {'inventory.json'}

    def reserve(name, names):
        key = name.casefold()
        rel = pathlib.PurePosixPath(key)
        need(not rel.is_absolute() and '..' not in rel.parts and '\\' not in key and ':' not in key, 'Unsafe logical name')
        need(key not in names and not any('/'.join(rel.parts[:i]) in names for i in range(1, len(rel.parts)))
             and not any(old.startswith(key + '/') for old in names), 'Duplicate or overlapping destination: ' + name)
        need(not any(key.startswith(old + '.part-') or old.startswith(key + '.part-') for old in names), 'Chunk-name collision: ' + name)
        names.add(key)

    total = 0
    for item in items:
        p = path(item['source'])
        size = len(raw) if p == output else p.stat().st_size
        total += size
        reserve(item['name'], logical_names)
        reserve(item['name'] + ('.gz' if size > 1024 * 1024 else ''), storage_names)
    # Exactly one write, after complete resolution and collision checks. No
    # evidence tree, source file, config, server or repository state is changed.
    with output.open('wb' if args.overwrite else 'xb') as stream:
        stream.write(raw)
    print(json.dumps({'selection': str(output), 'selection_sha256': hashlib.sha256(raw).hexdigest(),
                      'files': len(items), 'source_bytes': total, 'final_runtime_revision': pin,
                      'final_proof_label': label, 'excluded': excluded, 'uncreated_attempt_paths': sorted(missing_context),
                      'collected': False, 'published': False}, indent=2))


if __name__ == '__main__':
    main()
