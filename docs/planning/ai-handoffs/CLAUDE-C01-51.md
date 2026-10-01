# CLAUDE-C01-51: RSFSR President and Vice-President, 1991–1993

Owner: Claude. State: **claimed** (2026-10-01; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `ru_rsfsr_presidency#ru_rsfsr_president`, `ru_rsfsr_presidency#ru_rsfsr_vice_president`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-ru-51`. Base: `5ea4f8fc` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Fill the unresolved intervals of the existing roles ru_rsfsr_president and ru_rsfsr_vice_president (institution ru_rsfsr_presidency; one holder each) with dated primary attestations from the creation of the offices (12 June 1991 election; 10 July 1991 inauguration) to the end of the vice-presidency in 1993: Ельцин as President of the RSFSR / Russian Federation until the office is styled differently (coordinate with the existing ru_president role; never duplicate it), and Руцкой as Vice-President, including the decree of 1 September 1993 suspending him and the Supreme Soviet's 22 September 1993 'acting President' resolution (claims only). At most ten people. Keep existing holders unchanged. Sources: Vedomosti of the RSFSR Congress and Supreme Soviet, presidential decrees (pravo.gov.ru, kremlin.ru archives), stenograms; Russian as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/russia-rsfsr-president-vice-president-1991-1993-51.md`;
- `docs/campaign-certification/C01/research/russia.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/russia-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_russia_rsfsr_presidency_c01_51.py`, and pinned counts or exact sets in `test_russia_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_russia_presidents_c01_14.py`, `test_russia_heads_of_government_c01_19.py`, `test_russia_party_leaders_c01_28.py`, `test_russia_duma_faction_heads_c01_46.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the Russia, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
