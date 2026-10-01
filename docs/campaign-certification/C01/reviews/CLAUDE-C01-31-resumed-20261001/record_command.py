"""Capture a single review command without overwriting earlier outcomes."""
import argparse
import datetime
import json
import pathlib
import subprocess
import sys

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--name', required=True)
parser.add_argument('--cwd', required=True)
parser.add_argument('command', nargs=argparse.REMAINDER)
args = parser.parse_args()
command = args.command[1:] if args.command[0] == '--' else args.command
out = pathlib.Path(__file__).resolve().parent / 'validation'
out.mkdir(exist_ok=True)
record = {'name': args.name, 'command': command, 'cwd': str(pathlib.Path(args.cwd).resolve()), 'started_at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'log': args.name + '.log'}
with (out / record['log']).open('xb') as log:
    result = subprocess.run(command, cwd=args.cwd, stdout=log, stderr=subprocess.STDOUT)
record.update(exit_code=result.returncode, finished_at=datetime.datetime.now(datetime.timezone.utc).isoformat())
(out / (args.name + '.json')).write_text(json.dumps(record, indent=2) + '\n', encoding='utf8')
print(json.dumps(record), flush=True)
sys.exit(result.returncode)
