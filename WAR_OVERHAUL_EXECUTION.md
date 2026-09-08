# Spheres war overhaul: execution and integration plan

Prepared 2026-09-07 against public commit
`d0c95eff3ba3c51f2841522388d9a136bafa3e18` in
`ridgemerkley2-web/Spheres`.

Status: operational warfare version 1 is implemented on `codex/war-overhaul`.
Integrated release verification completed for runtime source `c9c267cf30d3`,
including the population integration from default tip `825f796` and persistent
conflict identities across peace, redeclaration and saved games.

[WARFARE.md](WARFARE.md) describes the delivered model and declared abstractions;
[the release record](docs/WARFARE_RELEASE.md) ties checks and integration status to
their exact revisions. Publication uses the explicit overhaul feature branch and
a pull request against GitHub's verified active default.

Delivered scope includes local combat and cohesion, dated deployment and retreat,
shared military supply, finite equipment and support missions, staff orders,
observed enemy estimates, an operation board, consented peace terms and supplied
garrisons. Existing industry, ammunition, aviation, economy and map authorities
remain their owners.

The sections below preserve the original execution plan, interface targets and
acceptance ambitions. They are not a claim that every proposed measurement or
future balance study was completed. Measured multi-seed panels are descriptive;
historical calibration and human balance/comprehension validation remain outside
the verified claims.

## Outcome and scope

The player should win by choosing achievable objectives, concentrating a finite
force, sustaining its advance, and recognizing when to consolidate or negotiate.
Orders are simple; local military and political consequences provide depth.

The complete overhaul covers local fronts, operational cohesion and retreat,
military supply, equipment roles and strategic missions, staff AI, useful combat
information, war aims, peace and occupation. A playable first increment is a
milestone, not the definition of the completed overhaul.

The acceptance story is a stronger attacker breaking through, outrunning supply,
and stalling. The defender commits reserves to threaten the corridor. The attacker
can restore supply, withdraw with surviving forces, or pursue a limited peace.
Those outcomes must follow ordinary commands and model rules, with explanations
the player can act on.

Keep command at theatre/operation level. Staff assigns forces to districts;
players choose objectives, approach, commitment, reserves and support missions.
Do not introduce individual soldier movement, tactical ship orders or an hourly
engine. Every country continues to use the same system.

## What the current system already owns

| Responsibility | Existing authority | Integration rule |
| --- | --- | --- |
| National force and overseas ceiling | `operations.rs::allocate`, `Snapshot` | Sectors divide an existing allocation; they cannot create another army. |
| Held equipment, serviceability and losses | `arsenal.rs`, `equipment.rs`, `operations.rs` | One physical inventory; preserve custom ground roles and maintenance effects. |
| Escalation, strategic objectives, ROE and access | `commitment.rs`, commands in `lib.rs` | Operation approach is subordinate to these choices, not a replacement price table. |
| Geographic front and control | `front.rs`, `control.rs`, `districts.rs` | The new solver changes local control; every economic reader uses the shared controller. |
| Commercial freight | `logistics.rs`, resource/commerce settlement | Reuse geography/capacity with a military permission policy; civilian routing is insufficient. |
| Daily cadence and deterministic replay | `clock.rs`, `SYSTEMS` in `lib.rs` | Same dated commands give the same world when stepped, batched or resumed. |
| Political resolve and existing peace/conquest | `war.rs`, `Belligerent.resolve` | Retain political willingness separately from formation cohesion. |
| Player input and presentation | `spheres-web/src/main.rs`, `ui/operations-ui.js`, command channel | Rust owns results/quotes; UI sends authenticated intent and displays authoritative state. |

The active designer already contributes fire support, protected mobility,
reconnaissance and air defense. Those roles must survive the move to local combat.
The overhaul must not award the same equipment or technology advantage twice.

## Required design decisions

1. **Local outcomes determine aggregate progress in the new mode.** The present
   front projection must spend a scalar movement budget, including an uncapped
   final sweep. Remove that guarantee only for the new solver. Derive aggregate
   conflict control from the resulting district state. Terrain, frontage and
   supply may then actually stop an advance.
