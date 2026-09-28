# SOURCE-17 bounded independent review

Accept the response-identity and cited-content repair at the pinned Claude head, together with the small Codex qualification repair. The current Senate response and the stated January 2023 raw Archive capture independently returned **24,950,218 bytes**, SHA-256 `d6c9c275da00f4d8b73a3127fa98c53a7723f7779445d6a33e93131d7ec854ee`. Both were ordinary requests with Python’s default User-Agent and identity encoding, without cookies or bypasses.

The final three extract snapshots are in `reviewed-extracts/`. `visual-review.json` identifies precisely which rendered pages were read. The page 20 uppercase `PCDOB` correction and page 7 election locator are supported. No dates, holders, historical boundaries or response hashes were changed by Codex.

The submitted wording inferred too much from repeated matching responses and Age: 0. Repair `de73e6c73a98a1240cc8860f6758528df7c84d6e` limits that assertion. The former `d9808101…f6a619` body was not kept; its cause and content equivalence remain unknown. Matching current/captured bodies cannot prove why it differed. Signature-service/QR verification was not performed.

This closes only the source follow-up. C01 coverage, C06, portrait permission and future/person-history acceptance are unchanged. The evidence is sufficient for bounded technical/source acceptance, not a new complete historical audit.

## Reproduction and evidence limits

Run `python -X utf8 verify-review.py` inside this packet for offline evidence-integrity checks. This verifies retained records/snapshots, not present network access. Focused test logs and the initial sparse-checkout failures are preserved in `logs/`; `checks.json` distinguishes them. All final listed suites passed. Rebuild the combined research index after integration.

Raw downloaded response bodies and page renders remain outside the repository at `work/campaign-certification/evidence/c01-source-followups-review-20260928/`. Their exact hashes are recorded. The packet does not republish original artwork, signature images, speeches or legal scans and asserts no open license. Retrieval/render scripts are exact copies of the local review scripts; their original relative layout assumes that external evidence directory and isolated review worktree, not this packet directory. The initial script’s method label says HTTPS even for HTTP records; original/final URLs identify the actual scheme. Its generic label was corrected before the three ancillary requests. No response record was rewritten to hide this labeling issue.

The fixture-only commit 26ce0929 is not an integration commit. Root integrates the scoped Claude source repairs, applies any named Codex repair, and separately updates global indexes/workboards. These review packets make no global state changes.
