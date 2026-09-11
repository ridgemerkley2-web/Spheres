# Spheres game review — 10 September 2026

## Overall assessment

Spheres is a substantial playable grand-strategy foundation, with a stronger
equipment economy than its current military command experience. Construction,
public finance, procurement, government decisions, history and recovery have
real simulation behind them. The recent aircraft workshop has become a useful
visual design tool, but its new mission interface is still a demonstration.

My assessment is **feature-rich alpha**. The main obstacle to a coherent game is
now integration and player comprehension. More individual systems or more mesh
detail will have less value than connecting the existing work into a few
understandable, rewarding campaign loops.

This review uses source inspection, current data inventories, repository history
and automated verification. It is not a fresh human playthrough or a new
1990–2035 balance study. Prior browser acceptance and the recent aircraft visual
inspection are identified separately from current test results.

## Which game this review covers

The active source is `codex/resume-spheres`, based on `b6f1e8f` plus the aircraft
work prepared for this push. After fetching on 10 September, the branch and
`origin/master` had **10 branch-only / 26 master-only commits** before this push.
Its own remote branch matched the local committed base exactly.

The active branch contains the domestic company procurement/refit lifecycle,
Government redesign, historical character work, guidance and detailed vehicle
art. Master at `485c223` separately contains the population and industry rebuilds,
country-scaled companies, operational warfare and fiscal recovery work. Those
commits are visible in GitHub history; their existence does not mean they are
integrated into this playset. Master also carries later terrain/coast work.

**First priority: create one tested integrated playset.** Merge ownership,
accounting, save formats and command flows deliberately. In particular, reconcile
the different company and warfare implementations before building aviation on
top. Pushing this task branch preserves the work; it does not update master or
replace a running game executable.

## Systems review

| Area | Current strengths | Main limitation in this playset |
| --- | --- | --- |
| Campaign foundation | Daily calendar, pause/speeds, 137 starting nations, successor identities, named saves, backups, retained history and recoverable command receipts. | Technical correctness is better evidenced than independent new-player comprehension and long-run balance. |
| Economy and construction | Financial construction budget, incremental funded progress, priorities, pause/cancel, suggestions and province/national effect previews. Small industrial modules are implemented. | Local jobs/prosperity and the reason for every treasury movement still need a clearer player-facing explanation. |
| Industry and trade | Operating plants consume real inputs, funding, power and storage; output and transport are accounted for. Province output reconciles with the nation. | The player can still cross several screens to solve one shortage. Some advanced economic systems require Economic Competition to be enabled. |
| Companies | Design → paid development → certification → company-owned stock → purchase → delivery → service is real. Ammunition supply and manufacturer refits share the same physical and financial rules. | One domestic state contractor is a narrow first implementation. Competing firms, foreign purchases, historical company identity and company AI are not completed here. |
| Research and equipment | Eight general research domains and component research; four tank classes, five ground specialists and two tactical aircraft have real configurable specifications. | The payoff from a component change needs to remain legible in actual military outcomes, beyond ratings and the model viewer. |
| Ground military | Shared national force limits, access, ammunition and equipment losses are conserved; specialist equipment has bounded combat roles. | Master’s newer operational warfare is not merged here. This branch retains the older aggregate command model. |
| Aviation | Tactical aircraft can be researched, commissioned, bought, maintained and supplied for the existing raid consumer. The new aircraft bench supports detailed meshes, picking, three LODs and GLB exports. | New squadrons, physical bases, flight range, fighters and the new command-screen mission loop remain unimplemented in campaigns. |
| Government and diplomacy | Illustrated Government room, party/institution views, exact immediate decision previews, consent-based diplomatic offers, sanctions and standing policies. | Much historical party coverage and richer political event feedback remain unfinished. Existing previews do not imply forecasts of all long-term consequences. |
| Diplomacy and campaign aims | Human consent, dated offers, standing policies and optional peaceful or domination goals use real state and preserve sandbox continuation. | Long campaigns still need measured pacing and fresh-player evidence; master’s negotiated operational peace is not integrated here. |
| Characters | Correct separation of people, party offices, executive eligibility, portrait eras and fictional successors; fixed cartoon style is implemented. | Worldwide coverage is far from finished: 53 historical cartoons and four fictional cartoons exist. |
| Map and art | Displaced terrain, borders, province selection, sourced city positions, symbolic skylines and extensive room/vehicle artwork. | Cities are symbolic rather than surveyed streets; the global art-budget audit is out of date with the mesh layout. |
| Tutorial/advisor | Optional guidance is connected to campaign state and navigation, with nine reading lessons and advisory presentation. | It needs stronger proof that a new player can complete an entire useful outcome without outside coaching. |

