#!/usr/bin/env python3
"""Derive bounded offline display assets; retain originals and their attribution.

No network calls. Content-addressed outputs are stable with the pinned Pillow
version (12.3.0). The generated Rust table embeds only these derivatives.
"""
from pathlib import Path
import hashlib
import io
import json
from PIL import Image, ImageOps

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "spheres-web/ui/display-art"
VERSION = "display512-v1"


def variant(source: Path) -> tuple[Path, dict]:
    raw = source.read_bytes()
    with Image.open(io.BytesIO(raw)) as opened:
        im = ImageOps.exif_transpose(opened).convert("RGBA" if "A" in opened.getbands() else "RGB")
        original_size = list(im.size)
        im.thumbnail((512, 640), Image.Resampling.LANCZOS)
        buf = io.BytesIO()
        im.save(buf, format="WEBP", quality=82, method=6, exact=True)
        encoded = buf.getvalue()
        # Never add bytes to already-small WebP sources.
        if source.suffix.lower() == ".webp" and len(raw) < len(encoded) and max(original_size) <= 640:
            encoded = raw
        filename = source.stem.split("-leader-")[0] + "-" + hashlib.sha256(encoded).hexdigest()[:16] + ".webp"
        OUT.mkdir(parents=True, exist_ok=True)
        path = OUT / filename
        if not path.exists() or path.read_bytes() != encoded:
            path.write_bytes(encoded)
        with Image.open(io.BytesIO(encoded)) as check:
            size = list(check.size)
        return path, {"source": str(source.relative_to(ROOT)).replace("\\", "/"),
                      "source_sha256": hashlib.sha256(raw).hexdigest(),
                      "display": str(path.relative_to(ROOT)).replace("\\", "/"),
                      "source_size": original_size, "display_size": size,
                      "source_bytes": len(raw), "display_bytes": len(encoded), "version": VERSION}


if __name__ == "__main__":
    from fetch_commons_portraits import write_rust_table, MANIFEST
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    entries = manifest.get("nations", manifest)
    write_rust_table(entries)
    rows = json.loads((OUT / "manifest.json").read_text())
    old, new = sum(r["source_bytes"] for r in rows), sum(r["display_bytes"] for r in rows)
    print(f"{len(rows)} assets: {old:,} -> {new:,} embedded bytes ({100*(1-new/old):.1f}% smaller). Originals retained.")
