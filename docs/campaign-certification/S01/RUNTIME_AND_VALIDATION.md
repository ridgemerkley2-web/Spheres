# S01 runtime and validation inventory

Date: 2026-09-10. Scope: source inspection and baseline freeze only.

| Source | Pinned revision |
| --- | --- |
| Active playset | `5f7f355502f17bd6bd8f0383a2d14f0024fa7884` |
| Fetched master | `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9` |

This report was produced by read-only source, Git-diff and test-inventory inspection. Its author did **not** execute tests, start a server, load or advance a campaign, merge branches, or change runtime files. The parent S01 task owns fresh baseline execution and evidence. Prior records and freshly supplied parent results are separated below. References without a revision prefix refer to the active pin; `master:` references refer to the master pin above. Line numbers are for those pins.

## Fresh S01 results supplied by the parent task

The parent ran these in isolated pinned worktrees. This report's author read the completed log summaries; native runs subsequently completed and the parent finalized the rows below. The final S01 execution manifest records raw logs and hashes. Evidence paths below are relative to the shared `work` directory, outside the game checkout.

| Check | Fresh parent result | Log |
| --- | --- | --- |
| Active UI | **1,371 passed, 0 failed, 1 skipped**; 1,372 discovered | `campaign-certification/evidence/S01/active-ui-unit.log` |
| Master UI | **1,093 passed, 0 failed, 0 skipped** | `campaign-certification/evidence/S01/master-ui-unit.log` |
| Active art audit | Fails immediately on `materialClasses`; no valid full budget census | `campaign-certification/evidence/S01/active-art-audit.log` |
| Master art audit | Fails with stale `docs/art/P0_BUDGETS.md` and **33 over-budget rows** | `campaign-certification/evidence/S01/master-art-audit.log` |
| Active/master native | Active: 1,271 passed / 0 failed / 71 ignored. Master: 1,339 passed / 0 failed / 70 ignored | `campaign-certification/evidence/S01/active-native.log`, `master-native.log` |

The active skip is the optional archived-API advisor test: its external evidence fixture is absent from the isolated checkout. The discovered test count agrees with the older 1,372-pass record; this is an environment/fixture distinction rather than a new failing assertion. Master's 33 over-budget rows are fresh measurements for **that pinned master**, distinct from both active's aborted audit and the obsolete “30 over budget” historical report. Do not merge these into one branch-independent art status.

## Recorded results and their limits

`docs/GAME_REVIEW_2026_09_10.md:203` records:

| Check | Prior recorded result | What this establishes |
| --- | --- | --- |
| `cargo test --locked --release --workspace --no-fail-fast` | 1,271 passed, 0 failed, 71 ignored | Native tests selected by the default workspace run passed; ignored cases did not pass by implication. |
| `node tools/ui/run-unit.cjs` | 1,372 passed, 0 failed | The selected Node UI contract/geometry tests passed. This is not 1,372 live-browser scenarios. |
| Generated aircraft GLB checks | Twelve GLBs match generation | The checked exported assets correspond to the generator at that recorded pass. |
| Aircraft in-app inspection | Cockpit, intake, exhaust and component changes inspected without browser errors | Narrow visual evidence for the aircraft work, not complete campaign qualification. |
| `node tools/ui/bench_art.cjs --check` | Fails immediately on changed vertex layout | The old art budget report cannot currently validate actual upload memory. |

The art failure is concrete: `tools/ui/bench_art.cjs:95–111` permits exactly three `Float32Array` attributes (`positions`, `normals`, `colors`) and computes bytes as triangles × 3 vertices × 3 components × 4 bytes × 3 attributes. Ground meshes now also emit `materialClasses`. Update measurement logic before applying budgets in the assigned later session. Do not suppress the guard or describe the obsolete historical “30 over budget” report as a current measurement. Dedicated aircraft LOD/geometry checks are separate passing evidence.

Earlier failed aircraft fixtures were corrected before the recorded 1,372-test pass; their former failure count is not a current defect count. The fresh master UI/art results above were executed by the parent, not this author. Configured CI is not evidence that a particular GitHub run passed.

## Which world is being tested

Headless/default worlds and browser campaigns intentionally differ. `spheres-sim/src/world.rs:942–1041` leaves daily simulation, market, logistics, operations, production, manufacturing, economic competition and political opt-ins disabled by default; resource gates default on. Never use a passing monthly default-world golden as proof that the browser daily profile passed.

