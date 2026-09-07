# Historical party leadership and characters, 1990–2026

**Current art direction:** the user confirmed actual rotatable 3D cartoon characters. See [the physical character roadmap](CHARACTER_3D_ROADMAP.md). Four early-1990s likeness studies are integrated; zero raster person avatars are accepted. The raster pipeline below is retained for reference-image provenance and optional future image work, not as the current production route.

The intended result is a government screen in which each party has the appropriate historical people available for the campaign date, and changes in government show those people's own portraits. Elections and other gameplay determine who takes power. The historical record supplies candidates, identity, affiliation and eligibility; it does not force the campaign to reproduce real election winners.

The intended final roster includes small and unrepresented political organizations as well as the game's current parties. This first inventory covers every party currently represented for all 160 stable nation identities: the 137 starting countries and 23 successor identities. A successor country needs its own reviewed party and person links when it becomes available. The period requested is 1990–2026; a source collection's `reference_through` records its actual research cutoff. The remainder of a partly researched year stays an explicit gap.

The 7 September 2026 UK pilot contains 43 authored people and 51 sourced term records across four simulation party records: Conservative (12), Labour (11), Liberal Democrat lineage (13), and the nationalist grouping's distinct SNP/Plaid components (15). All four remain partial, with explicit gaps and uncertain boundaries. The complete inventory contains 624 party records across 148 nation identities; all 624 still carry a history gap and none claims verified global-period coverage. The twelve other nation identities currently have no party record in the simulation input, which is an institutional inventory fact rather than a claim that their real societies had no political organizations. The registry now holds 284 identities and 132 explicit original-executive links, transcribed from existing sourced office records. Those imported identities do not establish party leadership or membership. This remains a partial historical dataset.

Accepted character counts come from `person_art_pipeline.py coverage` against the actual manifest. Source photographs, generated facial studies, pending jobs and unreviewed illustrations are not completed avatars. The pilot's short visual-era approvals also do not count as full 1990–2026 appearance coverage.

## Existing assets and the identity boundary

The national avatar catalogue is a separate presentation system. It deliberately chooses a figure from a nation's history. Lincoln for the United States, Attlee for the United Kingdom, de Gaulle for France and Faisal I for Iraq are national avatars, not the corresponding 1990 incumbents. Do not use `figureFor(nation)` to obtain the face of a party leader or officeholder.

The existing checked inventory contains 160 generated national avatars and 143 archival portrait sources. Its 303 display derivatives occupy 7,749,894 bytes, versus 322,059,267 bytes of retained source art. The source audit passes; every generated Rust alias matches its display manifest. These checks establish file/manifest consistency, not a fresh visual or historical review of every face.

Some identities can be reused after an explicit person link and visual-era review: the catalogues already contain Gorbachev, Havel, Marković, Pindling, Kaunda, Zayed and Lini, for example. Hussein and George Price also require their authored name aliases to be reconciled. Other existing subjects may be relevant to opposition or later terms. Being alive in 1990 or sharing a country is insufficient evidence of party leadership. Reuse the exact reviewed person's image bytes and provenance; never infer an identity from a nation filename.

## Data architecture

Keep four responsibilities separate:

| Record | What it determines |
| --- | --- |
| Authored person catalogue | Stable person ID, names, aliases, birth/death information and identity sources |
| Party and affiliation terms | Which person held which party role, eligible dates, coalition component and source confidence |
| Saved campaign incumbents | Who actually holds a role in this campaign, when they were seated and why |
| Person portrait manifest | Which reviewed depiction may represent that exact person in a visual era |

`spheres-sim/data/party_leaders.json` is the sourced catalogue. It contains `people` and one record per represented party with `kind`, historical `coverage`, `terms`, explicit `gaps` and sources. Terms reference a person ID rather than copying a name. Collective leadership, coalitions, acting leaders and co-leaders remain explicit roles instead of being collapsed into one invented president.

The existing `spheres-sim/data/leaders_1990.json` supplies the baseline executive records. A party chair, prime minister, president and monarch are distinct offices and may be held by different people. Executive records need authored person IDs too. A caption, fuzzy name match or exact spelling alone does not establish that link.

The new `spheres-web/data/person_portraits.json` is keyed by those person IDs. It stays outside simulation saves. Runtime web assets use the generated exact-file allowlist; user-provided paths are not a file-serving interface. The government API supplies the actual chosen person and their reviewed asset, while the renderer formats that result.

## Dates, eligibility and succession