### Economy: the requested direction is working

Construction itself no longer asks the player to acquire building materials.
Installation prices include them; the construction budget pays for work as it
progresses. Actual operating factories still require inputs and support. That is
a meaningful distinction: funding builds a facility, while a working economy
keeps it productive.

Previews and suggestions are worthwhile additions because they expose national
and provincial capabilities before spending. Suggestions account for capacity
already built or queued. Completed capacity does not automatically invent output
or return on investment.

The remaining presentation problem is following consequences. The finance page
distinguishes cash, debt, annual authority and dated spending, but it does not
fully reconcile trade, transfers and every other direct treasury movement in
one place. Facility jobs, profit and tax-return forecasts are not implemented.
The population work on master must be reconciled before promising those effects.

Source anchors: [industry.rs](../spheres-sim/src/industry.rs),
[construction_preview.rs](../spheres-sim/src/construction_preview.rs),
[construction_suggestions.rs](../spheres-sim/src/construction_suggestions.rs),
[gdp_projects.rs](../spheres-sim/src/gdp_projects.rs), [CASH_FLOW.md](../CASH_FLOW.md).

### Companies and equipment: the strongest distinctive loop

The country commissions a design, pays real development work, waits for
certification, then purchases completed stock belonging to the company. Purchase
payment, seven accessible delivery days, national inventory and maintenance are
separate states. Prototypes and unsold stock do not become free military power.
Refits reserve existing vehicles and use held funding; returning conversions
preserve exact model identity and age. This supports the requested manufacturer
relationship.

The weakness is choice. The domestic contractor shares one plant work packet
and has limited parallel development. That gives correct accounting but can feel
like another state production queue. A few differentiated domestic and foreign
suppliers, with clear specialties and reviewed imports, would make the company
layer strategically meaningful. Do this after integrating the company work
already present on master, not by creating a third supplier model.

Source anchors: [companies.rs](../spheres-sim/src/companies.rs),
[companies_refits.rs](../spheres-sim/src/companies_refits.rs),
[COMPANIES.md](../COMPANIES.md), [equipment.rs](../spheres-sim/src/equipment.rs).

### Military and flight: presentation is ahead of operations

The latest original tactical aircraft has **228,640 inspection triangles**,
14,904 at catalogue detail and 1,696 on the map. Its two-seat cockpit, transparent
canopy, intake fans and deep exhausts are actual geometry. Light attack has
197,632 inspection triangles. Components remain selectable and exportable.

These improvements do not create new simulation roles. Campaign aviation still
uses two tactical platforms with supported aircraft, compatible stores and coarse
theatre access for the existing raid behavior. The Air Force workshop’s eight
ready aircraft, base, assignment and reports are fictional demonstration state.
It sends no mission order to a campaign.

The best next military milestone is one complete arcade loop: buy aircraft,
assign them to a squadron at a base, see a single readiness explanation, select
one mission, review its cost/risk, and receive a dated result with losses and
remaining readiness. Routine support should stay automatic within the authorized
budget. Build on the integrated operations model and existing ammunition and
procurement, then add fighters. More specialist aircraft should follow that loop.

Source anchors: [AVIATION.md](../AVIATION.md),
[equipment_aviation_view.rs](../spheres-web/src/equipment_aviation_view.rs),
[flight-command.js](../tools/arsenal/flight-command.js),
[FLIGHT_LAYER_BUILDOUT.md](art/FLIGHT_LAYER_BUILDOUT.md).

### Government and characters: sound identity rules, substantial unfinished content

Government has real decision routing and previews on a disposable copy of the
world. The character system preserves saved incumbents, distinguishes political
offices and presents future candidates as fictional. Those are valuable
foundations for alternate history.

The current inventory contains **590 known real people, 393 sourced party-office
records, 624 represented game party rows and 57 finished cartoon assets**. Only
53 party rows have partial research; none has complete verified historical
coverage. There are **705 known historical artwork jobs still pending**, plus
unresearched countries/organizations and future art. The 2,556 future templates
are planning records, not completed characters. The represented game parties are
not an exhaustive census of every historical minor party.

