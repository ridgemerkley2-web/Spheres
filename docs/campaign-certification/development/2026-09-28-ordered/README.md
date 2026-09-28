# Ordered development checkpoint — 28 September 2026

The eight-task order is recorded in [the engineering handoff](../../../planning/ai-handoffs/CODEX-NEXT-ENGINEERING.md).
These are bounded engineering/review attempts. The failed A1 gate and unfinished
full campaign matrix must not be relabeled as complete to advance other work.
Canonical S23/S24/S25/S27 and CP1 prerequisites remain unchanged.

## Restored source and regression evidence

After rejecting A1 trial 01, the unchanged baseline was rebuilt from clean
`3ec6e155a416f713149820f9d6276b5ad2e7d721`. The full ordinary release workspace
suite passed **1,979 tests**, with 113 ignored and zero failed or filtered, across
67 test targets. The supplied skip string matched no test, so it excluded none.
This ordinary-suite pass does not include the separately ignored A1 outcome gate,
which still fails. The web build also passed. Exact binaries and command logs
are pinned in [the restored build receipt](evidence/restored-build-receipt.json).

The first UI regression invocation mistakenly included standalone browser
drivers that require explicit URLs/binary/output arguments. It reported 1,805
passes, eight invocation failures and one optional skip. The corrected inventory
matches `tools/ui/run-unit.cjs`: all `check_*.cjs` unit files excluding `_browser`
drivers. That run passed **1,798 tests**, zero failed and one optional browser
skip. Both the erroneous invocation and the corrected inventory/log are retained.
The browser drivers' real campaign runs are separate evidence, not skipped UI
failures presented as passes.

An intermediate island build occurred with the generated gap ledger dirty; its
`island-map-build-02.log` is retained but that executable is not the candidate.
After committing the ledger, the final web binary was built from clean
`8d338dd217af0e4c83725f999812eef37e55bbb7`, SHA-256
`fd6989f3cbbd60abb247fd289dc41eb12c30424222b1f09971ac0e7f9f50dec7`.
The final browser run uses that frozen binary with separately pinned harness
`c35310531c247e1c0754284672f7beb482d7f75c`.

## Startup and controlled succession

The restored baseline's native startup sweep passed all 137 identities, ordinary
seven-day progression and complete save/load comparison. The first full browser
sweep passed 131/137; its six failures exposed the harness's inappropriate
owned-polygon requirement for six canonically zero-district island countries.
Actual visible-label map selection was then added and checked. The first island
pilot revealed three city-radius collisions; visible country labels now take
priority while route/work interactions retain their priority. The final six-case
pilot passed. All original failed results remain beside later attempts.

The [final startup closeout](../../S24/preparation/worldwide-startup/README.md)
records 137/137 actual-browser and 137/137 native passes on the same 8d runtime.
Independent browser evidence verification passed 1,377 artifacts. An extra native invocation from the later integration checkout
was refused before execution because its HEAD differed from the requested 8d
binary. That refusal is retained. The exact 8d source was then checked out cleanly
for the final native refresh; no provenance check was bypassed.

The separate [controlled succession proof](../../S25/preparation/controlled-succession/README.md)
passes with twenty complete retained archives. It requires an actual USSR-to-Russia
transition, ten scheduled reloads and full archive comparisons; it cannot pass by
falling back to observation and makes no organic-history claim.

## Full matrix resource attempts remain incomplete

Both attempts kept the full eight-country, three-seed, 1990–2035 plan. Attempt 01
used eight workers on the external USB drive; observed disk contention prevented
useful throughput. Attempt 02 moved eight active cells to the internal SSD. Its
saved campaigns reached early 1994 and about 136 MB each, consuming the remaining
system-drive space. Only those owned test processes were stopped. Neither attempt
completed a cell or supplies a matrix pass. Operator interruption receipts retain
the exact process, binary and checkpoint identities.

