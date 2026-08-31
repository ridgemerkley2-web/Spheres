# SPHERES — the road to 1.0

*BIBLE.md is what the game is and refuses to be. ROADMAP.md is what is built.
BUGS.md is what is broken. This is the sequence between here and done, and why
it is in this order.*

**Where this came from.** Rewritten 2026-08-18 from a directed audit: ten
independent reads of the tree, five competing plans written from deliberately
opposed priors, four judges scoring all five, and a completeness critic. Every
judge picked a different winner; the plan below is the one no judge scored below
7.5, with the others' best steps grafted on. Figures in it are measured, not
estimated, and where a proposal asserted something false the correction is kept
beside it rather than quietly dropped.

**Ordering principle: rework-if-deferred, not interest.** Unchanged, and it is
still right. Every step states what it costs to land later.

**Definition of 1.0.** A single-player grand-strategy game, January 1990 start,
that a stranger can install and play for four hours without reading source.
Note that this clause has never once been measured — which is why step 11 is a
step and not a hope.

**The thesis.** SPHERES is not missing systems. It is missing a finished game.
So: the roster closes at 137, the simulation stops growing, and every remaining
hour goes to landing what exists, making the numbers it shows true, and putting
them in front of a person.

Sizes: **[S]** one session · **[M]** two to four · **[L]** a week or more.

---

## 0. Land trunk and make it double-clickable — **DONE 2026-08-18**

- `ladder-rebase` merged: exactly the three files `git merge-tree` predicted
  (`data/mod.rs`, `politics.rs`, `world.rs`), all three unions rather than
  decisions. Carries the century-run perf fix — **the suite went 311s → 167s**,
  and both golden hashes held unmoved, so no re-pin was needed.
- **`master` fast-forwarded 61 commits, d1c64fa → b55a29f.** The trunk now *is*
  the game: 137 nations, the commitment ladder, statecraft, governments, the
  tech tree. It had been advertising 24 nations while all of that sat on a side
  branch.
- `Play SPHERES.cmd` fixed. It ended `"%~dp0dist\spheres-web.exe"`; `dist/` is
  gitignored, nothing builds it, and the directory does not exist — so the one
  file a player double-clicks worked on no branch. Now builds through cargo,
  opens the browser *after* the build rather than onto a dead port, and says so
  if cargo is missing.
- BUGS.md B-1 corrected to its real site (`commitment.rs:159-160`).

**Still open from this step, and deliberately not done without your say-so:**
deleting the ~67 stale branches, and moving the worktree root off the OneDrive
path. Both are destructive or touch your environment. `.gitignore` gained
`target*/` after `git add -A` swept 1693 build artefacts into a commit that had
to be undone.

## 1. Instruments and CI, before anything moves a seed — **[M]**

Convert the four single-seed instruments to cross-seed median plus a control
arm, using `arms_transfers_build_a_client_army` as the literal template and
re-deriving every band at 137 nations: `sanctions_cost_the_target_real_growth`,
`a_trade_agreement_lifts_the_smaller_partner_and_then_binds_it`,
`the_frontier_does_not_run_away`, `a_large_nation_is_subjugated_rather_than_swallowed`.

Then promote `anomaly_sweep` into a **story census**: not just "did anything
break" but *what happened* — conflicts opened, endings by variant, conflicts
still open, borders moved, governments fallen, patrons flipped. Then one GitHub
Actions job on Linux running the suite, clippy, the century run, the census, and
the golden hashes **as a separate required job**.

*Why:* four of ten seeds already fall outside two of three acceptance bands on
unchanged code. Every step below deliberately moves those readings, and the only
available response to a red band without this is to widen it — which iron rule 5
forbids and which is how 61 commits ended up stranded.

## 2. Stop the surface lying — **[M]**

