# CLAUDE-C01-SOURCE-05: Recover and verify the Russian archive resolution

Owner: Claude. State: **ready_for_review** (27 September 2026; historical acceptance pending Codex). Parent: C01 (incomplete).

Assigned by `docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md` and `docs/planning/ai-task-queue.json` (task
`CLAUDE-C01-SOURCE-05`, queued 27 September 2026) as a source-review follow-up to the integrated CLAUDE-C01-05.
It repairs integrated research; it does not claim historical acceptance, which Codex decides.

Branch: `claude/c01-source-05`. Base: `a33a8987` (current `codex/campaign-certification`). Claim commit:
`d276830e`, this record's first commit on the branch.

## Bounded repair

Reproduce the Russian State Archive (projects.rusarchives.ru) response for CEC resolution No. 20-25 of 19 June 1991 (recorded 15,700 bytes, sha256 54000d3c…683e), or provide an accessible primary facsimile or archive capture with a content-level comparison. Codex's spot-check timed out; do not infer a wrong claim from that timeout. Preserve the unresolved leaf-number discrepancy (caption L. 6-7 versus folio numbers 5, 6 and 7).

Keep historical dates unchanged unless the cited evidence supports a correction. Do not convert missing
response identities into fabricated checksums; retain disclosed limitations and propose further bounded review
separately. Never bypass an access control, CAPTCHA or bot block; record a blocked source instead.

## Allowed files and checks

Allowed files:

- this record;
- `docs/campaign-certification/C01/research/russia.json` (the `ru_cec_resolution_20_25_19910619` source record only);
- `docs/campaign-certification/C01/research/sources/russia-garf-cec-result-19910619-facts.json` and any new specifically related `russia-*-facts.json` extract;
- `docs/campaign-certification/C01/research/ussr-russia-transition-1991-05.md`;
- `tools/avatars/test_ussr_russia_transition_c01_05.py` and any test pinning the changed values;
- `docs/campaign-certification/C01/research-index.json`, regenerated in a separate commit.

Checks: research-index `--check`; the Russia, research and campaign Python tests with game data present
(campaign census included); `campaign_census.py --check`; the atlas Node check; `workboard.py --check`;
`git diff --check`.

Return `ready_for_review` with exact commits, original-response identities where available, content locators and
remaining gaps.

## Result (27 September 2026)

