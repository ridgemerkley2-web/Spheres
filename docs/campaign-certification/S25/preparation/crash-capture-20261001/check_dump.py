"""Verify the retained owned proof and a deliberately truncated fixture offline."""
import argparse
import json
from pathlib import Path
from inspect_dump import inspect

p = argparse.ArgumentParser()
p.add_argument("raw", type=Path)
p.add_argument("output", type=Path)
args = p.parse_args()
args.output.mkdir(exist_ok=False)
dump = next((args.raw / "owned-proof-dumps").glob("*.dmp"))
r = inspect(dump)
checks = {
    "owned_pid_matches_printed_launch_identity": r["process_id"] == 42036,
    "owned_thread_matches_printed_launch_identity": r["exception"]["thread_id"] == 39588,
    "actual_unhandled_execute_fault": r["exception"]["code"] == "0xc0000005" and r["exception"]["address"] == "0x1" and r["exception"]["parameters"] == [8, 1],
    "full_memory_flag_and_complete_payload": r["full_memory"] and r["memory64_payload_complete"],
    "context_and_module_streams_present": r["exception"]["context_bytes"] > 0 and bool(r["modules"]),
    "wrong_pid_would_not_match_owned_identity": r["process_id"] != 42037,
}
truncated = args.output / "deliberately-truncated.dmp"
with dump.open("rb") as source, truncated.open("xb") as dest:
    dest.write(source.read(1024 * 1024))
try:
    inspect(truncated)
except ValueError as exc:
    checks["truncated_full_dump_rejected"] = True
    reason = str(exc)
else:
    checks["truncated_full_dump_rejected"] = False
    reason = "Unexpected accepted truncation"
assert all(checks.values()), checks
result = dict(passed=len(checks), failed=0, checks=checks, truncation_error=reason, native_campaign_launched=False)
(args.output / "owned-dump-inspection.json").write_text(json.dumps(r, indent=2) + "\n", encoding="utf8")
(args.output / "dump-checks.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf8")
print(json.dumps(result, indent=2))
