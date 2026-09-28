# CLAUDE-C01-32: Pan Africanist Congress presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (28 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [south-africa-pac-presidents-1990-2026-32.md](../../campaign-certification/C01/research/south-africa-pac-presidents-1990-2026-32.md).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `SouthAfrica/za_pac`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-za-32`. **Stacked on `claude/c01-za-30`** (a pending, unmerged packet that also edits `south-africa.json`), merged with current integration `032cd6a3` and again at `44098c5a` (merge `4fe2f1ba`): merge that packet first. Claim commit: this record's first commit on the branch (`a809bcba`).

## Bounded deliverable

Add one party role, za_pac_president (President of the Pan Africanist Congress of Azania; kind party_leader), to the existing IEC observation za_iec_n2024_039. Research its presidents from 1 January 1990 to the cutoff — at most ten people (expected Clarence Makwetu, Stanley Mogoba, Motsoko Pheko, Letlapa Mphahlele, Alton Mphethi, Luthando Mbinda, Narius Moloto, Mzwanele Nyhontso). Leadership disputes and rival claimants, expulsions and court orders are distinct dated claims; a disputed presidency is recorded as disputed, never resolved by inference. Sources: the PAC's own records, IEC records, court judgments (SAFLII) and Parliament only where they record the party office.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-pac-presidents-1990-2026-32.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_pac_presidents_c01_32.py`, and pinned counts or exact sets in `test_south_africa_research_s10h.py`, `test_south_africa_heads_of_state_c01_09.py`, `test_south_africa_anc_presidents_c01_16.py`, `test_south_africa_deputy_presidents_c01_21.py`, `test_south_africa_party_leaders_c01_30.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SouthAfrica, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 28 September 2026. Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-32.md` (this record);
- `docs/campaign-certification/C01/research/south-africa-pac-presidents-1990-2026-32.md` (new report);
- `docs/campaign-certification/C01/research/south-africa.json`;
- 38 new extracts `docs/campaign-certification/C01/research/sources/south-africa-pac-*-facts.json` (no existing extract is
  edited);
