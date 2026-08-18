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

## Closed: the roster is 108 nations, and it cost four calibration tests (three since recovered — see §0)

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

## Landed on master: the ladder rebased onto 108 nations

The ladder sat on `wip/ladder-merged` through the runtime-id refactor, the roster
expansion from 31 to 108 and the sanctions recalibration. Rebasing it turned up
four conflicts and only one of them was real.

**Theatres now derive from the roster.** `default_theatres` was eleven hand-picked
operating areas holding thirty-one hand-listed nations. At 108 that left
seventy-seven states home to no theatre — and a state home to nothing is
expeditionary in its own capital, defending its own border with its deployable
fraction instead of its whole force structure, which inverts the most consequential
number in §6. `region` was already a column on every roster row, so a theatre is
now a region: eighteen of them, one per region plus the sea lanes, with the Middle
East the one region that splits, because the Gulf littoral and the eastern
Mediterranean are 1,500km and a different set of hosts apart. Access hosts and
terrain stay transcribed — a region cannot state whose airfields you need.

That deleted `replace_home` and both its callers. Yugoslavia's successors are home
to the Balkans because the roster files them as Balkan, and the twelve post-Soviet
republics take their own seats the same way. `every_nation_has_a_home` is now a
guard on a mapping rather than on somebody's diligence.

**Two tests were re-expressed and neither bound moved.**

`magazines_run_dry` was the blocker. §6's three stocks are on deliberately
mismatched time constants, so whichever empties first ends the conflict and hides
the other two — and the government module now gives Iran pillars and a coalition
that strains, so Iran's *resolve* hit bottom first. The conflict ended in month 8,
"Iran sues for peace, ceding territory to Iraq", seven months short of the
magazine emptying. Iraq's ordnance was draining at 0.065/month exactly as designed
and would have been gone near month 15 — inside the 6..30 band. The test now holds
the political stock still (`hold_open`: resolve above `settlement_ripe`'s 0.45,
exhaustion below the white-peace 0.75) and leaves the logistical one alone. Its
second assertion moved for the same reason in a different place: it read the rung
at month 60 behind an `if let Some(c)`, so a settlement skipped it silently.
`magazines_are_not_a_bottomless_tap` was added to pin the same band with no war in
the way at all. Checked red both ways — `BURN_BY_RUNG[8]` at 0.020 gives 65.0
months and at 0.250 gives 4.1, and both tests fail at each.

`a_pact_drags_a_great_power_into_a_war_it_did_not_start` read 1/12 runs against a
floor of 3, and the cause was the ladder's own doing: it put standoff strike and
blockade *underneath* the border crossing that used to be the only trigger, and
left the guarantee call at rung 8. A patron could watch its client be bombed for
years and never be asked — and because an aggressor now weighs the opposition
again at every step of a climb, a guaranteed state was never climbed at at all.
Every guarantee had become a border. The call now happens at `SHOOTING_RUNG`, on
the aggressor's side only, and the guarantor arrives at the rung the aggressor is
standing on rather than always at 8. Measured over twelve thirty-year runs: 16
guarantees honoured, 8 by a great power, 5/12 runs, against master's 16/9/6.

Both hashes re-pinned once, at the end. 95 sim tests and 13 web tests green.

**Played, at 108 nations, on both surfaces.** As Iraq: `quarrel Kuwait` opens at
rung 1 in the Gulf, `commit 3` gives arms to a proxy with sanctions following,
`commit 6` gives standoff strike, `commit 8` brings the invasion — and with it the
coalition, Saudi Arabia's parliament *refusing* the United States the use of its
bases while Turkey grants them, which is §6's access requirement doing its job in
front of the player. Political capital 39 → 35 → 25 → 20 → 6 across the climb. The
war ran fourteen months, the British expedition wore down one rung a month from 8
to 4, and in May 1991 Iraq could no longer defend its own ground and quit. As
Ethiopia — one of the seventy-seven that had no theatre before this rebase — the
quarrel with Kenya opens over East Africa and is fought at home. In the browser the
same climb runs off the nine-rung sheet, each rung priced or refused in words, and
the conflict card reads out the deployable fractions the player is actually
fighting with: USA 0.158, Iraq 0.041, Kuwait 0.036, UK 0.065. Nobody typed any of
those four.

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

## Measured: the century run is going super-linear

A hundred years, headless, default seed, warm, release:

| nations | seconds | vs previous |
|---|---|---|
| 30 | 0.744 | — |
| 108 | 2.93 | 3.9x for 3.6x nations |
| 137 | — | not measured |
| 160 | 12.4 | **4.2x for 1.5x nations** |

The step from 108 to 160 is the one to look at: 1.5 times the nations for four
times the cost is worse than quadratic, and the relations matrix — rewritten
precisely to survive this — is no longer the whole story.

