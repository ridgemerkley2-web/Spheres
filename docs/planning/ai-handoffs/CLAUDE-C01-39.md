# CLAUDE-C01-39: current acceptance, 1 October 2026

Owner: Claude. State: **complete** for this bounded research delivery. Parent C01 remains incomplete.

All **24 originals, 29 claims and 12 added office observations** are independently reviewed and imported at `e56916b4ae3f3f6c3ca1e29aca091921adb58719`. [Acceptance and integration](../../campaign-certification/C01/integrations/CLAUDE-C01-39/README.md) pins the final review, prior failures and limits.

All 245 previously integrated sources, the two original DA holders, other organizations and the presidency remain unchanged. Three derived locators and their extract hashes are corrected. The Wednesday 23 October 2019 vacancy is independently supported by the Friday statement and corroborating Thursday statement; Zille's 2007 acceptance is an attestation only. Interim, DP/NNP and formation facts remain claims, not merged identities or complete tenures.

Fetch current `codex/campaign-certification` before further work. This is research acceptance only; no country cast, runtime identity, art permission or parent qualification is granted. The earlier claim and status checkpoints below are historical records, not current instructions or current task state.

---

## Preserved earlier claim and review checkpoints

# C01-39 integration status, 1 October 2026

The original claim below is preserved. The reviewed delivery at
`9cb02c20012a6e6314d7e64d52241609d566f738` is **held after independent source/content review**.
The latest delivery `b66f8c074431d7e1a8c9bfca228576c7d5134655` remains **ready_for_review**,
with the same source-access hold after a separate diff triage.
The [review receipt](../../campaign-certification/C01/reviews/CLAUDE-C01-39/README.md)
at `d1818a48a1b117d408dafb49d7943d35f3f7c2da` records one exact original and one
independently read claim. The second request returned HTTP 429; 22 sources were
not attempted after stopping. Thus 23 originals, 28 claims and all eleven new
holder observations remain held. These are not 23 failed requests.

The accessible 24 October 2019 statement supports the reported resignation and
vacancy context; no numeric effective end was installed. The next-day source
remains unread. A final fetch found correction `9e3d0b3c` and index tip `b66f8c07`:
the source identity inventory remains unchanged, but the proposal adds Zille's
2007 acceptance observation, sets a Maimane end of 23 October 2019 and corrects a
quote's apostrophe. The latest scope is twelve added holder observations, all
unverified. The 23-source / 28-claim hold persists. The changes and claims of
separate user rulings are not accepted solely from authored prose. See the
[combined review](../../campaign-certification/C01/reviews/PENDING-2026-10-01/README.md)
for the exact delta. Resume missing original content when normal service permits.

Only this registration and the held review receipt are integrated. No South Africa
research or tests from the new packet are imported or accepted. Its isolated
technical checks passed, with the initial handoff-binding failure and narrow repair
preserved. The task queue records the latest submission and the earlier independently reviewed
revision; the earlier receipt continues to refer only to `9cb02c20`.

See the [authored delivery and reported checks](https://github.com/ridgemerkley2-web/Spheres/blob/9cb02c20012a6e6314d7e64d52241609d566f738/docs/planning/ai-handoffs/CLAUDE-C01-39.md).
It reports 24 new sources, 29 claims and eleven added holder observations; those
counts are submission scope, not a full independent historical acceptance.

---

# CLAUDE-C01-39: Democratic Alliance federal leaders, 2000–2026

Owner: Claude. State: **claimed** (2026-09-30; in progress, not complete). Parent: C01 (incomplete).

Origin: part of the back-to-back C01 research pipeline the user asked for on 28 September 2026, taken from the
certified-country gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.md`) items `za_iec_n2024_027#za_da_federal_leader`, `SouthAfrica/za_dp`. Pending Codex
acceptance; not registered in the task queue.

Branch: `claude/c01-za-39`. Base: `02d2c5a2` (current `codex/campaign-certification`); not stacked on a pending packet. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Extend the existing role za_da_federal_leader on za_iec_n2024_027 (DEMOCRATIC ALLIANCE; current holders Steenhuisen 2023-04-03 and Hill-Lewis 2026-04-12) back to the DA's formation in 2000: expected Tony Leon, Helen Zille, Mmusi Maimane, John Steenhuisen (interim 2019, elected 2020) and Geordin Hill-Lewis — at most ten people. The Democratic Party (1989–2000; leaders Zach de Beer and Tony Leon), the 2000 DP/NNP alliance and the DA's formation are claims about the organizations only, never merged identities; interim leadership is claims only. The DA's own records (federal congress results, statements, archived da.org.za pages) are primary; IEC records only where they record the party office; news are leads.

Keep each distinct event (election or selection, appointment, assumption of office, acting or interim service,
resignation, removal, death, merger, renaming) as its own dated claim; acting service is claims only. Never infer an
end from a successor's start. Give a holder `from` or `until` only where a source states the day; otherwise record
`attested_on`. Party office and state office stay separate both ways. Organization identities, lifecycles and game
mappings stay unresolved; a name match to a simulation row is never a mapping. At most ten people. Primary sources
only; news and encyclopaedias are leads. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/south-africa-da-federal-leaders-2000-2026-39.md`;
- `docs/campaign-certification/C01/research/south-africa.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_south_africa_da_federal_leaders_c01_39.py`, and pinned counts or exact sets in `test_south_africa_research_s10h.py`, `test_south_africa_heads_of_state_c01_09.py`, `test_south_africa_anc_presidents_c01_16.py`, `test_south_africa_deputy_presidents_c01_21.py`, `test_south_africa_party_leaders_c01_30.py`, `test_south_africa_pac_presidents_c01_32.py` updated to
  the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change the gap ledger, shared UI, the
roadmap, game data or other country packets.

Checks: research-index `--check`; `campaign_census.py --check`; the SouthAfrica, research and campaign Python tests
(census included); the atlas Node check; `workboard.py --check`; `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
