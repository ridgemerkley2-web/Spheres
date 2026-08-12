# SPHERES Roadmap

## Done (v0.5 rebuild)
- Deterministic core, save/load, monthly ticks, state hashing
- Economy: growth/catchup/diminishing returns, inflation, budgets/debt, oil market, bubbles+hangover
- War: **the commitment ladder** — conflicts between coalitions, nine rungs each
  side picks for itself, theatres with an access requirement, force packages with
  a deployable fraction, munitions and resolve. Annexation vs subjugation,
  burned-hand learning and the nuclear taboo all survive underneath it
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
- **Political capital** exists as a currency (see below)
- **Statecraft — the namesake system**: mutual defence pacts with an upkeep both
  signatories pay, patronage as a standing share of the patron's output, trade
  dependency that accumulates and then becomes leverage, covert action that is
  deniable until it is caught. Every one of its seven commands is priced in
  political capital, and the acts that amount to breaking your word — renouncing
  a guarantee, cutting a client loose, tearing up a treaty — are charged to
  bankruptcy rather than refused, because a government can always renege

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

**Guarded now.** `the_frontier_does_not_run_away` asserts every mature 1990
economy compounds under 4%/yr across 35 years. Against the pre-fix behaviour the
USA compounded 4.79%, so it goes red on exactly the bug that prompted it.

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

## Landed: the commitment ladder (BIBLE §6)

War was a strength ratio pushing a progress bar. There was no decision in it. It
is now four objects, landed in two commits so that a red test could be attributed
to a cause.

**Commit A** — pure refactor. `War` became `Conflict`: coalitions instead of an
attacker-and-allies pair, a per-belligerent posture vector, control in -1..+1
where progress was in -100..100, and the resolver arithmetically unchanged. All
45 tests green on their existing thresholds, hash re-pinned once as a fingerprint
change.

**Commit B** — the mechanic.

- **Nine rungs**, each side choosing its own monthly, and mismatched rungs are the
  interesting state. The ladder **binds** the four statecraft systems rather than
  duplicating them: `commitment::bind_instruments` issues `Command::Sanction` at
  rung 2, `PledgeAid{Arms}` at 3 and 4, and rung 5 runs a quarterly
  `CovertAction` — all through `apply_command`, so a government without the
  standing to sanction somebody literally cannot climb to rung 2.
- **Force packages replace the `mil_strength` scalar** without deleting it.
  `mil_strength` is redefined as force structure and every existing read site is
  untouched; three multipliers now stand between it and combat power. The number
  doing the work is `capital_intensity` — budget per point of structure,
  normalised so the 1990 USA is 1.0 — and it is derived, never authored. It feeds
  the deployable fraction (USA 0.15, Iraq 0.04), quality (USA 1.37, Iraq 0.67),
  and the rate magazines refill. Nobody typed any of those figures.
- **The gate is a multiplication, not a branch.** `exposure` is the rung's own
  exposure, cut by terrain and urbanisation, scaled by the ratio of the two
  sides' quality. Desert Storm and Afghanistan come out of the same six lines.
- **Ten new player commands**, all priced: `OpenConflict`, `SetCommitment`,
  `SetObjective`, `SetRoE`, `SetCeiling`, `SetRedLine`, `RequestAccess`,
  `PressForAccess`, `GrantAccess`, `RevokeAccess`.
- **Theatres and access.** Eleven transcribed operating areas; no power commits
  above rung 5 into one it is not home to without a consenting host. Access is a
  standing consent granted by a parliament on a roll, revocable mid-campaign.
- **UI**: the wars card is now a conflicts card with a nine-cell ladder strip
  (yours amber, theirs red, ceiling notched, unreachable rungs hatched), and a
  conflict sheet where each rung is a clickable row that either shows its price
  or says in words why you cannot have it. Plus an access panel that works from
  both ends. CLI gains `commit`, `objective`, `roe`, `ceiling`, `redline`,
  `access`.

Verified: the Gulf War now runs nineteen months and ends with Iraq thrown back
and Kuwait alive, through the new arithmetic. `gulf_war_emerges`,
`yugoslavia_comes_apart_in_the_nineties` and
`slovenia_escapes_the_wars_that_consume_bosnia` all still pass.

## Landed: the ladder is climbed, invasions are decided, and there is a way in

Three findings from QA playing the ladder branch, all fixed on
`feat/ladder-fixes`. Measured with `war_census` (`#[ignore]`d, eight seeds x 35
years) before and after.

