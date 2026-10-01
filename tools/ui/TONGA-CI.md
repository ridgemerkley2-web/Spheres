# Tonga production browser journey

`ci-tonga.cjs` runs in the separate `tonga-browser` CI job on Windows and Linux.
It builds no service endpoint or alternate government implementation. The job
runs the focused native institutional and Tonga government/portrait regressions,
then builds the release web executable; the harness invokes the release example
`spheres-sim/examples/tonga_browser_fixtures.rs` from the exact clean checkout,
then launches its own server in a newly created disposable directory. It refuses
to run outside CI and accepts no existing server address or save directory.

The first case starts a real fresh Tonga campaign through the country picker.
It checks the separate King and opening Prime Minister, all four fictional
preview portraits, all 50 historical reference cards (including King IV's actual
image and Fatai Helu's explicit missing-art fallback) and mobile layout. Served
art and UI bytes must match the checkout and its exact build revision.

Later cases load simulation saves generated through public native init, command
and save APIs. Their dates and 500 political capital are authored inputs, with
zero simulated days. They deliberately retain the opening monarch and premier
counterfactually. The normal production Load path applies its usual legacy-save
capabilities. These cases establish interface and institution behavior, not
historical chronology, campaign endurance or the fresh integrated economy at a
later date.

The browser exercises review, cancellation and confirmation of Assembly reform,
election, a separate Prime Minister, non-elected minister and a real vacancy.
It checks the 2026-09-07 cutoff, first fictional day, inclusive 2035 endpoint and
2036 refusal of new appointments while existing incumbents remain. Kalolo stays
a fictional reference with no office action. Reviews/cancellations preserve the
complete saved world/history; Save/Load and reload/Continue preserve it too.
Equality uses the exact native serialized text, replacing only the final
top-level `saved_unix` value. Native 64-bit integers retain every digit, and
nested fields are never normalized. Load waits for the actual discovered live
session before asserting the normal replacement confirmation.
Every successful order uses the normal review token and command receipt channel,
spends the quoted political capital and preserves the Crown identity. No advance
request, synthetic API response, direct browser state mutation or forced click is
used. Refusals are checked in disabled controls and native preview responses.

Artifacts retain the exact revision, source/fixture hashes, export/server logs,
authored and completed saves, hashes of every comparison snapshot (two rolling
slots retain the final snapshots and their backups), command/refusal evidence,
screenshots and failures. `result.json` is
written with `passed: true` only after all browser assertions finish. An actual
passing run must be reviewed and pinned before changing the country's pending
`production_browser` acceptance item. This harness does not provide likeness
approval, all historical role chains, human signoff or CP1 qualification.

Local checks (no native build/server):

```
node --check tools/ui/ci-tonga.cjs
node --test tools/ui/check_tonga_ci_contract.cjs tools/ui/check_tonga_institutions.cjs
rustfmt --check --edition 2021 spheres-sim/examples/tonga_browser_fixtures.rs
```