Use half-open intervals: `from` is included and `until`/`to` is excluded. Preserve whether an input is known to a day, month or year; do not turn “1994” into a claimed 1 January appointment. Unknown and open-ended bounds are separate states. Keep contradictory accounts, possible boundary ranges and a short research note visible to the data audit.

Party office and party membership are separate intervals. A historical minister is not automatically the leader of their party. An election winner, acting party chair, coalition spokesperson and collective governing body also need distinct records. A person changing parties must retain their person ID while gaining a new authored affiliation.

Gameplay-driven succession should follow these rules:

1. Existing gameplay resolves the party or institution taking office.
2. The historical catalogue determines the sourced eligible candidates for that party and campaign date.
3. The campaign seats an exact person ID using a deterministic policy; it does not consume an extra random draw simply to select a portrait.
4. The saved incumbent remains the incumbent until an actual succession event changes that record. A catalogue update, month tick or image refresh must not silently replace a counterfactual election winner.
5. Death, retirement, term limits, acting appointments, party switches and institutional succession use explicit supported rules and records. Missing research produces a labelled unknown/collective officeholder rather than borrowing another person's identity.

The simulation's current historical seed sometimes keeps an office description after succession. The rollout must inspect every original, emergent, electoral, regime and death path so that a saved person binding cannot outlive an incompatible office or retain a deceased original by accident. An old save without this feature remains valid and unchanged. An enabled save with an unknown person ID, inconsistent role binding or unsupported catalogue version must not be silently repaired by choosing the newest candidate.

## Raster reference manifest contract (retained tooling)

The tooling is `tools/avatars/person_art_pipeline.py`. It reads authored data and validates or queues work. It never fetches, generates, edits, approves or substitutes an image.

The top-level shape is:

```json
{
  "version": 1,
  "people": {
    "authored-person-id": {
      "name": "Exact authored name",
      "identity_sources": ["https://example.org/person-record"],
      "portraits": []
    }
  }
}
```

An empty `portraits` list means no image is ready. The tool accepts a known-person registry as either the simulation's `people` list or a keyed `people` object. A supplied portrait name must match that known person's name, native name or explicitly authored alias. It never creates a person ID from a name.

Each completed portrait requires:

| Field | Requirement |
| --- | --- |
| `from`, `to` | Exact reviewed visual-era dates; exclusive `to`, or explicit `null` for no visual end |
| `asset`, `sha256` | Real safe repository-relative image path and full lowercase SHA-256 |
| `method` | `archival` or `generated` |
| `source_url`, `credit` | Traceable source and a nonempty credit |
| `identity_source` | Exact `person_id`, source URL and explicit `observed_portrait` or `authored_identity` kind |
| `review` | `identity`, `likeness`, `era`, `visual` explicitly true, plus `reviewer` and exact `reviewed_at` date |
| `crop` | Optional normalized `x`, `y`, `width`, `height`, entirely inside the image |
| `width`, `height` | Optional declared dimensions; when present they must match the physical image |
| `background_mode` | For the final cartoon, record `transparent`; this declaration requires actual fully transparent and visible alpha pixels |

Files may live in `spheres-web/ui/person-portraits/`, or reuse the existing `portraits/`, `leader-art/` and `display-art/` folders. Paths escaping those directories are refused, including resolved symlink escapes. The same physical file or exact image hash cannot be attributed to two distinct people. Overlapping portrait eras are refused so that selection remains unambiguous. There is no nearest-person, nation, or nearest-era fallback.

An archival image also requires `creator`, an approved `license`, `license_url` and `rights_statement`. Its observed identity reference carries the original file/hash, credit and source license. A crop or optimized derivative retains that reference.

A generated image additionally requires:

- `generator: "OpenAI built-in image_gen"`;
- `license: "generated"`, distinguishing generated output from the source's rights record;
- an exact `generated_at` date;
- `generation_record`, identifying the actual built-in output or call record;
- a repository-relative `prompt_record` file containing the exact submitted prompt; and
- either an observed likeness with its actual file/hash, credit and source rights, or an explicit authored-identity reference tied to this person's identity sources.

These records preserve evidence; they do not automatically certify likeness or legal reuse. The reviewer must actually inspect the image and source. Prompt queues never set review flags or mark their jobs as rendered. In particular, a text-led image must not be labelled as based on an observed photograph.

## Source collection and rights