- `tools/avatars/test_south_africa_pac_presidents_c01_32.py` (new), and pinned values in
  `tools/avatars/test_south_africa_research_s10h.py`, `tools/avatars/test_south_africa_heads_of_state_c01_09.py`,
  `tools/avatars/test_south_africa_anc_presidents_c01_16.py`, `tools/avatars/test_south_africa_deputy_presidents_c01_21.py`
  and `tools/avatars/test_south_africa_party_leaders_c01_30.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with other
  pending packets apart from the stacked CLAUDE-C01-30 files).

The packet gains 38 sources and 65 claims and one `party_leader` role, `za_pac_president`, on the PAN AFRICANIST CONGRESS
OF AZANIA observation (`za_iec_n2024_039`). The role holds 14 holder observations of six people, each dated by a PAC
in-office attestation (one by a High Court judgment's description of the office), with no start and no end, because no
record found states the day any President assumed or left the office. Six observations fall in years when two people
claimed the presidency and are marked disputed; no dispute is resolved by inference:

- Stanley Mogoba: 1998-05-19, 1998-06-16, 1998-10-12, 1998-11-03.
- Letlapa Mphahlele: 2008-09-20, 2008-10-03.
- Alton Mphethi: 2014-03-21 (disputed: Mphahlele's continuing claim after his expulsion in May 2013).
- Luthando Mbinda: 2016-01-31 (disputed: the same claim).
- Narius Moloto: 2018-05-25 (disputed: Mbinda's claim after his expulsion, effective 13 June 2017 by the PAC's account),
  2019-07-12 (the High Court's "current President", in an application the PAC brought through its Secretary General; the
  judgment's only stated basis is the consent order of 8 March 2019).
- Mzwanele Nyhontso: 2020-02-15, 2021-07-19 and 2021-12-31 (disputed: Moloto's rival election at Limpopo on 24-25 August
  2019 and the litigation to 2023), 2026-08-29 (the latest PAC record before the cutoff).

Zephania Mothopeng (President "1986 - 1990" in the PAC's lists, a retrospective range that states no day, and not in the
expected list), Clarence Makwetu and Motsoko Pheko appear only in retrospective lists, election references and an undated
manifesto: no dated record of them in office was found in the packet's source classes. Nine people are named as
Presidents, within the limit of ten; the EFF's leader and a deputy are named only incidentally in quoted text.

Verifier fixes, with the integrator's rulings (commit "Apply verifier fixes to CLAUDE-C01-32"):

- **Ruling: the PAC post of 19 January 2018 is republished news text.** It repeats word for word, without credit, the
  Political Analysis South Africa article "PAC case against Mbinda, for parliamentary seat to be heard in March 2018"
  (byline Mzoxolo Mpolase, published 18 January 2018; read in the Internet Archive capture 20230109165609). Like
  CLAUDE-C01-28's republished reference text, the source `za_pac_post_case_against_mbinda_20180119` is typed
  `party_republished_news_text`, its scope note names the original, and its four claims are kept as claims only; none
  feeds a holder. Moloto's 2018-01-19 holder observation is removed; his 2018-05-25 observation now says no PAC record of
  his election was found (he was styled Secretary General on 4 October 2017), and Mphethi's end rests only on the later
  "expelled" reference.
- **Ruling: Mphahlele's 2009-01-14 holder is demoted.** The date was only the page's Joomla "Last Updated" edit stamp (the
  same site stamps statements of 2008 "Last Updated ( Friday, 12 December 2008 )"); the statement is now an undated
  continuation claim with the stamp as its `printed_range`, its source has no `published_date`, and its "No end" text
  moved to the 2008-10-03 observation.
- **Ruling: Moloto's 2019-07-12 observation stays undisputed**, on the court's description in the PAC's own application;
  the report now says so exactly.
- Other corrections: the census failure is disclosed in the report's Checks; the Mothopeng range, the people count, a
  wrong normalisation note ("Mpinda"), the leave-to-appeal day, the second integration merge and two spacing slips
  ("20th July", "2019: PAC", checked against the sources) are corrected; the 4 October 2017 source's scope note says it
  reproduces a Political Analysis South Africa interview; the 2021-12-31 observation says the pending appeal concerned
  the order of 12 July 2019 and that the appeal court records the 2021 declarations as unappealed. Sources and claims stay
  38 and 65; holder observations fall from 16 to 14 and disputed observations from 7 to 6.

Decisions:

- **ZA-PAC-01 and ZA-PAC-04 unresolved:** no dated PAC, IEC, Parliament or court record of 1990-1997 (Mothopeng, Makwetu)
  or of Pheko (2003-2006); the PAC's two retrospective lists conflict on Mogoba ("Bishop L. Mokgoba (1999 - 2003)" against
  "1996 - 2003"), kept open.
- **ZA-PAC-03 accepted; ZA-PAC-02 and ZA-PAC-05 to ZA-PAC-11 accepted in part:** elections (December 1996; 25 September
  2006; 6 July 2008; 28 September 2014; the rival congresses of 24-25 and 29-30 August 2019), result publications, acting
  service (Mphethi, undated), the Secretary General's office (Moloto, 4 October 2017), the republished news text of 19
  January 2018, the undated 2009 statement, suspensions, expulsions (including Mbinda's, stated to have taken effect on
  13 June 2017), rival claims, court orders (20 April 2016, June 2016, 8 March 2019, 12 July 2019, 23 August 2021, 27
  October 2023) and the Supreme Court of Appeal's finding that Moloto "has not been re-elected as the President since
  2020" are claims and never starts or ends; the 2016 leave-to-appeal judgment prints
  its own day two ways (21/6/2016 and 20 June 2016) and is stored undated.
- **Separation:** parliamentary seats (Mogoba 1998, Mbinda 2015-2018, Nyhontso) and Nyhontso's ministerial office are
  never used; the 29 June 2016 sentence naming Mbinda's swearing-in as MP with the party office is a continuation claim
  only; Deputy President and Secretary General are other offices; no claim or source is shared with any other role.
- **Sources:** 32 PAC pages (www.paca.org.za 1998-2004, www.pac.org.za/pac.org.za 2008-2019, pacofazania.org.za and
  pacofazania.org 2019-2022; one, the post of 19 January 2018, is republished news text, typed
  `party_republished_news_text` and non-primary), two posts of the PAC's X account @MyPAConline (archived JSON), and four SAFLII judgments,
  all as raw Internet Archive captures made before the cutoff. A Truth and Reconciliation Commission transcript (7 October
  1997) and a UN record are leads outside the source classes. SAFLII, lawlibrary.org.za, DISA and the UN Digital Library
  block direct requests; nothing was done to pass them.

The packet-level coverage note is inserted after the CLAUDE-C01-30 note and before the CLAUDE-C01-21 and CLAUDE-C01-16
notes, which their tests pin as the last two entries.

Integration: the branch is stacked on `claude/c01-za-30` (tip `cb0f1153`) with `032cd6a3` merged (`73af5379`); merge
CLAUDE-C01-30 first. Before committing, `codex/campaign-certification` had moved to `44098c5a` (diagnostic evidence only)
and was merged without conflict (`4fe2f1ba`); `claude/c01-za-30` had not moved. The claim is not registered in the task
queue; that is left for Codex. New South Africa totals: 245 sources, 471 claims, 11 role observations; `mapping_pending`
stays 53 and the six discovery batches stay open.

## Checks

Run from the worktree with `PYTHONDONTWRITEBYTECODE=1` and `python -X utf8` (rerun in full after the verifier fixes, with
the same results):

- `tools/avatars/campaign_research.py` (regenerated), then `--check`: passed. 9 country packets, 1,648 sources, 4,311
  claims, 93 discovery batches; South Africa 245 sources, 471 claims, 11 role observations.
- `tools/avatars/campaign_census.py --check`: **exit 1, a pre-existing failure unrelated to this packet**: "C01 evidence
  differs: .../C01/census.json", because `spheres-sim/src/government.rs` changed in `262d5f61` (already in the base
  `032cd6a3` and in `44098c5a`) without `census.json` being regenerated (recorded 848,551 bytes, current 849,546). This
  packet touches no census input and `census.json` is outside its allowed files; it is left for Codex.
- `-m unittest discover -s tools/avatars -p "test_south_africa*.py"`: 50 tests OK (the new test's 8 included, with its 7
  validator and 45 invariant mutation cases all rejected, each by the rule it targets).
- `-p "test_*research*.py"`: 79 tests OK. `-p "test_campaign*.py"`: all 16 campaign tests ran and passed (including
  `test_campaign_census`; `spheres-sim/data` is present in this sparse worktree).
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 passed.
- `python tools/planning/workboard.py --check`: PASS.
- `git diff --check` on the touched paths: clean. The gap ledger was not touched.
- Known failures outside the listed checks, not fixed: `tools/avatars/test_certified_gap_ledger.py` errors on a new
  packet's sources ("no pinned attribution"; on this stacked branch it stops first at the pending CLAUDE-C01-30 source
  `za_acdp_party_history_page_2004`) until Codex classifies the packets' commits in `COMMIT_PACKETS` at integration;
  `tools/avatars/test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which the sparse checkout lacks
  ("Required input is missing: spheres-web/src/person_avatar_assets.rs"), and in a full checkout reports the packet as
  `unclassified_packet` with stale boundary-matrix files, so Codex must list the packet and regenerate
  `docs/campaign-certification/S23/preparation/boundary-matrix/` on integration.
