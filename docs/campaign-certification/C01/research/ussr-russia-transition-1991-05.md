# USSR/Russia transition 1991 05: the RSFSR presidency, Belovezha and Alma-Ata, and the end of Gorbachev's presidency

Packet: **CLAUDE-C01-05**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-ussr-05`, base
`04bc99a6` (current integration). Research access: 21 September 2026 (local).
The historical cutoff stays **7 September 2026**; every observation here is dated 1991.

This packet reviews eight observations about the 1991 executive transition, across
[ussr.json](ussr.json) and [russia.json](russia.json). It adds 20 sources (13 in Russia,
7 in USSR) and 37 claims. In Russia it adds one institution, `ru_rsfsr_presidency`,
with two roles (`ru_rsfsr_president`, `ru_rsfsr_vice_president`) and one holder
observation on each. In USSR it adds claims and one dated holder observation to the
existing `su_presidency`/`su_president`, and coverage notes to `su_supreme_soviet`. It
adds no organization, no USSR institution, no mapping between the two packets, no end
date, no game mapping, portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1)
remains open.

The research dossier and an independent check were prepared before this packet was
written. Every checker defect is applied below; where the fix took a stricter form than
the checker proposed, the reason is given (see [Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| SURU-TR91-01 | Which instruments created the RSFSR presidency, when, and in which packet does it belong? | **Accepted:** adoption 24 Apr (1098-I), entry into force from publication (1099-I, day unverified), Congress approval 22 May, Chapter 13-1 on 24 May; filed in Russia through the 2094-I renaming; `lifecycle.from` null |
| SURU-TR91-02 | Who won the first RSFSR presidential election, and how are the vote and the result dated? | **Accepted:** candidates noted 22 May; voting 12 Jun; result approved 19 Jun (CEC 20-25); publication date unknown; no holder starts here |
| SURU-TR91-03 | When did Yeltsin take the oath and assume the presidency? | **Accepted:** oath at the solemn session of 10 Jul; holders Yeltsin and Rutskoi from 1991-07-10, no end |
| SURU-TR91-04 | What do the 8 December Minsk (Belovezha) records establish? | **Accepted** as signed texts; RSFSR ratification 12 Dec (2014-I) kept separate; not an act of a USSR organ |
| SURU-TR91-05 | What do the 21 December Alma-Ata records establish? | **Accepted** from the UN English and Russian issues; the Russian Declaration is in the present tense; no USSR office addressed |
| SURU-TR91-06 | When and how did Gorbachev cease to act as USSR President? | **Accepted** for the day only: an announcement on 25 Dec, on two US primary records; no `until` on the holder; the memoir excerpt and the decree transcription are leads |
| SURU-TR91-07 | Is there a sourced end date for the USSR Presidency as an institution? | **Unresolved:** no abolishing instrument reviewed; `su_presidency.lifecycle.until` stays null |
| SURU-TR91-08 | What did the Council of Republics adopt on 26 December? | **Unresolved:** only a non-official transcription reviewed; lead, no claim, no lifecycle change |

### Date ledger

Each row is a separate dated fact with its own claim or field. Adoption, entry into
force, approval, amendment, voting, result approval, oath, signature, ratification,
renaming and cessation are not merged, and different dates for them are not
inconsistencies.

`attested_on` is the date of the act or state the source dates (for example, 12 June
for the vote reported in the undated CEC communication). The date a document was issued
or circulated is `published_date` or `document_date`. Holder `from` is used only where a
source states when office began.

| Date (1991) | Event | Claim or field |
|---|---|---|
| 17 Mar | RSFSR referendum, cited in the preamble of Law 1098-I (result not reviewed) | text of `ru_rsfsr_president_office_created_19910424` only |
| 24 Apr | Supreme Soviet adopts Law 1098-I; Yeltsin signs as Chairman | `ru_rsfsr_president_office_created_19910424` and three rule and signature claims |
| 24 Apr | Resolution 1099-I puts 1098-I into force from publication | `ru_rsfsr_president_law_in_force_on_publication_19910424` |
| unknown (portal card: *Rossiyskaya Gazeta*, 27 Apr) | Publication that brings 1098-I into force | none; `lifecycle.note`, SURU-TR91-01 |
| 22 May | Congress approves the law; six registered candidates noted | `ru_rsfsr_president_law_congress_approval_19910522`; `ru_rsfsr_candidates_registered_19910522` |
| 24 May | Chapter 13-1 written into the RSFSR Constitution (1326-I) | `ru_rsfsr_constitution_ch13_1_19910524` |
| 12 Jun | Voting | `ru_cec_election_voting_19910612` |
| 19 Jun | CEC resolution 20-25 approves the communication: figures; Yeltsin elected, Rutskoi considered elected | `ru_cec_resolution_20_25_19910619`, `ru_cec_communication_results_19910619`, `ru_cec_yeltsin_rutskoi_elected_19910619` |
| unknown | TASS publication of the communication (the "official announcement") | none; SURU-TR91-02 |
| 27 Jun | Law 1494-I on assuming office, in force on adoption; Yeltsin still signs as Chairman | `ru_rsfsr_inauguration_law_rules_19910627`; `ru_yeltsin_ss_chair_signature_19910627` |
| 10 Jul (session from 10:00; minute not printed) | Solemn session; Yeltsin's oath; both take office | `ru_steno_session_19910710`, `ru_steno_oath_19910710`; both holders `from` |
| 10 Jul | Yeltsin released as Chairman (1595-I); Rutskoi released as member (1596-I) | `ru_res_1595i_yeltsin_release_19910710`, `ru_res_1596i_rutskoi_release_19910710` |
| 8 Dec | Minsk Declaration and Agreement signed | `su_minsk_declaration_19911208`, `su_minsk_agreement_ussr_statement_19911208`, `su_minsk_agreement_art11_14_19911208`, `su_garf_belovezha_copy_19911208` |
| 12 Dec | RSFSR Supreme Soviet ratifies the Agreement (2014-I) | `su_rsfsr_minsk_ratification_19911212`, `su_garf_res_2014i_original_19911212` |
| 13 Dec | A/46/771 circulated (covering letter of 12 Dec) | `published_date` of `su_un_a46_771_minsk_19911208` |
| 21 Dec | Alma-Ata Protocol, Declaration and Annex IV | `su_almaata_protocol_19911221`, `su_almaata_declaration_19911221`, `su_almaata_coordinating_bodies_19911221` |
| 25 Dec, 10:03-10:25 a.m. (US time) | Telcon: Gorbachev's statement planned, decrees to take effect on announcement | `su_telcon_*`; `su_president` holder `attested_on` |
| 25 Dec | Law 2094-I renames the RSFSR, in force on adoption | `ru_law_2094i_rename_19911225`; signature claims `ru_law_2094i_published_signature_19911225`, `ru_garf_law_2094i_original_19911225` |
| 25 Dec, 9 p.m. (US time) | Bush: Gorbachev's decision to resign "today" | `su_bush_address_resign_19911225` |
| 26 Dec | Council of Republics declaration 142-N | lead only; SURU-TR91-08 |
| 30 Dec | A/47/60 circulated (covering letter of 27 Dec) | `published_date` of `su_un_a47_60_almaata_19911221` |
| unknown | Effective moment of Gorbachev's cessation; end of the USSR Presidency | SURU-TR91-06 and 07 |

### Identifier changes from the dossier

The dossier's observations C01-07-OBS-01 to 08 are SURU-TR91-01 to 08 here, in the same
order. Its proposed institution and role IDs (`ru_rsfsr_presidency`, `ru_rsfsr_president`,
`ru_rsfsr_vice_president`) are used unchanged. Existing USSR IDs are reused. Claim changes:

| Dossier | This packet | Why |
|---|---|---|
| `ru_cec_communication_results_19910612` | `ru_cec_communication_results_19910619` plus a new `ru_cec_election_voting_19910612` | The ID carried the voting day while the claim was attested on the approval day (defect D13). Voting and result approval are now separate claims |
| `ru_law_2094i_rename_19911225` | the same ID for the renaming, plus `ru_law_2094i_published_signature_19911225` and `ru_garf_law_2094i_original_19911225` | The published and signed signature titles conflict (D3) |
| `su_steno_gorbachev_confirms_oath_19910710` | `ru_steno_gorbachev_confirms_oath_19910710` | The stenogram is an RSFSR record in the Russia packet; its "President of the USSR" heading is folded into this claim |
| `su_steno_gorbachev_lukyanov_titles_19910710` | not imported | A USSR office observation inside a Russia-packet source; importing it on `su_president` would need the same record in both packets. The 25 December telcon carries the 1991 USSR observation |
| `su_un_a46_771_transmittal_19911212`, `su_un_a47_60_transmittal_19911227` | source `published_date`, `document_date` and `scope_note` | Transmission metadata, not an event of either state |
| `su_almaata_un_membership_decision_19911221`, `su_almaata_minutes_armed_forces_19911221`, `su_bush_address_recognition_19911225` | not imported (recorded in source scope notes) | Verified, but they concern no institution in either packet; UN-membership continuity and diplomatic recognition must not become a USSR-to-Russia mapping |
| `su_gorbachev_address_cessation_19911225`, `su_ved52_*` | leads | Secondary edition (D1) and non-official transcription (D2) |

## Observations

### SURU-TR91-01 — Creation of the RSFSR presidency

Evidence, all from the official legal portal (original editions) unless noted:

- Law 1098-I of 24 April 1991 (`ru_rsfsr_law_1098i_19910424`). Citing the first RSFSR
  referendum of 17 March 1991, the Supreme Soviet makes the President "the highest
  official of the RSFSR and the head of executive power". The President serves five
  years, at most two consecutive terms. A Vice-President is elected jointly and takes
  the powers on a vacancy. It is signed by Yeltsin as Supreme Soviet Chairman.
- GARF original (Rosarkhiv exhibit 09-36, `ru_garf_exhibit_law_1098i`): the typescript with
  Yeltsin's autograph, F. 10026, Op. 1, D. 480, L. 75-79. Previews of leaves 75 and 79
  were viewed and agree with the portal text.
- Resolution 1099-I of the same day (`ru_rsfsr_res_1099i_19910424`) puts the law into force
  "from the moment of publication" and sends it to the Congress.
- Congress resolution 1324-I of 22 May approves the law (`ru_rsfsr_res_1324i_19910522`).
- Law 1326-I of 24 May inserts Chapter 13-1 (Articles 121-1 to 121-11) into the RSFSR
  Constitution (`ru_rsfsr_law_1326i_19910524`).
- Law 2094-I of 25 December renames the state RSFSR the Russian Federation (Russia), in
  force on adoption (`ru_rsfsr_law_2094i_19911225`). The GARF original (exhibit 10-04,
  `ru_garf_exhibit_law_2094i`) is signed "President of the RSFSR"; the published text
  prints "President of the Russian Federation".

Decision: accepted. A new Russia institution `ru_rsfsr_presidency` is added; `russia.json`
had no presidency. It is filed in Russia because 2094-I renamed the RSFSR itself. That is
a sourced renaming, not a succession from the USSR, and the signature title is not part
of that argument. The institution's jurisdiction says `automatic_successor_mapping: false`,
and nothing in either packet maps `su_presidency` to it.

`lifecycle.from` is **null**. The only day available, 24 April, is an adoption day; the
law entered into force on a publication day that is not verified. The atlas shows
`lifecycle.from` as the office's start, so any value would be read as the date the office
legally existed. The lifecycle note lists all four dates (adoption, entry into force from
publication, Congress approval, constitutional amendment) as separate claims.

Roles are `institutional_office`. The statute says "highest official and head of executive
power", and no head-of-state or head-of-government classification is inferred.

Limits: the portal card cites *Rossiyskaya Gazeta* of 27 April 1991, but that issue was not
inspected, so 27 April is not recorded. The referendum result was not reviewed from a
primary record. Leaves 76-78 of the GARF original were not viewed. Yeltsin's signatures as
Chairman (24 April, 27 June) are claims only; the packet has no RSFSR Supreme Soviet
institution. The 1993 status of the law is outside this packet.

### SURU-TR91-02 — The first presidential election

Evidence:

- Congress resolution 1325-I of 22 May (`ru_rsfsr_res_1325i_19910522`) notes six registered
  candidates (Bakatin, Yeltsin, Zhirinovsky, Makashov, Ryzhkov, Tuleev) and orders a
  support vote for Zhirinovsky.
- GARF record in Rosarkhiv exhibit 09-37 (`ru_garf_cec_result_19910619`), three facsimile
  images viewed at full size. CEC resolution 20-25 of 19 June approves the communication and
  orders it published through TASS. The communication reports voting on 12 June in 88
  districts: 106,484,518 registered, 79,498,240 voted (74.66 per cent), Yeltsin 45,552,041
  (57.30 per cent). Under Article 15 of the election law it states that Yeltsin "was
  elected" and that Rutskoi "is considered elected" Vice-President.

Decision: accepted, as three separately dated claims: voting on 12 June
(`ru_cec_election_voting_19910612`), determination and approval on 19 June (the resolution,
figures and result claims), and publication on an unknown date. None is a term start.
The communication still describes Yeltsin as Supreme Soviet Chairman.

Limits: the TASS publication date is not given. That publication is the "official
announcement" from which Article 1 of Law 1494-I counts one month. The archival leaf is
unconfirmed: the exhibit captions say L. 6, L. 6 ob. and L. 7, while the handwritten folio
numbers on the images read 5, 6 and 7. Votes against each candidate, ballots with every
name struck out and the two annulled polling stations are not imported.

### SURU-TR91-03 — Oath and assumption of office

Evidence:

- Law 1494-I of 27 June (`ru_rsfsr_law_1494i_19910627`), in force on adoption: the President
  swears an oath at a solemn session of the highest body of state power; "from the moment
  of the oath" the President and the Vice-President are deemed to have taken office and
  given up other posts (Article 3). Yeltsin signs it as Chairman.
- The official stenographic record (`ru_prlib_inauguration_stenogram_19910710`, Presidential
  Library scan). The solemn session met in the Kremlin Palace of Congresses on 10 July 1991
  at 10:00, chaired by Khasbulatov (printed page 3). Yeltsin recites the oath under the
  heading "Yeltsin B. N., President of the Russian Soviet Federative Socialist Republic"
  (printed page 6). He later names Rutskoi "the first Vice-President of Russia" (page 12).
  Gorbachev, given the floor as "President of the USSR", says the act of the oath has just
  taken place (pages 12-13).
- Congress resolutions 1595-I and 1596-I of 10 July (`ru_rsfsr_res_1595i_19910710`,
  `ru_rsfsr_res_1596i_19910710`) cite each man's assumption of office under Article 3. They
  release Yeltsin as Supreme Soviet Chairman and Rutskoi as a member, and end both deputy
  mandates.

Decision: accepted. Two holder observations: Борис Николаевич Ельцин on `ru_rsfsr_president`
and Александр Владимирович Руцкой on `ru_rsfsr_vice_president`, each `from` 1991-07-10,
`until` null, `attested_on` null (so the atlas reads "Reported interval: 1991-07-10 → Not
established"). Yeltsin's holder cites 1494-I Article 3, the oath and 1595-I; Rutskoi's
cites 1494-I Article 3 and 1596-I. Neither cites the election or the result.

Limits: day precision only; the session opened at 10:00 and the oath minute is not
printed. No end or term is recorded, and the five-year rule is not used to infer one; the
August 1991 events and everything later are outside this packet. Only 20 of the
stenogram's 216 pages are public.

### SURU-TR91-04 — The Minsk (Belovezha) records of 8 December

Evidence:

- UN document A/46/771 (`su_un_a46_771_minsk_19911208`), the English translation circulated
  on 13 December. Annex I proclaims the Commonwealth; Yeltsin signs as "President of the
  RSFSR". Annex II: the three republics, as founder states of the USSR, declare that the
  USSR as a subject of international law and a geopolitical reality "no longer exists".
  Article 11 bars applying former-USSR law from signature; Article 14 ends the activity of
  former-USSR bodies in member territories.
- GARF photocopy with autograph signatures (exhibit 10-12, `su_garf_exhibit_belovezha_copy`),
  leaves 1 and 5. The Russian preamble reads "прекращает свое существование" (ceases its
  existence).
- RSFSR Supreme Soviet resolution 2014-I of 12 December 1991 (`su_rsfsr_res_2014i_19911212`,
  portal text) and its GARF original (exhibit 10-03, `su_garf_exhibit_res_2014i`, signed by
  Khasbulatov): the RSFSR ratifies the Agreement. Point 2 keeps former-USSR norms in force
  in the RSFSR, pending RSFSR laws, where they do not conflict with the RSFSR Constitution,
  RSFSR legislation and the Agreement.

Decision: accepted as signed texts, with ratification separate from signature. The
2014-I resolution is an RSFSR act. It is filed in the USSR packet because it dates the
Agreement's effect for one party and qualifies Article 11; wherever Article 11 is cited,
that qualification is recorded. These are claims on `su_presidency` as context. They are
not acts of a USSR organ and do not end the office.

Limits: the Belarus and Ukraine ratifications were not reviewed. 2014-I gives no
entry-into-force date of its own. The Alma-Ata rule that the Agreement binds each party
from ratification was signed on 21 December, after this ratification. The authentic
signed copies of the Agreement were not reviewed.

### SURU-TR91-05 — The Alma-Ata records of 21 December

Evidence: UN document A/47/60-S/23329 (`su_un_a47_60_almaata_19911221`), circulated on
30 December; the English issue plus the Russian-language issue of the same symbol.

- Annex I, the Protocol: eleven states constitute the Commonwealth; Georgia is absent. The
  Agreement enters into force for each party from ratification. Yeltsin signs for the
  "Russian Federation (RSFSR)".
- Annex II, the Declaration: the UN English reads "the Union of Soviet Socialist Republics
  ceased to exist"; the Russian issue reads "прекращает свое существование", in the present
  tense. Yeltsin signs as "Президент Российской Федерации (РСФСР)".
- Annex IV: a Council of Heads of State is created, and proposals on abolishing the
  structures of the former USSR are due by 30 December.

Decision: accepted. The English past tense is not used to date the end of the USSR or of
any office. No text addresses the USSR Presidency or Gorbachev. Annex III (interim command
of the armed forces) and Annex V (support for Russia's continuance of the USSR's UN
membership) were verified but are not imported; UN-membership continuity is not a domestic
institutional mapping.

### SURU-TR91-06 — Gorbachev's cessation

Evidence (US primary records only):

- The White House memorandum of Bush's call with "Mikhail Gorbachev, President of the Soviet
  Union", 25 December 1991, 10:03-10:25 a.m., Camp David (`su_nara_bush_gorbachev_telcon_19911225`).
  Gorbachev says he will speak on Moscow TV in about two hours. He has a decree on his
  resignation on his desk, will resign as Commander-in-Chief and transfer nuclear-use
  authority to the President of the Russian Federation, and will put the decrees into
  effect "as soon as I announce my resignation".
- Bush's address at 9 p.m. that day (`su_bush_address_cis_19911225`): the end of the old
  Soviet Union is "signified today by Mikhail Gorbachev's decision to resign as President".

Decision: accepted **for the day only**. Gorbachev announced his resignation as USSR President on
25 December 1991 (telcon: 'as soon as I announce my resignation'; Bush: 'decision to resign as
President'); that rests on the telcon and the Bush address, and it is
recorded as role claims on `su_president`. A second Gorbachev holder observation is added,
`attested_on` 1991-12-25 (the telcon's title), with `from` and `until` null. **No `until` is
set**: neither record is a Soviet instrument, one gives intent before the act and the other
a characterization after it, and neither states an effective moment. This is not a death,
removal or successor-derived end, and no successor is recorded in this office.

The only Soviet-side text reviewed, the 1000 Schlüsseldokumente edition of the address, is an
excerpt from Gorbachev's 1995 memoir; it is a lead, not a claim. Decree UP-3162, which
speaks of resignation, was seen only in a non-official transcription; it is a lead too.

Limits: the exact effective time, whether a separate decree "on resignation" existed, and a
Soviet primary text of the address remain open. The 1990 election and oath gaps in
`ussr.json` are unchanged.

### SURU-TR91-07 — End of the USSR Presidency as an institution

What the sources say: no reviewed instrument abolishes the office or amends Article 127. The
candidates compete and none is accepted as the end: the Minsk statement (8 December, three
republics), the Alma-Ata Declaration (21 December; present tense in Russian), Gorbachev's
announced cessation (25 December) and the Council of Republics declaration (26 December,
non-official transcription only). The transfer of nuclear-use authority to the President of
the Russian Federation and US recognition of Russia are not institutional succession.

Decision: **unresolved**. `su_presidency.lifecycle.until` stays null, and `su_presidency` is
not mapped to the RSFSR or Russian presidency. The candidates are listed in its coverage.

### SURU-TR91-08 — The Council of Republics on 26 December

What was seen: a non-official transcription of *Vedomosti Verkhovnogo Soveta SSSR* 1991
No. 52. It gives Declaration 142-N of the Council of Republics (26 December, signed by
A. Alimzhanov): with the creation of the Commonwealth, the USSR as a state and subject of
international law ceases to exist. It also gives order 141-N of the Council of the Union
chairman, releasing permanent deputies from 2 January 1992.

Decision: **unresolved**. No claim is imported. These would be acts of single chambers, not
of the whole Supreme Soviet, and `su_supreme_soviet` gains only a coverage note. Accepting
them needs an official host, a gazette facsimile or an archival copy, and the quorum and
vote.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `ru_rsfsr_law_1098i_19910424` | [Law 1098-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102011228&page=1&rdk=0), 24 Apr 1991 | 31,645 bytes, SHA-256 `e49f34e8…e7eb86`, identical on three retrievals (HTTP) |
| `ru_garf_exhibit_law_1098i` | [Rosarkhiv 09-36](https://projects.rusarchives.ru/statehood/09-36-zakon-prezident.shtml) | IA capture 20201130233931, 18,664 bytes, `9ead61c8…d55298`; previews of L. 75 (28,081 bytes) and L. 79 (23,640 bytes) viewed |
| `ru_rsfsr_res_1099i_19910424` | [Resolution 1099-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102011227&page=1&rdk=0), 24 Apr 1991 | 23,820 bytes, `5442288f…aea832`, identical on three retrievals |
| `ru_rsfsr_res_1324i_19910522` | [Resolution 1324-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102011540&page=1&rdk=0), 22 May 1991 | 23,707 bytes, `145bd8bd…496a0b`, identical |
| `ru_rsfsr_res_1325i_19910522` | [Resolution 1325-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102011539&page=1&rdk=0), 22 May 1991 | 25,056 bytes, `3f4310ff…553efe`, identical |
| `ru_rsfsr_law_1326i_19910524` | [Law 1326-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102011559&page=1&rdk=0), 24 May 1991 | 43,600 bytes, `1bf0fdfc…c4f2e4`, identical |
| `ru_garf_cec_result_19910619` | [Rosarkhiv 09-37](https://projects.rusarchives.ru/statehood/09-37-postanovlenie-vybory-prezident.shtml) | IA capture 20230604070106, 15,700 bytes, `54000d3c…19683e`; three facsimiles (98,762 / 133,816 / 114,508 bytes) viewed |
| `ru_rsfsr_law_1494i_19910627` | [Law 1494-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102011894&page=1&rdk=0), 27 Jun 1991 | 25,981 bytes, `c296f806…49e56a`, identical |
| `ru_prlib_inauguration_stenogram_19910710` | [Stenogram of 10 Jul 1991](https://www.prlib.ru/item/375688), Presidential Library | Item page 59,392 bytes, **no hash asserted** (volatile tokens); eight page images with hashes, identical in the dossier and check (image 8 re-fetched for this packet) |
| `ru_rsfsr_res_1595i_19910710` | [Resolution 1595-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012046&page=1&rdk=0), 10 Jul 1991 | 24,555 bytes, `105a436c…a02e84`, identical |
| `ru_rsfsr_res_1596i_19910710` | [Resolution 1596-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012050&page=1&rdk=0), 10 Jul 1991 | 24,498 bytes, `056f17a4…8e013b`, identical |
| `ru_rsfsr_law_2094i_19911225` | [Law 2094-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013769&page=1&rdk=0), 25 Dec 1991 | 20,875 bytes, `18db383a…f49935`, identical |
| `ru_garf_exhibit_law_2094i` | [Rosarkhiv 10-04](https://projects.rusarchives.ru/statehood/10-04-russian-federation.shtml) | Page IA capture 20191104174706, 12,807 bytes, `57573abb…b1abc5`; facsimile IA 20191104174711, 137,825 bytes, `5ef34cca…160830`, viewed |
| `su_un_a46_771_minsk_19911208` | [UN A/46/771](https://documents.un.org/api/symbol/access?s=A/46/771&l=en&t=pdf), 13 Dec 1991 | 389,492 bytes, `3a7726bd…8ccf93`, identical on three retrievals; PDF pages 1, 2, 3, 6 |
| `su_garf_exhibit_belovezha_copy` | [Rosarkhiv 10-12](https://projects.rusarchives.ru/statehood/10-12-soglashenie-sng.shtml) | IA capture 20241112065507, 36,632 bytes, `a5599981…2a1547`; leaves 1 and 5 viewed |
| `su_rsfsr_res_2014i_19911212` | [Resolution 2014-I](https://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013523&page=1&rdk=0), 12 Dec 1991 | 24,748 bytes, `5ec47cbe…2f37c0`, identical on this packet's two retrievals |
| `su_garf_exhibit_res_2014i` | [Rosarkhiv 10-03](https://projects.rusarchives.ru/statehood/10-03-postanovlenie-ratifikaciya-sng.shtml) | Page IA capture 20191207090109, 13,596 bytes, `aab8fb84…0d867a`; facsimile IA 20191207090121, 155,943 bytes, `94792a1d…b7d65b`, viewed |
| `su_un_a47_60_almaata_19911221` | [UN A/47/60](https://documents.un.org/api/symbol/access?s=A/47/60&l=en&t=pdf), 30 Dec 1991 | English 148,826 bytes, `cade387d…4b347a`; Russian issue 1,030,051 bytes, `6c164a21…01cf15`, page 5 read |
| `su_nara_bush_gorbachev_telcon_19911225` | [NARA telcon](https://catalog.archives.gov/medialz/presidential-libraries/bush/gb-nsc/321498139/1991-12-25--Gorbachev.pdf), 25 Dec 1991 | 259,720 bytes, `28e8ee5d…024d3f`, identical on three retrievals; pages 1-4 viewed |
| `su_bush_address_cis_19911225` | [Bush Library Public Papers](https://www.bush41library.gov/digital-research-room/finding-aid/public-papers/address-nation-commonwealth-independent-states) | 28,644 bytes, `8cf7d76e…82d3d6`, identical on three retrievals |

Each source has a checked-in derived factual extract under [sources/](sources/) (thirteen
`russia-*-1991*-facts.json` and seven `ussr-*-1991*-facts.json` files, LF, with their own
checksums in the packets). Original pages, PDFs and images are not checked in, and no
emblem, signature image or photograph is republished.

The official legal portal refused HTTPS from this environment (port 443 timed out), so its
texts and cards were read over HTTP. The register accepts only HTTPS links, so each packet
URL gives the same path on HTTPS; each extract records the HTTP URL actually read. The
Rosarkhiv host refused connections, so its exhibition pages and images were read from
Internet Archive raw captures; each packet URL is the official page and each extract records
the capture read.

## Leads not imported

- **Gorbachev's address of 25 December 1991, 1000 Schlüsseldokumente (BSB)**,
  [permalink oldid=9237](https://www.1000dokumente.de/index.php?title=Fernsehansprache_des_Staatspr%C3%A4sidenten_der_UdSSR_an_die_Sowjetb%C3%BCrger&oldid=9237).
  The Russian text is excerpted from Gorbachev's memoir *Zhizn' i reformy* (1995), vol. 1,
  pp. 5-8, with elisions. It says he is ceasing his activity as USSR President and speaks for
  the last time in that post. A secondary edition, so not a claim (D1). Its page hash is not
  reproducible: two retrievals of 67,751 bytes gave different SHA-256 values, with only
  MediaWiki request tokens differing (D8). The edition's introduction (19:00 broadcast, 19:30
  flag change) is scholarly narrative and is not used either.
- **Non-official transcription of *Vedomosti VS SSSR* 1991 No. 52**, [vedomosti.sssr.su](https://vedomosti.sssr.su/1991/52/)
  (312,345 bytes). It gives decree UP-3162 (25 December; resignation, command and nuclear
  authority), Declaration 142-N of the Council of Republics and order 141-N (26 December), and
  a closing notice that the gazette ceases publication. None is imported (D2); SURU-TR91-06
  to 08 stay open until an official host, gazette facsimile or GARF copy is reviewed.
- **Stenographic bulletin No. 23 of the Council of Republics, 26 December 1991**,
  [sten.sr.vs.sssr.su](https://sten.sr.vs.sssr.su/13/1/23/), a non-official host. It could
  document quorum and vote for 142-N once corroborated.
- The Gorbachev Foundation's text of the address (gorby.ru); Garant, ConsultantPlus and
  Wikisource copies of 142-N and UP-3162: non-governmental, commercial or crowd-sourced.
- The Presidential Library's "day in history" page on Law 1098-I with referendum figures: an
  editorial summary; the referendum protocol itself was not obtained.
- Rosarkhiv exhibits 09-30 (Gorbachev's election by the USSR Congress, 15 March 1990) and
  10-02 (RSFSR denunciation of the 1922 Union Treaty, 12 December 1991): outside the
  questions here. 09-30 bears on the existing 1990 election gap in `ussr.json`.
- The Kremlin's bank of 1991 RSFSR presidential decrees: later office observations, not
  parsed.
- The American Presidency Project mirror of the Bush address: the Bush Library text is used.
- Verified but not imported: Alma-Ata Annexes III and V, Bush's recognition paragraphs, and
  the stenogram chair's greeting of Gorbachev and Lukyanov (see the identifier table).

## Sources attempted

- `https://pravo.gov.ru/…` on port 443: connection timed out; the same paths over HTTP worked.
  The portal's number searches for "УП-3162" and "142-Н" and its listing for 26.12.1991
  returned no USSR acts (dossier).
