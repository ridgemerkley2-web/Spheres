# Tutorial and advisors

The optional Guidance room connects existing game screens without issuing orders. Open **Guided tutorial** from the main menu, **Tutorial** or **Advisors** above the map, the Economy advisor card, the floating Guidance button, or **F1**. Opening Guidance pauses the clock.

The nine lessons cover the map and time, budgets, construction effects, industry operations, government, component research, equipment procurement, world affairs, and saving. Players can select, skip, complete, or restart any lesson. Completion records reading only; it does not claim that construction, research, or a purchase succeeded. Following a lesson leaves a return-to-tutorial button available during play. The searchable Field guide explains common terms.

**Your advisors** reads current conditions and presents reasons, evidence, cautions, and links to existing review screens. Filters and temporary hiding help narrow the briefing. Hidden recommendations can be restored. A missing reading is not interpreted as a healthy economy or a completed action.

## Architecture and boundaries

- `spheres-web/ui/tutorial-model.js` owns the immutable lessons and validated reading-progress transitions.
- `spheres-web/ui/advisor-model.js` derives a bounded set of review recommendations from authoritative state and matching production data. It sends no commands and does not simulate outcomes.
- `spheres-web/ui/guidance-ui.js` owns the dialog, session controller, filters, glossary, navigation adapter, and browser preference `spheres.guidance.tutorial.v1`. Reading progress is shared across campaigns in this browser, separate from campaign saves. Storage failures retain progress for the current visit and display a notice.
- `GET /api/guidance?session_id=...` requires the exact active campaign session. Under one Game lock, it returns `{state, production}` using the existing serializers. Before nation selection, production is null. Reading does not select a nation, enroll a budget, settle work, or advance time.
- The controller rejects replaced local state, stale responses, and mismatched session, player, turn, or date. State changes invalidate recommendations. Busy orders and unsupported routes block navigation. Construction suggestions open a fresh effects review; the existing confirmation screens retain authority over actions.
- Native dialog behavior, semantic focus restoration, and the document-level open-dialog shortcut guard keep keyboard controls within Guidance. Escape closes the room. Refreshing advice cannot authorize game shortcuts behind it.

## Compatibility and verification

This is additive work on `codex/resume-spheres`; existing uncommitted leadership work is preserved. Upstream was fetched and reviewed at `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`. Both `origin/master` and the remote default `claude/admiring-euclid-a867c9` pointed there. Those changes **have not been merged** and require later reconciliation.

Upstream construction uses `production.industry_rebuild.enabled`, shared capacity, per-project and mine allocations, and physical inputs. Its facility operations distinguish potential capacity from dated output receipts. Advice must detect that model before applying funding-only assumptions; unsupported sized `industry_preset` suggestions must not become legacy workshop orders. Upstream fiscal recovery and industry navigation also need deliberate integration.

Run the focused checks from the repository root:

```powershell
node --test tools/ui/check_tutorial_model.cjs tools/ui/check_advisor_model.cjs tools/ui/check_guidance_ui.cjs tools/ui/check_guidance_host.cjs
$env:CARGO_TARGET_DIR='company-sim-target'
cargo test -p spheres-web --release --bin spheres-web
git diff --check
```

The native guidance tests cover no-player reads, exact session matching, state/production/date/player consistency, and unchanged serialized world state. JavaScript checks cover progress recovery, controller races, rendering, and navigation boundaries. These checks do not constitute a live browser playtest.

No updated game preview has been launched for this build. Earlier automatic approval review rejected the separate preview launch before execution with “blocked by policy” and no further stated reason; the existing served game remains the older binary. New assets and `/api/guidance` become available only when a later authorized launch serves the rebuilt executable.

An interactive review using the actual UI modules and illustrative data is available at `http://127.0.0.1:7841/tools/ui/guidance-review.html` on the existing static server. See `GUIDANCE_VALIDATION.md` for the final full-suite results and browser checks.
