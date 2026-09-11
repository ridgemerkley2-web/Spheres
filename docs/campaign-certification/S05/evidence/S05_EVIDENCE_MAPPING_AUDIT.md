# S05 candidate-2 evidence mapping audit

Read-only assessment of S1–S8 against `docs/campaign-certification/S01/RUNTIME_AND_VALIDATION.md:150–167`, the S05 README and `s05-staging/final-report-draft.md`. Source candidate: `8c9159786d3736a9953706262a52c50898225f0e`. Final test/UI patch and final browser/performance outcomes remain pending. This is not a completion or G1 marker.

Both source-bound native suites passed **1,475 tests, zero failed, 75 ignored**; both Node suites passed **1,459 tests, zero failed, one skipped**. Windows evidence: `evidence/S05-native-candidate-2.{json,log}`, `S05-ui-candidate-2.{json,log}`, `S05-external-candidate-2/result.json`. Linux: `evidence/S05-linux-candidate-2/{runner-result,summary}.json` and its logs. Exact default-test anchors and one-based log lines are listed below; ignored external checks are separate evidence.

## Coverage and remaining acceptance

| Cell | Demonstrated candidate-2 coverage | What remains before closing the cell |
| --- | --- | --- |
| S1 | Default tests cover raw legacy, equipment v1, party v1 with equipment 0/1, company/integrated envelopes and browser archive handling. Separately invoked original-active matrix passed all 28 lifecycle rows on both OSes. Separately invoked party wrapper test passed real supplier equipment versions 2–5 on both. | Genuine pinned-master positive is still failed on both: raw f32 JSON vs `to_value` promotion at `conflicts[0].front.KW-JA`. Final exact-wire comparison and explicit f32-bit regression must execute successfully. No tolerance widening is needed. |
| S2 | Unknown versions, mismatched capability/book pairs, nested unknown military properties, missing required fields, truncated/invalid-history archives and corrupt retained fiscal receipts refuse in native tests on both OSes. Decode constructs a replacement only on success. | Keep decoder-negative results separate from observed HTTP/browser errors. Successful final browser load/cancel/session checks belong to S8; no additional production defect was identified here. |
| S3 | Six explicit fresh-start tests pass. Mid-month pending-close replay, Japanese empty-component migration/refusal and historical incumbent retention pass. Fresh startup adds no supplier stock or simulated day; ordinary load remains opt-in. | Final candidate source binding for changed web assets/tests; no missing simulation acceptance found. All-137 startup and every succession remain later-session scope. |
| S4 | Original-active28 covers tank/mixed development, stock/transit/arrival, ammunition settlement/transit and ground/aircraft refit stages. Combined fixtures exercise paid ammunition actually used in an operational strike, partial refit plus paid financial construction through adoption, and pending movement/peace through archive continuation and once-only settlement. | Preserve each fixture's stated prerequisites and limits; do not imply every mission, country or full lifecycle cross-product was tested. No additional S4 production gap found. |
| S5 | Native receipts preserve one committed effect on retry, reject changed bodies and old sessions after archive replacement. Node full suite covers in-flight duplicate clicks, persisted intent, campaign replacement and stale review guards. | A final successful actual browser sequence must complete committed-response loss, reload/Continue, exact visible retry and named load into a new session. Candidate-2 browser execution stopped at an overlay, so it cannot close this row merely because native/Node checks passed. |
| S6 | Both Windows and Linux executed real temporary-file tests for partial staged-write errors, exact previous bytes, damaged-current protection of valid backup, paid-receivable recovery, once-only continuation, invalid slot names and rotating autosaves. | The actual master archive is **18,848,817 bytes**. Once its repaired external test passes, its native/browser encode/decode/continuation is a concrete larger-archive representative. Do not claim arbitrary archive sizes, real disk exhaustion or OS process-kill injection; those are not individually required cross-products of S01's listed cases. |
| S7 | Native previews preserve own/current actor, funds and RNG; integrated API tests and Node guards cover replaced campaign/player/day/target and tutorial destinations without orders. Actual browser helper now contains full saved-archive equality before/after government review cancellation, construction effects, company filtering and tutorial navigation. | Final helper must execute successfully on desktop and narrow viewport; observed Guidance interception of Construction is a real blocked-control defect until fixed and rerun, not bookkeeping. Independent human usability tests remain later scope. |
| S8 | Native exact-session route tests pass. The real-browser harness checks `/api/build`, served vs actual checkout bytes and normalized committed source, startup/menu/load/cancel, map and government/construction/company/research views. | Final exact compiled binary/browser run, screenshots, no page errors and required route completion are pending root. A failed intermediate run does not establish this result. Do not imply Linux browser or remote GitHub Actions execution from native WSL results. |

