# CLAUDE-C03-REVIEW-01 — cartoon review workbench

Owner: Claude; reviewer/integrator: Codex. State: **ready_for_review** (submitted 28 September 2026 UTC; not reviewed,
not accepted, not complete). Parent: C03, **preparation only**. Branch: `claude/c03-review-01`. Base: `e41aa18d`;
claim `28b3c577`; integration `846df479` merged at `c1a53196`; implementation `1b029cad`; submission: the commit that
adds the Result below. Touched paths: only the owned paths below. Next: Codex review of the tooling, export and sample.
Follow the [expanded working contract](CLAUDE-EXPANDED-NEXT.md).

## Build

Build a standalone reviewer for existing cartoon files, with country/person/era
filters, side-by-side small-card and full-size views, provenance, identity/appearance
bindings and visible missing/unknown/fictional labels. Use the approved country-selector
cartoon style and existing Tupou cartoon review as references. The user wants cartoons;
do not restart 3D character production or replace cartoons with photographs.

Add a read-only asset check/export listing file existence, dimensions, content hashes,
duplicate images bound to different people, appearance intervals, source/rights gaps
and review decisions. A duplicate is a review finding, not automatic proof of wrong
identity. Review six to eight existing portraits visibly at both sizes and record
specific fixes needed. This packet delivers working tooling and a reviewed sample,
not newly completed art jobs or historical acceptance.

## Owned paths

New `tools/ui/cartoon-review/`, `tools/ui/check_cartoon_review.cjs`,
`tools/avatars/cartoon_review.py`, `tools/avatars/test_cartoon_review.py`,
`docs/campaign-certification/C03/preparation/cartoon-review/`; this handoff.
Read existing manifests, art and `tools/ui/character-studio.html`; no shared UI,
manifest or image edits. Load repository assets locally without third-party runtime services.

## Acceptance

Test missing files, invalid intervals, duplicate bindings and unknown identities.
Demonstrate filtering, keyboard navigation and a narrow layout in the real browser;
retain screenshots and input hashes. Clearly distinguish automated findings from
actual visual approval. Export without changing production assets or a saved campaign.

## Result

Submitted `ready_for_review` on 28 September 2026 UTC. Touched paths (all new except this record):

- `tools/ui/cartoon-review/index.html`, `cartoon-review.css`, `cartoon-review.js`: the standalone reviewer;
- `tools/ui/check_cartoon_review.cjs`: 6 serverless tests plus a 7-step real-browser journey;
- `tools/avatars/cartoon_review.py` (read-only export, `--check`) and `tools/avatars/test_cartoon_review.py` (23 tests);
- `docs/campaign-certification/C03/preparation/cartoon-review/`: `cartoon-review.json` and `cartoon-review.md`
  (generated), `visual-review-sample.json` (authored), and `browser/` (six JPEG screenshots, 384 KB with
  `result.json`);
- this record.

No manifest, image, shared UI, production data, research index, campaign shell, save schema, CI workflow, task queue or
pathway changed. No artwork was created, edited, approved, accepted or replaced; no photograph stands in for a cartoon.

**Reviewer.** Serve the repository root on a local address (for example `python -m http.server 8790 --bind 127.0.0.1`)
and open `/tools/ui/cartoon-review/`. Before showing anything the page fetches all ten inputs pinned by the export and
compares bytes and SHA-256; a changed input fails closed with an alert. The contact sheet shows all 634 items at the
game's 106×152 government-card size with visible labels (*Historical person*, *Fictional · not a real person*,
*Country selector*, *Artwork missing*, *Unregistered file*, *Unknown identity*, *Country unknown*, *Duplicate image*,
*Interval issue*, *Coverage gap*, *Rights gap*, *Integrity error*, *Visual notes · not approval*). Filters: person, ID or
country search (diacritics ignored), country, era (decades, 2020 to the historical cutoff, the fictional period,
undated), an exact date using the half-open intervals, collection, and label or finding; filters and the selected
item persist in the URL. Keyboard: one card is in the tab order (roving tabindex); arrows, Home, End, Page Up and Page
Down move; Enter or Space opens; `[` and `]` step; Escape closes the narrow-screen dialog and returns focus.

The detail shows each cartoon side by side at three game sizes (person cartoons: 106×152, 90×132 and 156×218;
selector figures: 143×174, 116×145 and 174×218 using the display512 derivative the selector serves) next to the full
image, whose bytes are fetched and hashed before display. It can add the three style references at card size: the
person-cartoon anchor Thatcher v3 (with the user's 7 September style approval and its scope), the country-selector
reference USA Lincoln (cited by 159 of 160 selector prompt records), and the S10.c Tupou IV cartoon. Separate panels
hold appearance and identity (interval, sourced art windows, uncovered windows, life dates), automated findings, this
packet's visual notes, the review recorded in the manifest, and provenance and rights (file facts, prompt record with
reference note, licences, and the manifest record resolved from the verified manifest). Reference photographs appear
only in a collapsed panel marked "not game art; never shown as an avatar" and load only when opened. Missing art shows
dashed placeholders, never a substitute image.

