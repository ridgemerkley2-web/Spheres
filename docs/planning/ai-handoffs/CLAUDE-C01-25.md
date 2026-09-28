# CLAUDE-C01-25: Saudi Shura Council and Allegiance Commission chairs, 1990–2026

Owner: Claude. State: **ready_for_review** (claimed 25 September 2026; submitted 27 September 2026; not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 25 September 2026 instruction to start another
batch of five packets in parallel. It does not repeat accepted C01-01/02/03/04/07/08 or reclaim
the pending C01-05, C01-06 and C01-09 to C01-22. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-sa-25`. First stacked on CLAUDE-C01-06 (`claude/c01-saudi-06` at `7948ab98`, merged with integration
`ffe54b02` at `8fdc2e6e`) because both packets edit `saudi-arabia.json`; CLAUDE-C01-06 has since been integrated, so no
merge order remains. Claim commit: `5b80d442`, this record's first commit on the branch. Base: current integration
`a33a8987` (`codex/campaign-certification`), merged at `c7caa2b8` before the work; a fetch on 27 September 2026 found no
newer integration commit. Result commits: the packet commit and the separate index commit at the head of
`claude/c01-sa-25` at submission; to be recorded by the integrator. Reviewer/integrator: Codex.

## Bounded deliverable

Fill two existing empty roles: `sa_shura_chair` (Chairman of the Shura Council) of `sa_shura`, and `sa_succession_chair` (Chairman of the Allegiance Commission) of `sa_succession_commission`. Review at most ten observations between 1 January 1990 and the cutoff:

1. The Shura Council Law of 1992 and the council's formation, as procedure only.
2. The first Chairman's appointment (the council's first term).
3. The Chairman's reappointments across terms.
4. The 2009 appointment of a new Chairman.
5. Later appointments (2013 onward).
6. The Chairman at the council's current term.
7. The Allegiance Commission Law of 2006 and the commission's formation, as procedure only.
8. The Allegiance Commission's first Chairman.
9. Later Allegiance Commission Chairmen.
10. Official attestations of both chairs before the cutoff.

Keep a royal order of appointment, its stated effective day, the start of a council term, a relief from office and a death as distinct dated claims. The C01-06 king, crown-prince and prime-minister holders do not change. Never infer an end from a successor's start unless a source states it. Give a holder `from` only where a source states the day office was assumed or took effect, and `until` only where a source states the day the office ended; otherwise record `attested_on`. Constitution and statute texts may establish procedure only, never a date. Retrospective lists are claims, never boundaries. News, encyclopaedias and history sites are leads only. The historical cutoff stays 7 September 2026.

Primary sources are required: the Saudi Press Agency (spa.gov.sa, including archived pages), the Shura Council (shura.gov.sa), the Bureau of Experts at the Council of Ministers (laws.boe.gov.sa) and Umm al-Qura, the official gazette.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/saudi-shura-allegiance-chairs-1990-2026-25.md`;
- `docs/campaign-certification/C01/research/saudi-arabia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/saudi-arabia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_saudi_shura_allegiance_c01_25.py`, and pinned counts or exact sets in `test_saudi_executive_c01_06.py` and any other test pinning the Saudi packet
  updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks: research-index `--check`; the SaudiArabia, research and campaign Python tests; the atlas Node check;
`workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/saudi-shura-allegiance-chairs-1990-2026-25.md):
`saudi-shura-allegiance-chairs-1990-2026-25.md`. Built from three research dossiers (SA-CHR-01 to 03, 04 to 06, 07 to 10)
and an independent adversarial check of each. Every recorded response was re-downloaded twice on 27-28 September 2026
(UTC), at least 30 minutes apart (SPA responses the second time with a cache-busting query served by the origin); all 71
matched the byte count and SHA-256 the dossiers and checks recorded.

Observation decisions: SA-CHR-01, 03, 04, 05, 07, 08 and 10 accepted; SA-CHR-02 (no order of the first Chairman's
appointment read), SA-CHR-06 (continuity to the cutoff not attested) and SA-CHR-09 (no chairman after May 2017; absence
not proven) accepted in part.

Holders on `sa_shura_chair` (three tenures, each followed by its dated holder observations: 9, 5 and 12):
Muhammad bin Ibrahim bin Jubair (attested 1997-07-06, until 2002-01-24, the Royal Court's stated day of death), Saleh bin
Abdullah bin Humaid (attested 2002-02-07, the decree; from 2002-02-11, his stated first day as Chairman) and Abdullah bin
Mohammed bin Ibrahim Al Al-Sheikh (attested 2009-02-14, A/13; from 2009-02-28, its stated effective day). On
`sa_succession_chair`: Mishaal bin Abdulaziz Al Saud (attested 2007-12-09, A/180; until 2017-05-03, stated day of death;
three observations). Forming orders naming the sitting Chairman again, extensions and renewals are continuations, never
new holders; acting presiding by a Vice Chairman, oaths, term starts and openings, law texts, retrospective statements,
translations, republications and calendar anchors are claims only.

Decisions for the integrator: the Royal Embassy of Saudi Arabia's English releases (1997-2002) are treated as official
Saudi records; if that is refused, Jubair's `until` reverts to null and Humaid's `attested_on` moves to 11 February 2002.
The two Hijri-to-Gregorian `from` values rest on SPA's own equations (28/11/1422 = 11 February 2002; 3 and 4 Rabi
al-Awwal 1430 = 28 February and 1 March 2009), with a stated fallback to null.

All 32 checker defects are handled (all applied), and 20 of the 27 missing primary records the checks found are imported;
the other seven are two duplicate pages, three optional English companions and two unreachable texts.

Touched paths: this record; `docs/campaign-certification/C01/research/saudi-arabia.json` (71 sources, 108 claims, four
holder tenures and 29 holder observations on the two roles, a scope note on each role, four coverage notes on `sa_shura`,
two on `sa_succession_commission` and one packet coverage note; existing content unchanged); 71 new
`docs/campaign-certification/C01/research/sources/saudi-arabia-*-facts.json` extracts (no existing extract edited, and the
Bush Library source record and its extract, under repair by CLAUDE-C01-SOURCE-06, untouched); new
`docs/campaign-certification/C01/research/saudi-shura-allegiance-chairs-1990-2026-25.md`; new
`tools/avatars/test_saudi_shura_allegiance_c01_25.py`; `tools/avatars/test_saudi_executive_c01_06.py` (new exact totals,
the C01-06 source-order guard re-expressed and the packet-wide `until` guard extended to the two pinned deaths; none
loosened). Separate commit: `docs/campaign-certification/C01/research-index.json` only.

Checks (27-28 September 2026): research-index regeneration and `--check` (1,400 sources, 3,839 claims); `campaign_census.py
--check` (exit 0); the Saudi tests (17, 9 of them new); the research tests (79); the campaign tests (16, census included;
the sparse checkout has the game data and was not widened); the atlas Node check (11); `workboard.py --check` (44
markers); `git diff --check` on this packet's paths. The new test's 45 mutations each fail on the rule they break.
