# SPHERES — standing context for Claude Code

Grand strategy simulation game in Rust. Deterministic world sim, Jan 1990 start,
history emerges from mechanics (no scripted events except proliferation dates).

**SPEC.md is the authoritative statement of what this game is** — vision,
pillars, architecture, and every system's intended design. Read it before
proposing anything structural. This file is how to work; SPEC.md is what to
build; ROADMAP.md is what is built and what is next.

Two pillars from SPEC.md that the current code does NOT yet honour, and that
any new work must not contradict:
- **Two spend-currencies spine the game: economic output and political
  capital.** Political capital does not exist in the code yet. Every system is
  meant to be a buyer of one or both.
- **Spheres of influence is the namesake system.** Influence projection —
  great powers spending to hold clients, influence decaying as upkeep — is
  still only a bare relations matrix. `feat/statecraft` implements it and is
  green on its own branch, but collides with the expanded roster; see ROADMAP.

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
   is tracked, so every worktree under `.claude/worktrees/` builds into the
   same `target-dir` as the main checkout, and OneDrive resets mtimes often
   enough that cargo will reuse a test binary compiled from a different
   branch's source. That reads as a passing suite that never ran your code.
   Export a separate `CARGO_TARGET_DIR` when testing a worktree, and if a
   result is surprising in either direction, confirm the binary is yours
   before believing it.

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
