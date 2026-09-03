#!/usr/bin/env python3
"""Audit the historical-figure selector assets against the sim roster.

Run from the repository root with:

    python tools/avatars/check_assets.py

The script intentionally uses only the Python standard library.  NationId codes
are parsed from the Rust ROSTER itself; no second hand-written country list is
allowed to become an accidental source of truth.
"""

from __future__ import annotations

import ast
import hashlib
import json
import math
import re
import struct
import sys
import unicodedata
import xml.etree.ElementTree as ET
import zlib
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable
from urllib.parse import urlparse


REPO = Path(__file__).resolve().parents[2]
ROSTER_FILE = REPO / "spheres-sim" / "src" / "nations.rs"
MANIFEST_FILE = REPO / "spheres-web" / "data" / "nation_figures.json"
FLAGS_FILE = REPO / "spheres-web" / "ui" / "nation-flags-v1.svg"
PORTRAIT_DIR = REPO / "spheres-web" / "ui" / "portraits"
LEADER_ART_DIR = REPO / "spheres-web" / "ui" / "leader-art"
LEADER_ART_PROMPT_DIR = REPO / "tools" / "avatars" / "prompts"

EXPECTED_ROSTER_COUNT = 160
CONFIDENCE_VALUES = {"high", "medium", "low"}
PORTRAIT_FIELDS = (
    "asset",
    "source_url",
    "source_title",
    "creator",
    "license",
    "license_url",
    "rights_statement",
    "credit",
    "sha256",
    "focus_x",
    "focus_y",
)
PORTRAIT_ASSET_RE = re.compile(
    r"^(?P<nation>[A-Za-z][A-Za-z0-9]*)-(?P<digest>[0-9a-f]{6,16})\.webp$"
)
LEADER_ART_FIELDS = (
    "asset",
    "identity_source_asset",
    "style",
    "background_mode",
    "generated",
    "generator",
    "credit",
    "prompt_record",
    "sha256",
)
LEADER_ART_ASSET_RE = re.compile(
    r"^(?P<nation>[A-Za-z][A-Za-z0-9]*)-leader-(?P<digest>[0-9a-f]{12})\.png$"
)
SHA256_RE = re.compile(r"^[0-9a-fA-F]{64}$")
PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"
MAX_LEADER_ART_PIXELS = 16_777_216
HEX_COLOR_RE = re.compile(
    r"#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})"
)
URL_TOKEN_RE = re.compile(
    r"url\(\s*(?P<quote>['\"]?)(?P<target>[^)'\"\s]+)(?P=quote)\s*\)",
    re.IGNORECASE,
)
STYLE_COLOR_RE = re.compile(
    r"(?:^|;)\s*(?:fill|stroke|color|stop-color|flood-color|lighting-color)"
    r"\s*:\s*(?P<color>#[^;\s]+)",
    re.IGNORECASE,
)
SVG_COLOR_ATTRIBUTES = {
    "color",
    "fill",
    "flood-color",
    "lighting-color",
    "stop-color",
    "stroke",
}
FORBIDDEN_SVG_ELEMENTS = {"foreignObject", "script", "style"}


@dataclass(frozen=True)
class RosterRow:
    code: str
    display_name: str
    region: str
    start_1990: bool
    line: int


class ParseError(RuntimeError):
    """The authoritative Rust table could not be read safely."""


def strip_rust_comments(source: str) -> str:
    """Replace Rust comments with spaces while preserving offsets/newlines.

    ROSTER comments contain quoted text, commas and even the word ``row``.  A
    regular expression over the unstripped file therefore produces believable
    but wrong rows.  Rust block comments may nest, so the small lexer below
    handles that too.
    """

    out = list(source)
    i = 0
    state = "normal"
    block_depth = 0
    escaped = False
    while i < len(source):
        ch = source[i]
        nxt = source[i + 1] if i + 1 < len(source) else ""

        if state == "line_comment":
            if ch == "\n":
                state = "normal"
            else:
                out[i] = " "
            i += 1
            continue

        if state == "block_comment":
            if ch == "/" and nxt == "*":
                out[i] = out[i + 1] = " "
                block_depth += 1
                i += 2
                continue
            if ch == "*" and nxt == "/":
                out[i] = out[i + 1] = " "
                block_depth -= 1
                i += 2
                if block_depth == 0:
                    state = "normal"
                continue
            if ch != "\n":
                out[i] = " "
            i += 1
            continue

        if state in {"string", "char"}:
            quote = '"' if state == "string" else "'"
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == quote:
                state = "normal"
            i += 1
            continue

        if ch == "/" and nxt == "/":
            out[i] = out[i + 1] = " "
            state = "line_comment"
            i += 2
        elif ch == "/" and nxt == "*":
            out[i] = out[i + 1] = " "
            state = "block_comment"
            block_depth = 1
            i += 2
        elif ch == '"':
            state = "string"
            i += 1
        elif ch == "'":
            state = "char"
            i += 1
        else:
            i += 1

    if state == "block_comment":
        raise ParseError("unterminated block comment in nations.rs")
    return "".join(out)