The [read-only growth audit](evidence/state-growth-review/README.md) found 496,309
course records with distinct accumulated education funding. These represent
actual migration histories; there is no established stale-history/duplication
bug to justify discarding them. Fiscal/history retention already has explicit
bounds. No funding rounding, population pruning, shortened horizon or weaker
save comparison was introduced. Resource changes belong to test execution and
lossless evidence storage. A 2035 resource envelope has not yet been demonstrated.

Interrupted output roots are retained at:

- `D:/spheres-offload/codex-next-20260928/matrix-full-01`
- `D:/spheres-offload/codex-next-20260928/matrix-full-ssd-02-interrupted`

The second was moved intact from the recorded SSD path only after all owned
writers stopped and the read-only audit closed its handles. The original
absolute paths remain provenance. The France archive's 135,739,703 bytes and
SHA-256 `5480ccc296c2fe67ef2234871d1ef5fc9f098184ca1c83d885b07010a1368b20`
identify the audited 1 January 1994 save; a prior progress line was December 1993.

## Independent historical intake

France C01-23, Tonga C01-24, Saudi C01-25 and India C01-27 were integrated in that
order at `065341c94ee92b1f541c4a1c1cf1cfc401c0032b`. Their bounded reviews cover
243 declared source responses and 454 material claims, plus India's separate
dating anchor. Original access failures and subsequent recoveries remain
separate. The combined checks passed 268 test executions; these suites overlap,
so that is not a unique-test count. Source bodies and research scope do not grant
portrait reuse, runtime leader installation or complete historical coverage.
See the [ordered integration receipt](../../C01/integrations/ORDERED-2026-09-28/README.md).

The additional [A1 reassessment](evidence/a1-reassessment-02/README.md) found no
new defensible runtime correction. Source/test hashes remain unchanged and
reserved holdout seeds were not opened. The firing-site observation gap is
recorded as a possible diagnostic, not a proven bug or a passing gate.

[manifest.json](manifest.json) hashes every compact evidence file in this packet.
Large external raw saves are not embedded in this compact Git checkpoint; their
local locations and hash identities remain explicit.

## Distributed execution and verification tooling

Candidate `5d970f6d7370baf16760585c641d81253d1c2175` is pushed on
`codex/s25-run-20260928` and the integration branch. Its
[actual GitHub run](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36474011141)
freezes one compiled executable, the entire plan and verification code, then
assigns each full cell to its own standard Ubuntu runner. The new managed scratch
option preserves verified gzip evidence before deleting only its owned copied
files. Seventy-one synthetic tooling tests ran: 70 passed and one host-privilege
symlink-creation check skipped; the independent reparse refusal check passed.
The final source-clean guard change also passed all eleven distributed tests.
A real tiny NTFS inheritance/payload check passed separately. These are tooling
checks, not native campaign passes. Source review found and fixed a possible
cross-batch result-mixing gap before launch. Every shard now binds the exact
central batch manifest, and the aggregator must verify all 24 actual retained
archives before a full-matrix success can be recorded.

The root post-merge research spot-check initially used two nonexistent unittest
filename patterns for the campaign and atlas suites; both correctly reported zero
tests and nonzero exit. The existing recorded commands were then used unchanged
(`test_campaign*.py` and the Node leadership-research atlas check), and passed.
Both original command/log receipts and corrected commands are retained under
`evidence/root-combined-research-checks/`. This does not revise the already passing
268-execution combined source review.

The source-gap ledger was refreshed after the four intake decisions. It required
explicit per-packet acceptance records, correct original-commit attribution and
a separate completed-intake path rather than treating every completed task as a
source repair. All 28 ledger tests and exact regeneration checks pass. Its 50
party chains with unresolved days and 51 of 60 research roles with unresolved
days remain explicit; the four accepted packets do not grant runtime mappings.
The original regeneration/setup failures remain in `evidence/ledger-ordered-update/`.
