# CLAUDE-C01-34: PFL/DEM, PDT and PMDB/MDB national presidents, 1990–2026

Owner: Claude. State: **ready_for_review** (28 September 2026; submitted, not accepted). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Brazil/br_pfl`, `Brazil/br_pdt`, `Brazil/br_pmdb`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-br-34`. Base: `032cd6a3` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: `c2ff9396` (this record's first commit on the branch).

## Bounded deliverable

Add party-leader roles to the existing FEFC observations for MDB and PDT (and for DEM/União Brasil only if a source ties that label to the PFL line; otherwise record the PFL only as claims) and research each party's national president from 1 January 1990 to the cutoff — at most ten people across the chains. Renamings (PFL to DEM, PMDB to MDB) and mergers (DEM into União Brasil) are claims about the organization, never merged identities. Sources: the parties' own records, the TSE's records of national directorates (SGIP), Congress records only where they record the party office.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/brazil-party-presidents-1990-2026-34.md`;
- `docs/campaign-certification/C01/research/brazil.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_brazil_party_presidents_c01_34.py`, and pinned counts or exact sets in `test_brazil_research_s10f.py`, `test_brazil_presidents_c01_10.py`, `test_brazil_vice_presidents_c01_17.py`, `test_brazil_pt_presidents_c01_22.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Brazil, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 28 September 2026 on `claude/c01-br-34`, based on `032cd6a3` with the moved integration
`44098c5a` merged cleanly (`e8f38113`), and not stacked on any pending packet. Claim commit: `c2ff9396` (this record only). Report:
[brazil-party-presidents-1990-2026-34.md](../../campaign-certification/C01/research/brazil-party-presidents-1990-2026-34.md).
Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-34.md` (this record);
- `docs/campaign-certification/C01/research/brazil-party-presidents-1990-2026-34.md` (new report);
- `docs/campaign-certification/C01/research/brazil.json` (additions only: 76 sources and 127 claims appended after
  CLAUDE-C01-22's; on the MDB observation `br_tse_fefc_2024_party_01` and the PDT observation
  `br_tse_fefc_2024_party_02`, one new role each and the new ids appended to their sources and claims; one coverage note
  on each of the two observations and one on the packet);
- 76 new extracts `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` (no existing extract edited);
- `tools/avatars/test_brazil_party_presidents_c01_34.py` (new); `tools/avatars/test_brazil_research_s10f.py`,
  `tools/avatars/test_brazil_presidents_c01_10.py`, `tools/avatars/test_brazil_vice_presidents_c01_17.py` and
  `tools/avatars/test_brazil_pt_presidents_c01_22.py` (pins re-expressed exactly, none loosened);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with the
  other pending packets).

At most ten people are researched: Leonel Brizola and Carlos Lupi (PDT); Jader Barbalho, Michel Temer, Romero Jucá and
Baleia Rossi (PMDB/MDB); Jorge Bornhausen, Rodrigo Maia, José Agripino and ACM Neto (PFL/DEM, claims only). The PMDB
presidents of 1990-1998 and the PFL presidents before 1999 are outside the packet and proposed as next work.

Decisions per chain:

- **PDT** (`br_pdt_president`, eight holders): Leonel Brizola observed 11 April 1997, 26 August 1999 and 2 June 2004,
  until 21 June 2004 (the party's same-day report of his death in office; fallback: a claim only); Carlos Lupi observed
  6 July 2004, 9 February 2007, 21 December 2021, 21 May 2025 and 4 September 2026. No start. Who presided on 1 January
  1990 is open (the party's 2019 history ties Brizola's 1992 recondução to the death of Doutel de Andrade); Lupi's 2004
  assumption (automatic, interim from 28 June 2004, or 21 June 2004), his 2008-2009 leave with Vieira da Cunha acting,
  his 2023-2025 leave with André Figueiredo acting, the return decided on 20 May 2025, re-elections and two closed SGIP
  organs are claims. His Tijolaço column printed '06.05.2004' is undated: its own text places it in early June 2004.
- **PMDB/MDB** (`br_mdb_president`, five holders): Michel Temer observed 2 July 2003 and 29 March 2016; Baleia Rossi
  observed 17 October 2019, 16 July 2025 and 7 June 2026. No start or end. Jader Barbalho (elected 15 September 1998)
  has no day-dated party attestation and no holder. Romero Jucá's service from 5 April 2016 is recorded as acting (the
  party's roster and list and the TSE registry say 'em exercício' or 'interina'), so his unqualified stylings, including
  the signed edital of 23 November 2017, are claims (Codex to rule). The posse of the executive of 10 March 2010, printed
  on an undated roster, is a claim, not Temer's start. The renaming PMDB to MDB (convention of 19
  December 2017, filed with the TSE on 31 January 2018, approved by the TSE on 15 May 2018) is recorded as organization
  claims and ties the MDB label to the PMDB.
- **PFL/DEM** (claims only; no role, no holder, `UNIÃO` unchanged): Bornhausen (executive elected and invested 7 May
  1999; styled 7 March 2003; signs 13 March 2007), the renaming as Democratas (convention called for 28 March 2007;
  Rodrigo Maia signs as 'Presidente Nacional do Democratas' on 29 March 2007, still styled so on 15 February and 14 March
  2011), José Agripino (slate of 16 February 2011, election reported 16 March 2011, signs 24 March 2011, registry period
  2015-2018) and ACM Neto (executive elected 8 March 2018, registry periods 2018-2022, styled 5 October 2021 and 12
  January 2022). The fusion into União Brasil (joint convention of 6 October 2021, TSE registration of 8 February 2022)
  created a new party with its own number and CNPJ, and the registry names DEM 'extinto por fusão com PSL', so no
  PFL/DEM role is placed on the `UNIÃO` observation.

