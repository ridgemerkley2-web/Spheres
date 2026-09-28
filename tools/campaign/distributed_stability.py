"""One frozen full plan, independently retained cells, strict local aggregation.

This distributes execution only. Each shard retains the ENTIRE fixed full plan,
executes one named cell with the ordinary native test, and remains a partial
matrix. Only aggregate(), after verifying all 24 retained native runs and their
actual archives, can report full_matrix_passed. Certification is always false.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import stat
import subprocess
import sys

import stability_matrix as matrix

FORMAT = "spheres-distributed-stability/v1"
BUNDLE_FILES = ("native-test", "plan.json", "stability_matrix.py", "distributed_stability.py")


def content_pin(path):
    return {key: value for key, value in matrix.file_pin(path).items() if key != "path"}


def require_clean_revision(repo, revision):
    matrix.require(matrix.REVISION_RE.fullmatch(revision), "Exact candidate revision required")
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip()
    dirty = subprocess.check_output(["git", "status", "--porcelain", "--untracked-files=all"], cwd=repo)
    matrix.require(head == revision and not dirty, "Build bundle requires an unchanged checked-out candidate")


def require_tracked_input(repo, revision, relative, loaded):
    """Bind loaded source to the candidate blob, respecting Git newline filters."""
    tracked = repo / relative
    matrix.require(tracked.is_file() and content_pin(tracked) == content_pin(loaded),
                   f"Loaded source differs from candidate checkout: {relative}")
    lookup = subprocess.run(["git", "rev-parse", "--verify", revision + ':' + relative],
                            cwd=repo, text=True, capture_output=True)
    matrix.require(lookup.returncode == 0, f"Bundle input is not tracked at candidate: {relative}")
    actual = subprocess.check_output(["git", "hash-object", "--path", relative, "--stdin"],
                                     cwd=repo, input=Path(loaded).read_bytes()).decode().strip()
    matrix.require(actual == lookup.stdout.strip(), f"Bundle input differs from candidate blob: {relative}")


def prepare(binary, revision, out, repo=None):
    repo = Path(repo or Path(__file__).resolve().parents[2]).resolve(strict=True)
    require_clean_revision(repo, revision)
    out = Path(out).resolve()
    out.mkdir(parents=True, exist_ok=False)
    inputs = {"native-test": Path(binary).resolve(strict=True),
              "plan.json": repo / "tools/campaign/stability-full.json",
              "stability_matrix.py": Path(matrix.__file__).resolve(),
              "distributed_stability.py": Path(__file__).resolve()}
    for name, relative in (("plan.json", "tools/campaign/stability-full.json"),
                           ("stability_matrix.py", "tools/campaign/stability_matrix.py"),
                           ("distributed_stability.py", "tools/campaign/distributed_stability.py")):
        require_tracked_input(repo, revision, relative, inputs[name])
    plan = matrix.validate_plan(matrix.read_json(inputs["plan.json"]))
    matrix.require(plan["scope"] == "full", "Distributed execution requires the complete canonical plan")
    for name, source in inputs.items():
        before = content_pin(source)
        matrix.write_new(out / name, source.read_bytes())
        matrix.require(content_pin(out / name) == before == content_pin(source), "Bundle input changed during copy")
    (out / "native-test").chmod((out / "native-test").stat().st_mode | stat.S_IXUSR)
    manifest = {"format": FORMAT, "revision": revision, "created_utc": matrix.utc_now(),
                "workflow_run_id": os.environ.get("GITHUB_RUN_ID"),
                "workflow_run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
                "files": {name: content_pin(out / name) for name in BUNDLE_FILES},
                "cells": plan["cells"], "qualification": False, "s25_complete": False,
                "scope": "Distributed execution of the unchanged full 24-cell plan; shards are not full-matrix passes."}
    matrix.write_json_new(out / "manifest.json", manifest)
    return manifest


def bundle(root, expected_manifest):
    root = Path(root).resolve(strict=True)
    matrix.require(type(expected_manifest) is str and len(expected_manifest) == 64
                   and all(c in "0123456789abcdef" for c in expected_manifest), "Expected bundle SHA256 required")
    manifest_path = matrix.owned_file(root, "manifest.json")
    matrix.require(content_pin(manifest_path)["sha256"] == expected_manifest, "Central bundle manifest changed")
    manifest = matrix.read_json(manifest_path)
    matrix.require(manifest.get("format") == FORMAT and matrix.REVISION_RE.fullmatch(manifest.get("revision", "")), "Wrong distributed bundle")
    matrix.require(manifest.get("qualification") is False and manifest.get("s25_complete") is False, "Bundle cannot award certification")
    matrix.require(set(manifest.get("files", {})) == set(BUNDLE_FILES), "Incomplete/extra frozen bundle inputs")
    for name in BUNDLE_FILES:
        matrix.require(content_pin(matrix.owned_file(root, name)) == manifest["files"][name], f"Frozen bundle input changed: {name}")
    plan = matrix.validate_plan(matrix.read_json(root / "plan.json"))
    matrix.require(plan["scope"] == "full" and manifest.get("cells") == plan["cells"], "Central full plan membership changed")
    # Do not silently verify with a different implementation than the frozen one.
    matrix.require(content_pin(Path(matrix.__file__)) == manifest["files"]["stability_matrix.py"], "Loaded matrix verifier differs from frozen harness")
    matrix.require(content_pin(Path(__file__)) == manifest["files"]["distributed_stability.py"], "Loaded distributed verifier differs from frozen harness")
    return root, manifest, plan


def validate_shard_records(manifest, plan, cell_id, freeze, proof, expected_manifest):
    """Cross-shard identity checks, in addition to the native retained verifier."""
    declared = {c["id"]: c for c in plan["cells"]}
    matrix.require(cell_id in declared, "Unknown distributed cell")
    matrix.require(freeze.get("batch_sha256") == expected_manifest == proof.get("batch_sha256"),
                   "Shard belongs to another frozen batch or retry")
    matrix.require(freeze.get("candidate_revision") == manifest["revision"] == proof.get("revision"), "Mixed candidate revisions")
    for field, name in (("binary", "native-test"), ("frozen_plan", "plan.json"), ("frozen_harness", "stability_matrix.py")):
        pin = freeze.get(field, {})
        matrix.require({k: pin.get(k) for k in ("bytes", "sha256")} == manifest["files"][name], f"Mixed frozen {field}")
    matrix.require(freeze.get("only_cell") == cell_id == proof.get("only_cell"), "Cell selection does not match central assignment")
    matrix.require(freeze.get("jobs") == 1 and proof.get("jobs") == 1, "A shard must execute exactly one serial cell")
    matrix.require(proof.get("passed") is False and proof.get("coverage", {}).get("full_matrix_passed") is False,
                   "A shard cannot claim a full matrix pass")
    matrix.require(proof.get("selected_cell_passed") is True and proof.get("binary_unchanged") is True
                   and proof.get("interrupted") is False, "Shard did not complete unchanged")
    cells = proof.get("cells", [])
    matrix.require(len(cells) == 1 and cells[0].get("id") == cell_id and cells[0].get("passed") is True,
                   "Missing, duplicate or failed assigned cell")
    return cells[0]


def run_cell(bundle_root, expected_manifest, cell_id, out, timeout):
    root, manifest, plan = bundle(bundle_root, expected_manifest)
    matrix.require(cell_id in {c["id"] for c in plan["cells"]}, "Unknown assigned cell")
    proof = matrix.run_matrix(root / "native-test", manifest["revision"], root / "plan.json", out,
                              timeout=timeout, jobs=1, only_cell=cell_id, batch_sha256=expected_manifest)
    # Native execution already verified full artifacts; independent verification
    # happens again from the downloaded retained bytes during aggregate().
    validate_shard_records(manifest, plan, cell_id, matrix.read_json(Path(out) / "freeze.json"), proof, expected_manifest)
    return {"selected_cell_passed": True, "full_matrix_passed": False, "qualification": False}


def aggregate(bundle_root, expected_manifest, shard_root, output):
    root, manifest, plan = bundle(bundle_root, expected_manifest)
    shard_root = Path(shard_root).resolve(strict=True)
    expected = {c["id"] for c in plan["cells"]}
    entries = list(shard_root.iterdir())
    matrix.require(all(p.is_dir() and not p.is_symlink() for p in entries)
                   and {p.name for p in entries} == expected, "Exactly 24 assigned shard directories required; no missing/extra/retry directories")
    outcomes, retained, failures = [], [], []
    for cell in plan["cells"]:
        identity = cell["id"]
        run = shard_root / identity
        try:
            proof = matrix.read_json(matrix.owned_file(run, "result.json"))
            freeze = matrix.read_json(matrix.owned_file(run, "freeze.json"))
            outcome = validate_shard_records(manifest, plan, identity, freeze, proof, expected_manifest)
            verified = matrix.verify_retained_run(run)
            matrix.require(verified.get("selected_cell_passed") is True and verified.get("passed") is False,
                           "Independent retained shard verification did not pass its declared cell")
            outcomes.append(outcome)
            retained.append({"id": identity, "result": content_pin(run / "result.json"),
                             "freeze": content_pin(run / "freeze.json"), "verification": verified})
            print(f"Verified {identity}", flush=True)
        except Exception as error:
            failures.append({"id": identity, "error": f"{type(error).__name__}: {str(error)[:2000]}"})
            outcomes.append({"id": identity, "passed": False})
    coverage = matrix.coverage(plan, outcomes)
    result = {"format": "spheres-distributed-stability-result/v1", "revision": manifest["revision"],
              "bundle_manifest_sha256": expected_manifest, "finished_utc": matrix.utc_now(),
              "coverage": coverage, "passed": coverage["full_matrix_passed"] and not failures,
              "failures": failures, "retained_verifications": retained,
              "qualification": False, "s25_complete": False,
              "scope": "All 24 unchanged full-horizon cells independently verified from complete retained archives; controlled succession and formal prerequisites remain separate."}
    matrix.write_json_new(output, result)
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    prep = sub.add_parser("prepare")
    prep.add_argument("--binary", type=Path, required=True)
    prep.add_argument("--revision", required=True)
    prep.add_argument("--out", type=Path, required=True)
    cell = sub.add_parser("cell")
    cell.add_argument("--id", required=True)
    cell.add_argument("--out", type=Path, required=True)
    cell.add_argument("--timeout-seconds", type=float, default=18000)
    agg = sub.add_parser("aggregate")
    agg.add_argument("--shards", type=Path, required=True)
    agg.add_argument("--out", type=Path, required=True)
    for command in (cell, agg):
        command.add_argument("--bundle", type=Path, required=True)
        command.add_argument("--manifest-sha256", required=True)
    args = parser.parse_args(argv)
    try:
        if args.command == "prepare":
            result = prepare(args.binary, args.revision, args.out)
            print(json.dumps({"manifest_sha256": content_pin(args.out / "manifest.json")["sha256"], "cells": [c["id"] for c in result["cells"]]}))
        elif args.command == "cell":
            print(json.dumps(run_cell(args.bundle, args.manifest_sha256, args.id, args.out, args.timeout_seconds)))
        else:
            result = aggregate(args.bundle, args.manifest_sha256, args.shards, args.out)
            print(json.dumps({"passed": result["passed"], "coverage": result["coverage"]}))
            return 0 if result["passed"] else 1
    except Exception as error:
        print(f"Distributed stability refused: {type(error).__name__}: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
