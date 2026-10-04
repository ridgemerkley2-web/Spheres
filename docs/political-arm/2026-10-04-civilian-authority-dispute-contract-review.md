# Civilian-authority dispute contract (D1): automated review record

**Status: record of two automated adversarial reviews of the D1 design contract, and of how the revised contract disposes of each finding. Not approval.**

| | |
| --- | --- |
| Contract | [2026-10-04-civilian-authority-dispute-contract.md](2026-10-04-civilian-authority-dispute-contract.md) |
| Task | `CODEX-S27-A1-01`, direction D1 (design contract only; no code) |
| Date | 4 October 2026 |
| Reviewers | Claude, automated agent - not human or Codex review. Both reviews were run by Claude as automated agents with different lenses. Neither is human review, Codex review or approval. |
| Reviewed draft | The first draft of the contract (base `39369f0`; HEAD `eac153f7` at review time) |
| Revision | By Claude, automated agent, on 4 October 2026, after both reviews (R1-R28), then a verification pass, also by Claude, automated agent, that fixed factual and labelling errors only (R29). See the contract's section 18. Neither is human or Codex review. |

Both reviewers also made small factual corrections directly in the contract file. Those corrections are listed in their reports below and are retained in the revision.

The reports are reproduced **verbatim** from the review workflow's output. Finding IDs:

- **DA** is the design-authority lens.
- **ER** is the engineering-risk lens.

"Open question #N" refers to the revised contract's consolidated numbering in section 17:

- #1-#14: blocking before measurement;
- #15-#24: before implementation;
- #25-#26: later.

---

## Review 1: design-authority lens (Claude, automated agent - not human or Codex review)

**Claude, automated agent - not human or Codex review.**

<!-- verbatim begins -->

REVIEW: /home/user/Spheres/docs/political-arm/2026-10-04-civilian-authority-dispute-contract.md
Reviewer: Claude, automated agent. This is not human review, not Codex review, and not approval. Nothing was built, run, committed or pushed. HEAD is eac153f7, whose runtime source is identical to 39369f0.

Overall: the contract is careful and well cited. I spot-checked about 60 code and line citations against the source and all hold except the three corrected below. Labelling, A1 humility, the seed ban, cohort rules, rejected policies, lens-off and takeover-off byte identity, no RNG in government, and the refusal → plan → arm → effects plan parity are all handled correctly. The problems are three design defects or conflicts with binding rules, and several decisions the contract makes silently that should be open questions.

ISSUES FOUND

HIGH-1. The "electoral rule ended" lapse can never run, so a stale gauge can survive its dispute.
- Section 10.1 calls `civilian_authority::tick` only inside `government::tick`'s electoral branch (government.rs:9543-9548). That branch runs only while `is_electoral` holds (authoritarianism < .60).
- Section 8.3's `!is_electoral → ElectoralRuleEnded` row therefore never executes, and the AI response in 11.2 also runs only for electoral nations.
- Only `regime_break` closes a dispute, and these exits from electoral rule do not call it:
  - `suspend_constitution` (:7679), whether by the AI lever or a player's self-suspension;
  - `settle_uprising` (uprising and armed-opposition victory);
  - `generic_regime_collapse` (:9149);
  - authoritarianism drifting to .60 or above.
- After any of these, an open (possibly Enforced) record and its `authority_pressure` stay frozen in GovState.
- If the polity reopens and the same party governs again, validation passes (same issuer, electoral, above .20, institution present). Accrual then resumes from the old gauge, and the dispute can fire once the 12-month settled rule is met.
- This breaks I10, 8.3's own heading, and the proposal's "an old gauge cannot manufacture a coup after the dispute ends". No control covers a suspension, uprising or collapse in the middle of a dispute.
- Related: validation never re-checks legality rules 5 and 6. A ban (which clears `unrestricted_mandate`) or a reopening (which sets `awaiting_first_election`) leaves the dispute alive even though the order's lawful basis is gone.

