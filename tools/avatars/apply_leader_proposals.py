#!/usr/bin/env python3
"""Validate and apply the reviewed regional historical-leader proposals."""

from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path
from typing import Any


REPO = Path(__file__).resolve().parents[2]
MANIFEST = REPO / "spheres-web" / "data" / "nation_figures.json"
DEFAULT_PROPOSALS = (
    REPO / "tools" / "avatars" / "leader_proposals_africa_me.json",
    REPO / "tools" / "avatars" / "leader_proposals_europe.json",
    REPO / "tools" / "avatars" / "leader_proposals_world.json",
)
FIELDS = (
    "figure",
    "canonical_lookup",
    "years",
    "born",
    "died",
    "role",
    "rationale",
    "confidence",
    "review_note",
)


def atomic_text(path: Path, payload: str) -> None:
    temporary = path.with_name(path.name + ".tmp")
    with temporary.open("w", encoding="utf-8", newline="\n") as handle:
        handle.write(payload)
        handle.flush()
        os.fsync(handle.fileno())
    os.replace(temporary, path)


def proposal_entries(path: Path) -> dict[str, Any]:
    document = json.loads(path.read_text(encoding="utf-8"))
    if isinstance(document, dict) and isinstance(document.get("nations"), dict):
        return document["nations"]
    if isinstance(document, dict):
        return document
    raise ValueError(f"{path}: proposal root must be an object")


def main() -> int:
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8")
    parser = argparse.ArgumentParser()
    parser.add_argument("files", nargs="*", type=Path)
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()

    files = tuple(path.resolve() for path in args.files) or DEFAULT_PROPOSALS
    missing_files = [path for path in files if not path.is_file()]
    if missing_files:
        raise SystemExit("missing proposal file(s): " + ", ".join(map(str, missing_files)))

    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    nations = manifest["nations"]
    proposals: dict[str, dict[str, Any]] = {}
    owners: dict[str, Path] = {}
    errors: list[str] = []
    for path in files:
        for nation_id, proposal in proposal_entries(path).items():
            if nation_id in proposals:
                errors.append(
                    f"{nation_id} appears in both {owners[nation_id].name} and {path.name}"
                )
                continue
            if nation_id not in nations:
                errors.append(f"{path.name}: unknown NationId {nation_id}")
                continue
            if not isinstance(proposal, dict):
                errors.append(f"{path.name}: {nation_id} proposal is not an object")
                continue
            for field in FIELDS:
                if field not in proposal:
                    errors.append(f"{path.name}: {nation_id} is missing {field}")
            proposals[nation_id] = proposal
            owners[nation_id] = path

    uncovered = sorted(set(nations) - set(proposals))
    if uncovered:
        errors.append("proposal set does not cover: " + ", ".join(uncovered))
    usa = proposals.get("USA", {})
    if usa.get("canonical_lookup") != "Abraham Lincoln":
        errors.append("USA is an explicit user override and must remain Abraham Lincoln")
    if errors:
        print("leader proposal validation failed:", file=sys.stderr)
        for error in errors:
            print("  - " + error, file=sys.stderr)
        return 1

    changed = 0
    preserved_art = 0
    for nation_id, proposal in proposals.items():
        entry = nations[nation_id]
        old_lookup = entry.get("canonical_lookup")
        for field in FIELDS:
            entry[field] = proposal[field]
        if proposal.get("shared_figure") is not None:
            entry["shared_figure"] = proposal["shared_figure"]
        elif "shared_figure" in entry and old_lookup != proposal["canonical_lookup"]:
            entry.pop("shared_figure", None)
        if old_lookup != proposal["canonical_lookup"]:
            changed += 1
            # Leave the old portrait record in place only long enough for the
            # refresh tool to identify and remove its content-addressed file.
            # Generated art can never survive an identity change.
            entry.pop("leader_art", None)
        elif "leader_art" in entry:
            preserved_art += 1

    manifest["label"] = "Historical leaders"
    manifest["policy"] = (
        "One deceased historical leader per NationId, favoring civic, liberation, "
        "reform, resistance, independence, and nation-building leadership while "
        "avoiding genocidal, extremist, and totalitarian avatars. The selector "
        "prefers reviewed full-body character art tied to a verified identity "
        "portrait, then the portrait, then a named archival cameo."
    )
    manifest["version"] = max(int(manifest.get("version", 1)), 3)
    print(
        f"validated {len(proposals)} proposals from {len(files)} files; "
        f"{changed} identity changes; {preserved_art} existing artwork records preserved"
    )
    if args.apply:
        atomic_text(MANIFEST, json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
        print(f"applied to {MANIFEST.relative_to(REPO)}")
    else:
        print("check only; pass --apply to update the manifest")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
