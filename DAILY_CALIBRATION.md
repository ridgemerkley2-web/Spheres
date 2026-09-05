# Repeatable daily campaign panel

`daily_calibration` is a descriptive instrument, not a historical-fit test or a
set of tuned game-difficulty assertions. It starts fresh simulations and never
reads or writes a live campaign. The calendar includes actual leap days.

```text
cargo run -p spheres-sim --release --example daily_calibration -- --years 30 --seeds 1990,7,42 --output work/daily-off.csv --economic-competition false
cargo run -p spheres-sim --release --example daily_calibration -- --years 30 --seeds 1990,7,42 --output work/daily-on.csv --economic-competition true
```

Both commands default to physical logistics enabled. `--physical-logistics false`
provides a cheaper abstract-route diagnostic; its results are explicitly labeled
and cannot be presented as evidence for physical freight. `--scenarios` accepts
a comma-separated subset of the names below. Zero years validates initialization
and output shape. Seeds must be distinct. The configured rules are saved per run.

| Scenario | Player / policy fixture |
| --- | --- |
| idle_human | USA; no policy commands; automatic central bank |
| balanced_budget | USA; renew departmental spending authority at 95% of current revenue net of interest, preserving the inherited ministry proportions subject to caps |
| fiscal_stress | Brazil; controlled opening political capital of 100 so the intended test policy can pass; tax rate 10%, department authority at 1.5 times the initial plan subject to ministry caps |
| commodity_dependence | Saudi Arabia; transcribed oil dependence and no policy commands |
| small_state | Tonga; transcribed small economy and no policy commands |
| command_economy | Soviet Union; original government tracked separately from the continuing successor world |
| war_shock | Kuwait; controlled opening Iraqi political capital of 100, then an ordinary priced declaration against Kuwait; no player commands afterward |
| import_dependent_industry | Japan; no policy commands; transcribed industrial economy and import exposure |
| enrolled_budget | USA; renew the unchanged inherited annual ministry allocations and default department shares through the ordinary budget command |
| investment_program | USA; the same inherited annual budget as enrolled_budget, then one annual project attempt from the existing deterministic investment recommendation; pay normal political and construction costs and wait for real inputs |
| supply_disruption | Japan; observe paid spot imports during January 1990, then on 1 February attempt one ordinary priced sanction against the largest supplier by cumulative import value, with nation-id ordering breaking ties; no political-capital grant or forced success |

The two political-capital overrides are explicitly controlled fixtures, not new
campaign starting grants or simulated historical facts. Policy goes through normal
commands and pays normal prices; later refusals are counted. Every other opening
country quantity remains inherited from the data. Economic Competition, when
selected, governs only eligible AI countries; the player stays responsible for its
own budget choices and receives the same resource and fiscal constraints.

The balanced plan uses the same cloned opening-debt fiscal quote as the browser,
including its first year. It reserves debt service before allocating the 95%
spending envelope. An earlier development run read closed-book interest as zero;
that run is not the final balanced-budget baseline.

Annual CSV rows include GDP level/multiple/CAGR, population and GDP per person,
inflation, debt ratio, interest burden, cash/debt stocks, stability, political
capital, technology count, war/demand/shortage/hyperinflation day counts, outstanding
requests, command refusals, world GDP and living country count. Rows for a ceased
government retain its status and last economy; CAGR becomes unavailable and it is
excluded from the survivor CAGR summary rather than being silently replaced by a
successor. Other terminal metrics are explicitly the tracked government's state.

Instrument schema 2 adds project completion headlines, active/stalled projects,
project stall-days, actual civilian capital spending and accrued authority,
inherited-industry utilization, industrial site activity, election/coup headline
counts, conserved deployed/reserve force, arsenal inventory value, magazine
coverage and shortage-days, cargo delay-days, dated technology acquisitions, and
observer/simulation runtime. The summary retains each attempted command, its
outcome and actual political-capital charge; unavailable investment recommendations
and an absent January supplier are distinct from refused commands.

Civilian capital authority utilization is cumulative freshly expensed capital
spending divided by cumulative accrued authority in Infrastructure departments
0–3, Industry departments 0–4 and Science department 0. It excludes operating
expense and prepaid procurement. These are settled daily receipts, not summed
annualized snapshot rates. The pool includes relevant mines and prototype work;
it is not labeled as expenditure on construction projects alone. Inherited
industry utilization is the existing modeled output/capacity ratio and may exceed
100%; it does not measure physical pack-plant throughput. Site activity separately
counts owned site-days with positive output and all observed owned site-days.

Projects are stalled when their saved status is Paused, Blocked or Slowed.
Completion counts classify only the production headlines matching the tracked
country and a catalog project name followed by `level`, or its Starter Industry
module completion. Elections count only `COUNTRY votes: ` headlines; coups count
only `COUP IN COUNTRY: ... removes the government.` headlines. Scheduling an
election, proposing one or changing a coalition is not counted as an election.
Tracked-country and whole-world headline counts remain separate.

Force quantities are modeled force units. Arsenal book value is conditioned
inventory at catalog cost in billions of dollars; magazines are the existing
normalized 0–1 stock. Magazine-constrained days use the model's existing
`magazine_multiplier < 1` condition. The current saved world has no cumulative
casualty receipt: `cumulative_military_losses` is null with an explicit limitation.
Net strength changes are not substituted for losses.

Shipment delay is observed after each settlement: each inbound raw or manufactured
cargo still present on or after its booked due day contributes one overdue
cargo-day. Multiple delayed consignments contribute separately. Held cargo-days
and the greatest current overdue age are separate measures; normal transit time
is not a delay. This does not reconstruct transport damage or lost cargo.
Technology timing records each newly known stable technology id and its settlement
date; starting endowments are excluded and acquisition method is not inferred.

Observer time covers pre/post settlement collection; simulation time covers
`tick_day`, and run wall time also includes setup, policy and output overhead.
None of these clocks enter simulation decisions. Runtime is descriptive and
depends on machine load. Pre-schema-2 panels remain valid evidence for their
original fields and unchanged scenarios, but are labeled `legacy_instrument`
where compared with the extended panel; absent metrics are not treated as zero.

The sibling `.summary.json` gives each terminal metric's sample count, mean,
minimum, maximum and unbiased sample variance (`n-1`). Variance is unavailable for
fewer than two independent seeds. Summary metadata retains scenario descriptions,
feature flags and sampling limitations. Rows flush at each annual boundary, so
partial progress is inspectable if a scheduled run is interrupted.

The executable asserts only universal finite/positive economic quantities and
nonnegative opened treasury/debt stocks. A future statistical bar must state the
regression size it should detect, derive its sample size from that bar's own
measured variance and a false-red probability below 1%, and demonstrate power.
This instrument adds no calibration tolerance and cannot justify widening one.

Integrated initialization now calls `enrich_new_world` immediately after
`enable_new_world` and enables `military_operations`, matching the browser's
fresh-world setup. Physical freight defaults on; Economic Competition remains
explicit. Nightly/manual CI runs the eleven scenarios separately and includes a
matched balanced-budget panel with economic AI enabled. The earlier instrument
smoke used physical freight off and predates final integration; it is not the
release balance result. The release evidence lists the actual executed panels.
