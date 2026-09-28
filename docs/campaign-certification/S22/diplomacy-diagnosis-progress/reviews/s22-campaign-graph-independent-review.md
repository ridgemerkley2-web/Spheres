# S22 campaign graph handoff: independent source review

Implementation: `7e4edb326b51ae12a4ba5f560d8d39036059f87a`, based on integration `9823b070efea06a4f6e63e955d8ae23445108054`.

Author: Codex subagent `/root/s20_preflight`.
Independent reviewer: Codex subagent `/root/review_source05`.

The reviewer examined the production diff in `spheres-sim/src/campaign.rs` and `spheres-sim/src/campaign_supply.rs` before commit and reported no findings. The author then added the explicit all-fields Opening/same-date test without changing that reviewed production boundary. This record transcribes the review response; it is not a build, executed test, benchmark, or certification claim.

The review confirmed:

- `Graph::new` reads static geometry, production and terminal contractor modifiers, occupation/control and opening commercial usage.
- Between deployment's owned graph handoff and `prepare_with_graph`, only the detached campaign book is installed and supply requests are read. `control::controller` depends on conflicts and sovereignty, not detached campaign sectors. Supply-mission and garrison-request reads are pure.
- Same-day and empty-state guards keep their order. `remaining_capacity` still subtracts current military reservations; complete route results are not reused.
- The source includes exact graph-field/float-bit comparisons and an actual-checkpoint old fresh-graph oracle requiring nonzero handoff use.

The independent reviewer did not compile or run tests. The author performed Rust parser/format output checks and `git diff --check`, with no native build or test execution. Root owns coordinated validation and performance measurements.

A following test-only adaptation accepts either the complete campaign envelope or the integrated simulation save. It parses JSON once and takes the owned `world` value only when `format` is `spheres-campaign`; otherwise it retains the complete integrated simulation envelope, which itself also has a `world` field. It drops the remaining outer value and calls `crate::load_value` without reserialization. The reviewer separately identified the need for this format discriminator. This adaptation is outside the completed production review and makes no runtime behavior change.
