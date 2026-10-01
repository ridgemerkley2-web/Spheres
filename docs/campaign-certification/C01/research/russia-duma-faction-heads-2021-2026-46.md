# Russian State Duma faction heads 2021-2026 46: the eighth convocation's five faction heads from the 2021 elections to the 2026 attestations

Packet: **CLAUDE-C01-46**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-ru-46`; claim commit `3ee3357f` on `a81d2486`, the head of
`codex/campaign-certification`, which had moved to `d0c6676b` and was merged before the result commits (no conflict).
**Not stacked** (the held CLAUDE-C01-28, which also edits `russia.json`, was not used as a base). Research access: 1 October 2026 (UTC; 30 September local); this packet's two download passes ran at
2026-10-01T02:32:42Z-02:47:30Z and 2026-10-01T03:19:03Z-03:19:44Z (UTC). The historical cutoff stays **7 September 2026**.

This packet reviews six observations, RU-FH-01 to RU-FH-06, in [russia.json](russia.json). It extends the five existing
roles `ru_duma_faction_20211012_{er,kprf,srzp,ldpr,nl}_head` ("Руководитель фракции — head of the parliamentary
faction") with **18 holder observations** appended after the five original holders, which are unchanged, from **seven
State Duma website news items** (Internet Archive raw captures) carrying **27 claims**. Six people are named: Владимир
Васильев, Геннадий Зюганов, Сергей Миронов, Алексей Нечаев, Владимир Жириновский and Леонид Слуцкий. It adds no
organization, institution or role, changes no existing source, claim, holder or faction lifecycle, and touches no party
office (CLAUDE-C01-28's party roles live on its own branch and are not edited). No game mapping, portrait or avatar is
added. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

## Outcome

| ID | Question | Decision |
|---|---|---|
| RU-FH-01 | Who did the factions choose as heads in October 2021, and on what days | **Claims only:** news 52394 (11 Oct 2021) reports Васильев elected at the «Единая Россия» faction's organisational meeting "в четверг, 7 октября", Миронов elected by decision of his faction, and Зюганов, Жириновский and Нечаев as having taken the head of their factions; its subtitle says they "станут" (will become) heads. Elections and reported selections, never holder dates |
| RU-FH-02 | Dated attestations in the first session | **Accepted:** news 53098 (22 Dec 2021) names "Руководитель фракции" Зюганов, Жириновский, Нечаев and Васильев at the closing sitting of the autumn session (four holder observations, `attested_on` 22 Dec 2021) |
| RU-FH-03 | Жириновский's death and the other heads on that day | **Accepted, one end pending a ruling:** news 53988 (6 Apr 2022, 15:58), subtitled "Руководитель фракции ЛДПР ушел из жизни…", quotes the faction's statement "Сегодня … скончался" (`until` 6 Apr 2022 on Жириновский's December 2021 observation); the same item attests Васильев, Зюганов, Миронов and Нечаев. News 53987 (13:10) states no day of death and is a claim only |
| RU-FH-04 | The LDPR faction after the death | **Claims only:** news 54314 (18 May 2022): Слуцкий "избран руководителем фракции ЛДПР" at the faction's sitting "18 мая" (an election claim, no `from`) and was earlier "временно исполняющим обязанности руководителя фракции" (acting service, claims only) |
| RU-FH-05 | All five heads after the change | **Accepted:** news 54910 (7 Jul 2022) names the five "Руководитель фракции" at the President's meeting, Слуцкий's first holder observation (five observations, `attested_on` 7 Jul 2022) |
| RU-FH-06 | The latest attestation before the cutoff | **Accepted:** news 63980 (27 Jul 2026) names the five "руководитель фракции" at the President's meeting in the Kremlin (five observations, `attested_on` 27 Jul 2026) |

### Holders appended

Each role keeps its original holder first (`attested_on` 12 October 2021, news 52404, unchanged); this packet's
observations follow in date order. No observation has a `from`.

| Role | Name | `attested_on` | `until` | Claims |
|---|---|---|---|---|
| `…_er_head` | Владимир Васильев | 2021-12-22, 2022-04-06, 2022-07-07, 2026-07-27 | null | one attestation each (53098, 53988, 54910, 63980) |
| `…_kprf_head` | Геннадий Зюганов | 2021-12-22, 2022-04-06, 2022-07-07, 2026-07-27 | null | one attestation each (53098, 53988, 54910, 63980) |
| `…_srzp_head` | Сергей Миронов | 2022-04-06, 2022-07-07, 2026-07-27 | null | one attestation each (53988, 54910, 63980); he did not speak at the December 2021 sitting |
| `…_ldpr_head` | Владимир Жириновский | 2021-12-22 | **2022-04-06** | `ru_duma_news_53098_zhirinovsky_ldpr_head_20211222`, `ru_duma_news_53988_zhirinovsky_died_as_ldpr_head_20220406` |
| `…_ldpr_head` | Леонид Слуцкий | 2022-07-07, 2026-07-27 | null | one attestation each (54910, 63980) |
| `…_nl_head` | Алексей Нечаев | 2021-12-22, 2022-04-06, 2022-07-07, 2026-07-27 | null | one attestation each (53098, 53988, 54910, 63980) |

Every attestation is a State Duma news item that names the faction and the exact office ("Руководитель фракции …"),
dated by its own dateline (an official record naming the organisation and the exact office: C01-33, C01-35). Names are
the two-word forms the items print beside the office, the same form as the original holders; the pages' linked hover
biographies (with patronymics) are not used, and a genitive form (Нечаев, 6 April 2022) is kept as `printed_name` in the
extract row. "Лидер фракции" stylings are recorded in claim text only.

## Date ledger

Each row is a separate dated fact with its own claim. Elections, reported selections, acting service, death reports and
attestations are not merged.

| Date | Events | Holder field and claims |
|---|---|---|
| 7 Oct 2021 | «Единая Россия» faction elects Васильев (stated day) | claim only: `ru_duma_news_52394_vasilyev_elected_er_head_20211007` |
| 11 Oct 2021 (dateline) | Зюганов, Жириновский and Нечаев reported as heads; Миронов elected (no day stated) | claims only: `…_zyuganov_heads_kprf_reported_…`, `…_zhirinovsky_heads_ldpr_reported_…`, `…_mironov_elected_srzp_head_reported_…`, `…_nechaev_heads_nl_reported_…` |
| 12 Oct 2021 | original holders (news 52404, unchanged) | `attested_on` of the five original observations |
| 22 Dec 2021 | Зюганов, Жириновский, Нечаев, Васильев "Руководитель фракции" | `attested_on` (four observations) |
| 6 Apr 2022, 13:10 | death reported, no day stated | claim only: `ru_duma_news_53987_zhirinovsky_death_reported_20220406` |
| 6 Apr 2022, 15:58 | "Руководитель фракции ЛДПР ушел из жизни"; "Сегодня … скончался" | Жириновский `until`; Васильев, Зюганов, Миронов, Нечаев `attested_on` |
| before 18 May 2022 | Слуцкий acting head (no days stated) | claim only: `ru_duma_news_54314_slutsky_acting_ldpr_head_reported_20220518` |
| 18 May 2022 | Слуцкий elected at the faction's sitting (stated day) | claim only: `ru_duma_news_54314_slutsky_elected_ldpr_head_20220518` |
| 7 Jul 2022 | the five heads at the President's meeting | `attested_on` (five observations) |
| 27 Jul 2026 | the five heads at the President's meeting in the Kremlin | `attested_on` (five observations) |

## Observations

### RU-FH-01 — October 2021: the faction elections

Evidence: news 52394, "Фракции определились со своими руководителями в ГД нового созыва" (11.10.2021, 16:30), reports
that the factions met before the first sitting of the eighth convocation and elected their heads. Its subtitle says
"Руководителями фракций станут Владимир Васильев, Геннадий Зюганов, Владимир Жириновский, Сергей Миронов и Алексей
Нечаев". Only the «Единая Россия» election carries a day ("в четверг, 7 октября"); the others are dated by the item's
dateline.

Decision: claims only. An election, or a head reported as chosen before the faction is registered, is never a holder date
and never a start (C01-27, C01-40); the original observations of 12 October 2021 stay the first holder records.

### RU-FH-02 — December 2021: the closing sitting of the first session

Evidence: news 53098 (22.12.2021, 11:30) on the speeches at the closing plenary sitting of the autumn session names
"Руководитель фракции" Геннадий Зюганов (КПРФ), Владимир Жириновский (ЛДПР), Алексей Нечаев («Новые люди») and Владимир
Васильев («Единая Россия»). The «Справедливая Россия — За правду» speaker, Михаил Делягин, spoke for the faction and is
not styled its head; he is not recorded.

Decision: accepted. Four observations, `attested_on` 22 December 2021 (the dateline; the item does not print the
sitting's own day). Жириновский's is his last in-office attestation before his death.

### RU-FH-03 — 6 April 2022: Жириновский's death

Evidence: news 53987 (6.04.2022, 13:10), "Ушел из жизни Владимир Вольфович Жириновский", reports Volodin's announcement;
its subtitle says the deputies honoured "память руководителя фракции ЛДПР" with a minute of silence, and Volodin calls him
"Лидер фракции ЛДПР в Государственной Думе". It states no day of death. News 53988 (15:58) is subtitled "Руководитель
фракции ЛДПР ушел из жизни после тяжелой и продолжительной болезни" and quotes the faction's statement: "Сегодня после
затяжной болезни скончался Председатель ЛДПР Владимир Вольфович Жириновский". The same item names "Руководитель фракции"
Васильев, Зюганов and Миронов and reports the words of "руководителя фракции «Новые люди» Алексея Нечаева".

Decision: accepted, with a ruling requested. Жириновский's December 2021 observation gets `until` 6 April 2022: the
Duma's own subtitle names the faction office and the death, and the quoted "Сегодня" resolves from the item's dateline
(the death rule of C01-34 and C01-36; the relative-day rule of C01-29 and C01-39). The quoted statement names the party
office ("Председатель ЛДПР"), which is not recorded. Fallback if the ruling goes the other way: a death claim only and no
`until`. News 53987 stays a claim. The original 12 October 2021 observation is unchanged and has no end. Four further
observations of the other heads, `attested_on` 6 April 2022.

### RU-FH-04 — 18 May 2022: Слуцкий's election and acting service

Evidence: news 54314 (18.05.2022, 10:25), "Леонид Слуцкий избран новым руководителем фракции ЛДПР", subtitled
"Решение было принято депутатами на внеочередном собрании фракции": he "избран руководителем фракции ЛДПР в
Государственной Думе"; "Решение было принято депутатами на заседании фракции 18 мая. Ранее Леонид Слуцкий был временно
исполняющим обязанности руководителя фракции после ухода из жизни Владимира Жириновского."

Decision: claims only. The election on a stated day is an election claim with no `from` (a "newly elected" styling on
the election day: C01-27, C01-40); a ruling is requested on whether a faction's own election of its head gives `from` for
a parliamentary faction office. The acting service has no days and is claims only.

### RU-FH-05 — 7 July 2022: the five heads after the change

Evidence: news 54910 (7.07.2022, 21:24), on the President's meeting with the Duma's leadership and the faction heads,
names in its body "Руководитель фракции" Геннадий Зюганов (КПРФ), Леонид Слуцкий (ЛДПР; gallery caption adds
"Председатель Комитета по международным делам"), Сергей Миронов («Справедливая Россия — За правду»), Алексей Нечаев
(«Новые люди») and Владимир Васильев («Единая Россия»).

Decision: accepted. Five observations, `attested_on` 7 July 2022; Слуцкий's first holder observation. The committee
office is outside this packet.

### RU-FH-06 — 27 July 2026: the latest attestation before the cutoff

Evidence: news 63980 (27.07.2026, 21:06), "Владимир Путин встретился в Кремле с руководителями политических фракций":
"Во встрече приняли участие руководитель фракции «Единая Россия» Владимир Васильев, руководитель фракции КПРФ Геннадий
Зюганов, руководитель фракции «Справедливая Россия» Сергей Миронов, руководитель фракции ЛДПР Леонид Слуцкий и
руководитель фракции «Новые люди» Алексей Нечаев."

Decision: accepted. Five observations, `attested_on` 27 July 2026. No end: the eighth convocation's powers and the
ninth convocation's faction elections fall after the cutoff.

## Sources added

Seven sources, each with a checked-in derived factual extract under [sources/](sources/)
(`russia-duma-news-<number>-<yyyymmdd>-facts.json`, LF, format `spheres-c01-derived-factual-table/v1`, with its own
checksum in the packet). Each extract records the capture URL, the original URL, the capture time, the original
response's byte count and SHA-256, a stability record and one row per claim (claim_id, observation, role, `holder_name`
and the printed form, role title, event kind, date, text, locator). Source type `primary_legislature_news_notice_archived`;
publisher: the State Duma's official website (duma.gov.ru), raw capture by the Internet Archive. Original pages are not
checked in; no photograph, emblem or linked biography is republished.

| Source | Dateline | Capture | Claims |
|---|---|---|---|
| `ru_duma_news_52394_20211011` | 11.10.2021, 16:30 | 20211011155146 | 5 |
| `ru_duma_news_53098_20211222` | 22.12.2021, 11:30 | 20220119095504 | 4 |
| `ru_duma_news_53987_20220406` | 6.04.2022, 13:10 | 20220407135620 | 1 |
| `ru_duma_news_53988_20220406` | 6.04.2022, 15:58 | 20220407092508 | 5 |
| `ru_duma_news_54314_20220518` | 18.05.2022, 10:25 | 20220524092011 | 2 |
| `ru_duma_news_54910_20220707` | 7.07.2022, 21:24 | 20220710204413 | 5 |
| `ru_duma_news_63980_20260727` | 27.07.2026, 21:06 | 20260728062748 | 5 |

## Response identities and stability checks

Every recorded response is a raw Internet Archive capture (`id_` form, made before 7 September 2026) of a dated news
item, requested with curl's default User-Agent and no Accept-Encoding header. Each was downloaded twice by this packet,
at least 30 minutes apart (2026-10-01T02:32:42Z-02:47:30Z and 2026-10-01T03:19:03Z-03:19:44Z), with identical bytes and SHA-256; the
re-download by `packet_check.py` is recorded under [Checks](#checks).

| Source | Bytes | SHA-256 |
|---|---|---|
| `ru_duma_news_52394_20211011` | 247,852 | `bea477cefb12a59e2b6c13f534b6248c717b2797823ebac3369eca322e4b1b88` |
| `ru_duma_news_53098_20211222` | 245,393 | `5beb30c907d7fa7c019fb2ce3d79ebb97ef78e2ce4c2ad5ca81280a094590264` |
| `ru_duma_news_53987_20220406` | 225,439 | `3c4962730ce2199a54b1a25bad0328debbc482c897cbc12ac2aed06202f70197` |
| `ru_duma_news_53988_20220406` | 250,245 | `3cfc0a0403a6ba331e74c898428e5b4b923d7f5c26179e77cedcbbb7c9e7e7fc` |
| `ru_duma_news_54314_20220518` | 232,778 | `3851ebdab4786543605f9d72ad7725fac50d8c111e4e05596db550429236b7e8` |
| `ru_duma_news_54910_20220707` | 313,338 | `450c2f367443bad74ea39e987fe4a77f92ec08ee90977bcd4f7ef8f16a5dc47c` |
| `ru_duma_news_63980_20260727` | 53,864 (gzip as served) | `e9f8fddf860ee5b707d59991b97407fc3c6096ccb085da026d59ba06bdfc3a4f` |

The 2026 capture is served with `Content-Encoding: gzip` even without Accept-Encoding; its extract also records
`source_response_content_encoding: gzip` and the decoded identity (238,068 bytes,
`e8a6b66fa43a180ad4962ed2defcdb7ccb705a8a487ea0999537beee7168babf`). The other six are served uncompressed. Each capture
freezes the page's view counter and related-news list; none of them is used.

## Leads not imported

- **duma.gov.ru/news/55302** (21 September 2022, plenary speeches of the faction heads; capture 20220924022649, 367,810
  bytes, `338cacdda5c3…`, downloaded once): attests Зюганов, Слуцкий, Миронов and Нечаев a second time in 2022 (the
  «Единая Россия» speaker was a first deputy head); redundant with news 54910.
- **duma.gov.ru/news/52404** (the original source of 12 October 2021): Internet Archive captures exist (for example
  20211012102339 and 20211019231855). They could supply the original-response identity the existing source
  `ru_duma_factions_20211012` lacks; that source is not edited here (see next work).
- Other State Duma items found by search and not fetched: 63967 and 63962 (the closing sitting of the eighth
  convocation, July 2026), 63975, 62073 and 62077 (the President's meetings with faction heads, 2025), 60620 and 58531
  (session summaries), 56368, 61863 and 63020.
- **After the cutoff, not imported:** 64138 (Слуцкий elected head of the ЛДПР faction in the ninth convocation), 64147
  (Нечаев, ninth convocation), 64148 (Дмитрий Кобылкин elected head of the «Единая Россия» faction in the ninth
  convocation), 64156 and 64161 (the ninth convocation's first sitting), 64105 and 64106 (voting in September 2026).
- The party's own site (ldpr.ru events 211215 and 211532, on Слуцкий's election as faction head) is the party's record
  of a Duma office, not the Duma's; lead only. News agencies (ria.ru, iz.ru, vedomosti.ru, tass.ru, interfax.ru,
  pnp.ru) and ru.wikipedia (Слуцкий acting from April 2022) are leads.
- The faction pages `duma.gov.ru/duma/factions/72100004/` and similar are undated listings; a capture date is not an
  attestation date, so they are not used.

## Sources attempted

- `https://duma.gov.ru/news/52404/` and `http://duma.gov.ru/news/54314/` directly: connection timeouts (21 s and 30 s);
  a fetch of news 63980 through the web-fetch tool failed. Everything was taken from Internet Archive raw captures.
