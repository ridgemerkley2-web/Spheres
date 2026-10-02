# CLAUDE-C06-RUSSIA-01: USSR / Russia cast, batch 1 (identities, references, prompts; Codex renders)

Owner: Claude. State: **claimed** (2 October 2026; in progress, not complete). Parent: C06 (the certified `USSR -> Russia`
country case: people from both `ussr.json` and `russia.json`), with C03 cartoon production for these windows. Pending
Codex registration and acceptance.

Origin: the user asked on 1 October 2026 to continue the cast work in parallel after the France, Japan, Brazil and South
Africa batches (`CLAUDE-C06-FRANCE-01`, `CLAUDE-C06-JAPAN-01`, `CLAUDE-C06-BRAZIL-01`, `CLAUDE-C06-SOUTHAFRICA-01`) were
prepared. The CP1 plan (`CP1-ACCELERATION.md`) allows parallel artwork for another already-reviewed batch while Codex
owns Tonga (`CLAUDE-C06-TONGA-01`). Generator route, as for France: **Codex renders** with its built-in image tool.
Claude prepares the identity review, dated likeness references, exact prompts and input order, then reviews and
registers the outputs Codex returns. `person_art_pipeline.py` requires the generator "OpenAI built-in image_gen", and
Claude has no such tool, so Claude never generates, edits or labels artwork itself.

Branch: `claude/c06-ru-01`. Base: `2fd186d6` (current `codex/campaign-certification`); not stacked on any other cast
branch. Claim commit: this record's first commit on the branch.

## Bounded deliverable

Batch `SURU-CAST-B01`: **one portrait, plus one conditional reserve.** This is below the six-to-eight target of the
other batches because only two people of this case have registry IDs in `spheres-sim/data/party_leaders.json` (see
"Why one portrait" below). The accepted research is C01-41 (CPSU General Secretary) and C01-49 (President of the USSR)
in `docs/campaign-certification/C01/research/ussr.json`. The `to` dates are exclusive. A window is an appearance
interval, not a tenure, and grouping observations under a person ID proves no term.

