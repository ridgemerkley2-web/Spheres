# CLAUDE-C01-35: Democratic Russia and Soyuz group leaders, 1990–1991

Owner: Claude. State: **ready_for_review** (submitted 29 September 2026 UTC; not complete). Parent: C01 (incomplete).

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

## Submission (ready_for_review)

[Report](../../campaign-certification/C01/research/ussr-democratic-russia-soyuz-1990-1991-35.md):
`ussr-democratic-russia-soyuz-1990-1991-35.md`. Branch `claude/c01-su-35`, claim commit `c1475ada` on `44098c5a`; **not stacked**
(the CLAUDE-C01-SOURCE-26 repair is integrated). Result commits: the packet commit and the separate index commit at the head of
`claude/c01-su-35` at submission; to be recorded by the integrator. Research access 29 September 2026 (UTC). The Democratic
Russia part rests on a sub-agent's research dossier, the Soyuz part on this packet's reading; an independent adversarial check
re-downloaded and re-read each part, and all 19 of their defects are applied (14 Soyuz, 5 Democratic Russia).

Decisions per chain:

- Democratic Russia movement (`su_democratic_russia`, new organization, jurisdiction USSR at republic level, RSFSR; role
  `su_dr_co_chair`, kind `party_leader`, co-leadership). SU-DR-01 accepted in part (the bloc's call for a movement (which bloc not
  established), 22 Jun 1990; the election of the movement's Council of Representatives announced, 7 Dec 1990; its Coordinating
  Council's march application, 28 Mar 1991; a Council of Representatives plenum, 15 Sep 1991; no primary record of the founding
  congress). SU-DR-02 accepted in part (other deputies call Мурашев "председатель оргкомитета движения", 17 and 19 Dec 1990;
  claims only). SU-DR-03 accepted in part: seven holder observations of five co-chairs, Виктор Владимирович Дмитриев (attested
  1991-04-05, his own words at the RSFSR Congress), Юрий Николаевич Афанасьев (1991-07-15, his own words; 1991-09-12), Лев
  Александрович Пономарев (1991-09-12; 1991-12-12), А. Мурашев (1991-09-12) and Глеб Павлович Якунин (1991-12-12), from the
  Coordinating Council's letter signed "Сопредседатели КС „Дем. России“" (dated 12.09.91, registered 12.10.1991) and the RSFSR
  President's office list of 12 Dec 1991 (Yeltsin Center facsimiles of Presidential Archive copies); Пономарев's signature of 15
  Sep 1991 is a role claim. SU-DR-04 accepted in part (the second congress announced for 9-10 November, year not printed; an
  undated claim). SU-DR-05 not found (no end).
- Soyuz deputies' group (`su_soyuz_deputies_group`, new organization, union level; role `su_soyuz_co_chair`, kind
  `parliamentary_leader`, co-leadership). SU-SOYUZ-01 accepted (the group speaking, deciding and nominating through its general
  meeting in the USSR Congress, 12 Mar-27 Dec 1990). SU-SOYUZ-02 accepted in part: one holder, Анатолий Георгиевич Чехоев (attested
  1990-12-20, "Как один из сопредседателей депутатской группы «Союз»"; full name from resolution 1842-I); the other co-chairs are
  not named in any reviewed record. SU-SOYUZ-03 accepted (registration roll, 561 deputies, 25 Dec 1990). SU-SOYUZ-04 accepted in
  part (other deputies' "руководитель" and "лидеры", including the accusing proposal of 26 Aug 1991 naming Коган, Алкснис, Блохин,
  Чехоев and Петрушенко; role claims, never holders; integrator ruling requested). SU-SOYUZ-05 not found (latest record 3 Sep
  1991; no end).