2. **Introduce an explicitly versioned daily operational mode.** Missing fields
   retain the legacy rules and omitted serialization. Keep monthly replay and
   old scalar calibration intact. Enable the new mode for browser play only once
   its complete launch gate passes. Define one-time migration of active wars.
3. **Use one operational readiness measure initially.** It represents cohesion
   and fatigue, not equipment serviceability or political resolve. The player sees
   strength, readiness, supply and political willingness. Add separate troop
   morale only if playtesting demonstrates a distinct decision it would support.
4. **Orders express intent without charging the same commitment twice.** Probe,
   Advance, Breakthrough, Hold and Fighting withdrawal are operation approaches.
   Changing an approach is free; deployment, preparation time, consumption and
   exposure provide its cost. Existing political prices still apply to escalation
   and strategic objectives. New war aims receive one centrally defined quote.
5. **No invented supply units.** National ammunition is currently normalized
   rather than shell tonnage. Define the stock and transport adapter against the
   equipment/ammunition work that is active when implementation begins. Do not
   interpret a 0..1 magazine directly as tonnes or add a second oil inventory.

Milestone 0 records these as a dated superseding amendment in `BIBLE.md` and
updates `SPEC.md`/`CURRENT_ARCHITECTURE.md` alongside implementation. Preserve the
existing capability, access, deterrence, sovereignty and daily-clock contracts.

## Implementation sequence and completion gates

### M0 — Establish the base and contracts

- Fetch the repository, record the default branch and SHA, and use an isolated
  worktree and `CARGO_TARGET_DIR`.
- Recheck the equipment/industry branches listed below before defining inventory
  APIs. Write down which module owns availability, allocation, delivery, use,
  recoverable damage and irrecoverable loss.
- Add the operational mode/version and migration contract. Active-war migration
  preserves control, force, equipment, political resolve and existing expenditure.
- Store operational state in a focused module such as `campaign.rs`; keep broad
  `world.rs` and command-enum changes small and owned by one integrator.
- Capture current daily war scenarios and the legacy baseline before changes.

Done when the contract is implemented, migration is idempotent, legacy saves and
replays are unchanged, and the new mode can be exercised in deterministic fixtures.

### M1 — Ship orders and explanations with a visible operation card

- Add validated operation intent: conflict, target district, approach and reserve
  share. Keep existing national allocation ceilings and escalation constraints.
- Reject nonparticipants, foreign commands, impossible targets, nonfinite or
  out-of-range shares and closed conflicts before mutation or expenditure.
- Implement a pure Rust operation assessment shared by previews and resolution.
  Include fielded/reserve strength, readiness, supply state, dominant constraints,
  likely result range and last resolution receipt. Label unknown ETAs as unknown.
- Extend `operations-ui.js` and the existing conflict sheet with approach controls,
  selectable objective districts and a short staff report. Retain ROE, access,
  escalation and casualty red-line controls.
- Use the existing command receipt/recovery channel. A retry cannot duplicate a
  paid action; a refused order remains visible.

Done when a browser player can issue an order, read its consequence and recover
from a refusal/reload. Previewing cannot consume RNG, allocate stock or change state.

### M2 — Cohesion, local combat, reserves and retreat

- Staff divides each national deployment into bounded operational sectors using
  district adjacency and the selected focus. Assignment uses stable identifiers
  and deterministic tie order. Frontage limits useful concentration.
- Store cohesion with allocated force and reserves. Transfers carry weighted
  condition and preparation; changing allocations, splitting a sector, closing a
  war or loading a save cannot reset fatigue or create fresh strength.
- Redeployment and reinforcements use saved departure/arrival dates and legal
  routes. In-transit force and equipment remain allocated to the same national
  pool and cannot fight elsewhere. Changed focus or staff reassignment cannot
  teleport a ready force between distant sectors or simultaneous wars.
