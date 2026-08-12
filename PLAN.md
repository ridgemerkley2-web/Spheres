# SPHERES — the road to 1.0

*SPEC.md is what the game is. ROADMAP.md is what is built and what is next.
This is the whole sequence between here and done, and why it is in this order.*

**Ordering principle: rework-if-deferred, not interest.** A foundation that
every later system must be rewritten around is worth more than ten features.
Every phase below is placed by what it would cost to land it *later*, and each
one states that cost explicitly. Where a phase is out of order against what
would be more fun to build, that is deliberate.

**Definition of 1.0.** A single-player grand-strategy game, January 1990 start,
that a stranger can install and play for four hours without reading source: a
world of roughly 190 data-defined nations, an economy and a military both deep
enough to reward real intuition, spheres of influence as the central contest,
history that emerges rather than fires, and a nightly autonomous build that
keeps it honest. Everything below serves that sentence.

---

## Phase 0 — Green and honest

*Days. Blocks nothing structurally, but everything after it is measured against
a baseline, and the baseline has to be trustworthy.*

| | |
|---|---|
| 0.1 | Diagnose `embargoes_eventually_lift` and merge `fix/trade-level-effect`. The trade fix is the largest calibration correction found so far — USA 2025 from $30.8tn to $16.5tn against a real ~$14tn — and it is sitting on a branch because it re-breaks that test. Establish whether AI statecraft legitimately renews the grievance (in which case the test predates the mechanic and should be re-expressed) or whether relief is genuinely broken. **Do not widen the test to fit.** |
| 0.2 | **Add the guard that was missing.** No test locks a frontier economy's long-run trajectory, which is why a world compounding at twice reality passed every calibration test we had. Assert the USA's 35-year path stays inside a band around ~2.5%/yr. This one test would have caught the trade bug immediately. |
| 0.3 | Japan's residual. Even after the trade fix Japan is roughly twice its real size. A second cause is outstanding. |
| 0.4 | Golden state hash: commit `state_hash` of a known seed at a known month and assert against it, so a determinism break is a red test rather than a discovery. |
| 0.5 | Housekeeping: `.cargo/config.toml` is tracked, so every worktree builds into one `target-dir` and cargo will hand you a binary built from another branch's source. It has already produced false readings in both directions. Untrack it or make it per-worktree. Then `git worktree prune`. |
| 0.6 | Pick between the rival `feat/tech-biotech` and `feat/tech-transport` implementations, or merge the best of each. |

**Exit:** master green; a calibration test exists that fails against the
pre-0.1 behaviour; a golden hash is pinned.

---

## Phase 1 — Foundations

*The expensive-to-defer work. Nothing here is visible to a player, and all of it
gets dramatically more expensive after the roster grows.*

### 1.1 Nations become data, and ids become runtime — **XL, critical path**
`NationId` is a closed Rust enum with hand-written `name()`/`parse()` arms,
fixed-size roster arrays, and per-dyad `match` tables for war appetite. At 190
nations that is ~380 match arms and ~36,000 ordered dyads, which cannot be a
match statement at all — the appetite table has to become a *derived* function
of adjacency, relations and claims.

Moving nations to JSON only helps if the id becomes a runtime value (interned
string or slot index) at the same time; otherwise the enum still needs 190
variants. These are one change, not two.

*Deferred cost:* every system that pattern-matches on `NationId` is downstream.
The next hand-written roster expansion will work fine and make this strictly
larger without revealing the wall.

### 1.2 Relations as a dense matrix — **S, do it with 1.1**
`relation()` is a linear scan over a `Vec`, and `tech::absorptive_capacity`
calls it inside a per-nation loop. That is O(n⁴) per tick: invisible at 24
nations, roughly 646M comparisons per month at 190. `nation()`/`nation_mut()`
are linear scans too, called in every hot loop.

*Deferred cost:* small now; large once finance and trade also read the matrix,
and it will present as "the roster is slow" and be debugged in the wrong file.

### 1.3 Cross-platform determinism — **M**
`exp`, `powf` and `ln` are not IEEE-exact across platforms, and the tech module
multiplied the exposure to ~15 `exp` calls per nation per month. Both
determinism tests build two worlds in one process with one libm, so neither can
ever catch it. The endgame is developing on Windows while a Linux box runs
tests nightly — this is precisely the failure that produces a red test you
cannot reproduce locally.

Either replace the soft-cap shapes with rational/`powi` equivalents, or accept
platform-pinning and assert the golden hash per platform in CI.

### 1.4 Century-run CI — **S**
The risk register's top entry is 200 AI economies spiralling, and its stated
mitigation is headless century-runs as invariants. Cheap now, and it is the
thing that catches the next trade-shaped bug automatically.

**Exit:** a 190-nation smoke world ticks 100 years inside CI's patience; golden
hashes match across Windows and Linux; nations load from JSON with
`deny_unknown_fields` and two-pass validation.

---

## Phase 2 — Finish the declared spine

*SPEC pillar 2 says two currencies spine the game and every system is a buyer of
one or both. Half of that is true today.*

