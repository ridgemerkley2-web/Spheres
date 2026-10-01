"""Read retained S25 metadata and WER reports; never modify the stopped attempt."""
import datetime
import hashlib
import json
from pathlib import Path
import shutil

DEST = Path(__file__).resolve().parent
OFFLOAD = Path('D:/spheres-offload/codex-next-20260928')
RUN = OFFLOAD / 'full-matrix-local-20260930-01'
LAUNCH = OFFLOAD / 'lazy-digest-validation-20260930'
DEPENDENT = OFFLOAD / 'full-matrix-dependent-verification-20260930-01'
STAMP = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
RAW = OFFLOAD / ('s25-diagnosis-raw-' + STAMP)
result_path = DEST / 'failure-evidence.json'
if result_path.exists():
    raise SystemExit('Evidence already exists; copy recipe to a fresh directory to repeat.')
RAW.mkdir(exist_ok=False)


def pin(path):
    data = path.read_bytes()
    return {'path': str(path), 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest()}


def read(path):
    return json.loads(path.read_text(encoding='utf8'))


paths = [RUN / name for name in ('plan.json', 'freeze.json', 'journal.jsonl')]
paths += [LAUNCH / name for name in ('full-exit.json', 'full-launch.json', 'full.stdout.log', 'full.stderr.log', 'run_full.py')]
paths += [DEPENDENT / name for name in ('result.json', 'events.jsonl', 'dependent_verify.py', 'launch/worker.stderr.log')]
cells = []
for identity in ('japan-7', 'japan-42', 'india-1990', 'india-7'):
    cell = RUN / 'cells' / identity
    paths += [cell / name for name in ('execution.json', 'archive-manifest.json', 'transfer.json', 'stdout.log', 'stderr.log', 'native/result.json', 'native/progress.jsonl')]
    execution = read(cell / 'execution.json')
    report = read(cell / 'native/result.json')
    cells.append({'id': identity, 'exit_code': execution['exit_code'],
                  'finished_utc': execution['finished_utc'], 'elapsed_seconds': execution['elapsed_seconds'],
                  'last_native_date': report['end_native_date'], 'days_each_leg': report['days_each_leg'],
                  'native_report_passed': report['passed'], 'native_report_failure': report['failure']})

wer_root = Path('C:/ProgramData/Microsoft/Windows/WER/ReportArchive')
wer_entries = [
    ('japan-7', 'AppCrash_spheres-web-test_d0efd44c1624d342e21fd13ffdd2534459ddf586_e22ac86e_66d41d06-220f-4007-848c-2b0f66c45e56',
     'C:/ProgramData/Microsoft/Windows/WER/Temp/WER.140990f8-4589-4776-b46f-be0bb41e4526.tmp.mdmp'),
    ('india-7', 'AppCrash_spheres-web-test_ec5a2b88341c760a8de236c4ba5348b7c2f_e22ac86e_6fbc6939-ddbb-41e9-b585-1e2d6d46771d',
     'C:/ProgramData/Microsoft/Windows/WER/Temp/WER.3ee43afc-8725-4ed5-b5c8-d5e4d00b27b5.tmp.mdmp'),
]
wer = []
for identity, directory, temporary_dump in wer_entries:
    report = wer_root / directory / 'Report.wer'
    raw = report.read_bytes()
    text = raw.decode('utf-16') if raw.startswith(b'\xff\xfe') else raw.decode('utf8')
    fields = dict(line.split('=', 1) for line in text.splitlines() if '=' in line)
    target = RAW / (identity + '.Report.wer')
    shutil.copyfile(report, target)
    assert target.read_bytes() == raw
    wanted = ('EventType', 'EventTime', 'IntegratorReportIdentifier', 'AppSessionGuid', 'AppPath',
              'Sig[3].Value', 'Sig[4].Value', 'Sig[6].Value', 'Sig[7].Value')
    wer.append({'id': identity, 'original': pin(report), 'retained_external_copy': pin(target),
                'fields': {key: fields.get(key) for key in wanted},
                'listed_temporary_dump': temporary_dump,
                'listed_temporary_dump_exists': Path(temporary_dump).exists(),
                'archive_directory_files': sorted(p.name for p in report.parent.iterdir() if p.is_file())})

pins_before = [pin(path) for path in paths]
result = {'format': 'spheres-s25-failure-diagnosis/v1', 'checked_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'candidate_revision': '68ba0622ec709b78617aadd1f9198d18f532bb32', 'native_reexecuted': False,
          'old_metadata_pins': pins_before, 'cells': cells, 'wer': wer,
          'launcher': read(LAUNCH / 'full-exit.json'), 'dependent': read(DEPENDENT / 'result.json'),
          'limits': 'Neither access subtype/address nor native stack is present in Report.wer. PermissionError13 has no operation/traceback; no original cause is asserted.',
          'old_metadata_unchanged_during_review': pins_before == [pin(path) for path in paths]}
with result_path.open('x', encoding='utf8') as stream:
    json.dump(result, stream, indent=2)
    stream.write('\n')
print('Retained', len(paths), 'metadata pins and two WER identities; original files unchanged.')