- Resolve all local contacts from one opening snapshot. Apply existing capability
  gates, terrain, ROE and equipment roles at their appropriate local stage.
  Combat costs cohesion and equipment; depleted formations first lose position
  and retreat when a legal fallback exists.
- Derive aggregate control from local outcomes. Preserve legal ownership separately.
  Cap/normalize losses against actual opening deployment across every contact.
- Recovery requires time and adequate supply. Concentrated offensives expose
  flanks and consume preparation. Repeated order switching grants no reset.
- The initial fixture can inject a defined supply-service input while M3 is built;
  that diagnostic adapter must not be presented as delivered military logistics.

Done when a stronger force can break a weak sector, a prepared defender can hold
good ground, reserves can reverse local momentum, and withdrawal preserves some
combat power without teleporting across hostile territory.

### M3 — Connect military supply and encirclement

- Reuse the existing immutable network and infrastructure capacity. Add domestic
  military routes, friendly-controlled endpoints, explicit foreign transit/basing
  consent, reachable gateways and an expeditionary lift constraint.
- Freeze physical control before resolving any conflict. Supply that becomes
  available after a day's advance is usable on the next applicable supply step.
- Select one explicit military/civilian reservation order in `SYSTEMS` and document
  its tradeoff. All users share finite link capacity. Use stable priorities and
  bounded deterministic search; wall-clock performance must not decide access.
- Reconcile national sources, cargo and local reserves. Dispatch is a transfer;
  firing/operation consumes stock once. Demand and tonne conversion, where used,
  are declared modeled quantities grounded in the chosen equipment supply API.
- Tie fuel coverage to the existing oil/equipment-maintenance authority. If that
  authority only supplies a service ratio, label it honestly and do not mint fuel.
- Persist local reserves and real delivery dates. Isolation consumes reserves,
  then constrains operations; reopening a route recovers service after travel time.
- Replace geography-only pocket assumptions in the new mode with actual supply
  reachability. Islands require supported sea supply rather than automatic immunity.
  Surrender requires sustained inability to fight/withdraw, not one blocked tick.

Done when a corridor closure stalls an offensive after its reserves drain, a relief
operation restores delivery, and finite national inventory/transport remains
conserved across simultaneous domestic and overseas wars.

### M4 — Staff AI, intelligence and responsive pacing

- Make AI issue the same validated operation commands and use the same costs,
  force limits, supply permissions and available military intelligence as players.
- Give orders persistence and a deterministic review cadence. AI should prepare,
  protect supply, commit reserves, stop failing attacks and recognize withdrawal.
- Build a server-side observation policy before showing uncertain enemy estimates.
  The global simulation remains exact; opponent strength/supply estimates have
  confidence and age. Reconnaissance updates observations.
- Audit all relevant DTOs, map data and forecast endpoints for hidden military
  values. Hiding text in JavaScript alone does not establish fog of war.
- Produce event-driven, deduplicated warnings for breakthrough, threatened supply,
  encirclement, intervention and peace offers. Respect player pause preferences.

Done when AI can play the acceptance scenario without privileged resources or
hidden-state tactical choices, and players can explain why an operation stalled.

### M5 — Equipment differentiation and strategic air/naval missions

- Preserve active custom ground capabilities; consume each specialist rating once
  at the matching local combat, movement or reconnaissance step.
- Add strategic air intent: reconnaissance, interception, ground support and
  interdiction. Mission availability depends on actual serviceable equipment,
  basing/access, range support, readiness and ammunition/fuel availability.
- Add strategic naval intent: transport/escort and sea denial. Mission allocation
  shares finite holdings and lift with other theatres; sea supply can be contested.
- Specify the interaction between air defense, interception and the air portion
  of incoming attack. An air-defense bonus must not reduce unrelated ground fire.
- Use existing role/capacity APIs from any newly landed aviation/ammunition work.
  No additional named platform census or tactical ship simulator is required.

Done when equipment mixes create distinguishable tradeoffs, air power cannot
occupy territory alone, and ships/aircraft cannot serve unlimited missions at once.

