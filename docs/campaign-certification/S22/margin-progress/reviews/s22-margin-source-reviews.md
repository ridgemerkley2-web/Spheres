# S22 ammunition, arrival and population source reviews

Reviewer: Codex subagent `/root/review_gap_submission`.
Review date: 28 September 2026 UTC.

This record concerns source inspection only. The reviewer ran no builds, tests,
browser workloads, native benchmarks or campaign replays. Test descriptions below
describe the submitted assertions; they do not claim execution or passing results.
No performance improvement, S22 qualification, G5 or CP1 completion is awarded.

## Ground ammunition catalogue

Reviewed final commit `401464332b19752c1330f88ad35817aa18cf61a5`, authored by
the parent Codex agent. Files: `companies_imports.rs`, `military_ai.rs`,
`military_ai_tests.rs`, `military_empty_import_tests.rs` and
`military_ground_offer_tests.rs` under `spheres-sim/src/`.

Outcome: no actionable source finding. Filtering occurs after the original
`import_source` resolution, preserving first-match behavior for duplicate company
and product IDs, missing revisions and certification. The predicate removes
only non-ammunition, empty-stock or unrequired-family rows. The original ground
loop discards these before changing its reason, creating a purchase quote or
mutating the world. Matching refused offers remain. Public catalogue wrappers
still include every original row, and quote arithmetic is unchanged.

The original stable `total_cmp` ordering and tie-breakers retain the relative
order of surviving rows, including nonfinite prices and duplicate identities.
Live reserve/funding reads, command quotes/tokens and the first purchase return
remain in the original loop. The submitted tests compare complete offer bytes
and all three floating-point offer fields by bits, paid purchases and retries,
and 31 complete native days against the public-catalogue path. A final test adds
multiple required and unrequired families; the initial nonblocking coverage
limitation was therefore addressed. Synthetic duplicate first rows retain a
known family and vary stock rather than inventing an unknown ammunition family.

## Arrival payload ownership

Reviewed production commit `654ba15abf1f2b0cf557318bfe89ccb07eaebd24` and
test-only follow-up `c20dd87438b494899b26e2cdd90abda72ca86e6b`, authored by Codex
subagent `/root/review_source05`. Integration records at review time are
`56dd8387` and `802134d2`. Files: `logistics.rs`,
`resource_replenishment.rs` and `logistics_arrival_payload_tests.rs`.

Outcome: no actionable source finding. The factored posting function preserves
the disabled/already-posted guards, date stamps, clearing, route decisions,
sorts and ledger replacement. Its true value means a fresh pass completed,
including an empty fresh result; false never exposes a retained old arrival
ledger to the credit callback. All posting finishes before callbacks begin.

The private caller receives only copied buyer, commodity and quantity scalars.
Its market callback cannot mutate the borrowed world or affect subsequent route
decisions. Public `begin_month` still returns an independently owned full Cargo
vector; moving the completed vector into the ledger before cloning that public
result preserves order and exact values. The production resource caller avoids
that deep copy while retaining the same credit order.

Submitted assertions cover daily and legacy guards, empty/held/disabled/dead
states, owned public results, normal save/load retry, real raw purchases and
31-day complete-world/headline parity with nonzero removed payload counts.
The follow-up limits reload assertions to the normal delivered fixture; all
seven synthetic guard cases still compare complete worlds and retry behavior.
Deliberately malformed synthetic refusal states are not represented as valid
campaign saves. Payload-content counters are not allocator, memory or timing
measurements.

## Population course merging

Reviewed commit `616c90d8066587c4724861dba636969352f11cb6`, authored by Codex
subagent `/root/s20_preflight`. The author confirmed that committed content is
unchanged from the inspected working diff. Files: `population.rs` and
`population_course_merge_tests.rs`.

Outcome: no actionable source finding. The key exactly matches the original
five equality conditions: from/to skill, start day, and the raw bits of required
and funded days. Indexing only incoming keys does not normalize signed zero or
NaN payloads. Scanning the destination records only its first existing matching
row. A newly appended row immediately becomes the first match for later incoming
duplicates; no course is sorted, dropped or coalesced beyond the original rule.

Every incoming row and every original `people_m +=` execute in the original
order. The rest of the population and household arithmetic remains literal.
The map exists only during one merge, so subsequent funding changes are freshly
observed. Tests include duplicate destination rows, duplicate appends, rounding-
sensitive addition order, signed zero, distinct NaN payloads, empty inputs,
call-local identity changes, complete migration/world/headline comparisons and
an actual 31-day old-function oracle requiring avoided repeated searches.

The separate author proposal records snapshot workload counts and explicitly
bounded upper estimates. Those estimates are not exact next-day calls, measured
savings, or an independently replayed campaign. Execution and performance
qualification remain pending parent-coordinated runs.
