# Tank asset research and acquired reference

Reviewed 2026-09-08. Scope: realistic tanks with free, explicit redistribution rights and accessible official files. This extends `FREE_MILITARY_ASSET_RESEARCH.md`; prior Sketchfab anonymous-download 401 and BlendSwap login restrictions were not bypassed.

## Acquired and converted: Stridsvagn 103

The strongest accessible new candidate was [Stridsvagn 103 by canisferus / Lukasz Wesiora on OpenGameArt](https://opengameart.org/content/stridsvagn-103). The author specifies CC BY 3.0 and distributes a Blender scene with diffuse, gloss, normal and specular textures. The archive's own license notice names the same author and license. Attribution is required; no purchase or account was needed.

The current official link points to [`stridsvagn103_0.7z`](https://opengameart.org/sites/default/files/stridsvagn103_0.7z), 84,100,982 bytes, SHA-256 `2941ff2a6fc0c1dea12722baec5c19635f738bcc6840355157aab92a0b54b619`. An older same-host filename differs and was not used for conversion. Only the model, license and ordinary JPEG files were extracted; `Thumbs.db` was excluded.

**Direct scene inspection and a real converted render confirmed useful quality**, beyond the author preview: 25,679 triangles across 16 meshes, intact UVs, two full 4K atlases, separate wheel/track objects, modeled tools and fixtures, and baked panel/edge detail. The source uses older Cycles materials, so calling it native metallic-roughness PBR would be misleading. The source license is verified, but physical dimensions and historical configuration accuracy are not independently certified.

The delivered [asset package](../../spheres-web/ui/tank-assets/strv103/README.md) contains a self-contained **14,517,724-byte GLB**, 25,675 exported triangles after pruning four degenerate source faces, six embedded 2K maps, attribution, original license, reproducible conversion code and a structural/geometry/image validation receipt. The converted asset uses a documented dielectric approximation of the older gloss/specular material. No external texture URLs, compressed-mesh extensions or downloaded scripts are required at runtime.

Its role is a **credited historical art reference**, not a universal equipment replacement. The visibly turretless Strv103 silhouette must not stand in for every configurable tank. The procedural tank work can learn from its coherent running gear, hull construction, tools, surface wear, and separation of large forms from baked detail.

## Other reviewed candidates

| Candidate and primary page | Rights / actual access | Quality evidence and disposition |
| --- | --- | --- |
| [Recon Tank — Update, Mophs](https://opengameart.org/content/recon-tank-update) | CC BY 4.0; official ZIP downloaded without login; included credits name Mophs and the original author | Actual ZIP includes FBX, Blender and seven 1024² maps including base color, roughness, metallic and normal. Useful PBR reference, but the six-wheel vehicle is deliberately very low-poly and does not meet the detailed tracked-tank target. |
| [Original Recon Tank, MNDV.ecb / Eric Buisson](https://opengameart.org/content/recon-tank) | Author offers CC BY 4.0 / CC0; derivative uses CC BY 4.0 | Original listing gives 970 triangles and 962 vertices. Original creator's comment supports the update; preserve both credits if any derivative is used. Not shipped into the game. |
| [M1 Tank — Asset, Alstra Infinite](https://alstrainfinite.itch.io/m1-tank) | Page says CCBY but links a generic license overview and adds an ambiguous reupload restriction | Listed small OBJ/FBX with a 545-byte palette texture; not a detailed PBR tank. Not acquired for production. |
| [Tank, Gladius.s](https://opengameart.org/content/tank) | LGPL 2.1 / 3.0 options, outside this CC0/CC BY shortlist | Approximately 630 triangles with 1024 maps; insufficient quality improvement. Not acquired. |
| [Tank near future, Kiith-Sa](https://opengameart.org/content/tank-near-future) | CC BY 3.0 among the offered licenses | Higher geometry count but fictional design without UVs/textures. Not a ready realistic textured asset; not acquired. |

No files were copied from third-party game repositories that appeared to mirror Sketchfab assets. Asset quality, author provenance and redistribution rights must all be verified before integration; a public repository alone does not establish them.

## Local acquisition evidence

Raw downloads, original files, source-page captures and inspection images are outside the repo under `work/tank-reference-research`:

- `strv103-official/stridsvagn103/`: current official Blender scene, eight original 4096² RGB JPEGs, original `License.txt`.
- `strv103-blender-inspection.json`: actual bpy scene inventory, source transforms, material nodes and source hash.
- `strv103-glb-roundtrip-preview.png`: actual rendered GLB after conversion/reimport. `strv103-author-preview.png` is the separate original preview and is never presented as our render.
- `Recon_Tank.zip`: 2,572,810 bytes, SHA-256 `ee81bd54c1c7b0c871e83c76bf21df778badf94d5cdd55b022ffc0237854eaa0`. Selected ordinary files extracted to `recon-official`; the embedded `credits.txt` confirms the two creators and CC BY 4.0.
- `bpy-install-report.json`: isolated official Blender runtime distribution and verified archive hashes.

The official Blender binary download host was inaccessible in this environment. The independent [BlenderFoundation-maintained PyPI distribution](https://pypi.org/project/bpy/5.2.1/) was accessible and was installed in an isolated external virtual environment. Source autoexecution remained disabled throughout inspection, conversion and rendering. No existing game renderers, campaign state, model-family mappings or servers were changed by this asset task.
