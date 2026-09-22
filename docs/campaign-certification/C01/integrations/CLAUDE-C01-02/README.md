# CLAUDE-C01-02 — integration review

Accepted as a bounded research intake on 21 September 2026 (22 September UTC).
**C01 remains incomplete.** Reviewed submission:
`b6767837b8080cae133e073ffb9ecb6f9371ee5f`; qualified review source:
`c792588f` (full revision in the browser result); integration merge:
`2d5ffffea8141fa13fe51cc8ce6c6a95ca9fe6d9`. Reviewer: Codex.
[Evidence manifest](manifest.json).

The [submission](../../research/tonga-transition-2021-02.md) retains its original
status; this record supersedes it for integration. All six observations were
reviewed: four supported decisions are accepted, and two explicitly unresolved
questions remain unresolved. No research claim was rewritten during integration.

## Source review

Codex reopened the four Legislative Assembly notices and read their relevant body
text: interim Speaker appointment, nomination invitation, meeting announcement and
nomination close. They support procedural observations, not an election result or
a prime minister's start of office. No new original-HTML response hash is claimed.

The three PMO article pages did not load through the web reader. Their four linked
release images were fetched directly from the publisher, checked against Claude's
byte lengths and SHA-256 hashes, and visually read. All four hashes match (see
`evidence/source-checks.json`). The English appointment release supports effect
from 27 December 2021, separately from its 28 December publication and the earlier
Assembly selection. The Cabinet release supports effect from 28 December and
letters/oaths on 29 December. Its listed Deputy Prime Minister matches the packet.
The Tongan appointment release corroborates the printed 27/28 December distinction;
the independently readable English release supports the accepted effective date.

No signature or presentation date for the Royal Warrant, or exact end/caretaker
boundary for Tu'i'onetoa, is inferred. This review does not prove that no further
source exists. Prior claims outside the submission were not recertified. Original
source artwork stays outside Git; the packet contains factual extracts and hashes.
The frozen research cutoff remains 7 September 2026.

## Validation and limits

- Exact index regeneration passes: nine packets, 841 organization observations,
  27 institutions, 68 sources, 1,622 claims and 92 open batches.
- 28 Tonga tests, 16 campaign tests, 79 research tests and 11 atlas Node tests pass.
  These suites overlap; their counts are not summed as distinct tests.
- Chrome reviews all nine research packets, source integrity, stale requests,
  keyboard access and layouts at 1440, 390 and 320 pixels.
- New browser assertions and captures verify the two effective starts and unknown
  ends. The 390px captures were visually inspected. No game API is called.
- The source stayed clean and unchanged during the exact browser run. The initial
  launch command used an invalid PowerShell parent-path expression and failed before
  starting Chrome; correcting the environment paths required no product repair.

No installed party, avatar, government rule, saved appointment or campaign schema
changed. Tonga still has four provisional organizations and five institutions;
neither its complete leadership history nor the worldwide census is closed.

Next optional, unclaimed research packet:
[CLAUDE-C01-03](../../../../planning/ai-handoffs/CLAUDE-C01-03.md).