| Entry/profile | Active playset behavior | Master differences to reconcile |
| --- | --- | --- |
| Common browser `play_rules` | `spheres-web/src/main.rs:6928`: request daily play; enable resource market, ideology readouts, logistics routes, physical logistics, military operations, production and manufacturing; force ideology takeover off; ensure government; enable province economy. | `master:spheres-web/src/main.rs:7342–7363` additionally sets `operational_warfare = 1`, enrolls the operational campaign, and sets `industry_rebuild = true`. |
| Fresh browser campaign | `fresh_play_rules`, `main.rs:6949`: enable and enrich starting industrial inheritance, apply common rules, then enroll historical party leadership. Used by boot and new-game creation (`main.rs:7042`, `7097`, `/api/new` at `7904`). | `master:main.rs:7366–7375` enables contractor companies, population and fiscal recovery after common rules; it has no historical party-leadership enrollment. |
| Loaded browser campaign | `loaded_play_game`, `main.rs:6962`: new session ID, empty transport receipts; apply common browser rules, warm resource readouts, snapshot. It does **not** regrant starting industrial inheritance or indiscriminately enroll historical leadership. `/api/load` goes through `storage::read/decode` (`main.rs:7945`). | `master:main.rs:7381` applies its different common profile, including operational enrollment and industry. Fresh-only population/fiscal enrollment must not be assumed to run on every legacy load. |
| Legacy mid-month world | `clock::enable_daily_play`, `spheres-sim/src/clock.rs:46`, defers switching a non-daily world whose day is above 1 until the next month. | Preserve the cadence boundary and ensure the new ledgers do not settle twice during that transition. |
| Province economy | `spheres-sim/src/province_economy.rs:106` enables accounts only for daily or pending-daily worlds and preserves existing accounts. | Reconcile with master population/workforce/industry ownership; enabling an account is not permission to recreate its assets. |

Neither common browser function is a blanket enablement of every feature. Record actual serialized rule fields, cadence, pending transition, player, date, seed and enrollment state for every fixture. A rule-profile name without these values is insufficient.

## Save formats and state ownership

Active native format selection and validation are in `spheres-sim/src/lib.rs:1496–1597`:

- Raw legacy world when no relevant equipment/company or party book requires an envelope.
- `spheres-equipment-save` version 1 for equipment; versions 2, 3, 4 and 5 correspond to company tank, equipment, ammunition and refit-service books.
- `spheres-party-leadership-save` version 1, with `equipment_version` 0–5, wraps the enabled historical rule and saved person/party book. Mismatched rule/book/envelope or company-version combinations fail closed.
- Loading validates equipment and companies, migrates only documented legacy state, repairs derived indexes, and validates historical/future person bindings. The narrow legacy empty-Japanese-component migration is not general permission to invent missing historical people.

Master native `spheres-sim/src/lib.rs:1613–1632` writes/accepts only equipment envelope version 1 or a raw world and validates its operational warfare version. An active company v2–5 or party envelope is therefore **not** compatible with that loader. The same `companies` field names different models across the branches, so matching JSON keys is not sufficient.

The browser adds a second envelope, `spheres-campaign` version 1 (`spheres-web/src/storage.rs:16–82`), containing the native world save, retained history, log, history epoch and display metadata. Normal browser archives can nest an active party envelope through native `load` at `storage.rs:65`.

There is an additional active source-level import gap: `storage::decode` at `storage.rs:47–60` directly routes raw worlds and equipment envelopes to native `load`, but rejects a **standalone party envelope** before reaching it. Thus native party-save acceptance does not establish browser party-save import. Add a direct-party fixture alongside nested campaign fixtures in S05. This was inferred from the explicit branch logic, not reproduced by executing a load.

Both root `GameRules` and `WorldState` derive Serde deserialization without a blanket unknown-field rejection (`world.rs:941`, `1044`). Master adds saved operational campaign/supply/peace, population and fiscal state while lacking active party state. A superficially successful parse of a cross-branch raw/equipment save could therefore still discard unsupported fields; actual conservation assertions must establish compatibility. Do not claim such loss was reproduced here.

Storage already stages and syncs replacement files, retains a valid backup, restricts slot paths and rotates three autosaves (`storage.rs:86–238`). Master adds streaming save-list metadata parsing and tests (`master:storage.rs:197–435`); preserve full JSON readability/Unicode/trailing-data behavior while reducing listing memory. Listing `readable` means valid JSON, not that a supported campaign can be loaded.

## Existing validation inventory

