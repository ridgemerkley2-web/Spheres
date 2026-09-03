#!/usr/bin/env python3
"""Resumable production queue for full-body historical-leader character art.

Image generation itself stays in Codex's built-in image tool.  This helper owns
the deterministic parts around it: prompt records, alpha validation, content-
addressed project copies, per-country sidecars, sharding, and the final manifest
merge.  Workers never edit the shared manifest while generating.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import struct
import sys
from datetime import date
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[2]
MANIFEST = REPO / "spheres-web" / "data" / "nation_figures.json"
PORTRAIT_DIR = REPO / "spheres-web" / "ui" / "portraits"
ART_DIR = REPO / "spheres-web" / "ui" / "leader-art"
PROMPT_DIR = REPO / "tools" / "avatars" / "prompts"
RECORD_DIR = REPO / "tools" / "avatars" / "leader-art-records"
STYLE_ASSET = "USA-leader-088ed05335f8.png"
STYLE_VERSION = "soft-arcade-historical-leader-v1"
ASSET_RE = re.compile(r"^(?P<nation>[A-Za-z][A-Za-z0-9]*)-leader-(?P<hash>[0-9a-f]{12})\.png$")
SAFE_ID_RE = re.compile(r"^[A-Za-z][A-Za-z0-9]*$")
PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"


def atomic_text(path: Path, payload: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    with temporary.open("w", encoding="utf-8", newline="\n") as handle:
        handle.write(payload)
        handle.flush()
        os.fsync(handle.fileno())
    os.replace(temporary, path)


def load_manifest() -> dict[str, Any]:
    return json.loads(MANIFEST.read_text(encoding="utf-8"))


def portrait_asset(entry: dict[str, Any]) -> str | None:
    portrait = entry.get("portrait")
    if not isinstance(portrait, dict):
        return None
    asset = portrait.get("asset")
    return asset if isinstance(asset, str) and Path(asset).name == asset else None


def slug(value: str) -> str:
    normalized = re.sub(r"[^a-z0-9]+", "-", value.casefold()).strip("-")
    return normalized[:64] or "leader"


def prompt_path(nation_id: str, entry: dict[str, Any]) -> Path:
    if nation_id == "USA":
        return PROMPT_DIR / "USA-abraham-lincoln-v1.md"
    return PROMPT_DIR / f"{nation_id}-{slug(str(entry['figure']))}-v1.md"


def art_prompt(nation_id: str, entry: dict[str, Any]) -> str:
    display = entry["display_name"]
    figure = entry["figure"]
    years = entry["years"]
    role = entry["role"]
    source = portrait_asset(entry)
    if source:
        input_line = (
            f"Input images: Image 1 is the verified identity portrait of {figure}; "
            "Image 2 is the Abraham Lincoln character-art style and composition reference."
        )
        identity_direction = (
            "Preserve the recognizable facial identity and defining physical features "
            "shown in Image 1. Match Image 2's"
        )
        avoid_copy = "Do not copy Abraham Lincoln's face, beard, clothing, hat, body, or pose."
    else:
        input_line = (
            "Input images: Image 1 is the Abraham Lincoln character-art style and "
            f"composition reference. No freely licensed local likeness of {figure} "
            "was available, so this is a text-led historical interpretation."
        )
        identity_direction = (
            f"Make the character historically plausible and recognizably associated "
            f"with {figure} using the supplied name, lifespan, and documented role. "
            "Match Image 1's"
        )
        avoid_copy = "Do not copy Abraham Lincoln's face, beard, clothing, hat, body, or pose."
    return f"""Use case: identity-preserve
Asset type: full-body country-selector game character
{input_line}

Primary request: Create one respectful full-body cartoon character of {figure} ({years}), the {role} selected as the historical avatar for {display}. {identity_direction} polished, slightly realistic animated-feature rendering, adult proportions, clean silhouette, soft painterly shading, restrained detail, and approachable arcade-game energy. {avoid_copy}

Subject and clothing: Show {figure} alone in a relaxed, quietly heroic three-quarter standing pose with the entire body visible from head to footwear. Use historically and culturally appropriate documented clothing for the person's lifetime and public role. Keep props minimal and include one only when directly associated with the person's documented work. Avoid invented regalia, exoticized dress, generic stereotypes, military spectacle, and triumphalist imagery.

