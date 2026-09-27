# CLAUDE-C01-26: Soviet heads of government and Supreme Soviet chairs, 1990–1991

Owner: Claude. State: **ready_for_review** (submitted 26 September 2026 UTC; not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-22. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-su-26`. **Stacked on CLAUDE-C01-19** (`claude/c01-ru-19` at `6f4ef2c2`), which is
ready for review and not yet integrated, because both packets edit `ussr.json`: merge CLAUDE-C01-19 first. Claim
commit: `e4fec74d`, this record's first commit on the branch. `claude/c01-ru-19` was fetched again before the work and
had no commits missing from this branch. Result commits: the packet commit and the separate index commit at the head of
`claude/c01-su-26` at submission; to be recorded by the integrator. Reviewer/integrator: Codex.

## Bounded deliverable

Add one executive institution, `su_government` (the Council of Ministers of the USSR and, from 1991, the Cabinet of Ministers, kind `executive_institution`) with one role, `su_government_head` (head of the Union government, kind `head_of_government`), and extend the existing `su_supreme_soviet_chair` role. Review at most ten observations between 1 January 1990 and 26 December 1991:

1. The Chairman of the Council of Ministers when the period opens (Nikolai Ryzhkov).
2. March 1990: the election of a new Chairman of the Supreme Soviet after the Chairman became President.
3. The 1990–1991 constitutional change creating the Cabinet of Ministers (procedure only).
4. Ryzhkov's end, only as a source states it.
5. January 1991: Valentin Pavlov's appointment as Prime Minister.
6. August 1991: Pavlov's dismissal.
7. August–December 1991: the committee that took over the government's functions and its chairman.
8. The Chairman of the Supreme Soviet's end in 1991, only as a source states it.
9. The September 1991 reorganization of the Supreme Soviet and its later chairs.
10. The last records of both offices in December 1991.

Keep the Supreme Soviet's election or consent, the appointment decree, dismissal, resignation, arrest and acting service as distinct dated claims; acting and interim service are claims only. The C01-05 USSR presidency holders and all Russia holders (C01-05, C01-14, C01-19) do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the records of the USSR Congress of People's Deputies and Supreme Soviet (Vedomosti, stenograms), decrees of the President of the USSR, Izvestia and Pravda only as official publication of an act, and archival official records.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/ussr-government-and-supreme-soviet-1990-1991-26.md`;
- `docs/campaign-certification/C01/research/ussr.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_ussr_government_supreme_soviet_c01_26.py`, and pinned counts or exact sets in `test_ussr_research_s10h.py` and `test_ussr_russia_transition_c01_05.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the USSR, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/ussr-government-and-supreme-soviet-1990-1991-26.md):
`ussr-government-and-supreme-soviet-1990-1991-26.md`. Built from three research dossiers (SU-GOV-01 to 03, 04 to 06, 07 to
10) and an independent adversarial check of each; research access 26 September 2026 (UTC).

Observation decisions:

- SU-GOV-01 accepted in part: "Н. Рыжков" signs joint USSR-RSFSR resolutions of 24 December 1989 and 12 January 1990 (RSFSR
  government gazette, a check find) and resolution 525 of 26 May 1990; holder attested 12 January 1990; no record dated
  1 January 1990.
- SU-GOV-02 accepted in part: the GARF original of 1362-I (15 March 1990) elects the President (a `su_president` role claim;
  holders unchanged) and is signed "(А.Лукьянов)" as Chairman; holder attested 15 March 1990; the signatures of 27 December
  1990 (1870-I, a check find) and 14 January 1991 are claims; the election resolution 1367-I is a lead.
- SU-GOV-03 accepted in part (procedure only): Law 1861-I (Rada text and the Russian text in the Congress stenogram) renames
  the chapter, rewrites Articles 128-136 and sets the Premier's approval, release and no confidence; Law 1862-I keeps the
  Council of Ministers' powers until the new bodies are formed. Law 2033-I is a lead.
- SU-GOV-04 accepted in part: the last signature (24 November 1990, holder), the illness reports and telegram (26-27 December
  1990), the Minister of Finance's report for the government and a Deputy Chairman's signature (10 January 1991); no end and no
  acting Chairman stated.
- SU-GOV-05 accepted: the Supreme Soviet's approval of 14 January 1991 (Izvestia No. 13; a dynamic page rendering with its
  recorded request); holder attested 14 January 1991, no `from`.
- SU-GOV-06 accepted in part: order 943р (19 August 1991; the last signature, holder, and the interim direction entrusted to
  the First Deputy Premier, a claim), УП-2443, УП-2444, the agenda acts, 2366-I, УП-2461 and 2367-I, the session report and
  2371-I; no `until` (integrator ruling requested: 22 or 28 August).
- SU-GOV-07 accepted in part: the Committee for Operational Management of the National Economy and the Inter-republican and
  Interstate Economic Committees, and their head's signatures, as claims on `su_government` only; no holder.
- SU-GOV-08 accepted: signatures of 19, 21 and 22 August 1991 (holder attested 22 August), the removal from chairing and its
  approval, the chamber chairmen presiding, the suspension, the statement of 24 August, the consent to arrest, the Congress's
  vote and resolution 2389-I (4 September); no `until` (integrator ruling requested).
- SU-GOV-09 accepted in part: Law 2392-I (gazette and GARF original) creates two chambers and no Chairman; УП-2663; the chamber
  chairmen's October elections are leads.
- SU-GOV-10 accepted in part: ГС-13, the latest committee acts, RSFSR decree 299 and RSFSR resolution 2017-I; no lifecycle end.

Holders. `su_government_head` (four): Николай Иванович Рыжков (attested 1990-01-12, 1990-11-24), Валентин Сергеевич Павлов
(attested 1991-01-14, 1991-08-19). `su_supreme_soviet_chair` gains two after the unchanged CLAUDE-C01-05 observation of
1990-03-14: Анатолий Иванович Лукьянов (attested 1990-03-15, 1991-08-22). No holder has a `from` or an `until`. Acting,
interim and presiding service and the interim committees are claims only.

All 31 checker defects are handled: 29 applied and B13 and C12 applied in part. Every obtainable missing primary record the
checks found is imported; three primary records found by this packet on the legal portal are added (resolutions 1177 and 27,
order 943р).

Touched paths: this record; `docs/campaign-certification/C01/research/ussr.json` (31 sources, 77 claims; the institution
`su_government` with the role `su_government_head` and four holders; `su_supreme_soviet` and its role
`su_supreme_soviet_chair` extended with sources, claims, a scope note, two holders and four coverage items; `su_president`
gains one role claim and one source and `su_presidency` one coverage item, their holders unchanged; one packet coverage item);
31 new `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts (no existing extract edited); new
`docs/campaign-certification/C01/research/ussr-government-and-supreme-soviet-1990-1991-26.md`; new
`tools/avatars/test_ussr_government_supreme_soviet_c01_26.py`; `tools/avatars/test_ussr_research_s10h.py` and
`tools/avatars/test_ussr_russia_transition_c01_05.py` (new exact totals, institution set, access dates, PDF set and
work-order figures; the C01-05 vedomosti.sssr.su guard re-expressed to allow only scanned issue PDFs; none loosened); and,
outside this record's list, `tools/avatars/test_russia_heads_of_government_c01_19.py` (its one guard that no other
head-of-government role exists re-expressed as `[ROLE, 'su_government_head']`, required by the new role). `russia.json` is
unchanged. Separate commit: `docs/campaign-certification/C01/research-index.json` only.

Checks (26 September 2026 UTC): research-index regeneration and `--check` (367 sources, 2,123 claims); the USSR tests (27, 9
of them new) and the Russia tests (29); the research tests (79); the campaign tests (9 pass, and `test_campaign_census` errors
in setup because the sparse worktree has no `spheres-sim/data`; not widened); the atlas Node check (11); `workboard.py
--check` (44 markers); `git diff --check` on this packet's paths. The new test's 41 mutations each fail as intended.
