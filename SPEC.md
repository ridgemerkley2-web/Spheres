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
- **Hourly base tick + cadence scheduler** (full design): combat/movement hourly, markets daily, macro monthly, elections/tech yearly. *v0.5 rebuild runs monthly ticks only; reintroduce hourly cadence only with a scheduler design, never by brute force.*
- **State hashing** — FNV-1a hash of WorldState for determinism tests and replay verification.
- **Content as data** — nations and events in JSON files, two-pass validation, `deny_unknown_fields`. Moddable by design.
- **Engine-agnostic headless core** (`spheres-sim`) with no I/O; CLI runner (`spheres-cli`); Bevy reserved as the future view shell.
- **Starting data is transcribed, not invented** — real 1990 figures for GDP, population, oil output, debt, military.

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
- Budget: revenue = tax take + resource rents; spending = social (scales with democracy) + military + state investment.
- Real r−g debt dynamics: deficits add to debt/GDP; growth+inflation erode it; debt above ~90% GDP drags growth.
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
