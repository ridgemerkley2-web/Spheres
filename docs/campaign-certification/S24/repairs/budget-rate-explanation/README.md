# Budget interest explanation — completed repair

Claude's S24 successor preparation found that the treasury/budget API labeled
every difference between the raw real rate and charged rate as a sovereign
spread. When the simulation's real-rate floor applied, even a low-debt country
appeared to pay a large risk premium.

Candidate `52e1c2ab7aa2965fe5bfbb2685354231da9773fa` serves the floored base and
floor adjustment separately. The sovereign spread is the remaining difference
between that base and the effective rate. Both budget explanations now show
the components. The base comes from the existing simulator rate function with
zero debt spread; no economic formula, balance, save schema or charged amount
changed.

Validation:

- 25 cabinet UI tests pass. The new regression fails against the original UI;
  its failed output is retained.
- Two native money-card tests pass, covering floor-only, floor plus risk premium,
  ordinary rates and the sovereign-spread cap. They reconcile to the unchanged
  fiscal charge.
- A freshly built native binary passes actual Brazil and France first-budget
  browser journeys at 1440 and 390 pixels: four screenshots, no horizontal
  row overflow and no page errors. Both Brazil screenshots were visually
  inspected. These are ordinary fresh campaigns in a new disposable save
  directory, not edited campaign fixtures.

The browser checks pin the exact binary and embedded assets to the recorded
candidate. Reproduce with:

```text
node --test tools/ui/check_cabinet.cjs
cargo test --locked --release -p spheres-web the_money_card -- --nocapture
node tools/ui/check_budget_interest_browser.cjs BINARY FULL_REVISION NEW_OUTPUT
```

The browser command needs Playwright available through Node and a local Edge
installation by default; set `SPHERES_BROWSER_CHANNEL` for another installed
channel. It starts and stops its own server and refuses an existing output
directory. The recorded run used the bundled Node runtime and Edge.

The simulation tree remains unchanged from S22. S22's performance evidence
still describes its original `5d11dd6d` candidate; this focused presentation
repair does not constitute a fresh S22 run, complete S24, or award CP1.
The [manifest](manifest.json) records exact evidence bytes and source scope.