Observation decisions:

| ID | Decision |
|---|---|
| PDT-PRES-01 | Accepted in part: the 1990 holder open; Brizola observed 1997, 1999 and 2004, until 21 June 2004 (death in office) |
| PDT-PRES-02 | Accepted in part: Lupi observed 6 July 2004; three conflicting assumption versions, no start |
| PDT-PRES-03 | Accepted in part: Lupi observed 9 February 2007 and 21 December 2021; re-elections, leave, acting service and registry periods are claims |
| PDT-PRES-04 | Accepted in part: leave and Figueiredo's acting service (claims); Lupi observed 21 May 2025 and 4 September 2026 |
| MDB-PRES-01 | Accepted in part: Jader Barbalho elected 15 September 1998; no dated attestation, no holder; the end open |
| MDB-PRES-02 | Accepted in part: Temer observed 2 July 2003; the 2009 leave and Iris de Araújo's interim service are claims |
| MDB-PRES-03 | Accepted in part: Temer observed 29 March 2016; the 2010 posse on an undated roster, leaves, Raupp's acting service and conflicting records are claims |
| MDB-PRES-04 | Accepted in part: Temer's leave, Jucá as acting, the renaming PMDB to MDB (claims) |
| MDB-PRES-05 | Accepted in part: Baleia Rossi observed 17 October 2019, 16 July 2025 and 7 June 2026 |
| PFL-PRES-01 | Claims only: Bornhausen |
| PFL-PRES-02 | Claims only: the renaming as Democratas and Rodrigo Maia |
| PFL-PRES-03 | Claims only: José Agripino |
| PFL-PRES-04 | Claims only: ACM Neto and the fusion into União Brasil |

Checker defects: two independent checks read every claim, extract row and holder against the stored bodies (PDT and
PFL/DEM: 5 must-fix, 15 suggestions; PMDB/MDB: 8 must-fix, 8 suggestions). All must-fix defects are fixed (among them
Brizola's Tijolaço column, whose printed '06.05.2004' its own text contradicts, is now undated, so his 2004 observation is
the caucus meeting of 2 June 2004; the SGIP record dates; Temer's and Jucá's registry periods; the start of Temer's
mandate; the 2010 convention's setting day; the undated 2009 roster); the suggestions are applied, with Lupi's 2025
observation left to Codex's ruling. The raw SGIP bodies, which carry members' personal data, were deleted after hashing
and review; no personal field is copied.

Checks run on 28 September 2026 in this sparse worktree (`docs/campaign-certification/C01`,
`docs/campaign-certification/verification`, `docs/planning`, `spheres-sim/data`, `spheres-sim/src`, `spheres-web/data`,
`tools/avatars`, `tools/planning`, `tools/ui`; not widened), after merging `44098c5a`, with `PYTHONDONTWRITEBYTECODE=1`:

- `python -X utf8 tools/avatars/campaign_research.py` then `--check`: exact regeneration passes; 9 packets,
  841 organization and 35 institution observations, 1,647 sources, 4,309 claims, 93 discovery batches.
- `python -X utf8 tools/avatars/campaign_census.py --check`: fails, as on the base `032cd6a3` and on `44098c5a`
  ('C01 evidence differs: census.json'). Regenerated to a scratch directory, `countries.json`,
  `represented-organizations.json`, `roles-and-lifecycle.json` and `work-orders.json` are identical; `census.json`
  differs only in `source_files[1]` (`spheres-sim/src/government.rs`, recorded 848,551 bytes `0e2dbdef…`, now 849,546
  bytes `3f846b4b…`, changed by `262d5f61`), `production_snapshot.stale_inputs` (0 to 1 entry) and
  `production_snapshot.all_declared_inputs_current` (true to false). Not fixed here.
- Brazil tests (`-p "test_brazil*.py"`): 42 pass (9 of them in the new `test_brazil_party_presidents_c01_34.py`).
- Research tests (`-p "test_*research*.py"`): 79 pass.
- Campaign tests (`-p "test_campaign*.py"`): 16 pass. These suites overlap; their counts are not summed.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass.
- `python tools/planning/workboard.py --check`: passes (44 canonical markers, 33 bounded tasks).
- `git diff --check`: clean; every new or changed file is LF with no trailing whitespace.
- Known failures outside this packet, not fixed: `test_certified_gap_ledger.py` errors ('Source
  br_pdt_legalidade_interview_199611 (Brazil) has no pinned attribution') until Codex classifies the packet's commit;
  `test_certified_boundary_matrix.py` (S23) errors ('Required input is missing: spheres-web/src/person_avatar_assets.rs')
  in this sparse worktree. The gap ledger is not touched.
- The new test rejects hand-made regressions by rule and against the pinned lists: a next-day death report, a leave, a
  renaming or a registry end used as an end; an election, re-election, the executive posse of 2010, a retrospective
  assumption day, a predecessor's death, a return from leave or a registry start used as a start; an election or
  continuation claim cited by a holder; acting, interim, acting-period, conflicting or undated-roster service added as a
  holder (Figueiredo, Jucá, Iris de Araújo, Raupp, Jader Barbalho); cross-role and cross-institution moves between the
  PDT, MDB, PT and presidency records; any PFL/DEM claim or role placed on `UNIÃO`; a second party role or none; a
  lifecycle start; a retrospective span given a date; a printed civil name as the holder name; holders out of order;
  collapsed event dates; and checksum, path, beyond-cutoff, reversed-interval, cited-source, unknown-claim and
  foreign-mapping mutations.

C01 and all parent gates (C06, S23, WC1, CP1) stay open. No installed leader, avatar, portrait, campaign rule or
save schema changed.
