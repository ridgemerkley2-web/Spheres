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

### E-1 — the 1990 endowment IS paid twice, through `adoption`, and the rebase cannot see it — **FIXED 2026-08-31 on Ridge's ruling 1**

**FIX, and what it cost.** `rebase_to_transcribed` now neutralises the third term
as well. It records `tech_1990_deficit` — how far behind the January 1990
frontier the transcription left the nation, in technologies — and `apply_bonuses`
measures the convergence gap net of it, so a nation is paid convergence for
ground lost or won against the frontier *after* the transcription and for
nothing else. Three details carry the whole result and each was measured:

  * **The credit is scaled by the nation's transcribed 1990 income**, through the
    `development` proxy the module already uses. The raw shortfall asserts the
    nation HELD the technology and the file merely failed to list it — true of
    Belgium at two authored entries, false of China at five. Uncapped, China's
    thirty-year multiple falls **11.92x -> 9.05x** and `china_growth_miracle`
    goes red; scaled, it holds at 11.64x.
  * **The credit is consumed.** Held rather than spent, a sixteen-technology
    deficit excuses sixteen unlocks *every month for thirty years*; that read
    China at 10.48x on a credit of 1.6 technologies.
  * **A revelation is not an acquisition.** While the credit is open, a
    1990-vintage unlock is stock the transcribed trend already prices, so it
    does not count as absorption and its productivity increment is folded into
    `tfp_1990_offset` rather than into the trend. Without this last piece the
    test lands at 1.97e-4 against a 1.0e-4 bar — the gap was neutral and the
    productivity channel was not.

**MEASURED after, twelve months, worst over the twenty granted:** 9.4e-5
(Belgium) against the untouched 1.0e-4 bar, from 3.27e-3. The ragged edge this
entry's own note said the test did not claim is closed too: Finland 1.03e-2 ->
5.9e-5, Austria 7.4e-3 -> 5.9e-5, Denmark 6.5e-3 -> 5.9e-5, Norway 2.9e-3 ->
5.9e-5. What is left, ~6e-5 on every nation on the board, is the world reference
drifting differently in two worlds that hold different technology, and it is not
a double count.

**Calibration:** Spearman rho against reality 0.886 -> **0.943**, max mature
error 1.05 -> 0.84, all four ordering clauses hold, `the_frontier_does_not_run_away`
green with 1.97 points of margin, `china_growth_miracle` green at 11.64x. Both
goldens moved again and are still un-repinned — see the note below.

The original diagnosis is kept below, because the fix is only legible against it.

**Found:** 2026-08-31, verifying the economy fix. **Test:**
`tech::tests::the_1990_endowment_does_not_move_year_one_growth`, RED.
**Symptom, quoted:**

```
Japan was paid twice for its 1990 technology:
growth 0.007238 granted against 0.010512 ungranted
```

3.27e-3 against a bar of 1.0e-4 — **32x the bar**, and 180x the 1.83e-5 (Canada)
that the test's own comment records as the worst reading when it was written.

**The diagnosis is closed by construction, not by argument.** `apply_bonuses`
(`tech/mod.rs:1555`) assembles

```rust
n.tfp_trend = n.tech.tfp_base + (tech_tfp - reference) + adoption.min(ADOPTION_MAX);
```

`rebase_to_transcribed` neutralises the first two terms and **only** those two.
Its guard test, `granting_the_1990_stock_does_not_move_the_transcribed_trend`,
is GREEN with a worst residual of 3.47e-18 — so the static assembly is exact to
f64 noise. A world that is exact at t=0 and 32x out at t=12mo has diverged
through a term the rebase does not cover, and **`adoption` is the only other
term in the sum.**

The mechanism, on Japan: `adoption = ADOPTION_PER_TECH * absorption_rate *
gap^TACIT`, and `gap` is the distance to `frontier_known`. In the *granted* world
Japan holds the whole 1990 pool, so `gap` is 0 and `adoption` is 0. In the
*control* world Japan must acquire, so `gap` > 0 and it is paid `adoption` for
re-learning what it was refused. Granted grows *slower* than ungranted, which is
the direction observed.

**This is pre-existing, and the economy fix un-masked it rather than caused it.**
The test's own comment already names this exact mechanism — "a rich, open economy
denied the endowment sits at `gap ~ 1` ... and is paid up to ADOPTION_MAX =
4.5pp/yr for re-learning what it was refused" — but attributes it only to nations
*outside* the endowment boundary. It reaches inside too.

What changed is an accidental cancellation. The frontier-reversion predicate in
`economy.rs:244` was rewired by the economy fix:

```rust
-        if dev >= 1.0 && n.tech.tfp_base > FRONTIER_TFP {
-            n.tech.tfp_base += (FRONTIER_TFP - n.tech.tfp_base) * 0.008;
+        if dev >= 1.0 && n.tfp_trend > FRONTIER_TFP {
+            n.tech.tfp_base += (FRONTIER_TFP - n.tfp_trend) * 0.008;
```

The old predicate read `tfp_base`, which the rebase drives *down* for a granted
nation — so the reversion silently switched OFF in the granted world and ON in
the control, applying a downward pull to the control that happened to offset the
control's extra `adoption`. The new predicate is symmetric between the two
worlds, which is correct on its own terms and is well-argued in the comment
beside it — and removing the accidental offset let the real non-neutrality show.

**Not fixed here, and the reason is scope.** The rewired reversion line is right;
the defect is that `adoption` sits outside the rebase. Closing it means either
rebasing `adoption` too or making the endowment's edge non-ragged, and either
moves every calibration number in the suite and both goldens. It wants its own
branch, and it blocks the hash re-pin until it is ruled on.

*Ridge ruled on 2026-08-31: rebase adoption. Done, above. The reversion line at
`economy.rs:244` was left exactly as it stands — the accidental cancellation was
not restored, and the real non-neutrality was closed instead.*

---

### E-2 — the capital repair spent E-1's remaining headroom, and `the_1990_endowment_does_not_move_year_one_growth` is RED

**RED on the working tree, GREEN at `git HEAD`. Found 2026-08-31 in the final
verification pass, and it is the blocker on the golden re-pin.**

```
Belgium was paid twice for its 1990 technology:
  growth 0.001851 granted against 0.001749 ungranted     (tech/mod.rs:2216)
```

**The margin, measured on both trees with the same instrument**
(`spheres-sim/tests/endowment_margin_probe.rs`, which replicates the tracked A/B
exactly and prints what the test only asserts):

| tree | worst dgrowth (Belgium) | against the 1.0e-4 bar |
|---|---|---|
| `git HEAD` (eb7de26) | 9.8823e-5 | **98.8%** — green by 1.2% |
| working tree, capital repair applied | 1.0218e-4 | **102.2%** — red by 2.2% |

The `dgdp` arm is not close and is not the complaint: 1.0461e-4 against its own
2.0e-4 bar, 52.3% of it.

**ATTRIBUTION, measured rather than reasoned.** `spheres-sim/src/economy.rs` from
the working tree was copied onto an otherwise pristine `git HEAD` worktree
(separate `CARGO_TARGET_DIR`, iron rule 6) and the test rebuilt and re-run. It
reproduces the red **to the digit** — `0.001851` granted against `0.001749`
ungranted. So the cause is the capital channel repair alone; `lib.rs`'s
re-pointed tests are not involved, and independently could not be, since two
30-year headless runs on the repaired tree are byte-identical to the run taken
before those edits (md5 `f8ba3471388bfcf2a7456d0229ec4ed4`).

**MECHANISM, and why this is a sensitivity rather than a new double-payment.**
The endowment perturbs a nation's `development` by a hair through its GDP path,
and the capital rate arm is gated by `(1 - dev)`, so whatever residual
non-neutrality survives E-1 is *amplified in proportion to the size of that arm*.
For Belgium the arm roughly doubles under the repair — at `s = 0.2460` the old
`(s/0.20)^0.55 · 0.20 · 0.080 · (1-dev)` pays 0.256 pt/yr and the new
`(s - 0.125)·(0.20/0.075) · 0.080 · (1-dev)` pays 0.368 pt/yr, +44% — and
Belgium's residual moves +3.4% of the bar with it. Nothing is being paid twice
that was not being paid twice before; E-1 closed the channel to within 1.2% of
its bar and left no room for any downstream term to grow.

**NOT FIXED HERE, and the reason is doctrine, not difficulty.** Three repairs are
available and every one of them is the owner's call:

1. **Rebase `adoption` the rest of the way** — E-1's own "not fixed here" note
   says the ragged-edge cliff is `adoption` sitting outside the rebase, and the
   residual measured here is what is left of exactly that. This is the honest
   fix and it moves both goldens.
2. **Widen the 1.0e-4 bar.** Forbidden by iron rule 5 and named here only so the
   next session does not have to rediscover that it is forbidden. The bar is a
   neutrality claim, not a calibration tolerance: it says the endowment is
   invisible to year-one growth, and 1.0e-4 is already generous against a
   quantity that is meant to be zero.
3. **Trim the capital rate arm** until Belgium clears. This is fitting a
   production coefficient to a test, which is the act PLAN's point (C) forbids,
   and it would give back the China repair it was written for.

**Related, and the reason this entry sits under E-1 rather than under T-:** the
same amplification is why the arm should be read as a *sensitivity amplifier* for
every A/B in the suite. Any future term gated on `dev` or on the investment share
will move this residual again.

---

## Awaiting an owner ruling

### T-3 — two conquest tests lost their sample, not their bar

**RED, and new with the E-1 fix.** `a_large_nation_is_subjugated_rather_than_swallowed`
and `a_dead_nation_holds_no_districts` both fail on their **non-vacuity guard**,
not on anything they assert:

```
no conquest anywhere in twenty seeds of forty years, so the size rule was never
exercised — conquest may have become unreachable (BUGS.md O-1)
```

**The bar is untouched and still true.** Neither test found an annexation over
8m people; neither found an annexation at all. `conquest_seed_scan` was re-run
as both tests instruct, over seeds 0..120 and forty years:

```
war                      : 1 annexation  — seed 45, Luxembourg at 0.41m in 2014
control (ai_aggression 0): 0 annexations
```

So conquest is **not** unreachable — O-1's own correction records 10 `Ending::Conquest`
in 40 seeds, and the rare event is annexation, at 1-3 in 120 seeds. The E-1 fix
changes growth, growth changes which wars happen, and the one annexation inside
seeds 0..20 moved out of the window. O-1 already says 3-in-120 and 1-in-120 "are
single-digit Poisson counts and are not distinguishable evidence".

**The remedy the tests themselves authorise, not taken here.** Both comments say
the width "is a sampling choice and not a band: widen it if the branch stops
being reached", and seed 45 is the seed that would restore it. That is still a
change to a test made by the agent whose change turned it red, so iron rule 5
sends it here: **widening 0..20 to a window containing seed 45 is the owner's
call, not an agent's.** Nothing was touched.

**RECURRED 2026-08-31 with the `BUILD_KNEE` repair, and the recurrence is the
strongest evidence yet that the window is the problem.** Both tests were GREEN
again on the tree this pass started from — seeds 12 and 22 carried annexations,
of which seed 12 sat inside 0..20 — and raising the diffusion knee turned them
red once more. `conquest_seed_scan` was re-run over 0..120 on BOTH trees:

```
knee 0.004 (before) : 5 annexations — seeds 12, 22, 55, 89, 100
knee 0.008 (after)  : 2 annexations — seeds 46, 84
control (aggr 0)    : 0 on both
```

So conquest is reachable on both trees and annexation is rare on both. 5 and 2
in 120 seeds are the same single-digit Poisson counts O-1 already refuses to
treat as distinguishable evidence. **The window contained exactly ONE hit before
the change, so the tests hung on one seed and were a coin flip whatever anyone
did to the model.** Any change at all reshuffles which seeds annex: this is a
deterministic sim, so a perturbation of any size decorrelates a forty-year run.
That means these two tests will flip colour on roughly half of all future
changes until the window is widened, and widening it is still the owner's call.
Nothing was touched here either.

**RE-VERIFIED 2026-08-31 by the audit pass, on binaries rebuilt from scratch**
(every test `.exe` post-dating a full source touch, iron rule 6). Independent
re-run of `conquest_seed_scan` over seeds 0..120, forty years, both arms:

```
war                      : 2 annexations
    seed  46   Mongolia at 5.27m in 2025
    seed  84   Mongolia at 5.87m in 2029
control (ai_aggression 0): 0 annexations
```

This reproduces the previous pass's scan exactly (seeds 46 and 84), which is
itself worth recording: two agents, two builds, the same two seeds. **And it
settles what the tests would say if they reached the branch — both annexations
are of a nation at ~5m people, comfortably under the 8m rule, so `pop < 8.0`
holds every single time it is exercised.** These two tests are not failing
because the model breaks the rule they assert. They are failing because a
twenty-seed window contains no conquest to check the rule against.

**What the owner is actually being asked to rule on.** Not "is the model wrong"
— the evidence says it is not. It is: a non-vacuity guard whose sample is too
small to be reached will keep flipping colour forever, and the two candidate
remedies are (a) widen the window to include a seed that annexes, or (b) accept
that annexation is rare enough that these two tests should ride a dedicated
seed list rather than a range. Either is a test change made in response to a
red, so neither is an agent's call.

### T-1 — `a_trade_agreement_lifts_the_smaller_partner_and_then_binds_it` measures a different quantity now

**RED.** Median lift 1.233 → 1.092 against an unsourced bar of 1.20.

**The property the test is named for is algebraically intact.** With a single
pact the old and new rules are identical, both paying
`0.25 * depth * theirs / (mine + theirs)`. What changed is the *quantity*: Poland
holds other agreements in the control arm, so the test now reads the MARGINAL
value of one more pact, not the standalone value. The test's own comment already
records that 1.20 "WAS A COIN FLIP WEARING A THRESHOLD" and is unsourced.

**Recommendation, not done:** change the test's CONSTRUCTION so Poland is
otherwise unopen. That makes it measure what its name claims and keeps 1.20
intact. Untouched pending the ruling — iron rule 5 gives the owner this call.

**CLEARED 2026-08-31 — the ruling came back "do it", and the diagnosis above
held under its own check.** The control arm's Poland finished holding
Czechoslovakia and Hungary with a `trade_level_paid` of 0.079 .. 0.137 against
the treated arm's 0.246, and 1.246/1.11 = 1.12 is the 1.107 that was printing.
`run_warsaw_unopen` now strikes out every Warsaw agreement but the one under
test, in BOTH arms — `statecraft` ticks before `politics`, so a pact the AI
signs in month M is struck before `trade_level_gain` ever sees it, and the
control arm's `trade_level_paid` is `None`, i.e. no leakage. **The bar is
untouched at 1.20.** Median 1.222, ten-seed spread 1.186 .. 1.231, 8/10 over the
bar — a real margin, not a comfortable one, and far tighter than the old
1.018 .. 1.436, because that spread was Poland's other agreements moving, not
trade. The 1.20 remains unsourced and that debt is still open.

### T-2 — `a_poor_nation_still_picks_up_what_everyone_has` lost a windfall it was riding

**RED.** Seed 1990, Equatorial Guinea acquires 4 against a floor of 5.

Cause is the removal of an unreal trade windfall: micro-states holding a few
pacts were collecting a 100%+ permanent GDP uplift and now collect at most 25%.
That removal is correct. But diffusion affordability is sized against GDP, so
the bottom of the distribution can now fall further than it could.

The fix belongs in the diffusion floor (`absorptive_capacity` / `effective_cost`),
which is the technology branch's territory, not the economy's. Not softened here.