The frozen historical research cutoff is 7 September 2026; fictional candidate
windows begin afterward through 2035. That cutoff does not update automatically
with the current date. Finish complete country casts and dated successions in
batches, reporting coverage openly rather than treating placeholders as finished.

Source anchors: [government_view.rs](../spheres-web/src/government_view.rs),
[LEADERSHIP_PRODUCTION_2035.md](LEADERSHIP_PRODUCTION_2035.md),
[leadership_production_2035.json](../spheres-web/data/leadership_production_2035.json).

### Usability, map and visual quality

The illustrated rooms, clearer menus, native controls, component picking and
short reviews help. Guidance is connected to the game, whereas the standalone
guidance and leadership review pages are demonstration fixtures. The nine tutorial lessons currently record reading progress, not successful
gameplay. The next usability test should follow outcomes: fund a budget, finish a useful project,
operate it, buy and field a model, understand a problem, then save and resume.

The map already has real displaced peaks/valleys and 1,249 sourced city points.
Its native elevation spacing is 60 arc seconds, approximately 1.85 km at the
equator, with 3× vertical exaggeration. Zooming to 192× magnifies that dataset;
it does not reveal street-level geography. More visible links from a province’s
actual facilities, damage and activity to its city presentation would have more
gameplay value than more decorative geometry alone.

Visual consistency remains a worthwhile production task: cartoon characters,
illustrated backgrounds and increasingly detailed equipment need common framing,
lighting, spacing and information hierarchy. Triangle counts establish geometry
size, not realism or frame rate. Test cold loading, repeated room visits and
mobile/lower-end GPUs as well as the close-up inspection view.

Source anchors: [GUIDANCE.md](GUIDANCE.md), [ADVISOR_MODEL.md](ADVISOR_MODEL.md),
[PLAYTEST.md](../PLAYTEST.md), [README.md](../README.md),
[terrain-surface.js](../spheres-web/ui/terrain-surface.js).

## Recommended development order

1. **Integrate the active branches and choose one release build.** Preserve
   procurement, refits, character identity and old saves; reconcile population,
   construction, fiscal recovery and operational warfare. Verify both Windows
   and Linux on the exact integration commit.
2. **Finish the first-hour campaign experience.** One dated national briefing,
   three useful next actions, a complete cash-change explanation and tutorial
   completion based on real outcomes. Test with fresh players in small and large
   countries.
3. **Connect the arcade air-force loop.** Real fleet, squadrons, bases, readiness,
   mission review and results, using existing ownership and supply. Start with
   tactical aircraft and fighters.
4. **Make suppliers and opponents behave like participants.** Integrate company
   diversity, then offer sensible supplier choices/imports and bounded AI
   research, purchases, support and replacement policies.
5. **Continue content in complete batches and measure performance.** Country
   casts, dated succession, distinct aircraft families and coherent art styling.
   Repair the art audit and tune detail to measured costs without silently
   loosening its limits.
6. **Validate sustained play.** Fresh-user sessions plus multi-country,
   multi-seed campaigns should demonstrate understandable choices, recoverable
   setbacks and viable small-country paths. Passing unit tests alone does not
   establish an enjoyable or historically balanced 45-year campaign.

## Verification and open technical work

- The unchanged runtime source from the completed aircraft pass has **1,372 UI
  tests passing, zero failures**. Its twelve generated GLBs pass exact regeneration
  checks. Recent in-app inspection covered cockpit, intakes, exhausts and component
  changes with no browser errors.
- Fresh `cargo test --locked --release --workspace --no-fail-fast`: **1,271
  passed, zero failures, 71 ignored**. Ignored calibration/profile checks are not
  counted as passes. No user campaign was advanced, loaded over or restarted.
- `node tools/ui/bench_art.cjs --check` currently fails immediately because it
  assumes only positions, normals and colors; ground meshes now also carry
  `materialClasses`. Its old upload-memory arithmetic cannot be trusted. This is
  a separate unresolved audit problem; the historical “30 over budget” report is
  not a current measurement. Dedicated aircraft geometry/LOD checks pass.
- Some documents describe superseded behavior: for example older economic
  competition notes still require construction materials, while current code
  uses financial construction. Use dated implementation records and source, and
  consolidate the current player guide after integration.
- Remote CI success must be read from the new GitHub run. Local tests, earlier
  branch acceptance and a configured workflow do not establish that result.
