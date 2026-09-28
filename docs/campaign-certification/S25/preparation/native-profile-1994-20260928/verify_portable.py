#!/usr/bin/env python3
"""Read-only verifier for this portable retained diagnostic; never runs the game."""
import argparse
import csv
from datetime import date, timedelta
import gzip
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import re
import sys

REVISION = 'ae8084e853a8eb01ef4d834017af3e94c8e2cc82'
INPUT_SHA = '5480ccc296c2fe67ef2234871d1ef5fc9f098184ca1c83d885b07010a1368b20'
INPUT_BYTES = 135739703
BINARY_SHA = 'e90b8f0cb34603f5290c8a19b106a0d6dc31e3ea7ad293eb0135199fe1fd59e2'
BINARY_BYTES = 353133206


def require(condition, message):
    if not condition:
        raise ValueError(message)


def hash_file(path):
    digest = hashlib.sha256()
    count = 0
    with path.open('rb') as handle:
        while chunk := handle.read(1024 * 1024):
            count += len(chunk)
            digest.update(chunk)
    return count, digest.hexdigest()


def no_duplicate_keys(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, 'Duplicate JSON field: ' + key)
        result[key] = value
    return result


def read_json(path):
    return json.loads(path.read_text(encoding='utf-8-sig'), object_pairs_hook=no_duplicate_keys)


def local(root, relative):
    require(isinstance(relative, str) and '\\' not in relative, 'Nonportable payload path')
    path = PurePosixPath(relative)
    require(not path.is_absolute() and path.parts and all(x not in ('', '.', '..') for x in path.parts)
            and path.as_posix() == relative and ':' not in relative, 'Unsafe payload path: ' + relative)
    target = root.joinpath(*path.parts)
    for ancestor in (target, *target.parents):
        if ancestor == root:
            break
        require(not ancestor.is_symlink() and not getattr(ancestor, 'is_junction', lambda: False)(),
                'Linked payload path: ' + relative)
    require(target.is_file() and target.resolve() == target and target.resolve().is_relative_to(root),
            'Missing/linked/outside payload: ' + relative)
    return target


