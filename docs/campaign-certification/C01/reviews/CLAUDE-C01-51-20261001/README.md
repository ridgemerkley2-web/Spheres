# Independent Russia C01-51 review — 1 October 2026

**Accepted as bounded research after one report clarification.** All 16 official portal document texts, all 16 single-act metadata cards, all 20 new claims and all nine added holder observations were materially reviewed. The two original C01-05 holder records, every prior source and claim, and every submitted JSON object remain unchanged. This does not complete either presidency chain, the country census, C01, C06, S23 or a parent qualification.

Exact source: `origin/claude/c01-ru-51` at `2fe2b752e05125e204832f5cd1f60b4c640af28d`. Review base: `5ea4f8fcd05e1858b1dfa83ae34c6d3703d6b104`. Isolated branch: `codex/review-c01-51-20261001`.

- Exact authored import: `90f7f54c2cbc3685623409eece12ee4ca592b066` — 26 files verified against the submitted Git blobs; generated index excluded.
- Report clarification: `1369952268b6c5b65f9c74bcda0017f00dacd1d3` — one file, nine insertions and six deletions; no packet, extract, test, holder or pin edits.
- This separate receipt records the independent source interpretation and validation without self-referencing its eventual commit.

## Source reading and provenance

The root review agent retrieved the official `pravo.gov.ru` original-edition document frames and single-act portal cards. This reviewer read all 16 retained document texts and all 16 cards after Windows-1251 decoding, including dates, headings, signatures, operative points and publication citations. Each of the 32 successful HTTP 200 response bodies matches the submitted byte count and SHA-256. The three first-attempt failures and their separately retained successful retries remain in `source-attempts.json`. No network requests or silent response replacement were made by this reviewer.

The canonical packet URLs use HTTPS; the actually retrieved original identities are explicitly HTTP. These are portal original-edition HTML texts (`rdk=0`), not inspected gazette facsimiles. Twelve cards cite publications; the three VP orders and resolution 5780-I do not. Cited journal pages were not retrieved or inspected. Current portal validity labels are metadata, not office terms. Author-reported earlier stability checks are preserved; this receipt independently verifies only the retained root retrievals. Raw originals remain outside Git, with no open-license or portrait approval assertion.

## Accepted interpretation and correction

The four additional Yeltsin signatures and five Rutskoi office observations remain dated observations with no effective start. The postrename RSFSR signatures attest the exact printed styling; they do not extend an entity lifetime. The 1992–1993 Russian Federation vice-presidential title is retained on the existing research role with the title-specific evidence stated.

Decree 1328 explicitly names Rutskoi as vice-president while temporarily suspending his duties. That supports the dated title observation; suspension is not removal. Decree 1576 explicitly releases him and states effect on signature. The `until: 1993-10-03` records that decree-stated effect, **not an adjudication of constitutional legitimacy**, a whole interval of active service, or the office's abolition. Competing September 21/22 termination and acting-service declarations remain five attributed claims on `ru_president`, with no holder added or boundary granted. Decree 316 restyling and the three duties/procedure claims likewise remain claim-only.

The submitted report called a broader December 20–January 10 portal listing sweep complete and referred to a December 29 act. No listing response or December 29 original was retained. RU-RSP-02 now limits independently verified evidence to the retained December 26 decrees 316/318 and January 3 decree 245-н, and explicitly attributes the broader sweep to the author as unverified. No accepted claim relied on the missing listing. The report also states the entity-lifetime and constitutional-legitimacy limits above.

`scope.json` proves the data delta is append-only: 16 sources, 20 claims, nine holders, role citation suffixes and four coverage notes. All old objects are unchanged. `source-review.json`, `claim-review.json` and `holder-review.json` provide per-record dispositions rather than treating hashes or green tests as historical acceptance.

## Regression review and validation

All six modified older test modules and the full new module were read. Prior fixtures, source partitions, original holder hashes and office separation remain. The explicit new removal exception is independently constrained by exact holder fixtures and citation/date guards. No test deletion, skip or broad weakening was found; no test repair was necessary.

Fresh runs passed: 44 Russia, 48 USSR, 79 research, 16 campaign and 11 atlas Node tests. The Python groups overlap and are not a unique-test total. The authored 26 mutation checks pass. Six additional independent negative controls reject wrong starts, suspension ends, altered original holders, missing removal evidence, competing acting holders and inferred entity lifetimes. Census and workboard freshness checks pass. A locally regenerated research index passes `--check`; its prior bytes were restored and neither submitted nor generated index is imported. Exact commands, timestamps, logs and exit codes are under `validation/`.

No native build, full avatar suite, production browser test, human acceptance or new HTTPS packet re-fetch was performed. Root owns combined metadata regeneration and integration. Full 1991–1993 historical coverage remains open, including continuous intervals, source-supported title transition reconciliation and other constitutional-crisis evidence.

## Reproduction

External retained evidence: `D:/spheres-offload/codex-next-20260928/claude-review-20261001-04/russia-evidence`.

```powershell
python docs/campaign-certification/C01/reviews/CLAUDE-C01-51-20261001/verify.py --repo . --external-root D:/spheres-offload/codex-next-20260928/claude-review-20261001-04/russia-evidence
```

The verifier checks receipt integrity, exact commit-backed source blobs, original response bodies and retained failures. It cannot recreate historical reading or convert test/hash success into acceptance.