### 2.1 The AI buys with political capital — **M**
Every player command is priced. The AI is not, because `politics.rs` and
`war.rs` move state directly instead of going through the command queue —
which is also a standing violation of architecture rule 2, "no side doors".
Routing AI fiscal, monetary and above all war decisions through the same
pricing is what makes the currency bite for everyone. It will move emergent
history, so it needs its own calibration pass.

### 2.2 Influence projection — **L, the namesake**
Statecraft built pacts, patronage, trade dependency and covert action. What
SPEC still describes and the code lacks is *influence as a stock*: great powers
projecting it, decay as upkeep, a spend-to-hold economy, and alliance
hysteresis so alignments resist flipping and then flip hard. This is the game's
title and its central contest.

### 2.3 Finance and contagion — **M, currently blocked on data**
`feat/financial-system` exists but `WorldState.finance` covers only the
original 16 nations and `fin()` panics on the rest. Needs transcribed 1990
balance sheets for the newer nations and one created for Ukraine at birth.
After 1.1 this is data authoring, not code.

**Exit:** an Asia-1997-shaped crisis emerges across seeds without being
scripted; the superpowers compete for the same clients under a real budget and
sometimes cannot afford to.

---

## Phase 3 — The world

*Cheap after Phase 1. Ruinous before it.*

- **3.1** Roster to ~190 from data files. Transcription work, parallelises well.
- **3.2** Derived dyads: war appetite from adjacency, claims, ethnic overlap and
  relations, replacing the literal table.
- **3.3** Trade network proper: gravity model with sphere-alignment bonuses,
  dependency feeding diplomatic receptivity, and chokepoints — Hormuz, Suez,
  Malacca — as things a war can close.

**Exit:** ~190 nations, century run green in CI, every existing calibration test
still passing at its original threshold.

---

## Phase 4 — Military depth

*The other maximum-depth pillar, and the largest single body of work in the plan.*

- **4.1** Province map and adjacency as data.
- **4.2** Divisions, supply flow network, operational combat. This is where the
  hourly cadence scheduler finally earns its place — combat and movement hourly,
  markets daily, macro monthly, elections and tech yearly. Reintroduce it as a
  scheduler, never by brute-forcing 720× the ticks.
- **4.3** Statistical combat testing and deterministic replays.

**Exit:** the Gulf War emerges at operational scale — a coalition assembles,
supply constrains the advance, and the result is not predetermined by a strength
ratio.

*Honest note: this phase is comparable in size to everything before it. It is
also the most cuttable — a 1.0 that keeps the current abstract war model and
ships the rest is a real game. Decide deliberately rather than by drift.*

---

## Phase 5 — The game people see

- **5.1** The tech tree is invisible. 253 technologies and the UI does not
  mention one. Largest gap between what the sim knows and what the screen shows.
- **5.2** Map depth: borders that redraw on dissolution and conquest, influence
  shading, war fronts, and the tech frontier as a visible thing.
- **5.3** Advisors, event prompts with real choices, difficulty through
  `GameRules`.
- **5.4** The engine decision: keep the self-contained browser UI, or move to
  Bevy as SPEC reserves. Defer until 5.1–5.3 have shown what the UI actually
  needs; do not pay for an engine port before then.

**Exit:** a stranger plays 1990 to 2010 without reading source and can explain
afterwards why the world went the way it did.

---

## Phase 6 — Narrative and autonomy

- **6.1** Chronicle system: template combinatorics over what the sim already
  generates. The drama is real; this narrates it.
- **6.2** Ollama flavour and AI leader personality bias. **Never in the tick
  loop, never a mechanical decision** — pillar 6, and it is the one that
  silently destroys determinism if violated.
- **6.3** Nightly autonomous sessions on the Proxmox box: `claude -p` driven by
  CLAUDE.md, SPEC.md and ROADMAP.md, tests as the gate, commits as the log.

**Exit:** unattended nightly runs that produce green commits and stop when they
cannot.

---

## Critical path

```
0.1-0.2  →  1.1 + 1.2  →  1.3 + 1.4  →  3.1  →  2.2  →  5.1-5.3  →  6.3
            (data+ids)     (CI gates)   (roster)  (spheres)  (visible)  (autonomy)
```

2.1 and 2.3 can run beside 3.1 once 1.1 lands. Phase 4 hangs off the critical
path entirely and can start any time after 1.4 — or be cut.

## What I would cut under time pressure

1. **Phase 4's operational layer.** Keep abstract war. It is the single largest
   cost and the current model already produces good history.
2. **Bevy (5.4).** The browser UI is self-contained, has no build step, and is
   already the primary surface.
3. **190 nations → 60.** The G20 plus every regional actor that matters gets
   most of the texture at a third of the transcription.

## What I would not cut, at any pressure

Determinism, the test gate, and history-as-calibration. They are why this
codebase can be worked on by agents overnight without turning to mush — and the
three bugs found this week (registry indices in saves, the intervention-list
mismatch, trade paying a growth rate) were each caught by exactly that
discipline.
