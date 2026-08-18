# BUGS

Findings from the stability audit. Each entry carries the seed, the year and the
symptom, and — where the detector turned out to be firing on healthy behaviour —
what the investigation showed and why nothing was changed.

Reproduce the survey with:

```bash
cargo test --release -p spheres-sim anomaly_sweep -- --ignored --nocapture
```

## Method

Headless 40-year runs (480 monthly ticks), seeds 0..=20, 137 nations at the 1990
start rising to 158 as federations come apart. Every month, every living nation
was checked for: non-finite or non-positive GDP, non-finite inflation, negative
or non-finite military strength, non-finite population or political capital,
debt above 6.0x GDP, stability outside 0..=100, output above 100x or below 1% of
its 1990 figure, inflation sitting on either clamp for five years, stability
pinned at 0 or 100 for five years. The world was checked for a non-finite oil
price and for the price sitting on its $8 or $120 clamp for two years. Conflicts
were checked for dead belligerents, for single conflicts running 300 months, and
for dyads opening five or more separate conflicts.

## Result: the numbers are clean

**No numerical anomaly of any kind was found**, across 21 seeds, 40 years and
~137 nations a month: no NaN, no negative or runaway GDP, no debt spiral, no
inflation or stability pinned to a bound, no oil price at its clamp. The three
findings below are all structural, and two of them turned out to be healthy.

---

## Open bug, with a known site

### B-1 — a conflict that flares more often than every 18 months can never end

**Seeds:** 0, 1, 2, 3, 4, 5 and 22 more occurrences across 0..=20.
**Years:** first trips 2015; the conflicts themselves open in the early 1990s.
**Symptom:** single conflicts still open after 300+ months. On seed 0 at 2030 the
board still carries `Iraq/Kuwait 478mo` and `Iraq/Iran 475mo` — quarrels that
opened in 1990 and are still open forty years later. Six conflicts are open at
2030 on that seed.

**What the investigation showed.** These are not wars. Measured on seed 0, the
Iraq/Kuwait conflict is present for 479 of 480 months but **shooting in only 22
of them (5%), across 14 separate flare-ups**. `at_war` requires `shooting()`, so
no permanent war drag is being applied and no economy is being damaged — which is
why the numeric sweep is clean.

The defect is that the *exit ramp cannot be reached*, and the cause is not where
this entry first put it. `war.rs` ends a quarrel one of four ways: mutual
exhaustion above 0.75, a ripe settlement, an invasion verdict six months after
the guns stop, or — for everything else — freezing after 18 consecutive quiet
months and lapsing 42 months later at rung ≤ 1. For these dyads none of the first
three ever apply (`invasion_declared` is false throughout), so freezing is the
only way out, and `frozen_since` reads `false` at every sample from 1995 to 2030.

**CORRECTED.** This entry originally blamed the reset in `war.rs`, which zeroes
`quiet_months` on any month at the shooting rung. That cannot be the cause, and
the arithmetic says so: `quiet` is defined as exactly `!shooting()`, and
Iraq/Kuwait shoots in only 22 of 479 months, so the war.rs reset fires 22 times
in forty years and could never hold the counter at the 1-4 the samples show.

The real site is **`commitment.rs:159-160`**, inside `set_commitment`:

```rust
if rung > old {
    c.quiet_months = 0;
    c.frozen_since = None;
}
```

Any **upward rung change** resets the freeze clock — a nudge from rung 2 to 3,
nowhere near the shooting rung of 6. The comment beside it explains the intent,
and the intent is sound: "the freeze clock must not be allowed to kill a climb
halfway up: eighteen months is less than it takes a poor government to save the
standing for seven rungs." But the effect is unbounded. A quarrel where anybody
so much as shuffles a rung upward every year or so can never accumulate 18
consecutive quiet months, and the entire freeze/lapse subsystem — twenty-odd
lines, with its own headline — is dead code for that dyad forever.

**What the correction changes.** The original diagnosis made this a redesign of
the ladder's exit conditions, which is why it was logged rather than fixed. The
real cause makes it far smaller: the climb-reset is protecting a climb, and a
climb takes months, not years. Bounding it — resetting only for the duration a
climb plausibly needs, or decaying the counter rather than zeroing it — leaves
the stated intent intact while letting a quarrel nobody is prosecuting fall off
the board as the design already says it should.

