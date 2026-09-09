#!/usr/bin/env python3
"""Validate original future cartoons without claiming historical likeness.

Fictional portraits remain outside the historical person manifest. The runtime
catalogue supplies exact identity, name and appearance seed; physical images and
design/visual review establish readiness. This validator generates no artwork.
"""
from __future__ import annotations

import argparse
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import re
import shutil
import tempfile
import unittest

import person_art_pipeline as art

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "spheres-web/data/fictional_portraits.json"
CATALOG = ROOT / "spheres-web/data/future_candidates_2035.json"
CATALOG_PATH = "spheres-web/data/future_candidates_2035.json"
FIRST = "2026-09-08"
UNTIL = "2036-01-01"


def validate(manifest: dict, catalog: dict, root: Path = ROOT, historical_manifest: dict | None = None) -> dict:
    errors, ready = [], {}
    if not isinstance(manifest, dict) or type(manifest.get("version")) is not int or manifest.get("version") != 1 or not isinstance(manifest.get("people"), dict):
        return {"valid": False, "errors": ["Fictional portrait manifest requires version 1 and people object"], "ready": {}}
    if (catalog.get("from"), catalog.get("until_exclusive"), catalog.get("historical_reference_through")) != (FIRST, UNTIL, "2026-09-07"):
        return {"valid": False, "errors": ["Fictional catalogue dates must preserve the historical cutoff"], "ready": {}}
    candidates = {p["person_id"]: p for p in catalog.get("candidates", [])}
    if len(candidates) != len(catalog.get("candidates", [])):
        errors.append("Fictional catalogue person IDs are duplicated")
    if historical_manifest is None:
        historical_manifest = art.read_json(root / "spheres-web/data/person_portraits.json")
    historical_ids = set(historical_manifest.get("people", {}))
    historical_art = [p for person in historical_manifest.get("people", {}).values() for p in person.get("portraits", [])]
    forbidden_assets = {p.get("asset") for p in historical_art}
    forbidden_hashes = {p.get("sha256") for p in historical_art}
    owners, hashes = {}, {}
    for pid, person in manifest["people"].items():
        mark = len(errors)
        if not isinstance(person, dict):
            errors.append(f"{pid}: person must be an object")
            continue
        candidate = candidates.get(pid)
        if not candidate or not pid.startswith("fictional_") or candidate.get("origin") != "fictional_successor" or pid in historical_ids:
            errors.append(f"{pid}: exact fictional catalogue identity required; real people cannot be relabelled")
            continue
        if person.get("name") != candidate.get("name") or person.get("appearance_seed") != candidate.get("appearance_seed") or not re.fullmatch(r"[a-f0-9]{16}", str(candidate.get("appearance_seed", ""))):
            errors.append(f"{pid}: catalogue name and appearance seed must match exactly")
        records = person.get("portraits")
        if not isinstance(records, list):
            errors.append(f"{pid}: portraits array required")
            continue
        intervals, accepted = [], []
        for index, p in enumerate(records):
            where = f"{pid}.portraits[{index}]"
            if not isinstance(p, dict):
                errors.append(f"{where}: record object required")
                continue
            try:
                first, until = art.iso_day(p.get("from")), art.iso_day(p.get("to"))
                if not art.iso_day(FIRST) <= first < until <= art.iso_day(UNTIL):
                    raise ValueError("appearance era must be inside the explicit fictional period")
                intervals.append((first, until))
            except ValueError as error:
                errors.append(f"{where}: {error}")
            expected = {"method": "generated", "style": "cartoon", "status": "fictional-character",
                        "composition": "full-body", "background_mode": "opaque", "generator": art.BUILTIN_GENERATOR,
                        "license": "generated"}
            for key, value in expected.items():
                if p.get(key) != value:
                    errors.append(f"{where}.{key}: must be {value}")
            source = p.get("design_source", {})
            if source != {"kind": "authored_fiction", "person_id": pid, "appearance_seed": candidate["appearance_seed"], "catalog": CATALOG_PATH}:
                errors.append(f"{where}.design_source: exact authored-fiction identity, appearance seed and catalogue path required")
            if "identity_source" in p or "source_url" in p:
                errors.append(f"{where}: fictional designs must not claim a historical identity reference")
            review = p.get("review", {})
            if not isinstance(review, dict) or any(review.get(key) is not True for key in ("design", "visual")):
                errors.append(f"{where}: explicit design and visual review required")
                review = review if isinstance(review, dict) else {}
            if any(key in review for key in ("identity", "likeness", "era")):
                errors.append(f"{where}: design review must not masquerade as historical likeness or era verification")
            for record, key in ((p, "credit"), (p, "era_note"), (p, "generation_record"), (review, "reviewer")):
                if not art.nonempty(record.get(key)):
                    errors.append(f"{where}.{key}: nonempty text required")
            for value in (p.get("generated_at"), review.get("reviewed_at")):
                try:
                    art.iso_day(value)
                except ValueError:
                    errors.append(f"{where}: exact generation and review dates required")
            try:
                prompt = art.safe_path(root, p.get("prompt_record"))
                if not prompt.is_file() or not prompt.read_text(encoding="utf-8-sig").strip():
                    raise ValueError("exact submitted prompt is missing or empty")
            except (ValueError, OSError) as error:
                errors.append(f"{where}.prompt_record: {error}")
            try:
                file = art.safe_path(root, p.get("asset"), ("spheres-web/ui/person-portraits",))
                relative = file.relative_to(root / "spheres-web/ui/person-portraits").as_posix()
                if not re.fullmatch(r"[A-Za-z0-9_-]+-v\d+\.png", relative):
                    raise ValueError("a versioned top-level PNG filename is required")
                digest = hashlib.sha256(file.read_bytes()).hexdigest()
                if digest != p.get("sha256"):
                    raise ValueError("physical image hash does not match")
                from PIL import Image
                with Image.open(file) as image:
                    if image.format != "PNG" or image.size != (p.get("width"), p.get("height")):
                        raise ValueError("actual PNG dimensions do not match metadata")
                    image.verify()
                if p["asset"] in forbidden_assets or digest in forbidden_hashes:
                    raise ValueError("a real person's historical portrait cannot become a fictional character")
                if owners.get(relative, pid) != pid or hashes.get(digest, pid) != pid:
                    raise ValueError("these image bytes already belong to another fictional person")
                owners[relative], hashes[digest] = pid, pid
            except (ValueError, OSError, ImportError) as error:
                errors.append(f"{where}.asset: {error}")
            accepted.append(dict(p))
        intervals.sort()
        if any(next_start < end for (_, end), (next_start, _) in zip(intervals, intervals[1:])):
            errors.append(f"{pid}: fictional portrait eras overlap")
        if len(errors) == mark and accepted:
            ready[pid] = accepted
    if errors:
        ready = {}
    return {"valid": not errors, "errors": errors, "ready": ready}


