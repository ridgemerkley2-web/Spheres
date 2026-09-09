#!/usr/bin/env python3
"""Validate person-linked portraits and prepare honest, versioned artwork jobs.

This tool never downloads, generates, edits, approves or substitutes an image.
It reads authored person IDs, records actual coverage, and optionally writes
prompt/job documents. Nation avatars are not a source of identity mappings.
The active direction is fixed cartoon illustrations; see CARTOON_CHARACTER_ROADMAP.md.
This manifest counts registered exact-person images, not archived physical models.
"""

from __future__ import annotations

import argparse
from datetime import date, timedelta
import hashlib
import json
import math
from pathlib import Path, PurePosixPath
import re
import sys
from typing import Any
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_MANIFEST = ROOT / "spheres-web/data/person_portraits.json"
DEFAULT_REGISTRY = ROOT / "spheres-sim/data/party_leaders.json"
SCHEMA_VERSION = 1
JOB_VERSION = "person-cartoon-v4"
STYLE_ANCHOR = "spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png"
BUILTIN_GENERATOR = "OpenAI built-in image_gen"
ART_ROOTS = (
    "spheres-web/ui/person-portraits",
    "spheres-web/ui/portraits",
    "spheres-web/ui/leader-art",
    "spheres-web/ui/display-art",
)
ID_RE = re.compile(r"[A-Za-z0-9][A-Za-z0-9_.-]{0,119}\Z")
HASH_RE = re.compile(r"[a-f0-9]{64}\Z")
FREE_LICENSES = {
    "public domain", "pdm-1.0", "cc0", "cc0 1.0", "cc0-1.0",
    *{f"cc by {v}" for v in ("1.0", "2.0", "2.5", "3.0", "4.0")},
    *{f"cc by-sa {v}" for v in ("1.0", "2.0", "2.5", "3.0", "4.0")},
    *{f"cc-by-{v}" for v in ("1.0", "2.0", "2.5", "3.0", "4.0")},
    *{f"cc-by-sa-{v}" for v in ("1.0", "2.0", "2.5", "3.0", "4.0")},
}


def nonempty(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def web_url(value: Any) -> bool:
    if not isinstance(value, str):
        return False
    try:
        parsed = urlsplit(value)
        return parsed.scheme in {"https", "http"} and bool(parsed.hostname) and not parsed.username and not parsed.password
    except ValueError:
        return False


def iso_day(value: Any) -> date:
    if not isinstance(value, str) or not re.fullmatch(r"\d{4}-\d{2}-\d{2}", value):
        raise ValueError("expected an exact YYYY-MM-DD date")
    return date.fromisoformat(value)


def digest(value: Any) -> str:
    return hashlib.sha256(json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(",", ":")).encode()).hexdigest()


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8-sig"))


def has_transparent_alpha(path: Path) -> bool:
    """A painted checkerboard is opaque; an entirely invisible file is not art."""
    from PIL import Image
    with Image.open(path) as image:
        low, high = image.convert("RGBA").getchannel("A").getextrema()
    return low == 0 and high > 0


def safe_path(repo: Path, value: Any, allowed: tuple[str, ...] | None = None) -> Path:
    if not isinstance(value, str) or not value or "\\" in value:
        raise ValueError("expected a repository-relative path using forward slashes")
    posix = PurePosixPath(value)
    if posix.is_absolute() or any(part in {"..", "."} for part in value.split("/")) or ":" in value:
        raise ValueError("unsafe repository path")
    root = repo.resolve()
    result = (root / value).resolve()
    if not result.is_relative_to(root):
        raise ValueError("resolved path escapes the repository")
    if allowed and not any(result.is_relative_to((root / prefix).resolve()) for prefix in allowed):
        raise ValueError("asset is outside the approved portrait directories")
    return result


def people_from_registry(registry: Any) -> dict[str, dict[str, Any]]:
    """Read explicit IDs only; no name matching, slug creation or inference."""
    if not isinstance(registry, dict):
        raise ValueError("person registry must be an object")
    source = registry.get("people", {})
    result: dict[str, dict[str, Any]] = {}
    if isinstance(source, dict):
        items = source.items()
    elif isinstance(source, list):
        items = ((person.get("id") or person.get("person_id"), person) for person in source if isinstance(person, dict))
        if len(source) != sum(isinstance(p, dict) for p in source):
            raise ValueError("every registry person must be an object")
    else:
        raise ValueError("registry people must be a list or keyed object")
    for person_id, person in items:
        if not isinstance(person_id, str) or not ID_RE.fullmatch(person_id):
            raise ValueError(f"missing or unsafe authored person ID: {person_id!r}")
        if person_id in result:
            raise ValueError(f"duplicate person ID: {person_id}")
        if not isinstance(person, dict) or not nonempty(person.get("name")):
            raise ValueError(f"{person_id}: a person name is required")
        result[person_id] = dict(person, id=person_id)
    return result


