# CLAUDE-C01-SOURCE-06: Compare the changed Bush Library response

Owner: Claude. State: **complete — bounded source repair accepted and integrated** (submitted 27 September 2026; not complete; pending Codex acceptance).
Parent: C01 (incomplete).

Assigned by `docs/planning/ai-handoffs/CLAUDE-C01-NEXT.md` and `docs/planning/ai-task-queue.json` (task
`CLAUDE-C01-SOURCE-06`, queued 27 September 2026) as a source-review follow-up to the integrated CLAUDE-C01-06.
It repairs integrated research; it does not claim historical acceptance, which Codex decides.

Branch: `claude/c01-source-06`. Base: `a33a8987` (current `codex/campaign-certification` when claimed; the two
later integration commits, to `fbc2a041`, touch none of these files). Claim commit: `629048f7`. Result commits:
`Review source for CLAUDE-C01-SOURCE-06` (this record, the source record, the extract and the report) and the
separate `Regenerate the C01 research index for CLAUDE-C01-SOURCE-06` (index only); the branch head at
submission is the index commit. The sparse worktree was not widened; Codex's evidence files were read with
`git show`.

## Bounded repair

Compare the Bush Presidential Library's 8 August 1990 address page (recorded 31,889 bytes, sha256 539f1155…a1c3; fetched by Codex as 32,782 bytes, ad596dc9…aa8c) against the recorded extract. Identify dynamic-page changes versus changed factual content, and record a reproducible identity (for example a raw pre-cutoff archive capture) if one exists. Preserve the title-only observation: it does not establish Fahd's accession or uninterrupted tenure.

Keep historical dates unchanged unless the cited evidence supports a correction. Do not convert missing
response identities into fabricated checksums; retain disclosed limitations and propose further bounded review
separately. Never bypass an access control, CAPTCHA or bot block; record a blocked source instead.

## Result

**The changed response is dynamic page furniture; no factual content changed.** The address text, the
consultation paragraph and the editorial note naming King Fahd bin `Abd al-`Aziz Al Sa`ud are byte-identical
in the originally recorded response of 21 September, in the live responses of 27–28 September (UTC) and in a
raw pre-cutoff Internet Archive capture of the Library's former page for the same item. No claim, date or
holder changed. The observation stays title-only: it does not establish Fahd's accession or uninterrupted
tenure.

### Response identities

| Response | Retrieved | Bytes | SHA-256 |
|---|---|---:|---|
| Originally recorded response of `source_url` (kept as `source_response_bytes`/`source_response_sha256`) | 21 Sep 2026, researcher and checker; retained copy re-hashed 27 Sep | 31,889 | `539f115582aea3de304d4557c61bb18169f829616c04d72b50d591c5f91ba1c3` |
| Live response of `source_url`, same as Codex's fetch | 2026-09-27T23:38:07Z; 2026-09-27T23:43:24Z with `?c01src06=20260927T2342`; 2026-09-28T00:09:02Z | 32,782 | `ad596dc95af9b5842d2a64bebc1954196f09f743c9cc9f81dafb6a569033aa8c` |
| Raw capture of the former URL, `https://web.archive.org/web/20211205194503id_/https://bush41library.tamu.edu/archives/public-papers/2147`, with `Accept-Encoding: identity` (uncompressed body; CDX digest `F3TP5KPFN4APJEHNK4F3XBBSUOEWWG76`) | 2026-09-27T23:47:05Z; 2026-09-28T00:18:04Z | 16,655 | `247a050e3c68b78a72c1ee68590d9526683508cee8b65bd2ca92cd6636caa325` |
| Same former URL, capture `20190716005022id_` | 2026-09-27T23:48:59Z | 16,655 | `247a050e3c68b78a72c1ee68590d9526683508cee8b65bd2ca92cd6636caa325` |

The Internet Archive has no capture of `source_url` itself (CDX: no rows for http/https, with and without
`www`, exact and prefix; availability API: none; `id_` request for 20260906000000: 404). On 6 September 2026
the former URL redirected to the Library's home page, so the former capture is recorded as a corroborating
pre-cutoff identity, not as a replacement for the originally recorded response.

### Content identities and locators

| Region | Bytes | SHA-256 | Offset: recorded / live / former capture |
|---|---:|---|---|
| Address span: `<p>In the life of a nation` through the `</p>` closing `<p>Note: ` (**reproducible identity**) | 8,979 | `4bf739376d87f635a7687bf06f792ee84d504b921c9db9b82c85ffa7b055a4bf` | 16,996 / 19,506 / 4,844 |
| Consultation paragraph, paragraph 13 of 21 | 321 | `683d98cf9875f9f3bac53d720249fb0f3fadcf5a9a1f008d30beeaa3506ea5e8` | 22,408 / 24,918 / 10,256 |
| Editorial Note, paragraph 21 of 21 | 370 | `5299dd10cd70f381a602a7cbd82da4195346a2c0ff83b23c7e41dd19ae9a8482` | 25,605 / 28,115 / 13,453 |
| `<article>` element (ID 2147, date 08/08/1990, address) | 9,701 | `a24fc94cc2331063a6f76b46553227a6e3351a71e7b899301464ab332e283bbf` | 16,308 / 18,818 / not in that template |
| `<title>` | 159 | `618e4f715621949af44d16c7467b5e51b90ca1e5a4b302a088ee8cc2bf2d9846` | 3,190 / 3,190 / differs |
| Page-title `<h1>` | 206 | `ea19c5ba3408652fadd9598f8f7871d26f2d9bc3e23c1b41a46454cdfed33f9d` | 15,669 / 18,179 / differs (same title in an `<h3>`) |

