# Army funding inverse — diagnostic and bounded repair

Iteration 9 still produced fifteen removals of elected governments in each of
the two diagnostic 252-month runs, seeds 0 and 1. Their thirty events were
concentrated in São Tomé (7), Myanmar (7), Comoros (6), Guatemala (5), Guyana (2),
Suriname (2), and Georgia (1). These two trajectories do not replace the
prescribed A1 cohort or alter its acceptance thresholds.

The [pre-coup readings](pre-coup-fiscal.json) distinguish actual budget shortages
from an incorrect funding calculation. Initial coups were fiscally constrained.
In thirteen later pre-month states, the existing fiscal calculation could
afford the resources needed for the AI's intended 0.40 Army target. The old
inverse funded 0.40 **before** the existing war and civilian-confidence losses;
the actual pillar then deducted them. For example, seed 1 São Tomé at month 92
spent 2.0932% of GDP, exactly its old floor. Its raw target was 0.40 and its
actual target 0.221968. The corrected same-state cost was 3.9565%, below the
4.6441% available while retaining civilian expenditure and debt service.

The repair inverts the actual current target: its existing confidence penalty
and war loss are included when calculating required resources. The funded
fraction is capped at one full basket; unaffordable or overwhelming political
losses can still leave loyalty below the desired margin. The annual decision,
command price, gradual loyalty adjustment, spending hysteresis, fiscal cap,
player/owned-budget/program guards, and prospective programme veto are
unchanged. No new borrowing, free loyalty, historical headcount, resource-cost
coefficient or coup threshold is introduced.

The new regression exercises the real `pillar_targets` calculation after a
paid `SetMilSpend`: comfortable funding reaches 0.40, a smaller positive fiscal
allowance remains capped without cutting civilian spending, and severe losses
stop at full resources while the actual target remains below 0.35. It also
checks political-capital payment, no immediate loyalty change, pure reads,
save/load and flag-off behavior. The existing annual-AI and owned-budget tests
remain in place.

The [focused receipt](focused-receipt.json) records a passing external Rust test
using the exact extracted new function and real private target function linked
against the iteration-9 engine. Restoring the old inverse in that **external
copy only** makes the regression fail at its actual-target assertion, reading
zero rather than 0.40. No repository source was temporarily mutated. The final
combined native build subsequently passed the [86 iteration-10 focused checks](../political-calibration/iteration-10-focused-receipt.json),
including this regression. The original calibration still fails A1 concentration
and A5 returns; final workspace and larger-cohort checks remain pending. This
diagnostic does not claim that any particular coup disappears.

All rows are pre-month snapshots, so an intra-tick trigger can differ slightly.
War exhaustion was zero in these thirty snapshots. Georgia lacks an embedded
opening personnel reference and used the legacy spending-share target; its
resource-cost alternative is recorded as unknown. A separate iteration-10
repair saves a dated first-observed population for such successors, retaining
model-estimate provenance; it does not alter these earlier readings. Repeated returns to civilian
government also preserve substantial army autonomy, but no lifecycle change
is inferred or implemented from these two runs.