HIGH-2. The proposed "Floor" default contradicts the contract's own AI objective and the mechanism proposal.
- Section 11.1 says the AI reduces leverage "through lawful orders it can obtain" and "never pursues a dispute, a refusal or a coup for its own sake".
- Floor's second branch issues `BudgetOversight` when its own pure plan already says `complies: false`, then enforces it while it holds a majority.
- That is a known-refused order, so it obtains nothing. It is what the proposal forbids: "must not manufacture disputes", "positive governance objective".
- Under the 0.02 draw, the draw becomes a randomly timed start of a deterministic escalation countdown. That comes close to the forbidden background lottery.
- Prudent is the only temperament consistent with 11.1 as written. Floor should be offered neutrally, not as the default, or 11.1 must be rewritten and approved first.
- Assertive is worse. For a funded army (effective loyalty about .65), a full-scope refusal happens exactly when leverage exceeds about .67. The order → refusal → coup chain is then driven essentially by V-Dem leverage alone, which the hard constraints rule out.
- The "Expected direction" column in 11.3 describes A1-relevant outcomes per temperament, which invites selection by A1 effect despite the prohibition.

HIGH-3. Escalation reproduces route 2's material term without route 2's discontent guard.
- The gap is 0.35 − acceptance = (0.35 − eff) + 0.45·lev·s.
- For an Army with effective loyalty below .35 in a calm period (D < .25, which is exactly when Floor issues), the enforced dispute accrues 0.60·(0.35 − eff). That is route 2's own material-hostility term at route 2's slope, with no D ≥ .25 condition.
- Example, Sao Tome-shaped (leverage .25, effective loyalty .30): the gap is .0875, pressure grows about .053 a month, and the dispute fires in about 19 months with no discontent at all. Route 2 could never fire there.
- In effect this bypasses the live D ≥ .25 guard that the hard constraints protect, and it targets the core countries, adding repeats.
- Section 10.5 describes this as a "correlated input, not a double charge". In fact it is the same grievance feeding a second gauge without the guard. It is not raised as a question.

MEDIUM-1. Funding settles a dispute automatically by default, against two of three source texts.
- Sections 9.4 and 7.4 close a dispute as `CompliedWhileOpen` whenever funding lifts acceptance to .35, with no government action.
- The geography review says funding must retain its material benefit "while not automatically settling the specified dispute". The 22 September record says "A financial appropriation is not an automatic cure for political opposition to civilian authority".
- The contract relies on the proposal's weaker word "silently". Section 7.4's claim that this is the material benefit "the geography review requires to be kept" misreads the review.
- Q7's proposed default conflicts with the review and should say so.

MEDIUM-2. No saved institutional powers, so compliance ratchets leverage down.
- Proposal step 1 requires specifying "which saved institutional powers an order would remove" before any runtime trial. Here scopes are only stake multipliers and nothing is saved.
- The same scope can be ordered again after every 12-month cooldown, and each compliance calls `consolidate_transfer` again.
- That function's own doc comment says to use the lawful-transfer amount "once", and its caller (government.rs:7154-7163) prorates explicitly to stop "repeated snap elections from manufacturing extra institutional gains".
- Under the AI, quiet funded armies ratchet toward zero leverage: up to .125 a year this way, against .125 per term for lawful transfer. That shifts confidence penalties, programme vetoes and AI funding paths across the whole world.
- It also adds a third leverage-mutation path to `army_authority`, whose module contract allows only seizure and handover. This is not raised.

MEDIUM-3. The order outranks every stratagem card.
- Routing `ai_order` through `ai_lever` means the order takes precedence over all cards (stratagems.rs:567-575) in any month its preconditions hold.
- The order executes only when the 0.02 draw passes, so a qualifying nation plays no cards for about 50 months on average per order cycle, and again after each cooldown.
- The existing lever-first rationale ("its conditions are the narrower crisis", stratagems.rs:552-561) does not fit a calm-time order.
- The contract treats card displacement only as an RNG-stream risk, not as an AI behaviour decision.

