# Tonga — first identity and portrait slice

**CODEX-C02-TO-01 · 1 October 2026 · proposal only.** Base:
`c58ee025da50dfac164fc839b3fc021c39d483fe`. This gives Claude an ordered,
testable first production slice. It does not install people, appointments,
party terms, succession triggers or artwork. Tonga and C06 remain unfinished.
The historical cutoff stays **7 September 2026**.

[proposal.json](proposal.json) connects three existing opening identities to
their exact game records, reserves five later identity proposals, and copies
ten dated holder observations with their original uncertainty. Two proposed
royal alias associations remain held. The checker prints a people-only source
enrichment preview for the two supported existing identities; it never applies it.
The retained [people-only preview](people-only-preview.json) contains only those
two IDs, their existing names and the selected independently reviewed source
URLs. It is an importer-compatible proposal, not authorization to apply it.

## First eight portrait priorities

Every interval below is an **appearance request**, with an exclusive end. It
does not establish a person's office tenure or eligibility. New intervals need
dated likeness and rights review; they are not approved just because they are
listed here. Use the fixed full-body 2D cartoon style from the
[campaign art direction](../../../../planning/ai-handoffs/CLAUDE-CAMPAIGN-LEADER-ART.md).

| Order | Exact existing ID or explicitly reserved proposal | Appearance request | Disposition and next action |
|---|---|---|---|
| 1 | `taufaahau_tupou_iv` — existing King | 1990-01-01 → 1991-01-01 | **Reuse.** Existing reviewed cartoon covers this window. Do not redraw it or extend it through the reign. |
| 2 | `fatafehi_tuipelehake` — existing secondary PM | 1990-01-01 → 1991-01-01 | **Ready for reference sourcing.** Find a dated likeness identifying Fatafehi specifically, review rights, then draw under this existing ID. A later holder of Tu'ipelehake is not interchangeable. |
| 3 | `siaosi_taufaahau_manumataongo` — existing named heir | 1990-01-01 → 1991-01-01 | **Ready for reference sourcing for the existing heir only.** Match his full recorded name and near-1990 appearance. The later George V association remains held independently. |
| 4 | `to_baron_vaea_pm_1992` — reserved, uninstalled | 1992-01-01 → 1993-01-01 | **Identity review, then art.** Resolve the full identity in the 1990s PM government lists and distinguish the later Lord Vaea Speaker; do not merge by title. |
| 5 | `to_ulukalala_lavaka_ata_pm_2000` — reserved, uninstalled | 2000-01-01 → 2001-01-01 | **Identity/alias review, then art.** Use the existing 2002 PMO office profile and 2006 Crown Prince article to review one permanent cross-role identity before import; do not manufacture royal lineage or a vesting day. |
| 6 | `to_feleti_sevele` — reserved, uninstalled | 2006-01-01 → 2007-01-01 | **People-only import review, then art.** The contemporaneous PMO records name this person. Review the proposed identity and a dated licensed likeness; acting and substantive service remain separate. |
| 7 | `to_lord_tuivakano_pm_2010` — reserved, uninstalled | 2010-01-01 → 2011-01-01 | **Identity review, then art.** Resolve the named appointee/full name from the accepted Palace appointment source before adopting a permanent ID. Do not automatically attach Speaker observations or another titleholder. |
| 8 | `to_samuela_akilisi_pohiva` — reserved, uninstalled | 2014-01-01 → 2015-01-01 | **People-only import review, then art.** The fully named PMO holder supports this identity proposal. Review a dated likeness; party leadership and affiliation are a separate Claude research task. |

The first three are present in the actual 1990 game opening. Only the King has
an authoritative primary-executive identity link, which powers the country
selector. The heir and secondary PM are named starting roles; their portraits
would support their own person/office cards and future UI integration, not
replace the King on country cards. The later five are preparation for dated
government/person views; they have no runtime appointment path from this file.
No new-country or later-date playability is claimed.

## Dated role decisions

The existing research belongs to Claude. This slice uses the accepted
[C01-07 Crown receipt](../../../C01/integrations/CLAUDE-C01-04-07/README.md)
and [C01-08 PM receipt](../../../C01/integrations/CLAUDE-C01-08/README.md).
Those were **targeted critical-claim audits**, not exhaustive verification of
every claim in the packets. Each copied observation lists the specific claims
reopened in the earlier review. Copying a whole observation does not newly
verify its other claims. No historical source was fetched for this task.

- **King IV:** the existing exact ID and primary office link agree. The
  researched 1990 assents and 2006 end can enrich provenance. They do not
  re-establish the saved 1965 start or birthday; neither is changed.