### M6 — War aims, negotiation and occupation

- Add typed war aims distinct from the current resource-district `Conflict.aim`:
  expel an invader, recover specified territory, compel a defined concession or
  seek a government change. Each aim has measurable success and explicit terms;
  government change is not an automatic diplomatic result of winning a battle.
- Build peace proposal, counterproposal, acceptance and rejection commands with
  pure previews and atomic settlement. Humans receive offers through the consent
  inbox; no AI silently accepts terms for the player.
- Base willingness on aim progress, remaining means, political resolve, costs and
  coalition participation. Prevent a minor ally from ceding another member's land.
- Preserve live-party guards and sovereignty/sphere reconciliation when several
  conflicts end on one day. Cede only valid territory and transfer each asset once.
- Allocate garrisons from the same force pool. Resistance depends on local control,
  duration, policy and existing political/economic conditions. Stabilisation costs
  time and resources and should offer a different course from perpetual escalation.
- Occupation still stops the owner's production without awarding inventory,
  treasury or legal ownership to the occupier. Reconstruction uses existing paid
  construction; collateral damage and civilian harm have political consequences.

Done when limited wars can end without conquest, peace terms cannot duplicate
assets, and an easy battlefield victory can leave a costly occupation that the
player can meaningfully manage or exit.

### M7 — Balance, integration and release

- Exercise the full scenario matrix below with both AI and player command schedules.
- Run measured daily balance scans for war duration, local reversals, supply
  shortages, casualties, equipment replacement, escalation and settlement outcomes.
  Select statistical test sizes using each measured variance and a stated effect
  size. Do not widen existing tolerances or use legacy monthly results as proof of
  daily balance. Preserve the repository's false-red and power requirements.
- Measure late-campaign performance in release builds. Bound active contact count,
  route work, report retention and saves. Record environment and baseline; any
  gameplay work budget must be deterministic and tested at exhaustion.
- Run the repository's complete verification and browser playthrough on the final
  integrated source. Update docs and mark only demonstrated features complete.
- Recheck GitHub and integrate using the protocol below. CI results must belong to
  the exact pushed/integrated SHA; a subsequent material change requires revalidation.

Done means all M0-M7 gates are satisfied, the complete player loop is usable, the
final branch fits the active equipment/economy/map systems, and validation evidence
is attached to the release/PR. Partial milestone completion is not the full overhaul.

## Proposed interfaces and daily ordering

The original interface targets below are implemented by `campaign.rs`,
`campaign_supply.rs`, `campaign_peace.rs` and their authoritative web views.

- `OperationOrder`: validated player/AI intent, persisted through saves.
- `OperationalState`: staff allocation, cohesion, preparation, transfers and local
  supply; subordinate to national force/inventory accounting.
- `OpeningCampaignSnapshot`: forces, equipment, all conflict control, permissions,
  supply and observations captured before resolution.
- `resolve_contacts(snapshot, orders) -> CampaignDelta`: pure local calculations
  returning control changes, retreats, cohesion changes, aggregated losses,
  consumption and reports. National settlement commits those effects once.
- `OperationAssessment`: pure, viewer-authorized explanation derived from the
  resolver's actual terms; forecast uncertainty does not consume the world's RNG.

Daily sequence inside the existing authority: apply due intent; service existing
logistics/program steps in their declared order; capture opening campaign state;
reconcile staff assignments and military supply; resolve contacts and mission
effects; aggregate losses/consumption; commit force, front and condition changes;
evaluate political resolve/peace; invalidate derived control/resource views; emit
bounded reports. M0/M3 must name the exact `SYSTEMS` hook and shared-capacity
reservation order before connecting military demand.

Two current code traps have explicit regressions in this plan:

1. `war::resolve_conflicts` takes `w.conflicts` out before its loop. Calling
   `control::controller(w, ...)` inside that loop would lose wartime control.
   Snapshot the whole world's control before the take.
