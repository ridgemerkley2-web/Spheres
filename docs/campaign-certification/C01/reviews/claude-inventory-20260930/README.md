# Claude inventory — 30 September 2026

Seven research packets are newly ready relative to the current central queue/workboard. None is accepted by this audit. Snapshot integration: `44098c5a48f491f43fbcb74d121f290380af0b12`. All 50 fetched Claude tips are enumerated in `inventory.json` and `remote-tips.tsv`.

| Task | Scope | Exact remote tip | Central record |
|---|---|---|---|
| CLAUDE-C01-34 | Brazil: PFL/DEM, PDT and PMDB/MDB national presidents | `16ef41a66d992e7a09417ab332cb6fc8fb6e1c76` | absent from queue/workboard |
| CLAUDE-C01-37 | France: prime ministers | `eb7006176756796f5fd0a7244d9a3316ab275e60` | absent from queue/workboard |
| CLAUDE-C01-33 | India: Janata Dal presidents | `2c78e89f67e9e21090a7da775fd8897c71d7dbd6` | absent from queue; workboard mentions claim only |
| CLAUDE-C01-35 | USSR: Democratic Russia and Soyuz group leaders | `52e3e313b7c53601833c2b170d504b3bed9fe4ff` | absent from queue/workboard |
| CLAUDE-C01-36 | Tonga: deputy prime ministers | `6b82475ea8224a7fecf911f6442e8b6d7b506ca1` | absent from queue/workboard |
| CLAUDE-C01-30 | South Africa: ACDP, Freedom Front and IFP leaders | `cb0f1153f0d26729445acbc05843c3f16bf78ee7` | claimed |
| CLAUDE-C01-32 | South Africa: PAC presidents | `df4707462903b893916b5ef2d56a875a82f4030f` | absent from queue/workboard |

Each listed tip's own handoff explicitly says `ready_for_review`. These are seven delivered packets, not seven newly completed sessions. All merge-base diffs remain confined to C01 research, its generated index, scoped handoffs and avatar research tooling/tests; no runtime or art acceptance is implied. This is a filename/scope inventory, not a substantive diff review.

C01-32 is stacked on the whole unaccepted C01-30 submission. Review C01-30 before C01-32 or extract its exact bounded changes; a blanket merge would import both. C01-33 carries older C01-27 ancestry, whose independently repaired form is already integrated. Preserve that accepted record and regenerate combined indexes rather than replacing the country packet from an old branch.

Russia C01-28 remains submitted and unreviewed at `03141c43c5663e35d21eebc631aaf5eec4e909aa`. Japan C01-29 is unchanged at `3b30478562ec78c2392d0472061e773e23cb25dd`: its retained review still holds integration for19 unavailable originals (35/54 exact). This audit does not retry retrieval or duplicate research. C01-23/24/25/27 remain accepted bounded intake; the17 older research submissions remain integrated with broader historical acceptance pending. Completed source-repair and preparation packets retain their narrower scopes.

The older `claude/c01-gaps-01-fix` follow-up `1aa670479758a1320701639db28807fb3553adba` is also not an ancestor of integration and has no located review receipt. Its handoff asks for review; current code still hashes the whole queue, with no `queue_projection`. Treat it as a separate old tooling follow-up, not new historical research or an automatic replacement for the newer ledger. The legacy `admiring-euclid-a867c9` branch is already pinned in protected-originals evidence, dates from8September, and is not a newly ready packet. `c01-saudi-03` is the older-numbered alias of C01-06, not another submission.

Actionable bookkeeping drift: register C01-30 as submitted, register six delivered C01-32–37 packets, replace the C01-33 claim-only sentence, and reserve their exact targets against duplicate future gap-ledger assignments. The old Who owns what sentence still lists C01-23/24/25/27 as merely submitted even though the top-level closeout/queue correctly record accepted bounded intake. These are findings for the parent, not edits performed here. No C01-31 remote exists in this snapshot.

Evidence is compact: original remote handoffs and central input snapshots are copied unchanged; all review/acceptance evidence paths and Git-byte pins are retained in `inventory.json`. Raw tips, commit dates, ancestor checks, task rows and scoped changed-path inventories permit review without importing research. No network fetch, tests, builds, simulation, source retrieval, repo edits, merge or status change was performed.