**Not yet urgent, and worth saying why.** 12.4s over 1200 ticks is 10ms a month.
A player never sees it; monthly ticks are not a frame budget. What it costs is
CI: `a_century_holds_together` runs three seeds and takes ~37s on its own. At
190 nations plus the finance and trade layers still to come, that becomes the
slowest thing in the suite and eventually the reason someone stops running it.

**Where to look first, unprofiled.** `spheres-sim/src/tech/mod.rs` holds six
per-nation loops, and `absorptive_capacity` is the suspicious one: for every
nation it walks every *other* nation to average its relations, and then calls
`sanctioned_by_count`, which scans the sanctions list. That is O(n²) plus
O(n·s) every tick, and the openness figure it computes could be one pass over
the relations matrix per tick rather than n scans. **Measure before changing
it** — this project has twice chased a plausible cause that turned out to be a
coincidence, most recently blaming the technology tree for a bug that was one
line of trade code.

## Closed: an idle player could walk a nation through zero GDP

The most serious bug found so far, because it broke the game in the one
configuration a human actually uses. Found by `feat/influence` while doing
something else, and pre-existing on master.

Take the United States and advance the clock without touching anything:

```
    let mut w = world_1990(GameRules::default());
    w.player = Some(NationId::USA);
    for _ in 0..420 { tick_month(&mut w, &[]); }
```

GDP crosses zero in June 2016 at -10.98, `war.rs` square-roots a negative
budget, `mil_strength` becomes NaN, serde writes NaN as `null`, and the browser
refuses the save. **Two causes, and the interesting one is not the obvious one.**

**Nobody was governing.** `politics.rs` skips the player's central bank so the
AI can never overwrite a rate the player chose. It skipped a player who had
chosen *nothing* just as thoroughly, so the seat held 1990's 8% into a
deflation: a 13% real rate, a permanent -5.8pt demand gap, thirty-five years of
contraction that nobody decided. `WorldState::player_set_rate` latches the first
time the player sets a rate and the bank stands down for good; until then it
runs on their behalf. Latched on the *command* rather than on "the rate differs
from 1990" so that deliberately re-setting 8% still counts as governing. An idle
USA now runs 5.8tn -> 15.8tn over the campaign with 2.2% inflation and 0.94 debt.

**The arithmetic had no floor.** A deflation alone does not send output through
zero; a term proportional to `1/gdp` does. `oil_effect` divided by `n.gdp` and
was uncapped, while `embargo_drag` four lines below carries the same shape and
has been capped at 0.12 since it was written, with the comment "an uncapped
version would feed on its own collapse". As output fell the oil ratio climbed to
**65,700**, drove annual growth past -1200%, and turned the monthly factor
negative. The tell in the trace is that GDP fell by a near-constant ~57bn every
month at the end, which is what `gdp * (k/gdp)` looks like.

Two bounds, both on the *rate* rather than the result:

- `oil_revenue_gdp` caps at 2.0. Oil income is a share of output, and the
  ceiling is set from what governed play reaches: over three seeds and
  thirty-five years the highest any live producer saw was 1.25 (Kuwait 1994),
  p99.9 was 0.71, the median 0.038.
- `growth_annual` floors at -0.95/yr, which makes `gdp > 0` provable rather than
  patched: `1 + (-0.95)/12 = 0.921` is positive for every input, and `economy.rs`
  is the only site that scales a living nation's GDP.

**Neither binds on a working economy, and that is measured.** Dumping gdp,
inflation, mil_strength and debt for every nation after 420 months across three
seeds gives a file byte-identical to the previous code. The golden hashes moved
anyway, because `WorldState` gained a field and the hash fingerprints the struct
as well as the numbers in it; both re-pinned with that dump as the evidence.

