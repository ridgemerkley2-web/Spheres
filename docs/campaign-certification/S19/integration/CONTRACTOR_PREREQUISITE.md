# Contractor prerequisite — S19 development checkpoint

27 September 2026. Runtime repair: `8b93372e`; clean tested build: `2cb1da4a`.
S19, G4 and full campaign certification remain open.

## Player-facing repair

A fresh France campaign has no completed free Arms Plant. Previously the company
screen offered contractor establishment with an empty province, and its review
said the government no longer owned that province. It now disables that action,
shows an explicit construction prerequisite, and retains the **Build an arms
plant** navigation. If existing slots are unavailable, the message instead asks
the player to review commitments or add a plant. Native review also explains an
empty province correctly. No money, facilities, research or inventory is granted;
save format and construction timing are unchanged.

## Verification

- 16 company web tests passed; one existing ignored qualification test remains ignored.
- 11 company simulation tests passed.
- 25 outcome-recognition checks passed.
- 218 equipment, company-network and guidance UI checks passed.
- Locked release build passed with three existing vendor warnings.
- Fresh first-hour native/browser regression passed: real reviewed commands,
  delayed-response checks, named save/load, reload/Continue, new-campaign
  isolation, keyboard controls and narrow layout. Desktop and narrow screenshots
  were inspected.

[Raw results, exact sources, save hashes and file hashes](contractor-prerequisite-evidence/manifest.json).
The first-hour driver bytes were independently matched to their captured SHA-256
and to the committed source after line-ending normalization. This evidence does
not claim company delivery, squadron readiness or flown missions.

## Longer campaign qualification

The opt-in `SPHERES_GUIDANCE_LATER_PROCUREMENT=1` path extends the existing
browser driver using visible construction, Cabinet and company controls. The
default first-hour path retains its four command-endpoint confirmations.

The longer path builds the missing plant, renews annual budgets, commissions the
saved design, invests quoted company capital, waits for actual stock, buys a
reviewed unit and requires matching payment/delivery receipts. It then exercises
save/load, Continue and campaign isolation. A plant needs 720 funded work days;
waiting without renewing the annual budget correctly pauses construction.

An exploratory run completed the plant on 6 January 1992, then correctly refused
the default $25m investment because procurement authority was insufficient.
That run is not a procurement pass. The follow-up uses a reviewed $1m initial
investment and a separate investment based on the quoted stock requirement,
waiting for ordinary authority to accrue. That follow-up established the company,
commissioned and funded development, and completed tooling. It then stopped at a
real advanced-components shortage; no stock, purchase or delivery was qualified.
See the [supplier-input checkpoint](PROCUREMENT_SUPPLY.md) for the complete failed
route evidence and the next bounded action. The later outcome remains unqualified.
