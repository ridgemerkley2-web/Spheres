# Fictional succession through 2035

The historical reference ends on **7 September 2026**. Real people and sourced terms remain in `spheres-sim/data/party_leaders.json`; invented people never enter that roster. Missing historical research remains visible as a gap.

From **8 September 2026 through 31 December 2035**, an actual campaign succession can select an explicitly fictional party leader. National-office selection also follows the separate [executive eligibility policy](PARTY_EXECUTIVE_ELIGIBILITY.md): party membership alone grants no presidency or reviewed national-office role. Advancing the date does not replace any incumbent. Elections still come from the existing government simulation. Death and term limits exclude the same stable identity from subsequent selection. A person already seated in 2035 may remain after the candidate window closes.

## Authored content and research

[International IDEA's primer on political parties](https://www.idea.int/publications/catalogue/html/political-parties-constitutional-roles-recognition-rights-and) describes candidate recruitment, leadership selection, programme development, and the variety of party institutions. [UK Parliament's party-system overview](https://www.parliament.uk/about/mps-and-lords/members/partysystem/) provides a concrete example of legislative careers and distinct government/opposition roles. Those sources support the institutional framing. They do **not** identify future people, prove the invented biographies, or predict political outcomes.

Four gameplay profiles cover legislative organization, coalition negotiation, party administration, and a researched coalition component. Their assignment uses the existing simulation's institutional and ideological data. This is an authored game approximation, not a new global survey of current party selection rules.

The versioned name pools, exact dates of birth, brief careers, gender presentation, and appearance seeds are invented. Name pools provide romanized examples and need country-specific editorial review; they do not claim to represent every language, ethnicity, or naming tradition. No historical person's face is assigned to a fictional ID. Real-person sources stay empty for fictional people; the separate `fiction.research_basis` describes institutional background only.

The four LDP pilot characters have additional party-specific grounding from the [LDP's published presidential-election rules](https://www.jimin.jp/english/the-president/rules/), checked on 7 September 2026. The rules describe participation by party Diet members and eligible party members/associated organizations, and the candidate chapter specifies nomination by twenty party Diet members. The fictional pilots are hypothetical LDP Diet members with distinct careers in legislative organization, regional administration followed by Diet service, economic policy, and legislative negotiation. Those careers and nomination prospects are invented; continuing the published institutional arrangement through 2035 is a gameplay assumption. This source is scoped only to `Japan/jp_ldp`, never assigned to other countries or parties. Adding editorial research and biographies does not change any name, identity, appearance seed, profile ID, or saved identity hash.

## Coverage and limits

The runtime catalog contains four stable successor templates for each existing game party/component. This covers the simulation inventory, including its small parties. It is not a claim to cover every party that existed in the real world, and a template is not a completed avatar. The generated production export records these distinctions in `spheres-web/data/future_candidates_2035.json`.

The simulation can diverge: a party that dissolved historically may survive in a campaign. Such a party's fictional continuation is explicitly conditional. Browsing a future reference date excludes parties/components already known to have dissolved; the separate campaign preview can still show the conditional templates. No-party governments retain their existing institutional succession and are not assigned invented parties.

There are four candidates per component, not an unlimited generator. If all are unavailable, the system keeps the existing unfilled-identity behavior. Collective leadership is preserved; the catalog does not invent a sole executive when several component holders are equally eligible.

German Green and SPD future leadership additionally uses a separately reviewed two-seat policy within the same party. [The Green statute updated in July 2026, section 17](https://cms.gruene.de/uploads/assets/Gruene-Regeln.pdf), specifies two equal chairs including at least one woman and distinguishes replacement elections. [The SPD's published federal statute, section 23](https://www.spd.de/fileadmin/Dokumente/Parteiorganisation/SPD_Orgastatut_2025.pdf), permits one chair or a pair. Continuing a pair through 2035 is an authored game choice. The [2025 SPD resolution book](https://parteitag.spd.de/fileadmin/parteitag/Dokumente/Beschluesse_oBPT2025/Beschlussbuch_oBPT25.pdf) refers the proposal to explicitly permit a two-woman federal pair; the game therefore uses a conservative woman/man assumption for entirely fictional SPD teams without claiming a universal prohibition.

The seat policy adds no fake components and changes no saved identity. An actual future succession fills an empty pair or a missing co-holder, retaining every survivor and each person's original selection date. A historical Green trio is not trimmed on a calendar date; a surviving historical single SPD chair is not automatically converted to a pair. Other single-leader parties keep their behavior. Qualified candidates are separately authored by exact fictional ID, never inferred from artwork or names. If a required fictional qualification is exhausted, the vacancy stays open. Historical incumbents remain unclassified; a new fictional woman can be placed beside an unknown historical co-holder, and the read model explicitly does not claim that mixed team's full quota eligibility has been researched. Full congress choice, nomination procedures, board membership and statutory restrictions remain outside this bounded implementation.

Sequential organizations within one party are different from simultaneous coalition components. In Japan, future vacancies use the SDP organization formed in 1996 and the Komeito organization reconstituted in 1998. A previously bound campaign incumbent remains in place, including one associated with an earlier organization. Only an actual vacancy or office exclusion selects a successor from the surviving organization. A term limit preserves the old party leader while permitting a different executive candidate. Historical coalition components continue to hold separate leadership positions.

The full production catalog retains archived organization templates for traceability. `component_available: false` marks those templates; they are excluded from actual future candidate eligibility. This is why catalog-template coverage can exceed the number currently available for succession.

## Developer contract

- `party_leadership::fictional_person(id)` returns a `FutureCandidate` with a normal `Person`, separate fiction metadata, and a stable `appearance_seed`.
- `party_leadership::person_view(id)` returns the display person and a `fiction` object only for fictional identities. Historical `Person` and `Term` structures are unchanged.
- `parties[].future_preview` is always read-only and always has `eligible: false`; it explains the condition that the party/component survives in the campaign.
- `parties[].future_candidates` contains candidates for the selected future date. They are not historical incumbents, election forecasts, or appointable UI actions.
- A seated fictional leader appears in the usual `campaign[].person` and `executive_person` slots with explicit `fiction.origin: "fictional_successor"`.
- Candidate IDs include catalog version, nation, exact party, component, and slot. Existing saves retain their book structure. Load validation checks the candidate identity hash, party/component, event reason, and selection date.
- `future_party_leadership_seats.json` supplies separate German Green/SPD future seat rules and exact authored candidate qualifications. `parties[].future_leadership_seats` reports the target, current holders, vacancies, sources, continuity assumptions and any unclassified historical incumbents. Campaign rows expose the individual's `since_day` and the separate latest `party_succession_day`.

Future catalog structure also depends on the researched party/component inventory. Adding or replacing components, changing a fictional identity's name, birth date or profile ID, or reclassifying its saved affiliation can invalidate existing future bindings. Once those identities have entered saved campaigns, further research ingestion must include a deliberate saved-identity migration and replay validation before release. Do not assume that all new historical research is automatically compatible. Editorial biography/source additions preserve hashes because those fields are deliberately outside the identity hash; this narrower guarantee does not cover structural catalog changes.

Regenerate the production export with:

```powershell
$env:CARGO_TARGET_DIR='company-sim-target'
cargo run -p spheres-sim --example export_future_leadership -- spheres-web/data/future_candidates_2035.json
```

Targeted validation: `cargo test -p spheres-sim party_leadership --lib`. Tests cover exact cutoff boundaries, unchanged incumbents, real event integration, replay after save/load, term limits, death, forged bindings, dissolved reference parties, and complete game-party template coverage.

## Automatic upgrade of older research slots

The expanded Japanese research distinguishes the JSP/SDP organizations and the three Komeito organizational periods. Older campaigns stored one empty, unspecified organization slot for each of these two party rows. Loading those saves now expands **only empty** legacy `jp_jsp` and `jp_komeito` slots into their documented component slots. The new slots remain empty, and their saved date and reason remain unchanged. All other game state and person bindings are preserved.

This structural upgrade does not restart the campaign, fill historical gaps, appoint leaders, consume randomness, or advance time. The next normal save records the expanded structure, and subsequent loads leave it unchanged. A populated obsolete slot requires explicit identity migration and is rejected. Mixed, duplicated, unknown, or incomplete organization maps are also rejected; ordinary strict save validation still runs after the upgrade.

The same narrow upgrade also supports the reviewed French UDF (11 components) and German Union (CDU/CSU) imports described in [the executive policy](PARTY_EXECUTIVE_ELIGIBILITY.md). Only one empty legacy null row can expand; populated historical or fictional main-slot holders require explicit identity migration.

The read-only audit accepts an actual saved campaign path and verifies that every field outside the four reviewed party maps remains equal, that no holders were assigned, that save/load is stable, and that the input file remains byte-for-byte unchanged:

```powershell
cargo run -p spheres-sim --example audit_leadership_save_upgrade -- C:\path\to\save.json
```