def executive_links(executives: Any, known: dict[str, dict], authored_map: Any = None) -> dict:
    """Expose unresolved 1990 identities instead of borrowing national art.

    An optional mapping has entries [{nation, office, name, person_id}]. Exact
    composite keys prevent an office's successor from inheriting its portrait.
    Native executive rows/also rows may instead supply their own person_id.
    """
    entries = authored_map.get("entries", []) if isinstance(authored_map, dict) else []
    mappings: dict[tuple, str] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            raise ValueError("executive mapping entries must be objects")
        key = tuple(entry.get(field) for field in ("nation", "office", "name"))
        if not all(nonempty(v) for v in key) or key in mappings:
            raise ValueError("executive mapping needs a unique exact nation, office and name")
        if entry.get("person_id") not in known:
            raise ValueError(f"unknown mapped executive person ID: {entry.get('person_id')}")
        mappings[key] = entry["person_id"]
    resolved, unresolved = [], []
    for row in executives.get("rows", []) if isinstance(executives, dict) else []:
        for person in [row, *row.get("also", [])]:
            if not nonempty(person.get("name")):
                continue
            item = {"nation": row.get("nation"), "office": person.get("office"), "name": person["name"]}
            key = tuple(item[field] for field in ("nation", "office", "name"))
            person_id = person.get("person_id") or mappings.get(key)
            if person_id and person_id not in known:
                raise ValueError(f"unknown executive person ID: {person_id}")
            if person_id:
                resolved.append(dict(item, person_id=person_id))
            else:
                unresolved.append(dict(item, reason="No authored person ID; name matching is disabled."))
    return {"resolved": resolved, "unresolved": unresolved}