**CLEARED 2026-08-31 in `BUILD_KNEE`, and the windfall stays removed.** A
decomposition of the price the poorest economies actually face found the *floor*
binding, not the copy price: at thirty years `bio_universal_immunisation` has an
adopter share of 1.000 and the copying discount has taken the copy price to
nothing, so what Equatorial Guinea is charged is `cost * build * scale` — 19
months of its ENTIRE research budget, about twenty-one years of the one domain
that would fund it. The knee was raised 0.004 → 0.008, which is the reference
the microstate roster branch had already measured ("about $1.4bn of 1990
output") and which the integration of the two roster branches dropped when it
reconciled their two *shapes* and kept only one branch's *number*. Price falls
to 10-11 whole-budget months. Distribution over seeds 1990/7/42, technologies
held in 2020: min 3 → 9, p10 16 → 24, p25 31 → 39, p50 64 → 71, p75 101 → 109,
frontier 130 → 129. Equatorial Guinea 3 → 10, Sao Tome 3 → 11.

**Two things worth the owner's eye.** The wider reading of the same argument —
`knee = 0.020`, where capital-goods industries genuinely begin — was measured
and REJECTED: it takes the median nation to 87 and turns
`mature_economies_do_not_run_hot` and `the_1990_endowment_does_not_move_year_one_growth`
red. And the floor is the operative price for essentially every follower on
every ordinary technology, which means **one unargued constant sets the whole
world's technology distribution**: a 3x change in it moved the median nation
from 64 technologies to 108. That sensitivity is recorded rather than exploited.

### C-1 — what still pays the post-communist bloc, after the transition collapse landed

The transition collapse now exists: `capital_level_paid` is inherited at both
dissolution sites, so a successor is charged for the difference between the
union's investment programme and its own, and Russia's nineties fall from
+6.11%/yr to +1.40%/yr with a trough in 1998 and a recovery after. Three things
still pay the bloc more than reality did, all of them measured, none of them
closed here, and each blocked on something specific.

Read `transition_decomposition` and `transition_trajectory`
(`spheres-sim/tests/growth_decomposition.rs`, both `#[ignore]`d) for the tables.

**C-1a — every Soviet successor inherits the union's demography.** `dissolve_ussr`
carries the union's `pop_growth_offset` to all fifteen republics, which the block's
own comment defends as the honest carry: a republic that did not exist in 1990 has
no transcribed rate, and inventing one is a refusal. The consequence is that Russia
grows its population at 0.94%/yr for thirty-two years, against a real 1990-2025
rate of about -0.1%/yr, ending some 35% too populous. It is worth +0.56 pp/yr in
the `labour` term directly, and more than that again indirectly: too many people
depress `gdp_pc`, which lowers `dev`, which raises `catchup` (Russia reads +1.20
pp/yr after 2000) and `invest_effect` (+0.79). The carry is right for Central Asia
— Tajikistan lands at +1.84%/yr against a real +1.9% — and wrong for Russia,
Ukraine and the Baltics, because the union's 0.94% is an aggregate over republics
whose rates differ by a factor of fifteen.

*Blocked on:* fifteen republic-level 1990 rates of natural increase, transcribed
with sources. They are the same class of fact as the population shares this block
already transcribes from the 1989 census, so this is a data pass rather than a
design question — but writing them from memory and attributing them to Goskomstat
would be a fabricated citation, which BIBLE §8 forbids in the same breath that it
permits a stated rule. Not done here.

**C-1b — successors are still paid twice for the union's technology.** Already
documented at the head of `dissolve_ussr`; this entry only adds the measurement.
Russia's decomposed `tfp` term reads **1.494 pp/yr through the nineties against its
authored trend of 0.008**, because `TechState::inherit` takes the successor's own
cited trend into `tfp_base` and `apply_bonuses` then adds `(s - reference)` for the
entire inherited stock on top. Roughly +0.7 pp/yr.

*Blocked on:* an owner ruling and a re-pin, exactly as that comment says. Untouched.

**C-1c — nothing in the model can stop running a plan, and that is the deeper
finding.** `cmd%` is **100 for Czechoslovakia, Romania, Bulgaria and the USSR
across all thirty-five years and all ten seeds**. `shock_therapy` is the only site
that moves a nation off `EconomySystem::Command`, and its gate is
`(Command && growth_last < 0.02) || inflation > 0.30`. Those four grow at 1.8-5.0%
a year in the model with inflation near their transcribed opening figures, so the
gate never opens. The gate is not wrong — "nobody dismantles a system that is
working" is the correct principle and it is what keeps China out — the *input* is.

Why they grow: `dev = gdp_pc / 24000` reads income at market exchange rates.
Czechoslovakia's transcribed 1990 figure is $3,205 a head, which is right at the
official rate and roughly a third of its real purchasing power. The convergence
engine therefore reads an industrialised middle-income economy as a poor one and
pays it `catchup` 1.67 + `invest_effect` 1.52 = **3.2 pp/yr of convergence it
cannot have**, which is most of its 4.99%/yr. The whole CMEA bloc is understated
the same way, and so, in the other direction of the same denominator, is China.

*Blocked on:* that denominator is board-wide and `china_growth_miracle` is pinned
against it, so changing what `dev` measures is a calibration pass of its own with
its own re-pin. Named here rather than half-done: loosening `shock_therapy`'s gate
until Czechoslovakia happens to fire would be fitting a threshold to a table.

### T-4 — `gulf_war_emerges` reads forty seeds of a rate that needs two hundred

**RED at 18/40 against a bar of 20.** The bar is a majority and the bar is right.
The model's rate is a majority. The forty seeds it reads are not enough to say so.

**Measured, both arms, two hundred seeds** (`gulf_war_wide_scan`, ignored, in
`spheres-sim/tests/growth_decomposition.rs`):

```text
                                     rate        95% interval    first 40
  before the transition-collapse   123/200  61.5%  [54.8, 68.2]    22/40   GREEN
  after                            113/200  56.5%  [49.6, 63.4]    18/40   RED
```

The two rates are 5.0 points apart against a standard error on the difference of
4.9 points — **1.0 standard errors, indistinguishable from no change** — and
both are a clear majority. The colour changed; the mechanism did not.

**Why the reshuffle is unavoidable.** `ai_wars` rolls `w.rng.chance(p)` for every
candidate dyad in turn, so the number of draws consumed before Iraq's roll
depends on the state of every nation on the board. Anything that changes any
nation's growth anywhere — and `money_works` changes it for the third of the
roster whose currency is failing — re-deals the whole stream. Neither of the two
changes reaches Iraq or Kuwait directly: Iraq's transcribed 1990 inflation is
0.18 and Kuwait's is 0.035, both below the 0.40 crisis line at which
`money_works` first departs from exactly 1.0, so the factor is 1.000 for both in
every month of the window.

**This is the failure mode the test's own comment predicts, one sample size
later.** It records being widened from ten seeds to forty for exactly this
reason — "it has been red at 4/10 and green at 6/10 on trees whose forty-seed
rate was the same number" — and it pre-commits to the ruling: *"If the true rate
ever sits below it, that is a finding about the model's appetite pass and belongs
in a bug entry, not in this literal."* The true rate does not sit below it. So
this is the entry, and the literal is untouched.

**Recommendation, not done:** raise the sample to two hundred seeds and the bar
to a hundred — the identical claim, a majority, asked of five times the evidence,
which is the same conversion the ten-to-forty widening already performed and is a
strengthening rather than a widening. It costs about 25 seconds of wall clock.
Untouched pending the ruling: iron rule 5 gives the owner the call on a
calibration test, and a bar moved by the agent whose change turned it red is the
one edit the rule exists to forbid, even in the direction that keeps its meaning.

**RE-MEASURED 2026-08-31 by the audit pass, on a freshly built binary, and the
reading has moved in the model's favour:**

```text
  Iraq invades Kuwait in 110/200 seeds = 55.0%
    standard error 3.5 points; 95% interval [48.1%, 61.9%]
    first forty (what `gulf_war_emerges` reads, bar 20):  18/40   RED
    first ten  (what HEAD's version read,   bar  5):       5/10   GREEN
```

**Two facts the owner should have before ruling.**

1. **The model's Gulf War rate is a majority: 55.0%.** The claim the test makes
   is true of the model. The forty seeds it happens to read are a 45% draw, and
   at n = 40 the standard error is 7.9 points — so a 55% process producing an
   18/40 sample is an entirely ordinary outcome, about one standard error low.

2. **Under the test body git HEAD carries, `gulf_war_emerges` is GREEN today**
   — 5/10 against a bar of 5. The test went red not because the model's war
   appetite changed but because a previous pass widened its sample from ten to
   forty, which was the right move and made the test *more* honest, and the
   wider sample happens to land on the unlucky side. This is the clearest
   possible statement of the problem: the test is red at 40 seeds and green at
   10 seeds and green at 200 seeds, on one unchanged model.

The recommendation is unchanged and still not done. It is worth noting that it
is the only remedy of the three that makes the test *stop* being a coin flip;
reverting to ten seeds would restore the colour and destroy the evidence.

### T-5 — the golden re-pin, adjudicated: **the protocol's own precondition is not met, so NOTHING was re-pinned**

**Date: 2026-08-31. This is the ruling the whole sequence — the 1990 endowment,
the exposure gate, the second authoring pass, the economy fix, the adoption
rebase, PLAN step 7 and the transition collapse — was queued behind.**

The protocol is: *re-pin the two goldens ONCE, LAST, only when every
emergent-history calibration test is green untouched, no tolerance widened, no
test deleted.* Applied literally, it has two halves, and they came out
differently.

**HALF ONE — "no tolerance widened, no test deleted, no bar moved": SATISFIED,
and verified by byte-comparison against `git HEAD`, not by grep.** Every `#[test]`
body in the sim crate was extracted from `git show HEAD:<file>` and from the
working tree by brace-matching, hashed and compared, across `lib.rs`,
`tech/mod.rs`, `data/mod.rs`, `government.rs`, `world.rs`, `economy.rs`,
`politics.rs`, `statecraft.rs` and `war.rs`.

```
tests deleted anywhere in the crate ..................... 0
test bodies changed ..................................... 9
test bodies added ....................................... 24
```

The nine changed bodies, each with the verdict the diff supports:

| test | what changed | bar |
|---|---|---|
| `the_frontier_does_not_run_away` | 1 seed → median AND WORST of 10; control arm added | `4.0`/`0.5` byte-identical to HEAD's `0.040`/`0.005` |
| `gulf_war_emerges` | 5-of-10 → 20-of-40; control arm added | the identical "majority" claim at 4x the evidence |
| `a_trade_agreement_lifts...` | Poland's other pacts struck out in BOTH arms; control arm added | `1.20`, `0.02`, `×10` all unchanged |
| `a_poor_nation_still_picks_up...` | measures `count − granted` instead of `count` | `>= 5` unchanged; the quantity got **harder** |
| `sanctions_cost_the_target_real_growth` | body extracted to a helper; control arm added | unchanged |
| `a_large_nation_is_subjugated...` | pinned seed 93 → sweep of seeds 0..20; control arm added | `pop < 8.0` unchanged |
| `a_dead_nation_holds_no_districts` | pinned seed 93 → sweep of seeds 0..20 | invariant unchanged |
| `a_burned_aggressor_does_not...` | provenance assertion added | pure addition |
| `roster_decomposition` | a `credit` column | `#[ignore]`d readout, **zero assertions in either tree** |

Every one is a strengthening or a construction repair. Not one is a loosened
threshold. And the tests that would be the tempting places to cheat are
**byte-identical to HEAD**, which is the strongest form this claim can take:

```
china_growth_miracle ................ IDENTICAL TO HEAD   (band 11.0-19.0, floor 6.0)
mature_economies_do_not_run_hot ..... IDENTICAL TO HEAD   (band 0.008-0.026)
the_1990_start_is_pinned ............ IDENTICAL TO HEAD   (constant not re-pinned)
golden_hash_of_a_known_run .......... IDENTICAL TO HEAD   (constant not re-pinned)
determinism_same_seed_same_world .... IDENTICAL TO HEAD
save_load_roundtrip_continuity ...... IDENTICAL TO HEAD
```

**HALF TWO — "every emergent-history calibration test is green untouched": NOT
SATISFIED. Three are red.**

```
gulf_war_emerges ............................... RED   18/40 against a bar of 20
a_large_nation_is_subjugated_rather_than_swallowed  RED   non-vacuity guard
a_dead_nation_holds_no_districts ............... RED   non-vacuity guard
```

**RULING: re-pin NOTHING. The blocking test is `gulf_war_emerges`, with the two
conquest tests behind it.** Both goldens are left red with the HEAD constants
intact.

```
the_1990_start_is_pinned      expected 0xd022d50f43c984da   actual 0xa5c9c5b2306313d8
golden_hash_of_a_known_run    expected 0xbd5a...            actual 0x47581e52332a3e0b
```

**Why this is the right call even though all three reds are arguably cosmetic.**
The temptation here is real and should be named. None of the three reds is a
model defect: the Gulf War rate is a majority (55.0% over 200 seeds) and the
conquest rule holds every time it is exercised (both annexations at ~5m, against
an 8m bar). An agent could argue all three are sampling artifacts and re-pin. It
would be wrong to, for two reasons. **First**, the protocol says green, and the
whole value of a precondition is that it is not renegotiated by the party who
wants past it. **Second**, and concretely: a golden hash is the one artefact in
this repo that cannot be re-derived from evidence — it is a pure assertion that
*this* timeline is the intended one. Pinning it while three tests that read that
same timeline are red would be pinning a fingerprint of a world nobody can
currently certify. The reds are cheap to clear (T-3 and T-4 each name their
remedy, both are test-sample changes, both are the owner's call under iron rule
5). The re-pin should follow them, not precede them.

**FOR WHEN IT UNBLOCKS — every mechanism that moved across this sequence, which
the re-pin justification must name.** All seven are recorded as correct, with
the measurement that establishes each:

1. **The 1990 endowment** — nations may be authored as holding technology in
   January 1990. Correct because the alternative is a world where France
   re-invents the jet engine. Neutral to year-one growth by construction, and
   the neutrality is itself tested.
2. **The exposure gate and the second authoring pass** — the data those grants
   read, validated by nine new refusal tests in `data/mod.rs` (unknown field,
   misspelled key, duplicate grant, sourceless grant, anachronistic grant).
3. **The economy fix** — `invest_effect`, `labour` and `demand_gap` converted
   from paying a permanent RATE to paying a one-time LEVEL. BIBLE §8's error
   class. Max mature error 2.43 → 0.86, Spearman rho 0.086 → 0.886.
4. **The adoption rebase (E-1)** — `tech_1990_deficit`, its consumption, and the
   revelation rule that keeps a 1990-vintage unlock out of `absorption_rate`.
   Without it the endowment was non-neutral at 3.27e-3 against a 1.0e-4 bar;
   with it, 9.4e-5.
5. **Step 7's symmetric `MAX_DEMAND_GAP`** — a zero-mean term whose two tails
   were bounded at different distances is biased by construction. 0.35 is read
   off the bust side's existing bound of −0.344, so it provably cannot bind
   where anything was binding before; it binds in 0.116% of nation-months.
6. **Step 7's four sanction channels**, converted from counting flags to weighing
   output (`economy.rs` stability, `tech::research_output`,
   `tech::absorptive_capacity`, `government.rs` Business pillar). Two dead
   clamps removed as provably unreachable once the count is bounded.
7. **The transition collapse** — `money_works` gating the demand term's OUTPUT
   arm but not its PRICE arm, and successors inheriting `capital_level_paid`
   from their parent instead of being handed `None`. Russia now falls
   1995→1998, troughs in 1998 and recovers on oil; before, no post-communist
   economy contracted at all.

**Two mechanisms are specified but deliberately NOT shipped and must not be
silently included in a future re-pin:** the producer `oil_effect` level
conversion (fully written up in `economy.rs:369-434`, blocked because it takes
`china_growth_miracle` red at 10.86x and the honest fix is to `oil_market`
first), and the growth ceiling (ruled against on the merits — it would hide the
very bug it was sent to find).

#### SECOND ADJUDICATION, 2026-08-31 — the three original blockers cleared, a new one took their place. **STILL NOTHING RE-PINNED.**

Run after the capital-channel repair (ruling 1), the conquest/Gulf re-pointings
(ruling 2) and the sampling doctrine (ruling 3), on binaries rebuilt from touched
sources and watched (iron rule 6: every source stamped 19:26:17, every test
binary 19:26:31–19:26:34, the CLI 19:26:54).

**HALF ONE — "no tolerance widened, no test deleted": SATISFIED, and re-verified
independently rather than inherited.** Every `fn` body in `lib.rs` and
`economy.rs` was extracted from `git show eb7de26:<file>` and from the working
tree by brace-matching, comment bytes blanked so prose changes cannot mask an
assertion change, hashed and compared duplicate-aware:

```
test/function bodies DELETED ............................ 0
bodies whose CODE changed, lib.rs ....................... 7
bodies ADDED, lib.rs .................................... 2   (conquest_endings,
                                                               conquest_size_rule_scan)
bodies whose CODE changed, economy.rs ................... 1   (economy::tick)
bodies where only COMMENTS changed ...................... 0
```

The seven, with every assertion extracted and compared literal by literal:

| body | what changed | bar |
|---|---|---|
| `a_large_nation_is_subjugated_rather_than_swallowed` | board deaths → every `Ending::Conquest`, both verdicts asserted; 20 → 100 seeds | `8.0` and `0.6` unchanged; `!found.is_empty()` → `refused >= 15` |
| `a_dead_nation_holds_no_districts` | counted from `war::conquer`'s own headline, not "died and is not USSR/Yugoslavia"; 20 → 240 seeds | `annexations > 0` → `>= 1` (same claim) **plus a new ceiling `<= 40`** |
| `gulf_war_emerges` | 40 → 200 seeds | `>= 20` of 40 → `>= N/2` of 200. Same 50%, third literal |
| `china_growth_miracle` | 10 → 100 seeds; median written generically | `(11.0..19.0)` and `x > 6.0` byte-identical |
| `a_burned_aggressor_does_not_come_back_for_the_same_prize` | 10 → 20 seeds | **all four assertions byte-identical** |
| `gulf_wars` (helper) | seed range became a parameter | hit criterion unchanged |
| `gulf_war_incidence_scan` | 40 → 400 seeds, prints its own derivation | `#[ignore]`d, asserts nothing |

Not one loosened threshold; one *added* ceiling. `determinism_same_seed_same_world`,
`save_load_roundtrip_continuity`, `mature_economies_do_not_run_hot`,
`the_frontier_does_not_run_away`, `the_1990_start_is_pinned` and
`golden_hash_of_a_known_run` are all byte-identical to HEAD.

**A SCOPE QUESTION FOR RIDGE, raised rather than resolved.** The re-pin protocol
was handed down naming **two** authorised re-pointings. **Five** tracked test
bodies differ from HEAD. Each carries a dated authorisation block quoting him —
three cite ruling 2 (`a_large_nation_is_subjugated…`, `a_dead_nation_holds_no_districts`,
`gulf_war_emerges`), one cites ruling 3's named target "Gulf War n>=200, China
n>=100" (`china_growth_miracle`), and one cites iron rule 7's general doctrine
with no named target (`a_burned_aggressor…`, raised 10 → 20 because its measured
false-red rate was 8.06%). Nothing here is a widening and the count does not
change the ruling below, but **"exactly two" and "five, all documented" should be
reconciled by the owner before any re-pin**, because the re-pin is the act that
blesses the set.

**HALF TWO — "every emergent-history calibration test is green untouched": STILL
NOT SATISFIED.** The three reds this entry first recorded are now green — and
green *for the wrong reason*, which is worth saying plainly: `gulf_war_emerges`,
`a_dead_nation_holds_no_districts` and `a_large_nation_is_subjugated_rather_than_swallowed`
pass partly because the board grows faster and partly because ruling 2 gave them
samples that can see their own events. A fourth red replaced them:

```
tech::tests::the_1990_endowment_does_not_move_year_one_growth ... RED   E-2, above
tests::the_1990_start_is_pinned ................................ RED   pre-existing at HEAD
tests::golden_hash_of_a_known_run .............................. RED   pre-existing at HEAD
                                                          152 passed, 3 failed, 19 ignored
```

**RULING: re-pin NOTHING. The blocker is E-2** — `the_1990_endowment_does_not_move_year_one_growth`,
red at 102.2% of a bar it cleared at 98.8% on HEAD, caused by the capital repair
and proved so by isolating `economy.rs` onto a pristine HEAD worktree. It is a
neutrality claim about the 1990 board, so it is precisely a statement that *this*
timeline is the intended one — the same thing a golden hash asserts. Pinning a
fingerprint of a board whose own neutrality test is red would pin the defect.

**The two pins are stale at HEAD independently of all of this**, which is a
separate finding and must not be folded into the same fix:

```
                             pinned                 HEAD actual            repaired actual
the_1990_start_is_pinned     0xd022d50f43c984da     0xa5c9c5b2306313d8     0xa5c9c5b2306313d8
golden_hash_of_a_known_run   0xbd5ec0f43c5f2e3b     0x47581e52332a3e0b     0x20c24ab0f1581807
```

`the_1990_start_is_pinned` reads the pre-tick board, so the capital repair cannot
move it and did not — **bit for bit the same value on both trees**. HEAD ships a
1990 board that does not match its own 1990 pin. The timeline hash does move, as
it must: the capital channel changed.

**THE MECHANISM LIST THE EVENTUAL RE-PIN MUST NAME** is the seven above plus one:

8. **The capital-channel repair (ruling 1, 2026-08-31)** — `economy::tick`. The
   RATE arm `(s/0.20)^0.55 · 0.20` had no zero, so a nation investing 4% of
   output was paid capital deepening while its stock shrank, and concavity was
   applied to GROSS investment as though the replacement twelve points bought
   growth. Replaced by `(s − 0.125)·(0.20/(0.20 − 0.125))`, which equals the old
   term exactly at the reference `s = 0.20` and whose replacement line is `δ·(K/Y)`
   read off the constants already in the file. The LEVEL block's free `0.02`/month
   became `(δ+g)/12`, and its linearisation became `exp(gap) − 1`. China's
   30-year multiple 11.07x → 14.69x against a real 14.33x; below-floor rate
   45.8% → 3.3%; mature panel unmoved (rho 0.886, four clauses, max error 0.86 →
   0.84). **Cost: Indonesia +1.50 pt/yr further from reality, Vietnam and South
   Korea worse, and E-2.**

---

### T-6 — the tracked decomposition instrument still computes the OLD capital term

**`spheres-sim/tests/growth_decomposition.rs:56-57`**, in `terms()`:

```rust
let intensity = spheres_sim::exact::powf((invest / 0.20).max(0.0), 0.55) * 0.20;
let invest_effect = intensity * 0.080 * (1.0 - dev);
```

That is the formulation `economy::tick` replaced. Six readouts read `terms()` —
`growth_decomposition`, `developing_decomposition`, `mature_sanction_exposure`,
`transition_decomposition` and their neighbours at lines 149, 432, 772, 778, 1128
and 1136 — so their `invest` column and every `SUM` built from it is now wrong by
the size of the repair, which for China is 0.9 pt/yr. **The instrument is
`#[ignore]`d and asserts nothing, so nothing is red, which is exactly why it will
rot unnoticed.**

**Not affected, and checked rather than assumed:** `mature_panel`,
`mature_panel_wide` and `developing_panel` do not call `terms()` — they compound
`n.gdp` directly — so every panel number reported for this pass stands. The
untracked `spheres-sim/tests/capital_damage_audit.rs` carries a `terms()` that
*was* updated, which is why its TABLE C is the one to trust today.

**Left alone deliberately.** It is a tracked test file and doctrine says an agent
that thinks a test is wrong stops and reports. Fixing it is three lines and no
bar moves; it wants a nod, not a branch.

### T-7 — the mature panel's acceptance criterion is asserted NOWHERE

The standing bar on this project — *Spearman rho at or above 0.886, USA fastest,
Japan and Italy below the others, Germany below the UK, max error under a point*
— is computed by `growth_decomposition::mature_panel`, which is `#[ignore]`d and
contains **zero assertions**. `grep -c assert` over its body returns 0, and
`spearman()` appears nowhere in `lib.rs`.

So the headline criterion the last four sessions have been judged against cannot
fail a test run. A future change could take rho from 0.886 to 0.4 and invert
Germany and the UK, and `cargo test --workspace` would be green. The nearest
tracked cousins police different quantities: `mature_economies_do_not_run_hot`
guards a band per nation, `the_frontier_does_not_run_away` guards the fastest and
slowest, and neither reads the *ordering*.

This is iron rule 7's power clause pointed at the thing the rule was written to
protect. Making it a bar is a calibration decision and therefore Ridge's:
the honest construction is a tracked test asserting the four ordering clauses
and a rho floor, sized by the same variance arithmetic rule 7 asks for
(`mature_panel_wide`'s 40-seed pairwise matrix already gives it — P(UK faster
than Germany) = 0.93, which is the tight one).