Verifier fixes (29 September 2026 UTC): the appeal of 22 June 1990 no longer says which bloc «Демократическая Россия» is meant
(electoral or RSFSR deputies' bloc not established); the letter's date is no longer said to be in the text's hand; the report
records the RSFSR-RF.RU host's provenance (the private ISTNET project) and the full fallback if the non-official hosts are ruled
out; locators are added for RSFSR First Congress vol. II and Yeltsin Center item 10637; the d222 extract's file-name date is
declared a catalogue date. Two of this packet's own extracts were edited, scope notes only, with snapshots updated
(`ussr-rsfsr-snd1-stenogram-vol5-19900622-facts.json`, `ussr-yeltsin-center-f6-d222-l129-19911109-facts.json`). Rulings (Ridge):
the letter keeps `attested_on` 1991-09-12, the date it states (12.10.1991 records receipt); "лидер", "лидеры" and "руководитель"
stay claims only, because they do not name the office (co-chair); the Якунин and Пономарев holders of 12 December 1991 are kept,
since the President's office list names the movement and the exact office ("сопредседатели Координационного совета"), consistent
with CLAUDE-C01-33's standard. Integration note: evidence standard: CLAUDE-C01-29 admitted only a party officer's own statements;
CLAUDE-C01-33 and this packet also admit official records naming the organisation and the exact office. Under a strict C01-29
reading Якунин would drop out and Пономарев's 15 September 1991 signature would become his latest attestation; a single
cross-country ruling is for the integrator.

No holder has a `from` or an `until`; every observation is one person. The RSFSR deputies' group, bloc and faction «Демократическая
Россия» are separate identities, read and not imported. Neither organization is mapped to `USSR/su_dr` or `USSR/su_soyuz`. At most
ten people are reviewed: the six holders and the four other deputies named as the Soyuz group's "лидеры".

Touched paths: this record; `docs/campaign-certification/C01/research/ussr.json` (16 sources, 36 claims, two organizations with
one role each and eight holder observations, one packet coverage item; no existing content changed); 16 new
`docs/campaign-certification/C01/research/sources/ussr-*-facts.json` extracts (no existing extract edited); the new
`docs/campaign-certification/C01/research/ussr-democratic-russia-soyuz-1990-1991-35.md`; the new
`tools/avatars/test_ussr_democratic_russia_soyuz_c01_35.py`; `tools/avatars/test_ussr_research_s10h.py` (totals, organization
count, table-extract count, PDF-page set, access dates, index figures; the union-level jurisdiction guard re-expressed exactly
with `su_democratic_russia` pinned as republic, RSFSR; none loosened) and `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py`
(its new-source list pinned to positions 10-39, packet totals and index figures). `test_ussr_russia_transition_c01_05.py` and
`russia.json` are unchanged. Separate commit: `docs/campaign-certification/C01/research-index.json` only.

Checks (29 September 2026 UTC, sparse worktree with `spheres-sim/src`, `spheres-sim/data` and `spheres-web/data`): the research
index regenerated and `--check` passes (1,587 sources, 4,218 claims, 843 organization and 35 institution observations, 93
discovery batches); the USSR tests (37, 9 of them new) and the Russia tests (29) pass; the research tests (79) and the campaign
tests (16, census included) pass; the atlas Node check passes (11); `workboard.py --check` passes (44 markers); `git diff
--check` on this packet's paths is clean. The new test's 47 mutations each fail on the rule they break. Known failures outside
these checks, not fixed: `campaign_census.py --check` exits 1 on the integration base itself because commit `262d5f61` changed
`spheres-sim/src/government.rs` without regenerating `census.json` (regenerating to a scratch directory shows that `census.json`'s
record of that input is the only difference; the other four outputs are identical); `test_certified_gap_ledger.py` already fails
on the base (`ledger.json` stale) and errors on this packet's sources ("no pinned attribution") until Codex classifies the commit
in `COMMIT_PACKETS`; `test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, absent from the sparse checkout, and in a
full checkout reports the packet as `unclassified_packet`, so Codex must list it and regenerate the S23 boundary matrix.
