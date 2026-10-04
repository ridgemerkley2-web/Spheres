# Tonga cast: independent automated visual review (2026-10-04)

**Status:** Proposed independent automated visual review by Claude, automated agent - not human or Codex review; does not close the final-visual-review requirement, CLAUDE-C06-TONGA-01, C06, S23 or CP1.

Label: **Claude, automated agent - not human or Codex review.** This packet is proposed evidence for Codex/root and the user on the
"final visual review" item that remains open in `CLAUDE-C06-TONGA-01`. It approves nothing and changes no status.
Nothing outside this directory was edited, and nothing was committed or pushed by the review.

Repository state reviewed: branch `claude/nifty-bohr-0j27qi`, HEAD `cf29127965ec93b6bb01c057d48d94b9aec84cf6`, clean working tree.

## Scope

- **Reviewed: 46 of 46 active Tonga cast portraits (100%)** for 36 person_ids:
  42 historical portraits of 32 real people, plus 4 fictional civilians.
- **Required but missing:** Fatai Helu (`to_fatai_helu`, required 2022-08-29 to 2022-08-30) has no art. That makes 33
  historical identities against 32 with art, in line with the manifest's `historical_people=33` and
  `REMAINING-WORK.md`. Codex's external gallery recorded 47 cards: 42 + 4 + 1 explicit Fatai gap.
- **Inventory discrepancies found and reconciled** (details in `review.json` → `inventory.notes`):
  - Only 45 portraits are named `tonga-*.png`. The 46th is Tupou IV's opening portrait
    `taufaahau-tupou-iv-cartoon-1990-v1.png`, generated 2026-09-13. It is covered by none of the seven batch
    registrations, which register 41 historical jobs between them.
  - The manifest's `coverage_inventory.bound_historical_people=32` is a different set of 32 from the people
    with art: it includes Fatai and excludes the 1985-born heir, who has 3 portraits but no holder binding.
  - Two reference files are byte-identical: `references/taufaahau-tupou-iv-eth-1985-v1.jpg` and
    `references/tonga-taufaahau-tupou-iv-eth-1985-reference-v1.jpg`.
  - The country manifest is at `docs/campaign-certification/C06/countries/tonga/manifest.json`, not the
    `countries/tonga/` path the task text assumed.
  - The three rejected studies (Lavaka Ata v1, Fuko v1, Crown Prince 2030 v1) are not in the repo or the allowlist.
    The allowlist holds exactly the 46 active cartoons.
  - The Salote Tupou III national-representative art is not cast art and was excluded.
- Eight historical portraits of eight people have **no in-repo likeness reference** (authored identity). Seven of those people have no in-repo reference at all; Fakafanua has one only for his separate 2013 portrait. See the limits below.

## Method

1. **Rubric.** Each portrait was scored on six dimensions: style, likeness, era/age, technical, small card and
   fictional resemblance. Each dimension got pass, note, concern or not_applicable, and each portrait got an overall
   pass, pass_with_notes or concern. Definitions are in `review.json` → `rubric`. Overall is concern if any
   dimension is concern, otherwise pass_with_notes if any dimension is note. Five portraits (Fa'otusia 2020,
   Fusimalohi 2023, Tevita Poasi Tupou 2000, Pisila Tukuafu, Kalolo Matalehu) are pass_with_notes although every
   dimension is pass or n/a, because their passing dimension text records minor notes. fictional_resemblance has
   no separate notes field; its evidence is in the fictional likeness, style and small-card notes and the cast-wide pass.
2. **Anchor.** Every portrait was compared with the fixed style anchor `spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png`
   (sha256 `8fe7d0361e75f80f6e65fbe19f1507d94313e72105a60a42d8f5aa09798258ef`): bold outlines, a clear simplified face, a full-body composition, readable small-card
   silhouettes, and a quiet opaque dark teal background.
