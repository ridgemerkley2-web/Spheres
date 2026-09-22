# Tutorial and advisors

The optional Guidance room connects existing game screens without issuing orders. Open **Guided tutorial** from the main menu, **Tutorial** or **Advisors** above the map, the Advisors card in the Economy cabinet, or **F1**. The campaign hides the floating Guidance button; standalone pages such as the review page show it. Opening Guidance pauses the clock.

The ten lessons cover the map and time, budgets, construction effects, industry operations, government, component research, equipment procurement, the air force, world affairs, and saving. Players can select, skip, complete, or restart any lesson. Completion records reading only; it does not claim that construction, research, or a purchase succeeded. Following a lesson leaves a return-to-tutorial button available during play. The searchable Field guide explains common terms.

**Your first hour** is an optional six-step route shown after the lessons: enact a yearly budget, fund construction, research toward a design, buy equipment, prepare an air force, and save and resume. Each step shows reading progress (Read, Skipped, Not read) separately from the campaign result (Achieved, In progress, Not yet, Unknown). A result counts only when this campaign's own dated native records show it; reading a lesson never completes a step. One line beside lesson progress summarises the route. The contract is in `docs/campaign-certification/S19/DESIGN.md`.

**Your advisors** reads current conditions and presents reasons, evidence, cautions, and links to existing review screens. When campaign results are available, it adds at most one card for the next unmet route step. Filters and temporary hiding help narrow the briefing. Hidden recommendations can be restored. A missing reading is not interpreted as a healthy economy or a completed action.

## Architecture and boundaries

- `spheres-web/ui/tutorial-model.js` owns the immutable lessons and validated reading-progress transitions.
- `spheres-web/ui/advisor-model.js` exports `evaluate(state, production, outcomes, receipts)`, which derives a bounded set of review recommendations, and `recognize(response, receipts)`, which derives the six-step route. Both are pure: they send no commands and do not simulate outcomes. See `ADVISOR_MODEL.md`.
- `spheres-web/ui/guidance-ui.js` owns the dialog, session controller, filters, glossary and navigation adapter, plus two browser keys:
  - `spheres.guidance.tutorial.v1` holds reading progress, shared across campaigns in this browser and separate from campaign saves.
  - `spheres.guidance.receipts.v1` holds validated copies of native save and load responses (at most 20 of each). Stored rows are merged before every write and every refresh, and a `storage` event from another tab re-reads the current result. A record that cannot be read vouches for nothing and is never overwritten.
  - Storage failures keep progress and receipts for the current visit and display a notice.
- `api()` in `index.html` calls `window.GUIDANCE_RECEIPT({path, body, data, state})` after a successful `POST /api/save` or `POST /api/load`. Guidance validates the response before storing it, and a failure there never changes the save or load result.
- `GET /api/guidance?session_id=...` requires the exact active campaign session. Under one Game lock, it returns `{state, production, outcomes}`. `outcomes` holds the player's dated native records from `spheres-web/src/guidance_outcomes.rs`. Before nation selection, `production` and `outcomes` are null. Reading does not select a nation, enroll a budget, settle work, or advance time.
- The controller rejects replaced local state, stale responses, and mismatched session, player, turn, or date. Outcomes for another nation or day are treated as unknown. State changes invalidate recommendations; a verified route stays visible only as a labelled earlier reading of the same session and player. Busy orders and unsupported routes block navigation. Construction suggestions open a fresh effects review; the existing confirmation screens retain authority over actions.
- Native dialog behavior, semantic focus restoration, and the document-level open-dialog shortcut guard keep keyboard controls within Guidance. Escape closes the room. Refreshing advice cannot authorize game shortcuts behind it.

## Verification

Run the focused checks from the repository root, with a target directory that belongs to this worktree:

```powershell
node --test tools/ui/check_tutorial_model.cjs tools/ui/check_advisor_model.cjs tools/ui/check_guidance_outcomes.cjs tools/ui/check_guidance_ui.cjs tools/ui/check_guidance_host.cjs
$env:CARGO_TARGET_DIR="$PWD\target-guidance"
cargo test -p spheres-web --release --bin spheres-web guidance
git diff --check
```

The native guidance tests cover no-player reads, exact session matching, state/production/outcomes identity, player filtering and unchanged serialized world state. The JavaScript checks cover progress recovery, route recognition against native fields, receipt validation and cross-tab merging, controller races, rendering and navigation boundaries. These checks are not a live browser playtest.

`tools/ui/guidance-review.html`, served from the repository root on loopback HTTP, loads the actual modules with invented scenarios, including a fresh-campaign route. It has no campaign connection and sends no requests to the game.

## History

The first implementation (8 September 2026) was made on `codex/resume-spheres`. Notes from that checkpoint about unmerged upstream `485c223f`, industry-rebuild and fiscal-recovery integration, and a preview launch that was not run are historical; `GUIDANCE_VALIDATION.md` keeps that record.