**A floor under the result is not a floor under the arithmetic.** The first
attempt clamped the output — `gdp.max(0.001)` — and the suite went green.
Running past the clamp shows what that bought: the idle USA sits on the floor
for a year, then "recovers" to a **$120bn economy holding 100.00 stability**, all
the way to 2025, because the terms that drove it there go on compounding
underneath the clamp. Finite, positive, and nonsense. Worth remembering the next
time an invariant test passes: `an_idle_player_cannot_break_the_world` now holds
the played world to what `economic_invariants_50_years` holds the headless one
to — debt under 6.0, stability in range — plus a check that the seat has
not evaporated, and it was verified red against both the original bug (fails on
"USA debt spiral 6.008 in 2008", eight years before the crash) and against the
result clamp (fails on "seat has evaporated — gdp 561.07 against 5980 at
start, in 2014").

Two smaller things fell out of it. `a_player_who_sets_a_rate_keeps_it` goes red
on the first tick if the bank is ever re-enabled unconditionally for the player,
which is the tempting wrong fix. And the currency peg pins the rate at 5.5% and
says in as many words that it stops floating — but a player who pegs without
ever having set a rate reads as ungoverned, so the default bank drifted it:
0.055 pinned, 0.078 six months later, for 26 political capital. Enacting the peg
latches too. Nothing enacts stratagems for the AI yet, so that is a player-path
fix only.

## Next (rough priority)

### 0. The calibration tests are green, down from four red

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

**The fourth has since been fixed, by the shape change this entry called for:**

- **`arms_transfers_build_a_client_army`** — was 10.9 vs 7.7, a single-seed
  treated/untreated ratio sitting at 1.4993 against a bar of 1.50. Never an
  economic-calibration failure; the shape problem was the one this entry always
  described, that any treated/untreated ratio drifts as the world fills up
  because a filling region arms the control too. It is now a cross-seed median
  measured against the control arm rather than a remembered number, with a
  zero-transfer guard so it cannot become a test that passes on nothing. Green
  at 160 nations.

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
  CPI, a 9394% deposit rate — used to put GDP at +inf inside the first simulated
  year and fail twelve tests. Brazil, Poland and Yugoslavia therefore carry
  figures one decimal place low. The files now say so, in full, with the measured
  cost; the loader refuses anything outside the band the model can hold.

  **Re-measured since the growth floor landed, and the conclusion holds for a
  different reason.** Entering the true 2948%/9394% pair past the loader guard no
  longer breaks anything: over ten years **no nation loses its sign or its
  finiteness**, because `1 + growth/12` can no longer reach zero. What it does
  instead is collapse Brazil to **9.3% of its 1990 output within three years**,
  pinned at the -5% deflation clamp for four of them, against a real 1990
  contraction of 4.3%. So the floor converted a fatal figure into a merely wrong
  one — robustness, not fidelity — and the three nations still carry
  shifted decimals. The loader guard and its message have been re-justified on
  those grounds rather than on the sign change, which no longer happens.

  The lesson worth keeping is that the floor is a **backstop under this entry,
  not a substitute for it.** It guarantees the world survives a bad monetary
  input; it does nothing to make the input representable.
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

### 8. Three calibration tests are single-seed instruments and the roster is
now large enough that they re-roll every time it grows

Found on `feat/r2-pacific` while adding five island states, and it is a finding
about the suite rather than about the Pacific. `sanctions_cost_the_target_real_
growth`, `a_trade_agreement_lifts_the_smaller_partner_and_then_binds_it` and
`the_frontier_does_not_run_away` each take one 20-to-35-year whole-world run at
one seed and assert an absolute band on the result. Adding any nation to the
roster shifts the RNG stream — five more nations is five more draws a tick — so
the reading is re-rolled even when nothing about the measurement changed.

MEASURED, on master, roster and coefficients untouched, by inserting N discarded
`rng.f64()` draws per tick and running the suite:

| N | Brazil growth lost to sanctions | Poland trade ratio |
|---|---|---|
| 0 (shipped) | 1.88pt | 1.352 |
| 1 | 0.03pt | 1.277 |
| 3 | 0.25pt | 1.302 |
| 7 | 1.62pt | 1.159 |
| 11 | 0.54pt | 1.170 |
| 17 | 0.96pt | 1.170 |

The sanctions test's acceptance band is 1.2..2.5 and the trade test's threshold
is 1.20. Four of those six perturbations put sanctions outside its band and four
put trade below its threshold, with the model bit-identical apart from the
discarded draws.

Across ten seeds on master the same two quantities read: sanctions
`[1.81, 1.76, 0.91, 3.24, 1.08, 2.02, 2.10, 1.62, 2.57, 2.84]`, median 2.02;
trade `[1.187, 1.213, 1.351, 1.210, 1.168, 1.205, 1.028, 1.267, 1.208, 1.324]`,
median 1.210. So four of ten seeds fall outside the sanctions band and four fall
below the trade threshold on master itself. The instruments scatter wider than
their own acceptance bands.

`the_frontier_does_not_run_away` is the least fragile of the three but has the
same shape: on `feat/r2-pacific` the UK reads 4.1%/yr against a 4.0% ceiling at
the default seed, while seeds 0-5 on the same branch read 2.90, 3.64, 2.71,
3.66, 3.45 and 3.27. One seed is over; the world is not running away.

**What to do, and it is the integrator's call, not a roster author's.** The repo
has already solved this once — `arms_transfers_build_a_client_army` was
converted to a cross-seed median for exactly this reason — and the same remedy
fits here. It is a strengthening and not a widening: a median over ten seeds is
harder to satisfy by luck than one seed is. The catch is that master's trade
median is 1.2102 against a threshold of 1.20, so that test's threshold has to be
re-derived at the same time rather than carried over. Do this once, at whatever
the roster is when round two lands, not twelve times in twelve branches.

### 9. Later
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
