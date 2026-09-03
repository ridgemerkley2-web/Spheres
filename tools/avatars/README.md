# Nation historical figures and selector art

The campaign selector is presentation, not simulation state. Its identity key is
the stable `NationId` code in `spheres-sim/src/nations.rs`; names, ISO codes,
territory polygons and array positions are not substitutes. This matters for
the Soviet Union and Yugoslavia and their successors, for deliberately combined
1990 entities such as Germany and Yemen, and for renamed states such as Zaire,
Western Samoa and Swaziland.

`spheres-web/data/nation_figures.json` must contain exactly one entry for every
roster row: all 137 nations seated in January 1990 and all 23 successor identities.
The latter must be ready before a union dissolves or a latent state appears.

## Historical-leader policy

Each entry names one deceased historical leader connected to that exact national
identity. Leadership includes civic, liberation, reform, resistance and
independence leadership; it is not limited to heads of state. The selector treats
the person as an inviting historical doorway, not as a literal embodiment of
every citizen and not as an endorsement of the person's full record. Avoid
genocidal, extremist and totalitarian avatars, and qualify contested legacies.

Selections should be:

- historically and geographically defensible for the state represented by the
  `NationId`;
- distinct across nations unless every reused entry carries a nonempty
  `shared_figure` note explaining why reuse is intentional;
- described with a plain role and a concise rationale;
- explicit about contested, cross-border, dynastic, colonial, Indigenous or
  federation-wide associations in `review_note`; and
- reviewed as an individual choice, especially for composite and successor
  states. An ISO-to-person algorithm is not acceptable.

The manifest's display name, `start_1990` value and region are snapshots. The
checker compares all three to the authoritative Rust row so a roster change
cannot silently leave the selector describing a different country.

## Manifest contract

The top level requires `version`, `label`, `policy`, `reference_date` and a
`nations` object keyed by exact `NationId`. Every nation entry requires:

| Field | Meaning |
| --- | --- |
| `display_name` | Exact `NationRow.name` snapshot. |
| `start_1990` | Exact boolean `NationRow.start_1990` snapshot. |
| `region` | Exact `NationRow.region` snapshot. |
| `figure` | Player-facing name. |
| `canonical_lookup` | Stable research name used for duplicate detection. |
| `years` | Player-facing lifespan, including `c.` or BCE notation where needed. |
| `born`, `died` | Numeric research years or `null` when genuinely uncertain. |
| `role` | Short cultural or civic role. |
| `rationale` | Why this figure fits this exact national identity. |
| `confidence` | `high`, `medium` or `low`. |
| `review_note` | Nonempty qualification or `null`. |
| `portrait` | Verified local artwork metadata, or `null` for the safe fallback. |
| `leader_art` | Optional reviewed character-art metadata tied to `portrait.asset`, or to the exact approved Wikidata identity when no freely licensed portrait exists. |

`shared_figure` is optional. If two entries have the same normalized
`canonical_lookup`, every use must supply a nonempty explanation in this field.

## Freely licensed portrait policy

The game is a self-contained executable. Runtime images are local; a source URL
documents provenance and is never used as an `<img>` URL. A portrait may be
added only after the rights of the particular photograph, painting or scan have
been verified. A historical person being dead does **not** put every depiction
of them in the public domain, and a Wikimedia Commons file page is a repository
record rather than a license by itself.

Accepted sources must carry an explicit public-domain statement, CC0, CC BY,
CC BY-SA, MIT or another free-culture license deliberately added to the checker's
reviewed allow-list. Non-commercial (`NC`), no-derivatives (`ND`), fair-use,
permission-only and unknown-rights files are refused. Record the creator, source
page, work title, exact license identifier and URL, a rights statement explaining
the public-domain/license basis, the required credit and the local file hash.
Jurisdiction, publication date and author death date belong in the rights
statement when they are the basis for public-domain status.

A non-null `portrait` object has this contract:

```json
{
  "asset": "USA-a19e3f.webp",
  "source_url": "https://archive.example/item/123",
  "source_title": "Portrait title",
  "creator": "Photographer or artist",
  "license": "PDM-1.0",
  "license_url": "https://creativecommons.org/publicdomain/mark/1.0/",
  "rights_statement": "Why this particular work is public domain or freely licensed.",
  "credit": "Credit shown in the game's source record.",
  "sha256": "64 hexadecimal digits",
  "focus_x": 0.5,
  "focus_y": 0.35
}
```

The asset name is traversal-proof and content-addressed:
`<exact-NationId>-<6-to-16-hex>.webp`. The file lives at
`spheres-web/ui/portraits/<asset>` and is served at
`/art/portraits/<asset>`. `focus_x` and `focus_y` are normalized crop focal
coordinates from 0 through 1. The hex suffix must be the leading 6 to 16 digits
of the full recorded SHA-256. The checker confirms the file exists, has a WebP
header and matches that full hash.