def validate_manifest(manifest: Any, repo_root: Path = ROOT, known_people: dict[str, dict] | None = None) -> dict:
    """Return errors and physically verified assets; never treats metadata as art."""
    errors: list[str] = []
    warnings: list[str] = []
    ready: dict[str, list[dict]] = {}
    if not isinstance(manifest, dict) or type(manifest.get("version")) is not int or manifest.get("version") != SCHEMA_VERSION or not isinstance(manifest.get("people"), dict):
        return {"valid": False, "errors": ["manifest requires version 1 and a people object"], "warnings": [], "ready": {}}
    used_assets: dict[str, str] = {}
    used_hashes: dict[str, str] = {}

    def require_text(record: dict, field: str, where: str) -> None:
        if not nonempty(record.get(field)):
            errors.append(f"{where}.{field}: nonempty text required")

    def require_url(record: dict, field: str, where: str) -> None:
        if not web_url(record.get(field)):
            errors.append(f"{where}.{field}: an http(s) source URL is required")

    def check_file(record: dict, where: str) -> Path | None:
        sha = record.get("sha256")
        if not isinstance(sha, str) or not HASH_RE.fullmatch(sha):
            errors.append(f"{where}.sha256: full lowercase SHA-256 required")
        try:
            path = safe_path(repo_root, record.get("asset"), ART_ROOTS)
            if path.suffix.lower() not in {".png", ".webp", ".jpg", ".jpeg"}:
                raise ValueError("unsupported image extension")
            if not path.is_file():
                raise ValueError("file is missing; queued art is not a ready portrait")
            if hashlib.sha256(path.read_bytes()).hexdigest() != sha:
                raise ValueError("file does not match the recorded SHA-256")
            from PIL import Image
            with Image.open(path) as image:
                width, height = image.size
                if image.format not in {"PNG", "WEBP", "JPEG"}:
                    raise ValueError("file is not a supported raster image")
                image.verify()
            if min(width, height) <= 0:
                raise ValueError("empty image dimensions")
            if record.get("background_mode") == "transparent" and not has_transparent_alpha(path):
                raise ValueError("transparent background declaration requires actual transparent and visible alpha pixels")
            for field, actual in (("width", width), ("height", height)):
                if field in record and (type(record[field]) is not int or record[field] != actual):
                    raise ValueError(f"recorded {field} does not match actual image")
            return path
        except (ValueError, OSError, ImportError) as error:
            errors.append(f"{where}.asset: {error}")
            return None

    for person_id, person in manifest["people"].items():
        start_errors = len(errors)
        where = f"people.{person_id}"
        if not isinstance(person_id, str) or not ID_RE.fullmatch(person_id) or not isinstance(person, dict):
            errors.append(f"{where}: safe authored ID and object required")
            continue
        require_text(person, "name", where)
        if known_people is not None:
            known = known_people.get(person_id)
            if known is None:
                errors.append(f"{where}: unknown authored person ID")
            else:
                aliases = known.get("aliases", [])
                names = [known.get("name"), known.get("native"), *(aliases if isinstance(aliases, list) else [])]
                if person.get("name") not in names:
                    errors.append(f"{where}.name: not the known person's name or an authored alias")
        sources = person.get("identity_sources")
        if not isinstance(sources, list) or not sources or not all(web_url(s) for s in sources):
            errors.append(f"{where}.identity_sources: explicit identity-source URLs required")
            sources = []
        portraits = person.get("portraits")
        if not isinstance(portraits, list):
            errors.append(f"{where}.portraits: list required (empty means no portrait)")
            continue
        identity_valid = len(errors) == start_errors
        intervals: list[tuple[date, date, int]] = []
        person_ready = []
        for index, portrait in enumerate(portraits):
            mark = len(errors)
            at = f"{where}.portraits[{index}]"
            if not isinstance(portrait, dict):
                errors.append(f"{at}: portrait must be an object")
                continue
            interval = None
            try:
                first = iso_day(portrait.get("from"))
                if "to" not in portrait:
                    raise ValueError("explicit to date or null required")
                until = date.max if portrait["to"] is None else iso_day(portrait["to"])
                if first >= until:
                    raise ValueError("from must precede exclusive to")
                interval = (first, until)
                intervals.append((first, until, index))
            except ValueError as error:
                errors.append(f"{at}.era: {error}")
            require_text(portrait, "credit", at)
            require_url(portrait, "source_url", at)
            method = portrait.get("method")
            reference = portrait.get("identity_source")
            if not isinstance(reference, dict) or reference.get("person_id") != person_id:
                errors.append(f"{at}.identity_source: exact matching person_id required")
            else:
                require_url(reference, "source_url", at + ".identity_source")
                if reference.get("kind") == "observed_portrait":
                    check_file(reference, at + ".identity_source")
                    for field in ("license", "credit"):
                        require_text(reference, field, at + ".identity_source")
                    require_url(reference, "license_url", at + ".identity_source")
                    if str(reference.get("license", "")).lower().strip() not in FREE_LICENSES:
                        errors.append(f"{at}.identity_source.license: not an approved source license")
                elif reference.get("kind") == "authored_identity":
                    if reference.get("source_url") not in (sources or []):
                        errors.append(f"{at}.identity_source: authored identity must reference this person's identity_sources")
                    if method == "archival":
                        errors.append(f"{at}: archival artwork requires an observed_portrait identity reference")
                else:
                    errors.append(f"{at}.identity_source.kind: observed_portrait or authored_identity required")
            review = portrait.get("review")
            if not isinstance(review, dict):
                errors.append(f"{at}.review: explicit visual/likeness review and named reviewer required")
            else:
                for field in ("identity", "likeness", "era", "visual"):
                    if review.get(field) is not True:
                        errors.append(f"{at}.review.{field}: must explicitly be true")
                require_text(review, "reviewer", at + ".review")
                try:
                    iso_day(review.get("reviewed_at"))
                except ValueError:
                    errors.append(f"{at}.review.reviewed_at: exact review date required")
            crop = portrait.get("crop")
            if crop is not None:
                if not isinstance(crop, dict) or set(crop) != {"x", "y", "width", "height"}:
                    errors.append(f"{at}.crop: normalized x, y, width, height required")
                elif not all(type(v) in (float, int) and math.isfinite(v) for v in crop.values()):
                    errors.append(f"{at}.crop: finite numeric values required")
                elif crop["x"] < 0 or crop["y"] < 0 or crop["width"] <= 0 or crop["height"] <= 0 or crop["x"] + crop["width"] > 1 or crop["y"] + crop["height"] > 1:
                    errors.append(f"{at}.crop: rectangle must lie inside the image")
            if method == "archival":
                for field in ("creator", "license", "rights_statement"):
                    require_text(portrait, field, at)
                require_url(portrait, "license_url", at)
                if str(portrait.get("license", "")).lower().strip() not in FREE_LICENSES:
                    errors.append(f"{at}.license: not an approved archival license")
            elif method == "generated":
                if portrait.get("generator") != BUILTIN_GENERATOR:
                    errors.append(f"{at}.generator: built-in image_gen provenance required")
                if portrait.get("license") != "generated":
                    errors.append(f"{at}.license: use generated; source rights stay on identity_source")
                try:
                    iso_day(portrait.get("generated_at"))
                except ValueError:
                    errors.append(f"{at}.generated_at: exact generation date required")
                try:
                    prompt = safe_path(repo_root, portrait.get("prompt_record"))
                    if not prompt.is_file() or not prompt.read_text(encoding="utf-8-sig").strip():
                        raise ValueError("exact submitted prompt record is missing or empty")
                except (ValueError, OSError) as error:
                    errors.append(f"{at}.prompt_record: {error}")
                require_text(portrait, "generation_record", at)
            else:
                errors.append(f"{at}.method: archival or generated required")
            path = check_file(portrait, at)
            if path:
                canonical = str(path)
                owner = used_assets.get(canonical)
                if owner and owner != person_id:
                    errors.append(f"{at}.asset: already assigned to another person ({owner})")
                used_assets[canonical] = person_id
                hash_owner = used_hashes.get(portrait["sha256"])
                if hash_owner and hash_owner != person_id:
                    errors.append(f"{at}.sha256: these image bytes already belong to another person ({hash_owner})")
                used_hashes[portrait["sha256"]] = person_id
            if identity_valid and len(errors) == mark and interval:
                person_ready.append(dict(portrait, _index=index))
        intervals.sort()
        for previous, current in zip(intervals, intervals[1:]):
            if current[0] < previous[1]:
                errors.append(f"{where}: portrait eras {previous[2]} and {current[2]} overlap; selection would be ambiguous")
                person_ready = []
        if person_ready:
            ready[person_id] = person_ready
        elif not portraits:
            warnings.append(f"{where}: no portrait supplied")
    # A manifest with conflicting ownership cannot partially authorize assets.
    if errors:
        ready = {}
    return {"valid": not errors, "errors": errors, "warnings": warnings, "ready": ready}


