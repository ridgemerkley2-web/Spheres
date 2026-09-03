#!/usr/bin/env python3
"""Fetch auditable, freely licensed historical-figure portraits from Commons.

The curation decision (which person represents which NationId) lives in
``spheres-web/data/nation_figures.json``. This script only resolves that exact
name through Wikidata, accepts its declared P18 image when Wikimedia Commons
supplies explicit free-license metadata, and generates the Rust byte lookup.
It never substitutes a lookalike or a merely similar search result.
"""

from __future__ import annotations

import argparse
import hashlib
import html
import io
import json
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from datetime import date
from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "spheres-web" / "data" / "nation_figures.json"
PORTRAIT_DIR = ROOT / "spheres-web" / "ui" / "portraits"
LEADER_ART_DIR = ROOT / "spheres-web" / "ui" / "leader-art"
RUST_TABLE = ROOT / "spheres-web" / "src" / "portrait_assets.rs"
USER_AGENT = "SPHERES-game-avatar-builder/1.0 (local development; Wikimedia Commons client)"
APPROVED_LICENSES = frozenset(
    {
        "public domain",
        "cc0",
        "cc0 1.0",
        "cc by 1.0",
        "cc by 2.0",
        "cc by 2.5",
        "cc by 3.0",
        "cc by 4.0",
        "cc by-sa 1.0",
        "cc by-sa 2.0",
        "cc by-sa 2.5",
        "cc by-sa 3.0",
        "cc by-sa 4.0",
    }
)
NON_DEPICTION_TITLE_MARKERS = (
    "signature",
    "autograph",
    "handwriting",
    "wordmark",
    "logo",
    "coat of arms",
    "coats of arms",
    "family crest",
    "national emblem",
    "seal of",
    "flag of",
)
CURATED_NON_DEPICTION_TITLES = {
    "file:amos ferguson gallery at the national art gallery of the bahamas.jpg": (
        "gallery installation rather than a likeness of Amos Ferguson"
    ),
    "file:mystics in a garden.jpg": (
        "book illustration not identified as a likeness of Ali-Shir Nava'i"
    ),
    "file:tj-tajik writers union building, dushanbe (9).jpg": (
        "two-subject building facade does not identify Mirzo Tursunzoda at avatar size"
    ),
}
MIME_EXTENSIONS = {
    "image/jpeg": ".jpg",
    "image/png": ".png",
    "image/webp": ".webp",
    "image/gif": ".gif",
}
THROTTLED_HOSTS: set[str] = set()


class RemoteThrottled(RuntimeError):
    """Raised after a host's first 429 so the roster can fall back promptly."""


def guard_host(url: str) -> str:
    host = urllib.parse.urlsplit(url).netloc
    if host in THROTTLED_HOSTS:
        raise RemoteThrottled(f"{host} disabled for this run after HTTP 429")
    return host


def trip_circuit(host: str, error: urllib.error.HTTPError) -> None:
    THROTTLED_HOSTS.add(host)
    retry_after = error.headers.get("Retry-After") or "not supplied"
    print(
        f"[circuit] {host}: HTTP 429 (Retry-After={retry_after}); "
        "disabling host for this run",
        flush=True,
    )


def retry_wait(error: BaseException, attempt: int) -> float:
    """Short bounded retry for transient failures other than HTTP 429."""
    return min(1.5 * (attempt + 1), 6.0)


def api_json(
    url: str,
    params: dict[str, str],
    attempts: int = 3,
    *,
    post: bool = False,
) -> dict:
    encoded = urllib.parse.urlencode(params)
    request_url = url if post else url + "?" + encoded
    request_body = encoded.encode("utf-8") if post else None
    host = guard_host(url)
    for attempt in range(attempts):
        headers = {"User-Agent": USER_AGENT}
        if post:
            headers["Content-Type"] = "application/x-www-form-urlencoded"
        request = urllib.request.Request(request_url, data=request_body, headers=headers)
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                return json.load(response)
        except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as error:
            if isinstance(error, urllib.error.HTTPError) and error.code == 429:
                trip_circuit(host, error)
                raise RemoteThrottled(f"{host} returned HTTP 429") from error
            if attempt + 1 == attempts:
                raise
            wait = retry_wait(error, attempt)
            status = getattr(error, "code", type(error).__name__)
            print(
                f"[retry] {host}: {status}; attempt {attempt + 1}/{attempts}, "
                f"waiting {wait:.1f}s",
                flush=True,
            )
            time.sleep(wait)
    raise AssertionError("unreachable")


