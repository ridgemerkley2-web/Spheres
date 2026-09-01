# SPHERES — standing context for Claude Code

Grand strategy simulation game in Rust. Deterministic world sim, Jan 1990 start,
history emerges from mechanics (no scripted events except proliferation dates).

**SPEC.md is the authoritative statement of what this game is** — vision,
pillars, architecture, and every system's intended design. Read it before
proposing anything structural.

- **BIBLE.md** — what the game IS and refuses to be, and how to decide when
  those conflict. The target is a better version of HOI4 Millennium Dawn.
  Read it first; it outranks everything below when they disagree.
- **SPEC.md** — the technical design.
- **PLAN.md** — the whole sequence from here to 1.0, ordered by what it costs to
  defer each piece. Start here when choosing what to work on next.
- **ROADMAP.md** — what is built, what is broken, what is immediately next.
- **CLAUDE.md** (this file) — how to work.

Two pillars from SPEC.md, and where each now stands:
- **Two spend-currencies spine the game: economic output and political
  capital.** Political capital now exists: every nation holds a stock, earns it
  by delivering growth and order and stable prices, loses it to war and
  recession, and every player command is priced against it. What is not done is
  the other half of the sentence — the AI systems do not buy with it yet,
  because `politics.rs` and `war.rs` move state directly rather than through the
  command queue. A new system should take political capital as a cost from the
  start rather than be retrofitted.
- **Spheres of influence is the namesake system**, and it is now built.
  `spheres-sim/src/statecraft.rs` holds it: mutual defence pacts with an upkeep
  both signatories pay, patronage as a standing transfer of the patron's output
  capped at what the Soviet Union actually disbursed, trade dependency that
  accumulates and then becomes leverage, and covert action that is deniable
  until it is not. Relations are no longer the whole of diplomacy.

  The two collisions that parked this branch for weeks were not statecraft's
  fault and were never fixed directly. They dissolved when the growth model
  underneath them was repaired: a trade-dependency asymmetry that read 8.5x
  against an expected 10x, and an Iraqi embargo that would not lift. Both were
  the base being wrong, not the system on top of it. Worth remembering the next
  time a branch looks like it needs its thresholds widened.

## Iron rules
1. **Determinism is sacred.** One RNG (SplitMix64 in `WorldState.rng`). Never add
   another RNG, never use HashMap iteration order for anything that affects state,
   never let wall-clock time touch the sim. `determinism_same_seed_same_world`
   and `save_load_roundtrip_continuity` must always pass.
2. **All state changes flow through the command queue** (`Command` in lib.rs) or
   the tick systems. No side doors.
3. **History is calibration, not script.** New mechanics are validated by
   emergent-history tests (see `gulf_war_emerges`, `ussr_collapses_in_the_nineties`,
   `china_growth_miracle`). Events should *usually* happen across seeds, not always.
4. **Starting data is transcribed, not invented** — real 1990 figures in
   `spheres-sim/data/`, each file carrying its own `sources` block. **This now
   includes what a nation KNOWS**: starting technology is authored and sourced
   per nation, not derived from `tfp_trend` (BIBLE §8, amended 2026-08-30).
   Unsourced is a refusal, not a default; and a granted 1990 stock must be
   reconciled against `tfp_base` so the same technology is not paid twice.
5. Run `cargo test` before considering any change done. Never delete a
   regression test to make a change pass, and never widen a tolerance to make
   one pass — a test that cannot fail is worse than no test. When you tighten
   or add a calibration test, check it goes red against the behaviour it is
   meant to catch. Several here were wide enough to admit a full point of
   annual growth without noticing. A bar also has to be asked of enough seeds to
   mean anything — see rule 7, which is the sampling half of this rule.
6. **Do not trust a green test you did not watch build.** `.cargo/config.toml`
   is now untracked precisely because of this: while it was tracked, every
   worktree under `.claude/worktrees/` built into the same `target-dir` as the
   main checkout, and cargo would hand back a test binary compiled from another
   branch's source — a passing suite that never ran your code. It produced two
   wrong readings before it was caught, in both directions. Still export a
   separate `CARGO_TARGET_DIR` per worktree, and if a result surprises you
   either way, confirm the binary is yours before believing it.
