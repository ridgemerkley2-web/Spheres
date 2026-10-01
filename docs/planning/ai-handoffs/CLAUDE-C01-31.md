# CLAUDE-C01-31: Komeito representatives, 1990–2026

Owner: Claude. State: **ready_for_review** (30 September 2026; not complete, pending Codex acceptance). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Japan/jp_komeito`, `Japan/jp_komeito/jp_komei_1994`, `Japan/jp_komeito/jp_komeito_1998`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-jp-31`. Base: `ff01ee61` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, jp_komeito_representative (代表 / 委員長; kind party_leader), to the existing 公明党 election-list observation jp_sangiin_pr_2025_15. Research the party's leaders from 1 January 1990 to the cutoff (expected 石田幸四郎; the 1994 division into 公明 and the New Frontier Party and its 1998 re-formation as claims about the organization only; then 神崎武法, 太田昭宏, 山口那津男, 石井啓一, 斉藤鉄夫) — at most ten people. Each party-congress election, assumption and resignation is a distinct dated claim; the 1994-1998 organizations are never merged into one identity. Sources: Komeito's own records (komei.or.jp and archived pages); Diet minutes only where a party officer's statement records the party office. Names in Japanese as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-komeito-representatives-1990-2026-31.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_komeito_representatives_c01_31.py`, and pinned counts or exact sets in `test_japan_research_s10d.py`, `test_japan_prime_ministers_c01_12.py`, `test_japan_prime_ministers_c01_13.py`, `test_japan_ldp_presidents_c01_18.py`, `test_japan_sdp_chairs_c01_29.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Japan, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Integration registration — 30 September 2026

The original claim above is now registered in the integrated task queue as
`claimed`; its earlier unregistered statement describes submission time.
The exact authored claim remains available at `52b23d5961ea13355ab1e3edd497be0920b6bea6`.
The gap ledger reserves all three named targets. This registers work in
progress and imports no historical research, runtime mapping or artwork.

## Result (ready_for_review)

Claim commit `52b23d59` (this record only) on base `ff01ee61`; not stacked. Before the packet commits,
`origin/codex/campaign-certification` was fetched and merged twice: at `aba3902b` (merge commit `1fce7fd0`, no conflict) and at
`02d2c5a2` (merge commit `04ffe2d9`), where the only conflict was an add/add in this record: the integration branch's copy is the
claim record plus Codex's appended registration note, so it was taken unchanged and this section appended after it. The
integration branch changed no Japan research file, Japan test or `research-index.json`. Then the packet commit and the
research-index commit on `claude/c01-jp-31`. Report:
[japan-komeito-representatives-1990-2026-31.md](../../campaign-certification/C01/research/japan-komeito-representatives-1990-2026-31.md).

