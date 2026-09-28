# S22 diplomacy diagnosis and sovereignty validation

This append-only packet preserves the completed actual-2035 diagnosis and
candidate-scoped sovereignty tests. **Qualification is false. S22, G5 and CP1
remain unearned.** It does not contain results for the newer combined route
repairs; those builds/tests were still ongoing at the `8de6f2a0` snapshot.

At `9823b070efea06a4f6e63e955d8ae23445108054`, the instrumented diagnosis
matched the ordinary native world's complete serialized state and returned
headlines on all **31 days**, from 2035-11-30 through 2035-12-31. The source
remained unchanged; the ending world fingerprint was `3c3f21d9d67bd7b8`.
The original profile, provenance and native log are retained in `diagnosis/`.

The sovereignty stage took **14,170.8322 ms** while settling **1 December to
2 December 2035**. This located repeated trade-ledger scans for country pairs
that already lacked a necessary protection pact. These diagnostic timings are
not latency acceptance or a memory measurement; nested detail timers overlap
their parents and must not be added to them.

The narrow candidate prefilter keeps public quote details, live proposal order,
ranking and existing pact eligibility. It skips only pairs that the original
quote could never accept. At
**`6de1d9972fb3b1a7d5b67f4b73e08420687508d8`**:

- Ordinary candidate tests: **2 passed, 0 failed, 1 ignored**.
- Explicit actual-2035 original-search oracle: **1 passed**, comparing complete
  native worlds and returned/retained headlines for 31 days, with an unchanged
  source. It adds no player annual-budget renewal and makes no timing claim.

The actual-oracle provenance originally recorded the abbreviated `6de1d997`.
It is preserved unchanged. `validation/provenance-resolution.json` independently
resolves the full Git commit and records a fresh read-only SHA256 check of the
immutable simulation test binary:
`74611ed125e693a98d9a130806272e5b2b7dfe2f35425b3ea54c582c891baa9a`.
No executable is copied here.

`reviews/` contains independent **source-only agent reviews** of the sovereignty
prefilter, resource contract route reuse and immediate deployment/supply graph
handoff. Reviewers did not run builds, tests or benchmarks. The sovereignty
review's additional unusual-pact fixtures and integrated-envelope fixture fix
were added later; their validation is not inferred from the earlier 6de results.

The combined snapshot includes contract/campaign routing implementations and
fixture adapters, but this packet does not claim their in-progress checks have
passed. Fresh isolated native measurements are still needed after the combined
candidate is validated. No complete authoritative qualification pair has started.

The exact input is already archived in
[late-input-progress](../late-input-progress/inputs/reviewed-late-input-manifest.json),
uncompressed SHA256
`67aadca2f55280abc0e4ad97944a654ec7d173ffb65cb7d71dd59090e10503cb`.
`manifest.json` records source paths, byte counts and hashes. The large profile
is losslessly gzipped with its original hash and verified round trip. All other
captured artifacts retain exact bytes; `.gitattributes` disables newline
conversion. Original failures and earlier packets remain unchanged.