3. **References viewed.** Likeness was compared only against the in-repo reference photograph(s) that the project
   already labels for the exact person_id. Those are listed per portrait in `review.json` → `portraits[].references_compared`
   (31 distinct files, all under open or attribution-only grants per the inventory). For group photos, the reviewer used the subject the record names,
   for example the left microphone-holder in the 2000 Lavaka Ata photo. No identity was inferred from any face.
   For the eight authored-identity portraits, only consistency with the recorded text observations was checked.
4. **Mechanical checks (Pillow).** Size, mode and alpha; border colour uniformity; figure bounding box and margins;
   hands and feet at 2-3x zoom; and sha256 against the registry, registrations and receipts.
5. **Downscale checks.** 96x96 (squash, contain, and cover at object-position 50% 12% / 50% 8%) and 160 px tall.
   Some portraits were also checked in an exact simulation of the 93x118 `.nation-art` crop from `main-menu.css`.
6. **Adversarial challenge.** Every concern a per-portrait reviewer raised went to a separate challenger pass that
   tried to refute it by viewing the images and the records. Result: **3 raised, 3 upheld (all minor), 0 rejected.**
   Two of the three were narrowed (see below). Notes that were not concerns were not challenged.
7. **Cast-wide pass.** A contact sheet (`contact-sheet.png`) was used for house-style consistency, head proportions,
   card-size confusable pairs whose display windows overlap, same-person continuity across eras, and fictional
   distinctness. These findings were not adversarially challenged.
8. **Packet assembly checks.** While assembling this packet I re-verified every hash, re-read the records behind
   each upheld concern, and reconciled the places where the cast-wide text and the per-portrait notes disagree.

## Results

Overall: **3 pass, 42 pass_with_notes, 1 concern.**
Upheld concerns: **0 blocking, 3 minor** (on 2 portraits). Every one of the 46 files
matches its recorded sha256.

| Dimension | pass | note | concern | n/a |
|---|---|---|---|---|
| style | 26 | 20 | 0 | 0 |
| likeness | 31 | 10 | 1 | 4 |
| era_age | 32 | 14 | 0 | 0 |
| technical | 43 | 3 | 0 | 0 |
| small_card | 34 | 12 | 0 | 0 |
| fictional_resemblance | 4 | 0 | 0 | 42 |

Note: George Tupou V 1990 has one upheld minor concern, although its per-portrait overall was recorded as
pass_with_notes. Its concern is about the appearance window, not the artwork.

Intervals are end-exclusive appearance windows, not office terms.