def select(manifest: dict, catalog: dict, person_id: str, date: str, root: Path = ROOT, historical_manifest=None):
    try:
        when = art.iso_day(date)
    except ValueError:
        return None
    if not art.iso_day(FIRST) <= when < art.iso_day(UNTIL):
        return None
    checked = validate(manifest, catalog, root, historical_manifest)
    matches = [p for p in checked["ready"].get(person_id, []) if p["from"] <= date < p["to"]]
    return matches[0] if len(matches) == 1 else None


def self_test() -> int:
    with tempfile.TemporaryDirectory(prefix="spheres-fiction-art-") as directory:
        root = Path(directory)
        target = root / "spheres-web/ui/person-portraits/fixture-character-v1.png"
        target.parent.mkdir(parents=True)
        # Test-only byte copy establishes a physical PNG fixture, not a game asset.
        source = ROOT / "spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png"
        shutil.copyfile(source, target)
        prompt = root / "prompt.txt"
        prompt.write_text("Synthetic validator fixture only.", encoding="utf-8")
        pid, seed = "fictional_test_person_01", "abc0123456789def"
        candidate = {"person_id": pid, "name": "Example Invented Person", "appearance_seed": seed, "origin": "fictional_successor"}
        catalog = {"from": FIRST, "until_exclusive": UNTIL, "historical_reference_through": "2026-09-07", "candidates": [candidate]}
        portrait = {"from": FIRST, "to": UNTIL, "method": "generated", "style": "cartoon", "status": "fictional-character",
                    "asset": target.relative_to(root).as_posix(), "sha256": hashlib.sha256(target.read_bytes()).hexdigest(),
                    "width": 1024, "height": 1536, "composition": "full-body", "background_mode": "opaque",
                    "generator": art.BUILTIN_GENERATOR, "license": "generated", "generated_at": "2026-09-07",
                    "credit": "Fictional test character.", "era_note": "Fictional future test.", "generation_record": "Test fixture only.",
                    "prompt_record": "prompt.txt", "design_source": {"kind": "authored_fiction", "person_id": pid, "appearance_seed": seed, "catalog": CATALOG_PATH},
                    "review": {"design": True, "visual": True, "reviewer": "Test fixture", "reviewed_at": "2026-09-07"}}
        base = {"version": 1, "people": {pid: {"name": candidate["name"], "appearance_seed": seed, "portraits": [portrait]}}}
        historical = {"people": {}}

        class Checks(unittest.TestCase):
            def checked(self, value):
                return validate(value, catalog, root, historical)

            def reject(self, change):
                value = deepcopy(base)
                change(value["people"][pid]["portraits"][0])
                result = self.checked(value)
                self.assertFalse(result["valid"], result)
                self.assertFalse(result["ready"])

            def test_exact_fictional_identity_seed_file_and_dates(self):
                self.assertTrue(self.checked(base)["valid"])
                self.assertIsNotNone(select(base, catalog, pid, FIRST, root, historical))
                self.assertIsNone(select(base, catalog, pid, "2026-09-07", root, historical))
                self.assertIsNone(select(base, catalog, pid, UNTIL, root, historical))
                self.assertIsNone(select(base, catalog, "another_person", FIRST, root, historical))

            def test_wrong_name_seed_and_unknown_identity_fail(self):
                for field, value in (("name", "Wrong name"), ("appearance_seed", "wrong seed")):
                    m = deepcopy(base)
                    m["people"][pid][field] = value
                    self.assertFalse(self.checked(m)["valid"])
                self.reject(lambda p: p["design_source"].update(person_id="real_historical_person"))

            def test_historical_appearance_and_unbounded_future_fail(self):
                self.reject(lambda p: p.update(**{"from": "1990-01-01"}))
                self.reject(lambda p: p.update(to=None))
                self.reject(lambda p: p.update(to="2036-01-02"))

            def test_review_is_design_not_false_historical_likeness(self):
                self.reject(lambda p: p["review"].update(design=False))
                self.reject(lambda p: p["review"].update(likeness=True))
                self.reject(lambda p: p.update(identity_source={"kind": "observed_portrait"}))

            def test_physical_file_hash_prompt_and_path_are_required(self):
                self.reject(lambda p: p.update(sha256="0"*64))
                self.reject(lambda p: p.update(asset="../secret.png"))
                self.reject(lambda p: p.update(prompt_record="missing-prompt.txt"))

            def test_historical_image_reuse_is_rejected(self):
                old = {"people": {"real_person": {"portraits": [{"asset": portrait["asset"], "sha256": portrait["sha256"]}]}}}
                self.assertFalse(validate(base, catalog, root, old)["valid"])

            def test_overlapping_eras_never_select_an_avatar(self):
                value = deepcopy(base)
                value["people"][pid]["portraits"].append(deepcopy(portrait))
                self.assertFalse(self.checked(value)["valid"])
                self.assertIsNone(select(value, catalog, pid, FIRST, root, historical))
        result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Checks))
        return int(not result.wasSuccessful())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("validate", "self-test"))
    args = parser.parse_args()
    if args.command == "self-test":
        return self_test()
    result = validate(art.read_json(MANIFEST), art.read_json(CATALOG))
    print(json.dumps({"valid": result["valid"], "errors": result["errors"],
                      "validated_fictional_people": len(result["ready"]),
                      "validated_fictional_images": sum(map(len, result["ready"].values()))}, indent=2))
    return int(not result["valid"])


if __name__ == "__main__":
    raise SystemExit(main())
