# Fiscal recovery

This rebuild gives every country a common debt ledger, a visible recovery test,
and political consequences for sustained fiscal stress. Ridge authorized its
implementation after the 1990–2035 QA campaign exposed a player debt path that
could keep rising while inflation and stability remained comfortable.

The objective is affordable debt service and a stable debt path. A high debt/GDP
ratio alone does not trigger a penalty, a default, or a civil war.

## Playing the system

New browser campaigns enable fiscal recovery. Existing saves keep their fiscal
rules until the player chooses **Enable fiscal recovery for this campaign** in
Economy → Money & policy. The page discloses that this is a permanent campaign
upgrade. A separate save retains the old rules. Legacy daily-calendar transitions
must finish before the upgrade command can run.

The overview shows a compact status and a link to the full recovery card. The
policy card reports the simulation's own assessment:

| Reading | Meaning |
| --- | --- |
| Debt / GDP | Current debt stock relative to current real output. |
| Debt trend | Annualized observed change in the ratio, in percentage points. |
| Interest / revenue | The share of recorded government revenue absorbed by interest. No positive revenue is shown explicitly. |
| Primary balance | Revenue minus ministry cash spending, before interest, as a share of GDP. |
| Five-year debt / GDP | An illustrative continuation of the observed cash budget under the displayed assumptions. |
| Observed adjustment gap | The improvement in the primary balance estimated to stabilize the path and restore payment capacity. |

The details explain remaining adjustment time, months of stress and improvement,
the separate fiscal stability effect, and the projection assumptions. A
current-tax-policy gap distinguishes newly enacted revenue changes from the
trailing receipts that have not yet caught up. Before the first complete monthly
report, the outlook is explicitly provisional.

**Review taxes**, **Review ministry spending**, and **Review Cabinet recovery
actions** open the existing controls. They do not enact a policy. The player
continues to decide taxes, ministry allocations and Cabinet actions. Policy
drafts are preserved when navigating between those controls.

The annual budget preview adds a separate five-year **full-use scenario** when
fiscal recovery is active. It applies the draft tax, policy rate and authorized
budget on a world copy, then assumes all authorized spending is used. This is
not the recovery card's observed-cash projection. Lowering unused construction
authority can reduce the full-use scenario without saving any actual cash.

## Accounting and observation

`spheres-sim/src/fiscal_recovery.rs` owns surveillance and its pure
`assessment(world, nation)` query. The normal economy and program settlement
continue to own cash and debt. The new system observes their receipts rather
than charging a second fiscal bill.

For enabled daily campaigns, every living country opens an explicit treasury
and debt stock before the economic settlement. Existing explicit balances stay
intact. Unopened accounts receive zero cash and convert their recorded debt/GDP
ratio into debt at current GDP. The upgrade grants no starting reserve and
waives no interest. Successor countries receive the same treatment when they
appear.

Non-program countries post the exact fiscal amounts from their normal
settlement. Program countries supply their closed daily spending and interest
receipt. Observation windows retain up to twelve monthly records. Partial
periods are annualized using the calendar exposure actually observed; completing
a partial calendar month does not earn a whole month of adjustment time.

Receipts separate the ordinary fiscal bill from other movements in debt minus
cash. Other transactions still move the observed debt path. A conservative
recurrence estimate recognizes three consecutive positive monthly residuals,
or at least three payment months over nine observed months. The smallest
positive payment rate represents the recurring amount, with its observed
frequency retained; larger exceptional payments are not multiplied into every
month. This is a disclosed estimate, not a ledger that definitively identifies
one-off transfers. Negative receipts and write-downs do not become permanent
primary-budget improvements. Reducing unused department or project authority
is not classified as a cash saving.

The projection uses sixty monthly steps through the same real-rate and
cash-first financing rules. It holds the primary cash balance and policy rate,
updates the existing sovereign premium as debt changes, and bounds the assumed
real annual growth rate to −10% through +8%. These are disclosed scenario bounds,
not changes to actual GDP. Cash, debt and GDP use constant 1990 dollars;
inflation is not subtracted from the real debt stock a second time.

## Recovery and political consequences

