# CLAUDE-C06-SAUDIARABIA-01: Saudi Arabia cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **awaiting Codex render** (2 October 2026; batch 1 prepared; not ready_for_review, not complete). Parent: C06 (Saudi Arabia country cast),
with C03 cartoon production for these windows. Pending Codex registration and acceptance.

Origin: on 1 October 2026, after the France, Japan, Brazil and South Africa batches were prepared
(`CLAUDE-C06-FRANCE-01`, `CLAUDE-C06-JAPAN-01`, `CLAUDE-C06-BRAZIL-01`, `CLAUDE-C06-SOUTHAFRICA-01`), the user asked to
continue the cast work in parallel. The CP1 plan (`CP1-ACCELERATION.md`) allows parallel artwork for another
already-reviewed batch while Codex owns Tonga (`CLAUDE-C06-TONGA-01`). Generator route, as for France: **Codex renders**
with its built-in image tool. Claude prepares the identity review, dated likeness references, exact prompts and input
order, then reviews and registers the outputs Codex returns. `person_art_pipeline.py` requires the generator "OpenAI
built-in image_gen", and Claude has no such tool, so Claude never generates, edits or labels artwork itself.

Branch: `claude/c06-sa-01`. Base: `2fd186d6` (current `codex/campaign-certification`); not stacked on the France, Japan,
Brazil or South Africa branch. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Four portraits of two people who already have registry IDs in `spheres-sim/data/party_leaders.json`, accepted C01
research and no art for these windows. No reserve: the registry holds no third Saudi person ID (see below). The accepted
research is C01-45 (kings, crown princes) and C01-50 (prime ministers) in
`docs/campaign-certification/C01/research/saudi-arabia.json`. Every accepted holder observation of each person falls
inside one of that person's windows below. The `to` dates are exclusive. A window is an appearance interval, not a
tenure, and grouping observations under a person ID proves no term.

| person_id | requested window | accepted observations (attested_on) | window basis |
|---|---|---|---|
| fahd_bin_abdulaziz_al_saud | 1991-01-01 to 1996-01-01 | none inside (his only other observation, King 1990-08-08, is C01-06 and acceptance pending; it lies inside his existing 1990 art) | **needs Codex agreement**: he is the country card's leader by `office_links` (since 1982-06-13), and this window continues his existing 1990 art (1990-01-01 to 1991-01-01). It ends at the accepted 1 January 1996 decree delegating state affairs to the Crown Prince "while we enjoy rest and recuperation" (C01-45 claim `sa_fahd_delegation_19960101`), used only as an appearance boundary |
| fahd_bin_abdulaziz_al_saud | 1996-01-01 to 2005-08-01 | C01-45 King: 1996-01-01, 1996-02-12, 2005-07-31; C01-50 PM: 2004-10-03 | from the first accepted observation; ends at the Royal Court statement of 1 August 2005 announcing his death (claim `sa_fahd_death_announced_20050801`). That is an appearance boundary only, not a reign end |
| abdullah_bin_abdulaziz_al_saud | 1996-01-01 to 2005-08-01 | C01-45 Crown Prince: 1996-01-01, 2005-07-30 | from the first accepted observation; ends where his King window starts. Codex may move the start to 1990-01-01 on the seed heir record (`leaders_1990.json`: Crown Prince and First Deputy Prime Minister since 1982-06-13; not C01 research) |
| abdullah_bin_abdulaziz_al_saud | 2005-08-01 to 2015-01-23 | C01-50 PM: 2005-08-01, 2007-03-22; C01-45 King: 2006-10-20, 2014-05-20 | from the accepted 1 August 2005 PM observation; ends at the Royal Court statement of 23 January 2015 that he died at 1 a.m. that day (claim `sa_abdullah_death_20150123`; appearance only) |

The C01-06 named holder objects (Fahd as King 1990-08-08; Abdullah as King and as Crown Prince 2005-08-01) remain
`c01_integrated_pending` and are not used as anchors. They still fall inside the windows above or inside Fahd's
existing 1990 art. C01-50's three withheld meeting-only uses (`sa_fahd_pm_chairs_cabinet_19960304`,
`sa_fahd_pm_chairs_cabinet_20050425`, `sa_abdullah_pm_chairs_cabinet_20121229`) stay claims only.