2. `Snapshot::record_loss` currently inserts one value per nation/conflict.
   Calling it once per contact would overwrite prior losses. Aggregate contact
   deltas first, or provide a bounded accumulation API that also preserves the
   correct equipment exposure/mission breakdown.

## Evidence and scenario matrix

| Scenario | Required observable result |
| --- | --- |
| Exhausted offensive | Attacking repeatedly loses cohesion; a supplied pause restores it over time. |
| Terrain and crossing | A prepared difficult approach can constrain total movement in the new mode. |
| Corridor cut/reopened | Local reserves delay shortage; restored routes deliver on real dates. |
| Encirclement and relief | Isolation permits escape/relief first; delayed surrender removes only real force. |
| Multiple wars and reserves | Allocations, casualties, equipment and stock remain nationally conserved. |
| Coalition with zero deployment | An undeployed ally loses no battlefield equipment or force. |
| Overseas/island campaign | Access, gateway capacity, escort/denial and lift constrain supply. |
| Occupied factory and recapture | Shared control stops/resumes work; no free transfer or production. |
| Order toggles and save migration | No readiness reset, free resupply, duplicate force or repeated migration. |
| Redeployment between distant fronts | Saved travel time delays arrival; in-transit forces remain unavailable elsewhere. |
| Fog and forecasts | No hidden military values leak; read requests mutate neither RNG nor state. |
| Limited peace and simultaneous endings | Terms honor consent, live parties, actual holdings and atomic ownership. |
| Dated replay | Single days, batches and mid-war saves produce identical worlds for identical schedules. |

Preserve and extend `tests/military_operations.rs`, `tests/daily.rs`, relevant
equipment integration suites, logistics conservation/routing tests, and existing
war/front/settlement tests in `lib.rs`. Keep
`the_aggregate_tracks_the_scalar_it_replaced` as a legacy-mode invariant and add
separate local-solver tests; do not rewrite it to accept arbitrary new behavior.

For defect regressions, observe the relevant failure before the repair. Pure
implementation-mirroring tests are not enough. Browser QA must cover order issue,
refusal, focus/map selection, visible causes, alerts, peace offers, reload and
continued play. Record remaining human balance/comprehension limits honestly.

Use the actual commands from `.github/workflows/verify.yml` on final source:

```powershell
$env:CARGO_TARGET_DIR = 'C:\Users\ridge\spheres-war-overhaul\target-war-overhaul'
cargo test --locked --release --workspace --no-fail-fast
cargo test --locked --release -p spheres-sim --example campaign_report
node tools/ui/run-unit.cjs
cargo build --locked --release -p spheres-web
npm ci --prefix tools/ui
npx --prefix tools/ui playwright install --with-deps chromium
node tools/ui/ci-browser.cjs
```

These are the planned release gates. The [release record](docs/WARFARE_RELEASE.md)
records completed outcomes and the source revision each check exercised.
GitHub's Verify workflow runs the corresponding suite on Windows and Ubuntu,
including the new operation module and actual campaign browser checks.

## Parallel execution without competing edits

One integrator owns `world.rs`, `lib.rs`, `war.rs` tick wiring, shared DTOs and
`main.rs` routes. After M0 fixes contracts, independent work can proceed on:

- Local contact/cohesion resolver and scenario fixtures.
- Military routing/supply adapter and conservation fixtures.
- Operation UI and authoritative view integration against the agreed DTO.

Merge those modules through the integrator; then staff AI, missions and peace can
be developed against stable interfaces. Do not have several branches rewrite the
central tick/settlement code concurrently. Every milestone includes player-visible
behavior and its own relevant regression evidence.

## GitHub fit audit and final push protocol

The repository search found one matching owned Spheres repository:
`ridgemerkley2-web/Spheres`. The open-PR query returned no open PRs at inspection.
This is an integration audit of the user's game; it is not a recommendation to
import an unrelated engine or copy a third-party war system.

