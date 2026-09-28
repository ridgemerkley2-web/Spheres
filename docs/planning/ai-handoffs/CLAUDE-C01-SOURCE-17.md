# CLAUDE-C01-SOURCE-17: Compare the changed Brazilian Senate diary response

Owner: Claude. State: **ready_for_review** (27 September 2026; submitted, not accepted). Parent: C01 (incomplete).

Assigned by `docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md` and `docs/planning/ai-task-queue.json` (task
`CLAUDE-C01-SOURCE-17`, queued 27 September 2026) as a source-review follow-up to the integrated CLAUDE-C01-10 and CLAUDE-C01-17.
It repairs integrated research; it does not claim historical acceptance, which Codex decides.

Branch: `claude/c01-source-17`. Base: `a33a8987` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded repair

Compare the Senate's stored DCN No. 1/2023 download (BuscaPaginasDiario?codDiario=111711&download=true; recorded 24,950,218 bytes, sha256 d6c9c275…c854ee; fetched by Codex as 24,950,158 bytes, d9808101…a619) against the recorded extracts. Record the exact pages, the publication identity (PDF metadata, page count, verification code) and factual agreement or disagreement page by page for every cited page, rather than replacing a checksum blindly; if the file has changed, record both identities and what changed.

Keep historical dates unchanged unless the cited evidence supports a correction. Do not convert missing
response identities into fabricated checksums; retain disclosed limitations and propose further bounded review
separately. Never bypass an access control, CAPTCHA or bot block; record a blocked source instead.

## Allowed files and checks

Allowed files:

- this record;
- `docs/campaign-certification/C01/research/brazil.json` (the source records of the stored DCN No. 1/2023 issue only);
- the extracts of those source records under `docs/campaign-certification/C01/research/sources/`;
- the CLAUDE-C01-10 and CLAUDE-C01-17 reports under `docs/campaign-certification/C01/research/`;
- `tools/avatars/test_brazil_presidents_c01_10.py`, `tools/avatars/test_brazil_vice_presidents_c01_17.py` and any test pinning the changed values;
- `docs/campaign-certification/C01/research-index.json`, regenerated in a separate commit.

Checks: research-index `--check`; the Brazil, research and campaign Python tests with game data present
(campaign census included); `campaign_census.py --check`; the atlas Node check; `workboard.py --check`;
`git diff --check`.

Return `ready_for_review` with exact commits, original-response identities where available, content locators and
remaining gaps.

## Result

Submitted `ready_for_review` on 27 September 2026 (the downloads ran from 2026-09-27T23:42:00Z to
2026-09-28T00:45:09Z) on `claude/c01-source-17`, based on `a33a8987`. After the claim commit `803ad03d` come two
commits: "Review source for CLAUDE-C01-SOURCE-17" (`1de0bba6`; the records, extracts, reports and this record) and
"Regenerate the C01 research index for CLAUDE-C01-SOURCE-17" (`35d1d438`; `research-index.json` only).
`codex/campaign-certification` has since moved to `fbc2a041` with S19 commits that touch no C01 path, so no merge was
needed. The verifier's fixes follow in two more commits: "Apply verifier fixes to CLAUDE-C01-SOURCE-17" (page 7 of the
p1_8 review also lists `br_alckmin_elected_20221030`; the p1_8 extract becomes 23,995 bytes and `brazil.json` 784,614
bytes, SHA-256 `3c11128ec3dac3b18dbb07bff1b3cae498d67e30c913716e3353ec2cce8ee8fd`) and "Regenerate the C01 research
index after CLAUDE-C01-SOURCE-17 fixes" (`research-index.json` only). When the fixes were made,
`codex/campaign-certification` was at `e41aa18d`; its commits since `a33a8987` touch no C01 research file and none of
this repair's files, so again no merge was needed. This repairs integrated research; historical acceptance stays with
Codex.

### Finding

