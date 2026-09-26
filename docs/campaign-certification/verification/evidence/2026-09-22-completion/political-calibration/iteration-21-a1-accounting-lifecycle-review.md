# A1 accounting and repeated-coup lifecycle review

Read-only review of checkpoint `90fce9475d20049741589de0e5c5e1c1668a26f6` on 2026-09-22. Government source SHA256: `3254020ea00ec763eb95dabe0c03b79e33b861d4144d6de7307627c03de4e51c`. No Cargo, source edits, cohort changes or new outcome runs. The ordinary target executables were rejected iteration 22 when review began; no result from those binaries is attributed to restored iteration 21.

## A1 itself

No actionable event-accounting defect found.

- `spheres-sim/tests/bloc_census.rs:387-402` distinguishes annulment, elected-government coup and regime coup from three distinct production headlines. Only the elected-government branch increments both `coup_el` and the corresponding nation's count.
- `government.rs:8941` is the only production emitter of the elected-government coup headline. `maybe_electoral_coup` returns true after one `break_electoral`, and `tick` immediately continues to the next country, so the same government's regime branch cannot also emit another coup in that tick.
- `lib.rs:1510` clears the monthly headline buffer; `tick_month` returns that month's headlines. Daily mode returns each day's new slice rather than replaying the accumulated buffer. The original gate uses the ordinary monthly rules.
- `bloc_census.rs:652-659` sorts nation counts descending and divides the sum of the three largest by the exact elected-coup total. Ties only affect label order, not the numerator. The zero-event case is explicit, and the independent median-count arm still requires 4 through 14.
- `bloc_census.rs:209-217,985-992` uses an ordinary midpoint median and the strict concentration threshold `< .5`. `census` reruns the fixed `0..12` seeds, not a mutable diagnostic range or a cached different cohort. No rounding enters the assertion.
- The route-priority classifier affects bloc-transition attribution; it does not retroactively replace, multiply or filter A1's direct event count.

## Break, interim and returned government

No stale pre-coup pressure or missing office-age reset found in these paths.

- `government.rs:9293-9315` clears pressure, resets office age/fraction, ends completed/pending ballot flags, clears the election calendar and opening mandate, and reseats institutional loyalty (.90 mover/.72 others).
- `schedule_first_elections` at 7007-7032 marks the interim explicitly and clears pressure again. Army/Security retain live loyalty, rather than receiving an invented payment. The interim cannot trigger `maybe_electoral_coup`.
- `hold_election` at 7082 onward completes or annuls the actual ballot. A first free military handover seats the civilian winner, clears the pending marker only after succession, and `form_government` resets office age.
- `record_identity`/`remembered_record` at 6507-6575 retain a continuing actual incumbent but switch military regime history to the new party identity after a completed handover. `electoral_coup_settled_months` only reuses a record matching that governing party; it does not spend an unrelated old party's tenure to bypass the twelve-month protection.
- Same-party renewals intentionally do not erase accumulated governing history. Restored source includes the reviewed snap-election grace repair and successor-calendar repair. Neither is an accidental repeat-event multiplier.
- The trigger rechecks actual current Army hostility and discontent as well as accumulated pressure; a recovered Army cannot fire solely from its old gauge.

The retained copied-build-20 distribution diagnostic observed real returns between repeated coups and attributed 21 of 28 repeated events to Myanmar and Guatemala. This is supporting prior evidence, not a fresh iteration-21 trajectory claim. A1's remaining concentration is consistent with recurring modeled resource/crisis conditions, not a duplicate event counter.

## The Party-loyalty question

`blocs.rs:554-562` deliberately prefers any Party pillar for passive round-table drift, without checking the current executive's tie. An Army coup can re-seat a historical Party pillar through `seat_spec_pillars`, and the Party target later follows the current ruling movement's domestic share. Thus a weak Party apparatus can enable the passive opening rule despite a strong Army. This follows the helper's documented S4 semantics; changing it to the executive's actual institution would require an explicit model interpretation, not merely fixing a transcription typo.

It cannot explain the dominant Myanmar and Guatemala repeats: Myanmar's spec has Army/Security/Business, while Guatemala receives only its sourced Army overlay. Comoros has Army only. Sao Tome and Guyana do have Party pillars, so they are the bounded cases in which the role question could matter.

The paid AI round-table is distinct: `government.rs:8315` explicitly allows the civilian-franchise quorum even with strong armed institutions, and the action refusal shares that condition. For a pillar-led government with multiple civilian parties, `franchise_demand` treats those parties as excluded. This is the approved voluntary-opening mechanism, not a mistaken reading of weak Party loyalty. `ai_stratagems` charges the real action on its existing draw. Disabling it after coups or adding an arbitrary waiting period would be a policy change, not a discovered accounting repair.

## One confirmed separate correctness defect

`government.rs:9286-9288` reduces GDP by 3% in `regime_break` but omits `economy::refresh_debt_ratio`, although that helper's contract (`economy.rs:317-329`) requires refreshing the derived ratio after open-book GDP changes. The uprising and generic-collapse settlements already call it. Economy settles before government; there is no unconditional later refresh, and politics reads the stale ratio. Nash independently confirmed that a later load migration can repair a value that uninterrupted play retains.

This matters for open-book state/save and fiscal-read consistency, but **does not explain A1**: the original political census uses closed-book states whose independently simulated ratio is intentionally unchanged by `refresh_debt_ratio`. A narrow fix should preserve debt dollars/cash, refresh only the derived ratio, cover both direct command and normal tick boundaries, and leave the closed-book route byte-identical. No fix was applied in this review.

## Decision

No A1 metric, event classification, repeat reset or count-threshold defect is established. Do not alter those definitions or add ballot capability erosion based on this audit. The debt-ratio omission is a real independent repair opportunity; the remaining coup concentration still needs a separately justified behavioral diagnosis.