def select_portrait(manifest: dict, person_id: str, at_date: str, repo_root: Path = ROOT,
                    known_people: dict[str, dict] | None = None) -> dict | None:
    """Select only an explicitly reviewed exact-person era; no nearest fallback."""
    day = iso_day(at_date)
    result = validate_manifest(manifest, repo_root, known_people)
    if not result["valid"]:
        return None
    for portrait in result["ready"].get(person_id, []):
        if iso_day(portrait["from"]) <= day < (date.max if portrait["to"] is None else iso_day(portrait["to"])):
            return {key: value for key, value in portrait.items() if key != "_index"}
    return None


def missing_intervals(portraits: list[dict], first: date, until: date) -> list[tuple[date, date]]:
    intervals = sorted((max(first, iso_day(p["from"])), min(until, date.max if p["to"] is None else iso_day(p["to"]))) for p in portraits)
    cursor = first
    missing = []
    for start, end in intervals:
        if start >= end or end <= cursor:
            continue
        if start > cursor:
            missing.append((cursor, start))
        cursor = max(cursor, end)
    if cursor < until:
        missing.append((cursor, until))
    return missing


def bound_range(bound: Any, first: date, until: date) -> tuple[date, date, bool]:
    """Possible boundary range, retaining month/year uncertainty explicitly."""
    if isinstance(bound, str):
        value = iso_day(bound)
        return value, value, True
    if not isinstance(bound, dict):
        return first, until, False
    kind, value = bound.get("kind"), bound.get("value")
    if kind == "day":
        d = iso_day(value)
        return d, d, True
    if kind == "month" and isinstance(value, str) and re.fullmatch(r"\d{4}-\d{2}", value):
        d = iso_day(value + "-01")
        end = date(d.year + (d.month == 12), d.month % 12 + 1, 1) - timedelta(days=1)
        return d, end, False
    if kind == "year" and isinstance(value, str) and re.fullmatch(r"\d{4}", value):
        return date(int(value), 1, 1), date(int(value), 12, 31), False
    return first, until, False