def verify(root, expected_manifest):
    root = root.resolve(strict=True)
    require(re.fullmatch('[0-9a-f]{64}', expected_manifest) is not None, 'Expected manifest SHA-256 required')
    require(hash_file(root/'manifest.json')[1] == expected_manifest, 'Manifest does not match supplied external SHA-256')
    require((root/'manifest.sha256').read_text(encoding='ascii') == expected_manifest+'  manifest.json\n', 'Detached manifest hash differs')
    manifest = read_json(root/'manifest.json')
    require(manifest['format'] == 'spheres-native-diagnostic-portable/v1', 'Wrong packet format')
    require(manifest['qualification'] is False and manifest['source_revision'] == REVISION, 'Wrong scope/source')
    payloads = manifest['payloads']
    pins = {}
    for row in payloads:
        relative = row['path']
        require(relative not in pins and relative not in ('manifest.json','manifest.sha256'), 'Duplicate/self payload')
        target = local(root, relative)
        require(hash_file(target) == (row['bytes'],row['sha256']), 'Payload differs: ' + relative)
        pins[relative] = row
    actual = {p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file()}
    require(actual == set(pins) | {'manifest.json','manifest.sha256'}, 'Unlisted or missing local payload')
    require(manifest['payload_count'] == len(pins), 'Payload count differs')
    required = {'README.md','verify_portable.py','build-record.json','profile-1994.ps1',
                'review/README.md','review/analysis.json','review/manifest.json','review/review-attempt-01.txt'}
    required |= {'original-run/'+name for name in ['execution-start.json','execution.json','profile.json','process.csv','stdout.log','stderr.log']}
    require(required <= set(pins), 'Required evidence missing')

    input_record = manifest['input']
    compressed_path = local(root, input_record['path'])
    require(input_record['path'] in pins and input_record['decoded_sha256'] == INPUT_SHA
            and input_record['decoded_bytes'] == INPUT_BYTES, 'Wrong whole-input pin')
    compressed = compressed_path.read_bytes()[:10]
    require(compressed[:3] == b'\x1f\x8b\x08' and compressed[3] == 0 and compressed[4:8] == b'\0\0\0\0',
            'Gzip must have zero mtime and no filename/optional header')
    digest = hashlib.sha256()
    count = 0
    with gzip.open(compressed_path, 'rb') as handle:
        while chunk := handle.read(1024 * 1024):
            count += len(chunk)
            require(count <= INPUT_BYTES, 'Decoded input too large')
            digest.update(chunk)
    require((count,digest.hexdigest()) == (INPUT_BYTES,INPUT_SHA), 'Whole decoded input differs')
    # Interpret only the pinned input's ordinary JSON fields; no native replay.
    with gzip.open(compressed_path, 'rt', encoding='utf-8') as handle:
        campaign = json.load(handle)
    require(campaign['format'] == 'spheres-campaign' and campaign['version'] == 1 and campaign['player'] == 'France', 'Wrong campaign envelope')
    integrated = campaign['world']
    require(integrated['format'] == 'spheres-integrated-save', 'Wrong integrated world envelope')
    world = integrated['world']
    # Frozen world.rs uses serde first_day()=1 when this field is omitted.
    require((world['year'],world['month'],world.get('day',1)) == (1994,1,1), 'Input date differs')
    require(world['player'] == 'France' and world['rules']['seed'] == 1990, 'Input player/seed differs')
    require(len(campaign['history']) == 1110 and len(campaign['log']) == 1272, 'Input retained history/log differs')
    del world, integrated, campaign
    require(hash_file(compressed_path) == (pins[input_record['path']]['bytes'],pins[input_record['path']]['sha256']),
            'Compressed input changed during verification')

    binary = manifest['binary_reference']
    require(binary['included'] is False and binary['bytes'] == BINARY_BYTES and binary['sha256'] == BINARY_SHA, 'Wrong binary reference')
    execution = read_json(root/'original-run/execution.json')
    initial = read_json(root/'original-run/execution-start.json')
    require(all(execution.get(k) == v for k,v in initial.items()), 'Initial/final execution receipt mismatch')
    require(execution['source_revision'] == REVISION and execution['qualification'] is False, 'Execution source/scope differs')
    require(execution['exit_code'] == 0 and execution['timed_out'] is False and execution['inputs_unchanged'] is True, 'Execution did not pass')
    require(execution['args'] == ['performance::s22_daily_subsystem_diagnosis','--ignored','--exact','--nocapture','--test-threads=1'], 'Wrong test invocation')
    require(execution['input_bytes'] == INPUT_BYTES, 'Execution input length differs')
    for name in ['input_sha256_before','original_input_sha256_after','copied_input_sha256_after']:
        require(execution[name] == INPUT_SHA, 'Execution input pin differs: ' + name)
    require(execution['binary_sha256_before'] == execution['binary_sha256_after'] == BINARY_SHA, 'Execution binary pins differ')
    environment = execution['environment']
    require(environment['SPHERES_S22_DIAGNOSTIC_EXPECT_DATE'] == '1994-01-01'
            and environment['SPHERES_S22_RENEW_BUDGET'] == environment['SPHERES_S22_REQUIRE_CERTIFIED'] == '1'
            and environment['SPHERES_S22_ADOPT_COMPETITION'] is None, 'Diagnostic environment differs')
    profile = read_json(root/'original-run/profile.json')
    require(profile['revision'] == REVISION[:12] and profile['passed'] is True and profile['status'] == 'complete', 'Profile not passed')
    require(profile['input_unchanged'] is True and profile['requested_days'] == len(profile['rows']) == 31, 'Profile completion differs')
    require(profile['input_fingerprint']['bytes'] == INPUT_BYTES and profile['input_fingerprint']['fnv1a64'] == '4c3ae263d1492a35', 'Recorded input fingerprint differs')
    for key, calendar in [('starting',[1994,1,1]),('ending',[1994,2,1])]:
        facts = profile[key]
        require(facts['calendar'] == calendar and facts['player'] == 'France' and facts['seed'] == 1990 and facts['alive'] is True, 'Profile facts differ: ' + key)
    require(profile['certified_profile_required'] is True and profile['renew_existing_budget'] is True, 'Required diagnostic capabilities/renewal differ')
    stderr = (root/'original-run/stderr.log').read_text(encoding='utf-8-sig').strip().splitlines()
    require(len(stderr) == 31, 'Unexpected stderr content')
    for i,row in enumerate(profile['rows']):
        before, after = date(1994,1,1)+timedelta(days=i), date(1994,1,2)+timedelta(days=i)
        label = lambda value: f'{value.day} {"Jan" if value.month == 1 else "Feb"} {value.year}'
        require(row['index'] == i and row['date_before'] == label(before) and row['date_after'] == label(after), 'Date/index differs')
        require(row['exact_native_world'] is True and row['exact_returned_headlines'] is True
                and row['validation_error'] is None and row['first_difference_byte'] is None, 'Recorded daily comparison failed')
        require(row['expected_bytes'] == row['actual_bytes'] > 0, 'Recorded world lengths differ')
        require(row['headlines'] == row['log_after'] - row['log_before'], 'Recorded headline count differs')
        expected_line = f'S22 diagnosis day {i+1}/31: {label(before)} -> {label(after)}, observed '
        require(stderr[i].startswith(expected_line) and stderr[i].endswith('; exact world/headlines'), 'Stderr day proof differs')
    stdout = (root/'original-run/stdout.log').read_text(encoding='utf-8-sig')
    require('test performance::s22_daily_subsystem_diagnosis ... ok' in stdout
            and 'test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 475 filtered out; finished in 45.80s' in stdout, 'Test summary differs')
    samples = list(csv.DictReader(io.StringIO((root/'original-run/process.csv').read_text(encoding='utf-8-sig'))))
    require(len(samples) == 172 and all(float(b['elapsed_ms']) > float(a['elapsed_ms']) for a,b in zip(samples,samples[1:])), 'Process sample chronology differs')
    build = read_json(root/'build-record.json')
    require(build['source_unchanged'] is True and build['copied_input_unchanged'] is True, 'Packet build input changed')
    for key in ['source_before','source_after','copied_input_before','copied_input_after','decoded_input']:
        require(build[key]['bytes'] == INPUT_BYTES and build[key]['sha256'] == INPUT_SHA, 'Packet build pin differs: '+key)
    for row in read_json(root/'review/manifest.json')['files']:
        require(hash_file(local(root,'review/'+row['path'])) == (row['bytes'],row['sha256']), 'Copied review file differs')
    for name in ['execution-start.json','execution.json','profile.json','process.csv','stdout.log','stderr.log']:
        require(hash_file(root/'original-run'/name) == hash_file(root/'review/retained'/name), 'Review/raw artifact differs: '+name)
    return {'passed':True,'format':'spheres-portable-diagnostic-verification/v1','source_revision':REVISION,
            'manifest_sha256':expected_manifest,'verified_payloads':len(pins),'decoded_input_bytes':count,
            'decoded_input_sha256':digest.hexdigest(),'recorded_native_days_verified':31,
            'recorded_world_and_headline_comparisons_all_passed':True,'native_worlds_independently_replayed':False,
            'binary_included':False,'binary_executed':False,'qualification':False,
            'scope':'Checks retained artifact integrity, decoded whole input and recorded diagnostic comparisons only; no game execution or S25 qualification.'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('packet',type=Path)
    parser.add_argument('--manifest-sha256',required=True,help='Expected hash supplied independently of this packet')
    args = parser.parse_args()
    try:
        result = verify(args.packet,args.manifest_sha256)
    except Exception as error:
        print(json.dumps({'passed':False,'qualification':False,'error':str(error)},indent=2))
        return 1
    print(json.dumps(result,indent=2))
    return 0


if __name__ == '__main__':
    sys.exit(main())
