# Stridsvagn 103 historical reference asset

**Stridsvagn 103 by Lukasz Wesiora (canisferus), licensed under CC BY 3.0.**

[Author's asset page](https://opengameart.org/content/stridsvagn-103) · [License](https://creativecommons.org/licenses/by/3.0/) · [Original author notice](License.original.txt)

The author requests credit to Lukasz Wesiora and gives the contact `l.Wesiora@gmail.com`. Retain this attribution, license link, source link and modification notice when redistributing the converted model. These links are credits; loading the model needs no network resources beyond its same-origin GLB file.

This is a credited historical reference in the art inspection bench. It is a distinctive turretless vehicle, not a generic main battle tank, and has no assignment to the configurable equipment designer or campaign equipment. No endorsement by the artist is implied.

## Shipped file

| Property | Verified result |
| --- | --- |
| File | `strv103.glb` |
| Bytes | 14,517,724 (13.85 MiB) |
| SHA-256 | `2ee7df0f4b98ddadf94a0cf85070f96b5a3b80811338839dbc8d370849a2c9f4` |
| Geometry | 25,675 triangles; 24,603 exported vertices including UV/normal splits; 16 mesh nodes |
| Materials | Two core glTF metallic-roughness materials |
| Textures | Six embedded RGB images, all 2048 × 2048; two base-color JPEGs, two lossless normal PNGs, two roughness JPEGs |
| Runtime dependencies | Core glTF 2.0 only; no external buffers/images, Draco, Meshopt, KTX, material extensions or animation |
| Coordinates | Right-handed Y-up; gun toward +Z; ground at Y=0; centered in X/Z |
| Numeric dimensions | Width 3.543793, height 2.752504, length 8.951853 |

Source scene units were `NONE` with scale 1.0. Numeric scale is preserved; these values have **not** been independently calibrated as physical meters. The original separate wheels and tracks remain, but this export is static and has no animated rig or track scrolling.

## Source and conversion

Retrieved from the author's current official OpenGameArt download on 2026-09-08:

- [Official archive](https://opengameart.org/sites/default/files/stridsvagn103_0.7z): 84,100,982 bytes, SHA-256 `2941ff2a6fc0c1dea12722baec5c19635f738bcc6840355157aab92a0b54b619`.
- Source `stridsvagn103.blend`: 3,670,368 bytes, SHA-256 `b2a5742c939b4ad6c153cfe71b01969c3c2bea2673689d5ebf4c624dfe912d03`.
- The original archive contains the matching CC BY 3.0 notice, the Blender 2.70 scene, and 4K Body/Details diffuse, gloss, normal and specular JPEG atlases. `License.original.txt` is copied byte-for-byte.

Modifications by the Spheres project:

1. Evaluated static geometry and baked world transforms, preserving source UVs, split normals and material assignment, with only the documented degenerate-face pruning below. Centered the model and raised its lowest point to ground level. Converted Blender Z-up coordinates to glTF Y-up.
2. Resized diffuse, normal and gloss atlases from 4096² to 2048². Normal vectors were renormalized after resizing, preserving the original Blender tangent-space +Y/OpenGL convention.
3. Replaced the older Cycles diffuse/glass mixture with a conservative painted dielectric: metallic 0, IOR 1.5, roughness `clamp(1 - gloss, 0.20, 0.95)`. The source colored specular map and glass mixture cannot be represented exactly in core metallic-roughness; their omission is an intentional approximation, not a claim that the source authored modern PBR maps.
4. Embedded base-color JPEGs at quality 92, lossless normal PNGs, and roughness JPEGs at quality 95 with 4:4:4 sampling. Roughness remains in the glTF green channel. Repeated grayscale in unused red/blue channels improves compression; metallic remains zero and no occlusion texture uses those channels. Measured roughness RMS error is 1.63/1.68 on the 0–255 scale.
5. Repaired two zero tangents at singular source UV corners with finite orthogonal tangent vectors. Pruned four degenerate source triangles using edge cross-product length <1e-10 in preserved source units (twice triangle area), reducing 25,679 source faces to 25,675 exported faces. Vertex streams and texture bytes are unchanged. No fabricated detail or extra triangles were added.

## Reproduction and validation

`convert_reference.py` is project-authored conversion code, not a script supplied by the downloaded scene. It checks the exact source hash and opens Blender with `use_scripts=False` and script autoexecution disabled. The inspected source contains no embedded Text blocks. Run using an isolated official Blender Python runtime, with the extracted original source and temporary texture directory outside the repo:

```powershell
..\tank-reference-research\bpy-runtime\Scripts\python.exe spheres-web\ui\tank-assets\strv103\convert_reference.py ..\tank-reference-research\strv103-official\stridsvagn103 spheres-web\ui\tank-assets\strv103\strv103.glb --work ..\tank-reference-research\strv103-converted-maps
..\tank-reference-research\bpy-runtime\Scripts\python.exe spheres-web\ui\tank-assets\strv103\validate_reference.py
```

The conversion uses the official [BlenderFoundation `bpy` 5.2.1 Python distribution](https://pypi.org/project/bpy/5.2.1/), CPython 3.13 Windows x64. Its published wheel SHA-256 is `70fccb611c97d52710b1752ad1aad515cd24f831ddefbf7031ad8360032a7fc1`, verified against the installation report. Runtime and source archive are not shipped in the game.

`strv103.report.json` records conversion provenance and source/intermediate hashes. `strv103.validation.json` records shipped embedded-image hashes, structure, coordinate bounds and decoded topology. The validator checks finite normalized normals/tangents, in-range indices, UVs, texture decoding, grounding, size/hash, credits and absence of external resources/extensions. A fresh Blender import of the finished GLB preserved all 16 meshes, 25,675 exported triangles, two materials and six 2K images; its actual CPU render was visually inspected separately from the author's screenshot.
