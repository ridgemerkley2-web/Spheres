# CLAUDE-C01-SOURCE-17: Compare the changed Brazilian Senate diary response

Owner: Claude. State: **claimed** (27 September 2026; in progress, not complete). Parent: C01 (incomplete).

Assigned by `docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md` and `docs/planning/ai-task-queue.json` (task
`CLAUDE-C01-SOURCE-17`, queued 27 September 2026) as a source-review follow-up to the integrated CLAUDE-C01-10 and CLAUDE-C01-17.
It repairs integrated research; it does not claim historical acceptance, which Codex decides.

Branch: `claude/c01-source-17`. Base: `a33a8987` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded repair

Compare the Senate's stored DCN No. 1/2023 download (BuscaPaginasDiario?codDiario=111711&download=true; recorded 24,950,218 bytes, sha256 d6c9c275…c854ee; fetched by Codex as 24,950,158 bytes, d9808101…a619) against the recorded extracts. Record the exact pages, the publication identity (PDF metadata, page count, verification code) and factual agreement or disagreement page by page for every cited page, rather than replacing a checksum blindly; if the file has changed, record both identities and what changed.

Keep historical dates unchanged unless the cited evidence supports a correction. Do not convert missing
response identities into fabricated checksums; retain disclosed limitations and propose further bounded review
separately. Never bypass an access control, CAPTCHA or bot block; record a blocked source instead.

## Allowed files and checks

Allowed files:

- this record;
- `docs/campaign-certification/C01/research/brazil.json` (the source records of the stored DCN No. 1/2023 issue only);
- the extracts of those source records under `docs/campaign-certification/C01/research/sources/`;
- the CLAUDE-C01-10 and CLAUDE-C01-17 reports under `docs/campaign-certification/C01/research/`;
- `tools/avatars/test_brazil_presidents_c01_10.py`, `tools/avatars/test_brazil_vice_presidents_c01_17.py` and any test pinning the changed values;
- `docs/campaign-certification/C01/research-index.json`, regenerated in a separate commit.

Checks: research-index `--check`; the Brazil, research and campaign Python tests with game data present
(campaign census included); `campaign_census.py --check`; the atlas Node check; `workboard.py --check`;
`git diff --check`.

Return `ready_for_review` with exact commits, original-response identities where available, content locators and
remaining gaps.