def coverage_report(manifest: dict, registry: dict, repo_root: Path = ROOT,
                    first: str = "1990-01-01", until: str = "2027-01-01",
                    executives: dict | None = None, executive_map: dict | None = None) -> dict:
    start, end = iso_day(first), iso_day(until)
    if start >= end:
        raise ValueError("coverage from must precede exclusive to")
    known = people_from_registry(registry)
    checked = validate_manifest(manifest, repo_root, known)
    ready = checked["ready"]
    people = []
    for person_id, person in sorted(known.items()):
        gaps = missing_intervals(ready.get(person_id, []), start, end)
        people.append({"person_id": person_id, "name": person["name"],
                       "status": "ready" if not gaps else "partial" if ready.get(person_id) else "missing",
                       "portraits": len(ready.get(person_id, [])),
                       "missing_eras": [{"from": a.isoformat(), "to": b.isoformat()} for a, b in gaps]})
    parties = []
    for party in registry.get("parties", []):
        terms = []
        for term in party.get("terms", []):
            person_id = term.get("person") or term.get("person_id")
            begin_min, _, begin_exact = bound_range(term.get("from"), start, end)
            _, end_max, end_exact = bound_range(term.get("until"), start, end)
            lo, hi = max(start, begin_min), min(end, end_max)
            gaps = missing_intervals(ready.get(person_id, []), lo, hi) if lo < hi else []
            terms.append({"term": term.get("id"), "person_id": person_id,
                          "identity_known": person_id in known,
                          "date_precision": "exact" if begin_exact and end_exact else "uncertain",
                          "status": "outside_window" if lo >= hi else "unknown_identity" if person_id not in known else "covered" if not gaps else "missing_art",
                          "missing_eras": [{"from": a.isoformat(), "to": b.isoformat()} for a, b in gaps]})
        parties.append({"nation": party.get("nation"), "party": party.get("party"),
                        "historical_coverage": party.get("coverage", "gap"), "kind": party.get("kind"),
                        "declared_gaps": party.get("gaps", []), "terms": terms})
    result = {"version": SCHEMA_VERSION, "window": {"from": first, "to": until},
              "manifest_valid": checked["valid"], "errors": checked["errors"], "warnings": checked["warnings"],
              "counts": {"known_people": len(known), "ready": sum(p["status"] == "ready" for p in people),
                         "partial": sum(p["status"] == "partial" for p in people),
                         "missing": sum(p["status"] == "missing" for p in people),
                         "validated_portraits": sum(len(p) for p in ready.values()),
                         "party_records": len(parties),
                         "nations_with_party_records": len({p["nation"] for p in parties}),
                         "parties_verified": sum(p["historical_coverage"] == "verified" for p in parties),
                         "parties_partial": sum(p["historical_coverage"] == "partial" for p in parties),
                         "parties_with_history_gaps": sum(p["historical_coverage"] == "gap" or bool(p["declared_gaps"]) for p in parties),
                         "historical_terms": sum(len(p["terms"]) for p in parties),
                         "terms_with_unknown_identity": sum(not t["identity_known"] for p in parties for t in p["terms"])}, "people": people, "parties": parties,
              "note": "Artwork coverage is separate from historical candidate eligibility. Missing or uncertain history remains explicit."}
    if executives is not None:
        result["executives"] = executive_links(executives, known, executive_map)
    return result


def prompt_for_job(job: dict) -> str:
    return f"""# Person portrait job — {job['name']}

Job: `{job['id']}`
Person ID: `{job['person_id']}`
Requested visual era: {job['from']} to {job['to']} (exclusive end)
Status: pending; this document is not evidence that an image exists or is reviewed.

## Identity sources

{chr(10).join('- ' + source for source in job['identity_sources']) or '- Missing: research and author the identity before rendering.'}

## Prompt draft

Create one small full-body CARTOON character avatar of {job['name']}, identified by the authored
person ID {job['person_id']}. Use an observed, rights-reviewed likeness if
available. Preserve that person's recognizable identity through simplified
facial shapes. Match the current fixed 2D cartoon style anchor:
{STYLE_ANCHOR}. Inspect that reference first;
it supplies style and composition only, never this person's face or costume.
Use bold dark outlines, simplified expressive facial shapes, graphic cel shading,
and a slightly enlarged head with adult proportions of about 5.5 heads tall.
Show the entire figure including hands and feet in a compact portrait frame
against an intentionally opaque, quiet dark teal background. Transparency is
not required. Do not paint a checkerboard or photographic setting. Keep coherent
period clothing and a readable silhouette. Avoid photorealistic facial texture,
realistic painterly rendering or a miniature photographic person. A source
headshot supplies identity, not the final composition. No readable text,
invented insignia or other people.
Establish the actual depiction date and suitable age before rendering;
the requested era is a coverage need, not permission to invent appearance.

## Completion record

Use the built-in image_gen tool for generated artwork. Save the exact final
submitted prompt, tool output/generation record, source identity and its rights,
the actual file/hash/dimensions, crop and reviewed visual era. Explicitly review
identity, likeness, era and image quality before adding a ready manifest entry.
Record `style: cartoon`, `method: generated`, `status: illustrated-likeness`,
`composition: full-body`, and `background_mode: opaque`. Bind the artwork only
to `identity_source.person_id: {job['person_id']}` and the reviewed half-open
appearance era; never substitute a national figure or another person's image.
The job's portrait template is incomplete: its review flags start false and
may become true only after the corresponding checks actually take place.
Record the actual reviewer and exact review date. If Codex performs the checks,
identify the reviewer as Codex; do not imply human review or user approval.
An illustrated likeness is an artistic interpretation, not an official photo.
Leave this job pending until the physical file and all required provenance exist.
"""