Delete the JS growth model (`index.html:866-899`) and serve a sim-computed
projection instead. It uses `n.sanctioned_by_count * 0.006` — the flag-counting
rule `economy.rs` documents *deleting* — and omits both the tech energy-exposure
factor and the `MAX_OIL_SHARE` cap, so "Expected growth" is wrong for every
sanctioned nation and every oil importer. Wire a Load button to `/api/load`
(exists, called zero times). Narrow `is_major`, which degrades a USA player's
1YR and 5YR buttons to single-stepping.

## 3. Merge `feat/influence` — **[L]**

Five commits, 1019 lines: decay-as-upkeep, alignment hysteresis, contest cost, a
monthly political-capital bill, `seat_1990`. Three-file conflict, all command
dispatch. Re-derive its constants at 137 nations against the now-converted
instruments.

*Why:* sphere membership today is `patrons_of`, a query over live aid rows — cut
the cheque and the client leaves that month. Nothing decays, nothing was ever
held. This is the namesake mechanic and the largest built-but-unshipped asset in
the repo.

## 4. Seat the 1990 board — **[S], and read the warning first**

`data/mod.rs:602` is `statecraft: Statecraft::default()` — January 1990 with no
NATO, no Warsaw Pact, no EC.

> **⚠ The critic's catch, and it invalidates the naive version of this step.**
> `dyads.rs:172` returns `0.0` war appetite for *any* pair holding a pact — "A
> guarantee is not a modifier, it is a bar." Seating NATO and the Warsaw Pact
> would therefore zero war appetite across precisely the Soviet-periphery and
> Balkan dyads a 1990 start exists to dramatise. **"Seat the board" and "make
> borders move" are mechanically opposed.** Four of the five proposals missed
> this. Resolve the bar-versus-modifier question *before* transcribing anything.

*Correction kept from the audit:* two proposals claimed `ProposeAlliance` and
`ProposeTrade` are issued by no AI path. **False** — `politics.rs:969
ai_statecraft` issues both through `apply_command` monthly. The defect is only
the empty opening board.

## 5. Put the sphere on the screen — **[M]**

Serve statecraft on the payload; add the seven missing `parse_command` kinds (20
of 31 `Command` variants parse today); a Sphere card on the nation sheet; a
`MAP_MODES` entry colouring the world by patron.

*Why:* a grep for `statecraft|pact|aid|covert` returns zero hits across the whole
web layer. The game is named after a system the player can neither use nor
watch. Zero calibration risk — the commands are already priced.

## 6. Make wars end, and make endings cost something — **[L]**

(a) **B-1** at `commitment.rs:159-160`. (b) Give `Ending::White` consequences —
it is the modal outcome and sets nothing. (c) Add `origin_defender` so the
ending arms stop resolving the loser as `side_b.first()`. (d) Scale attrition by
committed force so rungs 1-5 stop being strictly dominated. (e) Route coalition
entry through the command queue — **today a human holding a defence pact is
written into a rung-8 campaign with no click and no charge**. (f) Add outcomes
below total collapse, and run the census before and after.

*Why:* forty years of war leave the world unchanged — one border moved in 480
nation-centuries. Highest-variance work here, which is why it sits behind the
instruments.

## 7. Fix the two economy errors that produce visibly wrong numbers — **[L]**

Convert `oil_effect` from a permanent growth *rate* to a one-time level shift —
the same error class BIBLE §8 records finding in trade pacts. Bound `demand_gap`
symmetrically; add a growth ceiling mirroring `WORST_ANNUAL_COLLAPSE`; swap the
four remaining flag-counting sanction sites to `sanction_weight`.

*Why, and this survived a direct challenge:* one proposal cut this on the grounds
that the 21-seed sweep found zero anomalies. **The sweep cannot see this.** Its
detectors are NaN, clamps, debt above 6x and output above 100x — none catch a
petro-state compounding an extra 6pp/yr, which reaches ~10x over 40 years and
passes clean. An oil shock is currently a net positive for world GDP.

## 8. Give the player something to want and something to pick — **[M]**

A standing ambition card drawn from state that already exists — share of world
output, clients held, distance to the frontier, conflicts settled on your terms.
No scripted objectives, no victory screen. Rebuild the setup screen: curated
starts with a pitch and a difficulty read, search, region grouping.