### C-2 — Indonesia is what the capital repair cost, and it is now visible in the league table

**No calibration bar covers Indonesia, which is why this needs writing down.**
The capital rate arm's reshape pays a nation in proportion to how far its
investment share sits above the replacement line, gated by how far it is from the
frontier. Indonesia holds a 33% share **and** a development gate that never
closes (`dev` reaches only 0.474 in 35 years), so it draws the arm at full width
for the whole run:

```
                 35y CAGR %/yr                    capital rate arm, pt/yr
              HEAD    repaired    real            HEAD      repaired
Indonesia     9.152    10.647    4.88(e)          1.870  ->  3.652     WORSE by 1.50
Vietnam       7.355     5.906    6.88(e)          1.262  ->  0.103     WORSE by 0.49
SouthKorea    5.083     5.232    4.53(e)          0.628  ->  1.191     WORSE by 0.15
China         8.275     9.078    8.70(e)          1.742  ->  2.642     FIXED
India         8.272     7.765    6.39(e)          1.436  ->  1.092     better
Poland        5.139     4.770    2.94             1.156  ->  0.721     better
Brazil        4.348     4.300    2.24(e)          1.325  ->  1.321     ~flat
Nigeria       6.926     6.082    4.37(e)          1.269  ->  0.386     better
```