**The recorded response reproduces exactly; no claim, date, locator or uncertainty changed.** Codex's
27 September sample requested the official page URL (the packet `url` and the extract's `source_url`) and timed
out. That URL is not the recorded response. The recorded response has always been the raw Internet Archive
capture in the extract's `source_response_url`, which the sample did not request. To re-check it:

```text
curl -H "Accept-Encoding: identity" -o page.html https://web.archive.org/web/20230604070106id_/https://projects.rusarchives.ru/statehood/09-37-postanovlenie-vybory-prezident.shtml
```

Identities are the SHA-256 of the body as served (no Content-Encoding, so uncompressed); every archive
response was an `X-Page-Cache: MISS`. Times are UTC, from 2026-09-27 23:40:28 to 2026-09-28 00:55:57.

| Response | Bytes | SHA-256 | Downloads |
|---|---|---|---|
| Recorded page, capture 20230604070106 (`source_response_url`) | 15,700 | `54000d3ca315063827147ddd5112ac8f2bb153d5d133ffc0d9ebb7c98019683e` | 2026-09-27T23:42:08Z, 2026-09-28T00:16:28Z, 2026-09-28T00:55:57Z; exact match each time |
| Page captures 20210225052658, 20210511100649, 20210612153223, 20210924090951, 20230929012544 (revisit) | 15,700 | the same | one download each (20210924090951 twice, at 2026-09-27T23:42:54Z and 2026-09-28T00:19:33Z); exact match each time |
| Page captures 20190823005913, 20191208064002 | 16,109 | `aadfea65a688e27838a92da1f8d483eb5acb5bd52a02ad7c514c832a25493a24` | one download each; differ from the recorded page only in the Yandex.Metrika counter script |
| Facsimile 1, capture 20191208064007 | 98,762 | `9052337ceab3818e95968197989853581e6b3aaf21560455ced9ad8c2c62b6c2` | 2026-09-27T23:43:29Z, 2026-09-28T00:17:14Z; exact match each time |
| Facsimile 2, capture 20191208064008 | 133,816 | `0a6d5e086e016decf3cd8167ce320e3628b10290f5698f8d6d8f6db4596a5229` | 2026-09-27T23:43:30Z, 2026-09-28T00:18:01Z; exact match each time |
| Facsimile 3, capture 20191208064005 | 114,508 | `188d308d5631fdb54d5873014d3f3a9b7a91b0f9c97cd337b21173c5a09e328e` | 2026-09-27T23:43:40Z, 2026-09-28T00:18:47Z; exact match each time |
| Official host `projects.rusarchives.ru` (92.50.233.124), ports 443 and 80 | none | none | no TCP connection at 2026-09-27T23:40:28Z, 2026-09-27T23:40:54Z, 2026-09-27T23:41:15Z, 2026-09-28T00:13:05Z, 2026-09-28T00:13:27Z, 2026-09-28T00:55:14Z, 2026-09-28T00:55:36Z (curl exit 28 after about 21 s); `rusarchives.ru` likewise at 2026-09-27T23:41:37Z |

Captures were listed through the Wayback CDX API bounded by the cutoff (`to=20260906235959`), and each CDX
digest equals the SHA-1 of the downloaded body. The 20230929012541 record is a redirect with no body. The six
identical page captures come from Archive Team, Common Crawl and the Internet Archive's own crawls, 2021-2023,
so the recorded bytes are what the official server sent, not a per-request rendering.

Content locators and comparison (all four claims of `ru_garf_cec_result_19910619` agree):

- Exhibit caption (page capture): resolution "Об итогах выборов Президента РСФСР", 19 June 1991, original
  typescript, GARF F. 10026, Op. 8, D. 204, L. 6-7; image captions L. 6, L. 6об., L. 7. Identical in every
  capture with a page body.
- Facsimile image 1 (handwritten folio 5): resolution No. 20-25 of 19 June 1991; point 1 takes note of chairman
  V. I. Kazakov's report on the results; point 2 approves the communication's text; point 3 orders it published
  through the Telegraph Agency of the Soviet Union (TASS) in the republican and local press; signed beside the
  typed names of V. Kazakov (chairman) and V. Prozorov (secretary). This matches
  `ru_cec_resolution_20_25_19910619` in every element.
- Facsimile images 2 and 3 (folios 6 and 7): voting on 12 June 1991, 88 districts, the figures, the invalid
  ballots and the Article 15 paragraphs, as in the other three claims.
- The page's own excerpt, which no claim cites, differs in wording only (for example "был избран" for the
  facsimile's "избран").

Leaf numbers, preserved exactly and unresolved: the exhibit caption gives L. 6-7 and captions the three images
L. 6, L. 6 ob. and L. 7, while the handwritten folio numbers on the images read 5, 6 and 7. The discrepancy is
identical in the 2019 and 2021-2023 captures, so it is in the published exhibit, not in a capture. Neither
numbering is adopted.

## Changed files

- `docs/campaign-certification/C01/research/sources/russia-garf-cec-result-19910619-facts.json`: a new
  `source_review` object holds the trigger, the method, every official-host attempt, every download with its
  identity, the capture table, the facsimile identities, six transient archive failures (five HTTP 429 rate
  limits and one failed connection (curl exit 7), all retried) and the content comparison. The provenance note gains one
  sentence ("not checked into this repository"). Snapshot 7,436 bytes (`6a1084e3…f690d3`) to 27,871 bytes
  (`23667eb7080c58e9466442dcd1cbb7ac96a9453e28db603cf2ef8694248d1e02`).
- `docs/campaign-certification/C01/research/russia.json`: the `ru_garf_cec_result_19910619` source record gains a
  `source_review` summary and the new snapshot identity; its claims and scope note are unchanged.
- `docs/campaign-certification/C01/research/ussr-russia-transition-1991-05.md`: dated section "Source review
  (CLAUDE-C01-SOURCE-05)".
- `tools/avatars/test_ussr_russia_transition_c01_05.py`: one new test pinning the reproduced identities, the six
  identical captures, the exact claim text and the leaf-number uncertainty. No existing pinned value changed
  (the response identities did not change), and nothing was loosened.
- `docs/campaign-certification/C01/research-index.json`: regenerated in its own commit (the `russia.json` input
  identity changed; the counts did not).

`codex/campaign-certification` advanced from `a33a8987` to `fbc2a041` after the claim (two S19 construction
commits). Neither touches these files, and this branch was not merged with it.

## Remaining gaps

- No response from the official host itself: `projects.rusarchives.ru` opened no connection from this
  environment on 21 or 27-28 September. A reviewer with access could compare the live page with the recorded
  bytes; until then the Internet Archive capture is the only reproducible identity.
- The archival leaf numbering (caption L. 6-7 against folios 5-7) needs GARF's own file or inventory.
- The TASS publication date of the communication remains unknown (proposed `C01-Russia-TR91-004`).
- The extract keeps the C01-05 `claims` layout; it was not converted to the later rows layout.

## Commits

Claim `d276830e`. The review commit (this record, the extract, `russia.json`, the report and the test) and the
research-index commit follow it on `claude/c01-source-05`; their hashes are returned with the review.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check -- <the six changed paths>
```

All passed on 28 September 2026 UTC (27 September local), from `C:/Users/ridge/Spheres-c01-src05` with
`PYTHONDONTWRITEBYTECODE=1` and game data present: the index regeneration and exact check (9 packets, 1,329
sources, 3,731 claims, counts unchanged); `campaign_census.py --check` (exit 0); 28 USSR tests (one new); 29
Russia tests; 79 research tests; 16 campaign tests, census included, none skipped; 11 atlas Node tests; the
workboard check (44 markers); and `git diff --check` on the changed paths. The new test also failed on each of
ten hand-made regressions (the recorded SHA-256, a single download, the folio numbers, the claim number, a
dropped capture, a post-cutoff capture, an invented live identity, a facsimile SHA-256, the packet summary's
SHA-256, and the leaf note marked resolved).
