# CLAUDE-C01-47: Socialist Party first secretaries, 1990–2026

Owner: Claude. State: **ready_for_review** (submitted 2026-10-01 UTC; not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `France/fr_ps`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-fr-47`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Add one party role, fr_ps_first_secretary (Premier secrétaire; kind party_leader), to the existing CNCCFP observation fr_cnccfp_76 (PARTI SOCIALISTE), through supplements/france.json and the importer. Research the First Secretaries from 1 January 1990 to the cutoff — at most ten people (expected Pierre Mauroy, Laurent Fabius, Michel Rocard, Henri Emmanuelli, Lionel Jospin, François Hollande, Martine Aubry, Harlem Désir, Jean-Christophe Cambadélis, Olivier Faure; an interim (e.g. 2017-2018) is claims only; if more than ten, cover the first ten and list the rest as next work). Each Congress or Conseil national vote, assumption and departure is its own dated claim. The party's own records (parti-socialiste.fr and archived pages, congress results) are primary; JORF only where it records the party office; news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/france-ps-first-secretaries-1990-2026-47.md`;
- `docs/campaign-certification/C01/research/france.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/france-*-facts.json` extracts, plus `docs/campaign-certification/C01/research/supplements/france.json` and a regenerated `france.json` (via `tools/avatars/import_cnccfp_census.py`);
- focused tests under `tools/avatars/`: a new `test_france_ps_first_secretaries_c01_47.py`, and pinned counts or exact sets in `test_campaign_research.py`, `test_import_cnccfp_census.py`, `test_france_presidents_c01_23.py`, `test_france_prime_ministers_c01_37.py`, `test_france_pm_boundaries_review.py`, `test_france_prime_ministers_c01_38.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the France, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.

## Result

Submitted `ready_for_review` on 1 October 2026 (UTC). Report:
`docs/campaign-certification/C01/research/france-ps-first-secretaries-1990-2026-47.md`. Branch `claude/c01-fr-47`; claim commit
`6f657179` on base `5ea4f8fc`; not stacked. Before committing, `codex/campaign-certification` was fetched again (it had not moved from `5ea4f8fc`).
Then two commits: "Add CLAUDE-C01-47: Socialist Party first secretaries, 1990–2026" (everything except the index) and
"Regenerate the C01 research index for CLAUDE-C01-47" (`research-index.json` only).

Outcome: the party role `fr_ps_first_secretary` (Premier secrétaire; `party_leader`) on `fr_cnccfp_76` (PARTI SOCIALISTE), with
24 party source responses (16 issues of the party weeklies Vendredi and L'Hebdo des socialistes as served by the Fondation
Jean-Jaurès's Centre d'archives socialistes; 8 raw Internet Archive captures of parti-socialiste.fr), 47 claims and thirteen
holder observations of ten people, all dated by `attested_on` with no `from` or `until`: Pierre Mauroy (1990-03-24, 1992-01-07),
Laurent Fabius (1992-01-10), Michel Rocard (1993-11-12), Henri Emmanuelli (1994-06-24), Lionel Jospin (1995-10-18, 1997-06-06),
François Hollande (1997-12-05), Martine Aubry (2008-11-29), Harlem Désir (2013-01-08), Jean-Christophe Cambadélis (2014-04-19)
and Olivier Faure (2018-07-09, 2023-03-11). Elections, results, ratifications, departures and resignations are separate dated
claims; the provisional direction of 1993, the Premier secrétaire délégué of 1997 and the collective interim of 2017–2018 are
claims only.

Touched paths (nothing else):

- `docs/planning/ai-handoffs/CLAUDE-C01-47.md` (this record);
- `docs/campaign-certification/C01/research/france-ps-first-secretaries-1990-2026-47.md` (new report);
- `docs/campaign-certification/C01/research/supplements/france.json` (appended only: 24 sources after CLAUDE-C01-38's, one packet
  coverage note, and the new key `organization_roles` with one entry for `fr_cnccfp_76`);
- `docs/campaign-certification/C01/research/france.json` (regenerated by `tools/avatars/import_cnccfp_census.py`, never edited by
  hand);
- 24 new `docs/campaign-certification/C01/research/sources/france-ps-*-facts.json` extracts;
- `tools/avatars/import_cnccfp_census.py` (optional `organization_roles` merge: append-only, fails loudly; outside
  `packet_check.py`'s allowed paths, required by this packet's scope — see Decisions);
- `tools/avatars/test_france_ps_first_secretaries_c01_47.py` (new);
- pinned tests re-expressed exactly (none loosened): `test_import_cnccfp_census.py`, `test_campaign_research.py`,
  `test_france_presidents_c01_23.py`, `test_france_prime_ministers_c01_37.py`, `test_france_prime_ministers_c01_38.py`;
- `docs/campaign-certification/C01/research-index.json` (index-only commit).

Decisions for Codex (detailed in the report): (1) the importer's new optional `organization_roles` key; (2) the
archives-socialistes.fr viewer marks records `allowDownload: false`, treated as a display preference because the PDF it loads is
public and served without login, cookie or token; (3) Fabius's resignation at the comité directeur of 3 April 1993 and the
no-confidence vote in Rocard of 19 June 1994 give no `until`; (4) elections by a competent body on a stated day are claims, not
`from`; (5) holders without an event day are dated by the party weekly's printed issue date, and the 1997 communiqué by its issue;
(6) Hollande's June 1997 statement dates Jospin as Premier secrétaire; (7) the 2025 ratification claim is dated by the 5 June 2025
vote.

Checks: import_cnccfp_census.py --check, campaign_research.py --check, campaign_census.py --check, the France (37), importer (10), research (79) and campaign (16) tests, the Node check (11) and `git diff --check` pass. `workboard.py --check` fails only because the sparse worktree lacks `docs/campaign-certification/S26/` (present at `5ea4f8fc`). `packet_check.py` reports the importer as outside the allowed paths (Decisions, 1). Outside the suite, `test_certified_gap_ledger.py` (no pinned attribution until Codex classifies the commit) and `test_certified_boundary_matrix.py` (needs `spheres-web/src`) are known failures. The gap ledger is untouched; the `packet_check.py 47` summary for the pushed head is returned to the pipeline.

## Repair round (packet_check)

On 1 October 2026 (UTC) the pipeline's repair agent was given one `packet_check.py` problem: "file outside allowed paths:
tools/avatars/import_cnccfp_census.py". It was reviewed and declined; nothing in the packet's data, extracts, tests or
importer changed. The base importer accepts a supplement with exactly `sources`, `institutions` and
`coverage_unresolved`, and only the generated CNCCFP rows become organizations, so a role cannot be attached to
`fr_cnccfp_76` as the scope requires without the importer change. Removing it would mean either dropping the role and its
thirteen holder observations, or hand-editing the generated `france.json`, which the France packets forbid and which
`import_cnccfp_census.py --check` rejects. Moving the role to a new supplement institution would create a second Parti
socialiste entry and is outside the scope. The importer change stays as Decision (1) for Codex: accept the optional
`organization_roles` merge, or rule on another mechanism. `packet_check.py` is expected to keep reporting this one path
until that ruling.

C01 and every parent gate stay open.