def download(url: str, attempts: int = 3) -> tuple[bytes, str]:
    host = guard_host(url)
    for attempt in range(attempts):
        request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
        try:
            with urllib.request.urlopen(request, timeout=60) as response:
                return response.read(), response.headers.get_content_type()
        except (urllib.error.URLError, TimeoutError) as error:
            if isinstance(error, urllib.error.HTTPError) and error.code == 429:
                trip_circuit(host, error)
                raise RemoteThrottled(f"{host} returned HTTP 429") from error
            if attempt + 1 == attempts:
                raise
            wait = retry_wait(error, attempt)
            status = getattr(error, "code", type(error).__name__)
            print(
                f"[retry] {host}: {status}; attempt {attempt + 1}/{attempts}, "
                f"waiting {wait:.1f}s",
                flush=True,
            )
            time.sleep(wait)
    raise AssertionError("unreachable")


def text(metadata: dict, field: str) -> str:
    raw = metadata.get(field, {}).get("value", "")
    raw = html.unescape(re.sub(r"<[^>]+>", " ", raw))
    return re.sub(r"\s+", " ", raw).strip()


def chunks(values: list, size: int = 50):
    for offset in range(0, len(values), size):
        yield values[offset : offset + size]


def routed_page(
    requested_title: str,
    payload: dict,
    *,
    expected_namespace: int,
) -> tuple[dict, bool] | None:
    """Return an API-normalized page or a target reached by an explicit redirect."""
    query = payload.get("query", {})
    normalizations = {
        item.get("from", ""): item.get("to", "")
        for item in query.get("normalized", [])
        if item.get("from") and item.get("to")
    }
    redirects = {
        item.get("from", ""): item
        for item in query.get("redirects", [])
        if item.get("from") and item.get("to")
    }
    pages = {
        page.get("title", ""): page
        for page in query.get("pages", [])
        if page.get("title")
    }

    current = normalizations.get(requested_title, requested_title)
    redirected = False
    seen: set[str] = set()
    while current in redirects:
        if current in seen:
            return None
        seen.add(current)
        redirect = redirects[current]
        if redirect.get("tofragment"):
            return None
        current = redirect["to"]
        redirected = True

    page = pages.get(current)
    if not page or page.get("missing") is True or "invalid" in page:
        return None
    if page.get("ns") != expected_namespace:
        return None
    if "disambiguation" in page.get("pageprops", {}):
        return None
    if not redirected and page.get("title") != current:
        return None
    return page, redirected


def wikipedia_candidates(
    selected: list[tuple[str, dict]], pause: float
) -> dict[str, dict]:
    """Resolve curated English Wikipedia titles in batches of at most 50."""
    results: dict[str, dict] = {}
    for batch_index, batch in enumerate(chunks(selected), 1):
        titles = [nation.get("canonical_lookup") or nation.get("figure") for _, nation in batch]
        try:
            payload = api_json(
                "https://en.wikipedia.org/w/api.php",
                {
                    "action": "query",
                    "format": "json",
                    "formatversion": "2",
                    "maxlag": "5",
                    "redirects": "1",
                    "prop": "pageprops|pageimages",
                    "ppprop": "wikibase_item|disambiguation",
                    "piprop": "name|original",
                    "pilicense": "free",
                    "titles": "|".join(titles),
                },
                post=True,
            )
        except Exception as error:
            for nation_id, _ in batch:
                results[nation_id] = {"kind": "network error", "error": str(error)}
            print(
                f"[wikipedia batch {batch_index}] failed for {len(batch)} titles: {error}",
                flush=True,
            )
            time.sleep(pause)
            continue

        resolved_count = 0
        for (nation_id, _), requested_title in zip(batch, titles):
            routed = routed_page(requested_title, payload, expected_namespace=0)
            if not routed:
                results[nation_id] = {
                    "kind": "no identity",
                    "requested_title": requested_title,
                }
                continue
            page, redirected = routed
            qid = page.get("pageprops", {}).get("wikibase_item")
            if not qid:
                results[nation_id] = {
                    "kind": "no identity",
                    "requested_title": requested_title,
                    "error": "resolved page has no Wikidata item",
                }
                continue
            results[nation_id] = {
                "kind": "resolved",
                "requested_title": requested_title,
                "resolved_label": page.get("title", requested_title),
                "wikidata": qid,
                "pageimage": page.get("pageimage"),
                "redirected": redirected,
            }
            resolved_count += 1
        print(
            f"[wikipedia batch {batch_index}] resolved {resolved_count}/{len(batch)} titles",
            flush=True,
        )
        time.sleep(pause)

    qid_owners: dict[str, list[str]] = {}
    for nation_id, result in results.items():
        if result.get("kind") == "resolved":
            qid_owners.setdefault(result["wikidata"], []).append(nation_id)
    for qid, owners in qid_owners.items():
        if len(owners) > 1:
            for nation_id in owners:
                results[nation_id] = {
                    "kind": "no identity",
                    "requested_title": results[nation_id]["requested_title"],
                    "error": f"identity collision on {qid} across {', '.join(owners)}",
                }
    return results


