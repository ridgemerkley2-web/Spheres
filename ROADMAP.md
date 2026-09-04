# SPHERES Roadmap

## Local review — not yet committed or pushed (2026-09-03)

Ministry programs: fifty visible department budgets under ten ministries,
atomic annual-plan commands, shared daily project funding, realized-spend fiscal
settlement, Defense ownership of supply and force support, and ten Industry
investment choices with a powered materials/machinery/construction chain.
See `MINISTRY_BUDGET_DESIGN.md` for implemented boundaries and remaining slices.
Keep general services prebaked; next decisions are balancing/build pacing and
which service department or manufactured-goods market to deepen after review.

Province GDP foundation: worldwide country/sector/province accounts, modeled
starting allocations anchored to existing national GDP, explicit unmapped
remainders, and local value-added receipts for current project activity.
Province cards and country Economy dossiers expose the composition and distinguish
production from enabling capacity. See `PROVINCE_ECONOMY.md` for accounting and
data boundaries. This extends, and does not discard, the ministry review build.

## Done (v0.5 rebuild)
- **Two Codex pushes checked and three of their daily passes repaired**
  (2026-09-03, branch `check/daily-ad60482` off `feat/hoi4-map-and-tech`
  ad60482, commits 7ccb706 and the docs commit after it; every figure below
  was measured on that tree in a watched clean release build). The two pushes
  since our 3ecea29 landing: **fb08fa5** "feat: add historical leaders and
  domination campaign" (660 files, +29,488/-63) and **ad60482** "feat: run
  the simulation daily with physical freight logistics" (30 files,
  +3,977/-338; Ridge's ruling quoted, "Can we get everything on a daily
  ticker not monthly?", BIBLE/CLAUDE/SPEC amended and DAILY.md added).
  **What held.** Both golden pins and both ACTUAL constants are textually
  unchanged and the actuals still MEASURE `0xe26e4bf8d6c60066` /
  `0xbe94d6125631829c`, run alone with `--exact`, twice; the four legacy
  daily-calendar tests are present, un-ignored and green; not one test
  function name is absent (447 → 509 `#[test]` attributes, 62 → 62
  `#[ignore`; 513 after the repairs); legacy
  headless determinism is byte-identical to 3ecea29 on all eight
  (seed × market) pairs of `run 35`, each run twice — market OFF 1990
  `d1a2cfbf7c6958d7` (3,501 lines), 7 `39dea3341a7f6e8c` (3,983); market ON
  1990 `1574abf65b382173` (3,873), 7 `f97da62d5daee785` (4,234). The daily
  push's own proration was read line by line and is correct wherever it
  touched (flows × dt, convergence through `clock::blend`/`decay`, hazards
  through `clock::chance`, counters and deadlines in actual days, levels and
  dimensionless ministry multipliers left unscaled — including all ten of our
  ministry arms, the treasury's fiscal block and `ResearchTerms::total()`).
  **What did not hold — three monthly passes the push never touched, now
  running thirty times a month, all fixed in 7ccb706 with a test watched red
  each (BUGS Y-1..Y-3).** (1) `resources::cool_refusals` steps its lattice of
  twenty-fourths once per call and `statecraft::tick` called it daily: a
  refusal forgotten in 24 DAYS, not months (row gone 1990-01-25 against
  1992-01-01, 30.4x). Gated to the month's last day; the lattice cannot be
  prorated. (2) The AI buy pass ran daily while physical freight counts only
  the day's fills, so it re-asked while its own signing was at sea — France
  signed SEVEN copper contracts with the USA on 3–9 January (seed 7, player
  Iraq) against one signing in three legacy months, and "cannot deliver"
  was headlined 88 times in 90 days. Gated to the month's last day. (3)
  Arsenal and manufacturing merged order rows on `due_days`, which moves
  every day: 49,271 rows against 1,619 after twelve default months (USA 364
  against 12), the save 8,264,934 bytes against 1,034,809. One
  `arsenal::book_order` keys daily rows on the DELIVERY calendar month; after
  it 1,621 rows (USA 12), 1,092,641 bytes, board book value 2984.567 bn in
  both modes. A year-long replay test (stepped == resumed at day 100 ==
  `tick_month`-batched; seed 1990 `0x3391cbcef7c62098`, seed 7
  `0x5f6650ed1e0cd621`) now sits in `tests/daily.rs`.
  **Filed, not fixed, because each needs a ruling (BUGS Y-4..Y-15):** the
  daily integrator is biased LOW against the calibrated monthly model
  (the audit's skeptic probe: 131/137 nations lower after one month, median
  −1.6e-4 of level, USA −8.1e-5; twelve months median −3.3e-3) because legacy evaluates the growth
  rate at the start of the month and daily along the path — the "own
  evidence" CLAUDE.md asked for, negative for year one; the player's private
  investment flow is 3.0% over under `blend`; the tech burst cap is per tick;
  covert upkeep's phase moved; the force-majeure headline is per day; the
  domination campaign adds the game's only ending, requires war to reach it,
  runs its observer daily, writes a sovereignty row nothing reads, and
  amended no doctrine and quoted no ruling; the daily amendment struck
  nothing through; two spheres-web assertions changed meaning; the browser's
  "Enact & advance" enacts nothing on untouched dials; and the binary
  weight — 303 `include_bytes!` totalling 322,059,267 bytes (160 AI-generated
  leader PNGs 305,173,085 + 143 Commons portraits 16,886,182), the pack
  291.44 MiB, `spheres-web.exe` 349,737,875 bytes. Licence state as measured:
  every one of the 143 portraits carries a free licence on the allow-list
  (108 public domain, 11 CC0, 4 CC BY 4.0, 2 CC BY 2.0, 1 CC BY 3.0, 8 CC
  BY-SA 3.0, 5 CC BY-SA 4.0, 3 CC BY-SA 2.0, 1 CC BY-SA 2.5), 17 nations
  fall back to a cameo, all 160 leader artworks record "OpenAI image
  generation" with a prompt record and sha256; `tools/avatars/check_assets.py`
  passes; `tools/resources/check_resources_1990.py --fast` 60 checks, 0
  failed. **Suite after the repairs, watched clean:** spheres-cli 1 / 0; spheres-sim 331 passed / 3 failed / 52 ignored (lib 288 / 3 / 22 in 192.79 s; tests/daily.rs 6 / 0; ministries 19; research 8; treasury 10; the five ignored-only probes 30); spheres-web 112 / 0 / 2; clean+build+test wall 15:31:10 to 15:36:57 = 347 s; the daily replay test reads 0x3391cbcef7c62098 (seed 1990) / 0x5f6650ed1e0cd621 (seed 7); both goldens re-run alone twice, actuals 0xe26e4bf8d6c60066 / 0xbe94d6125631829c; all eight `run 35` digests re-measured equal to 3ecea29 after the repairs — the reds
  exactly the three deliberate ones (BUGS E-3 and the two goldens).
- **The treasury, the escalating interest and the ministry collapse**
  (2026-09-02, Ridge's call, quoted: "Add in an interest over GDP figure that
  inflates based on percentage. You can cook the rest into the GITHUB dir";
  branch `feat/ministry-economy`, ~~20~~ **21** commits on
  `feat/hoi4-map-and-tech` 9274baa — recounted 2026-09-02,
  `git rev-list --count 9274baa..7e8028e` = 21, the docs commit itself being the
  twenty-first). Three things landed together.
  **(1) Money became a stock.** `treasury_bn` and `debt_bn` are `Option<f64>`
  in billions of 1990 dollars, `skip_serializing_if = "Option::is_none"`, seated
  by the first `Command::SetAnnualBudget`; `debt_gdp` remains the stored field
  and the single source of truth, with the fiscal block its only writer on the
  open arm. `economy::charge` absorbed all ~~five~~ hand-rolled ratio pushers
  (`resources::settle` both legs, pact upkeep, aid, covert action, patronage),
  taking BOTH the dollars and the caller's exact pre-treasury ratio so the
  closed arm never recomputes. **Corrected 2026-09-02 on the merge below:** five
  was the count on this branch alone, at six call sites; upstream had written
  three more direct legs after the helper existed, and routing them took it to
  **nine call sites**, MEASURED — `resources::settle` (two), `apply_market_net`
  (two), `start_mine`, `statecraft.rs` (three) and `government.rs` (one). Interest escalates: `real = (policy -
  inflation).max(-0.02)`, `spread = ((debt_gdp - 0.60).max(0) * 0.06).min(0.06)`.
  MEASURED at 5% policy against 3% inflation — 30% of GDP pays 2.0000%/yr,
  60% 2.0000%, 90% 3.8000%, 150% 7.4000%, the cap binding from 1.600 up; the
  roster's median 1990 ratio is 0.52, so the median borrower pays the policy
  real rate exactly. 79 of 137 nations carry a transcribed 1990 `reserves_bn`
  (World Bank FI.RES.TOTL.CD, end-1989 observation), 17 refused for want of a
  source and 41 left out as immaterial (BUGS M-9, M-10).
  **(2) The thirty scattered addends went**, and each ministry took one or two
  named arms defined once in the new `spheres-sim/src/ministries.rs` and called
  from all twelve charge sites. Potential growth lost 5 addends, demand 3,
  unemployment 5, private investment 4, stability 3 plus `ds += social_gap *
  12.0`, and the cohesion term lost housing's half. Families became **Housing**
  in the constant, the served JSON key, the UI label and the summing helper, on
  Ridge's standing ruling. Research gained `ResearchTerms` — seven named arms
  reproducing the old scalar chain in order, asserted `to_bits` equal across
  36,709 nation-months — plus an eight-way `SetResearchAllocation` priced
  like the surviving `SetResearchPriority` preset.
  **(3) The cards were made to read the sim**: treasury, revenue, spending,
  interest (dollars, share of GDP and the effective rate) and net position, with
  debt service as an eleventh unelectable row above the ten dials; every
  per-ministry sentence served from `ministries::arms_at` rather than recomputed
  in JavaScript; stability quoted as a DESTINATION, never a first-year rate.
  **Five defects found by review and repaired in the same branch**: a doctest
  that was a fourth red; the social-aggregate route by which all six social
  ministries moved demand and inflation identically to the last digit; debt
  charged TWICE on the open-books arm (0.025200 of output against a closed-books
  nation's 0.006000, a factor of 4.20) now gated so `debt_drag` and cash interest
  are mutually exclusive; four arms clamped downstream of `ministries.rs` and so
  quoted at slopes the sim never charged (the stability card promised up to +169
  points on a 0..100 scale); and health's and industry's first-draft x20 slopes,
  which left 70.3% and 78.0% of the mean nation's dial buying nothing, re-derived
  to 6.0 and 4.2 the way education's 15.0 was.
  **THE MEASURED SUITE**, release, `--no-fail-fast`, watched building in its own
  `CARGO_TARGET_DIR` (2026-09-02, on `feat/ministry-economy` 532c818):
  spheres-sim `--lib` **197 passed / 3 failed / 22 ignored**; `tests/ministries`
  19/0/0; `tests/research` 8/0/0; `tests/treasury` 7/0/0; `growth_decomposition`
  0/0/20; `capital_damage_audit` 0/0/5; `endowment_channel_probe` 0/0/1;
  `endowment_margin_probe` 0/0/1; `sample_size_audit` 0/0/3; **spheres-web
  94 passed / 0 failed / 2 ignored**; **spheres-cli 1/0**; Doc-tests 0/0/0.
  **The three reds are the three deliberate ones and the pins were NOT touched**:
  `the_1990_start_is_pinned` (lib.rs:4193) still 0xd022d50f43c984da reading actual
  **0xa5c9c5b2306313d8**, `golden_hash_of_a_known_run` (lib.rs:4432) still
  0xbd5ec0f43c5f2e3b reading actual **0x20c24ab0f1581807**, and
  `the_1990_endowment_does_not_move_year_one_growth` (tech/mod.rs:2461) at Belgium
  0.001851 granted against 0.001749 ungranted — all three unmoved from the
  base, so the whole branch is inert on the default path. All four headless
  references reproduce byte for byte, each run twice (`spheres-cli run 35`):
  market OFF seed 1990 `d1a2cfbf7c6958d7` / 3501 lines, seed 7 `39dea3341a7f6e8c`
  / 3983; `SPHERES_RESOURCE_MARKET=1` seed 1990 `6cb6c97ab33fb80d` / 4007, seed 7
  `8d29fecfd4ff9bf4` / 4258.
  **MERGED ONTO `feat/hoi4-map-and-tech` 61b388f** (2026-09-02, a merge and not
  a rebase — the rebase failed on commit 1 of 21). Origin had moved 21 commits
  under this branch with Codex's province production, manufacturing and arcade
  logistics, and the collision was semantic: upstream added three NEW direct
  `debt_gdp` money legs that this branch's register bar forbids outright, and
  raised its own count of the field to seven while this branch had driven it to
  zero. All three — the aggregate spot settlement's two arms in
  `apply_market_net` and the mine investment in `start_mine` — were routed
  through `economy::charge`, which is what BUGS M-2 asked for and what Ridge's
  design already implied. The bar now asserts the union: `resources.rs` names
  none of `debt_gdp`, `treasury_bn` or `debt_bn`, and its whole reach into a
  nation's finances is five `economy::charge` calls.
  MEASURED ON THE MERGE, and the inertness claim is now stronger than it was on
  the branch alone. The merged tree's two ACTUALS are **origin's**, exactly —
  0xe26e4bf8d6c60066 (start) and 0xbe94d6125631829c (run), where this branch
  alone read 0xa5c9c5b2306313d8 / 0x20c24ab0f1581807 and the whole of the
  difference is upstream's serialized `district_population`. The two golden
  PINS, 0xd022d50f43c984da and 0xbd5ec0f43c5f2e3b, were not touched and are
  still red. All FOUR headless `run 35` streams are byte-identical to origin's
  own at 61b388f, each measured twice: market OFF seed 1990
  **d1a2cfbf7c6958d7** (3,501 lines) / seed 7 **39dea3341a7f6e8c** (3,983), and
  market ON **1574abf65b382173** (3,873) / **f97da62d5daee785** (4,234). The
  market-ON pair recorded above against `867b3d6` — 30cf39058ba9ae1f /
  6daccc96382f7659 — was already stale at 61b388f: origin moved it, not this
  merge, which was confirmed by building 61b388f alone and reading the same two
  hashes. Suite: spheres-sim 272 passed / 3 failed / 52 ignored with exactly the
  three deliberate reds, spheres-web 102 passed, spheres-cli 1 passed.
  Doctrine amended in the house style: SPEC §3 (the treasury, the escalating
  interest, the ministry map), BIBLE §4 (what each ministry buys), CLAUDE.md iron
  rule 8 (one named arm per effect, defined once), BUGS D-1 closed and D-2
  settled player-only, and every invented coefficient filed as BUGS I-1..I-16.
  **AND THE MERGE'S OWN BLIND SPOT, found by review and closed the same day
  (BUGS M-11).** The bar it re-expressed holds `resources.rs` to the one fiscal
  channel; `stratagems.rs` was holding two legs older than the channel and was
  never looked at. `debt_restructuring` and `mass_privatisation` both moved
  `debt_gdp` and left `debt_bn` alone, so for a government on the books the
  fiscal block recomputed the ratio from the untouched stock and erased the act
  inside a month — measured, Brazil's $500.500bn of debt still $500.500bn after
  restructuring, against $275.275bn. Both are now routed through
  `economy::charge`, bit-identically on the closed board (the write-down passes
  `ratio * 0.55 - ratio`, exact by Sterbenz; the sale passes `-0.08`), and a
  second register bar in `spheres-sim/tests/treasury.rs` holds that module to
  the channel, red-checked once per leg.
- **The resource pass back under budget, and a market-on guard that can fail**
  (2026-09-02, `63e6c90` "perf: one stall mask a month, not twelve a dyad" and
  this branch's "test: make the market-on row bar a ratio" plus its
  re-derivation onto `867b3d6`): `2f9791e` pointed
  `dyads::last_resort` at `resources::action_stalled`, which pays a `draw` — and
  therefore an `arsenal::pick` fold over the whole 46-entry `DECK` — plus a
  binary search into the 552-row ledger, ONCE PER TRACKED COMMODITY PER DYAD PER
  MONTH. `tests::the_resource_pass_stays_under_budget` went from 0.0400
  ms/month at `e4e3c03` to **1.3598** against its 0.15 bar. Three repairs, each
  measured on its own: `resources::action_stalled_mask` built once per (nation,
  month) rather than twelve times per dyad (appetite 1.2657 → 0.0469),
  `resources::change_market_stock` collapsed from three binary searches to one
  (the resources row 0.0580 → 0.0348), and `arsenal::pick` reading a precomputed
  value ranking instead of refolding 46 divisions (appetite 0.0439 → 0.0226).
  **Total 1.3598 → 0.0766**, 17.8x; 0.0821 after the rebase onto `a9a373d` and
  0.0637 after the rebase onto `867b3d6`.
  **The bar was not touched** and no test was deleted, ignored or widened.
  Behaviour is bit-identical: both golden ACTUALS still read
  `0xe26e4bf8d6c60066` and `0xbe94d6125631829c`, and all four headless digests
  are unmoved, the two market-ON ones included.
  **The new guard, and why it is a ratio.** `the_resources_row_is_free` was
  market-OFF only and stayed green through the whole regression. A market-ON arm
  was added at 0.10 ms/month — a bar that could not have gone red for the
  0.0577 it named — then tightened to 0.055, and a review then measured 0.055
  going RED ON HEALTHY CODE on a busy box (fourteen saturated readings
  0.0472-0.0611, four over the bar). Confirmed here: four saturated invocations
  read best-of-five 0.0562, 0.0574, 0.0582 and 0.0661, every one over 0.055, one
  individual pass at 0.1186 with nothing wrong. **An absolute millisecond bar
  cannot do this job on a box that builds several worktrees at once** — healthy-
  under-load and the regression overlap — so the arm now asserts the row as a
  **share of the rest of the same month tick**, in which machine speed and
  every other process cancel. Measured on `867b3d6`: healthy 0.01798 mean over
  30 quiet readings and 0.01725 over 10 saturated, range 0.0150-0.0192;
  **bar 0.022**, rule 7's floor being mean + 2.326·sd = 0.0198 (z = 5.0) and
  the bar within 1% of the geometric midpoint of the gap it has to sit in.
  **Red-checked**: reintroducing `4fbc806`'s three-binary-search
  `change_market_stock` reds it five invocations out of five, **two of them
  under full sixteen-core saturation**, which the millisecond bar could not do
  at all. The market-OFF arm and its 0.02 bar are untouched and read
  0.0029-0.0031 throughout — blind to the regression, which is the blindness
  the ON arm exists to end. **Re-derive, do not scale**: the share is a property
  of the whole tick, and `867b3d6`'s province manufacturing moved it from the
  0.02108 measured one rebase earlier, so the bar was re-derived from its own
  forty readings rather than carried across. Recorded in the comment beside the
  bar, per rule 7: it catches about 1.3x and no less, it is blind to anything
  that slows this row and the whole tick equally, and it will red for a large
  speed-up elsewhere in the tick.
  Also corrected here, both from the same review: the budget test's comment
  claimed "30.5x" against its own table's 17.8x and claimed the appetite term
  was "back inside the 0.0112 it read before the merge" when the table says
  0.0226 — twice it, not inside it; and `arsenal::value_order` now asserts every
  `deck_value` is finite, a NaN there being enough to make the ranking
  comparator non-transitive and `sort_by`'s order unspecified.
  Filed and closed: BUGS **P-1** and **P-2**.
  **Suite at ship** (2026-09-02, `cargo test --release --workspace
  --no-fail-fast` after `cargo clean -p spheres-sim -p spheres-web -p
  spheres-cli --release`, isolated target, all three Compiling lines watched and
  the test binaries confirmed to post-date every source): spheres-sim **238
  passed / 3 failed / 22 ignored**, spheres-web **94 / 0 / 2**, spheres-cli
  **1 / 0 / 0**, the five integration targets all-ignored as before. The three
  reds are the expected ones and are untouched —
  `tech::tests::the_1990_endowment_does_not_move_year_one_growth` (BUGS E-3,
  Belgium 0.001851 against 0.001749), `tests::the_1990_start_is_pinned` and
  `tests::golden_hash_of_a_known_run`, whose pins stay at `0xd022d50f43c984da`
  (`lib.rs` 4141) and `0xbd5ec0f43c5f2e3b` (`lib.rs` 4382) and whose actuals are
  unmoved at `0xe26e4bf8d6c60066` and `0xbe94d6125631829c`. Headless
  determinism, `spheres-cli run 35 <seed> | sha256sum | cut -c1-16`, each run
  twice and each pair byte-identical: market OFF `d1a2cfbf7c6958d7` (seed 1990,
  3501 lines) and `39dea3341a7f6e8c` (seed 7, 3983);
  market ON `1574abf65b382173` (3873) and `f97da62d5daee785` (4234).
  **THE TWO MARKET-ON DIGESTS MOVED, AND NOT HERE.** They were
  `30cf39058ba9ae1f` (4110 lines) and `6daccc96382f7659` (3967) through
  `a9a373d`, and this branch reproduced both on that rebase. `867b3d6`
  "feat: add province manufacturing lines" moved them: a pristine `git archive`
  of `867b3d6` built into its own target directory produces
  `1574abf65b382173` and `f97da62d5daee785` — byte-for-byte what this branch
  produces — while both market-OFF digests are unchanged on both sides. So the
  market-on timeline moved with province manufacturing, which consumes from the
  market, and nothing on this branch moved anything; the branch's own diff is
  a test module, a one-off assertion in `arsenal::value_order` that cannot
  reorder anything, and these two documents.
  `tools/resources/check_resources_1990.py --fast`: **60 checks, 0 failed**.

- **Codex's province trade and mines** (2026-09-02, Ridge's own merge `e4e3c03`
  "merge: integrate Codex province trade and mines" of `9274baa` (ours) with
  `3f7eaf2` (`feat: integrate province resources trade and mines`), plus four
  repair commits on `fix/merge-repairs`): living province population — the 1990
  district residents in `spheres-web/data/district_population.json`, 2,610
  provinces, growing with the current owner's demographic and technology path and
  staying with the ground across a border transfer — and the district mine,
  `Command::DevelopResource`, a twelve-month build on a mapped deposit priced at
  `MINE_PC_COST` political capital and an investment charged ~~to `debt_gdp`~~
  **through `economy::charge` — corrected 2026-09-02 by the
  `feat/ministry-economy` merge, which routed this leg and the two spot-market
  arms through the single fiscal helper; the ratio pushed is unchanged, so a
  closed-books nation is bit-identical (BUGS M-2)**.
  **Four repairs on landing**: (1) `2b10e78` — the four `START_ACTUAL` /
  `RUN_ACTUAL` constants in `the_resource_layer_is_inert_at_1990`,
  `the_resource_layer_is_inert_over_time` and
  `the_market_switch_is_off_for_the_suite_and_deterministic_when_on` still
  carried `0xa5c9c5b2306313d8` / `0x20c24ab0f1581807` and were pointed at the
  tree's measured actuals `0xe26e4bf8d6c60066` / `0xbe94d6125631829c`; these
  track the tree's current actual by construction (their own comment at
  `lib.rs` 4338-4340 says so) and are not a golden re-pin. (2) `24b110e` — three
  merge notes the landed merge had falsified, corrected as comments only, the
  earlier reading kept legible as history. (3) `7958ff4` — Codex's two dropped
  province-population guards restored from `3f7eaf2`,
  `opening_population_covers_every_province_and_closes_to_national_totals` and
  `province_population_follows_its_current_owners_demography`, each red-checked on
  both of its arms and each revert recorded; the reconstruction was corrected in
  the doing, because the opening split is **not** renormalised at birth
  (`data/mod.rs` 812 seeds straight from `districts::population_1990()`, so
  `reseed_population`'s renormalising loop is only reached from `load()` for a
  pre-layer save) — blanking that loop left the coverage test green, which is how
  it was found, so the guard is on the committed artifact itself. (4) `104f851` —
  `tools/resources/check_resources_1990.py` failed 2 of 60 `--fast` and 4 of 63
  on the full run (the repair session's reading, taken before its own fix), one
  root cause: JSON line endings. LF chosen as the
  convention and pinned by a repo-root `.gitattributes`, because the checker
  already carried a bar demanding it (CHECK 5, "the committed file uses LF
  newlines only"), because git already stores all 148 of those files with LF so
  the rule changes no blob, and because a byte-hash of a transcribed data file is
  a claim about the DATA (iron rule 4) and must not depend on which OS did the
  checkout. Checker re-run for this record on 2026-09-02 at `104f851`: **60 checks, 0
  failed** (`--fast`) and **63 checks, 0 failed** (full).
  **The pins were kept**: `the_1990_start_is_pinned` stays at
  `0xd022d50f43c984da` (`lib.rs` 4069) and `golden_hash_of_a_known_run` at
  `0xbd5ec0f43c5f2e3b` (`lib.rs` 4310), both deliberately red until BUGS E-3's
  endowment bar is green — but **both actuals moved**, to
  `0xe26e4bf8d6c60066` and `0xbe94d6125631829c`, and **nothing in the simulation
  moved with them**: Codex's `district_population` and
  `district_population_scale` are `#[serde(default)]` with no
  `skip_serializing_if` (`world.rs` 947-952), unlike the ten fields around them,
  so they always enter `state_hash`. Stripping exactly those two blocks and the
  comma their removal orphans from the merged saves at t=0 and t=240 months
  yields text byte-identical to the `9274baa` saves and re-hashes to
  `0xa5c9c5b2306313d8` / `0x20c24ab0f1581807`. See BUGS **M-5** — reverting that
  is a save-format change and Ridge's call.
  Filed, not fixed: BUGS **M-1..M-8** (the mine's five bare constants with two of
  them inert and 96.79% of the board at the price floor; ~~the `debt_gdp` write
  against the ruling at `resources.rs` 66-68, invisible to its own guard because
  that guard ticks with an empty command slice~~ — half answered 2026-09-02 by
  the `feat/ministry-economy` merge, which routed the write through
  `economy::charge` and drove the module's direct naming of `debt_gdp` to zero;
  the doctrine half of M-2 is still open, because the module still has a fiscal
  channel into growth, the header sentence at `resources.rs` 66-68 is
  un-amended, and the guard still ticks with an empty command slice; the player-only mine against
  R-1's zero resource wars; the mine's four lost guards; the serialization above;
  the orphaned `/api/district-populations`; no browser load path; and the daily
  invariant that has never seen `DevelopResource`).
  **Suite at ship** (2026-09-02, `cargo test --release --workspace
  --no-fail-fast` after `cargo clean -p spheres-sim -p spheres-web -p
  spheres-cli --release`, isolated target, all three Compiling lines watched and
  the test binaries post-dating every source). This branch was **rebased onto
  2f9791e `feat: establish conserved resource market`**, which landed on
  `origin/feat/hoi4-map-and-tech` while the ship pass was running; all six
  commits replayed with no conflict, and the figures below are from the rebased
  tree. spheres-sim **209 passed / 4 failed / 22 ignored**, spheres-web
  **89 / 0 / 2**, spheres-cli **1 / 0 / 0**; the five spheres-sim integration
  targets contribute 0 passed / 0 failed / 30 ignored between them.

  Three of the four failures are the deliberate reds, at unchanged actuals:
  `tech::tests::the_1990_endowment_does_not_move_year_one_growth` (BUGS E-3,
  Belgium 0.001851 granted against 0.001749 ungranted), and the two goldens,
  which panic at `lib.rs` 4068 with actual 0xe26e4bf8d6c60066 against the
  untouched pin 0xd022d50f43c984da and at `lib.rs` 4314 with actual
  0xbe94d6125631829c against the untouched pin 0xbd5ec0f43c5f2e3b. **The
  simulation did not move under 2f9791e**: both actuals read exactly what they
  read before the rebase.

  **The fourth red is upstream's, not this branch's.**
  `tests::the_resource_pass_stays_under_budget` fails at `lib.rs` 4742: the
  resource pass costs **1.7819 ms/month against a 0.15 ms bar**, and the cost is
  almost entirely the new appetite term (resources 0.0680, buy pass 0.0424,
  appetite term **1.6714**). It was proved upstream by checking 2f9791e out on
  its own detached worktree, with none of this branch's commits present, where
  the same test fails at 1.5173 ms/month. No commit on this branch touches
  `resources.rs` or that test. **It is a throughput bar, not a correctness one,
  and it needs an owner ruling: profile the appetite term down under 0.15, or
  re-argue the bar for a market that now conserves.**

  Headless `run 35` (sha256, first 16 hex, the convention used above), each run
  twice and byte-identical across the pair: market OFF seed 1990
  **d1a2cfbf7c6958d7** (3,501 lines) / seed 7 **39dea3341a7f6e8c** (3,983) —
  **still equal to 9274baa's**, so the default-off world is untouched by the new
  market. Market ON now reads **30cf39058ba9ae1f** (4,110) /
  **6daccc96382f7659** (3,967), moved from the pre-rebase 6cb6c97ab33fb80d /
  8d29fecfd4ff9bf4 by 2f9791e itself, which is what a conserved-market feature
  is expected to do. Provenance: `check_resources_1990.py` 60 checks 0 failed
  `--fast`, 63 checks 0 failed full.
- **The daily calendar and the ten-ministry annual budget** (2026-09-02, Ridge's
  call — "I like the 10 ministry budget and the 1 day ticker so if the bible needs to be ammended we can do that.";
  `origin/codex/trading-system` 4875ea5 merged as 253ff2d onto
  `feat/hoi4-map-and-tech` 2cc76a6, two hunks resolved by hand): `tick_day`
  steps the playable calendar one Gregorian day at a time (leap days included)
  and settles the same SYSTEMS table once, on the month's last day; commands
  apply on the day issued through the shared `apply_command`; `AnnualBudget`
  with ten ministries (health, education, families, pensions, infrastructure,
  industry and energy, science, defense, security, diplomacy) capped by
  `BUDGET_CAPS`, enacted by `Command::SetAnnualBudget` priced in political
  capital, composing the three aggregates the model has always priced; the
  browser's side columns became drawers (P cabinet with the budget card, E
  intel) with 1/7/30/365-day advance keys and `/api/advance {days}`. All of it
  inert on the default path (`None` arms, `skip_serializing_if`).
  **Three fixes on landing**: (1) `tick_day` no longer clears `headlines` on
  every day — the record is kept for the month, as `tick_month` keeps it —
  which is what turned the new `tests::the_daily_clock_preserves_the_market_on_world`
  (RED at birth: first divergence month 3, daily 0x42003eb0969c6720 against
  monthly 0x1ae2b9b73492d296, the two saves identical once `headlines` was
  stripped) green; (2) `tick_month` resets `day` to 1 at the month boundary
  instead of `min(days_in_month)`, so a mid-month save resumed on the month
  path lands on day 1 after its first settlement; (3)
  `Command::SetAnnualBudget` leaves `social_spend_gdp`, `state_invest_gdp` and
  `mil_spend_gdp` untouched when every enacted allocation is bit-identical to
  the plan in force (the stored plan, or the inherited one when none is), only
  seating the plan — enacting the inherited split unchanged had moved
  `social_spend()` by one ulp for 49/137 nations and `state_invest_gdp` for
  32/137 and flipped `social_spend_gdp` `None` -> `Some` for all 137; asserted
  by `tests::enacting_the_inherited_budget_unchanged_is_a_no_op` (52fd9f6).
  The daily-clock test was then extended (5ebde69) to issue its orders on the
  10th, 20th, 31st and 15th and to assert the per-day returns of `tick_day`
  add up to `tick_month`'s. **The pins were kept**: Codex's re-pin of both goldens to their
  current actuals was reverted (BUGS D-7) — `the_1990_start_is_pinned` stays at
  0xd022d50f43c984da reading actual 0xa5c9c5b2306313d8,
  `golden_hash_of_a_known_run` stays at 0xbd5ec0f43c5f2e3b reading actual
  0x20c24ab0f1581807, red for E-3's reasons and at the same two actuals before
  and after the landing. Doctrine amended in the house style: BIBLE §4 (budget
  row) and §5 (the daily-calendar amendment, with-commands half now asserted),
  SPEC §2 and §3, CLAUDE.md owner preferences. Filed, not fixed: BUGS D-1..D-9
  (the ministry channels, the player-only investment arm, the budget that never
  lapses, the month-only CLI, the headless references moved by `date_str`, the
  page's JavaScript price, the refused re-pin, the deck's claims, the three
  re-targeted web tests).
  **Suite at ship** (2026-09-02, `cargo test --release --workspace --no-fail-fast`
  after `cargo clean` of the three crates, isolated target, binaries post-date
  every source): spheres-sim 197 passed / 3 failed (the
  endowment guard and the two goldens, same actuals) / 22
  ignored, spheres-web 87 / 0 / 2,
  spheres-cli 1 / 0; headless `run 35` market OFF
  seed 1990 sha256 d1a2cfbf7c6958d7 / seed 7 39dea3341a7f6e8c (the trial merge
  read d1a2cfbf7c6958d7 / 39dea3341a7f6e8c, equal to 2cc76a6's
  2409583ac6951b46 / 03fb32b79aaf948b once the day is stripped from every
  date — BUGS D-5), market ON 6cb6c97ab33fb80d / 8d29fecfd4ff9bf4 (trial merge
  6cb6c97ab33fb80d / 8d29fecfd4ff9bf4).
- **The resource system** (2026-09-01, `scratchpad/resourcesys/SPEC-RESOURCE-SYSTEM.md`,
  seven commits 1744e0c..HEAD): twelve commodity lines that stay separate; a
  1990 ledger derived from transcribed national production apportioned to
  located districts (`spheres-sim/data/resources_1990.json`, generator and
  63-check gate under `tools/resources/`); a stockpile gate at the arsenal
  that binds ONLY when a gated line asks for what the pile cannot feed (no
  ambient drag: growth, oil, stability and munitions never read it, and
  `gates_write_nothing_the_growth_model_reads` proves it over 40 y x 6 seeds);
  negotiated trade — `ProposeDeal / AcceptDeal / DeclineDeal / CancelDeal`,
  one pure `evaluate()` that accepts, counters with a price, or refuses with
  one of twelve fixed sentences, contracts that settle money on `debt_gdp` and
  land through `transfer_district`; the arcade surface (key B, the folded
  board, three cards a line, three-click talks with the sim's answer printed
  before the click, the globe's tint / contract arcs / aim ring, the dossier's
  "What it holds") with no coefficient in the page; and, behind
  `GameRules::resource_market`, the AI buy pass, the refusal memory, the
  sanction ration and the five-clause last-resort war (`dyads::last_resort`,
  a {0,1} term worth exactly `RESOURCE_WORTH` 0.75).
  **Fork F1 was called (b)**: the market ships OFF in every test and every
  headless path and ON for every browser game (`play_rules`, at boot and on
  `/api/new`). Why: the protocol re-pin is blocked by BUGS E-3, not by this
  system, and (b) is the only way to land S3 while both goldens keep their
  actuals — `the_1990_start_is_pinned` 0xa5c9c5b2306313d8 and
  `golden_hash_of_a_known_run` 0x20c24ab0f1581807, red for the known reasons,
  reproduced byte-for-byte by every one of the seven landings; nothing
  re-pinned. The cost is filed (BUGS R-8): until T-5 / E-3 close, the
  calibration suite is blind to the world the player actually plays.
  **D2 landed as data (fork F3, now)**: the six presence-only lines carry 1990
  national figures with their citations — cobalt 13 rows (USGS MYB 1990 Table
  14), gold 58 (Table 15), phosphate rock 33 (Table 30), platinum-group 11
  (Table 15), rare earths 9 (Table 12), uranium 22 (OECD/NEA-IAEA Red Book,
  Table 6) — located by the same site-count rule (9 / 49 / 30 / 9 / 4 / 11
  nations located), priced from the same sources, contractable on the board.
  The USSR holds its 14,000 tU through the unlocated-producer rule (a survey
  hole, asserted). NOT landed: spec §7.2's consumers — the reactor / magnet /
  battery / hydrogen research gates and the Bomb's uranium gate — so those six
  lines have HAVE and contracts but no NEED and read IDLE ("nothing in this
  build draws on it"). Figures with no roster seat (Namibia's copper 27,800 t,
  gold 1,700 kg, uranium 3,211 t; Burkina Faso, Mali, Guinea gold; Togo, Nauru
  phosphate; Niger uranium) are kept in `meta.transcription` and seated nowhere.
  **Suite at ship** (forced rebuild, isolated target, binaries post-date every
  source): spheres-sim 188 passed / 3 failed / 22 ignored (the endowment guard
  and the two goldens, same actuals), spheres-web 83 / 0 / 2, spheres-cli
  1 / 0; headless `run 35` on seeds 1990 and 7 byte-identical to the
  pre-system tree with the market off (sha256 2409583ac6951b46 /
  03fb32b79aaf948b) and byte-identical run-to-run with it on
  (3f97191a92733a42 / 550fd1473d00c898). Profile: the whole layer with the
  market on 0.039 ms/month at 137 nations against a 0.05 bar. Forks F2 and F4
  are NOT decided — see §1c under Next.
- **Terrain pass** (2026-08-30, BIBLE §5 as amended): every district carries a
  transcribed Natural Earth terrain class (`t`), feature name (`f`) and
  river-crossed neighbour subset (`riv`), merged into districts.json by mapgen
  from the new one-shot generators in `tools/terrain/` (run only when the data
  needs regenerating — see its README). In the sim, the per-district class
  supersedes the theatre `rough` scalar inside the front's capped phase — six
  tempo constants plus a river shave and a floor, distribution and tempo only;
  the uncapped sweep that glues the aggregate to the legacy control equation
  is untouched. The map gains a baked hillshade underlay (`/terrain.png`,
  multiply-blended under the district layer) and the 263 major rivers + 29
  lakes (`/rivers.js`), both served from the binary like world.js. The 66°N
  latitude-band override in the classifier fixes RU-YAN (Yamal-Nenets) to
  tundra; golden hash re-pinned and the conquest seed pair re-scanned to
  seed 9 per their own comments
- **UI restyled toward HOI4** (2026-08-30): setup screen rebuilt as a searchable,
  sortable, always-startable picker; a political map mode (curated colors for the
  majors, deterministic muted fallback for the rest) is now the default shading,
  with dark ocean, thin dark borders and a gold selection ring; chrome moved to
  steel-and-brass — beveled buttons, resource-strip header with a recessed date
  plate, brass-etched card headers and legend plaque. Pure look-and-feel: no ids,
  no API, no game logic touched; TERRITORY/LABEL_AT byte-identical
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
- **Technology tree**: 328 technologies across eight domains plus a foundation set.
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

## Fixed: the century run was never about the technology tree

**The suspect was cleared and the actual cost was somewhere nobody had looked.**
The unprofiled guess in this section's previous version was `tech/mod.rs`, on
the strength of `absorptive_capacity` being O(n²). It is O(n²), and it is not
the problem: the whole technology tree was 0.78ms of a 5.67ms month at 137
nations, and its cost grows *sub*-linearly with the roster — 30 nations to 137
is 4.6x the nations for 2.9x the tech time, which is the opposite of what an
O(n²) term that mattered would do. Third time this
project has been handed a plausible cause that was a coincidence, and the first
time the profile ran before the rewrite instead of after.

**Where a month actually went, at 137 living nations, per subsystem, best of
three 1200-month passes** (`century_run_profile`, below):

| subsystem | before | after |
|---|---|---|
| politics | 3.938 ms | 1.120 ms |
| tech | 0.778 ms | 0.701 ms |
| government | 0.438 ms | 0.357 ms |
| economy | 0.120 ms | 0.101 ms |
| ai_stratagems | 0.162 ms | 0.083 ms |
| statecraft | 0.146 ms | 0.067 ms |
| war | 0.048 ms | 0.023 ms |
| stratagems | 0.041 ms | 0.042 ms |
| **whole month** | **5.67 ms** | **2.48 ms** |

Politics was seven tenths of the tick, and inside it two things:

1. **`Relations::pairs_mut` was O(n³) in the width of the roster.** It recovered
   the row and column of each slot by counting up from zero, for every one of
   the n(n+1)/2 slots — 1.36 million loop iterations per sweep at 160 wide, once
   a month. This is the whole of the super-linearity, and it is why it hid: the
   cost is set by how wide the matrix is, not by how many nations are alive, so
   the roster growing from 108 rows to 160 more than doubled it while every
   per-nation loop grew by a quarter. Carrying (row, column) forward makes one
   sweep O(n²). Measured: 1.569s -> 0.054s over a century. **29x.**
   `Relations::serialize` had the same walk, which put it in every determinism
   test and every `state_hash`.
2. **`dyads::war_appetite` asked its questions expensively.** It is called for
   all 2,143 contact dyads every month; each call built two `String`s with
   `format!` to ask `has_flag` about `pressed_A_B` and `burned_A_B`, and did
   about eight linear scans of `nations` (three of them per major power, through
   `would_intervene`). The flags list reaches 260 entries by 2090, so the cost
   also grew with elapsed time. Now: `has_pair_flag` builds the key in a stack
   buffer, and `WorldState` carries a non-serialized id -> position index that
   makes `nation`/`nation_mut`/`nation_opt` O(1). ai_wars: 3.27s -> 1.60s.

**The 0.744 / 2.93 / 12.4 curve was not a scaling curve.** Those three numbers
were taken on three different commits months apart, so the roster was not the
only thing that changed between them — government, the commitment ladder and
statecraft all landed in the same window. Measured properly, on one binary, by
retiring nations from a full-width world:

| living nations | before | after |
|---|---|---|
| 30 | 2.758s | 0.648s |
| 108 | 5.262s | 2.165s |
| 137 (all) | 6.807s | 2.974s |

0.0216, 0.0200, 0.0217 seconds per nation after: flat. The curve is linear now,
and the thing that was bending it is gone rather than reduced.

**End to end**, headless CLI, CPU time, best of five: 35 years 3.66s -> 1.92s;
100 years 8.03s -> 4.64s. The CI cost this section was actually written about:
`a_century_holds_together` 27.0s -> 12.2s, and the whole sim suite 40.3s ->
18.5s with four more tests in it.

**Nothing moved.** Both pinned hashes hold, all 95 pre-existing tests stay
green, and the entire headline stream plus the closing league table is
byte-identical against master at 279414c for seeds 1990, 1, 7 and 42 over 35
years and for seed 1990 over 100. That is the only acceptable outcome for a
refactor, and it is checked by diffing the runs, not by assuming.

**What is left, named and measured rather than guessed.** `ai_wars` is still
the largest single item at ~1.1ms of the 2.48ms month. What remains in it is
the appetite pass itself: 2,143 dyads a month, each allocating a `Vec` in
`pact_partners` and re-deriving reach and disposition that did not change since
last month. Nobody should touch it without re-running the profile first.

**The instrument stays.** `century_run_profile` in `spheres-sim/src/lib.rs` is
`#[ignore]`d:

    cargo test --release -p spheres-sim --lib -- --ignored --nocapture profile

It times the `SYSTEMS` table that `tick_month` runs, so it cannot drift out of
sync with the tick, and it reports the best of three passes because this machine
returns anywhere from 6.8s to 11.4s for the same binary on the same run — a mean
here measures the other processes on the box.

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

## Stability audit

`BUGS.md` holds the findings from the standing stability pass, and the survey
behind it is reproducible:

```bash
cargo test --release -p spheres-sim anomaly_sweep -- --ignored --nocapture
```

Twenty-one seeds, forty years, every living nation checked every month. It found
**no numerical anomaly of any kind** — no NaN, no negative or runaway GDP, no
debt spiral, nothing pinned to a clamp. One structural finding is open under
"Needs design" there: a conflict that flares more often than every eighteen
months can never reach any of the ladder's four exit conditions, so seed 0 still
carries Iraq/Kuwait as an open quarrel at 478 months in 2030. It is not a war —
shooting in 22 of 479 months — and applies no drag, which is why the numbers stay
clean.

## Next (rough priority)

### 0-ter. STATE OF THE SUITE after Ridge's three rulings, 2026-08-31 — supersedes 0-bis below

0-bis stands as history. These are the numbers after the capital-channel repair
(ruling 1), the conquest and Gulf-War re-pointings (ruling 2) and the sampling
doctrine now in CLAUDE.md as iron rule 7 (ruling 3). Every source file was
touched at 19:26:17 and every test binary rebuilt and watched afterwards
(19:26:31–19:26:34, the CLI at 19:26:54) — iron rule 6, and stated as a range
because a touched-source rebuild makes "post-dates" checkable rather than
assumed.

```
spheres-sim  --lib ............ 152 passed   3 failed   19 ignored   (218.2s)
capital_damage_audit .......... instruments only         5 ignored
endowment_margin_probe ........ instrument only          1 ignored
growth_decomposition .......... instruments only        20 ignored
sample_size_audit ............. instruments only         3 ignored
spheres-cli ................... no tests
spheres-web ................... 17 passed    0 failed
```

**The three reds, quoted:**

```
tech::tests::the_1990_endowment_does_not_move_year_one_growth
  Belgium was paid twice for its 1990 technology:
  growth 0.001851 granted against 0.001749 ungranted            tech/mod.rs:2216

tests::the_1990_start_is_pinned
  the 1990 start state changed (actual 0xa5c9c5b2306313d8)      lib.rs:3186

tests::golden_hash_of_a_known_run
  timeline fingerprint changed (actual 0x20c24ab0f1581807)      lib.rs:3421
```

**2026-09-02, the codex/trading-system landing:** both goldens remain
deliberately RED at the same two actuals — `the_1990_start_is_pinned`
0xa5c9c5b2306313d8, `golden_hash_of_a_known_run` 0x20c24ab0f1581807 — before
and after the merge, and Codex's re-pin of them to those actuals was reverted
(BUGS D-7: no schema expansion reaches the default-path save, and E-3 is still
red). Nothing is re-pinned until every calibration bar is green.

`spheres-web the_map_ships_terrain_and_rivers` is **green (17/17)**, but that
reading belongs to the concurrent map/tech swarm, which has `spheres-web/src/main.rs`,
`spheres-web/ui/index.html`, `ui/relief.png` and `tools/terrain/` modified in this
same checkout. It is reported here as theirs, not claimed as a fix.

**THE THREE REDS 0-bis LISTED ARE GONE**, and the honest reading of that is
mixed: `gulf_war_emerges` and the two conquest tests are green partly because the
board grows faster under the capital repair and partly because ruling 2 gave them
samples that can see their own events. A bar that is green today for an unrelated
reason is exactly the inverted-signal problem, which is why they were re-pointed
rather than left alone.

**THE HEADLINE RESULT — NOT REGRESSED, and the max error improved:**

```
  nation   model    real   error        Spearman rho vs reality   0.886  (10 seeds)
     USA    2.01    2.50   -0.49                                  0.886  (40 seeds)
   Japan    1.32    0.83   +0.49        USA strictly fastest       true
 Germany    1.78    1.28   +0.50        Japan below USA/UK/FR/DE   true
  France    1.86    1.50   +0.36        Italy below USA/UK/FR/DE   true
      UK    1.82    1.93   -0.11        Germany < UK               true
   Italy    1.60    0.76   +0.84        max |error|         0.86 -> 0.84
```

Germany < UK is the clause this change costs the most and it holds with room:
margin +0.04 at ten seeds, +0.030 at a hundred, and P(UK faster than Germany) =
0.93 over the 40-seed pairwise matrix. The whole capital channel pays the six
between −0.02 and +0.04 pt/yr — the rate arm is gated to zero at the frontier and
the level write is exactly zero on five flat 420-month investment shares.

**THE DEVELOPING PANEL, 100 seeds, 30-year multiple and CAGR:**

```
     nation   30y mult   30y CAGR   35y CAGR   real 35y      verdict
      China    14.4549     9.3118     9.0784    8.70(e)      FIXED
      India     9.7191     7.8750     7.7649    6.39(e)      better
 SouthKorea     5.4483     5.8137     5.2322    4.53(e)      worse  0.15
     Poland     4.2353     4.9292     4.7700    2.94         better
     Brazil     3.6505     4.4107     4.2995    2.24(e)      ~flat
    Nigeria     6.2352     6.2906     6.0820    4.37(e)      better
  Indonesia    24.3907    11.2348    10.6466    4.88(e)      WORSE  1.50   BUGS.md C-2
    Vietnam     5.0182     5.5241     5.9062    6.88(e)      WORSE  0.49   input defect
```

China across **300 fair seeds**: median 30-year multiple **14.6940x** against a
real 14.33x, mean 14.3077x, p05 11.380, p95 16.074, min 8.653. **10 of 300 below
the old 11.0 floor (was 140 of 300); 173 of 300 reach reality (was 0); 0 of 30
disjoint ten-seed blocks would red (was 11 of 30).** `china_growth_miracle` now
passes with margin instead of on a lucky draw.

**Emergent history holds**, seed 7, thirty years: Yugoslavia dissolves Dec 1991
and the USSR Sep 1993; seven wars open and five close by negotiated peace with
territorial cession (Bosnia 1995, Laos 2000, Guatemala 2001, Yemen 2001 and
2011); the Gulf War emerges in **246 of 400 seeds (61.5%)** against a doctrinal
50% bar and 125 of the 200 the test now reads; conquest fires **107 times in 240
seeds** across 78 seeds, of which 10 are annexations (Mongolia ×6, Bhutan,
Luxembourg, Ireland — largest 5.351m against the 8m bound) and 97 are refusals.
The 2020 league table reads USA, **China**, Japan, India, Germany, Indonesia, UK,
France, Russia, Italy — **China overtakes Japan, which HEAD did not manage**, and
Indonesia climbing to 6th is the repair's cost made visible (BUGS.md C-2).

**Determinism holds:** two 30-year headless runs byte-identical at 256,881 bytes,
md5 `f8ba3471388bfcf2a7456d0229ec4ed4`, confirmed with `cmp` as well as the hash.

**THE GOLDEN RE-PIN IS STILL BLOCKED, and the blocker changed.** It is now
`the_1990_endowment_does_not_move_year_one_growth` — BUGS.md **E-2**, red at
102.2% of a bar it cleared at 98.8% on HEAD, with the capital repair proved to be
the sole cause by isolating `economy.rs` onto a pristine HEAD worktree. Half the
precondition is met and was re-verified independently by brace-matched body
extraction against `git HEAD`: **zero bodies deleted, zero tolerances widened,
one ceiling added**. Both pins keep the HEAD constants; `the_1990_start_is_pinned`
reads bit-for-bit the same actual on both trees, so HEAD ships a 1990 board that
does not match its own 1990 pin, independent of this work. Full second
adjudication in **BUGS.md T-5**.

**Three things the next pass must know before it starts.** `mature_economies_do_not_run_hot`
still has Italy at **+0.0008** above its 0.008 floor (it moved the right way, from
+0.0007). `growth_decomposition.rs`'s `terms()` still computes the OLD capital
formula and every decomposition column built on it is wrong by the size of the
repair (**BUGS.md T-6**). And the Spearman/ordering result above is printed by an
`#[ignore]`d instrument and **asserted nowhere** — nothing in `cargo test` can
catch a regression in it (**BUGS.md T-7**).

### 0-bis. STATE OF THE SUITE, 2026-08-31 — supersedes §0 below

Everything in §0 still stands as history; these are the numbers as of the audit
pass. Binaries rebuilt from scratch with every test `.exe` post-dating a full
source touch (iron rule 6).

```
spheres-sim  --lib ............ 150 passed   5 failed   18 ignored
growth_decomposition .......... instruments only        20 ignored
spheres-cli ................... no tests
spheres-web ................... 16 passed    1 failed  (the swarm's, see below)
```

`spheres-web the_map_ships_terrain_and_rivers` is red and belongs to the
concurrent map/tech swarm; it is not counted against this work.

**What landed since §0 was written**, in order: the 1990 technology endowment
and its data validation; the adoption rebase that made the endowment neutral
(BUGS.md E-1); the economy fix converting `invest_effect`, `labour` and
`demand_gap` from a permanent RATE to a one-time LEVEL; PLAN step 7's symmetric
`MAX_DEMAND_GAP` and the four sanction channels converted from counting flags to
weighing output; the diffusion-knee repair (`BUILD_KNEE` 0.004 → 0.008); the
trade-pact test's construction repair; and the transition collapse
(`money_works`, plus successors inheriting `capital_level_paid`).

**The headline result, and it is the thing to protect:**

```
  nation   model    real   error        Spearman rho vs reality   0.886  (10 seeds)
     USA    2.02    2.50   -0.48                                  0.886  (40 seeds)
   Japan    1.35    0.83   +0.52        USA strictly fastest       true
 Germany    1.82    1.28   +0.54        Japan below USA/UK/FR/DE   true
  France    1.91    1.50   +0.41        Italy below USA/UK/FR/DE   true
      UK    1.89    1.93   -0.04        Germany < UK               true
   Italy    1.62    0.76   +0.86        max |error|                0.86
```

Emergent history holds: Yugoslavia dissolves in Dec 1991 and the USSR in Sep
1993; nine wars open over 35 years and five close by negotiated peace with
territorial cession; the 2025 league table reads USA, China, India, Japan,
Germany, Russia, Mexico, France, Indonesia, Italy, UK. Determinism holds: two
30-year headless runs are byte-identical at 257,583 bytes,
sha256 `96d75860d2d15bf2f47dc5eb422004caf78c51dc4a3cd7045257594e8d3395dc`.

**Two green tests are on a knife edge and the next pass must know before it
starts.** `china_growth_miracle` reads a median of **11.32x against a band floor
of 11.0** — 0.32x of margin, with 4 of 10 seeds individually below the floor.
`mature_economies_do_not_run_hot` has an Italy floor margin of **+0.0007**
(growth_last 0.0087 against a floor of 0.008) on seed 42. Either can be turned
red by a change in any direction.

**Five red, none of them a moved bar, and the golden re-pin is BLOCKED.** The
two goldens plus `gulf_war_emerges` (18/40 against 20) and the two conquest
tests (both on their non-vacuity guard, not their bar). The re-pin was
adjudicated formally and refused — see **BUGS.md T-5**, which carries the
byte-comparison against `git HEAD` proving zero tests deleted and zero bars
moved, and names every mechanism a future re-pin must cite.

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

### 1c. The resource system's two owner forks, F2 and F4 — measured twice, decided by nobody

The spec pre-registered a band for resource-motivated wars, λ ∈ [0.05, 0.69]
per seed over 480 months, PROVISIONAL until measured twice independently, and
left two forks to the owner: **F2**, the temperature (relation floor
{−20, 0, +10} × sanction ration {on, off}; the design cell is −20 with the
ration on), and **F4**, the census bar and its seed count, to be set from the
measured rate by iron rule 7's arithmetic. Both counts are in; here is what
they say, so the pick is from data.

**Count one** (`resource_war_census_one`, in-process, 200 seeds 0..199 × 480
months × 7 arms, one thread per arm; `scratchpad/resourcesys/census1*`) and
**count two** (out-of-process CLI runs of a `git archive` of 242d178 with a
measurement patch that reads the two knobs from the environment; 400 seeds in
two blocks, A = 0..199 and B = 1000..1199; `scratchpad/resourcesys/c2_census.txt`)
agree on every shared number: block A reproduces count one's per-arm "WAR:"
totals line for line (control 1,686; −20/on 1,552; 0/on 1,557; +10/on 1,547;
+10/off 1,676), block B is the independent draw, and the control arm is
byte-identical to the pre-S3 f03668e binary on 400/400 seeds with the 12 × 30-year
readout 6,6,11,10,10,6,7,7,8,11,9,7 / 98.

```
resource wars per seed, {count: seeds}         count one (n=200)     count two (n=400)
  floor -20 · ration on  (the design)          {0: 200}  λ 0.000     {0: 400}  λ 0.000
  floor -20 · ration off                       {0: 200}  = control   {0: 400}  = control
  floor   0 · ration on                        {0: 200}  λ 0.000     {0: 400}  λ 0.000
  floor   0 · ration off                       {0: 200}  = control   {0: 400}  = control
  floor +10 · ration on                        {0: 200}  λ 0.000     {0: 400}  λ 0.000
  floor +10 · ration off                       {0: 200}  λ 0.000     {0: 400}  λ 0.000
  control (market off)                         {0: 200}              {0: 400}
95% upper bound on λ (0 of n):                 0.015 / seed          0.0075 / seed
```

The two rates agree within their own sampling error trivially — both are zero
with zero variance, the intervals nest, and a two-sample comparison has no
events to compare. Over the 400 distinct seeds the rate is λ̂ = 0.000 with a
one-sided 95% upper bound of 0.0075 resource wars per seed per 480 months —
BELOW the band's floor of 0.05 in every cell, by at least a factor of six.

**Where the chain stops** (the clause funnel, count two): in the design cell
233/400 seeds stall a line (copper, every time: Iraq 83 seeds, Vietnam 64,
Nicaragua 37, Syria 32, Ethiopia 18, Libya 12, Pakistan 11, Croatia 9; 71,677
stall line-months), every seed carries refusal rows, and the UNIVERSAL refusal
never completes — the deepest set is 17 of 59 copper producers (Malawi) — so
`last_resort` is never `Some` and the expected count is 0 by construction.
At floor 0 the deepest set is 32/59 (Pakistan). Only at +10 does clause 2
complete (26 seeds with the ration, 19 without; Laos, Cambodia, Thailand,
Taiwan — 59/59 and 60/60) and `last_resort` return `Some` (970 / 1,142
nation-months); the appetite roll then never lands, with p ≤ 0.0014 a month
at 95% and a summed expectation of about 0.024 wars over 200 seeds in count
one. Over 59 copper producers the world always sells or prices out.

**F2, the pick.** Every cell reads zero; the difference between them is how
far the chain gets, not how many wars come out. −20/on (the design) is the
honest choice as shipped — the market is live, 233/400 seeds stall and buy,
and the AI never invades for a mine. +10 is "hot in every seed" (238,466
refusals over 400 seeds) and still zero. Ration off at −20 and 0 is
bit-identical to control: without the ration an open world never has a
shortfall. If the owner wants ruling 4's "not never", no cell delivers it;
the knobs that would are named in BUGS R-1 and each is its own ruling.

**F4, the proposal (not set).** Rule 7's "at least once" arithmetic is
n = ln(0.01) / ln(1 − p) with p the measured per-seed probability of ≥ 1
resource war. Checked against the spec's own table (p 0.2 → 21 / 39 / 55 for
floors ≥ 1 / 3 / 5; 0.3 → 13 / 25 / 35; 0.4 → 10 / 18 / 25). At the measured
p = 0.000 there is no finite n for any floor ≥ 1; at the 95% upper bound
0.0075 it is 612 seeds, and at +10's chain expectation (about 1.2e-4) it is
38,375. So the only bar the measurement supports is the one §9.8 calls
decorative: **0 resource wars in 200 seeds × 480 months in the shipped cell**,
variance 0, false-red 0 under the measured p — a guard that the chain has not
been silently opened, not a calibration bar. Proposed wording for the comment
beside it: "measured 0/200 (count one) and 0/400 (count two); p-hat = 0, upper
bound 0.0075; n is the census's own 200; this bar cannot see a rate below
0.023 (the 1%-power floor at n = 200) and is not meant to". A real F4 bar
waits on an F2 cell with a non-zero rate.

