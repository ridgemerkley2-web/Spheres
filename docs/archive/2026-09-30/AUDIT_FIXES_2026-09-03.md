# Game audit repairs — 3 September 2026

Ridge requested: “Fix all of the issues and when you feel good about the update push to git.”
This repair set addresses the eleven reproducible findings from the preceding game audit.
It also preserves the earlier ministry-program and province-GDP implementation in this branch.

## Integration baseline

Reviewed and merged both new upstream commits before repairing the combined tree:

- `7ccb706`: monthly refusal cooling, monthly AI buying, and calendar-month equipment-order coalescing.
- `67e55e0`: the accompanying daily-simulation audit and outstanding design decisions.

The resource stockpile gate, physical freight, annual ministry plans spent daily,
and province value-added ledger remain the authoritative paths. No simulation
RNG, calibrated threshold, historical data row or golden hash was replaced.

## Repairs

| Finding | Repair and regression coverage |
| --- | --- |
| Annexed nations could collect a second queued war victory | Recheck living, distinct parties at settlement and inside transfer helpers. Chained and competing conquests run in both cadences; no dead seat receives GDP, population or districts. Cancelling a war also retires its daily clocks so reused IDs do not inherit its age. |
| Refresh offered no way back to the live/saved campaign | Campaigns offers Continue, Load saved campaign and a separate new-campaign picker. Replacement requires confirmation; campaign-specific readers/caches reset. Loading states explicitly that history restarts at the saved date. |
| Accepting an old trade offer bypassed current hard conditions | One pure acceptance guard checks expiry, living parties, commitments, diplomatic bars and land ownership before mutation or political cost. The previously offered price is not renegotiated. |
| New drafts vanished when an earlier turn returned | A turn owns a frozen submitted batch. Only unchanged submitted objects retire; edits added or replaced while it is in flight remain queued. |
| Small-country debt and interest used a fictitious $100m GDP floor | Open-book ratios use actual positive output. Transfers, war terms, project output and other GDP changes refresh the derived ratio without billing cash twice. Closed-book arithmetic remains unchanged. |
| New-game response reported monthly rules before daily rules were installed | Browser rules are installed before its first state response and history snapshot. |
| Invalid advance commands disappeared while the calendar moved | Parse the complete batch first. A malformed batch returns a definite rejection, without commands or time changing. Valid gameplay refusals still follow the existing per-order tick semantics. |
| Save/advance failures were invisible and retry could be ambiguous | Show errors, retain unconfirmed drafts, use campaign-bound request receipts for safe retries, and preserve them across browser reload. Conflicting/stale receipts require explicit review instead of blind replay. |
| Globe opened at the north-pole sentinel | Use an uninitialized camera center until the first layout; an actual zero coordinate remains a valid dragged position. |
| Prepaid equipment procurement could wait forever at a full but insufficient reserve | Preserve the ordinary monthly reserve, adding the exact next prepaid-inclusive recipe once. Pending freight counts toward purchasing. Tests cover staff and directed procurement, real arrival delays, exact save/resume and no second prepaid expense. |
| Large finite research shares overflowed their total and zeroed spending | Command validation, restored-state shape repair and live normalized weights share one finite-positive-total check. Rejection spends no political capital. |

## Verification

New deterministic regressions were run against the faulty implementation before
the corresponding fixes. Existing assertions were retained; VM fixtures were
extended for the new session state, and the existing province microstate fixture
was completed with both treasury and debt stocks so it actually opens its books.

Watched release workspace result: **526 passed, 3 pre-existing failures, 54
ignored**. The unchanged legacy actual fingerprints remain
`0xe26e4bf8d6c60066` (start) and `0xbe94d6125631829c` (run).
All nine Node UI suites pass (**136 tests**); the web Rust suite was rerun after
the last layout changes (**122 passed, 2 ignored**). Real Edge/Playwright checks exercised lost
committed responses followed by reload/Continue/retry, atomic malformed-batch
rejection, in-flight Cabinet edits, save errors, save/load, history-fetch failure,
and six rooms at 1440px and 414px widths without unhandled browser errors.
The separate daily-logistics browser check passed with eight real cargo rows,
one recorded arrival, and the land-only route-policy change.

The release workspace suite, all nine Node UI suites, and real-browser checks
are the release gates. The real-browser script is
`tools/ui/check_session_browser.cjs`; it must run against a **disposable server
in a disposable working directory**, because it deliberately saves and loads
its test campaign. No existing user campaign or save is reset by this workflow.
`tools/logistics/check_browser.cjs` uses the visible More menu for multi-day turns.

Visual QA additionally caught and repaired the campaign-action row overlapping
the nation picker and background menus remaining above a newly opened room.
The final rebuilt binary passed real hit-testing of Load, New, France and Start
at 1440×1000, 414×1000 and 820×700, plus the complete browser recovery and freight
checks again. Existing user servers and saves were left untouched.

The combined daily invariant test runs enrolled budgets and the province ledger
for both USA and Tonga campaigns, checks daily financial consistency and exact
save/load continuity, then reconciles every living nation's sectors and provinces.

### Pre-existing calibration work is not silently closed

Three failures predate this repair set:

- `tech::tests::the_1990_endowment_does_not_move_year_one_growth` — BUGS E-3 documents the moving world-reference calibration issue and the choices that need a design decision.
- `tests::the_1990_start_is_pinned` — existing stale pin.
- `tests::golden_hash_of_a_known_run` — existing stale pin.

The repository's documented protocol prohibits repinning while the calibration
failure remains. None of those tests was weakened, ignored or repinned here.
Other entries in BUGS requiring new gameplay/data rulings remain separate from
the eleven confirmed implementation failures this update repairs.
