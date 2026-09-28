# Worldwide startup engineering review

The final browser sweep passed **137 of 137 ordinary 1990 starters**, each through seven visible one-day advances, an ordinary named save/load, and complete native archive equality apart from the terminal save timestamp. The copied runtime is `8d338dd217af0e4c83725f999812eef37e55bbb7`; the separately committed browser harness is `c35310531c247e1c0754284672f7beb482d7f75c`.

The [same-build native startup refresh](native-final/result.json) also passed 137 of 137 countries, with the same binary hash and seed, an unchanged source revision and clean completion. Its [stdout log](native-final/stdout.log) is retained. The native route checks reads, seven days, duplicate-turn handling and full campaign save/load; it does not substitute for the browser interaction proof.

This is bounded S24 preparation. It does **not** close S24, approve historical country coverage, certify a campaign, establish later-campaign continuity, or replace the outstanding S23 work. [manifest.json](manifest.json) keeps that limited scope explicit.

## What was exercised

Every country was selected through the ordinary nation picker with seed 17. The browser inspected the rendered map, government, all ten budget ministries and advisors at 1440×1000, advanced seven actual days through the visible step control, saved and loaded through Campaigns, then repeated the room checks at 390×844. Native pauses were retained. No campaign state, money, dates, fixtures or orders were injected.

The source-pinned roster includes exactly the 137 `start_1990` countries. Saved envelopes bind the display name separately from the inner native player ID, `world.rules.seed`, and 8 January 1990. Raw native JSON is compared without reserializing integers, floats, history, logs or journey records. Loading must renew the session. The only exempted POST is the game's read-only budget preview; the mutation sequence is new campaign, seven advances, save, load, save.

All six canonically empty district inventories—Bahrain, Cape Verde, Comoros, Maldives, Mauritius and Seychelles—retain their intentional geometry limitation. Their actual drawn country labels must be visible on the map, have an unobstructed click position on the canvas, successfully receive an ordinary mouse click and open the correct national dossier at both viewports. Countries with district geometry still require owned districts. No substitute territory was invented.

The fix added current-frame hit targets for drawn country labels where basemap geometry is absent. Visible country text takes precedence over an overlapping city's larger hit radius. Route/work picks retain their precedence; ordinary polygon, province and city picks outside those labels keep their original path. The focused source tests comprise 6 map-picking checks and 12 startup/verifier checks, all passing. Source-only peer review found no blocking issue; that is distinct from the live browser results.

## Results and retained failures

- [Final browser result](browser-final/result.json): 137 passed, zero failed; 959 visible advances, 274 complete native archives and 1,096 screenshots. Browser/server cleanup and frozen input rehashes passed.
- [Built-in evidence verification](browser-final/retained-verification.json) and [independent retained-tool verification](browser-final/independent-retained-verification.json): both passed all 1,377 referenced files, totaling 889,233,260 bytes. The independent invocation used the copied verifier and dependencies inside the retained run, without reading the working checkout.
- [Original complete sweep](attempts/full-01/result.json): 131 passed and six failed under the initial requirement that every starter own a district. Its failed status remains unchanged; it led to inspection of the missing label-picking path.
- [First island pilot](attempts/islands-pilot-01/result.json): three passed; Cape Verde, Maldives and Seychelles exposed nearby-city interception of the country label. The actual failures and original build pins remain intact.
- [Corrected island pilot](attempts/islands-pilot-02/result.json): all six passed on the final runtime, followed by the complete 137-country rerun. Its [retained verification](attempts/islands-pilot-02/retained-verification.json) also passed.
- [Earlier refused native invocation](attempts/native-final-02-refused.log): the expected revision differed from checkout HEAD. The launcher rejected it before any country ran. It is separate from the successful native03 refresh and is not counted as a game test failure or an executed country cell.

Earlier locator/tool pilots remain under `D:/spheres-offload/codex-next-20260928/startup-browser-pilot-01` through `startup-browser-pilot-04`. They preserved the sparse district lookup, read-only budget-preview and display-name/native-ID assumptions corrected before the final harness. None awards worldwide coverage.

The preserved full-01 provenance text also contains the superseded claim that a save has no original seed field. The actual field is nested at `world.world.rules.seed`; the final harness and copied verifier require that value to equal the ordinary new-campaign seed. The earlier failed record was retained rather than rewritten.

## Compact Git bundle and external evidence

**This Git packet is a partial review bundle, not a standalone copy of the full evidence.** It contains the original result/verification JSON, the exact captured driver and dependencies, source-pinned native roster/geometry tables, and 18 representative screenshots: all six island maps at both viewports and South Africa's government, budget and advisors at both viewports. [Copied artifact pins](copied-artifact-pins.json) identify the exact received bytes, protected from Git line-ending conversion by the local attributes file.

[artifact-ledger.json](artifact-ledger.json) inventories every referenced final proof file, with compressed and raw campaign hashes, native identity/date/seed, screenshot viewports, and explicit Git-retained versus external-only locations. The complete original root is:

`D:/spheres-offload/codex-next-20260928/startup-browser-full-02`

The 274 gzip campaign archives and unselected screenshots remain there. Complete re-verification requires that retained root; running the verifier against this partial Git directory must fail on missing evidence. With the original root present, run:

```powershell
node D:/spheres-offload/codex-next-20260928/startup-browser-full-02/sources/tools/ui/verify_worldwide_startup.cjs D:/spheres-offload/codex-next-20260928/startup-browser-full-02
```

The binary SHA-256 is `fd6989f3cbbd60abb247fd289dc41eb12c30424222b1f09971ac0e7f9f50dec7`. It was checked before and after the run; 22 served UI assets were checked against its exact source revision at startup. A later working-checkout HEAD is recorded separately and is not represented as the compiled revision. Neither the passing browser reports nor this preparation packet claims a performance measurement.
