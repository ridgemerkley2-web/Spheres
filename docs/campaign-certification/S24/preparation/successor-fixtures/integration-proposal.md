# S24 successor fixtures — integration proposal (not applied)

Status: **proposal for Codex review.** CLAUDE-S24-SUCCESSORS-01 owns no production
code, command dispatch, shared schema or CI; nothing below has been applied. The
harness records the affected cases as `unrun` / `no_native_hook` and cites this file.

## P1 — Namibia and East Timor have no activation path (needs a ruling first)

Both are successors in every source (`nations.rs` `start_1990: false`, C01
`countries.json`, census `successor_nations: 23`), but no existing entry point can
bring either onto the board:

| Needed for activation | Soviet/Yugoslav successors | Namibia, East Timor |
|---|---|---|
| A dissolution that seats the nation (`politics::dissolve_ussr` / `dissolve_yugoslavia`) | yes | **none** |
| `districts::SUCCESSOR_PARENTS` entry | yes | **none** |
| District list owned by a 1990 starter (so a parent can hand it over) | yes | **none** — `NA-*` and `TL-*` (13 each) are listed only under the successor, so the ground has no start owner |
| `campaign_journey::successors` continuation family | yes | **none** |
| Sourced opening state (a `data/nations` file is refused for non-starters by design) | shares of the parent, in `politics.rs` | **none** |

`nations.rs` already says so for East Timor ("A GAP THE INTEGRATOR SHOULD SEE …
Nothing in the sim currently spawns this nation"), and Namibia's row explains it is a
successor only because independence (21 March 1990) falls after the start date.

**Why the harness does not fake it.** Staging a parent collapse (recipe H1) cannot
help: neither has a parent whose collapse seats it. Writing a Namibia or East Timor
nation record into a save would invent an unsourced opening state (GDP, population,
finances, forces, technology) — iron rule 4 refuses that — and would be a forged
activation rather than a fixture of the game's own behaviour.

**Decision needed (Ridge/Codex).** Whether either state should become activatable in
1990–2035 at all, and by what non-scripted mechanism (BIBLE §5). Namibia's independence
was a settlement already in force on 1 January 1990 with a known date; East Timor's
depends on Indonesian politics. Neither is a federation coming apart, so the S21
"continue as a successor of your dissolved government" model does not fit either one:
South Africa and Indonesia survive the separation.

**If approved, the smallest native change** (Codex-owned files):
1. Sourced opening data for each (a successor data file the loader accepts for a
   non-starter, with a `sources` block), and a decision on 1990 ownership of the
   `NA-*` / `TL-*` districts.
2. One seat function in `politics.rs`, e.g. `seat_independence(w, heir, from)`, that
   seats the nation from that data, transfers only its own district list, and runs
   `government::ensure_all` exactly as the dissolutions do (so a first-day save
   validates, the S21 repair).
3. Whatever trigger the ruling allows, through the tick systems (no side door).
4. Only if a player could legitimately continue into them: a continuation rule in
   `campaign_journey.rs` (a separation, not a dissolution).

## P2 — optional test-only exporter for every activatable successor

The existing exporter `campaign_journey::tests::s21_export_review_fixtures` writes the
USSR dissolution only; the Yugoslav family therefore uses the harness-staged recipe H1.
The harness's cross-check showed that H1 (with the served aim command) produces a
**byte-identical simulation world** to the native USSR fixture, so this is a
simplification, not a correction. A native Yugoslav fixture would need eight lines in
the existing test module, reusing its `dissolved` helper unchanged:

```rust
#[test]
#[ignore = "Exports authored S24 successor fixtures to a NEW SPHERES_S24_FIXTURE_DIR"]
fn s24_export_successor_fixtures() {
    let path = std::path::PathBuf::from(std::env::var_os("SPHERES_S24_FIXTURE_DIR").unwrap());
    assert!(path.is_absolute() && !path.exists()); std::fs::create_dir(&path).unwrap();
    for (parent, file) in [(NationId::USSR, "ussr-dissolved.json"), (NationId::Yugoslavia, "yugoslavia-dissolved.json")] {
        std::fs::write(path.join(file), storage::encode(&dissolved(parent)).unwrap()).unwrap();
    }
}
```

`run.cjs` would then read `yugoslavia-dissolved.json` like `succession.json` (a
new native recipe beside N1). If P1 is approved, the same exporter is where a labelled
Namibia/East Timor fixture (seat, then authored player control) would be produced;
until then no exporter can create them.

## P3 — budget card mislabels the real-rate floor as a "sovereign spread" (finding F1)

Observed in every activated successor's Economy → *Your yearly budget* card, e.g.
Russia on the fixture date: *Rate paid −2.00% · real −70.00% +68.00pp sovereign spread
at 35% of GDP*, and debt service −$6.2bn/yr (the screenshot
`evidence/screenshots/serbia-after_activation-budget.jpg` shows Serbia's −34.90% /
+32.90pp at 28%).

* `economy::effective_interest_rate` = `(policy − inflation).max(REAL_RATE_FLOOR)` +
  debt spread, with `REAL_RATE_FLOOR = −0.02` and no debt spread below the 60% knee.
  The **effective rate (−2%) is the simulation's own and is served correctly.**
* `policy_json` in `spheres-web/src/main.rs` recomputes `real_rate = interest_rate −
  inflation` **without the floor** and serves `spread = effective_rate − real_rate`, so
  the floor adjustment is presented as a sovereign spread at a debt ratio where the
  simulation charges none. This is the "number recomputed outside the definition"
  pattern iron rule 8 warns about.
* The successors open with authored inflation above their policy rate (Russia 0.90
  against 0.20, from `dissolve_ussr`), so all 21 show it; any starter whose inflation
  exceeds its policy rate by more than two points would too.

Focused fix (Codex): have `economy.rs` expose the two parts it already computes (floored
real rate and debt spread) through one function used by both
`effective_interest_rate` and `policy_json`; serve `real_rate` as the floored value
(optionally with the unfloored rate and a `real_rate_floored` flag); and let the card
say "real rate floored at −2%" instead of "sovereign spread" when the floor binds. No
simulation number changes. The harness records the served values (`observations[F1]`
in `evidence/result.json`) and asserts no money semantics.