- `projects.rusarchives.ru` directly: timed out or refused (dossier, check and this packet).
  The full-size images of exhibit 09-36 return 4,688-byte HTTP 404 pages in every capture tried.
- The Internet Archive was intermittently "Temporarily Offline" and rate-limited (HTTP 429)
  during this packet; capture lookups were retried (see D9).
- `bush41library.tamu.edu` telcon copy: redirected to a homepage; the NARA catalog copy is used.
- GovInfo *Public Papers* 1991 book 2, guessed page anchors: "Page Not Found".
- `zakon.rada.gov.ua` guessed identifiers for UP-3162 and 142-N: HTTP 404 or no connection.
- Presidential Library search: JavaScript-only results.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| D1 | SURU-TR91-06 rested partly on the BSB edition, a 1995 memoir excerpt | **Applied**: the claim is not imported and the edition is a lead; the day rests on the NARA telcon and the Bush address, as the report and holder state |
| D2 | An accepted observation listed decree UP-3162 from the non-official transcription | **Applied**: removed; UP-3162, 142-N, 141-N and the issue notes are leads only, named in SURU-TR91-06 to 08 and in `su_presidency`/`su_supreme_soviet` coverage |
| D3 | 2094-I signature conflict; OBS-01 used the published title as evidence | **Applied**: the renaming and the published signature are separate claims; GARF exhibit 10-04 added with the signed "Президент РСФСР"; the filing rests on the renaming alone |
| D4 | Bush address locator | **Applied**: "Paragraphs 5-6 and closing note", counted from "Good evening" |
| D5 | `lifecycle.from` 1991-04-24 risked collapsing adoption into existence | **Applied in a stricter form**: `from` is null rather than 24 April with precision `enactment_day_not_entry_into_force`. The atlas shows `from` as "Start" whatever the precision says, so 24 April would still read as the office's start. The note keeps all four dates, and the entry-into-force day remains open (next work) |
| D6 | Stale note that the 09-36 facsimiles could not be viewed | **Applied**: previews of L. 75 and L. 79 recorded with their capture URLs and hashes, and viewed again. The check gave the preview range as 28,081-32,831 bytes; the L. 79 preview is 23,640 bytes, recorded as measured |
| D7 | Four stenogram image URLs were prose | **Applied**: exact IIP URLs for images 1, 5-9, 14 and 15, from the viewer metadata |
| D8 | BSB and Presidential Library page hashes do not reproduce | **Applied**: the stenogram page asserts no hash (null, with the two observed values and the cause in the extract); the BSB page is a lead and its non-reproducibility is disclosed there |
| D9 | Year-wildcard capture URLs; CEC leaf locator incomplete | **Applied**: see the capture note below. The CEC resolution claim now records the captions L. 6 / 6 ob. / 7 against the folios 5 / 6 / 7, and says the archival locator is unconfirmed |
| D10 | "Russian original not reviewed" for the Alma-Ata Declaration | **Applied**: the Russian wording and the Russian-issue PDF (with hash) are in the claim and extract; the English past tense is not used to date anything |
| D11 | RSFSR ratification left open although official sources exist | **Applied**: 2014-I added from the portal (downloaded for this packet) and GARF exhibit 10-03; dated 12 December, separate from signature; the point-2 qualification is noted on Article 11. Belarus and Ukraine remain open |
| D12 | 1099-I gazette citation incomplete | **Applied**: No. 17, art. 513 |
| D13 | CEC claim ID carried the voting day | **Applied**: renamed to `ru_cec_communication_results_19910619`, with a separate voting claim `ru_cec_election_voting_19910612` |

