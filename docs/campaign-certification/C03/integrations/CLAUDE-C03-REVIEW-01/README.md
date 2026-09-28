# Cartoon review workbench — accepted with integration repairs

**CLAUDE-C03-REVIEW-01 is complete as a bounded preparation task.** Codex reviewed
Claude's submission `c9e4d51f` and integrated its implementation and original
browser evidence in `595a6a85` / `9a89e259`. This accepts the review tooling and
its explicitly limited sample notes. It does not close C03, C06 or S23, approve
new artwork, or change a production portrait or saved campaign.

The standalone [workbench](../../../../../tools/ui/cartoon-review/index.html)
shows country/person/era/date filters, historical/fictional/missing labels,
small-card and full-size comparisons, style references, provenance and separate
visual notes. Missing art remains a labeled placeholder. Photographic identity
references stay in a collapsed section and never substitute for game cartoons.
Serve the repository root over loopback and open `/tools/ui/cartoon-review/`.

## Repairs found during independent review

The first integrated Python run passed 22 tests and failed the repository export
check. The unchanged S10.c JSON evidence had CRLF working bytes while Claude's
export pinned its LF bytes. The actual original file and its source record were
not rewritten. JSON input pins now declare `hash_scope: utf8-lf`: only CRLF pairs
are replaced with LF for hashing. Python and the browser perform the same byte
operation. BOMs, Unicode, escaped newlines, lone carriage returns and all other
bytes remain significant. The browser separately records received raw byte
counts and hashes. Raw-scope verification remains supported and unknown scopes
are rejected. Images still require their exact original byte hashes.

The browser evidence writer also previously converted source bytes through a
Latin-1 string and then hashed UTF-8, which misrepresented non-ASCII source text.
It now records the actual UTF-8 source after the declared CRLF-to-LF conversion.
Claude's original report and screenshots remain unchanged in the preparation
packet; this directory contains the independent results after repair.

## Verification

- **24 Python tests passed**, none skipped, including CRLF/LF stability without
  input or image mutation and rejection of real content changes. The deterministic
  export `--check` passed.
- **15 Node/browser entries passed**, none skipped, on headless Edge
  **146.0.3856.97**. These include the browser journey and seven child checks.
  All ten inputs verified. Filters, roving keyboard focus, comparison sizes,
  altered-image detection, stale-input refusal, 390px/320px layouts and Escape
  focus return passed. All **239 requests** were GETs to one loopback origin,
  with no HTTP or page errors.
- Codex visually inspected the independently captured desktop style comparison,
  French missing-art sheet, 390px detail dialog and 320px contact sheet. Labels,
  focus styling, small-card comparisons and placeholders remain readable with
  no horizontal overflow. These are workbench layout observations, not a new
  likeness approval or a game-renderer qualification.

The current export reports **634 items**: 54 historical cartoons, four fictional
cartoons, 160 country-selector figures, five unregistered files and **411 people
with sourced art windows but no cartoon**. Its integrity findings remain zero
errors, 19 warnings and 647 notices. Warnings/notices include source or rights
record gaps and appearance coverage; zero integrity errors does not mean complete
historical/art coverage. Claude's eight visual-note entries remain proposals and
reference-check requests. Their likeness assertions were not independently
re-approved here.

[manifest.json](manifest.json) pins the reviewed source, export, original failed
run, final logs and new browser screenshots. The original screenshot packet is
retained [under preparation](../../preparation/cartoon-review/browser/result.json).
Image-header dimensions and exact-hash duplicate detection retain their stated
limits. No new images, production bindings, historical identities or gameplay
behavior were installed by this task.
