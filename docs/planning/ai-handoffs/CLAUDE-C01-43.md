# CLAUDE-C01-43: Research PRN, PTC and Agir national presidents, 1990–2026

Owner: Claude. State: **ready_for_review**. Registered by Codex on 1 October 2026.
Parent C01 remains incomplete. Branch: `claude/c01-br-43`.

Exact claim: `6e6a9d06cf7c6a2e0f46f16d13119ab8bc822d39`. Exact checked tip: `68fb8f863398247ba1515a1ff3f413a444a6d7f5`.
The [authored claim/delivery](https://github.com/ridgemerkley2-web/Spheres/blob/68fb8f863398247ba1515a1ff3f413a444a6d7f5/docs/planning/ai-handoffs/CLAUDE-C01-43.md)
is preserved on its own branch. Its assertions about user instructions are not independent authorization.

New delivery at 68fb8f86: fourteen proposed originals and twenty-one claims, with five Daniel Tourinho observations. Independently review all original content, renaming/office scope and preservation before import. No source retrieval or historical acceptance in this pass; Archive retrieval closed after a different packet returned HTTP 429. Preserve PDS claims-only limits and unavailable SGIP evidence.

Keep the historical cutoff at 7 September 2026, separate party and state offices,
and preserve unknown dates. No runtime, portrait, country-cast or CP1 acceptance
is implied. Fetch current integration before a follow-up, use the existing claim,
and keep source recovery separate from the ready Tonga production assignment.

## Claude's claim and delivery record

Owner: Claude. State: **ready_for_review** (1 October 2026 UTC; submitted, not accepted). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `Brazil/br_prn`, `Brazil/br_pds`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-br-43`. Base: `509bd289` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: `6e6a9d06` (this record's first commit on the branch).

## Bounded deliverable

Add one party role, br_agir_president (kind party_leader), to the existing TSE 2024 observation br_tse_fefc_2024_party_07 (AGIR). Research the national presidents of the Partido da Reconstrução Nacional (PRN), renamed Partido Trabalhista Cristão (PTC) and then Agir, from 1 January 1990 to the cutoff — at most ten people. The renamings (PRN to PTC, PTC to Agir) are organization claims, recorded with the TSE's own decisions, never merged identities beyond what the TSE registry states (same registration/CNPJ, as C01-34 ruled for PMDB to MDB). The Partido Democrático Social (PDS, 1980–1993, merged into PPR) has no research observation: its national presidents are claims only. Party records (archived party pages, convention editais) and TSE/SGIP registry records are primary; never copy SGIP personal data (hash, then delete raw bodies); news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/brazil-prn-ptc-agir-presidents-1990-2026-43.md`;
- `docs/campaign-certification/C01/research/brazil.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_brazil_prn_agir_presidents_c01_43.py`, and pinned counts or exact sets in `test_brazil_research_s10f.py`, `test_brazil_presidents_c01_10.py`, `test_brazil_vice_presidents_c01_17.py`, `test_brazil_pt_presidents_c01_22.py`, `test_brazil_party_presidents_c01_34.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Brazil, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 1 October 2026 (UTC) on `claude/c01-br-43`, based on `509bd289` with the moved
integration `229df210` merged cleanly (`803b2627`), and not stacked on any pending packet. Claim commit: `6e6a9d06`
(this record only). Two further commits: `Add CLAUDE-C01-43: PRN / PTC / Agir national presidents, 1990-2026` (this
record, the report, the packet, the extracts and the tests) and `Regenerate the C01 research index for CLAUDE-C01-43`
(`research-index.json` only). Report:
[brazil-prn-ptc-agir-presidents-1990-2026-43.md](../../campaign-certification/C01/research/brazil-prn-ptc-agir-presidents-1990-2026-43.md).
Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-43.md` (this record);
- `docs/campaign-certification/C01/research/brazil-prn-ptc-agir-presidents-1990-2026-43.md` (new report);
- `docs/campaign-certification/C01/research/brazil.json` (additions only: 14 sources and 21 claims appended after
  CLAUDE-C01-34's; on the AGIR observation `br_tse_fefc_2024_party_07`, one new role and the new ids appended to its
  sources and claims; one coverage note on that observation and one on the packet);
- 14 new extracts `docs/campaign-certification/C01/research/sources/brazil-*-facts.json` (no existing extract edited);
- `tools/avatars/test_brazil_prn_agir_presidents_c01_43.py` (new); `tools/avatars/test_brazil_research_s10f.py`,
  `tools/avatars/test_brazil_presidents_c01_10.py`, `tools/avatars/test_brazil_vice_presidents_c01_17.py`,
  `tools/avatars/test_brazil_pt_presidents_c01_22.py` and `tools/avatars/test_brazil_party_presidents_c01_34.py` (pins
  re-expressed exactly, none loosened);
- `docs/campaign-certification/C01/research-index.json` (regenerated, separate commit; the only file shared with the
  other pending packets).

One person is researched as holder: Daniel Tourinho. The PDS (1980-1993) is claims only and names no person.

Decisions:

- **One office across the renamings.** `br_agir_president` is placed on the AGIR funding observation as one office of
  the party the court registered as the PRN (definitive registration 22 February 1990) and renamed PTC (24 April 2001,
  PET nº 341) and AGIR (31 March 2022, in the registration case RPP nº 51-91.1989), following CLAUDE-C01-34's ruling for
  PMDB to MDB: the renamings are the court's own changes of name of one registered party, recorded as organization
  claims; no other identity is merged and the observation's identity, lifecycle and empty mapping are unchanged. Ridge
  accepted this in the checker round (ruling (b) below); Codex may still decide otherwise.
- **Holders** (no start, no end anywhere; six observations): Daniel Tourinho observed on 16 May 2014 (PTC item of that
  day), 19 July 2018 (convocation signed as 'Presidente do Diretório Nacional - PTC', scanned PDF read visually; ruling
  (a)), 25 July 2018 (Resolução nº 02/2018, signed as 'Presidente Nacional PTC', scanned PDF read visually), 7 August
  2020 (Resolução nº 001/2020), 23 July 2021 (signed communiqué) and 11 November 2022 (Agir item of that day).
- **Claims only:** the court's registration, renaming and fusion decisions (their decision days stored as `attested_on`
  of organization claims, as CLAUDE-C01-34 stored the PMDB renaming); the registry history republished on the PTC site
  in 2007 (`party_republished_reference_text`); the undated 2007 roster; the convention of 25 July 2015 and his styling
  on that day (ruling (e)); the minutes of 3 July 2018 ('o Presidente'); the party's renaming statement of 1 June 2021;
  the executive meeting of 10 November 2022 (day resolved from 'ontem'); an undated 2024 styling as 'presidente do
  partido'; and the court's live list captured 2 August 2026.