- The Internet Archive CDX API answered repeated HTTP 503 and 429 responses and one "Temporarily Offline" page under the
  parallel load of other packets; retries backed off 20-100 seconds and were never worked around. The CDX lookup for
  news 62073 failed after five tries, and 60620 and 62831 were not looked up.
- Two raw-capture requests (news 53988, first pass) answered 429 and succeeded after 30- and 60-second waits.
- `transcript.duma.gov.ru` and `sozd.duma.gov.ru` were not tried: the news items carried every fact needed, and the Duma
  stenograms are proposed as next work.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Russia-FH-001`: an integrator ruling on Жириновский's `until` 6 April 2022 (the Duma's subtitle names the faction
  office and the death; the day comes from the quoted faction statement's "Сегодня", which names the party office).
  Fallback: a death claim only.
- `C01-Russia-FH-002`: an integrator ruling on whether a faction's own election of its head on a stated day (Васильев,
  7 October 2021; Слуцкий, 18 May 2022) gives `from` for a parliamentary faction office, which has no separate
  assumption act.
- `C01-Russia-FH-003`: the Duma stenograms of 12 October 2021 (faction registration), of the sitting at which Слуцкий's
  election was announced (May 2022) and of the closing sitting of the eighth convocation (July 2026), when
  `transcript.duma.gov.ru` is reachable or captured.
- `C01-Russia-FH-004`: re-point the existing `ru_duma_factions_20211012` source (news 52404) to an Internet Archive raw
  capture to record its original-response identity (an edit of an existing source; declare it).
- Deputy faction heads, the Duma's leadership, committee chairs and the ninth convocation (after the cutoff) are outside
  this packet.

## Integration notes (outside this packet's file boundary)

- **Not stacked.** Base `a81d2486` (`codex/campaign-certification`). CLAUDE-C01-28 (party leaders, held in Codex review)
  edits `russia.json` and the Russia tests on its own branch; this packet's `russia.json` changes are append-only
  (sources at `sources[170:]`, claims and holders appended to the five faction roles, one unresolved line per faction and
  one packet line), and CLAUDE-C01-28's party roles are not touched. Codex resolves the overlap at integration; the source
  slices pinned in the Russia tests (`sources[68:170]` for C01-19, `sources[170:]` for this packet) shift if C01-28 lands
  first.
- **Roadmap.** The user chose to start this batch (C01-42 to C01-46) before Codex's "continue existing claims first"
  roadmap line.
- `research-index.json` is regenerated in a **separate commit**; it is the only file this packet shares with the other
  pending packets (C01-38 to C01-45). New Russia totals: 177 sources and 322 claims (previously 170 and 295); entries
  (21), roles (9), role observations (9), `mapping_pending` (21) and work orders unchanged. If another packet lands
  first, regenerate the index rather than merging it.
- Existing tests updated, none loosened:
  - `test_russia_research_s10h.py`: totals 170→177 sources and 295→322 claims; the C01-19 slice re-expressed as
    `sources[68:170]` and a C01-46 slice `sources[170:]` (7 sources, `ru_duma_news_*`, accessed 2026-10-01); the
    one-holder-per-faction assertion re-expressed as the original holder (all its checks kept) followed by this packet's
    exact (name, `attested_on`, `from`, `until`) list per role.
  - `test_russia_heads_of_government_c01_19.py`: its source slice `sources[68:170]`, totals (177, 322, 21, 9) and index
    source claims 322.
  - `test_russia_presidents_c01_14.py`: index source claims 322.
  - `test_ussr_government_supreme_soviet_c01_26.py` (not in the packet's pin list, but its `RUSSIA_HOLDERS_SHA256`
    hashes every Russia holder): the original hash is kept and now applies to every holder present at the base, and a
    second constant pins this packet's 18 appended observations.
  - `test_russia_party_leaders_c01_28.py` does not exist on this base; when CLAUDE-C01-28 is integrated its totals need
    the same +7 sources and +27 claims.
- The new test `test_russia_duma_faction_heads_c01_46.py` pins the 18 holder observations, the unchanged originals,
  every claim's date, kind, role and observation, every response identity (with the decoded gzip identity), the extracts
  and snapshots, the separation from every other Russia role and from the faction entries, and 23 mutations.
- In the atlas, the five faction roles gain 18 holder observations. No UI code changed.
- The accessed date is 1 October 2026 (UTC), the day of the recorded download times; it is 30 September local time.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
python -X utf8 D:/spheres-scratch/c01-pipeline/tools/packet_check.py 46
```

