# A1 route exposure — next investigation narrowed

Owner: Codex. Requested 30 September 2026 while the independent S25 campaign
matrix continued. Base: `aba3902b7e56804eca6f52edcc32bfe7fbea9b2c`;
local branch: `codex/political-balance-investigation`.

**A1 remains failed.** This investigation adds a reproducible reader of the
existing seed-0 trace, separating access to the electoral-coup route from its
firing conditions. It establishes no additional gameplay defect or justified
coefficient change. Both original A1 acceptance limits remain unchanged.

## Findings

These nine names are examples in the existing A1 test comment, not a newly
accepted historical sample. The earlier [historical contract audit](../../../verification/evidence/2026-09-22-completion/a1-historical-contract/audit.md)
already records mixed event categories and absent countries. This follow-up
connects those limitations to the recorded simulation path.

| Example | Trigger checks | Eligible crisis checks | Eligible hostile/crisis checks | Recorded observation |
|---|---:|---:|---:|---|
| Thailand | 252 | 0 | 0 | Electoral at all 252 later funding observations; required crisis absent. |
| Haiti | 0 | 0 | 0 | Non-electoral at all 252 funding observations; no electoral trigger recorded. |
| Algeria | 24 | 0 | 0 | Leaves electoral status between trigger and funding on 1 December 1991. |
| Peru | 252 | 211 | 0 | Eligible crises occur without a hostile army. |
| Sierra Leone | — | — | — | Outside the reviewed roster. |
| Gambia | — | — | — | Outside the reviewed roster. |
| Niger | — | — | — | Outside the reviewed roster; it is not Nigeria. |
| Pakistan | 252 | 0 | 0 | Electoral at all 252 later funding observations; required crisis absent. |
| Honduras | 252 | 0 | 0 | No crisis coincides with an eligible trigger. |

This does not prove those countries should have had coups. Scripted country
events or adding other event categories to the counter would not repair A1.

Funding observation follows the trigger and election processing. There are
**13 same-month exits** from electoral status between those sites, including
all ten recorded elected-government coups. Algeria has 24 electoral trigger
checks but only 23 electoral funding observations. A later regime snapshot must
not erase an earlier opportunity or be treated as its input. The other three
exits are not relabeled as coups.

## Reproduction and checks

```sh
python -B -m unittest tools.campaign.test_a1_route_exposure
python -B tools/campaign/a1_route_exposure.py --packet docs/campaign-certification/S27/preparation/a1-firing-observer-20260928 --out /new/path/route-exposure.json
```

Run from the repository root with a new output filename. The reader checks
compressed/decoded trace bytes and native-result identity, then reconciles
**146,525 observations**, all stage totals and ten firing cases/headlines. It
checks contemporaneous guards and rejects duplicate or contradictory outcomes.

The roster, government and A1 census Git bytes match observer candidate
`6818e4f0d94b01c86d7a9acc4252260947d13504` at the reviewed base. This is source
applicability evidence, not a fresh simulation or a claim that all sources match.
[route-exposure.json](route-exposure.json) retains pins, revisions, 158 observed
countries, three absent examples, intervals and original line-number witnesses.

**10 focused tests pass**, covering same-tick changes, non-coup exits, guard
boundaries, duplicates, contradictions, altered thresholds, roster aliases,
missing months and firing chronology. All trigger totals, branches and
eligibility/crisis counts agree with the previous independent geography report
for all 118 trigger countries. [validation.json](validation.json) records checks.

The first reader integration run stopped before output because it treated the
native headline array as a scalar count. It now validates the array, length and
event months, with a regression. This was a tooling failure, not a native failure.

## Next investigation

Trace actual regime-opening decisions on the already-used seed-0 Haiti trajectory:
public demand, military-executive preferences, affordability and eligibility.
The existing trace does not retain every opening-decision input, so it does not
establish a missed command. Separately, a civilian–military political-conflict
mechanism needs an observable dispute and a reviewed contract. Autonomy alone,
country names and desired event frequency are not that dispute. Quiet funded
forces, resolved conflicts and strong civilian control remain counterexamples.

Only a demonstrated correction should proceed to a declared candidate and the
unchanged A1–A10 checks. Earlier findings and rejected trials remain intact.
This uses one development seed, not independent samples or a causal treatment
experiment. No native game, holdout, simulation source, save, campaign worker or
frozen matrix input was changed or run. S27, G5 and CP1 remain unqualified.
