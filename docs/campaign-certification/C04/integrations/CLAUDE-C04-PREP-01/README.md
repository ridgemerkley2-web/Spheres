# CLAUDE-C04-PREP-01 — independent bounded integration review

**Accepted with repairs as preparation only · 28 September 2026.**
Reviewer: Codex `/root/s20_preflight`. Original author: Claude; its deliverable
commit and authorship are preserved. No production identity, artwork, party row,
executive grant or succession action is installed. **C04, C06 and S23 remain open.**

The submitted remote revision is `62aae6b76b4789bbb9501bdd768fa25176a66b15`;
its five-file deliverable is `63389675`. Review starts from integration
`75626a137d5efd1ca1cca0d6f76c4d8502909bee`. See the exact
[submitted handoff](claude-submission-handoff.md) and
[original Git-blob pins](original-submission.json). The global handoff, queue,
workboard, runtime and art were not edited.

## Review result

All eight dossiers remain explicitly invented, with four France and four Tonga
proposals. Their IDs do not collide with the inspected repository corpus;
computed ages satisfy the reviewed institutional floors and authored plausibility
bands. Potential eligibility begins after the frozen 7 September 2026 cutoff and
ends no later than 31 December 2035. Dates neither appoint the drafts nor replace
historical or saved incumbents. No hereditary descent, peerage or royal parentage
is invented. Appearance briefs are text-only cartoons; they are not reviewed art.

| Draft | Reviewed role | Window start | Conservative age bounds | Scope |
| --- | --- | --- | --- | --- |
| `draft_c04_fr_01` Ondine Rivasseau | PS First Secretary | 2029-09-08 | 50–57 | Party process and accrued membership; no executive grant |
| `draft_c04_fr_02` Loïc Ferrandou | PCF National Secretary | 2026-09-08 | 42–52 | Winning congress list and authored prior cycle |
| `draft_c04_fr_03` Gwenola Pérignac | Presidential contender | 2029-09-08 | 54–61 | Membership, designation, presentations and an election; no acting presidency |
| `draft_c04_fr_04` Anatole Villaume | National Assembly deputy | 2026-09-08 | 37–47 | Legislative election; party/group offices remain separate |
| `draft_c04_to_01` Lesieli Fotu | People's representative | 2026-09-08 | 41–51 | Non-noble people's-seat election and qualifications |
| `draft_c04_to_02` Sitani Lolohea | Prime minister via people's seat | 2026-09-08 | 54–64 | Seat, vacancy, Assembly recommendation and royal appointment |
| `draft_c04_to_03` Pisila Tukuafu | Non-elected Cabinet minister | 2026-09-08 | 46–56 | Vacancy, nomination, cap and appointment; caretaker continuity |
| `draft_c04_to_04` Kalolo Vaikona | PTOA leadership | 2026-09-08 | 49–59 | Party rules unresolved; no seat follows from leadership |

The negative hereditary-role policy is deliberately conservative; it is not a
claim that every excluded office is itself inherited. PTOA's incorporation and
internal rules remain unknown. Its generic societies-law context is not proof
of an applicable leadership-selection procedure. UM-01 through UM-10 remain
unresolved; none is converted into an executable eligibility grant.

## Repairs and reproduced defects

1. **Self-declared counts.** The original validator accepted one French dossier
   when the same packet lowered `expected_counts` to one. The checker now owns
   the fixed four-plus-four pilot requirement.
2. **Role-gate removal.** A Tonga prime-minister dossier still passed with only
   a vacancy gate after removing its election, Assembly and appointment gates.
   The checker now retains the reviewed role/nation/category/affiliation/minimum-age and
   required gate-kind contracts outside the editable packet, rejects unknown
   substitute roles and empty gate text, and keeps the existing conservative
   PS membership windows. These checks validate this pilot; they do not interpret
   arbitrary prose or establish a general legal eligibility engine. Parent review
   also reproduced a coordinated affiliation change that bypassed the PS floor.
   Reviewed catalog and pilot affiliations are now pinned, and the membership
   floor depends on the reviewed role rather than the mutable party value.