| person_id | requested window | accepted observations (attested_on) | window basis |
|---|---|---|---|
| mikhail_gorbachev | 1991-01-01 to 1991-12-26 | C01-49 President: 1991-01-22, 1991-08-22, 1991-10-05; C01-41 General Secretary: 1991-08-22 | the country card's leader by `office_links` (USSR, since 1985-03-11); continues his existing 1990 art, which ends 1991-01-01; ends the day after 25 December 1991, the gap ledger's audit boundary for USSR-identity roles (not an accepted dissolution date). C01-05's 25 December 1991 observation (integrated, acceptance pending) also falls inside. Window needs Codex agreement |
| nikolai_ryzhkov (reserve, conditional) | 1990-01-01 to 1990-11-25 | none accepted; C01-26 head of the Union government: 1990-01-12, 1990-11-24 (integrated, acceptance pending) | the 1990 seed (`leaders_1990.json`, the USSR row's second executive: Chairman of the Council of Ministers since 1985-09-27) to the day after his last C01-26 observation. Enters only if Codex accepts C01-26 or rules otherwise; no term end is asserted |

Gorbachev's accepted 1990 observations (C01-41: 1990-02-05, 1990-07-10; C01-49: from 1990-03-15) fall inside his
existing art (`mikhail-gorbachev-cartoon-1990-v1.png`, 1990-01-01 to 1991-01-01). So every accepted observation of
Gorbachev falls inside one of his windows. A later Gorbachev window, for a campaign in which the USSR outlasts 1991, is
not claimed: no accepted observation supports it, and it needs a Codex ruling.

Production job IDs these windows close. The default inventory is `python -B -X utf8 tools/avatars/person_art_pipeline.py
jobs --out D:/spheres-scratch/ru-cast/jobs.json` (sha256 `5f83d33def208c44f4a7a31e2b7f4ff5a8b3f15cf283abefba3455b7a87041ec`,
identical to the France and Japan inventories). Window-scoped IDs are the pipeline's own output for the exact window
(`jobs --from <from> --to <to>`, written to `D:/spheres-scratch/ru-cast/jobs-<from>-<to>.json`; 1991 file sha256
`d24c04a8a7594697af62c3c198959aa5ef93c5fe54c97c22a6455c8670ea04a7`, 1990 file sha256
`bf751521791e80b25eb5c58f268ffd1dd7ed6ff8213efba1508fdc0925633ece`). Nothing was written into the repository. No
generated `CA-USSR-*` or `CA-Russia-*` work orders exist in `C01/work-orders.json`.

| person_id | window | pipeline job, window-scoped | default-inventory job (partly closed) | leadership-production cartoon jobs closed |
|---|---|---|---|---|
| mikhail_gorbachev | 1991-01-01 to 1991-12-26 | `mikhail_gorbachev-73ead8847756` | `mikhail_gorbachev-6e5c079d8a6d` (1991-01-01 to 2027-01-01) | none; his only required window (1990-01-01 to 1990-01-02) is covered by the 1990 art (`known_windows_covered`) |
| nikolai_ryzhkov (reserve) | 1990-01-01 to 1990-11-25 | `nikolai_ryzhkov-524fe0aabf35` | `nikolai_ryzhkov-2607eb5bdffa` (1990-01-01 to 2027-01-01) | none exist (`eligibility_research_required`) |

### Why one portrait, and not the other heads of state and party leaders

The registry (`party_leaders.json`, 620 people) holds exactly two people of this case: `mikhail_gorbachev` (the only
USSR `office_links` row) and `nikolai_ryzhkov` (no term). All eight USSR and Russia party rows (`su_cpsu`, `su_dr`,
`su_soyuz`, `ru_ldpr`, `ru_vybor`, `ru_kprf`, `ru_apr`, `ru_yabloko`) have no terms and a history gap from 1990-01-01,
and Russia has no `office_links` row, so the Russia card has no leader person at all. No other person of this case has
a person record in `spheres-sim/data` or `spheres-web/data`.

The C01 gap ledger (`docs/campaign-certification/C01/gap-ledger/ledger.json`, last refreshed at `1fffb983`) classes the
case's 107 holder observations as 68 `c01_accepted`, 30 `c01_integrated_pending` and 9 `s10_discovery_intake`. Of the 68
accepted, only Gorbachev's seven (C01-41: 3; C01-49: 4) belong to a registered person. The other 61 belong to 22 people
with no registry person ID. Their art could not be registered, and this task may not edit `party_leaders.json`:

- USSR (`ussr.json`): Vladimir Ivashko (C01-41 CPSU Deputy General Secretary: 1990-07-13, 1991-08-21); the Democratic
  Russia co-chairs Viktor Dmitriev, Yuri Afanasyev, Lev Ponomarev, A. Murashev and Gleb Yakunin (C01-35, 1991); the
  Soyuz co-chair Anatoly Chekhoev (C01-35, 1990-12-20).
- Russia (`russia.json`): Boris Yeltsin (C01-51 RSFSR President: 1991-07-19, 1991-11-19, 1991-12-26, 1992-01-03);
  Alexander Rutskoy (C01-51 Vice-President: 1991-07-29 to 1993-09-01, until 1993-10-03); Gennady Zyuganov (C01-28 KPRF:
  1998-05-23, 2025-07-05, 2026-08-28; C01-46 faction head 2021-2026); Vladimir Zhirinovsky (C01-28 LDPR: 1996-11-23,
  2022-03-30; C01-46 to 2022-04-06); Leonid Slutsky (C01-28 LDPR 2022-2026; C01-46); Grigory Yavlinsky (C01-28 Yabloko:
  1998-03-14, 2001-12-23, 2004-07-04), Sergey Mitrokhin, Emilia Slabunova and Nikolai Rybakov (C01-28 Yabloko); Mikhail
  Lapshin and Vladimir Plotnikov (C01-28 Agrarian Party); Yegor Gaidar (C01-28 Democratic Choice of Russia, 1994-1997;
  its relation to the `ru_vybor` row is not established); and the 2021 Duma faction heads Vladimir Vasilyev, Sergey
  Mironov and Alexey Nechaev (C01-46).

The pending observations (C01-05, C01-14, C01-19, C01-26) cover the Union premiers and Supreme Soviet chairman (Ryzhkov,
Pavlov, Lukyanov), the RSFSR start dates of Yeltsin and Rutskoy, and the Russian presidents and chairmen of the
government from Yeltsin to Mishustin; only Ryzhkov among them has a registry ID.

A batch 2 needs Codex to add registry person IDs (and, for the Russia card, an `office_links` row) for these people;
organization-to-party-row mapping also stays with Codex (C01-28 was imported without runtime mapping). Proposed order,
by what the game would show first: Yeltsin, then Rutskoy and Ivashko, then the party leaders matched to the game's party
rows (Zyuganov, Zhirinovsky, Yavlinsky, Lapshin, Slutsky). Batch 2 would claim its own windows; none are claimed here.

### Photograph leads

Planner's leads (Commons API search metadata only, saved under `D:/spheres-scratch/ru-cast/plan/`; nothing was
downloaded or verified, and preparation must confirm each one's licence string, creator, photograph date and identity):

- Gorbachev, 1991: `File:President Bush and President Gorbachev sign the Strategic Arms Reduction Treaty (START) in the
  Kremlin in Moscow... - NARA - 186435.tif` (Susan Biddle, White House, 31 July 1991; public domain; 3000x2043), the same
  NARA series as his 1990 reference; `File:GorbachevMS.jpg` (Leo Medvedev, 24 October 1991; CC BY-SA 4.0; 2800x4050;
  authorship of the own-work claim to be checked); Japanese Ministry of Foreign Affairs photographs of his April 1991
  visit (listed as CC BY 4.0; other people in frame).
- Ryzhkov, 1990: `File:Nikolay Ryzhkov 1990.jpg` and `File:Nikolai Ryzhkov.jpg` (Australian Department of Foreign
  Affairs and Trade, February 1990; listed as CC BY 4.0).

If no qualifying photograph exists, that window is recorded as not ready; no face is invented and no other person
substitutes.

Claude delivers:
- `docs/campaign-certification/C06/production/ussr-russia/identity-review-batch-01.json`, a proposal in the shape of
  France's identity review, with the accepted holder dictionaries copied exactly;
- dated likeness photographs of the exact person, copied byte-identical into
  `spheres-web/ui/person-portraits/references/<slug>-<yyyy>-reference-v1.<ext>` (yyyy is the photograph's year), with
  licence strings from `FREE_LICENSES` only. Ported licences such as `CC BY-SA 3.0 DE`, NC/ND terms, government licences
  not on the list, agency or press photos, TV or video stills and non-free files are rejected. Gorbachev's existing
  `mikhail-gorbachev-1990-reference.jpg` is not touched;
- exact LF prompts `tools/avatars/person-prompts/mikhail-gorbachev-cartoon-1991-v1.txt` (and, for the reserve,
  `nikolai-ryzhkov-cartoon-1990-v1.txt`), in the shape of the France prompts (style anchor
  `margaret-thatcher-cartoon-1990-v3.png` as image 1, STYLE ONLY; the identity reference as image 2), disclosing any age
  gap between the photograph and the window;
- a render request for Codex: per job, the prompt file, the input order and the output size (1024x1536 RGB);
- after Codex returns the unchanged outputs: the visual review, the batch generation record, the `person_portraits.json`
  records and the registration receipt.

The registry gives Gorbachev's birth date (1931-03-02), so he is 59 to 60 in the 1991 window. Ryzhkov's registry row has
none; any age in his prompt comes from a cited source and is labelled general biographical knowledge, not C01 research.
The registry is not edited.

Any reviewer string names the actual reviewer. No human approval is claimed. Country sign-off for the USSR / Russia case
stays with the user and Codex.

## Allowed files and checks

Allowed files:
- this record;
- `docs/campaign-certification/C06/production/ussr-russia/`;
- `spheres-web/ui/person-portraits/references/` (new USSR / Russia reference files only);
- `tools/avatars/person-prompts/` (new USSR / Russia prompt and batch files only);
- after rendering, additive `people[pid].portraits[]` records in `spheres-web/data/person_portraits.json` and the new PNG
  files, coordinated with Codex.

Not touched: `party_leaders.json` (including the missing registry IDs above), existing portrait records and files
(including Gorbachev's 1990 art, reference and prompt), the pipeline code, `.gitattributes`, shared runtime, the task
queue, the workboard and the other cast branches. Regenerated shared outputs are left to Codex's integration. Original
source metadata and image bytes stay outside Git under `D:/spheres-scratch/ru-cast/refs/<person_id>/`, pinned by sha256;
response-header captures are not kept.

Inputs at claim (sha256 of the committed bytes at `2fd186d6`):

| path | sha256 |
|---|---|
| `docs/campaign-certification/C01/research/ussr.json` | `ba7a27399237dfc49cb9d1f0e569c72b42b687a475a91c28c7152c997ff1cdd5` |
| `docs/campaign-certification/C01/research/russia.json` | `ff3d166ec6b4b2b7406e48f6d55b907b632dfcb988350e5a307aeea4df5537e9` |
| `docs/campaign-certification/C01/gap-ledger/ledger.json` | `a7b3c1f1924394743cfebfc1fc7cac7ee24970f40451616273f91aafa374962b` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-41/README.md` | `210c512749291702751f9d1184108673cdceb2cda5d99b4927a6a7baa026d650` |
| `docs/campaign-certification/C01/integrations/CLAUDE-C01-49/README.md` | `3c9b460ee3a9b22ca428e566a9bbdda261e266ff820e4d7ed3696eabeb85ab35` |
| `docs/campaign-certification/C01/reviews/CLAUDE-C01-49-20261001/README.md` | `a076718a5fa7b4f94c483c9548fce576a9d57bae1634d76fa5f873994424953e` |
| `spheres-sim/data/party_leaders.json` | `b330e2c49fa14a615bcb50fe7e5c6b240bd6678076869699788fdd36f6fdf077` |
| `spheres-sim/data/leaders_1990.json` | `cdc868e9c60ad7df6ca92d4d8a30e1b355cab953df46108f1328d7dc6ab46008` |
| `spheres-web/data/person_portraits.json` | `45fb29edd8b2b3499f47593376fe009db9a6da329d2d79e061128d2d0e333825` |
| `spheres-web/data/leadership_production_2035.json` | `42bdb24bf47cd31367b25069ca2a2381ebff44b9d5dcc55d884550777b8df7c4` |
| `tools/avatars/person_art_pipeline.py` | `b9715a5e841c7e8683bc3ec60a26754244b62d755150f06ebc0911be171487ab` |

Checks: `person_art_pipeline.py self-test` and `validate`; `cartoon_review.py --check`; `leadership_production.py check`;
`campaign_census.py --check`; `python -m unittest discover -s tools/avatars`; `workboard.py --check`; `git diff --check`.
