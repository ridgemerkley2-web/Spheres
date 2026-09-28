# S19 — supplier-input checkpoint

27 September 2026. **Later procurement qualification did not pass.**
S19 remains in progress; S20 and CP1 retain their dependencies.

The ordinary France campaign ran on clean runtime `2cb1da4a`, with the contractor
prerequisite repair in `8b93372e`. It used visible controls and native daily
advancement. No campaign money, research, stock or response bodies were fabricated.

## Observed results

- Built the missing Arms Plant in Auvergne-Rhône-Alpes; a free slot became
  available on 6 January 1992.
- Renewed the 1991, 1992 and 1993 budgets through the Cabinet screen.
- Established a manufacturer with a reviewed $1m investment; reopened the real
  saved tank draft in the Designer and commissioned development.
- Invested another $46m through a reviewed company order. The stock-capital quote
  was $41.006m; the chosen amount rounded up a 10% allowance for changing prices.
- The native company reading reached certified development ($133.23m paid) and
  30 of 30 tooling workdays ($22.21m paid).
- Finished stock stayed at zero. The last recorded product reading, 2 June 1993,
  reports: “The government warehouse lacks advanced components. Production and
  arrived purchases supply this shared finite stock.”

The driver stopped after its 540-day development/stock bound. Its raw result
remains `passed: false`. It placed no equipment purchase, so it cannot qualify
payment, delivery, later save/resume or flight. The separately retained
[first-hour regression and prerequisite checks](CONTRACTOR_PREREQUISITE.md) passed.

## Evidence and recovery

[Manifest, source hashes and exact boundary](procurement-supply-evidence/manifest.json)
includes raw browser results, dated product readings, annual-budget guidance,
captured driver files and their original run-start hashes. The binary hash was
unchanged throughout the run; there were no page errors.

[Recorded native campaign](procurement-supply-evidence/recorded-france-supplier-checkpoint.json.gz)
is the unedited **1 July 1993 France autosave** from that run. Its compressed and
uncompressed hashes are recorded in the manifest. Decompress into a separate
server's `saves` directory under a new slot name and use the visible Load flow.
Preserve the original file and do not replace another campaign. Loading this
checkpoint is the start of the next investigation, not evidence that delivery
or save/resume qualification already passed.

## Next bounded Codex work

1. Resume the recorded campaign and inspect the company's exact next input packet,
   warehouse stock, normal industrial production and available purchase routes.
2. Determine whether the missing step is player navigation, a funding/supply
   decision or an implementation defect. Reproduce and test any defect before
   changing runtime rules. Do not waive the physical input requirement or inject
   advanced components to make the test pass.
3. Supply the company through ordinary player controls, obtain finished stock,
   place a reviewed purchase and verify its exact paid/delivered receipt.
4. Save, load and Continue; then check a fresh campaign cannot inherit results.
   Preserve the verified construction evidence. Readiness and a supported flown
   mission remain separate S19 work after procurement.

The opt-in driver remains exploratory until this missing input path and the
downstream checks pass. Its default first-hour mode remains separately qualified.
