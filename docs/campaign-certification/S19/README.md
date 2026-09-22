# S19: outcome-aware first-hour guidance

Status: **ready_for_review** at `f3d4f82ae673859052404529c32853f398019464`, packet
CLAUDE-S19-01 (Claude). This is not a closure record. Codex integrates the work,
reruns the exact-build checks and sets the canonical status. G4 and CP1 remain open.
Evidence and hashes are in the [submission manifest](manifest.json). The route
contract is in [DESIGN.md](DESIGN.md).

## What changed

Guidance now keeps what the player **read** separate from what the campaign
**achieved**. The Tutorial carries a "Your first hour" route of six steps:

1. finances
2. a funded construction project
3. research/design
4. a company purchase
5. air-force preparation
6. save and resume

Each step shows two separate chips:

- **Reading:** Read, Skipped or Not read.
- **Campaign:** Achieved in this campaign, In progress, Not yet or Unknown.

It also lists dated milestones, and for any step not yet achieved it names the
current obstacle, with a button that opens the existing review screen.

A step counts as achieved only on a dated native record for the player, no later
than the reading's day. `/api/guidance` serves these in a new read-only `outcomes`
block (`spheres-web/src/guidance_outcomes.rs`). Save and resume come from the native
`/api/save` and `/api/load` responses, captured by a host hook. The following can
never complete a step:

- reading a lesson;
- opening a screen;
- a command response;
- a preview;
- a stale or foreign reading.

Missing data reads Unknown.

The advisors gain one "next step" card. Three rules also change:

- A budget counts as due for renewal only when programmes are enabled.
- A zero treasury balance no longer raises a card.
- Fiscal-recovery distress opens Money and recovery.

A tenth lesson covers the air force. The docs describe the whole contract:
`docs/GUIDANCE.md`, `docs/ADVISOR_MODEL.md` and `DESIGN.md`.

## Coordination patch for Codex

- **`spheres-web/src/main.rs`:** `mod guidance_outcomes;`, one `outcomes` key in
  `guidance_json`, and tests.
- **`spheres-web/ui/index.html`:**
  - Guidance routes `cash_flow`, `air` (with page `bases`), `companies`, an
    equipment tab, and `industry` with `district` and `project_kind`.
  - `window.GUIDANCE_RECEIPT`, plus one hook in `api()` after a successful save
    or load.

Nothing else in the host changes. S20 navigation work can build on these routes.

## Validation

- **Node:** `node tools/ui/run-unit.cjs` gives 1663 pass, 0 fail, 1 skip. The skip
  is a pre-existing archived-fixture test.
- **Focused guidance checks:** tutorial, advisor, outcomes, UI and host.
- **Native:** `cargo test -p spheres-web --release` gives 414 passed, 0 failed,
  21 ignored, after merging integration `5b46e40a`.
- **Adversarial review:** a five-lens review confirmed 16 findings, and all were
  fixed with regressions. Each regression was checked to fail without its fix.

## Browser route (real disposable campaign)

`tools/ui/ci-guidance-route.cjs` ran twice against the release binary built from
`f3d4f82` (sha256 `cde6fee0…d5f`). Both runs passed, with system Chrome 153 and a
free port. The campaign is a fresh France start through the ordinary picker. Every
game action goes through a visible control, and the driver logs the exact selector
it clicked.

| Step | Result on the dated campaign |
|---|---|
| Finances | Enacted with "Enact & advance 1 day": Achieved, 1990-01-01. Matches the `/api/cash-flow` decision. |
| Construction | Starter workshop in Île-de-France. The step read Not yet while the workshop was only queued, and Achieved once work was paid. Matches `/api/production` spend. |
| Research/design | Valid design draft saved: Achieved. Matches the `/api/equipment` draft. |
| Air force | Airbase foundation: Not yet when ordered, In progress while funded, then Achieved on completion, 1990-01-15. Matches `/api/equipment` bases. |
| Procurement | Honest Not yet. The "Establish a manufacturer" obstacle opens Companies & Procurement. |
| Save and resume | Named save: In progress. Load (new session): Achieved, with the other steps re-derived. Reload + Continue keeps it. |

The run also covers:

- stale readings on three paths: close and reopen, a second reading inside the
  dialog, and a real later-day identity race from a second page;
- lesson skip and restart;
- keyboard (Enter on an obstacle, F1, Escape);
- the Advisors next-step card;
- a new campaign that inherits nothing;
- 390px layouts at the fresh and final states, with no horizontal overflow.

There were four commands, one per visible confirm: construction budget, workshop,
design save and airbase. There were 17 one-day advances. Guidance itself sent no
request. No page errors occurred.

The retained files are under `evidence/`:

- screenshots;
- every guidance reading as rendered, accepted and native;
- `result.json`;
- the gzipped named save;
- the repeat run's `result.json`.

## Known limitations

- **Not reachable in the first hour.** A fresh campaign cannot reach these, so the
  browser shows them as Not yet and Node/native tests cover them:
  - procurement;
  - construction completion and site output;
  - squadrons, readiness and missions;
  - design commissioning and development payment;
  - completion of component research.
- **Covered only by Node tests:**
  - receipts arriving from another tab;
  - an unreadable receipt store;
  - a backup load;
  - a new campaign with a different nation.
- **Outside S19:** the airbase review's default name comes from the first listed
  province (`equipment_flight_view.rs:412-420`).
