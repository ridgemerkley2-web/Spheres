# CLAUDE-S24-SUCCESSORS-01 — reviewed preparation

**Accepted for bounded preparation integration. S24 remains planned pending S23. No S24, S25, historical-leader, organic-succession, or campaign qualification is awarded.**

Codex `/root/review_gap_submission` reviewed Claude's submission `5a23ebe018612ab008a6ac911af6c2ca498231f6` against integration baseline `d2b21c15ad8070ad9bde5363d888a31e1c4dc63c`. All 27 owned deliverable files were imported byte-for-byte in `9c8453c0`; the global handoff was excluded. `authored-deliverable.json` pins those original Git bytes. Existing authored evidence under `S24/preparation/successor-fixtures/evidence/` was not rewritten.

## Changes and verification

The original 13 tests passed. Review found that the harness deleted original fixture inputs, swallowed served-asset verification failures, accepted an arbitrary single passed check, and returned success for valid records containing failed/blocked cases. Native exporter metadata and screenshot references also lacked sufficient validation; duplicate JSON keys could cause staging of a shadowed value.

`abd6ecf2` requires complete unique checks, runtime/asset/native exporter identity, and verified original gzip inputs, including the untouched and edited staging saves. Failed/blocked cases return nonzero; a legitimate early environment block remains a block even with zero executed checks. References cannot escape the evidence directory, staging rejects duplicate keys, and cleanup is confined to resolved isolated save paths. Historical hash-only receipts require an explicit inspection option and do not satisfy current original-input acceptance.

`32135770` checks every served UTF-8 asset against its exact expected-revision Git blob, allowing **only CRLF → LF** differences. Raw served and Git hashes, sizes and normalization counts remain in each result. This does not claim that this review checkout reproduces the original Windows build checkout's raw newline bytes. Content drift, invalid UTF-8 and lone-CR differences are rejected; the shared `ci-integrated.cjs` was not changed.

All **26 targeted Node tests passed**, and the current inventory check passed for **23 identities / 2 explicit discrepancies**. Applying the final 26 tests to the preserved authored modules yields **13 original passes and 13 new regression failures**. The first 25-test negative run is also retained. These are focused harness checks, not a full UI/workspace rerun.

The unchanged native S21 exporter ran twice using the frozen `5d11dd6dae436eeca105dbaf0d02732cd768b35b` test executable: **1 passed each**, both original succession saves retained, identical archive projection `810292a432f5d839ae02f3ad14da96254f57429b2de2327a48f3ecd68b5cce48`. Executable hashes were checked before and after. `runtime-source-equivalence.json` proves the baseline/harness runtime Git trees equal that binary's source revision. No Cargo build was run by this review.

| Attempt | Guards | Activation + reload + seven days | Browser | Exit |
| --- | --- | --- | --- | --- |
| `independent-01`, harness `abd6ecf2` | 23 passed | 21 passed, 2 unrun | 4 blocked, 19 unrun | 1 |
| `independent-02`, harness `32135770` | 23 passed | 21 passed, 2 unrun | 4 passed, 19 unrun | 0 |

Attempt 01 stopped browser work because `ci-integrated` compared immutable served bytes to this checkout's different raw newline bytes (`fiscal-recovery-ui.js`). That attempt is retained unchanged. After the narrow comparison correction, attempt 02 passed Russia, Kazakhstan, Serbia and Croatia through ordinary Load → Continue → Government/Budget/Advisors → Save → Load controls, including post-load 390px inspection. Both attempts have complete, independently revalidated result records; validation success alone does not turn attempt 01's blocked cases into passes. Servers stopped and transient save directories were empty after both runs. Every original fixture and staging input remains recoverable from gzip.

## Scope and remaining work

- **All native/browser evidence here is baseline 5d11, not root's subsequent F1 budget fix `52e1c2ab` or current-runtime S24 qualification.** The pre-fix Serbia interest-floor presentation is visible and retained, not silently corrected in these screenshots. Root separately owns its fix and newer evidence.
- The 23-identity expectation stays intact. Namibia and East Timor have no native activation hook; their activation/browser cases remain `unrun/no_native_hook` with the existing integration proposal.
- Browser scope is four representatives; 17 supported identities were not browser-selected, plus two with no hook. Map ownership comes from native served district data, not globe inspection. Guidance checks identify the governed country and response, not advice quality.
- Every activation is an explicitly forced/authored **2 January 1990 fixture**. Seven days of successor play is not organic dissolution, historical timing, a full campaign, or qualification of the historical officeholder/portrait roster. Government role fallback text is not proof of named historical leaders.
- Claude's original exporter archives were not retained in its submission. Its claimed two-run reproducibility is preserved as historical hash receipts; these new independently executed/retained originals supply review evidence without fabricating recovery of Claude's deleted bytes.
- The canonical S24 session still depends on S23, a complete current-build review, and resolution of the two missing activation paths. No global workboard or queue was modified by this review.

## Inspect and restore

`manifest.json` hashes every packet file other than itself as exact raw bytes (`.gitattributes` disables text conversion). `source-map.json` retains original local evidence paths. Result descriptors under each attempt's `fixture_inputs[].retained` contain gzip and decoded SHA-256/byte counts; both were independently checked. The original N1 input is `evidence/independent-02/fixtures/n1-ussr-succession.json.gz`; the second exporter original is `evidence/native-s21-02-succession.json.gz`. `exporter-file-inventory.json` identifies unused active/endpoint exporter outputs without crediting them as tested successor inputs.

To restore, read the selected gzip, verify its compressed descriptor, decompress into a **new** destination, and verify the decoded byte count/SHA-256 before loading through an isolated game server. Do not overwrite a campaign or change the preserved inputs. All archive payloads were decoded and hash-checked; no whole packet or campaign replay restoration is claimed. The binary is an external immutable hash reference, not included in Git.

The complete replay command and environment are in `evidence/independent-02-invocation.json`. Supply a new absolute `--out`, the pinned binary and retained native succession input/provenance. Re-run the exporter if using a new runtime and retain that run's original input bytes; do not relabel old results as a new build.