def find_matching(text: str, start: int, opening: str, closing: str) -> int:
    if start >= len(text) or text[start] != opening:
        raise ParseError(f"expected {opening!r} at offset {start}")
    depth = 0
    quote: str | None = None
    escaped = False
    for i in range(start, len(text)):
        ch = text[i]
        if quote is not None:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == quote:
                quote = None
            continue
        if ch in {'"', "'"}:
            quote = ch
        elif ch == opening:
            depth += 1
        elif ch == closing:
            depth -= 1
            if depth == 0:
                return i
    raise ParseError(f"unterminated {opening!r} beginning at offset {start}")


def split_top_level(arguments: str) -> list[str]:
    parts: list[str] = []
    start = 0
    stack: list[str] = []
    pairs = {")": "(", "]": "[", "}": "{"}
    quote: str | None = None
    escaped = False

    for i, ch in enumerate(arguments):
        if quote is not None:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == quote:
                quote = None
            continue
        if ch in {'"', "'"}:
            quote = ch
        elif ch in "([{":
            stack.append(ch)
        elif ch in ")]}":
            if not stack or stack.pop() != pairs[ch]:
                raise ParseError("unbalanced delimiter in a ROSTER row")
        elif ch == "," and not stack:
            parts.append(arguments[start:i].strip())
            start = i + 1
    parts.append(arguments[start:].strip())
    return parts


def rust_string(value: str, *, line: int, field: str) -> str:
    try:
        parsed = ast.literal_eval(value)
    except (SyntaxError, ValueError) as exc:
        raise ParseError(f"nations.rs:{line}: invalid {field} string: {exc}") from exc
    if not isinstance(parsed, str):
        raise ParseError(f"nations.rs:{line}: {field} is not a string literal")
    return parsed


def parse_roster(path: Path) -> list[RosterRow]:
    raw = path.read_text(encoding="utf-8")
    clean = strip_rust_comments(raw)
    marker = clean.find("pub const ROSTER")
    if marker < 0:
        raise ParseError("nations.rs has no `pub const ROSTER`")
    initializer = clean.find("=", marker)
    if initializer < 0:
        raise ParseError("ROSTER has no initializer")
    array_start = clean.find("&[", initializer)
    if array_start < 0:
        raise ParseError("ROSTER has no array body")
    bracket = array_start + 1
    array_end = find_matching(clean, bracket, "[", "]")
    body = clean[bracket + 1 : array_end]

    rows: list[RosterRow] = []
    cursor = 0
    call_re = re.compile(r"\brow\s*\(")
    while True:
        match = call_re.search(body, cursor)
        if match is None:
            break
        open_paren = body.find("(", match.start())
        close_paren = find_matching(body, open_paren, "(", ")")
        absolute = bracket + 1 + match.start()
        line = raw.count("\n", 0, absolute) + 1
        args = split_top_level(body[open_paren + 1 : close_paren])
        if len(args) != 9:
            raise ParseError(
                f"nations.rs:{line}: row has {len(args)} fields; expected 9"
            )
        if args[6] not in {"true", "false"}:
            raise ParseError(
                f"nations.rs:{line}: start_1990 must be true or false, got {args[6]!r}"
            )
        rows.append(
            RosterRow(
                code=rust_string(args[0], line=line, field="NationId code"),
                display_name=rust_string(args[1], line=line, field="display name"),
                region=rust_string(args[3], line=line, field="region"),
                start_1990=args[6] == "true",
                line=line,
            )
        )
        cursor = close_paren + 1

    if not rows:
        raise ParseError("ROSTER contained no row(...) calls")
    return rows


