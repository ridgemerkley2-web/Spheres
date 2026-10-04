# Tonga production-browser refresh: proposed for review

**Status:** proposed for Codex/root review. Reviewer label: **Claude automated review** (a Claude Code agent, not Codex, root or a human). This record does not close CLAUDE-C06-TONGA-01, C06, S23 or CP1. It does not replace root acceptance, and it does not mark the refresh requirement complete. The [historical receipt](../ci-browser-acceptance-20261001/README.md), the [source-refresh record](../../../render-return-20261002/tonga-source-refresh.json) and [REMAINING-WORK](../REMAINING-WORK.md) were read but not modified.

## Run tested

Tested source: `39369f0ee6451466d03778d0273a7f72aafcf8c0`, the current `codex/campaign-certification` tip, in [GitHub run 37056033496](https://github.com/ridgemerkley2-web/Spheres/actions/runs/37056033496) (#418, push).

| Job | ID | Conclusion | Browser step | Artifact (not retrieved) |
|---|---|---|---|---|
| tonga-browser (ubuntu-latest) | 111000855524 | success | success, 19:54:17–19:55:07Z | 11248956048, 2,789,848 B, zip sha256 `2ac3314c…565ada` |
| tonga-browser (windows-latest) | 111000855604 | success | success, 20:07:02–20:08:03Z | 11249411890, 2,697,792 B, zip sha256 `9d65c5b1…dc76f4` |

Both jobs passed the same native tests on each platform:
- 10 `institutional_leadership` tests passed.
- 3 Tonga government and portrait tests passed.
- The 3000-day supplier exporter was ignored, as before, and is not counted as a pass.

Both jobs used Chrome for Testing 145.0.7632.6 (playwright v1208), the build used in the accepted run.

The whole run failed. The aggregate gate shows `TONGA_BROWSER_RESULT: success` and `POLITICAL_RESULT: failure`, so the failure comes from political calibration, a separate blocker. No green workflow is claimed.

The two job logs are retained in `linux/job.log` and `windows/job.log`, exactly as the GitHub MCP `logs_content` field returned them. Each holds the full line count (1059 and 921 lines). Neither has a trailing newline.

## Counts: what was and was not observed

`tools/ui/ci-tonga.cjs` prints nothing when it succeeds. The historical log shows the same silence. The counts exist only in the artifact's `result.json`. Both artifacts were found, and the API returned signed URLs. The download then failed with a proxy 403 on `productionresultssa9.blob.core.windows.net`. The proxy recorded "policy denial or upstream failure", and its documentation treats a 403 as a host blocked by egress policy. One verification retry at 03:04Z failed the same way. **The CI-side counts and screenshots at the tip are therefore unobserved.** The ZIP digests above come from the GitHub API and the upload log only, not from a downloaded copy.

What the CI evidence does establish:
- The unchanged harness exited 0 on both platforms against the exact tip.
- It reaches exit 0 only after these assertions pass:
  - every journey check;
  - `errors=[]`, meaning no page errors;
  - zero `/api/advance` requests;
  - the build-revision check.
- Its fixed control flow implies the historical 14 checks, 12 commands, 7 refusals, 46 snapshots and 50 bindings. That is an **inference from code, not an observed count**.

**Local diagnostic (Linux, not equivalent to CI).** The harness was run locally against the same tip, with the binary reporting `39369f0ee645`.

- **First run, as specified:** it exported the four native fixtures. Their hashes are byte-identical to the historical CI fixtures (see `as-specified-run-fixture-provenance.json`). The browser then failed to start, because only Chromium 141 (revision 1194) is installed and Playwright 1.58.2 needs revision 1208.
- **Second run, with a substituted browser:** the preinstalled Chromium 141 headless shell was aliased as revision 1208 in a scratch browsers path. The harness, fixtures and binary were not changed. The run **passed**:
  - 14 checks, 12 confirmed commands, 7 refusals, 46 archive snapshots;
  - 0 page errors, 0 advances;
  - 50 historical bindings for 32 people; only `to_fatai_helu` has no art;
  - four fictional previews, with 10 portrait byte checks.
- **Comparison with the accepted Linux result.json:**
  - The checks, commands, refusals and portraits are identical, and so are all 46 archive hashes.
  - The opening and latest government payloads are identical.
  - The requests differ only in their session, client and review tokens.
  - `opening-mobile.png` is byte-identical. `appointed-desktop.png` differs in 208 pixels, all in the tab-label strip.

These outputs are retained in `local-linux-diagnostic/`. They support determinism of this journey's native save, archive and government payloads across the two environments. They are not production-browser evidence from CI, and they say nothing about Windows.

## Source pins

The 19 `source_pins` were recomputed at the tip. **16 are unchanged and 3 changed:**

| Pinned path | Change from f5bd to the tip (introduced in `a982f752`) |
|---|---|
| `spheres-web/data/person_portraits.json` | Now `aafc849d…`. Still 620 people, none added or removed. 32 entries outside Tonga gained portraits and identity sources. All 30 `to_*` entries and the 3 Tonga Crown entries are value-identical. |
| `spheres-web/src/person_avatar_assets.rs` | 38 lines added, 0 removed. All are asset arms for non-Tonga cartoons. |
| `spheres-web/src/person_portraits.rs` | 51 lines added, 0 removed. One new boundary test, which is why 481 tests are now filtered out instead of 480. |

**The manifest hash does not match the refresh record.** The record states `cc843d78…`, which was correct at `0c74f00a`/`ed9786fb`. At the tip the manifest is `aafc849d…`, so the record is stale. This receipt records the mismatch but does not repin anything.

Other changes since the accepted run:
- The harness inputs are byte-identical at f5bd and at the tip. That covers `ci-tonga.cjs`, `ci-tonga-contract.cjs`, `ci-integrated.cjs`, the fixture example, `verify.yml` with its unchanged Tonga job, and the npm lockfile.
- `leadership_production_2035.json`, which is not pinned, also changed. Git grep finds no runtime reference to it.
- 81 reference and cartoon images were added for other countries.

## Still required before any acceptance

- A reviewer with artifact access should download both ZIPs before **2026-10-09**, when the artifacts expire.
- That reviewer should then:
  - verify the digests;
  - read `result.json` and `fixture-provenance.json`, including the Windows CRLF checkout bytes;
  - inspect the screenshots.
- Root then decides whether to accept and repin.

Unchanged from the historical record, all of these remain open: the eleven historical chains, Fatai Helu's verified likeness, final likeness review and country signoff. The authored date fixtures exercise interface and rule behaviour; they do not establish elapsed campaign stability.

## Automated verification

A second Claude automated pass rechecked this record on 2026-10-04. It compared the run, job, artifact and log facts with the GitHub Actions API, and recomputed the hashes and pins with git. It also re-derived the local comparisons. It is not Codex, root or human review, and it confers no acceptance. Its checks and corrections are listed under `automated_verification` in `receipt.json`.