The stored DCN nº 1/2023 response has **not changed**, and the endpoint does not rebuild it per request. All 9
downloads of `https://legis.senado.leg.br/diarios/BuscaPaginasDiario?codDiario=111711&download=true` in this review (6
origin responses with `Age: 0`, plain and cache-busted; the rest edge-cache copies; curl 8.16.0 and Python urllib; no
browser User-Agent) returned the recorded 24,950,218 bytes and SHA-256, and so does the raw Internet Archive capture
of 9 January 2023. The text of all 59 pages and the renders of every compared page are identical to those made from
the recorded response on 24-25 September and from the page-range form on 23 September. Codex's 24,950,158-byte body
(SHA-256 `d9808101…f6a619`) could not be reproduced; it is not a truncated copy of the recorded file, and the Senate
service has returned damaged HTTP 200 bodies before (this URL gave 19,397,271 bytes to the CLAUDE-C01-17 verification
at 01:34:49 UTC on 25 September, then the recorded identity six minutes later). The recorded identity is kept, not
replaced. Every claim on the cited pages agrees with the file; one transcription is corrected (page 20: the diploma
prints "FE Brasil (PT/PCDOB/PV)" and `br_alckmin_diplomado_tse_20221212` had "PCdoB"). No date, holder, role, URL,
access date or response identity changes.

### Identities

- Recorded (CLAUDE-C01-10 on 23 September 2026; CLAUDE-C01-17 on 24-25 September): 24,950,218 bytes, SHA-256
  `d6c9c275da00f4d8b73a3127fa98c53a7723f7779445d6a33e93131d7ec854ee`.
- Codex's premerge sample (27 September 2026, fetch time not recorded): 24,950,158 bytes, SHA-256
  `d9808101c1b9926db26d4c0aaaaaa39d98d4dabaf0fc55200ebc305521f6a619`. Not reproduced; the first 24,950,158 bytes of
  the recorded file hash to `6149e0ee49ca18f915ca0598f2a45568ac3bdff3fe2017e04bd933baa2c9761f`. The body was not kept
  anywhere on this machine.
- This review (each 24,950,218 bytes with the recorded SHA-256):
  - 2026-09-27T23:42:00Z: plain URL; curl 8.16.0 default User-Agent; Accept-Encoding: identity; HTTP 200, Age 0,
    chunked (origin).
  - 2026-09-27T23:46:27Z: cache-busting query &_cb=1790552787; curl; HTTP 200, Age 0, chunked (origin).
  - 2026-09-27T23:49:04Z: plain URL; curl with Accept-Encoding: gzip and the body left undecoded; HTTP 200, Age 422,
    Content-Length 24950218 (edge cache); served without Content-Encoding: the stored bytes, not a gzip body.
  - 2026-09-27T23:52:24Z: cache-busting query with a Range: bytes=-300 header; curl; HTTP 200, Age 0, chunked
    (origin); the origin ignored the Range header and sent the whole file.
  - 2026-09-27T23:55:18Z: cache-busting query; Python 3.13 urllib with its default headers; HTTP 200, Age 0, chunked
    (origin).
  - 2026-09-28T00:12:23Z: plain URL, 30 minutes after the first; curl; HTTP 200, Age 1822, Content-Length 24950218
    (edge cache).
  - 2026-09-28T00:15:08Z: cache-busting query, 33 minutes after the first; curl; HTTP 200, Age 0, chunked (origin).
  - 2026-09-28T00:41:32Z: cache-busting query &_cb=1790556092g; curl with Accept-Encoding: gzip and the body left
    undecoded; HTTP 200, Age 0, chunked (origin); no Content-Encoding: the origin also sends the stored bytes to a
    client that accepts gzip.
  - 2026-09-28T00:45:09Z: plain URL, 63 minutes after the first; curl; HTTP 200, Age 1965, Content-Length 24950218
    (edge cache); the edge stored this copy at about 00:12:25 UTC (Age 1965 at 00:45:10), so it is a second origin
    response for the plain URL, taken 30 minutes after the first.
- Independent capture:
  <https://web.archive.org/web/20230109012106id_/https://legis.senado.leg.br/diarios/BuscaPaginasDiario?codDiario=111711&paginaInicial=&paginaFinal=>
  (memento 2023-01-09T01:21:06Z, fetched 2026-09-27T23:58:45Z with `Accept-Encoding: identity`, served uncompressed
  with Content-Length 24950218, so the identity is the uncompressed body): 24,950,218 bytes, SHA-256
  `d6c9c275da00f4d8b73a3127fa98c53a7723f7779445d6a33e93131d7ec854ee`. CDX digest `5XNO3LSPQRA632PUOW7CPJCNFSAD6SEY` =
  base-32 SHA-1 `eddaedae4f8441ede9f475be27a44d2c803f4898` of the recorded file. It captures the viewer's older
  whole-issue form; the download=true URL has no capture.
