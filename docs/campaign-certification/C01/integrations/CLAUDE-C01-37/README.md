# Independent review — CLAUDE-C01-37

**Accepted as a bounded source-observation intake after a conservative effective-boundary repair.** This does not close C01, C06, S23, WC1 or CP1.

Reviewer: Codex `/root/s20_preflight`, 30 September 2026. Exact received submission: `eb7006176756796f5fd0a7244d9a3316ab275e60`; own pre-packet parent: `44098c5a`. Isolated branch: `codex/review-c01-37-20260930`. [scope-audit.json](scope-audit.json) pins all 39 submitted paths and preserves the received and reviewed holder records.

## Findings and repair

All **31 original response identities match exactly** after an initial 20 successes and an 11-source missing-only retry. All 11 original connection failures are retained. Both gzip responses also match their declared decoded identities. All **48 material claims and locators** were independently read in the archived Journal officiel text pages. No original source hash, event date, claim text or locator was replaced. No PDF was needed or visually reviewed for this HTML-text packet.

The original submission inferred **12 effective starts and 14 effective ends** from appointment/cessation instrument signing dates. The actual text supports the named operative acts and their dates, but supplies no separate explicit effective-date clause. The report acknowledged that inference and attributed a general ruling to Ridge; that attribution was not independently established and is not used as authority. Under the existing accepted evidence contract, the inferred effective boundaries were removed. This is not a finding that the proposed historical dates are false.

Every appointment and cessation instrument remains a dated claim, separate from the resignation letters mentioned in the decrees and their publication metadata. Sixteen named observations of ten people remain: four in-office signatures/countersignatures and twelve appointment-instrument observations. `attested_on` dates each first cited source event; it does not assert that an effective term began on that date. All reviewed `from`/`until` values are unknown. The original 26 proposed values and the exact replacements are in [boundary-review.json](boundary-review.json). Effective-term dates, missing decree texts, any current-affairs continuation, and the post-March-2014 batch remain open research.

A separate wording correction removes the claim that publication is always later: the preserved Bérégovoy cessation page itself prints 29 March 1993 as both instrument date and Journal officiel header date. Publication metadata remains distinct and is not substituted as a boundary. The receipt does not independently authenticate that printed header against a paper issue.

Repairs:

- `2942fcd7`: distinguish publication metadata and the unverified interpretation attribution.
- `73d748ef`: remove inferred effective boundaries, retain observations, clarify uncertainty, update focused checks.
- `9cdae891`: regenerate the research index.

The hand-maintained supplement is the only data input edited. `france.json` was regenerated using the **unchanged** `import_cnccfp_census.py`. Accepted C01-23's 49 sources and their extracts, presidential institution/holders and earlier coverage note are unchanged, as are all 635 financial-register organizations. No runtime, artwork, game mapping, successor, party-history or source-image rights were changed or granted.

## Validation

Final checks passed: **20 France tests**, **10 importer tests**, **79 research tests**, **16 campaign tests**, **11 Node leadership-review tests**, importer/index/workboard checks and whitespace validation. Discovery patterns overlap; these are execution counts, not unique-test totals.

The new focused regression first failed against the original unchanged data, then passed after repair. It checks every named observation and rejects more than 100 attempted promotions of signing/publication dates into effective starts/ends. Existing claim/date/order/source-separation checks remain. The retained `validation/corrected-first/france.log` contains an intermediate indentation error in the new test after unused-local cleanup; the final France log records the correction and 20 passing tests. Neither that failure nor the original red regression was overwritten.

`campaign_census.py --check` retains the known pre-existing stale government-file pin. [baseline-limit.json](baseline-limit.json) proves the checker, census and government bytes are identical before this packet and at submission. This unrelated failure is not hidden by regenerating the census. No Cargo, gameplay browser campaign, full avatar suite, or historical holdout was run. Packet attribution and combined-index reconciliation remain the integrator's responsibility.

## Retention and independent checks

[manifest.json](manifest.json) hashes every checked-in receipt payload exactly. [source-attempts.json](source-attempts.json) retains all successful and failed attempt identities. [source-verification.json](source-verification.json) lists the exact raw and decoded pins. [claim-review.json](claim-review.json) records the bounded content decisions. Full source bodies and extraction text remain external at `D:/spheres-offload/codex-next-20260928/c01-37-review-evidence-01`; they are not republished in Git.

```powershell
python -B -X utf8 verify_receipt.py
python -B -X utf8 verify_receipt.py --external-evidence D:/spheres-offload/codex-next-20260928/c01-37-review-evidence-01
```

The first checks compact receipt integrity. The second additionally rehashes all 31 external raw responses and two decoded files. These checks do not repeat human source-content review, legal interpretation or historical research. The Git bundle alone does not contain full original responses. The accepted scope is evidence-backed observations with explicit gaps, not complete uninterrupted prime-minister terms or a certified campaign.
