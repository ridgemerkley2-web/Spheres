# S08-only status changes after final acceptance

Read-only audit of clean runtime `e6187df8044556a8262154e7a6c8baa32b283799`. No repository files were changed by this draft. Line numbers refer to this pin.

Only these existing status-bearing documents need changes:

1. `docs/CERTIFIED_CAMPAIGN_PATHWAY.md:3`: change “S01–S07 complete; S08 in progress” to “S01–S08 complete; S09 pending.” Preserve the original pathway approval date; completion evidence carries 11 September 2026 separately.
2. `docs/CERTIFIED_CAMPAIGN_PATHWAY.md:63`: change the core-session summary to S01–S08 complete, S09–S30 planned, execution stopped after S08.
3. `docs/CERTIFIED_CAMPAIGN_PATHWAY.md:453`: mark S08 Complete only after acceptance. Check its three boxes at lines 457–459 using the justified matrix and final imported-APC paid-service/browser evidence. Add an S08 evidence link immediately after the third checkbox, before the S09 anchor. Do not weaken or delete the matrix requirement.
4. `docs/CERTIFIED_CAMPAIGN_PATHWAY.md:465`: **leave S09 Planned** and its three checkboxes at lines 469–471 unchecked. “Pending” in summaries means this existing Planned status, not In progress.
5. `docs/CERTIFIED_CAMPAIGN_PATHWAY.md:901`: replace only the latest S08-in-progress sentence with its recorded completion/evidence and that execution stopped before S09. Preserve earlier history and G1; award no later gate or campaign/content/release certificate.
6. `docs/campaign-certification/S08/README.md:3`: replace In progress with Complete, dated 11 September 2026, on the unchanged qualified runtime pin. Replace future-tense qualification at line 19 with actual counts and links. Keep the default-budget failure, chosen policy tradeoff/cost, synthetic-versus-genuine boundaries, and no-full-second-replay limitation.

New evidence publication fields/files:

- `docs/campaign-certification/S08/manifest.json`: create after final collection. Set `session=S08`, `status=complete`, completion date, exact runtime revision, prior source revision `68e863055ee6ddf6c9796547995dccf455557edd`, binary hashes, native/Node/external counts, genuine journey and four archive hashes, browser's own dates/paid-service receipt, performance acceptance and preservation. Set `next_session=S09`, `next_session_status=planned`, `execution_stopped_after=S08`; no automatic next-session activation.
- Final collector-generated evidence/inventory and any compression attributes under `docs/campaign-certification/S08/` should retain source hashes and original failed attempts. Use the collector's real output format; this draft is not a substitute for collection or evidence verification.
- Current S08 has only README.md; no existing S08 manifest/status JSON to update. Repository search found no other current S08 status summary requiring change. Do not edit historical S01–S07 evidence manifests or embedded source snapshots.

The outside `manifest-draft.json` remains `verification_pending` with null completion date and explicit pending final acceptance fields. It is safe to copy only after root fills/reviews the missing outcomes; do not mechanically promote its pending rows to passes.
