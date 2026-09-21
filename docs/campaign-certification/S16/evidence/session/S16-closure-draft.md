# S16 / G3 closure draft — qualification pending

This is a working checklist, not a completion certificate. S16 is authorized by the 14 September 2026 instruction “continue”; execution must stop after S16. S17 remains planned. Runtime files are frozen for root qualification unless a specific fix is requested.

## Exact completion boundary

The pathway's G3 row is **G2 + S11–S16: “Ground and three supported air missions use real campaign forces and results.”** S16's marker is **“G3: Defend skies works with a researched, company-supplied fighter platform.”** Its three acceptance requirements are:

1. The fighter has legal component research, development, purchase, delivery, upkeep and stores.
2. Interception changes a hostile strike's result; fighter and ground-air-defense contributions remain distinct and coordinated.
3. A target aircraft cannot be destroyed twice, and depleted defense visibly loses effectiveness.

Sources: `integration/docs/planning/campaign-pathway.json` S16 and `integration/docs/CERTIFIED_CAMPAIGN_PATHWAY.md` gate table. G3 may be recorded as earned only after evidence addresses all three requirements and the inherited prerequisites. G3 does not award G4, G5 or CP1.

## Implemented scope to qualify

- `air_fighter` is a distinct researched defensive platform. `air_fighter_integration` requires `air_propulsion_integration`, `air_mission_systems` and the existing CMOS prerequisite. Research grants component knowledge; company development, paid fabrication, purchase and dated delivery remain separate.
- Fighters use a frozen interception rating, zero strike rating and finite `air_missile_short_range` finished stores. Baseline radius is a modeled 900 km including return flight; extended fuel gives 1,215 km. Existing paid upkeep, support caps, exact-revision inventory, squadron claims, basing, transit and service exclusions apply.
- Fighter-only hardware cannot be installed on the existing light-attack or tactical-strike families. Existing strike profiles remain byte-compatible through an omitted zero interception field; fighter profiles use rules version 5 and newly touched equipment ledgers use version 9.
- Defend skies confronts opposing, explicit Support army or Strike target orders in the same supported conflict and defended area. The campaign contact remains the owner of ground results. Fighter interception and ground air defense receive separate report fields and one combined physical aircraft-loss settlement.
- Existing authorized opposing orders may execute independently of which government is currently selected. Public review and queue commands retain directing-government permission and protected request receipts.
- No fighter participates in the old implicit bombing path before squadrons exist. Research alone adds no standing combat-technology modifier, aircraft or missiles.

These are implemented behaviors pending final qualification, not assertions that every browser step or loss outcome has already been observed.

## Evidence required before S16 / G3 closure

| Acceptance / risk | Required retained observation |
|---|---|
| Legal fighter lifecycle | Native checks for locked prerequisites and dated research, company development and tooling, actual raw-material/fabrication payment, finite shelf stock, paid purchase, delivery delay, exact national ownership and funded upkeep. Identify which steps use ordinary commands and which initial conditions or research-point inputs are authored. |
| Compatible stores | Actual finished missile production or paid supplier stock, separate delivery, exact-family compatibility, finite consumption, and automatic support's upkeep-first cap. Save/load preserves the support purchase receipt and pending arrivals prevent duplicate purchases. |
| Interception affects the campaign | Controlled native comparison of the same hostile explicit strike with funded defenders versus no/depleted defense, showing changed hostile applied power and resulting campaign contact behavior. Preserve the exact worlds and explain any common authored setup. |
| Ground defense stays distinct | Native checks and dated reports distinguish fighter interception from existing paid ground-air-defense contribution. Both feed a single bounded casualty settlement; neither invents a second target or free stores. |
| No duplicated aircraft losses | Simultaneous attackers/defenders, shared ammunition-family coverage, whole-aircraft loss bounds, unaffected other squadrons/refit reservations, and deterministic replay. A fractional expected loss is not proof of a whole aircraft destroyed. Report actual observed whole losses literally. |
| Eligibility and shortages | Correct-family orders, basing/access/radius, transit/service, zero or partial missile coverage, changed/ended conflict, and stale or busy orders are refused or blocked through native rules. Zero coverage does not produce paid interception power. |
| Saved opposing orders | Prepared and queued opposing missions continue across Save/Load and player selection; current player authority cannot erase an already authorized opposing launch. Forged or stale prepared plans remain invalid. |
| Ordinary browser path | Actual Defend skies review, native blockers, confirmation, dated results, support/store availability and settled geography. Bind browser served assets, executable SHA, source revision and driver revision. Compare named native full-world checkpoints plus Save/Load/Continue; list exact counts rather than reusing S15 totals. |
| Protected mutation path | Existing session/client/sequence receipt tests plus the actual air-mission review/confirm path. Keep native interrupted-response tests separate if the authored browser run does not itself lose a response. |
| Visible depletion and reporting | Capture readable funded and depleted defense/review/result states at the widths actually inspected. Record actual reviewer findings and screenshot names. Do not infer a 320px or human-playtest result from desktop/390px automation. |
| Compatibility | Final full suites include the old ground and S11–S15 aviation tests. Old version-8 strike saves round-trip with no interception field; fighter revision/profile forgery is rejected. Retain source/build bindings and all existing ignores/skips literally. |