MEDIUM-4. Rule 9 extends the user's decision without saying so.
- The 4 October decision covers "the Army's civilian-confidence channel".
- Rule 9 and I11 extend it to orders, accrual and firing. That is sensible and protects A6, but it extends a design-authority ruling and should be confirmed rather than presented as "applies the user's decision".

MEDIUM-5. Legibility choices are made silently.
- The census tests the text after `COUP IN <NATION>:` with `contains("removes the elected government")` (bloc_census.rs:377-398). A suffix clause naming the refused order would still be counted the same by A1, A6 and A9, and it would be legible. The contract wrongly implies the exact template is required.
- Without a fifth road (Q10), the takeover watch's coup road reads far from its trigger, and the map hatching and header chip stay quiet, right before a dispute coup. Q10's rationale should state this cost.

MEDIUM-6. Who counts as a lawful issuer is decided silently.
- Rule 5 requires `g.elected`, which means "anyone has voted yet under this module", plus the six-month record.
- Every 1990 incumbent is excluded until its first modelled ballot, including governments with sourced opening mandates. This has A4 timing consequences (before end-1996) and is not stated as a decision.

MEDIUM-7. Sequencing is undefined if D2 regresses a gate.
- D2's plan keeps the change whatever the gate outcome.
- If the post-D2 baseline fails any gate other than A1, two things in the contract become ill-defined: the stop rule's "every other gate passes" branch (Q15) and the condition for the one 0-199 run.

LOW-1. Enforcement continues for free once political capital reaches 0, because upkeep floors at zero. Consistent with existing upkeep, but unstated.

LOW-2. There is no hysteresis. `electoral_army_tick` walks loyalty before the dispute tick, so an order refused at issue can close as CompliedWhileOpen in the same tick, producing a refuse/carry-out headline pair.

LOW-3. Ordinal mapping:
- `InstitutionRef.ordinal` indexes `GovState.pillars`, but `institution_name` reads `polity_in().pillars`, and `seat_spec_pillars` appends entries at the end. The two lists' order should be asserted to correspond.
- spheres-web already has a private `institution_name(w, id, Pillar)` at government_view.rs:58. The new sim reader should replace it so there are not two name readers.

LOW-4. SPEC §4 convention says every invented coefficient is "filed in BUGS with what would calibrate it". The register gives a basis but no "what would calibrate it" entries.

CORRECTIONS MADE (factual only, in the contract file)

1. Header Base row: added that the commits through eac153f7 are docs-only, so the runtime source and line numbers are unchanged.
2. §2.1: BUGS S4-1 lists four removals (Pakistan, Thailand, Haiti, Turkey 1980), not three. Reworded to say three are relevant here.
3. §3.4: the hook goes after the route-2 block at `:9543-9548`, not "after `:9544`". Also, `electoral_coup_settled_months` (:8857) is already `pub`, so I removed it from the `pub(crate)` list.
4. §10.3: `break_electoral` caps authoritarianism at .98; added "capped at .98".
5. §16 A2 row: the gate is "more than 6 and at most 10 of 12; Algeria Islamist under 6, annulled at least 2" (bloc_census.rs:1025-1027). Current values added (Algeria Islamist 0/12, annulled 12/12).

OPEN QUESTIONS TO ADD

QA (must be answered before measurement is predeclared): Which exits from electoral rule close an open dispute, and with what resolution? This covers suspension (including self-suspension by the issuing government, which today is a 40 PC escape from an enforced dispute), uprising or armed victory, collapse, and drift to .60 or above. Should validation run for every nation holding a record, regardless of branch? Should it re-check legality rules 5 and 6 (restricted mandate, interim government) during a dispute? Add controls for each case.