**It has reached the headline artefact.** Seed 7, thirty years, same run on both
trees — the 2020 league table moves Indonesia from **11th to 6th**, above the
United Kingdom, France, Russia, Italy and Mexico:

```
HEAD      ... 6 Russia 2407, 7 France 2395, 8 Italy 1993, 9 UK 1990,
              10 Mexico 1866, 11 Indonesia 1749, ... 16 Thailand 1077
REPAIRED  ... 6 INDONESIA 2626, 7 UK 2048, 8 France 2047, 9 Russia 2024,
              10 Italy 1935, 11 Mexico 1902, ... 13 Thailand 1501
```

The same table is where the repair's win shows: **China moves from 3rd (4337,
below Japan) to 2nd (5926, above Japan)**, which is the single most important
structural fact about the 1990–2020 world and which HEAD did not produce.

**TWO DIAGNOSES, and only one of them is the channel's fault.**
 * **Indonesia and South Korea** are the reshape doing what it says on a very
   high share. The formulation holds the replacement line at the world 1990
   reference and does not let it rise as a nation accumulates capital; the
   `(1 - dev)` gate does the expiring instead, and Indonesia is where that
   simplification is most exposed. The proper fix is a per-nation
   capital-output ratio, which is **not transcribed anywhere in this repo** —
   inventing one is iron rule 4's refusal.
 * **Vietnam is an input defect, not a formulation defect.** The model drives
   Vietnam's investment share to 0.1022 by 2000, below the 0.125 replacement
   line, so the arm correctly pays a negative. Real Vietnamese gross capital
   formation in the 1990s ran roughly 25–30% of output. Same shape as China's
   share falling 0.300 → 0.261 where reality rose from ~35% to ~42%: a debt-path
   defect in `politics.rs`'s ratchet, which the capital repair made the channel
   *robust to* rather than curing.