Differences between the recorded and live bodies: six hunks, +893 bytes net, all dynamic page furniture from
a site update. Recorded line 12 → live 12, Generator meta Drupal 10 → Drupal 11; 21–22 → 21–22, renamed
stylesheet aggregates; 24 → 24–27, the head's empty script slot now holds the settings JSON (moved unchanged
from the footer), a header script aggregate and `menu-accordion-init.js?tm0po6`; 64 → 67 and 69 → 72,
site-search wrapper and form attributes; 426–427 → 429–432, footer scripts (jQuery 4.0.0, new aggregates,
jQuery Migrate 3.5.2). Neither body contains a session value, form token, nonce or timestamp.

### Files changed

- `docs/campaign-certification/C01/research/saudi-arabia.json`: `sa_bush41_address_19900808` gains a
  `source_review` note; its `snapshot` is now 13,185 bytes, SHA-256 `1b27b30a8837d78b2a6400a021c123e4699ed3467895a275330639fd198c6b1c`. Claims unchanged.
- `docs/campaign-certification/C01/research/sources/saudi-arabia-bush41-address-19900808-facts.json`:
  `provenance_note` updated and `source_review` added (method, every response identity, the hunk
  classification, content comparison, reproducible identity, archive search). `source_response_bytes` and
  `source_response_sha256` keep the originally recorded response; claims unchanged.
- `docs/campaign-certification/C01/research/saudi-executive-chronology-06.md`: dated section "Source review
  (CLAUDE-C01-SOURCE-06)"; the Bush row under "Sources added" points to it.
- `docs/campaign-certification/C01/research-index.json`: regenerated in the separate commit; only the Saudi
  packet's byte count and SHA-256 change.
- This record. No test changed: no pinned value changed (`BUSH_RESPONSE` keeps the original identity, and the
  snapshot checksum is verified against the packet).

## Allowed files and checks

Allowed files:

- this record;
- `docs/campaign-certification/C01/research/saudi-arabia.json` (the Bush Library source record only);
- `docs/campaign-certification/C01/research/sources/saudi-arabia-bush41-address-19900808-facts.json` and any new specifically related `saudi-arabia-*-facts.json` extract;
- the CLAUDE-C01-06 report under `docs/campaign-certification/C01/research/`;
- `tools/avatars/test_saudi_executive_c01_06.py` and any test pinning the changed values;
- `docs/campaign-certification/C01/research-index.json`, regenerated in a separate commit.

Checks, run from `C:/Users/ridge/Spheres-c01-src06` with `PYTHONDONTWRITEBYTECODE=1` and game data present:

```text
python -X utf8 tools/avatars/campaign_research.py            # exit 0; 9 packets, 1,329 sources, 3,731 claims (unchanged)
python -X utf8 tools/avatars/campaign_research.py --check    # exit 0
python -X utf8 tools/avatars/campaign_census.py --check      # exit 0
python -X utf8 -m unittest discover -s tools/avatars -p "test_saudi*.py"      # 8 tests OK
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"  # 79 tests OK
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"   # 16 tests OK, census included, none skipped
node --test tools/ui/check_leadership_research_review.cjs    # 11 pass, 0 fail
python tools/planning/workboard.py --check                   # PASS: 44 markers; 10 bounded tasks
git diff --check -- <this record, saudi-arabia.json, the Bush extract, the report, research-index.json>  # clean
```

## Remaining gaps

- The originally recorded 31,889-byte response can no longer be downloaded. A byte-for-byte re-download of
  `source_url` returns the 32,782-byte template until the site changes again (for example new asset names or a
  new `tm0po6` query string); only the address-span identity is reproducible from the live URL.
- No Internet Archive capture of `source_url` exists. The corroborating capture is of the former URL, and its
  link to the current page rests on content (title, date, item 2147, identical address bytes), not a redirect.
  Whether it may serve as the reproducible identity is Codex's decision.
- The source remains a foreign record read from a web transcription; the GPO print edition is an unretrieved
  lead, and no Saudi 1990 record was sought (proposal `C01-SaudiArabia-EXEC-003` stands).
- The observation is title-only: no accession date, term start or continuity for Fahd is established.
- Historical acceptance of C01-06 and of this repair is Codex's decision; C01 remains open.

Integration review: `63746790`. Current/pre-cutoff original bodies and cited content spans reproduced exactly; obsolete-body template diff remains independently unverified. See [independent evidence](../../campaign-certification/C01/integrations/CLAUDE-C01-SOURCE-06/README.md). C01 remains open.
