# Civilian-control guard for sourced Army leverage (S27 D2)

Task: `CODEX-S27-A1-01`, direction D2. Prepared 4 October 2026 by **Claude, automated agent - not
human or Codex review**, at the user's request. Base: integration tip
`39369f0ee6451466d03778d0273a7f72aafcf8c0`. Change commit: `51a7ddfad9fccad821802d8aa4a5380b1ab373d7` (tree `011baaa1…`) on the
local branch `claude/civilian-guard-wip` in a sparse linked git worktree of the main checkout
(`scratchpad/wt-d2`; git-dir `.git/worktrees/wt-d2`). **It is not pushed and not integrated.**

**Decision source: the user (design authority), 4 October 2026, D2 "Docs: restore the guard".**
[`docs/political-arm/2026-09-22-calibration-repairs.md`](../../../../political-arm/2026-09-22-calibration-repairs.md)
is authoritative: civilian control at authoritarianism .20 or below blocks the Army's
civilian-confidence channel. The code must apply that guard even when sourced V-Dem leverage exists.
This packet implements that decision. It does not re-decide it, and the decision stands whatever any
gate reads.

**What this does not do.** It does not repair A1, and A1 still fails exactly as before. It closes
neither `CODEX-S27-A1-01`, S27, G5 nor CP1. It changes no coefficient, threshold, cohort, seed, data
file or test gate, and `spheres-sim/tests/bloc_census.rs` is byte-identical. The D1
civilian-authority-dispute contract is a separate deliverable and is not in this packet.

## 1. Which channel the documented guard covers

- **Line 87.** "This channel is zero below `.25` discontent, ... or with the lens off. Civilian
  control at authoritarianism `.20` or below blocks it." Here "it" is "this channel": the Army
  **civilian-confidence penalty**, whose formula appears just above.
- **Line 306.** "That boundary suppresses the civilian-confidence penalty". This names the same channel.
- **Iteration-20 code comment** at `government.rs:8503`: "Known historical leverage replaces the old
  authoritarianism proxy only in this political channel." The proxy
  `clamp((authoritarianism − .20)/.40, 0, 1)` was the only place the .20 boundary lived. Sourced
  leverage therefore bypassed it (`civilian_army_executive_leverage`, `government.rs:8508-8513`).

**Guarded:** `army_civilian_confidence_penalty`. Its two runtime consumers inherit the guard with no
edit of their own:
- the Army line of `pillar_targets` (`government.rs:8723/8725`);
- the AI appropriation inverse `ai_army_funding_floor` (`government.rs:9408`).

**Deliberately not changed:**
- **`army_programme_veto_loyalty` (`government.rs:8591`).** The record describes it as a separate
  term at lines 107-117: "civilian control ... resists that term"; "the `.35` loyalty threshold,
  monarchy exception, and authoritarianism guard remain". Its only runtime consumer is
  `annulment_check`, which already returns `None` below `ANNULMENT_AUTH` .35. The veto therefore has
  no runtime effect at or below .20 with or without this change (control 5 in section 3). Whether the veto
  should also carry the .20 guard explicitly is a reviewer question. It is runtime-inert either way.
  (`plan.json`, written before the change, cites this passage as lines 105-113. The exact span is
  107-117. The plan is left as written.)
- `civilian_army_executive_leverage` and its unknown-assessment proxy.
- The `army_authority` source and live records. The guard only reads them; it never rewrites them.
- The test-only A1 observer readout `executive_leverage`.

## 2. Exact change

[`change.diff`](change.diff) is the full commit diff. Runtime part, in `spheres-sim/src/government.rs`:

```rust
const CIVILIAN_CONTROL_AUTHORITARIANISM: f64 = 0.20;   // names the existing documented boundary
...
pub fn army_civilian_confidence_penalty(w: &WorldState, id: NationId) -> f64 {
    let leverage = civilian_army_executive_leverage(w, id);
    if leverage <= 0.0 { return 0.0; }
    if w.nation(id).authoritarianism <= CIVILIAN_CONTROL_AUTHORITARIANISM { return 0.0; }   // new
    ...
```

- The boundary is inclusive, ".20 or below", which matches the proxy's zero at exactly .20.
- It reads the nation's **current** authoritarianism. A nation whose authoritarianism later rises
  above .20 is charged again from its unchanged sourced leverage.
- The `.20` is the boundary already present in the proxy expression. It is not a new coefficient.
- Doc comments record the rule.

