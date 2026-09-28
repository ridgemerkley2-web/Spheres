#!/usr/bin/env python3
"""Verify and extract a package into a new directory without game source."""
import argparse
import json
from pathlib import Path
from release_package import PackageError, extract_verified, json_bytes


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('archive', type=Path)
    parser.add_argument('destination', type=Path)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    args.report.parent.mkdir(parents=True, exist_ok=True)
    with args.report.open('xb') as report:
        try:
            result = extract_verified(args.archive, args.destination)
        except (PackageError, OSError, ValueError) as error:
            report.write(json_bytes({'format': 'spheres-package-verification/v1', 'passed': False, 'error': str(error)}))
            parser.exit(2, f'Package verification refused: {error}\n')
        report.write(json_bytes(result))
    print(json.dumps({'passed': True, 'report': str(args.report), 'extracted_root': result['extracted_root']}))


if __name__ == '__main__':
    main()
