"""Verify returned image bytes, prompt/input pins and optional retained originals.

Run from any directory with Python 3. No external libraries are required.
This is an integrity check, not historical, likeness or country acceptance.
"""
import argparse
import hashlib
import json
import struct
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--require-originals', action='store_true')
    args = parser.parse_args()
    package = Path(__file__).resolve().parent
    root = package.parents[3]
    data = json.loads((package / 'outputs.json').read_text(encoding='utf-8'))
    errors = []
    original_count = 0

    def check(path, expected, label):
        try:
            actual = hashlib.sha256(path.read_bytes()).hexdigest()
        except OSError as error:
            errors.append(f'{label}: {error}')
            return False
        if actual != expected:
            errors.append(f'{label}: SHA-256 mismatch')
            return False
        return True

    for output in data['outputs']:
        path = root / output['path']
        check(path, output['sha256'], output['path'])
        if path.is_file():
            header = path.read_bytes()[:29]
            if header[:8] != b'\x89PNG\r\n\x1a\n':
                errors.append(f'{output["path"]}: not PNG')
            else:
                width, height, depth, colour = struct.unpack('>IIBB', header[16:26])
                mode = {2: 'RGB', 6: 'RGBA'}.get(colour)
                if (width, height, mode) != (output['width'], output['height'], output['mode']):
                    errors.append(f'{output["path"]}: dimensions/mode mismatch')
        check(root / output['prompt_path'], output['prompt_sha256_submitted'], output['prompt_path'])
        for reference in output['inputs']:
            check(root / reference['path'], reference['sha256'], reference['path'])
            if reference.get('original_requested_path'):
                check(root / reference['original_requested_path'], reference['original_requested_sha256'], reference['original_requested_path'])
        original = Path(output['original_tool_output_path'])
        if original.is_file():
            original_count += int(check(original, output['sha256'], str(original)))
        elif args.require_originals:
            errors.append(f'Missing original: {original}')
    for country in data['country_returns']:
        check(root / country['path'], country['sha256'], country['path'])
    expected_sums = ''.join(o['sha256'] + '  ' + o['path'] + '\n' for o in data['outputs'])
    if (package / 'SHA256SUMS.txt').read_text(encoding='utf-8') != expected_sums:
        errors.append('SHA256SUMS.txt differs from output inventory')
    result = {'valid': not errors, 'returned_outputs': len(data['outputs']),
              'selected_for_claude_review': sum(o['selected_for_claude_review'] for o in data['outputs']),
              'original_files_verified_on_this_machine': original_count, 'errors': errors,
              'scope': 'Byte integrity only; no registration or likeness approval.'}
    print(json.dumps(result, indent=2))
    return int(bool(errors))


if __name__ == '__main__':
    raise SystemExit(main())
