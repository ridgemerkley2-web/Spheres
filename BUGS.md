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