It is still not a one-liner to land safely: it changes how long conflicts stay
on the board, which moves war calibration across the suite and both golden
hashes. It wants its own branch, a failing test first, and a multi-seed
re-measurement. But it is a bug with a known site and a bounded fix, not an open
design question.

**Not yet addressed:** a related finding from the direction audit is that
`dyads.rs:172` returns `0.0` war appetite for any pair with a pact between them
("A guarantee is not a modifier, it is a bar"), and both nuclear status and pacts
spread monotonically over a run. So the set of dyads that can ever produce a war
shrinks with time. That is a *starting*-layer constraint and this entry is an
*ending*-layer one; fixing B-1 changes how the existing quarrels end, not how
many exist.

---

## Investigated, not a bug

### B-2 — a dissolved nation is listed in a live conflict for exactly one tick

**Seeds:** 3, 8, 11, 12, 16, 18, 20. **Year:** 1993 in every case, the month the
USSR dissolves. **Symptom:** the detector for "dead nations still acting" fires
on the USSR being listed in a live conflict's belligerent list while `alive` is
false.

**What the investigation showed.** Measured longest run: **1 tick**, on every
seed that trips it. The tick order is economy → tech → war → politics.
`war.rs:400` prunes dead belligerents with `side_a.retain(...)`, and then
`politics.rs` dissolves the USSR later in the *same* tick, leaving the corpse
listed until the next tick's war phase prunes it. It never acts while listed:
strength lookups return 0 for a dead nation (`war.rs:125`).

So nothing is acting, and the inconsistency heals itself in one month. The only
reachable consequence is that a save serialised in exactly that month carries a
dead belligerent, which the browser UI would render for one frame. Not worth
touching the dissolution path for, which is the riskiest code in the sim.

### B-3 — "repeat war" dyads are repeat quarrels, not repeat invasions

**Seeds:** 2, 12, 16 (Iraq/Syria), 7 (Iran/Afghanistan). **Year:** by 2030.
**Symptom:** the same pair opening five separate conflicts in forty years, which
looks like the burned-hand flag failing to stick.

**What the investigation showed.** On seed 2, the four Iraq/Syria conflicts
opened 1995-10, 2008-07, 2013-01 and 2021-06 with **peak rungs of 2, 2, 4 and
3** — none of them reached an invasion, `invasion_declared` is false on all four,
and `burned_Iraq_Syria` is correctly never set. The burned-hand flag is
specifically about invasions (`war.rs:741`), and no invasion happened, so it
behaved exactly as specified. Four diplomatic crises between Iraq and Syria in
forty years is not an absurdity. The detector's threshold was naive.

---

## Observed while adding coverage

### O-1 — conquest is very nearly unreachable

Measured across twelve seeds and forty years, the only nations that ever leave
the board are **Yugoslavia (1991) and the USSR (1993) — the two modelled
dissolutions, in every single seed — plus exactly one conquest: Finland, seed 9,
2013, at 5.3m people.** One annexation in roughly 480 nation-centuries.

That is not a bug, and it may well be the intended shape: the coalition response
exists to save Kuwait, `desert_storm_is_quick_when_they_stand_and_fight` asserts
Kuwait survives, and `conquer` requires control saturated *and* rung 8 *and* the
defender's resolve spent, which is a demanding conjunction. But a world where
borders effectively never change over forty years is worth knowing about
deliberately rather than discovering later, and it is why
`a_large_nation_is_subjugated_rather_than_swallowed` is pinned to the one seed
that exercises the branch: a broad sweep would assert almost nothing while
costing minutes.

If that test ever fails on its non-vacuity guard, conquest has stopped happening
altogether, and that is the finding rather than a flaky test.

## Documentation drift found while auditing

Not bugs in the sim, recorded here because the audit is where they surfaced:

- README.md claims "24 nations at the start and up to 30 once federations come
  apart" and "46 calibration/invariant tests". Measured: **137 nations at the
  1990 start, peaking at 158**, and 98 sim + 13 web tests.
- README.md describes `feat/statecraft` as written but unmerged. `statecraft.rs`
  is in the tree and its commands are priced.
- SPEC.md section 9 still describes the v0.5 roster as 16 nations.