Final Windows native web, simulation, integration and Node reports; corresponding Linux qualification; the reviewed executable; native fixture export; final browser result; and review-launch/copy verification must all bind the intended candidate. A tools-only driver follow-up must list its exact changed paths and explicitly prove unchanged runtime paths. Root chooses the actual runner filenames; none are assumed complete here.

The first development binary reportedly passed four fighter tests and all 126 equipment tests. That run preceded additional automatic-support assertions and is **development feedback only**. Use final proof outputs for published counts, dates, failures, ignores and status.

## Inherited qualification and evidence retention

Preserve prior evidence without relabeling it as an S16 run:

- S11 manifest: `integration/docs/campaign-certification/S11/manifest.json`; runtime `d9afd1604219e160cc9deaa4b3999df7db5cde57`, completed `2026-09-14T03:07:38.443Z`. This retains its original authored ground-operation scope.
- Combined S12–S15 manifest: `integration/docs/campaign-certification/S15/manifest.json`; runtime `4c4129abeeec4e9e2987f121e875f5efc54b4c7f`, browser driver `1653638fc050b4b4796875ffdeb7fbff9e7b6d7e`, completed `2026-09-14T05:04:03.364Z`. This retains its original authored flight journey and literal zero observed whole-aircraft losses.
- S16 starts from publication commit `ab17714cb88141f9ee14b97457688bf7de2732de`. The final S16 candidate must be read from root's qualification proofs, not inferred from this starting pin.

Minimum new retained inputs: exact runner/driver sources; final proof JSON and full logs; final fixture manifest and all expected saves; browser result, reviewed screenshots and actual source archives; canonical full-world audits and history-envelope comparisons; reviewed copied save and native copy audit; original-campaign preservation record and its baseline; selected failed attempts with clear outcomes; collector selection and byte-verified inventory. Hash executables and identify their source location; do not add executable copies to Git merely for retention. Preserve complete selected artifacts, including native fields and errors.

Before publishing, verify retained source-to-storage SHA/size mappings and reconstruction, clean candidate bindings, actual outcomes and counts, and unchanged original-campaign preservation. Link inherited manifests by exact source revision and retained identity. Do not copy prior success totals into new proof fields.

## Proposed final summary template

> S16 adds a researched, company-supplied defensive fighter and Defend skies. Fighters use paid upkeep, geographic bases and separately manufactured air-to-air missiles. In the qualified authored scenario, interception changed the result of opposing explicit Support army / Strike target orders; dated reports separate fighter and ground-air-defense contributions while conserving aircraft and stores. [Insert only final observed days, commands, comparisons, mission counts, whole losses, platforms and inspected widths from retained proofs.]
>
> S11 and S12–S15 retain their prior pinned qualification; the final S16 build rechecks the affected military, supply and save paths. [Only after all acceptance evidence passes: S16 is complete and G3 is earned for this supported military-loop scope.] Execution stops after S16. S17 requires a new instruction.

## Limits that remain explicit

- Defensive interception covers the supported explicit Support army and Strike target orders. It does not establish an all-air-threat interception system, strategic bombing defense, naval aviation or unscheduled national air-superiority simulation.
- Opposing missions in the authored journey are disclosed preconditions or deliberately issued orders. They are not evidence that S17 AI independently researched, financed, produced, purchased and launched fighters.
- Authored money, factory capacity, resources, research-point inputs, starting designs/holdings, access and opposing orders must be enumerated. A browser that starts with a certified fighter cannot claim it earned that certification during the browser journey.
- Whole-aircraft losses, finite stores, suppression, radius and upkeep use explicit deterministic game assumptions. Report observations; do not present those ratings as historical aircraft specifications.
- No full CP1 campaign, eight-country cast, 2035 endpoint, worldwide content census, finished S18 inspection-art contract, independent human playtest, new performance certificate or release certificate follows from S16 alone.
- The approved G2 gameplay/content separation remains intact. C01–C07 and the later historical/content, AI, usability, long-campaign and release requirements remain at their existing statuses.
- New completion documentation must mark only S16 complete and G3 earned if qualified, advance the next session to S17, retain the stop-after-S16 boundary and require a new instruction for S17. Until proof outputs pass, keep S16 in progress and G3 unearned.