Composition: Centered portrait canvas with generous open space around the silhouette; face clear at card size; hands and feet fully visible; no cropping.
Color and mood: Soft dusty lavender, slate blue, warm cream, and muted charcoal accents, adapted naturally to the documented clothing; warm, dignified, inviting.
Background: a smooth, edge-to-edge, very pale warm-lavender and cream studio backdrop with at most a subtle soft vignette. This is an intentional opaque game-card background, not transparency. No checkerboard, grid, visible pattern, scenery, horizon, floor rectangle, or hard shadow box.
Constraints: one person; preserve identity; no flag, map, seal, emblem, text, caption, weapon, podium, border, watermark, extra people, photorealism, giant head, chibi anatomy, mockery, or caricature.
"""


def ensure_prompt(nation_id: str, entry: dict[str, Any]) -> Path:
    path = prompt_path(nation_id, entry)
    if nation_id == "USA" and path.is_file():
        return path
    source = portrait_asset(entry)
    relative_source = f"spheres-web/ui/portraits/{source}" if source else "none"
    body = (
        f"# {entry['display_name']} — {entry['figure']} character art v1\n\n"
        f"Prepared: {date.today().isoformat()}  \n"
        f"Identity source: `{relative_source}`  \n"
        f"Style reference: `spheres-web/ui/leader-art/{STYLE_ASSET}`  \n"
        "Generator: OpenAI image generation\n\n"
        "## Art prompt\n\n"
        + art_prompt(nation_id, entry)
        + "\n## Background correction\n\n"
        "If a render contains a transparency checkerboard or other visible pattern, "
        "run one precise background replacement that changes only the background to "
        "the smooth pale-lavender/cream studio treatment and preserves the person, "
        "face, pose, clothing, hands, props, edges, and rendering.\n"
    )
    atomic_text(path, body)
    return path


def validate_png(path: Path) -> tuple[int, int, str]:
    payload = path.read_bytes()
    if len(payload) < 33 or not payload.startswith(PNG_SIGNATURE):
        raise ValueError(f"{path} is not a PNG")
    if payload[12:16] != b"IHDR":
        raise ValueError(f"{path} has no leading IHDR")
    width, height, bit_depth, color_type, _compression, _filtering, _interlace = struct.unpack(
        ">IIBBBBB", payload[16:29]
    )
    if bit_depth not in {8, 16} or color_type not in {2, 6}:
        raise ValueError(f"{path} is not an RGB/RGBA PNG (color type {color_type})")
    # The full audit validates chunks and decodes RGBA rows to prove that files
    # marked transparent contain real alpha. Intentional RGB studio cards are
    # accepted after visual review rejects checkerboards and other patterns.
    sys.path.insert(0, str(Path(__file__).parent))
    from check_assets import validate_rgba_png  # type: ignore

    errors: list[str] = []
    validate_rgba_png(path, "generated artwork", errors, allow_rgb=True)
    if errors:
        raise ValueError("; ".join(errors))
    return width, height, "transparent" if color_type == 6 else "soft-pastel"


def record_path(nation_id: str) -> Path:
    return RECORD_DIR / f"{nation_id}.json"


def load_record(nation_id: str) -> dict[str, Any] | None:
    path = record_path(nation_id)
    if not path.is_file():
        return None
    value = json.loads(path.read_text(encoding="utf-8"))
    return value if isinstance(value, dict) else None


def record_matches(nation_id: str, entry: dict[str, Any], record: object) -> bool:
    if not isinstance(record, dict):
        return False
    art = record.get("leader_art")
    source = portrait_asset(entry)
    return (
        record.get("nation_id") == nation_id
        and record.get("figure") == entry.get("figure")
        and isinstance(art, dict)
        and art.get("identity_source_asset") == source
        and isinstance(art.get("asset"), str)
        and (ART_DIR / art["asset"]).is_file()
    )


def manifest_art_matches(entry: dict[str, Any]) -> bool:
    art = entry.get("leader_art")
    source = portrait_asset(entry)
    return (
        isinstance(art, dict)
        and art.get("identity_source_asset") == source
        and isinstance(art.get("asset"), str)
        and (ART_DIR / art["asset"]).is_file()
    )


def queue(manifest: dict[str, Any]) -> tuple[list[dict[str, str]], list[dict[str, str]]]:
    ready_or_pending: list[dict[str, str]] = []
    blocked: list[dict[str, str]] = []
    for nation_id, entry in sorted(manifest["nations"].items()):
        if manifest_art_matches(entry) or record_matches(nation_id, entry, load_record(nation_id)):
            continue
        source = portrait_asset(entry)
        item = {
            "nation_id": nation_id,
            "display_name": entry["display_name"],
            "figure": entry["figure"],
            "source": str(PORTRAIT_DIR / source) if source else "",
            "style_reference": str(ART_DIR / STYLE_ASSET),
        }
        (ready_or_pending if source else blocked).append(item)
    return ready_or_pending, blocked


def command_prompt(args: argparse.Namespace) -> int:
    manifest = load_manifest()
    entry = manifest["nations"].get(args.nation)
    if not isinstance(entry, dict):
        raise SystemExit(f"unknown NationId: {args.nation}")
    ensure_prompt(args.nation, entry)
    print(art_prompt(args.nation, entry))
    return 0


def command_queue(args: argparse.Namespace) -> int:
    if args.shard_count < 1 or not 0 <= args.shard_index < args.shard_count:
        raise SystemExit("shard index must be from 0 through shard-count - 1")
    pending, blocked = queue(load_manifest())
    selected = [item for index, item in enumerate(pending) if index % args.shard_count == args.shard_index]
    print(json.dumps({"pending": selected, "blocked_no_portrait": blocked}, ensure_ascii=False, indent=2))
    return 0


def command_status(_args: argparse.Namespace) -> int:
    manifest = load_manifest()
    pending, blocked = queue(manifest)
    merged = sum(manifest_art_matches(entry) for entry in manifest["nations"].values())
    staged = sum(
        record_matches(nation_id, entry, load_record(nation_id))
        for nation_id, entry in manifest["nations"].items()
        if not manifest_art_matches(entry)
    )
    print(
        json.dumps(
            {
                "total": len(manifest["nations"]),
                "merged": merged,
                "staged": staged,
                "pending": len(pending),
                "blocked_no_portrait": len(blocked),
            },
            indent=2,
        )
    )
    return 0


def command_accept(args: argparse.Namespace) -> int:
    if not SAFE_ID_RE.fullmatch(args.nation):
        raise SystemExit("invalid NationId")
    manifest = load_manifest()
    entry = manifest["nations"].get(args.nation)
    if not isinstance(entry, dict):
        raise SystemExit(f"unknown NationId: {args.nation}")
    source_asset = portrait_asset(entry)
    generated = Path(args.generated).expanduser().resolve()
    if not generated.is_file():
        raise SystemExit(f"generated file does not exist: {generated}")
    width, height, background_mode = validate_png(generated)
    payload = generated.read_bytes()
    digest = hashlib.sha256(payload).hexdigest()
    filename = f"{args.nation}-leader-{digest[:12]}.png"
    ART_DIR.mkdir(parents=True, exist_ok=True)
    destination = ART_DIR / filename
    if destination.exists() and destination.read_bytes() != payload:
        raise SystemExit(f"refusing to overwrite different bytes at {destination}")
    if not destination.exists():
        shutil.copyfile(generated, destination)
    prompt = ensure_prompt(args.nation, entry)
    prompt_relative = prompt.relative_to(REPO).as_posix()
    record = {
        "nation_id": args.nation,
        "figure": entry["figure"],
        "leader_art": {
            "asset": filename,
            "sha256": digest,
            "identity_source_asset": source_asset,
            "style": STYLE_VERSION,
            "background_mode": background_mode,
            "generated": date.today().isoformat(),
            "generator": "OpenAI image generation",
            "credit": (
                f"AI-generated game character of {entry['figure']} based on the "
                "verified freely licensed portrait recorded for this nation."
                if source_asset
                else f"AI-generated historical interpretation of {entry['figure']} "
                "from the reviewed roster identity and archival record."
            ),
            "prompt_record": prompt_relative,
            "review": (
                f"Identity, full-body composition, period clothing, and "
                f"{background_mode} background reviewed at {width}×{height}."
            ),
        },
    }
    if source_asset is None:
        record["leader_art"]["identity_source_wikidata"] = entry.get("wikidata")
        record["leader_art"]["identity_review"] = (
            "Text-led interpretation: the exact Wikipedia/Wikidata identity was "
            "resolved, but no suitable freely licensed local portrait was available."
        )
    atomic_text(record_path(args.nation), json.dumps(record, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps(record, ensure_ascii=False, indent=2))
    return 0


def command_merge(args: argparse.Namespace) -> int:
    manifest = load_manifest()
    nations = manifest["nations"]
    applied = 0
    for nation_id, entry in nations.items():
        record = load_record(nation_id)
        if record_matches(nation_id, entry, record):
            entry["leader_art"] = record["leader_art"]
            applied += 1
    remaining, blocked = queue(manifest)
    if args.require_all and (remaining or blocked):
        raise SystemExit(
            f"refusing partial merge: {len(remaining)} pending and {len(blocked)} blocked"
        )
    manifest["version"] = max(int(manifest.get("version", 1)), 2)
    atomic_text(MANIFEST, json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
    sys.path.insert(0, str(Path(__file__).parent))
    from fetch_commons_portraits import write_rust_table  # type: ignore

    write_rust_table(nations)
    print(f"merged {applied} staged records; {len(remaining)} pending; {len(blocked)} blocked")
    return 0


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8")
    parser = argparse.ArgumentParser()
    commands = parser.add_subparsers(dest="command", required=True)
    prompt_command = commands.add_parser("prompt")
    prompt_command.add_argument("nation")
    prompt_command.set_defaults(run=command_prompt)
    queue_command = commands.add_parser("queue")
    queue_command.add_argument("--shard-index", type=int, default=0)
    queue_command.add_argument("--shard-count", type=int, default=1)
    queue_command.set_defaults(run=command_queue)
    status_command = commands.add_parser("status")
    status_command.set_defaults(run=command_status)
    accept_command = commands.add_parser("accept")
    accept_command.add_argument("nation")
    accept_command.add_argument("generated")
    accept_command.set_defaults(run=command_accept)
    merge_command = commands.add_parser("merge")
    merge_command.add_argument("--require-all", action="store_true")
    merge_command.set_defaults(run=command_merge)
    args = parser.parse_args()
    return args.run(args)


if __name__ == "__main__":
    raise SystemExit(main())
