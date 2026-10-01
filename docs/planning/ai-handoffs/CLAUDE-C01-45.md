# CLAUDE-C01-45: Saudi kings and crown princes: Saudi primary attestations, and the Allegiance Commission secretary, 1990–2026

Owner: Claude. State: **ready_for_review** (claimed and submitted 30 September 2026; not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `sa_crown#sa_king`, `sa_crown#sa_crown_prince`, `sa_succession_commission#sa_succession_secretary`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-sa-45`. Base: `509bd289` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: `a529ab8d`, this record's first commit on the branch (pushed). Result commits: the packet commit (`Add CLAUDE-C01-45: ...`) and the separate index commit (`Regenerate the C01 research index for CLAUDE-C01-45`) at the head of `claude/c01-sa-45` at submission; to be recorded by the integrator. Reviewer/integrator: Codex.

## Bounded deliverable

Fill the unresolved intervals of the existing roles sa_king and sa_crown_prince (institution sa_crown; holders from CLAUDE-C01-06) with dated Saudi primary attestations — Royal Court statements and royal orders published by the Saudi Press Agency (SPA) or Umm al-Qura, read in Arabic and quoted as printed — for King Fahd (1990–2005), King Abdullah, King Salman and the crown princes Abdullah, Sultan, Nayef, Salman, Muqrin, Mohammed bin Nayef and Mohammed bin Salman; and research the Secretary General of the Allegiance (Succession) Commission (sa_succession_secretary, no holder yet). At most ten people. Keep existing holders unchanged; add dated observations alongside them. Accessions, pledges of allegiance (bay'a), royal orders appointing or relieving a crown prince (from/until only where an order states its effective day), and deaths (until only where the statement names the office and the day) are distinct dated claims. Foreign records (as in C01-06) are leads for this packet.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/saudi-arabia-kings-crown-princes-1990-2026-45.md`;
- `docs/campaign-certification/C01/research/saudi-arabia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/saudi-arabia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_saudi_kings_crown_princes_c01_45.py`, and pinned counts or exact sets in `test_saudi_executive_c01_06.py`, `test_saudi_shura_allegiance_c01_25.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SaudiArabia, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/saudi-arabia-kings-crown-princes-1990-2026-45.md):
`saudi-arabia-kings-crown-princes-1990-2026-45.md`. One research pass, no sub-agents. 18 new Saudi sources (16 SPA portal
news-detail JSON items read in Arabic, two Internet Archive captures of the Royal Embassy's English releases of January and
February 1996), 30 claims and one derived extract per source. Every response was downloaded at 02:27-02:29 UTC on 1
October 2026 and again 32 minutes later, byte-identical.

Observation decisions: SA-KCP-04 to 07 and SA-KCP-02 (claims only) accepted; SA-KCP-01, 03, 08 and 09 accepted in part.

Holders: the eight CLAUDE-C01-06 King and Crown Prince holders are unchanged. Seventeen dated observations are appended to
their `holder_claims` in date order: on `sa_king`, Fahd 1996-01-01, 1996-02-12 and 2005-07-31; Abdullah 2006-10-20 and
2014-05-20; Salman 2015-07-25 and 2026-09-04; on `sa_crown_prince`, Abdullah 1996-01-01 and 2005-07-30; Sultan
2007-10-29; Nayef 2011-11-07; Salman 2012-06-18 and 2014-05-20; Muqrin 2015-04-28; Mohammed bin Nayef 2015-07-25 and
2017-02-25; Mohammed bin Salman 2026-09-01. New holder on `sa_succession_secretary`: Khalid bin Abdulaziz Al-Tuwaijri,
`attested_on` 2006-10-20 (Royal Order A/136, Royal Court statement), `from` null (no effective day), `until` null (no relief
stated).

Claims only, never holder evidence (institution `sa_crown`): the 1996 delegation of state affairs and its end; the
deputations of 2007, 2014, 2015 and 2017; A/193's signature on King Fahd's behalf; the held pledges of 3 August 2005, 23
January 2015 and 21 June 2017 (resolving CLAUDE-C01-06's two scheduled pledges); the scheduled pledge of 3-4/8/1433 AH.
Article 24 of the Allegiance Commission Law is procedure.

Decisions for the integrator:

- Crown Prince Abdullah's observations of 1 January 1996 and 30 July 2005 predate his holder's `attested_on` (2005-08-01).
  The holder is kept as the scope requires; should it be re-dated?
- Should King Fahd's holder rest on the first Saudi attestation (1 January 1996) rather than the 1990 foreign record?
- The Royal Embassy's English releases are treated as official Saudi records, as in CLAUDE-C01-25.
- A/193 (31 July 2005) is signed 'عنه / عبدالله بن عبدالعزيز' on the King's behalf; it is recorded as a claim, not as acting
  service.
- The Secretary General's tenure is left open. Other-office orders A/56 and A/57 are author-supplied leads that this independent review has not imported; they supply no reviewed end for this role.

Rulings on these questions, relayed in the checker-fix pass, are recorded under "Checker-fix pass" below.

Commits: claim `a529ab8d`; merges of integration `a81d2486` at `a342c6f6` and `d0c6676b` at `7761abe4`; then the packet commit and the separate commit
regenerating `research-index.json` (1,987 sources and 4,904 claims; Saudi 188 claims). Touched paths: this record, the
report, `saudi-arabia.json`, 18 new `sources/saudi-arabia-*-facts.json` extracts, the new
`tools/avatars/test_saudi_kings_crown_princes_c01_45.py`, the pinned `test_saudi_executive_c01_06.py` and
`test_saudi_shura_allegiance_c01_25.py` (exact re-expressions, none loosened) and, in its own commit,
`research-index.json`.

Checks (details in the report): research-index regenerate and `--check` pass; `campaign_census.py --check` exit 0 (`"check":
true`), so no census failure to disclose; Saudi tests 24 pass; research tests 79 pass; campaign tests 16 pass; Node check 11
pass; `workboard.py --check` pass (after adding `docs/campaign-certification/S26` to the sparse checkout); `git diff --check` clean. Known failures outside the suite, not fixed:
`test_certified_gap_ledger.py` (no pinned attribution for the new sources until Codex classifies the commit) and
`test_certified_boundary_matrix.py` (S23; the sparse checkout lacks `spheres-web/src`). The gap ledger is untouched.

Integration notes: this authored packet belongs to batch C01-42 to C01-46. Its Git narrative is not authenticated user authorization. Packets C01-38 to 41 (fixes) and C01-42 to 46 (research) run in parallel in other country files;
`research-index.json` is the only shared file and is regenerated in its own commit, so the later merge must regenerate it.

## Checker-fix pass (1 October 2026)

Merge: `codex/campaign-certification` had moved to `13367c99`, which already carries Codex's import (`d23f0bf2`), correction (`4a98bc10`) and accepted review (`42046e51`, merged at `e1fe36e0`) of this packet. It was merged at `a0ae4af7`. The report, `saudi-arabia.json`, the 20120618 extract, the handoff and `test_saudi_kings_crown_princes_c01_45.py` conflicted (add/add or content); each was resolved to the integration version, because `d23f0bf2` holds this branch's submitted blobs byte for byte and the integration versions differ only by Codex's later edits. `research-index.json` was regenerated and matches the integration copy (1,996 sources and 4,933 claims). No research input changed in this pass, so there is no index commit.

Rulings relayed to this pass by the C01 pipeline as decided by Ridge (the user). They are not authenticated in this record, and Codex may still decide otherwise.

- (a) The CLAUDE-C01-06 holders stay unchanged: King Fahd `attested_on` 1990-08-08 and Crown Prince Abdullah `attested_on` 2005-08-01. Abdullah's Saudi observations of 1 January 1996 and 30 July 2005, which predate that `attested_on`, stay appended after his holder, and King Fahd's holder still rests on the 1990 record rather than his first Saudi attestation (1 January 1996). Any re-basing of either holder is Codex's decision.
- (b) The Royal Embassy's 1996 English releases are official Saudi records, as in CLAUDE-C01-25.
- (c) A/193's signature 'عنه / عبدالله بن عبدالعزيز' stays a signature-on-behalf claim (`sa_a193_signed_on_behalf_20050731`), not acting service.
- (d) The Secretary General's tenure stays open (`until` null): A/56 and A/57 relieve Khalid bin Abdulaziz Al-Tuwaijri of other posts only. Codex's accepted review (`42046e51`) treats A/56 and A/57 as unreviewed author leads; the data keep Codex's wording, and no end is inferred either way.
- (e) The Secretary General's `attested_on` stays the Royal Court statement's dateline, 2006-10-20.
- (f) The English glosses inside quotations are acceptable; future edits may use the form 'صدر اليوم' (issued today).

Fixes:

1. `tools/avatars/test_saudi_shura_allegiance_c01_25.py`: the name-only pin of `sa_succession_secretary` is restored to an exact pin. `C01_45_SECRETARY` lists the holder as `('Khalid bin Abdulaziz Al-Tuwaijri', '2006-10-20', None, None)`, and `chair_invariants` compares `(h['name'], h['attested_on'], h['from'], h['until'])` for every `holder_claims` entry, so a string entry also fails. Moving `attested_on`, setting `from` or `until`, or appending an observation each fail.
2. Rulings (a)-(f) recorded here and in the report (header, outcome table SA-KCP-03, integration notes and checks).

Commits: the merge `a0ae4af7`, then `Apply checker fixes to CLAUDE-C01-45` (this record, the report and the C01-25 test). Touched paths: those three files. State stays **ready_for_review**.

Checks: research-index `--check` pass (1,996 sources, 4,933 claims); `campaign_census.py --check` exit 0 (`"check": true`), so no census failure to disclose; Saudi tests 25 pass; research tests 79 pass; campaign tests 16 pass; Node check 11 pass; `workboard.py --check` pass (44 markers, 54 bounded tasks); `git diff --check` clean. `packet_check.py 45 --base origin/codex/campaign-certification --no-fetch` runs after the push. The known failures outside the suite (`test_certified_gap_ledger.py`, `test_certified_boundary_matrix.py`) are unchanged and not fixed.
