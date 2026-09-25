# CLAUDE-C01-19: Russian heads of government, 1991–2026

Owner: Claude. State: **ready_for_review** (submitted 25 September 2026; not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-17. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-ru-19`. **Stacked on CLAUDE-C01-14** (`claude/c01-ru-14` at `1d749e15`), which is
ready for review and not yet integrated, because both packets edit `russia.json`: merge CLAUDE-C01-14 first. Claim
commit: `cb64061a`, this record's first commit on the branch. `claude/c01-ru-14` was fetched again before the work and
had no commits missing from this branch. Result commits: the packet commit and the separate index commit at the head of
`claude/c01-ru-19` at submission; to be recorded by the integrator. Reviewer/integrator: Codex.

## Bounded deliverable

Add one executive institution, `ru_government` (Правительство Российской Федерации — Government of the Russian Federation, kind `executive_institution`), with one role, `ru_government_chairman` (Председатель Правительства — Chairman of the Government, kind `head_of_government`), and review at most ten observations between November 1991 and the cutoff:

1. 1991–1992: who headed the RSFSR/Russian government from November 1991, and any acting chairman in 1992, as claims.
2. December 1992: Viktor Chernomyrdin's appointment after the Congress of People's Deputies' consent, and his 1996 reappointment.
3. 1998: Chernomyrdin's dismissal and Sergei Kiriyenko's appointment after the State Duma's consent.
4. 1998: Kiriyenko's dismissal and Yevgeny Primakov's appointment.
5. 1999: Primakov's dismissal, Sergei Stepashin's appointment and dismissal, and Vladimir Putin's appointment.
6. 2000–2004: Mikhail Kasyanov's appointment and dismissal.
7. 2004–2007: Mikhail Fradkov's appointment, his 2004 reappointment and his 2007 resignation.
8. 2007–2008: Viktor Zubkov's appointment and Putin's 2008 appointment.
9. 2012–2020: Dmitry Medvedev's appointments of 2012 and 2018 and the government's 2020 resignation.
10. 2020–2026: Mikhail Mishustin's appointments of 2020 and 2024, and an official attestation of the holder in office before the cutoff.

Keep the President's nomination, the parliament's consent, the appointment decree, the government's resignation or dismissal, continued duties until a new government is formed, and an acting chairman as distinct dated claims. Acting and continuing service are claims only, never holders. The presidency holders from CLAUDE-C01-05 and C01-14 do not change. Names follow the packet's convention (Russian as printed). Portal searches must be date-restricted single-document cards, never unrestricted number searches whose results grow. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the President of Russia's decrees and the Government's own records (kremlin.ru, government.ru, including archived pages), the official legal-information portal (pravo.gov.ru) and official gazettes, and the State Duma's resolutions and transcripts.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/russia-heads-of-government-1991-2026-19.md`;
- `docs/campaign-certification/C01/research/russia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/russia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_russia_heads_of_government_c01_19.py`, and pinned counts or exact sets in `test_russia_research_s10h.py`, `test_ussr_russia_transition_c01_05.py` and `test_russia_presidents_c01_14.py`
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the Russia, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/russia-heads-of-government-1991-2026-19.md):
`russia-heads-of-government-1991-2026-19.md`. Built from three research dossiers (RU-GOV-01 to 04, 05 to 07, 08 to 10)
and an independent adversarial check of each.

Observation decisions:

- RU-GOV-01 accepted: decrees 171 and 172 (6 November 1991) put the Government under the President, who heads it (no
  Chairman in the listed composition); Gaidar's acting service (decrees 633 and 1570) and decree 1569's statement that
  the post was vacant until 14 December 1992 are claims only. No holder.
- RU-GOV-02 accepted in part: Chernomyrdin attested on 14 December 1992 (decree 1567, after the Congress's approval
  4088-I) and on 10 August 1996 (decree 1152, after the presented letter, the 314-85-3 vote and consent 624-II ГД);
  decree 1146 accepts the Government's resignation (two rows added to the C01-14 source). The letters' dates and the
  9 December 1992 ballot result were not found.
- RU-GOV-03 accepted in part: decrees 281, 287 and 288 (Kiriyenko acting), three presentations, two failed votes, the
  251-25 secret ballot and the three resolutions; Kiriyenko attested on 24 April 1998 (decree 436).
