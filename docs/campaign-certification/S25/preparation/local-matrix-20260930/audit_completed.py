"""Read-only snapshot of completed cells in an ACTIVE matrix; never a matrix pass.

Uses the byte-pinned, unchanged frozen verifier's existing cell validation.
Does not manufacture a matrix result or change the running matrix directory.
Output must be a new directory. Large saves remain at their pinned source paths.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path

FROZEN_SHA256 = "c9cb10fd8ee1e9eaf6359316504cd0436e4893d0795bae9360c44880c13d74a0"
REVISION = "68ba0622ec709b78617aadd1f9198d18f532bb32"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    root, output = args.root.resolve(strict=True), args.out.resolve()
    if output.exists() or output.is_relative_to(root) or root.is_relative_to(output):
        raise ValueError("Output must be new and disjoint from the active run")
    helper = root / "stability_matrix.py"
    if hashlib.sha256(helper.read_bytes()).hexdigest() != FROZEN_SHA256:
        raise ValueError("Frozen verifier changed")
    spec = importlib.util.spec_from_file_location("frozen_matrix", helper)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    require = module.require
    output.mkdir(parents=True, exist_ok=False)
    copied = []

    def capture(relative, data=None):
        source = module.owned_file(root, relative)
        data = source.read_bytes() if data is None else data
        destination = output / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        module.write_new(destination, data)
        copied.append({"relative": relative, "source": str(source), "bytes": len(data),
                       "sha256": hashlib.sha256(data).hexdigest()})

    for name in ("freeze.json", "plan.json", "resource-setup.json", "stability_matrix.py"):
        capture(name)
    freeze = module.read_json(output / "freeze.json")
    plan = module.validate_plan(module.read_json(output / "plan.json"))
    require(freeze["candidate_revision"] == REVISION and plan["scope"] == "full", "Wrong candidate/plan")
    require(not (root / "result.json").exists(), "Use the full verifier for a completed matrix")
    for key, path in (("frozen_plan", "plan.json"), ("plan", "plan.json"),
                      ("frozen_harness", "stability_matrix.py"), ("harness", "stability_matrix.py")):
        module.verify_pin(root / path, freeze[key])
    module.verify_pin(Path(freeze["binary"]["path"]), freeze["binary"])
    module.verify_pin(root / "resource-setup.json", freeze["resource_execution"]["setup"])
    journal = (root / "journal.jsonl").read_bytes()
    require(journal.endswith(b"\n"), "Retry after the journal writer completes its current line")
    capture("journal.jsonl", journal)
    events = [module.parse_json(line) for line in journal.splitlines() if line.strip()]
    require(events[0]["event"] == "matrix_started" and events[0]["revision"] == REVISION,
            "Wrong journal identity")
    require(not any(e["event"] == "matrix_finished" for e in events), "Use full verification after completion")
    starts = {e["id"]: e for e in events if e["event"] == "cell_started"}
    finishes = {e["id"]: e for e in events if e["event"] == "cell_finished"}
    require(len(starts) == sum(e["event"] == "cell_started" for e in events), "Duplicate start")
    require(len(finishes) == sum(e["event"] == "cell_finished" for e in events), "Duplicate finish")
    require(set(finishes) <= set(starts) <= {c["id"] for c in plan["cells"]}, "Unknown cell")
    captured_utc = module.utc_now()
    rows = []
    for cell in plan["cells"]:
        cell_id = cell["id"]
        row = {**cell, "status": "running" if cell_id in starts else "queued"}
        if cell_id not in finishes:
            rows.append(row)
            continue
        prefix = "cells/" + cell_id
        cell_root = root / prefix
        native = cell_root / "native"
        verdict = module.read_json(cell_root / "verdict.json")
        execution = module.read_json(cell_root / "execution.json")
        require(verdict["id"] == cell_id and verdict["passed"] is True and verdict["failure"] is None,
                "This snapshot audit requires a completed passing cell")
        require(finishes[cell_id]["passed"] is True and finishes[cell_id]["exit_code"] == 0,
                "Journal does not confirm success")
        require(execution["exit_code"] == 0 and execution["binary_before"] == freeze["binary"] == execution["binary_after"],
                "Cell executable drift or failed execution")
        require(execution["args"] == [freeze["binary"]["path"], "--exact", module.TEST_NAME,
                                      "--ignored", "--nocapture", "--test-threads=1"], "Wrong native invocation")
        origin = module.recorded_join(freeze["resource_execution"]["scratch_root"], prefix)
        require(module.logical_path(execution["cwd"]) == origin, "Wrong execution root")
        native_origin = module.recorded_join(origin, "native")
        module.verify_pin(cell_root / "request.json", starts[cell_id]["request"])
        files = verdict["files"]
        require(len({f["relative"] for f in files}) == len(files), "Duplicate retained file")
        actual_files = {p.relative_to(cell_root).as_posix() for p in cell_root.rglob("*") if p.is_file()}
        require(actual_files == {f["relative"] for f in files} | {"verdict.json"}, "Unlisted/missing cell file")
        for entry in files:
            path = module.owned_file(cell_root, entry["relative"])
            module.verify_pin(path, {k: entry[k] for k in ("path", "bytes", "sha256")})
            if not entry["relative"].endswith(".gz"):
                capture(prefix + "/" + entry["relative"])
        capture(prefix + "/verdict.json")
        receipt = module.read_json(cell_root / "transfer.json")
        require(receipt["verified_before_cleanup"] is True, "Missing verified transfer")
        require(module.logical_path(receipt["source"]) == origin and
                module.logical_path(receipt["destination"]) == module.logical_path(str(cell_root)), "Wrong transfer roots")
        transferred = [e for e in events if e["event"] == "scratch_transfer_verified" and e["id"] == cell_id]
        require(len(transferred) == 1, "Missing transfer journal")
        module.verify_pin(cell_root / "transfer.json", transferred[0]["receipt"])
        require({r["relative"] for r in receipt["files"]} == {f["relative"] for f in files} - {"transfer.json"},
                "Incomplete transfer coverage")
        for transferred_file in receipt["files"]:
            require(all(transferred_file["original"][k] == transferred_file["retained"][k] for k in ("bytes", "sha256")),
                    "Transfer changed bytes")
            module.verify_pin(module.owned_file(cell_root, transferred_file["relative"]), transferred_file["retained"])
        records = verdict["archives"]
        require(module.read_json(cell_root / "archive-manifest.json") ==
                {"format": "spheres-stability-archives/v1", "archives": records}, "Archive manifest mismatch")
        for record in records:
            module.verify_archive_record(native, native_origin, record)
        require(module.validate_test_log(cell_root / "stdout.log") == verdict["test_execution"], "Test log mismatch")
        module.verify_pin(native / "result.json", verdict["native_result"])
        print(f"Revalidating {cell_id}: complete retained archives and native checks", flush=True)
        validated = module.validate_native_result(module.read_json(native / "result.json"), cell, REVISION,
                                                   native, records, native_origin)
        require(validated == verdict["native_validation"], "Independent replay differs from recorded validation")
        row.update(status="completed_pass_revalidated", finished_utc=finishes[cell_id]["utc"],
                   files_verified=len(files), archives_verified=len(records), validation=validated)
        rows.append(row)
        print(f"Verified {cell_id}", flush=True)
    module.verify_pin(Path(freeze["binary"]["path"]), freeze["binary"])
    report = {"format": "spheres-active-matrix-snapshot/v1", "captured_utc": captured_utc,
              "audit_finished_utc": module.utc_now(), "candidate_revision": REVISION, "source": str(root),
              "auditor": module.file_pin(Path(__file__)), "frozen_helper_sha256": FROZEN_SHA256,
              "scope": "Completed-cell audit of an active run; not full-matrix retained verification",
              "counts": {status: sum(row["status"] == status for row in rows)
                         for status in ("completed_pass_revalidated", "running", "queued")},
              "cells": rows, "copied_metadata": copied, "native_reexecuted": False,
              "full_matrix_passed": False, "qualification": False, "s25_complete": False}
    module.write_json_new(output / "snapshot.json", report)
    print(json.dumps(report["counts"]), flush=True)


if __name__ == "__main__":
    main()
