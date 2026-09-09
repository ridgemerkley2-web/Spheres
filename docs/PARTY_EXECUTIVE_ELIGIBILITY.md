# Party offices and national executives

A researched party leader can appear in the government page and hold a campaign party role without becoming the country's national executive. The selection policy is stored separately in `spheres-sim/data/party_executive_eligibility.json`, version 1, reviewed through **7 September 2026**. Neither the historical `Person`/`Term` schemas nor their identity hashes change.

Every new historical role defaults to **party-only**. A committee chair, party secretary, constituent leader, parliamentary leader, or a party officer titled “President” is not inferred to be a national president. Initial historical party bindings still use researched dates, including collective/component leadership. New national-office selections additionally require an exact policy grant.

## Institutional research and gameplay choices

- **United States:** [USAGov](https://www.usa.gov/election) distinguishes presidential nomination, the general election and the Electoral College. National committee chairs such as Ron Brown and Lee Atwater receive no presidential authorization from that party office.
- **France:** [Constitution articles 6–8](https://www.elysee.fr/en/french-presidency/constitution-of-4-october-1958) distinguish the elected President of the Republic and the President's appointment of the Prime Minister. The PS first secretary, an RPR party president, and UDF constituent officers are separate roles. This uses the institutional distinction, not today's term-length rules as a claim about 1990.
- **Germany:** The [Bundestag](https://www.bundestag.de/en/parliament/function/chancellor) describes election of the Chancellor and subsequent presidential appointment. CDU and CSU chairs remain distinct party officers; there is no invented overarching Union chair.
- **Italy:** [Constitution article 92](https://www.quirinale.it/allegati_statici/costituzione/costituzione_inglese.pdf) provides for presidential appointment of the President of the Council of Ministers.
- **India:** The [President's 2024 appointment statement](https://www.presidentofindia.gov.in/press_releases/press-communique-13) records appointment under article 75(1), following assessment of Lok Sabha majority support.
- **China:** [Constitution article 62](https://www.npc.gov.cn/englishnpc/constitution2019/201911/t20191120_384295.html) identifies selection of state offices by the National People's Congress. Party and state offices are not interchangeable.
- **Brazil, South Africa, Canada and Australia:** The separate policy records official electoral, constitutional and parliamentary sources for each. Party administration alone does not authorize a future national executive. South Africa’s 1996 constitutional source is explicitly not backdated to its 1990 institutions.
- **United Kingdom and Japan:** [UK Parliament](https://www.parliament.uk/about/how/role/relations-with-other-institutions/parliament-government/) describes government and parliamentary confidence; [Japan's Constitution, article 67](https://www.shugiin.go.jp/internet/itdb_kenpou.nsf/html/kenpou/en/constitution.htm) provides for Diet designation of the Prime Minister from its members. The exact 93 pre-policy UK/Japan terms have explicit **existing parliamentary gameplay** grants. These retain established game behavior; they do not claim that every historical member/qualification interval has been researched. Newly imported UK/Japan terms receive no automatic allowance.

The current government engine has one principal leadership row and generic succession events. It does not yet implement separate French presidential/parliamentary mandates, cohabitation, American presidential primaries or the Electoral College, or every country's appointment procedure. This policy prevents false identity substitution while those systems remain incomplete. No new historical executive contender grants are asserted in this delivery.

## Grant/import contract

`historical_grants[]` identifies the exact `nation`, `party`, `person`, `term`, `component`, and `source_role` string. It supplies a separately reviewed `executive_role`, `basis`, nonempty HTTPS `sources`, and an explanatory `note`. Matching a person name or a substring such as “president” never grants permission. Unknown terms, identity/component/title mismatches, duplicate grants, and missing sources are rejected.

New grants use `basis: researched_executive_role`. USA/France/Brazil require `executive_role: presidential_contender`; Germany/Italy/India/Canada/Australia require `parliamentary_government_contender`; China/South Africa require `national_executive_contender`. An import should record the separately evidenced executive candidacy as its own `kind: candidate` term where appropriate. It must not rewrite a saved party-chair term into a presidential term. A reviewed candidate can be selected for the executive while the committee chair retains the party role. Source links require human editorial review; URL validation alone does not prove the claim.

`future_office_grants[]` is a separate, currently empty map of exact fictional identities to authored executive roles. USA, France, Germany, Italy, India, China, Brazil, South Africa, Canada and Australia require this review before a new future national-office selection. Their fictional party leaders and previews remain available. The map uses the same exact identity keys and `source_role: fictional party successor`, with `basis: authored_fictional_executive_role`; sources describe institutions, and the note must disclose the invented nomination/background. Adding a reviewed grant does not rename, regenerate, or rehash the fictional person. Outside these ten newly reviewed countries, the previous fictional executive selection behavior remains explicitly labeled `existing_fictional_gameplay`, pending further country review.

The policy is applied only when the existing government engine actually seats a new office. It does not change votes, spend money, consume randomness, schedule historical winners, or replace an incumbent on a calendar date. Death, office exclusions, exact dates and ambiguity checks still apply. A failed policy load denies new named executive selection but leaves party succession functional. Existing original office links and valid saved executive bindings are read and validated under their unchanged identity contract, regardless of current selection permission.

## Read-model contract

Historical, uncertain, eligible, campaign, future-candidate and future-preview rows expose `executive_eligibility` with `policy_version`, `authorized`, `role`, `basis`, `sources`, `requires_executive_role_review`, `requires_presidential_role_review`, `note`, and `condition`. `authorized` means role permission for a new selection, not a prediction, appointment action, or unconditional date eligibility. An unreviewed row returns `role: party_only` and `basis: executive_role_unreviewed`.

The top-level `executive_policy` explains the relevant institution and declares `applies_to: new_executive_selections_only` and `existing_bindings_preserved: true`. A saved incumbent can therefore remain seated while its current permission for a *new* selection is party-only. The production future export contains the same separate permission information.

## Narrow save upgrades for new organization slots

The existing empty-slot migration now also recognizes the exact reviewed French UDF and German Union inventories:

- `France/fr_udf`: `fr_udf_federation`, `fr_udf_pr`, `fr_udf_dl`, `fr_udf_cds`, `fr_udf_fd`, `fr_udf_radical`, `fr_udf_psd`, `fr_udf_perspectives`, `fr_udf_ppdf`, `fr_udf_direct`, `fr_udf_pril`, in that order.
- `Germany/de_union`: `de_union_cdu`, `de_union_csu`, in that order.

Before the reviewed data is imported, these two parties retain their old empty component schema. After ingestion, loading an old campaign expands only a single **empty** legacy `component: null` assignment. Every new slot retains the original `since_day` and `reason`, and receives no person. The whole upgrade is planned before mutation and normal strict save validation follows it. Populated old slots—including fictional `main` identities—require a separate deliberate identity migration and are rejected. Mixed, duplicate, unknown, incomplete, or differently ordered component schemas are rejected; nothing guesses which constituent a saved person should represent.

These rules supplement the earlier Japanese JSP/Komeito migration. No other country's component map is expanded automatically. Adding organization components changes the future catalog's identity inventory; existing saved fictional identities must be audited before further structural imports.

Validation: `cargo test -p spheres-sim party_leadership --lib`. The read-only `audit_leadership_save_upgrade` example checks an actual archived user campaign, all state outside the four reviewed party maps, empty-holder metadata, deterministic second load, and unchanged input bytes.


## Research import safeguards

`tools/avatars/import_historical_people.py` preserves existing identity facts and adds only newly researched people and sources. `tools/avatars/import_party_terms.py` imports validated chronology into empty party rows; editing a populated chronology requires a separate review. Both tools reject source/receipt paths that collide with protected data and use exact input snapshots to detect concurrent edits.

All output JSON is validated and staged before any file is replaced. Each replacement is atomic, and ordinary write failures roll already replaced files back. This is not a filesystem-wide power-loss transaction. Exact repeat imports preserve existing data bytes and timestamps. New organization component inventories require an explicit native save migration and a matching reviewed importer schema before ingestion.

For static visual review without running a server, `cargo run -p spheres-sim --example export_party_leadership_views -- output.json` exports native 1990 and 2030 read-only views for the twelve researched/queued country batches. It creates an in-memory 1990 campaign, verifies that reading either reference date changes no campaign state, and does not load or write a user's save. The 2030 view previews fictional party candidates alongside unchanged campaign bindings; it does not simulate or predict a future government.


## Organization continuity in fictional previews

`spheres-sim/data/future_party_continuation.json` contains separately reviewed editorial disclosures. Future candidate and preview rows, and the production export, expose `historical_continuation_status` and `historical_continuation` (status, sources and note); unreviewed parties return null. These metadata never enter identity hashes or control campaign succession.

South Africa's NP/NNP is labeled `ceased`: the [IEC's 31 August 2006 statement](https://www.gov.za/news/independent-electoral-commission-cancellation-registration-political-parties-31-aug-2006) records final cancellation of registration effective 3 July 2006, distinct from the earlier dissolution decision. The original DP is labeled `unverified`: the [DA's party history](https://www.da.org.za/why-the-da/history/) describes a subsequent alliance/transition, while the exact original DP integration interval remains unresolved here. Its fictional people are not DA leaders.

A campaign may retain either original party after diverging from history. Their character templates therefore remain available for actual campaign vacancies, and existing incumbents remain untouched. A read-only future reference respects a sourced dissolution bound; a template preview never claims that an organization is currently active or predicts its survival.


Canada's original federal PC and Reform/Alliance registrations are also labeled `ceased`. [Elections Canada's 2003 report](https://www.elections.ca/content.aspx?dir=rep%2Foff%2Fsta2003&document=fo&lang=e&section=res) records their replacement by the newly merged Conservative Party on 7 December 2003. The [Chief Electoral Officer's 2000 decision](https://elections.ca/content.aspx?dir=pre&document=decision&lang=e&section=med) establishes that Reform-to-Alliance was a name change effective 27 March 2000, not a legal dissolution. The intervening Alliance leadership and name-phase presentation remain unreviewed. A dedicated lifecycle correction receipt records changes to party boundary/source/note fields only; historical people, terms, office links and gap notices are unchanged.
