# SPHERES — painted places, version 1

Ten original environment paintings give the game's main rooms a distinct identity. The visual direction is a geopolitical world beginning in 1990: oil and gouache brushwork, deep navy and teal shadows, restrained brass and ivory highlights, atmospheric depth and believable working places.

## The ten paintings

| Area | Scene | Where it appears |
| --- | --- | --- |
| Cabinet | Council chamber above a river capital at dawn | Cabinet overview |
| Treasury | Central-bank dealing room, green CRTs and paper ledgers | Budget and policy |
| Production | Operating steelworks in an industrial river valley | Production and industry |
| Research | Observatory and research campus at blue hour | Research field chooser and accessible research list |
| Diplomacy | Delegations meeting around an oval conference table | Decisions and sphere relations |
| Military | Command room overlooking a rainy airfield | Force allocation and arms manufacturing |
| Logistics | Coastal container port with freighter and rail connections | Logistics |
| Resources | Mountain river basin, mine, hydroelectric dam and farmland | Strategic Supply Command and resource exchange |
| Chronicle | Archive reading room, atlas, photographs and bound journals | Campaign Chronicle |
| Campaign | Globe and atlas in a cartographic study above a harbor | Campaign welcome |

These are fictional illustrative settings. They do not identify a country, claim historical accuracy for a location, or encode game state. National portraits and flags retain their existing source records. This first pass covers game systems; country and regional variants are future work.

## Integration

The paintings occupy framed panels beside headings or above controls. They remain fully opaque; data and controls use their own surfaces. Responsive layouts stack the art on narrow screens. The research graph keeps its established geometry and interaction area.

The shared helper in `spheres-web/ui/area-art.js` permits ten fixed local URLs. The server embeds the WebP files, serves an immutable versioned URL for each, and rejects unknown paths. No external image service or internet connection is required during play. Decorative markup has empty alternative text and no focus stops. Native HTML contains all meaningful labels.

## Sources and generation

All ten images were generated with the built-in OpenAI image generation tool on 2026-09-05. Cabinet established the style; the other nine used that painting as a style reference. No artist's name was requested.

- Unchanged original PNGs: `tools/area-art/source/*-v1.png`.
- Exact final generation prompts and reference provenance: `tools/area-art/prompts/*-v1.md`.
- Display WebPs and hashes: `spheres-web/ui/area-art/manifest.json`.
- Reproduce display conversion: `python tools/area-art/prepare.py` with Pillow 12.3.0.

Display conversion preserves the 1536 × 1024 dimensions and creates quality-88 WebP assets. This is compression only; source paintings are retained unchanged, including their embedded provenance metadata. The portable release includes prompts and the manifest. Source PNGs live in the source repository and the binary source patch, rather than adding their full size to the playable package.

## Extending the set

Keep a consistent medium, shadow palette, natural materials and restrained light. Give each new region or subject its own landscape, architecture and period-appropriate details. Avoid generic repeated castles, visible UI text, logos, invented flags or futuristic technology. Preserve readable silhouettes at small sizes and leave room for a centered 16:9 crop.

Country-specific art needs researched regional references and a clear distinction between a representative scene and a real named landmark. Reserve new filenames such as `cabinet-v2.webp` for revisions so immutable caching cannot serve old artwork. Retain each original and its final prompt. Expand portraits, event illustrations and map terrain as separate deliberate passes.