## Material findings for finalization

1. **Known remaining blockers only:** genuine master positive comparator, final actual-browser blocked-control/near-map fixes and final source-bound performance evidence. No additional production blocker emerged from this audit. Performance is a pending session-level requirement; its result must remain separate from functional native checks.
2. **S6 is no longer a missing-platform gap:** Windows and WSL GNU/Linux storage tests actually ran. The 18.8 MB master fixture provides an explicit larger input; qualify its full positive roundtrip after the comparator repair rather than inventing a size threshold or new stress project.
3. **Policy wording is stale:** game/browser execution was authorized and actual browser runs have occurred. Resolve/remove the old `S05-GAP-POLICY` restriction instead of claiming checks are still prohibited. Keep unexecuted browser/performance work pending on its actual merits.
4. **Draft counts/provenance:** derive embedded-asset count from the final `ASSETS` list, which currently contains 18 rather than the draft's 17. Preserve the original-active provenance qualification: its exporter binaries are associated with S01 by exact logged paths, unchanged pinned source and recorded fingerprints/timestamps; their current before/after SHA-256 hashes are not historical S01 cryptographic pins. The master binary does have its recorded S01 hash.
5. **Source and evidence boundaries:** candidate-2 full native results remain valid evidence for its unchanged simulator runtime. Later test-only comparator and UI changes need final web/default, Node, external and browser execution as planned, with a recorded diff proving the reuse boundary. Do not relabel earlier logs as having run on the final commit.

## Exact passing native anchors

Every entry below was found as one unambiguous `... ok` line in both candidate-2 native logs. `W` means `evidence/S05-native-candidate-2.log`; `L` means `evidence/S05-linux-candidate-2/native.log`.