- **PDS:** one claim (the court glossary's record of its fusion with the PDC into the PPR, Res.-TSE nº 19.133 of 8 June
  1993), never cited; no primary record naming a PDS national president of 1990-1993 was found (leads only).
- **Gaps:** no contemporaneous dated record of 1990-2013 and none of 2023-2026 naming the office; the court's SGIP
  registry returned its election-period outage page on 1 October 2026, so no registry organ was read.

Rulings decided by Ridge in the checker round (recorded as his decisions and applied; Codex may still decide
otherwise):

- **(a)** The convocation of 19 July 2018, signed by 'Daniel Sampaio Tourinho' as
  'Presidente do Diretório Nacional - PTC', is a holder observation on `br_agir_president` with `attested_on`
  2018-07-19, `from` and `until` null (a signed act attests the office; the C01-37/C01-38 signature rule); its claim
  is re-kinded `in_office_attestation`.
- **(b)** One role across PRN, PTC and Agir is accepted (the TSE register records both renamings as changes of name of
  one registered party, PET nº 341 and RPP nº 51-91.1989; as CLAUDE-C01-34's PMDB to MDB).
- **(c)** The TSE's printed decision days stay as `attested_on` on organization claims, never holder boundaries.
- **(d)** Party items with no event day are dated by their own printed date (the C01-29/C01-31 issue-date ruling); for
  11 November 2022 the present-tense styling dates the item and the meeting of 10 November stays a separate claim.
- **(e)** The styling of 25 July 2015 stays a claim (it cannot be separated from that day's election of a new
  executive).

Observation decisions:

| ID | Decision |
|---|---|
| AGIR-PRES-01 | Claims only: the PRN's registration and the renaming to PTC (court records); republished registry history; undated 2007 roster |
| AGIR-PRES-02 | Accepted: Daniel Tourinho observed 16 May 2014, 19 July 2018 (signed convocation), 25 July 2018, 7 August 2020 and 23 July 2021; the convention of 25 July 2015 and his styling that day, and the minutes, are claims |
| AGIR-PRES-03 | Accepted in part: renaming to Agir (party statement and court decision, claims); Daniel Tourinho observed 11 November 2022; later stylings and listings are claims |
| PDS-PRES-01 | Claims only: fusion into the PPR; no PDS president found |

Integration notes: the user chose to start this batch before Codex's 'continue existing claims first' roadmap line.
Regenerate `research-index.json` on integration rather than merging it (parallel packets C01-38 to C01-46 touch it).
`test_certified_gap_ledger.py` reports 'no pinned attribution' for this packet until Codex classifies its commit, and
`test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which the sparse checkout lacks; neither is fixed
here and the gap ledger is not touched. The SGIP outage also blocks re-downloading CLAUDE-C01-34's twelve SGIP sources
until the service returns.

Checks:

Run in the worktree on 1 October 2026 (UTC) after merging the moved integration `229df210` (`803b2627`) and before
committing, at least 30 minutes after the first downloads:

- `python tools/avatars/campaign_research.py` then `--check`: pass (9 country packets, 1,985 sources, 4,897 claims).
- `python tools/avatars/campaign_census.py --check`: pass, on the unchanged base `509bd289` (claim commit `6e6a9d06`)
  and again with this packet on the merged integration; nothing to disclose.
- `python -m unittest discover -s tools/avatars -p 'test_brazil*.py'`: 51 tests OK (the new
  `test_brazil_prn_agir_presidents_c01_43.py` and the five re-expressed Brazil tests).
- `-p 'test_*research*.py'`: 79 tests OK; `-p 'test_campaign*.py'`: 16 tests OK.
- `node --test tools/ui/check_leadership_research_review.cjs`: pass (0 failures).
- `python tools/planning/workboard.py --check`: PASS. After the merge it first reported 'Missing task handoff:
  docs/campaign-certification/S26/preparation/RECRUITMENT.md' only because the sparse checkout omitted
  `docs/campaign-certification/S26` (the file is in the merged tree); that directory was added to this worktree's sparse
  checkout and the check passes.
- `git diff --check`: clean.
- Every single-quoted passage of an HTML source claim (62 quotations) was checked by script against the decoded
  capture; the three scanned PDFs were read visually.
- Known failures outside the suite, not fixed here: `test_certified_gap_ledger.py` (setUpClass: 'Source
  br_agir_tse_partidos_registrados_capt20260802 (Brazil) has no pinned attribution', until Codex classifies this
  packet's commit) and `test_certified_boundary_matrix.py` (S23; 'Required input is missing:
  spheres-web/src/person_avatar_assets.rs' in the sparse checkout).
- `D:/spheres-scratch/c01-pipeline/tools/packet_check.py 43` re-downloads every recorded response and reruns this
  suite after the push; its summary is reported with the handoff.

## Checker round (1 October 2026 UTC)

State: **ready_for_review** again after the fixes (not accepted). Before the fixes, `origin/codex/campaign-certification`
had moved to `13367c99` and was merged (`7a002dbe`). Two conflicts: the generated `research-index.json` (regenerated with
`campaign_research.py`) and this record (add/add: the integration carries Codex's registration of this record from
`e7592644`). Codex's registration is kept verbatim at the top and this record follows it unchanged under 'Claude's claim
and delivery record' (only its duplicate title dropped), as CLAUDE-C01-42's merge kept Codex's registration; Codex may
prefer another resolution.

Commits: `Apply checker fixes to CLAUDE-C01-43` (this record, the report, `brazil.json`, the one re-kinded extract and
two tests) and `Regenerate the C01 research index for CLAUDE-C01-43` (`research-index.json` only).

Fixes applied:

1. Ruling (a): the convocation of 19 July 2018 is a sixth holder observation of Daniel Tourinho (`attested_on`
   2018-07-19, `from` and `until` null). Its claim `br_agir_ptc_tourinho_convokes_convention_20180719` is re-kinded
   `in_office_attestation` in `brazil-ptc-edital-convocacao-20180719-facts.json` (claim text unchanged; snapshot now
   4,866 bytes, SHA-256 `93bdd1bead4e97bfd2f22ac4ebc3ab4168e454a0ea0c15c4030b07e67347355b`) and its uncertainty
   rewritten. The 25 July 2018 holder's uncertainty, the role's scope note and both coverage notes now count six
   observations.
2. `test_brazil_prn_agir_presidents_c01_43.py` pins the six holders and the convocation as an in-office attestation of
   its own day with no start or end. 2018-07-19 leaves the never-holder days, while 2018-07-28 (the convoked convention)
   stays and is pinned as no holder date. Two mutations are re-indexed to the same holders as before, and three are
   added: a start from the convocation, an end at the convoked convention, and the convocation moved to the convention
   day.
3. `test_brazil_party_presidents_c01_34.py`: the stale comment now reads 'Exactly four party offices, each once, each on
   its own funding observation (CLAUDE-C01-43's AGIR role last).'; the follow-on comment it subsumes is removed and no
   assertion changed.
4. Report: the dating paragraph now names the PTC item of 16 May 2014 and the Agir item of 11 November 2022 (dated by its
   own printed date for its present-tense styling, the meeting of 10 November a separate claim). The AGIR-PRES-02
   outcome now reads 'the convention of 25 July 2015 (no elected President named) and his styling that day'. The
   stylings kept as claims are now the convention-day styling of 25 July 2015, the minutes of 3 July 2018 and the
   undated 2024 item, and the convocation follows ruling (a). The holder table, date ledger and observation text are
   renumbered, a section records rulings (a)-(e), and the integration notes and checks are updated.
5. Rulings (a)-(e) are recorded above as decided by Ridge, and the research index is regenerated.

Checks, run in the worktree after the merge and the fixes, before committing:

- `python tools/avatars/campaign_research.py` then `--check`: pass (9 country packets, 2,010 sources, 4,954 claims).
- `python tools/avatars/campaign_census.py --check`: pass; nothing to disclose.
- `python -m unittest discover -s tools/avatars -p 'test_brazil*.py'`: 51 tests OK (the focused test now pins six holder
  observations, the convocation of 19 July 2018 as an in-office attestation with no start or end, and three further
  mutations); `-p 'test_*research*.py'`: 79 tests OK; `-p 'test_campaign*.py'`: 16 tests OK.
- `node --test tools/ui/check_leadership_research_review.cjs`: 11 pass, 0 fail.
- `python tools/planning/workboard.py --check`: PASS.
- `git diff --check`: clean.
- Known failures outside the suite, unchanged and not fixed here: `test_certified_gap_ledger.py` ('Source
  br_agir_tse_partidos_registrados_capt20260802 (Brazil) has no pinned attribution', until Codex classifies this
  packet's commit) and `test_certified_boundary_matrix.py` ('Required input is missing:
  spheres-web/src/person_avatar_assets.rs' in the sparse checkout).
- No source was added or changed, so `packet_check.py 43 --base origin/codex/campaign-certification --no-fetch` reruns
  this suite after the push; its summary is reported with the handoff.
