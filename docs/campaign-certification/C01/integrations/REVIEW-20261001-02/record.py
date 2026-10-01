from pathlib import Path
import datetime, hashlib, json, subprocess, sys, time
folder = Path(__file__).resolve().parent / 'validation'
folder.mkdir(exist_ok=True)
label, command = sys.argv[1], sys.argv[2:]
started = datetime.datetime.now(datetime.timezone.utc).isoformat()
at = time.monotonic()
result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
log = folder / (label + '.log')
log.write_bytes(result.stdout)
(folder / (label + '.json')).write_text(json.dumps(dict(command=command, cwd=str(Path.cwd()),
    started_utc=started, duration_seconds=round(time.monotonic()-at, 3), exit_code=result.returncode,
    log=log.name, bytes=len(result.stdout), sha256=hashlib.sha256(result.stdout).hexdigest()), indent=2)+'\n', encoding='utf-8')
print(result.stdout.decode('utf-8', 'replace')[-6000:])
print('RECORDED_EXIT', result.returncode)
sys.exit(result.returncode)