def build_jobs(manifest: dict, registry: dict, repo_root: Path = ROOT,
               first: str = "1990-01-01", until: str = "2027-01-01") -> dict:
    coverage = coverage_report(manifest, registry, repo_root, first, until)
    known = people_from_registry(registry)
    jobs = []
    for person in coverage["people"]:
        record = known[person["person_id"]]
        sources = record.get("identity_sources", record.get("sources", []))
        sources = sources if isinstance(sources, list) and all(web_url(s) for s in sources) else []
        for gap in person["missing_eras"]:
            content = {"version": JOB_VERSION, "person_id": person["person_id"], "name": person["name"],
                       "from": gap["from"], "to": gap["to"], "identity_sources": sources,
                       "style_anchor": STYLE_ANCHOR,
                       "portrait_template": {
                           "from": gap["from"], "to": gap["to"], "style": "cartoon", "method": "generated",
                           "status": "illustrated-likeness", "composition": "full-body", "background_mode": "opaque",
                           "identity_source": {"person_id": person["person_id"], "kind": None, "source_url": None},
                           "review": {"identity": False, "likeness": False, "era": False, "visual": False,
                                      "reviewer": None, "reviewed_at": None},
                       }}
            jobs.append(dict(content, id=f"{person['person_id']}-{digest(content)[:12]}",
                             status="needs_source_and_visual_review" if sources else "blocked_missing_identity_sources"))
    return {"version": SCHEMA_VERSION, "job_version": JOB_VERSION, "registry_sha256": digest(registry),
            "manifest_sha256": digest(manifest), "window": coverage["window"],
            "manifest_valid": coverage["manifest_valid"], "errors": coverage["errors"],
            "counts": dict(coverage["counts"], pending_jobs=len(jobs), rendered_by_this_tool=0), "jobs": jobs}


def write_new_or_identical(path: Path, text: str) -> None:
    if path.exists():
        if path.read_text(encoding="utf-8") != text:
            raise ValueError(f"refusing to overwrite a different versioned document: {path}")
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8", newline="\n")


