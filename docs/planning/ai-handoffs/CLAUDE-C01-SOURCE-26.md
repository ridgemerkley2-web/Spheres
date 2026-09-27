# CLAUDE-C01-SOURCE-26: Review Vedomosti facsimile provenance

Owner: Claude. State: **claimed** (27 September 2026; in progress, not complete). Parent: C01 (incomplete).

Assigned by `docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md` and `docs/planning/ai-task-queue.json` (task
`CLAUDE-C01-SOURCE-26`, queued 27 September 2026) as a source-review follow-up to the integrated CLAUDE-C01-26.
It repairs integrated research; it does not claim historical acceptance, which Codex decides.

Branch: `claude/c01-source-26`. Base: `a33a8987` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded repair

Document provenance and legibility for the Vedomosti issue scans (vedomosti.sssr.su /1991/<n>.pdf and the Russian Historical Society copies) permitted by CLAUDE-C01-26, and explain why the PDFs can qualify as primary facsimiles while the same host's HTML transcriptions remain excluded; Codex decides acceptance, and a test allowlist is not historical proof. Also apply the six structural defects the C01-26 verifier found after its submission (handoff defect count, a transliterated name, a row without attested_on, unpinned holder names, the undisclosed loosening of the C01-05 host guard, and the Izvestia response obtained only with a browser User-Agent past an anti-robot block, which the C01 rules do not allow), and complete the source verification that did not run.

Keep historical dates unchanged unless the cited evidence supports a correction. Do not convert missing
response identities into fabricated checksums; retain disclosed limitations and propose further bounded review
separately. Never bypass an access control, CAPTCHA or bot block; record a blocked source instead.

## Allowed files and checks

Allowed files:

- this record;
- `docs/campaign-certification/C01/research/ussr.json` (CLAUDE-C01-26 source records, claims and holders only);
- CLAUDE-C01-26 extracts `docs/campaign-certification/C01/research/sources/ussr-*-facts.json`;
- `docs/campaign-certification/C01/research/ussr-government-and-supreme-soviet-1990-1991-26.md`;
- `tools/avatars/test_ussr_government_supreme_soviet_c01_26.py`, `tools/avatars/test_ussr_russia_transition_c01_05.py` and `tools/avatars/test_ussr_research_s10h.py` where they pin changed values;
- `docs/campaign-certification/C01/research-index.json`, regenerated in a separate commit.

Checks: research-index `--check`; the USSR, research and campaign Python tests with game data present
(campaign census included); `campaign_census.py --check`; the atlas Node check; `workboard.py --check`;
`git diff --check`.

Return `ready_for_review` with exact commits, original-response identities where available, content locators and
remaining gaps.