**Export** (`cartoon-review.json` 770,915 bytes; `cartoon-review.md`). It pins ten inputs by path, bytes and SHA-256:
`person_portraits.json` `1a73390a…`, `fictional_portraits.json` `54a16634…`, `nation_figures.json` `f2fbd1a1…`,
`display-art/manifest.json` `afa2c9bb…`, `leadership_production_2035.json` `f98cbae9…`, `future_candidates_2035.json`
`75afd1f5…`, `spheres-sim/data/party_leaders.json` `18774ffa…`, `references/source-review-uk-v1.json` `16baa8bb…`,
`S10/c/manifest.json` `5fd649de…` and `visual-review-sample.json` `b5ad58c1…` (full hashes are in the export). It
covers 54 historical, 4 fictional and 160 selector cartoons, 5 unregistered cartoon-root files and 411 known people
with sourced art windows but no cartoon, plus 677 tracked image files, all present, with header dimensions and
SHA-256. Findings: **0 errors**. **19 warnings**: 17 selector figures whose identity photograph is CC BY-SA but whose
character art records no derivative licence (the Tupou IV record sets the precedent), and 2 cartoons (Kinnock, Ashdown)
whose prompt record names an earlier generated study as the identity reference. **647 notices**: 411 missing art, 143
identity photographs recorded as automated title matches, 49 historical references not shipped and without a recorded
licence, 18 coverage gaps, 17 text-led selector interpretations, 5 unbound files (four earlier Thatcher studies or
source images and one Kinnock character study), 3 appearance intervals that continue after the recorded death (Rajiv Gandhi,
Oliver Tambo, Zephania Mothopeng), and 1 cartoon with no country binding (Luiz Gushiken). No exact duplicate image is
bound to different identities. The manifests record 54 historical reviews with identity, likeness, era and visual all
true, 4 fictional design and visual reviews, and 160 free-text selector reviews. The export reproduces these reviews; it
does not re-decide them.

**Visual review sample** (`visual-review-sample.json`, 8 portraits). Each master was viewed at full size, at 106×152,
90×132 and 156×218 (scratch renderings, not committed), and in native-resolution face and hand crops. Margins,
backgrounds and fabric contrast were measured from pixels. Only the three references retained in the repository (Tupou
IV, Kaifu, Doi) were compared for likeness; for the others, the drawing was compared with its recorded prompt. The
decisions are only `fixes_proposed` or `reference_check_required`; the tool refuses any approval value, and each entry is
bound to its file hash.

| Portrait | Decision | Specific fixes (abridged) |
|---|---|---|
| Taufa'ahau Tupou IV (Tonga) | fixes proposed | Sleeker grey hair brushed back from the receding hairline of the 1985 reference; heavier upper lids; flat cel shading; slightly larger head. |
| Toshiki Kaifu (Japan) | fixes proposed | Longer face and a higher, swept-back hairline, as in the 1989 reference; grey-blue suit; optional Diet badge. |
| Takako Doi (Japan) | reference check required | The only reference is a low-resolution profile; reduce hair volume; plain or branch-shaped brooch and pearl stud; margins of 64–70 px; separate older variant for the 1996–2003 gap. |
| Jacques Chirac (France) | reference check required | Confirm the prompted tweed jacket, black tie and odd trousers against the unretained 1990 reference; lighter line work; record the reference licence. |
| Rajiv Gandhi (India) | fixes proposed | End the interval by 1991-05-22 (recorded death 1991-05-21); bold contour on the white garments; record the rights basis of the agency reference. |
| Leonel Brizola (Brazil) | fixes proposed | Use the prompted neutral stance instead of the raised-arms rally pose copied from the reference; face toward the viewer; anchor proportions; at least 64 px of top margin; later variant for the 1995–2004 gap. |
| Oliver Tambo (South Africa) | fixes proposed | Prompted large tinted aviator glasses (the drawing has clear rectangular lenses); relaxed pose or a documented raised fist; fix the left hand's anatomy; end the interval by 1993-04-25. |
| Nakano Emi (fictional, Japan) | fixes proposed | The teal suit matches the background (1.00–1.08:1), so change it and its prompt; check resemblance to real politicians before use. |

Across the sample, faces are about 18 px wide at 106×152, so on small cards identity rests on hair, costume and
silhouette. A reviewed head-and-shoulders card crop would need a UI change: the portrait schema already accepts a
normalized crop. Face shading drifts from the anchor's flat cel style, and figure scale varies with margins and pose.

