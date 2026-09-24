# CLAUDE-C01-12: Japanese prime ministers, 1990–2006

Owner: Claude. State: **claimed** (24 September 2026; in progress, not complete). Parent: C01 (incomplete).

Origin: a self-proposed follow-up packet, started on the user's 24 September 2026 instruction to continue
development. The workboard asks for a distinct bounded packet that does not repeat accepted
C01-01/02/03/04/07/08 or reclaim the pending C01-05 (USSR/RSFSR), C01-06 (Saudi Arabia), C01-09 (South Africa),
C01-10 (Brazil) and C01-11 (India). This packet touches only the Japan research packet, which none of those
change. It is pending Codex acceptance and is not registered in `docs/planning/ai-workstreams.json`.

Branch: `claude/c01-jp-12`. Base: `ffe54b02` (current `codex/campaign-certification`). Claim commit: this
record's first commit on the branch.

## Bounded deliverable

The Japan packet has election-list, parliamentary-group and party-office observations only. Add one executive
institution, `jp_prime_minister`, with one role, `jp_pm` (内閣総理大臣 — Prime Minister of Japan, kind
`head_of_government`), and review at most ten observations between 1 January 1990 and the end of the Koizumi
cabinets in September 2006. The 2006–2026 prime ministers are left to a later packet.

1. The holder when the period opens (Toshiki Kaifu), and his re-designation and new cabinet after the
   February 1990 general election.
2. 1991: the Kaifu cabinet's resignation en masse and Kiichi Miyazawa's designation and appointment.
3. 1993: the Miyazawa cabinet's resignation and Morihiro Hosokawa's designation and appointment.
4. 1994: Tsutomu Hata's designation and appointment.
5. 1994: Tomiichi Murayama's designation and appointment.
6. 1996: Ryūtarō Hashimoto's designation and appointment, and his November 1996 re-designation.
7. 1998: Keizō Obuchi's designation and appointment.
8. 2000: Obuchi's incapacity, the acting prime minister (臨時代理), the cabinet's resignation, and Yoshirō
   Mori's designation and appointment, and his July 2000 re-designation.
9. 2001: Jun'ichirō Koizumi's designation and appointment, and his re-designations of 2003 and 2005.
10. 2006: the Koizumi cabinet's resignation, the date the office passed, and the successor's appointment only
    as the closing boundary of this packet.

Keep the Diet's designation (each House's vote, and a joint-committee or House of Representatives resolution
where they differ), the Imperial appointment ceremony (親任式), a cabinet's resignation en masse (総辞職),
continued performance of duties until a successor is appointed, and an acting prime minister as distinct dated
claims. Never infer an end from a successor's appointment unless a source states it. Give a holder `from` only
where a source states the day of appointment or assumption of office, and `until` only where a source states the
day the office ended; otherwise record `attested_on`. Continued performance of duties and an acting prime
minister are recorded as claims, never as holders. Follow the packet's existing convention for names (Japanese
as printed by the source), with a romanized form where the report needs one. Ministers, deputy prime ministers
and party offices are outside this packet; the existing LDP president observations stay separate and unchanged.

Primary sources are required: the Prime Minister's Office and Cabinet (kantei.go.jp, including archived pages),
the minutes of the National Diet (kokkai.ndl.go.jp) and the two Houses' own records, the Official Gazette (官報)
where accessible without restriction, and the Imperial Household Agency. Constitution texts may establish
procedure only, never a date. Retrospective lists of cabinets are claims, never boundaries. News, encyclopaedias
and history sites are leads only. The historical cutoff stays 7 September 2026.

## Allowed files and checks

Allowed files:

- this record;
- a new `docs/campaign-certification/C01/research/japan-prime-ministers-1990-2006-12.md`;
- `docs/campaign-certification/C01/research/japan.json` and specifically related new
  `docs/campaign-certification/C01/research/sources/japan-*-facts.json` extracts;
- focused tests under `tools/avatars/`: a new `test_japan_prime_ministers_c01_12.py`, and pinned counts or exact
  sets in `test_japan_research_s10d.py` updated to the new totals (none loosened).

Put generated `research-index.json` changes in a separate commit. Do not change shared UI, the roadmap, game
data or other country packets.

Checks:

- research-index `--check`;
- the Japan, research and campaign Python tests;
- the atlas Node check;
- `workboard.py --check`;
- `git diff --check`.

Mark the packet `ready_for_review` when done. C01 and all parent gates stay open.
