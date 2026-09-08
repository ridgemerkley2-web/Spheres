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
control.

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

Final local checks passed on 2026-09-08:

- 1,337 workspace Rust tests, zero failures; 70 existing tests remain ignored.
- The separate resource timing test passed at **0.0739 ms/model month** against
  the unchanged **0.15 ms** gate. Its executable remained byte-identical after
  the final web-only fixes, so the isolated reading still covers the final
  simulation source.
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

Three problems found during QA were repaired rather than weakening checks:

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

The final fiscal observer executable has SHA-256
`6696E9B9C401FB057F502E0F40E23321451EF2DB586760D4EB13D24EF446CEB0`.
The final browser QA executable has SHA-256
`F87E99A6878CE3553BBD8C5CBC5E011B6122493F9405D0CC6E37566003DADD66`.
The browser fixes did not change the simulation used by the endurance observer.
Local receipts, earlier failed runs and final results are retained under
`artifacts/fiscal-recovery/` (not committed). The earlier
`USER_QA_1990_2035.md` describes the prior fiscal model; it is separate evidence.