**The one lever that would trade Indonesia against China is `REPLACEMENT_SHARE`,
and it was deliberately not swept.** It is derived from the reference share 0.20
and δ; tuning it until Indonesia behaved would be closing a residual with a
coefficient.

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

**CORRECTED 2026-08-31 — conquest is NOT unreachable; annexation is.** A funnel
instrument (`conquest_funnel`, `#[ignore]`d) separated the gates over seeds 0..40
and forty years:

```
conflicts opened                          691
...that ever declared an invasion         285
...that ever saturated control (>= 0.97)  122
...saturated AND standing at rung 8       117
Ending::Conquest reached                   10
...that ANNEXED   (loser < 8m and calm)     1
...that SUBJUGATED (loser too big or angry) 9
```

**`Ending::Conquest` fires 10 times in 40 seeds — the branch is healthy and
thoroughly reachable.** The demanding conjunction this entry blamed is not the
binding constraint. What is rare is *annexation*: 9 of 10 losers were ≥8m people
or ≥0.6 separatism, so `conquer()` subjugated them instead. The rate itself has
not moved — 3-in-120 and 1-in-120 are single-digit Poisson counts and are not
distinguishable evidence.

**The test-health finding this exposes, awaiting a ruling.**
`a_large_nation_is_subjugated_rather_than_swallowed` is named after the
subjugation rule and **structurally cannot see it**: it inspects only nations
that *left the board*, so it reads 1 sample in 120 seeds while the rule it names
fires 9 times in 40. Its non-vacuity guard is riding a single seed — exactly the
pin fragility its own comment says the twenty-seed sweep exists to escape.
`a_dead_nation_holds_no_districts` rides the same one seed for the same reason.

*Recommendation, not done:* assert that every `Ending::Conquest` whose loser is
≥8m or ≥0.6 separatism leaves that loser alive with its districts intact, and
count those toward non-vacuity. That is a strengthening rather than a widening,
but it is a calibration test and the doctrine gives the owner that call.

## Documentation drift found while auditing

Not bugs in the sim, recorded here because the audit is where they surfaced:

- README.md claims "24 nations at the start and up to 30 once federations come
  apart" and "46 calibration/invariant tests". Measured: **137 nations at the
  1990 start, peaking at 158**, and 98 sim + 13 web tests. Re-measured
  2026-08-31: **173 sim tests (150 pass, 5 fail, 18 ignored), 17 web tests
  (16 pass, 1 fail)**, plus 13 `#[ignore]`d audit instruments in
  `spheres-sim/tests/growth_decomposition.rs`.
- README.md describes `feat/statecraft` as written but unmerged. `statecraft.rs`
  is in the tree and its commands are priced.
- SPEC.md section 9 still describes the v0.5 roster as 16 nations.

## Awaiting an owner ruling (added 2026-08-31 by the crash-and-numbers fixer)

### W-1 — the browser's policy ledger mirrors economy.rs WITHOUT its three bounds, and the runaway it was reported for is no longer reachable

**Reported as TRIAGE F-07** by china-thirty-05: the policy ledger printed
`Expected growth -105356.7%` and `Debt drift +5556746164.7pp/yr`.

**The structural claim is true.** `index.html` mirrors four of economy.rs's
functions (`potentialGrowth`, `demandOf`, `dragsOf`, `ledgerOf`, index.html
~2090-2130) and reproduces none of the three bounds the sim applies to the same
arithmetic:

| sim bound | site | mirrored in the browser? |
|---|---|---|
| `MAX_DEMAND_GAP = 0.35` | economy.rs:390-391 | no |
| `MAX_OIL_SHARE = 2.0` | economy.rs:520-522 | no |
| `WORST_ANNUAL_COLLAPSE = -0.95` | economy.rs:676-682 | no |

**The reported SYMPTOM could not be reproduced, and the measurement says why.**
Scanned every nation seated in 1990, every month, 12 seeds x 60 years (8,640
world-months, ~1.3M nation-months), computing the browser's own expressions from
the same payload fields the browser reads:

```
worst demand gap       +0.1888  (+18.9pp)   sim clamp +-0.35   Sudan, seed 0, Feb 1990
worst expected growth  -0.1510  (-15.1%)    sim floor  -0.95   Nicaragua, seed 11, Jun 1992
worst oil share         1.2198              sim cap     2.0    Kuwait, seed 11, Mar 1993
worst debt_gdp          3.1407  (any nation, seated or not)    Nicaragua, seed 1, Feb 1990
```

Not one of the three bounds is reached. `-105356.7%` requires `drag.debt` near
1053, i.e. `debt_gdp` near 52,650 — **four orders of magnitude above anything
the current model produces** — and the huge drift is the same event seen through
`drift = deficit - debt * (expected + inflation)`.

**The likeliest explanation is that the sim was repaired and the mirror was
not needed after all.** `MAX_OIL_SHARE`'s own comment records that "the runaway
it stops had reached 65,700"; once the sim stopped producing runaway inputs, the
browser stopped printing runaway outputs, because it is fed by the payload.

**Why nothing was changed.** There is no before-and-after to show: on every state
the model can currently reach, applying the bounds changes no digit on screen.
Shipping them would also mean either three more mirrored literals in index.html —
the exact class of defect PLAN step 2 and the surface audit say to hunt — or
promoting three function-local constants in economy.rs to `pub` and transporting
them on the state payload the way `front::HELD_BAND` already is. The second is
the right shape, and it edits the sim for a defect that cannot currently occur.
That is the owner's call.

**Recommendation, not done:** transport the three as a `bounds` object on
`state_json`, exactly as `front_held_band` is transported, and clamp in
`demandOf`/`dragsOf`/`paintPolicy` against the transported values. Cost: three
`const` -> `pub const` moves in economy.rs (values untouched), one payload field,
three clamps in index.html. Benefit today: none measurable. Benefit if the sim
ever produces a wilder state again: the screen stays inside the sim's own range
instead of printing ten digits.

**Already fixed separately, and it was the reachable half of the same report:**
the drift line had no debt floor either, and that one DID fire in ordinary play —
41 of one world's 156 living nations sat at exactly zero debt while the ledger
told them their debt was falling. Committed as "web: a nation with no debt is no
longer shown paying it down" (TRIAGE F-12).

### W-2 — a join is quoted at 14 PC and charges up to 56, and the difference is `seek_access` spending on the player's behalf