**Checks** (sparse worktree, Windows; `PYTHONDONTWRITEBYTECODE=1`):

- `python -X utf8 tools/avatars/cartoon_review.py`: writes the export. `--check`: exit 0, before and after merging
  `846df479` (no pinned input or art root changed), and regeneration is byte-identical.
- `python -X utf8 -m unittest discover -s tools/avatars -p "test_cartoon_review.py"`: 23 pass. The fixtures cover
  missing files, hash, dimension and unsafe paths, empty, absent and overlapping intervals and period bounds,
  coverage gaps and death clipping, duplicates for different people as warnings rather than errors, same-identity
  duplicates, unknown or mismatched historical, fictional and selector identities, unbound files, rights gaps, prompt
  records (without local paths, LF-normalized), the approval guard, deterministic regeneration, stale `--check`
  (exit 1), CRLF checkouts, and unchanged inputs. Repository tests confirm the export is current and read-only, and
  that header dimensions match Pillow 11.3.0 for PNG, JPEG, WebP VP8 and WebP VP8X.
- `node --test tools/ui/check_cartoon_review.cjs`: 6 pass and 1 skipped. The browser journey skips when
  `SPHERES_BROWSER_CHANNEL` is unset, so `run-unit.cjs` and the CI JavaScript job stay browser-free.
- `SPHERES_BROWSER_CHANNEL=chrome NODE_PATH=C:/Users/ridge/spheres-war-overhaul/tools/ui/node_modules
  CARTOON_REVIEW_EVIDENCE_DIR=docs/campaign-certification/C03/preparation/cartoon-review/browser node --test
  tools/ui/check_cartoon_review.cjs`: 14 pass (6 tests, plus the journey and its 7 steps), at `1b029cad`, in headless
  Chrome 154.0.8037.57 with Playwright 1.58.2. An in-process server listened on 127.0.0.1 at an OS-assigned port and
  closed at the end. The journey checked:
  - the in-browser SHA-256 of all ten inputs;
  - filters: Tonga 2; Tonga on 1990-06-01, 1 (Tupou IV only); fictional 4; French missing art 36, with no images;
    `salote` 1; 2000s 106; unknown identity 5;
  - arrows, Home, End, Enter, `[` and `]`, a single tab stop and a 3 px focus ring;
  - card boxes of exactly 106×152, 90×132 and 156×218, and the verified 1024×1536 full image;
  - a swapped image flagged as "Bytes differ" and a changed input failing closed;
  - 390 and 320 px with no horizontal overflow, a dialog detail, and Escape returning focus;
  - 231 requests, all GET and all to the loopback origin, with no HTTP or page errors.
- Mutation checks on scratch copies, not committed: six export guards (duplicate finding, approval guard, empty
  interval, missing file, unknown identity, CRLF normalization) and four reviewer faults (stale export shown, ArrowDown
  by one, overlay not a dialog, full-size bytes not compared). Each turned the intended test red.
- `python -X utf8 tools/planning/workboard.py --check`: PASS (44 markers, 17 tasks). `git diff --check`: clean.

**Screenshots** (`browser/`, JPEG quality 70):

- `desktop-tonga-filter-side-by-side.jpg`: Tonga filter with Tupou IV at three sizes beside the verified full image.
- `desktop-tupou-side-by-side-with-style-references.jpg`: the same view with the style references strip.
- `desktop-missing-art-france.jpg`: 36 French people with visible *Artwork missing* placeholders and no substitute image.
- `narrow-390-sheet.jpg` and `narrow-320-sheet.jpg`: Japanese cartoons on narrow screens.
- `narrow-390-detail-dialog.jpg`: the full-screen detail dialog.

`browser/result.json` records the revision, the browser version, the input hashes as verified in the browser, the
reviewer source hashes (LF-normalized), each check's readings and each screenshot's size and hash.

**Provenance and licences.** The export and reviewer read only checked-in files. Nothing was downloaded, and no
third-party runtime service is used. The screenshots show repository cartoons only; no reference photograph appears in
them. Rights findings flag what is missing from the records for review; they are not legal conclusions.

**Limitations.** Dimensions come from file headers (pixels are not decoded), and duplicates are exact-hash only. The
export must be regenerated whenever a pinned input or art root changes; `--check`, the repository test and the reviewer
then report it stale by design. Countries and art windows come from the production inventory, whose recorded input
hashes are checked. The selector sizes approximate `index.html`'s CSS, and the reviewer is a standalone page, not the
game renderer. This worktree added `spheres-sim/data`, `spheres-web/ui/display-art`, `spheres-web/ui/leader-art` (305
MB) and `tools/planning` to its sparse set; a full checkout needs nothing extra. Fictional resemblance to real people
was not checked. C03 art batches, historical acceptance and any art job remain open.