All passed on 1 October 2026 (UTC) before the result commits: the index regeneration and `--check` (C01 totals
1,976 sources and 4,901 claims, previously 1,969 and 4,874; Russia 177 sources and 322 claims);
`campaign_census.py --check` (exit 0; it does not fail on this head, so nothing about `census.json` needs
disclosing); 36 Russia tests (7 of them new) and 48 USSR tests; 79 research tests; 16 campaign tests (census
included); 11 atlas Node tests; `workboard.py --check` (44 markers, 49 bounded tasks); `git diff --check`. The
workboard check first failed with "Missing task handoff: docs/campaign-certification/S26/preparation/RECRUITMENT.md":
that file arrived with the merged `codex/campaign-certification` (`f6b74e99`) and lies outside the sparse checkout,
so `docs/campaign-certification/S26` (14 files, 41,667 bytes) was added to the sparse checkout; nothing was edited.
The new test's 23 mutations each fail as intended: the elections as starts or attested days, a capture day as the
attested day, the latest attestation and the successor's election as ends, acting service and a reported selection
as holders, an election claim cited by a holder, the dateless death report as the end, an end on the unchanged
original, the original removed, the death end removed, a cross-role holder, a cross-role claim on the Government, a
faction entry citing a new claim, patronymic and genitive names, a date beyond the cutoff, a party mapping, a
faction lifecycle end, a snapshot checksum mismatch and a claim left uncited by its role.

`packet_check.py 46` runs after the push (it re-downloads every source a third time); its summary goes in the
submission to the integrator. Known failures outside the suite, not fixed: `test_certified_gap_ledger.py` reports no
pinned attribution for this packet until Codex classifies its commit, and `test_certified_boundary_matrix.py` needs
`spheres-web/src`, which the sparse checkout lacks. The gap ledger is not touched.
