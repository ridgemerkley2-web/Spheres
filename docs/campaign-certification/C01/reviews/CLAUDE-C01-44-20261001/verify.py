"""Offline receipt/preservation verifier. It never retrieves or approves history."""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]


def read(name):
    return json.loads((HERE / name).read_text(encoding="utf8"))


def sha(path):
    with path.open("rb") as source:
        return hashlib.file_digest(source, "sha256").hexdigest()


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT)


def blob(revision, path):
    return git("show", revision + ":" + path)


def tree(revision):
    result = {}
    for record in git("ls-tree", "-r", "-z", revision, "--", "docs/campaign-certification/C01/research/sources").split(b"\0"):
        if record:
            meta, path = record.split(b"\t", 1)
            result[path.decode()] = meta.split()[-1].decode()
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--external-root", type=Path)
    args = parser.parse_args()
    manifest = read("manifest.json")
    seen = set()
    for item in manifest["payloads"]:
        rel = item["path"]
        assert rel not in seen and not Path(rel).is_absolute() and ".." not in Path(rel).parts
        seen.add(rel)
        path = HERE / rel
        assert path.stat().st_size == item["bytes"] and sha(path) == item["sha256"], rel
    decision = read("decision.json")
    assert decision["decision"] == "held" and decision["publication"] == "receipt_only"
    for key in ("accepted_for_integration", "runtime_roster_modified", "art_authorized", "qualification"):
        assert decision[key] is False
    sources, claims = read("source-review.json"), read("claim-review.json")
    assert len(sources["rows"]) == 5 and len(claims["rows"]) == 6
    assert sum(row["materially_read"] for row in sources["rows"]) == 4
    assert sum(row["materially_read"] for row in claims["rows"]) == 5
    assert all(not row["accepted_for_integration"] for row in sources["rows"] + claims["rows"])
    held = next(row for row in sources["rows"] if not row["materially_read"])
    assert held["source_id"] == "to_pmo_20200317_ptoa_reply"
    assert held["http_status"] == 429 and held["retry_after"] is None
    assert held["retrieval"]["bytes"] == 117
    assert held["retrieval"]["sha256"] == "ae138caf8767f7be2fe6f47f1663b0e2e28d903264707aa9b6f73bb7b223902c"
    assert claims["new_holders"] == 0
    validation = read("validation.json")
    assert validation["tests_passed"] == 141 and validation["tests_failed"] == 0
    assert all(run["exit_code"] == 0 for run in validation["runs"])

    base, tip = decision["base"], decision["submission"]
    path = "docs/campaign-certification/C01/research/tonga.json"
    prior, candidate = json.loads(blob(base, path)), json.loads(blob(tip, path))
    assert len(prior["sources"]) == 207
    assert sum(len(s["claims"]) for s in prior["sources"]) == 319
    assert candidate["sources"][:207] == prior["sources"]
    assert len(candidate["sources"]) == 212
    assert sum(len(s["claims"]) for s in candidate["sources"][207:]) == 6
    new_claim_ids = {c["id"] for s in candidate["sources"][207:] for c in s["claims"]}
    assert new_claim_ids == {row["claim_id"] for row in claims["rows"]}
    for category in ("organizations", "institutions"):
        assert [e["id"] for e in prior[category]] == [e["id"] for e in candidate[category]]
        for old, new in zip(prior[category], candidate[category]):
            assert old["roles"] == new["roles"], old["id"]
            for key in set(old) | set(new):
                if old.get(key) == new.get(key):
                    continue
                if key in ("sources", "claim_ids"):
                    assert old["id"] == "to_dpfi" and new[key][:len(old[key])] == old[key]
                elif key == "coverage":
                    a, b = copy.deepcopy(old[key]), copy.deepcopy(new[key])
                    first, second = a.pop("unresolved"), b.pop("unresolved")
                    assert a == b and second[:len(first)] == first
                else:
                    raise AssertionError((old["id"], key))
    a, b = copy.deepcopy(prior), copy.deepcopy(candidate)
    for key in ("sources", "organizations", "institutions"):
        a.pop(key)
        b.pop(key)
    first, second = a["coverage"].pop("unresolved"), b["coverage"].pop("unresolved")
    assert a == b and second[:len(first)] == first and len(second) == len(first) + 1
    originals = {s["snapshot"]["path"] for s in prior["sources"] if s.get("snapshot")}
    first, second = tree(base), tree(tip)
    assert len(originals) == 194
    assert all(first[path] == second[path] for path in originals)
    authored = git("diff", "--name-only", base + "..." + tip).decode().splitlines()
    authored.remove("docs/campaign-certification/C01/research-index.json")
    assert len(authored) == 10 and sorted(authored) == sorted(read("scope-review.json")["imported_paths"])

    external_count = 0
    if args.external_root:
        for item in read("external-files.json")["files"]:
            path = args.external_root / item["path"]
            assert path.stat().st_size == item["bytes"] and sha(path) == item["sha256"], str(path)
            external_count += 1
    print(json.dumps(dict(status="technical_receipt_and_preservation_verified", payloads=len(seen),
                         prior_extracts=194, prior_sources=207, prior_claims=319, external_files=external_count,
                         packet_decision="held", accepted_for_integration=False, historical_approval=False)))


if __name__ == "__main__":
    main()
