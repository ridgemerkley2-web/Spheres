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

The two political-capital overrides are explicitly controlled fixtures, not new
campaign starting grants or simulated historical facts. Policy goes through normal
commands and pays normal prices; later refusals are counted. Every other opening
country quantity remains inherited from the data. Economic Competition, when
selected, governs only eligible AI countries; the player stays responsible for its
own budget choices and receives the same resource and fiscal constraints.

Annual CSV rows include GDP level/multiple/CAGR, population and GDP per person,
inflation, debt ratio, interest burden, cash/debt stocks, stability, political
capital, technology count, war/demand/shortage/hyperinflation day counts, outstanding
requests, command refusals, world GDP and living country count. Rows for a ceased
government retain its status and last economy; CAGR becomes unavailable and it is
excluded from the survivor CAGR summary rather than being silently replaced by a
successor. Other terminal metrics are explicitly the tracked government's state.

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

Integration contract: the root release supplies the new inherited-sector
`enrich_new_world` initialization immediately after `enable_new_world`, and the
`military_operations` flag alongside its browser rules. The initial development
smoke panel predates those two hooks and is evidence about this instrument and
its explicit policy fixtures, not the final integrated browser campaign. The
final release must run matched panels after those hooks and the freight repair.
