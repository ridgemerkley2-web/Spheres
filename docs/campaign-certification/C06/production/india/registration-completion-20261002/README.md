# India batch registration proposal — 2 October 2026

Eight existing requested portraits are reviewed and ready for root's append-only merge. This directory does not register them itself. No human or Claude approval is claimed, and broader country research remains open.

- [Manifest fragment](manifest-fragment.json): exact saved person names, identity-source URLs and **new portrait records only**.
- [Generation record](../../../../../../tools/avatars/person-prompts/india-cast-registration-20261002.json): exact prompts, ordered image inputs, original output paths and hashes, source rights and named Codex reviewers.
- [Receipt](receipt.json) and [temporary-merge validation](validation.json).
- [Independent render review](../../../render-return-20261002/india-review.json) by Codex agent `/root/tonga_civilian_roster`; the renderer and registration reviewer is Codex agent `/root/tonga_royal_identities`.
- The original [render return](../render-return-batch-01-20261002.json) and [identity preparation](../identity-review-batch-01.json) remain unchanged historical records.

| Exact person ID | Appearance from | Exclusive end | Source / adaptation licence |
| --- | --- | --- | --- |
| harkishan_singh_surjeet | 1992-01-31 | 2005-04-01 | CC BY 2.5 / CC BY 2.5 |
| lal_krishna_advani | 1995-01-01 | 2005-12-31 | CC BY-SA 3.0 / CC BY-SA 3.0 |
| prakash_karat | 2005-04-30 | 2015-04-01 | CC BY-SA 4.0 / CC BY-SA 4.0 |
| rajnath_singh | 2005-12-31 | 2014-07-09 | CC BY-SA 2.0 / CC BY-SA 2.0 |
| nitin_gadkari | 2009-12-19 | 2013-01-23 | Open Government Licence v1.0 / same |
| sitaram_yechury | 2015-04-30 | 2024-09-12 | CC0 1.0 / CC BY 4.0 |
| jagat_prakash_nadda | 2020-01-20 | 2026-01-20 | CC BY-SA 4.0 / CC BY-SA 4.0 |
| m_a_baby | 2025-04-06 | 2026-09-08 | CC BY-SA 4.0 / CC BY-SA 4.0 |

Keep all existing portraits. Match exact person IDs and saved names, union identity-source URLs, then append these records; never replace the person's existing portrait array with this additions-only array. Validate the resulting full manifest before writing it.

The same pipeline checked a disposable full manifest containing these eight and Saudi Arabia's four additions: exit 0, zero errors, 48 boundary assertions passed, 12 additions and 119 total proposed portraits. All old portraits and untouched people were preserved. Three negative OGL controls were rejected. The shared manifest was not changed. Run `python -X utf8 docs/campaign-certification/C06/production/india/registration-completion-20261002/validate_registration.py` before merging to repeat this bounded check; its temporary manifest is removed automatically.

Appearance bounds remain the original art proposals, including gaps between party presidencies. Source-date ambiguities, Karat's bounded undated photograph, older/younger artistic interpretations and Nadda's pre-window source remain explicit. M. A. Baby's slightly parted smile is a recorded, non-blocking expression deviation. OGL source rights and attribution are retained as OGL, not relabelled public domain or CC. Amit Shah's superseded job and unprepared reserves remain excluded.
