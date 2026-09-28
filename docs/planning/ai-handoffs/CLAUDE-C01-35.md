# CLAUDE-C01-35: Democratic Russia and Soyuz group leaders, 1990–1991

Owner: Claude. State: **claimed** (2026-09-28; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `USSR/su_dr`, `USSR/su_soyuz`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-su-35`. Base: `44098c5a` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Research the leaders (co-chairs, coordinators) of the Democratic Russia movement and of the Soyuz deputies' group in the USSR Congress/Supreme Soviet from 1 January 1990 to 25 December 1991 — at most ten people. Add new organization observations only from primary records of each body (its own documents, the Congress or Supreme Soviet records naming the group and its leaders); co-leadership is recorded as such, never collapsed into one holder. Sources: official records of the USSR Congress of People's Deputies and Supreme Soviet, the organizations' own documents in archival official publications; news and encyclopaedias are leads only. Names in Russian as printed.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/ussr-democratic-russia-soyuz-1990-1991-35.md`;
- `docs/campaign-certification/C01/research/ussr.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_ussr_democratic_russia_soyuz_c01_35.py`, and pinned counts or exact sets in `test_ussr_research_s10h.py`, `test_ussr_russia_transition_c01_05.py`, `test_ussr_government_supreme_soviet_c01_26.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the USSR, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