Touched paths (all inside this record's allowed files):

- `docs/planning/ai-handoffs/CLAUDE-C01-31.md` (this record: state line and this section);
- `docs/campaign-certification/C01/research/japan-komeito-representatives-1990-2026-31.md` (new report);
- `docs/campaign-certification/C01/research/japan.json` (additions only: 31 sources and 64 claims appended after CLAUDE-C01-29's;
  on the 公明党 organization observation `jp_sangiin_pr_2025_15`, the new role `jp_komeito_representative` (its `roles` was empty),
  its `sources` and `claim_ids` extended and one coverage note appended; one packet coverage note appended; no existing text changed);
- 31 new `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts, one per new source (no existing extract
  edited);
- `tools/avatars/test_japan_komeito_representatives_c01_31.py` (new);
- `tools/avatars/test_japan_research_s10d.py`, `tools/avatars/test_japan_prime_ministers_c01_12.py`,
  `tools/avatars/test_japan_prime_ministers_c01_13.py`, `tools/avatars/test_japan_ldp_presidents_c01_18.py` and
  `tools/avatars/test_japan_sdp_chairs_c01_29.py` (pinned counts, exact source lists, holder lists, access dates, role counts and
  coverage-note positions re-expressed exactly; the starts guard extended exactly; none loosened, no assertion removed);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit).

The packet adds the party role `jp_komeito_representative` (委員長 / 代表, kind `party_leader`) with fifteen holder
observations of seven people and no new organization or institution; `jp_pm`, the LDP presidency, the SDP chair and every other
role are unchanged. Holders, in order: 石田幸四郎 observed 29 January 1993 (the secretary-general 市川雄一's Diet statement naming
我が党の石田委員長); 神崎武法 observed 8 November 1998 (the party organ's report of the founding convention of the new 公明党), 3
November 2002 and 31 October 2004 (re-elections); 太田昭宏 from 30 September 2006 (his own address: 本日の党大会において…公明党代表に就任
させていただきました); 山口那津男 observed 8 September 2009 (selection by the national meeting of representatives) and at the
conventions of 22 September 2012, 21 September 2014, 17 September 2016, 30 September 2018, 27 September 2020 and 25 September 2022;
石井啓一 observed 28 September 2024; 斉藤鉄夫 observed 9 November 2024; 竹谷とし子 observed on 14 March 2026 (her own address: ただいま…公明党
代表の大任を担わせていただくことになりました). Only 太田昭宏 has a stated start; the reviewed 竹谷 statement does not separately state an effective assumption date; no holder has a stated end. The 1994 acting
chair (委員長代行), the 代表代理 styled 「代表」 of 22 January to 14 March 2026, other organizations' offices (公明代表, 中道改革連合共同代表),
selections, recommendations, resignation announcements, predecessor references, continuations and retrospective records are claims
only. The December 1994 division, 公明, 公明新党, 新進党, 新党平和, 黎明クラブ, the November 1998 merger and 中道改革連合 are claims about
the organizations: no identity is merged or mapped.

Observation decisions:

| ID | Decision |
|---|---|
| KOMEITO-01 | Accepted in part: 石田幸四郎 observed 29 January 1993; continuation 16 June 1994 with an acting chair (claim); May 1989 assumption recalled (month only); no start or end |
| KOMEITO-02 | Accepted in part (organization claims only): 公明's page and the 2003 party history record the 5 December 1994 division, 公明, 公明新党, the 新進党 (10 December 1994; dissolved 27 December 1997), 新党平和 and 黎明クラブ; retrospective; no holder |
| KOMEITO-03 | Accepted in part: the 1998 report (merger form; name decided 24 October; officers confirmed) and his Diet statement dating the merger to 7 November (organization claim); 神崎武法 observed 8 November 1998 |
| KOMEITO-04 | Accepted in part: 神崎武法 observed 3 November 2002 and 31 October 2004; the 2000 re-election not found; no end |
| KOMEITO-05 | Accepted in part: 太田昭宏 elected and from 30 September 2006; the 2008 re-election not found; no end |
| KOMEITO-06 | Accepted in part: 山口那津男 chosen and observed 8 September 2009; re-elected and observed 22 September 2012; 2010 not found |
| KOMEITO-07 | Accepted in part: 山口那津男 re-elected and observed 2014, 2016, 2018, 2020 and 2022; no end |
| KOMEITO-08 | Accepted in part: 石井啓一 observed 28 September 2024; resignation intention 31 October 2024 (not an end) |
| KOMEITO-09 | Accepted in part: 斉藤鉄夫 recommended 7 November, elected and observed 9 November 2024; 中道改革連合 founded 22 January 2026 (organization claim; his co-representative office is another office); called 前代表 22 January 2026; no end |
| KOMEITO-10 | Accepted in part: 竹谷とし子 代表代理 styled 「代表」 from 22 January 2026 (claims only); recommended 11 March; elected and observed on 14 March 2026, without an inferred effective start |

Decisions (the report's table gives each): the interim 代表代理 is claims only (the pipeline's ruling on party-labelled interim
periods); 太田昭宏's explicit statement of having taken office at that day's convention supports a start; 竹谷とし子's acceptance of confidence and duty supports a dated attestation, while statements of being chosen or of continuing after a re-election are not; the 1998 and 2002
holders are dated by the page's date line because the pages print no convention day; 神崎武法's Diet statement of 30 November 1998
is used only for the organization claim (the record identifies him only as speaking for his group). Every recorded identity was
downloaded by the researcher on 30 September 2026 (21:40Z-22:31Z) and again by this packet at 22:26:11Z-22:28:03Z and 22:59:10Z-23:01:04Z
(UTC), the 7 Diet responses also with a cache-busting query at 23:01:08Z-23:01:30Z, with the same byte count and
SHA-256 each time. The Internet Archive refused connections for a few minutes at 22:02Z; requests were then made one at a time.
No terms, logins or CAPTCHAs were met. The central task-queue entry is Codex's to update.

Checks run on 30 September 2026 in this sparse worktree (not widened), with `PYTHONDONTWRITEBYTECODE=1`, after the merge:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets, 844 organization and 36 institution observations, 1,917 sources, 4,791 claims, 93 open batches (Japan: 503 sources, 863 claims, 7 role observations).
- `python -X utf8 tools/avatars/campaign_census.py --check`: exit 0 (it passes on this branch after the merge).
- Japan tests (`-p "test_japan*.py"`): 54 pass (9 in the new `test_japan_komeito_representatives_c01_31.py`, whose mutation test rejects 29 rule mutations, 7 validator mutations, 5 row mutations and 3 event re-datings; 10 in `test_japan_sdp_chairs_c01_29.py`, 9 in `test_japan_ldp_presidents_c01_18.py`, 9 in `test_japan_prime_ministers_c01_13.py`, 9 in `test_japan_prime_ministers_c01_12.py` and 8 in `test_japan_research_s10d.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass. Campaign tests (`-p "test_campaign*.py"`): all 16 pass.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: PASS.
- `git diff --check` on the packet's paths: clean.
- `python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 31`: run after the push; result in the return note.

Known failures outside the listed checks (disclosed, not fixed): `tools/avatars/test_certified_gap_ledger.py` errors with 'Source
jp_komeito_diet_hr_budget_19930129 (Japan) has no pinned attribution' until Codex classifies this packet's commit in
`COMMIT_PACKETS`; `tools/avatars/test_certified_boundary_matrix.py` (S23) errors with 'Required input is missing:
spheres-web/src/person_avatar_assets.rs' in this sparse checkout, so Codex lists the packet and regenerates
`docs/campaign-certification/S23/preparation/boundary-matrix/` on integration. The gap ledger was not touched.

## Independent review checkpoint — 1 October 2026

Held, not accepted: 24 of 31 original responses were independently reproduced and inspected; seven remain unavailable after connection failures and rate limiting. Fifty claims are supported in their bounded scope; fourteen claims and five holder rows retain missing-original dependencies. The original author's internal 'Accepted in part' labels above do not grant integration acceptance. Takeya's proposed 14 March 2026 effective start was conservatively replaced by a same-day attestation; all original response hashes, quoted wording, claim IDs and event dates remain preserved. See [the complete review](../../campaign-certification/C01/reviews/CLAUDE-C01-31/README.md). C01/C06/S23/CP1 remain open.