| # | Portrait | person_id | Interval | Overall | Note |
|---|---|---|---|---|---|
| 1 | Prince Fatafehi Tu'ipelehake 1990 | `fatafehi_tuipelehake` | 1990-01-01 to 1992-01-01 | pass_with_notes | Consistent with the 1971 reference and plausibly aged to about 60-68; close to the Tupou IV 1998 card during the 1991 overlap, separated by tie colour and build. |
| 2 | George Tupou V (as Crown Prince Tupouto'a) 1990 | `siaosi_taufaahau_manumataongo` | 1990-01-01 to 2006-09-11 | pass_with_notes | Consistent with both labelled references (moustache, face, build). Upheld minor concern: the glasses-free near-1990 look is shown until 2006, but the dated 1997 reference shows spectacles. |
| 3 | George Tupou V 2011 | `siaosi_taufaahau_manumataongo` | 2006-09-11 to 2012-03-19 | pass_with_notes | Uniform, sash and grey moustache match the 2011 reference; the bald crown hidden by the cap in the reference is a reconstruction the prompt asked for but the era_note does not record; near-photographic rendering. |
| 4 | Taufa'ahau Tupou IV 1990 | `taufaahau_tupou_iv` | 1990-01-01 to 1991-01-01 | pass_with_notes | Strong match to the 1985 reference; reads marginally younger than the reference because of cartoon smoothing; one-year window. |
| 5 | Taufa'ahau Tupou IV 1998 | `taufaahau_tupou_iv` | 1991-01-01 to 2006-09-11 | concern | Face matches the 1985 reference. Two upheld minor concerns: the spectacles are unreferenced and missing from the era_note, and the face reads about 70-75 rather than the recorded about 80 (window runs to about 88). |
| 6 | 'Aisake Valu Eke 2025 | `to_aisake_valu_eke` | 2020-01-01 to 2027-01-01 | pass_with_notes | Strong match to the 2025 reference; small head (about 7 heads); card is close to Lavemaau 2020 in the same window. |
| 7 | 'Alipate Halakilangi Tau'alupeoko Tupou (Baron Vaea) 1996 | `to_baron_vaea_pm_1992` | 1991-01-01 to 2001-01-01 | pass_with_notes | Matches the 1996 Courier halftone; clearly distinct from his son Lord Vaea; very distinctive ta'ovala card. |
| 8 | Lord Fakafanua 2013 | `to_fatafehi_fakafanua` | 2012-01-01 to 2018-01-01 | pass_with_notes | Strong match to the 2013 reference; cap removed and hair reconstructed, as recorded. |
| 9 | Lord Fakafanua 2026 | `to_fatafehi_fakafanua` | 2018-01-01 to 2027-01-01 | pass_with_notes | No in-repo reference (authored identity). Consistent with the text observations and the 2013 face; may read old for 2018-2020. |
| 10 | Feleti Vaka'uta Sevele 2008 | `to_feleti_sevele` | 2006-01-01 to 2011-01-01 | pass | Strong match, including the cheek mole; high card-confusion risk with Tangi 2008 (identical window). |
| 11 | Siaosi 'Alokuo'ulu Wycliffe Fusitu'a 1998 | `to_fusitua_speaker_1996` | 1991-01-01 to 2001-01-01 | pass_with_notes | No in-repo reference. Consistent with the recorded text; dark suit is close to Cocker and Veikune at 96 px. |
| 12 | James Cecil Cocker 2014 | `to_james_cecil_cocker` | 2000-01-01 to 2015-01-01 | pass_with_notes | No in-repo reference. The 2014 look is applied back to 2000, as recorded; low-contrast dark card. |
| 13 | Langi Kavaliku 1998 | `to_langi_kavaliku` | 1995-01-01 to 2001-01-01 | pass_with_notes | No in-repo reference. Consistent with the text; most distinctive silhouette in its group. |
| 14 | Havea Lasike (Lord Lasike) 2011 | `to_lord_lasike_speaker_2011` | 2008-01-01 to 2014-01-01 | pass_with_notes | Matches the 2021 reference. The recorded 10-years-younger look rests on facial smoothing (hair still light grey); resembles the Sevele and Tangi cards. |
| 15 | Lord Ma'afu 2016 | `to_lord_maafu_dpm_2017` | 2013-01-01 to 2021-12-12 | pass_with_notes | Close match to the 2016 reference; a minor jacket-closure oddity is visible only at full size. |
| 16 | Havea Tu'iha'angana (Lord Tu'iha'angana) 2019 | `to_lord_tuihaangana_deputy_speaker_2025` | 2018-01-01 to 2027-01-01 | pass_with_notes | Matches the 2019 reference, including the garland; a tiny indistinct lapel pin despite the prompt's 'No political badges or regalia'. |
| 17 | Malakai Fakatoufifita (Lord Tu'ilakepa) 2015 | `to_lord_tuilakepa_speaker_2008` | 2007-01-01 to 2018-01-01 | pass_with_notes | Matches the 2015 reference (sunglasses, hair); gold frames (as the prompt asked) where the reference reads silver/gunmetal; low-contrast all-navy card. |
| 18 | Siale'ataongo Tu'ivakano 2011 | `to_lord_tuivakano_pm_2010` | 2010-01-01 to 2020-01-01 | pass_with_notes | Close match to the 2011 reference and outfit; no greying for late-2010s use. |
| 19 | Siale'ataongo Tu'ivakano 2003 | `to_lord_tuivakano_pm_2010` | 2000-01-01 to 2010-01-01 | pass_with_notes | Same face as the 2011 reference; the recorded 'about eight years younger' is barely visible. |
| 20 | 'Alipate Tu'ivanuavou Vaea (Lord Vaea) 2020 | `to_lord_vaea_speaker_2025` | 2018-01-01 to 2027-01-01 | pass_with_notes | Matches the 2020 reference; busy shirt print still reads well at card size; distinct from his father. |
| 21 | Poasi Mataele Tei 2021 | `to_poasi_mataele_tei` | 2018-01-01 to 2027-01-01 | pass_with_notes | No in-repo reference. Consistent with the text; the burgundy blazer is distinctive. |
| 22 | Pohiva Tu'i'onetoa 2019 | `to_pohiva_tuionetoa` | 2019-01-01 to 2024-01-01 | pass_with_notes | Strong match, including glasses and cane; smallest head ratio in its review group (about 7.9 heads) and among the smallest in the cast. |
| 23 | Samiu Kuita Vaipulu 2023 | `to_samiu_kuita_vaipulu` | 2018-01-01 to 2027-01-01 | pass_with_notes | Very strong match, including the garland; detailed, photo-derived face. |
| 24 | Samiu Kuita Vaipulu 2011 | `to_samiu_kuita_vaipulu` | 2011-01-01 to 2018-01-01 | pass_with_notes | Same person as 2023; the recorded 12-years-younger interpretation is only modestly visible. |
| 25 | Samuela 'Akilisi Pohiva 2015 | `to_samuela_akilisi_pohiva` | 2010-01-01 to 2020-01-01 | pass_with_notes | Strong match to the 2015 reference; with George Tupou V 2011, the most painterly, near-photographic rendering in the cast. |
| 26 | Semisi Kioa Lafu Sika 2019 | `to_semisi_kioa_lafu_sika` | 2015-01-01 to 2027-01-01 | pass_with_notes | Matches the 2019 reference; jacket a brighter navy than the reference; low-contrast suit. |
| 27 | Crown Prince Tupouto'a 'Ulukalala 2022 | `to_siaosi_manumataongo_tukuaho_1985` | 2018-01-01 to 2027-01-01 | pass_with_notes | Close match to the 2022 reference (tied hair, goatee, ta'ovala). |
| 28 | Crown Prince Tupouto'a 'Ulukalala 2012 | `to_siaosi_manumataongo_tukuaho_1985` | 2012-01-01 to 2018-01-01 | pass_with_notes | Matches the far-left subject of the 2012 reference; tie pattern differs; high card-confusion risk with Latu 2013. |
| 29 | Crown Prince Tupouto'a 'Ulukalala 2030 (v2) | `to_siaosi_manumataongo_tukuaho_1985` | 2027-01-01 to 2036-01-01 | pass_with_notes | Background fix verified (no black residue). The recorded future projection shows almost no ageing compared with 2022. |
| 30 | Siaosi 'Ofakivahafolau Sovaleni 2022 | `to_siaosi_ofakivahafolau_sovaleni` | 2014-12-31 to 2027-01-01 | pass_with_notes | Matches the 2022 reference, including the moles; slightly more smiling; same broad type as Sika. |
| 31 | Sione Vuna Fa'otusia 2020 | `to_sione_vuna_faotusia` | 2018-01-01 to 2021-09-01 | pass_with_notes | Matches the 2020 reference (moustache plus white chin beard); hair curlier than the reference; navy suit is a prompt choice (reference black). |
| 32 | Taniela Likuohihifo Fusimalohi 2023 | `to_taniela_likuohihifo_fusimalohi` | 2020-01-01 to 2027-01-01 | pass_with_notes | Strong match to the 2023 reference; busy shirt print; seated-to-standing extension, as recorded. |
| 33 | Tesina Fuko 1996 | `to_tesina_fuko` | 1996-01-01 to 2009-01-01 | pass | Strong match to the 1996 Courier inset; the v2 hand fix is verified. |
| 34 | Tevita Lavemaau 2020 | `to_tevita_lavemaau` | 2017-01-01 to 2027-01-01 | pass_with_notes | No in-repo reference. Consistent with the text; card is close to Eke 2025 in the same window. |
| 35 | Tevita Poasi Tupou 2000 | `to_tevita_poasi_tupou` | 1998-01-01 to 2005-01-01 | pass_with_notes | Matches the 2015 reference without the wig; moustache greyer than requested for 2000. |
| 36 | Tupou VI 2019 | `to_ulukalala_lavaka_ata_pm_2000` | 2012-01-01 to 2027-01-01 | pass_with_notes | Matches the 2019 reference; one mature image covers 2012-2026, as recorded. |
| 37 | Prince 'Ulukalala Lavaka Ata 2000 (v2) | `to_ulukalala_lavaka_ata_pm_2000` | 2000-01-01 to 2012-01-01 | pass_with_notes | Correct subject (left, no glasses); the face rests mainly on the undated reference, as recorded. |
| 38 | Tupou VI 2030 | `to_ulukalala_lavaka_ata_pm_2000` | 2027-01-01 to 2036-01-01 | pass_with_notes | Plausibly older than 2019 (modest greying); at 96 px told apart from 2019 mainly by the tie. |
| 39 | Hon. Veikune 2005 | `to_veikune_speaker_2001` | 1999-01-01 to 2007-01-01 | pass_with_notes | No in-repo reference. Consistent with the text; darkest card in its review group (all-black suit and shirt); resembles Ma'afu, but they are never co-displayed. |
| 40 | Viliami Ta'u Tangi 2008 | `to_viliami_tau_tangi` | 2006-01-01 to 2011-01-01 | pass_with_notes | No in-repo reference. The younger salt-and-pepper look is weakly expressed; high card-confusion risk with Sevele. |
| 41 | Viliami Uasike Latu 2013 | `to_viliami_uasike_latu` | 2010-01-01 to 2019-01-01 | pass_with_notes | Matches the 2013 reference; the numeric age is unsourced; low-contrast charcoal card, confusable with Crown Prince 2012. |
| 42 | Viliami Uasike Latu 2025 | `to_viliami_uasike_latu` | 2019-01-01 to 2027-01-01 | pass_with_notes | Plausibly aged forward from 2013, though modestly (reads mid-to-late 50s). |
| 43 | Lesieli Fotu 2026 (fictional) | `fictional_to_lesieli_fotu` | 2026-09-08 to 2036-01-01 | pass | Fictional; distinct from the whole cast; unprompted pearl studs echo the anchor. |
| 44 | Sitani Lolohea 2026 (fictional) | `fictional_to_sitani_lolohea` | 2026-09-08 to 2036-01-01 | pass_with_notes | Fictional; one hand in a pocket; ta'ovala worn under the jacket is flagged for a cultural reviewer. |
| 45 | Pisila Tukuafu 2026 (fictional) | `fictional_to_pisila_tukuafu` | 2026-09-08 to 2036-01-01 | pass_with_notes | Fictional; minor prompt deviations (build, bag, frame shape). |
| 46 | Kalolo Matalehu 2026 (fictional) | `fictional_to_kalolo_matalehu` | 2026-09-08 to 2036-01-01 | pass_with_notes | Fictional; distinctive red shirt and headphones; decorative ta'ovala flagged for a cultural reviewer. |

## Upheld concerns

### 1. George Tupou V 1990 (`tonga-george-tupou-v-cartoon-1990-v1.png`, `siaosi_taufaahau_manumataongo`)

- **Severity:** minor (upheld by the challenger).
- **Concern:** A near-1990, glasses-free depiction (about age 42) is shown from 1990-01-01 to 2006-09-11. The asset's own dated, in-window 1997 reference shows him wearing spectacles.
- **Challenger's reasoning:** The challenger confirmed the window in `person_portraits.json` and the manifest. By the challenger's count it is the longest of the 138 dated appearance windows in the registry. Codex's own `royal-identity-review.json` notes 'glasses' for the 1997 photo, and the era_note and prompt never say glasses were omitted. Other long Tonga windows (Cocker, Tu'ilakepa, Fuko) state their stretch explicitly, and Tupou IV's near-1990 art is limited to 1990 while a separate older variant covers 1991-2006. The concern was narrowed: the 'greying temples' in the 1997 halftone cannot be confirmed, so only the spectacles count. It is minor because the identity, style, technical quality and card readability are sound; the problem is that the recorded era does not match the window.
- **Suggested fix (for Codex/root or the user):** Narrow the window and add a later variant with glasses based on the 1997 reference, or amend the era_note to say that 1997-2006 keeps the glasses-free near-1990 look as an artistic interpretation.

### 2. Taufa'ahau Tupou IV 1998 (`tonga-taufaahau-tupou-iv-cartoon-1998-v1.png`, `taufaahau_tupou_iv`): spectacles

- **Severity:** minor (upheld by the challenger).
- **Concern:** The cartoon adds rectangular dark-rimmed spectacles. The only labelled reference (1985 ETH, no glasses) does not support them, and the era_note, visual_note, credit, batch-05 job sub-object and registration do not record them.
- **Challenger's reasoning:** Correction: the spectacles were requested on purpose in the prompt TXT and the batch-05 exact_prompt, under 'artistic ageing'. The challenger still upheld the concern for three reasons. An accessory does not follow from ageing a face. The 1990 record that the 1998 identity_source cites says 'No glasses'. And the project treats glasses as a trait worth recording (Lavaka Ata v1 was rejected partly for unsupported glasses). It is minor because the face and identity are right, no in-window evidence shows him without glasses, and the spectacles are faint at 160 px and not legible at 96 px. The challenger's background knowledge that he often wore glasses later in life is not project evidence.
- **Suggested fix (for Codex/root or the user):** Record the spectacles as an unreferenced artistic addition (reversing the 1990 'No glasses' note), or attach a reviewed, dated 1991-2006 reference showing glasses, or regenerate without them.

### 3. Taufa'ahau Tupou IV 1998: weak age projection

- **Severity:** minor (upheld by the challenger).
- **Concern:** The era_note says the face was 'artistically aged thirteen years forward' from the 1985 portrait to about 1998, and the prompt asks for 'aged80'. The cartoon reads about 70-75: about the same as, or only a few years older than, the 1985 photograph (about 67). The window runs to 2006-09-11 (about 88).
- **Challenger's reasoning:** The challenger compared zoomed faces of the 1985 photo, the 1990 cartoon and the 1998 cartoon. The age cues are real (creases, under-eye bags, age spots, receded grey hairline), and the 1998 cartoon reads clearly older than the 1990 one. But the photo's heavy eyelids and jowls are stronger than the cartoon's, and no record says the drawn ageing falls short of the stated ageing. It is minor because the likeness is unchanged, the face is not younger than the reference, the window is disclosed as an interpretation, and cartoon age is subjective.
- **Suggested fix (for Codex/root or the user):** Add an honest note that the rendered ageing is modest, or consider a more strongly aged regeneration or a later split variant (about 2000-2006).

## Rejected concerns

None. All three concerns raised were upheld. Two parts were narrowed rather than accepted as first written:

- George V: the claim of "greying temples" in the 1997 photograph is dropped, because the halftone cannot show it.
- Tupou IV: the claim that the spectacles were entirely "unrecorded" is corrected. They are in the prompt TXT and the
  batch-05 exact_prompt, but not in the era_note, visual_note or registration.

## Cast-wide findings

The cast-wide findings are unchallenged observations, offered for triage. The full text is in `review.json` → `cast_findings`.

- **House style.**
  - Background and framing pass for all 46: opaque flat dark teal (RGB about 13-19/39-46/48-55), full body and centred,
    and no cropped heads or shoes.
  - Palette passes, and no hand, foot or limb artefacts were seen.
  - **Systematic deviation:** the whole Tonga cast is more naturalistic than the anchor. Heads are smaller (roughly
    5-8 heads tall, mostly 6-7.5, against the anchor's about 4.2-4.4 and the prompts' 5.5), and faces are painterly and semi-realistic.
  - The four fictional portraits are closer to the anchor (bolder outlines, flatter cel shading) than the historical ones.
  - Weakest adherence: George Tupou V 2011 and Akilisi Pohiva 2015 (near-photographic shading, strongly turned heads).
- **Card-size confusion between portraits shown at the same time.**
  - High risk:
    - Crown Prince 2012 and Viliami Latu 2013 (2012-2018).
    - Feleti Sevele 2008 and Viliami Tangi 2008 (identical 2006-2011 window).
  - Moderate-high: Lavemaau 2020 and Eke 2025 (2020-2027).
  - Moderate: Lasike 2011 with Sevele and Tangi, plus several other pairs.
  - Suggested fix: change one card per high-risk pair (suit colour, tie or accessory) and record it as an artistic
    interpretation.
- **Same-person continuity.**
  - Strong or plausible for every multi-portrait person.
  - Weak age separation: Tu'ivakano 2003 vs 2011 (the record's "about eight years younger" is hardly visible) and
    Crown Prince 2022 vs 2030.
  - Biggest jump: Lavaka Ata 2000 to Tupou VI 2019.
  - The Tupou IV 1998 spectacles gap is the same issue as upheld concern 2.
- **Fictional people.**
  - All four are distinct from the all-male historical cast and from each other.
  - Lolohea's outfit resembles Crown Prince 2030, but the faces differ clearly.
  - The records show style-only input and no likeness reference.
- **Reconciling the cast-wide text with the per-portrait notes:**
  - Head proportions: the cast pass reports about 6.5-7.5 heads for all 42 historical portraits and about 6.3-6.5 for the fictional ones. The per-portrait passes estimated about 5 (Fakafanua 2026), about 5.9-6 (Akilisi, Fakafanua 2013, Sevele, Vaipulu 2011), about 6.5-7 (Eke, Baron Vaea, Poasi Tei, Vaipulu 2023) and about 7.9 (Tu'i'onetoa), and about 5.5 for Kalolo Matalehu. These are rough estimates made with different methods. The direction agrees: the Tonga cast is more naturalistic than the anchor (about 4.2-4.4 heads) and the prompts' 5.5.
  - Head turns: the cast pass calls George Tupou V 2011 and Akilisi Pohiva 2015 'the only non-frontal heads in the cast'. The per-portrait notes also record a head turn or sideways gaze for Fakafanua 2013, Sevele 2008 and both Vaipulu portraits, so that wording is overstated. The two named portraits remain the most strongly turned.
  - Lavemaau/Eke ties: the cast pass says both wear 'a blue tie'. The per-portrait notes and a comparison crop viewed while assembling this packet show Lavemaau's tie has grey and navy diagonal stripes, while Eke's is bright blue with a pattern. The pair's moderate-high confusion rating still stands: same navy suit, light-blue shirt and grey hair.
  - Lasike tie: the cast pass says 'orange striped', the per-portrait note says 'navy/gold diagonal-striped'. The same crop shows navy with orange-gold stripes; both descriptions fit.
  - Outline weight: the cast pass describes the historical cast as having 'thin, broken outlines', while many per-portrait notes say 'bold dark outline'. Read together, the historical portraits do have a contour, but it is lighter and more painterly than the anchor's and the fictional portraits'.
  - Fa'otusia suit colour: the per-portrait note says the navy suit was specified by the prompt, while the cast pass says the change from the reference's black suit is not recorded. Both are true: it was a deliberate prompt choice that the era_note does not list as an interpretation.
  - Head proportions, additional cases (fact-check): Tu'iha'angana 2019's style note says '5.5-head adult proportions', which repeats the prompt's figure. A guide-line measurement during the fact-check gave about 6.2 heads for Tu'iha'angana, about 6.1 for Tangi 2008, about 7.4 for George Tupou V 2011, about 7.6 for Tu'i'onetoa, about 5 for Fakafanua 2026 and about 4.4 for the anchor. The cast pass's 'about 6.5-7.5 for all 42 historical portraits' is therefore approximate; the conclusion that Tonga heads are smaller than the anchor's and the prompts' 5.5 is unchanged.
  - Crown Prince 2012 / Latu 2013 ties: the cast pass says both wear 'a striped grey tie'. The per-portrait note and a crop viewed during the fact-check show the Crown Prince's tie is a grey/black check (argyle) pattern, while Latu's is diagonally striped. At 64-96 px both read as grey patterned ties on a charcoal suit, so the high confusion rating stands.

## Fact-check (2026-10-04)

Label: **Claude, automated agent - not human or Codex review.** A second automated pass re-checked this packet. It
viewed both upheld-concern portraits and every pass_with_notes or concern row next to its labelled references,
re-verified every path, every sha256 and every count, and checked the status and labels. No verdict, dimension value,
severity or count changed. Corrections (details in `review.json` → `fact_check` and `errata`):

- The eight no-reference portraits are of eight people, not seven.
- Results-table notes were narrowed where they overgeneralised: George V 1990, George V 2011 (the bald crown was
  requested in the prompt), Tu'iha'angana (exact prompt quote), Tu'ilakepa (gold frames were prompted), Tu'i'onetoa,
  Akilisi Pohiva and Veikune.
- Upheld concern 2: the spectacles are faint at 160 px and not legible at 96 px. Upheld concern 3 now quotes the
  era_note and prompt exactly.
- Overall-rating rule documented, including the five pass_with_notes portraits whose dimensions are all pass or n/a.
- Two reconciliation items and three errata added. `files.json` was regenerated last.

## Limits

- An automated model's likeness judgement is not identity proof. No face recognition was run and no identity was inferred from any face; each cartoon was compared only with the reference photograph(s) the project already labels for that person_id.
- No new references were fetched: the container cannot reach the original sources. Everything here rests on material already in the repository.
- Eight historical portraits of eight people have no in-repo likeness reference (authored_identity); seven of those people have no in-repo reference at all, and Fakafanua has one only for his separate 2013 portrait: Fusitu'a 1998, Cocker 2014, Kavaliku 1998, Veikune 2005, Tangi 2008, Poasi Tei 2021, Lavemaau 2020 and Fakafanua 2026. For these, only style, technical quality and consistency with the recorded text could be checked. The restricted research photographs were not viewed and are not in the repo.
- Fatai Helu (to_fatai_helu) has no art. His required interval, 2022-08-29 to 2022-08-30, remains open and is outside this review.
- The fictional people were checked only against this Tonga cast and the anchor. This review cannot establish that they resemble no real person anywhere.
- Apparent-age readings of simplified cartoons are subjective. Head-proportion and age figures are estimates.
- Attire questions (Lolohea's ta'ovala under the jacket, Matalehu's decorative ta'ovala) need a cultural or attire reviewer; an automated review cannot judge them.
- The external Codex cast gallery (D:/...) and its screenshots could not be re-inspected. The game UI was not run. Card-size checks are Pillow simulations (96x96 squash/contain/cover, 160 px tall, 93x118 .nation-art crop for some), not screenshots.
- Not covered: the post-cutoff exposure audit (open_requirements.historical_future_exposure_audit) and the current use of the retired Salote Tupou III national-representative art.
- The cast-wide findings and per-portrait notes were not adversarially challenged; only the three listed concerns were. Abbreviated-hash typos in three reviewer notes are listed under errata; full hashes were re-verified.
- Scratch paths cited in notes (/tmp/claude-0/...) are ephemeral working files and are not part of this packet.

## Files

| File | Contents |
|---|---|
| `README.md` | This summary |
| `review.json` | Machine-readable results: rubric, scope, per-portrait results with challenges and verdicts, inventory items and notes, cast findings, reconciliation, errata, limits |
| `contact-sheet.png` | Tile 00 is the style anchor, then all 46 reviewed portraits at 160 px tall (01-18 multi-portrait people, 19-42 single portraits, 43-46 fictional; tile numbers are not the results-table numbers). It contains no reference photographs. |
| `files.json` | Bytes and sha256 of every packet file except itself |