QB (before measurement): Should the escalation gap be limited to the authority component (for example capped at 0.45·lev·s), or should orders or enforcement be refused while effective loyalty is below .35 (route 2's domain), so that a dispute cannot bypass route 2's D ≥ .25 guard?

QC: Are complied scopes saved as institutional powers, so the same scope cannot be ordered again? Or may each order reduce leverage again? Should the total civilian-order consolidation be bounded or prorated like the lawful transfer? Does `army_authority`'s two-path contract change?

QD (before measurement): Should the order outrank stratagem cards in `ai_lever`, or apply only in months when no card is chosen? This interacts with Q11.

QE: Confirm that the 4 October D2 decision extends to orders, accrual and firing (rule 9, I11), not only the confidence channel.

QF: Should Q1 be reframed? Floor's refused branch and all of Assertive conflict with 11.1 and the proposal. Should the A1-direction column be removed from 11.3?

QG: Should Q7 be reframed to quote the geography review ("not automatically settling") and the 22 September record ("not an automatic cure") against the proposed default?

QH: Should the escalation headline carry a census-compatible suffix naming the refused order? Should Q10's rationale record that the coup road gauge will read far from its trigger before a dispute coup?

QI: Do sourced opening mandates (pre-1990 elected incumbents) count as lawful authority to issue an order before their first modelled ballot?

QJ: If D2's measurement regresses any gate other than A1, does D1 proceed? How do the stop rule, Q15 and the 0-199 condition apply?

QK: Should enforcement continue at zero political capital, or lapse into withdrawal?

<!-- verbatim ends -->

---

## Review 2: engineering-risk lens (Claude, automated agent - not human or Codex review)

**Claude, automated agent - not human or Codex review.**

<!-- verbatim begins -->

Review of /home/user/Spheres/docs/political-arm/2026-10-04-civilian-authority-dispute-contract.md

The contract is close to implementable, but two problems need a decision before anything is predeclared. Its lapse rules cannot fire once a state stops being electoral, and the AI temperaments as written conflict with the hard constraints. I checked every citation against the code at 39369f0; spheres-sim and spheres-web are unchanged through eac153f7, so the line numbers still hold. I made five small factual corrections in the file. Nothing was committed or pushed, and the file is still listed in .git/info/exclude.

ISSUES FOUND (most severe first)

HIGH-1: Lapse rules cannot fire outside the electoral branch, so an old gauge can come back
- The dispute tick is called only in tick's electoral branch (after government.rs:9543-9548).
- Several paths end electoral rule without calling regime_break:
  - suspend_constitution (:7679)
  - settle_uprising (:9077)
  - generic_regime_collapse (:9149)
  - the security_crackdown card's +.06 authoritarianism (stratagems.rs:381)
  - campaign_peace.rs:438
- After any of these, the tick never runs. The 8.3 "ElectoralRuleEnded" row is unreachable, and an open dispute (possibly Enforced, with pressure up to 1.5) stays in the save.
- If the state reopens and the same party leads, validation passes and accrual resumes; 10.2 accrual does not check awaiting_first_election. The old gauge can then fire 12 settled months after the first ballot. That breaks I10 and the rule that an old gauge cannot manufacture a coup.
- Fix: run the lapse step for every nation with a dispute, before the electoral/regime split (behind the switches, doing nothing when there is no dispute), or hook every exit path. Add a control: suspend with an open Enforced dispute, expect an ElectoralRuleEnded lapse with pressure deleted, then reopen and expect no dispute.

HIGH-2: The AI temperaments recreate forbidden triggers
Issuance and response are deterministic functions of live state, so the AI policy as a whole acts as a trigger rule:
- **Assertive:** the Army refuses exactly when eff < .35 + .45·lev. A funded army at eff ≈ .65 fights exactly when lev > .667. In effect "high sourced leverage plus calm" becomes the coup trigger, which hard_constraints exclude.
- **Floor:** AI disputes arise exactly when (loyalty_k − F_N) < .35 + .15·lev, with D < .25 and a majority, and escalation has no discontent term. For AI-governed states that is route 2 with its D ≥ .25 guard removed and its .35 line raised by .15·lev: the forbidden "delete the live guards / lower a ceiling", reached by combination even though route-2 code is untouched.
- **Prudent:** the only option with no AI-originated disputes.

Q1 raises the temperament choice but does not connect it to these constraints. The design authority needs to rule on this explicitly.

MEDIUM-1: The order displaces electoral AI card play
- In ai_stratagems, a lever always beats a card (stratagems.rs:569-573), and the 0.02 draw only gates execution after that choice is made.
- Under Floor or Assertive, ai_order returns an order in every eligible month (calm, majority, PC ≥ 45, outside the cooldown, leverage > 0).
- Electoral AIs therefore lose card play for most of each roughly 50-month wait, not only in the months the draw passes. Affected cards include liberalisation (which prints a census "opens up." headline), professional_army, austerity and debt_restructuring.
- The code's own rationale ("the lever comes first because its conditions are the narrower crisis", :552-560) does not hold for an order issued in calm times.
- Section 16 lists only the RNG consequence. This needs a precedence decision and a control showing that a chosen card is unchanged.

MEDIUM-2: Political-capital drain is missing from the risk register
Each AI order costs 25 PC, at most once every 12 months, plus 0.20 a month while enforced. That competes with SuspendConstitution (55), BanParty (60), cards (cost + 20) and SecurePillar (above 55). It is a behavioural path to A2, A4 and A7 separate from the RNG shift.

MEDIUM-3: Acceptance is undefined mid-dispute
`acceptance` returns None when legality rules 3-11 fail. Rules 5 and 6 can fail during a dispute: a party ban clears unrestricted_mandate, or the state enters interim status. 8.3 has no lapse for either, so tick steps 3-5 (compliance, gap, firing) are then undefined. Either define acceptance for open disputes from the institution and leverage terms only, or add lapses for rules 5 and 6.

MEDIUM-4: Resolutions can act on a dispute whose issuer is already gone
- Validation runs only in the dispute tick, which comes before hold_election in the same branch.
- The month-end ai_response, or a player command, can then act on a dispute whose issuer the ballot has just replaced. Example: a Negotiate leads to NegotiatedCompliance and consolidate_transfer is credited to the new government.
- resolution_refusal/plan should apply the 8.3 checks first. Rule 12 also wrongly refuses the new government's own order while that stale dispute is open.

MEDIUM-5: Soft risk of fitting to seeds 0-11 in how Q1 is framed
The "expected direction" columns in 11.2/11.3 and the Pakistan/Thailand verdicts in 7.5 come from the seeds 0-11 minimum loyalties (.647/.650). They sit next to a choice the contract says must not be made by its A1 effect. Those columns should be omitted from the decision record, or labelled as derived from seeds 0-11.

LOW
- **L1:** The regime_break hook still runs in takeover-off worlds (maybe_coup calls regime_break), so it can close a "frozen" dispute. That contradicts 8.3 and Q16. Gate the hook on the takeover switch, or document the exception.
- **L2:** After Negotiate, pressure accrued against the wider order carries into the narrower one if it is enforced again (9.2).
- **L3:** For Escalated and GovernmentRemoved receipts, `leverage_after` is not specified as before or after army_seizure.
- **L4:** The new types are shown without derives. Command derives Clone, Debug, Serialize, Deserialize, PartialEq (lib.rs:105), so DisputeAction, InstitutionRef and AuthorityScope need at least those.
- **L5:** 14.3 should name `the_daily_clock_preserves_the_political_arm_on_world` (lib.rs:2183). It is the existing whole-world daily-equals-monthly test with both switches on, and it will now see AI orders. The test-only observer field `selected_lever` (government_a1_observer.rs:80) will also change.
- **L6:** The new `institution_name(w, id, InstitutionRef)` shares a name with the web crate's `institution_name` (government_view.rs:58). It is harmless but confusing.
- **L7:** `/api/load` forces ideology_takeover off (spheres-web/src/main.rs:7166). Any save with an open dispute loaded in the browser is therefore frozen and, because the UI shows disputes only with both switches on, invisible. Q16 should be answered before stage 1.

Checked and correct:
- All other code anchors in sections 3.4 and 6 and Appendix A.
- The 7.5 arithmetic.
- The claim that the shared programme-veto helper keeps results bit-identical.
- I2 (takeover off adds no draw).
- The census headline patterns in 9.3 match the parser (bloc_census.rs:372-497).
- Sudan is the only polity with two Army entries, and it is a regime in 1990.
- The D2 plan claims.
- The citations of CONTRIBUTING, verify.yml, BUGS S4-1, the 22 September record and SPEC (Argentina 59/60).

Note on the baseline: the D2 packet on disk (civilian-guard-20261004, not committed, excluded by .git/info/exclude) reports every gate unchanged. So the conditional post-D2 baseline in 15.3 would apply if that result is integrated.

CORRECTIONS MADE IN THE FILE
1. Section 10.2 illustration: Pakistan accrues .0378 a month and reaches 1.0 at the 27th monthly accrual (1/.0378 ≈ 26.5); Thailand accrues .0449 and reaches it at the 23rd (≈ 22.3). The file had said about 26 and about 22.
2. Appendix A: `actionCard` is at government-ui.js:86, not :88.
3. Sections 3.4 and 6: the command arms are dispatched by `dispatch` (lib.rs:1026), which `apply_command` (:900) reaches. They now cite the lever arms at :1412-1418.
4. Section 10.3: `coup_pressure` is cleared by regime_break, not army_seizure. Added that regime_break also resets political capital.
5. Section 16 performance row:
   - The 0.0623 ms/month figure (bar 0.15) comes from the resource-timing test, which times only the `resources` row of a lens-off world (lib.rs:5966), so it cannot show this change's cost.
   - ai_lever is evaluated every tick (every day on the daily clock), not every month.

OPEN QUESTIONS TO ADD
- **Q18:** Where do lapses run? Should suspension, uprising, collapse or a crackdown that ends electoral rule close a dispute immediately through a hook, or at the next tick in any branch? (Must be resolved; HIGH-1.)
- **Q19:** Should losing rule 5 or 6 mid-dispute (party ban, interim status) lapse the dispute, or should acceptance for open disputes ignore those rules?
- **Q20:** In ai_stratagems, should the order come before or after cards, or only when no card is chosen?
- **Q21:** Does Floor or Assertive satisfy the bans on a "leverage alone" trigger and on bypassing route 2's D ≥ .25 guard by combination? Should Prudent be the only admissible temperament?
- **Q22:** Should pressure accrued at a wider scope survive negotiation to a narrower one?
- **Q23:** Must every resolution and order re-run the 8.3 validation first, to avoid acting on a dispute whose issuer is gone within the same tick?
- **Q24:** Should the regime_break hook respect the takeover switch, so that a frozen dispute stays frozen?
- **Q25:** Should the AI keep a political-capital reserve above C12 so orders do not starve SuspendConstitution, BanParty and cards? (Could fold into Q13.)
- **Q26:** Should the cooldown carry over to a new governing party, or reset on IssuerReplaced?
- **Q27:** Should the seeds 0-11 "expected direction" material be excluded from the record of the Q1 decision?

<!-- verbatim ends -->

---

## Disposition of every finding in the revised contract

The disposition values are:

- **fixed:** an engineering defect with an unambiguous fix that needs no design decision, made in the contract text;
- **converted to open question #N:** a design choice or conflict, left to the design authority;
- **disagreed:** with a reason. No finding was disagreed with. Where the revision chose a different fix from the one a reviewer suggested, the row says so.

Section and R numbers refer to the revised contract.

| Finding | Disposition | Where (revised contract) |
| --- | --- | --- |
| DA HIGH-1 | Fixed. Validation and lapses run for every nation holding a record, before the electoral/regime split, behind both switches, doing nothing without a record. The drafted resolutions are mapped to every listed exit. The residual design choices became open questions #23 and #24. | 3.4, 8.3, 10.1, I2, I10, controls 27-31; R3, R4, R29 |
| DA HIGH-2 | Converted to open question #1 (blocking). The Floor default was withdrawn; the temperaments are presented neutrally, with an analysis showing that only Prudent is consistent with 11.1 as written. The expected-direction column was removed. | 11.2, 11.3, 17.1; R12, R13 |
| DA HIGH-3 | Converted to open question #4 (blocking). The gap decomposition and the guard bypass are stated, and the "correlated input, not a double charge" claim is withdrawn. | 10.2, 10.5, 16; R14 |
| DA MEDIUM-1 | Converted to open question #5 (blocking). Both source texts are quoted, and the misreading of the geography review is withdrawn. | 7.4, 9.4; R15 |
| DA MEDIUM-2 | Converted to open question #6 (blocking). The ratchet, the missing saved institutional powers, the `consolidate_transfer` "once" contract and `army_authority`'s two-path contract are stated. | 7.3, 7.6, 16; R16 |
| DA MEDIUM-3 | Converted to open question #3 (blocking). Card displacement is stated as an AI-behaviour decision, with a conditional control. | 11.4, 3.3, 16, control 35; R17 |
| DA MEDIUM-4 | Converted to open question #7 (blocking). Rule 9 is now labelled an extension of the user's decision that needs confirmation. | 5, I11, 7.6; R18 |
| DA MEDIUM-5 | Converted to open question #13 (blocking). The census-compatible suffix and the watch-legibility cost are stated. (This is the legibility finding. The design review's expected-direction finding is the last bullet of HIGH-2 and QF.) | 9.3, 12, I12; R22 |
| DA MEDIUM-6 | Converted to open question #8 (blocking). Rule 5's choice of lawful issuer is stated. | 5; R19 |
| DA MEDIUM-7 | Converted to open question #14 (blocking, conditional). The revision records that D2's local Linux gate run printed every value identical to 39369f0, so the post-D2 baseline equals the pre-D2 baseline; Windows and CI are pending. A stop-rule branch was added. | 3.1, 15; R20 |
| DA LOW-1 | Converted to open question #12 (blocking). | 6; R21 |
| DA LOW-2 | Converted: folded into open question #5 (hysteresis), with the same-tick refuse/carry-out sequence stated. | 9.4; R15 |
| DA LOW-3 | Fixed. Ordinal correspondence is asserted by a test across the roster and the reseat paths. One sim name reader replaces the web's private reader, with byte-identical output. | 3.4, 4.2, control 33; R10 |
| DA LOW-4 | Fixed. A "What would calibrate it" column drafts the BUGS entries, to be filed only on approval. | 7.6; R25 |
| DA correction 1 (header Base row) | Retained. Extended: D2's commits after eac153f7 do change government.rs, so the post-D2 line shifts were added. | header; R1, R28 |
| DA corrections 2-5 | Retained as made in the file. | 2.1, 3.4, 10.3, 16; R28 |
| DA QA | Fixed in its engineering part: where validation runs, and which exit gives which resolution. The residual design parts became open questions #23 (rules 5/6 re-check) and #24 (the issuer's own suspension or ban as an escape). The requested controls are added. | 8.3, controls 27-30; 17.4 |
| DA QB | Converted to open question #4 (blocking). | 17.1 |
| DA QC | Converted to open question #6 (blocking). | 17.1 |
| DA QD | Converted to open question #3 (blocking). | 17.1 |
| DA QE | Converted to open question #7 (blocking). | 17.1 |
| DA QF | Converted to open question #1 (blocking). The A1-direction column was removed (R12). | 11, 17.1 |
| DA QG | Converted to open question #5 (blocking), quoting both texts. | 7.4, 17.1 |
| DA QH | Converted to open question #13 (blocking), with the watch cost stated. | 12, 17.1 |
| DA QI | Converted to open question #8 (blocking). | 17.1 |
| DA QJ | Converted to open question #14 (blocking, conditional on D2's pending runs). | 15, 17.1 |
| DA QK | Converted to open question #12 (blocking). | 17.1 |
| ER HIGH-1 | Fixed, as the reviewer proposed: the lapse step runs for every nation with a dispute, before the split, behind the switches, and does nothing without a dispute. The suspend → lapse → reopen control and the controls for the other exits are added. | 8.3, 10.1, controls 27-31; R3, R4 |
| ER HIGH-2 | Converted to open question #1 (blocking). The formula analysis of Floor and Assertive is adopted in 11.3, and no default is proposed. | 11.3, 17.1; R13 |
| ER MEDIUM-1 | Converted to open question #3 (blocking), with the precedence control (control 35). | 11.4, 16; R17 |
| ER MEDIUM-2 | Converted to open question #9 (blocking), merged with the first draft's Q13 on prices. A register row was added. | 6, 7.6, 16; R21 |
| ER MEDIUM-3 | Fixed in its engineering part: dispute acceptance must be total on validated records (I16, control 34). The choice between a lapse and ignoring rules 5-6 became open question #23. | 8.3, 10.1-10.3; R6 |
| ER MEDIUM-4 | Fixed. Every refusal, plan, arm, effects function, `ai_order` and `ai_response` re-runs validation first (the post-lapse view). Rule 12 no longer blocks a new government because of a stale record; the arm closes it first. | 4.4, 5, 9.2, 11.2, I15, control 32; R5 |
| ER MEDIUM-5 | Fixed. Expected-direction material was removed or labelled "derived from seeds 0-11, must not inform the decision". The Q1 decision record may not cite it. The verification pass also labelled section 1.2 and removed the 10.2 illustration's link to a temperament. | 1.2, 1.3, 7.5, 10.2, 11, 15, 16; R12, R29 |
| ER L1 | Converted to open question #22. The proposed freeze requires gating the hook, and this is stated. | 8.3; R23 |
| ER L2 | Converted to open question #11 (blocking). | 9.2; R24 |
| ER L3 | Fixed. `leverage_after` is defined as read before `army_seizure` on Escalated and GovernmentRemoved receipts. | 8.1, 10.3, control 36; R7 |
| ER L4 | Fixed. Derives are specified for every new type. | 4.1, 4.4, 8.1; R8 |
| ER L5 | Fixed. The daily==monthly test (lib.rs:2183) and the observer's `selected_lever` (government_a1_observer.rs:80) are named. | 14.3; R9 |
| ER L6 | Fixed. There is one sim reader, `institution_spec_name`, and the web's private `institution_name` is replaced. | 3.4, 4.2; R10 |
| ER L7 | Converted to open question #22, answered before stage 1. | 8.3, 12; R23 |
| ER corrections 1-4 | Retained as made in the file. | 10.2, Appendix A, 3.4, 6, 10.3; R28 |
| ER correction 5 (performance) | Retained and extended: no figure measures this path. Added the per-nation validation check and D2's 0.1114 reading. | 16; R26 |
| ER Q18 | Fixed: the lapse runs at the next tick in any branch, before the split, and the arms re-validate. Closing at once through a hook on every exit path was not chosen, because one pre-split step covers every present and future exit without touching each exit function. The closure date differs by at most one tick, and nothing can act on the record in between (post-lapse view). | 8.3; R3 |
| ER Q19 | Converted to open question #23. | 17.2 |
| ER Q20 | Converted to open question #3 (blocking). | 17.1 |
| ER Q21 | Converted to open question #1 (blocking). | 17.1 |
| ER Q22 | Converted to open question #11 (blocking). | 17.1 |
| ER Q23 | Fixed: yes, every resolution and order re-runs validation first. | 4.4; R5 |
| ER Q24 | Converted to open question #22. | 17.2 |
| ER Q25 | Converted to open question #9 (blocking), merged with the first draft's Q13. | 17.1 |
| ER Q26 | Converted to open question #10 (blocking), merged with the first draft's Q14. | 17.1 |
| ER Q27 | Fixed by removal (R12). Open question #1 also requires the decision record to cite no seeds 0-11 material. | 11.3, 15, 17.1 |

### The first draft's own open questions

The first draft's Q1-Q17 are merged into the consolidated list. The map from old to new numbers is in the contract's section 17.4.

### What this record does not do

- It does not answer any open question.
- It does not approve the contract.
- It does not close A1, `CODEX-S27-A1-01`, S27, G5 or CP1.

No code, data, test, threshold or cohort was changed in reviewing or revising the contract.
