# SPHERES Roadmap

## Done (v0.5 rebuild)
- Deterministic core, save/load, monthly ticks, state hashing
- Economy: growth/catchup/diminishing returns, inflation, budgets/debt, oil market, bubbles+hangover
- War: coalitions, exhaustion, annexation vs subjugation, burned-hand learning, nuclear taboo
- Politics: central bank AI, fiscal AI, elections, USSR dissolution, 1998 proliferation
- Oil embargo: sanctions cut *exports* via `oil_export_share`, hitting the producer's
  revenue and budget, not just a GDP drag; embargoes outlast their war and lift as
  grievance decays (Iraq's runs ~10y, against the historical 1990-2003)
- Negotiated peace: ceasefire and territorial concession, not only white peace
- Yugoslavia + successor states — the breakup and its wars emerge from separatism
- Roster expansion: Brazil, Indonesia, Egypt, Israel, Turkey, Nigeria, Vietnam, and
  Ukraine as a USSR successor alongside Russia. 24 at the start, up to 30 after
  federations come apart
- Browser UI: map, charts, league table, dispatch feed, readable history
- **Technology tree**: 253 technologies across eight domains plus a foundation set.
  Research funded out of output, spent across domains, diffusion weighted by GDP
  and openness, unlocks gated by prerequisites and year floors
- **Productivity rebuilt on the tree**: the tree is scored against the technology
  the world economy on average operates with rather than added on top of a 1990
  trend that already priced it in, so it differentiates nations instead of
  inflating all of them by the same amount
- **Convergence separated from diffusion**: two different things that were once
  conflated. Income catch-up — capital deepening and reallocation — stays in
  `economy.rs` where it always was. Technological adoption is paid on technology
  a nation actually puts into service, not on how far behind it is, so a country
  that learns nothing is no longer paid as though it were catching up. The cost
  floor now falls away as a technology approaches universal, which is what lets a
  small poor economy pick up ordinary things it could previously never afford
- **Nations are data**: all 24 start nations and the seed relations matrix live in
  `spheres-sim/data/` as JSON, loaded through two-pass validation with
  `deny_unknown_fields`, so a misspelled key is a refusal rather than a silent
  zero. Each file carries its own `sources` block, and the browser nation sheet
  serves it under "Where the 1990 figures come from" — provenance a player can
  read, not a comment in a file nobody opens
- **Political capital** exists as a currency (see below)
- **Statecraft — the namesake system**: mutual defence pacts with an upkeep both
  signatories pay, patronage as a standing share of the patron's output, trade
  dependency that accumulates and then becomes leverage, covert action that is
  deniable until it is caught. Every one of its seven commands is priced in
  political capital, and the acts that amount to breaking your word — renouncing
  a guarantee, cutting a client loose, tearing up a treaty — are charged to
  bankruptcy rather than refused, because a government can always renege

## Closed: nation identity is a runtime value, and dyads are derived

**Phase 1.1.** `NationId` was a closed 30-variant enum with hand-written
`name()`/`parse()` arms, fixed-size roster arrays, and — the part that could
not survive the roster growing — a `match (attacker, target)` in `ai_wars`
holding fourteen hand-set war appetites. At 190 nations that is ~380 match arms
and ~36,000 ordered dyads, and the second of those cannot be a match statement
at any size.

- **Identity is now an interned index.** `nations.rs` holds the roster as data
  (code, display name, aliases, region, land borders, claims, and the two
  flags that used to be the `PATRONS` and `MAJORS` arrays); `NationId` is a
  `u16` handle into it. Adding a country is adding a row. The static table can
  become a JSON file without anything above that module noticing, because
  nothing above it knows how many nations there are.
- **Saves carry codes, never indices.** `NationId` serializes as its stable
  code and the relations matrix — dense in memory, keyed by index — writes
  named triples and *drops* a code this build cannot resolve rather than
  reinterpreting it as whoever now holds that slot. Exactly the discipline the
  technology tree already follows, for exactly the reason it had to.
- **War appetite is derived.** `dyads.rs` computes it from reach (border or
  region), claim (the *share of the target* a state says is its own), grudge,
  the aggressor's own authoritarianism and militarisation, digestibility and
  worth — then the same circumstance terms as before. Iraq still wants Kuwait
  an order of magnitude more than Saudi Arabia, and Belgrade still wants Bosnia
  and not Slovenia, but now because the 1991 census put 31% Serbs in Bosnia and
  2% in Slovenia and because Serbia and Slovenia share no border.
- **The missing half of `burned_`.** A repelled invasion was remembered; a
  *successful* one was not, so a state that took what it claimed came back for
  it every two years. Measured across twelve thirty-year runs on master, 182 of
  300 wars were somebody invading Bosnia again. A claim pressed to a conclusion
  is now a claim collected (`pressed_A_B`): the grievance survives, the war aim
  does not.

**The golden hash moved and was re-pinned** (`0xb675826e8941683d` ->
`0x19c5c5dafb18dbd9`). The identity half is separately proven not to move it:
with the runtime id in place and master's literal dyad table restored on top,
the fingerprint and the 2025 league table reproduce master exactly. All of the
movement is the derived model, which is the point of it.

**Known thin spots, in order of how much they should worry you.** The world is
quieter than master — 75 wars across twelve thirty-year runs against 300 — but
most of what went is the repeat-invasion loop above. `china_growth_miracle`
asserts a 14x ceiling that master clears only on the default seed (it runs
14.9x, 16.8x and 15.1x on seeds 0, 42 and 3), so it is a one-seed test standing
on luck and the next timeline change will likely tip it. And
`a_pact_drags_a_great_power_into_a_war_it_did_not_start` now lands on 3/12
against a floor of 3 — passing, but with no margin.

## Closed: the whole world was about twice too large

Measured on the default seed, clean builds, `run 35 1990`:

| | 1990 | sim 2025 | real 2025 (1990 $) | sim CAGR | real CAGR |
|---|---|---|---|---|---|
| USA | $5,980bn | $30,762bn | ~$14,000bn | 4.8% | 2.5% |
| Japan | $3,140bn | $13,742bn | ~$4,300bn | 4.3% | 0.9% |

**Diagnosed, and it was not the technology tree.** An earlier version of this
entry blamed the tree's +0.020 productivity cap on the arithmetic that 2.2
points of excess growth matches a 2.0-point cap. That was a coincidence.

The cause is `statecraft.rs`. A trade agreement multiplied *both* signatories'
GDP by up to 2.4% a year for every month it existed, applied directly and
outside the growth accounting. Each pact therefore raised its signatories'
growth rate permanently rather than their output level, and they stack: the
United States signs enough of them to compound 4.5% a year while its own
briefing reports 1.8%, because none of it passes through the growth model where
anybody would see it. That gap between reported and realised growth is the
tell, and it is worth remembering as a smell — if a nation's GDP series and its
growth series disagree, something is writing to GDP directly.

**Fixed and merged.** The gain is paid only as integration deepens, so a mature
pact is worth a permanently larger economy rather than a permanently faster one.
The USA at 2025 goes from $30.8tn to $16.5tn against a real ~$14tn.

The embargo test it re-broke was diagnosed rather than widened: the coalition
erodes correctly — all five majors in 1995, only the United States by 2000 —
but America's own covert action drives its relation with Iraq from -54 to -81,
renewing the grievance the relief rule reads. It lifts in 2025, at 35 years.
That is inside the real range (Iraq 13 years, Cuba past 60); the 25-year
assertion had encoded an assumption that stopped holding once covert action
existed. Replaced with a stronger test that locks the erosion pattern too.

**Guarded now.** `the_frontier_does_not_run_away` asserts that every mature 1990
economy compounds under 4%/yr across 35 years, taken as a **median over ten
seeds** rather than a single draw, plus a pooled median over all sixty
(nation, seed) readings under 3.5%/yr. Against the pre-fix behaviour the USA
compounded 4.79%, so it goes red on exactly the bug that prompted it; measured
against a uniform growth injection it goes red at +0.58 points a year on the
pooled line and at -2.2 points on the floor.

Japan remains roughly twice its real size at ~3.0%/yr, so a second, smaller
cause is still outstanding there — see the Japan entry below.

Nothing guards this. `china_growth_miracle` asserts a floor (>6x in 30 years)
and every other calibration test is about a *relative* outcome, so a world that
uniformly doubles passes all of them. A test that locks the frontier economy's
35-year trajectory would have caught it years of game-time earlier.

## Closed: output ignores labour

Growth was `TFP + investment effect + catch-up - drags`, with population reaching
output only through GDP per head — a proxy already capped at 1.0 for every rich
nation, so population growth had no effect whatever on a developed economy. The
demographic transition made Japan's population fall correctly and changed its GDP
by exactly zero.

Closed by the labour term SPEC section 3 asks for: output growth now carries
0.6 times workforce growth, so a shrinking workforce is a headwind investment
cannot offset. It landed with the other half the entry demanded — returns to
investment made concave around a 20% reference, so a nation can no longer buy
growth indefinitely by raising its investment share.

Still untested, and worth knowing that. No assertion locks a frontier economy's
long-run trajectory, so a world that uniformly doubles still passes every
calibration test we have.

## Closed: the roster is 108 nations, and it cost four calibration tests (all four since recovered — see §0)

Ten regional branches landed on master one at a time, each merged and committed
separately so a bad one could be reverted alone: western Europe (11), eastern
Europe (5), the post-Soviet ten, Latin America (10), the Middle East (8), north
Africa (5), sub-Saharan Africa (11), south Asia (5), east and southeast Asia
(9), and the anglosphere three. 31 nations became 108. Every conflict was an
append-at-the-end collision in the same four places — the `ROSTER` table, the
`embedded.rs` manifest, the `POLITIES` table and the web UI's `TERRITORY` map —
plus neighbour lists that two authors had both extended, which are unioned
rather than chosen between. `the_roster_is_internally_consistent` is the guard
that a union was missed, and it is green.

World 1990 GDP is now $23.3tn against $18.8tn at 31 nations, and the real
figure is about $22.8tn. It was believed at the time of the integration that
this was the single most consequential number in it, because `tech::tick` sizes
what a nation can afford as `sqrt(gdp / world_gdp)` and so every coefficient
fitted before it was fitted against a world roughly 18% too small.

**That turned out to be wrong, and the measurement is in §0 below.** The
denominator was swept 3.2x and did not move the calibration; what the roster
changed was how often a nation gets into a war and is sanctioned for it. The
$23.3tn figure is still the right one — it is a fuller world, and it is within
2% of the real 1990 total, which also means it has very nearly converged and
the remaining ~80 nations will barely move it.

Cost and headroom, measured:
- `a_century_holds_together` passes at 108 nations. A headless century is 2.9s
  where 31 nations was 0.40s on the same machine — 7.4x wall clock for 3.5x
  nations, so the relations matrix survived the scale-up without going
  quadratic, but the margin is no longer generous.
- `hungary.json` gdp_bn 33.1 -> 34.5. It cited World Bank NY.GDP.MKTP.CD series
  HUN 1990, which returns $34.478bn; the figure moved, not the citation, and
  the reasoning is in the file and its commit.
- Four calibration tests were red and NONE of them was widened. Three are now
  green again after the sanctions refit; see §0.

## Next (rough priority)

### 0. The suite is green, and the last two failures were the tests' shape

**Closed.** Four calibration tests went red when the roster went 31 -> 108.
The sanctions refit below cleared three by fixing the model. The last one, and
one of the three, were then re-expressed as **cross-seed statistics**, because
measurement said the defect was in the tests and not in the sim.

Both read one seed and asserted an absolute, and both were knife edges master
already fell off on other seeds:

- **`the_frontier_does_not_run_away`** read six mature economies on seed 1990
  once. The UK came in at 4.37%/yr against a 4.0% ceiling — a single draw from
  a distribution 1.7 points wide (France runs 2.17..3.84 across seeds 0..9).
  It now takes the **median of ten worlds** per economy against the same 4.0%
  ceiling and the same 0.5% floor, both numbers unchanged, plus a new pooled
  median over all sixty readings under 3.5%/yr. The pooled line is the binding
  one and it is a guard the single-seed form never had: a uniform level shift
  trips it at +0.58 points a year where the loosest per-nation median needs
  +0.74. Shipped world reads pooled 2.98%/yr, 15% of margin.
- **`arms_transfers_build_a_client_army`** read seed 6 once and asserted a
  treated/untreated ratio over 1.50. It read 1.4186 — the **joint lowest of
  ten seeds**, in a distribution whose median is 1.688. It now asserts a
  per-seed floor of 1.20x in every world and a median band of 1.40x-2.50x.

Neither was widened. The arms floor is numerically lower than the old 1.50 bar
and the reason is that 1.50 sat *inside* the distribution — four of ten seeds
fall below it — so it bounded nothing; it was a coin flip that had been landing
heads. Both re-expressions were checked red in both directions against the
behaviour they exist to catch, with the tables in the test comments:

| test | lever | red below | red above |
|---|---|---|---|
| frontier | uniform constant added to `growth_annual` | -0.025 (floor) | +0.010 (both ceilings) |
| arms, floor | `ARMS_ALPHA` scaling the transfer in `aid_flows` | 0.25 (both floors) | — |
| arms, ceiling | `ARMS_DAMP`, transfers *added* not converged | — | 0.0 (ceiling) |

**Two things worth carrying forward.** The arms test needed two different
levers because scaling arms-aid effectiveness globally arms the **control** as
well — the AI runs about twenty arms flows of its own — so at alpha 8.0 an
untreated Kuwait goes 6.55 -> 15.95 while the treated goes 11.45 -> 28.07 and
the ratio barely moves. Any ratio between two clients of the same world is
blind to a change that world applies to both of them, and that is a general
caution about treated/untreated tests here, not a quirk of this one. And
normalising the arms dose to the client's own budget was tried and **rejected
on measurement**: the pledge is made at t=0 where every seed holds the identical
transcribed 1990 world, so the dose is already x19.93 on all ten seeds, and
re-normalising moved the cross-seed spread the wrong way (0.42 -> 0.49/0.51/0.52
at x8/x10/x12). The spread lives in the eight years that follow the pledge and
only a cross-seed statistic reaches it.

Two doctest failures inherited from the sanctions refit were also fixed:
`SANCTION_BITE`'s two indented tables in economy.rs were being parsed as Rust
doctests. Fenced as ```` ```text ````. Doc comments only, no behaviour, hash
untouched. `cargo test --release --workspace` is now fully green: 82 sim, 13
web, 0 doctest failures.

### 0b. How the three were cleared, and why not by widening anything

The refit that entry demanded has landed, and it was **not** the change the
entry predicted. Nothing was widened, nothing was deleted, one test was added,
and the golden hash was re-pinned a fourth time
(`0xc274968416c655b7` -> `0xef3e968249846a49`). Run
`cargo test --release -p spheres-sim roster_scale_readout -- --ignored --nocapture`
for the instrument, and `china_trouble_readout` for the one that settled it.

**The catchup coefficient was never wrong.** The suspicion was that
`sqrt(gdp / world_gdp)` in `tech::tick` had invalidated everything fitted under
it. Two measurements killed that:

- The affordability denominator was swept 3.2x, spanning the 31-nation world
  and beyond the 108-nation one. China's median went 13.34, 11.23, 11.89,
  10.13, 11.64, 10.11 — non-monotone noise, not a response. The per-seed
  figures reshuffle wholesale, because the perturbation changes *which wars
  happen*, not how fast anyone grows.
- Set `ai_aggression = 0.0` and let China simply grow for thirty years, and at
  108 nations it finishes at a median **14.02x against the real 14.33x**. The
  growth model is right to within 2%. Raising catchup to lift the ten-seed
  median would have pushed a peaceful China past 19x — fitting a constant to a
  test while the constant was already correct.

**What the roster actually moved was sanctions.** At 108 nations China has
fourteen land neighbours instead of two and fights in 6 of 10 seeds instead of
4. `sanction_drag` then charged the coalition that forms — always the same five,
USA/UK/France/Germany/Japan, ~52% of world output — a flat **3.0 points of
annual growth for fifteen years and more**, because it counted flags rather than
weighing output. That is the bimodality `nations.rs` documents at its East Asia
block: China either stays at peace and finishes at 13-18x, or fights and
finishes at 6-10x, and the median is decided by which side five or six of ten
seeds fell on.

The rule now reads the sanctioners' share of world GDP, which is what
`oil_blockade` next door has always done. It is **roster-proof by construction**:
a G5 regime weighs what a G5 regime weighs at 108 nations or at 190, whereas the
old rule's total bill rose without limit as the roster grew, and priced
Luxembourg joining an embargo at 30% of the world economy. Anchored on the two
clean non-oil regimes of the period — the US alone against China in 2018-19
(~24% of world output, ~0.6pt) and the near-universal embargo of South Africa
in 1985-93 (~80%, ~2.5pt). Russia 2014 and Iran 2012 were deliberately *not*
used: both targets are petro-states whose loss ran mostly through oil, which
this model already prices separately, so calibrating on them counts the barrel
twice.

Three of the four reds cleared, and the movement is distributional rather than a
lucky reshuffle:

- **`china_growth_miracle`** 10.13x -> **11.16x**. Ten seeds span 8.68..17.25
  against 6.64..18.39; at zero sanction drag they span only 10.01..15.52, so the
  spread really is this one coefficient. Green by 0.16 against an 11.0 floor —
  **still fragile, and the test says so in its own comment.**
- **`the_frontier_does_not_run_away`** UK 4.37%/yr -> **2.91%/yr** on the
  default seed. Across seeds 0..9 the UK now reads [2.80..3.50] with **zero**
  seeds at or over the 4.0 ceiling, against [2.64..4.69] with two. Every mature
  economy tightened.
- **`a_poor_nation_still_picks_up_what_everyone_has`** Afghanistan 4 -> **10** on
  seed 42. The poorest nation across twelve seeds is now 6..11 against 4..10.
  Improved, and still the thinnest margin in the suite: two seeds sit at 6
  against a floor of 5.

**Was still red after the refit, and has since been fixed in §0 above:**

- **`arms_transfers_build_a_client_army`** — 10.9 vs 7.7, the identical two
  figures it failed on before the refit. A single-seed treated/untreated ratio
  sitting at 1.4993 against a bar of 1.50. Not an economic-calibration failure;
  its shape problem is the one this entry always described — any treated/
  untreated ratio drifts as the world fills up — and the fix was the test's
  shape, a cross-seed statistic, not the bar. Note also that
  `the_frontier_does_not_run_away` was cleared here by the model moving, but its
  shape was the same defect and it was re-expressed alongside the arms test
  rather than left as one more knife edge waiting for the next roster change.

**A test was added, because the audit found a hole.** Running the whole suite at
sanction bite 0.000 — sanctions costing a target no growth whatsoever, in a game
whose namesake system is spheres of influence — left everything green except the
hashes. Nothing constrained the coefficient from below.
`sanctions_cost_the_target_real_growth` is that missing guard, two-sided, on a
non-oil target so `oil_blockade` cannot mask it.

### 0b. Follow-ups this refit deliberately did not do

Kept out so the golden hash movement is attributable to one line. All three are
cheap and all three are the same defect:

- **Three sanction channels still count flags.** `research_output` and
  `absorptive_capacity` in `tech/mod.rs`, and the stability term in
  `economy.rs`, all read `sanctioned_by_count`. They should read
  `sanction_weight`. Together they still cost a G5 target 0.46pt/yr on their
  own, which is why the growth drag was set at the low end of its anchor
  bracket.
- **Sanctions regimes last too long.** China's run 16-21 years — longer than
  Iraq gets for annexing a country, against a grievance-decay rule calibrated to
  give Iraq ~10. That is the other half of why China's median is still below
  reality, and it is a `politics.rs` question.
- **China's war incidence.** Six of ten seeds is high for a state whose real
  1990-2020 record is one border skirmish. `nations.rs` already flags the cause:
  `dyads.rs` has no sealift or power-projection term, so reach is a border or a
  shared region and nothing else.


### 1. Blocked on a decision, not on work
- **`feat/financial-system`** — currencies, FX regimes, contagion. `WorldState.finance`
  covers only the original 16 nations and `fin()` panics on the rest, so seven new
  nations and Ukraine need transcribed 1990 balance sheets (stance, reserves,
  FX debt, openness, hot money, risk) and Ukraine needs one created at birth.
  Partial work exists: currency and region arms for all eight, plus `Region::LatinAmerica`
  and `Region::Africa`.

### 1b. The real-rate demand channel is unbounded, and three nations pay for it
`economy.rs` turns `neutral - (interest_rate - inflation)` into annual growth at a
coefficient of 0.55, linearly and without limit. Nothing else in the model has
that shape. Two consequences, both live:

- **The 1990 hyperinflators cannot be transcribed.** Brazil's real prints — 2948%
  CPI, a 9394% deposit rate — put GDP at +inf inside the first simulated year and
  fail twelve tests. Brazil, Poland and Yugoslavia therefore carry figures one
  decimal place low. The files now say so, in full, with the measured cost; the
  loader refuses anything outside the band the model can hold.
- **All three of them boom instead of collapsing.** Their entered policy rates sit
  *below* their entered inflation, so the loosest real rates in the roster belong
  to the three economies that were contracting hardest. Brazil grows 13.9% through
  1990 against an actual 4.3% contraction, Poland 7% against 11.6% the other way,
  Yugoslavia 6% against roughly 7%.

The fix is to bound the channel. It is not a one-liner: clamping the gap at
+/-0.20 was tried and costs `china_growth_miracle` (China runs to 17.1x in 30
years) and `a_trade_agreement_lifts_the_smaller_partner_and_then_binds_it`, which
means China's miracle is currently being paid for partly by an unbounded rate
term. Doing this properly is a recalibration of the demand side, and it should be
done before any more 1990 monetary data is transcribed.

### 2. The tree is invisible
253 technologies and the browser UI does not mention one of them. The owner's
stated preference is to see the game; this is the largest gap between what the sim
knows and what the screen shows.

### 3. Political capital — half done
The currency exists. Every nation holds a stock, seated from the order it keeps
and the prices it holds, earned by delivering growth and lost to recession and
war exhaustion; every player command is priced against it, and one that cannot be
afforded is refused rather than quietly applied. It is visible in the CLI
briefing, the headless report and the browser panel.

What is missing is the other half of SPEC's sentence: *every system* is meant to
be a buyer. Statecraft is — all seven of its commands are priced — but the AI
still is not, because `politics.rs` and `war.rs` move state directly instead of
going through the command queue, so it neither earns nor spends. Routing the AI's own decisions — the fiscal and
monetary rules, and above all the decision to go to war — through the same
pricing is what would make it bite for everyone. That is a real change to how
those systems are written, and it will move emergent history, so it wants its own
branch and its own calibration pass.

### 4. Harvested from `feat/tech-eras`, deferred (tag `archive/tech-eras`)
The scalar model was retired in favour of the tree, but two of its ideas were not
ported and are worth their own branches:
- **Live era rotation.** `Era` is a static calibration bracket. It should be a
  paradigm the world passes through, opening on a frontier threshold rather than a
  date, with a vigour curve so a fresh paradigm is fertile and an exhausted one is
  not.
- **Human capital.** `absorptive_capacity` reads development, openness, the command
  penalty and sanctions — but not schooling.

### 5. Known modelling gap: the tail of the roster
Growth for the smallest and poorest states is the least trustworthy part of the
model. The tree now lets them absorb ordinary technology instead of nothing at
all, but a nation the size of Bosnia is carried almost entirely by the income
catch-up term and the investment term, with the tree contributing little either
way. Sanctioned and post-war states swing hard between runs — Iraq rebounding at
double digits after an embargo lifts is the clearest case. Nothing here is wrong
enough to name a bug; it is simply where the model is thinnest, and it is worth
knowing that before reading any single small nation's numbers as a result.

### 6. Japan — mostly closed, residual documented
Three changes, each defensible on its own and none of them named after Japan:

- **The advantage of backwardness expires.** A trend rate earned while catching
  up cannot be held once a nation *is* the frontier, so `tfp_base` converges
  toward ~1.1%/yr (the US average for the period) at full development. Japan's
  transcribed 1.8% was a 1980s number it never saw again.
- **Pushing on a string.** Demand no longer responds fully to cheap money when
  there is no rate left to cut and balance sheets are impaired. Japan ran zero
  rates for two decades against a corporate sector deleveraging, and the naive
  rule was reading that same zero as permanent stimulus.
- **Balance-sheet recessions heal in ~20 years, not ~9.** The lost decade is
  properly the lost decades.

Result: Japan 1990-2025 falls from 3.03%/yr to 2.28%/yr, and **China now
overtakes Japan**, which it should and previously never did.

**Residual:** real Japan grew ~0.9%/yr. We are at 2.28%. Germany and France are
similarly high (~3%/yr against a real ~1.5%). The remaining excess is European
and Japanese mature-economy growth generally, not Japan specifically — which
suggests one more systemic cause rather than three national ones. Do not chase
it with country-specific constants.

### 7. Superseded: the old Japan entry
Japan carries the highest transcribed 1990 trend of any nation and nothing ever
takes it away, so it settles near 2.8% and outgrows the United States for the whole
run. The lost decade is modelled as a bubble hangover rather than the permanent
break it was. Wants a demographic or balance-sheet mechanism.

### 7. Two domains have rival implementations
`feat/tech-biotech` and `feat/tech-transport` hold uncommitted second versions —
32 biotech techs against the 29 merged, 27 transport against 26, sharing only about
five ids with what is on master. Someone has to pick, or merge the best of both.

### 8. Later
Hourly-cadence scheduler, WASM, multiplayer spheres.

## Housekeeping

**Every worktree shares one cargo target directory.** `.cargo/config.toml` is
tracked, so a worktree checked out under `.claude/worktrees/` inherits
`target-dir = C:/Users/ridge/.cargo-target/spheres` and builds into the same place
as everything else. Combined with OneDrive resetting mtimes, cargo will happily
serve a test binary built from another branch's source — green tests that never ran
your code. When testing a worktree, set `CARGO_TARGET_DIR` to something else, and
if a result looks impossible, check the binary before you believe it.

OneDrive also holds locks on `.git/worktrees/*` and the worktree directories, so
`git worktree remove` and `git worktree prune` fail with "Permission denied" and
the branches those worktrees hold cannot be deleted. Pausing sync should clear it.