| Layer | Source/entry point | Coverage and limits |
| --- | --- | --- |
| Native workspace | Cargo workspace tests; `spheres-web/src/transport.rs`, `storage.rs`, `main.rs` | Actual simulation/API helpers, receipts, state ownership, migrations and storage behavior. Storage tests include failed writes, archive roundtrip and slot/autosave isolation (`storage.rs:249`, `283`, `307`). Native command tests include once-only payments and stale turns (`main.rs:8663`, `8718`, `8809`). |
| Node UI suite | `tools/ui/run-unit.cjs:3` | Selects `check_*.cjs` **except names containing `_browser`**. Many tests extract real host functions into VM fixtures. They validate useful contracts but do not run a rendered browser or compiled server. |
| Transport/session UI | `check_campaign_transport.cjs`, `check_session.cjs`, `check_campaign_dialogs.cjs` | Lost committed responses, same-receipt retries, blocked second actions, queued draft retention, stale campaign/target/confirmation guards, save errors and cancellation. |
| Government/guidance UI | `check_government.cjs`, `check_government_leadership.cjs`, `check_leadership_government_review.cjs`, `check_guidance_host.cjs`, `check_guidance_ui.cjs`, `check_advisor_model.cjs`, `check_tutorial_model.cjs` | Exact reviewed action routing, leader presentation, navigation-only guidance and read-only advisor/tutorial models. The suite's dynamic discovery is authoritative. |
| Terrain/map UI | `check_map_focus.cjs`, `check_map_detail.cjs`, `check_globe_terrain.cjs` and terrain checks | Focus/picking/projection and terrain contracts; master adds city-layer/city-mesh checks. Rendered scene agreement still needs visual evidence. |
| Real browser regressions | `check_session_browser.cjs:1–12` and five other `_browser` files | Requires disposable server **and working directory**, explicitly writes/loads campaigns; exercises desktop/narrow panels. Do not point it at the user's campaign server. Excluded siblings are `check_starting_industry_browser.cjs`, `check_small_modules_browser.cjs`, `check_materials_browser.cjs`, `check_flight_command_browser.cjs`, `check_competition_browser.cjs`. |
| CI real-browser smoke | `tools/ui/ci-browser.cjs:8–49` | Starts an exact release binary in a fresh disposable directory; USA fresh daily campaign, Advisor, map Find, committed-response loss/retry, save/history reload, desktop/narrow research screenshots and page errors. This is one narrow scripted smoke, not the proposed 137-country/23-successor matrix. |
| Cross-platform CI | `.github/workflows/verify.yml:12–33` | Ubuntu/Windows: native release tests, Node suite, release web binary, Playwright Chromium smoke, browser artifacts. Require actual run IDs, tested commit and artifacts before claiming CI passed. |
| Calibration workflow | `.github/workflows/daily-calibration.yml:13–32` | Eleven scenarios × seeds 1990/7/42, economic competition off, plus balanced-budget economic-AI-on for those three seeds; 30-year runs. It is not the agreed eight-country campaign through end-of-2035 qualification. |

The optional advisor test at `tools/ui/check_advisor_model.cjs:129–133` is skipped when `../leadership-2035-evidence/source-state-before.json` is absent. That file was present beside the primary checkout during inspection, but is outside this repository and absent from the isolated active baseline. The fresh parent run therefore records one skip. Freeze the fixture/provenance or record this skip explicitly; never manufacture the archived API evidence.

### The 71 ignored native checks

A static count of lines matching `^\s*#\[ignore` in active `spheres-sim` and `spheres-web` yields 71, agreeing with the prior runtime tally. This inventory is source inspection, not execution:

| File | Ignore attributes |
| --- | ---: |
| `spheres-sim/src/blocs.rs` | 1 |
| `spheres-sim/src/dyads.rs` | 3 |
| `spheres-sim/src/exact.rs` | 1 |
| `spheres-sim/src/lib.rs` | 15 |
| `spheres-sim/src/logistics.rs` | 2 |
| `spheres-sim/src/resources.rs` | 2 |
| `spheres-sim/src/tech/mod.rs` | 2 |
| `spheres-sim/tests/bloc_census.rs` | 7 |
| `spheres-sim/tests/capital_damage_audit.rs` | 5 |
| `spheres-sim/tests/companies_integration.rs` | 2 |
| `spheres-sim/tests/company_ammunition.rs` | 1 |
| `spheres-sim/tests/company_refits.rs` | 1 |
| `spheres-sim/tests/daily_balance.rs` | 1 |
| `spheres-sim/tests/endowment_channel_probe.rs` | 1 |
| `spheres-sim/tests/endowment_margin_probe.rs` | 1 |
| `spheres-sim/tests/growth_decomposition.rs` | 20 |
| `spheres-sim/tests/sample_size_audit.rs` | 3 |
| `spheres-web/src/main.rs` | 2 |
| `spheres-web/src/performance.rs` | 1 |
| **Total** | **71** |

