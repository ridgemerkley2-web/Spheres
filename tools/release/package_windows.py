#!/usr/bin/env python3
"""Package a committed, already-built Windows or Linux executable; never compile."""
import argparse
import json
from pathlib import Path
import sys
from release_package import PackageError, package_name, package_release


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--name', type=package_name)
    parser.add_argument('--platform', choices=['windows', 'linux'], default='windows' if sys.platform == 'win32' else 'linux')
    parser.add_argument('--base', help='Optional source-patch base commit; omitted means no patch.')
    args = parser.parse_args()
    try:
        result = package_release(Path(__file__).resolve().parents[2], args.binary, args.output, args.name, args.platform, args.base)
    except (PackageError, OSError, ValueError) as error:
        parser.exit(2, f'Packaging refused: {error}\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