The commit also contains:
- A new focused test file, `spheres-sim/src/government_civilian_guard_tests.rs`. It is a `cfg(test)`
  child module of `government`, following the convention of `government_opening_tests.rs`.
- One updated existing test assertion (section 4).

The plan was written before any runtime edit: [`plan.json`](plan.json).

## 3. Red first, then green

**Red** ([`red.log`](red.log)) ran on the unmodified runtime with only the tests added. The test-only
diff at that stage is [`red-stage.diff`](red-stage.diff). Result: exit 101, 5 passed, 1 failed.

`sourced_leverage_does_not_bypass_civilian_control_at_or_below_point_twenty` reads the affected
nations from the opening roster. It selects electoral nations with an Army, sourced leverage above 0
and authoritarianism ≤ .20; none is hard-coded. It stages the existing sustained-crisis fixture:
- discontent ≥ .25, asserted;
- a 24-month civilian record at performance −.80.

Nine nations were still charged. Solomon Islands is in the leverage catalogue at .12/.25 but is not
selected. The Army coverage receipt classifies it `source_unknown`, so its 1990 state has no Army
pillar and no seeded leverage record.

| Nation | Auth | Leverage | Penalty | Army target vs material | 48-month Army loyalty vs proxy twin (pressure 0 unless shown) |
|---|---|---|---|---|---|
| Uruguay | .12 | .6 | .323375 | .273868 vs .597243 | .276 / **1.5** vs .598 / 0 |
| PNG | .15 | .5 | .269479 | .580521 vs .85 | .581 vs .828 |
| Brazil | .20 | .375 | .202109 | .647891 vs .85 | .648 vs .828 |
| Venezuela | .18 | .286 | .154142 | .695858 vs .85 | .691 vs .828 |
| Argentina | .15 | .2 | .107792 | .742208 vs .85 | .732 vs .828 |
| Jamaica | .15 | .2 | .107792 | .742208 vs .85 | .732 vs .828 |
| Botswana | .20 | .2 | .107792 | .742208 vs .85 | .732 vs .828 |
| Spain | .13 | .167 | .090006 | .624490 vs .714496 | .625 vs .707 |
| France | .12 | .1 | .053896 | .796104 vs .85 | .780 vs .828 |

The AI appropriation floor was also higher than the proxy twin's in seven of the nine. In this
staged fixture on the unmodified runtime, Uruguay's army walks below the .35 line with capped coup
pressure. That is the A6 tripwire noted by the scout.

**Green** ([`green.log`](green.log)) ran after the change: 6/6 pass. The five controls passed on both
runtimes:
1. **Above the boundary, sourced leverage keeps its penalty.** This covers native roster nations above
   .20 (including Pakistan) and the nine nations raised to .21 and .45. The penalty equals the formula
   at the sourced leverage, not at the proxy.
2. **The unknown-assessment proxy is unchanged.** Leverage is 0 at .12 and .20, .025 at .21, .5 at .40
   and .975 at .59, and the penalty equals the formula.
3. **Lens off reads no confidence channel.** Leverage is 0, the penalty is 0 and there is no AI floor.
   The Army target is the original share arm verbatim. Save bytes and RNG are unchanged.
4. **The guard is a pure read.** The `army_authority` record, RNG and save bytes are unchanged, and
   save/load gives the same reading. Raising authority to .45 restores the sourced penalty.
5. **The programme-veto consumer stays closed under civilian control.** With sourced leverage 1.0,
   `annulment_check` on the staged Algerian state returns the FIS winner at .58 and `None` at .20
   and .12. (The test reads the check only; no annulment is executed.)

## 4. Existing tests and the one updated assertion

| Run | Runtime | Result | Log |
|---|---|---|---|
| `--lib -- government army_authority stratagem blocs --skip civilian_guard` | unmodified | 161 passed, 0 failed, 1 ignored | [existing-before](logs/existing-before.log) |
| `--lib -- government army_authority stratagem blocs` | changed, before the test update | 166 passed, **1 failed**, 1 ignored | [existing-after-before-test-update](logs/existing-after-before-test-update.log) |
| same | changed, after the update | **167 passed, 0 failed, 1 ignored** | [existing-after](logs/existing-after.log) |

The first run skips the new module; the later two include it. The 167 is the same 161 pre-existing
tests plus the 6 new `civilian_guard` tests, not 6 additional pre-existing tests.