- Publication identity: `DCN-1-2023.pdf`, 59 pages, `%PDF-1.4`; Producer "pdfTeX; modified using iTextSharp 5.3.0 (c)
  1T3XT BVBA"; Creator "pdfTeX + pdfx.sty with a-2b,mathxmp option"; CreationDate `D:20230102105150-03'00'`; ModDate
  `D:20230102105722-03'00'`; trailer /ID `F1B0B1E3FCCB08D1E49118BC6481F4B2` (both entries); XMP DocumentID
  `uuid:21C75AEE-FD83-0922-042D-64C0C5F394C5`, InstanceID `uuid:897B27B4-2FEF-4C85-D158-17566F6146CE`, PDF/A-2B; no
  embedded signature dictionary. Printed on all 59 pages: verification code `E08D962B004C6AB2` ("ARQUIVO ASSINADO
  DIGITALMENTE"), http://www.senado.gov.br/sigadweb/v.aspx and document number 00100.000338/2023-38.
- Page-range contrast: the two surviving copies of pages 1-8 from CLAUDE-C01-10's check and verification (4,301,947
  bytes each; `1e7d89ee…d8b9f5` and `06abb771…044883`) differ in 38 bytes, the Aspose.PDF for Java 23.6
  CreationDate/ModDate (09:25:06 and 11:00:05 -03:00 on 23 September 2026) and the trailer /ID, as the records already
  say; the download=true file has no such variation.

### Locators and results

| Source record | PDF pages compared (claims) | Result |
|---|---|---|
| `br_cn_dcn1_20230102_p1_8` | 1 (0), 3 (0), 4 (0), 5 (2), 6 (6), 7 (5), 8 (1) | all agree |
| `br_cn_dcn1_20230102_p18_19` | 18 (1), 19 (0) | agrees |
| `br_cn_dcn1_20230102_p20_26` | 17 (0), 20 (1), 21 (0), 22 (0), 23 (1), 24 (0), 26 (1) | agree; page 20 "PCdoB" corrected to the printed "PCDOB" |

Every page's observation (quotations, TSE authenticity codes `14eca4364702198c0fb5dabb014ece5f` on page 18 and
`fa11f8629e61074499d6e7268a9e665b` on page 20, and what was compared) is in each extract's `source_review.pages` and
in the "Source review (CLAUDE-C01-SOURCE-17)" sections of
[brazil-presidents-1990-2026-10.md](../../campaign-certification/C01/research/brazil-presidents-1990-2026-10.md) and
[brazil-vice-presidents-1990-2026-17.md](../../campaign-certification/C01/research/brazil-vice-presidents-1990-2026-17.md).
Two notes that change no claim: page 7's termo reading drops "Rodrigues" from the heading (already in the claim's
uncertainty), and the signed termo on page 23 cites "artigo sessenta e cinco do Regimento Comum" where the reading on
page 7 says art. 75.

### Changed files

- `docs/campaign-certification/C01/research/brazil.json`: a compact `source_review` on `br_cn_dcn1_20230102_p1_8`,
  `br_cn_dcn1_20230102_p18_19` and `br_cn_dcn1_20230102_p20_26`; their snapshot values; the text of
  `br_alckmin_diplomado_tse_20221212` ("PCdoB" to "PCDOB"). After the verifier fixes it is 784,614 bytes, SHA-256
  `3c11128ec3dac3b18dbb07bff1b3cae498d67e30c913716e3353ec2cce8ee8fd` (784,569 bytes, `1477443b…b95164`, at
  submission).
- `sources/brazil-congress-dcn-lula-posse-20230102-facts.json`: the full `source_review`; 10,023 bytes (`b03c6ef0…`)
  to 23,995 bytes, SHA-256 `92d55224586cd9d035b0a48be87c765605e7e66b868de7495709c427b546caf3`.