**Found while verifying the F-20/F-21 fixes in the browser, not reported. It is
the ROOT CAUSE of TRIAGE F-13**, which recorded the same shape on the ladder
("quotes 25 pc and the queue charges 30.9, exactly 6.00 every time, every
seed") without naming the mechanism. F-13's fixed 6.00 is the one-host case of
what is really a per-host charge.

**Symptom.** The war sheet's TAKE A SIDE button is captioned `14 PC`, which is
the sim's own price for `Command::JoinConflict` (lib.rs:234). Pressing it as
Iraq on seed 1990 against the Korean conflict took the political capital from
41.22 to 9.22 — **32 charged against 14 quoted**. Measured in the browser and
reproduced headless.

**Mechanism, and it is exact.** `commitment::join_conflict` ends with

```rust
// An expeditionary power that has just committed itself goes round the
// neighbours the same month, because everything above rung 5 depends on it.
if !theatre::is_home(w, joiner, th) {
    seek_access(w, joiner, th);
}
```

Each of those approaches is a real `Command::RequestAccess`, priced at 6.0
(lib.rs:263), and `apply_command` re-reads the payer's balance AFTER dispatch
before taking the join's own 14 — so the two charges compound. Joining Iraq to a
conflict in each theatre it is not home to, holding 500 PC, quoted 14.0 every
time:

| theatre | hosts in range | charged |
|---|---|---|
| Gulf (Iraq is home — `seek_access` skipped) | 7 | **14.0** |
| South Asia / North America / Central Africa / Oceania | 2 | 26.0 |
| East Asia / West Africa / Southern Africa | 3 | 32.0 |
| Southeast Asia / Central Europe / North Africa / East Africa | 4 | 38.0 |
| Balkans / Latin America | 5 | 44.0 |
| Western Europe | 7 | **56.0** |
| Levant | 5 | 38.0 (one host not approached) |

`charged = 14 + 6 x (hosts approached)`. The home case is exactly the quote,
which confirms the extra is entirely `seek_access`. **Quoted 14, charged up to
56 — four times the price on the button.** The player is billed for requests
they did not make and which are not itemised anywhere on the sheet.

**Why this was not fixed, and it is the whole reason it is here.** Every repair
crosses one of the standing lines:

1. *Quote the real number.* The UI cannot compute it — the count depends on
   `seek_access`'s host selection, which is sim logic. Re-deriving it in
   index.html is the mirror class PLAN step 2 exists to hunt, and it would need
   the literal 6.0 copied in as well.
2. *Ship the real number.* The honest version: give the sim a pure
   "what will this cost" query that `join_conflict` and the quote both use. That
   is a real refactor of the access path — `seek_access` currently decides and
   spends in one pass — and it is a sim change made for a UI caption.
3. *Make the automatic requests free, or charge them once.* Moves a price.
   Forbidden.
4. *Stop joining from seeking access.* Changes gameplay. Forbidden.

**It is also not obvious which way the owner wants it.** The 6-per-host charge
may be exactly the intended cost of committing to somebody else's war abroad, in
which case the defect is only that the button lies about it; or the compounding
may be an accident of `apply_command` re-reading the balance after dispatch, in
which case the sim is overcharging. That is a design question, not a fixer's.

**Recommendation, not done:** option 2, and F-13 should be re-pointed at this
entry — its "exactly 6.00 every time" is a single-host measurement of a
per-host rule, so a fix that hard-codes 6 would be right in the Gulf and wrong
in Western Europe by 36 PC.

---

### E-3 — E-2's mechanism is wrong: the residual is the world reference, not the capital arm

**Measured 2026-09-01 by the sim-and-web fixer, with a new instrument
(`spheres-sim/tests/endowment_channel_probe.rs`) added for the purpose. This
entry CORRECTS E-2's mechanism paragraph and does not dispute its attribution:
the capital repair did move Belgium 9.8823e-5 -> 1.0218e-4, exactly as E-2
measured. What is wrong is the account of HOW.**

```
cargo test --release -p spheres-sim --test endowment_channel_probe -- --ignored --nocapture
```

`endowment_margin_probe` says WHICH nation is worst and by how much. This one
says WHICH TERM the difference is in — it runs the identical A/B and prints
`tfp_base`, `saturated_tech_tfp`, `reference` and `adoption` separately for both
worlds, so the residual is attributed rather than reasoned about.

**E-2 says:** "the capital rate arm is gated by `(1 - dev)`, so whatever residual
non-neutrality survives E-1 is *amplified in proportion to the size of that
arm*." **Measured, that is not merely imprecise — it is inverted.**

| nation | `1 - dev` | capital rate arm | dgrowth |
|---|---|---|---|
| Switzerland | 0.0000 | **identically zero** | 7.462e-5 |
| Sweden | 0.0000 | **identically zero** | 7.355e-5 |
| Japan | 0.0000 | **identically zero** | 6.714e-5 |
| China | 0.9847 | largest on the board | 6.413e-5 |
| India | 0.9843 | largest on the board | 6.390e-5 |

`invest_effect = net_intensity * 0.080 * (1 - dev)` is **exactly zero** for
Switzerland, Sweden and Japan, and they sit ABOVE the two nations whose arm is
the largest there is. A term that is identically zero cannot be amplifying
anything. And `capital_level_paid` is **bit-identical between the two worlds for
every nation checked** — 0.09020584 against 0.09020584 for Belgium — because
`ai_aggression = 0.0` leaves the investment share flat for twelve months, so
`entitled` never differs and the whole level block is a no-op across the A/B.

**WHAT THE RESIDUAL ACTUALLY IS, and the accounting closes at 100.0%.** `d.trend`
starts at 1.784e-7 — the rebase works — so the entire residual is the MOVEMENT in
`d.trend` over the twelve months. `d.tfp_base + d.sat` is a t=0 constant that the
revelation machinery preserves to four figures, so what is left is two terms:

```
                d.trend m1   d.trend m12    -D(d.ref)   D(d.adopt)   explained
   Belgium        1.784e-7      3.040e-4     3.220e-4    -1.814e-5      100.0%
   Switzerland    1.784e-7      2.849e-4     3.220e-4    -3.719e-5      100.0%
   China          1.784e-7      2.849e-4     3.220e-4    -3.719e-5      100.0%
   India          1.784e-7      2.849e-4     3.220e-4    -3.719e-5      100.0%
```

`-D(d.reference)` is **+3.220e-4 and IDENTICAL for every nation**, because
`world_reference` is one GDP-weighted scalar the whole board reads. In the
control world the top twenty are learning fast and the reference climbs; in the
granted world they are already saturated and it does not. The two worlds'
references converge from 1.923e-3 apart to 1.601e-3 apart, and that 3.22e-4 IS
the residual. **This is a uniform board-wide shift, not a per-nation double
payment** — which is exactly what E-1's own closing note said was left, and this
is the measurement of it.

**WHY BELGIUM AND NOT SOMEBODY ELSE.** Every nation gets the same +3.220e-4 from
the reference. What separates them is the second column: Belgium's `D(d.adopt)`
is **-1.814e-5** where every other nation gets **-3.719e-5**. Belgium's control
world still has adoption running at month twelve when everyone else's has
decayed. The reason is in the data, not the arithmetic — Belgium's authored 1990
technology file holds **2 grants**, the thinnest among the twenty largest
economies:

```
United States 40   Japan 27   Germany 22   France 21   United Kingdom 20
Netherlands 17   Italy 16   Canada 15   Sweden 12   Switzerland 11
South Korea 7   Taiwan 6   China 5   India 5   Brazil 4   Mexico 3   Belgium 2
```

So Belgium has the most left to reveal, reveals it slowest, and carries adoption
furthest into the year. **The nation this test goes red on is decided by which
file is least authored.**

**NO FIX IS AVAILABLE INSIDE THE STANDING LINES, and the reason is now sharper
than E-2's.** The residual is not a bug in the rebase — the rebase is exact to
1.784e-7 at month one and the revelation machinery holds `d.tfp_base + d.sat`
constant to four figures. The residual is the definition of `world_reference`
meeting an A/B that changes what the world knows. Repairing it means one of:

1. **Change how `reference` enters `tfp_trend`** — freeze it, or rebase it
   per-nation the way the 1990 deficit is. That is the core productivity model,
   it moves every growth figure and both goldens, and it is the owner's.
2. **Author Belgium's 1990 technology** so its file is not the thinnest on the
   board. This is the fix that addresses the 1.9e-5 that makes Belgium
   specifically the worst nation — but authored 1990 data is corrected only
   against a source, never to make a test behave (iron rule 4), so it is the
   owner's and it needs a source, not an agent.
3. **Widen the bar** — forbidden by iron rule 5, restated here only so it is not
   rediscovered as available.

E-2's option 3, "trim the capital rate arm until Belgium clears", should be
struck: this measurement shows it would not clear Belgium, because the capital
arm is not carrying the residual. Trimming it would move the answer only through
the GDP weights inside `world_reference`, which is the same coincidence that
moved it 3.4e-6 in the first place.

**WHAT THE CAPITAL REPAIR ACTUALLY DID, then.** It changed every nation's GDP
path by a hair; `world_reference` is GDP-WEIGHTED; so the weights inside the one
scalar carrying the whole residual moved, and the residual moved 3.4e-6 with
them. The transmission is the reference's weighting, not the `(1 - dev)` gate.
That also means E-2's closing warning should be re-pointed: the sensitivity
amplifier for every A/B in this suite is **`world_reference`'s GDP weighting**,
and any future change that moves the GDP path of a large economy will move this
residual again, whether or not it touches anything gated on `dev`.

**Nothing was changed. The instrument was added and is `#[ignore]`d.**

---

### W-3 — "+5 YR" delivers about one month, and the fix is a design ruling

**Measured 2026-09-01 by the sim-and-web fixer. TRIAGE F-10. NOT FIXED: every
repair is a decision about what should interrupt a player, and that is pacing.**

**Symptom.** The advance buttons are captioned +1 MO, +6 MO, +1 YR and +5 YR, and
the ? card documents keys 2/3/4 for the last three. Measured on the live server,
asking for 60 months every call until 240 months of history had passed:

| governing | calls to move 240 months | months per "+5 YR" click |
|---|---|---|
| United States | 202 | **1.19** |
| Oman | 190 | **1.26** |
| Bhutan | 134 | **1.80** |

**A 33x to 50x shortfall.** Two hundred and two clicks to play twenty years as
the United States.

**Cause, and it is working as written.** `Game::advance` stops early on any
headline `is_major` finds, and the browser banners the reason, so nothing is
silent or broken — a player is told why it stopped. The complaint is the
FREQUENCY. `is_major`'s structural list contains `escalates to rung`, which the
commitment ladder writes about 0.4 times a month across 137 nations, plus
`grants` and `revokes` for basing rights. The reasons a small nation actually
gets, quoted from the run above:

```
Oman:   "Thailand escalates to rung 2 — sanctions."
        "Revolution in Peru — the old regime falls."
Bhutan: "Iraq escalates to rung 2 — sanctions."
        "India escalates to rung 2 — sanctions."
USA:    "Zaire and United States sign a trade agreement."
```

Not one of those is something an Omani or Bhutanese government can act on, and
the function's own doc comment says the interrupt is for "an event worth
reacting to".

**Why it is not fixed here.** Every available repair decides what deserves to
stop a player's clock — drop the ladder from the structural list, gate the
structural list on the player's own theatres or relations, make the frequent
clauses interrupt only above some rung, or let the caller say how much news it
wants to be stopped for. Each is a pacing decision, each changes how the game
plays, and none of them is a wrong number or a false label. **Ridge's call, not
an agent's.**

**One part of it WAS a defect and IS fixed**, separately and on its own merits:
`is_major`'s second arm read a headline as being about the player whenever it
contained their name's LETTERS, so an Omani player's clock was stopped for
Romanian election results. See the commit "web: an Omani player's news was mostly
Romania's". That removes false interrupts; it does not touch the pacing.

**A twin is left standing on purpose.** `spheres-cli/src/main.rs:513` carries the
identical bare-substring `is_major`, so the CLI has the Oman/Romania defect too.
Untouched here only to stay out of another session's file — it is one line and it
is the same fix.

---

### W-1 UPDATE — the three bounds no longer need a ruling, because the browser no longer assembles the sum

**Added 2026-09-01 by the surface-lies fixer, while fixing TRIAGE F-35 / PLAN
step 2.** Not a new bug; it retires a question W-1 left open.

W-1 asked Ridge to choose between mirroring `MAX_DEMAND_GAP`, `MAX_OIL_SHARE`
and `WORST_ANNUAL_COLLAPSE` as literals in index.html or promoting three
function-local constants in economy.rs to `pub` and transporting them.

**Neither is needed now.** `economy::growth_terms` is the single definition of a
nation's year and `economy::tick` charges by it; `policy_json` serves its output.
The first two bounds are applied INSIDE that function, before anything crosses
the wire, so the browser cannot fail to apply them and never learns their values.
The third is carried as `GrowthTerms::floor` — a field on a value the sim
returns, not a constant made public — so the panel is handed the bound it needs
to floor a sum it assembles from served terms.

**No constant moved and no value changed.** `golden_hash_of_a_known_run` and
`the_1990_start_is_pinned` report the same actuals across the extraction
(`0x20c24ab0f1581807`, `0xa5c9c5b2306313d8`), which is the evidence that the
refactor was byte-for-byte behaviour-preserving.

**What W-1 measured is still true and is still the reason none of this shows up
as a changed digit for those three bounds specifically**: on everything the model
can currently reach, only `MAX_DEMAND_GAP` binds, and it binds where the browser
was previously furthest wrong — Zaire opens 1990 with a raw gap of +40.6% by the
browser's old arithmetic against the sim's clamped +35.0%.

---

### W-4 — the interest-rate slider cannot reach the rate it is displaying, and the fix is blocked by two tests that pin the bound as page text

**Measured 2026-09-01 by the surface-lies fixer. Written, verified, and
REVERTED**, because it turns two spheres-web tests red and both of them are
assertions.

**Symptom.** Zaire opens January 1990 with a policy rate of **45%**. The rate
slider renders:

```
label            "45.0%"
input value      400        (the browser clamping 450 to the end of the track)
input max        400
ghost marker     left: 112.5%   — painted OUTSIDE its own control
```

The ghost is the mark whose entire job is to show where the standing value sits,
and it is off the right-hand end of the widget. A range input takes a value from
a click anywhere on its track, so **the first touch queues a five-point rate cut
nobody asked for** — and because the first rate command latches the central bank
away for the rest of the game (see "web: the interest-rate slider is a one-way
door and never said so"), that touch is irreversible.

**Cause.** `renderLeft` spells all four slider ranges into the page, and one of
them disagrees with the sim: `0..0.40` against `Command::SetInterestRate`'s
`0..0.60`. The other three (tax 0.02..0.60, military 0..0.35, investment
0..0.40) match. The sim itself writes each bound twice — once in
`command_price` and once in `apply_command` — which is what let one of them
drift in the first place.

**Reachable, not theoretical.** Every nation whose policy rate exceeds 40% has
this. The AI's Taylor rule runs to 0.45, and the transcribed 1990 board seats
several nations above 0.40 on day one.

**The fix, written and measured before it was reverted.** Four named ranges in
lib.rs (`RATE_RANGE`, `TAX_RANGE`, `MIL_SPEND_RANGE`, `STATE_INVEST_RANGE`),
used by both sites there so the sim stops writing each bound twice — VALUES
UNTOUCHED — served on the policy payload as `bounds`, and `sliderHtml` built
from what it is given. After, on the same world:

```
label            "45.0%"
input value      450
input max        600
ghost marker     left: 75%      — inside the track, measured against the track's
                                  own client rect
```

The other three sliders render identically to before. No sim value moves; both
golden hashes report their usual actuals.

**Why it is reverted.** Two spheres-web tests pin the slider bounds as literal
text in `ui/index.html`, and any fix that moves the bounds off the call site
turns both red:

1. `the_page_can_see_who_is_running_the_central_bank` asserts
   `INDEX.contains(r#"sliderHtml("rate", "Interest rate", m.rate, 0, 0.40, rateSeat())"#)`
   — the whole call, including the wrong bound. Its INTENT is "the rate slider
   still says who is holding it", which the fix preserves; what it actually pins
   is the argument list.
2. `the_force_line_is_the_force_the_sim_sustains` **parses the military slider's
   upper bound out of the page source** —
   `INDEX.split_once("sliderHtml(\"military\", \"Military spending\", m.mil_spend, 0, ")`
   — to check the served force curve covers what the slider can ask for. With
   the bound served rather than spelt, the string it parses no longer exists and
   the test panics on its own `expect`.

The standing instruction is absolute: no test assertion is to be widened,
narrowed or corrected, and a fourth red is to be reverted. Both apply.

**Recommendation, not done, and it is small.** Re-point both assertions at the
served bounds rather than at the page text:

- (1) becomes `INDEX.contains(r#"sliderHtml("rate", "Interest rate", m.rate, rateSeat())"#)`,
  which pins exactly what that assertion is for.
- (2) reads `state_json(...)["policy"]["bounds"]["military"][1]` instead of
  parsing the source, which is STRICTER than what it does today: it would then
  be checking the curve against the bound the sim enforces rather than against a
  number the page happens to be carrying.

Both are Ridge's edits to make, not a fixer's.