- RU-GOV-04 accepted in part: decree 983 (Chernomyrdin acting), decree 987, the failed votes of 31 August and 7 September
  and Primakov's consent; Primakov attested on 11 September 1998 (decree 1087).
- RU-GOV-05 accepted in part: decree 580 and the Duma's statement; Stepashin attested on 19 May 1999 (decree 611, no
  effect clause; resolution 905 attests him in office); decree 1012; Putin from 16 August 1999 (decree 1052).
- RU-GOV-06 accepted: Kasyanov from 17 May 2000 (decree 861); decree 264 and the "former Chairman" styling are claims,
  not an end.
- RU-GOV-07 accepted in part: Fradkov from 5 March 2004 and from 12 May 2004 (decrees 300 and 610); order 608-r, decree
  585 and the acting signatures of May 2004, the 2007 resignation request and acceptance and decree 1184 are claims; no
  end (an integrator ruling is requested on the 2007 acceptance).
- RU-GOV-08 to 10 accepted: Zubkov from 14 September 2007, Putin from 8 May 2008, Medvedev from 8 May 2012 and 8 May
  2018, Mishustin from 16 January 2020 and 10 May 2024 (each on a decree in force on signing); the Government's orders
  laying down its powers (2008, 2012, 2018, 2024), decree 14 of 2020 and the acting service of 2012, 2020 and 2024 are
  claims; the 2024 submission is dated 9 May by the Duma stenogram; resolution 1125 of 3 September 2026 attests
  Mishustin in office.

Holders on `ru_government_chairman` (fifteen): Черномырдин (attested 1992-12-14, 1996-08-10), Кириенко (1998-04-24),
Примаков (1998-09-11), Степашин (1999-05-19), Путин (from 1999-08-16), Касьянов (from 2000-05-17), Фрадков (from
2004-03-05, from 2004-05-12), Зубков (from 2007-09-14), Путин (from 2008-05-08), Медведев (from 2012-05-08, from
2018-05-08), Мишустин (from 2020-01-16, from 2024-05-10). No holder has an `until`: no reviewed source states the day a
Chairman's office ended. Acting and continuing service are claims only. The presidency holders and `ussr.json` are
unchanged.

All 36 checker defects are handled: 33 applied (C4 in part), B13 and C10 resolved by not recording live pages, and C5
declined. Every missing primary record the checks found is imported, except four State Duma website news items (press
reports) and one Government order downloaded once.

Touched paths: this record; `docs/campaign-certification/C01/research/russia.json` (102 sources, 153 claims, the
institution `ru_government` with the role `ru_government_chairman` and fifteen holders, ten institution coverage notes
and one packet coverage note); 102 new `docs/campaign-certification/C01/research/sources/russia-*-facts.json` extracts;
**one edited existing extract, `docs/campaign-certification/C01/research/sources/russia-ips-ukaz-1146-19960809-facts.json`**
(CLAUDE-C01-14; two rows added, `bounded_scope` widened, one sentence appended to each of `scope_note` and `stability_check`, response identity unchanged); new
`docs/campaign-certification/C01/research/russia-heads-of-government-1991-2026-19.md`; new
`tools/avatars/test_russia_heads_of_government_c01_19.py`; `tools/avatars/test_russia_research_s10h.py`,
`tools/avatars/test_ussr_russia_transition_c01_05.py` and `tools/avatars/test_russia_presidents_c01_14.py` (new exact
totals, institution sets, access dates and work-order sizes; the C01-14 guards re-expressed to name the two C01-19 rows
exactly; none loosened). Separate commit: `docs/campaign-certification/C01/research-index.json` only.

Checks (25 September 2026): research-index regeneration and `--check` (336 sources, 2,046 claims); the Russia tests
(29, 10 of them new) and USSR tests (18); the research tests (79); the campaign tests (9 pass, and
`test_campaign_census` errors in setup because the sparse worktree has no `spheres-sim/data`; not widened); the atlas
Node check (11); `workboard.py --check` (44 markers); `git diff --check` on this packet's paths. The new test's 44
mutations each fail as intended.