| Cell | Exact test name | W line | L line |
| --- | --- | ---: | ---: |
| S1 | `storage::tests::envelope_load_preserves_sim_timeline_and_archive_legacy_still_loads` | 1723 | 1721 |
| S1 | `storage::tests::combined_company_save_import_retains_capability_and_archive_without_load_adoption` | 1900 | 1902 |
| S1 | `storage::tests::integrated_warfare_archive_and_standalone_import_keep_every_capability` | 1903 | 1903 |
| S1 | `s05_save_matrix_tests::standalone_party_v1_with_no_equipment_and_native_equipment_v1_uses_both_loaders` | 1743 | 1726 |
| S2 | `versioned_economy_refuses_downgrade_and_separate_master_property` | 1416 | 1416 |
| S2 | `combined_capability_cannot_be_downgraded_or_mislabeled` | 1454 | 1453 |
| S2 | `integrated_capabilities_refuse_downgrades_and_nested_unknown_property` | 1478 | 1478 |
| S2 | `s05_save_matrix_tests::malformed_archives_and_mismatched_direct_party_books_refuse_clearly` | 1833 | 1796 |
| S2 | `retained_master_receipt_does_not_relax_current_or_corrupt_receipts` | 1487 | 1487 |
| S3 | `s05_fresh_startup_tests::fresh_start_adopts_all_capabilities_without_free_work_or_supplier_stock` | 1698 | 1719 |
| S3 | `s05_fresh_startup_tests::fresh_startup_repeat_and_save_resume_preserve_exact_owned_state` | 1711 | 1724 |
| S3 | `s05_fresh_startup_tests::opening_market_preserves_legacy_cover_and_posts_nothing` | 1705 | 1707 |
| S3 | `s05_fresh_startup_tests::opening_market_refuses_a_disabled_market_without_changes` | 1691 | 1691 |
| S3 | `s05_fresh_startup_tests::setup_without_a_selected_nation_and_tiny_new_campaign_use_unified_rules` | 1697 | 1697 |
| S3 | `s05_fresh_startup_tests::legacy_load_keeps_company_and_operational_upgrades_opt_in` | 1690 | 1692 |
| S3 | `s05_save_matrix_tests::browser_archive_retains_pending_midmonth_close_and_resumes_daily_once` | 1703 | 1699 |
| S3 | `party_leadership::tests::legacy_japanese_empty_slots_upgrade_without_changing_campaign_state` | 467 | 464 |
| S3 | `party_leadership::tests::japanese_component_upgrade_rejects_populated_and_mixed_maps_atomically` | 473 | 488 |
| S3 | `party_leadership::tests::incumbent_survives_reference_expiry_without_votes_cash_or_rng_changes` | 452 | 453 |
| S4 | `s05_paid_company_air_stores_survive_pending_save_then_feed_operational_strike` | 978 | 978 |
| S4 | `s05_partial_refit_and_paid_building_continue_once_across_all_capability_adoption` | 999 | 999 |
| S4 | `s05_save_matrix_tests::pending_peace_and_real_movement_survive_archive_then_consent_settles_once` | 1787 | 1752 |
| S4 | `s05_campaign_api_tests::integrated_paid_day_retry_archive_reload_and_next_day_keep_one_timeline` | 1901 | 1901 |
| S5 | `s05_campaign_api_tests::integrated_paid_orders_reconcile_exactly_once_and_changed_receipts_refuse` | 1790 | 1739 |
| S5 | `s05_campaign_api_tests::integrated_paid_day_retry_archive_reload_and_next_day_keep_one_timeline` | 1901 | 1901 |
| S5 | `transport::tests::diplomatic_receipt_retry_posts_one_cost_and_one_dispatch` | 1896 | 1895 |
| S5 | `transport::tests::malformed_immediate_batch_never_half_commits` | 1893 | 1894 |
| S5 | `tests::browser_advance_lost_response_retry_is_exactly_once_and_session_bound` | 1772 | 1774 |
| S6 | `storage::tests::integrated_paid_work_survives_interrupted_write_and_valid_backup_recovery` | 1902 | 1900 |
| S6 | `storage::tests::failed_write_keeps_previous_valid_campaign_and_archive` | 1708 | 1702 |
| S6 | `storage::tests::slots_cannot_escape_root_and_three_autosaves_rotate` | 1713 | 1714 |
| S7 | `government_view::tests::government_preview_requires_own_exact_current_action` | 1667 | 1664 |
| S7 | `government_view::tests::government_institution_review_shows_real_cash_debt_and_pc_without_spending` | 1666 | 1668 |
| S7 | `tests::construction_impact_preview_is_read_only_scoped_and_validates_the_reviewed_project` | 1757 | 1765 |
| S7 | `tests::guidance_serves_one_players_state_and_production_at_the_same_date_without_orders` | 1806 | 1827 |
| S7 | `companies_view::tests::directory_and_adoption_preview_are_pure_and_player_scoped` | 1604 | 1604 |
| S8 | `equipment_view::tests::equipment_routes_require_current_campaign_on_get_and_post` | 1652 | 1654 |
| S8 | `tests::guidance_route_requires_the_exact_active_campaign_session` | 1777 | 1779 |
| S8 | `tests::industry_board_route_requires_the_current_campaign` | 1783 | 1783 |

## Separately invoked external anchors

- `s05_active_fixture_tests::original_active_company_stages_preserve_property_across_load_and_explicit_adoption` — passed both; 28 distinct `S05_ACTIVE_FIXTURE` rows, exact original file hashes, twice-canonical load, browser archive and explicit-adoption continuation. Windows `S05-external-candidate-2/2.log`; Linux `S05-linux-candidate-2/active-fixtures.log`.
- `s05_save_matrix_tests::standalone_party_v1_preserves_real_supplier_books_for_equipment_versions_2_through_5` — passed both; four `S05_PARTY_ENVELOPE` rows (2,3,4,5). Windows `S05-external-candidate-2/3.log`; Linux `S05-linux-candidate-2/party-versions.log`.
- `s05_master_migration_tests::actual_pinned_master_archive_preserves_property_history_and_next_day` — **failed both candidate-2 runs** at the f32 comparison described above. Windows `S05-external-candidate-2/1.log`; Linux `S05-linux-candidate-2/master-archive.log`. The pending repaired test must carry its own final binary/source/input evidence.
