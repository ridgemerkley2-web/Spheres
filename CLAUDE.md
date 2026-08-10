# SPHERES — standing context for Claude Code

Grand strategy simulation game in Rust. Deterministic world sim, Jan 1990 start,
history emerges from mechanics (no scripted events except proliferation dates).

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
   regression test to make a change pass.

## Layout
- `spheres-sim/` — the library. world.rs (state/RNG), init.rs (1990 data),
  economy.rs, war.rs, politics.rs (AI/events), lib.rs (commands, tick loop, tests)
- `spheres-cli/` — `run` (headless), `play` (interactive), `resume`

## Owner preferences
- Owner: Ridge. Wants playable results early; CLI play mode is the current game surface.
- Eventual goal: nightly autonomous dev sessions via cron on his Proxmox box,
  driven by this file + ROADMAP.md. Keep both current as work completes.
- Monthly ticks for now (v0.4 had hourly cadence; reintroduce only with a
  scheduler design, not by brute-forcing 720x more ticks).

See ROADMAP.md for what to build next.
