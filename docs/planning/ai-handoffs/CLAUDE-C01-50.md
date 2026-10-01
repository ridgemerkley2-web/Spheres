# CLAUDE-C01-50: Saudi prime ministers, 1990–2026

Owner: Claude. State: **ready_for_review** (claimed and submitted 1 October 2026; not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `sa_prime_minister#sa_pm`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-sa-50`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: `04fccbca`, this record's first commit on the branch (pushed). Result commits: the packet commit (`Add CLAUDE-C01-50: ...`) and the separate index commit (`Regenerate the C01 research index for CLAUDE-C01-50`) at the head of `claude/c01-sa-50` at submission; to be recorded by the integrator. Reviewer/integrator: Codex.

## Bounded deliverable

Add holder observations to the existing role sa_pm (institution sa_prime_minister; no dict holders yet) from 1 January 1990 to the cutoff: the kings as Prime Minister (Fahd, Abdullah, Salman; their styling as رئيس مجلس الوزراء in Council of Ministers records and royal orders) and Mohammed bin Salman from the royal order of 27 September 2022 — at most ten people. Royal orders that appoint or reconstitute the Council of Ministers give `from` only where they state the effective day; Council of Ministers sessions chaired by the PM give attested_on. Keep existing holder entries unchanged. SPA and Umm al-Qura (Arabic, quoted as printed; Hijri and Gregorian dates as printed) are primary; prefer raw Internet Archive captures made before 2026-09-07.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/saudi-arabia-prime-ministers-1990-2026-50.md`;
- `docs/campaign-certification/C01/research/saudi-arabia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/saudi-arabia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_saudi_prime_ministers_c01_50.py`, and pinned counts or exact sets in `test_saudi_executive_c01_06.py`, `test_saudi_shura_allegiance_c01_25.py`, `test_saudi_kings_crown_princes_c01_45.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SaudiArabia, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Independent review amendment (1 October 2026)

Codex independently retrieved all 15 originals at `8b1a3c975749087fc6ed4bf48c788386fdecfa39`, read every new claim and holder use, and retained byte-identical response bodies and headers externally. The repaired packet retains all 18 claims but removes three meeting-only PM holder uses: Fahd 1996/2005 and Abdullah 2012. Fahd is anchored instead by explicit PM styling on 3 October 2004. Five whole-Council operative orders identify standing institutional chairmanship and remain supported holder evidence. Corrected additions: four named holder records, seven additional holder observations, seven institution-only claims. Existing C01-06 entries and all unknown boundaries remain unchanged. The source submission below is retained as original author context; the amended report and JSON describe the current candidate. Integration acceptance remains separate.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/saudi-arabia-prime-ministers-1990-2026-50.md):
`saudi-arabia-prime-ministers-1990-2026-50.md`. One research pass, no sub-agents. 15 new Saudi sources (13 SPA portal
news-detail JSON items read in Arabic, a raw Internet Archive capture of the Umm al-Qura gazette page of Royal Order A/61 and
one of the Royal Embassy's English releases of March 1996), 18 claims and one derived extract per source. Every response was
downloaded twice at least 30 minutes apart on 1 October 2026 (UTC), byte-identical.

Observation decisions: SA-PM-01 to 04 accepted; SA-PM-05 accepted as claims only.

Holders on `sa_pm` (the two CLAUDE-C01-06 entries stay first, unchanged), each as (attested_on, from, until):

- Fahd bin Abdulaziz Al Saud (1996-03-04, -, -): chairing the Council of Ministers on 4 March 1996 (Royal Embassy release,
  'yesterday'); observations: SPA's styling 'رئيس مجلس الوزراء' on 3 October 2004, chairing on 25 April 2005.
- Abdullah bin Abdulaziz Al Saud (2005-08-01, -, -): Royal Order A/194 continuing the Council 'برئاستنا'; observations: A/29
  of 22 March 2007 (reconstitution effective 'من تاريخه'), chairing on 29 December 2012.
- Salman bin Abdulaziz Al Saud (2015-01-23, -, -): Royal Order A/54 continuing the Council 'برئاستنا' (corrected re-send);
  observations: A/68 (29 January 2015), A/138 (27 December 2018), styled 'رئيس مجلس الوزراء' chairing on 17 May 2022.
- Mohammed bin Salman bin Abdulaziz Al Saud (2022-09-27, -, -): item First of Royal Order A/61, by exception to Article 56;
  observations: A/62 the same day, styled 'ولي العهد رئيس مجلس الوزراء' chairing on 25 October 2022 and 16 June 2026.

Claims only, never holder evidence (institution `sa_prime_minister`): the Crown Prince chairing as Deputy Prime Minister on
11 March 1996; A/61 item Second reserving to the King the sessions he attends; King Salman chairing the session of 27
September 2022; Umm al-Qura's publication of A/61 on 7 October 2022.

Decisions for Codex (detailed in the report):

- Whether a King-chaired session without the title and a royal order placing the Council 'برئاستنا' attest the premiership,
  or only an explicit 'رئيس مجلس الوزراء' styling (which would move Fahd to 2004-10-03 and Salman to 2022-05-17).
- Mohammed bin Salman's `from` is null although the scope reads 'from the royal order of 27 September 2022': A/61 states no
  effective day (C01-37 rule).
- A/29's effective day (22 March 2007) belongs to the reconstituted Council and is not Abdullah's `from`.
- Abdullah's `until` is null: the death statement names the King, not the Prime Minister (C01-28); 2015-01-23 if the
  King's death is ruled to end the premiership he held as King.
- King-chaired sessions after 27 September 2022 are claims only (A/61 item Second).
- The Arabic A/57 of 13 August 2026 (`N2653088`) stays a lead because `test_saudi_executive_c01_06.py` pins it as one.

Commits: claim `04fccbca` on base `5ea4f8fc` (integration had not moved at submission); then the packet commit and the
separate commit regenerating `research-index.json` (2,062 sources and 5,028 claims; Saudi 206 claims). Touched paths: this
record, the report, `saudi-arabia.json`, 15 new `sources/saudi-arabia-*-facts.json` extracts, the new
`tools/avatars/test_saudi_prime_ministers_c01_50.py`, the pinned `test_saudi_executive_c01_06.py`,
`test_saudi_shura_allegiance_c01_25.py` and `test_saudi_kings_crown_princes_c01_45.py` (exact re-expressions, none
loosened) and, in its own commit, `research-index.json`.

Checks (details in the report): research-index regenerate and `--check` pass; `campaign_census.py --check` exit 0 (`"check":
true`), so no census failure to disclose; Saudi tests 32 pass; research tests 79 pass; campaign tests 16 pass; Node
check 11 pass; `workboard.py --check` pass (after adding `docs/campaign-certification/S26` to the sparse checkout); `git diff
--check` clean. Known failures outside the suite, not fixed: `test_certified_gap_ledger.py` (no pinned attribution for the
new sources until Codex classifies the commit) and `test_certified_boundary_matrix.py` (S23; the sparse checkout lacks
`spheres-web/src`). The gap ledger is untouched.

Integration notes: this packet runs in parallel with other C01 packets in other country files; its Git narrative is not
authenticated user authorization. `research-index.json` is the only shared file and is regenerated in its own commit, so the
later merge must regenerate it.