def commons_images(filenames: list[str], pause: float) -> dict[str, dict]:
    """Fetch auditable image metadata in Commons batches of at most 50 files."""
    results: dict[str, dict] = {}
    unique_filenames = list(dict.fromkeys(filenames))
    for batch_index, batch in enumerate(chunks(unique_filenames), 1):
        requested_titles = ["File:" + filename for filename in batch]
        try:
            payload = api_json(
                "https://commons.wikimedia.org/w/api.php",
                {
                    "action": "query",
                    "format": "json",
                    "formatversion": "2",
                    "maxlag": "5",
                    "redirects": "1",
                    "prop": "imageinfo",
                    "iiprop": "url|mime|size|extmetadata",
                    "titles": "|".join(requested_titles),
                },
                post=True,
            )
        except Exception as error:
            for filename in batch:
                results[filename] = {"kind": "network error", "error": str(error)}
            print(
                f"[commons batch {batch_index}] failed for {len(batch)} files: {error}",
                flush=True,
            )
            time.sleep(pause)
            continue

        resolved_count = 0
        for filename, requested_title in zip(batch, requested_titles):
            routed = routed_page(requested_title, payload, expected_namespace=6)
            if not routed:
                results[filename] = {"kind": "not Commons"}
                continue
            page, _ = routed
            infos = page.get("imageinfo", [])
            if not infos:
                results[filename] = {"kind": "not Commons"}
                continue
            info = infos[0]
            info["page_title"] = page.get("title", requested_title)
            results[filename] = {"kind": "resolved", "info": info}
            resolved_count += 1
        print(
            f"[commons batch {batch_index}] resolved {resolved_count}/{len(batch)} files",
            flush=True,
        )
        time.sleep(pause)
    return results


def accepted_license(info: dict) -> tuple[str, str] | None:
    metadata = info.get("extmetadata", {})
    license_name = text(metadata, "LicenseShortName") or text(metadata, "UsageTerms")
    normalized_license = re.sub(r"\s+", " ", license_name).strip().casefold()
    if normalized_license not in APPROVED_LICENSES:
        return None
    return license_name, text(metadata, "LicenseUrl")


def non_depiction_reason(source_title: str) -> str | None:
    curated_reason = CURATED_NON_DEPICTION_TITLES.get(source_title.strip().casefold())
    if curated_reason:
        return curated_reason
    folded = re.sub(r"[_-]+", " ", source_title).casefold()
    folded = re.sub(r"\s+", " ", folded)
    if "no signature" in folded or "without signature" in folded:
        return None
    for marker in NON_DEPICTION_TITLE_MARKERS:
        if marker in folded:
            return marker
    return None


def as_webp(payload: bytes) -> bytes:
    """Re-encode without cropping or altering the depicted work."""
    with Image.open(io.BytesIO(payload)) as source:
        frame = source.convert("RGBA" if "A" in source.getbands() else "RGB")
        output = io.BytesIO()
        frame.save(output, format="WEBP", quality=86, method=6)
        return output.getvalue()


def atomic_write_text(path: Path, payload: str) -> None:
    """Durably replace one generated text file without exposing a partial file."""
    temporary = path.with_name(path.name + ".tmp")
    with temporary.open("w", encoding="utf-8", newline="\n") as handle:
        handle.write(payload)
        handle.flush()
        os.fsync(handle.fileno())
    os.replace(temporary, path)


