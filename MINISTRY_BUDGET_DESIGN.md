# Ministry programs and investment budgets

Status: approved for local implementation by Ridge, 2026-09-03.
The first playable implementation is local and awaiting review before any Git
commit or push. Existing campaigns enroll only when a department plan is enacted.

## Implemented review slice

- Ten ministry envelopes, five department rows each. Industry, Infrastructure,
  Science and Defense expose allocation controls; the other six retain visible,
  automatically managed service splits, not fifty invented service simulations.
- `SetProgramBudget` validates all ten parents and fifty integer basis-point
  shares atomically. A read-only server preview quotes the same political price.
- Opening-GDP daily authorization is shared by every consumer. Unused capital
  is not charged; actual services, work and interest settle once after all systems.
  Funds carry within the fiscal year, not past it; existing work keeps its progress.
- Infrastructure's four capital departments jointly fund existing network works.
  Maintenance retains its existing extraction-service arm. Science protects
  operating research and funds laboratory construction separately.
- Industry offers estates, machinery, generation, grids, mines, processing,
  freight terminals, warehouses, automation and energy-efficiency upgrades.
  The physical chain is raw resources → powered processing → machinery → usable
  construction packs. Storage, grid, generation and upgrade limits have consumers.
- Defense procurement and arms plants share one pool; operating force support
  and ammunition maintenance use their own service allocations.
- Capital work—not unused authorization or civilian running costs—feeds the
  existing macro investment channel, with a one-day settlement lag. No project
  writes a flat GDP bonus. Old saves and unenrolled simulation runs stay inert.

### Verification for this local review

- 102 UI checks pass, including allocation conservation, asynchronous preview
  invalidation/failure recovery, precise slider values and safe rendered labels.
- Web contract suite: 115 passed, 2 ignored. Department parsing, pure preview,
  fifty rows, ten investments, actual political price and embedded assets covered.
- Full simulation run: 354 passed, 52 ignored; only the three pre-existing
  baseline failures (1990 endowment and two golden comparisons). Actual start
  and long-run hashes stayed `0xe26e4bf8d6c60066` / `0xbe94d6125631829c`.
  Final targeted rechecks: 16 program and 15 manufacturing tests passed.
- Desktop 1280px and narrow 414px reviewed in an isolated localhost campaign.
  Narrow board has no horizontal overflow; browser error/warning log was empty.
  First enactment advanced exactly one day. A bauxite-blocked estate used no
  materials or work money; a separate power project advanced and its $208,333
  work charge exactly matched Industry's actual spend. Malformed budget input
  returned HTTP 400 without moving the date. No user save file was overwritten.

Deliberate boundaries: annual amounts remain GDP-share run-rates, not a fixed
nominal-dollar appropriation. New industrial packs/power are modeled game units,
not sourced 1990 factory or energy data; goods are usable inventory, not a new
profit, household-demand or manufactured-export simulation. General service
department mechanics, separate roads/rail/port/airport project types, private
firms, and balancing across small and large countries remain future slices.

## Direction

Keep ten ministries. Give each five departments with their own sub-budget.
Industry & Energy has two investment choices within each department: ten
choices total, not ten extra pots of money. Keep the annual plan and daily
simulation. Department defaults and automatic management keep this playable;
the player need not operate fifty dials every day.

The current Industry allocation already feeds national investment and funds
civilian-industry and power-grid construction. Its additional named ministry
effect is magazine refill. The replacement makes civilian development visible
and moves responsibility for ammunition supply to Defense. Transfer that
effect; do not duplicate it in both ministries.

## Department catalogue

These are proposed gameplay departments, not claims about real 1990 budget
breakdowns. Any initial split without national sourcing is explicitly a game
preset. Amounts always reconcile to the existing national budget.

| Ministry | Five departments |
| --- | --- |
| Health | Primary care; Hospitals; Medicines & supplies; Prevention; Emergency medicine |
| Education | Primary schools; Secondary schools; Vocational training; Universities; Teachers & facilities |
| Housing | Public housing; Home renovation; Housing assistance; Water & sanitation; Urban development |
| Pensions | Retirement benefits; Disability benefits; Survivor benefits; Minimum-income supplements; Benefits administration |
| Infrastructure | Roads & bridges; Railways; Ports; Airports; Network maintenance |
| Industry & Energy | Factories & construction; Energy supply; Minerals & processing; Industrial supply chains; Industrial modernization |
| Science | Basic research; Computing & communications; Materials & energy research; Life sciences; Aerospace research |
| Defense | Personnel & training; Operations; Maintenance & supply; Equipment procurement; Military research |
| Security | Policing; Courts & corrections; Border security; Civil protection; Domestic intelligence |
| Diplomacy | Embassies; Trade diplomacy; Foreign aid; International institutions; Negotiations & mediation |