The assessment combines the observed and projected trajectory with
interest/revenue. The initial model considers interest above 30% of revenue
unaffordable. Rising-path tests use a material change over several years or
several observed months, so an isolated small move does not automatically demand
an adjustment. These thresholds are game tuning, not a claim of universal
historical debt limits.

Governments receive twelve observed months to adjust inherited policy. Interest
remains payable throughout. After that period, sustained deterioration builds a
bounded fiscal-confidence pressure. Improvement is measured from results; merely
announcing a plan does not reset pressure. The statuses distinguish a watch,
required adjustment, recovery in progress, a confidence crisis, and debt under
control. Severe unresolved stress keeps the crisis headline even when recent
cash results improve. The advice acknowledges improvement and asks the player
to review whether further adjustment is needed, allowing already-enacted policy
time to enter receipts. It does not present a small improvement as restored
affordability.

The fiscal arm enters the existing stability integrator once. Its initial raw
cap is 0.20, equivalent to −0.05 stability points per month before the
integrator's bounds and other contributions. It is not a daily subtraction.
The national stability breakdown displays the same **Fiscal confidence** term.
Inflation, service gaps and unemployment retain their existing owners; the
fiscal assessment does not charge those harms again.

## AI and Cabinet actions

`spheres-sim/src/fiscal_recovery_ai.rs` reviews each living AI government every
90 days after a monthly cash report exists. The player is excluded. Proposals
use the shared assessment, the ordinary command prices, and an eight-point
political-capital reserve. The old ratio-triggered fiscal autopilots are bypassed
for these campaigns so they cannot independently change the same policies.

Recovery campaigns also apply the existing service, employment, demographic and
private-investment consequences to AI domestic policy. Raising tax or cutting
housing therefore carries the same modeled tradeoffs as the player's policy.
Legacy campaigns retain their calibrated player-only domestic-policy gates.

Small tax adjustments are limited to one percentage point per review and stop
at the assessed current-policy gap. Selective operating reductions preserve
essential services, education, pensions, security, science, productive
maintenance and industry operations. Eligible reductions have floors tied to
the inherited envelope; repeated reviews cannot compound them down to nothing.
Defense reductions are limited to peacetime. Project-funded departments are
excluded from claimed operating savings.

The existing **Fiscal Consolidation** Cabinet action now changes the active
ministry/department budget through the same budget path. Its package vote pays
34 political capital once and costs nine stability. It can raise tax by up to
five points and reduce eligible discretionary spending while protecting the
same service floors. The vote itself neither pays off debt nor clears fiscal
pressure.

The existing **Restructure the Debt** action still writes down 45% of the actual
debt stock, costs 30 political capital, raises stability by four, and reduces
major-country relations by eight. Recovery campaigns share a five-year
restructuring cooldown between player and AI. Ongoing deficits remain after a
write-down. AI reserves this action for a sustained severe service burden and a
large unresolved adjustment, with the normal political price and availability
checks.

## Compatibility and scope

`GameRules.fiscal_recovery` defaults to false. The new state is omitted when
empty. Legacy headless runs and saves retain their previous fiscal behavior
unless explicitly enabled; loading a browser save alone does not enable the new
model. The command queue, deterministic clock and save serialization remain the
only mutation paths.

This rebuild does **not** introduce a lending market, a borrowing limit, arrears,
payment defaults, bond maturities, or gradual refinancing. The existing effective
rate still prices the debt stock. Borrowing remains available under the existing
cash/debt settlement rules. **Fiscal confidence crisis** therefore describes
political pressure from an unsustainable path, not a declared payment failure.
Debt does not directly create inflation or open a civil-war conflict. Existing
political and conflict systems retain their own triggers.

The older federation-dissolution rules still author successor debt ratios and
do not allocate every predecessor debt/cash balance. The stock-preservation
guarantee here covers upgrades and ordinary living-country accounting;
sovereign-succession accounting remains a separate limitation.

## Validation record

The regression suite covers shared receipt accounting, inherited adjustment
time, neglected stress versus improving results, legacy inertness, save/replay,
protected spending floors, and priced AI action. Web tests cover exact
simulation-to-payload values, explicit player-bound upgrades, and reconciliation
of the displayed fiscal stability contribution. Browser unit tests exercise
units, provisional readings, full-use draft separation, navigation, duplicate
and stale commands, replaced campaigns, command refusals and escaped text.