The ignored test is the opt-in `a1_seed0_exact_firing_diagnosis` observer. The set includes:
- the Algeria chronology tests `a_paid_army_seats_the_islamists_and_a_hostile_one_annuls_the_algerian_way`
  and `the_algeria_shaped_annulment_fires_under_its_conditions_and_not_under_the_court`;
- the inertness tests `the_bloc_layer_is_inert_at_1990` and `every_road_reads_closed_while_takeover_is_off`.

All of them pass.

The single failure was the one predeclared in `plan.json`:
`government::tests::sourced_executive_removal_leverage_is_distinct_from_auth_resources_and_coup_risk`.
- **Old assertion.** Algeria with sourced leverage 1.0, moved from .58 to .05 authoritarianism, keeps
  its nonzero penalty. On the changed runtime it read left 0.0, right 0.5441583.
- **Why it could be updated.** That assertion directly encodes the overturned behaviour.
- **What was updated.** The original intent ("a nominal authoritarianism edit is not evidence of new
  military authority") is kept above the boundary, at .25, where the penalty is unchanged. At .05 the
  test now asserts penalty 0 and an unchanged authority record.
- **Side effect (review note, Claude, automated).** The rest of the test still runs on the .05 world.
  Its later `no_army` check therefore still passes, but its zero-penalty assertion no longer
  distinguishes the missing-Army return from the new guard. Its `current_leverage == None` assertion
  is unaffected.

Old versus new expectations are in [`edited-tests.json`](edited-tests.json). No other test was edited.

## 5. The unchanged political gate

Command: `cargo test --locked --release -p spheres-sim --test bloc_census -- --include-ignored --skip bloc_census --nocapture`.
Full output: [log](logs/gates-nocapture.log). It finished 10 passed, 1 failed (A1), exit 101, in
97.68 s, on the same contended local container.

The gate binary SHA-256 is `20b22f0e43e0fba9a4a7c9a272dbed7b6db6d238d0d292487cc99f5fed8ef323`. The
39369f0 baseline binary was `81f13be5…aac31`, so the gate ran on a different build.

| Gate | Baseline 39369f0 | With guard | Change |
|---|---|---|---|
| A1 elected coups | median 7.5; top-3 4/7 = 0.571429 **FAIL** | median 7.5; top-3 4/7 = 0.571429 **FAIL** | none |
| A1 per seed | 3/7, 4/6, 3/5, 5/8, 4/8, 4/8, 4/7, 5/8, 4/8, 4/7, 5/8, 4/7 | identical | none |
| A2 | 8/12; Algeria Islamist 0/12, annulled 12/12 | 8/12; 0/12; 12/12 | none |
| A3 | 0/20 | 0/20 | none |
| A4 | median 16 | median 16 | none |
| A5 | 12/12 | 12/12 | none |
| A6 | 0 route events in 12/12 (420 months) | 0 in 12/12 | none |
| A7 | 8.462 | 8.462 | none |
| A8 | 40/40 | 40/40 | none |
| A9 | 168/168 | 168/168 | none |
| A10 | 0.335 | 0.335 (identical per-seed values) | none |
| Attribution | pass | pass | none |

The 12 A1 seed rows ([table](logs/a1-seed-table.txt)) hash to SHA-256 `87b3ea9b…3deb`. That is
byte-identical to the local baseline and to both CI jobs of run 37056033496, as recorded in the
[scout packet](../a1-claude-scope-20261004/README.md). Every printed gate line matches the baseline
`--nocapture` output.

**The change is live in the campaign.** The scout's read-only harness was rebuilt unchanged against
this checkout: [harness](harness/d2-diag/src/main.rs), byte-identical to
`a1-claude-scope-20261004/harness/a1-diag/src/main.rs`, SHA-256 `b45f5f07…ba73`. Its output is
[d2-diag-seeds0-11.log](diagnostics/d2-diag-seeds0-11.log) and the
[diff against the 39369f0 log](diagnostics/cty-diff-vs-39369f0.txt).

On seeds 0-11, the maximum pooled penalty changed as follows:

| Nation | Penalty at 39369f0 | Penalty with guard | Note |
|---|---|---|---|
| Uruguay | .222 | 0 | minimum effective loyalty .470 → .587 |
| Brazil | .185 | 0 | |
| Venezuela | .131 | 0 | |
| PNG | .088 | 0 | |
| Argentina | .102 | .052 | charged only in months when its live authoritarianism is above .20, as designed |

Peru's minimum effective loyalty moved .380 → .381, an indirect world effect. Every elected-coup
line, including its pre-tick readings, is identical. So the guard changes political trajectories in
the protected democracies, but not any counted event in the 12-seed cohort.

## 6. Workspace suite and resource timing

**Full workspace suite** ([log](logs/workspace.log)):

    cargo test --locked --release --workspace --no-fail-fast -- --skip tests::the_resource_pass_stays_under_budget

It ran with `-j 3` for the build and finished in 18 min 2 s wall time, including the build, with
exit 0. Totals across 68 test targets, doc-tests included:
- **2,014 passed**;
- **0 failed**;
- 115 ignored;
- 1 filtered, which is the timing test.

The spheres-sim library alone gave 1,090 passed, 0 failed, 44 ignored. The only build warnings are
the two pre-existing ones in vendored `tiny_http`.

`bloc_census` runs inside this suite without `--include-ignored`, giving 5 passed and 7 ignored.
A6, A8, A9, A10 and the attribution control run and pass there. A1-A5, A7 and the measurement-only
`bloc_census` scan are `#[ignore]`d by design. The full gate set is the separate
`--include-ignored` run in section 5.

**Isolated resource timing** ([log](logs/resource-timing.log)):

    cargo test --locked --release -p spheres-sim --lib tests::the_resource_pass_stays_under_budget -- --exact --nocapture --test-threads=1

It passed with total **0.1114 ms/month**, under the 0.15 bar:
- resources 0.0640;
- buy pass 0.0197;
- appetite term 0.0278.

The test's own 0.05 ms/month advisory reading was not met. **This is contended container hardware,
not the pending quiet Windows measurement.** The separate S25 diagnostic was using about 98% of one
of the 4 CPUs throughout, and the load average was 2.7 before the run.

**Environment and provenance:**
- Toolchain: local Linux x86_64, rustc 1.97.0 (2d8144b78 2026-07-07), cargo 1.97.0.
- `CARGO_TARGET_DIR` was the private `scratchpad/target-d2`.
- The checkout is a sparse linked git worktree of the base. It omits `.github/` and every `docs/` subtree except
  `design`, `political-arm` and `research`. All targets built and passed without those paths.
- The committed source is byte-identical to the source that was built and tested. Both files were
  last modified before the final test build.
- Library test binary SHA-256: `42f6bffea8d06bff79ded579c387f61397b3bc7f1002d938060d7ab8dac75519`.
- Build logs: [red](logs/build-red.log), [green](logs/build-green.log) and [gate](logs/gate-build.log).
  All three have zero warnings in spheres-sim.

## 7. What this establishes, and what it does not

**It establishes:**
- The documented civilian-control guard now holds in code for sourced leverage. This is demonstrated
  by a regression that fails on the unmodified runtime, plus passing controls.
- The original A1-A10 and attribution outputs are unchanged on local Linux.
- A6's protected democracies no longer carry a civilian-confidence penalty while their live
  authoritarianism stays at or below .20. A later authority route (D1) can reuse this boundary, but
  must apply it itself. This change guards only the confidence penalty and, through it, the Army
  target and the AI floor. The constant is private and `army_programme_veto_loyalty` is unguarded.

**It does not establish:**
- **A1 still fails** (7.5 / 0.571429), as predicted in the plan.
- Nothing here closes or qualifies A1, `CODEX-S27-A1-01`, S27, G5 or CP1.
- The Windows run of the gate and the CI run are pending. The gate ran locally only.
- The quiet-hardware resource timing is pending (see section 6).
- No development (0-199) or reserved (1000-series) cohort was run.
- The programme-veto question in section 1 is left for review.

**Reviewer label:** this work was done by Claude, an automated agent. It carries no human or Codex
review or approval.

## 8. Follow-up after the automated review

The review found that the edited test's later `no_army` control also ran at authoritarianism .05, where
the guard already makes the penalty zero, so it no longer detected a broken missing-Army return. A
follow-up commit runs that control at .25 after asserting the channel is open there
([followup.diff](followup.diff)). The focused government, army_authority, stratagem and blocs tests
then gave 167 passed, 0 failed, 1 ignored ([log](logs/followup-focused-tests.log)). This test-only change
does not affect the gate binary's behaviour.

Commits as proposed in the pull request (cherry-picked unchanged from the local branch):
`215bd023` (guard, from `51a7ddfa`) and `2bce829a` (follow-up test, from `f6597cf5`).
