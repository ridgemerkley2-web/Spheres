# S22 sovereignty candidate gate: independent source review

Reviewer: **Codex /root/review_gap_submission**. Review date: 28 September 2026 UTC.
Reviewed implementation: `3d52e40fcf962b868a9f4b3274d015f83dae0e68`, compared
against `9823b070efea06a4f6e63e955d8ae23445108054`.

Scope: `spheres-sim/src/sovereignty.rs`, the new candidate tests, and the called
world/pact/trade-dependency helpers. The reviewer performed read-only source
inspection. **No edits, builds, tests or benchmarks were executed for this
review.** This records an independent agent review, not human approval or S22
qualification.

No blocking semantic findings:

- The prefilter uses exactly the `pact_partners(partner).contains(patron)` gate
  already required by the public quote. When it is false, the original quote
  cannot be ready, including non-finite dependency cases. Public quote metrics
  and refusal reasons remain intact.
- Surviving nations retain their original iteration order and dependency
  `total_cmp`/nation-ID tie-breaking. The loop still reads each patron's live
  world after earlier proposals, and commands remain on `apply_command`.
- `pact_partners` checks either endpoint and uses membership, preserving
  duplicate and endpoint-reversed pacts. Self-pacts and unrelated, absent or
  dead-nation endpoint rows retain the original quote behavior.
- Skipped quote dependencies are pure reads. `resources::have` borrows an
  existing derived table or builds a local owned value; it does not mutate the
  world's cache. No skipped reader consumes RNG.
- The old-candidate test flag restores on unwind. The actual-checkpoint oracle
  compares full serialized worlds (including RNG) and returned/retained
  headlines for 31 complete native days, requires monthly review and actual
  prefilter use, and checks that the source is unchanged. Its source design is
  adequate for this narrow optimization, but source inspection is not an
  executed test result.

The serialized-world oracle excludes derived caches and web history. This is
an explicit scope limit: skipped reads cannot change those caches, and the
oracle does not claim web-workload timing or browser coverage.

The review suggested adding endpoint-reversed and self/unrelated pact fixtures:
the initial `reverse()` case reversed vector order only. Root subsequently
added that coverage in `292b2ac2`. That later fixture change is distinct from the
completed production review and does not gain a test pass from this record.
The later integrated-envelope fixture correction in `6f6b61d3` likewise requires
its own coordinated validation. Executed two-test and actual-2035 results at
`6de1d997` are archived separately with their original provenance.
