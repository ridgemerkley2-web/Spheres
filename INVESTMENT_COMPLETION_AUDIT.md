# Buildable investment completion audit — 0.6

All thirteen exposed project kinds have a consumer. This is a code/contract audit,
not a claim that any project is profitable in every country. The production quote
shows the actual effect, funding, prerequisites and required inputs. Completed
capacity can remain idle when funding, control, inputs, storage or demand bind.

| Investment | Consumer after completion | Player-visible payoff / constraint |
| --- | --- | --- |
| Infrastructure | Production throughput | 10% faster province construction per level. |
| Industrial Estates | National construction capacity | 0.15 additional work capacity per level; no free manufacturing inventory. |
| Power Grid | Industrial power delivery | Up to five modeled local power units/day per level; requires generation. |
| Research Center | Funded research prototype operations | Real Science authority and intermediate/capital goods buy bounded eligible prototype credit; at most 20% of a discovery's bill, with existing year and prerequisite gates. |
| Arms Plant | Manufacturing lines | Equipped capacity for real recipes and procurement funding; placing an order is not receiving equipment. |
| Machinery Works | Civilian industrial operation | Intermediate packs, copper and power produce capital-goods packs. |
| Power Generation | Industrial electricity dispatch | Capacity burns fuel when used; empty generation is not fictional national GDP. |
| Materials Processing | Civilian industrial operation | Ore, coal and power produce intermediate packs. |
| Freight Terminal | Shared freight corridor capacity | 25% higher modeled gateway throughput per level for every eligible shipment. |
| Industrial Warehouses | Manufactured storage caps | Additional 250 capacity for each good per level; full stores pause producers. |
| Factory Automation | Real civilian line throughput | 20% additional throughput per level with matching input, power and cash use; technology prerequisite. |
| Energy Efficiency | Real plant electricity and fuel use | 10% reduction per level capped at 50%; technology prerequisite. |
| Starter Industry | Paid scalable industrial modules | Fractional processing, power, grid and construction support sized to a small economy; no starting stock grant. |

The upstream economic integration at 4b31168 already supplied the missing
Research Center consumer and scaled modules. This release preserves those
mechanisms, improves the two vague catalog descriptions, and connects guidance
to live projects and their realized results. It does not add a second research
multiplier or double-count GDP for enabling capacity. The `research_centers`,
`industry`, `manufacturing`, `production`, `small_country_modules`, accounting and
logistics regression suites exercise these consumers in the full verification.

The earlier PROVINCE_ECONOMY.md statement that laboratories were reserved data
is historical. Current laboratory activity, source profiles and occupation rules
are described in CURRENT_ARCHITECTURE.md and the domain amendments.