*Why:* there is no victory condition, score, end date or objective anywhere in
the codebase. First contact is 137 undifferentiated tiles. **After step 7**,
because a curated-starts screen headlines Saudi Arabia and Poland — exactly the
tiles running nonsense trajectories today.

## 9. Make the stratagem deck good enough to justify refusing focus trees — **[M]**

Ten entries today. Grow to ~35, a third repeatable on cooldown, at least five
reachable by a stable rich democracy. Pin it: every start has two offers in
January 1990 and one in an average month.

*Why:* BIBLE §5 stakes an accepted product risk on this deck being the answer to
focus trees. A USA 1990 player fails nine of ten gates and reads "The world is
offering you nothing this month" permanently.

## 10. Port the government surface to the browser — **[M], designated slack**

`government.rs` is 6,684 lines, a quarter of the sim, reachable only from the
CLI. A port, not a design. **This is the item that falls off if the calendar
slips** — naming it in advance is the point.

## 11. Play it for four hours, watched, three times. Then freeze and tag — **[S]**

1990 to 2010 as the USA, as Poland, and as a middle power picking a patron,
without opening source, writing down every moment you do not know what to do or
what just happened. Fix that list and nothing else. Freeze. Tag 1.0.

---

## Not doing

- ~~**PLAN Phase 4 in full** — province map, divisions, supply, operational
  combat, hourly cadence. Not deferred: *deleted*, along with SPEC §1's
  "HOI4-style operational warfare" pillar. BIBLE §6's replacement is built and
  is the best-surfaced system in the game.~~ **Partially reinstated
  2026-08-30** by BIBLE §5's tactical-map amendment (Ridge's call): the
  operational district map — fronts, district combat, encirclement over the
  ~1,500 admin-1 districts — is back in scope, projected from §6's commitment
  ladder rather than replacing it. Still out: hourly cadence, and any grid
  finer than admin-1.
- **Any roster growth past 137**, the Horn of Africa branch, the ROSTER-to-JSON
  refactor. Adding Somalia does not help while Somalia cannot collapse.
