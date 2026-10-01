"""Verify pinned review evidence; this does not repeat historical reading."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess


HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--repo", type=Path)
parser.add_argument("--originals", type=Path,
                    help="Relocated directory containing retained source bodies and headers")
parser.add_argument("--receipt-revision", default="HEAD",
                    help="Receipt Git revision; use ':' for the staged index before its commit")
args = parser.parse_args()
repo = args.repo or Path(subprocess.check_output(
    ["git", "rev-parse", "--show-toplevel"], cwd=HERE, text=True).strip())
receipt_path = HERE.relative_to(repo).as_posix()


def git(*command):
    return subprocess.check_output(["git", *command], cwd=repo)


def blob(revision, path):
    spec = ":" + path if revision == ":" else revision + ":" + path
    return git("show", spec)


def receipt(name):
    return blob(args.receipt_revision, receipt_path + "/" + name)


def load(name):
    return json.loads(receipt(name))


def check_bytes(data, pin):
    assert len(data) == pin["bytes"], pin
    assert hashlib.sha256(data).hexdigest() == pin["sha256"], pin


def changed(first, last):
    return git("diff", "--name-only", first, last).decode().splitlines()


manifest = load("manifest.json")
for pin in manifest["files"]:
    assert not Path(pin["path"]).is_absolute() and ".." not in Path(pin["path"]).parts
    check_bytes(receipt(pin["path"]), pin)

decision = load("decision.json")
base = decision["base"]
authored = decision["authored_import"]
reviewed = decision["review_correction"]
assert git("rev-parse", authored + "^").decode().strip() == base
assert git("rev-parse", reviewed + "^").decode().strip() == authored
assert sorted(changed(base, authored)) == sorted(decision["authored_paths"])
assert sorted(changed(authored, reviewed)) == sorted(decision["correction_paths"])
index_path = decision["excluded_shared_index"]
assert blob(base, index_path) == blob(authored, index_path) == blob(reviewed, index_path)
for pin in load("reviewed-inputs.json"):
    for stage in ("submitted", "authored", "reviewed"):
        check_bytes(blob(pin[stage]["revision"], pin["path"]), pin[stage])
    assert pin["submitted"]["sha256"] == pin["authored"]["sha256"]

originals = args.originals or Path(decision["retained_originals_default"])
sources = load("source-review.json")
assert len(sources) == 15 and all(row["exact_response_match"] for row in sources)
for row in sources:
    for kind in ("body", "headers"):
        pin = row["retrieval"][kind]
        # Retrieval paths were recorded on Windows; basename works after relocation.
        name = pin["path"].replace("\\", "/").rsplit("/", 1)[-1]
        check_bytes((originals / name).read_bytes(), pin)

claims = load("claim-review.json")
assert len(claims) == 18 and all(row["source_text_preserved"] for row in claims)
data_path = "docs/campaign-certification/C01/research/saudi-arabia.json"
old, new = (json.loads(blob(rev, data_path)) for rev in (base, reviewed))
assert new["sources"][:len(old["sources"])] == old["sources"]
assert new["organizations"] == old["organizations"]
old_institutions = {row["id"]: row for row in old["institutions"]}
new_institutions = {row["id"]: row for row in new["institutions"]}
assert all(new_institutions[key] == value for key, value in old_institutions.items()
           if key != "sa_prime_minister")
holders = new_institutions["sa_prime_minister"]["roles"][0]["holder_claims"]
assert holders[:2] == old_institutions["sa_prime_minister"]["roles"][0]["holder_claims"]
used = {cid for holder in holders
        for cid in ([holder] if isinstance(holder, str) else holder["claim_ids"])}
assert not used.intersection(decision["unsupported_holder_uses_removed"])
assert set(decision["retained_standing_institution_orders"]) <= used
assert sum(isinstance(holder, dict) for holder in holders) == 4
assert all(holder["from"] is None and holder["until"] is None
           for holder in holders if isinstance(holder, dict))
for item in load("validation.json")["checks"]:
    name = "validation/" + item.get("label", "diff-check") + ".log"
    assert hashlib.sha256(receipt(name)).hexdigest() == item["log_sha256"], name

print(json.dumps({"integrity": "verified", "originals": 15, "claims": 18,
                  "new_named_holders": 4, "additional_holder_observations": 7,
                  "reviewed_revision": reviewed, "material_reading_repeated": False,
                  "native_reexecuted": False, "qualification": False}))
