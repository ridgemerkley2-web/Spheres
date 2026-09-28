# S22 contract dependency pair guard: independent source review

Reviewed commit: `2774b0c5495a92de0ffd7abfedf10e5a7dd6eeaa`.
Reviewer: Codex subagent `/root/review_gap_submission`.
Scope: source-only inspection of `resources::contract_dependency`, its unchanged
original implementation, HAVE and delivery accessors, native callers, and the
new `resource_dependency_tests.rs`. No compilation, tests or benchmarks run by
this reviewer. No blocking source findings within this scope.

The pair-presence predicate exactly matches the cases that can select a
contract's other endpoint as `partner` in the original loop. When no such
contract exists, `from_partner` remains its original positive zero for all
eleven non-oil commodities. The `<= 0.0` branch therefore skips before the
denominator and leaves `best` at positive zero. Unrelated contracts may compute
negative, NaN or infinite intermediate `all` values, but cannot write the
partner numerator or change that branch. The new return has the same bits.

Self-pairs need an actual self-contract to contribute: the original `k.to == id`
branch takes precedence, and that still reaches the unchanged original body.
Reversed and duplicate contracts match the conservative presence check; their
original iteration order, barter side, quantity sums and depth arithmetic are
unchanged. Zero depth, signed zero, NaN/infinite/negative quantities, expired
duration fields and dead nations do not create an additional shortcut for a
matched pair. Oil-only, monetary and empty matched contracts likewise retain
the full original path.

`have` borrows a built derived cache or constructs a local owned value. It does
not warm or mutate the world. For an absent pair, missing/stale/nonfinite cached
flow values cannot reach the dependency numerator or denominator; for a matched
pair the same accessor and entire computation remain in place. No RNG, ordering
or serialized state side effect is removed. This is equivalence for the
function's game-data computations, not a claim to preserve allocation failure
or arbitrary process-corruption behavior.

The synthetic oracle compares f64 bits for all ordered pairs across empty,
barter, self, reversed, duplicate, zero/negative/NaN/infinite quantity, NaN/zero
depth and dead-nation cases. It checks ordinary cold and warmed caches, not
deliberately malformed cached rows. That is a bounded coverage limitation, not
a discovered defect. The ignored actual-input oracle retains an original-path
selector, requires nonempty contracts and nonzero unrelated-pair skips, compares
31 complete native worlds and headlines, and verifies the source remains
unchanged. Source inspection is not evidence that either test was executed.

The optimization plausibly removes repeated work: an unrelated pair pays one
endpoint scan instead of eleven commodity scans, local HAVE construction when
uncached, and nested delivery/committed-out computations. `trade_dependency`
feeds sovereignty, commitment and theatre decisions. The amount saved in an
actual late campaign requires new measurements; this review makes no speedup,
qualification or session-closure claim.