Implementation checks passed locally on 2026-09-08 at `5cc39c1`:

- 1,337 workspace Rust tests, zero failures; 70 existing tests remain ignored.
- The separate resource timing test passed at **0.0739 ms/model month** against
  the unchanged **0.15 ms** gate. Its executable remained byte-identical after
  that commit's final web-only fixes. The later presentation correction was
  checked separately in CI as recorded below.
- Four campaign-observer tests and all **1,093 UI unit tests** passed. The UI
  count includes 13 fiscal tests and 25 existing Cabinet tests.
- All three unchanged browser suites passed on the final web executable:
  startup smoke, campaign module, and actual-server campaign integration.
  Original navigation timeouts and gameplay assertions remain in force.
- The HTTP connection reproduction completed **320/320** requests over ten
  32-connection bursts on a server restricted to one processor core, with zero
  stranded requests and no probe/release intervention; maximum response time
  was 178 ms in this local check.

Manual browser QA covered fresh-campaign tax drafting, draft preservation across
the policy/budget tabs, one enact/one day, loading the existing 2035 France
campaign, the explicit recovery upgrade, and desktop/mobile recovery navigation.
The upgrade comparison preserved existing cash/debt, date, political capital,
1,331 history points and 79,819 dispatches, while seating 155 previously unopened
living-country accounts. The original save was untouched. The final executable
also passed the exact post-vote navigation sequence: renew the 2035 budget once,
advance from 1 to 2 January, then follow recovery links to taxes and the budget.

Problems found during QA were repaired rather than weakening checks:

1. An existing coup GDP shock left debt/GDP stale. The mutation owner now
   refreshes the ratio, and fiscal closing-day surveillance also reconciles it
   without changing the debt stock or charging another bill.
2. Recovery links rendered during a budget vote stayed disabled afterward.
   Existing lock refreshes now update those controls, with regression coverage
   for completed votes, pending commands and stale campaign state.
3. The earlier intermittent browser startup failure was traced to queued
   keep-alive connections exhausting the HTTP library's available workers.
   The vendored `tiny_http` 0.12.0 changes one production comparison; the normal
   workspace suite runs its deterministic liveness regression. Restoring the
   old comparison makes the regression fail. See
   [`vendor/tiny_http/PATCHES.md`](../vendor/tiny_http/PATCHES.md) for provenance.
4. The completed guided 2035 report exposed a severe debt spiral labeled
   **Recovery in progress** after three improving months. The existing crisis
   thresholds now take precedence over that headline, with advice that
   acknowledges improvement and retains the need to review further action. This
   changes only the reported status, label and next-action text; cash flows,
   fiscal confidence, AI proposals and persisted AI reasons remain unchanged.
   Coverage includes severe and moderate improving cases, inherited grace,
   query purity and unchanged policy proposals.

The final fiscal observer executable has SHA-256
`6696E9B9C401FB057F502E0F40E23321451EF2DB586760D4EB13D24EF446CEB0`.
The browser executable used for the three local suites and HTTP reproduction
has SHA-256
`F87E99A6878CE3553BBD8C5CBC5E011B6122493F9405D0CC6E37566003DADD66`.
The browser fixes did not change the simulation used by the endurance observer.
The later crisis-headline correction changes presentation in the observer's
annual payloads, but not the world path or fiscal calculations. Frozen raw
reports retain their original presentation for provenance.

The clean browser build of `5c7d044` has SHA-256
`4DA16DE5C2F219979BCC90C0912703EBA9094CEE3B16FB034AFCF3099F52F248`.
Manual continuation loaded the post-vote France campaign on 2 January 2035
and followed the overview link to Money & policy, with enabled recovery controls
and no browser warnings or errors. Saving again preserved the complete parsed
campaign payload, including 1,332 history points and 79,829 dispatches; only the
save metadata timestamp differed. The original campaign save remained intact.
The saved campaign payload hash, excluding that timestamp, is
`544452438de6adf7e7d006623c15a9ac0493b933aa1aa4da94cdd583b2f0b969`.
Local receipts, earlier failed runs and final results are retained under
`artifacts/fiscal-recovery/` (not committed). The earlier
`USER_QA_1990_2035.md` describes the prior fiscal model; it is separate evidence.

