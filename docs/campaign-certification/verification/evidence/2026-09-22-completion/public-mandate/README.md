# Civilian public mandate: bounded model change

The gradual civilian-crisis confidence penalty previously treated economic
hardship as the same political signal whether an elected coalition retained
broad public support or had lost it. It now multiplies only that penalty by
the national constituency outside a valid governing coalition. Resource
shortfalls, war exhaustion, already-low loyalty and prospective military veto
mechanics remain separate. No coup threshold or acceptance bar changes.

Powell (2012), pages 1020-1021, discusses public legitimacy and the acceptability
of intervention separately from military organizational grievances. This is
support for separating the mechanisms, not empirical validation of the exact
linear multiplier. The model's party shares are constituency proxies rather
than measured approval ratings. [Original paper](https://jonathanmpowell.com/wp-content/uploads/2025/10/powell-2012jcr-determinants-of-the-attempting-and-outcome-of-coups-detat.pdf).

Eligibility requires a completed, unrestricted elected mandate, with no bans
or pending first election. The helper distributes each national bloc share
among that bloc's party support and admits only known, legal coalition parties
with representation. It does not add support for extra seats, foreign backing,
or movements that have no party. A normalized one-party ballot carrying 20%
of the country therefore supplies a 20% buffer, not complete protection.

## Diagnostic before resimulation

The unchanged iteration8 simulation was run with seed 0 for 96 months. A
standalone probe used the extracted new helper to compare the two penalty
readings at each recorded **same pre-tick state**:

| Event after this tick | Party support | Chamber seats | National mandate | Old penalty | Buffered penalty |
| --- | ---: | ---: | ---: | ---: | ---: |
| Guatemala, month 23 | 39.433% | 53.200% | 39.433% | 0.290144 | 0.175732 |
| Sao Tome, month 39 | 82.749% | 93.359% | 48.157% | 0.256265 | 0.132855 |

Sao Tome's party support is conditional on the represented electorate; the
lower national share preserves its unrepresented movements. These readings
do **not** assert that either coup disappears when the new mechanism is active.
Full resimulation and unchanged A1-A10 gates must establish that outcome.

The new focused regression compares broad and minority consent at the same
economy, excludes seat amplification, missing seats, unknown parties,
restricted/interim mandates and unrepresented movements, and verifies foreign
backing, pure reads, unchanged RNG and exact save/load behavior. The old
resource-confidence and prospective-veto tests remain intact. Compilation
and full native validation are coordinated separately by the parent task.

The standalone extracted helper compiled and ran successfully using `rustc`,
without Cargo or source mutation during the probe. [Compact results and hashes](same-state-probe.json).
