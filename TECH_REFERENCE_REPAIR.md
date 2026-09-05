# E-3 productivity benchmark repair (2026-09-04, approved audit work)

The user approved the audit report's changes. This implements the E-3 design
repair while preserving every existing calibration tolerance. It changes a core
productivity semantic, so the default monthly timeline is expected to change.
It is not covered by the optional UI features' legacy-inertness promise.

A credited revelation is a technology missing from a country's 1990 authored list
whose productivity was already included in its transcribed trend. The engine
already subtracts its realized saturated value from that country's TFP base and
records the offset. But the old global reference still included the revelation.
It therefore reduced every country's trend when an authoring gap closed, although
no economically new productivity had appeared. BUGS E-3 measured this mechanism.

The runtime benchmark now subtracts only the cumulative productivity value of
subsequent credited revelations. The raw initial GDP-weighted reference and the
initial inherited stocks retain their existing definitions and weighting. Genuine
uncredited technology still moves the benchmark. Both daily and monthly ticks use
this same rule; no technology cost, acquisition cap, growth coefficient or starting
country figure changes.

A sparse `TechState.tfp_1990_revelation` ledger records exactly the existing
`credited` amount at the same settlement. It inherits with the technology stock
on succession, resets on explicit setup rebasing, and survives save/resume. Old
saves default to zero: no retrospective credit or benefit is invented on load.
New worlds omit the field until their first positive credited revelation, so this
repair does not move the 1990 start hash by itself. It intentionally changes later
serialization and the economic path; later stochastic events may consequently
change even though the one RNG and its deterministic ordering are preserved.

Evidence from the separate diplomacy-changes target directory:

- Original upstream 4b31168 behavior was observed red: Belgium growth 0.001851
  granted versus 0.001749 control, exceeding the unchanged 1e-4 bar.
- The final repair passes that 12-month A/B with identical RNG streams. Across
  its 20 granted countries, worst growth difference is 4.5080e-5 (45.1% of the
  existing bar); worst GDP difference is 4.9489e-5 (24.7% of the 2e-4 bar).
- The new invariant reveals economically meaningful 1990 knowledge and verifies
  that the raw benchmark changes, the corrected benchmark does not, and genuinely
  uncredited progress still changes it. The successor carries the credit.
- Twelve technology tests pass; one existing descriptive scan remains ignored.
  The original 35-year frontier panel, China growth-miracle panel and mature-economy
  panel pass unchanged. These bars retain their existing samples and tolerances.

An earlier broad candidate subtracted the whole rebased 1990 offset. It changed
how inherited stocks were weighted as economies grew and produced a mature-economy
failure. That candidate was discarded. The final change only corrects subsequently
credited revelations, preserving historical weighting rather than tuning a growth
coefficient until a seed passes.

The existing endowment channel probe now reads the runtime benchmark when
attributing the trend; its setup still uses the raw initial reference. Golden
hashes are intentionally not re-pinned in this commit. The final integrated tree
must first establish the expected semantic/serialization changes and run the full
workspace suite; only then should its documented start and 20-year references
replace stale literals. No test is deleted or weakened to accomplish that.
