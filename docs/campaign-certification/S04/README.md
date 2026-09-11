# S04 — Operational warfare and coupled map integration

**Status: draft; integrated acceptance and completion evidence pending.** This report records intended behavior and prepared source. It does not declare S04 complete, earn G1 or certify a campaign. The parent task must bind every final result to the exact applied candidate and artifacts before changing status.

## Source and application boundary

| Input | Exact identity and role |
| --- | --- |
| Original active playset | `5f7f355502f17bd6bd8f0383a2d14f0024fa7884`; supplier, equipment, party and art ownership to preserve. |
| Pinned upstream master | `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`; selected campaign, peace, movement and coupled map implementation. This is the master pin, not `5d07fa4`. |
| Simulation patch base | Integrated S02 `cd9eb8f18579a9c37fd10b1d4ba86b0d50b2a03a`; the four replaced simulation files were unchanged by S03 when staged. |
| Root-wiring staging input | S03 `5d07fa4d5956ad168504086a00db327b085d3170`; `lib.rs`, `world.rs` and `data/mod.rs` patches were prepared and application-checked against this earlier S03 snapshot. |
| Final S03 predecessor | `8b1ff717ae5c21d1b96d183fbd987164ed10b03b`; later S03 refinements require fresh input-hash/application checks. Earlier patch checks are not verification of this revision. |
| Applied/tested S04 candidate | **PENDING:** commit, clean/dirty state, executable hash, embedded-asset manifest and test-log bindings. |

Staging inventories are `manifest.json`, `root-wiring-manifest.json` and `web-manifest.json`. Source copies and patches are not proof of an applied build. The separate `campaign-aims-history.patch` retains completed achievements and bounds abandoned selections; its application and verification must be recorded independently. Copy the durable manifests/evidence when this draft moves into `docs/campaign-certification/S04/`.

## Resulting behavior to qualify

The national war tick takes one `operations::Snapshot`. Campaign preparation, local engagements and the existing final loss settlement consume that same opening allocation. Ground force, custom ammunition, Arsenal holdings and population casualties retain their existing owners. Operational local results replace the corresponding legacy calculation; they must not add a second national casualty, ammunition or equipment debit. Saved reserves, sectors and transfers locate existing forces across conflicts; merely issuing an order does not create an army.

`campaign_supply` models finite sustainment **services**: source service moves into booked cargo, arrival moves it into local buffers and daily support consumes it. It shares freight capacity with commerce and retains blocked journeys until their normal dated handling. Those service units are not physical ammunition, fuel barrels or equipment stock. `operations::Snapshot` and the equipment subsystem remain responsible for physical ammunition once. Loading, previewing or retrying an order must not dispatch or settle support.

Physical occupation uses `control::controller/can_operate` and blocks relevant operating sites without transferring legal province ownership or acquiring a supplier's factory, cash, stock or contracts. Limited peace requires the saved coalition's consent and rechecks live eligibility at command/settlement time. Consented cession uses `districts::transfer_district`; reparations use the existing cash-transfer authority and do not create GDP. Historical participants and expired pending offers can remain in a valid save until the normal peace tick processes them.

Operational state uses an explicit versioned capability. The staged `spheres-integrated-save` v1 envelope declares `warfare_version: 1` alongside equipment, party, economy, company-network and supplier-operations subversions. The existing company decoder is extended, not replaced by the older master's equipment-only loader. Recognized legacy shapes have a documented identity migration; absent capabilities stay absent. `EnableOperationalWarfare` is the reviewed player upgrade. Fresh unified adoption is a separate S05 startup change; ordinary loading must not enable or enroll warfare. Read-only validators inspect identities, quantities, dates, route geometry and consent, preserve high-water IDs, and reject unknown nested fields in the 17 stored structs. They do not reseed fronts, reconcile armies, expire records or perform settlement.

Map integration keeps globe camera/focus, terrain displacement, coast/land masks, river/lake surfaces, city geometry, labels and picking together. `globe3d.js`, `city-layer.js`, `city-mesh.js`, `terrain-surface.js`, river paths, lake raster and lake-surface metadata form a coupled change. Existing detailed tank and aircraft geometry is preserved. The larger zoom range enables procedural city/ground detail, but the underlying elevation samples remain approximately **1,855 m apart** (`globe3d.js`, zoom documentation). This does not provide newly measured street-scale terrain, surveyed city buildings or a new elevation dataset.

## Acceptance — all final results pending

| Check | Required evidence | Status |
| --- | --- | --- |
| Force and ammunition conservation | One/two-war fixtures, simultaneous coalitions, losses, reserves, movement, garrisons and actual custom ammunition; one population casualty debit. | Pending |
| Service cargo | Finite source/dispatch/arrival/use, shared commerce capacity, blocked access/sea lift, overdue cargo and same-day idempotence. | Pending |
| Occupation and peace | Actual site blockers; unchanged corporate property; human coalition consent; stale offers; bounded cession/reparations and one settlement. | Pending |
| Save ownership | Twelve staged validator/strict-decode fixtures plus native/browser roundtrips, preserved legacy defaults and high-water IDs, explicit adoption and deterministic continuation. | Pending |
| Coupled map | Numerical geometry/relief/selection checks and required overview/near-zoom visual evidence for mountains, coast/lake edges, cities and microstates. | Pending |
| Regression and performance | Exact integrated native and UI suites, source-bound performance cases, and preserved supplier/party/art contracts. | Pending |

Staged `git apply --check --whitespace=error` success establishes patch applicability to the checked input, not compilation or runtime correctness. Selected Node results recorded in `web-manifest.json` describe only those staged source checks; attach their raw logs and source hashes before using them as final evidence. Native suites, browser behavior, all-city sweeps and the applied candidate must have their own results. Missing or policy-restricted cells remain evidence gaps, never implicit passes. Broader campaign, content, platform and packaging certification stays with the pathway's later sessions.
