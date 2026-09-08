# Population, opportunity and work

The population rebuild gives peaceful development a readable arcade loop:
**build useful capacity → create jobs → train the people those jobs need → raise
household living standards**. A shortage should suggest a decision the player
can make. It should not require managing thousands of individual households.

The main screen presents jobs, living standards, education and opportunity.
Four national priorities choose the direction of training: Balanced opportunity,
Trade schools, University drive and Back to work. The education ministry pays
for teaching and training capacity; the priority distributes that capacity.
The player does not set each province's enrollment or each employer's wages.
A focus change costs 8 political capital and settles for 180 days before it can
be changed again. The command uses the same price and refusal as the preview.

## Shared population ledger

`spheres-sim/src/population.rs` stores compact province populations, with an
explicit national remainder for people who have no mapped district. Children,
working-age adults and retirees close to total population. Working-age adults
have mutually exclusive qualification groups: foundations, basic, secondary,
vocational and tertiary. Courses refer to those adults; enrollment must never
add another copy of a person to the population.

The same records produce the national and province views. District ownership
selects which nation receives a province's people and capabilities. A takeover
keeps the province's age groups, qualifications, courses and household history.
It does not reseed them from the new owner. Explicit migration and daily ticks
are the only population-model writers; opening a panel is read-only.

New browser campaigns enable the rebuilt system. Older campaigns retain their
existing state until the explicit migration action enables it. The enable
operation is idempotent, freezes its initialization and preserves the RNG.
Legacy monthly replay keeps the existing path. Daily save/load must preserve
all course progress, baselines, priorities and dated settlement markers.

## Jobs and qualifications

The inherited economy supplies jobs across the eight existing economic
sectors. Sector output and explicit game labor-productivity weights distribute
the opening jobs; income is not used to invent historical education. Completed
facilities and active construction create additional demand. General, technical
and professional positions have different qualification requirements.

Nine operating facility kinds share the industry's hiring recipes: civilian
industry, arms plants, shipyards, processing plants, starter industry, machinery
works, advanced industry, offices and research centers. Generation, power grids,
freight, warehouses and infrastructure remain passive capacity upgrades without
additional persistent hiring in this arcade model; their active construction
still needs crews. Mine construction competes for those same crews.

Matching conserves the labor force: a worker can fill one full-time position.
Qualified people can take lower-skilled work, so unemployment, vacancies and
underemployment may coexist. Students and the modeled military reservation
reduce the pool available to civilian employers. Sector wage indices report
the pressure from shortages while household class changes settle gradually.

The player sees these distinct measures:

| Measure | Denominator |
|---|---|
| Unemployment | Civilian jobseekers divided by the civilian labor force |
| Participation | Civilian labor force divided by working-age population |
| Employment | Employed civilians divided by working-age population |
| Underemployment | Employed workers working below their qualification |
| Vacancies | Positions still seeking suitable workers |

Bounded household relocation follows domestic job opportunities. A source
province can lose at most 0.5% of its population per year through this route;
destination vacancies and the available jobseeking pool further limit moves.
Qualifications and course progress travel with the households. This is an
internal relocation model; it does not invent international migration flows.

The least-filled required qualification limits a new facility's usable
production capacity. General workers cannot substitute for absent researchers.
The staffing bound is applied at the operating boundary once, alongside
electricity, inputs and operating funds. Physical nameplate capacity remains
the physical size of a plant. Construction takes the lower of assigned physical
capacity and available crew capacity before applying a contractor's work-rate
service; it does not multiply the same staffing shortage twice.
An assigned worker and a worker utilized in today's production receipt are
different views: a one-day input shortage must not manufacture a second pool
of workers or silently duplicate GDP.

## Education takes time

School coverage changes the preparation of future workers. Birth cohorts stay
out of the labor force until they age into it. Adult courses have explicit
calendar duration and funded progress: foundation training takes six months,
secondary catch-up two years, technical training eighteen months and university
study four years. Secondary catch-up connects basic schooling to the admission
requirements for trade courses and university, using the same four priorities.
A spending
increase expands usable places and supports course progress; it does not
immediately produce graduates or researchers.

Teacher availability constrains education capacity. The different priorities
shift the mix of training places within that constraint. Cutting funding can
slow progress; restoring funding resumes the existing course. Completion moves
people between qualification groups while conserving their number.