### 2. The tree is invisible
328 technologies and the browser UI does not mention one of them. The owner's
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

**Every worktree shares one cargo target directory — fixed, but stay careful.**
`.cargo/config.toml` is untracked now, so a worktree no longer inherits
`target-dir = C:/Users/ridge/.cargo-target/spheres` from the repo. The hazard it
created is worth remembering: combined with OneDrive resetting mtimes, cargo
would happily serve a test binary built from another branch's source — green
tests that never ran your code, and it produced two wrong readings in opposite
directions before anyone caught it. Still set `CARGO_TARGET_DIR` per worktree,
and if a result looks impossible either way, check the binary before you believe
it. `.gitignore` now carries `target*/` so those directories cannot be committed
by accident.

OneDrive also holds locks on `.git/worktrees/*` and the worktree directories, so
`git worktree remove` and `git worktree prune` fail with "Permission denied" and
the branches those worktrees hold cannot be deleted. Pausing sync should clear it.

**Run the suite with `--no-fail-fast` (2026-09-02).** While the three
deliberate reds stand, `cargo test --release --workspace` **short-circuits**: it
runs spheres-cli and spheres-sim, and because the spheres-sim lib target fails it
stops before spheres-web ever executes, ending with `error: 1 target failed:
`-p spheres-sim --lib``. A run like that silently reports nothing at all for
spheres-web, which is easy to misread as a pass. Add `--no-fail-fast` and every
target runs; that is how the ship tally above was taken, and it is the reason
the earlier repair pass had to invoke `-p spheres-web` separately to get its
88 / 0 / 2. Until BUGS E-3 and the two goldens go green, treat a bare
`--workspace` figure as incomplete.

**JSON in this repo is LF, pinned by `.gitattributes` (2026-09-02).**
`tools/resources/check_resources_1990.py` byte-hashes the transcribed sources it
regenerates `spheres-sim/data/resources_1990.json` from, and on a Windows
checkout half of those hashes were CRLF-worktree digests and half were LF-blob
digests — 2 of 60 `--fast` checks and 4 of 63 full checks failing for that one
reason. The repo-root `.gitattributes` now carries `*.json text eol=lf`. It
changes no blob (git already stored all 148 of those files with LF), so
`git diff --numstat -- '*.json'` over the rewritten worktree is empty; what it
changes is what a checkout writes to disk. **Expect the first checkout after this
lands in any tree — including `C:/Users/ridge/Spheres` — to rewrite its JSON
files from CRLF to LF.** That is the intended effect and produces no git diff,
but the bytes on disk do change, so nothing should be holding one of those files
open mid-write. If that pair of provenance checks ever fails together again, look
at line endings first; the note is repeated in the checker's own docstring.
