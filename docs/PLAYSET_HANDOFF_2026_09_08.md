# Character, guidance and armored art playset

Publication branch: `codex/resume-spheres`. This checkpoint collects the recent
character, government, tutorial and military art work in this playset. The
armored workshop now also saves each vehicle family's visual draft in the
browser, including components, finish, condition and detail level.

## Included work

- Rebuilt configurable tanks and five supporting armored families: IFV, APC,
  reconnaissance, self-propelled artillery and mobile air defense. The separate
  mechanized-infantry arsenal formation also has a matching geometry pass.
- Authored material classes, camouflage, condition treatment and contact
  shading, plus locally served licensed surface maps. Canonical GLBs retain
  their exact specification rebuild contract. The licensed Strv 103 remains an
  optional historical art reference, with attribution and conversion receipts.
- Historical party research, fixed cartoon portraits, explicit coverage gaps,
  separately identified fictional successors through 2035, dated avatar
  selection and narrowly validated save upgrades. Worldwide history and
  artwork coverage remain unfinished; see the
  [cartoon roadmap](CARTOON_CHARACTER_ROADMAP.md).
- Nine tutorial lessons, a field guide, contextual advisors and a session-bound
  read-only guidance endpoint. The guidance interface offers explanations and
  links to existing decisions; it never issues orders automatically.

## Workshop draft behavior

Each family keeps its own components, finish, condition and detail level, and
the workshop restores the last selected family. Restored components pass the
same allowlists and weapon/ammunition/sensor pairing used by edits before the
first model is built. Invalid selections return to compatible defaults.

Storage failures leave the visual draft usable for the current visit. Opening
the workshop never overwrites an existing record; an unsupported schema remains
untouched. The existing Restore vehicle preset action resets only the current
family's components, retaining its appearance and other families' drafts.
These preferences belong to this browser origin, not campaign saves or
government procurement. If multiple workshop tabs edit at once, the last
successful save wins; there is no cross-tab merging of simultaneous edits.

## Git integration boundary

On 8 September 2026, `origin` was fetched before publication. This branch
contains Claude's `feat/hoi4-map-and-tech` model work through `fc0f0c2`, and the
earlier `feat/art-p0` integration through `c2e49c6`. The remote
`codex/resume-spheres` branch is an ancestor, so this publication is a normal
fast-forward push.

`origin/master` and `origin/claude/admiring-euclid-a867c9` were both at
`485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`. Their newer population, industry,
warfare, fiscal and map changes are separate from this checkpoint and have not
been merged here. A future integration must preserve both sets of work and
reconcile shared equipment, government and guidance interfaces; pushing this
branch does not replace the default branch or update a running game.

## Review entry points

Serve the repository root on loopback HTTP and open:

- `tools/arsenal/armored-inspection.html` for the supporting vehicle workshop.
- `tools/arsenal/tank-inspection.html` for tanks and the credited reference.
- `tools/ui/leadership-government-review.html` for exported government views.
- `tools/ui/guidance-review.html` for illustrative tutorial/advisor scenarios.

The static reviews do not modify a campaign. Native routes and artwork have
been built and tested; this publication does not launch a replacement native
game process. Art geometry and shader work do not change simulation statistics.
The models remain fictional configurable game vehicles, and military-unit
mesh placement on the active world map remains outside this work.

## Publication verification

- Full serverless UI batch: **1,305 passed**, no failures or skips. Run with
  `node --test --test-concurrency=2` over the non-browser `check_*.cjs` files;
  two workers avoid the host memory pressure seen in an earlier focused run.
- Native release checks: **44 leadership**, **one determinism**, **one save/load**
  and **274 web** tests passed; three existing web tests remain ignored.
- **49 Python tests** passed across the artwork, production, import and static
  review tools. All ten researched country imports are repeatable no-ops.
- Kaifu's portrait source record was reconciled with three already vetted
  research links. Only the manifest and its two generated metadata hashes
  changed. Eleven importer checks and seven native portrait tests passed after
  that source-only reconciliation.
- Publication checks exposed CRLF-dependent hashes in authored inputs. The
  importer now writes canonical LF JSON, keeps already-compliant no-op bytes
  and timestamps, and retains atomic rollback. Two regressions fail against
  the former writer; all six transaction checks pass. The two byte-hashed Rust
  inventory files now explicitly use LF. All eight active provenance inputs
  match Git's stored bytes; the [normalization receipt](research/publication-line-ending-normalization.json)
  connects historical execution hashes to the canonical files. Active board
  and review artifacts were regenerated, and **104 focused Government/UI
  checks** passed afterward.
- All 12 canonical equipment GLBs regenerate exactly. The asset audit found
  no changed file above 50 MB, accidental saves, caches, executables or secret
  patterns. Third-party attribution and runtime checksums were verified.
- Actual browser reloads restored a customized eight-wheel, 35 mm scout and
  its sensor mast, finish, condition and detail level, then restored the IFV's
  independent appearance and last-selected family. Browser warning/error logs
  were empty. Eighteen focused workshop tests cover persistence and export.
- Final native release build passed. Executable SHA-256:
  `f6d30bb2ff876fb96ac4297adc2a310d66facb9495df2cb9ceee3bda53a0f5ff`.
  The executable remains a local build output, not a committed repository asset.

This is local verification on Windows, not a completed GitHub Actions run or
a playtest of a newly launched native game. Existing campaign saves and the
running native game process were not changed.
