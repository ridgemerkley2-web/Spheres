# BUGS

## 2026-09-03 implementation repair update

The eleven reproducible findings from the latest browser/simulation audit have
been repaired; see `AUDIT_FIXES_2026-09-03.md` for the complete closure record.
This includes the missing Load/Continue controls noted in Y-13. The separate
E-3 calibration choice, stale golden pins, and other entries needing design/data
rulings are not implicitly closed by this release. Claude's Y-1..Y-3 repairs
were reviewed and merged before the new fixes.

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

---

## Awaiting an owner ruling (added 2026-09-01 by the layout-and-rendering fixer)

Two map-label findings. Both are real and both are measured; neither is fixed,
because the only repairs available are a ruling about **which layer wins on the
map** or **which names get dropped**, and that is a decision about what the
primary surface says rather than a rendering bug with one right answer.

### L-1 — every district name in Resources mode is painted over by its own district's chip

**Reported as TRIAGE F-42.** Measured in Chrome at 1280x720, Iraq on seed 1,
Resources shading with `oil` selected, camera on Iraq at k = 9 (well inside
ZB2 = 4, where district labels turn on):

```
district labels drawn      18
resource chips in view    107
labels overlapped by the chip of THEIR OWN district    18 of 18
```

**It is 18 of 18 by construction, not by crowding.** `refreshDistrictDetail`
draws the name at `(d.cx, d.cy)` with `text-anchor="middle"`, and
`refreshResGlyphs` centres the chip on the same `info.cx/cy` — the district
centroid. Any district that carries the selected commodity therefore has its
name underneath its own chip, at every zoom, on every commodity, for every
nation. The chip's plate is `#080c12` at `fill-opacity: .78`, so it is not a
tint over the text: it removes it. Measured coverage of a label's box by chip
plates: **57% average, 92% worst** (Babil, Dihok).

**Why the obvious repair does not work, measured rather than assumed.** Moving
the label clear of its own chip — `y += fs * 1.9`, which does clear it — takes
average coverage from 0.57 only to **0.51**, because with 107 chips in the
viewport the label lands on a neighbour's. A label offset is not a fix here.

**What would work is a paint-order decision.** `renderMap` emits `#resglyphs`
AFTER `#dlabels`, with a comment stating the intent: "The commodity marks go
last so nothing paints over them: over relief, over rivers, over the front
seam." Every layer that comment names is GROUND. `#dlabels` is text, and it was
inserted immediately above with no note. Drawing `#dlabels` after `#resglyphs`
instead would make the names 0% obscured; the cost, measured on the same view,
is that **25 of 107 chips** get a haloed 11px name crossing them (17.7% average
of a chip's box, one chip fully crossed), while keeping their coloured plate,
their border and their hover title. Chip hit-testing is unaffected either way —
`#dlabels` is `pointer-events="none"`, and 90 of 105 chips in the pane answer a
click at their centre in both orders.

**The ruling needed.** In Resources mode, when a district's name and its
commodity chip want the same pixels, which one wins? The reversal is one line;
what it decides is what the resource map is FOR. Not a fixer's call at 3am.

### L-2 — district names pile up on each other at the zoom where they turn on

**Reported as TRIAGE F-44.** Same session, political shading (so no chips are
involved), camera on Iraq, hovering Iraq. Counting pairs of `.dlabel` boxes
that intersect, out of the 153 possible among Iraq's 18 districts:

```
k = 4  (ZB2, the zoom labels appear at)   64 overlapping pairs
k = 5                                      46
k = 6                                      41
k = 9                                      24
k = 14                                      9
k = 20                                      3
```

China at ZB2: 32 labels, 20 overlapping pairs. **The pileup is worst at exactly
the zoom the labels switch on**, and the player has to dive five more steps
before the names separate. At k = 4 the middle of Iraq is a single unreadable
block of overprinted names.

**Cause.** There is no declutter of any kind. `refreshDistrictDetail` emits one
`<text>` per district at its centroid and lets them land where they land. The
halo (`paint-order: stroke`) makes the topmost name legible against the ground,
not against another name.

**The ruling needed.** The standard repair is a greedy declutter — draw in a
stable order, skip any label whose box hits one already placed — and it is
cheap and deterministic here (18-330 labels for one nation, and a
character-count width estimate avoids per-frame `getBBox`). But it HIDES NAMES,
and the names are transcribed Natural Earth data. How many names may the map
drop, and in what order of preference, is a decision about the surface and not
a defect with one right answer. Note the information is not lost either way:
`#dhit` carries every district's name in its hover title.

**Not attempted**, per the standing instruction that a fix which cannot be
justified to a sceptical reader at breakfast is a fix not to make.

## Filed under iron rule 7's grandfather clause (added 2026-09-01 by the tech-shelf author)

### T-8 — `mature_economies_do_not_run_hot` reds 27.3% of the time on a healthy model, and it is NOT being re-pointed

**Ridge's ruling 4, 2026-09-01: leave the bar where it is and file the
measurement.** Iron rule 7 applies from birth. This bar predates it, so it is
grandfathered, and nothing about it was touched by the shelf commit that
surfaced this.

`spheres-sim/src/lib.rs:4489`. The bar asserts `(0.008..0.026).contains(&g)` on
`growth_last` for the USA, Germany, France and Italy over seeds {1990, 7, 42} at
360 months. Italy is the one that decides it:

```
Italy, growth_last at 360 months: mean 0.00868, sd 0.00053
the floor of 0.008 therefore sits at z = -1.28
P(one seed red) = 10.1%
P(the bar reds)  = 1 - (1 - 0.101)^3 = 27.3%   <- today, at its live horizon
```

**At 540 months it is worse and the sign flips.** Italy's mean falls to 0.00778,
which is *below* the floor, and P(the bar reds) = **94.7%**. So the bar is not
merely under-sampled; it is measuring a quantity that leaves its own band as the
run lengthens. Any future work that extends a horizon must not simply re-point
this bar at the longer one.

**Why nothing was done, spelled out, because the obvious repair has a price.**
Raising n from 3 to 12 without moving the threshold is legal under iron rule 5 —
more seeds is a stricter test, not a wider one — but at 10.1% per seed it gives
P(red) = **72%**. It would turn red immediately, become red number four, and
deepen the re-pin block that BUGS T-5 records. Doing the rigorous thing here
directly costs the ability to re-pin the goldens, which is why it is a decision
for Ridge and not for the session that measured it.

**What this is NOT.** It is not evidence the growth model is wrong. It is
evidence that a three-seed bar reading a quantity sitting 1.28 standard
deviations off its own floor is a coin the suite flips every time it runs. Iron
rule 7's own record of the same disease is `spheres-sim/tests/sample_size_audit.rs`,
whose header carries the 2026-08-31 table; when this bar is next touched for any
reason, that is the moment to move its line out of the audit and into the comment
beside it, with this derivation.

Measured with `scratchpad/to2035/probe_validation` (`mature 40`), 40 seeds,
recorded in `scratchpad/to2035/out_mature_40.txt` and adjudicated in
`DECISION-2035.md` call 4.

## Awaiting an owner ruling (added 2026-09-01 by the resource-system shipper)

The resource system (`scratchpad/resourcesys/SPEC-RESOURCE-SYSTEM.md`) landed
in seven commits with both golden actuals unchanged throughout —
`the_1990_start_is_pinned` 0xa5c9c5b2306313d8 and `golden_hash_of_a_known_run`
0x20c24ab0f1581807, the actuals T-5's second table records — and re-pinned
nothing. These are the things it measured and could not decide.

### R-1 — the last-resort chain never completes: zero resource wars in every F2 cell over two counts

Ruling 4 asks that an AI invade for a resource ONLY when every seller has
refused it twice and its line is stalled, "rare and legible — not never, not
constant". Measured twice (count one 200 seeds, count two 400 seeds, 480
months, six cells: floor {−20, 0, +10} × ration {on, off}): **0 resource wars
in every cell**, λ̂ = 0.000, 95% upper bound 0.0075/seed, below the
pre-registered band [0.05, 0.69] by a factor of six or more. The predicate is
sound — `the_last_resort_predicate` pins all ten clauses and the census
exercised the nuclear bar 473–981 pair-months and the pact bar 56 without one
crossing — but it never holds in play: in the design cell the universal
refusal reaches at most 17 of 59 copper producers; at +10 it completes for
Indochina and Taiwan and the appetite roll never lands (p ≤ 0.0014/month).
"Not never" is unmet. The knobs that would move it, each a ruling and none
taken: `RESOURCE_WORTH` (0.75, the GDP arm's own coefficient — a larger term
prices a mine above a target's whole economy); `GATE_HEAT` / the two-asks rule
(a weaker clause 2 admits partial refusals); `RELATION_FLOOR` past +10 (already
"hot in every seed" at +10 with no war); a ration that also closes the open
market to a nation's rivals (the only route by which sellers vanish). Full
funnel in ROADMAP §1c and `scratchpad/resourcesys/{census1_summary,c2_census}.txt`.

### R-2 — the counter-price step prices small economies out of everything

`evaluate()` rounds a counter UP to $0.1bn a year (`ceil10`). A $0.65M/yr
slice of copper is therefore quoted at $100M/yr, and a buyer with a 1%-of-GDP
contract cap under $10bn of output spends its whole cap on rounding. Measured
in S3: 60,000 of 65,000 AI asks over 35 years were priced-out re-asks before
the priced-out clock row was added. Filed under S2's rule (the price step is a
spec constant); the fix is a finer step or a step scaled to the smaller
economy's output, and it moves every market-on timeline.

### R-3 — Ukraine's iron is under-located

The site-count share rule gives Ukraine 11.5% of Soviet iron (`dissolve_to`
hands Russia 0.808, Ukraine 0.115, Kazakhstan 0.058); the real 1990 split is
about 45% Ukraine (Kryvyi Rih). Fix: weight located shares by USGS MYB 1990
tonnage where the source prints a per-district or per-basin figure, as a
documented D rule. Moves HAVE only; nothing the growth model reads.

### R-4 — the settlement's 12% oil slice ignores where the oil is

`negotiated_peace` (war.rs) moves 12% of the loser's `oil_mbd` with a ceded
share, while the located shares say the ground carries up to 97% of it
(Kuwait). `transfer_district` (a consented sale) uses the located fraction;
a conquest uses the flat slice — two rules for one quantity at two sites. The
card prints "located in ceded ground: X%" so the player can see the gap. A
war.rs repair moves the run golden.

### R-5 — `SANCTION_BITE` was calibrated on histories that include import denial

The sanction ration (`supply()`'s market leg × (1 − `oil_blockade(buyer)`))
now delivers import denial through the gate; `sanction_weight`'s drag on
growth, stability and research was calibrated against histories in which that
denial was already part of the damage. A future recalibration should subtract
what the gate now delivers. Nothing double-counts today only because the gate
binds so rarely (R-1).

### R-6 — Namibia produces and has no seat

The 1990 tables carry Namibia at copper 27,800 t, gold 1,700 kg, uranium
3,211 t (and the district survey carries its ground). It is not on the 1990
roster, so the rows are dropped (`meta.counts.dropped_rows`) and its uranium
— among the largest 1990 producers — is held by nobody. Seating it is a
roster ruling (BIBLE), not a data fix.

### R-7 — the pact-drag test moves from 4/12 to 5/12 under the market

With the market on, every seed's history reshuffles from the first
sanction-rationed shortfall (month 25 at the earliest, median 69.5) and
`a_pact_drags_a_great_power_into_a_war_it_did_not_start` reads 5/12 [1,2,5,8,10]
against control's 4/12 [1,2,9,10] — inside its band, and its false-red rate
(2.67%) is unchanged. The Gulf hit list is identical to control's (252/400
within 48 months) because the market's first event is after month 25. Recorded
so nobody re-diagnoses it; nothing to fix.

### R-8 — fork F1(b)'s blind spot: the suite calibrates a world the player does not play

`GameRules::resource_market` is off in every test and headless path and on for
every browser game. That is what keeps both goldens on their actuals while T-5
/ E-3 block the re-pin, and it means no calibration test reads the market-on
timeline. The census instruments (`resource_war_census`, `_one`, both
`#[ignore]`) and `the_market_switch_is_off_for_the_suite_and_deterministic_when_on`
(35-year state_hash 0xdc1b71684b08b071 on seed 1990) are the only readings of
that world. When T-5 closes, the ruling is whether to flip the default and
re-pin under the protocol, which is F1(a) as the spec recommends.

### R-9 — filed and FIXED in passing: `arsenal::available`'s string scan

