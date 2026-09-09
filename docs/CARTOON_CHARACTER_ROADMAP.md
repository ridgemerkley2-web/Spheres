# Cartoon characters through 2035

The user approved the fixed cartoon style on 7 September 2026 and requested all parties worldwide through 2035, including small coalition components. **Real people cover 1990 through 7 September 2026; later successors are explicitly fictional.** The former rotatable 3D and100k-triangle requirements are superseded.

## Visual standard and delivered art

Use bold dark outlines, expressive simplified faces, graphic cel shading, slightly enlarged heads and distinctive silhouettes. Show the complete figure, hands and shoes on opaque dark teal. The style anchor is `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`. Style approval is separate from the review of each artistic likeness.

All current originals are1024×1536 PNGs. The initial expansion supplied **15 historical cartoons and four original fictional cartoons**:

- UK: Margaret Thatcher, Neil Kinnock, Paddy Ashdown, Dafydd Elis-Thomas, John Major, Tony Blair, Gordon Brown, David Cameron and Theresa May.
- Japan: Toshiki Kaifu, Takako Doi, Kōshirō Ishida, Eiichi Nagasue and Tetsuzō Fuwa, covering the five represented parties'1990 starter leaders.
- USA: George H. W. Bush, separately linked to his executive office.
- Fictional Japan LDP candidates: Nakano Emi, Mizuno Naoki, Okazaki Haruka and Nakano Haruto.

The next worldwide batch adds researched casts from France, Germany, Italy, India, China, Brazil, South Africa, Canada and Australia. Its versioned images and exact prompts are in `tools/avatars/person-prompts/world-*-1990-v1.json`. The [live production inventory](../tools/ui/leadership-art-progress.html) gives the current physical artwork counts and country filters; [the static government review](../tools/ui/leadership-government-review.html) presents the actual renderer with exported, read-only simulation records.

Artwork covers only the explicit appearance intervals in the manifests, separate from party terms. A later source photograph can inform an earlier illustrated appearance only with that interpretation documented. Never borrow a national-selector face for a different identity. This is **not** completed global research or artwork coverage.

## In-game behavior

`person_portraits.json` binds real people to exact reviewed images and dates. `fictional_portraits.json` separately binds invented people to exact candidate IDs and appearance seeds, never claiming historical likeness.

Government Overview and Party leadership display uncropped lazy-loaded cartoons with era details and source/credit disclosures. Each party has a read-only “Future candidates through2035” section with fictional badges, invented biographies, institutional sources and remaining editorial work. Historical reference mode contains real source records only.

Fictional candidates can enter succession from8September2026 through31December2035. The date never replaces a saved incumbent. Actual gameplay events select candidates. Death and executive term limits exclude the appropriate identities. Existing saves preserve holders; narrowly reviewed empty-slot upgrades for Japanese, French and German components are documented in [the succession implementation](FICTIONAL_SUCCESSION_2035.md).

Unknown identities, missing art, uncertain dates and unresearched roles stay visible. Committee chairs, parliamentary leaders and executives are distinguished before linking them to succession. The US starter committee-chair terms are now imported as party offices only. New historical terms default to party-only; separately reviewed national roles are required under [the executive eligibility policy](PARTY_EXECUTIVE_ELIGIBILITY.md). Existing executive bindings are preserved.

## Worldwide production sequence

1. Inventory every represented party/component, including small coalition rows, and record missing real-world parties separately.
2. Research identities, roles, affiliations and organization lifecycles before making portraits; preserve uncertainty and gaps.
3. Finish country starter casts, then researched changes through the source cutoff. Add appearance variants for meaningful ageing rather than redundant yearly portraits.
4. Extend country- and party-specific research for fictional careers. Author distinct imagined people and appearances; broad global templates still need local editorial review.
5. Generate one cartoon per reviewed job with built-in image_gen. Inspect the actual output and record exact prompts, limitations, hashes and reviews.
6. Validate manifests and generated asset allowlists; inspect desktop and narrow layouts while preserving the campaign.
7. Introduce measured compressed display derivatives as the catalog grows, retaining original masters and lazy loading.

The [production inventory](LEADERSHIP_PRODUCTION_2035.md) and `tools/ui/leadership-art-progress.html` show researched history, pending art, fictional templates and actual PNGs separately. Four templates per game party/component are not four finished avatars or exhaustive real-world party coverage.

## Provenance and validation

New art used **OpenAI built-in image_gen**. Exact prompts, reference roles, correction inputs and retained output filenames are in `tools/avatars/person-prompts/cartoon-set-1990-v1.json` and `expansion-2035-batch1.json` through `expansion-2035-batch3.json`. Thatcher's original v3 prompt is retained. Discarded studies remain preserved.

Licensed source images retain credits in manifests/reference audits. Copyrighted pages used only for visual identity research are not shipped as game art. Generated illustrations do not relabel source rights.

Run historical/fictional art validators, production audit, generated allowlist check, native portrait tests and focused Government UI checks. Inspect date boundaries, credits, fictional disclosures, loading and unchanged campaign state.

Older3D characters remain archived. Equipment keeps its separate3D renderer.