The published implementation, commit
[`5cc39c1`](https://github.com/ridgemerkley2-web/Spheres/commit/5cc39c1044064fa7514bd46677f77e95ee565ffb),
also passed the full GitHub workflow on both Windows and Linux on the active
branch and `master`. All four jobs ran the unchanged workspace, isolated
resource timing, observer, UI and three browser checks, without retries. The
active branch's resource readings were 0.0747 and 0.1047 ms/model month
respectively; `master` recorded 0.1116 and 0.1048. All passed the 0.15 ms limit.
See the completed
[active-branch verification](https://github.com/ridgemerkley2-web/Spheres/actions/runs/34264793999)
and [master verification](https://github.com/ridgemerkley2-web/Spheres/actions/runs/34264794813).

The final fiscal code, commit
[`5c7d044`](https://github.com/ridgemerkley2-web/Spheres/commit/5c7d0444dba070aea6a5b2e24dafdd4fe74f2630),
passed the same complete workflow on both branches and both platforms. Each
job passed **1,338 workspace tests**, the isolated resource test, four observer
tests, **1,093 UI tests**, and all three browser suites. Seventy existing tests
remain ignored; the resource test is deliberately filtered out of the workspace
run and executed separately. The added crisis-presentation regression passed
in all four jobs and in the local 18-test fiscal suite.

| Final-code CI | Linux resource time, ms/month | Windows resource time, ms/month |
| --- | ---: | ---: |
| Active branch | 0.1080 | 0.1173 |
| Master | 0.1075 | 0.1185 |

All four original jobs completed successfully without a rerun or relaxed
assertion. These resource readings pass the unchanged 0.15 ms hard gate; the
separate 0.05 ms aspiration was not met. See the final
[active-branch workflow](https://github.com/ridgemerkley2-web/Spheres/actions/runs/34270309707)
and [master workflow](https://github.com/ridgemerkley2-web/Spheres/actions/runs/34270310733).
Full job logs and the checked counts are retained in
`artifacts/fiscal-recovery/ci-5c7d044/final-summary.json` and adjacent logs.

## Completed guided campaign: 1990–2035

The frozen fiscal observer ran seed 1990 through **16,436 daily ticks**, from
1 January 1990 to 1 January 2035. It used normal starting data, daily industry,
manufacturing, logistics, warfare, population, company and province-economy
systems. The USA renewed its inherited annual program budget and followed the
same priced recovery proposals available to AI governments, with the same
eight-point political-capital reserve. No cash, political capital or growth
overrides were injected.

| USA, 1 January | Debt / GDP | Tax rate | Real GDP, $bn | Stability |
| --- | ---: | ---: | ---: | ---: |
| 1990 | 62.00% | 27.00% | 5,980.00 | 78.00 |
| 2000 | 41.52% | 30.33% | 6,899.43 | 73.46 |
| 2010 | 15.47% | 30.33% | 7,921.95 | 70.02 |
| 2016 | 0.00% | 30.33% | 8,662.80 | 68.61 |
| 2035 | 0.00% | 30.33% | 11,173.59 | 65.27 |

Only four recovery commands were executed, all tax adjustments during 1990:
28%, 29%, 30% and 30.3323%. They cost **10.6634 political capital** in total,
with zero execution errors. The observer needed no spending-cut or
restructuring command. The first zero-debt annual sample was 2016. By 2035,
the primary cash surplus was 2.8482% of GDP and treasury cash was $5,501.87bn.
No annual USA sample had fiscal-confidence pressure; other political and
economic effects continued to affect stability.

Daily checks confirmed finite positive GDP, bounded stability, nonnegative
cash/debt and debt/GDP reconciliation within `1e-9` for every living country.
A save at the terminal state reproduced seven more days exactly. Opening world
hash: `2a6d3e40f04682c6`; terminal hash: `f60790d8d77ab680`.

Of 156 living countries in this guided world, **146 were assessed as under
control** in 2035. That includes some countries above 100% debt/GDP with an
affordable, broadly stable path. Severe outliers remain: Zaire reached
8,883.08% debt/GDP and 22.66 stability; El Salvador and Afghanistan exceeded
1,700% debt/GDP. Unlimited borrowing allows those extreme paths to persist.
The system exposes their fiscal stress and political pressure; it does not
implement a payment default or guarantee recovery through ordinary adjustments.
The Zaire reading also supplied the crisis-headline regression described above.

## Matched unchanged-policy comparison: 1990–2035

The unchanged-policy USA run completed the same 16,436 days, all daily checks
and seven-day exact save replay. It had the identical opening world hash,
`2a6d3e40f04682c6`, and renewed the same inherited player program budget
unchanged each year. It enacted no recovery votes and left tax at 27%.
Its terminal hash was `aea36307d5f18825`.

| USA on 1 January 2035 | Unchanged policy | Guided recovery |
| --- | ---: | ---: |
| Debt / GDP | 543.20% | 0.00% |
| Tax rate | 27.00% | 30.33% |
| Real GDP, $bn | 12,374.80 | 11,173.59 |
| Primary cash balance / GDP | −0.52% | +2.85% |
| Interest / revenue | 163.37% | 0.00% |
| Fiscal stability effect, points/month | −0.05 | 0.00 |
| Stability | 52.86 | 65.27 |
| Inflation | 2.00% | 2.00% |

The unchanged-policy run first shows fiscal pressure in the 2000 annual
sample and reaches the pressure cap by the 2004 sample. By 2035, its debt ratio
was rising 37.17 percentage points per year, with a five-year projection of
772.40%. It remained in a fiscal-confidence crisis, with 427 stress months.
Debt pressure did not automatically raise inflation or create a civil war.

The guided run trades a higher tax rate and lower modeled output for fiscal
solvency and higher final stability. The total 12.41-point stability difference
is the outcome of two evolving game worlds, not a measurement of the fiscal
term alone. This comparison does not disable tax, employment, political or
international feedback. The separate fiscal contribution is shown explicitly
above. In the unchanged-policy world, 146 of 156 living countries were under
control and eight had stability below 30; unlimited borrowing still produced
severe outliers, including Nicaragua at 23,505.20% debt/GDP.

## Completed all-AI campaign: 1990–2035

The all-AI scenario also completed 16,436 daily ticks and the seven-day exact
save replay. Opening world hash: `59a54b63968168c4`; terminal hash:
`d2abe18f62959fc5`. It used the same seed and enabled daily systems, with no
player. The observer did not enroll the USA in a player program budget. This
therefore tests an autonomous world, rather than isolating the effect of the
four guided tax votes in the matched player-budget setup above.

The AI-run USA ended at **50.4831% debt/GDP**, 32.9499% tax and 68.3071 stability.
Interest absorbed **3.7827% of revenue**, and the primary cash surplus was
1.9023% of GDP. The assessment included an estimated 1.5926% of GDP in other
recurring cash obligations. Its five-year debt projection was 52.4879%; it was
assessed as under control with no fiscal-confidence pressure. This illustrates
affordable ongoing debt, rather than requiring every government to reach zero.

Of 156 living countries, **148 were under control**. Thirteen still had debt
above 100% of GDP, and six had stability below 30. Nicaragua reached
22,924.38% debt/GDP, with interest equal to 3,424.92% of revenue and stability
37.05. Zaire reached 8,041.86% debt/GDP and 23.25 stability. Both received the
maximum fiscal-confidence pressure. Ordinary adjustments do not guarantee an
escape from those extreme paths, especially after tax reaches its AI ceiling.
Conversely, some low-stability countries had little debt and no fiscal pressure;
other game systems remain capable of causing instability.

These are descriptive results from one seed, not historical calibration or a
claim that every economy is balanced. Raw outputs, annual country rows and the
guided executed-action ledger are retained under `artifacts/fiscal-recovery/`.
The action ledger is not a complete audit of all AI proposals or unaffordable
commands skipped by the observer.

All three runs use the frozen fiscal simulation described above. The separately
developed replacement of industrial packs with factory capacity is not included
in these campaign comparisons and requires its own integration verification.