Each department shows annual allocation, share of its parent, funding available
today, committed work, actual spending to date, and its delivered service or
capacity. Operating services and capital projects are distinct types of claim
within that department, not unbounded extra budgets.

Existing service departments may start automatically managed with visible
prebaked allocations. Do not expose a supposedly meaningful independent slider
until its effect has a real simulation consumer. In the target design every
department is adjustable; phased implementation must identify any departments
still managed together and must not invent five effects from one existing arm.

## Industry & Energy: ten investment choices

| Department | Investment | Intended result | Current integration status |
| --- | --- | --- | --- |
| Factories & construction | Industrial estates | More national construction capacity and sites for later civilian production | Reuse civilian-industry construction capacity; civilian production sites need a consumer |
| Factories & construction | Machinery works | Capital goods for construction and industrial production | New civilian production/recipe consumer required; not another flat GDP bonus |
| Energy supply | Generation projects | Usable power for new industrial projects and lines | New incremental energy-capacity consumer; fuel and technology prerequisites |
| Energy supply | Grid upgrades | Deliver available power to industrial sites | Existing power-grid levels need an actual delivery consumer |
| Minerals & processing | Mine development | More production of a located map resource | Existing deposit, mine and stockpile system; migrate financing carefully |
| Minerals & processing | Processing plants | Turn existing raw inputs into usable industrial intermediates | New recipes/industrial inventory, separate from mapped mineral deposits |
| Industrial supply chains | Freight terminals | Remove a real handling or route bottleneck | Existing freight routes; new terminal-upgrade consumer |
| Industrial supply chains | Strategic warehouses | More usable storage and reserve capacity | Existing physical stocks/caps; new storage-capacity consumer |
| Industrial modernization | Automation retrofits | More output from supported production lines | Requires researched technology and a real line; replace its throughput calculation |
| Industrial modernization | Efficiency retrofits | Lower fuel/power input per unit of industrial output | Requires an operating energy/production consumer; not free resources |

No option sells a cosmetic capability level as an economic payoff. A choice
without its consumer is clearly unavailable until that consumer is built.
Technology, province ownership, located deposits, routes, materials and
existing facility requirements determine eligibility.

Preserve the resource ruling: resource shortages block the actions/lines that
need them, not an unrelated ambient GDP penalty. New power requirements first
apply to the new modeled industrial activities. Do not invent nationwide
1990 generating stations or overwrite the existing national oil model.

## The daily funding loop

1. The annual ministry envelope is split among five departments. Child shares
   sum exactly to 100% of the parent. A new department budget reallocates money;
   it does not create an additional national appropriation.
2. The server derives the day's funding authority once using the existing
   calendar fraction and an agreed GDP snapshot. Keep the current top-level
   GDP-share convention initially: displayed annual dollar amounts are a
   run-rate, not a secretly fixed cash appropriation. Fixed cash annual caps
   would be a separate explicit design change.
3. Operating commitments and already-approved projects compete inside their
   own department. The player selects normal/high/low project priority and
   can protect an operating-service allocation. Priority redistributes money;
   it never manufactures money, workforce or materials.
4. A project requests funding for feasible work. Its progress is limited by
   funding, construction capacity, remaining work and the full resource bundle.
   Reserve and settle those inputs atomically. Blocked work spends neither
   its construction funding nor its materials.
5. Unused capital authorization can carry within the fiscal year. It is an
   authorization, not a new currency. At rollover uncommitted authority expires;
   continuing projects keep built progress but need a renewed funding plan.
   Enactment, cancellation and repeated previews cannot mint cash or reset spend.
6. The preview and settlement read the same allocation result, including
   available funding, shortages, daily progress and completion estimates.

Illustration only: a department has $12m available for work today. Two projects
request $8m and $6m of feasible work. Their combined authorization cannot exceed
$12m. Funding one at $8m leaves at most $4m for the other. If the first lacks
materials, the plan can reassign its unused allowance to eligible work using
the same deterministic priority rules. Starting a third project does not give
all three another full $12m.

