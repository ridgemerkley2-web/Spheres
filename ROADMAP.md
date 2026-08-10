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
  trend that already priced it in. Convergence comes from the distance to the
  frontier — the flat bonus for being poor is gone, because the tree already
  modelled diffusion and the two together counted it twice

## Next (rough priority)

### 1. Blocked on a decision, not on work
Both branches are green in isolation and collide with the expanded roster.

- **`feat/statecraft`** — pacts, patronage, covert action, trade. This is the
  namesake system: spheres of influence are still only a bare relations matrix
  without it. Merging turns two tests red. Its own trade test wants the small
  partner to lose 10x what the large one does on abrogation and gets 8.5x, because
  the wider roster left the United States marginally more trade-dependent; and
  Iraq keeps a sanction 25 years on, because statecraft couples pacts and trade to
  sanction state and there are now more parties to sustain one. Decide whether the
  thresholds or the models move.
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

### 3. Political capital
SPEC names two spend-currencies — economic output and political capital — and the
second does not exist in the code. Every system is meant to be a buyer of one or
both.

### 4. Harvested from `feat/tech-eras`, deferred (tag `archive/tech-eras`)
The scalar model was retired in favour of the tree, but two of its ideas were not
ported and are worth their own branches:
- **Live era rotation.** `Era` is a static calibration bracket. It should be a
  paradigm the world passes through, opening on a frontier threshold rather than a
  date, with a vigour curve so a fresh paradigm is fertile and an exhausted one is
  not.
- **Human capital.** `absorptive_capacity` reads development, openness, the command
  penalty and sanctions — but not schooling.

### 5. Known modelling gap: Japan
Japan carries the highest transcribed 1990 trend of any nation and nothing ever
takes it away, so it settles near 2.8% and outgrows the United States for the whole
run. The lost decade is modelled as a bubble hangover rather than the permanent
break it was. Wants a demographic or balance-sheet mechanism.

### 6. Two domains have rival implementations
`feat/tech-biotech` and `feat/tech-transport` hold uncommitted second versions —
32 biotech techs against the 29 merged, 27 transport against 26, sharing only about
five ids with what is on master. Someone has to pick, or merge the best of both.

### 7. Later
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
