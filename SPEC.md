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
- **Daily model, amended 2026-09-03 by Ridge.** Browser boot/new/load set `daily_simulation`; each `tick_day` applies commands, advances construction, runs the fixed SYSTEMS table with prorated flows, and advances the Gregorian date. `tick_month` in daily mode is only a batch of the remaining daily ticks. `clock.rs` owns calendar fractions, compounding, hazard conversion and deadlines. Annual budgets remain annual. Existing headless monthly mode remains unchanged for reproducible legacy runs. Same dated commands, including save/resume, must replay identically; issuing a command later can now change outcomes. See DAILY.md and the superseding BIBLE amendment. No hourly engine is introduced.
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
- ~~Real r−g debt dynamics: deficits add to debt/GDP; growth+inflation erode it; debt above ~90% GDP drags growth.~~ **AMENDED 2026-09-02 – Ridge's call, quoted: "Add in an interest over GDP figure that inflates based on percentage."** There are now TWO arms and a nation is on exactly one of them. **Books closed** (`treasury_bn` and `debt_bn` both `None`, which is every nation on the default board): unchanged — deficits add to `debt_gdp`, growth and inflation erode it, and `growth_terms` charges `debt_drag` above 90% of GDP, bit for bit what shipped. **Books open** (the first `Command::SetAnnualBudget` seats them): money becomes a STOCK of billions of 1990 dollars. `treasury_bn` and `debt_bn` are `Option<f64>` with `#[serde(default, skip_serializing_if = "Option::is_none")]`, like `social_spend_gdp` beside them, so a nation that never opens its books serializes nothing and the 1990 save is byte-identical. `debt_gdp` REMAINS the stored field and the single source of truth for all its readers; on this arm the fiscal block is its only writer and sets it to `debt_bn / gdp` at the end of the month. Revenue is `(tax_rate + budget_oil_revenue) * gdp / 12`, spending is `(social + military + state investment) * gdp / 12`, the balance flows to the treasury, a treasury that would go negative issues debt instead, and a receipt retires debt before it accumulates as cash — which is how a net creditor becomes representable at all. The old growth+inflation renormalisation and the `.max(0.0)` floor are deleted on this arm; that floor is what used to annihilate the remainder of an over-large receipt (MEASURED: $8.200bn destroyed out of a $10.000bn leg to Kuwait, before the repair). **Interest escalates with the debt ratio**, which is Ridge's amendment and the one thing that changed from the proposal put to him: `real = (interest_rate - inflation).max(-0.02)`, `spread = ((debt_gdp - 0.60).max(0.0) * 0.06).min(0.06)`, `effective = real + spread`, `interest_bn = debt_bn * effective / 12`. It is reported as a share of GDP — `interest_gdp = interest_bn * 12 / gdp`, ONE definition that the browser reads rather than recomputing, so the ledger and the card cannot disagree — and it sits as an eleventh, unelectable row above the ten dials so debt service visibly crowds the ministries out. MEASURED at a 5% policy rate against 3% inflation: 30% of GDP pays 2.0000%/yr, 60% pays 2.0000% (exactly at the knee), 90% pays 3.8000% (+1.80pp), 150% pays 7.4000% (+5.40pp); the cap binds from a debt ratio of 1.600 upward, measured by sweep rather than read off the literal. The roster's own median 1990 debt ratio is 0.52, so the median 1990 borrower pays the policy real rate exactly. **The debt is charged once, not twice**: `debt_drag` is gated on `n.debt_bn.is_none()`, so an open-books nation pays cash interest and no drag and a closed-books nation pays the drag and no cash. Before that repair an open-books nation at 120% of GDP paid 0.025200 of output against an identical closed-books nation's 0.006000, a factor of 4.20, and the browser printed both as though they were different costs. ~~The five hand-rolled ratio pushers — `resources::settle` (both legs), pact upkeep, aid, covert action and the patronage envelope — all route through one helper, `economy::charge`~~ **AMENDED 2026-09-02 on the merge of `origin/feat/hoi4-map-and-tech` 61b388f.** EVERY hand-rolled ratio pusher routes through one helper, `economy::charge`, and there are now NINE call sites rather than six, MEASURED on the merged tree: `resources::settle` (both legs), `resources::apply_market_net`'s outflow and receipt arms, `resources::start_mine`'s construction investment, pact upkeep, aid, covert action and the patronage envelope. The three new ones are upstream's, written after the helper existed and pushing `debt_gdp` directly; each was routed through `charge` with the `share` argument character for character the ratio the line already pushed, so a nation with the books closed is bit-identical. The helper performs today's EXACT arithmetic when the books are closed and debits the stock when they are open, and it RETURNS the dollars of a receipt the debt could not absorb, which only a closed-books caller keeping a till of its own (that is `apply_market_net`, and only it) ever acts on. It takes BOTH the dollars and the caller's exact pre-treasury ratio, precisely so the closed arm never recomputes: `(share * gdp) / gdp` is not `share` in binary floating point, and that rounding is measurable over 240 months. The four successor states built by struct literal in `politics.rs` carry `treasury_bn: None, debt_bn: None` EXPLICITLY rather than by inheritance, because `debt_gdp` at those sites is an authored successor figure (Russia 0.35 against the union's 0.45) and not a share of the parent's dollar stock. Foreign reserves are transcribed, not invented: 79 of 137 nations carry a sourced 1990 `reserves_bn` (World Bank FI.RES.TOTL.CD, the END-1989 observation, because a reserve is a STOCK and the stock on the morning of 1 January 1990 is the end-1989 figure — the same reasoning `EconomyRecord::pop_growth_1990` already gives for ending its window on 31 December 1989); 17 are REFUSED for want of a source and 41 left out as immaterial, both listed in BUGS M-9 and M-10.
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
- **The political arm — the bloc lens (built 2026-09-05, stages S0-S2 of "The Political Arm of SPHERES", revision 2, on Ridge's approval quoted: "Go ahead and build it with code").** Five blocs in one fixed order — Western, Communist, Nationalist, Islamist, Non-Aligned; ties break in that order everywhere — read off the party tables that already exist: `Family::bloc()` gives the default (Liberal, Christian-Democratic, Conservative, Social-Democratic, Green and Agrarian are Western; Communist is Communist; Nationalist is Nationalist; Religious is Islamist; Big-Tent and Regionalist are Non-Aligned), 34 sourced per-party overrides on `PartySpec.bloc` correct it (democratic umbrellas such as Solidarity, DEMOS and Civic Forum are Western while state big tents stay Non-Aligned; the ten non-Islamic Religious parties are Western; `in_bjp` Nationalist; `ru_apr` Communist), and `government::bloc_of` is the one function that answers. A pillar installs a bloc: Army Nationalist, Security Non-Aligned, Business Western, Party Communist where the regime's largest party is Communist else Non-Aligned, Clergy Islamist where the clergy is Muslim else Western. **Shares.** Electoral nations: S_B is the sum of party support over the parties of bloc B — nothing stored, so a vote or a drift moves it live. Pillar regimes: `GovState.movements`, seeded FLAT from the leader row the first time the arm sees the nation (ruling bloc 0.60, the rest split equally over the non-ruling blocs PRESENT in the polity — a party of that bloc in the dormant table or the bloc's installing pillar — floor 0.002, normalised); dormant party tables stay dormant. **Ruling bloc.** The coalition leader's bloc, recomputed whenever `form_government` runs; for a regime, `GovState.regime_bloc` seeded from the leader row (the leader's party, else the pillar he heads, else a sourced `bloc_override` — Sudan: Islamist). Monarchy exception: where the leader row ties an ELECTORAL polity to a pillar and authoritarianism is at or above 0.40 (Jordan in 1990), the court pillar rules and the chamber leader is served as `government_of_the_day`. **Discontent** = clamp(0.50·min(1, order) + 0.20·prices + 0.20·growth + 0.10·war, 0, 1) over the same `government::pains` the party model already moves support with (stability 40 alone reads 0.1667, stability 25 alone 0.2917). **Influence** I_B = S_B + F_B, where F_B is foreign backing summed from `Statecraft.backing` — EMPTY until S3, served as zero. **The takeover watch** (`blocs::takeover_readout`) serves four roads as gauges — coup: army loyalty ≤ 0.35, discontent ≥ 0.25, coup pressure ≥ 1/crisis_intensity; uprising: discontent ≥ 0.45 and the strongest non-ruling bloc's influence ≥ 0.45, that bloc named; round table: Western influence ≥ 0.40, Party loyalty ≤ 0.55, stability inside 30..70; collapse: stability ≤ 12 — and EVERY road reads closed, reason "not in this build", because `ideology_takeover` is off and S4 has not landed; the numbers are served so the screen can draw them, and the map hatches a nation whose threshold gauges include one at or past half its trigger (a band gauge has no half and is not read). **The leader table** `spheres-sim/data/leaders_1990.json`, embedded beside the nations: the person who DIRECTED THE EXECUTIVE on 1 January 1990, one row per roster nation (137), each carrying its office and since-date, born (for the S4 hazard draw), a tie to a party or a pillar OF ITS OWN POLITY TABLE, an optional sourced bloc_override, heir / must_leave_by / also as facts OF that date, and a non-empty sources list. The loader refuses an empty sources list, a nation not in the roster, a party or pillar not in that polity, a date that is not one or that names anyone after the start, exactly as `parse_nations` refuses today. A row whose tie NO FETCHED SOURCE SUPPORTS is kept REFUSED — name and tie null, the note opening with the word and saying why — and the nation is described from its table by institution, never by an invented name; five rows of the 1990 table are refused (Chile, Comoros, Cyprus, Greece, Panama; BUGS.md P-7). The 23 successor states have no row by design (D2) and are described from birth. **Switches and inertness.** `GameRules.ideology_blocs` (on in browser play at boot, new game and load; off headless) and `GameRules.ideology_takeover` (off everywhere), both default false and unserialised when false; `movements`, `banned`, `regime_bloc`, `WorldState.leadership` and `Statecraft.backing` serialise nothing when empty; the module draws no RNG and writes nothing the model reads, so with the switch off the 1990 hash and the headless digests are unchanged, and with it on the timeline is bit-identical to off. **Surface.** `/api/state` per nation gains `ruling_bloc`, `discontent`, `blocs[{bloc, share, backing, banned}]`, `leader`, `government_of_the_day`, `takeover` (all null when off); `GET /api/government?nation=` serves the government screen (key **I**): the five-segment bar, the party table grouped by bloc or the named pillars with loyalty and coup pressure, the watch as gauges, and the four government commands and two political stratagems each with the sim's own `price_of` and `refusal_of`; the header carries the DISCONTENT chip (green under 25, amber 25-49, red at 50) and the ruling swatch; the map gains a ninth mode, Ideology, coloured by ruling bloc. Nothing on the page is computed in JavaScript. **STILL DESIGN, not built** (S3-S5): the roads as mechanics, the five new commands including Ban and Back a movement, movement drift, foreign backing and the regional sponsors flag (D3), the census, and leader mortality by the annual hazard draw (D1). The Nepal (May 1991) and Haiti (December 1990) dormant party tables of D4 are `government::D4_POLITIES`, beside `POLITIES` and NOT wired (BUGS.md P-8; wired under the switch by a `polity_in` on 2026-09-06 and reverted the same day, S7-3 — Ridge's call).
- **The political arm — movements, foreign backing, the five levers, the four roads, succession and mortality (built 2026-09-06, stages S3-S4 of the same design, on Ridge's approval of all six decisions at their recommendation; shipped after three skeptic passes and five repairs, recorded in BUGS R-1..R-10).** Everything below is behind `rules.ideology_blocs` (the lens; `play_rules` turns it on in the browser) and the roads behind `rules.ideology_takeover`, which nothing in the tree turns on: the browser reads every road `calibration pending` with live gauges until the S5 census calibrates them. Every coefficient is INVENTED, labelled at its definition, filed in BUGS with what would calibrate it. **Movements** (`government::drift_movements`, after `regime_tick` in the non-electoral branch, the same shape as `drift_support` so the two cannot disagree in kind): record = 0.35 − (0.90·prices + 0.90·growth + 1.10·war) off `pains()`; the ruling bloc moves by record·0.006·dt, the transfer bounded 0.015·dt; the outflow goes to present non-ruling blocs in proportion to the MEAN of `appeal()` over the bloc's member families in the dormant table (representatives where none: Western = Liberal/Social-Democratic/Conservative, Non-Aligned = Big-Tent), never the max; then a blend(0.005) reversion toward the flat seed, floor 0.002, normalise; no RNG. The liberalisation seam reseeds party support from the movements when the branch schedules a regime's first free elections (S_B × table weight within the bloc; a bloc with a movement and no party is dropped and named in the headline). A non-ruling movement crossing 0.30 upward prints "The {bloc} movement in {X} passes a third of the country" once, latched, cleared under 0.25; the latch is seeded closed at the 1990 seed, on a programme, on any break and on an uprising. **Foreign backing** (`CovertOp::BackBloc(Bloc)`, parse `back:<bloc>`, the same 5 PC REFUSABLE command, sovereignty gate, service charge gdp·0.0008, `success_p` / `expose_p` (now `statecraft::covert_odds`, the one place, quoted on the dossier as "works p% / exposed q%"), heat +0.18 and two `chance` rolls in the same order — no third draw): FIXED +0.06 per clean op, per-sponsor cap 0.12, per-bloc cap 0.25, in `Statecraft.backing` sorted; refused when the bloc is the target's ruling bloc ("You cannot back a government covertly — send aid"); decays 0.006·dt a month, retained while > 0, gated on the lens; exposure halves every entry of that sponsor in that target and taints the bloc −0.02 of support (electoral: from its parties by size; regime: from the movement), the headline naming sponsor and bloc. Patronage gravity is a VIEW: each live `AidFlow` counts 0.10·min(1, infusion_share/0.10) of the patron's ruling bloc toward F_B inside the 0.25 cap. Influence I_B = S_B + F_B; F_B never leaks into support; effective army loyalty = loyalty(Army) − F_Nationalist. D3: `NationDef.ideological_sponsor` on Saudi Arabia, Iran, Pakistan (Islamist), Libya (Nationalist), Cuba (Communist), read only by the gated AI covert arm (`politics.rs`: a present non-ruling bloc at I_B ≥ 0.15 that is the sponsor's own colour or sits in a rival's client, the draw after the choice), never by `patrons()`. The Security Crackdown gains one gated arm: F_B halved for every non-ruling bloc. **The five levers** (`Command::SuspendConstitution` 40 PC, `BanParty` 18, `LegalizeParty` 12, `DeclareProgramme` 35, `ConveneRoundTable` 30, all REFUSABLE, refused before any state read — "This world does not model ideological movements" with the lens off — each built as refusal → plan → arm → effects off ONE plan struct, so the card and the world take the same clamped numbers, rule 8): a suspension needs an electoral polity, stability < 45 or government seats < 0.5, authoritarianism ≥ 0.20, and writes authoritarianism max(auth + 0.30, 0.65), the incumbent's colour as `regime_bloc`, the cabinet kept dormant, movements seeded from the bloc sums with the ruler +0.10, stability −6, −10 with every democracy (auth < 0.30); a ban needs authoritarianism ≥ 0.30 or a pariah, never the coalition leader, and takes seats not voters (`seats_from_legal`, a gated wrapper over an untouched `seats_from`; support kept and counted in I), authoritarianism +0.04, stability −3 when the party holds ≥ 0.15, −4 with democracies — and NO cabinet arm (R-4); legalising reverses it at −0.02 / +3; a programme needs a regime with the bloc's movement ≥ 0.25 or the strongest pillar's colour, and writes the colour, stability −5, Clergy +0.15 if Islamist else −0.10, Army +0.10 if Nationalist, Party −0.15 leaving Communist, −15 with every patron of the old colour and +10 with every patron of the new, the movement +0.10 then normalise; a round table needs discontent ≥ 0.40 and the largest non-ruling movement ≥ 0.25, and writes authoritarianism min(auth − 0.25, 0.59), stability +8, every ban lifted, first free elections in six months. The AI (`government::ai_lever`, pure) asks for a suspension when electoral, stability < 30, auth ≥ 0.25, 55 PC; a ban on the largest party of the strongest non-ruling non-Western bloc at I ≥ 0.35, auth ≥ 0.40, 60 PC; a programme toward the strongest pillar's colour when the ruling movement < 0.30 at 70 PC; a round table when non-electoral, discontent ≥ 0.50, the largest movement ≥ 0.35, the armed mean loyalty < 0.50, 60 PC — and acts on the deck's ONE 0.02 monthly draw in `stratagems::ai_stratagems`, the lever before the card, one decision a month (R-3); the government module draws no RNG in any state. **The roads** (all behind `ideology_takeover`; D = discontent, R = the ruling bloc, W = argmax of I_B over the present non-ruling winnable blocs, ties in enum order, never Regionalist-only, never Western without a party table; crisis = `crisis_intensity.max(0.1)`). Route 2, a coup against an elected government: electoral polities whose state holds an Army walk the Army and Security lines of `pillar_targets` (regime_tick's formulas, factored) and accrue pressure 0.30·(2·(0.35 − eff_army) + (D − 0.25))·dt while eff_army < 0.35 and D ≥ 0.25, cap 1.5, else −0.03·dt; at pressure ≥ 1/crisis after 12 months in office, before the fragile branch, `regime_break` — `maybe_coup`'s block, ONE function shared by the regime's own coup, route 2 and the annulment: stability −16 floor 5, gdp ×0.97, political capital reseated, the mover 0.90 and the rest 0.72, the office clock to zero — with authoritarianism max(auth + 0.25, 0.65), the Nationalist colour, the cabinet dormant, the movements seeded from the party bloc sums; "COUP IN {NATION}: {army} removes the elected government." The regime's own coup keeps its colour under the lens alone and rules in the mover's colour only under the roads, the deposed movement latched (R-2). The annulment, inside `hold_election` between the seats and the formation: a live Army, a Communist or Islamist would-be leader, authoritarianism ≥ 0.35, D ≥ 0.25, no court → the break with every party of the winner's bloc banned, "COUP IN X: the army annuls the election {party} won." Route 3, the uprising: the `politics.rs` collapse chain gains an arm at the same site with the same `monthly_chance(0.10·crisis)` draw (the off world's code and draw untouched), firing on stability < 12 or [D ≥ 0.45 and I_W ≥ 0.45 and coercion fails: regime — weakest armed loyalty < 0.35 or mean < 0.35 (CALIBRATED 2026-09-06, S5, from the design's 0.50 / 0.55: both now quote the pillar model's unpaid line, `regime_tick`'s pressure and `ai_government`'s purchase — anchor A3, BUGS S5-2; before → after at N=60, Communist non-ballot takeovers 60/60 seeds at 1/3/4 a seed → 58/60 at 0/2/3, Iraq's Nationalist/Non-Aligned flip-flop gone); electoral — only if W is non-Western or authoritarianism ≥ 0.40]; the crown goes to W only where the MOVEMENT armed the collapse (`blocs::uprising_armed` at the firing site — CALIBRATED 2026-09-06, S5, anchor A3, BUGS S5-2: a stability < 12 collapse the movement did not arm runs the pre-arm collapse block verbatim, its random authoritarianism shift included, "the old regime falls"; Peru 60/60 → 0, Georgia 60/60 → 0, A3 per seed 3/5/6 → 1/3/4); stability 45, gdp ×0.93, no random auth shift under the switch, the winner's authoritarianism (Western min(auth, 0.40), Communist max(auth, 0.80), Nationalist max(auth, 0.72), Islamist max(auth, 0.80), Non-Aligned max(auth, 0.65), clamp 0.05..0.95), the cabinet cleared, `regime_bloc` = W, W +0.15 then normalise, the home pillar 0.80 and the rest 0.55; "Revolution in {}: the {bloc} movement takes power." Route 4, the round table: non-electoral, I_Western ≥ 0.40, stability 30..70, loyalty(Party) < 0.55 (the weakest armed where there is no Party) → authoritarianism −0.01·dt a month to a floor of 0.55, the only authoritarianism drift; crossing 0.60 makes the state electoral and the branch schedules first free elections. The foreign payoff on any non-ballot takeover (`statecraft::takeover_payoff`, one plan): every sponsor holding ≥ 0.10 behind the winner +40 with the new regime and −25 with the target's previous top patron, exposed on the spot with the existing costs at covert heat ≥ 0.50; democracies −8 with a new Communist/Nationalist/Islamist regime and +8 with a Western one; patrons whose own colour was the loser's −10. **Succession** (D2, `government::Succession` / `seat_office`, on every change of government, gated on the lens): the leader row becomes `Office::Emergent {described, party, pillar, since}` — the same leading party keeps the transcribed person; a new leading party seats "the {party} government"; a coup or a Nationalist/Non-Aligned takeover seats the polity's real pillar name; an Islamist/Communist takeover the bloc's largest party or the Clergy/Party pillar; a reached `must_leave_by` "a new {party} president"; a monarch or party-state leader removed by a Party coup or a programme the transcribed heir ONCE then "the ruling house"; a removed incumbent never returns by name; NO NAME is written for a date after 1 January 1990 but the heir. A vote never unseats a transcribed holder tied to a pillar — a crown, a court, a party-state chief — the chamber's winner is the government of the day beside the row (R-1). **Mortality** (D1, `politics::mortality`, last in `SYSTEMS`, gated on the lens): one draw per living transcribed leader per game year in sorted NationId order on the one RNG, annual hazard q(age) = 0.015·2^((age − 65)/7) — INVENTED, a Gompertz curve fitted by eye to 1990 male life tables — "{name} dies in office.", the office seated by the leader's own party or pillar. **D4** (Nepal May 1991, Haiti December 1990) is transcribed beside the table as `D4_POLITIES` and NOT wired: wiring it moves the 1990 start hash with the switch off (P-8, S4-10), Ridge's call. **The web** (`spheres-web`): the government screen serves the five levers with `price_of`, the refusal prose and the effects list off the plan; the covert card on the TARGET's dossier (`GET /api/covert?nation=`) carries the three operations and "Back the {bloc} movement" per present bloc with "works p% / exposed q%" from `covert_odds`; the bar hatches each bloc's F_B "backed from abroad", the sponsor named only after exposure (the server never sends an unexposed one); the takeover watch reads every road with its served reason; the political stems are filed and promoted as event cards; the header chip pulses and the dock names the road when any gauge passes half. Nothing is recomputed in JavaScript; `ideology_takeover` stays off in the browser.
- **The political arm — S5, the census, the first pins and the switch decision (2026-09-06, the pin-and-wire pass on `feat/ideology-census`).** `spheres-sim/tests/bloc_census.rs` measures the design's anchors A1-A10 per seed (both switches on, monthly, 252 months or `SPHERES_CENSUS_MONTHS`, seeds from `SPHERES_CENSUS_SEEDS`, no player), and its bars sit beside the scan reading the same code. The N=200 reading of this tree, per seed min/med/max: A1 coups against elected governments 4/6/9 (mean 6.35, sd 0.85) — inside 4..14 and degenerate, Sao Tome 545, Philippines 460, Comoros 200 of 1269; A2 Islamist takeover without a ballot by 2000 0/200 seeds; A3 Communist takeover without a ballot 197/200 seeds, 0/2/4 a seed (Belarus 181, Cambodia 142, Ukraine 91); A4 1990 regimes electoral by end-1996 1/2/3; A5 ex-communist return by ballot 0/200; A6 route events in 1990 democracies 0 in 200/200 (0 in 60/60 at 420 months, Argentina's pre-existing collapse beside in 59/60); A7 ballot bloc flips / non-ballot takeovers 0.16 median; A8 8 of 8 big regimes kept in 200/200; A9 3718/3900 = 0.953 of coups in nations under $8,000 a head; A10 discontent lead 0.27/0.33/0.38. FOUR BARS ARE PINNED, each with its seed count derived from that sample (iron rule 7; the variance, the n and a power statement beside each): A6 as an invariant over 420 months on 12 seeds (a power budget: sees a road touching one democracy in five seeds at 0.93); A8 on 40 seeds, 24 must keep six of eight (sees the eight's survival falling to a coin toss at 0.87; decorative against one regime falling — said so); A9 on 12 seeds pooled ≥ 0.80 (sees the rich-nation coup share rising ×4, not ×2; NOT red with route 2 thrown open, 0.878 — it guards the transcription of which polities carry an Army, recorded as decorative against the road constants); A10 on 12 seeds, median ≥ 0.25 (sees the lead falling by 0.10 at 0.92). Each was watched red against a mutation of the lines it guards, the readings in the file. SIX ARE WRITTEN AND `#[ignore]`d at their measured reading, never widened: A1's concentration arm (top-3 share median 1.00), A2 (0/200), A3 (0.985 against ≤ 0.10), A4 (median 2 against 15..40), A5 (0/200), A7 (0.16 against ≥ 3); the reasons are BUGS S5-5..S5-8 and S6-3. THE SWITCH DECISION: `ideology_takeover` stays OFF in browser play (`play_rules`) and the watch keeps reading `calibration pending`, because five anchors and one arm are outside their bands (BUGS S6-5 carries the list and the distances); it turns on only when every anchor is inside. D4 (above) was wired under the lens switch by that pass and REVERTED by the ship pass (BUGS S7-3): `D4_POLITIES` stays beside the table, unread, Ridge's call. The ship pass also repaired A6's attribution — a route event's uprising is the crowned headline, the firing site's own clause, and not the pre-tick flag (S7-1) — recorded that two of A5's six named returns are unreachable by transcription (S7-5), and re-read the census and every watched red on the tree it ships (S7-2, S7-4).
- **The political arm — the history pass and the calibration pass (2026-09-06, `feat/ideology-history` off 2e164ae, on Ridge's determinations, quoted: "Make determinations for these and continue. I want it based on historical data.").** Every constant below carries its basis at its definition; the default path is byte-identical throughout (both golden actuals, both market-OFF digests, the government module draws no RNG), and everything is behind `rules.ideology_blocs`. **M1, presence through backing** (`blocs::PRESENCE_BACKING` = 0.05, the ruling's number): a bloc is present in a regime when foreign backing behind it (`blocs::backing`, stock plus gravity) is at or over the line, so `bloc_present` = `bloc_in_table || bloc_backed`; a backed non-Western bloc can win an uprising; the AI sponsor (`politics::ai_back_bloc_choice`) backs its OWN bloc in a target where that bloc is absent from the TABLE when the target is hostile (`HOSTILE_TARGET` = −30, `best_covert_target`'s line, named) or a rival's client — the first clean operation creates the presence (anchor: the Peshawar parties, the NIF, the contras). The ideological sponsors' block acts once a calendar month whenever its choice stands and the channel has cooled to `SPONSOR_PROGRAMME_HEAT` — DERIVED, 0.18 − 0.012·(`BACKING_STEP`/`BACKING_DECAY`) = 0.06, the heat left when the last operation's money has run out (Operation Cyclone was funded every fiscal year 1980-92) — and draws nothing itself; the patron arm keeps its 0.022 draw (BUGS H-1). **R2, the annulment needs a HOSTILE army**: `annulment_check` returns `None` where `effective_army_loyalty >= ELECTORAL_COUP_ARMY` (0.35, no new constant; Algeria's ANP of January 1992, Jordan's never-annulled 1989 chamber, Turkey's 1997 memorandum); the Jordan assertion of the Algeria test re-expressed to it (H-2). **M2, pay per soldier in the Army target** (`government::army_pay_ratio` = `mil_spend_gdp · population / personnel_1990`, the ruled (milex/personnel)/(gdp/population) with the output cancelled; `army_loyalty_target(mil, exhaustion, pay_ratio, weight)`: `None` is the share arm's literal bit for bit, `Some(r)` blends `0.20 + 0.65·((1 − w)·share + w·pay) − 0.45·exhaustion` with pay the ratio's clamped position between `ARMY_PAY_UNPAID_AT` 1.0 — the ruling's "national average income" — and `ARMY_PAY_FULL_AT` 2.0, INVENTED and labelled; `ARMY_PAY_WEIGHT_RULED` 0.80 derived from the pillar line, `ARMY_PAY_WEIGHT` 0.0 SIZED BY MEASUREMENT: on the fetched 1990 figures the ruled ratio reads Sudan the best-paid army of the nineteen and Algeria's ANP as paid at any positive weight, because dividing by a poor country's income inverts Londregan and Poole's gradient; Powell 2012 and Londregan and Poole 1990 cited at the function; the arm applies only under the lens, the one place the lens changes a target; BUGS H-3 tables Powell's own variable for Ridge). **R1, D4 wired** (`government::polity_in` serves `D4_POLITIES` — Nepal May 1991, Haiti December 1990 — under the lens and `POLITIES` otherwise; the inertness test's government clause narrowed for those two nations as ruled, H-4). **R3(d), the successor flag** (`PartySpec.successor_of_ruling_party`, 24 sourced rows, `every_successor_flag_has_a_source`; A5 re-expressed to it). **R3(e), 1990 armed forces** (`military.personnel_1990` in 125 files, `milex_1990_usd_bn` in 118, World Bank MS.MIL.TOTL.P1 / MS.MIL.XPND.CD with the fallback year named, else the CIA Factbook; 12 nations refused to the share-only line, H-5). R3(a), (b), (c) — the successors' 1990 tables, Algeria's 1987 table, the seven regimes' families — are transcribed under `docs/political-arm/*-pending*` and NOT landed: each moves a golden or a pinned vector, three re-pin questions for Ridge (H-5). **THE CALIBRATION PASS (BUGS H-6) MOVED NO CONSTANT.** The census on this tree, N=200, 252 months, per seed min/med/max: A1 coups against elected governments 5/6/9 (top-3 share 1.00; Sao Tome 541, Philippines 447, Comoros 200 of 1252), annulments 0/1/1 (Algeria 105/200 alone — Belarus's and Ukraine's 200/200 are gone with R2); A2 1/200 by 2000 (the Islamist bloc is present in Kabul from month 0 and stops at I <= 0.383 against the 0.45 literal, coercion never failing at a 15% defence share); A3 117/200 = 0.585, Cambodia alone (2e164ae 197/200); A4 1/2/3; A5 0/200; A6 0 in 200/200 and 0 in 60/60 at 420 months; A7 0.148; A8 8/8 in 200/200; A9 3472/3472 = 1.000; A10 0.26/0.28/0.32. Every permitted move for an out-of-band anchor is either pinned by an existing test — the flat seed and the drift constants by `a_regime_s_movements_move_with_the_pains_and_revert` (0.5988, 0.65..0.80), the AI's round-table lines by `the_ai_takes_each_lever_only_under_its_thresholds`, the road lines by `every_road_reads_closed_while_takeover_is_off` — or short of the band by arithmetic on those pins; the historical bases fetched for the moves drafted (Benin 1989-90: "riots broke out when the regime did not have enough money to pay its army"; Zambia 1990-91: Kaunda's 24% against Chiluba's 75%; Cambodia 5 July 1997) are quoted in H-6 for the re-expression that would admit them. The four pinned bars' n and power are re-derived from this sample (A9 pooled 1.000, sd 0, sees a quarter of coups moving to rich nations and not a fifth; A10 0.279, sd 0.0103, n = 0.68, sees a seventh of the lead going and not a fourteenth) and re-watched red on a fresh build; the six ignored bars carry this tree's readings. **`ideology_takeover` stays OFF** — out: A1's concentration (1.00 against < 0.5), A2 (1/200 against a majority), A3 (0.585 against <= 0.10), A4 (2 against 15..40), A5 (0/200), A7 (0.148 against >= 3); inside: A1's band, A2's annulment sub-anchor, A6, A8, A9, A10, A12, A13.

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
# Current extension — daily ministry programs (2026-09-03, local review)

Approved design: `MINISTRY_BUDGET_DESIGN.md`. `Nation.program_budget` is optional
and absent from legacy saves. `SetProgramBudget` carries fiscal year, ten annual
GDP allocations and a 10×5 integer-basis-point matrix; every row totals 10,000.
Only explicit player enrollment activates the model. Deprecated coarse fiscal
commands cannot bypass an enrolled plan. Read-only previews never enact or pay.

One daily opening GDP/calendar basis releases service spending and capital
authorization. Capital consumers preflight authority, raw inputs, industrial
goods and work capacity. The tightest temporary input sets one throughput ratio;
work, every input draw, manufactured goods and cash all scale by that ratio.
Zero feasible throughput pauses without spending or losing completed work.
Hard blocks are reserved for structural failures such as invalid ownership,
contested ground or unavailable technology. All consumers share departmental
balances. End-of-day settlement posts actual
new spending once; previously expensed procurement funds are not billed twice.
Unused capital authority expires on fiscal-year rollover; renewal does not
duplicate accrual or erase unfinished physical work. Capital formation excludes
industrial running costs and reaches the existing macro channel next day.

`production.industry` holds opt-in manufactured goods, extended province sites,
project/mine financing and daily operations. Seven added construction kinds plus
existing estates/grids and mapped mine development provide ten Industry choices.
Processing and machinery consume the existing raw ledger and modeled power;
freight terminals improve real gateway capacity; warehouses hold manufactured
packs; automation/efficiency consume researched upgrades. No direct GDP writes.
UI prices, balances, recipes, progress, eligibility and effects come from Rust.

## Current extension — provincial GDP accounts (2026-09-03, local review)

Approved foundation: `PROVINCE_ECONOMY.md`. Browser worlds enable an optional
worldwide economic ledger; legacy/default worlds serialize no new ledger.
The existing national GDP is decomposed into eight modeled sectors and mapped
province accounts with an explicit unallocated remainder. These are estimates
anchored to the existing national data, not sourced historical local accounts.

Successful project work and physical operating flows post local value added.
New material and capital-goods packs use named model accounting prices, raw
inputs use the resource price table, and internal power is deducted by its
consumer and credited to its producer once. Empty enabling capacity and military
order placement do not fabricate production. Prior project output rates are
replaced each day, never accumulated into GDP as cash. An enrolled department
budget's explicitly represented capital does not additionally drive aggregate
public-investment growth; the inherited public-investment reference remains
with the background economy. Private investment remains a macroeconomic input.

Ownership changes reconcile local accounts to existing national transfers.
Save/resume retains the same estimates, receipts and timeline. Read-only
province and country endpoints serve full-precision figures for all owners;
the browser only formats them and discloses their modeled provenance.

## Current extension — inherited Materials operation (2026-09-04, integrated release)

Approved pilot: `MATERIALS_OPERATIONS.md`. Fresh campaigns with frozen inherited
industry can commission finite government toll-manufacturing orders. Commands
reserve estimated Materials capacity, never grant packs on signing. Production
consumes government raw stocks and actual conversion/energy department authority
after existing funded plants, sharing those plants' remaining generation, grids
and storage. Quantities are finite, fractional and deadline-bound. Scarce inputs
reduce delivery proportionally; an empty input pauses the order without changing
background national production, charging cash or losing its remaining quantity.

Actual Materials value-added receipts identify their already-inherited portion.
GDP settlement uses only the difference as additional output; the UI separately
reports observed output, inherited overlap, unobserved background and addition.
Internal generation uses the existing single fuel/power accounting route.
Finite domestic commitments cover near-term demand and require real power in
capacity planning, but cannot count as installed factories, stock or imports.
Economic Competition opponents use the same quoted, priced command path.

The subsequent approved AI integration (`MATERIALS_AI_INTEGRATION.md`) permits
a bounded 15-pack startup intention for the first machinery line, separate from
public demand. Owned inputs must back the full finite lot; a copied capacity
plan checks its power alongside the prospective machine. Current operating
authority and both political prices plus the existing reserve gate the paired
commands. Waiting for funding does not change the physical budget target.
Existing warehouse consumption and the startup lot net owned/imported/contracted
supply once. Immediate sale-policy updates protect the new consumer's packs;
reachable capital-goods imports avoid an unnecessary warehouse rescue machine.
Orders signed after the day's production settlement start service tomorrow,
so their stated delivery window contains the full number of workable dates.
Pre-settlement commands still start today; loading does not extend old deadlines.

The optional order ledger is absent from untouched legacy serialization. Existing
saves without inherited capacity are not silently reseeded. Default headless
rules and their baseline replay remain unchanged. Household demand, employment,
private-company profit and the other inherited manufacturing groups are later
extensions, not implied by this pilot.

## Current extension — AI industrial supply manager (2026-09-04, integrated release)

Approved layer: `AI_INDUSTRIAL_SUPPLY.md`. Opt-in Economic Competition governments
review Materials and Machinery supply every 30 days while daily work, production
and freight continue. A pure 90-day forecast counts remaining project goods
once, recurring operating and prototype use across the horizon, bounded startup
reserves, stock, paid inbound lots, finite domestic Materials contracts and only
positive output settled today or yesterday. It never calls a signature,
unfinished capacity or an estimate stock or production.

One replenishment action is limited to the 90-day gap, one review period of
recurring use plus one-off project and startup needs, and unclaimed storage.
Foreign purchases are additionally capped by an executable quote, treasury cash
and 0.1% of GDP, with at most one manufactured-goods import per review. Feasible
inherited Materials capacity precedes foreign trade; an accepted import ends the
review and invalidates the pre-purchase processor choice. A domestic finite lot
cannot exceed the complete unreserved raw bundle already owned; a one-day flow
quote is not treated as a month of inputs. Raw-market failure may start one
mapped mine, and storage expands only for positive demand with near-full
committed capacity. AI exports reserve project goods, committed startup inputs
and 30 days of recurring use. Once paid or actually produced supply begins
filling a prospective first-machine lot, at most 15 additional packs are
protected so partial lots cannot churn back into the market.

Every mutation uses the ordinary priced command path. The player receives a live
Rust forecast; world reporting reads each AI government's dated `supply_review`,
captured after its recorded action, so successful purchases and orders already
appear as incoming coverage. Exchange snapshots and price quotes require the
exact active campaign session, so an older tab fails closed rather than mixing
campaign ledgers. Forecasting alone is read-only and does not serialize a review
into legacy or default worlds. This layer adds no general raw-stockpile plan,
household or labor demand, free goods, guaranteed output, or AI authority over
the player's country. Descriptive six-year evidence and remaining limits are in
`AI_INDUSTRIAL_SUPPLY_RESULTS.md`.