Capture note (D9). The dossier's facsimile URLs used year wildcards (`2023id_`, `2024id_`).
Each was requested again on 21 September 2026 and the Internet Archive's redirect recorded:
the CEC images resolve to 20191208064007, 20191208064008 and 20191208064005, and Belovezha
leaves 1, 4 and 5 to 20220401204228, 20220401204302 and 20220401120715. Every resolved
capture was re-downloaded and matches its recorded hash. The check did not record which
captures of the 10-03 and 10-04 exhibition pages it read; the captures nearest their
facsimiles (20191207090109 and 20191104174706) return identical bytes and are recorded. The
extracts now carry only fixed 14-digit capture timestamps.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-USSR-TR91-001`: a Soviet primary record of the 25 December address and its broadcast
  time (*Izvestiya* or *Pravda* of 26 December 1991, TASS, a Gosteleradio log, or an archival
  typescript). Resolves the Soviet side of SURU-TR91-06.
- `C01-USSR-TR91-002`: an official facsimile of *Vedomosti VS SSSR* 1991 No. 52 (printed pages
  2058-2060) or GARF copies of UP-3162, 142-N and 141-N, with the Council of Republics
  stenographic bulletin (quorum and vote). Needed for SURU-TR91-06 to 08.
- `C01-Russia-TR91-003`: *Rossiyskaya Gazeta* of 27 April 1991 and the RSFSR protocol of the
  17 March 1991 referendum question on the presidency. Could give `lifecycle.from` an
  entry-into-force day (SURU-TR91-01).
- `C01-Russia-TR91-004`: the TASS or press publication of the CEC communication (SURU-TR91-02).
- `C01-USSR-TR91-005`: the Belarus and Ukraine ratification instruments for the Minsk
  Agreement, and any deposit record (SURU-TR91-04).
- `C01-USSR-TR91-006`: Gorbachev's 1990 election and oath (Rosarkhiv 09-30 and the Congress
  stenogram), the existing `ussr.json` gap.
- An integrator decision on whether to add RSFSR Supreme Soviet and Congress institutions, so
  that Yeltsin's chairmanship (signatures of 24 April and 27 June, release on 10 July) can be
  recorded as holder observations.

## Integration notes (outside this packet's file boundary)

- `research-index.json` is regenerated in a **separate commit**. New totals: 88 sources and
  1,659 claims (previously 68 and 1,622); 28 institution observations (previously 27).
  Organization, packet and batch counts are unchanged. USSR keeps one open batch of four
  members, now with 21 claims. Russia's role observations go from 5 to 7 and its second
  batch, `C01-Russia-DISC-B002`, goes from nine members to ten (`ru_rsfsr_presidency`).
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left
  for the integrator. This handoff is self-proposed and not yet registered there.
- Existing tests updated, none loosened:
  - `test_ussr_research_s10h.py`: totals 3→10 sources and 8→21 claims. The 1990 extracts must
    still record no PDF page, and the set of extracts with PDF pages is pinned to the two UN
    documents and the NARA telcon. The 1990 sources keep their 13 September access date, and
    the access-date set is pinned to exactly two values.
  - `test_russia_research_s10h.py`: totals 2→15 sources, 24→48 claims, 19→20 entries, 5→7
    roles, 5→6 institutions and batches [10, 9]→[10, 10]. The institution kinds and source
    hosts are pinned to the new exact sets. The faction checks, which looped over every
    institution, now loop over the five factions through a helper that also asserts the only
    other institution is `ru_rsfsr_presidency`. Access dates are pinned per source.
- In the atlas, Russia gains "Presidency of the RSFSR (Президент РСФСР)" with two offices, each
  "Reported interval: 1991-07-10 → Not established". The USSR Presidency shows a second
  Gorbachev observation, "Observed on 1991-12-25". No UI code changed; the Node checks pass,
  and no browser review was run.
- The sparse worktree lacks `spheres-sim/data`, so `test_campaign_census.py` was not run; the
  worktree was not widened.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr_*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia_*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

All passed on 21 September 2026: the exact index regeneration and check (88 sources, 1,659
claims); 79 research tests; 18 USSR tests (8 of them new) and 10 Russia tests; 28 Tonga tests;
11 atlas Node tests; the workboard check (44 markers); `git diff --check`, including the new
files once staged. `test_campaign_census.py` errors in its setup because the sparse worktree has
no `spheres-sim/data`; that is expected and was not widened. The new test was also run against
five hand-made regressions (an end date on Gorbachev's 1991 holder, 24 April as the presidency's
start, the vote re-dated to the approval day, an inferred Yeltsin end, and the result used as
Yeltsin's start); it failed on each.