- `sources/brazil-congress-dcn-lula-diploma-20230102-facts.json`: the full `source_review`; 3,760 bytes (`09bdecde…`)
  to 14,943 bytes, SHA-256 `eb3981bb77ea7effe603ab624ccc236b8b52c5a7d023d375c3337cf80b67dd13`.
- `sources/brazil-congress-dcn-alckmin-diploma-termo-20230102-facts.json`: the full `source_review` and the corrected
  row text; 5,878 bytes (`b657689a…`) to 19,836 bytes, SHA-256
  `08b9dee4ce9808fe103c5b7de173862b861a359b2aebe477dca8ea626f934ec5`.
- `brazil-presidents-1990-2026-10.md` and `brazil-vice-presidents-1990-2026-17.md`: a dated "Source review
  (CLAUDE-C01-SOURCE-17)" section each (CRLF working-tree endings kept).
- This record.
- `docs/campaign-certification/C01/research-index.json`: regenerated in its own commit, and again in "Regenerate the
  C01 research index after CLAUDE-C01-SOURCE-17 fixes" (totals unchanged; only `brazil.json`'s bytes and SHA-256
  change).
- Verifier fixes ("Apply verifier fixes to CLAUDE-C01-SOURCE-17"): `br_alckmin_elected_20221030`, whose locator cites
  the termo on PDF page 7, is added to page 7's claims in the p1_8 `source_review.pages` of `brazil.json` and of the
  extract and in the CLAUDE-C01-17 report's page list; page 7 agrees (the termo says both were elected "no dia 30 de
  outubro de 2022"). The p1_8 extract identity is updated in `brazil.json`, both reports and this record.
- Tests: none changed. No pinned value changed: the response identities, URLs, access dates and page pins are the
  same, the corrected claim text is not pinned, and extract snapshots are checked against the files.

### Checks

- `python -X utf8 tools/avatars/campaign_research.py`: pass (9 packets, 1,329 sources, 3,731 claims, 93 discovery
  batches)
- `python -X utf8 tools/avatars/campaign_research.py --check`: pass (9 packets, 1,329 sources, 3,731 claims, 93
  discovery batches)
- `python -X utf8 tools/avatars/campaign_census.py --check`: pass (exit code 0)
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_brazil*.py"`: pass: 33 tests, OK (none skipped)
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"`: pass on rerun: 79 tests, OK (none
  skipped); the first run of this final pass hit the flake noted below (1 error in test_research_import_io)
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"`: pass: 16 tests, OK (none skipped)
- `node --test tools/ui/check_leadership_research_review.cjs`: pass: 11 passed, 0 failed
- `python tools/planning/workboard.py --check`: pass: PASS: 44 canonical markers, each assigned once; handoffs and
  dependencies exist. No status is duplicated. 10 bounded tasks validated separately.
- `git diff --check (this repair's paths)`: pass (exit code 0)
- Flake seen, not caused by this repair: in 17 runs of the research pattern during this review, three ended with one
  error in `test_research_import_io` (each kept traceback shows `PermissionError: [WinError 5] Access is denied` from
  `os.replace` inside `%TEMP%`); that test touches no repository file and passed on rerun, and every other check
  passed on its first run.
- Rerun after the verifier fixes: all nine checks above pass with the same results (9 packets, 1,329 sources, 3,731
  claims and 93 discovery batches; 33, 79 and 16 tests, OK, none skipped; 11 node tests passed; 44 workboard markers;
  `git diff --check` exit code 0 on this repair's paths).

### Remaining gaps

- Codex's 24,950,158-byte body and its fetch time are unknown, so which bytes differed cannot be shown. A re-download
  that is not 24,950,218 bytes should be retried: origin responses are chunked without a Content-Length, so a damaged
  body is not visible in the headers. The raw capture above is a second way to check the identity.
- The Senate's verification service (sigadweb/v.aspx) was not queried and the QR codes were not decoded; only the
  printed verification code is recorded.
- Proposed separately, not done here: a test pinning the review's identities and the corrected text, if Codex wants it
  (tests change here only where a pinned value changed); the same comparison for the other Senate stored issues in
  `brazil.json`, whose responses were not in Codex's sample.