- **Fatafehi:** the existing full named secondary PM matches the government
  lists. Preserve the year-precision 1990 observation, its null term bounds,
  and the saved starting facts. The old IPU source was not independently
  reopened in C01-08 and is excluded from the source-enrichment preview.
- **Baron Vaea:** retain the conservative 1992–1998 observation and unknown
  boundaries. The lists disagree about the end year; 3 January 2000 is his
  successor's commencement, not proof of Vaea's end.
- **Lavaka Ata:** retain the explicit 3 January 2000 commencement and
  11 February 2006 resignation acceptance. They establish neither the previous
  PM's end nor a later royal alias by themselves.
- **Sevele:** retain the 7 April 2006 substantive attestation. The earlier
  acting appointment and later leave-taking audience do not fill term bounds.
- **Tu'ivakano:** the 22 December 2010 appointment audience remains an event
  observation; no separate effective date is invented.
- **Pohiva:** keep the 30 December 2014 appointment observation and the
  appointment effective 2 January 2018 as two observations. Do not bridge the
  2017 uncertainty or make a precise 2019 death/end from an upper bound.

Two alias checks are explicit next evidence, not vague country-wide holds:

1. **1990 heir → George Tupou V.** C01-07's proclamation links Crown Prince
   Tupouto'a to George V. It does not, within this slice's independent review,
   resolve the existing longer-name 1990 ID. Obtain/review a primary source
   explicitly linking that full person name to the Crown Prince/George V;
   preserve the existing ID, name and saved identity hash. Do not create a
   second George V person as a workaround. The exact George V reign observation
   is copied as `alias_held` and supplies no runtime mapping.
2. **Lavaka Ata → Tupou VI.** The already-recorded PMO article
   `to_crown_crown_prince_tupouto_a_lavaka_2006` links the former PM to Crown
   Prince Tupouto'a Lavaka, and the 2012 proclamation links that title to
   Tupou VI. The earlier audit did not explicitly record reopening the 2006
   bridge. Review that original and the full identity before accepting the
   join. Its relative “yesterday” is not a structured vesting date. The
   proposal pins that claim and leaves the Tupou VI association `alias_held`.

Do not add a Tonga party row from these PM or Crown observations. The modeled
1990 King/court estate is not a modern political party. Claude's local and
remote `claude/c01-to-44` tip `fdb71175e6a2f251c6e51361187068c0141cac9f`
claims DPFI/PDP research and explicitly excludes game mappings. That claim was
checked before starting; it is preserved without edits to its research or tests.

## Executable acceptance and handoff

From the repository root:

```sh
python -X utf8 tools/avatars/check_country_cast.py
python -X utf8 -m unittest discover -s tools/avatars -p test_country_cast.py -v
```

The checker requires exact existing identity/opening records, unchanged Crown
office link, exact selected research observations and claim/source pins, and
unchanged receipt text. Additive unrelated research is allowed. It rejects
invented precise dates, changed uncertainty, title/alias promotion, party grants,
portrait-window widening, reserved identities masquerading as installed people,
and country completion claims. Receipt text pins normalize platform newlines.
It reads files and prints JSON; there is no apply or network mode.

Acceptance for **this bounded deliverable** requires the commands to pass,
review of the eight dispositions, preservation of the original research/runtime
files, and explicit acknowledgment of the two held aliases. The
[validation receipt](validation.json) records the actual commands and scope.
These checks verify proposal integrity, not the truth of a source or a likeness.
The final run passed **31 cast checks and five existing importer self-tests**.
The cast checks include an in-memory source-enrichment dry run that preserves
every existing identity fact, party record, office link and portrait, and is
idempotent. Protected research/runtime input bytes were unchanged.

For Claude's next art delivery, start with rows 2 and 3. Deliver the exact
existing person ID, retained dated reference and rights record, proposed
appearance interval, physical PNG, prompt, hashes/dimensions and actual
identity/likeness/era/visual review. Stop an individual portrait if its reference
cannot establish the named person; keep the other row moving. Reuse row 1.
Rows 4–8 can proceed independently through their stated identity checks, but
their reserved IDs are not approved runtime IDs and should not enter the art
manifest until reviewed. Art review may narrow any requested interval.

Once identity changes are accepted, use the established people-only importer;
review office eligibility separately, run the required game checks, and prove
that the relevant person/office surface resolves the exact ID at its intended
date. A calendar date must never force a researched succession into a diverged
campaign. The existing fictional-successor pilot remains proposal-only and
cannot create royal or hereditary candidates. This slice does not close C02,
C03, C04, C06, S23 or CP1.
