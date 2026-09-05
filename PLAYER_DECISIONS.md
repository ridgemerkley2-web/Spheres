# Player decisions — approved audit repair (2026-09-04)

The user approved implementing the audit recommendations. These are named game
rules, not claims about historical deadlines or central-bank behavior.

AI defense-pact and trade offers to the human enter a persistent inbox. The
standing response defaults to **Ask me** for pacts, trade, and calls to arms;
each category may explicitly accept when legal or decline future offers.
Changing policy never answers existing requests. Treaties allow 30 calendar days,
defense requests seven. An unanswered request is declined on the first simulation
settlement at/after its deadline. Replying cannot consume an acceptance die.
Acceptance rechecks current treaty, sovereignty, conflict, access and nuclear
conditions. Resolved requests are removed exactly once; the latest 32 decisions
survive saves. Reused conflict IDs retire their previous invitations.

A defense pact remains a commitment with its existing upkeep and refusal costs.
A guaranteed defense refusal/expiry ends the pact, costs 25 reputation and 45
relations, and applies the existing broader credibility loss. Voluntary friendly
intervention can be declined without the treaty penalty. The player never enters
war through the old relation-only automatic intervention route. Sovereignty and
voluntary economic compacts keep their existing mechanisms.

Player sanctions persist until lifted or the target disappears. Reapplying an
existing sanction or lifting an absent one costs no political capital. The
existing diplomatic AI retains its automatic grievance-based repeal.

In daily simulation a peg holds the stated interest rate for every government.
The player must explicitly exit before changing it. Exit costs 12 political
capital (permitted even when reserves are low), five stability and two percentage
points of inflation, then resumes the automatic bank. The already-used stratagem
flag remains, preventing repeated peg-benefit farming. A floating manual bank
can resume automatic policy for free. Legacy daily peg flags adopt their saved
rate without replaying benefits; untouched monthly headless worlds retain their
old serialization and AI behavior.

The browser's Decisions button shows pending count, deadlines, acceptance blocks,
consequences, standing policies, monetary regime and recent outcomes. Opening it
pauses the clock; its native dialog keeps keyboard focus. Actions use the existing
session-bound command API without advancing time. `agency::view` and `/api/agency`
are pure reads. State is sparse and the new module uses no RNG.

Validation: three integration checks were first observed failing against upstream
4b31168 (automatic treaty acceptance, duplicate sanction charge, freely overridden
peg). Nine targeted simulation regressions and a web command/state transport test
then passed in a separate CARGO_TARGET_DIR. Tests cover save continuity, deadlines,
exactly-once costs, stale conflict references, explicit policy, and unchanged
serialization on a world that has not used the feature. Root runs the full suite
and integrated browser checks; no calibration tolerance or replay pin is changed
by this repair.
