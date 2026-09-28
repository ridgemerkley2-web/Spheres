"""Run the combined preparation checks into a fresh evidence directory."""
import argparse
import hashlib
import importlib.metadata
import json
import os
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path


def pin(path, root):
    data = path.read_bytes()
    return {"path": path.relative_to(root).as_posix(), "bytes": len(data),
            "sha256": hashlib.sha256(data).hexdigest(), "hash_scope": "raw"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[5])
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root, output = args.root.resolve(), args.output.resolve()
    if output.exists() and any(output.iterdir()):
        parser.error("Use a new empty output directory; retained runs are immutable.")
    output.mkdir(parents=True, exist_ok=True)
    env = {**os.environ, "PYTHONDONTWRITEBYTECODE": "1", "PYTHONIOENCODING": "utf-8"}
    scripts = [
        ("census-check", ["tools/avatars/campaign_census.py", "--check"]),
        ("research-check", ["tools/avatars/campaign_research.py", "--check"]),
        ("gap-ledger-check", ["tools/avatars/certified_gap_ledger.py", "--check"]),
        ("cartoon-export-check", ["tools/avatars/cartoon_review.py", "--check"]),
        ("boundary-matrix-check", ["tools/avatars/certified_boundary_matrix.py", "--check"]),
        ("successor-validator", ["tools/avatars/check_successor_proposals.py"]),
        ("workboard-check", ["tools/planning/workboard.py", "--check"]),
        ("source-17-evidence", ["docs/campaign-certification/C01/integrations/CLAUDE-C01-SOURCE-17/verify-review.py"]),
        ("source-26-evidence", ["docs/campaign-certification/C01/integrations/CLAUDE-C01-SOURCE-26/verify-review.py"]),
        ("avatar-suite", ["-m", "unittest", "discover", "-s", "tools/avatars", "-p", "test_*.py"]),
    ]
    record = {"format": "spheres-s23-combined-preparation-checks/v1",
              "scope": "Offline tooling/research regression only; no campaign certification, art approval or native performance rerun.",
              "started_utc": datetime.now(timezone.utc).isoformat(),
              "python": sys.version, "python_executable": sys.executable,
              "head_before_checks": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
              "inputs": [], "checks": []}
    try:
        record["pypdf_version"] = importlib.metadata.version("pypdf")
    except importlib.metadata.PackageNotFoundError:
        record["pypdf_version"] = None
    tracked = subprocess.check_output(["git", "ls-files", "tools/avatars", "tools/planning",
                                     "docs/campaign-certification/C01/research", "docs/planning/ai-task-queue.json",
                                     "docs/planning/ai-workstreams.json", "docs/planning/campaign-pathway.json",
                                     "docs/campaign-certification/C04/preparation"], cwd=root, text=True).splitlines()
    record["inputs"] = [pin(root / path, root) for path in tracked if (root / path).is_file()]
    for name, argv in scripts:
        cmd = [sys.executable, "-X", "utf8", *argv]
        started, tick = datetime.now(timezone.utc).isoformat(), time.monotonic()
        result = subprocess.run(cmd, cwd=root, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        log = output / f"{name}.log"
        log.write_bytes(result.stdout)
        record["checks"].append({"name": name, "command": cmd, "started_utc": started,
                                 "duration_seconds": round(time.monotonic() - tick, 3),
                                 "returncode": result.returncode, "log": pin(log, output)})
        (output / "validation.json").write_text(json.dumps(record, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
        print(f"{name}: {'PASS' if result.returncode == 0 else 'FAIL'}", flush=True)
    unchanged = all(pin(root / row["path"], root) == row for row in record["inputs"])
    runtime = subprocess.run(["git", "diff", "--quiet", "75626a137d5efd1ca1cca0d6f76c4d8502909bee",
                              "--", "spheres-sim", "spheres-web"], cwd=root)
    record.update(finished_utc=datetime.now(timezone.utc).isoformat(), checked_inputs_unchanged=unchanged,
                  production_trees_unchanged_since_s22_closeout=runtime.returncode == 0,
                  production_tree_compare_returncode=runtime.returncode)
    passed = unchanged and runtime.returncode == 0 and all(c["returncode"] == 0 for c in record["checks"])
    record["passed"] = passed
    (output / "validation.json").write_text(json.dumps(record, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    return 0 if passed else 1


if __name__ == "__main__":
    raise SystemExit(main())
