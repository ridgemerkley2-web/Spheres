# CLAUDE-C01-SOURCE-05: Recover and verify the Russian archive resolution

Owner: Claude. State: **claimed** (27 September 2026; in progress, not complete). Parent: C01 (incomplete).

Assigned by `docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md` and `docs/planning/ai-task-queue.json` (task
`CLAUDE-C01-SOURCE-05`, queued 27 September 2026) as a source-review follow-up to the integrated CLAUDE-C01-05.
It repairs integrated research; it does not claim historical acceptance, which Codex decides.

Branch: `claude/c01-source-05`. Base: `a33a8987` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded repair

Reproduce the Russian State Archive (projects.rusarchives.ru) response for CEC resolution No. 20-25 of 19 June 1991 (recorded 15,700 bytes, sha256 54000d3c…683e), or provide an accessible primary facsimile or archive capture with a content-level comparison. Codex's spot-check timed out; do not infer a wrong claim from that timeout. Preserve the unresolved leaf-number discrepancy (caption L. 6-7 versus folio numbers 5, 6 and 7).

Keep historical dates unchanged unless the cited evidence supports a correction. Do not convert missing
response identities into fabricated checksums; retain disclosed limitations and propose further bounded review
separately. Never bypass an access control, CAPTCHA or bot block; record a blocked source instead.

## Allowed files and checks

Allowed files:

- this record;
- `docs/campaign-certification/C01/research/russia.json` (the `ru_cec_resolution_20_25_19910619` source record only);
- `docs/campaign-certification/C01/research/sources/russia-garf-cec-result-19910619-facts.json` and any new specifically related `russia-*-facts.json` extract;
- `docs/campaign-certification/C01/research/ussr-russia-transition-1991-05.md`;
- `tools/avatars/test_ussr_russia_transition_c01_05.py` and any test pinning the changed values;
- `docs/campaign-certification/C01/research-index.json`, regenerated in a separate commit.

Checks: research-index `--check`; the Russia, research and campaign Python tests with game data present
(campaign census included); `campaign_census.py --check`; the atlas Node check; `workboard.py --check`;
`git diff --check`.

Return `ready_for_review` with exact commits, original-response identities where available, content locators and
remaining gaps.
