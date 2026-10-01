# CLAUDE-C01-44: Tongan opposition party leaders (DPFI and PDP), 1990–2026

Owner: Claude. State: **ready_for_review** (1 October 2026 UTC; 30 September local; submitted, not accepted). Parent: C01
(incomplete).
Report: [tonga-dpfi-pdp-leaders-1990-2026-44.md](../../campaign-certification/C01/research/tonga-dpfi-pdp-leaders-1990-2026-44.md).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `to_dpfi#to_dpfi_leader`, `to_dpfi#to_dpfi_president`, `to_pdp#to_pdp_leader`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-to-44`. Base: `509bd289` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing roles to_dpfi_leader and to_dpfi_president (Democratic Party of the Friendly Islands, organization to_dpfi; one holder each from CLAUDE-C01-04) and to_pdp_leader (People's Democratic Party, to_pdp; no holder yet) from each party's founding to the cutoff — at most ten people. Keep the existing holders unchanged. Party conventions, elections of leaders and presidents, deaths in office (only where the office and the day are named), acting leadership (claims only) and splits/foundings are distinct dated claims; organization identities stay unresolved. Party records and the Tongan government's own releases (PMO, Legislative Assembly, Gazette) only where they record the party office are primary; Matangi Tonga and other news are leads; party and state office stay separate.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/tonga-dpfi-pdp-leaders-1990-2026-44.md`;
- `docs/campaign-certification/C01/research/tonga.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/tonga-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_tonga_dpfi_pdp_leaders_c01_44.py`, and pinned counts or exact sets in `test_tonga_research_s10g.py`, `test_tonga_*_c01_*.py`, `test_tonga_deputy_prime_ministers_c01_36.py`, `test_tonga_dpfi_c01_04.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Tonga, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 1 October 2026 (UTC; 30 September local) on `claude/c01-to-44` (claim `fdb71175`, base
`509bd289`, not stacked). `codex/campaign-certification` moved to `efc88b85` during the work and was merged before the
packet commits (merge `73ae1261`, no conflict). Commits: `Add CLAUDE-C01-44: Tongan opposition party leaders (DPFI and
PDP), 1990-2026` (everything except the index) and `Regenerate the C01 research index for CLAUDE-C01-44`
(`research-index.json` only). Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-44.md` (this record);
- `docs/campaign-certification/C01/research/tonga-dpfi-pdp-leaders-1990-2026-44.md` (new report);
- `docs/campaign-certification/C01/research/tonga.json` (additions only: 5 sources and 6 claims appended after every earlier
  packet's; on `to_dpfi`, the 5 sources and 6 claims appended; five coverage notes appended on `to_dpfi`, two on `to_pdp` and
  one on the packet; no holder, organization, role or name observation);
- 5 new extracts `docs/campaign-certification/C01/research/sources/tonga-*-facts.json` (no existing extract edited);
- `tools/avatars/test_tonga_dpfi_pdp_leaders_c01_44.py` (new); `tools/avatars/test_tonga_research_s10g.py` (totals pin
  re-expressed exactly, 212 sources and 325 claims, none loosened);
- `docs/campaign-certification/C01/research-index.json` (separate commit).

Decisions (details in the report):

- No holder is added. No primary record (party record, PMO, Assembly or Gazette naming the party office) names a DPFI/PTOA
  leader, president or chair after 'Akilisi Pohiva, or any PDP officer; later names are news or tertiary leads.
- TO-OPP-02: a Supreme Court recital (AM 20/2013, 17 January 2014) calls Mr Pohiva "the leader of the Tonga Democratic
  Party". Kept as a claim, not a `to_dpfi_leader` holder, because the name differs from DPFI (ruling (a), decided by
  Ridge: stays a claim, routed to `C01-Tonga-OPP-005`).
- TO-OPP-03: the Court of Appeal (16 September 2015) calls the "Friendly Islands Democratic Party" an unincorporated body,
  and the PMO (17 March 2020) asserts that PTOA was unregistered with no legal body or constitution: claims only;
  TO-DPFI-04 stays unresolved. No name observation is added (ruling (b)).
- TO-OPP-01: founding statements (September 2010 relayed third-party text; 2010 profile line) are claims only;
  `lifecycle.from` stays null. The relayed PGA text is typed `legislature_republished_reference_text` (accepted by
  ruling (c)).