## Full-body character art

The arcade selector may place a reviewed cartoon rendering in front of the
archival portrait. This layer is optional: a verified portrait is the preferred
identity and provenance anchor. When no suitable freely licensed depiction is
available, the record instead names the exact approved Wikidata identity and
documents a text-led historical interpretation. The UI falls back to the
archival portrait or named cameo whenever character art is absent.

Character files live in `spheres-web/ui/leader-art/` and use the content-addressed
name `<NationId>-leader-<12-hex>.png`. A `leader_art` record contains the asset
and full SHA-256, either the exact `identity_source_asset` or the exact
`identity_source_wikidata`, a versioned `style`, the generation date and
generator, player-facing credit, a repository-relative `prompt_record`, a
`background_mode`, and a review note. Prompt records live under
`tools/avatars/prompts/` so later rerenders can reproduce the same intent.

The production file must be either a true RGBA cutout with transparent pixels
(`background_mode: "transparent"`) or a reviewed opaque character card with a
smooth pale lavender/cream studio background (`background_mode: "soft-pastel"`).
A checkerboard painted into an opaque image is never acceptable.
`fetch_commons_portraits.py` includes both verified portraits and optional leader
art in the executable's generated asset table. The browser uses character art
first, while retaining the archival record for identity checking and source
credit.

## Fallback behavior

`"portrait": null` is valid and intentional. It tells the UI to show the named
historical figure with the soft archival cameo and initials instead of displaying
unverified, broken or remotely hosted artwork. Missing metadata never triggers a
network request and never borrows another country's portrait.

Flags do not have the same optional fallback: the local SVG sprite must contain
one `flag-<NationId>` symbol for every roster entry. The generator prefers an
exact-era Commons asset from `historical_flags/`, then emits a current ISO flag
with a warning when that historical asset has not been cached yet. A state with
neither source receives a labeled neutral symbol, so missing art can never remove
a country from the selector. The summary counts make every fallback visible.

The current checked cache contains sixteen exact-era flags: USSR, Russia, Iraq,
Brazil, Yugoslavia, Bulgaria, Serbia, Bosnia, Albania, Belarus, Georgia,
Venezuela, Syria, Lebanon, Oman and Bahrain. The 19 deliberate current-flag
fallbacks are Libya, South Africa, Ethiopia, Zaire, Afghanistan, Myanmar,
Mongolia, Cambodia, Congo, Honduras, Kyrgyzstan, Turkmenistan, Seychelles,
Comoros, Cape Verde, Macedonia, Montenegro, Zambia and Lesotho. Czechoslovakia
uses the local `cz` artwork because its flag design is identical. These
fallbacks keep offline builds complete; they are not claims that the modern
artwork is correct for 1990.

## Build the flag sprite

The ordinary flag source is exactly [`flag-icons` 7.5.0](https://www.npmjs.com/package/flag-icons/v/7.5.0)
(MIT). Its extracted package is a temporary build dependency and is not committed.
From the repository root, create the ignored directory first:

```powershell
New-Item -ItemType Directory -Force .tmp-flag-icons | Out-Null
```

or on POSIX shells:

```sh
mkdir -p .tmp-flag-icons
```

Then download, verify and extract the pinned package:

```sh
npm pack flag-icons@7.5.0 --pack-destination .tmp-flag-icons
python -c "import hashlib,pathlib; p=pathlib.Path('.tmp-flag-icons/flag-icons-7.5.0.tgz'); h=hashlib.sha256(p.read_bytes()).hexdigest(); assert h == 'c0b80bf0e08006a60f56621d6bc49f8c7131f4d1fef6737a165a673431f4b518', h"
tar -xzf .tmp-flag-icons/flag-icons-7.5.0.tgz -C .tmp-flag-icons
python tools/avatars/build_flags.py --offline
python tools/avatars/check_assets.py
```

`--offline` is the deterministic no-network build: it uses cached historical
files and reports the deliberate current-flag fallbacks above. Omit it to fetch
missing exact-era Commons files into the checked source cache. The generator
refuses a missing, incomplete or non-7.5.0 flag-icons directory with setup
instructions; use `--flag-icons <extracted-package>` only for an equivalent
custom location. It writes `spheres-web/ui/nation-flags-v1.svg`.

## Run the audit

From the repository root:

```powershell
py -3 tools/avatars/check_assets.py
```

or on any system where `python` selects Python 3:

```sh
python tools/avatars/check_assets.py
```

The audit uses only the standard library. It parses `ROSTER` directly, requires
exact equality of all 160 manifest keys, validates the historical-figure,
optional portrait and optional leader-art records, hashes local image files, and
parses the SVG to require every `flag-<NationId>` symbol. It reports all
discovered problems in one run and exits nonzero if any invariant is broken.

After the audit is green, run the web tests as the integration check:

```sh
cargo test -p spheres-web
```
