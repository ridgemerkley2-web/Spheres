# Military paint surface maps

These two images are byte-identical copies of the original 1K JPEG maps from **Blue Metal Plate**, created by **Rob Tuytel** and distributed by **Poly Haven** under **CC0 1.0 Universal**. Only the filenames changed. No resizing, recompression, painting, or channel changes were applied.

- [Primary asset page](https://polyhaven.com/a/blue_metal_plate)
- [Poly Haven asset license and redistribution statement](https://polyhaven.com/license)
- [CC0 1.0 Universal legal code](https://creativecommons.org/publicdomain/zero/1.0/legalcode)
- [Official file metadata used to verify each download](https://api.polyhaven.com/files/blue_metal_plate)
- [Poly Haven texture conventions, including OpenGL normal maps](https://docs.polyhaven.com/en/technical-standards/textures)

The asset license permits commercial use and redistribution of these raw files. Attribution is optional; the credit above records provenance. The game serves the included files locally and does not need the Poly Haven API or CDN at runtime.

| Runtime file | Original filename | Dimensions | JPEG mode | Bytes | SHA-256 |
| --- | --- | --- | --- | ---: | --- |
| `paint-normal.jpg` | `blue_metal_plate_nor_gl_1k.jpg` | 1024 × 1024 | RGB | 238,025 | `970f0273c9e2e3b4fc8338bfd58a28c413b70ac4d1734d9dc8a136724bde56e6` |
| `paint-roughness.jpg` | `blue_metal_plate_rough_1k.jpg` | 1024 × 1024 | Grayscale | 402,450 | `37168cf57144db28dd0743dab47fbebc09147fac919d5e2b5610b069c0b13a46` |

Direct original files:

- [OpenGL normal JPEG](https://dl.polyhaven.org/file/ph-assets/Textures/jpg/1k/blue_metal_plate/blue_metal_plate_nor_gl_1k.jpg)
- [Roughness JPEG](https://dl.polyhaven.org/file/ph-assets/Textures/jpg/1k/blue_metal_plate/blue_metal_plate_rough_1k.jpg)

The downloaded byte counts and MD5 checksums matched the official metadata: normal `f14c0f01832f8b18c39f07e4b5d6f749`, roughness `76c3911d8ad2416e4db3104a2ef831c7`. Both images decoded and verified successfully. The original downloads and metadata remain in `work/military-material-research` outside this repository.

## Rendering contract and visual review

`paint-normal.jpg` is the supplied **OpenGL / +Y tangent-space normal** map, not the DirectX variant. Decode RGB from `[0,1]` to `[-1,1]`, apply modest XY strength, then normalize and transform through the appropriate signed tangent basis for each triplanar projection. Keep image-row orientation consistent with the chosen texture coordinates; a vertical image flip and a normal-Y inversion are different operations. The downloaded GL/DX pair was checked: red and blue nearly agree, while green is inverted within JPEG quantization tolerance.

Treat both textures as **linear data**, not sRGB color. The grayscale roughness file contains roughness directly; do not invert it as a gloss map. Both dimensions are powers of two, so WebGL 1 can use repeat wrapping and generated mipmaps. The textures need no UV attributes when sampled through the renderer's triplanar mapping.

Visual inspection found a largely smooth painted steel surface with shallow scuffs and occasional scratches. The normal map is close to neutral: mean RGB approximately `(128.18, 127.37, 254.18)`. Roughness averages approximately `0.284`; use it as restrained variation around each vehicle material's chosen roughness rather than replacing military paint's base roughness. The source asset represents a 2.5 m-wide surface. It contains straight plate seams and some worn marks, so it is **not a uniform micrograin map**: excessive strength or dense repetition would stamp distracting stripes onto every vehicle. Preserve the existing material palette; the blue albedo is deliberately not included.

The initially considered ambientCG Metal032 material is also CC0, but its official download redirect returned HTTP 403 during this task. No Metal032 files are included, and that access restriction was not bypassed.
