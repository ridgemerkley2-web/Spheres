# CLAUDE-C01-SOURCE-06: Compare the changed Bush Library response

Owner: Claude. State: **claimed** (27 September 2026; in progress, not complete). Parent: C01 (incomplete).

Assigned by `docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md` and `docs/planning/ai-task-queue.json` (task
`CLAUDE-C01-SOURCE-06`, queued 27 September 2026) as a source-review follow-up to the integrated CLAUDE-C01-06.
It repairs integrated research; it does not claim historical acceptance, which Codex decides.

Branch: `claude/c01-source-06`. Base: `a33a8987` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded repair

Compare the Bush Presidential Library's 8 August 1990 address page (recorded 31,889 bytes, sha256 539f1155…a1c3; fetched by Codex as 32,782 bytes, ad596dc9…aa8c) against the recorded extract. Identify dynamic-page changes versus changed factual content, and record a reproducible identity (for example a raw pre-cutoff archive capture) if one exists. Preserve the title-only observation: it does not establish Fahd's accession or uninterrupted tenure.

Keep historical dates unchanged unless the cited evidence supports a correction. Do not convert missing
response identities into fabricated checksums; retain disclosed limitations and propose further bounded review
separately. Never bypass an access control, CAPTCHA or bot block; record a blocked source instead.

## Allowed files and checks

Allowed files:

- this record;
- `docs/campaign-certification/C01/research/saudi-arabia.json` (the Bush Library source record only);
- `docs/campaign-certification/C01/research/sources/saudi-arabia-bush41-address-19900808-facts.json` and any new specifically related `saudi-arabia-*-facts.json` extract;
- the CLAUDE-C01-06 report under `docs/campaign-certification/C01/research/`;
- `tools/avatars/test_saudi_executive_c01_06.py` and any test pinning the changed values;
- `docs/campaign-certification/C01/research-index.json`, regenerated in a separate commit.

Checks: research-index `--check`; the SaudiArabia, research and campaign Python tests with game data present
(campaign census included); `campaign_census.py --check`; the atlas Node check; `workboard.py --check`;
`git diff --check`.

Return `ready_for_review` with exact commits, original-response identities where available, content locators and
remaining gaps.
