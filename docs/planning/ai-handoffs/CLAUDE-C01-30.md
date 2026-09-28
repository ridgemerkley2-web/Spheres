# CLAUDE-C01-30: ACDP, Freedom Front and IFP leaders, 1990–2026

Owner: Claude. State: **ready_for_review** (28 September 2026; submitted, not accepted). Parent: C01 (incomplete).
Report: [south-africa-acdp-ff-ifp-leaders-1990-2026-30.md](../../campaign-certification/C01/research/south-africa-acdp-ff-ifp-leaders-1990-2026-30.md).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `SouthAfrica/za_acdp`, `SouthAfrica/za_ff`, `SouthAfrica/za_ifp`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-za-30`. Base: `b949f64e` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch (`68535825`).

## Bounded deliverable

Add one party-leader role (kind party_leader) to each of three existing IEC observations: za_iec_n2024_008 (AFRICAN CHRISTIAN DEMOCRATIC PARTY), za_iec_n2024_051 (VRYHEIDSFRONT PLUS, the Freedom Front's later name) and za_iec_n2024_034 (INKATHA FREEDOM PARTY). Research each party's national leader or president from 1 January 1990 (or the party's founding) to the cutoff — at most ten people in total (expected Kenneth Meshoe; Constand Viljoen, Pieter Mulder, Pieter Groenewald; Mangosuthu Buthelezi, Velenkosini Hlabisa). The Freedom Front's renaming to Freedom Front Plus and mergers are claims about the organization, never merged identities; Buthelezi's death in 2023 and any transition are distinct dated claims. Party office stays separate from state office (the IFP leader's ministerial posts never feed the party role). Sources: each party's own records (official sites, archived pages, congress resolutions and statements), IEC records and Parliament only where they record the party office.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-acdp-ff-ifp-leaders-1990-2026-30.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_party_leaders_c01_30.py`, and pinned counts or exact sets in `test_south_africa_research_s10h.py`, `test_south_africa_heads_of_state_c01_09.py`, `test_south_africa_anc_presidents_c01_16.py`, `test_south_africa_deputy_presidents_c01_21.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SouthAfrica, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Integration claim registration — 28 September 2026

The preceding text is the authored remote claim, preserved from
`68535825eca53986feb5b6d9bf5ad44981bc6543`. Its original statement that the
claim was not yet registered is historical: Codex has now mirrored the claim
into the central task queue. No research content, tests, runtime changes or
source/identity acceptance were imported. The claim remains in progress;
continue its existing branch and bounded three-party scope.

## Result

Submitted `ready_for_review` on 28 September 2026. Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-30.md` (this record);
- `docs/campaign-certification/C01/research/south-africa-acdp-ff-ifp-leaders-1990-2026-30.md` (new report);
- `docs/campaign-certification/C01/research/south-africa.json`;
- 39 new extracts `docs/campaign-certification/C01/research/sources/south-africa-{acdp,ff,ifp}-*-facts.json` (no existing
  extract is edited);
- `tools/avatars/test_south_africa_party_leaders_c01_30.py` (new), and pinned values in
  `tools/avatars/test_south_africa_research_s10h.py`, `tools/avatars/test_south_africa_heads_of_state_c01_09.py`,
  `tools/avatars/test_south_africa_anc_presidents_c01_16.py` and `tools/avatars/test_south_africa_deputy_presidents_c01_21.py`;
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with other
  pending packets).

The packet gains 39 sources and 64 claims and three `party_leader` roles: `za_acdp_president` on the AFRICAN CHRISTIAN
DEMOCRATIC PARTY observation, `za_ff_leader` on VRYHEIDSFRONT PLUS and `za_ifp_president` on INKATHA FREEDOM PARTY. The
roles hold 22 holder observations of seven people, each dated by a party's own in-office attestation, with no start and
no end, because no party record found states the day any leader assumed or left the office:

- ACDP President: Kenneth Meshoe (1999-05-01, 2001-10-31, 2014-02-18, 2018-02-14, 2026-03-04).
- Freedom Front / Freedom Front Plus Leader: Constand Viljoen (1997-08-26); Pieter Mulder (2001-06-21, 2003-09-28,
  2016-11-12); Pieter Groenewald (2016-12-27, 2020-04-02, 2024-08-22); **Corné Mulder** (2025-07-16, 2026-03-25).
  Corné Mulder was not in the expected list: the party's own records style him its leader in 2025 and 2026 (no archived
  party record of his February 2025 election was found; news reports are leads).
