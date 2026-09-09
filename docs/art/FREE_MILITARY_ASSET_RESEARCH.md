# Free military mesh research

Reviewed 2026-09-08. This is an acquisition shortlist, not a shipped asset manifest. No model from this report is registered in the game. Triangle counts establish possible detail, not accuracy or visual quality; the shortlisted showcase models still require an actual render, topology inspection, and articulation review.

The best next acquisitions are **alexxx_xarchenko's T-90** and **Arash1984's F-22**. Both have primary-site CC-BY 4.0 records and substantial geometry. Ship candidates exist, but the accessible choices have either older Blender materials, incomplete technical metadata, or an AI-generation disclosure. The two assets downloaded without an account are lower-detail CC0 fallbacks, not evidence that the desired realism is finished.

## Eight candidates

“Not exposed” means the public listing or metadata did not establish the downloadable archive's contents. Do not assume a `.glb` extension from the existence of a web viewer.

| Asset and primary source | Creator / license verified | Geometry and format evidence | Acquisition and suitability |
| --- | --- | --- | --- |
| [T-90](https://sketchfab.com/3d-models/t-90-9bb8af8876a6478aa92089eff058d4db) | alexxx_xarchenko; **CC-BY 4.0**, confirmed by original public API | **304,272 triangles**, 169,533 vertices. Download archive format/size not exposed by public metadata. | Strong detailed tank candidate; author identifies it as their school course work and links their portfolio. Download enabled, but official archive acquisition needs authentication. Inspect exact variant, textures, turret/gun separation, and track detail before binding a historical equipment ID. |
| [F-22](https://sketchfab.com/3d-models/f-22-ad21e82384434bc5a8602c3e331f7c38) | Arash1984; **CC-BY 4.0**, confirmed by original public API | **95,370 triangles**, 51,146 vertices. Author says made in 3ds Max and PBR textured with base color, metalness, roughness, and normal maps. Actual archive formats/size not exposed. | Best close-up aircraft candidate in this pass. Official download endpoint returned **401 Unauthorized** without credentials. No archive obtained. Native authoring application is not proof that a MAX file is included. |
| [Type-054A Class Frigate](https://sketchfab.com/3d-models/type-054a-class-frigate-948cca64a5274644a35e4600e023d61d) | Muhamad Mirza Arrafi; **CC-BY 4.0**, confirmed by original public API | **67,936 triangles**, 39,890 vertices. Actual archive formats/size not exposed. | Conditional naval candidate. Page explicitly marks **Generated with AI**; factual silhouette, equipment, and topology need extra visual review. Do not infer accurate real dimensions or service dates from its description. Download enabled; no archive obtained. |
| [HMS Daring](https://blendswap.com/blend/7050) | Alexdark; **CC-BY** on asset and download pages; exact license version remains to be checked in the acquired bundle | **Blender 2.6x**, **38.6 MB** listing. Triangle count not published. Textures plus PSD sources; guns rigged, radar/propeller animated, two untextured helicopters. | Useful named destroyer and component-separation candidate. [Download page](https://blendswap.com/blend/7050/download) requires sign-in. Blender Internal materials/animation need conversion; no bundle inspected. |
| [Zumwalt.blend](https://blendswap.com/blend/4896) | jocko79; **CC0** on asset and download pages | **Blender 2.6x**, **326 KB** listing; author describes low-to-medium polygon count, exact triangles not published. | Small ship fallback/reference, not a demonstrated high-detail replacement. [Download page](https://blendswap.com/blend/4896/download) requires sign-in. Screenshot ocean/sky from CG Textures are explicitly **not included**; they are not cleared by this model's CC0 grant. |
| [Armored Cavalry 2](https://blendswap.com/blend/10568) | SONGKRO; **CC0** on asset and download pages | **Blender 2.6x**, **608 KB** listing. Exact triangles not published; comments describe array-based details. | Fictional armored infantry carrier candidate, useful for a configurable AFV family. It is not a verified real historical vehicle. [Download page](https://blendswap.com/blend/10568/download) requires sign-in. Modifier evaluation and materials still need inspection. |
| [Abrams tank](https://opengameart.org/content/abrams-tank) | Sketlux, based on yd's Freeciv tank; **CC0**. The [parent Freeciv model set](https://opengameart.org/content/blender-models-for-freeciv-units) is also CC0. | **2,942,084 bytes actually downloaded**; header `BLENDER-v279` confirms Blender 2.79 binary. No verified triangle count. | Direct no-login source successfully obtained. Lower-detail baseline/reference; do not market it as a high-resolution Abrams. Actual geometry and embedded material quality have not been rendered or approved. |
| [fighter jets](https://opengameart.org/content/fighter-jets) | Captain_Ahab_62; **CC0**, with optional requested credit | **859,946-byte ZIP actually downloaded**, containing only `basic_replacements.blend` (**3,476,936 bytes**, header `BLENDER-v301`). Author describes low-poly aircraft made for YSFlight. Exact aircraft roster and triangle counts unverified. | Direct no-login aircraft source pack successfully obtained. Suitable for offline inspection or distant LOD candidates, not yet an approved close-up aircraft set. No separate license file in the ZIP. |

## Download evidence and handoff

All downloaded material is outside the repository:

`C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\military-model-research`

| File | SHA-256 / inspected contents |
| --- | --- |
| `abrams-tank.blend` | `52e17f448c9fe37a37b340933c504e221dd4c47c944a9a6f1e1ba7ea211e2aab` |
| `basic_replacements.blend.zip` | `d445e1fcd68e9771df5c646ccd737c2bd259f1f9c1cd584dc4a36a2e6ad42a67`; one ordinary `.blend` member, no executable, script, license, or other archive member |

Original direct download URLs: [Abrams binary](https://opengameart.org/sites/default/files/abrams-tank.blend) and [aircraft ZIP](https://opengameart.org/sites/default/files/basic_replacements.blend.zip). Their original source pages are saved as `abrams-tank-source.html` and `fighter-jets-source.html`; `freeciv-parent-source.html` preserves the parent CC0 evidence. `CC0-1.0-legalcode.html` is a separately downloaded official license text, **not an author-supplied license file**. No claim is made about embedded Blender text blocks, because Blender was not opened.

The three Sketchfab records are saved as `<model-id>-metadata.json` from `https://api.sketchfab.com/v3/models/<model-id>`. They record downloadable status, exact face/vertex counts, and a license URL of `http://creativecommons.org/licenses/by/4.0/`. They contain no downloadable archive URL. The F-22 official `/download` endpoint required authentication; no attempt was made to bypass it or acquire a mirror. BlendSwap's visible download pages also require sign-in. No downloaded model scripts, Blender scenes, or executables were run.

## Current renderer compatibility

`spheres-web/ui/equipment-import.js` is a strict Spheres round-trip GLB reader. Its requirements are much narrower than glTF generally:

- Self-contained GLB 2.0, one embedded geometry buffer, one mesh, one primitive, one node with identity transforms, and at most one material.
- Unindexed independent triangles. `POSITION`, `NORMAL`, and `COLOR_0` are required as tightly packed float32 `VEC3` arrays. No UVs, integer/quantized colors, sparse accessors, or interleaved attributes.
- No textures, images, samplers, skinning, animation, or extensions; this includes Draco and material extensions. A normal-mapped PBR model will be rejected, even when its file is named `.glb`.
- Spheres part ranges and specification extras are optional, but third-party geometry will not acquire meaningful tank upgrade selections automatically. Non-finite geometry, invalid colors/normals, inconsistent counts, and incorrect bounds are rejected.

No explicit triangle-count ceiling was found in this importer. That is not a promise of good performance: the current unindexed position/normal/color layout alone needs about **10.8 MB per 100,000 triangles**, before CPU copies and GPU buffers. The designer also performs triangle picking.

Prefer a separate, standards-based textured GLB presentation path with indexed geometry, PBR textures, node transforms, and named articulation nodes. Keep the existing configurable geometry/export contract intact until the new renderer also supports specification-to-part mapping. Decoding compression, converting old Blender materials, defining turret/gun/wheel pivots, checking real scale, and generating deliberate near/far LODs are asset-preparation tasks. Dropping textures to satisfy this importer would lose much of the requested surface realism.

## Exclusions and practical license constraints

- [BMP-2 by 42manako](https://sketchfab.com/3d-models/bmp-2-a843e922897c499ca53e718235b3875a) has a CC label but explicitly says it was **ripped from Squad**. Excluded; the uploader's label does not establish rights to distribute that game's art.
- [BMP-2 by VolodymyrPr](https://www.cgtrader.com/free-3d-models/vehicle/military-vehicle/bmp-2-3613abee-e8df-4888-88ae-c4a98ca183cb) is a tempting 117,040-polygon PBR listing with BLEND/OBJ/FBX/glTF/DAE files, but its Royalty Free No AI license is **not suitable for unrestricted repository/GLB export**. The [official terms, sections 21A–21B](https://www.cgtrader.com/pages/terms-and-conditions) limit redistribution to incorporated products and require measures against extracting assets. No download performed.
- The [dannzjs listing called Abrams M1A2 SEPv3](https://sketchfab.com/3d-models/abrams-m1a2-sepv3-eb6f5560198740269507e9948376414c) advertises 256.8k triangles and CC Attribution, but this pass did not establish the vehicle's visual identity. Do not bind its title to an Abrams simulation model without inspection; the T-90 has clearer first-party authorship notes for the initial tank pilot.
- Noncommercial, no-derivatives, editorial-only, game-ripped, and unlicensed mirrors are not acquisition paths for this build.

CC-BY assets need creator/source/license attribution and a record of modifications, including conversions and generated LODs. CC0 permits reuse without an attribution requirement, though preserving provenance remains useful. These are copyright license findings, not a guarantee of every depicted logo or third-party contribution. Keep source notices with exported derivatives and do not imply creator or manufacturer endorsement. [CC-BY 4.0](https://creativecommons.org/licenses/by/4.0/), [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