def write_manifest(manifest: dict) -> None:
    atomic_write_text(MANIFEST, json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")


def write_rust_table(entries: dict[str, dict]) -> None:
    assets: dict[str, tuple[str, str]] = {}
    for nation in entries.values():
        portrait = nation.get("portrait")
        if isinstance(portrait, dict) and portrait.get("asset"):
            assets[portrait["asset"]] = (
                "image/webp",
                f'../ui/portraits/{portrait["asset"]}',
            )
        leader_art = nation.get("leader_art")
        if isinstance(leader_art, dict) and leader_art.get("asset"):
            assets[leader_art["asset"]] = (
                "image/png",
                f'../ui/leader-art/{leader_art["asset"]}',
            )

    lines = [
        "//! Generated by tools/avatars/fetch_commons_portraits.py. Do not edit by hand.",
        "",
        "pub struct PortraitAsset {",
        "    pub bytes: &'static [u8],",
        "    pub content_type: &'static str,",
        "}",
        "",
        "pub fn get(name: &str) -> Option<PortraitAsset> {",
        "    match name {",
    ]
    for filename, (mime, include_path) in sorted(assets.items()):
        escaped = filename.replace("\\", "\\\\").replace('"', '\\"')
        escaped_path = include_path.replace("\\", "\\\\").replace('"', '\\"')
        lines.extend(
            [
                f'        "{escaped}" => Some(PortraitAsset {{',
                f'            bytes: include_bytes!("{escaped_path}"),',
                f'            content_type: "{mime}",',
                "        }),",
            ]
        )
    lines.extend(
        [
            "        _ => None,",
            "    }",
            "}",
            "",
            "#[allow(dead_code)]",
            "pub const FILENAMES: &[&str] = &[",
        ]
    )
    lines.extend(f'    "{filename}",' for filename in sorted(assets))
    lines.extend(["];", ""])
    atomic_write_text(RUST_TABLE, "\n".join(lines))


def portrait_file(portrait: object) -> Path | None:
    if not isinstance(portrait, dict):
        return None
    asset = portrait.get("asset")
    if not isinstance(asset, str) or not asset or Path(asset).name != asset:
        return None
    return PORTRAIT_DIR / asset


def discard_portrait(nation: dict) -> None:
    path = portrait_file(nation.get("portrait"))
    if path and path.is_file():
        path.unlink()
    nation["portrait"] = None


def fetch_one(
    nation_id: str,
    nation: dict,
    refresh: bool,
    candidate: dict,
    image_records: dict[str, dict],
) -> str:
    existing = nation.get("portrait")
    existing_file = portrait_file(existing)
    if isinstance(existing, dict):
        reason = non_depiction_reason(str(existing.get("source_title", "")))
        if reason:
            discard_portrait(nation)
            existing = None
            existing_file = None
            nation["portrait_status"] = (
                f"Commons page image rejected as a non-depiction ({reason})"
            )

    if candidate.get("kind") == "network error":
        if not refresh and existing_file and existing_file.is_file():
            nation.pop("portrait_status", None)
            return "kept"
        nation["portrait"] = None
        nation["portrait_status"] = (
            "Wikipedia batch lookup failed: " + candidate.get("error", "unknown error")
        )
        return "network fallback"

    if candidate.get("kind") != "resolved":
        if existing_file and existing_file.is_file():
            discard_portrait(nation)
        nation.pop("wikidata", None)
        requested = candidate.get("requested_title") or nation.get("canonical_lookup")
        detail = candidate.get("error")
        nation["portrait_status"] = (
            f"Wikipedia title {requested!r} did not resolve exactly or by explicit redirect"
            + (f": {detail}" if detail else "")
        )
        return "no identity"

    qid = candidate["wikidata"]
    label = candidate["resolved_label"]
    nation["wikidata"] = qid
    if (
        not refresh
        and isinstance(existing, dict)
        and existing.get("wikidata") == qid
        and existing_file
        and existing_file.is_file()
    ):
        nation.pop("portrait_status", None)
        return "kept"
    if isinstance(existing, dict) and existing.get("wikidata") != qid:
        discard_portrait(nation)

    image_name = candidate.get("pageimage")
    if not image_name:
        nation["portrait"] = None
        nation["portrait_status"] = (
            f"Wikipedia page {label!r} ({qid}) has no page image"
        )
        return "no image"

    image_record = image_records.get(image_name, {"kind": "not Commons"})
    if image_record.get("kind") == "network error":
        nation["portrait"] = None
        nation["portrait_status"] = (
            "Commons batch lookup failed: " + image_record.get("error", "unknown error")
        )
        return "network fallback"
    if image_record.get("kind") != "resolved":
        nation["portrait"] = None
        nation["portrait_status"] = (
            f"Wikipedia page image {image_name!r} is not hosted on Wikimedia Commons"
        )
        return "not Commons"
    info = image_record["info"]
    rejection = non_depiction_reason(str(info.get("page_title", image_name)))
    if rejection:
        nation["portrait"] = None
        nation["portrait_status"] = (
            f"Commons page image rejected as a non-depiction ({rejection})"
        )
        return "non-depiction skipped"
    license_record = accepted_license(info)
    if not license_record:
        nation["portrait"] = None
        nation["portrait_status"] = "Commons image is not on the exact approved free-license allowlist"
        return "license skipped"

    source_url = info.get("descriptionurl", "")
    if not source_url:
        nation["portrait"] = None
        nation["portrait_status"] = "Commons metadata omitted a source URL"
        return "metadata skipped"
    # upload.wikimedia.org can impose a long IP-wide Retry-After on direct
    # thumbnail URLs. Commons' thumbnail endpoint serves the same
    # file through its image scaler and keeps this auditable pass moving.
    asset_url = "https://commons.wikimedia.org/w/thumb.php?" + urllib.parse.urlencode(
        {"f": image_name, "w": "768"}
    )
    payload, content_type = download(asset_url)
    if content_type.lower() not in MIME_EXTENSIONS:
        nation["portrait"] = None
        nation["portrait_status"] = f"unsupported downloaded MIME type {content_type}"
        return "format skipped"

    webp = as_webp(payload)
    digest = hashlib.sha256(webp).hexdigest()
    filename = f"{nation_id}-{digest[:12]}.webp"
    (PORTRAIT_DIR / filename).write_bytes(webp)
    if existing_file and existing_file.is_file() and existing_file.name != filename:
        existing_file.unlink()
    metadata = info.get("extmetadata", {})
    license_name, license_url = license_record
    if not license_url and "public domain" in license_name.lower():
        license_url = "https://creativecommons.org/publicdomain/mark/1.0/"
    creator = text(metadata, "Artist") or "Creator not recorded on the Commons file page"
    credit = text(metadata, "Credit") or creator
    nation["portrait"] = {
        "asset": filename,
        "wikidata": qid,
        "resolved_label": label,
        "source_title": info.get("page_title", "File:" + image_name),
        "source_url": source_url,
        "original_url": info.get("url", ""),
        "creator": creator,
        "license": license_name,
        "license_url": license_url,
        "rights_statement": f"The Wikimedia Commons file page marks this particular work as {license_name}.",
        "credit": credit,
        "sha256": digest,
        "focus_x": 0.5,
        "focus_y": 0.38,
        "retrieved": date.today().isoformat(),
        "review": (
            "automated explicit Wikipedia redirect candidate"
            if candidate.get("redirected")
            else "automated exact Wikipedia-title candidate"
        ),
    }
    nation.pop("portrait_status", None)
    return "fetched"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--refresh", action="store_true", help="re-resolve portraits already present")
    parser.add_argument("--only", action="append", default=[], help="fetch one NationId (repeatable)")
    parser.add_argument("--delay", type=float, default=0.08, help="polite pause between identities")
    args = parser.parse_args()

    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    nations = manifest.get("nations", {})
    wanted = set(args.only)
    unknown = sorted(wanted - set(nations))
    if unknown:
        print("Unknown NationId(s): " + ", ".join(unknown), file=sys.stderr)
        return 2

    PORTRAIT_DIR.mkdir(parents=True, exist_ok=True)
    counts: dict[str, int] = {}
    selected = [(key, value) for key, value in nations.items() if not wanted or key in wanted]
    pause = max(args.delay, 0.0)
    candidates = wikipedia_candidates(selected, pause)
    pageimages = [
        candidate["pageimage"]
        for candidate in candidates.values()
        if candidate.get("kind") == "resolved" and candidate.get("pageimage")
    ]
    image_records = commons_images(pageimages, pause)

    for index, (nation_id, nation) in enumerate(selected, 1):
        try:
            candidate = candidates.get(
                nation_id,
                {"kind": "network error", "error": "batch produced no result"},
            )
            result = fetch_one(
                nation_id,
                nation,
                args.refresh,
                candidate,
                image_records,
            )
        except RemoteThrottled as error:
            result = "throttled fallback"
            nation["portrait"] = None
            nation["portrait_status"] = f"fetch deferred: {error}"
        except Exception as error:  # keep the rest of a large roster moving
            result = "network error"
            nation["portrait_status"] = f"fetch failed: {error}"
        counts[result] = counts.get(result, 0) + 1
        # A long Commons pass must survive interruption without losing every
        # resolved identity. Both generated files are replaced atomically, so
        # the server never observes a half-written manifest or Rust table.
        write_manifest(manifest)
        write_rust_table(nations)
        print(f"[{index:03}/{len(selected):03}] {nation_id}: {result}", flush=True)
        time.sleep(pause)

    write_manifest(manifest)
    write_rust_table(nations)
    print("\n" + ", ".join(f"{key}: {value}" for key, value in sorted(counts.items())), flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
