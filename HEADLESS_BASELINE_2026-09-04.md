# Approved headless replay baseline — 2026-09-04

The user approved implementing the full audit, including the E-3 design repair
and subsequent assessment of stale replay references. This records the new
baseline rather than describing the fingerprint update as a gameplay fix.

| Fixture | Previous golden literal | Observed before this repair | New assessed reference |
| --- | --- | --- | --- |
| Default seed 1990, January 1990 before any tick | `0xd022d50f43c984da` | `0xe26e4bf8d6c60066` | `0xe26e4bf8d6c60066` |
| Same world, 240 monthly ticks with no commands or player | `0xbd5ec0f43c5f2e3b` | `0xbe94d6125631829c` | `0x0cbd02497c30957c` |

The fixture uses `GameRules::default()`: legacy monthly simulation, ordinary
aggression/crisis intensity, resource gates on but resource market, physical
logistics, production, manufacturing and Economic Competition off. No budget,
player inbox, peg, optional aim or inherited industrial estimate is enrolled.
Root release feature additions must remain default-off here. The integrated
workspace suite independently checks these same fixtures after all merges.

The start fingerprint's previous drift predates this work. BUGS E-3 and the
existing resource/treasury inertness tests already recorded the actual
`0xe26e4bf8d6c60066`: district population and population-scale fields serialize
unconditionally. The E-3 correction adds no key or initial arithmetic at t=0,
so that observed start remains exactly unchanged.

The 20-year reference changes intentionally for two reasons:

1. Runtime productivity now excludes subsequently credited revelations of
   already-priced 1990 knowledge. This corrects the board-wide growth effect
   of an authoring gap, while preserving inherited-stock GDP weighting and all
   research prices and growth coefficients. Different resulting economies may
   produce different later stochastic events; a historical RNG state is not an
   invariant across an approved semantic change.
2. Each positive credited revelation now records its exact cumulative value in
   the sparse `tfp_1990_revelation` ledger. This persists and inherits with the
   known technology, so future replay does not lose the benchmark correction.
   It is omitted when zero and defaults to zero in older saves.

Determinism remains the invariant: one RNG, no wall-clock input, no unordered
state-changing iteration, repeatable runs and uninterrupted-versus-resumed worlds
must agree. The 12-month endowment A/B still has identical RNG streams. The
existing save/load continuity check passes after the correction. The golden
fingerprint catches any future unassessed difference in the entire serialized
state; its tolerance remains exact equality.

Validation before re-pinning: the original endowment guard, all twelve technology
tests, the 35-year frontier panel, China growth-miracle panel and mature-economy
panel pass with unchanged samples and tolerances. The endowment margin scan reads
worst growth difference `4.5080e-5` versus `1e-4`, and GDP difference `4.9489e-5`
versus `2e-4`. No historical data was adjusted to obtain those results.

Negative controls were actually run: temporarily restoring the raw runtime
benchmark makes both the new credited-revelation invariant and the original
endowment guard fail. Belgium returns to 0.001851 granted versus 0.001749 control.
The mutation was restored in a finally block before the final build. This is in
addition to observing the original endowment failure on upstream 4b31168 before
implementing the repair. Logs are retained in the audit workspace.

The two golden literals and the duplicated *actual* 20-year references in the
resource/treasury inertness checks move together to this assessed value. The
inertness assertions themselves still run: gates on/off worlds must match, empty
resource and treasury state remain absent, and save/resume must reproduce the
same future. No threshold was widened, assertion removed, or expected error
ignored. Root runs the final full workspace suite and should reject this baseline
if any integrated default feature unexpectedly changes these measured values.
