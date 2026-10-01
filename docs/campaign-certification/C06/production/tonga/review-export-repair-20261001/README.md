# Cartoon review sparse-input failure

Cause: the sparse Windows checkout omitted tracked `docs/campaign-certification/S10/c/manifest.json`. It was marked optional, so the exporter silently excluded it. Full CI included it and correctly found both committed exports stale. The failure is not a line-ending difference.

The isolated reproduction materialized246 exact Git-blob text dependencies and762 Git-verified images (hardlinks; no duplicated image data). With the S10.c manifest present, both exports differed. Removing only that file reproduced both committed exports exactly. The changes are one input fingerprint, one reviewed reference and the Tupou IV item's reference annotation/label; findings, asset bytes and counts are unchanged. The relevant exporter/input/art/prompt/output files are identical between failing CI commit `d42bf94e090aac5adae8b7ddb44f7572c29b2e15` and reproduction base `d6380e16b4f5facb87ae72ca37ece45c223d2056`.

Fix commit: `c179878fd76d80823689aef53da43f9958868375` (`codex/cartoon-review-sparse-input-20261001`). Only the exporter and its test file change. A missing optional file tracked in the Git index now stops generation with its path and a materialization instruction. Genuinely untracked optional fixture files remain optional.

The new tracked-missing regression fails before the fix; its untracked-optional control passes. After the fix, all31 cartoon-review tests pass using disposable regenerated exports. A real tracked omission also exits1 with the intended message. No acceptance guard is weakened.

Root must materialize only the exact S10.c manifest after integration cherry-picks, regenerate both exports and rerun final avatar checks. The guard commit deliberately excludes generated outputs. This was Windows execution with Git-blob inputs matching full Linux checkout inputs, not a Linux runtime or native validation claim. Exact commands, source identities, before/after logs and reproduction differences are in the receipt.
