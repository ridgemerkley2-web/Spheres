# S21 · Campaign goals, history and continuation

Owner: Codex. Status: in progress. Base: `624d35df53404e74f8ed6c3b44ec39c206ae1e0a`.

S19 remains Claude's claimed tutorial/advisor work. S20 waits for that integration.
S21's canonical prerequisites S10 and S15 are complete. This session does not
award G4, which still needs the other player-journey sessions.

## Bounded implementation

- Campaign overview with national metrics, simulation-authored peaceful aims,
  access to military agendas, and paginated economic/government/military dispatches.
- Explicit end-of-government result, observer continuation, and live successor
  selection after the simulation's USSR/Yugoslavia dissolution. Never invent a
  successor, transfer extra resources or copy the old country's achievements.
- Settle every date through 31 December 2035, then pause for a result review.
  Continuing into later dates is an explicit sandbox choice, retained in saves.
- Existing receipt recovery, campaign/date identity checks and no new simulation
  reward rules. Continuation metadata lives in the campaign save envelope.

## Verification

Native regressions cover endpoint settlement/refusal, stale/duplicate requests,
successor legality and state preservation, goals without rewards, full-history
paging and save/load. Browser checks cover real actions and desktop/narrow-screen
navigation, focus, failure handling and read-only inspection. Authored late-date
and dissolution scenarios must be labelled fixtures; they do not replace S25's
full-length campaign qualification.

Shared-file edits are limited to module/asset/endpoint wiring and a Campaign
dock button plus state-refresh hook in `index.html`. Tutorial/advisor modules and
Claude's handoff remain untouched.
