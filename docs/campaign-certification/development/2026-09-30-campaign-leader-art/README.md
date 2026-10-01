# Dated campaign leaders and opening cartoon batch

Development packet, 30 September 2026. Base:
`301f63cb20ff3d2a90076ef978f20f2db62a1f1c`.
Task branch: `codex/opening-leader-art-20260930`.

The user retired timeless country-selector representatives in favor of the
actual campaign leader for the displayed date. The opening selector, live nation
dossier and War Room now consume the server's read-only `campaign_leader`
contract. The same saved government identity supplies its name and exact-person
portrait. Gameplay succession takes precedence over historical reference dates;
unknown identities, missing art and expired depiction intervals never borrow
another person's portrait. Fictional successors remain explicitly labeled.

The cartoon reviewer removes national representatives from active collections
and style references. Its export retains all 160 originals under
`archived_selector`, including source, rights, hashes and earlier review evidence.
Old Afghanistan and Greece selection links preserve their country context.
Greece currently has no active campaign-person records in that review inventory;
the resulting empty state is an honest content gap.

## Five new illustrations

All five are original, fixed 2D cartoons generated through OpenAI's built-in
`image_gen` tool and saved as its unchanged final PNG bytes. Each is 1024 × 1536
with an opaque teal background. Singh and Fahd received a background correction
through that same image tool; no scripted pixel editing was used.

| Country | Exact identity | Reference and limits |
|---|---|---|
| France | `francois_mitterrand` | Rob Croes/Anefo, 7 May 1988, CC0. Suit colors and full-body pose are artistic interpretation. |
| Brazil | `jose_sarney` | White House photograph, 10 September 1986, public domain. Standing-body extension and near-1990 age treatment are interpretation. |
| India | `v_p_singh` | Christian Lambiotte/European Commission, 30 May 1983, CC BY 4.0. The seven-year reference gap is disclosed; this is not a documentary 1990 photograph. Attribution and derivative license are retained. |
| Saudi Arabia | `fahd_bin_abdulaziz_al_saud` | White House/Bush Library, 21 November 1990, public domain. Reference identity is Fahd seated at the right in the white/gold cloak. Standing pose is interpretation. |
| USSR | `mikhail_gorbachev` | Susan Biddle/White House, 9 September 1990, public domain. The 200 × 187 source crop limits fine facial detail; headset removal and standing-body extension are interpretation. |

The approved Thatcher cartoon is the style anchor. Appearance registration is
**1990-01-01 inclusive to 1991-01-01 exclusive** for each image. This is a visual
depiction interval, not a claim about an office term. No historical identity,
office binding, party term or source claim changed.

Physical files and retained reference photographs are in
`spheres-web/ui/person-portraits/` and its `references/` subdirectory. Exact prompts,
reference URLs and licenses, final file sizes/hashes, dimensions, generation
outputs and correction history are in
`tools/avatars/person-prompts/opening-1990-batch-v1.json` and its adjacent prompt
files. Codex inspected identity, likeness, era interpretation and small-card
readability; this does not claim individual human approval of the five new images.

Production inventory now contains 59 historical cartoon identities/assets and
four fictional successor cartoons. There are still 699 pending historical art
jobs. Five completed illustrations do not establish worldwide content coverage.

## Verification and retained checkpoints

- `validation/final-verification-results.json` records the full avatar pass:
  594 tests. Its first UI attempt failed on absent sparse-checkout fixtures and
  is retained as a failed checkpoint.
- `validation/ui-verified-result.json` and `ui-unit-tests-verified.log` record the
  final complete `node tools/ui/run-unit.cjs` pass: 1,806 passed, zero failed and
  one intentionally skipped browser journey, exercised separately below.
- `validation/final-freshness.json` passes cartoon-review and boundary-export
  freshness plus the workboard contract. `doc-links.json` verifies all 44 links
  in the five affected documentation entry points.
- `browser/verified/result.json` records the final 109-test focused UI/reviewer
  run with no skipped tests, plus screenshots at 106 × 152, 90 × 132 and
  156 × 218 and larger previews for all five portraits.
- `browser/native-selector-final/result.json` passes all six real native-browser cases:
  the five new portraits and Afghanistan's named missing-art fallback. The
  compiled HTML and served PNG bytes match the reviewed working-tree files.
  There were no legacy figure requests, POSTs or saves, and `/api/state` was
  byte-identical before and after. The disposable server was shut down. The earlier
  `native-selector/` run remains retained; the final run pins the explicit release
  build, SHA-256 `563476f896b1a5378e229ee45c4212f9bc7a338fe73087f141a696d8ef3474e9`.
- `native/attempt-02/results.json` records the successful full web package:
  455 passed, zero failed, 28 explicitly ignored; the separate release build also
  passed with unchanged source input hashes. Those ignored fixture/qualification
  tests were not claimed as run. Full-workspace results remain separate.
- `native/workspace-02/results.json` records the complete ordinary workspace pass:
  1,989 passed, zero failed, 115 ignored and one explicitly excluded resource
  timing test. It includes all restored examples. `workspace-01` retains an
  intentional compile-phase stop before any tests ran; resuming with two build
  workers reused compiled targets without changing assertions or filters.
  These results test base `301f63cb` plus this art/presentation change. The later
  upstream political repairs require a separate combined integration check.
- The first avatar discovery run failed because this sparse checkout omitted
  checked-in C04 and source-audit fixtures. Restoring the original files resolved
  the failure; no assertions or source evidence were weakened.
- Two full UI checkpoints lacked original equipment-model/arsenal/fixture files
  and then S24 successor evidence. Restoring the checked-in originals produced
  the final green run. Both failed logs remain alongside it; no game or test
  assertion was changed to accommodate missing files.
- The first boundary export similarly lacked the checked-in integration record.
  Restoring that original input allowed generation and freshness verification of
  all 8,127 existing cases.
- `browser/final/` preserves a failed expanded checkpoint that exposed Greece's
  absence from the exported country-name catalog. The repair keeps all 160
  country names while leaving retired portraits inactive. The final verifier also
  fixes an inherited receipt bug: the last passing subtest cannot overwrite an
  earlier failure with an overall success flag.
- `native/attempt-01/` preserves an unsuccessful package-build invocation
  (exit 101, no test results). It is not counted as a passing native check.

This packet changes presentation, art and production tooling. It awards no C03,
C06, S23 or CP1 completion. Existing full-campaign and A1 calibration jobs and the
user's active server/saves are preserved. Absolute resource timing must run alone;
concurrent campaign/calibration work is not valid timing qualification.

## Claude handoff

The new direction and current research inventory are recorded in
`docs/planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md`, linked from `CLAUDE.md`,
the workboard and both current Claude entry handoffs. Remote claims/deliveries are
distinguished from accepted research, installed identities and finished art.