Start from the authored person's Wikidata ID, and use its P18 statements to discover candidate Commons files. P18 is a representative image property; it is not proof of a specific year or a head-and-shoulders photograph. Inspect the exact candidate and its era before selecting it. [Wikidata P18 documentation](https://www.wikidata.org/wiki/Property:P18).

Fetch the exact Commons file's metadata through `imageinfo`: source URL, MIME type, dimensions, file hash, file revision and `extmetadata`. Preserve the creator, credit or custom attribution, license identifier/URL, rights basis, depiction date and retrieval date. Photograph date and upload date are different facts. Metadata may contain formatted or ambiguous text, so automated discovery should produce candidates for review. [CommonsMetadata documentation](https://www.mediawiki.org/wiki/Extension:CommonsMetadata), [Imageinfo API](https://www.mediawiki.org/wiki/API:Imageinfo).

Wikidata's structured records are CC0. That does not change the license of a linked photograph. Retain a per-image credit and license record, including any applicable source/derivative terms, and provide accessible in-game credits. The local tool accepts the project's explicit public-domain, CC0, CC BY and CC BY-SA identifiers; unknown, noncommercial and no-derivatives records remain unaccepted. [Wikidata data access](https://www.wikidata.org/wiki/Wikidata:Data_access), [Commons reuse guidance](https://commons.wikimedia.org/wiki/Commons:Reusing_content_outside_Wikimedia).

Use a cached, resumable collection job with a contact-bearing User-Agent, conservative serial requests, `maxlag`, and `Retry-After` handling. Request expensive file metadata in small batches and use the thumbnail URL returned by the API. Returned dimensions may exceed the requested thumbnail width, so check the actual file. Do not hotlink remote artwork at runtime. [API etiquette](https://www.mediawiki.org/wiki/API:Etiquette), [current Wikimedia API limits](https://www.mediawiki.org/wiki/Wikimedia_APIs/Rate_limits).

The existing national-avatar fetcher should not be mistaken for this pipeline: despite its old P18 description, `wikipedia_candidates` uses English Wikipedia `pageimages`. It also rejects repeated QIDs across nations and regenerates its Rust lookup solely from `nation_figures.json`. Those behaviors do not suit a person shared across party or country records. Its explicit license checks, file hashes, retained provenance and bounded display conversion are reusable components after their inputs are separated.

## Visual style and size

Before confirming actual rotatable 3D characters, the user selected small full-body cartoon character avatars and rejected the first realistic full-body attempts. The following describes the superseded raster study direction, not the current physical-model requirement. The actual USA selector character (`spheres-web/ui/leader-art/USA-leader-088ed05335f8.png`) is the style anchor: clean outlines, simplified animated facial shapes, restrained shading, about 5.5-head adult proportions and true transparent alpha. Show the complete figure including feet. A flat or checkerboard background is not transparency, and a realistic painterly miniature does not satisfy this direction. Existing national art supplies style and composition only, never the person's identity or costume. A photographic or generated headshot is an identity study rather than the final avatar. Preserve rejected v1 files and prompts as provenance rather than treating them as approved final characters. Review age, hairstyle, clothing and office-era context against the observed source. Keep source limitations explicit; avoid manufactured insignia or generic national costumes.

A person can retain one suitable portrait over several years. Add another era version when there is a meaningful, reviewed appearance change; there is no requirement to render a new picture for every calendar year. Party or office changes alone do not change a person's face.

Proposed delivery sizes are a 256-by-320 thumbnail and a 512-by-640 detail image, bounded while preserving aspect ratio and a reviewed crop. Keep source images and exact provenance; ship compressed, content-addressed display assets and load only visible rows. Measure actual byte and decode cost before setting a release budget.

For scale, existing 512-by-640-bounded archival derivatives average 36,233 bytes, and generated avatar derivatives average 16,053 bytes. At the current archival average, 1,000 portraits would be about 36 MB of compressed display data, before any additional variants; this is an estimate, not an achieved budget. Avoid embedding thousands of full-resolution generated PNGs or eagerly decoding an entire global roster.

## Tool commands and module API

From the repository root, with Python 3 and Pillow available:

```powershell
py -3 tools/avatars/person_art_pipeline.py self-test
py -3 tools/avatars/person_art_pipeline.py validate
py -3 tools/avatars/person_art_pipeline.py coverage --from 1990-01-01 --to 2027-01-01
py -3 tools/avatars/person_art_pipeline.py jobs --out work/person-jobs-v3.json --prompts-dir work/person-prompts-v3
```

`--manifest`, `--registry` and `--repo` select explicit inputs. A missing manifest may be used for coverage/jobs and reports zero ready portraits; validation itself refuses a missing manifest. `--executives spheres-sim/data/leaders_1990.json` adds executive linkage coverage. Records without `person_id` remain unresolved unless an authored `--executive-map` supplies exact `{nation, office, name, person_id}` entries. Repeated names are never resolved automatically.

Job IDs include a digest of the authored person, requested era, sources and job version. The retained `person-character-v3` raster prompt implements the earlier cartoon/transparent-USA study style, so earlier headshot and realistic full-body jobs are not silently reused. Jobs and prompts are new versioned documents; the writer refuses to overwrite different existing content. This preserves an earlier submitted prompt and prevents silent provenance edits. Output reports distinguish ready, partial, missing, pending and missing identity sources. Party history gaps and date uncertainty remain separate from artwork coverage. CLI output is UTF-8 so authored native names remain intact when redirected on Windows.

Importable helpers are `people_from_registry`, `executive_links`, `validate_manifest`, `select_portrait`, `coverage_report`, `build_jobs` and `prompt_for_job`. `select_portrait` returns only a valid exact-person depiction covering the requested day. Invalid manifests do not authorize a partial asset set. The module does not modify a campaign or serve arbitrary asset paths.

The 16 focused self-tests cover real source-file/hash verification, unknown or wrongly named people, missing images, path escapes, cross-person image reuse, era overlap, crop bounds, actual transparent alpha, explicit review, licenses, built-in provenance and prompt records, partial coverage, deterministic unrendered jobs, executive identity gaps and uncertain dates. The tests read existing artwork; they create no new portraits.

## Rollout and acceptance

| Stage | Deliverable | Acceptance condition |
| --- | --- | --- |
| 1. Inventory and schema | All represented nations/parties, baseline executives, stable people and explicit research gaps | Every game party maps to a record; collective and unknown leadership are represented honestly |
| 2. Candidate foundation | Sourced historical roles, affiliations and uncertain date bounds | Exact links validate; unsupported dates do not acquire fabricated people |
| 3. Campaign succession | Saved person bindings integrated with elections, appointments, death and other actual events | Deterministic replay, old-save preservation, no extra opposition-party mortality rolls or forced historical election results |
| 4. Portrait pilot | A small representative set of actual people, eras and source methods | Likeness/style/source review complete, manifest valid, correct portrait changes with the real incumbent |
| 5. Regional research batches | Expand country/party coverage across 1990–2026, including successor identities | Per-party gaps decrease with dated sources; progress reports count verified people and physical images separately |
| 6. Government presentation | Party leader cards, head-of-state/head-of-government separation, candidate details and credits | Foreign views remain read-only; missing data has a useful labelled fallback; narrow layouts and keyboard use pass |
| 7. Complete coverage and release | Every represented party audited; all eligible people supplied or explicitly recorded as gaps | Identity, source, image, date, save and browser checks pass; no unreviewed asset is described as complete |

Implementation files span the sourced simulation catalogue and government succession code; the person portrait registry and generated web asset lookup; `main.rs`/`government_view.rs` read models; `government-ui.js`/`.css`; the offline person-art tools; and focused simulation, API, renderer and browser checks. Existing national-avatar records and their menu/dossier uses remain a separate catalogue. Expanding the political inventory beyond the present simulation rows (including later-founded and minor parties) is still required; the624-row inventory is not a census of every real-world political organization.

This roadmap and the person-art tool are the current deliverables of the tooling work. Research queues and schema placeholders are not completed historical coverage. Hundreds of new portraits, completed global party histories and a finished succession rollout must only be reported after those artifacts exist and their respective checks pass.

The initial likeness audit retained two Elis-Thomas source files in `spheres-web/ui/person-portraits/references/`, with exact hashes, attribution and era qualifications in `source-review-uk-v1.json`. The official Senedd first-Assembly image is a later reference and the NLW image an earlier reference; neither is relabelled a 1990 photograph. Gordon Wilson's currently discoverable Commons crop has conflicting person/date metadata, while the correctly captioned Scottish Political Archive images are not freely licensed. These unresolved source candidates remain outside the accepted avatar manifest. [Senedd source photograph](https://www.flickr.com/photos/seneddcymru/53965497458/), [Scottish Political Archive November 1987 record](https://www.flickr.com/photos/scottishpoliticalarchive/5071297148/).
