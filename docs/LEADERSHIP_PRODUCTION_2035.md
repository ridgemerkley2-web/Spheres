# Worldwide party characters, 1990–2035

The user approved the current fixed cartoon style on 7 September 2026 and asked
for every party in every country through 2035. Historical people stop at the
research cutoff, **7 September 2026**. The following period uses explicitly
fictional successors informed by research, with gameplay determining succession.
Fictional biographies and future officeholders are never written into historical
term records as facts.

The durable production board is
[`leadership_production_2035.json`](../spheres-web/data/leadership_production_2035.json).
The searchable, read-only art and coverage report is
[`leadership-art-progress.html`](../tools/ui/leadership-art-progress.html).
Serve the repository root with the existing static art preview server to use it.

## Worldwide expansion batch — 7 September 2026

The first worldwide expansion after the UK/Japan pilots adds research for France, Germany, Italy, India, China, Brazil, South Africa, Canada and Australia, plus the separately reviewed US committee-chair starter terms. The registry now contains **590 real people and 393 sourced party-office records**. Fifty-three simulation party rows have partial research; none claims a complete all-party national history. The inventory still covers 160 nation identities and 624 current game party rows.

There are **53 reviewed historical cartoons and four fictional cartoons**, totaling 57 physical 1024×1536 PNG assets. This batch adds 38 historical cartoons to the previous 19-image library. Exact prompts, identity references, correction steps and original filenames are recorded in `tools/avatars/person-prompts/world-*.json`. Historical references are not shipped unless separately licensed. Charles Blunt's researched opening identity still needs a clearer likeness reference; the available 1987 group photo is too small for a satisfactory individual illustration.

The expanded gallery has country/name filters and incremental loading. The [government review](../tools/ui/leadership-government-review.html) reuses the production renderer for 12 countries across a 1990 sample campaign, a 1990 historical reference and a 2030 fictional cast. It is a static source review, not a running or saved player campaign.

The future export contains 2,556 templates across 148 countries with game parties. These are not finished avatars or a complete country-specific cast. The current inventory has **705 known historical artwork jobs remaining**, in addition to all unresearched countries, missing minor parties and future artwork. No country is marked globally complete.

Integration separates party offices from national executive eligibility, retains distinct coalition/components and co-leaders, and gives explicit historical continuation notes for closed or unresolved organizations. Research has not changed saved incumbents. Existing Person/Term facts are preserved; narrow corrections to newly imported, never-launched Canadian terms and lifecycle-only party corrections have dedicated receipts under `docs/research/`.

See [executive-role rules](PARTY_EXECUTIVE_ELIGIBILITY.md), [fictional succession](FICTIONAL_SUCCESSION_2035.md), and [native/source/save validation](research/leadership-save-static-validation.json). The current localhost game remains on the earlier executable; the new source and release build require a separately permitted launch.

## The approved style

The anchor is the full-body
[`margaret-thatcher-cartoon-1990-v3.png`](../spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png):
bold contours, clear facial shapes, expressive features, compact adult cartoon
proportions and a quiet opaque dark teal backdrop. Preserve each person's own
face, age, hair and period clothing. The style approval does not establish
historical accuracy, approve unmade images or license using one person's face for
another. Record the actual generation prompt, source identity, file hash, visual
era and review for every completed image.

The current approved presentation is fixed 2D illustrations. Archived physical
character meshes and national selector figures are separate assets and do not
count toward this programme.

## Coverage is deliberately measurable

The board inventories every stable nation identity directly from the simulation
roster and cross-checks the national selector's identity inventory. It compares
party records with both `POLITIES` and `D4_POLITIES`, including the `pariah`
constructor and small parties. It keeps SNP and Plaid Cymru as separate
components of the current nationalist simulation grouping.

At the initial board build, the game contained 160 nation identities (137 starting
and 23 successor countries), 624 simulation party rows across 148 countries, and
12 countries without simulation party rows. Those twelve remain explicit
institutional research tasks. None is declared to have had no political parties.

The initial historical catalogue contained 284 people and 51 sourced party terms.
Four party rows had partial research, all 624 had declared historical gaps, and
four physical cartoon illustrations were validated. These figures are a dated
baseline: **the JSON and report provide the current reproducible counts** after
new research and art are registered.

The current simulation parties are not an exhaustive real-world census. Every
country therefore retains an open requirement to research small, unrepresented,
dissolved and successor organizations, their legal and historical identities,
coalition membership, collective leadership and temporary officeholders.

## Historical production stages

1. **Country and party census.** Identify organizations throughout the period,
   including times before independence, mergers, bans and dissolution. Keep the
   stable game nation IDs and explain historical jurisdiction changes. Record
   collective institutions and independent factions without inventing parties.