These are not one homogeneous optional suite:

- `blocs.rs:1447` is explicitly **RED BY DESIGN**, tracked as P-6: transcribed starting bloc rows disagree with authored brief bars. Six ignored assertions A1/A2/A3/A4/A5/A7 in `tests/bloc_census.rs:946–1110` are explicitly parked historical calibration bars. Their comments are dated evidence, not newly observed current-run failures. They cannot be represented as passing historical coverage.
- `main.rs:8962` (`browser_growth_model_gap`) and `9036` (`classify_corpus`) are measurement instruments whose comments explicitly say they assert nothing. Growth/endowment/capital/sample-size probes are also primarily diagnosis, not acceptance gates.
- Timing/profile checks include `lib.rs:4650`, `logistics.rs:1248`, `1934`, and `performance.rs:326`. The latter requires a disposable `SPHERES_PROFILE_OUT`, defaults to 30 years and **rejects years above 30** (`performance.rs:330–334`). It cannot currently certify 1 January 1990 through 31 December 2035 without an intentional later harness change.
- Long censuses include the two 200-seed × 480-month × 7-arm tools at `lib.rs:5776`, `5967`; select them deliberately rather than launching every ignored case together with timing checks.
- Four company fixture exporters (`tests/companies_integration.rs:1638`, `1745`; `company_ammunition.rs:1284`; `company_refits.rs:1244`) deliberately write synthetic QA stages only when their named output environment variable is supplied. Generated fixtures must remain labeled synthetic and isolated from user saves.
- `daily_balance.rs:397` labels itself a dated calibration diagnosis, not a balance target or census. `exact.rs:269` dumps numeric pins. A successful execution of either would not establish a playable campaign.

## Source-embedded runtime and asset risk

The compiled server embeds HTML, JS, CSS and asset bytes, including `INDEX` (`spheres-web/src/main.rs:42`), government UI (`113–115`), and equipment import/mesh/model/export modules (`136–146`). `/api/build` at `7155` exposes compiled build information. An already-running executable does not adopt edited source files automatically. A static workshop can therefore show newer art than the actual campaign executable.

The S01 manifest and subsequent evidence should bind source revision, executable SHA-256, `/api/build`, data/art manifest hashes, actual rule profile, fixture hash, platform/browser and reference hardware. Test the packaged binary's routes and assets in S05 and final packaging, not only on-disk workshop pages. No assertion about the user's presently running binary is made here.

## S04 integration risks and required matrix

Master's operational path is not yet active behavior. `master:WARFARE.md:41–82` describes opening-snapshot national loss settlement, shared freight after commercial reservations, supported tactical rung-6 strike/ammunition and once-only population casualties. `master:WARFARE.md:94–114` distinguishes control, ownership and consented settlement. Preserve active equipment/company accounts while introducing that path.

| Cell | Fixture/action | Required invariant/evidence |
| --- | --- | --- |
| W1: allocation and settlement | One nation in simultaneous conflicts; approach, reserve, withdrawal and active contact; save between orders and result. | Sum of assigned/exposed/reserve/transit forces is conserved; equipment and force losses are bounded by the single opening national snapshot and debited once. No reused conflict ID after closure or reload. |
| W2: military/commercial freight | Saturated shared route, foreign access granted/revoked, blocked cargo, retreat with held supply. | Commercial reservations and military service use the same capacity; denied access cannot route through foreign territory; held cargo is retained without duplicating stock. Service dispatch does not invent cash, ammunition, oil or GDP. |
| W3: physical control | Occupy an industrial district without cession, ceasefire, then formally cede in a separate fixture. | Legal owner, controller, facility operation, route access and displayed map agree. Occupation does not award the occupier inventory, assets or cash automatically. Formal transfer happens once. |
| W4: equipment/air losses | Paid custom tactical aircraft with compatible bombs at supported rung 6; unsupported recon/interception/lift/rung case; grounded aircraft. | Only supported missions activate the custom rating and stores; parked aircraft are not exposed; paid company stock and national equipment remain distinct; combat loss and population casualty settlement occur once. |
| W5: peace and politics | Human coalition member, stale offer, coalition changed, last-province/nuclear limits, limited cash reparation, political opening. | Every required member consents; AI never accepts for the human; stale review does not execute; cash transfer and control return settle once; political result preserves active person identity/eligibility hooks. |
| W6: map integration | High mountains, coast/lake edge, dense city, microstate and successor; sweep zoom and click the displaced surface before/after reload. | Same live relief used by terrain, city, label and picking; no camera inside mountains or buried/floating cities; district/country IDs and selection are stable. Visual evidence at overview and near zoom accompanies numerical checks. |

