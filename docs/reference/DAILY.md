# One daily clock

Approved by Ridge, 2026-09-03: “Can we get everything on a daily ticker not monthly?”

## Player contract

Each +1 day runs the whole simulation. GDP and province population, treasury,
stockpiles, market dispatches, research, manufacturing, front movement and
political recovery can change that day. Goods are unavailable until arrival.
Decisions have their real issue date. +30 days means thirty daily simulations,
not one end-of-month settlement. Browser fast-forward stops on major events.

Annual budget dials remain annual authorizations; spending and interest accrue
daily. Multi-year contracts, elections and technology availability dates keep
their authored duration. Large one-off purchases, political costs, territorial
transfers and event shocks are not divided by thirty.

## Accounting and time

- `clock::month_fraction`: 1 / the current Gregorian month's 28–31 days.
  `year_fraction` is that / 12. A full year therefore contains twelve equal
  budget shares, including leap years; daily amounts vary slightly by month.
- Stocks, targets, capacities expressed as levels and dimensionless ministry
  multipliers are not time-scaled. Mutation sites scale actual flows.
- Monthly convergence `r` becomes `1-(1-r)^dt`; retention factors become
  `factor^dt`. A recurring monthly event probability uses the equivalent daily
  hazard. Fixed-input cumulative hazards are preserved, not the exact sequence
  of monthly random events.
- Economic innovations are drawn once per calendar month and saved, then
  applied across its days. This avoids amplifying noise by drawing a full-size
  monthly GDP shock every day. All random state remains the world's seeded RNG.
- Mines, procurement orders and contracts carry sparse daily timers. Freight
  carries dispatch/due day numbers. Old saves initialize missing daily timers
  from remaining month-unit terms; old month-only freight cannot recover an
  unrecorded historical loading date. Migration never invents a past day.
- Freight corridor reservations reset daily. Dispatch removes the seller's
  goods and charges once; delivery adds to buyer stock once. Held cargo is
  checked each day. Market/resource/arsenal ledgers guard duplicate same-day
  posting. The outer simulation tick itself always advances time.
- Reserve policy and recipe APIs can remain monthly forecasts. Daily action
  demand uses `resources::tick_draw`; procurement uses
  `manufacturing::tick_allocations`, including saved purchasing funds once.

## Compatibility and evidence

`GameRules.daily_simulation` defaults false, omitted from legacy serialization.
Browser `play_rules` enables it at boot, new game and load. Legacy command
schemas with per-month contract quantities remain supported and are prorated
when they execute. Sparse save fields retain old monthly hashes when unused.

A legacy save loaded partway through a month first completes its one outstanding
monthly settlement, then automatically enables daily integration. Its earlier
days had not accrued any macro flows; immediately prorating only the remaining
days would silently lose those funds and production. The pending transition is
saved and shown in the browser. No past command dates are invented or replayed.

Daily mode is a new integration cadence, not a claim of identical monthly
economies or historical outcomes. Legacy calibration pins are deliberately
untouched. Daily tests cover conservation, deadlines, daily mutation, replay,
calendar boundaries, leap years and saved timers. Long-run calibration needs
separate daily-mode sampling; a few deterministic smoke seeds are not a census.

### Verification on 2026-09-03

- Workspace compilation passes. Daily simulation filter: 28 unit tests plus
  four integration tests pass, including three one-year seeds and a leap-month
  dated-command/save-resume comparison.
- Full browser-server suite: 111 passed, two existing ignored; the subsequent
  two-days-of-manufacturing-stock regression also passes. Manufacturing's full
  module suite: 14 passed. The pending old-save transition has its own replay
  test.
- Real Edge/Playwright checks on port 7782: +1 day changes GDP and population;
  history records that date; resource/research/manufacturing surfaces show daily
  values; freight has actual due dates; desktop and narrow logistics views work
  without browser exceptions. Re-run with `tools/logistics/check_browser.cjs`
  only against a disposable game (`--allow-new-game` resets that server's game).
- The full simulation unit suite remains at the same three known failures:
  `the_1990_endowment_does_not_move_year_one_growth`,
  `golden_hash_of_a_known_run`, and `the_1990_start_is_pinned`.
  Actual fingerprints remain `0xbe94d6125631829c` / `0xe26e4bf8d6c60066`;
  no baseline pins or calibration tolerances were changed.