- ~~**The tech tree screen (old 5.1) and any research command.** The marginal
  frontier technology is worth 0.004pp of annual growth. A read-only screen
  would render 253 items in high resolution and make the inertness the story.
  Precondition to revisit: the Productivity reweight.~~ **Reinstated
  2026-08-30** (Ridge's call, amended in the Phase-4 districts style above):
  the precondition is met — ROADMAP records "Productivity rebuilt on the
  tree" — and the research-command half of the cut was already overtaken
  (research_focus / research_priority exist and drive the left column). Back
  in scope: a full-screen technology graph with a symbol language and routed
  prerequisite pathways, driving only the two commands that already exist.
  Still out: any new sim command, any per-tech sim state.
- **Finance and contagion**, and an emergent Asia-1997.
- **New macroeconomics** beyond step 7.
- **Routing AI fiscal and monetary policy through the command queue.** A real
  iron-rule-2 violation, and invisible to a player. Only the coalition
  war-entry door closes, in step 6, because it conscripts the human.
- **Nightly unattended agent commits as a 1.0 requirement.** CI on Linux stays;
  agents committing overnight moves to after the tag.
- **Chronicles, Ollama, Bevy, the gravity trade model, and the ~395
  undocumented public items.** None are 1.0 gates.

## Wanted, recorded 2026-08-18: districts with resources

Ridge wants a map of locations that own resources — bigger than HOI4 provinces,
anchored on major cities.

**This probably needs no amendment.** BIBLE §5 already says the map carries
“roughly 1,500 admin-1 districts as **political** geography, never 13,000
provinces as tactical geography”, and admin-1 districts are exactly
states/oblasts/prefectures anchored on their principal cities. What §5 refuses is
*tactical* geography — combat width, encirclement, front lines, counters moved
square to square. Districts that own oil, ore, industry and population, and that
change hands at a settlement, are the sanctioned reading rather than a reversal.

Two things to settle before building it: what a district owns that the economy
does not already model at the national level, and whether  (which
already names operating areas over districts) is the place it hangs.

### Answered 2026-08-31 — Ridge's rulings, and the measurements behind them

**The first question is answered: LOCATION AND GRANULARITY.** A district owns no
quantity the nation does not already have. Oil is a complete national system in
`economy.rs` — production, revenue, a growth term, embargo drag, inflation, 55%
to the budget, and a world price responding to disruption — so a district that
*creates* oil pays every barrel twice. What districts add is *where* the value
sits, and therefore what a settlement actually moves: with resources placed,
taking Khuzestan takes a visible share of Iran's oil, and the front engine
already decides who holds it. This is the same hazard the technology endowment
hit (`tfp_base` already priced 1990 technology) and it is now the standing test
for any new layer: name the national quantity it would double-count, or it is
the wrong design.

**Stability becomes per-district, and the national figure becomes its
aggregate.** Rulings:

1. **Population-weighted**, not a plain mean — Chukotka must not weigh as much as
   Luxembourg's single district. District population shares come from a gridded
   raster against each nation's already-transcribed 1990 total, which is
   derivation from source rather than invention; the modern-boundaries caveat
   ships with it.
2. **Deviation storage, never write-back.** Districts hold a *deviation* from
   the nation's transcribed baseline, and `Nation.stability` stays the
   authoritative figure this feature never writes. Two measurements decided it.
   A re-derived mean fails bit-exactness for 84 of 131 nations at t=0 (Mexico
   55.0 → 55.000000000000036); the deviation mean fails 0 of 131, because an
   empty map returns the baseline through a `None` arm and executes no
   arithmetic at all. Exactness is control flow, not a numerical hope. And the
   trap nobody saw until it was measured: under write-back, the national mean
   reversion at `economy.rs:333` cannot tell an unstable nation from one with a
   mine, so it *eats* the penalty — 40.7% retained at year 30, decaying to zero,
   and failing invisibly. You would tune the constant up, watch it not bite, and
   tune again.
3. **The periphery gets to be cheap, for now.** For deposits in near-empty
   districts the whole thirty-year national consequence is under 0.06 political
   capital. That is defensible — it is *why* states mine peripheries rather than
   heartlands — so it ships as written, and whether district unhappiness should
   also feed `n.separatism` is decided on play data rather than in advance.
4. **Environmental damage is permanent until remediated** — Ridge's call,
   overruling a 277-month half-life. It does not quietly fade; cleaning it up is
   a second decision with its own political-capital price. The jobs boost still
   ramps over 8 months and fades on a 46-month half-life, so the shape of the
   choice is unchanged and sharper: the government that opens the mine collects
   inside one term and the bill never leaves.

**Gated, not deferred:** the develop command waits on the resource placement
data being clean. Two independent passes found the same contamination — Norway's
"bauxite" districts are aluminium *smelters*, and Steep Rock Iron Mine is filed
under both bauxite and iron — and presence-without-production is simultaneously
the honest signal for an undeveloped deposit and the place the contamination
hides. Shipping unblocked means Norway mining bauxite it does not have.

## Open questions — yours, not an agent's

The audit recommended answering these; three are product calls and one overrides
a stated BIBLE non-negotiable, so they are asked rather than assumed:

1. **Close the roster at 137?** This contradicts BIBLE §3.3's ~190 target. The
   plan above assumes yes.
2. **Delete Phase 4 outright**, rather than deferring it? Plan assumes yes; it
   is the only one that blocks anything.
3. **Archive SPEC.md and PLAN.md to `docs/attic/`**, keeping BIBLE, CLAUDE,
   ROADMAP and BUGS live? The audit recommended archive-not-delete; two judges
   called outright deletion the largest unforced risk in the field. **Not done
   — this file is still here.**
4. Delete the ~67 stale branches and move the worktree root off OneDrive?
5. Is a forty-year simmering border quarrel a bug or the intended output?
   (India–Pakistan has run seventy-eight years on that shape.)
