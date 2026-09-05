#!/usr/bin/env python3
"""Prepare versioned display derivatives from unchanged generated source PNGs."""
from pathlib import Path
import hashlib, json
from PIL import Image, __version__ as pillow_version
ROOT = Path(__file__).resolve().parents[2]
KEYS = ("cabinet", "treasury", "production", "research", "diplomacy",
        "military", "logistics", "resources", "history", "campaign")
def record(path):
    with Image.open(path) as im:
        width, height = im.size
    return {"path": path.relative_to(ROOT).as_posix(), "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
            "bytes": path.stat().st_size, "width": width, "height": height}
def main():
    display = ROOT / "spheres-web/ui/area-art"
    display.mkdir(parents=True, exist_ok=True)
    areas = []
    for key in KEYS:
        source = ROOT / f"tools/area-art/source/{key}-v1.png"
        target = display / f"{key}-v1.webp"
        with Image.open(source) as im:
            assert im.size == (1536, 1024), (key, im.size)
            im.convert("RGB").save(target, "WEBP", quality=88, method=6)
        areas.append({"key": key, "version": 1, "source": record(source), "display": record(target),
                      "prompt": f"tools/area-art/prompts/{key}-v1.md",
                      "style_reference": None if key == "cabinet" else "tools/area-art/source/cabinet-v1.png",
                      "object_position": "50% 50%"})
    manifest = {"schema": 1, "generated_date": "2026-09-05",
                "generator": "OpenAI built-in image generation tool",
                "setting": "Fictional illustrative environments for a geopolitical game beginning in 1990; not depictions of identified countries, locations or historical events.",
                "display_processing": {"library": "Pillow", "version": pillow_version, "format": "WebP",
                                       "quality": 88, "method": 6, "resize": False, "creative_edits": False},
                "source_note": "Original generated PNG bytes retained unchanged; provenance metadata remains in source files.",
                "areas": areas}
    (display / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"paintings": len(areas), "source_bytes": sum(a["source"]["bytes"] for a in areas),
                      "display_bytes": sum(a["display"]["bytes"] for a in areas)}, indent=2))
if __name__ == "__main__":
    main()

