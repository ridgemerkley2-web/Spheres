"""Read-only receipt integrity check; no network, source-content approval or parent qualification."""
import argparse
import hashlib
import json
from pathlib import Path


def digest(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--external-evidence", type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    manifest = json.loads((root / "manifest.json").read_text(encoding="utf-8"))
    seen = set()
    for row in manifest["files"]:
        path = (root / row["path"]).resolve()
        assert path.is_relative_to(root) and row["path"] not in seen
        seen.add(row["path"])
        raw = path.read_bytes()
        assert len(raw) == row["bytes"] and digest(raw) == row["sha256"], row["path"]
    expected = {p.relative_to(root).as_posix() for p in root.rglob("*") if p.is_file() and p.name != "manifest.json"}
    assert seen == expected
    sources = json.loads((root / "source-verification.json").read_text(encoding="utf-8"))["sources"]
    assert len(sources) == 38 and len({r["source_id"] for r in sources}) == 38
    checked = 0
    if args.external_evidence is not None:
        external = args.external_evidence.resolve(strict=True)
        for row in sources:
            path = (external / row["body_relative_path"]).resolve(strict=True)
            assert path.is_relative_to(external)
            raw = path.read_bytes()
            assert len(raw) == row["actual_bytes"] == row["expected_bytes"], row["source_id"]
            assert digest(raw) == row["actual_sha256"] == row["expected_sha256"], row["source_id"]
            checked += 1
    print(json.dumps({"receipt_integrity_passed": True, "payloads": len(seen), "external_raw_bodies_rehashed": checked,
                      "source_content_review_repeated": False, "parent_qualification": False}, indent=2))


if __name__ == "__main__":
    main()