1. **Conflicts were born at rung 8.** All 82 of them, because the coalition,
   the guarantors and the interveners were welded to the act of *creating* a
   conflict rather than to the act of crossing a border in force. The whole
   nine-rung design behaved as a three-state machine: born at 8, collapse to 5,
   decay to 1. `war::invasion_begins` is now lifted out of `declare_war` and
   hangs off the rung, firing once per conflict when somebody on the aggressor's
   side reaches 8; `politics::ai_wars` opens a quarrel at rung 1 instead;
   `commitment::ai_ladder` has an `ambition` and climbs toward it, paying at
   every step and paced by political capital rather than by a timer. Nothing in
   the world is born above rung 1 any more, and Iraq spends nineteen months
   climbing to Kuwait.
2. **Nothing resolved.** A resolve collapse always slid a belligerent one rung
   down and reset its will to a quarter, so a beaten state never stopped being
   a belligerent and the war just went quiet: one capitulation in 82 conflicts.
   Now a collapse *while the enemy holds your ground at a campaign rung* is a
   capitulation, and an invasion that goes quiet gets a verdict within six
   months — settled if the aggressor holds what it took, repelled if not. 27 of
   the last census's conflicts became invasions and 28 endings were recorded.
3. **No way in.** Playing the USA there was no verb that made you a party to
   anything, so every ladder command answered "not a party to that conflict".
   `Command::JoinConflict` (14 pc, enters at rung 1) and a browser/CLI surface
   for it and for opening a quarrel.

Also: `apply_command` no longer charges for a command the world refuses;
escalating on your own ground is charged at 0.30, because a parliament has to
be talked into an expedition and not into a defence; and defensive objectives
mirror only *shooting*, since a state need not run a deniable service because
somebody is running one at it.

### Open, and deliberately not done in this branch
- **Rung 5 is still the resting place of the world.** 9,747 belligerent-months
  of 26,800 in the last census, against rung 1's 11,359. It is the highest rung
  reachable without a consenting host, so everyone who cannot project parks
  there, and long multi-party wars keep their quiet fronts there for years. It
  is no longer the collapse point it was — the ladder is climbed through it
  rather than falling back to it — but a state standing on deniable forces for
  a decade with nothing happening should get bored, and nothing makes it.
- **Capitulation is reachable but rare, and annexation never happens.** One
  capitulation and no annexations across the last census; wars end at a table
  (19 settlements) or with the aggressor thrown back (5). Whether that is right
  for 1990-2025 is a judgement call — it is nearly right for the period — but
  the conquest path deserves its own look, since `conquer` is now reached almost
  only through a side emptying.
- **The AI's judgement is still thin.** `ambition` reads the objective, the
  force ratio it could field in that theatre, and its own announced ceiling. It
  does not weigh access before committing, never chooses an objective or rules
  of engagement, and does not read the opponent's announced ceiling.
- **Interveners still join for free.** `invasion_begins` pushes majors and
  guarantors onto a side at rung 8 without charging anybody, which is the
  standing PLAN 2.1 "no side doors" violation and predates this work. The rung
  changes are priced; the joining is not — though a player's own joining now is.
- **Deviation from the design's burn table, stated plainly.** The design
  specified `BURN_BY_RUNG[8] = 0.140`. Measured, that empties the United States'
  magazines in eight months of a rung-8 campaign — before the control track can
  resolve — so every conventional war froze at rung 5 with both sides dry and
  neither able to finish. Rung 8 is 0.070 and rung 7 is 0.055 here, with rung 6
  left at 0.090 as the design intends, because standoff strike is all ordnance
  and no ground. Rung 6 remains the hungriest, which was the point of the table.

## Finding: `china_growth_miracle` was a false green, and the war model is not why

`china_growth_miracle` asserted `6.0 < x < 14.0` on the single default seed.
Measured across eight seeds, **master's** China runs 9.7x to 17.1x and breaches
14.0 on four of them. The bound was passing by seed-luck.

The decisive measurement: with `ai_aggression = 0.0`, master and the commitment
ladder produce **byte-identical** results — 14.76x mean, spread 0.9. That is
China's actual resting growth in this model, it is above the old ceiling, and the
war layer cannot touch it.

Reality is about 13x (1990–2020, ~9%/yr), so **the growth model runs China hot by
roughly a seventh.** That is a real, open gap. It wants a demographic or
convergence mechanism — China's population also reaches 2,157m by 2020 against a
real 1,411m, which is very likely the same bug seen from the other end. The test
now measures the war-free resting state across eight seeds within ±0.8, which is
a far stricter guard than the one it replaced, and the overshoot is recorded here
rather than hidden.

## Next (rough priority)

### 1. Blocked on a decision, not on work
- **`feat/financial-system`** — currencies, FX regimes, contagion. `WorldState.finance`
  covers only the original 16 nations and `fin()` panics on the rest, so seven new
  nations and Ukraine need transcribed 1990 balance sheets (stance, reserves,
  FX debt, openness, hot money, risk) and Ukraine needs one created at birth.
  Partial work exists: currency and region arms for all eight, plus `Region::LatinAmerica`
  and `Region::Africa`.

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
