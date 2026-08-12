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