Checks (after the merge; full lines in the report): research-index `--check` pass; `campaign_census.py --check` pass (also
on `509bd289`, so no census failure is disclosed); Tonga tests 101 pass; research tests 79 pass; campaign tests 16 pass;
`test_country_cast.py` 31 pass; Node leadership check 11 pass; `workboard.py --check` pass; `git diff --check` clean. The
sparse checkout was widened by `docs/campaign-certification/C06` and `S26` (22 small files) for the merged workboard and
cast checks. Known failures outside the suite, disclosed and not fixed: `test_certified_gap_ledger.py` ("no pinned
attribution" for this packet's sources until Codex classifies the commit) and `test_certified_boundary_matrix.py` (needs
`spheres-web/src`). `D:/spheres-scratch/c01-pipeline/tools/packet_check.py 44` is run on the pushed head; its summary is
returned to the pipeline and saved in `D:/spheres-scratch/verify/C01-44/packet_check.json`.

Integration: the user chose to start this research batch before Codex's roadmap line "continue existing claims first";
`research-index.json` is shared with the parallel packets (C01-38 to 41 in fixes, C01-42 to 46 in research), so regenerate it
on any index-only conflict. C01 and all parent gates stay open.

## Checker-fix round

Applied on 1 October 2026 (UTC; 30 September local) on `claude/c01-to-44` from head `206df905`. State stays
**ready_for_review**. Commits: `Apply checker fixes to CLAUDE-C01-44` (extracts, `tonga.json`, report, this record and the
focused test) and `Regenerate the C01 research index for CLAUDE-C01-44` (`research-index.json` only; the `tonga.json`
checksum is the only change).

Fixes:

1. `holder_name` set to null in the extract rows `to_la_pga_dpfi_established_sept2010`,
   `to_la_profile_dpfi_established_2010` and `to_ca_fidp_unincorporated_body_20150916`, whose sources name no holder of any
   office. The three snapshots in `tonga.json` are updated (bytes and SHA-256: 4,942 `189f0fc6…`, 4,391 `86dafd09…`,
   5,924 `902bcaef…`), and `test_tonga_dpfi_pdp_leaders_c01_44.py` pins the three values as null (exact re-expression).
   No claim text, source, holder or coverage entry changes.
2. The report's next-work line `C01-Tonga-OPP-005` now asks for a ruling on whether "Tonga Democratic Party" (2014),
   "Friendly Island Democratic Party" (FIDP; Assembly news item of 16 September 2011, singular 'Island' as printed) and
   "Friendly Islands Democratic Party" (Court of Appeal AC 9/2015) name `to_dpfi`. No other place printed 'Islands' for the
   2011 item.
3. Rulings (a)-(c) are recorded here and in the report.

Rulings, recorded as decided by Ridge (Codex may still decide otherwise):

- (a) The Supreme Court recital in Pohiva v Tu'ivakano ("He is the leader of the Tonga Democratic Party") stays a claim:
  the record does not name the organization as DPFI/PTOA or by any name already observed for `to_dpfi`, so attaching it
  would settle an identity question. Routed to `C01-Tonga-OPP-005`.
- (b) No name observation is added for "Tonga Democratic Party", "Friendly Island Democratic Party (FIDP)" or "Friendly
  Islands Democratic Party"; routed to the Tonga reconciliation work order (`C01-Tonga-OPP-005`). The focused test's
  unmapped-name guard now also lists the singular 'Island' forms.
- (c) The new `source_type` `legislature_republished_reference_text` is accepted: non-primary, claims only, never holder or
  organization evidence.

Integration branch: `codex/campaign-certification` moved to `13367c99`. It carries Codex's held partial review of this
packet (`214121f9`; receipt `docs/campaign-certification/C01/reviews/CLAUDE-C01-44-20261001/`: four of five originals read,
the PMO original unread after HTTP 429, zero new holders) and Codex's registration of this record (`e7592644`). Merging it
conflicts on `research-index.json` and, add/add, on this file (Codex's registration record at the same path), so the
branch was **not merged** in this round; the conflict is reported for a decision. The review's prose correction
(`9955de17` on `codex/review-c01-44-20261001`) is outside this round's fix list and is not applied here.

Checks on the fix tree (full lines in the report): research-index `--check` pass; `campaign_census.py --check` pass (no
census failure to disclose); Tonga tests 101 pass; research tests 79 pass; campaign tests 16 pass; `test_country_cast.py`
31 pass; Node leadership check 11 pass; `workboard.py --check` pass; `git diff --check` clean. Known failures outside the
suite are unchanged (`test_certified_gap_ledger.py`, "no pinned attribution"; `test_certified_boundary_matrix.py`, needs
`spheres-web/src`). `packet_check.py 44 --base origin/codex/campaign-certification --no-fetch` is run on the pushed head
and its summary returned to the pipeline.