Map changes must move together. Master changes the globe maximum zoom from 192 to 1500 and introduces `exaggerationFor` (`master:spheres-web/ui/globe3d.js:125–178`); terrain focus, labels and picking all consume it. Master also introduces `city-layer.js`, `city-mesh.js` and host integration, while changing river rings and the six lake path hashes (`terrain-surface.js:13`). Copying only the zoom constant or only one geometry/data file can break the coupled camera/terrain/city contract. The higher zoom adds procedural cover/city detail; the source itself says underlying elevation samples remain approximately 1,855 m apart, so do not advertise new measured terrain resolution.

Government needs a deliberate semantic merge: active `government.rs:6105` calls `party_leadership::ensure_all`, and `8857` calls `party_leadership::on_succession`. Master lacks those modules/hooks and adds population hardship (`master:government.rs:6299`). Preserve both the new hardship/accounting input and saved real/fictional person bindings. Government preview already clones the actual world (`spheres-web/src/government_view.rs:192–218`); its exact-command review must stay aligned with the integrated resolver.

## S05 save/session/command validation matrix

Each positive row requires native load/save, browser archive load/save where supported, then a short deterministic continuation compared with an uninterrupted equivalent under the **same declared rule profile**. Use isolated copies only. Expected migration differences must be enumerated rather than hidden behind whole-save hash changes.

| Cell | Minimum representative states | Acceptance/evidence |
| --- | --- | --- |
| S1: dialects | Raw legacy; active equipment v1; company v2/v3/v4/v5; party v1 with each legal nested equipment version; browser campaign v1; pinned-master campaign with population/fiscal/warfare state. | Exact party/person/company/model/project IDs, money/debt, paid work, corporate/national stock, deliveries, ammo/refit reservations and RNG/date survive. Explicit mapping handles the two company books and differing rule fields. |
| S2: corrupt/unsupported | Unknown versions, wrong envelope/book/rule pairing, direct party envelope, missing mandatory fields, truncated archive, invalid history. | Supported cases import through every intended entry point; unsupported or inconsistent cases fail clearly before replacing live state. No silent drop of master operational/population state or active party/company property. |
| S3: adoption boundaries | Fresh 1990 campaign; legacy save at day 1 and mid-month; already-enrolled population/fiscal/industry/leadership; old empty Japanese component migration. | No fresh inheritance on load, no double enrollment/settlement, correct delayed cadence, saved incumbent retained; only documented empty components migrate. |
| S4: transactions | Save during construction funding, prototype development, stock purchase/shipment, ammo order, refit, pending peace and military movement. | Resume preserves entitlement and liability; completion/cancellation/refund/delivery/loss happens exactly once. A company asset never becomes government property merely by loading. |
| S5: uncertain commands | Committed response lost; double click; page reload; retry same receipt; server restart/load creating a new session; changed body using an old receipt ID. | Same live-session retry confirms one committed action; duplicate body mismatch rejects; stale session is not automatically replayed. After restart/adoption, receipts are not assumed durable: review current state and keep uncertainty explicit. Native `transport.rs:24–84`, host tests and real browser evidence must agree. |
| S6: storage failure | Named slots, invalid path names, staged-write failure, damaged current file with valid backup, interrupted save, three rotating autosaves, large archives. | Last valid state remains recoverable; damaged bytes cannot overwrite a valid backup; metadata listing does not replace load validation; history/log retained. Capture Windows and Linux evidence. |
| S7: UI review isolation | Government preview/confirmation, Advisor recommendation, construction/procurement quote while campaign/player/target changes, tutorial navigation. | Previews/readers do not mutate or consume RNG; stale response cannot populate a new campaign; only the reviewed current command executes once; guidance opens the authoritative destination without placing orders. |
| S8: binary and browser | Exact integrated executable and assets on desktop/narrow viewport; boot/continue/save menus; map selection; government/research/construction/company routes. | `/api/build` matches manifest, required embedded assets/routes resolve, no page errors/invalid labels/blocked core controls. Run existing CI smoke plus the missing integration cells; do not equate a workshop pass with campaign evidence. |

S04/S05 release-blocking failures include state loss, duplicate charges/assets/losses, wrong-country mutation, unreadable supported saves and unrecoverable campaigns. Missing positive/negative matrix cells are evidence gaps, not passes. Historic calibration/content deficiencies and the obsolete art audit retain named owners and later session markers; do not silently broaden S01 into fixing them.

The larger all-137-start smoke, 23-successor activation/load checks, eight-country long campaigns and human playtests remain later pathway work. These source inventories and the existing USA CI smoke do not certify that scope.
