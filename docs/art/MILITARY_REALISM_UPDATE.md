# Military form and surface refinement — 8 September 2026

The second realism pass changes the large forms that define a vehicle, then adds restrained surface texture. It builds on the earlier mechanical fittings and mesh shadows. The result is original configurable game art; the new meshes are not scanned assets or verified historical replicas.

## Geometry

- Configurable tanks and specialist vehicles gain folded nose armor that meets the hull. Tank turrets narrow around the gun opening while retaining their crew compartment. Headlights are smaller, neutral-colored and carried by brackets attached to the deck.
- The two configurable aircraft gain bounded curved fuselage transitions, volumetric wing-root fairings, and exposed inlet lips, ducts and recessed fans. Both inlet mouths remain open across every legal engine choice.
- Eight catalogue fighter recipes gain seated canopy hoods and raked windscreens. Conventional airframes use smoother station transitions; stealth airframes retain their hard sections.
- Three surface-ship recipes gain flared forward hull sections, crowned decks, actual recessed bridge glazing and open boarding boats with visible floors and seats.

The existing track, suspension, control-surface and service fittings remain. Component slots, part identities, picking, dimensions and simulation values are unchanged. All 18 configurable ground coarse meshes and all 46 far catalogue meshes retain their exact previous buffers. Aircraft still have inspection geometry only.

See [configurable equipment measurements](EQUIPMENT_REALISM_PASS.md) and [catalogue measurements](ARSENAL_REALISM_PASS.md). The 46-model near catalogue totals **343,519 triangles / 35.38 MiB** of CPU attributes; far attributes add 4.07 MiB. The largest catalogue model remains 15,760 triangles. This pass does not increase any geometry budget or relax visual-distinction or normal-quality criteria.

## Surface maps and resource cost

The shared military shader now samples locally bundled 1K normal and roughness maps from [Blue Metal Plate by Rob Tuytel / Poly Haven](https://polyhaven.com/a/blue_metal_plate), distributed under [CC0](https://polyhaven.com/license). Original files, hashes and color-space conventions are recorded in the [texture provenance README](../../spheres-web/ui/military-textures/README.md). The blue albedo is not used, so existing vehicle paint choices remain intact.

Triplanar mapping follows the source's 2.5 m surface scale. Normal strength and roughness variation are restrained to keep its seams and wear from overwhelming small fittings. Paint classes are still estimated from the existing vertex palette; this is not a per-part authored material set. Glass and tyres retain their separate response. Character and civil-art shading stays separate.

Both files total **640,475 bytes**. Two 1024-square RGBA textures and their mipmaps require approximately **10.67 MiB per active WebGL context**, or 21.33 MiB when both equipment and arsenal renderers are active; browser image caching and other resources are additional. Catalogue cards and sprites share one material pack. Civil-only scenes do not allocate it, and no new WebGL context is introduced.

The loader resolves only the two fixed same-origin local paths. Maps become active only after both valid 1K images upload. It preserves pixel-store flags and the active texture unit; its units 1/2 do not replace the designer's shadow unit 0. Failed or unsupported uploads retain untextured lighting. An initial allocation failure does not trigger a nested repaint during a draw; later asynchronous completion or failure requests the required repaint. Image callbacks, context loss, restoration and texture disposal are covered by checks. Native exact JPEG routes embed the same local files, so gameplay needs no provider API or CDN.

## Exports and review

All twelve shipping GLBs were regenerated. Tank files are 7.56–8.27 MB, specialist ground files 2.61–4.88 MB and aircraft 2.41–2.73 MB, within the existing shipping budgets. They contain geometry, vertex paint, semantic parts and component metadata. Shared surface maps, lighting and shadows are not embedded. The workshop states this beside its download control.

[Open the military inspection workshop](http://127.0.0.1:7841/tools/arsenal/military-inspection.html). Browser review covered the revised heavy tank in sand at close range, mounted headlights, the upgraded tactical aircraft and all sixteen visible arsenal cards. The inspected heavy preset has 85,652 triangles and 24 selectable parts; the upgraded tactical aircraft has 27,584 triangles and 17 parts. The final browser warning/error query was empty. This was desktop visual review, not a frame-rate benchmark or a native campaign playtest.

The existing static server was reused. No game API command, save operation or new server launch was attempted. The native release is built separately; a prior automatic approval review rejected launching a new preview with “blocked by policy,” and this pass does not retry or work around that launch.

## Integration validation

- Final complete serverless UI suite: **1,240 passed**, zero failures, skips or cancellations; 88.8 seconds. Log: `work/military-realism-ui-tests-final.log` outside the repository. This includes the two added regressions for initial texture failures; the preceding 1,238-test run also passed.
- All twelve GLBs pass byte-for-byte regeneration checks, with export/import contracts included in the full UI suite.
- Component coverage: **186 distinct / 0 weak / 0 absent**, using unchanged production criteria.
- New tests cover folded armor, mounted lamps, inlet visibility, wing fairings, exposed bridge glass, open boats, texture limits/failures, shared renderer resources and context restoration. Existing finite-coordinate, nondegenerate-triangle, winding, normal, deterministic generation and coarse-hash checks remain intact.

- Native release web suite: **274 passed**, zero failures, three existing ignored tests; 80.59 seconds. Log: `work/military-realism-native-tests.log` outside the repository.

- Final release build passed in 37.23 seconds. Log: `work/military-realism-release-build.log` outside the repository. The built, unlaunched executable is `company-sim-target/release/spheres-web.exe`, SHA256 `BFAE24FC644D67E4989876CC179C9B1446941C34D1EAEC8B9965C82F817F34CB`.
- `git diff --check` passes. The large existing leadership/tutorial worktree remains preserved; this pass is not committed or pushed.

## Remaining quality work

Higher fidelity still needs authored material assignments, model-specific UV textures and carefully prepared near/far assets. The current Spheres importer remains a strict round-trip reader for its own GLBs; external indexed and textured glTF models need a separate compatible loading path plus component articulation mapping. No third-party military vehicle mesh was integrated during this pass.