- IFP President: Mangosuthu Buthelezi (1995-10-24, 2012-12-16, 2019-01-20, 2019-08-24); Velenkosini Hlabisa
  (2019-09-12, 2023-09-09, 2024-08-10, 2026-02-01).

Decisions per chain:

- **ACDP** (ZA-ACDP-01..05; 01 accepted in part, the rest accepted): founding "in December 1993" and Meshoe as founder are
  organization claims; the May 1998 newsletter is month-only; the first holder rests on the home page as served on
  1 May 1999; no ACDP election of its President was found.
- **Freedom Front** (ZA-FF-01..09; 06 and 09 accepted, the rest accepted in part): founding "March 1994", the 27 September
  2003 merger and name Vryheidsfront Plus, and the Referendum Party's 2026 joining are organization claims, never merged
  identities or lifecycle boundaries; the 2001 election month conflicts (March or April) and stays open; Groenewald's
  12 November 2016 designation is never a start; the 22 July 2024 release, which names the party leader beside his
  appointment as Minister of Correctional Services, is a continuation claim only; Corné Mulder's designation as
  parliamentary leader (22 July 2024) is a different office, never a holder.
- **IFP** (ZA-IFP-01..08; 01-04 accepted in part, 05-08 accepted): the 1990 transformation of Inkatha into the IFP is an
  organization claim dated 14 July 1990 by the IFP's 1997 and 2000 pages and 10 July 1990 by its later timeline (kept
  open); at the August 2019 conference Buthelezi's last address as President (24 August) dates his last holder, while
  the valedictory statement, Hlabisa's acceptance, the President Emeritus resolution, the result published on 25 August
  ("last night") and the timeline's retrospective "24 August ... officially retires" are claims, never boundaries;
  Buthelezi's death on 9 September 2023 as President Emeritus is a claim that ends no observation; ministerial,
  parliamentary and legislature offices (including Leader of the Official Opposition in KwaZulu-Natal) never feed the role.

No acting or interim leader was found in any chain. The presidency, ANC and DA roles, claims and holders are unchanged, and
no claim or source is shared between the three new roles or with any other role. The packet-level coverage note is
inserted after the CLAUDE-C01-09 note and before the CLAUDE-C01-21 and CLAUDE-C01-16 notes, which their tests pin as the
last two entries.

Integration: before committing, `codex/campaign-certification` was fetched; it had moved to `3ec6e155` and was merged
(merge commit `b69e579a`, tree equal to `3ec6e155`). The only conflict was an add/add on this record: the integration
copy is the claim record byte for byte plus the registration note above, so it was taken unchanged and this result was
appended. After the packet commits (`033f098d`, index `cd335f3b`), `codex/campaign-certification` had moved again, to
`5d970f6d` (the integrated CLAUDE-C01-23, 24, 25 and 27 packets among others); it was merged in `a39f1ac8`, whose only
conflict, `research-index.json`, was regenerated. The task queue entry (state `claimed`) is left for Codex. New South
Africa totals: 207 sources, 406 claims, 10 role observations; `mapping_pending` stays 53 and the six
discovery batches stay open.

## Checks

Run from the worktree with `PYTHONDONTWRITEBYTECODE=1` and `python -X utf8`:

- `tools/avatars/campaign_research.py` (regenerated), then `--check`: passed. On the packet commits: 9 country packets,
  1,367 sources, 3,792 claims, 93 discovery batches; after merging `5d970f6d`: 1,610 sources, 4,246 claims, 93 batches.
  South Africa: 207 sources, 406 claims, 10 role observations throughout.
- `tools/avatars/campaign_census.py --check`: exit 0 (`spheres-sim/data` is present in this sparse worktree).
- `-m unittest discover -s tools/avatars -p "test_south_africa*.py"`: 42 tests OK (the new test's 8 included, with its
  7 validator and 35 invariant mutation cases all rejected).
- `-p "test_*research*.py"`: 79 tests OK. `-p "test_campaign*.py"`: all 16 campaign tests ran and passed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 passed.
- `python tools/planning/workboard.py --check`: PASS.
- `git diff --check` on the touched paths: clean. The gap ledger was not touched.

All of these checks were run again on the merged tree `a39f1ac8` with the same results (42, 79 and 16 tests OK; atlas 11
passed; census exit 0; workboard PASS).