2. **People and roles.** Author stable person IDs, names and dated identity
   sources. Distinguish party leader, chair, president, minister, acting leader,
   co-leader and parliamentary spokesperson. Do not infer party leadership from
   executive office or a nation's selector figure.
3. **Dated terms and life constraints.** Record source precision and conflicting
   boundaries. Retain unknown dates as research tasks. Death, retirement,
   membership and leadership are distinct facts; do not extend one from another.
4. **Appearance references and illustrations.** Create dated cartoon likenesses
   only for relevant windows. Review source identity, age, clothing and the
   physical image. Split long needs into at most five-year production windows;
   a reviewed depiction may cover a shorter period.
5. **Game integration and review.** Bind each reviewed image to the exact person
   ID and half-open appearance era. Verify minority-party views, coalitions,
   succession, counterfactual incumbents, old saves and missing-art presentation.

Known historical art jobs use the verified interior of a sourced term: a start
known only to a month uses that month's latest possible day; an end known only
to a year uses that year's earliest possible day. The raw uncertain term remains
in the report, so this conservative art-planning window is not a claim of an
exact appointment date. An ongoing historical term stops at the source cutoff.
Known birth/death bounds further clip the window.

An original 1990 executive link establishes a need at the **1 January 1990 seed
observation only** when its end is not sourced. It does not authorize a 46-year
portrait window or a fabricated term. Known people without researched eligibility
receive an eligibility research task, not portraits for every campaign year.
Gameplay may later need additional appearance windows; those require an explicit
reviewed extension rather than silently expanding the historical record.

## Fictional successors, 8 September 2026–31 December 2035

Future successors belong in a separate catalogue with an explicit fictional
identity marker, eligibility dates and source-informed background profile. The
source record should explain the real party's ideology, recruitment institutions,
career paths, language and naming conventions. These sources inform plausibility;
they do not verify a fictional person's existence or forecast election results.

Each party and distinct coalition component needs its own candidate pool. The
selection policy may account for age, service, factions and gameplay support.
It must not overwrite a campaign incumbent simply because the calendar reaches
the cutoff, and it must not insert fictional people into missing earlier history.
Portraits for fictional people still need actual generated files, distinct faces,
the approved style and visual QA. Templates, name pools and generated candidate
metadata do not count as artwork.

The Government tab now exposes **Future candidates through 2035** as a read-only
disclosure on each party's campaign card. It preserves coalition-component names,
labels fictional biographies and birth dates, and places institutional research
in a separate disclosure. Pending fictional artwork remains an explicit
placeholder. The historical date browser retains its historical cutoff and
excludes these previews. A fictional person who later becomes a campaign leader
retains the fictional label on both their party card and the executive overview.
When succession is disabled, the future cast remains a preview rather than an
active appointment list.

Fictional images are registered separately in
`spheres-web/data/fictional_portraits.json`. They require an exact catalogue
person ID, name and appearance seed, `status: fictional-character`, an
`authored_fiction` design source and explicit design/visual review. They do not
claim a historical source photograph or likeness approval. The validator rejects
historical portrait-byte reuse and dates outside 8 September 2026–2035. The web
runtime serves them at their explicit future appearance date only inside a
clearly marked future preview; ordinary historical lookups return no fictional
artwork. Run `python tools/avatars/fictional_art_pipeline.py validate` and
`python tools/avatars/build_person_avatar_assets.py --check` after registration.

The board records whether the future profile catalogue exists. When
`spheres-web/data/future_candidates_2035.json` has been exported by the simulation,
it verifies the actual unique fictional IDs, dates and nation/party/component
coverage, then reports the candidate-template count separately from physical
cartoon artwork. If that export is absent, it reports the runtime count as
unaudited. It does not multiply a party count by a template pool size and label
that number completed characters. The templates themselves still require
country-specific editorial review.

## Updating and verifying the board

Run from the repository root:

```powershell
python tools/avatars/leadership_production.py build
python tools/avatars/leadership_production.py check
python tools/avatars/leadership_production.py self-test
```

The tool validates physical cartoon images through the existing person-art
pipeline, then records hashes of its source inputs. `check` fails if the committed
board is stale. `self-test` checks complete nation/party inclusion, separate
coalition components, life and cutoff clipping, uncertain boundaries, relevant
appearance windows and the separation of future fiction from historical facts.
This tool performs no downloads, generation, image editing, approval or game
state mutation.

Country completion requires an independently reviewed exhaustive party census,
sourced relevant people and roles, finished reviewed art for every required era,
and tested historical/future game behavior. Registering a task is progress in
planning; it is never completion of the underlying task.