## Cash and growth: implementation boundaries

Current construction reads the full parent ministry allocation as a funding
speed signal for every project. It shares construction capacity, not dollars.
The existing fiscal system already charges the full ministry envelope. Adding
a project cash debit on top would charge that spending twice.

Recommended target: distinguish appropriation from actual expenditure.
Operating services post their cost daily; capital projects post only work
actually funded. Unused authorization leaves cash unspent. The fiscal ledger
is the single owner of treasury/debt settlement. Achieving this requires
replacing the current blanket ministry debit for enrolled program budgets,
not bolting another debit onto production. Legacy plans retain their existing
behavior until explicitly migrated/enacted.

A transitional prepaid-authority implementation is possible, but its UI must
say the money has already been appropriated and paid. It must not advertise
idle project money as treasury savings or refund unused authorization as cash.
Do not mix this transition model with actual-expenditure accounting.

Construction needs explicit modeled monetary costs: today's funding_required
is a GDP-share speed reference, not a project price. Price work separately from
physical inputs already bought into stock. Imports currently have their own
cash settlement; show their cost separately until an all-in procurement budget
actually routes that settlement into a department. Never pay twice for an input.

New program-funded mines must replace their existing upfront investment charge.
Existing mines stay prepaid. Equipment orders and the arsenal's banked money
also need explicit migration: Defense procurement, building arms plants, and
operating supply cannot each reuse the entire Defense envelope.

Industry, Infrastructure and Science already enter the macro investment
aggregate. Concrete project effects must replace the corresponding abstract
capital contribution, not add a second GDP reward. Implement one reviewed
capital/realized-investment bridge before advertising factory returns. Keep
operating-cost effects, capital accumulation, resource inputs and tax receipts
separate. No flat GDP bonus for clicking an investment card.

## Arcade presentation

- Main cabinet: ten illustrated ministry cards, each with its budget, delivery
  summary and at most one important warning.
- Open a ministry: five large department cards, allocation bars and readable
  service/project outputs. Annual allocation and today's spending are labeled
  separately.
- Industry: two illustrated investment cards per department. A card shows
  what it builds, where it can go, cash/material needs, operating requirements,
  expected time and the precise payoff. Select it to choose a province.
- Presets: Balanced, Development, Services, Emergency. Presets are visible
  drafts; they do not enact or move time by themselves. They are not promises
  of an optimal strategy.
- Auto-manage can maintain a department's allocation and project priorities
  within its envelope. It cannot raise taxes, borrow beyond the fiscal rules,
  change another ministry or initiate foreign deals without player authority.
- Default view emphasizes delivery: "2 projects building; 1 needs copper"
  instead of exposing fifty small numeric controls at once.

## Build sequence and acceptance

1. Program schema, atomic annual command, five-part defaults, reconciliation,
   one daily allocation plan and accurate cabinet previews. Record the ruling
   in BIBLE/SPEC when adopting the implementation contract.
2. Industry vertical slice: real project finance for existing civilian industry,
   grids and mines; move ammunition funding to Defense with a named mapping.
   Build the energy/civilian consumers needed for meaningful new investments.
3. Complete the ten investment choices and the single macroeconomic bridge.
   A card is playable only when its outcome exists and passes its acceptance
   tests. Then deepen the remaining ministries' departmental effects.
4. Verify across nations and dated daily schedules before enabling automated
   AI program budgeting. Preserve old saves and default headless replay.

Required checks: child totals equal parent; spending never exceeds authority;
one accrual/settlement per date; no duplicated funding across projects; atomic
resource/cash/work settlement; no double import/mine/procurement charge; no
duplicate GDP channel; deterministic priorities; faithful preview; 28/29/30/31
day calendars and year rollover; cancel/re-enact conservation; save/load
continuity; legacy no-op behavior. Existing calibration failures are reported,
not repinned or hidden.

## Code anchors inspected

- world.rs: AnnualBudget and investment_total.
- production.rs: ProjectSpec, catalog, funding_ratio, rate_for and tick_day.
- economy.rs: Fiscal::of, charge and fiscal settlement.
- ministries.rs: named effects shared by simulation and UI.
- resources.rs: start_mine, stockpile consumption and trade settlements.
- arsenal.rs / manufacturing.rs: banked procurement and envelope allocation.
- clock.rs / lib.rs: calendar fractions, command validation and daily tick order.
