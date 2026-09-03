# SPHERES — Complete Game Specification
*Consolidated from all design and build sessions. This is the authoritative reference for what the game is. For current build status see ROADMAP.md; for session rules see CLAUDE.md.*

---

## 1. Vision & Pillars

A Millennium Dawn-style geopolitical sandbox grand-strategy game starting **January 1, 1990** — the hinge of modern history — playable from the Cold War's end into the deep future. Deep macroeconomic simulation, HOI4-style operational warfare, spheres-of-influence diplomacy, and emergent political drama across ~190 nations. Single-player. Built in Rust.

**Pillars (in priority order):**
1. **Interconnection is the product.** Every system spends something and is spent by something. No system exists in isolation; no system "solves" the game. The fun lives in economy pressuring politics pressuring diplomacy pressuring war.
2. **Two spend-currencies spine the game:** economic output and political capital. Every other system is a buyer of one or both.
3. **Realistic dynamics, not literal simulation.** Model the *behaviors* of the real world (business cycles, deterrence, influence competition) with tractable aggregate math, so real-world intuition is rewarded.
4. **Determinism is sacred.** Same seed + same commands = same game, always. This is the replay system, the test oracle, and the debugging superpower.
5. **Complexity is budgeted.** Economy and military get maximum depth; politics stays deliberately lightweight but load-bearing.
6. **The LLM is an author and advisor, never a mechanic.** Local LLM (Ollama on the homelab) generates flavor and biases AI personality — it never makes a per-tick mechanical decision.
7. **History is calibration, not script.** Historical events (Gulf War, USSR collapse, Japan's lost decade, China's miracle, Asia 1997) must *emerge* from modeled incentives across most seeds — never be hardcoded. The only near-scripted items are physical facts like nuclear test dates.

**Scope:** single-player only, but "controller" is a per-nation property (human/AI), never a baked-in assumption.

---

## 2. Architecture (the constitution)

- **One `WorldState`** — a single serializable struct containing the entire simulation. Save = serialize it. Load = deserialize and continue the exact same timeline (serde_json with `float_roundtrip`).
- **One RNG** — SplitMix64, stored inside WorldState. Never a second RNG, never HashMap iteration order affecting state, never wall-clock time.
- **One command queue** — every mutation by player or AI flows through the same `Command` enum. No side doors.
- **Fixed system order** each tick: apply commands → economy → military/war → politics/AI → events.
- **Hourly base tick + cadence scheduler** (full design): combat/movement hourly, markets daily, macro monthly, elections/tech yearly. *v0.5 rebuild runs monthly ticks only; reintroduce hourly cadence only with a scheduler design, never by brute force. A daily CALENDAR over the monthly model is admitted as of 2026-09-01 (BIBLE §5): the player steps days, the systems settle at month-end, and a day-stepped month must equal a month-stepped one byte for byte — asserted with commands issued on the 10th, 20th, 31st and 15th of their months, the market on and a mid-month save/load by `tests::the_daily_clock_preserves_the_market_on_world` (2026-09-02).*
- **State hashing** — FNV-1a hash of WorldState for determinism tests and replay verification.
- **Content as data** — nations and events in JSON files, two-pass validation, `deny_unknown_fields`. Moddable by design.
- **Engine-agnostic headless core** (`spheres-sim`) with no I/O; CLI runner (`spheres-cli`); Bevy reserved as the future view shell.
- **Starting data is transcribed, not invented** — real 1990 figures for GDP, population, oil output, debt, military, **and the technology each nation had deployed by January 1990** (BIBLE §8, amended 2026-08-30: a sourced deployment date is a fact of the same class as a GDP figure, so the starting tech stock is authored per nation rather than derived from `tfp_trend`).

---

## 3. Economy (maximum depth pillar)