def self_test() -> int:
    """Read-only regression checks using the repo's already-audited source art."""
    from copy import deepcopy
    import unittest

    figures = read_json(ROOT / "spheres-web/data/nation_figures.json")["nations"]
    figure = figures["USSR"]
    source = figure["portrait"]
    person_id = "portrait-test-gorbachev"
    identity_url = "https://www.wikidata.org/wiki/" + figure["wikidata"]
    registry = {"version": 1, "people": [{"id": person_id, "name": figure["figure"], "sources": [identity_url]}], "parties": []}
    known = people_from_registry(registry)
    reference = {"person_id": person_id, "kind": "observed_portrait", "source_url": source["source_url"],
                 "asset": "spheres-web/ui/portraits/" + source["asset"], "sha256": source["sha256"],
                 "credit": source["credit"], "license": source["license"], "license_url": source["license_url"]}
    portrait = {"from": "1990-01-01", "to": "2000-01-01", "method": "archival", **{k: reference[k] for k in ("asset", "sha256", "source_url", "credit", "license", "license_url")},
                "creator": source["creator"], "rights_statement": source["rights_statement"], "identity_source": reference,
                "review": {"identity": True, "likeness": True, "era": True, "visual": True, "reviewed_at": "2026-09-07", "reviewer": "synthetic validator test"}}
    base = {"version": 1, "people": {person_id: {"name": figure["figure"], "identity_sources": [identity_url], "portraits": [portrait]}}}

    class Checks(unittest.TestCase):
        def reject(self, change):
            value = deepcopy(base)
            change(value["people"][person_id]["portraits"][0])
            result = validate_manifest(value, ROOT, known)
            self.assertFalse(result["valid"], result)
            self.assertEqual(result["ready"], {})

        def test_exact_file_and_identity(self):
            self.assertTrue(validate_manifest(base, ROOT, known)["valid"])

        def test_no_substitution_on_unknown_person_or_era(self):
            self.assertIsNotNone(select_portrait(base, person_id, "1990-01-01", ROOT, known))
            self.assertIsNone(select_portrait(base, person_id, "2000-01-01", ROOT, known))
            self.assertIsNone(select_portrait(base, "USSR", "1990-01-01", ROOT, known))

        def test_wrong_hash_and_missing_file(self):
            self.reject(lambda p: p.update(sha256="0" * 64))
            self.reject(lambda p: p.update(asset="spheres-web/ui/person-portraits/not-rendered.png"))

        def test_path_escape_and_unapproved_folder(self):
            self.reject(lambda p: p.update(asset="../outside.png"))
            self.reject(lambda p: p.update(asset="spheres-web/ui/government-art/council-v1.png"))

        def test_era_and_crop_constraints(self):
            self.reject(lambda p: p.update(to="1990-01-01"))
            self.reject(lambda p: p.update(crop={"x": 0.8, "y": 0, "width": 0.5, "height": 1}))
            self.reject(lambda p: p.update(crop={"x": float("nan"), "y": 0, "width": 1, "height": 1}))

        def test_no_metadata_auto_approval(self):
            self.reject(lambda p: p.pop("review"))
            self.reject(lambda p: p["review"].update(likeness="true"))
            self.reject(lambda p: p["identity_source"].update(person_id="another-person"))

        def test_unknown_person_and_wrong_name(self):
            self.assertFalse(validate_manifest(base, ROOT, {})["valid"])
            value = deepcopy(base)
            value["people"][person_id]["name"] = "Abraham Lincoln"
            self.assertFalse(validate_manifest(value, ROOT, known)["valid"])

        def test_same_asset_cannot_be_two_people(self):
            value = deepcopy(base)
            value["people"]["other-person"] = deepcopy(value["people"][person_id])
            value["people"]["other-person"]["portraits"][0]["identity_source"]["person_id"] = "other-person"
            self.assertFalse(validate_manifest(value, ROOT)["valid"])

        def test_overlapping_eras_refuse_selection(self):
            value = deepcopy(base)
            value["people"][person_id]["portraits"].append(deepcopy(portrait))
            self.assertFalse(validate_manifest(value, ROOT, known)["valid"])
            self.assertIsNone(select_portrait(value, person_id, "1995-01-01", ROOT, known))

        def test_archival_license_and_observation(self):
            self.reject(lambda p: p.update(license="CC BY-NC 4.0"))
            self.reject(lambda p: p["identity_source"].update(kind="authored_identity", source_url=identity_url))

        def test_generated_requires_builtin_and_actual_prompt(self):
            value = deepcopy(base)
            p = value["people"][person_id]["portraits"][0]
            p.update(method="generated", asset="spheres-web/ui/leader-art/" + figure["leader_art"]["asset"],
                     sha256=figure["leader_art"]["sha256"], generator=BUILTIN_GENERATOR, license="generated",
                     generated_at="2026-09-07", generation_record="synthetic validator fixture",
                     prompt_record=figure["leader_art"]["prompt_record"])
            self.assertTrue(validate_manifest(value, ROOT, known)["valid"])
            p["generator"] = "inferred from filename"
            self.assertFalse(validate_manifest(value, ROOT, known)["valid"])
            p["generator"] = BUILTIN_GENERATOR
            p["prompt_record"] = "tools/avatars/prompts/nonexistent-test-prompt.md"
            self.assertFalse(validate_manifest(value, ROOT, known)["valid"])

        def test_coverage_keeps_partial_and_missing_distinct(self):
            report = coverage_report(base, registry, ROOT)
            self.assertEqual(report["counts"]["partial"], 1)
            self.assertEqual(report["people"][0]["missing_eras"], [{"from": "2000-01-01", "to": "2027-01-01"}])
            report = coverage_report({"version": 1, "people": {}}, registry, ROOT)
            self.assertEqual(report["counts"]["missing"], 1)
            self.assertEqual(report["counts"]["ready"], 0)

        def test_jobs_are_deterministic_and_never_rendered(self):
            a = build_jobs(base, registry, ROOT)
            self.assertEqual(a, build_jobs(base, registry, ROOT))
            self.assertEqual(a["counts"]["rendered_by_this_tool"], 0)
            self.assertEqual(a["jobs"][0]["from"], "2000-01-01")
            self.assertIn("not evidence that an image exists", prompt_for_job(a["jobs"][0]))
            self.assertIn("small full-body CARTOON character avatar", prompt_for_job(a["jobs"][0]))
            self.assertIn("intentionally opaque, quiet dark teal background", prompt_for_job(a["jobs"][0]))
            self.assertIn(STYLE_ANCHOR, prompt_for_job(a["jobs"][0]))
            self.assertIn("do not imply human review or user approval", prompt_for_job(a["jobs"][0]))
            self.assertEqual(a["job_version"], "person-cartoon-v4")
            template = a["jobs"][0]["portrait_template"]
            self.assertEqual((template["style"], template["method"], template["status"]),
                             ("cartoon", "generated", "illustrated-likeness"))
            self.assertEqual(template["identity_source"]["person_id"], person_id)
            self.assertEqual((template["from"], template["to"]), (a["jobs"][0]["from"], a["jobs"][0]["to"]))
            self.assertTrue(all(template["review"][field] is False for field in ("identity", "likeness", "era", "visual")))
            self.assertIsNone(template["review"]["reviewer"])
            self.assertIsNone(template["review"]["reviewed_at"])

        def test_transparency_is_checked_from_pixels(self):
            usa = figures["USA"]["leader_art"]["asset"]
            self.assertTrue(has_transparent_alpha(ROOT / "spheres-web/ui/leader-art" / usa))
            self.assertFalse(has_transparent_alpha(ROOT / "spheres-web/ui/portraits" / source["asset"]))
            self.reject(lambda p: p.update(background_mode="transparent"))

        def test_executives_never_match_by_name(self):
            row = {"nation": "USSR", "name": figure["figure"], "office": "President", "also": []}
            self.assertEqual(len(executive_links({"rows": [row]}, known)["unresolved"]), 1)
            mapping = {"entries": [{**row, "person_id": person_id}]}
            self.assertEqual(executive_links({"rows": [row]}, known, mapping)["resolved"][0]["person_id"], person_id)

        def test_uncertain_dates_are_not_promoted_to_exact(self):
            value = deepcopy(registry)
            value["parties"] = [{"nation": "USSR", "party": "su_cpsu", "kind": "party", "coverage": "partial", "terms": [{"id": "test-term", "person": person_id, "from": {"kind": "year", "value": "1990"}, "until": {"kind": "day", "value": "1991-01-01"}}], "gaps": [{"reason": "not yet sourced"}]}]
            party = coverage_report(base, value, ROOT)["parties"][0]
            self.assertEqual(party["terms"][0]["date_precision"], "uncertain")
            self.assertEqual(party["historical_coverage"], "partial")
            self.assertEqual(len(party["declared_gaps"]), 1)

    result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Checks))
    return 0 if result.wasSuccessful() else 1


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("validate", "coverage", "jobs", "self-test"))
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--registry", type=Path, default=DEFAULT_REGISTRY)
    parser.add_argument("--repo", type=Path, default=ROOT)
    parser.add_argument("--from", dest="first", default="1990-01-01")
    parser.add_argument("--to", dest="until", default="2027-01-01", help="exclusive end")
    parser.add_argument("--executives", type=Path)
    parser.add_argument("--executive-map", type=Path)
    parser.add_argument("--out", type=Path, help="write a new versioned JSON document; otherwise print")
    parser.add_argument("--prompts-dir", type=Path, help="jobs only: write pending prompt drafts, never images")
    args = parser.parse_args(argv)
    try:
        if args.command == "self-test":
            return self_test()
        registry = read_json(args.registry)
        manifest = read_json(args.manifest) if args.manifest.is_file() else {"version": 1, "people": {}}
        if args.command == "validate":
            if not args.manifest.is_file():
                raise ValueError(f"portrait manifest does not exist: {args.manifest}")
            result = validate_manifest(manifest, args.repo, people_from_registry(registry))
        elif args.command == "coverage":
            result = coverage_report(manifest, registry, args.repo, args.first, args.until,
                                     read_json(args.executives) if args.executives else None,
                                     read_json(args.executive_map) if args.executive_map else None)
        else:
            result = build_jobs(manifest, registry, args.repo, args.first, args.until)
            if args.executives:
                result["executives"] = executive_links(read_json(args.executives), people_from_registry(registry),
                                                        read_json(args.executive_map) if args.executive_map else None)
            if args.prompts_dir:
                for job in result["jobs"]:
                    write_new_or_identical(args.prompts_dir / f"{job['id']}.md", prompt_for_job(job))
        text = json.dumps(result, ensure_ascii=False, indent=2) + "\n"
        if args.out:
            write_new_or_identical(args.out, text)
            print(f"Wrote {args.command} document: {args.out}")
        else:
            sys.stdout.write(text)
        return 0 if result.get("valid", result.get("manifest_valid", True)) else 1
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(f"person portrait pipeline: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    # Authored native names must also survive redirected Windows console output.
    for stream in (sys.stdout, sys.stderr):
        if hasattr(stream, "reconfigure"):
            stream.reconfigure(encoding="utf-8")
    raise SystemExit(main())