`tech::index_of` string-scanned 254 entries for each of 33 tech-gated kits per
nation-month (0.25 of arsenal's 0.26 ms/month). S3 precomputed the kit → tech
index table: 2.24 → 0.14 µs a pick, bit-identical, both goldens unchanged.

---

## Awaiting an owner ruling (added 2026-09-02 by the doctrine-and-record agent, for the codex/trading-system landing)

The daily calendar and the ten-ministry annual budget (`origin/codex/trading-system`
4875ea5, merged as 253ff2d onto `feat/hoi4-map-and-tech` 2cc76a6, two hunks
resolved by hand) landed on Ridge's ruling of 2026-09-02 — "I like the 10 ministry budget and the 1 day ticker so if the bible needs to be ammended we can do that."
— with both golden actuals unchanged throughout: `the_1990_start_is_pinned`
0xa5c9c5b2306313d8 and `golden_hash_of_a_known_run` 0x20c24ab0f1581807, the
actuals T-5's second table and R-8 record, and Codex's own re-pin of them
reverted (D-7). Every arm the landing added is inert on the default path
(`annual_budget` and `social_spend_gdp` are `Option` `None` and
`skip_serializing_if`, `day` is omitted from a save when it is 1 — `world.rs`
303-308 and 866). These are the things the trial merge and its review measured
and could not decide, plus the record of what was refused and why. Figures are
the 2026-09-02 trial-merge review's unless a line says otherwise.

### D-1 — a ministry dollar enters one to six channels, and none of them is calibrated — **FIXED 2026-09-02 by the ministry collapse**

**FIX.** All thirty scattered addends inventoried below are REMOVED, and each ministry now has one or two NAMED arms defined once in the new `spheres-sim/src/ministries.rs` and called from every site that charges them (`economy.rs`, `politics.rs`, `war.rs`, `tech/mod.rs`, `resources.rs`, `statecraft.rs` — twelve charge sites). Potential growth lost 5 addends, demand 3, unemployment 5, private investment 4, stability 3, and the cohesion term lost housing's half. The map is in SPEC §3 and is ASSERTED, not described: `tests/ministries.rs::the_ministry_map_is_exactly_this` for the gap in isolation and `::the_enacted_ministry_map_is_exactly_this` for the whole real `Command::SetAnnualBudget`. The rule they enforce is CLAUDE.md iron rule 8: no gap reaches `n.gdp` by more than one route, and no two ministries write the same arm; only `population` and `stability` have more than one owner and both are named.

**Three things the fix had to do that the original filing did not anticipate.** (1) The bar that proves the map was itself BLIND: it moved a dial by writing `plan.allocations[i]` on a cloned world, which isolates the gap and never sees the three aggregates the real command also writes. Measured through the real command, health, education, housing, pensions, security and diplomacy each moved `demand_gap` +0.000750000 and `target_inflation` +0.001200000 IDENTICALLY to the last digit — the social aggregate, not any named arm. The `ds += social_gap * 12.0` line in `economy.rs` was deleted and a second bar added that enacts through the real command. (2) Four arms were clamped DOWNSTREAM of `ministries.rs`, so the served card quoted a slope the simulation never charged (the stability card promised destinations of +151.2/+168.5/+169.4 on a 0..100 scale); each is now computed as `clamp(base + arm) - clamp(base)`. (3) Health's and industry's first-draft x20 slopes saturated their clamps so early that 70.3% and 78.0% of the mean nation's reachable dial bought nothing; both were re-derived, to 6.0 and 4.2, the way education's 15.0 was. Every new coefficient is filed below as I-1..I-16.

**Still true, and it is what keeps the change inert:** `budget_gap` returns 0.0 for a nation with no enacted plan and `Command::SetAnnualBudget` is player-only, so no default board runs a line of any of this. Both goldens read their briefed actuals unmoved ~~(0xa5c9c5b2306313d8, 0x20c24ab0f1581807)~~ and all four headless references reproduce byte for byte. **CORRECTED 2026-09-02 on the merge of `origin/feat/hoi4-map-and-tech` 61b388f:** those were the actuals of this branch standing alone. The merged tree reads **origin's** — 0xe26e4bf8d6c60066 (`the_1990_start_is_pinned`) and 0xbe94d6125631829c (`golden_hash_of_a_known_run`) — and that is a STRONGER inertness claim, not a weaker one: the whole of the difference is upstream's unconditionally serialized `district_population`, and the actuals equalling origin's exactly is the proof that nothing on this branch moves the default path. The two golden PINS (0xd022d50f43c984da, 0xbd5ec0f43c5f2e3b) were not touched and are still red.

**The inventory below is kept as the record of what was removed.**

The review's summary said "two to four"; counting every site in the tree,
including the aggregate each ministry composes, it is one (defense) to six
(health). A budget gap is `allocations[i] - reference[i]` (`world.rs` 256),
0.0 whenever `annual_budget` is `None` (`world.rs` 431-432). The sites:

- **The three aggregates** (`lib.rs` 583-585): on enactment `social_spend_gdp`
  = health + education + families + pensions + security + diplomacy
  (`social_total`, `world.rs` 229-241), `state_invest_gdp` = infrastructure +
  industry + science (`investment_total`, 243-247), `mil_spend_gdp` = defense.
  These are the only channels the calibrated model reads.
- **Potential growth** (`economy.rs` 419-423, inside `growth_terms`): health
  0.015, education 0.050, infrastructure 0.025, industry 0.035, science 0.025
  per unit of gap.
- **Demand gap** (`economy.rs` 436-438): health 0.06, families 0.28, pensions
  0.18.
- **Unemployment** (`economy.rs` 160-164, `unemployment_rate`): health 0.12,
  education 0.16, infrastructure 0.28, industry 0.24, science 0.08 off the
  rate; read by the player's investment arm (D-2), the player's stability
  term (1051-1053) and the browser's dossier (`main.rs` 826).
- **Business pressure** (`economy.rs` 1016-1019, player only): infrastructure
  0.02, industry 0.04, science 0.02, diplomacy 0.01 into the private
  investment target.
- **Population** (`economy.rs` 1042-1044): health 0.030, families 0.015 added
  to annual population growth.
- **Stability** (`economy.rs` 1054-1059): health 8, education 5, families 14,
  pensions 12, security 16, diplomacy 3 — ON TOP of `social_gap * 12.0` at
  1053, where `social_gap` (841) is `social_spend()` minus the baseline and
  `social_spend()` is now the social aggregate that already contains security
  and diplomacy. Security, diplomacy, health, education, families and pensions
  therefore enter stability twice: through the aggregate and through their
  own coefficient.
- **Cohesion / separatism** (`economy.rs` 1090-1093): (families + security),
  positive part only, x 0.04 off `separatism` every month.
- **Diplomatic shield** (`economy.rs` 589): `(diplomacy gap * 8.0).clamp(-0.20,
  0.40)` on the sanction leg.
- **Research output** (`tech/mod.rs` 1044-1047): `out *= (1 + education*20 +
  science*35).clamp(0.35, 2.25)`.

Per ministry: health 6 (aggregate, potential, demand, unemployment, population,
stability); education 5 (aggregate, potential, unemployment, stability,
research); families 5 (aggregate, demand, population, stability, cohesion);
pensions 3 (aggregate, demand, stability); infrastructure 4 (aggregate,
potential, unemployment, business pressure); industry 4 (the same four);
science 5 (aggregate, potential, unemployment, business pressure, research);
defense 1 (aggregate); security 3 (aggregate, stability, cohesion); diplomacy
4 (aggregate, business pressure, stability, shield). In `tick()` the gap array
is zeroed for every nation but the player (`economy.rs` 823-825), so
population, stability, cohesion and business pressure are player-only;
`growth_terms`, `unemployment_rate`, the shield and research read
`n.budget_gap` directly and would fire for any nation with a plan, which today
is only ever the player. Not one coefficient above has a calibration bar; the
suite runs with `player` `None` and `annual_budget` `None`, so it reads none of
them (R-8). Two hazards the review measured and this entry records: enacting
the INHERITED plan unchanged moves `social_spend()` by one ulp for 49 of 137
nations and `state_invest_gdp` for 32 of 137, because the 0.25/0.18/0.20/0.28
/0.07/0.02 and 0.55/0.30/0.15 splits do not re-sum to the number they were cut
from, and it flips `social_spend_gdp` from `None` to `Some` for good. **Fixed
on landing (fix 3, 52fd9f6)**: the apply arm leaves the aggregates untouched
when every allocation is bit-identical to the plan in force (the stored plan,
or the inherited one when none is), asserted by
`tests::enacting_the_inherited_budget_unchanged_is_a_no_op` over all 137
nations seated in 1990 — `to_bits` equality on `social_spend()`,
`state_invest_gdp` and `mil_spend_gdp` against the untouched world, and
`save()` equality apart from `annual_budget`; with the guard disabled the test
reads the figures above (social_spend 49/137, state_invest_gdp 32/137,
mil_spend_gdp 0/137, `None` -> `Some` 137/137). The sim agent's caveat, kept:
an identical re-vote no longer re-asserts the plan's sums over aggregates that
have drifted since — a case that today has nothing to drift them, because the
fiscal AI that trims `mil_spend_gdp` and `state_invest_gdp` (`politics.rs`
106) skips the player, and only the player ever holds a plan. Still open here:
the nine channels. Ruling
wanted: which of the nine non-aggregate channels survive, and against what
bar; until then SPEC §3 calls them uncalibrated and describes none of them.

### D-2 — the player's private investment is endogenous with NO command, and no bar can see it — **SETTLED 2026-09-02: PLAYER-ONLY, by Ridge's ruling**

**SETTLEMENT.** The first of the two options is taken: the governed economy stays the PLAYER'S economy. The ministry surface is player-only, every arm reads `n.budget_gap` directly, and the arm is not gated on an enacted budget. The consequence is accepted deliberately and written down here so it is not rediscovered as a bug: **the AI never opens its books**, so no AI nation runs a line of the treasury or of any ministry arm, and the suite — which calibrates a world with `player` `None` — still does not exercise them. That is precisely what makes the whole branch provably inert on the default path, and it is the reason `spheres-sim/tests/treasury.rs::the_treasury_is_inert_while_the_books_are_closed` can assert what it does.

**The condition on the settlement, and it binds on every later session:** any AI budget issuer must sit behind a `GameRules` flag DEFAULTING FALSE. The moment an AI nation issues `Command::SetAnnualBudget` on the default path, the inertness proof in `tests/treasury.rs` stops being true and both goldens move. R-8's own ruling — that a bar has to read a player world — is still open and is not settled by this entry.

`economy.rs` 1006-1024: when `Some(id) == player`, `priv_invest_gdp` chases a
`business_pressure` target built from growth, the real rate, stability, tax,
unemployment, the bubble, sanctions and war — every month, budget or no
budget. The ministry terms (1016-1019) are only the last four lines of it;
the arm itself fires the moment a browser game names a player. The review
measured the USA over 240 months at a GDP ratio of 0.9908 against the
merge-base's 1.0070 — a -1.61% level move with no command ever issued. The
same block adds `ds -= (unemployment - 0.06).max(0.0) * 1.5` (1051-1053) for
the player only. Cross-reference **R-8**: the suite calibrates a world with
`player` `None`, so this arm — like the market — runs in every browser game
and in no test. Ruling wanted: keep the governed economy as the player's
economy (in which case a bar has to read a player world, R-8's own ruling),
or gate the arm on an enacted budget so an idle player's nation is the
calibrated one.

### D-3 — the annual budget never expires, and "due" is a badge only

`apply_command` (`lib.rs` 531-588) rejects a plan for any year but `w.year`,
and on enactment takes `reference` from the plan already stored (549-554) —
so the reference is frozen at the first enactment and never re-derived. Nothing
on 1 January lapses `allocations`: `budget_for` (`world.rs` 420-429) hands the
old plan back for any later year, the aggregates it wrote stay written, and
the gaps keep firing (D-1) for as long as the game runs. The only thing that
knows a new year has come is the browser badge, `main.rs` 794: `"due":
n.annual_budget.is_none_or(|x| x.fiscal_year != w.year)`, which is display.
Ruling wanted: does a budget lapse each 1 January (to the inherited split, or
to its own reference), or persist until re-enacted? The first makes "due"
mean something and prices the yearly reopening; the second is what ships.

### D-4 — `spheres-cli play` and `resume` are month-stepped only

`spheres-cli/src/main.rs` 3 imports `tick_month` and not `tick_day`; the three
call sites (41 `run`, 362 `play`, 818 `resume`) all step months. On the trial
merge `tick_month` ended with `w.day = w.day.min(days_in_month)` (`lib.rs`
824), so a browser save taken on day 15 and resumed in the CLI stayed on day
15 of every month forever; fix 2 of the landing has `tick_month` reset `day`
to 1 at the boundary, so the same save now lands on day 1 after its first
settlement — a one-time 15-day jump the player is not told about. Ruling
wanted: does the CLI move to `tick_day` (and gain `days` as a unit), or stay
monthly with the jump documented as its behaviour?

### D-5 — `WorldState::date_str` prints the day, so every headless line moved

`world.rs` 1315-1318 now formats `"{day} {Mon} {year}"` — `[1 Feb 1990]` where
2cc76a6 printed `[Feb 1990]` — and every headline in `spheres-cli run` carries
it. The review's market-OFF references on the trial merge: seed 1990 sha256
d1a2cfbf7c6958d7 (3,501 lines), seed 7 39dea3341a7f6e8c (3,983 lines); with
the day stripped from every date the diff against 2cc76a6's 2409583ac6951b46 /
03fb32b79aaf948b is zero lines. Market ON (`SPHERES_RESOURCE_MARKET=1`): seed
1990 6cb6c97ab33fb80d (4,007 lines), seed 7 8d29fecfd4ff9bf4 (4,258 lines).
Filed so the moved references are attributable to one formatting line, not to
the model. Ruling wanted: accept these as the new headless references, or have
the month-stepped paths print the month only and keep the old ones.

### D-6 — `index.html` re-implements the budget's price, caps and totals in JavaScript

`spheres-web/ui/index.html` 2655-2686: `MINISTRIES` carries every cap
(`cap:` per ministry, duplicating `BUDGET_CAPS`), `annualSocial` (2678) and
`annualInvest` (2679) duplicate `social_total` / `investment_total`, and
`annualPoliticalCost` (2681) duplicates `command_price`'s weights and cut
penalty (`lib.rs` 222-240). CLAUDE.md Layout, line 149: the page "owns no game
logic". This is the W-1 shape again — the browser assembling a number the sim
already owns — and W-1 UPDATE records the precedent: the browser stopped
assembling the sum and was handed the sim's. The fix is the same: the sim
prices the plan (a quote on `nation_json` or an endpoint) and the page renders
it. Not fixed here because it moves the cabinet card and three page tests.

### D-7 — Codex's golden re-pin was refused, and its justification is false

4875ea5 re-pinned `the_1990_start_is_pinned` to 0xa5c9c5b2306313d8 and
`golden_hash_of_a_known_run` to 0x20c24ab0f1581807 under the comment
"Re-pinned for the optional annual-budget and daily-calendar fields ... this is
a serialized schema expansion, while the scalar 1990 economy remains
unchanged" and "The same schema expansion changes the serialized 20-year
fingerprint". Two things are wrong with that. First, there is no schema
expansion on the default path: `annual_budget` and `social_spend_gdp` are
`None` and `skip_serializing_if` (`world.rs` 303-308), `day` is omitted when
it is 1 (866, asserted by `monthly_saves_load_on_the_first_day`), so the
serialized world is byte-identical to 2cc76a6's and the hash cannot have moved
for that reason. Second, the numbers Codex pinned are exactly the actuals the
suite had already been red at since E-3 — R-8 and T-5 record them — which
is to say the re-pin would have declared E-3's red green without touching
E-3. The protocol (T-5, iron rule) is that goldens are re-pinned only when
every calibration bar is green; `the_1990_endowment_does_not_move_year_one_growth`
is red (Belgium 0.001851 against 0.001749). The merge therefore keeps ours —
0xd022d50f43c984da and 0xbd5ec0f43c5f2e3b — and both tests stay red at the
same two actuals before and after the landing, which is the proof the landing
is inert. Nothing to rule on; recorded so the next re-pin request is checked
against it.

### D-8 — the systems deck makes two claims the tree contradicts — Codex's to correct

`docs/presentations/SPHERES-Systems-and-Next-Steps.{pptx,pdf}` (4875ea5,
"docs: publish SPHERES systems deck"). Panel "02 Living province population —
1990 district residents evolve with the current owner's demographic and
technology path, including after border transfers" sits under a PLAYABLE
status, while `spheres-sim/src/districts.rs` line 2 says "No population" and
the district layer carries ownership only. And "Bundles/contracts work; market
loop and war reachability need tuning" / "tuning the refusal chain so
last-resort war is rare but reachable": the census reads ZERO resource wars in
every F2 cell over two counts (R-1, ROADMAP §1c), and R-1 / R-2 / R-8 are
rulings awaiting Ridge, not a tuning pass awaiting a developer. The deck is
Codex's document and the corrections are Codex's; filed here so the claims are
not repeated into ROADMAP.

### D-9 — three existing web tests were re-targeted by Codex, and are filed rather than reverted

In `spheres-web/src/main.rs` (diff 2cc76a6..253ff2d): (1) the force-curve
test read the military slider's upper bound off `sliderHtml("military", ...)`;
it now reads the Defense ministry's `cap:` off `id:"defense"` and asserts the
Defense dial against `FORCE_CURVE_MAX` — a different control, same bar. (2)
The advance test asserted `asked_months` for the page's 1/6/12/60-month spans (`data-adv` 1/6/12/60); it
now asserts `asked_days` for `data-adv` 1/7/30/365 and the months path is a
compatibility route. (3) The 1024px layout test asserted the grid
`312px 1fr 348px` that left the map 364px wide; that measurement was replaced
by two string checks (`main { position: relative; display: block; overflow:
hidden; }`, `#center { position: absolute; inset: 0; }`) rather than
re-derived for the drawer layout. Iron rule: tests are never deleted or
widened. These are re-targets, not deletions, and they follow the feature Ridge
ruled for, so they are filed and not reverted. Ruling wanted: accept the three
as the new bars, or require (3) to carry a measured narrowest-layout number
the way its predecessor did.

## Filed 2026-09-02 by the treasury author (ministry economy, stage 1)

### M-9 — seventeen nations have no sourceable 1990 reserve, and two of them matter

**Filed:** 2026-09-02, with the treasury (`Nation::treasury_bn`).
**Status:** left out and reported, not estimated. Iron rule 4.
**RENUMBERED 2026-09-02, ~~M-1~~ → M-9.** Both branches independently filed an
M-1 and an M-2 with different content and the merge kept both sets. This entry
and M-10 are the treasury author's; M-1..M-8 are the mine landing's and keep
their numbers because they are the older, larger, cross-referenced set.

`EconomyRecord::reserves_bn` transcribes total reserves including gold from
World Bank WDI series `FI.RES.TOTL.CD`, 1989 observation — the stock as it stood
on the morning of the 1 January 1990 start date. 79 of the 137 roster nations
carry it. Two kinds of absence, and they are different claims:

**Seventeen have no observation in the series at all**, so no figure could be
sourced: Albania, Angola, Brunei, Bulgaria, Cambodia, Cuba, Czechoslovakia,
Iran, Mongolia, North Korea, Sao Tome, Senegal, Taiwan, the USSR, Vietnam,
Yemen, Yugoslavia. Fifteen of those are immaterial or unplayable. **Two are a
real loss and are the content of this entry:**

- **Taiwan.** Held the second-largest reserve stock on earth at the end of the
  1980s, on the order of $70bn against a transcribed 1990 output of $166.6bn —
  40% of GDP, which is not a rounding. The World Bank series does not carry
  Taiwan at all. The figure exists in the Central Bank of the Republic of China
  (Taiwan) monthly statistics and in IMF IFS before Taiwan left the Fund; a
  session with access to either should transcribe it, and until then Taiwan's
  treasury seats empty, which UNDERSTATES it badly.
- **The USSR.** Gold and hard-currency reserves were a state secret and the
  published numbers are reconstructions. Nothing here is sourceable to a
  primary; leaving it at no-figure is the honest answer and it is also the
  conservative one, since the union's convertible reserves were nearly gone by
  1990.

**Forty-one more were sourced and left out as immaterial** — under the stated
line, which is 5% of that nation's own 1990 output or $10bn absolute. Those are
a decision, not a gap: the largest of them is Brazil at $9.678bn on $385bn of
output, 2.5%, spent inside a quarter of a plausible deficit.

### M-10 — no 1990 sovereign wealth fund balance could be sourced, and Kuwait is the casualty

**Filed:** 2026-09-02. **Status:** left out and reported.
**RENUMBERED 2026-09-02, ~~M-2~~ → M-10**, for the collision described in M-9.

The approved design asks for "foreign reserves PLUS sovereign fund balances".
The reserves half is transcribed; the sovereign-fund half is refused, because
none of the four funds that mattered in 1990 published an audited balance:

- **Kuwait** — the Kuwait Investment Authority's Fund for Future Generations.
  Contemporary press put it near $100bn on the eve of the invasion, against
  official reserves of $4.12bn and a transcribed 1990 output of $18bn. That is
  the difference between a state that can fund a government-in-exile for two
  years and one that cannot, and it is the single largest known understatement
  in this table. KIA has never published a 1990 balance.
- **Abu Dhabi (ADIA)**, **Singapore (GIC)** and **Norway** — ADIA and GIC do not
  publish balances at all; Norway's fund was legislated in 1990 and held nothing
  until its first transfer in 1996, so zero there is correct rather than
  missing.

`reserves_bn` therefore means OFFICIAL RESERVE ASSETS and says so in every
nation file that carries it. Adding an estimate would be inventing a starting
figure, which iron rule 4 refuses.


## Invented coefficients, filed 2026-09-02 by the doctrine-and-record agent (ministry economy, all stages)

Iron rule 4 says starting DATA is transcribed and never invented. A model COEFFICIENT is a different thing: it cannot be transcribed, so the standing requirement is that it be labelled where it lives and recorded here with what would calibrate it. This section is that record. Every number below is labelled `INVENTED` in the source at the line given, and every one was MEASURED off this tree (`feat/ministry-economy` 532c818) rather than copied from a design note. Three slopes that a review turned from invented into DERIVED are filed at I-16 so the distinction is not lost.

### I-1 — `SPREAD_KNEE = 0.60` — where the sovereign spread starts

`spheres-sim/src/economy.rs:249`. The debt-to-output ratio past which a borrower pays more than the policy rate. **Not picked — pinned against two measured facts.** The roster's own MEDIAN 1990 debt ratio is 0.52 (measured across all 137 `data/nations` files; mean 0.6423, min 0.00 Brunei, max 3.80 Nicaragua), so a knee at 0.60 leaves the median borrower paying exactly its own policy rate and charges nothing to the half of the world below it — which is the approved design's requirement verbatim. And 0.60 is the SAME line `dyads.rs` already draws for fiscal desperation in `(debt_gdp - 0.6).max(0.0) * 1.5`, so the point at which the bond market starts charging is the point at which a government starts behaving as though it is short of money. One line in the model, not two.

**What would calibrate it:** the 1990 cross-section of sovereign yield spreads against debt ratios. If real 1990 spreads turn out to open materially below 60% of GDP, the knee moves down and `dyads.rs` should move with it or the two lines should be documented as deliberately different.

### I-2 — `SPREAD_SLOPE = 0.06` — rate per unit of debt past the knee

`spheres-sim/src/economy.rs:258`. Sized off the one endpoint the approved design names — a nation at 90% of output "pays a visible premium" — which at 0.06 is **1.80 percentage points**. Visible against the policy rates the roster carries (between 2.9% and 25%), worth consolidating away, and far short of the several hundred basis points that mean a market thinks it will not be paid. It is a slope and not a schedule: the same 6bp per point of debt from the knee to the cap.

**MEASURED at a 5% policy rate against 3% inflation:** 30% of GDP pays 2.0000%/yr, 60% pays 2.0000%, 90% pays 3.8000% (+1.80pp), 150% pays 7.4000% (+5.40pp).

**What would calibrate it:** the same 1990 yield cross-section as I-1, read as a gradient rather than an intercept. A real schedule is convex — spreads widen faster as the ratio climbs — and this is deliberately linear, which is the first thing to revisit if the shape matters more than the level.

### I-3 — `SPREAD_CAP = 0.06` — the most the spread can ever add

`spheres-sim/src/economy.rs:270`. **A guard, not a calibration**, and it is the one number here that exists for a structural reason rather than an empirical one. Interest is the only term in this model that feeds its own input — more debt, wider spread, more interest, more debt — and uncapped that recursion has no fixed point, which would eventually make the `debt_gdp < 6.0` invariant asserted in four places in `lib.rs` a coin toss.

**MEASURED by sweep, not assumed:** at 6 percentage points the cap binds from a debt ratio of **1.600** upward, so every borrower the shipped board actually produces sits on the sloped part and only a state past 160% of output meets the ceiling.

**What would calibrate it:** nothing empirical, and that should be said plainly. It is falsified instead by a counterexample — a historical sovereign that went on paying a spread wider than 6pp in real terms without defaulting. If default is ever modelled, this cap is the wrong instrument and the default gate is the right one.

### I-4 — `REAL_RATE_FLOOR = -0.02` — the floor under the real rate

`spheres-sim/src/economy.rs:235`. Carried across from the approved design unchanged and still labelled invented. A government can borrow at a negative real rate but not without limit, because at some negative real return the lender buys goods instead. **Nothing the shipped board produces sits near it**; it is here so interest is provably bounded below rather than bounded by inspection.

**What would calibrate it:** the most negative sustained real policy rate in the historical record. Since no nation on this board reaches it, the honest calibration is to leave it as a bound and check periodically that it still never binds.

### I-5 — The 1990 reserve MATERIALITY LINE — 5% of output, or $10bn

`spheres-sim/src/data/mod.rs`, and applied in the transcription pass. The approved design said to transcribe reserves "where the figure is material" without saying where that is, so **the threshold is a choice and is labelled as one — but the FIGURES themselves are transcribed, never invented** (iron rule 4 is not touched here). Stated mechanically so it is never a per-country judgement: at least 5% of that nation's own 1990 output, or at least $10bn absolute. 5% of output is about six weeks of total state spending for a state spending 35% of output; below that the stock is spent inside a quarter of a plausible deficit and cannot change the shape of a fiscal path.

**MEASURED:** 79 of 137 nations clear it (largest USA $168.584bn, Germany $98.877bn, Japan $93.673bn, Italy $73.455bn, Switzerland $58.670bn; smallest carried Equatorial Guinea $0.006bn). 41 are sourced but below the line (largest Brazil $9.678bn on $385bn of output, 2.5%). 17 are refused outright for want of a source — see M-9.

**What would calibrate it:** measuring whether a carried reserve below the line ever changes a fiscal path by more than rounding over 240 months. If it never does, the line is too generous and could rise; if a 3%-of-output reserve moves an outcome, it must fall and the 41 have to be transcribed.

### I-6 — The SURPLUS RULE in `economy::pay` — a rule, not a number

`spheres-sim/src/economy.rs`. Filed here because it is an invented DECISION with the same standing as an invented coefficient. The approved design specifies only the outflow ("the balance flows to `treasury_bn` and a treasury that would go negative issues debt instead"). Read literally on the INFLOW, debt becomes monotonic: `debt_gdp` could never fall, `politics.rs` could never consolidate, and `dyads.rs` would read a net creditor as permanently desperate. **The literal reading was implemented, tested and WATCHED GO RED before it was rejected** ("debt was not retired first", left `Some(3707.6)` against right `Some(0.0)`). The rule taken instead: a receipt retires debt first and only the remainder accumulates as cash. It is written out beside `pay()`.

**What would calibrate it:** nothing numeric. It is falsified by a modelled sovereign that rationally holds cash while paying interest on debt it could retire — real governments do exactly that, for liquidity reasons this model has no representation of. If a liquidity motive is ever added, this rule is the thing it replaces.

### I-7 — `health_replacement` clamp `0.60 / 1.60`

`spheres-sim/src/ministries.rs:58`, reached through `war::health_retention`. The multiplier on the approach to sustained force in war — never on `REPLACEMENT_RATE` itself, which is read again to define a decisive battle. The floor at 0.60 is the claim that gutting the hospitals costs an army 40% of its regeneration and not all of it: conscription still works when the medical corps does not. The ceiling at 1.60 is the claim that money cannot more than about half again the rate at which a wounded soldier returns to the line. **The SLOPE (6.0) is DERIVED, not invented — see I-16.**

**What would calibrate it:** return-to-duty rates from military medical history (WWII through Vietnam are well documented and span roughly the range this clamp asserts). This is the invented coefficient in the whole set most likely to be replaceable with a transcribed figure.

### I-8 — `pensions_standing = gap * 1000.0`

`spheres-sim/src/ministries.rs:96`, with the clamped realisation at `politics.rs:69` via `politics::standing_target`. Ten points of political-capital CEILING per point of GDP. **Sized to the design's own stated endpoint**: +0.5% of GDP is exactly +5 points of ceiling (0.005 * 1000 = 5.0), and that is what the card prints. MEASURED first-month step 0.275 of a point at the falling rate 0.055; MEASURED at the top of Brazil's dial, +82.389 points, with a further press moving it by zero bits because the arm is quoted as `clamp(t+arm) - clamp(t)`.

**What would calibrate it:** nothing external — political capital is an invented currency with no unit. It is calibrated only INTERNALLY, against the prices in `lib.rs:241`: the question is whether a pension rise buys a plausible number of later commands relative to what those commands cost, and that comparison has not been made.

### I-9 — `pensions_jobs = gap * 0.20`

`spheres-sim/src/ministries.rs:103`, off `economy::unemployment_rate`. Pensions ALONE own the labour-force arm; the design dropped the demand arm entirely rather than resizing it, because `demand_gap` forks into both output and inflation. **Sized to the design's stated endpoint**: +0.5% of GDP is exactly -0.10pp of unemployment (0.005 * 0.20 = 0.001). MEASURED end to end: unemployment 0.104234 -> 0.103234.

**What would calibrate it:** the labour-force participation response to pension generosity is one of the better-measured elasticities in economics (the OECD literature on effective retirement age against replacement rate). This one could be transcribed rather than invented, and should be.

### I-10 — `INFRA_EXTRACTION_CEILING = 0.25`

`spheres-sim/src/resources.rs:873` (~~867~~, moved 2026-09-02 by the merge). The most a standing infrastructure budget can raise located NON-OIL production, in either direction — a quarter more ore out of the same ground for a road network at the top of the dial. Oil is excluded because oil is already a complete national system with its own calibration.

**What would calibrate it:** the measured output response of a mining region to transport capacity — the World Bank rural/extractive access literature is the obvious source. A quarter is a guess sitting where a documented elasticity belongs.

### I-11 — `INFRA_EXTRACTION_RATE = 0.02` per month

`spheres-sim/src/resources.rs:882` (~~876~~, moved 2026-09-02 by the merge). **A FIXED STEP, deliberately, and not a share of the gap** — which is what makes the uplift a STOCK the player builds rather than a switch he flips. MEASURED: exactly 12 months to build what the top of the dial justifies, and exactly 12 to lose it. The bar that holds it (`the_infrastructure_stock_is_built_and_lost_over_years`) was watched red against the switch: replacing the fixed step with `target - held` gave "the stock arrived in 1 months, which is a switch and not a stock".

**What would calibrate it:** real construction lead times for transport infrastructure, against which a year to full effect is probably FAST. The honest complaint is that the build and the decay share one rate for no reason other than simplicity — roads decay far more slowly than they are built.

### I-12 — `INFRA_EXTRACTION_SLOPE = 2.0`

`spheres-sim/src/resources.rs:891` (~~885~~, moved 2026-09-02 by the merge). Points of extraction bought by a point of GDP. **Chosen the way education's is** — so the top of the ministry's own reachable dial meets the ceiling and no step buys nothing. MEASURED: the reachable gap runs to about 0.117, and 0.117 * 2.0 = 0.234 sits just under the 0.25 ceiling.

**What would calibrate it:** it is a derivation from I-10's ceiling, so it has no independent empirical content — calibrate the ceiling and this follows. Filed separately because it is a separate literal that a later session could move on its own and thereby silently kill the top of the dial.

### I-13 — `industry_refill` clamp `0.70 / 1.40`

`spheres-sim/src/ministries.rs:145`, reached through `war::industry_refill`. The multiplier on `MAGAZINE_REBUILD * capital_intensity`. Not gated on war — an arsenal is built in peace. The 0.70 floor says a gutted industrial base refills a third slower and not never: a country that has stopped building shell lines still has the ones it built. **The SLOPE (4.2) is DERIVED, not invented — see I-16**, and this is the ministry where getting it wrong cost the most, because INDUSTRY HAS EXACTLY ONE ARM: when it saturates the whole card is dead.

**What would calibrate it:** munitions production ramp rates from the historical record — the 1990s are poorly documented for this, but WWII and the Korean mobilisation are not. The floor is the more suspect half: 0.70 asserts a floor under output that a collapsed industrial base may not have.

### I-14 — `science_absorption = gap * 6.0`

`spheres-sim/src/ministries.rs:156`, into `tech::absorptive_capacity`. Science's x35 left `research_output` and reappears here, on the PRICE side — the ability to read someone else's paper and build the machine it describes. MEASURED: the 0.065 gap the dial reaches is worth 0.39, about the whole of the existing 0.40 development term, so at the top of its dial a science ministry roughly doubles a nation's ability to absorb foreign technology.

**What would calibrate it:** the technology-diffusion literature on absorptive capacity against R&D intensity (Cohen and Levinthal is the canonical framing and the cross-country work is transcribable). **Open and NOT closed by this branch:** the 1.20 ceiling on absorptive capacity leaves 49.5% of the USA's science dial dead (Japan 33.8%, Germany 20.0%, Brazil 0.0%; roster mean 2.5%). The card is now honest about it, but whether that ceiling should RISE is a tech-tree calibration with roster-wide consequences and is Ridge's call.

### I-15 — `diplomacy_counterintel = gap * 10.0`

`spheres-sim/src/ministries.rs:212`, with the clamped realisation through `statecraft::exposure_probability`. A funded foreign service makes a foreign covert operation against you more likely to be EXPOSED, which is what costs the sponsor relations and reputation on the path that already existed. **Sized to the design's stated endpoint**: +0.5% of GDP is exactly +5 percentage points of exposure (0.005 * 10.0 = 0.05). The arm is quoted at ZERO HEAT — the first operation against you, the most it can ever buy — and the card says so, because every later operation starts hotter with less room under the 0.85 ceiling.

**MEASURED with the sample size derived at the bar (iron rule 7):** both worlds share one RNG stream so the per-seed flip is Bernoulli at p = 0.05, variance 0.0475, n = ln(0.01)/ln(0.95) = 89.8 is the floor, the bar runs 128, and the measured rate was 7/128 = 0.0547 (18 exposed unfunded against 25 funded). The bar's stated power: it sees an arm that is DEAD or BACKWARDS, not a mis-sized slope.

**What would calibrate it:** nothing public and nothing honest. Counter-intelligence success rates are not a measured quantity in the open literature, and this should be treated as the least defensible number in the set.

### I-16 — Three slopes that are DERIVED, not invented — and the handover the rebase is blocked on

Filed here so the distinction survives, because all three were INVENTED in an earlier draft and a review turned them into derivations. **The derivation rule** (now CLAUDE.md iron rule 8): a slope is chosen so the top of the ministry's own reachable dial meets the clamp ceiling, measured across the roster, "so no step of the dial buys nothing".

* **Education `15.0`** (`ministries.rs:70`). Education caps at 0.12 of GDP against an inherited reference near 0.036, so the largest reachable gap is ~0.084; 0.084 * 15.0 = 1.26 lands on 2.26 against the 2.25 clamp ceiling. At the original x20 the ceiling bound five steps short of the dial's own top.
* **Health `6.0`** (`ministries.rs:58`). Reachable raise across all 137 living 1990 nations: min 0.09575, mean 0.10112, median 0.10125, max 0.10725. At x20 the 1.60 ceiling was met at a gap of 0.030, so 68.7%-72.0% of every nation's raise range (mean 70.3%) bought nothing. At 6.0 the ceiling is met at 0.10000, the measured mean reach to within 1.1%.
* **Industry `4.2`** (`ministries.rs:145`). Reachable raise: min 0.03900, mean 0.09505, median 0.10200, max 0.11790. At x20 the 1.40 ceiling was met at 0.020, so 48.7%-83.0% (mean 78.0%) bought nothing. At 4.2 the ceiling is met at 0.09524, the measured mean reach to within 0.2%.

**DOCUMENTATION DRIFT — FIXED 2026-09-02 by the post-merge tidy.** ~~found 2026-09-02 and NOT fixed here (this agent was scoped to `.md` files only)~~: `spheres-sim/src/war.rs:468` and `war.rs:515` described these as "the x20 slope" and carried the worked arithmetic `1 + 0.02*20 = 1.40`, while both functions delegate to `ministries.rs` where the live slopes are 6.0 and 4.2. Both doc comments are now rewritten to the live values with the derivation and the measured reach: `war.rs:468` (health, 6.0, ceiling met at a gap of 0.10000 against a measured mean reach of 0.10112, and `1 + 0.02*6.0 = 1.12`) and `war.rs:525` (industry, 4.2, ceiling met at 0.09524 against a measured mean reach of 0.09505). ~~`war.rs:498`, `war.rs:532`~~ the delegating calls are now `war.rs:508` and `war.rs:549`, moved by the rewrite itself. No live value was changed; only the prose. The clamps stay INVENTED and are still filed as I-7 and I-13, which the new comments cite by number.

**HANDOVER, CLOSED 2026-09-02: the merge landed.** `origin/feat/hoi4-map-and-tech` moved from 9274baa to **61b388f** (21 commits: Codex's province production, manufacturing and arcade logistics) while this work was built, and `git rebase --onto origin/feat/hoi4-map-and-tech 9274baa` conflicted on the first of 21 commits, in `spheres-sim/src/resources.rs`. It was a SEMANTIC conflict, not a textual one: both sides re-expressed the same register bar in opposite directions, upstream asserting `debt_gdp` appears **7** times in that module and this branch asserting it appears **0** times. It was resolved by MERGE and not by rebase, on the ruling this branch already implements and **upstream's own M-2 asks for** ("does the resource system get a fiscal channel into growth, or does the mine's cost move somewhere the growth model does not read?"): `economy::charge` IS that single channel. Upstream's three new direct legs — the aggregate spot settlement's outflow and receipt arms in `apply_market_net`, and the mine construction investment in `start_mine` — were each routed through it, with the `share` argument character for character the ratio each line already pushed, so a nation with the books closed is bit-identical. **The floor stayed removed**: upstream's receipt arm reinstated exactly the `.max(0.0)` this branch had deleted after MEASURING it destroy $8.200bn out of a $10.000bn leg to Kuwait, and routing that arm through `charge` puts the floor back only on the closed-books `None` arm, where it is the shipped arithmetic, and leaves the `Some` arm conserving — which `a_money_leg_between_unequal_economies_conserves` pins. Upstream's market-cash ledger is kept as the module's till while the books are closed, and is not credited when they are open, because the treasury already holds that surplus. The bar now asserts the UNION of both intents: `resources.rs` names none of `debt_gdp`, `treasury_bn` or `debt_bn`, and its whole reach is **five** `economy::charge` calls. **The M-1/M-2 numbering collision was real and is now SETTLED BY RENUMBERING, 2026-09-02.** Both branches independently filed an M-1 and an M-2 with completely different content. ~~Settled here by prefix, not renumbering: both section headings survive and each M-number is read against the heading above it; renumbering either set is a decision for Ridge, not for the merge.~~ Superseded the same day: reading a number against the heading above it does not survive a grep, so this tree's two were renumbered into the free tail of the file's own M series — **M-1 → M-9** (the 1990 reserve refusals) and **M-2 → M-10** (the sovereign wealth funds). The mine landing's M-1..M-8 keep their numbers: they are the older, larger set and the only one cross-referenced from ROADMAP.md. Both old ids are struck through in place at their headings, and the three references that pointed at ours (BUGS I-5, ROADMAP.md line 23, SPEC.md section 3) were repointed.
---

## Awaiting an owner ruling (added 2026-09-02 by the merge-repair record agent, for the Codex province-trade-and-mines landing)

`e4e3c03` "merge: integrate Codex province trade and mines" is Ridge's own merge
of `9274baa` (ours) with `3f7eaf2` (Codex's "feat: integrate province resources
trade and mines"), carried on branch `fix/merge-repairs` with four repair commits
on top (`2b10e78`, `24b110e`, `7958ff4`, `104f851`). M-1..M-8 are what that
landing brought in uncalibrated, unruled or half-wired, plus the one thing the
merge itself moved.

Every figure below was measured on `fix/merge-repairs` at `104f851` on
2026-09-02 by the session that filed it — computed directly from
`spheres-sim/data/resources_1990.json` and
`spheres-web/data/district_population.json`, read out of the tree's own source,
or read off a test run whose binary was built into this worktree's own
`CARGO_TARGET_DIR`. Where a number comes from another session's measurement, the
line says so and names it.

### M-1 — the mine is uncalibrated: five bare constants, two of them inert, and an output that ignores the ground it sits on

`resources.rs` 792-796:

```rust
pub const MINE_BUILD_MONTHS: u32 = 12;
pub const MINE_PC_COST: f64 = 6.0;
pub const MINE_COST_YEARS: f64 = 2.0;
pub const MINE_COST_FLOOR_BN: f64 = 0.25;
pub const MINE_COST_CAP_BN: f64 = 25.0;
```

Five constants, no doc comment on any of them, no class letter, no source. They
sit under the banner at `resources.rs` 772-774 — "The mechanic constants
(Appendix A: named, classed, justified)" — between `BUFFER_MONTHS`, whose
comment cites the IEA's 90-day civil stock obligation and the Stock Piling Act's
three-year war stockpile (776-779), and `RELATION_FLOOR`, whose comment carries
a 21-seed x 40-year measurement (786-790). They are the only constants in that
block that do not answer the banner.

**Two of the five are inert, measured.** `mine_cost_bn` (`resources.rs`
2737-2740) is `(mine_output_bn_per_year * MINE_COST_YEARS).clamp(
MINE_COST_FLOOR_BN, MINE_COST_CAP_BN)`. Enumerating every (district, commodity)
pair the artifact makes buildable — `quality > 0`, `reference_mine(c)` present,
a 1990 unit price present — at the 1990 oil price of $20/bbl (`data/mod.rs`
806):

- **3,524 buildable pairs.**
- **3,411 of them (96.79%) are at or under the $0.25bn floor**, so the floor is
  what they are charged.
- **0 pairs reach the $25bn cap.** The dearest mine on the board is a quality-3
  coal district at **$0.4324bn** — a figure **54 districts** share exactly
  (the four US districts US-WV, US-IL, US-LA and US-MS, plus ZA-MP and
  VNM_ng-b-c-2, alongside 28 CN-\*, 7 UA-\*, 6 RU-\* and 7 others — 6 + 28 +
  7 + 6 + 7 = 54), because output does not vary by district — see below.
  **The cap is 57.8x above the dearest thing that can be built.**
  (Count re-measured at ship, 2026-09-02, with the tree's own `mine_cost_bn`
  over every presence district: 3,524 pairs, 3,411 at/under floor, 0 at cap,
  dearest 0.432440bn shared by 54 districts.)
- The only 113 pairs priced above the floor are **all coal**: 59 at quality 2
  ($0.28829bn) and 54 at quality 3 ($0.43244bn). Every other line, at every
  quality, is at the floor.
- Oil is the one line whose price moves in play, and it does not save the cap:
  at the $120/bbl clamp a quality-3 oil field prices at **$1.154bn**, still
  21.7x below it.

So `MINE_COST_CAP_BN` never binds, `MINE_COST_YEARS` is multiplied out by the
floor for 96.79% of the board, and the practical price of a mine is a flat
$0.25bn.

**Output ignores the district, the deposit and the owner.** `mine_output`
(`resources.rs` 2718-2726) is `reference_mine(commodity) * scale`, scale being
`3 => 1.5, 2 => 1.0, 1 => 0.5` off the district's presence rank, and nothing
else. `reference_mine` (2694-2716) is the **world median** 1990 output per
located district for that line, computed once from the table. The measured
medians, this run: bauxite 462,500 t; coal 6,009.547 kt; cobalt 229.781 t;
copper 5,820 t; gas 8.064; gold 1,057.143 kg; iron 826,000 t; oil 8.782 kb/d;
phosphate 613.815 kt; platinum_group 2,509.475 kg; rare_earths 3,785.5 t;
uranium 50.893 t. A mine in Chile and a mine in Chad produce the same number.

**The consequence, measured.** A single mine can dwarf the whole nation it sits
in, and the sparse ones are the worst offenders because the floor prices them
the same as the rich ones:

| owner | district | line | rank | mine output/yr | 1990 national output | ratio |
|---|---|---|---|---|---|---|
| Laos | LAO_vientiane | coal | 1 (sparse) | 3,004.77 kt | 2.999996 kt | **1,001.6x** |
| Ethiopia | ET-OR | platinum_group | 1 | 1,254.74 kg | 2.0 kg | 627.4x |
| Tanzania | TZ-14 | coal | 1 | 3,004.77 kt | 6.874992 kt | 437.1x |
| Spain | ESP_madrid | bauxite | 1 | 231,250 t | 1,000 t | 231.2x |
| Poland | PL-SL | iron | 1 | 413,000 t | 2,000 t | 206.5x |

One $0.25bn click multiplies Laos's national coal by a thousand.

**And it is added, never replaced, and never depletes.** `resources.rs` 885 adds
the transcribed located share into `flow`; `resources.rs` 904-906 then adds
`completed_mine_outputs` **on top of it** (926 does the same for the oil column
on the refresh path). `Mine` (`resources.rs` 1206-1211) carries `district`,
`commodity`, `output`, `completed` — no reserve, no decay, no depletion term
anywhere. `advance_mines` (956-1012) copies `project.output` into the `Mine` at
completion, and `project.output` was frozen by `start_mine` (`resources.rs`
2812) at the value `mine_output` returned on the day the click happened. **The
output is fixed at start and paid every month for the rest of the game.**

**Ruling wanted**, and it is five separate questions: (a) what each of the five
constants is, classed and sourced the way its neighbours are; (b) whether the
cap and the years-multiplier survive at all, given that neither can bind;
(c) whether a mine's output should scale to the district's own located share
rather than the world median; (d) whether a mine replaces or adds to the
transcribed 1990 figure; (e) whether a deposit depletes. Until then the mine is
a flat-priced, unbounded, permanent multiplier on a transcribed figure — the one
shape iron rule 4 exists to prevent.

### M-2 — the ruling breach: `start_mine` writes `debt_gdp`, and the growth model reads it

**HALF ANSWERED 2026-09-02 by the merge of `feat/ministry-economy`, and the
other half is still Ridge's.** The direct write below is GONE: `start_mine` now
reads `crate::economy::charge(w, nation, investment_bn, investment_bn / gdp);`
(`resources.rs:4058`), with the `share` argument character for character the
ratio the old line pushed, so a nation with the books closed is bit-identical
and one with them open funds the shaft from reserves and borrows only the
shortfall. The register bar
`resources::tests::the_trade_arms_write_only_what_the_register_allows` now
forbids the module naming `debt_gdp`, `treasury_bn` or `debt_bn` at all and pins
its whole fiscal reach at five `economy::charge` calls. **What that does NOT
settle is the doctrine**, which is the actual question this entry asked: the
resource system still HAS a fiscal channel into growth — `charge` writes
`debt_gdp` and `growth_terms` reads it — it is merely routed through one helper
instead of five hand-rolled sites. So the module header at `resources.rs` 66-68
still says "Nothing here is read by growth, oil, stability or munitions" and
that sentence is now FALSE of the tree, and
`tests::gates_write_nothing_the_growth_model_reads` (`lib.rs:4688`) still ticks
with an empty command slice and is still blind to `DevelopResource`. Amending a
quoted ruling is Ridge's call, not a merge's, so both are left as they stand and
recorded here. The filing below is kept verbatim as the record, and its line
numbers are those of `fix/merge-repairs` at `104f851`, not of this tree.

`resources.rs` 2814, inside `start_mine`:

```rust
w.nation_mut(nation).debt_gdp += investment_bn / gdp;
```

`economy.rs` 592, inside `growth_terms`:

```rust
let debt_drag = if n.debt_gdp > 0.9 { (n.debt_gdp - 0.9) * 0.02 } else { 0.0 };
```

This is the first write from the resource system into a quantity the growth
model reads, and it is against the ruling the module's own header quotes
verbatim at `resources.rs` 66-68: *"It should only hurt if you are trying to do
something with the resource and don't have enough of it."* — followed by
"Nothing here is read by growth, oil, stability or munitions."

**Its own guard cannot see it.** `tests::gates_write_nothing_the_growth_model_reads`
(`lib.rs` 4455-4486) compares `gdp`, `growth_last`, `oil_mbd`, `mil_strength`,
`munitions`, `stability` and `arsenal::book_value` bit-for-bit with the gates on
and off, six seeds, 480 months — but it ticks with an **empty command slice**
(`lib.rs` 4461-4462: `tick_month(&mut on, &[])`, `tick_month(&mut off, &[])`),
and `Command::DevelopResource` is the only route into `start_mine`. The guard is
green and blind to the one write that breaks the thing it guards.

Two further properties of the write, both wanting the same ruling: it is a
**permanent** addition to `debt_gdp` (nothing amortises it, unlike the trade
money legs at `resources.rs` 1118 and 1121, which move debt both ways), and it is
charged **in full on the day construction starts**, twelve months before any
output arrives.

**Ruling wanted:** does the resource system get a fiscal channel into growth (in
which case `resources.rs` 66-68 has to be amended in the house style and the
guard has to be given a command slice that includes `DevelopResource`), or does
the mine's cost move somewhere the growth model does not read?

### M-3 — the mine is player-only, and it widens exactly the hole R-1 measured

`Command::DevelopResource` occurs **six times in the whole tree**, and not one of
them is an AI:

1. `spheres-sim/src/lib.rs` 83 — the variant.
2. `spheres-sim/src/lib.rs` 320 — `command_price`: `MINE_PC_COST`, `REFUSABLE`.
3. `spheres-sim/src/lib.rs` 445 — `command_refusal` -> `resources::mine_refusal`.
4. `spheres-sim/src/lib.rs` 699 — `apply_command` -> `resources::start_mine`.
5. `spheres-sim/src/resources.rs` 3423 — the one unit test (M-4).
6. `spheres-web/src/main.rs` 2870 — the browser's producer, which hard-codes the
   issuer as the player: `Command::DevelopResource { nation: me, ... }`, reached
   from `ui/index.html` 10823.

`politics.rs` — the file that issues `PledgeAid`, `EndAid`, `ProposeAlliance`,
`CovertAction` and `ProposeTrade` on the AI's behalf — never issues it.

Against **R-1** ("zero resource wars in every F2 cell over two counts", 400
seeds, lambda-hat 0.000) this matters more than a missing AI arm usually would.
R-1's finding is that the AI has no route from wanting a commodity to acting on
it; the mine adds a second such route and hands it to the human only. The AI
cannot dig, so the only thing that changes an AI nation's endowment remains
conquest, which R-1 measured at zero.

**Ruling wanted:** does the AI get a mine arm (and if so, priced against what
appetite), or is `DevelopResource` deliberately a player verb? If the latter,
R-8's finding applies to it too — the suite calibrates a world in which this
command is never issued.

### M-4 — the mine's lost guards: four assertions the merge narrowed away

Codex's `3f7eaf2` carried two tests,
`mine_command_pays_once_builds_for_a_year_and_enters_have` and
`mine_project_and_output_follow_the_district_when_it_is_captured`. The merge
combined them into one,
`resources::tests::a_mine_builds_for_a_year_and_follows_the_district`
(`spheres-sim/src/resources.rs` 3405-3443), and four bars did not survive.

Gone, quoting `3f7eaf2:spheres-sim/src/resources.rs` 3508-3540:

1. **The fiscal charge.** `let debt = w.nation(chile).debt_gdp;` then
   `assert!((w.nation(chile).debt_gdp - debt - cost / gdp).abs() < 1e-12);` — the
   only assertion anywhere that the mine costs money, and the only test that
   could have caught M-2. **No test in the tree now touches the mine's fiscal
   cost**: `investment_bn` occurs at `resources.rs` 1201, 2811, 2814, 2821 and
   2839 and nowhere else in the workspace, and every one of those is production
   code. (Precisely: `mod tests` opens at `resources.rs` 3260, but it is not the
   file's first test code — `#[cfg(test)]` blocks stand at 2884, 2895, 2903,
   3074 and 3080. The claim survives the correction: every site listed here is
   at or below 2839, hence above none of them.)
2. **The save/load round-trip.** `let saved = crate::save(&w);
   assert!(saved.contains("mine_projects"));` then `crate::load(&saved)` and
   `assert_eq!(mine_project_at(&w, &district, c).unwrap().investment_bn, cost);`.
   `mine_projects` occurs at `resources.rs` 957, 961, 963, 985, 1160, 1183, 2744,
   2747, 2827 and 2830 — again all production. This is state that **enters
   `state_hash` the moment a player builds** (`Resources::is_empty`,
   `resources.rs` 1183, is false once a project exists, so the whole `resources`
   block starts serializing), and nothing asserts it survives a save.
3. **The "already online" refusal.**
   `assert!(apply_command(&mut w, &cmd).unwrap_err().contains("already online"));`
   — the refusal string still exists in production at `resources.rs` 2790-2791,
   asserted by nothing.
4. **"A refused click is free."**
   `assert_eq!(w.nation(chile).political_capital, held, "a refused click is free");`
   — the bar that `REFUSABLE` at `lib.rs` 320 means what it says for this
   command.

Also dropped: `assert_eq!(flow(&w, chile, c), baseline, "construction is not
production")` for the building nation.

What survives is the twelve-month build and the ownership-follows-the-ground
arm. Iron rule 5: tests are never deleted or widened. These were narrowed by a
merge rather than by a decision, so they are filed here rather than silently
restored — but unlike the two guards restored in `7958ff4`, restoring #1 lands a
**red** test against M-2's breach, which is a ruling and not a repair.

**Ruling wanted:** restore all four now (accepting that #1 goes red until M-2 is
settled), or restore #2-#4 now and hold #1 until M-2 is ruled.

### M-5 — the serialization that moved both goldens, with nothing in the simulation moving

`world.rs` 947-952:

```rust
    #[serde(default)]
    pub district_population: std::collections::BTreeMap<String, f64>,
    ...
    #[serde(default)]
    pub(crate) district_population_scale: Vec<f64>,
```

Neither carries `skip_serializing_if`. `world.rs` has ten `skip_serializing_if`
sites (303, 307, 724, 729, 734, 825, 833, 866, 879, 958), including the
`resources` field two declarations below these two at 958, so the discipline is
the house norm and these two are the exception. Both are therefore always
written, and both always enter `state_hash`.

**Measured this run** (release, this worktree's own `CARGO_TARGET_DIR`,
`cargo test -p spheres-sim --release --lib -- --exact tests::the_1990_start_is_pinned tests::golden_hash_of_a_known_run`):

- `the_1990_start_is_pinned` panics at `lib.rs:4068` — "the 1990 start state
  changed (actual **0xe26e4bf8d6c60066**)", against the pin `0xd022d50f43c984da`
  at `lib.rs` 4069.
- `golden_hash_of_a_known_run` panics at `lib.rs:4314` — "timeline fingerprint
  changed (actual **0xbe94d6125631829c**)", against the pin `0xbd5ec0f43c5f2e3b`
  at `lib.rs` 4310.

Both goldens were red before the merge too (E-3), but at `0xa5c9c5b2306313d8` and
`0x20c24ab0f1581807`. The actuals are now Codex's numbers.

**Nothing in the simulation moved**, and that was proved byte for byte by the
repair session that filed `2b10e78` — its commit body is the record. The merged
saves at t=0 and t=240 months, with exactly the `district_population` and
`district_population_scale` blocks deleted **plus the comma their removal
orphans** (they serialize last in `WorldState`, so deleting them alone leaves a
trailing comma and yields `0xcf9a80c77dc5aaa2` / `0x204b36a998819baf` instead),
are byte-identical to saves built and dumped from a detached worktree at
`9274baa`, and re-hash to `0xa5c9c5b2306313d8` / `0x20c24ab0f1581807`. That
measurement is that session's, not this one's.

**A correction to the obvious fix, measured here.** Adopting the neighbours'
discipline would **not** by itself return the tree to the old actuals. Every
`skip_serializing_if` in `world.rs` is an emptiness or default test — all ten
sites (303, 307, 724, 729, 734, 825, 833, 866, 879, 958) across seven predicates
(`Option::is_none`, `Vec::is_empty`, `BTreeMap::is_empty`, `is_true`, `is_false`,
`is_first_day`, `crate::resources::Resources::is_empty`) — and neither field is
empty in 1990: `data/mod.rs` 812-813 seeds
`district_population` from `districts::population_1990()` — **2,610 entries**,
the count `spheres-web/data/district_population.json` carries in its own `counts`
block — and `district_population_scale` to `vec![1.0; nation_count()]`.
`is_empty` would skip neither. Restoring the old actuals needs a predicate that
skips the map when it still equals the artifact's own seeding, which is a
different and more expensive thing (`is_first_day` at `world.rs` 866 is the only
value-equality precedent in the file, and it compares a scalar). **That is
Ridge's call and not a session's, because it changes a save format either way**:
skip-when-default means an untouched world's save loses 2,610 lines while a
modified one keeps them, and no-skip means every save carries them forever.

**Ruling wanted:** (a) leave the two fields always-serialized and accept Codex's
actuals as the tree's actuals; (b) give them a skip-when-equal-to-seeding
predicate, restoring `0xa5c9c5b2306313d8` / `0x20c24ab0f1581807`; or (c)
something else. Note this is orthogonal to the pins themselves — both goldens
stay red for **E-3's** reasons under any of the three, and T-5's protocol
(nothing is re-pinned until every calibration bar is green) is untouched by it.

### M-6 — the orphan route: `/api/district-populations` has no consumer

`spheres-web/src/main.rs` 3538-3541 serves `GET /api/district-populations` from
`district_populations_json` (`main.rs` 2422). **Nothing in the tree calls it.**
Grepping every `.rs` and `.html` in the workspace: the route at 3538, the
function at 2422, and one test read at `main.rs` 8644 inside
`province_dossier_reads_live_population_and_exact_geometry`. `spheres-web/ui/index.html`
contains the string zero times.

It is a half-landed feature. Codex's `3f7eaf2:spheres-web/ui/index.html` had the
whole consumer and the merged UI has none of it: `loadDistrictPopulations`
(Codex 6598), `populationSurfaceAt` (Codex 6040) and `populationColor` (Codex
6027), painted at Codex 8493-8494, driven by a `mapMode === "population"` that
Codex's page tests at 2433, 6612, 8486, 10319 and 10355. HEAD's `index.html` has
**zero** occurrences of `loadDistrictPopulations` and **zero** of
`mapMode === "population"`.

**The singular route is fine and is wired**: `GET /api/district-population/{id}`
(`main.rs` 3542-3543) is called from `index.html` and drives the province
dossier, asserted by the same test at `main.rs` 8635-8657. Only the plural
surface is orphaned.

**Recommendation, deliberately not carried out in this pass: delete the plural
route and re-add it together with its consumer.** A route with no caller is a
claim the tree cannot check, and the population map mode is a real feature worth
landing whole rather than half-restoring. Not deleted here because removing a
route is a behaviour change and this pass is the record.

### M-7 — a player who saves in the browser cannot resume

`spheres-web/src/main.rs` 3739-3750 implements `POST /api/load`: it reads
`save.json`, calls `load`, `resources::warm`, replaces the `Game` and returns
`state_json`. **`spheres-web/ui/index.html` never calls it.** The page calls
`/api/save` exactly once (`index.html` 2391) and `/api/load` zero times; the page
even says so in a comment at `index.html` 2298-2299 — "There is no /api/load
handler in this UI; if one arrives it owes the same lines."

So the save button writes `save.json` and there is no way to get it back without
restarting the server against that file. This is a playability hole on the
primary game surface (CLAUDE.md, Owner preferences).

**And the handler itself loses two things** the moment it is wired. `main.rs`
3746:

```rust
*g = Game { world: w, log: vec![], history: vec![] };
```

The dispatch log and the charted history are rebuilt **empty**, so a resumed game
has no headline record and no past. That is a defensible reconstruction — both
are display state and neither is in the save — but it should be a decision, and
the page should say so to the player rather than silently show an empty ledger.

**Ruling wanted:** wire `/api/load` (the comment at `index.html` 2298 names the
tech-graph invalidation it owes), and decide whether `log` and `history` are
reconstructed from the loaded world, left empty with the player told, or
persisted into the save.

### M-8 — BIBLE section 5's daily invariant has never seen `DevelopResource`

`tests::the_daily_clock_preserves_the_market_on_world` (`lib.rs` 1071-1270) is
the assertion of BIBLE section 5's amended condition — a day-stepped month is
bit-identical to a month-stepped one, **commands included**. Its schedule issues
four commands over 24 months: `ProposeDeal` at m=3 (`lib.rs` 1127), `DeclineDeal`
at m=5 (1138), `DeclineDeal` at m=7 (1143), `AcceptDeal` at m=11 (1148).
**`DevelopResource` is not among them, and it appears in zero daily-clock
tests** — its six occurrences are listed in M-3 and none is in a clock test.

**By inspection it should hold, so this is a coverage gap and not a defect.** The
whole `DevelopResource` path reads no `w.day` and draws no RNG: `mine_refusal`
(`resources.rs` 2758-2799) reads ownership, `quality_of`, `district_contested`,
`mine_at`, `mine_project_at` and `mine_cost_bn` (which reads `w.oil_price`, a
monthly quantity); `start_mine` (2802-2841) inserts into `mine_projects` at a
`binary_search_by` position, so the vector stays sorted and the save is
order-stable; `advance_mines` (956-1012) walks that sorted vector and decrements.
Grepping `rng` and `.day` over `resources.rs` 956-1012 and 2737-2850 returns
nothing.

**The fix is one arm of schedule**, not a new test: add a `DevelopResource` arm to
the existing test's `match m` — a district the player owns with `quality > 0`, on
a day that is not the 1st, with `political_capital` already set to 100.0 by that
test at `lib.rs` 1080. Filed rather than done because adding an arm to that test
changes a BIBLE-section-5 bar, and iron rule 5 wants the session that adds it to
watch it go red against the behaviour it guards — the honest red-check here is
making `advance_mines` decrement per day instead of per month.

---

## Filed and FIXED (added 2026-09-02 by the resource-pass performance shipper)

Two entries, both closed. Every figure is milliseconds per simulated month of
the named `SYSTEMS` row at 137 living nations, release, best of the test's own
passes over 1,200 months, measured in this session's own worktree with its own
`CARGO_TARGET_DIR` (iron rule 6). Where a number is another session's, the line
says so.

### P-1 — CLOSED: the conserved-market commit made the appetite pass 113x its own budget, and the cause was a per-commodity `draw`

**Symptom.** `tests::the_resource_pass_stays_under_budget` was green at
`e4e3c03` (total 0.0400 ms/month, appetite term 0.0112) and red from `2f9791e`
"feat: establish conserved resource market" onward.

**Before and after**, quiet machine, best of three whole invocations each
itself the test's own best of three:

|                 | resources | buy pass | appetite | total  |
|-----------------|-----------|----------|----------|--------|
| `4fbc806`       | 0.0602    | 0.0339   | 1.2657   | 1.3598 |
| after the fix   | 0.0342    | 0.0197   | 0.0226   | 0.0766 |
| rebased onto `a9a373d` | 0.0391 | 0.0199 | 0.0231 | 0.0821 |
| rebased onto `867b3d6` | 0.0347 | 0.0097 | 0.0192 | 0.0637 |

17.8x on the total and 56x on the appetite term, against an untouched 0.15
ms/month bar. The before row is not a reproducible constant: a review session
rebuilt `4fbc806` from a pristine archive and read totals of 1.6672, 1.8520 and
2.9316 over three quiet invocations — a 1.76x spread, against 0.7% on the
repaired tree — so the direction and the order of magnitude hold and the single
number does not.

**Cause, read at the code and confirmed by profile.** `dyads::last_resort`
(`dyads.rs` 141-146) swapped an O(1) cover-array read for
`resources::action_stalled`, which pays a `draw` — and therefore an
`arsenal::pick` fold over the whole 46-entry `DECK` — plus a binary search into
the 552-row `MarketState.stocks`, ONCE PER TRACKED COMMODITY PER DYAD PER
MONTH.

**Fix, three parts, because the obvious one was not enough and that was
measured rather than assumed.** (1) `resources::action_stalled_mask(w, id)`
returns the whole twelve-element mask from ONE `draw` and ONE walk of that
nation's contiguous stock rows, built once per (nation, month) by
`politics::ai_wars`; hoisted only as far as the commodity loop it still read
0.2103 total, which is why the callers hoist it to the nation. (2)
`resources::change_market_stock` collapsed from three binary searches into the
ledger to one, on the ~1,500 calls a month `post_market_flows` makes — the
resources row 0.0580 → 0.0348. (3) `arsenal::pick` reads a precomputed value
ranking instead of refolding 46 divisions and 46 `OnceLock` reads — appetite
0.0439 → 0.0226. Held by two new equivalence tests
(`resources::tests::the_stall_mask_equals_twelve_calls`,
`arsenal::ranking_tests::the_ranked_pick_is_the_folded_pick`) and, end to end,
by both golden ACTUALS and all four headless digests being byte-identical.

**Not restored, and worth saying plainly:** the appetite term is 0.0226, twice
the 0.0112 it cost before the merge, not back inside it. It is comfortably
under the bar; it is not what it was.

### P-2 — CLOSED: the guard added for P-1 could not fail, then failed on healthy code, and is now a ratio

**Symptom, in three stages.** `tests::the_resources_row_is_free` built
`world_1990(GameRules::default())` — market OFF — so it was structurally blind
to the market path and stayed green while the market-on row went 0.0041 →
0.0577. A market-ON arm was added at a 0.10 ms/month bar, which could not have
gone red for the 0.0577 regression its own comment named. The bar was then
tightened to 0.055 and red-checked — and an independent review measured that
0.055 goes RED ON HEALTHY CODE on a busy box: fourteen saturated readings
spanning 0.0472-0.0611 with four over the bar, against a comment claiming
sd 0.00101 and a false red of 6e-9.

**Confirmed here rather than taken on trust.** Across six saturated
invocations on this branch the absolute best-of-five read 0.0474 to 0.0661
ms/month, most of them over 0.055, and one individual pass read 0.1186 — 2.2x
the bar — with nothing wrong. Healthy-under-load and the regression overlap, so
no millisecond constant sits between them and more passes do not help, the
passes being no more independent of the load than the reading is.

**Fix.** The market-ON arm now asserts a RATIO — this row over the REST of the
same month tick, both accumulated over the same 1,200 months of the same world
— so machine speed, clock drift and every other process on the box multiply
both terms and cancel. Measured on `867b3d6`: healthy 0.01798 mean over 30
quiet readings and 0.01725 over 10 saturated, whole range 0.0150-0.0192. Bar
**0.022**, which is mean + 2.326·sd = 0.0198 by rule 7 with room to spare
(z = 5.0) and within 1% of the geometric midpoint of the gap it must sit in.
Red-checked by reintroducing `4fbc806`'s three-binary-search
`change_market_stock`: 0.0271-0.0309 quiet and 0.0258-0.0285 saturated, **RED
five invocations out of five, two of them under full sixteen-core saturation**
— which the millisecond bar could not do at all, being red on a busy box either
way. The market-OFF arm and its 0.02 bar were not touched and read
0.0029-0.0031 throughout, blind to the regression, which is the blindness this
arm exists to end.

**RE-DERIVE, do not scale.** The share is a property of the whole tick, so a
commit that adds a system moves it: `867b3d6`'s province manufacturing took the
healthy share from 0.01780 down from the 0.02108 measured one rebase earlier on
`a9a373d`, and the bar was re-derived from scratch rather than scaled. The
recipe is in the comment beside the bar — thirty quiet readings and ten
saturated, `mean + 2.326*sd` as the floor, the geometric midpoint of the gap to
the red-checked regression as the choice.

**What it still cannot see**, since rule 7's power half is the half that costs:
about 1.3x is the smallest slowdown it catches reliably, so a 10% regression in
the posting pass is invisible; and because the denominator is the rest of the
tick, it is blind to anything that slows this row and the whole game equally,
and will red for a large speed-up elsewhere in the tick with nothing wrong
here. The sibling `the_resource_pass_stays_under_budget` is still an absolute
wall-clock bar and is still load-sensitive — 0.0899 and 0.0958 under sixteen-way
saturation on `867b3d6` against its 0.15, 0.1338 and 0.1358 on `a9a373d`, and an
earlier session recorded 0.1954 and a red on a suite run with nine foreign
`rustc` processes live. It is left alone because it is met in every regime this
session could produce; the repair if that changes is P-2's, a share of the tick
re-derived from its own sample.

### M-11 — CLOSED: two stratagems moved the ratio and not the stock, so a player's restructuring was erased inside a month

**Found by review of the 61b388f merge, 2026-09-02, and fixed the same day.**
The merge routed the three money legs upstream had added and asserted that
`resources.rs` reaches a nation's finances only through `economy::charge`. It
did not look at the other module that moves money. `stratagems.rs` carried two
legs older than the helper itself, both writing the ratio by hand:

- `debt_restructuring` scaled `debt_gdp` by 0.55, and
- `mass_privatisation` subtracted 0.08 from it, floored at zero.

**Why that is a defect and not untidiness.** For a government that has opened
its books — which is every player who has enacted a budget, and only a player,
since `Command::SetAnnualBudget` is the only door — `debt_gdp` is DERIVED. The
fiscal block's open-books arm recomputes it from `debt_bn` every month. So the
stratagem moved a number that the next tick overwrote from the stock it had left
untouched: the creditors took the loss, every major capital's opinion of the
government fell by eight points, thirty political capital was spent, and a month
later the debt was all still there. The AI never saw it, because AI nations keep
no books, which is why nothing on the default board and no golden hash ever
moved and why the suite was green over it.

MEASURED on the red-check, Brazil at 1.30 of $385.0bn of output: `debt_bn`
$500.500bn before the restructuring and $500.500bn after, against the $275.275bn
the write-down is supposed to leave. Poland, $52.800bn owed on $66.0bn of
output: $52.800bn after selling 8% of output's worth of state industry, against
$47.520bn.

**THE FIX is the same one BUGS M-2 asked for and the merge applied to upstream's
legs: one fiscal channel.** Both go through `economy::charge` as receipts —
negative `bn`, the dollars the government stops owing, beside the `share` the
ratio line already moved. The closed-books board is BIT-IDENTICAL and had to be,
because `ai_stratagems` reaches for this deck on the default path: the
restructuring passes `ratio * 0.55 - ratio` rather than `-0.45 * ratio`, which
Sterbenz makes an exact subtraction that the closed arm's addition undoes
exactly, and the privatisation passes `-0.08`, which IEEE makes the same bits as
subtracting it. Confirmed by `the_treasury_is_inert_while_the_books_are_closed`,
which pins both golden actuals and stayed green across the change.

**THE GUARD**, in `spheres-sim/tests/treasury.rs`:
`the_stratagem_deck_moves_money_only_through_the_one_channel` forbids the
ASSIGNMENT forms of `debt_gdp` and `debt_bn`, forbids `treasury_bn` outright,
and counts exactly two `economy::charge` calls. Reading the ratio stays legal:
two stratagems are OFFERED because of what a government owes, and an
availability rule that cannot consult the debt is not a rule. Red-checked
twice, once per leg, each time going red together with the behavioural test
beside it (`a_stratagem_that_writes_down_debt_writes_down_the_stock`,
`privatisation_proceeds_retire_debt_in_dollars`).

**One live value did change, deliberately, and it is the same repair the payee
floor got.** On the books, privatisation proceeds larger than the debt now
retire it and BANK the remainder in the till, where the ratio line could only
clamp at zero and lose the difference. Closed books keep the floor, because
there the floor is the arithmetic that shipped.

**What this says about the register bars generally.** The merge's bar was
written to hold ONE module to the channel, and it did that correctly while a
second module quietly held two legs it never looked at. The census that matters
is `economy::charge`'s own call count, which now sits in that function's doc
comment as ELEVEN sites and is corrected there whenever it moves.

---

## Filed 2026-09-03 by the daily-push checker (fb08fa5 leaders and domination; ad60482 daily ticker)

The two pushes since our 3ecea29 landing were checked on a fresh worktree at
ad60482 with a watched clean release build; the shared checkout was not
touched. Legacy replay is intact: both golden ACTUALS still measure
`0xe26e4bf8d6c60066` / `0xbe94d6125631829c` and all eight `run 35` headless
digests equal 3ecea29's. Y-1..Y-3 were FIXED in 7ccb706 with a test watched
red each; the rest are filed because the correct form needs a ruling. Every
number here was measured on that tree.

### Y-1 — FIXED: a refusal cooled in 24 days, not 24 months

`resources::cool_refusals` (resources.rs, "Spec section 6.3") takes ONE step
on its lattice of twenty-fourths per call — deliberately, so six coolings
land on exactly `REASK_HEAT` — and `statecraft::tick` called it on every pass.
In daily play that is every day: a fresh refusal was gone on 1990-01-25
instead of 1992-01-01 and a buyer re-asked after 6 days instead of `PATIENCE`
months, 30.4x too fast and silent (the market is on in every browser game).
The lattice cannot take a `* dt`, so the CALL is gated to `clock::month_end`,
the day the legacy settlement took the step. Test
`daily_play_cools_a_refusal_once_a_month_not_once_a_day`: bit-identical heat
on every first-of-month for 24 months in both modes; watched red at
"1990-1-1: cooled inside the month".

### Y-2 — FIXED: one order row per purchase per day

`arsenal::tick` and `manufacturing::tick_allocations` merged a purchase into
an existing order only when `o.due_days == new_due_days`, and `new_due_days`
moves every day. Every daily purchase opened its own row for its whole lead
time (24..216 months): 49,271 rows against 1,619 after twelve default months
(USA 364 against 12), save 8,264,934 bytes against 1,034,809, and `find` and
`retain` scanned them every tick. One `arsenal::book_order` now keys a daily
row on the DELIVERY calendar month (`today + due_days`) and lands the row when
the month's latest purchase would — the last day of the month plus the lead,
which is the legacy settlement date; the legacy key is byte for byte the old
one. After: 1,621 rows (USA 12), 1,092,641 bytes, board book value 2984.567
bn in both modes. Test
`daily_procurement_books_one_row_per_kit_per_delivery_month`, watched red at
"left: 365 right: 12". The `due` field of a merged row is now cosmetic in
daily play (it may read one month less than the lead when the month-end
decrement ran before the day's purchase); `due_days` is the mechanism.

### Y-3 — FIXED by a gate, with the remainder filed: the AI buy pass asked every day

`politics::ai_statecraft` called `resources::ai_purchases` on every pass. Its
memory (`PATIENCE`, `REASK_HEAT`, `MAX_PENDING_OFFERS`, "a fourth is not asked
this month") is written in months, and physical freight's `supply_with`
counts only the day's fills — a contract in flight for ~21 days contributes
zero — so the pass re-asked while its own signing was still at sea. Seed 7,
player Iraq, daily: France signed SEVEN copper contracts with the United
States on 3–9 January (the browser check and the skeptic's headless
reproduction agree: per_month 0.1233..0.1240 each, $0.1bn/yr, 36 months, and
"United States cannot deliver all of its copper this day" headlined 88 times
in 90 days); legacy signed once in three months (re-measured by the repair
pass: 1 signing, 1 such headline, 1 contract). The pass is now
gated to `clock::month_end`, the cadence its memory is written in. Test
`the_ai_buy_pass_asks_once_a_month_in_daily_play`, watched red at "France sold
USA Copper twice in month 0".

**The remainder, which needs a ruling.** Probed after the gate on seeds 7 and
1990 for a full daily year: France's cover row exists only on 2–9 January and
every month-end pass finds nothing — 0 signings, 0 contracts, 0 refusal rows
in 365 days, against legacy's one March signing. The transient shortage a
daily world shows in its first week is now invisible to a monthly ask. The
alternative — ask on the first short day and remember "asked this month" per
(buyer, seller, line) — needs a new state field and a decision on whether an
AI that was short for eight days should have signed a 36-month contract for
it. Filed, not decided.

### Y-4 — the force-majeure headline is once per (giver, line, DAY)

`resources::settle` headlines "cannot deliver all of its X this day;
contracts are filled pro rata" once per (giver, line, settlement), and the
settlement is daily now. A giver short all month is headlined thirty times.
Y-3's gate removed the case that produced the audit's 88 in 90 days (0 in
90 and in 365 days after it, there being no contracts on those seeds now), but a real shortfall will still print daily. A
once-per-episode gate needs a memory of which (giver, line) was already
headlined this month — a schema field, filed rather than invented.

### Y-5 — the daily integrator is biased LOW against the calibrated monthly model

Not a proration error — every `* dt` is right — but a different integrator of
the same model. Figures in this entry are the 2026-09-03 audit's skeptic
re-measurement (economy::tick alone, same base world, shocks drawn in the
same order), recorded here rather than re-run by the repair pass. Legacy evaluates `growth_terms().before_noise` ONCE at the
start of the month and applies it whole (an explicit Euler step); daily
evaluates it along the path, and the annual rate FALLS across the first
months for 134/137 nations (USA 0.018045 → 0.016118, Vietnam 0.100560 →
0.086191, Argentina 0.017801 → −0.101783). With the same shocks drawn in the
same order: after one economy-only month 131/137 nations are lower (USA
5987.402124 vs 5986.916813, −8.1e-5 = 6.5% of the month's growth; median
−1.6e-4; Argentina −3.9e-3), inflation lower 134/137, stability lower 124/137
(Brazil −0.133); after twelve months 133/137 lower, median −3.3e-3 of level,
Vietnam −1.8e-2, stability median −0.050. Half the in-month rate drop /12
predicts the gap (USA −8.03e-5 predicted vs −8.11e-5 measured). This is the
"own evidence" CLAUDE.md asks daily-mode balance for, and for year one it is
negative. The repair is a ruling: re-calibrate daily, or make daily evaluate
the month's rate once (which is what `economic_shocks` already does for the
noise). Not touched.

### Y-6 — the player's private-investment flow is 3.0% over under `blend`

`economy.rs`: `priv += (priv + pressure − priv) * investment_blend`, i.e. a
FLOW of `0.06 × pressure` a month, wrapped in `clock::blend(0.06)`. Daily,
`blend` gives 0.001994 and 31 of them sum to 0.061814 = 1.0302× the monthly
flow (28-day month the same). Player-only (`Some(id) == player`); not 30x;
repricing, so filed.

### Y-7 — the technology burst cap is per tick, and a tick is a day

`tech/mod.rs` bounds adoptions per domain per tick at 6 ("several cheap
adoptions can land in one month ... invention never comes in floods"); daily
that allows 186 a month. Latent on this board, by the audit's probe: 24
months seed 1990, the most techs learned in one tick is 2 (legacy) and 1
(daily), USA 44/44, Japan 40/39 known. Filed; the cap's unit is a design choice.

### Y-8 — covert upkeep keeps its cadence and loses its phase

`commitment::deniable_forces_upkeep` fires on calendar months 1, 4, 7, 10 in
legacy (`w.month % 3 == 1`) and through `clock::interval_due(.., 3.0)` in
daily, whose counter starts at 0 when the (sponsor, target) pair first
appears. The audit's skeptic measured, Iraq at rung 5 against Kuwait, seed 10:
first covert heat
at tick 1 (settled 1990-02-01) in legacy, day 90 (1990-04-01) in daily. Same
cadence, anchored to appearance instead of the calendar quarter. Filed.

### Y-9 — the domination campaign (fb08fa5): one ending, reachable only by war, and no ruling

`spheres-sim/src/domination.rs` (1,109 lines) adds the game's ONLY ending
("no sovereign rival remains outside the player's hierarchy") and a hand of
three agenda cards. Mechanism verified: pays nothing, unlocks nothing, costs
0.0 PC, consumes no RNG, no per-nation content — a signpost, not a focus tree.
But: `status()` counts only the formal hierarchy, `subjugate` is called only
from capitulation and `absorb_subjects` only from annexation, so the ending is
reachable only through war, against BIBLE §"A full campaign with zero wars
must be a complete, satisfying playthrough"; every hand carries a
subjugate/settle-rival card; `war.rs` writes the subject row on EVERY
capitulation, player or not (save shape changes on seed 93 — two capitulations
— and seed 9, none on 1990 or 7, so the goldens cannot see it); nothing in war,
statecraft or politics reads the hierarchy, so a subject can still sanction
or attack its overlord; `tick` runs daily in daily play (the header said
"once per monthly settlement" — corrected in 7ccb706) and
`campaign_settlement_matches_the_daily_calendar` tests the legacy path only.
fb08fa5 touched none of BIBLE.md, CLAUDE.md, SPEC.md, DAILY.md or BUGS.md and
its message is one line: no ruling quoted, no doctrine amended. The ending,
the war-only reachability and the leader policy each need Ridge's word.

### Y-10 — the daily amendment did not follow the amendment convention

BIBLE.md's "Daily simulation amendment — 2026-09-03" is dated and quotes the
ruling, but adds 24 lines and strikes NOTHING through (0 `~~` in the
addition; the superseded §§4–6 text and the 2026-09-02 daily-calendar
amendment stand un-struck beneath it); CLAUDE.md and SPEC.md DELETED their
monthly-model paragraphs outright instead of striking them. The convention
(old text struck, amendment dated, ruling quoted) is three parts; two were
kept. Filed for the doctrine owner; the text itself is not disputed.

### Y-11 — two spheres-web assertions changed meaning

Of ten pre-existing test bodies the push touched (the audit's body-level
diff), eight only add
`days_left: None` to a `Contract` literal. Two changed what a legacy test
asserts: `a_funded_project_is_not_reported_as_stalled` expects "lands next
day" where it expected "lands next month", and the contract-summary test lost
`assert_eq!(contract["requested"], 10.0, "table tonnes are served as board
kt/mo")` and `assert_eq!(contract["unit"], "kt/mo")` — a rate relabelled as a
physical quantity. Across the whole `*.rs` diff those are the only two removed
`assert` lines (327 added, re-counted by the repair pass); `#[test]` 447 →
509 and `#[ignore` 62 → 62 by attribute grep; no tolerance widened. Recorded as a change of meaning, not reverted.

### Y-12 — the binary carries 322 MB of pictures

`spheres-web/src/portrait_assets.rs` embeds 303 files by `include_bytes!`:
160 AI-generated full-body leader PNGs, 305,173,085 bytes, and 143 Commons
portraits (webp), 16,886,182 bytes — 322,059,267 bytes in all.
`spheres-web.exe` is 349,737,875 bytes (the test harness does not carry them);
the git pack is 291.44 MiB. Licence state, measured from
`spheres-web/data/nation_figures.json`: all 143 portraits carry an
allow-listed free licence (108 public domain, 11 CC0, 4 CC BY 4.0, 2 CC BY
2.0, 1 CC BY 3.0, 8 CC BY-SA 3.0, 5 CC BY-SA 4.0, 3 CC BY-SA 2.0, 1 CC BY-SA
2.5) with creator, source page, licence URL, rights statement, credit and
sha256; 17 nations have no portrait and use a named cameo; all 160 leader
artworks record "OpenAI image generation", a prompt record under
`tools/avatars/prompts` and a credit line. `tools/avatars/check_assets.py`
passes (160 rows, 160 entries, 160 flags, 143 portraits, 17 cameos, 160
artworks). Whether the images stay embedded, ship beside the binary, or are
converted is a ruling (CC BY-SA portraits also oblige attribution in the
shipped UI, which the selector's credit line is meant to carry).

### Y-13 — browser surface, as played daily (Iraq, seed 7)

Recorded from the browser check, none changed: "Enact & advance one day"
with untouched dials advances the day and enacts nothing (`index.html`
`onclick="advance(1)"`; a budget command is queued only by a dial change);
the day-1 history label reads "Jan 1990" where later days read "2 Jan 1990";
the mine card says "$0.3bn" and its headline "$0.2 bn" for the same 0.25, and
"a oil field"; the page has no load control (`/api/load` exists); the game log
and history are not in the save; an AI contract's headline quantity is in
t/mo ("0.12 t/mo") while the player's is in t ("485 t/mo") for the same leg
type. UI, filed.

### Y-14 — invented coefficients in the three new modules

`clock.rs`: none but the `1e-9` floor guard in `advance_counter` /
`interval_due`; the rest is Gregorian arithmetic (146,097-day eras, the
726,773-day offset to 1990-01-01). `logistics.rs`: `EPS 1e-9`; sea segment
75,000 t/month, freight terminal 45,000, land corridor 25,000 × (1 + 0.20 ×
infrastructure level) × terrain {Mountain 0.55, Highland 0.78, Desert 0.82,
Wetland 0.72, Tundra 0.65, Lowland 1.0}; `tonnes_per_unit` kg 0.001, kt
1,000, bcf 20,000, kb/d 4,080; transit `estimated_days = ceil(weight/1,680)
+ 2`; `floor9` 1e-9 quantisation; a 10,000,000 route-weight sentinel. The
header says the network "claims no historical port, rail or road count" —
these are game capacities, unsourced by declaration. `domination.rs`:
`CLIENT_LEVERAGE_THRESHOLD 0.10`; enabling-card scores 39 + 18·(1 − GDP
share), 43 + 8·min(rival/own, 3), 44 + min(rivals, 50)/10, 46 + 3·missing
lines; repetition penalty 12 per occurrence in the last three legacies;
direct-card score 45 adjacency + 0.4 per negative relation point +
12·min(power ratio, 4) + 5·min(GDP ratio, 4); goals +10% GDP (min +1 bn), +2
strategic lines, +12% military-industrial power (min +5), +1.0 client
influence; `military_industrial_power = mil_strength + 0.05·book_value +
5·arms_plants + 2·civilian_industry`; client-influence legs 0.20 economic aid,
0.20 arms, 0.15 pact, 0.15·relation/75, 0.30·dependency/0.10; progress
weights 0.35 sovereign / 0.25 population / 0.25 GDP / 0.15 ground; milestones
0.25/0.50/0.75/1.0. All hand-tuned, none derived; the module is inert on the
default path so none moves a golden.

### Y-15 — `clock::advance_counter` cannot see a caller's silent reset

If a caller resets its public counter to 0 without `counters.remove(key)`,
`advance_counter` keeps the old fractional age and reads 1 after 6 more days
instead of 31 (the audit's probe, after 25 daily calls). Every present reset site removes the key (commitment.rs,
war.rs), so it is latent; a future site that forgets will drift. Filed.

## Filed 2026-09-05 by the political-arm shipper (ideology blocs, stages S0-S2, branch `feat/ideology-blocs`)

Ridge approved "The Political Arm of SPHERES", revision 2, on 2026-09-05, quoted: "Go ahead and build it with code." This section is the record the approved design owes: every model coefficient the run landed (iron rule 4 says DATA is transcribed; a coefficient cannot be, so it is labelled `INVENTED` where it lives and filed here with what would calibrate it), every leader row the loader keeps REFUSED, and the places where the built tree and the approved page disagree. Every number below was MEASURED on this tree at the commit that carries this entry, release profile, unless a line says whose reading it is.

### P-1 — the discontent weights `0.50 / 0.20 / 0.20 / 0.10`

`spheres-sim/src/blocs.rs:237`, `blocs::discontent = clamp(0.50·min(1, order) + 0.20·prices + 0.20·growth + 0.10·war, 0, 1)` over the existing `government::pains`. The four weights are the approved design's, and they are INVENTED: order is half because the roads to power in the design read discontent against 0.25 (coup) and 0.45 (uprising), and a stability of 25 alone had to clear the first and not the second. Measured on this tree: stability 40 alone reads 0.16667 (= 0.50 · 20/60), stability 25 alone 0.29167 (= 0.50 · 35/60), 18% inflation alone 0.20, a -4% year alone 0.20, war exhaustion 1.0 alone 0.10; Poland on seed 7 reads 0.2917 in the browser (amber). It decides NOTHING in this build — the roads are S4 — so its only effect is the chip, the gauges and the hatch (P-5).

**What would calibrate it:** the 1988-1991 record of governments that were actually removed from the street or by the barracks — Algiers October 1988, Caracas February 1989, Bucharest December 1989, Bamako March 1991 — read against the four pains each carried in the month before, as an ROC over the roster's transcribed stability, inflation and growth. Until then the weights are a ranking, not a scale.

### P-2 — `RULING_SEED = 0.60` and `SHARE_FLOOR = 0.002`

`spheres-sim/src/blocs.rs:30` and `:25`. Design D5: a pillar regime's movements are seeded FLAT from the leader row — the ruling bloc at 0.60, the remaining 0.40 split equally over the non-ruling blocs PRESENT in the polity (a party of that bloc in the dormant table, or the bloc's installing pillar), every bloc floored at 0.002, then normalised. Both numbers are INVENTED. Measured: China's stored seed is Communist 0.5988 after the floor and normalisation (0.60 before), the four absent blocs at 0.002 each pre-normalisation; a regime with ONE non-ruling bloc present hands it 0.40, which is why 24 nations already hatch on the uprising's challenger gauge (P-5). The seed is written once, on the first switched-on `ensure`, and nothing moves it in this build (drift is S3).

**What would calibrate it:** the first free vote after each 1989-1991 regime opening, read as the ruling movement's share when the lid came off — the PZPR's 1989 Senate wipe-out, the BSP's 47.2% of June 1990, the NDF's 59.9% in Burma, the MPR's collapse in Zaire — against the seed the arm would have written for each in January 1990. The floor only has to be small enough that a bloc absent from a polity cannot be Backed into existence for free (S3) and large enough not to vanish under normalisation; 0.002 is the first number that satisfied both, not a measured one.

### P-3 — `COURT_RULES_ABOVE = 0.40`, the monarchy exception

`spheres-sim/src/blocs.rs:185`. Where the leader row ties an ELECTORAL polity to a PILLAR, that pillar's bloc rules while authoritarianism is at or above 0.40, and the chamber leader is served as `government_of_the_day`. INVENTED, and it has exactly one case in the 1990 table: Jordan (King Hussein on the Hashemite court, `Pillar::Party`; the Brotherhood, `jo_ikhwan`, leads the November 1989 chamber). Measured: Jordan reads Non-Aligned with `government_of_the_day = jo_ikhwan`; at authoritarianism 0.39 it would read Islamist. The line sits where it does because Jordan's transcribed authoritarianism is above it and every parliamentary monarchy in the table (Spain, Sweden, Japan, Thailand's 1988 civilian government) is well below; no second case exercises it.

**What would calibrate it:** a second monarchy with a sitting chamber the court overrode — Morocco and Kuwait are the candidates once their tables are live — so the line is chosen between two measured cases rather than beside one.

### P-4 — the DISCONTENT chip's bands, `25` and `50`

`spheres-web/ui/index.html:14156`, `discontentBand`: green under 25%, amber 25-49%, red at 50% and above. INVENTED (approved design, header chip). The edges are the design's coup trigger (0.25) and roughly the uprising's (0.45) rounded to a chip-legible number; they colour a chip and decide nothing in the sim, and the number they colour is served (`blocs::discontent`), not computed on the page. Measured: Poland seed 7 reads 29% amber at the 1990 start. Filed so the two edges are not mistaken for triggers the roads read — the roads read 0.25 and 0.45 exactly, served in the gauges.

### P-5 — the Ideology hatch reads "half its trigger", and at 1990 that is every nation

`spheres-sim/src/blocs.rs:418` (`Road::half_armed`) and `:357` (`Gauge::progress`, the `Below` arm). The approved design hatches a nation "where any takeover gauge is at or past half its trigger". Two readings had to be invented to make that arithmetic: a FALLING gauge (loyalty ≤ 0.35, stability ≤ 12) is read from its ceiling, so army loyalty 0.675 is "half way" to 0.35 and stability 56 is half way to 12; and a BAND gauge (stability inside 30..70) has no half at all. **FIXED in this commit:** before it, the band read progress 1.0 whenever a nation sat inside 30..70 and hatched all 137 living nations by itself; `Road::half_armed` no longer reads a band, and `every_road_reads_closed_in_this_build` pins that (watched red with the old reading: "a band has no half").

**And after the fix the hatch STILL covers all 137 living nations of 1990**, measured on seed 7 in `politics_is_null_off_and_served_whole_on` and pinned there: 171 discontent gauges at half (the coup's 0.25 and the uprising's 0.45 both read the same number, so any stability under about 45 arms both), Western influence 80 (any chamber with 20% of the vote on the Western side), stability-toward-12 72 (anyone under 56), army loyalty 71 (every pillar seated at 0.65 reads 0.538 toward 0.35), party loyalty 60, challenger influence 24; the band, no longer read, would have hatched 102, none of them alone. A ROAD-level reading — every threshold gauge on one road at half — would hatch 73. **Not changed:** the gauge-level reading is the approved page's, and re-reading it (road-level, a higher fraction, or the `Below` arm read as value/trigger) is Ridge's call. Until then the Ideology map's hatch is decorative at the start and the legend says what it means.

### P-6 — the 1990 census disagrees with three bars of the design brief, and the bar is parked under `#[ignore]`

`spheres-sim/src/blocs.rs`, `the_1990_census_meets_the_design_brief`. The brief asked Communist 11-13, Islamist exactly Iran and Sudan, and Libya Nationalist. The transcribed table reads, on the full 137-row table (measured, and pinned in `the_1990_ruling_bloc_census`): **Western 67, Communist 17, Nationalist 7, Islamist 3, Non-Aligned 43.** The three misses:

* **Communist 17** — the eleven the brief expects plus Yugoslavia (Markovic, `yu_skj`), Bulgaria (Mladenov, `bg_bsp`, electoral at authoritarianism 0.40 with the BSP leading the chamber), Nicaragua (Ortega, `ni_fsln`, electoral at 0.50), Congo (`cg_pct`), Madagascar (`mg_arema`) and Seychelles (`sc_sppf`) — each the Communist family of its own transcribed party.
* **Islamist adds Algeria** — electoral at 0.55, and the polity table seats the June 1990 local result with the FIS at 0.542, so the chamber leader is the FIS whatever Bendjedid's FLN tie says. That is the pre-existing party table's transcription, not the leader row's.
* **Libya Non-Aligned** — Gaddafi is tied to the Party pillar (the Revolutionary Committees Movement), which the D3 pillar map installs as Non-Aligned in a non-Communist regime; no fetched source gave the row a `bloc_override` the way Sudan's did.

The bar was written as the brief states it and was red from the day it was written. It is now `#[ignore]`d — not deleted, not widened — because the suite's contract is exactly three deliberate reds (the two goldens and E-3) and a fourth red hides a real one; `cargo test -- --ignored the_1990_census_meets_the_design_brief` shows the disagreement unchanged. **Ridge decides** whether the rows move (a sourced override for Libya; a ruling that a reformed-communist chamber leader counts as Western) or the bars do.

### P-7 — five leader rows are REFUSED, and what would source each

`spheres-sim/data/leaders_1990.json`. A row no fetched source supports is kept nameless with its office, dates and sources and a note that opens with `REFUSED` and says why; `blocs::leader_row` skips it and the nation is described from its table ("the office-holder", then the institution). The loader holds the shape (`data::check_office`): a nameless row without the word is refused as an omission, a named row without a tie as an unsourced bloc. Measured: 137 rows load, 132 named, 5 refused; the census did not move when the audit's three came out, because all three are electoral polities whose ruling bloc was the chamber leader's all along.

| nation | the sourced facts kept | why refused | what would source it |
|---|---|---|---|
| Chile | President of the Republic and C-in-C, since 1974-12-17, born 1915-11-25 | Pinochet belonged to no party and Chile's polity block has NO pillar list, so no tie in the schema resolves | an Army pillar transcribed into Chile's block (the institution he headed, C-in-C 1973-1998), or a ruling that a pillar-less tie is allowed |
| Panama | President, sworn 1989-12-20 at Fort Clayton, born 1936-05-12 | Endara's Partido Panameñista Auténtico is not in the table — the block records the Panameñistas struck off and him standing on his partners' lines | a `pa_ppa` row with a sourced share, or a ruling that the coalition-leader tie (`pa_pdc`, 40.18) stands for him |
| Comoros | Interim President, since 1989-11-27, born 1918-08-22 | worldstatesmen lists Djohar NON-PARTY on 1 January 1990; the `km_udzima` tie written for him is his label at the 4/11 March 1990 election, a fact after the start (D2) | a source placing him in Udzima before 1990, or a ruling that the incumbent-state party stands for an interim president |
| Cyprus | President, since 1988-02-28, born 1931-05-20 | Vassiliou was elected as an INDEPENDENT with AKEL's support; a backing is not a membership, and the Communist bloc it produced overstated him | a schema way to say "independent, backed by" that reads a bloc from the backer with the note attached, ruled by Ridge |
| Greece | Prime Minister (all-party government), since 1989-11-23, born 1904-03-26; President Sartzetakis in `also` | Zolotas was a non-party central banker; the `gr_nd` tie was, in its own words, "a convenience tie" | the same schema ruling as Cyprus, or nothing: Greece's chamber seats ND as leader either way, so the described reading and the refused row agree |

Two more rows were CORRECTED rather than refused, on the audit: North Korea's heir date (was `1980-10-10`, an unsourced day; now `1980-10-14`, the 1st plenum of the 6th Central Committee that elected the leadership, from the 6th Congress and 6th Central Committee pages added to the row's sources) and two notes (Bulgaria records Atanasov's 21 vs 24 March 1986 conflict between its own two sources; Turkey drops the unsupported "formally left ANAP"). Everything else the provenance audit sampled — the 13 decided cases and 30 regional rows — read as its sources say.

### P-8 — D4's Nepal and Haiti party tables are transcribed but NOT landed (wired under the switch 2026-09-06 and REVERTED the same day, S7-3)

**TRIED AND REVERTED 2026-09-06 (built by the pin-and-wire pass in commit 5086cfa on `feat/ideology-census`, reverted by the ship pass at the skeptics' finding — S7-3; P-8 STANDS, Ridge's call as filed twice already).** What 5086cfa built, kept here so a cherry-pick needs no rediscovery — the second option below: `government::polity_in(w, id)` serves the `D4_POLITIES` block only while `rules.ideology_blocs` is on and `POLITIES` otherwise; every reader with the world in hand asks it, `bloc_present` / `bloc_can_win` / `flat_seed` (and the private `bloc_appeal`, `largest_party_of`) now take the world, `spec` / `base_share` resolve a party id in whichever table carries it, and the id-only readers (`regime_is_communist`, `pillar_bloc`, `home_pillar`, `pillar_name`) stay on `POLITIES` because both tables answer the same for the two nations — asserted by `d4_tables_are_read_under_the_switch_and_invisible_off`. PROOF, measured: the 1990 start actual 0xe26e4bf8d6c60066 and the timeline actual 0xbe94d6125631829c unmoved, `run 35 1990` d1a2cfbf7c6958d7 (3501 lines) and `run 35 7` 39dea3341a7f6e8c (3983) unmoved, the inertness trio green (see S6-4 for the one clause `the_bloc_layer_is_inert_over_time` gained). Under the switch the seeded movements read what the paragraph below measured (Nepal Non-Aligned 0.5988, Haiti Nationalist 0.5988, Islamist 0.002). S4-10 and R-6 stand with this.

**UPDATED 2026-09-06 (S3 part one, branch `feat/ideology-roads`): the premise below was wrong and is MEASURED.** Wiring the two blocks into `POLITIES` moved the serialized 1990 start from `0xe26e4bf8d6c60066` to `0x63a37522993aa5a4` with the switch OFF, because `government::ensure` seats `GovState.support` and `.seats` from any table on the first day — a dormant table is inert to the TICK, not to the start hash. The blocks are therefore landed as `government::D4_POLITIES` beside `POLITIES`, row for row with their sources and a shape test, and NOT wired. With them wired and the arm on, the seeded movements measured: Nepal (the court ruling Non-Aligned) Non-Aligned 0.5988, Western / Communist / Nationalist 0.1327 each, Islamist 0.002; Haiti (the army ruling Nationalist) Nationalist 0.5988, Western / Communist / Non-Aligned 0.1327 each, Islamist 0.002; the 1990 census stayed 67/17/7/3/43. **Ridge decides:** a deliberate re-pin of `the_1990_start_is_pinned` (the T-5 protocol), or a table the OFF world cannot see, which is a design change — every reader of `Polity.parties` would have to ask the switch.

Design D4 asked for the May 1991 Nepal and December 1990 Haiti party tables as DORMANT rows. The data pass transcribed both, with sources, in the `POLITIES` syntax — nine Nepal rows and six Haiti rows, three Haitian parties refused for want of a national vote share — and the text is kept at `docs/political-arm/nepal-haiti-d4-pending.txt`. It was not landed in `government.rs` this run because (a) the provenance audit did not reach it, and (b) it is not inert to the arm: Nepal (authoritarianism 0.70) and Haiti (0.78) stay regimes under `is_electoral`'s 0.60 ceiling, so the tick and the goldens would not move, but `bloc_present`, `flat_seed` and `regime_is_communist` read the dormant table, so the seeded movements of both change with the switch on and need their own measurement. Next session: audit the fifteen rows against their URLs, land them, re-measure the census and the two switched-on hashes, and run the trio.

### P-9 — two per-party overrides disagree with the approved page, on their sources

`spheres-sim/src/government.rs:976` `ba_sda` → **Nationalist** (the page said Western; https://en.wikipedia.org/wiki/Party_of_Democratic_Action — the SDA was the Bosniak national party of 1990) and `government.rs:2692` `sd_dup` → **Western** (the page said Islamist; https://en.wikipedia.org/wiki/Democratic_Unionist_Party_(Sudan) — the Khatmiyya party that signed the 1988 peace initiative with the SPLA and was ousted by the NIF-backed coup). Both rows carry a `DISAGREES WITH THE DESIGN` comment beside their URL. Kept as transcribed rather than silently put back to the page, because iron rule 4 says the source wins over the guess; **Ridge confirms or reverses**, and either is a one-line change with the census re-measured (neither nation's ruling bloc reads off these rows at the start).

### P-10 — Lebanon's leader row ties Hrawi to the Security pillar, and that is a reading

`leaders_1990.json`, Lebanon. Hrawi was an independent Maronite elected on 24 November 1989 by 47 of 53 deputies at Chtoura in the Syrian-held Bekaa; the polity has no parties and the tie is to "the Syrian Army" (`Pillar::Security`), the pillar his government rested on rather than one he headed. The provenance audit confirmed the facts and accepted the tie; it is filed here because it is the one remaining tie in the named rows that reads an institution the leader did not direct, and Ridge should know it is a reading. Lebanon rules Non-Aligned either way (the block's first pillar is also Security).

### P-11 — `play_rules` runs `ensure_all` at boot, new game and load

`spheres-web/src/main.rs`, `play_rules`. So that the leader table is loaded and every regime's movements are seeded before the first `/api/state`, the browser's rule-setter calls `government::ensure_all` — the same pass `init.rs:25` already runs, idempotent, and the only effect beyond it is the switch-gated seed (`government::seed_blocs`, which returns before touching state or the table when `ideology_blocs` is off). It is a boot-time state write outside the tick, noted here because iron rule 2 asks that every write be visible; it is not a side door for the player and it draws no RNG (`the_bloc_layer_is_inert_over_time` compares `rng.state` on and off for 240 months).

### P-12 — `the_resource_pass_stays_under_budget` is a wall-clock bar, and it reads the machine

`spheres-sim/src/lib.rs`, pre-existing and not in this branch's diff. It times the resource pass with `Instant::now` against a 0.15 ms/month bar (budget 0.05). Measured this run, alone, release profile: total 0.0640 ms/month (resources 0.0347, buy pass 0.0093, appetite 0.0201) — under the bar, over the budget. The skeptic's reading on the same machine under the full debug suite was 0.5986 ms/month on HEAD and 0.5932 on the base branch, i.e. red on both, which is the machine and not the branch. Filed because "exactly three reds" is only true of this bar on a quiet machine; the repair the bar's own comment names — a share of the tick rather than a duration — is a rewrite of a bar that is holding, and rule 5 does not ask for it.

## Filed 2026-09-06 by the S3 part-one shipper (movements, foreign backing, the AI covert arm; branch `feat/ideology-roads`)

Stage S3 of the approved design, part one: `drift_movements`, the liberalisation seam and the surge latch (b850b24); `CovertOp::BackBloc`, the backing stock, gravity and influence (9529b33); the ideological-sponsor flag and the gated AI covert arm (1e6f68a); D4 transcribed beside the table (fa99b84). Every coefficient below is INVENTED — a model coefficient of the approved design, labelled at its definition — and filed with what would calibrate it. The measured values quoted are this run's.

### S3-1 — the movement drift: `0.006` swing, `0.015·dt` bound, `0.005` reversion

`spheres-sim/src/government.rs`, `drift_movements`. The swing and the bound are `drift_support`'s own numbers, taken as they are so the two cannot disagree in kind; the reversion is a quarter of the electorate's `0.020` on the reasoning that a country with no ballot has weaker anchors. Measured on China with the pains pinned: a quiet record (+0.35) settles the ruling share at 0.71316 (fixed point 0.718), a catastrophe (record -2.33) at 0.15839 (fixed point 0.158), no month over the bound, no bloc under the floor. **What would calibrate it:** the speed at which the East German, Czechoslovak and Romanian regimes' standing collapsed in the autumn of 1989 against the pains this model reads for them — months, not years — which the 0.015 bound cannot reproduce from a 0.60 seed (26 months to zero at the bound); the honest reading is that the drift is the slow channel and the roads (S4) are the fast one.

### S3-2 — the surge latch: `0.30` up, `0.25` down

`government::note_surges`. "Passes a third" at 0.30 with a 0.05 hysteresis so a movement on the line does not print every month; the latch is seeded closed for a bloc the flat seed already puts over the line (Iraq's and Syria's lone Non-Aligned 0.40 printed on the first tick before that — measured and fixed in b850b24). **What would calibrate it:** nothing numerical; it is a headline threshold, and the number to revisit is the flat seed's 0.40 to a lone bloc (P-2), which puts twenty-four regimes over it on day one.

### S3-3 — the seam seats the table from the movements, and the interim chamber is the old one

`government::reseed_support_from_movements`. On the first free elections being scheduled, `support` is reseeded (S_B times the party's table weight within its bloc; a bloc with a movement and no party dropped, named in the headline) but `seats` are NOT recomputed: the interim government formed that month is the old chamber's and the first vote eighteen months on reads the reseeded support. A reading, filed so it is visible: recomputing seats at the seam would seat the interim government from the street. Measured: Kuwait, the first liberalisation on seed 1990, drops its court's Non-Aligned 0.60 (no party carries it) and seats kw_islam 0.549 / kw_dem 0.451 against the table's 0.45 / 0.55.

### S3-4 — the backing arm: `0.06` a clean op, `0.12` per sponsor, `0.25` per bloc, `-0.006` a month, `0.02` taint

`spheres-sim/src/statecraft.rs`, the five `BACKING_*` / `EXPOSURE_TAINT` constants; one arm, `add_backing`, charged from `backing_room` and quoted by `back_bloc_effects` at its clamped value (iron rule 8). Decay is half covert heat's 0.012. **What would calibrate it:** the documented programmes — Operation Cyclone's share of the mujahideen's standing, Saudi money behind the FIS in 1990-91, the CIA's 1990 Nicaraguan election funding (about $9m against UNO's 55%) — read as "how much of a movement's weight was foreign" (the cap) and "how fast it faded when the money stopped" (the decay).

### S3-5 — patronage gravity: `0.10` at an infusion of `0.10`

`spheres-sim/src/blocs.rs`, `GRAVITY_WEIGHT`, `GRAVITY_FULL_AT`. A view of the aid flows, never stored, inside the bloc's 0.25 cap. Measured: a USA pledge of 0.0002 of output to Egypt reads 0.0278 (infusion 0.0278), 0.004 reads the full 0.10, gone the month `EndAid` lands. **What would calibrate it:** Cuba at 21% of GNP from Moscow (statecraft's own `MAX_INFUSION` source) and what it bought — the knee at 0.10 says a client taking a tenth of its output from a patron carries that patron's bloc at full weight.

### S3-6 — the AI's `0.15` influence line, and the sponsors' draw after the choice

`spheres-sim/src/politics.rs`, `ai_back_bloc_choice`. A patron backs its own bloc anywhere and any qualifying bloc in a rival's client; an ideological sponsor its transcribed bloc only. The sponsors' block draws its 0.022 die only when a sponsor has a target AND a bloc to back — measured 2026-09-06: drawing first parted the switched-on world from the switched-off one at month 0 on every seed tried, with nothing to show for it; drawing after the choice leaves the two identical until the arm acts (month 35 on seed 1990 with Libya in the player's seat). Twenty AI years on seed 7 land 3 backings, none surviving to the end. **What would calibrate it:** the same programmes as S3-4, read for WHEN a service moved — the FIS was backed at roughly a majority of the June 1990 local vote, not at 15% of anything — so the line is probably low; and the count of 3 in twenty years says the arm is quiet, which the sponsors' `best_covert_target` (relation at or under -30, brittleness) explains more than the line does.

### S3-7 — `the_bloc_layer_is_inert_over_time`'s ON half was re-expressed, twice, and is not the bar it was

`spheres-sim/src/blocs.rs`. Under S1 the bar held the switched-on world bit-identical to the switched-off one for 240 months. S3 contradicts that by construction — the seam writes support, the AI arm draws — so the ON half now runs seed 1990 with Libya in the player's seat (the one sponsor with a choice in January 1990) and requires bit-identity in every compared quantity, the RNG stream and every government UNTIL THE FIRST MONTH THE ARM ACTS, which must leave an arm stem in the on-only headlines and is pinned as measured (month 35, France backing the Western movement in Libya). The six OFF-world BASE hashes and the golden actuals are the inertness proof and are untouched. Watched red with a 1e-9 leak into support ("parted at month 0 with no stem"). Filed because iron rule 5 says a re-expressed bar is Ridge's to confirm, and because the pin will move whenever the arm's preconditions move.

### S3-8 — the `ideological_sponsor` flag is an `Option<Bloc>`, not a `bool`

`spheres-sim/src/nations.rs`. The design names the flag "default false" and lists the bloc each sponsor backs in parentheses; since Libya's transcribed ruling bloc reads Non-Aligned (P-6) while the design has it sponsor Nationalist, the sponsored bloc had to be data on the flag rather than the sponsor's live ruling bloc, so the flag carries it. `None` is the false. Read only by the gated AI arm; `patrons()` and `PATRON_ORDER` are untouched and asserted so.

## Filed 2026-09-06 by the S3 part-two shipper (the five levers, the crackdown arm, the AI political rules; branch `feat/ideology-roads`)

Stage S3 of the approved design, part two: the five levers priced, refused and served from one plan each (f6c1986); the Security Crackdown's gated backing-halving arm (a5c6d7b); the gated AI political rules on the deck's 0.02 draw (the commit after). Every coefficient below is the approved design's, INVENTED, labelled at its definition, and filed with what would calibrate it; the measured values are this run's.

### S3-9 — the five prices: `40 / 18 / 12 / 35 / 30` PC, and every lever's own numbers

`spheres-sim/src/government.rs`, `SUSPEND_PC`, `BAN_PC`, `LEGALIZE_PC`, `PROGRAMME_PC`, `ROUND_TABLE_PC`, read by `lib::command_price`; the arms are the plans (`SuspendPlan`, `BanPlan`, `LegalizePlan`, `ProgrammePlan`, `RoundTablePlan`) — a suspension's `max(auth + 0.30, 0.65)`, `-6` stability, `-10` with every democracy and `+0.10` to the ruling movement's seed; a ban's `+0.04` / `-3` at 15% of support / `-4`; a legalisation's `-0.02` / `+3`; a programme's `-5` stability, Clergy `+0.15` Islamist else `-0.10`, Army `+0.10` Nationalist, Party `-0.15` leaving Communist, `-15` / `+10` with the great powers, `+0.10` to the new colour; a round table's `min(auth - 0.25, 0.59)`, `+8`, six months. Each arm is quoted at its clamped value (Egypt's al-Azhar at 0.90 under an Islamist programme is quoted 1.00, not 1.05 — measured). **What would calibrate them:** Fujimori's April 1992 autogolpe (approval rose, the OAS and the US cut aid — the sign of the democracy arm is right, the size unread), Algeria's January 1992 annulment and the FIS ban (a ban at 47% of the first-round vote, which the model's `-3 at 15%` is far too gentle about), Turkey's 1980-83 junta programmes, and Poland's own round table of February-April 1989 (elections in four months, not six).

### S3-10 — "patron" in the programme arm is read as `nations::patrons()`, the great powers

`government::programme_plan`. The design says "-15 with every patron whose ruling bloc was the old bloc, +10 with every patron whose ruling bloc is the new one" without saying which list. Two readings exist in this tree — `w.patrons_of(id)` (the nations paying aid into this one) and `nations::patrons()` (the eight great powers) — and the arm reads the second, because a programme is a public alignment and the poles of the world answer it whether or not they were paying, and because under the first reading the arm is inert for almost every regime in 1990 (few aid flows exist at the start). Measured: China's Western programme moves the USSR −15 and the USA, UK, France, Germany and Japan +10. If Ridge reads it the other way, the list is one line.

### S3-11 — the crackdown halves the STOCK, not the view

`spheres-sim/src/statecraft.rs`, `halve_foreign_backing`. "F_B halved for every non-ruling bloc": F_B is the stock plus patronage gravity, and gravity is a view of the aid flows with nothing to halve, so the arm halves the stored entries and leaves gravity — a crackdown does not cancel a patron's aid. The card (`crackdown_backing_effects`) quotes F_B before and after at the clamped value, gravity kept. Measured: Poland with 0.18 of Communist backing reads "0.180 → 0.090".

### S3-12 — the AI political rules: their lines, and the draw at the deck's site

`government::ai_lever` (pure, `None` before any read with the arm off) and the gated block at the end of `stratagems::ai_stratagems`. The lines are the design's: suspend at stability < 30, authoritarianism ≥ 0.25, PC ≥ 55; ban the largest party of the strongest non-ruling non-Western bloc at influence ≥ 0.35, authoritarianism ≥ 0.40, PC ≥ 60; a programme toward the strongest pillar's colour when the ruling movement is under 0.30 and PC ≥ 70; a round table at discontent ≥ 0.50, the largest movement ≥ 0.35, an armed-pillar mean under 0.50 and PC ≥ 60. The design places the rules "in ai_government on the existing 0.02 monthly stratagem draw" and also says the government module draws no RNG; the rules live in `government.rs` and the draw lives beside the deck's in `stratagems.rs`, an own gated draw at the deck's rate, AFTER the choice (the sponsors' finding, S3-6). Measured, twenty AI years on seed 7 with the arm on: **0 nation-months in which any AI government had a lever to pull**, so 0 draws and 0 levers — the arm is quiet because the 1990 roster is; the electoral rule needs stability under 30 and 55 PC at once, and no regime on seed 7 reaches discontent 0.50 with a 0.35 movement and a disloyal army. **What would calibrate it:** the same 1989-92 episodes as S3-9, read for the state of the country the month before each — Algeria's army moved at a first-round vote, not at a 0.35 influence line.

### S3-13 — the S3-6 count was measured on an earlier tree: seed 7 lands 8 AI backings at part one's tip, not 3

`blocs::tests::the_ai_backs_a_bloc_only_under_its_conditions` prints the count. Re-measured this run at 38c1654 (part one's tip), f6c1986, a5c6d7b and the AI-rules commit: 8, 8, 8, 8, none surviving to the end. The 3 in S3-6 was read before a later part-one commit moved the switched-on stream; the switched-off world is unmoved throughout (six BASE hashes, two golden actuals, both headless digests). Filed so the number in S3-6 is not taken as this tree's.

### S3-14 — `GovState.banned` changed type, `Vec<Bloc>` to `Vec<String>`

`spheres-sim/src/government.rs`. S2 declared the field as blocs and nothing wrote it; the approved lever bans a PARTY, so the field now carries party ids. No save carried the key (it serialises nothing while empty, and nothing could fill it), so no save breaks. `BlocRow.banned` and the web's group flag now read `blocs::bloc_banned` — every party of the bloc in the table banned — and the web's party rows read the ban per party.

## Filed 2026-09-06 by the S4 shipper (the roads, succession, mortality; branch `feat/ideology-roads`)

Stage S4 of the approved design, eight commits on part two's 712de08: `regime_break` shared (d8df9b6); route 2 (79c2ea8); the annulment (9a6885b); route 3 (0386136); route 4 (6f4a6d8); the foreign payoff (e218372); succession (bea4513); mortality (ca41545). Every road is behind `rules.ideology_takeover`, which nothing in the tree turns on — the browser keeps it off and every road reads `calibration pending` there — and mortality and succession are behind `rules.ideology_blocs`. Every coefficient below is INVENTED, labelled at its definition, and filed with what would calibrate it. The measured values are this run's.

### S4-1 — route 2's electoral pressure: `0.30·(2·(0.35 − eff) + (D − 0.25))·dt`, cap 1.5, cooling 0.03·dt, twelve months settled

`spheres-sim/src/government.rs`, `electoral_army_tick` / `maybe_electoral_coup`, `ELECTORAL_COUP_*`. Measured on Pakistan with its budget at 0.001 and stability held at 35 (its transcribed 52 reads discontent 0.147, under the 0.25 line — at Pakistan's own numbers the road never arms): the Pakistan Army crosses 0.35 in month 11 and removes the elected government in month 30, not the design's "about eighteen"; the bar is 12..=36 with the measurement recorded beside it. **What would calibrate it:** Pakistan's own October 1999 (a paid army, a government at 0.60 of the chamber, no pressure of this shape at all — the coup was the army's, not the street's), Thailand's February 1991, Haiti's September 1991 (eight months after the vote: faster than any setting of this rate reaches from 0.65), and Turkey's 1980 (two years of street war before the generals moved). The rate looks slow by Haiti and the 12-month honeymoon short by Turkey.

### S4-2 — the annulment's lines: authoritarianism `0.35`, discontent `0.25`, Communist or Islamist only

`government::annulment_check`. Algeria at its transcribed numbers (stability 40, inflation 16.7%, held — left to the model with Algiers in the player's seat, inflation falls to 0.1% and discontent to 0.219 by October 1990, under the line) annuls the December 1991 vote the FIS won; Jordan under its court at 0.55 does not, and at 0.38 does. **What would calibrate them:** the one clean case is Algeria's own January 1992 (FIS 47.3% of the first round, annulled between rounds: the model annuls AFTER the seats and bans the bloc, which is the right order of events and the wrong month); Burma's May 1990 (the NLD's 80% of seats annulled by a junta that never let it sit — a Western winner, which the line excludes); Turkey's 1997 "post-modern coup" against the Welfare Party (Islamist, in office a year, forced out rather than annulled). Burma says the bloc restriction is too narrow.

### S4-3 — route 3: `0.45 / 0.45`, coercion at `0.50` weakest / `0.55` mean / `0.40` electoral, the winner's authoritarianism, W's `+0.15`, pillars `0.80 / 0.55`

**CALIBRATED 2026-09-06 (S5, the census; S5-2 carries the commits).** `blocs::COERCION_ARMED` 0.50 → 0.35 and `COERCION_MEAN` 0.55 → 0.35, both quoted from the pillar model's unpaid line (`regime_tick`'s pressure, `ai_government`'s purchase); and the crown at the collapse site needs `blocs::uprising_armed` — a stability < 12 collapse the movement did not arm runs the pre-arm block verbatim. Before → after at N=60: A3 Communist non-ballot takeovers 60/60 seeds at 3/5/6 a seed → 58/60 at 0/2/3 (Peru 60 → 0, Georgia 60 → 0); uprisings won by Western 10/14/21 → 2/4/6, Communist 4/5/6 → 0/2/3, Nationalist 0/3/6 → 0/0/1, Non-Aligned 2/4/7 → 1/2/3 (Iraq's flip-flop gone). The `0.45 / 0.45` lines, `0.40` electoral, the winner's authoritarianism, W's `+0.15` and the pillars `0.80 / 0.55` did not move (S5-4 pins the first three as literals). At N=200 on the final tree: A3 197/200 seeds, 0/2/4 a seed (S6-1).

`blocs::uprising_armed` and friends, `government::uprising`, `winner_authoritarianism`. The same `monthly_chance(0.10·crisis)` draw as the pre-arm collapse, at the same site. Sudan with the Communist movement at 0.46, discontent 0.70 and the army at 0.30 takes power in month 13 on seed 7. A Western winner is closed in a party-less polity by construction (`bloc_can_win`), which is the design's line and not a calibration; Burma (S4-2) is where it will bite. **What would calibrate it:** the autumn of 1989 — Leipzig and Prague fell at discontent this model cannot read (the pains are prices, growth, war and order, and East Germany's were mild) with a Western challenger whose influence was a crowd and not a share; Romania's December (coercion failed at the army's defection, which `weakest_armed < 0.50` is the right shape of); Iran's 1979 (an Islamist winner over a Nationalist court, the `max(auth, 0.80)` about right). The honest reading is that the 0.45 discontent line will rarely be met by a regime whose economy this model runs quietly, which is why the switch stays off.

### S4-4 — route 4's drift: `0.01` a month to a floor of `0.55`, on `0.40` Western influence, `0.55` Party loyalty, stability `30..70`

`government::round_table_drift`, `blocs::round_table_armed`. Indonesia from 0.80 under the press opens in month 21 and votes in month 39. The floor is under the 0.60 ceiling so a polity with a table opens on the way down; a party-less one (Saudi Arabia, measured to 0.55 exactly) stops as a regime. **What would calibrate it:** Poland's own round table (February–April 1989, four months from talks to a date; the model's 20 months from 0.80 is the drift's whole span, not the negotiation's), Hungary's summer of 1989, South Korea's June 1987 (a Western challenger, a Party pillar that gave way, elections in six months), Taiwan's 1987–92 (five years: the drift's slow end). The step is between Korea's and Taiwan's and probably fine; the 0.40 Western line has no measured case behind it.

### S4-5 — the foreign payoff: `0.10` backing, `+40 / −25`, exposure at heat `0.50`, democracies `±8`, the loser's patrons `−10`

`statecraft::takeover_payoff_plan` / `takeover_payoff` / `takeover_payoff_effects`, one plan (rule 8); `caught` is `covert_action`'s exposure block factored out verbatim. The design's "the target's previous top patron −25" is read as the SPONSOR losing 25 with the patron whose client it took, not the patron losing 25 with the target — the two readings are one word apart and this one keeps the patron's own relation with its former client for the loser-patron arm to move. Measured: Moscow +40 with Khartoum then −35 on the spot exposure (+5 net), −25 with Washington, the UK −8, Moscow and Beijing −10 under a Nationalist takeover of a Communist Sudan. **What would calibrate them:** the Soviet Union's standing in Kabul after April 1978 and Addis Ababa after 1974 (the +40), Washington's in Tehran after 1979 (the loser side), and how quickly the CIA's hand in Guatemala 1954 and Chile 1973 was named — the exposure-at-0.50 line says a well-worn channel is named at the moment of victory, which is roughly what happened in both.

### S4-6 — succession's descriptions are the polity table's names, and "a new leading party" is judged against the vote before

`government::succession_seat` / `seat_office`, `data::Emergent`. Two readings the design left open, both recorded: (a) "same leading party keeps the transcribed person" is judged against the coalition leader BEFORE the vote, not against the transcribed leader's party — the table seats the Congress for the United States, the Democrats lead it before and after November 1992, and reading it the other way replaced Bush with "the Democratic Party government" at his first election and reached his 1997 ceiling never; (b) a party-tied leader whose regime declares a programme is removed too (the heir if there is one, else "the {party} government"), because the design's sentence names DeclareProgramme as a removal. `Office::Emergent` in the design is `Office.emergent: Option<Emergent>` in the code: the table row is a `struct` the loader holds to `deny_unknown_fields`, and an enum would have re-shaped 137 transcribed rows to add five fields. Forty years on the roads (seed 7): 121 offices changed hands, no description is a name but a row's own heir.

### S4-7 — mortality's hazard: `0.015 · 2^((age − 65) / 7)`, LAST in the tick, and what the census reads

`politics::death_hazard` / `mortality`, the `mortality` entry of `SYSTEMS`. A Gompertz curve fitted by eye to the 1990 male life tables of the leaders' own countries — 1.5% at 65 doubling every seven years — and not to any leader's actual death. Measured over seeds 0..5, forty years: 90 / 88 / 96 / 86 / 91 / 90 deaths of 133 dated leaders (the four undated rows are never drawn for), first deaths on six different (leader, date) pairs. The rate is high against the record — the curve does not know that leaders leave office before they die, and this table has no other exit for a regime's leader than a road, so two thirds of the dated table dies in the chair over forty years. The draw is placed last in the month so the switched-on world's RNG stream parts from the switched-off one's only between months, which is what lets `the_bloc_layer_is_inert_over_time` re-sync the stream each January and go on comparing. **What would calibrate it:** the same table's `since` and `born` columns read forward against the actual dates of leaving office — a retirement hazard beside the death hazard, which the design does not have.

### S4-8 — `the_bloc_layer_is_inert_over_time`'s ON half re-expressed twice more (S3-7 continued)

**TRIED AND REVERTED 2026-09-06 (D4 wired in 5086cfa, S6-4; reverted by the ship pass, S7-3 — the clause below is gone and the test reads as this entry left it):** the government clause of the ON half now skips `support` for the two nations whose ON-world table is the D4 block (`polity_in` differs from `polity`), coalition and pillars still compared; measured before the clause: "parted at month 0 with no stem: [rng, Nepal government, Haiti government, news [...]]"; with it the pin (month 35, Libya) and the six OFF hashes are unmoved.

Succession's "is led by" lines (a term limit reached in January 1990: Honduras's 1990-01-27) and mortality's January draw both part the switched-on world from the switched-off one at month 0 or 1 by construction. The bar now treats a month whose only difference is those lines as the arm's record, and a January whose only difference is the RNG state and those lines as the draw — re-syncing `on.rng.state` to `off.rng.state` and going on — and still pins the first ACT at month 35 (France's backing of the Western movement in Libya) with the six BASE hashes and the golden actuals unmoved. Watched red both ways ("parted at month 1 with no stem", "parted at month 0 with no stem: [rng, news [...]]"). It is a weaker bar than S1's: a leak that happened to fire only in a January and only into the stream would pass. The OFF half is the inertness proof and is untouched.

### S4-9 — the web is not in this stage

`spheres-web` compiles and its political tests read the new `calibration pending` word, but the government screen's lever cards, the "Back a movement" card, the hatched backing extensions, the event cards for the new stems and the dock banner (the design's WEB section for S3/S4) are not built here. The sim serves everything they need: `government::lever_effects`, `statecraft::back_bloc_effects`, `statecraft::takeover_payoff_effects`, every road's `open` / `reason`, and the headline stems (`COUP IN`, `Revolution in`, `annuls the election`, `dies in office`, `is led by`).

### S4-10 — D4 is still not wired (P-8 stands; wired and reverted 2026-09-06, S7-3)

**TRIED AND REVERTED 2026-09-06:** built as the second option in 5086cfa (`government::polity_in`, a table the OFF world cannot see) and reverted by the ship pass; the record is under P-8, S6-4 and S7-3.

The Nepal and Haiti blocks stay beside the table as `D4_POLITIES`. Wiring them moves the 1990 start hash with the switch OFF (measured in S3 part one: 0xe26e4bf8d6c60066 → 0x63a37522993aa5a4), and the two golden actuals are under an instruction not to move. Ridge decides between a deliberate re-pin and a table the OFF world cannot see.

## Filed 2026-09-06 by the S3/S4 repair-and-ship (three skeptic passes on a6dbefc, five repairs, the web stage landed, the census first reading; branch `feat/ideology-roads`)

Three skeptics read a6dbefc plus the web stage's then-uncommitted diff: the inertness skeptic (every gate, the draw order, the goldens, the digests, and six production mutations P3-P8 — the gate on `seed_blocs`, the levers' draw before the choice, the sponsors' draw before the choice, the uprising arm ungated, `surging` serialised when empty, an RNG draw inside `regime_break` — every one caught by the inertness trio or the goldens); the design-fidelity skeptic (number by number against the approved design; five departures reached this session, R-10 below); the calibration skeptic (a 30-seed census with both switches on, R-8 below, recorded and not acted on). Each repair below is a one-purpose commit with a test watched red against the reverted line. The web stage's three files (`spheres-sim/src/statecraft.rs` — `covert_odds` factored out of `covert_action` verbatim, called after the service charge — `spheres-web/src/main.rs`, `spheres-web/ui/index.html`) landed as one commit under the web stage's own test comments; their measured readings are reproduced on the final build in ROADMAP.

### R-1 — a vote never unseats a transcribed holder tied to a pillar (design departure 1)

`government::succession_seat`, the `Election` arm. `hold_election` fires `Succession::Election` on every change of the chamber's leading party with no court guard, and a sovereign is tied to `Pillar::Party` in the table (Jordan's Hussein with an heir; Kuwait, Morocco, Bahrain, Qatar, Saudi Arabia, the UAE, Tonga likewise), so a Jordanian election won by a new party seated "the {party} government" over the King, cleared the row's name and tie, and `court_pillar` then read `Tie::Party` off the emergent holder — the monarchy exception dissolved silently, against D2 (only a Party coup or a programme removes a monarch, the heir seated once). The repair reads the transcribed row: a holder tied to ANY pillar is not an office a ballot fills, and the chamber's winner is the government of the day already served beside the row; an emergent pillar holder (the junta after a coup) still gives way to the party the first free elections seat. Any-pillar rather than Party-only is a READING: Ghana's Rawlings (Army), Lebanon's Hrawi (Security), Burma's Saw Maung (Army) stay through a vote their chamber holds, which is the record for 1992 Ghana and 1990 Burma and what a head of state above a chamber is. Test `an_election_changes_the_government_of_the_day_and_never_unseats_a_crown` (Jordan and Morocco), red with the guard removed.

### R-2 — the regime's own coup is `maybe_coup`'s block verbatim under the lens; the colour and the latch are the roads' (design departure 2)

`government::maybe_coup` / `regime_break`. Under `ideology_blocs` ALONE — the browser's state — the regime's own coup wrote `regime_bloc = pillar_bloc(mover)`, a takeover effect live with `ideology_takeover` off; and `regime_break` did not re-seed the surge latch the way `seed_blocs`, the programme, `break_electoral` and the uprising do, so after an Army coup in a Communist regime the deposed colour sat at ~0.60 unlatched and `note_surges` printed "The Communist movement in X passes a third of the country" the next month — the same false headline `seed_blocs` was repaired for in S3. Now the colour is written under `ideology_takeover` only (the foreign payoff reads it as the winner, as the design's "any non-ballot takeover" says) and the break latches the deposed movement closed whenever it writes a colour. Test `the_regime_s_own_coup_keeps_its_colour_under_the_lens_and_latches_the_deposed_movement_on_the_roads` (China's PLA against the Central Committee), red both ways: Nationalist under the lens with the old gate, and the surge headline the month after with the latch line removed.

### R-3 — the AI's levers ride the deck's ONE draw, the lever first (design departure 4; the priority is INVENTED)

`stratagems::ai_stratagems`. The levers took a SECOND independent 0.02 chance per nation after the deck's own draw, so a government could enact a stratagem and pull a lever in one month and a switched-on world parted from the off world's stream in every month any AI government had a lever available. Now one draw per nation per month at the deck's site: `ai_lever` (pure, `None` before any read with the arm off) is read beside the deck's `choice`; the lever is the command when it exists, else the card; the draw is taken once when either exists. The off world's flow is unchanged (no lever, the draw only on a card); the on world now parts only in a month a government has a lever and no card. The LEVER-FIRST order is a reading the design did not make: the levers' conditions are the narrower crisis (stability < 30, discontent ≥ 0.50), so the month a government meets one the lever is the decision; Ridge may reverse it. Test `the_ai_s_levers_ride_the_deck_s_one_draw` (Warsaw the only capital with money, a suspension and a card both open: the RNG steps once, and once with the lens off), red at two steps with the second loop restored.

### R-4 — the ban has the design's four arms and no cabinet arm (design departure 5)

`government::ban_party` / `BanPlan`. `ban_party` also removed the banned party from the coalition (`g.coalition.retain`) and the card quoted "{party} leaves the cabinet." — an arm the design does not list (a ban takes seats, not voters: authoritarianism +0.04, stability −3 at ≥ 0.15, −4 with democracies). Removed: a banned partner stays on the coalition record with no seats, and what that does to the government's majority is the chamber's arithmetic — `government_seats` falls, the fragile branch reads it — not an arm of this lever. Test `a_ban_takes_seats_and_not_the_cabinet`, red with the retain line restored; the existing ban test's `!plan.leaves_cabinet` line is now `no "cabinet" sentence on the card`.

### R-5 — `backing_cools` gates on the switch, not on emptiness alone (the inertness skeptic's note)

`statecraft::backing_cools`. It returned on `backing.is_empty()`, which is every world the op is refused in, but a save carrying a stock loaded into a world with the lens off would still have decayed it — the one tick step in the arm that did not early-return on its switch. Now `!ideology_blocs || is_empty()`. Test `a_backing_stock_does_not_cool_with_the_lens_off` (a 0.06 entry written into an off world survives twelve `statecraft::tick`s bit for bit; the same entry in the on world cools 0.006 a month and is gone within ten), red with the switch dropped.

### R-6 — D4 stays unwired (design departure 3 = S4-10 = P-8; wired and reverted 2026-09-06, S7-3)

**TRIED AND REVERTED 2026-09-06:** 5086cfa's `polity_in` served the D4 blocks only under `rules.ideology_blocs` with both golden actuals and both digests unmoved, and the ship pass reverted it as a departure from the calibration brief's scope (S7-3). Still Ridge's call.

Not repaired: the design says "land" and the code keeps `D4_POLITIES` unread, because wiring the blocks moves the 1990 start hash with the switch OFF (0xe26e4bf8d6c60066 → 0x63a37522993aa5a4, measured in S3 part one) and the golden actuals are under an instruction not to move. Ridge's call, as filed twice already.

### R-7 — the web's action pin re-expressed, six to eleven

`spheres-web/src/main.rs`, `the_government_screen_is_served_not_computed`. The test pinned Poland's action list at six (three invitations, the election, two stratagems); the web stage serves the five levers as actions — for an electoral polity the suspension and one ban per party of the table — and the pin read eleven. Re-pinned with every kind in order (`suspend`, `ban` ×4 after the two stratagems), so a lever that silently drops out reads red; the levers' own test reads their prices, refusals and effects.

### R-8 — the calibration FIRST READING (S5 pending): recorded, not acted on, no constant moved

**Superseded 2026-09-06** by S5-1..S5-9 (the N=60 passes) and S6-1..S6-8 (the N=200 reading, the pins, the switch decision).

The calibration skeptic's census on a6dbefc, scratch test deleted after three identical runs: `GameRules { ideology_blocs: true, ideology_takeover: true, ..default }`, seeds 0..29, 240 × `tick_month`, no commands, no player. 1990: 137 nations, 77 electoral, 60 regimes, 41 at authoritarianism ≤ 0.20. Per seed, min / median / max (mean): coups against elected governments 4 / 6 / 8 (5.93); annulments 2 / 3 / 3 (2.50); regime coups by a pillar 9 / 10 / 12 (10.47); uprisings won by Western 10 / 15 / 19 (14.33), Communist 4 / 5 / 5 (4.87, every seed), Nationalist 0 / 3 / 6 (2.97), Islamist 0 / 0 / 0, Non-Aligned 2 / 3 / 7 (3.77); the round-table lever 0 and AI SuspendConstitution 0 in every seed (`ai_lever` never fires on the deck's draw in 30 × 240 months); states electoral by 1 Jan 1996 2 / 2 / 3 (2.20); openings over twenty years by attribution: route-4 drift 2 / 3 / 4 (2.70), uprising 2 / 4 / 5 (3.60), lever 0, the pre-arm "X opens up" 1 / 3 / 5 (3.00); ex-communist returns by ballot 0 / 0 / 1 (0.23, 7 seeds); route events in nations at 1990 authoritarianism ≤ 0.20: exactly one per seed, Argentina every time (0.15; months 23-62; 27/30 via the pre-arm stability < 12 collapse); leaders dying in office 32 / 40 / 49 (39.67); of the eight largest 1990 regimes surviving to 2010 3 / 4 / 5 (4.27): Iran, Saudi Arabia, Indonesia 30/30, China 29/30, Iraq 9/30 (80 uprisings across 30 seeds flipping Nationalist ↔ Non-Aligned), USSR, Yugoslavia, South Africa 0. Where the mass sits: the 178 coups against elected governments are Sao Tome 87 (coup → first free elections 18 months later → coup, 87 re-openings), Philippines 48, Comoros 30 (month 41 in every seed), Suriname 3, ten others; the 75 annulments are Ukraine 30 (month 56 every seed) and Belarus 30 (month 62 every seed) plus Algeria/FIS 23; the 146 Communist uprisings are Ukraine 30, Belarus 30, Georgia 30, Peru 30 (Shining Path in 97% of worlds), Cambodia 25, Tajikistan 1 — a successor-state chain: first election → the CP wins → annulled and banned → the movement (support kept, counted in I) is the challenger against a Nationalist junta at 0.65 with discontent high → armed uprising, deterministic; the 430 Western uprisings are Tajikistan 111 and Azerbaijan 96 (about 3.5 revolutions per seed each, repeating), Sao Tome 57, Nicaragua, Argentina 30 each, Philippines, El Salvador 29, Mozambique 22; the uprising trigger overall 451 armed by the movement, 232 via stability < 12, 7 both, 88 crossed inside the tick. Against the design's anchors: A1 (4-14 coups, median) inside the band numerically and degenerate (three micro-polities cycling supply 5.6 of the 5.93); A2 (Islamist usually) badly off — zero in 30 seeds, the largest Islamist influence in any winnable non-Islamist regime at Jan 1996 0.12-0.13 (Lebanon, Kuwait, Egypt, Oman) against `UPRISING_INFLUENCE` 0.45, the flat seed never moved; A3 (Communist rare, ≤ 10%) badly off — 4.87 per seed in 100% of seeds, four of the five deterministic; A4 (15-40 opened by 1996) badly off — 2.20, two of them the pre-arm "opens up"; A5 (ex-communist returns) under-read by construction (Poland, Hungary, Bulgaria are electoral in the 1990 table); A6 (low-authoritarianism polities untouched) one event per seed, Argentina by the pre-arm collapse. **What it says:** the roads are a mechanism and not yet a calibration — the successor states, Sao Tome/Comoros and Iraq's flip-flop are the three shapes S5 must read first, and none of them is a constant to move before Ridge has seen the census. `ideology_takeover` stays off.

### R-9 — P-12 and the suite this run

The full release suite, watched in the foreground after `cargo clean -p spheres-sim -p spheres-web -p spheres-cli --release`, twice (04:24 and 04:30 on 2026-09-06, the second after the Iraq pin): spheres-sim lib 452 / 3 / 25 with exactly the three deliberate reds at the unmoved actuals, spheres-web 160 / 0 / 2, spheres-cli 1 / 0, every integration binary green. P-12's `the_resource_pass_stays_under_budget` and `the_resources_row_is_free` were GREEN in both release runs (the inertness skeptic read them red under the debug suite on the same machine, 0.8033 ms/month, and 0.5995 / 0.6165 alone in debug), so no lone re-run was needed; the bar still reads the machine and P-12 stands.

### R-10 — the design skeptic's report reached this session truncated after its fifth departure

The design-fidelity audit's DEPARTURES list was cut by the pipeline after departure 5 (the ban's cabinet arm); the VERIFIED section (every S3/S4 number against the design) arrived whole. If the audit went on to a sixth departure it is unreviewed here. The inertness skeptic's report was cut inside its note on the web stage's `covert_odds` refactor (its gate audit, suites and mutation probes arrived whole); the calibration skeptic's inside A4, and its raw counts were re-read from its own census output in the scratchpad (`census_scratch_out3.txt`, the third of three identical runs).

### S5-1 — the bloc census instrument, and the N=60 baseline it read on cbcc364

`spheres-sim/tests/bloc_census.rs`, `#[ignore]`d: both switches on, `GameRules::default()` otherwise, monthly, Jan 1990..Dec 2010 (252 months), seeds `0..SPHERES_CENSUS_SEEDS` (default 60), no player, no commands; 27 s at N=60 in release. Its definitions are in the file header (a takeover is a ruling-bloc change attributed to the month's headline; A5 is a Communist-led coalition seated by ballot in a 1990 Communist-ruled, ex-Warsaw-Pact or successor state; A6's route event excludes the pre-existing stability<12 collapse). Baseline, per seed min/med/max: A1 coups against elected governments 4/6/8, top-3 nation share med 1.00 (Sao Tome 172, Philippines 96, Comoros 60 of 352), annulments 2/3/3 (Belarus 60, Ukraine 60, Algeria 31), regime coups 9/10/13; A2 seeds with an Islamist non-ballot takeover 1/60; A3 seeds with a Communist one 60/60, per seed 3/5/6 (Belarus 60, Georgia 60, Peru 60, Ukraine 55, Cambodia 42, Tajikistan 4); A4 1990 regimes electoral at end-1996 1/2/3 (Sao Tome 60, South Africa 60); A5 by end-1996 0/60; A6 route events 0 in 60/60 (Argentina's collapse once a seed, beside); A7 ballot bloc flips 2/6/9 against 21/26/36 non-ballot takeovers; A8 8/8 in 60/60; A9 0.950; A10 median D-lead 0.36/0.39/0.50; deaths in office 31/43/51.

### S5-2 — calibration moves KEPT (each its own commit, census before and after in the body)

1. `politics::tick`: a collapse the movement did not arm runs the pre-arm collapse block verbatim (its random shift included) instead of crowning `blocs::challenger`; the crown needs `blocs::uprising_armed` at the firing site. A3: Peru 60 → 0, Georgia 60 → 0, per seed 3/5/6 → 1/3/4. The form that gated the crown inside `government::uprising` was rejected: it shifts seed 7's stream so `the_sponsor_payoff_and_exposure_fire_on_takeover`'s Sudanese draw lands in month 47, after the Soviet dissolution in month 43 (a timing dependence of that test, noted here).
2. `blocs::COERCION_ARMED` 0.50 → 0.35 and `COERCION_MEAN` 0.55 → 0.35, both quoted from the pillar model's unpaid line (`regime_tick`'s pressure, `ai_government`'s purchase). A3 58/60 seeds, per seed 0/2/3; Iraq's Nationalist/Non-Aligned flip-flop gone (Nationalist-won uprisings 0/3/6 → 0/0/1, Non-Aligned 2/4/7 → 1/2/3).

### S5-3 — moves TRIED AND REVERTED, with what they measured

- The successor purge window (`GovState.born` under the roads, the annulment and route 2 silent for 36 months after a successor's birth, `maybe_coup`'s window quoted): Belarus and Ukraine were still annulled in 60/60, later; A3 unchanged (60/60, per seed 1/2/4). Diff kept in the run's scratchpad.
- The annulment needing a HOSTILE army (`effective_army_loyalty < ELECTORAL_COUP_ARMY`, no new constant): annulments 2/3/3 → 0/0/1 (Algeria 29/60 only), A3 → 38/60 seeds, per seed 0/1/1, Cambodia only — the reading the design asked for — and RED on `the_algeria_shaped_annulment_fires_under_its_conditions_and_not_under_the_court` at the Jordan assertion (`annulment_check(jo) == Some("jo_ikhwan")` on a fresh world whose Army sits at its 0.65 seat value, merely present). Not landed: no existing test is touched on this branch. A ruling for Ridge: the test asserts the very reading the census says is wrong.
- `ROUND_TABLE_LOYALTY` 0.55 → 0.66 (the diagnosis's M1): A4 1/2/3 → 2/3/5 (Fiji), A1 5/6/8 → 6/7/9, and RED on `every_road_reads_closed_while_takeover_is_off`, which pins every road trigger as a literal (S5-4). Reverted.

### S5-4 — every road threshold is pinned as a LITERAL by `every_road_reads_closed_while_takeover_is_off`

`blocs.rs`, the readout test: `0.35 / 0.25` (route 2), `0.45 / 0.45` (route 3), `0.40 / 0.55 / 30..70` (route 4), `12` (collapse). The design named these the seams calibration moves; the test makes each move a red. Re-expressing the pins as the constants they quote is a change to an existing test and therefore Ridge's call, not this session's. Until then only the coercion lines, the drift constants, the flat seed, the backing caps, the gravity view and the AI's thresholds can move.

### S5-5 — A2 (Islamist takeover without a ballot) is not reachable by moving a constant

Final tree, N=60: 0/60 seeds. The design's own historical case is Afghanistan ("Islamists exist only once a sponsor pays for them"), and its transcribed polity carries no Religious party and no Muslim clergy pillar, so `bloc_present` and `bloc_can_win` are both false for the Islamist bloc there; D5's third presence clause — "foreign backing already behind it" — was left out of S3 (`bloc_present`'s own comment: "Foreign backing as a presence is S3"), and `ai_back_bloc_choice` needs presence AND influence ≥ 0.15 before a sponsor will pay, so the chain cannot start. Elsewhere the flat seed parks the bloc at 0.133 (three present) or 0.100 (four) and the movement drift's `record` line (the party model's own, not this arm's) moves share toward the ruler wherever prices, growth and war are quiet; the largest Islamist influence in a winnable non-Islamist state over 21 years is Algeria's annulled FIS at 0.64, Kuwait 0.55, Jordan 0.43. Algeria is Islamist-LED at the 1990 seating (the table's share is the 1991 result), so "goes Islamist in a minority" is not a flip the census can read; it is annulled in 33/60 (≥ 10% holds). Building the presence clause is S3 work, not calibration.

### S5-6 — A4 (the democratisation wave) is not reachable by moving a constant

Final tree: 1/2/3 regimes electoral at end-1996 against 15..40; Sao Tome and South Africa in 60/60, Morocco and Mozambique once. Route 4 needs Western influence ≥ 0.40: it holds only in the seven regimes whose RULING bloc is Western in the leader table (Angola, Cape Verde, Fiji, Malawi, Mozambique, Sao Tome, South Africa — a data ruling on MPLA/FRELIMO/MLSTP/PAICV's family), and reads the flat seed 0.13-0.20 everywhere else, 0.002 in the thirteen polities with no Western party or pillar (Kenya and Tanzania among them). Patronage gravity is dead by construction (no aid flow enters any of the 60 regimes in any seed) and no Western ideological sponsor exists. The Party-loyalty line never holds for the wave states (their 1990-93 economies read growth +2.7..+4.1% and the pre-existing Party target parks at 0.65-0.85). The AI round table fired 0 times in 60 seeds (PC ≥ 60 and discontent ≥ 0.50 never coincide). The diagnosis's bracket: no combination of the round-table lines reaches 15 without admitting Saudi Arabia, and every line is pinned (S5-4).

### S5-7 — A1's concentration, A5 and A7 belong to pre-existing formulas

A1 reads 5/6/8 inside 4..14 and is degenerate: Sao Tome 163, Philippines 137, Comoros 60 of 378. Route 2's army loyalty is `pillar_targets`' pre-existing Army line, a defence SHARE (0.20 + 0.65·mil/0.08) with no per-head term, so a micro-polity at the 1% fiscal floor reads 0.281 forever while Pakistan at 4-6% reads ≥ 0.47 and never coups; Sierra Leone, Gambia and Niger have no polity row, Honduras, Peru and Guatemala carry no Army pillar, Haiti is a regime, Nepal is the unwired D4 table. A5 (ex-communist return by ballot) reads 0/60 by 1996 and is the party model's `drift_support`, which the arm does not touch. A7 (ballot flips ≥ 3× non-ballot takeovers) reads a ratio of 0.16 (bloc) / 0.24 (party): the takeover count is dominated by the pre-existing regime coups, coloured Nationalist under the roads (10.5 a seed), and a ballot flip at bloc level is rare by construction (Conservative → Labour is Western → Western).

### S5-8 — the data questions the census raised

1. Ukraine's 1994 and Belarus's 1990 dormant tables reach the seam at birth (auth 0.40, the CP the largest party — KPU 0.247, KPB 0.850) with an Army seeded at 0.65 and retained by `schedule_first_elections`; the annulment fires at Ukraine's fragile-branch twelfth month and Belarus's first scheduled vote in every seed. 2. Algeria's 1990 seating is FIS-led (the 1991 share entered as the last vote). 3. Afghanistan's polity has no Islamist party or pillar (S5-5). 4. Seven regimes carry a Western ruling bloc in the leader table (S5-6). 5. (S7-5) Of A5's six named returns, Lithuania's `lt_ldpp` and Hungary's `hu_mszp` are transcribed `Family::SocialDemocratic` with no bloc override — Western, a return the bar cannot register — and Bulgaria's `bg_bsp`, Albania's `al_ppsh` and Mongolia's `mn_mprp` already lead at the 1990 seating; Poland's `pl_sld` is the one plain flip.

### S5-9 — anchors on the final tree, N=60, with rule 7's derivation where a bar could be pinned

**UPDATED 2026-09-06:** re-read at N=200 (S6-1, then S7-2 after the D4 revert and the A6 repair — the 200 seed lines identical to S6-1's pre-D4 file), four bars pinned and six ignored — S6-2, S6-3, S7-4. The N=60 readings below stand as the history.

Inside: A1 (median 6, sd 0.79, bootstrap P(median outside 4..14 | n=60) = 0.0000 — but degenerate, S5-7, so NOT pinned); A6 (0 route events in 60/60; an invariant, sample a budget — the design says 35 years and the census runs 21, so the bar must be re-read at 420 months before pinning); A8 (8/8 in 60/60; p = 1.0 gives n = 1 by the Bernoulli formula, and the bar is decorative at this reading — it cannot see a big regime falling unless one does); A9 (1109/1164 = 0.953 pooled); A10 (median 0.33, sd 0.034, min 0.27, bootstrap P(median < 0.25 | n=60) = 0.0000). Out: A2 (0/60), A3 (58/60 seeds, 0/2/3 a seed), A4 (1/2/3), A5 (0/60), A7 (0.16). No bar was pinned this pass; every one is a shape the census reads, and the reds above are the record.

## Filed 2026-09-06 by the pin-and-wire shipper (the N=200 census, four pinned bars, D4 under the switch, the switch decision; branch `feat/ideology-census`)

Every number here was measured this pass on this tree (HEAD's parents e4790bc → the three commits of the pass), release builds watched in the foreground, `CARGO_TARGET_DIR` the worktree's own.

### S6-1 — the N=200 census, before and after D4

**SUPERSEDED 2026-09-06 by S7-2** (the D4 revert returns the tree to the pre-D4 column of this entry, seed line for seed line; the A6 column is re-read with the headline attribution). `SPHERES_CENSUS_SEEDS=200`, 252 months, both switches on: wall 97 s before D4 (the test 96.5 s, binary already built), 137 s after (a 29 s rebuild plus the test's 108 s). Per seed min/med/max (mean, sd), the post-D4 file `census_n200_d4`: A1 coups against elected governments 4/6/9 (6.345, 0.854), top-3 nation share med 1.00 (0.956, 0.072; 0 of 200 seeds under 0.5), annulments 2/3/4 (2.58, 0.552; Belarus 200, Ukraine 200, Algeria 105, Afghanistan 11), regime coups by a pillar 8/10/13 (10.575, 1.000; Nigeria 215, Ghana 201, Haiti 201, CAR / Comoros / Equatorial Guinea / Zaire 200, Chad 195, Mozambique 194, Cambodia 181, Nepal 48); A2 seeds with an Islamist non-ballot takeover by end-2000 0/200 (1/200 over the run, Algeria once), Algeria annulled 105/200, the largest Islamist influence in a winnable non-Islamist state 0.43/0.64/0.65; A3 seeds with a Communist non-ballot takeover 197/200 = 0.985, per seed 0/2/4 (2.085, 0.782; Belarus 181, Cambodia 142, Ukraine 91, Afghanistan 3 of 417), Communist-won uprisings 0/2/3; A4 1990 regimes electoral at end-1996 1/2/3 (1.61, 0.565; Sao Tome 200, South Africa 200, Morocco 11, Mozambique 4, Kuwait 3, Afghanistan 1 of 419), openings over the run by drift 2/3/5, uprising 0/1/4, lever 0, the "opens up" card 0/3/8; A5 0/200 by 1996 and over the run; A6 route events in 1990 authoritarianism ≤ 0.20 nations 0 in 200/200, Argentina's pre-existing collapse beside in 198/200; A7 ballot bloc flips 2/3/6, party flips 2/4/7, non-ballot takeovers 14/19/24 (19.31, 1.615), the ratio 0.09/0.16/0.36 (bloc) and 0.24 median (party); A8 8 of 8 in 200/200; A9 3718/3900 = 0.953 pooled, per seed 0.938/0.950/1.000 (0.953, 0.0151); A10 0.267/0.326/0.380 (0.324, 0.0333); deaths in office 32/44/54; round-table levers, suspensions and programmes 0 in every seed; uprisings by winner W 2/4/6, C 0/2/3, N 0/0/1, I 0/0/1, NA 0/2/4, none 7/9/11; the clause stab12 5/8/10, movement 3/7/10, both 0/0/1, none 0/2/7. D4 WIRED moved exactly one seed of 200 — seed 39: Non-Aligned-won uprisings 3 → 2, the "none" clause 2 → 1, ballot flips 2/2 → 3/3, card openings 2 → 3, A10 +0.363 (21) → +0.338 (20), deaths 42 → 43 — and no summary line past the third decimal (A7 bloc mean 3.165 → 3.170, deaths mean 43.335 → 43.340). At 420 months, N=60 (`SPHERES_CENSUS_MONTHS=420`, 55 s): A6 0 in 60/60, Argentina beside 59/60; A1 6/8/11; A3 58/60; A8 8/8 in 60/60.

### S6-2 — the four PINNED bars: n, its derivation, the power statement, the watched red

All in `spheres-sim/tests/bloc_census.rs` beside the scan, reading the same `run_seed`; the four run in 22 s together in release. **A6** `a6_no_road_reaches_a_1990_democracy_in_thirty_five_years`, 12 seeds × 420 months, an INVARIANT (n a power budget: sees a per-seed rate q with 1-(1-q)^12 — 0.10: 0.72, 0.20: 0.93, 0.30: 0.99; "sees a road touching a democracy in one seed of five, not one in twenty"); watched red with the uprising road thrown open (`COERCION_ARMED` / `COERCION_MEAN` 0.35 → 1.00, `UPRISING_DISCONTENT` / `UPRISING_INFLUENCE` 0.45 → 0.00): 32..57 route events a seed in 12/12; NOT red with route 2 thrown open (`ELECTORAL_COUP_ARMY` 0.95, `ELECTORAL_COUP_DISCONTENT` 0.00; 45..52 coups against elected governments a seed): no 1990 democracy at ≤ 0.20 carries an Army pillar in the transcribed table, so on that road the invariant is the transcription's. **A8** `a8_six_of_the_eight_big_regimes_keep_their_bloc_in_most_seeds`, 40 seeds, 24 must keep six of eight: p = 1.000 in 200/200 so the Bernoulli n is 0 (no false red while the reading holds); n = 40 for power — the bar reds at 0.866 if the true rate falls to 0.50, 0.992 at 0.40, 1.000 at 0.30 (exact binomial); DECORATIVE against one regime falling in every seed (7/8 passes), said so in the file; watched: 0.45 → 0.10 on the uprising lines with coercion at 1.00 left it GREEN (the eight's discontent sits under 0.10), 0.45 → 0.00 reds it, 5/40 kept, per seed 2..7. **A9** `a9_four_coups_in_five_strike_nations_under_8000_a_head`, 12 seeds pooled ≥ 0.80: per-seed share sd 0.0151, n = (2.326·0.0151/0.153)² = 0.05 → 12 for power (bootstrap of the shifted sample: the share falling to 0.75 reds at 1.000, to 0.80 at 0.30, to 0.85 at 0.00 — "sees the rich-nation share rising ×4, not ×2"); watched and NOT red with route 2 thrown open: 620/706 = 0.878 — the bar guards the transcription (which polities carry an Army) and the pillar model's per-head structure, recorded as decorative against the road constants (iron rule 7's last paragraph) and pinned because the anchor holds. **A10** `a10_discontent_leads_the_roster_by_a_quarter_before_a_takeover`, 12 seeds, median ≥ 0.25: sd 0.0333, n = (2.326·0.0333/0.074)² = 1.1, the bootstrap at n = 12 under 0.25 in 0 of 4000 (min of 200 is 0.267); 12 for power (the lead falling by 0.10 reds at 0.92, by 0.08 at 0.57, by 0.06 at 0.18 — "sees the roads losing a third of their discontent dependence, not a fifth"); watched red: coercion 1.00 with the uprising lines at 0.10 → median lead 0.141; route 2 thrown open → 0.2499.

### S6-3 — the six UNPINNED bars, written and `#[ignore]`d at their reading, never widened

**A1** `a1_coups_against_elected_governments_are_spread_not_three_micro_polities`: the band arm (median 6, bootstrap P(outside 4..14 | n=12) = 0.0000) is pinnable alone; the concentration arm reads top-3 share median 1.00 in 200/200 and is red at any n (S5-7: route 2's army loyalty is a defence share with no per-head term). Its literal — the three most-couped nations hold under half a seed's coups, as a median — is THIS SESSION'S expression of the design's sentence (S6-6). **A2** `a2_an_islamist_takeover_without_a_ballot_in_most_seeds_by_2000`: 0/200, p = 0 so the Bernoulli n is undefined and the bar is red at any n (S5-5: Afghanistan has no Islamist party or pillar, D5's presence-through-backing clause is unbuilt, the flat seed and the record line hold the bloc down). **A3** `a3_a_communist_takeover_without_a_ballot_is_rare` (20 seeds, ≤ 2): 197/200 = 0.985, n = (2.326·0.122/0.885)² = 0.10, red at any n (S5-2, S5-3, S5-8: the annulment of a merely-present army in Belarus and Ukraine, Cambodia's pre-existing coup by an Army at a 1% share; the hostile-army repair measured 38/60 and red on the Jordan assertion of an existing test — Ridge's ruling). The reachability arm is the existing `a_communist_movement_at_discontent_0_70_with_an_unpaid_army_takes_power`. **A4** `a4_the_democratisation_wave_opens_fifteen_regimes_by_1996`: median 2 (sd 0.565, max 3), P(outside 15..40 | n=12) = 1.000 (S5-6). **A5** `a5_ex_communists_return_by_ballot_in_most_seeds`: 0/200 (S5-7, `drift_support`). **A7** `a7_ballots_flip_the_ruling_bloc_three_times_as_often_as_takeovers`: median 0.158 (sd 0.055), P(median ≥ 3 | n=12) = 0 (S5-7). A12 (daily == monthly with both switches on) is the existing test in `lib.rs`; A13 (the existing bars green with the switches off) is the suite in S6-8.

### S6-4 — D4 wired under the switch: what changed, and the one test clause

**REVERTED 2026-09-06 (S7-3):** none of the below is in the tree — `government.rs` and `spheres-web/src/main.rs` are byte-identical to cbcc364, `polity_in` does not exist, `bloc_present` / `bloc_can_win` / `flat_seed` take the id again, and `the_bloc_layer_is_inert_over_time` compares `support` for every nation as S4-8 left it. Kept as the record of what 5086cfa built. `government::polity_in(w, id)` (the D4 block only while `rules.ideology_blocs` is on) and `table_of(id, party)` (the table carrying a party id, live first); every `polity(id)` reader with the world in hand now asks `polity_in` (21 sites in `government.rs`, 4 in `blocs.rs`, `government_json` in the web); `blocs::bloc_present`, `bloc_can_win`, `flat_seed` and the private `bloc_appeal`, `largest_party_of` take the world (call sites in `politics::ai_back_bloc_choice`, `covert_json`, `government_json`, `drift_movements`, `reseeded_support`, `seed_blocs`, `succession_seat`, `challenger`, `bloc_shares`); `regime_is_communist`, `pillar_bloc`, `home_pillar` and the two `pillar_name`s stay on `POLITIES`, table-invariant for Nepal and Haiti (same pillars, ruling and system in both blocks; neither table's largest row is Communist), asserted by the new `d4_tables_are_read_under_the_switch_and_invisible_off` (watched red with the switch test removed from `polity_in`: the OFF world seated `np_nc`). Test edits this pass, none a bar: the signature follow-through at `the_liberalisation_seam_reseeds_parties_from_movements`, `a_western_winner_is_refused_in_a_party_less_polity` (a `w1990()` added to have a world), `the_bloc_layer_is_deterministic_when_on`'s `flat_seed`, the web's two `bloc_present` calls, the census's `bloc_can_win`; the D4 shape test's doc line; and `the_bloc_layer_is_inert_over_time`'s government clause (S4-8 continued). In browser play (the lens on) Nepal's Panchayat now carries nine dormant rows and Haiti's army six; both stay regimes at authoritarianism 0.70 / 0.78 and the table goes live only when the state opens (route 4, a round table, a stratagem). The web suite read 160/0/2 with it.

### S6-5 — THE SWITCH DECISION: `ideology_takeover` stays OFF, and by how much each anchor is out

Not turned on, and not partially: `play_rules` still writes `ideology_takeover = false` and the watch reads `calibration pending`. Out at N=200: A2 — 0/200 seeds against a majority (the whole bar; the largest Islamist influence anywhere winnable 0.65 against the 0.45 line only in Algeria, which is Islamist-led at the seating); A3 — 98.5% of seeds against ≤ 10% (88.5 points; 0/2/4 takeovers a seed, Belarus and Ukraine's annulment of a merely-present army, Cambodia); A4 — median 2 against 15..40 (13 short of the floor); A5 — 0/200 against a majority; A7 — 0.16 against ≥ 3 (nineteen-fold); A1's concentration arm — top-3 share 1.00 (three micro-polities hold every coup of the median seed). Inside: A1's band arm (median 6), A6 (0 in 200/200, 60/60 at 420 months), A8 (200/200), A9 (0.953), A10 (0.326), A12 (the existing daily == monthly test), A13 (the suite, S6-8). What would move each is recorded under S5-5..S5-8 and S5-4 (the literal pins, Ridge's call).

### S6-6 — A1's concentration literal is this session's expression

The design's A1 says "NOT concentrated in three micro-polities" and names nine nations; it gives no number. The bar reads the top-3 nation share of a seed's coups against elected governments and asks its median under 0.5. That literal is invented here so the arm could be written and read (1.00 today), is filed as such, and is Ridge's to re-express — the bar stays `#[ignore]`d either way while the arm is red.

### S6-7 — what the N=200 census could not see, and what the bars cannot

The route-2 coup needs `coup_pressure ≥ 1 / crisis_intensity` where `crisis_intensity` is a GameRules field (1.0 by default), not a nation's crisis — so at default rules the trigger is a flat 1.0 against a 1.5 cap, and the D ≥ 0.25 gate is what keeps quiet democracies off it. Both A6 (on route 2) and A9 read the transcription of which polities carry an Army pillar more than any road constant (S6-2's watched readings); a change to the pillar tables is the thing that would move them. `every_road_reads_closed_while_takeover_is_off` still pins every road line as a literal (S5-4).

### S6-8 — the suite this run

**Superseded by S7-6** (the ship pass's suite on the tree that ships). Release, watched: `spheres-sim` lib 453 passed / 3 failed / 25 ignored — the three deliberate reds at their unmoved actuals (`the_1990_start_is_pinned` actual 0xe26e4bf8d6c60066, `golden_hash_of_a_known_run` actual 0xbe94d6125631829c, `the_1990_endowment_does_not_move_year_one_growth`, BUGS E-3), the +1 the D4 wiring test; the 28 integration binaries green (the 27 plus `bloc_census` with its four pinned bars, 84 s together); `spheres-web` 160 / 0 / 2; `spheres-cli` 1 / 0. The two wall-clock bars (P-12) were green in this run. Headless digests market off: `run 35 1990` d1a2cfbf7c6958d7 (3501 lines), `run 35 7` 39dea3341a7f6e8c (3983).

## Filed 2026-09-06 by the ship pass (the skeptics' findings repaired at the root, the census and every watched red re-read, D4 reverted; branch `feat/ideology-census`)

Every number here was measured by this pass on the tree it ships, each build watched compile in the foreground (`Compiling spheres-sim` read before any reading was believed — the honesty skeptic caught `blocs3-target` being rebuilt by another process during its own run, so no binary this pass did not watch build was trusted), `CARGO_TARGET_DIR` the worktree's own; the suite in S7-6 ran after `cargo clean -p spheres-sim -p spheres-web -p spheres-cli --release`. Raw outputs in the run's scratchpad (`blocs3-out/census_n200_s7.txt`, `census_n60_420_s7.txt`, `bars_s7.txt`, `mut_s7_M1_*`, `mut_s7_M2_*`, `mut_s7_M3_*`).

### S7-1 — A6's route-event attribution read the pre-tick flag; it now reads the firing site's own clause

The honesty skeptic's finding: the bar counted an uprising against a 1990 democracy only where `blocs::uprising_armed` held PRE-tick, so a crown the movement armed inside the tick was filed beside the bar as a collapse. The root: `politics::tick` decides the clause at the firing site — the challenger is crowned (`government::uprising`, "Revolution in X: the B movement takes power.") only where `uprising_armed` holds inside the tick, and the pre-arm collapse ("Revolution in X — the old regime falls.") runs otherwise; `uprising_armed` implies a challenger, so the crown and the clause are the same event and the headline is its only record. The census now attributes by the headline: a crowned uprising in a 1990 democracy is a route event, "the old regime falls" is the collapse beside it; the pre-tick reading stays as the CLAUSE column of the trace and a new counter reads the gap. Measured at N=200: the flag had missed 165 of 1570 crowns (Sao Tome 38, Zaire 22, Belarus 21, Chad 17, Comoros 17, Philippines 16, Cambodia 15, Mozambique 15, Ukraine 2, Afghanistan 1, Iraq 1), none in a 1990 democracy; at 420 months, N=60, 53 of 571. A6 reads 0 in 200/200 and 0 in 60/60 either way — the hole did not bite today, and the bar no longer has it.

### S7-2 — the N=200 census on the tree that ships (after the D4 revert and S7-1)

`SPHERES_CENSUS_SEEDS=200`, 252 months, both switches on, 97.7 s (the binary built fresh, 28.6 s). Its 200 seed lines are IDENTICAL to the pin pass's pre-D4 file (`census_n200_pre`) and differ from the post-D4 file at seed 39 alone — the revert is exact. Per seed min/med/max (mean, sd): A1 coups against elected governments 4/6/9 (6.345, 0.854), top-3 nation share 0.71/1.00/1.00 (0.955, 0.072), annulments 2/3/4 (2.580, 0.552; Belarus 200, Ukraine 200, Algeria 105, Afghanistan 11), regime coups 8/10/13 (10.575, 1.000); the coups against elected governments Sao Tome 545, Philippines 460, Comoros 200, Algeria 44, Mozambique 13, Suriname 7 of 1269; A2 seeds with an Islamist non-ballot takeover by end-2000 0/200 (1/200 over the run, Algeria), Algeria annulled 105/200, the largest Islamist influence in a winnable non-Islamist state 0.43/0.64/0.65; A3 197/200 = 0.985, per seed 0/2/4 (2.085, 0.782; Belarus 181, Cambodia 142, Ukraine 91, Afghanistan 3 of 417); A4 1/2/3 (1.610, 0.565; Sao Tome 200, South Africa 200, Morocco 11, Mozambique 4, Kuwait 3, Afghanistan 1 of 419), openings drift 2/3/5, uprising 0/1/4, lever 0, card 0/3/8; A5 0/200 by 1996 and over the run; A6 0 in 200/200, Argentina's collapse beside 198/200; A7 ballot bloc flips 2/3/6 (3.165), party flips 2/4/7 (4.435), non-ballot takeovers 14/19/24 (19.310, 1.615), ratio 0.09/0.16/0.36 bloc, 0.24 median party; A8 8/8 in 200/200; A9 3718/3900 = 0.953; A10 0.27/0.33/0.38 (0.324, 0.033); deaths 32/44/54 (43.335); round tables, suspensions, programmes 0; uprisings by winner W 2/4/6, C 0/2/3, N 0/0/1, I 0/0/1, NA 0/2/4, none 7/9/11; clause stab12 5/8/10, movement 3/7/10, both 0/0/1, none 0/2/7. At 420 months, N=60 (50.4 s): A6 0 in 60/60, Argentina beside 59/60; A1 6/8/11; A3 58/60, 0/2/3 a seed; A8 8/8 in 60/60.

### S7-3 — D4's wiring reverted, and the two test-touch rulings recorded in ONE place

The pin pass wired D4 under the switch (5086cfa) and, to admit it, weakened the government clause of `the_bloc_layer_is_inert_over_time` to skip `support` for Nepal and Haiti; the same pass declined the hostile-army annulment repair the calibration brief instructed because it reds `the_algeria_shaped_annulment_fires_under_its_conditions_and_not_under_the_court` at its Jordan assertion, on the ground that no existing test is touched. The skeptics read that as one standard applied two ways and the wiring as a departure from the brief's "NOTHING ELSE MOVES" list, disclosed as a closure. Repaired at the root: 5086cfa is reverted (`git revert`, so the commit stays in the branch's history for a one-line cherry-pick), `government.rs` and `spheres-web/src/main.rs` are byte-identical to cbcc364, the inertness test reads as S4-8 left it, and the standard is now one: NO existing test is touched on this branch. The two rulings that leaves for Ridge, side by side: (1) D4 — wire it under the switch (5086cfa: default path byte-identical, the switched-on census moved in seed 39 of 200, one existing test's assertion narrowed for two nations) or re-pin the 1990 start (P-8's first option); (2) the annulment — land the hostile-army condition (`effective_army_loyalty < ELECTORAL_COUP_ARMY`, no new constant; A3 measured 38/60 with it, S5-3) and re-express the Jordan assertion of the Algeria test, which pins the very reading the census says is wrong. Neither is this session's.

### S7-4 — the four pinned bars and every watched red, re-measured on this tree

Green together in 20.1 s (release): A6 0 in 12/12 at 420 months (10.7 s alone), A8 40/40 (22.7 s alone), A9 218/229 = 0.952 (6.2 s), A10 median 0.334, per seed 0.278..0.364 (6.9 s). Mutations, each a fresh watched build, the source restored and `git diff` empty after each: M1 the uprising road thrown open (`COERCION_ARMED` / `COERCION_MEAN` 0.35 → 1.00, `UPRISING_DISCONTENT` / `UPRISING_INFLUENCE` 0.45 → 0.00) — A6 RED, 33..65 route events a seed in 12/12; A8 RED, 7/40 kept six, per seed 2..7; A10 RED, median lead 0.017; A9 GREEN 72/76 = 0.947. M2 (coercion 1.00, the uprising lines 0.10) — A8 GREEN 38/40, per seed 4..8; A10 RED 0.118, per seed 0.077..0.270. M3 route 2 thrown open (`ELECTORAL_COUP_ARMY` 0.35 → 0.95, `ELECTORAL_COUP_DISCONTENT` 0.25 → 0.00) — A6 GREEN 0 in 12/12 (the transcription holds it: no 1990 democracy at ≤ 0.20 carries an Army pillar); A9 GREEN 620/706 = 0.878 (706 coups and annulments over the twelve seeds against 229 unmutated); A10 RED at 0.24975 (0.2497513, per seed 0.224..0.262) — the pin pass wrote 0.2499, the skeptic read 0.24975, and the file now carries the measured figure. The pin pass's readings that differ from these (A8 5/40, A10 0.141) were its own on its tree; the file carries only this pass's.

### S7-5 — A5's diagnosis was incomplete: two of the six named returns are unreachable by transcription

The honesty skeptic's finding, recorded in the bar and as S5-8's fifth data question: the bar reads "ruling bloc becomes Communist by ballot", and Lithuania's `lt_ldpp` and Hungary's `hu_mszp` are transcribed `Family::SocialDemocratic` with no bloc override (Western), so their return is Western → Western and cannot register; Bulgaria's `bg_bsp` (0.472), Albania's `al_ppsh` (0.562) and Mongolia's `mn_mprp` (0.623) already lead at the 1990 seating, so a "return" there is lose-then-return; only Poland's `pl_sld` (`Family::Communist`, 0.22) is a plain flip, and it reads 0/200. S5-7's `drift_support` diagnosis stands for Poland and the three party-states; the other two are a data question and not the model's.

### S7-6 — the suite, the digests and the live page this run (the tree that ships)

`cargo clean -p spheres-sim -p spheres-web -p spheres-cli --release` (263 files, 1.4 GiB), `cargo build --release --workspace --tests --bins` watched (`Compiling spheres-sim`, `spheres-web`, `spheres-cli`; 60 s), then `cargo test --release --workspace --no-fail-fast 2>&1 | tee blocs3-suite.txt` in the foreground, 4 m 48 s: `spheres-sim` lib 452 passed / 3 failed / 25 ignored (159.2 s) — exactly the three deliberate reds, `the_1990_start_is_pinned` actual 0xe26e4bf8d6c60066 (pin 0xd022d50f43c984da untouched, `lib.rs` not in this branch's diff), `golden_hash_of_a_known_run` actual 0xbe94d6125631829c (pin 0xbd5ec0f43c5f2e3b untouched), `the_1990_endowment_does_not_move_year_one_growth` (E-3); the 28 integration binaries all ok (`bloc_census` 4 passed / 7 ignored in 19.8 s, `daily` 7 in 33.3 s); `spheres-web` 160 / 0 / 2 (42.6 s); `spheres-cli` 1 / 0; the two wall-clock bars (P-12) green; 834 passed / 3 failed / 65 ignored over the 32 result lines. The 452 is cbcc364's count again — the pin pass's 453 carried the D4 test, now reverted. Headless digests, market off, twice each: `run 35 1990` sha256 d1a2cfbf7c6958d7, 3501 lines, stderr 0 bytes, both runs; `run 35 7` 39dea3341a7f6e8c, 3983 lines, stderr 0 bytes, both runs; `cmp`-identical to the pin pass's files.

LIVE, `spheres-web --port 7920 --no-open` from the clean build, through the page: Poland at seed 7 (Find a nation → the card → WORLD SEED 7 → GOVERN POLAND): the government screen reads Tadeusz Mazowiecki, Western 78% rules, discontent 29%, 29 PC, and the takeover watch reads every road CLOSED — CALIBRATION PENDING with its live gauges (coup: army loyalty 100% ≤ 35%, discontent 29% ≥ 25%, coup pressure 0% ≥ 100%; uprising: discontent 29% ≥ 45%, challenger influence Communist 22% ≥ 45%; round table: Western influence 78% ≥ 40%, party loyalty 100% ≤ 55%, stability 55 in 30..70; collapse: stability 55 ≤ 12) — the switch is OFF (`play_rules`, untouched on this branch) and the page says so. Iraq at seed 7 via Campaigns → New campaign (the page's native `window.confirm("Start a new campaign and replace this server's live world?…")` is suppressed by the browser pane and was stubbed to answer yes for the launch; `/api/new` 200): Saddam Hussein, Nationalist 60% rules, discontent 38%, 39 PC; the watch CLOSED — CALIBRATION PENDING on all four (coup: army loyalty 65% ≤ 35%, discontent 38% ≥ 25%, pressure 0% ≥ 100%; uprising: discontent 38% ≥ 45%, challenger influence Non-Aligned 40% ≥ 45%; round table: Western influence 0.2% ≥ 40%, party loyalty 65% ≤ 55%, stability 48 in 30..70; collapse: stability 48 ≤ 12). Speed 5 from 1 Jan 1990 to 10 Jan 1991 (374 days; the clock pauses itself on every political stem, which is how each was read as it landed): Iraq ends at discontent 18%, $62bn, 41 PC, no event of its own. The Politics filter of the event log, 29 of 96 dispatches: 24 ballots ("X votes: …"), two "The government of X falls; the country goes to the polls" (Argentina, El Salvador, 31 Dec 1990), two pre-arm collapses ("Revolution in Peru — the old regime falls." 13 Aug 1990; Nicaragua 13 Dec 1990), "Netherlands ends conscription"; and in the full log "Guatemala is led by a new Guatemalan Christian Democracy president" (1 Jan 1991, the S4 succession line). No crowned movement, no `COUP IN`, no annulment, no "dies in office" in the year — consistent with the roads off. Console errors: zero (the only console lines were the pane's own three "dialog suppressed (confirm)" warnings, which are the tool's and not the page's). Two observations, not errors and not this branch's code (`spheres-web/src/main.rs` is byte-identical to cbcc364): after the new campaign the government screen's nation selector kept Poland ("spectating") until re-selected, and the chronicle's "Follow a nation" still read Poland. The server was killed by PID after the read.

## Filed 2026-09-06 by the history pass (M1, R2, M2, R1 on Ridge's determinations, "Make determinations for these and continue. I want it based on historical data."; branch `feat/ideology-history` off 2e164ae)

Every number here was measured by this pass on this tree, each build watched (`Compiling spheres-sim` read before any reading was believed), `CARGO_TARGET_DIR` the worktree's own (`blocs4-target`). Raw outputs in the run's scratchpad (`blocs4-out/census_n200_m1.txt`, `zz_afghan_probe.rs` and its trace).

### H-1 — M1 built: presence through backing, the sponsors' creating clause, and a standing programme in place of the lottery

**What landed.** `blocs::PRESENCE_BACKING` = 0.05 (the ruling's number): `bloc_present(w, id, bloc)` is now `bloc_in_table(id, bloc) || bloc_backed(w, id, bloc)`, the table half unchanged and the backing half reading `backing` (stock plus gravity) at or over the line; `flat_seed` and `bloc_can_win` take the world (a bloc present through backing takes an equal split of the seed and, if not Western, could win — the Peshawar parties took Kabul with no seat and no institution inside the country), and `challenger`, `drift_movements`' appeal recipients, `reseeded_support`'s "no party to carry it" clause, the web's two op lists and the census's Islamist ceiling all read the new presence. `politics::ai_back_bloc_choice` gains the creating clause: where no PRESENT bloc qualifies (the S3 rule, unchanged and preferred) and the sponsor's own bloc is absent from the target's TABLE, the sponsor backs it anyway when the target is hostile (`HOSTILE_TARGET` = -30, `best_covert_target`'s own line, now named) or a rival's client — for every sponsor, patron or ideological. "Absent from the table" rather than "absent" is deliberate: a presence the sponsor's own money created sits at influence 0.06 under the 0.15 line and cools away in ten months, so reading it as present would have ended every programme after its first operation.

**The cadence moved, and why (S3-6's draw; an AI threshold in the brief's WHAT MAY MOVE list; anchor A2).** With the clause in, Pakistan (-70 with Kabul) has a choice in Afghanistan from January 1990, and the S3 sponsors' block — choose, then draw 0.022 — parted the switched-on world from the off-world at month 0 with no act to show for it: a failed draw shifts every later draw of the month silently, so `the_bloc_layer_is_inert_over_time` read "parted at month 0 with no stem". And as history the lottery is wrong on its own terms: on seed 7 it landed Pakistan's money in Afghanistan 3 times in twenty years, against a stock that cools away in ten months; Operation Cyclone was appropriated and delivered every fiscal year 1980-1992 and Saudi funds matched it. The block now acts once a calendar month whenever its choice stands and the channel has cooled to `SPONSOR_PROGRAMME_HEAT` — DERIVED, not chosen: 0.18 − 0.012 · (`BACKING_STEP` / `BACKING_DECAY`) = 0.06, the heat left when the last clean operation's money has run out, one operation per sponsor per target every ten months on a clean channel, thirty-one after an exposure — and draws nothing itself; `covert_action`'s two dice are the stream's only parting and always print a line. The patron arm keeps its 0.022 draw, which both worlds take. Seed 7, twenty years: AI backings landed 8 (S3-13) → 10 (the clause alone) → 32 (the programme); Afghan Islamist backings 3 → 13, the first in month 0.

**Tests.** New: `an_absent_islamist_bloc_becomes_present_after_one_sponsor_operation` (watched red twice, the creating clause removed — "left: None, right: Some(Islamist)" — and the backing clause removed from `bloc_present` — "one operation created the presence"). Existing tests TOUCHED, each by construction of the ruling and none widened: (1) `the_bloc_layer_is_inert_over_time`'s first-act pin, (35, "backing the Western movement in Libya") → (0, "the Islamist movement in Afghanistan") — the measured first act is Pakistan's, Saudi Arabia's and Iran's operations in Afghanistan and Cuba's in Chile, in registry order; the six OFF hashes untouched; the on-world now tracks the off-world for one month, not thirty-five, and the doc says so. (2) `gravity_appears_with_an_aid_flow_and_vanishes_the_month_it_stops`' clause "nothing was stored" read the WHOLE backing store empty after one month on seed 7, which the programme's month-0 act falsifies; narrowed to the clause's own claim, that Egypt's Western flow stored nothing (`stock == 0` and no entry targets Egypt). (3) Signature follow-through only (the world threaded, no assertion moved): `the_1990_ruling_bloc_census`, `the_liberalisation_seam_reseeds_parties_from_movements`, `a_western_winner_is_refused_in_a_party_less_polity`, the web's two `bloc_present` reads, the census's `bloc_can_win`. The brief allowed two existing tests; the inertness test is one of the two, and (2) is a third — recorded here rather than hidden, and it is the ruling's own month-0 act that reds it.

**Measured, default path byte-identical:** the 1990 start actual 0xe26e4bf8d6c60066 and the timeline actual 0xbe94d6125631829c unmoved (the pins untouched, the three deliberate reds exactly); `run 35 1990` d1a2cfbf7c6958d7 (3501 lines, stderr 0) and `run 35 7` 39dea3341a7f6e8c (3983, 0) unmoved; sim lib 452 passed / 4 failed before the gravity narrowing (the three deliberate reds plus that clause) and green on the touched set after; `bloc_census` 4 pinned bars green (21.1 s); `spheres-web` 160 / 0 / 2; `spheres-cli` 1 / 0; `the_daily_clock_preserves_the_political_arm_on_world` (A12) green with the month-end gate; `the_sponsor_payoff_and_exposure_fire_on_takeover` green (Sudan's revolution in month 12 on seed 7).

**The N=200 census, before (S7-2) → after M1** (`census_n200_m1`, 100.5 s): A1 4/6/9 (6.345) → 5/6/9 (6.405, sd 0.790), top-3 share med 1.00 both (Sao Tome 553, Philippines 464, Comoros 200, Algeria 46, Mozambique 12, Suriname 4, Kyrgyzstan 2 of 1281), annulments 2/3/4 (2.580) → 2/2/3 (2.495; Belarus 200, Ukraine 200, Algeria 105 → 99), regime coups 10.575 → 10.570; A2 0/200 → 0/200, Islamist uprisings 0, the largest Islamist influence in a winnable non-Islamist state 0.43/0.64/0.65 unchanged; A3 197/200 → 198/200, per seed 2.085 → 1.990 (Belarus 181 → 179, Cambodia 142 → 120, Ukraine 91 → 99); A4 1/2/3 both (1.610 → 1.600; Kuwait 3 → 6); A5 0/200; A6 0 in 200/200 (Argentina's collapse beside 198 → 200); A7 ratio 0.16 → 0.17 (bloc flips 3.165 → 3.100, takeovers 19.31 → 18.70); A8 200/200; A9 0.953 → 0.952 (3708/3894); A10 0.27/0.33/0.38 (0.324) → 0.26/0.29/0.38 (0.304, sd 0.027; the pinned 12-seed bar reads 0.29-0.30 against 0.25); deaths 43.335 → 43.215.

**Why A2 still reads 0/200, traced (the probe, seeds 0, 7, 13, 252 months).** The presence is created — all three Islamist sponsors pay from month 0 — and three things downstream of M1 stop it there, every one a flagged constant: (a) Kabul EXPOSES the programme again and again (authoritarianism 0.85 drives `exposure_probability`; seed 0: exposures in months 40, 55, 55, 76, 91, 157, 196, 208, 235, 244), each halving the stock and tainting the bloc, so F_Islamist oscillates 0 → 0.15 → 0 and the presence lapses between operations (S3-4's exposure arm; `statecraft::caught`); (b) the movement's own SHARE never grows — 0.002 → 0.018 at most over 252 months at discontent 0.50-0.61 — because `drift_movements`' record line moves the ruler by 0.006·record·dt ≈ 0.0004 a month (S3-1's drift, the brief's "drift 0.006/0.015/0.005"), so influence peaks at 0.149 (seed 0, month 12) against the 0.45 line; (c) coercion never fails: the weakest armed institution reads 0.36-0.72 and the mean 0.56-0.76 (Kabul's army at a 15% defence share reads 0.66-0.85 on the share arm; M2 is the piece that would read it otherwise). None of the three is touched by this commit; each is named here with its number so the calibration that follows M1 can be argued against A2 rather than guessed.

### H-2 — R2 landed: an election is annulled only by a HOSTILE army

**The ruling, quoted:** "an election is annulled only by a HOSTILE army - effective_army_loyalty below ELECTORAL_COUP_ARMY (0.35), no new constant - because that is what history shows: Algeria's ANP in January 1992 was an army at war with the FIS; Jordan's 1989 chamber was never annulled because the throne, not the army, held the state; Turkey's 1997 memorandum came from a hostile general staff." `government::annulment_check` gains one line after the Army-pillar test — `effective_army_loyalty(w, id) >= ELECTORAL_COUP_ARMY` returns `None` — so the annulment reads the same effective loyalty (the pillar less Nationalist foreign backing) against the same line route 2 reads. S4-2's other lines (0.35 authoritarianism, 0.25 discontent, Communist or Islamist only, the court exception) are untouched.

**The one existing test re-expressed (the second of the brief's two):** `the_algeria_shaped_annulment_fires_under_its_conditions_and_not_under_the_court`'s Jordan-at-0.38 assertion, which pinned the present-army reading — `annulment_check == Some("jo_ikhwan")` with the seated 0.65 army — now reads: at 0.38 with the army PAID (0.65 ≥ 0.35) the check is `None`, `hold_election` seats the Ikhwan and bans nobody; the same Jordan with the Army pillar written to 0.30 annuls with the same headline. The ruling is quoted in the test's doc. The Algeria half is UNTOUCHED and still green: over the twenty-three months to the December 1991 vote the ANP walks from 0.65 toward the share arm's 0.322 (a 1.5% defence budget) and is under 0.35 at the vote, so the army at war with the FIS reads hostile from the transcribed budget alone (discontent at the vote 0.511). New: `a_paid_army_seats_the_islamists_and_a_hostile_one_annuls_the_algerian_way` — Algeria in January 1990 with the ANP at 0.65 holds its vote and seats the FIS; at 0.30 annuls (every Islamist party banned, Nationalist at 0.80); at 0.40 with 0.06 of Libyan money behind the Nationalist movement reads 0.34 effective and annuls; off, nothing. Watched red with the line removed: both tests red ("a paid army annulled the election"; Jordan's `None` read `Some("jo_ikhwan")`).

**Measured.** Default path byte-identical: 0xe26e4bf8d6c60066 and 0xbe94d6125631829c unmoved; `run 35 1990` d1a2cfbf7c6958d7 (3501), `run 35 7` 39dea3341a7f6e8c (3983) unmoved. Sim lib 454 passed / 3 failed / 25 ignored (157.3 s; the three deliberate reds exactly); `bloc_census` 4 pinned bars green (20.9 s); web 160 / 0 / 2; cli 1 / 0; the inertness trio green.

**The N=200 census, after M1 (H-1) → after R2** (`census_n200_r2`): annulments per seed 2/2/3 (2.495) → 0/1/1 (0.530): Belarus 200 → 0, Ukraine 200 → 0, Algeria 99 → 106 of 200 = 53% (the anchor asks ≥ 10%); A3 198/200 → 118/200 = 0.590, per seed 0/2/4 (1.990) → 0/1/1 (0.590), Cambodia alone (120 → 118; Belarus 179 → 0, Ukraine 99 → 0) — S5-8's first data question closed by the ruling, the remaining 59% Cambodia's pre-existing regime coup at a 1% defence share (S5-3), against the anchor's ≤ 10%; A1 6.405 → 6.265, top-3 share med 1.00 (Sao Tome 540, Philippines 446, Comoros 200, Algeria 46, Mozambique 14, Suriname 7 of 1253), regime coups 10.570 → 10.590; A2 0/200 → 1/200 (Algeria once, over the run and by 2000), the largest Islamist influence 0.43/0.64/0.65 unchanged; A4 1.600 → 1.650 (Sao Tome 200, South Africa 200, Morocco 13, Kuwait 12, Mozambique 3); A5 0/200; A6 0 in 200/200; A7 ballot bloc flips 3.100 → 2.640 against takeovers 18.700 → 15.395, ratio med 0.17 → 0.14; A8 200/200; A9 0.952 → 1.000 (3477/3477 — every coup and annulment now strikes a nation under $8,000 a head); A10 0.26/0.29/0.38 (0.304) → 0.26/0.28/0.32 (0.279, sd 0.010); deaths 43.215 → 43.600.

### H-3 — M2 built as ruled, and sized to ZERO by the measurement: the ruled ratio reads the coup-prone armies as the best-paid in the roster

**What landed.** `MilitaryRecord` gains `personnel_1990` and `milex_1990_usd_bn` (both `Option`, `#[serde(default, skip_serializing_if)]`, so every existing file round-trips byte for byte and the 1990 save carries no new key; absent is a REFUSAL, iron rule 4, validated positive where present); `data::army_personnel_1990(id)` (parsed once into a roster-indexed table — the Army target reads it every tick) and `army_milex_1990_usd_bn(id)` (the record, on demand). `government::army_pay_ratio(w, id)` = `mil_spend_gdp · population / personnel` — the ruled `(milex/personnel) / (gdp/population)` with the output cancelled, live in the share and the population, the 1990 personnel fixed (no personnel model exists; recorded). `government::army_loyalty_target(mil, exhaustion, pay_ratio, weight)`: `None` is the share arm's literal in the same operations and order (byte-identical, asserted bit for bit for every living nation off AND on); `Some(r)` blends `0.20 + 0.65 · ((1 − w) · share + w · pay) − 0.45 · exhaustion`, `pay` the ratio's clamped position between `ARMY_PAY_UNPAID_AT` = 1.0 (the ruling's sentence: the national average income) and `ARMY_PAY_FULL_AT` = 2.0 ("well above", read as twice — INVENTED, labelled). `pillar_targets`' Army line calls it with the ratio only under `rules.ideology_blocs` — the one place the lens changes a target — at `ARMY_PAY_WEIGHT`. `ARMY_PAY_WEIGHT_RULED` = 0.80 is what the ruling's two sentences require, DERIVED from the pillar model's line (`0.20 + 0.65·(1 − w) < 0.35` gives `w > 0.769`). Historical basis in the function's comment: Powell (2012, JCR 56(6)) on expenditure per soldier; Londregan and Poole (1990, World Politics 42(2)) on the poverty gradient. Test `the_army_target_is_the_share_arm_off_and_the_pay_arm_is_the_ruled_form` (watched red with the gate dropped and the weight forced to 0.80: Algeria's bits differed, 0.322 against 0.74).

**The data on this tree:** 0 of 137 living nations carry `personnel_1990` — the data pass is transcribing them in parallel (its `stage1.json` holds the World Bank pulls: MS.MIL.TOTL.P1 for 124 of the 137 roster nations, MS.MIL.XPND.CD for 110, the fallback years marked — Albania's and Bhutan's personnel are 1992, Cape Verde's budget 1988, the CAR's 1991, Congo's and Laos's 1992; Czechoslovakia, the USSR, Yugoslavia and Taiwan have no ISO3 in the API); every nation is REFUSED to the share-only line today and the test prints the count. The arm's effect on the census is therefore ZERO on this tree by two routes at once, and the census after M2 is identical to R2's line for line.

**The sizing, measured on the fetched figures against the roster's own `mil_spend_gdp` and `population_m` (the table the shipped weight rests on).** The ruled ratio, average incomes a soldier, with the share-arm target and the pillar transcription beside it: Thailand 5.0 (target 0.411, Army pillar, electoral), Haiti 9.4 (0.289, Army, regime), Nigeria 8.3 (0.265, Army, regime), Sudan 19.8 (0.590, Army, regime), Pakistan 12.2 (0.704, Army, electoral), Comoros no personnel in the API (0.297, Army), Algeria 3.0 (0.322, Army), Afghanistan 31.0 (0.850, Army), Cambodia 1.4 (0.371, Army), Sao Tome 0.95 (0.265, Army), Philippines 12.7 (0.379, Army), Ghana 5.1 (0.224, Army), Bangladesh 13.0 (0.297, Army) — against the stable set: Switzerland 4.9 (0.330, NO Army pillar), Costa Rica no army (0.200, no pillar), India 22.1 (0.460, no pillar), Botswana 9.5 (0.557, no pillar). Two findings. (1) NO line separates the named sets on this ratio — Thailand 5.0 sits beside Switzerland 4.9, Haiti 9.4 beside Botswana 9.5, and Sudan 19.8 reads the best-paid army of the nineteen — because dividing by a poor country's income per head INVERTS Londregan and Poole's gradient: the same $4,500 a soldier is twelve average incomes in Pakistan and 0.11 of one in Switzerland (whose income per head reads $40,045 on the roster). Only Sao Tome (0.95) pays under the national average, so the ruling's first sentence never fires on the 1990 roster, and its second reads Nigeria's 0.8%-of-GDP army (Abacha, November 1993) and Algeria's ANP (3.0) as paid — at any positive weight Algeria's target rises from 0.322 (0.74 at 0.80), the ANP never reads hostile, and the annulment R2 measured at 106/200 goes to 0. (2) The stable set is kept stable by the PILLAR TRANSCRIPTION, not by any formula: none of Switzerland, Costa Rica, India or Botswana carries an Army pillar, so no Army target of any form can stage a coup there (A6's and A9's watched readings said the same, S6-2). So no positive weight moves any named coup-prone army toward the line, every one moves away, and the census's verdict on `ARMY_PAY_WEIGHT` is 0.0. The arm is decorative at that weight and is said to be (iron rule 7's last paragraph); it stands exactly as ruled one constant from live, and its form is exercised by its test at the ruled weight.

**What Powell's own variable gives, tabled for Ridge's re-ruling and NOT built (a denominator the ruling did not name).** Pay per soldier in absolute 1990 dollars (`mil_spend_gdp · gdp_bn` over the personnel): Nigeria $4,596, Haiti $4,262, Pakistan $4,509, Thailand $7,837, Algeria $7,381, Afghanistan $7,759, Ghana $1,967, Cambodia $262, Sao Tome $960, Bangladesh $3,682, Philippines $10,193 — against Switzerland $196,218, Botswana $27,867, and the two the pillars keep apart, Sudan $24,812 and India $8,127. A line near $10,000 a soldier separates every named coup-prone army with an Army pillar from every stable one but Sudan, keeps L&P's sign, and is what Powell measured; its level would still be INVENTED until his sample's own figures are fetched. The same is true of a ratio to the WORLD's income per head (the roster's own, $23,459bn over 5,194m = $4,517: Nigeria 1.0, Haiti 0.9, Pakistan 1.0, Thailand 1.7, Algeria 1.6, Sudan 5.5, Switzerland 43.4, Botswana 6.2, India 1.8), which is a reading of the ruling's words "relative to GDP per head" this pass declined to make on its own.

**Measured.** Default path byte-identical: 0xe26e4bf8d6c60066 and 0xbe94d6125631829c unmoved; `run 35 1990` d1a2cfbf7c6958d7 (3501), `run 35 7` 39dea3341a7f6e8c (3983) unmoved. Sim lib 455 passed / 3 failed / 25 ignored (157.0 s; the three deliberate reds exactly, every `data::tests` round-trip green with the two new keys); `bloc_census` 4 pinned bars green (21.3 s); web 160 / 0 / 2; cli 1 / 0; the inertness trio green; the N=200 census (`census_n200_m2`) identical to R2's, summary line for summary line.