Research uses the resulting qualified workforce relative to its frozen opening
baseline. Research facilities still require their existing funded operations,
materials and power. New center professionals support their paid prototype
work; they do not also increase the ordinary inherited researcher multiplier.
The former direct education-spending research multiplier
belongs only to the legacy path; applying it alongside the new qualified-worker
route would pay for the same education twice.

## Classes and household outcomes

The six livelihood groups are agricultural households, routine wage workers,
skilled workers, professionals and managers, small proprietors and capital
owners. They describe household economic position, including dependents.
They are not additional workers and do not change the population total.

Qualification, job assignment, household class and wealth remain distinct.
Graduation alone does not guarantee a professional income; a suitable job
matters. Persistent changes in employment and livelihoods move the household
mix gradually. Each class has its own living-standard history: routine-worker
job losses can lower wage-worker security while a shortage of qualified staff
improves skilled workers' wage prospects. Wealth follows the class's resulting
income. These are game indices, not reported historical household balance
sheets.

The politics integration weights class-specific unemployment risk and living
conditions by the households they affect. This hardship measure reaches the
existing stability and political-capital mechanisms, so different class
outcomes have a consequence beyond the chart. Household class does not dictate
an ideology, party or scripted election result.

## Accounting boundaries

The population module never writes GDP, treasury or random state. Its results
reach the existing economy through defined interfaces:

| Existing system | Population result |
|---|---|
| Economic growth | Change in effective inherited employment replaces the previous raw population-growth labor proxy |
| Industry and materials | Suitable staff bound usable capacity once |
| Construction | Available construction staff bound work alongside assigned industrial capacity and paid inputs |
| Research | Qualified-worker availability replaces the direct education-spending research bonus |
| Politics and political capital | The counted unemployment rate and household hardship inform existing pressures |
| Population total | Cohorts, aging, births and deaths replace the previous scalar growth write when active |

New projects already record actual value added through the province economic
ledger. Their job creation does not receive a second generic growth award.
Ministry spending remains charged by the fiscal system; population training
does not charge the same allocation again. Pension effects change participation
instead of relabeling retirement as job creation.

## Historical inputs and explicit game assumptions

The source snapshot covers all 137 opening nation IDs. It includes age,
participation, unemployment and education observations, each with its actual
year and source. Missing observations remain null in the source artifact.
See [POPULATION_DATA.md](POPULATION_DATA.md) for measured coverage, denominators,
territorial reconstructions and offline reproduction.

Province distributions, job recipes, initial ownership/class allocation,
military manpower reservations, course speeds and wealth indices are explicit
game assumptions. Some countries also require modeled education or labor
starting values where no suitable observation exists. These estimates are
disclosed separately from the historical sources, and are not inferred from
GDP rankings. The source artifact contains no fabricated historical class
shares.

The practical test is a province with unemployed general workers and unfilled
technical positions: a factory creates useful demand, a trade-school priority
supplies the qualifications after its training delay, employment rises without
duplicating people, and household conditions respond. Integration validation
also checks ownership transfer, save/resume, policy cost and cooldown, multiple
country sizes, and the absence of immediate education bonuses.

## Release verification

The combined simulation, CLI, integration and web gates passed 1,229 Rust
tests. The final interface checks passed 394 tests, including the latest water
detail checks. Disposable browser campaigns exercised People, Companies and
Industry together at 1440, 820 and 390 pixels, including policy prices,
cooldowns, education navigation, construction presets and capacity controls.
The source snapshot also reproduces byte for byte with the offline check.

The ignored campaign diagnostics check all nations' age, qualification, class,
employment and population totals, ownership transitions and finite economic
values each year. Their readings are gameplay diagnostics, not historical
calibration claims. `second_seed_daily_population_census` provides a separate
entry point for the second ordinary browser campaign.
Both ordinary campaigns, seeds 7 and 1990, completed January 1990 through
January 2000 with every annual accounting and finite-value check passing.

Optional Economic Competition was checked separately for 32 days with and
without population, including its first monthly AI construction review and
population-accounting checks. On the validation machine the review created
about 100 projects and took roughly 8 seconds in both cases. Ordinary days
before projects were cheaper. Large AI planning reviews remain a performance
limitation; the ten-year AI-heavy stress run was interrupted and is not claimed
as a completed validation. These wall-clock observations do not enter the sim.