Production job IDs these windows close. Window-scoped IDs come from `person_art_pipeline.py jobs --from <from> --to <to>`.
Default-inventory IDs come from `jobs` over 1990-01-01 to 2027-01-01 and are closed only for the requested windows.
Both were written to `D:/spheres-scratch/sa-cast/`, never into the repository. The inventory file `jobs.json` has
sha256 `5f83d33def208c44f4a7a31e2b7f4ff5a8b3f15cf283abefba3455b7a87041ec` (registry digest `baa3d822beff...`, manifest
digest `41db40b69ded...`, as France's). `docs/campaign-certification/C01/work-orders.json` has no Saudi Arabia work
order.

| person_id | window | pipeline job, window-scoped | default-inventory job (partly closed) | leadership-production cartoon jobs closed |
|---|---|---|---|---|
| fahd_bin_abdulaziz_al_saud | 1991-01-01 to 1996-01-01 | `fahd_bin_abdulaziz_al_saud-912f7750444a` | `fahd_bin_abdulaziz_al_saud-b847f208ede0` | none exist |
| fahd_bin_abdulaziz_al_saud | 1996-01-01 to 2005-08-01 | `fahd_bin_abdulaziz_al_saud-23affa4e0bcb` | `fahd_bin_abdulaziz_al_saud-b847f208ede0` | none exist |
| abdullah_bin_abdulaziz_al_saud | 1996-01-01 to 2005-08-01 | `abdullah_bin_abdulaziz_al_saud-236612642866` | `abdullah_bin_abdulaziz_al_saud-c6fae3497c05` | none exist |
| abdullah_bin_abdulaziz_al_saud | 2005-08-01 to 2015-01-23 | `abdullah_bin_abdulaziz_al_saud-a0b3ce91e5c0` | `abdullah_bin_abdulaziz_al_saud-c6fae3497c05` | none exist |

Leadership production (`leadership_production_2035.json`) has no open cartoon job for either person. Fahd's only
required window (1990-01-01 to 1990-01-02, the 1990 executive observation) is covered, so his status is
`known_windows_covered`. Abdullah is `eligibility_research_required` with no required window. These windows therefore
close pipeline jobs only.

Alternatives and residuals, computed the same way:
- Fahd as one window, 1991-01-01 to 2005-08-01: `fahd_bin_abdulaziz_al_saud-f17eedfe6634`.
- Abdullah from 1990-01-01 to 2005-08-01: `abdullah_bin_abdulaziz_al_saud-0ae5f49261bc`.
- Still open after registration:
  - Fahd 2005-08-01 to 2027-01-01: `fahd_bin_abdulaziz_al_saud-d3ec1b0b9097`.
  - Abdullah 1990-01-01 to 1996-01-01: `abdullah_bin_abdulaziz_al_saud-0aa00159df24`.
  - Abdullah 2015-01-23 to 2027-01-01: `abdullah_bin_abdulaziz_al_saud-9bb74634df25`.

Fahd's split at 1996-01-01 is for appearance: the accepted decree records his withdrawal for rest. Long-standing
reporting that he had a stroke in late 1995 and was visibly frail afterwards is general biographical knowledge, not C01
research. Codex may rule one window instead.

### Why two people, and not six to eight

- `party_leaders.json` holds exactly two Saudi person IDs: `fahd_bin_abdulaziz_al_saud` and
  `abdullah_bin_abdulaziz_al_saud`. It has one Saudi `office_links` row (Fahd, since 1982-06-13) and no Saudi party rows.
  Leadership production agrees: 0 party rows and 0 institutional candidates. This task does not edit the registry.
- `campaign_leader` (`spheres-web/src/person_portraits.rs`) resolves the saved executive by the `since` of its office
  link. Later incumbents are produced by play, never scheduled by date. So Fahd after 1990 is the card's first need.
  Abdullah is the seed heir.
- The C01 gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.json`, last refreshed at `1fffb983`) classes
  Saudi Arabia's 78 holder observations as 62 `c01_accepted` (C01-25, C01-45, C01-50) and 16
  `c01_integrated_pending` (C01-06).
- These people have accepted observations but no registry person ID: Salman bin Abdulaziz (C01-50 PM 2015-01-23;
  C01-45 King and Crown Prince observations), Mohammed bin Salman (C01-50 PM 2022-09-27; C01-45 Crown Prince),
  Sultan, Nayef and Muqrin bin Abdulaziz and Mohammed bin Nayef (C01-45 Crown Prince), the Shura chairs Muhammad bin
  Jubair, Saleh bin Humaid and Abdullah Al Al-Sheikh, and the Allegiance Commission chair Mishaal bin Abdulaziz
  (C01-25), and its secretary general Khalid Al-Tuwaijri (C01-45). They are batch-2 candidates once Codex adds registry
  identities. Salman, the reigning king since the 23 January 2015 pledge, and Mohammed bin Salman come first.
- The six C01 organizations (opposition and rights groups) record no holders.

Planner's photograph leads. These come from Commons API metadata only; nothing was downloaded or verified, and
preparation must confirm each one:

- Fahd, 1991-1995:
  - George H. W. Bush Presidential Library, "President George H. W. Bush meets with Saudi Arabian King Fahd in
    Riyadh", dated 1992-12-31, Public domain, 600x410. Fahd shares the frame with Bush.
  - If nothing closer qualifies, the existing `fahd-1990-reference.jpg` (21 November 1990, PDM-1.0) may be reused
    unchanged.
- Fahd, 1996-2005:
  - Helene C. Stikkel (DoD), "Defense.gov News Photo of King Fahd (cropped)", 1998-10-13, Public domain, 681x866.
  - David Bohrer (White House), "Dick Cheney and King Fahd of Saudi Arabia", 2002-03-16, Public domain, 397x263.
    Identity must be checked: Commons has a Cheney and Abdullah file from the same day.
- Abdullah as Crown Prince:
  - US Treasury, "Secretary of the Treasury Robert Rubin meets with the Saudi Crown Prince", 1998-09-25, Public
    domain, 1500x1197.
  - Japan State Guest House, Obuchi and Abdullah at Akasaka Palace, 1998-10-21, CC BY 4.0 on Commons. The licence and
    provenance must be checked.
  - Tina Hager (White House), "Saudi Crown Prince Abdullah and George W. Bush", 2002-04-25, Public domain.
- Abdullah as King:
  - Cherie A. Thurlby (DoD), "King Abdullah bin Abdul al-Saud January 2007", 2007-01-17, Public domain, 648x848.
  - White House photographs of Cheney with King Abdullah at Tabuk, 12 May 2007, Public domain.
  - State Department photographs of Kerry with King Abdullah, 2014, Public domain.
- Rejected on sight:
  - files credited to the Saudi Press Agency, even when tagged CC BY-SA 4.0;
  - GODL-India files (Republic Day 2006);
  - out-of-window portraits from 1982 and 1985.

If no qualifying photograph exists for a window, that window is recorded as not ready; no face is invented and no other
person substitutes.

Claude delivers:
- `docs/campaign-certification/C06/production/saudi-arabia/identity-review-batch-01.json`, a proposal in the shape of
  France's identity review, with the accepted holder dictionaries and claim-only observations copied exactly;
- dated likeness photographs of the exact person, copied byte-identical into
  `spheres-web/ui/person-portraits/references/<slug>-<yyyy>-reference-v1.<ext>` (slugs `fahd-bin-abdulaziz` and
  `abdullah-bin-abdulaziz`, matching Fahd's existing art; yyyy is the photograph's year). Licence strings come from
  `FREE_LICENSES` only. Ported licences, NC/ND terms, agency or press photos (including SPA and Royal Court releases),
  government licences not on the list, TV or video stills and non-free files are rejected;
- exact LF prompts in `tools/avatars/person-prompts/`, in the shape of the France prompts (style anchor
  `margaret-thatcher-cartoon-1990-v3.png` as image 1, STYLE ONLY; the identity reference as image 2). They are
  `fahd-bin-abdulaziz-cartoon-1991-v1.txt`, `fahd-bin-abdulaziz-cartoon-1996-v1.txt`,
  `abdullah-bin-abdulaziz-cartoon-1996-v1.txt` and `abdullah-bin-abdulaziz-cartoon-2005-v1.txt`. Each prompt discloses
  any age gap between the photograph and the window and describes the actual dress honestly. It adds no insignia,
  flags, text or extra people;
- a render request for Codex: per job, the prompt file, the input order and the output size (1024x1536 RGB);
- after Codex returns the unchanged outputs: the visual review, the batch generation record, the `person_portraits.json`
  records and the registration receipt.

The registry gives no birth date for either person (`leaders_1990.json` leaves Fahd's null because sources disagree).
Any age in a prompt is labelled general biographical knowledge, not C01 research; the registry is not edited.

Codex decisions requested at claim:
- the Fahd 1991-1996 window and its basis;
- the split at 1996-01-01, or the single window instead;
- Abdullah's start: 1996-01-01, or 1990-01-01 on the seed heir record;
- registry identities for the accepted holders listed above, for batch 2.

Any reviewer string names the actual reviewer. No human approval is claimed. Country sign-off (CS-SaudiArabia) stays
with the user and Codex.

## Allowed files and checks

Allowed files:
- this record;
- `docs/campaign-certification/C06/production/saudi-arabia/`;
- `spheres-web/ui/person-portraits/references/` (new Saudi Arabia reference files only);
- `tools/avatars/person-prompts/` (new Saudi Arabia prompt and batch files only);
- after rendering, additive `people[pid].portraits[]` records in `spheres-web/data/person_portraits.json` and the new PNG
  files, coordinated with Codex.

Not touched:
- `party_leaders.json` and `leaders_1990.json`;
- existing portrait records, including Fahd's 1990 art, `fahd-1990-reference.jpg`, its prompt and
  `opening-1990-batch-v1.json`;
- the pipeline code, `.gitattributes` and shared runtime;
- the task queue and the workboard;
- the France, Japan, Brazil and South Africa branches.

Regenerated shared outputs are left to Codex's integration. Original source metadata and image bytes stay outside Git
under `D:/spheres-scratch/sa-cast/refs/<person_id>/`, pinned by sha256. Response-header captures are not kept.

Inputs at claim (sha256 of the committed bytes at `2fd186d6`):

| path | sha256 |
|---|---|
| `docs/campaign-certification/C01/research/saudi-arabia.json` | `731f2c47c30aaae1dc36ae5427976d7700ae02dc1fa2b12635755aad277b5bb1` |
| `docs/campaign-certification/C01/gap-ledger/ledger.json` | `a7b3c1f1924394743cfebfc1fc7cac7ee24970f40451616273f91aafa374962b` |
| `docs/campaign-certification/C01/reviews/CLAUDE-C01-45-20261001/README.md` | `7337fa6e5526231c81b2a4a11ed79ab5d7ad553b1aed0592ebf791dcf129f05f` |
| `docs/campaign-certification/C01/reviews/CLAUDE-C01-45-followup-20261001/README.md` | `21ebdd83881e29f7534ee2c20414e61049157b9ba757bf290fa8e2e67f263eb3` |
| `docs/campaign-certification/C01/reviews/CLAUDE-C01-50-20261001/README.md` | `3d45ec8dc8e4ffbb6df307e9d88ba23b51e45f8f223f792dfa35985187d0587e` |
| `spheres-sim/data/party_leaders.json` | `b330e2c49fa14a615bcb50fe7e5c6b240bd6678076869699788fdd36f6fdf077` |
| `spheres-sim/data/leaders_1990.json` | `cdc868e9c60ad7df6ca92d4d8a30e1b355cab953df46108f1328d7dc6ab46008` |
| `spheres-web/data/person_portraits.json` | `45fb29edd8b2b3499f47593376fe009db9a6da329d2d79e061128d2d0e333825` |
| `spheres-web/data/leadership_production_2035.json` | `42bdb24bf47cd31367b25069ca2a2381ebff44b9d5dcc55d884550777b8df7c4` |
| `tools/avatars/person_art_pipeline.py` | `b9715a5e841c7e8683bc3ec60a26754244b62d755150f06ebc0911be171487ab` |

Checks: `person_art_pipeline.py self-test` and `validate`; `cartoon_review.py --check`; `leadership_production.py check`;
`campaign_census.py --check`; `python -m unittest discover -s tools/avatars`; `workboard.py --check`; `git diff --check`.

## Preparation

Batch 1 (`SA-CAST-B01`) was prepared on 2 October 2026. Each of the four portrait windows had one preparation agent
and one independent verification agent; Claude assembled the results and fixed the verifiers' mechanical findings. No
image was generated, edited or labelled. The next step is Codex's render:
[render request](../../campaign-certification/C06/production/saudi-arabia/render-request-batch-01.md). The
[identity review](../../campaign-certification/C06/production/saudi-arabia/identity-review-batch-01.json) and
[README](../../campaign-certification/C06/production/saudi-arabia/README.md) carry the observations, references and job
IDs.

In (four primary renders; every photograph is exactly dated, lies inside its window and is a US federal public-domain
work whose Commons licence string is exactly `Public domain`):

| person_id | window | likeness reference (photograph date, licence) | prompt |
|---|---|---|---|
| fahd_bin_abdulaziz_al_saud | 1991-01-01 to 1996-01-01 | Bush Library, official White House photograph P38849-17, Riyadh, 31 Dec 1992; Public domain (PD-USGov) | `fahd-bin-abdulaziz-cartoon-1991-v1.txt` |
| fahd_bin_abdulaziz_al_saud | 1996-01-01 to 2005-08-01 | Helene C. Stikkel / DoD 981013-D-2987S-196, Riyadh, 13 Oct 1998; Public domain (PD-USGov-Military) | `fahd-bin-abdulaziz-cartoon-1996-v1.txt` |
| abdullah_bin_abdulaziz_al_saud | 1996-01-01 to 2005-08-01 | R. D. Ward / DoD 981103-D-9880W-172, Riyadh, 3 Nov 1998; Public domain (PD-USGov-Military) | `abdullah-bin-abdulaziz-cartoon-1996-v1.txt` |
| abdullah_bin_abdulaziz_al_saud | 2005-08-01 to 2015-01-23 | Cherie A. Thurlby / DoD 070117-D-7203T-016 (Commons crop), 17 Jan 2007; Public domain (PD-USGov-Military) | `abdullah-bin-abdulaziz-cartoon-2005-v1.txt` |

Reserve: none. The registry holds no third Saudi person ID.

Excluded: nobody. The verifier passed the Abdullah 1996-2005 window with two record nits, both fixed. The other
findings were fixed in the records and prompts:
- Fahd 1991: provenance now cites White House photograph P38849-17. Wayback holds the Bush Library's own file
  (byte-identical to the reference) and its gallery page. Both were re-fetched once each and pinned under
  `D:/spheres-scratch/sa-cast/refs/fahd_bin_abdulaziz_al_saud/wayback/`. The creator field is now clean, the
  wikitext-revision count is corrected, and the job IDs are recorded in the France shape.
- Fahd 1996: the complexion in the prompt is now "medium olive-tan", and the reference has `source_license_statement`
  and `rights_statement`.
- Abdullah 2005: the record now describes the embedded IPTC and XMP date, by-line and DoD number. It no longer says
  "no EXIF".
- Assembler's own finding: the Abdullah 2005 prompt's "light olive-tan" became "medium olive-tan", to match the
  photograph and the 1996 prompt.

Prompt pins after the fixes (LF sha256, as committed):
- `fahd-bin-abdulaziz-cartoon-1991-v1.txt`: `b6c2539c63ac0d31bdcb9b46e8c19a828e72e350117d140b49e1b865709828eb`
- `fahd-bin-abdulaziz-cartoon-1996-v1.txt`: `bedda1e071b620c748b17f2e9e23868abdf4d709ab61bede163800a91fb8be86`
- `abdullah-bin-abdulaziz-cartoon-1996-v1.txt`: `c58709f80c63f395148cca942143fa8bdb98affe15df3a263d224d007b2f6084`
- `abdullah-bin-abdulaziz-cartoon-2005-v1.txt`: `9a2c0e346a3291f7a34b871bb194b687fe13f1229a4168f3c3dad3d79ddc6614`

Still open for Codex:
- the Fahd 1991-1996 window, which has no accepted observation inside it;
- the Fahd split at 1996-01-01, or the single window `fahd_bin_abdulaziz_al_saud-f17eedfe6634`. One window needs a v2
  prompt, so please rule before rendering Fahd;
- Abdullah's start: 1996-01-01, or 1990-01-01 (`abdullah_bin_abdulaziz_al_saud-0ae5f49261bc`). A 1990 start needs a v2
  prompt;
- keeping Abdullah's January 2007 appearance (about 82) to the end of his window in January 2015, with no further
  ageing;
- optionally, the PDM 1.0 URL instead of `Template:PD-USGov` for the Fahd 1992 `license_url`;
- an `eol=lf` rule in `.gitattributes` for the four prompts. That file is not on this task's list; each pin also gives
  the CRLF-checkout hash;
- registry identities for batch 2: Salman and Mohammed bin Salman first, then Sultan, Nayef, Muqrin, Mohammed bin Nayef,
  the Shura chairs Jubair, Humaid and Al Al-Sheikh, the Allegiance Commission chair Mishaal and its secretary general
  Tuwaijri.

Residual pipeline jobs that stay open after registration: `fahd_bin_abdulaziz_al_saud-d3ec1b0b9097`,
`abdullah_bin_abdulaziz_al_saud-0aa00159df24` and `abdullah_bin_abdulaziz_al_saud-9bb74634df25`. No leadership-production
cartoon job is closed, because none exists for either person.