**Production & growth**
- Cobb-Douglas production per nation (full design); v0.5 uses the reduced form: growth = TFP trend + investment effect + catch-up − drags.
- Catch-up growth: poorer nations grow faster per unit of investment (convergence toward the frontier).
- Diminishing returns: capital deepening effect shrinks as GDP/capita approaches the frontier (~$24k in 1990 dollars).
- Command economies pay an allocation penalty that worsens with development — they can grow fast when poor, then stagnate (the Soviet trap).

**Money & prices**
- Phillips/Taylor monetary framework: demand gap driven by real rate vs neutral (~2.5%); inflation converges toward demand pressure + oil pass-through + war premium.
- AI central banks run a Taylor-lite rule; the player sets their own rate.
- Hyperinflation and currency reform mechanics (v0.4; not yet in v0.5 rebuild).
- Permanent-income consumption smoothing — the fix that eliminated the deflation-floor trap (v0.4).

**Fiscal & debt**
- ~~Budget: revenue = tax take + resource rents; spending = social (scales with democracy) + military + state investment.~~ **AMENDED 2026-09-02 – Ridge's call, quoted: "I like the 10 ministry budget and the 1 day ticker so if the bible needs to be ammended we can do that."** Revenue is unchanged: tax take + resource rents. Spending is now planned as a ten-ministry annual budget (`spheres-sim/src/world.rs` `AnnualBudget`), every allocation a share of GDP capped per ministry by `BUDGET_CAPS`: health 0.15, education 0.12, families 0.15, pensions 0.20, infrastructure 0.15, industry and energy 0.12, science 0.08, defense 0.35, security 0.12, diplomacy 0.08 — and at most 0.70 of GDP in total. The first time a nation opens its books the inherited plan (`AnnualBudget::inherited`) is a fixed split of the three aggregates it already ran — social 0.25 / 0.18 / 0.20 / 0.28 across health / education / families / pensions with 0.07 security and 0.02 diplomacy; state investment 0.55 / 0.30 / 0.15 across infrastructure / industry / science; defense as itself — and that split is frozen as the plan's `reference`, against which every later gap is read. A budget is enacted by `Command::SetAnnualBudget { nation, fiscal_year, allocations }`, priced in political capital by the weighted movement of every allocation from the current plan (cuts at 1.35x, +4 PC to reopen a year already enacted), and refused for any fiscal year but the current one. **The three aggregates are still the priced quantities**: on enactment `social_spend_gdp`, `state_invest_gdp` and `mil_spend_gdp` are overwritten with the plan's social, investment and defense sums when the enacted allocations differ from the plan in force; an enactment bit-identical to it (the inherited split included) seats the plan and leaves the three aggregates exactly as they were (fix 3, 52fd9f6, `tests::enacting_the_inherited_budget_unchanged_is_a_no_op`); and what the growth, debt and force models read is those three, exactly as before; a budget never enacted leaves all of it `None` and the calibrated envelopes in force. ~~The per-ministry channels beyond the aggregates — each ministry's gap from its reference entering potential growth, demand, unemployment, business pressure, population, stability, separatism, the diplomatic shield and research output — are shipped UNCALIBRATED: no calibration bar reads them, they are inert on every default path, and they are filed as BUGS D-1 (the channels) and D-2 (the player-only investment arm) rather than described here as model.~~ **AMENDED 2026-09-02 – Ridge's call, quoted: "Add in an interest over GDP figure that inflates based on percentage. You can cook the rest into the GITHUB dir".** The thirty scattered addends are GONE and each ministry now has one or two NAMED arms, defined once in `spheres-sim/src/ministries.rs` and called from every site that charges them, so the browser card and the simulation cannot disagree. The map, and it is asserted rather than described — `tests/ministries.rs::the_ministry_map_is_exactly_this` for the gap alone and `::the_enacted_ministry_map_is_exactly_this` for the whole enacted command: **0 health** — population (`gap*0.030`, INCUMBENT) and, in war only, the return of the wounded to the line (`(1+gap*6.0).clamp(0.60,1.60)` on the approach to sustained force, never on `REPLACEMENT_RATE` itself, which is read again to define a decisive battle); **1 education** — research points alone (`(1+gap*15.0).clamp(0.35,2.25)`), science having been removed from that expression; **2 housing** (renamed from Families on Ridge's standing ruling, "Families should be housing instead") — population (`gap*0.015`) and stability (`gap*14.0`), both INCUMBENT; **3 pensions** — the labour force (`gap*0.20` off unemployment), the political-capital ceiling (`gap*1000.0`) and stability (`gap*12.0`, INCUMBENT); the demand arm was DROPPED entirely rather than resized, because `demand_gap` forks into both output and inflation; **4 infrastructure** — located NON-OIL extraction only, as a STOCK that walks toward `(gap*2.0).clamp(0,0.25)` at 0.02 a month, so it is built and lost over a year rather than switched; oil is excluded because oil is already a complete national system; **5 industry and energy** — the magazines refill faster (`(1+gap*4.2).clamp(0.70,1.40)`), and NO energy system, which the design refused as unbuildable honestly today; **6 science** — absorptive capacity (`gap*6.0`), science having left the research multiplier for the price side; **7 defense** — NOTHING. This row's job is to add nothing, and a bar holds it to that so no later session gives it an arm on the grounds that every other ministry has one; **8 security** — stability (`gap*16.0`, INCUMBENT) and, alone, separatism suppression (`gap.max(0.0)*0.04`, a POSITIVE gap only, so cutting the police cannot conjure secession that is not already there); **9 diplomacy** — the sanction shield (`(gap*8.0).clamp(-0.20,0.40)`, INCUMBENT, ceiling 0.40) and counter-intelligence (`gap*10.0` on the chance a foreign operation against you is exposed, which is what costs the sponsor relations and reputation on the path that already existed). **The rule the whole thing is held to**: no ministry gap may reach `n.gdp` by more than one route, and no two ministries may write the same arm — only `population` and `stability` have more than one owner, and the design names both. The social-aggregate route that let all six social ministries move demand and inflation identically to the last digit was closed at the same time (the `social_gap * 12.0` line in `economy.rs`, deleted). All of it stays INERT on the default path: `budget_gap` is 0.0 for a nation with no enacted plan, and `Command::SetAnnualBudget` is player-only. BUGS **D-1 is closed** by this work; **D-2 is settled player-only**, every arm reading `n.budget_gap` directly, with any AI budget issuer required to sit behind a `GameRules` flag defaulting false.
- ~~Real r−g debt dynamics: deficits add to debt/GDP; growth+inflation erode it; debt above ~90% GDP drags growth.~~ **AMENDED 2026-09-02 – Ridge's call, quoted: "Add in an interest over GDP figure that inflates based on percentage."** There are now TWO arms and a nation is on exactly one of them. **Books closed** (`treasury_bn` and `debt_bn` both `None`, which is every nation on the default board): unchanged — deficits add to `debt_gdp`, growth and inflation erode it, and `growth_terms` charges `debt_drag` above 90% of GDP, bit for bit what shipped. **Books open** (the first `Command::SetAnnualBudget` seats them): money becomes a STOCK of billions of 1990 dollars. `treasury_bn` and `debt_bn` are `Option<f64>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`, like `social_spend_gdp` beside them, so a nation that never opens its books serializes nothing and the 1990 save is byte-identical. `debt_gdp` REMAINS the stored field and the single source of truth for all its readers; on this arm the fiscal block is its only writer and sets it to `debt_bn / gdp` at the end of the month. Revenue is `(tax_rate + budget_oil_revenue) * gdp / 12`, spending is `(social + military + state investment) * gdp / 12`, the balance flows to the treasury, a treasury that would go negative issues debt instead, and a receipt retires debt before it accumulates as cash — which is how a net creditor becomes representable at all. The old growth+inflation renormalisation and the `.max(0.0)` floor are deleted on this arm; that floor is what used to annihilate the remainder of an over-large receipt (MEASURED: $8.200bn destroyed out of a $10.000bn leg to Kuwait, before the repair). **Interest escalates with the debt ratio**, which is Ridge's amendment and the one thing that changed from the proposal put to him: `real = (interest_rate - inflation).max(-0.02)`, `spread = ((debt_gdp - 0.60).max(0.0) * 0.06).min(0.06)`, `effective = real + spread`, `interest_bn = debt_bn * effective / 12`. It is reported as a share of GDP — `interest_gdp = interest_bn * 12 / gdp`, ONE definition that the browser reads rather than recomputing, so the ledger and the card cannot disagree — and it sits as an eleventh, unelectable row above the ten dials so debt service visibly crowds the ministries out. MEASURED at a 5% policy rate against 3% inflation: 30% of GDP pays 2.0000%/yr, 60% pays 2.0000% (exactly at the knee), 90% pays 3.8000% (+1.80pp), 150% pays 7.4000% (+5.40pp); the cap binds from a debt ratio of 1.600 upward, measured by sweep rather than read off the literal. The roster's own median 1990 debt ratio is 0.52, so the median 1990 borrower pays the policy real rate exactly. **The debt is charged once, not twice**: `debt_drag` is gated on `n.debt_bn.is_none()`, so an open-books nation pays cash interest and no drag and a closed-books nation pays the drag and no cash. Before that repair an open-books nation at 120% of GDP paid 0.025200 of output against an identical closed-books nation's 0.006000, a factor of 4.20, and the browser printed both as though they were different costs. The five hand-rolled ratio pushers — `resources::settle` (both legs), pact upkeep, aid, covert action and the patronage envelope — all route through one helper, `economy::charge`, which performs today's EXACT arithmetic when the books are closed and debits the stock when they are open. It takes BOTH the dollars and the caller's exact pre-treasury ratio, precisely so the closed arm never recomputes: `(share * gdp) / gdp` is not `share` in binary floating point, and that rounding is measurable over 240 months. The four successor states built by struct literal in `politics.rs` carry `treasury_bn: None, debt_bn: None` EXPLICITLY rather than by inheritance, because `debt_gdp` at those sites is an authored successor figure (Russia 0.35 against the union's 0.45) and not a share of the parent's dollar stock. Foreign reserves are transcribed, not invented: 79 of 137 nations carry a sourced 1990 `reserves_bn` (World Bank FI.RES.TOTL.CD, the END-1989 observation, because a reserve is a STOCK and the stock on the morning of 1 January 1990 is the end-1989 figure — the same reasoning `EconomyRecord::pop_growth_1990` already gives for ending its window on 31 December 1989); 17 are REFUSED for want of a source and 41 left out as immaterial, both listed in BUGS M-1 and M-2.
- Sovereign default with forced austerity (v0.4; not yet in v0.5 rebuild).
- Fiscal AI: nations consolidate (raise taxes, trim spending) when debt runs hot.

**Bubbles**
- Asset bubble intensity per nation (Japan starts at 0.95). Bubbles add demand while inflating; tight real rates pop them; a pop flips into a negative "hangover" (balance-sheet recession) whose drag fades over ~a decade. Japan's lost decade must emerge.

**Oil (world market)**
- Global price driven by supply disruption: wars involving producers and heavy sanctions take barrels off the market.
- Producers gain terms-of-trade revenue when oil is dear; importers eat an inflation shock. **Oil shock → global inflation propagation is a locked regression test.**
- Full embargo plumbing (pending): sanctions on a producer must cut oil *exports* specifically and tighten the world market, not just apply a GDP drag.

**Trade network (v0.4; not yet in v0.5 rebuild)**
- Bilateral trade allocated via gravity model with sphere-alignment bonuses; sanctions and war sever links; trade dependency feeds diplomatic receptivity.
- Future: chokepoints (Hormuz, Suez, Malacca), financial contagion — Asia 1997 should be emergeable.

---

## 4. Politics (lightweight but load-bearing)

- **Computed stability** (0–100) with asymmetric inertia: built by growth, spent by inflation, war exhaustion, sanctions, and (for command economies) stagnation — their legitimacy is growth-bought.
- **Ideology drift** and authoritarianism scale (0–1) gating elections, social spending, coup risk.
- **Security-state coup suppression** (v0.4): military spending buys coup protection for autocrats.
- **Democratic elections** every 4 years: bad times throw the bums out; new governments reset some legitimacy.
- **Regime collapse**: stability floor triggers revolution — new regime, GDP hit, ideology reshuffle.
- **Separatism strain** per nation, growing under instability, decaying under stability.
- **Union dissolutions** — two modeled systems (Soviet, Yugoslav) spawning latent successor states from separatist pressure, not on a date. USSR: stability < ~25 or separatism > ~0.9 → dissolution; Russia inherits ~55–65% of economy/military and the arsenal (full design spawns all successor tags incl. Ukraine; v0.5 abstracts to Russia). Yugoslavia (pending in rebuild): fragments in the 90s with regional war risk; Serbia as successor.

---

## 5. Diplomacy — Spheres of Influence (the namesake system)

- **Relations matrix** (−100..100), symmetric, shifted by actions (sanctions −15, war −60, diplomatic pushes +6, coalition responses −25).
- **Influence projection** (v0.4 full system; simplified in v0.5): great powers project influence into other nations; influence decays as upkeep, creating a spend-to-hold economy.
- **Alliance hysteresis**: alignments resist flipping, then flip hard.
- **Trade dependency feeds receptivity** — economic ties make nations amenable.
- **Sanctions**: imposer/target pairs; growth drag on target; sanctioned oil producers sell less; sanctions lift at peace.
- **Coalition response**: aggressors get sanctioned by the majors; friends of the victim intervene militarily (relation ≥ 40 gate), with the nuclear caveat below.

---

## 6. Military & War

- **Strength as a stock** fed by military spending (sqrt-scaled budget → sustained strength); attrition in war; peacetime decay toward what the budget sustains.
- **War resolution**: side strength ratio pushes a progress bar (−100..100) with noise; exhaustion accrues (faster on the losing side) and reduces effective strength; white peace when both sides are spent.
- **Victory outcomes**: small nations (pop < ~8M) can be annexed (war-damaged GDP transfer, oil capture, separatism strain for the occupier); large nations are *subjugated* instead — reparations, ceded industry, disarmament, regime destabilization. No swallowing India whole.
- **Defeat**: repelled aggressors lose stability, and the lesson sticks — a permanent "burned" flag kills their appetite for that target (Saddam doesn't retry Kuwait).
- **AI war decisions**: appetite = base rate (historically-plausible dyads only) × aggression setting × fiscal desperation × strength ratio × hostility — with expected defense including likely interveners. First-time gamblers heavily discount the coalition (Saddam's 1990 misjudgment); after being burned once, they weigh it fully.
- **Nuclear taboo**: no direct wars between nuclear powers, ever; non-nuclear nations never attack nuclear ones. Proliferation: India & Pakistan test May 1998 (near-scripted physical fact), after which deterrence descends on the subcontinent.
- **Full design (not yet built)**: HOI4-style operational layer — divisions, province map, supply flow network, statistical combat testing, deterministic replays.

---

## 7. Technology

- **Tech eras** layered as TFP waves over continuous R&D (v0.4; not yet in v0.5 rebuild): ~1995 internet boom (favors open economies), ~2007 mobile, ~2023 AI era. Tech leader/laggard dynamics.

---

## 8. Narrative & LLM layer (future)

- The sim generates real drama; the narrative layer narrates it. Template combinatorics + local LLM (Ollama) flavor for chronicles, advisors, and AI leader personality bias.
- LLM is never in the tick loop and never makes mechanical decisions.

---

## 9. Roster

- v0.5 rebuild: 16 nations (USA, USSR→Russia, China, Japan, Germany, UK, France, Italy, India, Pakistan, Iraq, Kuwait, Saudi Arabia, Iran, South Korea, Poland).
- v0.4 reached 32 nations across 9 regions (G7, Gulf, South Asia, Global South complete).
- Expansion order: Yugoslavia+successors, Brazil, Indonesia, Egypt, Israel, Turkey, Nigeria, Vietnam, Ukraine. Full vision: ~190.

---

## 10. Testing (the harness IS the game's quality)

Every mechanic is validated three ways; `cargo test` gates every change; regression tests are never deleted or weakened.
1. **Determinism**: same seed ⇒ identical serialized world after decades; different seeds diverge.
2. **Invariants**: 50-year runs with no NaN, no negative GDP, no debt spirals, bounded stability, sane oil prices.
3. **Save/load continuity**: a loaded save continues the *exact* timeline.
4. **Historical calibration** (emergent, across multiple seeds — usually, not always):
   - Iraq invades Kuwait early-90s; coalition repels; never retried
   - USSR dissolves in the 90s in most seeds
   - China: >6x real growth over 30 years (~9x observed, matching constant-dollar reality)
   - Japan's bubble pops into a lost decade; Japan does not overtake the US
   - Stable democracies never hyperinflate
   - Oil shock raises importer inflation vs baseline
   - Nuclear taboo holds
   - Rich democracies settle near ~2% growth; inflation converges near target

---

## 11. Player experience (current surface: CLI play mode)

- Pick any nation, January 1990. Monthly loop: briefing (GDP, growth, inflation, rate, debt, tax, military, stability, oil, wars) → queue commands → advance (`next`/`year`/N months) → world reacts, headlines print.
- Commands: `rate` / `tax` / `military` / `invest` (percent-of-GDP levers), `improve` / `sanction` / `lift` / `war` (confirmed), `status` / `world` / `relations`, `save` / `resume` / `quit`.
- Losing: your nation annexed = game over; subjugation and revolution you play through.
- `GameRules` knobs: seed, `ai_aggression`, `crisis_intensity` — the difficulty surface.
- Future surfaces: event-pause prompts ("Iraq invaded Kuwait — respond?"), advisors, Bevy map shell, WASM browser build.

---

## 12. Build history & phased plan

- **v0.1**: deterministic core, hourly ticks, macro economy, politics, JSON events, utility AI, 9 tests. China 390B→7.4T emergent.
- **v0.2–v0.3**: diplomacy/spheres, trade network, war system, oil market.
- **v0.4** (lost with its container; being restored): 32 nations, tech eras, dual dissolution systems, sovereign default, hyperinflation/currency reform, coup suppression, 14-test suite, Gulf War regression locked.
- **v0.5** (current, compact rebuild): monthly ticks, 16 nations, all 8 tests green, interactive CLI play mode — first playable slice.
- **Next**: restore v0.4 scope (embargo plumbing, Yugoslavia) → roster expansion → tech eras → negotiated peace deals → play-mode polish → financial system/contagion → hourly scheduler → Bevy/WASM UI.

## 13. Workflow & endgame

- Development moving from chat sessions (zip courier) to Claude Code against a persistent git repo.
- End state: **autonomous nightly cron sessions on the Proxmox homelab** — `claude -p` driven by CLAUDE.md + ROADMAP.md as standing memory, tests as the gate, commits as the log.

## 14. Risk register

1. Economic calibration at scale — 200 AI economies must not spiral (Victoria 3's saga). Mitigation: headless century-runs as CI invariants.
2. Map rendering — deceptively fiddly. Mitigation: early prototype when the time comes.
3. Scope — years of work. Mitigation: vertical-slice discipline; every phase ships a playable artifact.
4. Generic-feeling narrative — mitigated by the sim generating real drama plus template combinatorics + LLM flavor.
5. Balance — mitigated by statistical combat testing and deterministic replays.
