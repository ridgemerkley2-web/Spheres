# S19 candidate integration review

**Reviewed subset passes. S19 is still in progress, not accepted or certified.**
Claude's handoff remains `in progress` at `6289c4a0`; this review does not change it.
The active playset stays on `codex/campaign-certification`. Candidate branch:
`codex/s19-review-01`.

## Exact candidate

- Integration base: `5b46e40af8a2ee13b0c1c7186ede0fdd89302e76` (S21 and S22 preparation).
- Claude implementation snapshot: `6289c4a0`, including `495a7a4d` and `aa46add0`.
- Clean merge: `d35dae5c4b29658a739cad1ecf07b552a342cbe3`.
- Binary SHA-256: `02970ec69a2f4f5ccc03be8fe32bce28d6d47a9116b7cf2070e55cd7d6b22723`.
- No integration source repair was necessary. Shared `main.rs` / `index.html`
  changes merge cleanly, retaining S21 campaign receipts and the repaired renderer.
- [Manifest and evidence hashes](manifest.json). Runtime built from a clean candidate;
  documentation is added afterward. Windows release build; Linux not checked here.

## Results

| Check | Result |
|---|---|
| Focused tutorial/advisor/controller/host tests | 114 passed, 1 skipped, 0 failed |
| Native guidance endpoint/outcome tests | 22 passed |
| Locked native release build | Passed |
| Real disposable France campaign | Passed at desktop 1440×1000 and narrow 390×844 |
| Original saves / protected worktrees | Eight saves and both worktrees preserved |

The browser starts France using the nation selector, changes construction funding,
orders a small workshop, advances a day and reads the actual native outcome records.
Guidance recognizes the enacted budget and paid construction. A second day with
zero funding spends nothing more and retains the dated payment and achieved step.
Canceling load changes no campaign state; named save/load restores exactly the same
construction state under a new session. Reload and Continue retain the valid result.
Reading and skipping lessons do not change campaign achievement. The research,
procurement and air-force advice buttons open the actual Designer, Companies and
Flight → Bases pages with no military/construction commands or day advances.
No browser page errors were recorded.

[Final browser record](candidate-navigation/result.json),
[desktop route](candidate-navigation/route-desktop.png),
[narrow construction result](candidate-navigation/construction-result-390.png).

An initial diagnostic (`candidate-before`) failed because this review harness
incorrectly expected pausing construction to erase `last_spent_bn`. Native records
correctly preserve the previous payment and its date. The harness was corrected;
no game fix was made. The two successful runs and initial diagnostic are retained
separately. The final retained runner matches `candidate-navigation`; the failed
diagnostic is not qualification evidence.

## Remaining handoff work

Claude should finish its existing packet and record exact source/build evidence for
the complete first-hour route, including a real design/research outcome, company
delivery/service and air-force preparation/mission results. These paths have unit
coverage but were not executed end to end by this review. Add later-campaign and
campaign-switch browser checks, then explicitly mark the handoff ready for review.
Codex must rebase/merge and recheck any changes after the pinned snapshot before
acceptance. S20 shared navigation and G4 remain pending.

Usability follow-up for S20: on a 390px screen the equipment header, workflow links,
tabs and saved draft occupy much of the first viewport. The airbase page opens
correctly, but its working controls require scrolling. This review does not award
the later navigation or human playtest gates.

## Reproduction and preview

Run the focused commands recorded in `focused.json` / `native.json` from the pinned
checkout, with a separate Cargo target directory. The retained integration runners
are exact local-layout scripts: put the checkout at `<base>/s19-review-01`, the
release executable at `<base>/s19-review-target/release/spheres-web.exe`, and the
runner at `<base>/s19-route-review.cjs`. Make Playwright available through NODE_PATH.
Run `node s19-route-review.cjs <new-evidence-label> achieved` from `<base>`;
it requires a clean pinned checkout and creates a fresh disposable server/campaign.
The small S07 helper is loaded from the checkout and its hash is recorded.
Neither executable nor the 14 MB disposable campaign is checked in; the retained
launch record identifies the copied save and executable hashes.

Local candidate preview: <http://127.0.0.1:7863/>. Choose Continue, then F1 for the
tutorial. It has France on 3 January 1990 with the reviewed construction result.
Save/resume recognition requires a browser save/load action in that browser;
the launcher's native load is not presented as a player receipt. This separate
preview does not replace the active 7862 playset or any original save.