def nonempty_string(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def normalized_name(value: str) -> str:
    value = unicodedata.normalize("NFKC", value)
    return " ".join(value.casefold().split())


def valid_web_url(value: Any) -> bool:
    if not nonempty_string(value):
        return False
    parsed = urlparse(value)
    return parsed.scheme in {"http", "https"} and bool(parsed.netloc)


def freely_licensed(value: str) -> bool:
    """Accept explicit public-domain or permissive/free-culture identifiers.

    NC, ND, fair-use and merely "copyrighted" records are deliberately not
    accepted.  Keep the normalized list narrow and add a reviewed identifier
    here and in README.md when the project admits another license.
    """

    token = re.sub(r"[\s_]", "-", value.strip().upper())
    token = re.sub(r"-+", "-", token)
    if token in {
        "PD",
        "PUBLIC-DOMAIN",
        "PUBLIC-DOMAIN-MARK",
        "PDM-1.0",
        "CC0",
        "CC0-1.0",
        "MIT",
        "MIT-0",
    }:
        return True
    return bool(
        re.fullmatch(r"CC-BY(?:-SA)?-(?:1\.0|2\.0|2\.5|3\.0|4\.0)", token)
    )


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def validate_portrait(
    nation_id: str, portrait: Any, errors: list[str]
) -> bool:
    """Validate one optional local portrait; return True when one is present."""

    where = f"nations.{nation_id}.portrait"
    if portrait is None:
        return False
    if not isinstance(portrait, dict):
        errors.append(f"{where} must be null or an object")
        return False

    for field in PORTRAIT_FIELDS:
        if field not in portrait:
            errors.append(f"{where} is missing {field!r}")

    asset = portrait.get("asset")
    if not nonempty_string(asset):
        errors.append(f"{where}.asset must be a nonempty basename")
        asset_match = None
    else:
        asset_match = PORTRAIT_ASSET_RE.fullmatch(asset)
        if asset_match is None:
            errors.append(
                f"{where}.asset must match "
                "<NationId>-<6..16 hex>.webp and contain no directory"
            )
        elif asset_match.group("nation") != nation_id:
            errors.append(
                f"{where}.asset belongs to {asset_match.group('nation')!r}, "
                f"not {nation_id!r}"
            )

    for field in (
        "source_title",
        "creator",
        "license",
        "rights_statement",
        "credit",
    ):
        if not nonempty_string(portrait.get(field)):
            errors.append(f"{where}.{field} must be a nonempty string")
    for field in ("source_url", "license_url"):
        if not valid_web_url(portrait.get(field)):
            errors.append(f"{where}.{field} must be an absolute http(s) URL")

    license_value = portrait.get("license")
    if nonempty_string(license_value) and not freely_licensed(license_value):
        errors.append(
            f"{where}.license {license_value!r} is not an approved free license "
            "or public-domain identifier"
        )

    recorded_hash = portrait.get("sha256")
    if not nonempty_string(recorded_hash) or not SHA256_RE.fullmatch(recorded_hash):
        errors.append(f"{where}.sha256 must contain exactly 64 hexadecimal digits")
    elif asset_match is not None:
        short_hash = asset_match.group("digest")
        if recorded_hash.lower()[: len(short_hash)] != short_hash:
            errors.append(
                f"{where}.asset digest {short_hash!r} is not the leading part "
                "of its recorded sha256"
            )

    for field in ("focus_x", "focus_y"):
        value = portrait.get(field)
        if (
            isinstance(value, bool)
            or not isinstance(value, (int, float))
            or not math.isfinite(float(value))
            or not 0.0 <= float(value) <= 1.0
        ):
            errors.append(f"{where}.{field} must be a finite number from 0 to 1")

    if asset_match is not None:
        portrait_path = PORTRAIT_DIR / asset
        if not portrait_path.is_file():
            errors.append(
                f"{where}.asset references missing file "
                f"{portrait_path.relative_to(REPO)}"
            )
        else:
            with portrait_path.open("rb") as handle:
                header = handle.read(12)
            if not (header.startswith(b"RIFF") and header[8:12] == b"WEBP"):
                errors.append(f"{portrait_path.relative_to(REPO)} is not a WebP file")
            if nonempty_string(recorded_hash) and SHA256_RE.fullmatch(recorded_hash):
                actual_hash = file_sha256(portrait_path)
                if actual_hash.lower() != recorded_hash.lower():
                    errors.append(
                        f"{where}.sha256 is {recorded_hash.lower()}, but the file is "
                        f"{actual_hash}"
                    )
    return True


def paeth_predictor(left: int, above: int, upper_left: int) -> int:
    estimate = left + above - upper_left
    left_distance = abs(estimate - left)
    above_distance = abs(estimate - above)
    upper_left_distance = abs(estimate - upper_left)
    if left_distance <= above_distance and left_distance <= upper_left_distance:
        return left
    if above_distance <= upper_left_distance:
        return above
    return upper_left


def validate_rgba_png(
    path: Path,
    where: str,
    errors: list[str],
    *,
    allow_rgb: bool = False,
) -> None:
    """Validate a small, non-interlaced RGBA PNG and its real alpha content.

    Checking only the filename or MIME signature would let an opaque RGB render
    silently regress the transparent character cutouts.  The standard-library
    decoder below reverses PNG row filters just far enough to inspect alpha; it
    deliberately rejects interlaced artwork so this audit remains deterministic
    and dependency-free.
    """

    try:
        payload = path.read_bytes()
    except OSError as exc:
        errors.append(f"{where}.asset could not be read: {exc}")
        return

    try:
        relative = path.relative_to(REPO)
    except ValueError:
        relative = path
    if not payload.startswith(PNG_SIGNATURE):
        errors.append(f"{relative} is not a PNG file")
        return

    offset = len(PNG_SIGNATURE)
    ihdr: tuple[int, int, int, int, int, int, int] | None = None
    compressed = bytearray()
    saw_iend = False
    chunk_index = 0
    while offset < len(payload):
        if offset + 12 > len(payload):
            errors.append(f"{relative} has a truncated PNG chunk")
            return
        length = struct.unpack(">I", payload[offset : offset + 4])[0]
        chunk_type = payload[offset + 4 : offset + 8]
        data_start = offset + 8
        data_end = data_start + length
        crc_end = data_end + 4
        if crc_end > len(payload):
            errors.append(f"{relative} has a truncated PNG chunk")
            return
        chunk_data = payload[data_start:data_end]
        recorded_crc = struct.unpack(">I", payload[data_end:crc_end])[0]
        actual_crc = zlib.crc32(chunk_type)
        actual_crc = zlib.crc32(chunk_data, actual_crc) & 0xFFFFFFFF
        if actual_crc != recorded_crc:
            errors.append(
                f"{relative} has an invalid {chunk_type.decode('ascii', 'replace')} "
                "chunk checksum"
            )
            return

        if chunk_index == 0 and chunk_type != b"IHDR":
            errors.append(f"{relative} does not begin with an IHDR chunk")
            return
        if chunk_type == b"IHDR":
            if ihdr is not None or length != 13:
                errors.append(f"{relative} has an invalid IHDR chunk")
                return
            ihdr = struct.unpack(">IIBBBBB", chunk_data)
        elif chunk_type == b"IDAT":
            compressed.extend(chunk_data)
        elif chunk_type == b"IEND":
            if length != 0:
                errors.append(f"{relative} has an invalid IEND chunk")
                return
            saw_iend = True
            break

        offset = crc_end
        chunk_index += 1

    if ihdr is None:
        errors.append(f"{relative} has no IHDR chunk")
        return
    if not saw_iend:
        errors.append(f"{relative} has no IEND chunk")
        return

    width, height, bit_depth, color_type, compression, filtering, interlace = ihdr
    if width < 1 or height < 1 or width * height > MAX_LEADER_ART_PIXELS:
        errors.append(
            f"{relative} has invalid or excessive dimensions {width}x{height}"
        )
        return
    if allow_rgb and color_type == 2 and bit_depth in {8, 16}:
        try:
            zlib.decompress(bytes(compressed))
        except zlib.error as exc:
            errors.append(f"{relative} has invalid compressed image data: {exc}")
        return
    if color_type != 6 or bit_depth not in {8, 16}:
        errors.append(
            f"{relative} must be an 8- or 16-bit RGBA PNG "
            f"(IHDR bit depth {bit_depth}, color type {color_type})"
        )
        return
    if compression != 0 or filtering != 0:
        errors.append(f"{relative} uses unsupported PNG compression or filtering")
        return
    if interlace != 0:
        errors.append(
            f"{relative} must be non-interlaced so transparency can be audited"
        )
        return
    if not compressed:
        errors.append(f"{relative} has no image data")
        return

    bytes_per_sample = bit_depth // 8
    bytes_per_pixel = 4 * bytes_per_sample
    stride = width * bytes_per_pixel
    expected_size = height * (stride + 1)
    try:
        scanlines = zlib.decompress(bytes(compressed))
    except zlib.error as exc:
        errors.append(f"{relative} has invalid compressed image data: {exc}")
        return
    if len(scanlines) != expected_size:
        errors.append(
            f"{relative} decodes to {len(scanlines)} bytes; expected {expected_size}"
        )
        return

    previous = bytearray(stride)
    transparent_pixel = False
    cursor = 0
    alpha_offset = 3 * bytes_per_sample
    for _row in range(height):
        filter_type = scanlines[cursor]
        cursor += 1
        encoded = scanlines[cursor : cursor + stride]
        cursor += stride
        reconstructed = bytearray(stride)
        for index, byte in enumerate(encoded):
            left = reconstructed[index - bytes_per_pixel] if index >= bytes_per_pixel else 0
            above = previous[index]
            upper_left = previous[index - bytes_per_pixel] if index >= bytes_per_pixel else 0
            if filter_type == 0:
                predictor = 0
            elif filter_type == 1:
                predictor = left
            elif filter_type == 2:
                predictor = above
            elif filter_type == 3:
                predictor = (left + above) // 2
            elif filter_type == 4:
                predictor = paeth_predictor(left, above, upper_left)
            else:
                errors.append(f"{relative} uses invalid PNG filter {filter_type}")
                return
            reconstructed[index] = (byte + predictor) & 0xFF

        for pixel_start in range(0, stride, bytes_per_pixel):
            alpha_start = pixel_start + alpha_offset
            alpha = reconstructed[alpha_start : alpha_start + bytes_per_sample]
            if any(component != 0xFF for component in alpha):
                transparent_pixel = True
                break
        previous = reconstructed

    if not transparent_pixel:
        errors.append(f"{relative} is RGBA but contains no transparent pixels")


def validate_leader_art(
    nation_id: str,
    leader_art: Any,
    nation: Any,
    errors: list[str],
) -> bool:
    """Validate one optional generated full-body leader cutout."""

    where = f"nations.{nation_id}.leader_art"
    if not isinstance(leader_art, dict):
        errors.append(f"{where} must be an object when present")
        return False

    for field in LEADER_ART_FIELDS:
        if field not in leader_art:
            errors.append(f"{where} is missing {field!r}")

    asset = leader_art.get("asset")
    if not nonempty_string(asset):
        errors.append(f"{where}.asset must be a nonempty basename")
        asset_match = None
    else:
        asset_match = LEADER_ART_ASSET_RE.fullmatch(asset)
        if asset_match is None:
            errors.append(
                f"{where}.asset must match "
                "<NationId>-leader-<12 lowercase hex>.png and contain no directory"
            )
        elif asset_match.group("nation") != nation_id:
            errors.append(
                f"{where}.asset belongs to {asset_match.group('nation')!r}, "
                f"not {nation_id!r}"
            )

    for field in (
        "style",
        "background_mode",
        "generated",
        "generator",
        "credit",
        "prompt_record",
    ):
        if not nonempty_string(leader_art.get(field)):
            errors.append(f"{where}.{field} must be a nonempty string")

    identity_source = leader_art.get("identity_source_asset")
    portrait = nation.get("portrait") if isinstance(nation, dict) else None
    portrait_asset = portrait.get("asset") if isinstance(portrait, dict) else None
    if nonempty_string(identity_source):
        if not nonempty_string(portrait_asset):
            errors.append(
                f"{where}.identity_source_asset requires a verified local portrait"
            )
        elif identity_source != portrait_asset:
            errors.append(
                f"{where}.identity_source_asset is {identity_source!r}; "
                f"portrait.asset is {portrait_asset!r}"
            )
    elif identity_source is None:
        identity_qid = leader_art.get("identity_source_wikidata")
        nation_qid = nation.get("wikidata") if isinstance(nation, dict) else None
        if not nonempty_string(identity_qid):
            errors.append(
                f"{where}.identity_source_wikidata is required when no portrait is available"
            )
        elif identity_qid != nation_qid:
            errors.append(
                f"{where}.identity_source_wikidata is {identity_qid!r}; "
                f"nation.wikidata is {nation_qid!r}"
            )
        if not nonempty_string(leader_art.get("identity_review")):
            errors.append(
                f"{where}.identity_review is required for a text-led interpretation"
            )
    else:
        errors.append(f"{where}.identity_source_asset must be a string or null")

    prompt_record = leader_art.get("prompt_record")
    if nonempty_string(prompt_record):
        prompt_path = Path(prompt_record)
        unsafe = (
            prompt_path.is_absolute()
            or bool(prompt_path.drive)
            or ".." in prompt_path.parts
        )
        if unsafe:
            errors.append(
                f"{where}.prompt_record must be a safe repository-relative path "
                "under tools/avatars/prompts"
            )
        else:
            resolved_prompt_root = LEADER_ART_PROMPT_DIR.resolve()
            resolved_prompt = (REPO / prompt_path).resolve()
            try:
                resolved_prompt.relative_to(resolved_prompt_root)
            except ValueError:
                errors.append(
                    f"{where}.prompt_record must resolve under tools/avatars/prompts"
                )
            else:
                if not resolved_prompt.is_file():
                    errors.append(
                        f"{where}.prompt_record references missing file "
                        f"{prompt_record}"
                    )

    recorded_hash = leader_art.get("sha256")
    if not nonempty_string(recorded_hash) or not SHA256_RE.fullmatch(recorded_hash):
        errors.append(f"{where}.sha256 must contain exactly 64 hexadecimal digits")
    elif asset_match is not None:
        embedded_hash = asset_match.group("digest")
        if recorded_hash.lower()[:12] != embedded_hash:
            errors.append(
                f"{where}.asset digest {embedded_hash!r} is not the leading "
                "12 characters of its recorded sha256"
            )

    if asset_match is not None:
        art_path = LEADER_ART_DIR / asset
        if not art_path.is_file():
            errors.append(
                f"{where}.asset references missing file "
                f"{art_path.relative_to(REPO)}"
            )
        else:
            if nonempty_string(recorded_hash) and SHA256_RE.fullmatch(recorded_hash):
                actual_hash = file_sha256(art_path)
                if actual_hash.lower() != recorded_hash.lower():
                    errors.append(
                        f"{where}.sha256 is {recorded_hash.lower()}, but the file is "
                        f"{actual_hash}"
                    )
            background_mode = leader_art.get("background_mode")
            if background_mode not in {"transparent", "soft-pastel"}:
                errors.append(
                    f"{where}.background_mode must be 'transparent' or 'soft-pastel'"
                )
            validate_rgba_png(
                art_path,
                where,
                errors,
                allow_rgb=background_mode == "soft-pastel",
            )
    return True


def validate_manifest(
    rows: list[RosterRow], manifest: Any, errors: list[str]
) -> tuple[int, int, int]:
    if not isinstance(manifest, dict):
        errors.append("manifest root must be a JSON object")
        return 0, 0, 0
    if (
        isinstance(manifest.get("version"), bool)
        or not isinstance(manifest.get("version"), int)
        or manifest["version"] < 1
    ):
        errors.append("manifest.version must be a positive integer")
    for field in ("label", "policy", "reference_date"):
        if not nonempty_string(manifest.get(field)):
            errors.append(f"manifest.{field} must be a nonempty string")

    nations = manifest.get("nations")
    if not isinstance(nations, dict):
        errors.append("manifest.nations must be an object keyed by exact NationId")
        return 0, 0, 0

    roster_by_id = {row.code: row for row in rows}
    roster_keys = set(roster_by_id)
    manifest_keys = set(nations)
    missing = sorted(roster_keys - manifest_keys)
    extra = sorted(manifest_keys - roster_keys)
    if missing:
        errors.append("manifest is missing NationIds: " + ", ".join(missing))
    if extra:
        errors.append("manifest has unknown NationIds: " + ", ".join(extra))

    figure_owners: dict[str, list[str]] = {}
    portraits = 0
    leader_artworks = 0
    for nation_id in sorted(roster_keys & manifest_keys):
        expected = roster_by_id[nation_id]
        entry = nations[nation_id]
        where = f"nations.{nation_id}"
        if not isinstance(entry, dict):
            errors.append(f"{where} must be an object")
            continue

        # These nullable fields cannot use ``dict.get`` to distinguish an
        # intentional JSON null from an accidentally omitted key.  The other
        # required fields are already rejected by their value validators below.
        for field in ("born", "died", "review_note"):
            if field not in entry:
                errors.append(f"{where} is missing required field {field!r}")

        snapshots = {
            "display_name": expected.display_name,
            "start_1990": expected.start_1990,
            "region": expected.region,
        }
        for field, wanted in snapshots.items():
            if entry.get(field) != wanted:
                errors.append(
                    f"{where}.{field} is {entry.get(field)!r}; "
                    f"ROSTER line {expected.line} says {wanted!r}"
                )

        for field in (
            "figure",
            "canonical_lookup",
            "years",
            "role",
            "rationale",
        ):
            if not nonempty_string(entry.get(field)):
                errors.append(f"{where}.{field} must be a nonempty string")

        for field in ("born", "died"):
            value = entry.get(field)
            if value is not None and (
                isinstance(value, bool)
                or not isinstance(value, (int, float))
                or not math.isfinite(float(value))
            ):
                errors.append(f"{where}.{field} must be numeric or null")

        confidence = entry.get("confidence")
        if confidence not in CONFIDENCE_VALUES:
            errors.append(
                f"{where}.confidence must be one of "
                f"{', '.join(sorted(CONFIDENCE_VALUES))}"
            )
        review_note = entry.get("review_note")
        if review_note is not None and not nonempty_string(review_note):
            errors.append(f"{where}.review_note must be null or a nonempty string")
        if "shared_figure" in entry and not nonempty_string(entry["shared_figure"]):
            errors.append(f"{where}.shared_figure must be a nonempty explanatory note")

        lookup = entry.get("canonical_lookup")
        if nonempty_string(lookup):
            figure_owners.setdefault(normalized_name(lookup), []).append(nation_id)

        if "portrait" not in entry:
            errors.append(f"{where} is missing required portrait field (use null for fallback)")
        elif validate_portrait(nation_id, entry["portrait"], errors):
            portraits += 1

        if "leader_art" in entry and validate_leader_art(
            nation_id, entry["leader_art"], entry, errors
        ):
            leader_artworks += 1

    for normalized, owners in sorted(figure_owners.items()):
        if len(owners) < 2:
            continue
        without_note = [
            nation_id
            for nation_id in owners
            if not nonempty_string(nations[nation_id].get("shared_figure"))
        ]
        if without_note:
            errors.append(
                f"canonical figure {normalized!r} is shared by {', '.join(owners)}; "
                "every use must explain the reuse in shared_figure"
            )

    return len(nations), portraits, leader_artworks


def validate_flags(rows: Iterable[RosterRow], errors: list[str]) -> int:
    try:
        root = ET.parse(FLAGS_FILE).getroot()
    except FileNotFoundError:
        errors.append(f"missing flag sprite {FLAGS_FILE.relative_to(REPO)}")
        return 0
    except ET.ParseError as exc:
        errors.append(f"{FLAGS_FILE.relative_to(REPO)} is not valid XML: {exc}")
        return 0

    elements = list(root.iter())
    all_ids = [element.get("id") for element in elements if element.get("id")]
    duplicate_ids = sorted(
        {value for value in all_ids if all_ids.count(value) > 1}
    )
    if duplicate_ids:
        errors.append("duplicate SVG ids: " + ", ".join(duplicate_ids))

    symbols = [
        element for element in elements if element.tag.rsplit("}", 1)[-1] == "symbol"
    ]
    flag_ids = [
        element.get("id")
        for element in symbols
        if element.get("id", "").startswith("flag-")
    ]
    duplicates = sorted({value for value in flag_ids if flag_ids.count(value) > 1})
    if duplicates:
        errors.append("duplicate SVG flag ids: " + ", ".join(duplicates))

    available = set(flag_ids)
    missing = [f"flag-{row.code}" for row in rows if f"flag-{row.code}" not in available]
    if missing:
        errors.append("flag sprite is missing symbols: " + ", ".join(missing))

    # The sprite is assembled from third-party SVGs and served verbatim by the
    # game.  Audit the active content and every reference, not just the outer
    # symbol names.  References must resolve inside their own symbol: a flag
    # must not accidentally borrow a gradient or path from another country.
    forbidden_elements: set[str] = set()
    event_attributes: set[str] = set()
    external_references: set[str] = set()
    dangling_references: set[str] = set()
    malformed_urls: set[str] = set()
    invalid_colors: set[str] = set()

    for element in elements:
        local_tag = element.tag.rsplit("}", 1)[-1]
        if local_tag in FORBIDDEN_SVG_ELEMENTS:
            forbidden_elements.add(local_tag)
        for attribute in element.attrib:
            local_attribute = attribute.rsplit("}", 1)[-1]
            if local_attribute.lower().startswith("on"):
                event_attributes.add(local_attribute)

    for symbol in symbols:
        symbol_id = symbol.get("id") or "<unnamed-symbol>"
        local_ids = {
            element.get("id") for element in symbol.iter() if element.get("id")
        }
        for element in symbol.iter():
            for attribute, value in element.attrib.items():
                local_attribute = attribute.rsplit("}", 1)[-1]

                if local_attribute == "href":
                    if not value.startswith("#"):
                        external_references.add(f"{symbol_id}: {value}")
                    elif value[1:] not in local_ids:
                        dangling_references.add(f"{symbol_id}: {value}")

                url_matches = list(URL_TOKEN_RE.finditer(value))
                if value.lower().count("url(") != len(url_matches):
                    malformed_urls.add(f"{symbol_id}: {value}")
                for match in url_matches:
                    target = match.group("target")
                    if not target.startswith("#"):
                        external_references.add(f"{symbol_id}: {target}")
                    elif target[1:] not in local_ids:
                        dangling_references.add(f"{symbol_id}: {target}")

                if (
                    local_attribute in SVG_COLOR_ATTRIBUTES
                    and value.startswith("#")
                    and HEX_COLOR_RE.fullmatch(value) is None
                ):
                    invalid_colors.add(f"{symbol_id}: {local_attribute}={value!r}")
                if local_attribute == "style":
                    for match in STYLE_COLOR_RE.finditer(value):
                        color = match.group("color")
                        if HEX_COLOR_RE.fullmatch(color) is None:
                            invalid_colors.add(f"{symbol_id}: style color {color!r}")

    if forbidden_elements:
        errors.append(
            "flag sprite contains forbidden active SVG elements: "
            + ", ".join(sorted(forbidden_elements))
        )
    if event_attributes:
        errors.append(
            "flag sprite contains event-handler attributes: "
            + ", ".join(sorted(event_attributes))
        )
    if external_references:
        errors.append(
            "flag sprite contains non-local references: "
            + "; ".join(sorted(external_references))
        )
    if dangling_references:
        errors.append(
            "flag sprite contains unresolved local references: "
            + "; ".join(sorted(dangling_references))
        )
    if malformed_urls:
        errors.append(
            "flag sprite contains malformed url() values: "
            + "; ".join(sorted(malformed_urls))
        )
    if invalid_colors:
        errors.append(
            "flag sprite contains invalid hexadecimal colors: "
            + "; ".join(sorted(invalid_colors))
        )
    return len(available)


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise ParseError(f"missing manifest {path.relative_to(REPO)}") from exc
    except json.JSONDecodeError as exc:
        raise ParseError(
            f"{path.relative_to(REPO)}:{exc.lineno}:{exc.colno}: {exc.msg}"
        ) from exc


def main() -> int:
    errors: list[str] = []
    try:
        rows = parse_roster(ROSTER_FILE)
        manifest = load_json(MANIFEST_FILE)
    except (OSError, ParseError) as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1

    codes = [row.code for row in rows]
    duplicate_codes = sorted({code for code in codes if codes.count(code) > 1})
    if duplicate_codes:
        errors.append("ROSTER repeats NationIds: " + ", ".join(duplicate_codes))
    if len(rows) != EXPECTED_ROSTER_COUNT:
        errors.append(
            f"ROSTER has {len(rows)} rows; this audited asset set expects "
            f"{EXPECTED_ROSTER_COUNT}"
        )

    manifest_count, portrait_count, leader_art_count = validate_manifest(
        rows, manifest, errors
    )
    flag_count = validate_flags(rows, errors)

    if errors:
        print(f"avatar asset audit failed with {len(errors)} problem(s):", file=sys.stderr)
        for problem in errors:
            print(f"  - {problem}", file=sys.stderr)
        return 1

    print(
        "avatar asset audit OK: "
        f"{len(rows)} roster rows, {manifest_count} manifest entries, "
        f"{flag_count} flag symbols, {portrait_count} local portraits, "
        f"{len(rows) - portrait_count} intentional cameo fallbacks, "
        f"{leader_art_count} generated leader "
        f"{'artwork' if leader_art_count == 1 else 'artworks'}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
