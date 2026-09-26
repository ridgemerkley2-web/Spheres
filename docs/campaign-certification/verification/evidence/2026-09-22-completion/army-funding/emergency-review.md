# Midyear appropriation response

The [candidate-11 lifecycle trace](candidate11-lifecycle.json) uses the same
development seeds 0 and 1 for 252 monthly steps. This is the `.024` candidate,
not the earlier iteration-10 binary or the reserved independent cohort. It
records ten and seven removals of elected governments: twelve first coups and
five repeats. No removal of an unvoted interim, retained military holder after
a genuine civilian handover, or misattributed civilian performance record was
identified. Existing material grievances persist across an election; the
repair does not clear them or reset pressure.

One remaining response delay is explicit in São Tomé, seed 0:

| Observation | Spend, GDP share | Affordable share | Current required share | Army target / actual loyalty |
| --- | ---: | ---: | ---: | ---: |
| January 1998, one-month civilian record | .020914 | .058794 | .020914 | .400000 / .346 |
| July 1998, seven-month civilian record | .020885 | .073541 | .041048 | .206740 / .327 |
| Immediately before month-108 coup | .020885 | .085767 | .041463 | .202350 / .276 |

The January decision correctly prices the then-current resource need. Once
the new government's confidence loss becomes established midyear, a useful
appropriation is affordable, but the old annual-only rule makes no decision
until the following January. No future confidence loss needs to be forecast:
the shortfall is already visible in the current state.

The bounded policy repair retains January's existing review below `.50`
loyalty and adds a calendar-month-end emergency review when actual Army
loyalty is below the existing `.40` desired margin. The funding floor must
still exceed current spending by at least `.001`, pass its fiscal and budget
authority guards, and be purchased through the normal `SetMilSpend` command
with political capital. The calculation does not buy a prospective veto,
increase debt authority, mint funds, alter loyalty directly, or defer coups.

The [focused receipt](emergency-focused-receipt.json) uses the exact new
`ai_government` function and new test linked against the candidate-11 engine.
It passes the paid July response, retained pressure/loyalty, no repeat payment,
save/load, small-change hysteresis, quiet annual review, unavailable fiscal or
political capital, player/flag-off guards, and equal month-end timing in daily
play. Restoring the annual-only condition in an external copy makes the same
test fail: spending stays `.001` instead of the priced `.03903314547720797`.
No repository source was temporarily mutated. Combined native tests and
campaign measurements must establish the final effect; this diagnosis does
not claim that the A1 concentration requirement now passes.
