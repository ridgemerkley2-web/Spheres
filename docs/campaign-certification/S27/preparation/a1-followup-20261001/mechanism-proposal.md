# Proposed next mechanism: a recorded dispute over civilian authority

**Design proposal only. No new trigger is installed or accepted.** The current
evidence does not demonstrate another implementation defect. It does identify
what an additional political mechanism would have to observe before it could
be implemented honestly. This proposal does not predict an A1 pass.

## What the existing evidence can and cannot explain

The accepted repair's unchanged twelve-seed A1 result is 7.5 median coups and
4/7 median top-three share. One seed is strictly below one half; three equal
one half. Reducing or redistributing totals to pass that statistic is not a
causal explanation. The gates and cohorts remain fixed.

The separately retained **pre-repair** seed-0 trace shows Pakistan and Thailand
reach the electoral Army tick throughout all 252 months, but never satisfy
either the hostile-Army or discontent threshold. Haiti and Nigeria never reach
that electoral branch. A missed funding command or faster response to an
already selected negotiation cannot create the absent political condition.
These are observations of modeled behavior, not claims that particular real
coups must occur, and not a current repaired-runtime trace.

The existing route models a materially or economically distressed Army. The
existing election annulment separately models a prospective programme veto.
Neither route records a refusal of a specific civilian command outside an
election. The [earlier model review](../a1-geography-20260928/model-review/README.md)
already identified that limitation; this proposal makes its implementation
boundary explicit rather than treating military autonomy itself as a trigger.

## Observable trigger and persistent record

Add no background country/date lottery. A dispute can begin only after an
actual player or AI command has legally attempted a concrete change to an
Army institution's executive authority, such as bringing appointments under
civilian control, and the institution has explicitly refused that order.
This would require a new visible governance action and a separately justified
institutional acceptance rule; neither is present in the retained trace.

Record the institution identity, originating command receipt, issuing civilian
government, requested authority change, refusal date, unresolved issue and
subsequent resolution receipts. Keep this campaign record separate from the
immutable 1989 authority assessment and from material funding/loyalty. Old
saves and unknown institutions must load with **no dispute**, not inferred
hostility. Multiple institutions cannot be collapsed into the first Army row.

A refusal must not follow automatically from a nonzero historical leverage
score. Before coding, specify which saved institutional powers an order would
remove, what lawful authority the government has to issue it, and what
observable compliance or refusal event follows. Sourced initial authority and
a disclosed game rule must be distinguished. A party change, a poor country,
a historical coup date, or an unpopular government cannot stand in for that
event. The current evidence supplies no independent calibration for refusal
probabilities or escalation rates; none are proposed here.

## Player-facing behavior and action limits

Expose one government alert naming the order, institution and unresolved
issue. The same server-owned assessment should quote three possible actions:
negotiate a narrower reform, withdraw that order, or continue enforcing the
lawful reform. Their costs and consequences need a reviewed design before
implementation. Paying equipment or operating bills retains its material
benefit but cannot silently settle the authority dispute.

The AI must use the same commands, prices and refusal reasons for a positive
governance objective; it must not manufacture disputes to satisfy a coup
count. Only an unresolved recorded dispute could feed a distinct political
pressure route. Resolution stops further accumulation immediately; an old
gauge cannot manufacture a coup after the dispute ends. Election annulment
must remain a separate event and cannot charge the same refusal twice.

## Smallest implementation and red-regression plan

1. Review the command, institution powers, compliance rule and visible costs.
   No runtime trial is justified until these specify an independently
   observable event and a desired behavior, including the AI's reason to act.
2. Introduce a receipt-backed dispute record and a read-only assessment before
   any overthrow path. A regression should show a legally issued, refused
   order is not currently represented; merely lowering loyalty is not that
   regression. Keep this initial state/command work separate from calibration.
3. Add control pairs with identical provisioning and macro conditions: no
   refused order produces no dispute; an actual recorded refusal creates one;
   accepted/withdrawn/resolved orders clear it. Missing Army, missing civilian
   authority, routine succession, and strong lawful compliance are controls.
   Check distinct same-type institutions, daily/monthly clocks, save/load,
   replay and no added RNG draw on the inactive path.
4. Only after the political escalation contract is approved, demonstrate its
   own failing regression and review its costs, warnings, resolution and
   attribution. Keep the existing material and programme-veto paths intact.
5. Re-run the original A1–A10 gates with their original cohorts and horizons,
   plus the focused political regressions and required engineering checks.
   Report failures without changing thresholds, adding country exceptions,
   sampling the reserved cohort or selecting coefficients for the observed
   twelve seeds. A useful mechanic can still fail A1; that must stay visible.

**Immediate disposition:** record the passed resource assertion with its
background-load limitation; uncontended timing is still pending. Keep A1 and
S27 open. This contract proposal is the next reviewable design
decision; it is not a substitute for the missing trigger evidence or a reason
to revive the rejected urgent-response or confidence-weight trials.