3. **Caretaker continuity.** The non-elected minister biography said she would
   leave with the government. Clause51(3) preserves caretaker service after a
   general election until revocation or continuation. The dossier, source-claim
   summary and readable review now preserve that distinction. [Tonga Constitution](https://ago.gov.to/cms/images/LEGISLATION/PRINCIPAL/1988/1988-0002/ConstitutionofTonga.pdf_3.pdf), pp20–21.
4. **PS waiver scope.** The presidential dossier assumed a National Council
   seniority exception. The explicit Council exception in art5.1.5 names
   legislative, senatorial and European candidacies. This pilot uses ordinary
   accrued membership and no unreviewed waiver; it does not decide every possible
   presidential exception. [PS statutes](https://parti-socialiste.fr/wp-content/uploads/2026/06/REFORME_Statuts_V9.pdf), arts2.6.5 and5.1.5.

## Source evidence and limits

[Response recaptures](source-recapture.json) attempt only the 18 sources the
submission previously marked read. **Nine original bodies reproduced exactly;
two public HTML responses differed; seven direct requests were denied.** Eleven
new successful response bodies are retained losslessly as gzip with original and
encoded SHA-256/size. [Body verification](source-body-verification.json) checked
all11 round trips. Original hashes/fetch records remain intact; changed pages
are not represented as reproductions. The ten originally blocked/missing URLs
were not retried. No alternate authenticated route was used.

Critical institutional gates were checked in the primary Constitution/Electoral
Act, official French Assembly texts and PS statutes, with rendered official
presidential, PCF, RN and Tonga Assembly sources documented in the
[browser review](browser-source-review.json). The 2022 presidential memento,
[deputy eligibility fiche](https://questions.assemblee-nationale.fr/contenu/telechargement/625543/file/Fiche%203%20-%20Election%20des%20d%C3%A9put%C3%A9s.pdf)
and party-rule editions retain their dated scope. The [PCF source](https://www.pcf.fr/statuts_du_pcf_adopt_s_au_39e_congr_s)
corroborates the list-based party office; it is not a national executive grant.

This is not independent acceptance of all67 inherited claim summaries, proof
that no later amendment exists, or exhaustive historical/country coverage.
Current-source access does not move the 2026-09-07 research cutoff. Name checking
cannot exclude private individuals or establish approved likeness. Source-based
constraints still need accepted C02 inputs, edition review and C03 artwork before
production integration.

## Validation

- Original checker: **pass**, eight dossiers,878 real names and3,146 IDs inspected;
  original suite: **65 passed**. [Baseline runs](baseline-runs.json).
- The first sparse-checkout attempt omitted the read-only runtime constant file:
 65 tests ran with one `FileNotFoundError`; this was an environment setup error,
 not an original-validator defect. Adding the exact base-revision file fixed it.
- Six new regressions ran against the unchanged original checker: **71 tests,
 14 failing subtests**, including all eight vacancy-only role bypasses. The
 [failed output](new-regressions-before-fix.log) is retained.
- First repair: **71 passed**, retained in [intermediate runs](intermediate-71/final-runs.json).
- Parent-identified affiliation bypass: **two focused tests failed** before repair;
  [failed output](affiliation-regressions-before-fix.log) retained.
- Final repaired checker: **pass**; repaired suite: **73 passed**. [Final runs](final-runs.json).
 No original assertions were removed. No native build, campaign run or broad
 repository suite was executed for these isolated Python/dossier changes.

Run the targeted checks from a checkout with the current read-only data and C01
research inputs:

```text
python -X utf8 tools/avatars/check_successor_proposals.py --list-inputs
python -X utf8 -m unittest discover -s tools/avatars -p test_successor_proposals.py
```

`capture_sources.py` is an optional explicit network recapture, not part of the
validator. Its output must remain a separately dated observation. Do not replace
these reviewed captures or claim every successful body proves every claim.