7. **A calibration bar must sample enough seeds to be worth believing.**
   Added 2026-08-31 on Ridge's ruling — the third of three settled that day,
   alongside the capital-channel repair and the re-pointing of the two
   mis-sampled conquest tests. Every bar that reads a STATISTIC across seeds —
   a count, a rate, a median — must sample enough of them that its probability
   of going red while the model is healthy is **under 1%**, and that number must
   be derived from **that test's own MEASURED per-seed variance**, not guessed
   and not inherited from a neighbouring test. **Record the required n in the
   test's comment beside the bar**, with the variance it came from, so the next
   session can re-derive it instead of taking it on trust; the honest way to
   carry that measurement is a wider `#[ignore]`d scan of the same quantity
   sitting beside the bar, the way `gulf_war_incidence_scan` and
   `conquest_size_rule_scan` sit beside theirs. For the bars that predate this
   rule, `spheres-sim/tests/sample_size_audit.rs` is the record: it measures
   every seed-sampling bar in the suite and its header carries the table as of
   2026-08-31. That is the stopgap, not the destination — **when you touch a bar
   for any reason, that is the moment to move its line out of the audit and into
   the comment beside it**, and a new bar carries its derivation from birth.

   The arithmetic, so nobody has to invent it. Most bars here read a per-seed
   Bernoulli event, whose variance IS p(1-p) once p is measured. For a bar that
   scales with the sample ("a majority of worlds"), n = (2.326·sd / margin)²,
   margin being the distance from the true rate to the bar. For a bar of "at
   least once", n = ln(0.01) / ln(1 - p). For a median or a band, bootstrap the
   measured per-seed sample; do not assume it is normal.

   **This does not apply to an INVARIANT** — "no dead nation holds districts",
   "a stable democracy never hyperinflates", "GDP stays finite". A universal
   claim cannot produce a false red from a small sample; it can only lose power,
   so seeds there are a budget question and not a correctness one. And note the
   trap in a test that asserts a universal claim per seed AND a statistic across
   them: raising n makes the per-seed arm STRICTER. Size that arm to what is
   true, not to what is large.

   **The other half of the same question is POWER**, and it is the half that
   cost this project the most. A sample can be quiet enough never to red falsely
   and still be blind: state the size of the regression the bar exists to catch,
   and check that the sample can actually see a move that big. A bar whose
   false-red probability is zero because the true rate is nowhere near it is not
   safe, it is decorative, and it should be recorded as such rather than
   believed.

   Iron rule 5 still binds on top of this: the repair for an under-sampled bar
   is **more seeds, never a wider bar**. Widening a bar because a wider sample
   crossed it is the thing this rule exists to stop. And a bar whose literal is
   tied to the sample size — "fewer than all twelve" — cannot be widened without
   re-expressing the bar, which is a decision for Ridge and not for the session
   that noticed it.

   WHY, measured 2026-08-31 rather than argued. Three of this suite's bars were
   read against their own variance and their false-red probabilities were **67%**
   (annexation asked of twenty seeds, when annexation is a ~2-4% per-seed event),
   **4.6%** (the Gulf War's majority bar asked of forty — `gulf_war_incidence_
   scan` re-derives 5.0% from its own p = 0.615, which is the same finding), and
   **37.6%** (China's thirty-year median asked of ten, on the pre-repair tree).
   A suite in that state does not fail loudly —
   it INVERTS. The same pass found China's thirty-year multiple had fallen
   14.29x to 11.07x, **22.5% of level and most of a point of annual growth**, and
   `china_growth_miracle` was GREEN throughout, because seeds 0..9 happened to be
   a +1.3% lucky draw. Two red tests that were reading noise, and one green test
   sitting on a real regression, all from the same defect.

## Layout
- `spheres-sim/` — the library. world.rs (state/RNG), init.rs (1990 data),
  economy.rs, war.rs, politics.rs (AI/events), lib.rs (commands, tick loop, tests).
  The tick order is the `SYSTEMS` table in lib.rs, and `century_run_profile`
  times exactly that table — so when you add a system, add it there and the
  profiler picks it up. Run it before optimising anything: the last two
  performance guesses this project made were both wrong.
- `spheres-sim/src/tech/` — the technology tree. mod.rs is the engine and the
  foundation set; the eight domain files beside it are data only. Productivity
  is scored against what the world on average knows, not added on top of the
  1990 trend, and convergence is the distance to the frontier. Tick order is
  economy -> tech -> war -> politics.
- `spheres-cli/` — `run` (headless), `play` (interactive), `resume`
- `spheres-web/` — local server + browser UI (`ui/index.html`, one self-contained
  file, no build step and no CDN). It owns no game logic: it holds one WorldState,
  routes player actions through the same `Command` queue, and renders. Keep it that
  way — the sim stays the single source of truth.

## Owner preferences
- Owner: Ridge. Wants playable results early, and wants to SEE the game — prefer
  work on the visible surface over deepening sim internals unless asked.
- The browser UI (`spheres-web`) is the primary game surface; the CLI remains for
  headless runs and quick calibration checks.
- Eventual goal: nightly autonomous dev sessions via cron on his Proxmox box,
  driven by this file + ROADMAP.md. Keep both current as work completes.
- The MODEL ticks monthly, and stays that way (v0.4 had hourly cadence;
  reintroduce only with a scheduler design, not by brute-forcing 720x more
  ticks). The CALENDAR may advance daily — BIBLE §5, amended 2026-09-01 — on
  the strict condition that a day-stepped month is bit-identical to a
  month-stepped one, commands included. A daily calendar is pacing; a daily
  system is the thing that is still refused.

See ROADMAP.md for what to build next.
