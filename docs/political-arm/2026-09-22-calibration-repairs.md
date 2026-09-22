# Political mechanics repairs — 22 September 2026

**Validation status: follow-up validation is pending.** These changes must satisfy the
existing A1–A10 tests and the normal regression suite. No historical acceptance
band or test threshold was widened. The political switches remain opt-in; these
notes do not certify the calibration or enable it in a campaign.

## Defence resources, local costs, and political confidence

The old defence-share calculation made a small force with a small national
budget permanently hostile, regardless of what that force could actually buy.
An intermediate universal dollar threshold had the opposite data problem:
inexpensive conscript forces in poor countries were treated as chronically
unpaid. Resources now use an explicit operating-cost model:

| Quantity | Definition |
| --- | --- |
| Annual defence resources per recorded force member | `defence GDP share × GDP in billions × 1e9 / personnel` |
| Annual operating basket per member | `$2,500 + 1.5 × local GDP per head`, in model 1990 dollars |
| Material loyalty target | `.20 + .65 × clamp(resources / (2 × basket), 0, 1) − .45 × war exhaustion` |
| Local-cost proxy | `GDP in billions × 1000 / population in millions` |

The fixed basket component represents imported equipment and maintenance; the
local component represents personnel and locally purchased support. A second
basket represents full provisioning, reserves, and modernisation. **The dollar
amount, multipliers, and loyalty endpoints are game-design assumptions, not
historical salaries or estimates from an empirical regression.** Defence
expenditure includes personnel, operations, and equipment; it cannot be read
as a soldier's wage. [World Bank expenditure indicator definition](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/MS.MIL.XPND.GD.ZS).
The denominator is the existing total armed-forces personnel series, including
qualifying paramilitaries; it is not a land-Army headcount. The Army pillar
represents the conventional military institution, while this broader denominator
remains a resource proxy. Readouts should say **defence resources per recorded
force member**, retain `Sourced1990` versus `ModelEstimate`, and never label
that quantity salary or Army pay. [Personnel indicator definition](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/MS.MIL.TOTL.TF.ZS).
The direction of the resource effect follows expenditure per member as an
organisational-resource proxy in [Powell (2012), pp. 1026–1029](https://jonathanmpowell.com/wp-content/uploads/2025/10/powell-2012jcr-determinants-of-the-attempting-and-outcome-of-coups-detat.pdf).
That paper does not supply this game's cost basket or causal loyalty formula.

The legacy AI can buy an annual military appropriation through the existing,
priced `SetMilSpend` action. Iteration 10 corrects its inverse to target `.40`
in the actual current Army pillar, including the existing war and civilian
confidence losses. The required funded fraction is
`clamp((.40 − .20 + .45 × exhaustion + confidence penalty) / .65, 0, 1)`.
It stops at full provisioning (two operating baskets per member); an unreachable political margin cannot
justify spending beyond useful resources. The allocation is limited by actual
revenue after civilian expenditure and debt service, and does not authorise
additional deficit borrowing. The separate prospective programme veto is not
priced into this current-target calculation. The fiscal
consolidation routine respects that same affordable allocation. The annual
January review remains. After the candidate-11 diagnostic, an emergency
calendar-month-end review also responds when actual Army loyalty falls below
the existing `.40` desired margin. The unchanged `.001` minimum spending
increase avoids repeated purchases of tiny changes. Player budgets, open fiscal books, programme budgets, and the
fiscal-recovery mode retain their own authority.

The [midyear response trace](../campaign-certification/verification/evidence/2026-09-22-completion/army-funding/emergency-review.md)
records a civilian confidence shortfall emerging after January while a useful
appropriation was already affordable. The emergency review uses that current
need and the ordinary paid action; it grants no pressure reset or direct
loyalty and does not forecast future political losses. Its exact extracted
regression passes; restoring annual-only timing fails. Its compiled regression
passes in iterations 12 and 13. The combined campaign readings below still
fail A1 concentration; this correctness repair is not full certification.

The [thirty-event diagnostic and regression proof](../campaign-certification/verification/evidence/2026-09-22-completion/army-funding/README.md)
retain the distinction between genuine fiscal shortages and later affordable
targets missed by the old inverse. The new test reaches `.40` through a paid
command with no immediate loyalty gift; insufficient funds remain capped and
overwhelming losses remain below `.35` even at full resources. Restoring the
old inverse in an external copy makes that regression fail. The new inverse
and successor-reference tests pass in the iteration-10 focused run; full
calibration and the final whole-workspace checks remain pending.

Resources do not guarantee political obedience. An actual Army pillar in an
electoral state can lose confidence after at least six months of the same
government's recorded performance. The confidence penalty is:

```text
autonomy = clamp((authoritarianism − .20) / .40, 0, 1)
crisis = .5 × current discontent + .5 × clamp(−remembered performance, 0, 1)
target penalty = .65 × autonomy × crisis × (1 − public mandate)
```

This channel is zero below `.25` discontent, without the requisite performance
record, without a real Army pillar, outside electoral rule, or with the lens
off. Civilian control at authoritarianism `.20` or below blocks it. It changes
the gradual loyalty target, not equipment, military strength, or the fiscal
balance. The existing loyalty lag and accumulated coup-pressure rules still
apply. These coefficients are explicit design assumptions.

The iteration-9 refinement measures a public mandate only for a completed,
unrestricted elected coalition. It distributes each bloc's national constituency
among the parties that carry it, then counts the known, represented governing
parties. A single offered party carrying 20% of the country supplies a 20%
buffer even if its conditional ballot or seats total 100%. Opposition, foreign
backing and unrepresented movements add no governing consent. Missing history,
bans and an unvoted interim supply no buffer. This multiplier only limits the
gradual crisis-confidence penalty; material shortfalls, war exhaustion, prior
loyalty and the separate prospective programme veto remain effective. Powell's
discussion of legitimacy on pp. 1020–1021 motivates this distinction, not the
linear coefficient. [The same-state diagnostic](../campaign-certification/verification/evidence/2026-09-22-completion/public-mandate/README.md)
compares the proposed formula without claiming that the old trajectory used it.
Iteration 9 measured these combined effects; its unresolved failures and the
larger development cohort are recorded below.

At a completed election, prospective acceptance of a **new** Communist or
Islamist programme also reads institutional autonomy and that winner's actual
seat share. The same established-government requirement applies;
the additional veto term is `.45 × autonomy × winner seat share`. Strong army
loyalty, civilian control, or retention of the incumbent
programme resists that term. The material-grievance route retains its existing
`.25` discontent requirement; a credible institutional veto independently
constitutes a political crisis even when prices and output have recovered.
The `.35` loyalty threshold, monarchy exception, and authoritarianism guard
remain. A financial appropriation is not an automatic cure for political
opposition to civilian authority.

## Missing personnel are estimates, not new historical records

The embedded roster currently contains **125 valid, positive sourced
personnel/population pairs**. Their median force share is
`0.005431034482758621` people per resident. When the political lens is enabled
and a force has no sourced personnel count, that ratio supplies a model force
size proportional to its **opening population**, matching the sourced series'
reference date. The iteration-9 correction prevents later population growth
from recruiting additional model soldiers only in countries with missing data.
Neither sourced nor estimated counts currently model a changing force size.
Invalid live population still rejects the missing-data assessment. Iteration 9
measured this correction and passed its source-parity regression. It also
exposed a missing reference for successors without an embedded opening record.

Iteration 10 gives such an unsourced successor a saved, explicitly dated
`army_population_reference`: its first valid population observed by the
political-lens government setup. This observation remains fixed as population
changes and across saves. An older successor save receives a current-date
observation, not an invented 1990 population. Dead or invalid states do not
create it; sourced forces and embedded opening estimates retain their original
paths. The flag-off path does not create this metadata. The assessment remains
`ModelEstimate`, and the historical source API still returns `None`.

`ArmyPersonnelAssessment` exposes `Sourced1990` or `ModelEstimate` alongside
the quantity. For example, Comoros's opening population gives approximately
2,238 **modelled** members. This is **not** a claim about the historical size
of its Presidential Guard. `data::army_personnel_1990` continues to return
`None`; no nation source or save is populated with invented historical data.
An explicitly supplied zero, negative, or non-finite count is not replaced by
an estimate. Invalid population or force-share inputs are rejected.

The median is derived reproducibly from the embedded roster, rather than
chosen for a particular country. The flag-off path retains the original
behaviour. Diagnostic callers must retain the assessment's provenance when
displaying an estimated force or resources per member.

## Programme continuity and event attribution

An uncrowned stability collapse changes order, output, and authoritarianism.
It does not identify a movement or install a new political programme. Previously,
crossing the electoral boundary discarded the live governing party's bloc and
resurrected the programme described in a historical leader row. The generic
collapse now preserves the actual elected programme and party-derived movement
shares when it closes electoral rule. Saving, loading, and lazy government
seeding preserve that result. A real crowned uprising can still install its
winning programme. The flag-off numerical collapse remains identical.

Internal regime coups similarly retain the governing programme unless an
independently organised domestic alternative has greater support. A party or
established movement must supply that organisation; the coup-making Army or
Security pillar's generic affinity alone is insufficient. The state's Party
apparatus belongs to the **current** governing movement and does not resurrect
its 1990 affiliation following a revolution. Such an internal Party coup still
changes office, causes disruption, and appears in the event record. Foreign
backing alone does not satisfy the domestic-support condition.

The census separately records an uncrowned `Collapse` and a crowned
`Uprising`. If a ballot changes the governing bloc and an uncrowned collapse
occurs in the same tick while the state remains electoral, the ballot owns the
party change. Real movement takeovers, annulments, coups, and collapses that
actually end electoral rule retain non-ballot priority. Collapse counts remain
visible, including A6's separate collapse observations.

## Genuine first elections and physical coercion

An interim cabinet formed when a regime opens is not an elected government.
The saved `awaiting_first_election` marker preserves the first-ballot deadline
across reloads, prevents the ordinary fragile-cabinet clock preempting that
ballot, and prevents the interim cabinet being counted as a government already
removed by an electoral coup. Only a completed, unannulled ballot records an
elected mandate. A new coup, a closed regime or an annulment clears the pending
election consistently. Missing metadata in an older save defaults to false;
the lens-off lifecycle retains its previous behaviour.

The same pending marker now preserves an existing armed supply contest during
an unvoted opening. Announcing a ballot does not feed the incumbent's forces or
settle that contest. A funded interim remains safe; actual sustained supply
failure can still let its established armed opponent prevail. A completed free
ballot or a different incumbent retires that old contest, preserving the
existing protection for constitutional successors. The optional saved module
and both feature guards remain unchanged. This iteration-9 lifecycle repair
is included in the measured results below.

At the first completed free election, an original or emergent military office
yields even if the interim cabinet's leading party wins again. Iteration 7
extends this to the exact generic `head of state` role generated for a Party
or other pillar takeover. This is a generated role identifier, not a guess
from a historical leader's name or ideology. Inherited sourced pillar offices,
including a monarch's heir retaining the King office, remain separate from
the chamber. The historical US presidency/chamber separation is preserved.

Physical suppression now reads actual Army/Security pillars when present.
A weak Party bureaucracy alone no longer proves a loyal armed service unable
to suppress an uprising. A polity modeled solely through a Party apparatus
retains that fallback. Internal Party coups still use the separate political
organisation rule and remain real events; this repair does not erase them.

## Remembered performance and a lawful mandate

Voter accountability now combines one quarter of current performance with
three quarters of a rolling government record. The polity's ordinary term
provides the memory's time constant, with a 48-month fallback. These weights
are game assumptions; saved performance is an estimate, not a reconstructed
historical time series. Sustained recovery can replace the earlier record.
The iteration-9 continuity correction keys that record to the actual continuing
party through an institutional opening and a same-party first ballot. A live
party tie, matching programme and real party-table entry are all required;
an unknown or military interim keeps its regime identity. A leading opposition
list in an unvoted provisional table is not charged the incumbent's record.
A genuine new-party or military-to-civilian handover starts a new record.
Equivalent older regime keys may be relabelled for their observed current party
without erasing recorded months or performance. Save and clock-mode controls
passed in iteration 10; combined campaign effects remain under calibration.
Support losses respond faster than gains, and the complete normalized vote
change is capped at 1.5 percentage points per model month, including the small
pull toward the latest constituency estimate.

The optional `vote_anchor` records vote support at the latest completed legal
ballot, never the electoral system's amplified seat shares. Restricted or
annulled elections cannot overwrite it. Older saves keep their opening
estimate until a qualifying ballot occurs. The `unrestricted_mandate` marker
separately records that the outgoing elected government began through a free
ballot and has not subsequently banned a party. A missing old-save marker is
false: an unknown electoral history cannot earn a consolidation reward.

## Civilian authority after a lawful transfer

The model previously increased authoritarianism after a coup while giving
successful constitutional transfers no converse institutional effect. The
bounded consolidation change reduces authoritarianism by `.05` per full
outgoing elected term, prorated by `outgoing months / constitutional term`
and capped at one term. It requires a completed, legal change of the civilian
governing party: interim cabinets, annulled or banned elections, and same-party
renewals receive no gain. The outgoing mandate must have been unrestricted
throughout; lifting a recent ban does not retroactively make it a free term.
An inherited pillar office remains separate and unchanged. The rule changes
no votes or support and consumes no random draw.

The direction follows the discussion of democratic deepening and peaceful
constitutional transfers in the Solomon Islands and Fiji in
[V-Dem's Democracy Report 2026, printed p. 29](https://www.v-dem.net/documents/75/V-Dem_Institute_Democracy_Report_2026_lowres.pdf).
That comparative account provides **directional grounding only**. The `.05`
coefficient, tenure proration and game-authoritarianism interpretation are
explicit design assumptions, not an estimated effect from that report.
The focused regression requires five transfers after one-fifth of a term to
give exactly the gain of one full-term transfer; repeated snap elections
therefore cannot create a faster consolidation rate. The original focused
consolidation guard passed in iteration 8; later mandate-lifecycle refinements
still require the next combined regression run. That focused pass did not
resolve the outstanding campaign-calibration failures recorded below.

## Explicit institution presence, not inference from a headcount

The `army_institutions_1990.json` overlay initially corrected three independently
verified missing institutions under the political lens:

| Nation | Institution evidence | Scope |
| --- | --- | --- |
| Peru | [1979 constitution, Arts. 273–278](https://www3.congreso.gob.pe/Docs/sites/webs/quipu/constitu/1979.htm) names Army/Navy/Air Force and constitutional command. | Adds the armed-services institution; does not invent hostility. |
| Guatemala | [IACHR 1988–89](https://cidh.oas.org/annualrep/88.89eng/chap.4b.htm) records attempted coups and loyal forces suppressing them; [1989–90 report](https://cidh.oas.org/annualrep/89.90eng/chap4c.htm) records distinct Government and Army participants. | Both loyal and hostile outcomes remain governed by live conditions. |
| Honduras | [USDOS report for 1990, archived by ecoi](https://www.ecoi.net/en/document/1066990.html) describes the elected government and Armed Forces' legal and institutional autonomy. | A yearly source establishes institution presence, not a January loyalty score. |

Seventeen further presence-only rows use the **Defense Forces / Branches**
entries in the [CIA's 1990 World Factbook, electronically republished by Project Gutenberg](https://www.gutenberg.org/cache/epub/14/pg14-images.html).
The edition generally uses information available on 1 January 1990; major
political updates extend through 30 March. The entries explicitly identify
conventional military services, separately from police and paramilitaries:
Chile, El Salvador, Nicaragua, Sri Lanka, Colombia, Guyana, Senegal, Zimbabwe,
Mexico, India, Malaysia, Bulgaria, Poland, Bolivia, Dominican Republic, Ecuador
and Singapore. The combined Guyana Defence Force includes military services.
This source proves presence only, not political autonomy or coup propensity.

That first expansion brought the overlay to 20 institutions; its source scope
remains recorded in the [earlier receipt](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/army-coverage-factbook-1990.json).
The current overlay contains **57 sourced institutions**. A complete audit of
the **137 opening-country records** found **71 existing base Army institutions**,
these **57 omissions**, and **nine explicit exclusions or unresolved cases**.
The additional 37 rows use the same edition's country-specific branch entries.
Its exact publication hash, source anchors and exceptions are embedded in the
[complete coverage receipt](../../spheres-sim/data/army_institution_coverage_1990.json);
the [institution rows](../../spheres-sim/data/army_institutions_1990.json) link to
their supporting sections.

Cape Verde is the only additional country that opens above `.20`
authoritarianism: its existing two-party table had no Army despite the source
identifying the People's Revolutionary Armed Forces. The other 36 additions
complete coverage among countries at or below `.20`; this is not a promise of
coup immunity. That boundary suppresses the civilian-confidence penalty, but
the electoral coup route can still respond to material grievances and crisis.
The unchanged campaign gates, including A6, must therefore be measured again.
Niger and Mali are outside the current country roster. Fiji, Thailand and
Pakistan already have base Army institutions and receive no duplicate overlay.

The overlay reuses the original party table, election dates, electoral system
and ruler. Funding constants, personnel records and starting party shares are
unchanged. New armies start at the standard institutional loyalty of `.65`.
An older lens save gains a missing verified pillar once and retains its live
loyalty thereafter. With the lens off, the original polity and save behaviour
remain unchanged. Named institution and takeover readers use the same live
lookup rather than disagreeing with the simulation.

This completes the explicit institution-presence audit of the current opening
roster, **not a worldwide historical or campaign certification**. It does not
add absent countries or establish later successor institutions.
Positive personnel totals do not automatically create an Army: police and
paramilitaries can be counted in that series. Bahamas, Costa Rica, Iceland,
Maldives, Mauritius, Panama, Samoa, Solomon Islands and Vanuatu retain their
existing institution tables. The receipt distinguishes coast guard/police,
explicit absence, Panama's military dissolution, and genuinely unknown source
entries; unknown is not reported as proven absence. The loader rejects
unknown institution kinds, missing/invalid source metadata, duplicate rows,
existing Army duplicates, unknown countries, contradictory absent/zero
personnel and invented loyalty fields. The parser cannot itself authenticate
a historical claim; the explicit sources above provide that review. The
original nation personnel data, including its previously documented date
fallbacks, is not rewritten from these publications.
The complete-set regression covers all 137 dispositions, all 57 overlays,
original party-table identity, default-off save bytes, unchanged government
roles and election dates, and preservation of existing live pillar loyalties.
The full coverage regression passes in iterations 12 and 13. Iteration 12
also exposed a stale D4 assertion that expected France's entire Polity pointer
to remain unchanged despite its new sourced Army. The corrected check retains
the identical party-table and flag-off requirements and passes in iteration 13.

## Evidence and required validation

Iteration 3's existing executable was run for twenty seeds and 252 months in
verbose diagnostic mode. It recorded 122 coups against elected governments:
Comoros 88, São Tomé and Príncipe 20, and Myanmar 14. It also recorded eight
Communist bloc changes across six seeds: four Cambodian coups/uprisings and
four Tajik collapses across the electoral boundary. These are **pre-repair
diagnostics**, not passing results for the final code.

The local evidence directory is
`work/campaign-certification/evidence/political-calibration-20260922/`.
`iteration-3-census-verbose-provenance.json` records the diagnostic command,
binary and log hashes; `iteration-3-resource-model-rationale.json` records the
cost-model assumptions and rounded roster projections.

Iteration 4 passed 47 focused tests. Its calibration remained incomplete:
A1 measured a median 14.5 coups with a `.85` top-three concentration; A3 still
measured Communist non-ballot changes in 6/20 seeds. The twenty-seed diagnostic
identified remaining Cambodian and Afghan programme reversals; Tajikistan's
phantom historical-programme changes were gone. The Party-identity and
independent-organisation changes above, and the separate institutional-veto
eligibility were made after that measurement. Iteration 5 measured A3 at
2/20 and Algeria's annulment route at 12/12; those assertions passed. Other
calibration failures remained, so that result was not certification.

Iteration 6 passed **61/61 focused tests**, including five election-lifecycle
and physical-coercion regressions. The unchanged A1–A10 suite still failed
three gates: A1 median 14.5 coups and `.80` top-three concentration, A2
non-ballot Islamist success in 6/12 seeds, and A5 ex-Communist returns in
6/12. A3 remained passing at 2/20; Algeria's institutional annulment remained
12/12. Other gates passed. `iteration-6-results.json` records exact source and
binary hashes; `iteration-6-A1-A10.log` retains all outcomes.

A separate four-seed monthly diagnostic (0, 1, 37, 38) found no electoral-coup
event retaining the previous Army holder after the lifecycle repair. The
remaining coups followed real ballots, so the remaining concentration cannot
be dismissed as that counting/office bug. It also revealed a generic Party
regime office remaining after a ballot, which the iteration-7 extension above
addresses. Iteration 7 passed **71/71 focused tests**, including those five
new Army-presence and generic-Party handover tests. Its A1 median was 16.5
coups with `.69` top-three concentration; A2 remained 6/12; A3 measured 4/20.
Those three gates failed. A5 improved to 12/12 and passed. The remaining gates
passed. These measurements do not certify the full political calibration.

After iteration 7, review found a further lifecycle defect: a generated Party
regime office could become the historical office's ruling house on a later
internal coup, death or programme change. The bounded repair retains the
generated role through all three paths, so it can subsequently yield at a
genuine ballot; an actual inherited crown still follows its sourced heir/house
rules. It also prevents a retained historical heir from spontaneously
restoring a deposed crown. That regression and the expanded 20-institution
coverage controls passed in iteration 8.

Iteration 8 passed **78/78 focused tests**. A3 improved to 1/20 and passed;
A4 measured 22 and A5 remained 12/12. A1 still failed at a median 21 coups and
`.56` top-three concentration, and A2 failed at 2/12 non-ballot Islamist
successes while Algeria's annulment route remained 12/12. The census executable
reported nine passing checks and two failing checks (A1 and A2), including the
separate source guard. The [recorded output](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-8-A1-A10.log)
and [source/binary manifest](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-8-results.json)
identify that measured build. Further source changes are not certified by
these earlier measurements.

Iteration 9 passed **83/84 focused checks** and the whole compiled workspace
reported **1,839 passed, one failed, 89 ignored and one resource test filtered**.
The failure was a test fixture that assumed scheduling an initial ballot also
rebuilt the current chamber. Its correction explicitly stages the provisional
opposition table while preserving the accountability assertions. This corrected
fixture was not in the iteration-9 binary. The [workspace receipt](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-9-workspace-receipt.json)
retains the original failure and does not count ignored checks as passes.

The iteration-9 original gates again measured nine passes and two failures:
A1 median 15 coups with `.63` top-three concentration, and A5 zero of twelve
seeds with the required successor returns. A2 passed at 8/12 and A3 at 1/20.
The [larger development cohort](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-9-development-analysis.json)
revealed why these smaller successes are insufficient evidence: A2 was 99/200
(`.495`, below the required majority), and A3 was 34/200 (`.17`, above `.10`).
A1's median became 14 but its concentration remained `.64`; A5 remained 0/200.
The instrument's exit zero only means it completed. Both sets of readings are
retained, and neither overrides the other or certifies the mechanics.

Iteration 10 includes the bounded funding inverse, successor reference and
fixture repairs. All **86 focused checks pass**. Its original gates still
fail A1 (median 9.5 coups, concentration `.62`) and A5 (0/12); A2 reads 8/12 and
A3 1/20. The [completed development cohort](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-10-development-n200-summary.log)
reads A2 at exactly 100/200, which does not meet a strict majority, A3 at
22/200, and A5 at 0/200. Its diagnostic exit zero means completion only.

The [predeclared accountability comparison](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/accountability-calibration-plan.json)
measures `.018`, `.024` and `.030` as gameplay response assumptions. Negative
performance transfers that fraction of the incumbent constituency per unit
of adverse record per month. The 1.5-percentage-point **complete** monthly
vote-change cap, `.005` constituency reversion, term memory, positive response,
family issue weights and outcome thresholds remain fixed. The `.030` candidate
was evaluated to investigate the narrow `.024` majority, not to relabel the
latter's measured pass as a failure.

| Build | Response | Focused checks | Original A1–A10 | A1 median coups / top-three share | A2 / A3 / A5 in 200 development seeds |
| --- | --- | --- | --- | --- | --- |
| 11 | `.024` | 85/86 | 9/10 pass | 9 / `.67` | 127 / 4 / 104 |
| 12 | `.024`, complete Army coverage and midyear funding | 87/88 | 9/10 pass | 7 / `.57` | 125 / 2 / 104 |
| 13 | `.030`, same mechanical base as 12 | 88/88 | 9/10 pass | 7 / `.57` | 162 / 14 / 105 |

These are distinct frozen executables. [Iteration 11](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-11-results.json),
[iteration 12](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-12-results.json)
and [iteration 13](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-13-results.json)
retain source/binary receipts; the neighbouring gate logs retain every failed
assertion. Each original suite also passes its separate attribution guard,
which is not an eleventh historical outcome target. A1 concentration is the
remaining failed original target in all three builds.

The two focused failures are retained rather than counted as passes. Iteration
11's ruined-equilibrium test used a `>.05` lower bound derived from the prior
`.018` response. At `.024`, the measured equilibrium is `.041954`; the unchanged
transfer/reversion equation predicts approximately `.042`. The replacement
checks that analytic fixed point, explicit convergence, the constituency floor
and conserved total while retaining quiet/war/monotonic/clock-bound controls.
It passes in 12 and 13. Iteration 12's D4 pointer mismatch is the sourced-overlay
fixture issue described above; its corrected requirements pass in 13.

The same-base [iteration-12 cohort](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-12-development-n200-summary.log)
versus [iteration-13 cohort](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-13-development-n200-summary.log)
does not show a meaningful A5 robustness improvement: `.520` becomes `.525`.
Both produce returns in Poland and Bulgaria, with the third country still
usually Russia. A2 moves from `.625` to `.810`, and A3 from `.010` to `.070`.
The smaller `.024` response is therefore selected for the next combined base;
`.030` is retained as a measured comparison, not the chosen final value.

A5's `.520` is a measured narrow pass, **not a demonstrated reliable majority**:
its Wilson 95% interval is approximately `[.451, .588]`. The plug-in sample-size
calculation gives 3,380 after rounding, but that does not prove the underlying
probability exceeds one half. Running seeds `0..3379` would also contaminate
the reserved independent block and is expressly not authorised by this record.
No outcome interval, winner definition or 1996 deadline is changed.

The [confidence comparison plan](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/confidence-calibration-plan.json)
predeclares `.65`, `.90` and `1.20` for the existing political-confidence
coefficient on the same accepted base. No changed-confidence candidate is
certified here. Resources, paid commands, useful-resource ceilings, affordability,
mandate and civilian-control guards, and all A1–A10 thresholds remain fixed.
First/repeat coup distribution and actual appropriations must be reviewed;
the grid must not be extended solely until a quota passes.

The completed confidence comparison uses `.024` accountability and the same
runtime base, including the correction that reconciles a historical parliamentary
office with its first completed simulated ballot. It does not initialize a new
historical public-mandate record. The candidates are design assumptions, not
empirical estimates.

| Build | Confidence weight | Focused checks | Original A1–A10 | Development A1 median / concentration | Development A2 / A3 / A5, out of 200 |
| --- | --- | --- | --- | --- | --- |
| 14 | `.65` | 91/91 | 9/10; A1 fails | 8 / `.62` | 125 / 1 / 113 |
| 15 | `.90` | 90/91 | 8/10; A1 and A5 fail | 10 / `.56` | 133 / 0 / 108 |
| 16 | `1.20` | 89/91 | 7/10; A1, A3 and A5 fail | 11 / `.50` | 125 / 11 / 105 |

The [iteration-14](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-14-results.json),
[iteration-15](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-15-results.json)
and [iteration-16](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-16-results.json)
receipts retain the original failures. Their separate attribution control
passes. None of the three candidates passes every original gate, and even
the last development median equals rather than satisfies A1's strict `<.50`
concentration requirement. Larger sampling is not an A1 repair.

Iteration 15's funding test assumed its severe political-crisis fixture could
reach `.40` loyalty through provisioning. At `.90` its full-resource ceiling
was approximately `.257`. Iteration 16 separates a reachable moderate-autonomy
positive case, which passes, from political impossibility. That new negative
fixture mistakenly uses authoritarianism `.60`, outside the strictly electoral
branch, and fails its useful-resource equality. This is retained as a failed
fixture rather than a pass. The second iteration-16 focused failure is the
Algeria end-to-end scenario: an ordinary military coup occurs before its
scheduled hostile-army ballot setup. The separate original A2 census still
records twelve annulments in twelve seeds. These are distinct tested paths;
neither result overrides the other.

The [larger cohort summaries](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-16-development-n200-summary.log)
again do not certify all targets. A5's development proportions `.565`, `.540`
and `.525` have approximate Wilson 95% intervals `[.496,.632]`, `[.471,.608]`
and `[.456,.593]`. All include one half. The predeclared plug-in sizing rule
would give 320, 840 and 2,160 seeds respectively, but does not establish
power, historical correctness, or a population majority. Any revised sample
must be fixed prospectively after model selection, use a disjoint uninspected
range, preserve every original small-cohort failure, and run once without
appending seeds until green. No sample size or acceptance rule was changed
for this comparison, and the reserved independent cohort remains untouched.

Iteration 14 records canonical source hashes and raw lengths; iterations 15
and 16 additionally record exact raw SHA-256 hashes. Older manifests retain
their original scope rather than being rewritten to imply stronger provenance.

After restoring `.65` confidence with `.024` accountability, iteration 18 adds
the bounded sourced Suriname opening-mandate record and the successor
force-reference clock correction. The [measured build](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-18-calibration-summary.json)
passes **99/99 focused checks**, including the unchanged whole-world political
clock test. It passes nine of the ten original outcome gates plus the separate
attribution guard; A1 still fails at seven median coups and `.69` concentration.
Its 200-seed development cohort reads A1 at seven and `.67`, A2 at125/200,
A3 at5/200, A4 at19, A5 at112/200 and A7 at8.29. A5 still consists of Poland
and Bulgaria in every seed plus Russia in112. The source-correct mandate is
not a calibration remedy, and no subsequent authority-model proposal has
been tested by this binary. All previous failures remain in the archive.

The [development analysis plan](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/development-cohort-analysis-plan.json)
and the untouched [independent cohort](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/predeclared-independent-cohort.json)
remain separate. All development runs above use seeds 0–199 for 252 months;
their A6 reading is not a 420-month validation. The original A6 test separately
passes 12 seeds at 420 months. The reserved 1000-series block remains uninspected.
A later source or leadership correction needs its own compiled verification.

Focused regressions cover real units, monotonic resource funding, local cost
differences, the inverse fiscal allocation, paid actions and budget authority,
missing-data provenance and invalid inputs, sustained crisis versus quiet and
fresh-government controls, civilian control, prospective programme vetoes,
save continuity, uncrowned versus crowned transitions, and exact flag-off
behaviour. Final calibration measurements and held-out seeds must be recorded
before this document's pending status is replaced with a certification claim.

## Iteration 19: snap ballots retain accountable-party coup grace

A paid snap ballot previously renewed the same governing party's full first-year
coup grace despite twelve or more months of observed governing history. The fix
uses matching, finite accountable-party history without changing the election
clock, the twelve-month threshold, genuine new-party protection, interim ballot
protection or missing-save fallback. Three command-level regressions bring the
focused set to **102/102 passes**.

The [original gate run](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-19-A1-A10.log)
passes nine of ten outcome gates plus attribution; A1 still fails at median
7.5 coups and .71 top-three concentration. The separate fixed
[200-seed development cohort](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-19-development-n200-summary.log)
reads A1 7/.67, A2 118/200 by end-2000, A3 2/200, A4 median19 and A5 **99/200**.
A5 therefore fails the development majority bar even though the original
12-seed assertion passes7/12. Every outcome is retained; no cohort, target or
coefficient was changed to suppress the result. The source snapshot and both
native executable copies were recorded before later authority edits. The
1000-series independent cohort remains uninspected.
## Iteration 20: sourced executive-removal leverage

A separately sourced 1989 proxy now initializes military political leverage,
with an immutable source record and a live campaign value. Army seizures and
qualifying civilian transfers change only the live value. Unknown old saves
retain the explicit legacy proxy; paid resources, war effects, thresholds and
response coefficients remain unchanged. The source is not a coup probability.

The [corrected build](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-20-calibration-summary.json)
passes **107/107 focused checks**. Its full workspace result is **1,862 passed,
zero failed, 89 ignored and one resource timing check filtered** across 66
targets. A terminology edit's encoding error was found before measurement;
all 501 affected sequences were independently reconstructed and matched the
corrected source hash. The superseded corrupt build was never measured.

The original political tests still pass only nine of ten outcome gates plus
attribution: A1 fails at **8.5 coups/.62 top-three concentration**. The fixed
200-seed development cohort has A1 **8/.62**, A2 **128/200** by end-2000,
A3 **2/200**, A4 median **19** and A5 **124/200**. All other measured development
bars pass within their stated horizon. This is not full certification.
The independent 1000-series cohort remains uninspected. A separate prospective
single-candidate confidence plan is recorded before its implementation; no
candidate result is implied by the current algebra or ordinary workspace pass.
The copied-build [distribution diagnostic](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-20-confidence-diagnostic.summary.json)
observes 103 elected coups in seeds 0–11: 75 first events and 28 repeats, all
following completed model ballots. Myanmar and Guatemala account for 21 of
those repeats. No pre-coup snapshot is at full resource saturation. All 103
could reach the AI's .40 target if fully supplied, while 99 need more spending
than the current nondeficit affordability bound. Eleven pre-tick snapshots
still have an affordable useful increase available; these observations precede
the entire monthly tick and do not by themselves prove an AI command defect.

High-leverage near misses also distinguish crisis from mere source authority.
El Salvador's maximum confidence penalty is .2962, Philippines' .2295 and
Peru's .1765. Their corresponding fully provisioned, no-war political targets
remain .5538, .6205 and .6735. Thailand and Pakistan have no discontent-at-least-.25
months in the observed 3,024 electoral months each. A response coefficient
cannot, and should not, invent a crisis there. The separate prospective
candidate requirement and controls address reachable political disloyalty in
an actual prolonged severe crisis; these country trajectories are diagnostics,
not instructions to reproduce dated historical coups.
A subsequent contextual check finds that **all eleven** apparently available
appropriation cases received a real funding increase in the immediately
previous month, and their current military share matches that increase.
Changing funding needs/caps and gradual loyalty recovery explain why a
pre-tick opportunity can still be visible. This audit identifies no confirmed
forgotten-funding defect, and no additional funding change is justified by
these snapshots.
## Iteration 21: successor calendars preserve actual accountability

Giving an already electoral successor state its missing election calendar no
longer treats the scheduling operation as a fresh regime opening. Its actual
governing record and ordinary eighteen-month deadline survive. This does not
claim a completed ballot; actual regime interims and the disabled path retain
their original protection. Four regressions bring focused checks to **111/111**.

The [unchanged original and development cohorts](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-21-calibration-summary.json)
now pass A5 at **12/12** and **200/200**. Bulgaria, Croatia and Poland supply
three observed successor-party returns in every development campaign. No
political coefficient, historical winner, sample or outcome threshold changed.
A1 remains the sole failed outcome gate: original **8.5/.62**, development
**8/.62**. All other bars pass within their recorded scopes. The prospective
single response candidate remains distinct from this completed baseline.

## Iteration 22: prospective response candidate rejected

The [single predeclared confidence candidate](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-22-calibration-summary.json)
passes its three new fully supplied severe-crisis controls, but only **113/114
focused checks** overall. The long Algeria annulment scenario suffers an
earlier elected-army coup before reaching its staged election condition; the
original failure and complete log are retained. Eight of ten original outcome
gates pass: A1 remains outside its concentration bar at **12.5 coups/.52**, and
A2 falls to **6/12**. A5 remains **12/12**.

The same fixed 200-seed development cohort gives A1 **13 coups/.50**, which
still fails the strict below-.50 concentration requirement. A2 is **125/200**,
A3 **10/200**, A4 median **19**, and A5 **200/200**. All other development bars
pass within their stated horizons; diagnostic completion does not convert
the original failures into passes. The independent 1000-series cohort was
not inspected.

The candidate was rejected under its prospective joint criteria. The
[restoration receipt](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-22-rejection-restore.json)
records exact restoration of iteration 21 government source, retaining the
successor-calendar and sourced-authority repairs. Candidate fixtures,
source/binary manifests and every failed measurement remain archived. The
ordinary target executables still represented candidate 22 at that rejection;
they were not attributed to the restored source. A subsequent exact restored-21
workspace build passed 1,866 tests, zero failed, 89 ignored and one resource
timing check filtered, as recorded in the separate restored-workspace receipt.
No additional coefficient candidate follows automatically from this result.

## Iteration 23: accounting corrections and actual executive choice

Fiscal consolidation now moves toward its bounds without increasing a low
budget or lowering an already high tax rate. Legacy policy respects explicit
stock, annual-plan, player, program and recovery ownership. The paid card
retains its real bill and updates the actual fiscal owner. Coups synchronize
open-book debt-to-GDP after their existing output loss, preserving dollar debt,
cash and the independent legacy ratio. An exact original-21 public-tick witness
fails the booked-ratio assertion; both corrected debt regressions pass.

The voluntary franchise-only AI route now requires the actual executive to
have a party in the promised ballot. A dormant civilian cabinet does not give
a fresh military executive that preference. Existing bans are not a blocker,
because this action lifts them. Public demand, the legal player command, its
price and the weak-armed AI route remain unchanged. This is an explicit
strategy correction, not a claim that military rulers cannot negotiate.

All [128 focused checks](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-23-focused-receipt.json)
pass. The [unchanged original gates](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-23-calibration-summary.json)
still pass only nine of ten outcome bars: A1 is **7 coups/.59 concentration**.
A2 is 8/12, A3 1/20, A4 median 15, A5 12/12; A6-A10 pass their original scopes.
The fixed 200-seed development cohort also fails only A1 at **7/.57**. It reads
A2 133/200 by end-2000, A3 1/200, A4 median 16 and A5 200/200. Its zero A6 events
cover 252 months; the original separate twelve-seed check covers 420 months.

The [pooled distribution](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-23-coup-distribution.json)
contains 1,498 elected coups in nine countries: Guatemala 327, Sao Tome 276,
Myanmar 232, Guyana 222, Ecuador 200, Comoros 199, Mozambique 26, Chad 9 and Cambodia 7.
The pooled top-three share is .5574; it is distinct from the median per-seed
measure used by A1. Existing logs truncate each seed's country names at five,
so they do not establish an exact first-versus-repeat count. No additional
trajectory was run to fill that gap. No cohort, threshold or authority
coefficient changed, and the independent 1000-series cohort remains untouched.

Source capture now enumerates current tracked inputs directly, including newly
committed modules, data, tests, vendor/build inputs and all tracked web UI
assets. The receipt contains 1,963 raw/canonical input hashes, 468 byte-exact
native source bodies and both copied test binaries. Earlier limited manifests
remain unchanged rather than being retroactively expanded.

## Iteration 24: explicit annual-plan ownership boundary

A remaining partial-save boundary allowed the Army funding fallback to overwrite
an explicit annual plan when treasury and debt stocks were absent. The fallback
now independently checks annual-plan ownership, matching the fiscal policy
boundary already repaired in iteration 23. It does not change the ordinary
planless simulation, military coefficients or outcome gates.

The [original-23 library witness](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/army-plan-owner-original23-witness-result.json)
compiles and fails at the public government tick: the existing annual plan
becomes `None`. Its initial wrapper import error is retained separately as a
compile preflight, followed by the corrected wrapper and genuine failing test.
The earlier application receipt hashes that preflight wrapper; the witness
result hashes the corrected one. The source regression covers an unchanged
plan across direct and save/load paths, plus actual paid funding when no other
owner exists. The corrected regression passes in checkpoint 24's 1,005-test
simulation library run. The broader workspace still fails three separate
checks, as recorded below. No evidence links this partial-save defect to A1's
remaining concentration; the existing failed outcome readings remain authoritative.

The [checkpoint-24 workspace receipt](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-24-workspace-result.json)
retains **1,881 passed, three failed, 89 ignored and one timing check filtered**
across 66 targets. The treasury source audit sees a `debt_gdp` assignment in
new test fixture code. Moving that unchanged `cfg(test)` module to its own file
keeps the production audit intact. The two web failures concern advancing time
after the player's country disappears and a scenario that did not exercise a
dead belligerent. Follow-up test changes pass checkpoint25; none converts the
failed checkpoint-24 result into a pass.

The [fixed seed-0 diagnostic](../campaign-certification/verification/evidence/2026-09-22-completion/political-calibration/iteration-24-seed0-trace-analysis.md)
records ten elected coups: six first and four repeats. Nine preceding monthly
snapshots cannot afford the desired Army funding under the existing policy;
Myanmar's later repeat retains low actual loyalty while its target is already
recovering toward .40. Four small increases below the AI spending threshold
would change targets already above .397 to .400, so they do not demonstrate a
blocked target rescue. An additional read-only emitter reproduced every
original output record and the same final state/RNG. The six-country authority
review found no erroneous zero or weighting and no source-supported reason to
reset authority at a first civilian ballot. No further mechanics change follows.

Compact source/plans, all actual coup-case readings, the four hysteresis cases,
and executable/library/log hashes are archived. The two approximately 6 MB raw
trace files remain external and are identified by the receipt; no binaries or
redundant full logs were added to Git. A1 remains open, and the reserved
independent cohort remains uninspected.

## Checkpoint26: load preservation and fresh-browser construction

[Checkpoint25](../campaign-certification/verification/evidence/2026-09-22-completion/checkpoint25-validation/README.md)
passes 1,884 ordinary native tests with no failures, but separately exercising the
unchanged genuine pinned-master archive finds an Army-pillar insertion during
load. Three other archive checks and both mature native latency checks pass;
the original fixture hashes stay unchanged.

The repair removes that existing-state retrofit. New browser games instead pass
the lens to the authoritative world initializer before governments are seated,
which also corrects missing sourced authority, established movements and prior
mandates on that fresh path. Saved January 1 campaigns retain their recorded
institutions and unknown source fields. The former unit expectation requiring
retroactive Army insertion is explicitly superseded by the stronger original
archive-preservation contract; its old body is archived. No source dataset or
A1–A10 coefficient/threshold changed. The rebuilt repair passes 1,886 ordinary native tests, all four original archive checks and the browser journey; its source-bound receipts are in the linked checkpoint26 validation record.

The original 12-seed diagnostic captures all country counts without changing its A1
assertions. It still fails at median 7 and top-three share 0.585714. A displayed
inverse-HHI/effective-country comparison is a different, unapproved criterion;
it is not called equivalent and cannot certify the current model. The independent
1000-series cohort remains untouched.
