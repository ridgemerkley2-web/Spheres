# CLAUDE-C01-03 — bounded 2024–2025 transition review

**Decision: accepted and integrated as bounded research** at `5a63d7b6060d07c15b2069c611f1b0716a488bfe`. Codex independently reviewed the bounded research delta from `14f01c5a` to `3291facf` (merge includes C01-03 tip `387e4526`). This is research acceptance, not closure of C01 or promotion into runtime leader data.

## Validation

- 69 Tonga tests and 16 campaign tests passed (85 total; no skips). Exact commands and log hashes are in isolated-validation.json.
- campaign_research.py --check, campaign_census.py --check, workboard.py --check and git diff --check passed.
- Review checkout remained clean at the exact reviewed commit. No Cargo, merges, commits or edits in the active integration checkout were performed.
- Incoming delta changes 27 research/handoff/test files. Runtime source, runtime data, art, mapping and central roadmap/workboard paths are byte-identical Git objects between the two reviewed commits.
- All 157 existing claims survive unchanged; 27 new claims make 184. Sources increase 103 to 119: 16 added; the existing shared IPU source gains the transition claim and provenance union. All organization, institution and role identities survive unchanged.

## Independent source checks

Seven critical original responses independently reproduced their exact recorded SHA-256 and byte counts: three Assembly minutes (9 Dec 2024, 24 Dec 2024, 31 Jan 2025), archived 22 Jan 2025 PMO appointment,28 Jan 2025 Cabinet PDF,6 Jan 2025 caretaker image,10 Dec 2024 Gazette. This verifies original-response provenance separately from checked-in factual-extract hashes. Relevant PDF pages and caretaker image were also visually inspected; specific pages are listed in isolated-validation.json.

The original records support the packet's careful distinctions: Sovaleni's stated 9 December resignation/acceptance; Vaipulu's attested deputy/caretaker service with no invented start/end; Eke's 24 December selection separate from 22 January appointment; Cabinet effectiveness 28 January separate from planned 30 January letter presentation and 31 January parliamentary oath. Eke's holder remains an appointment event, not an invented continuous interval. The existing 2021 warrant/end questions remain unresolved.

Initial cached Parliament download forms returned HTML; normal fresh public forms yielded the exact PDFs. The live PMO appointment endpoint was unavailable through the web tool; the raw Wayback capture independently reproduced exactly. The original Government Act and Royal Warrant were not independently located, and the packet explicitly leaves those issues open. Ancillary dynamic HTML sources were sampled rather than all re-fetched. No claim is made that every inaccessible historical instrument has been recovered.

## Integrated checks and remaining scope

The integrator updated the atlas browser assertion to display Sovaleni's stated 9 December 2024 end while retaining unknown ends for holders without a sourced end. The date contract cites both the resignation and Palace acceptance claims. A third interval check covers Fusimalohi's stated Cabinet commencement on 28 January 2025. The [merged validation](merged-validation.json) passed 69 Tonga Python tests, 16 campaign Python tests, 11 atlas Node tests, the exact research-index check and all 44 workboard markers. The [static browser qualification](browser-validation.json) passed all nine research packets at 1440, 390 and 320 pixels, including source-integrity refusal, request-race controls and the three interval checks. Its source files stayed unchanged during the run; the shared working tree was explicitly dirty with unrelated political development. No game API or campaign save was accessed.

The integrated campaign-census checksum check remains a documented **nonpass**: only its `production_snapshot` and `source_files` metadata differ, with `government.rs` the changed direct source. The integrator will regenerate this shared provenance after political source freeze. The isolated exact packet passed its census check; this follow-up did not repin shared data merely to silence the current failure.

Both workboards and the C01-03 handoff now record acceptance; only C01-05 and C01-06 remain pending in the submitted research inventory. All prior C01-08 claims survive unchanged. The historical cutoff remains 7 September 2026. C01, C06, S23, WC1 and CP1 remain open, and Tonga's exhaustive country census remains incomplete. This intake does not install leaders, create avatars, grant portrait rights or complete the game-wide test campaign.
