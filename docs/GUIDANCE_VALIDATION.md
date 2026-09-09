# Tutorial and advisor validation — 8 September 2026

Implemented on `codex/resume-spheres` with the existing uncommitted worldwide leadership work preserved. Nothing committed or pushed during this task.

- Serverless UI suite: **1,173 passed**, zero failures or skips. Log: workspace `work/guidance-ui-tests-final.log`.
- Release native web suite: **274 passed**, zero failures, three pre-existing ignored tests. Log: workspace `work/guidance-native-tests.log`.
- Release build passed. Log: workspace `work/guidance-release-build.log`.
- Release executable: `company-sim-target/release/spheres-web.exe`, SHA256 `E5218D6ECEA27DEBAC9DBA8772F8A3109B8E82ABCBA84EF2E0A18ED1077BCFC3`.
- `git diff --check` passed. Initial UI failures were an expected old navigation assertion and keyboard-listener extraction compatibility; both were corrected before the final full run.

## Browser review

The existing static server hosts [the interactive review](http://127.0.0.1:7841/tools/ui/guidance-review.html). It loads the actual tutorial, advisor and presentation modules with explicitly illustrative scenarios and an in-memory preference adapter. It has no campaign connection and makes no API requests. The production game bridge is tested separately through its actual extracted host functions.

Checked desktop and 390×844 layouts, lesson completion versus skipping, opening a destination and returning to its lesson, searchable glossary with retained input focus, annual budget and paused construction advice, incoming diplomatic deadline routing to Decisions, government filtering, hiding the final card with focus returning to Close, unavailable briefing, and artwork loading. No horizontal dialog overflow or browser errors/warnings remained. Responsive override was reset; browser tab 25 is marked as the deliverable.

This is not a browser playtest of the rebuilt native game. Its new routes and assets are compiled and tested but have not been launched. The earlier automatic approval rejection of a preview launch remains in effect; no launch, process replacement or alternate-port workaround was attempted.

## Campaign preservation

The only request to the current 7843 game during final verification was read-only `/api/build`; it still reports revision `98a27249e580`, build timestamp `1788824130`. No orders, time advances, loads or saves were sent. The existing `before-leadership-2035.json` archive still has SHA256 `44CAE2CACB2F0E6A83AA6DEEF7D1E465FC6F0A652DBE5AB018D9AEAD3F631DB9`.

Upstream default/master `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9` was fetched and inspected, not merged. Later publication must reconcile the character tree and upstream industry changes. The advisor detects optional zero construction capacity, and it rejects unsupported suggestion shapes instead of issuing legacy orders for them.
