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

  Phase 2.2 added the stock underneath it, in `influence.rs`: a patron's
  position in a client decays every month, the four instruments feed that one
  number rather than standing alone, alignments resist and then flip hard, and
  holding a sphere is a standing bill in political capital that a weak government
  cannot pay. January 1990 opens with the real board, derived from the
  transcribed relations matrix. Both surfaces reach it.

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
4. **Starting data is transcribed, not invented** — real 1990 figures in `init.rs`.
5. Run `cargo test` before considering any change done. Never delete a
   regression test to make a change pass, and never widen a tolerance to make
   one pass — a test that cannot fail is worse than no test. When you tighten
   or add a calibration test, check it goes red against the behaviour it is
   meant to catch. Several here were wide enough to admit a full point of
   annual growth without noticing.
6. **Do not trust a green test you did not watch build.** `.cargo/config.toml`
   is now untracked precisely because of this: while it was tracked, every
   worktree under `.claude/worktrees/` built into the same `target-dir` as the
   main checkout, and cargo would hand back a test binary compiled from another
   branch's source — a passing suite that never ran your code. It produced two
   wrong readings before it was caught, in both directions. Still export a
   separate `CARGO_TARGET_DIR` per worktree, and if a result surprises you
   either way, confirm the binary is yours before believing it.

## Layout
- `spheres-sim/` — the library. world.rs (state/RNG), init.rs (1990 data),
  economy.rs, war.rs, politics.rs (AI/events), lib.rs (commands, tick loop, tests)
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
- Monthly ticks for now (v0.4 had hourly cadence; reintroduce only with a
  scheduler design, not by brute-forcing 720x more ticks).

See ROADMAP.md for what to build next.