At inspection, the GitHub default `claude/admiring-euclid-a867c9`, `origin/master`,
`origin/feat/art-p0` and `origin/HEAD` all contain the identical public base
`d0c95eff3ba3c51f2841522388d9a136bafa3e18`. Use the verified default as the initial
PR target rather than selecting `master` solely by its name.

| Concurrent line | Observed relationship to public base | Effect on this overhaul |
| --- | --- | --- |
| Local `codex/industry-rebuild` at `40dea66ad9ec316d7668f3b572fac488445305d4` | 67 ahead / 1 behind; working changes present | Strong pending integration line: funded industry, equipment lifecycle, ammunition and aviation; inspect its eventual landed contracts. |
| `origin/codex/resume-spheres` at `fa2b4a025094a904dfa1401aea0f1f4a0d4f2c85` | 66 ahead / 23 behind; contained in industry line | Compare committed equipment/supply APIs; do not blindly merge the whole divergent branch. |
| Local `codex/company-system` | Same committed base; working changes present | Overlaps logistics, equipment, state and UI; preserve working files and coordinate via landed commits. |
| `origin/feat/arsenal-models-codex` | Contained in public default | Preserve current assets; no separate merge is required for these models. |

The committed industry/default comparison overlaps `operations.rs`, `war.rs`,
`equipment.rs`, `world.rs` and web integration. Committed `front.rs` and
`logistics.rs` are identical at those tips; uncommitted work can still change them.
These facts are a snapshot, not a promise about the integration base at completion.

After the complete overhaul is built:

1. Fetch origin again; query GitHub's actual default, current heads, open PRs and CI.
2. Recompute ancestry/diffs and whether the industry/company lines have landed.
   Inspect current equipment, ammunition, fuel, maintenance, freight and economic
   control APIs. Write a fresh fit report tied to exact SHAs.
3. Update the isolated overhaul branch onto the verified active integration tip.
   Reconcile shared interfaces intentionally; do not copy dirty working trees,
   reset another task's files or push to an old feature branch as a shortcut.
4. Re-run relevant regression suites and the complete release/browser gate on that
   integrated source. Include the new war scenarios and migration evidence.
5. Push the overhaul feature branch normally, using an explicit destination such
   as `git push -u origin HEAD:codex/war-overhaul`. Never rely on inherited upstream
   tracking or force-push a shared branch. Open a reviewable PR against the freshly
   verified active default with exact validation results and migration behavior.
6. Inspect CI for that feature SHA on both platforms. Before final merge, re-fetch
   and check whether the base changed; integrate and validate material changes.
   Report the PR/commit and any remaining integration blockers accurately.

This plan's isolated worktree is `C:\Users\ridge\spheres-war-overhaul`, on
`codex/war-overhaul`. Its starting upstream was the public default; publication
names the feature-branch destination explicitly.

## Delivered verification — 2026-09-07

The implementation and integrated runtime at `c9c267cf30d3` completed the local
release checks below. The original GitHub snapshot above remains a planning
record; subsequent integration and publication status belong to
[WARFARE_RELEASE.md](docs/WARFARE_RELEASE.md).

| Check | Recorded result |
| --- | --- |
| Release Rust workspace | 1,291 passed, 0 failed, 70 existing ignored tests across 50 targets |
| Campaign diagnostic observer | 4 passed; original-war identity/history are tracked without changing world state or RNG |
| UI unit suite | 1,064 passed |
| Browser verification | Five suites passed: original recovery/save flow, operation module, actual campaign server flow, industry/Companies and People |

The browser scenarios cover authenticated orders, visible refusal, daily reports,
enemy-field redaction, saved operation/offer restoration and opposing human peace
consent. Desktop and mobile screenshots were inspected. The full-world campaign
panel records real commands, movement, service, force losses and peace outcomes;
it adds no statistical balance threshold or claim of historical fit.

The final base is default tip `825f796`, including the landed industry, shoreline
and population work. Reused conflict IDs were reproduced and fixed before this
gate, including saved diplomatic calls and closed-war policy cleanup. The release
record distinguishes local results, descriptive diagnostics and remote CI status.
