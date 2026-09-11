# S02 preservation check coverage

This maps the twelve proposed fixtures in [S01 ECONOMY_INTEGRATION](../S01/ECONOMY_INTEGRATION.md#proposed-s02-acceptance-fixtures) to executable assertions in the S02 runtime candidate identified by [manifest.json](manifest.json). The comparison baselines remain active A `5f7f355502f17bd6bd8f0383a2d14f0024fa7884` and master M `485c223f60d5ff6e46f6ae17164bf1ee3a8764d9`.

This is a source coverage review dated 2026-09-10, not an execution report. Final execution status is recorded separately in the [session report](README.md) and manifest. Test presence, assertions described here, and words such as “covered” do not assert a passing result. Execution evidence and any acceptance decision must identify the tested source separately.

“Composed coverage” means several bounded tests exercise different parts of the requirement; it does not mean one fixture runs the entire combined scenario. “Remaining scope” identifies a missing combined fixture, a narrower assertion, or intentionally deferred functionality. Tests are controlled simulation fixtures, not calibration or proof of a playable 1990–2035 campaign.

## S02-EF-01 — Legacy financial project

**Implemented assertions:**

- [`industry::tests::migrated_contract_preserves_paid_history_and_only_prices_unfinished_work`](../../../spheres-sim/src/industry.rs) creates a half-finished Warehouse with historical iron, prior cash/goods receipts and no frozen contract. A preview leaves serialization unchanged. Funding freezes only the unpaid remainder, preserves receipts, and direct/resumed progression completes for the remaining contracted cash.
- [`industry::tests::legacy_daily_rows_gain_financing_and_budget_activation_preserves_sunk_work`](../../../spheres-sim/src/industry.rs) removes the financing row and program budget from an existing daily project. With no funding, progress pauses; later budget activation advances only paid work and preserves the historical iron receipt.
- [`explicit_adoption_preserves_paid_work_and_current_accounts`](../../../spheres-sim/tests/s02_integration.rs) adopts connected economy after a real paid construction day. Project/industry records and the program plan stay exact, as do current population/GDP and existing cash/debt stocks. A second enrollment and save/load are exact.
- [`province_economy::tests::pending_transition_and_save_resume_preserve_accounting_dates`](../../../spheres-sim/src/province_economy.rs) separately checks monthly-to-daily transition and resumed accounting dates.

**Remaining scope:** Composed coverage. The pre-contract Warehouse cases are already daily, and the calendar transition case does not contain that Warehouse. There is no single archived old monthly input exercising transition, historical materials, funding adoption and completion together. The legacy Warehouse fixtures use USA as a technical fixture only.

## S02-EF-02 — Active company and party envelopes

**Implemented assertions:**

- [`s02_connected_economy_preserves_supplier_stock_receivables_and_party_identities`](../../../spheres-sim/tests/companies_integration.rs) has tank supplier stock, a pending purchase and historical party state. Connected enrollment plus reload preserves whole company, equipment, arsenal and party records, public net cash and leased slots; three subsequent simulation days compare exact saves and reconcile company cash.
- [`extending_an_existing_tank_company_preserves_its_old_save_and_promotes_only_new_platforms`](../../../spheres-sim/tests/companies_integration.rs) covers equipment envelope v2 preservation and explicit promotion to v3, old model/revision retention and resumed company work.
- [`old_company_save_versions_remain_exact_until_explicit_ammunition_supply`](../../../spheres-sim/tests/company_ammunition.rs) covers v2/v3 saves and explicit ammunition promotion to v4, with unchanged old vehicle property and refusal of forged versions.
- [`old_supplier_books_remain_byte_exact_until_service_booking_and_native_claims_cannot_be_forged`](../../../spheres-sim/tests/company_refits.rs) preserves older company books until booking, promotes the equipment envelope to v5, and rejects erased or inconsistent refit ownership, duplicate advance receivables and forged cash/capital state.
- [`party_leadership::tests::campaign_envelope_roundtrip_and_downgrade_are_fail_closed`](../../../spheres-sim/src/party_leadership.rs) separately verifies party envelope v1 roundtrip and refusal of invalid roster/version or erased party state. Its valid case has equipment version zero.

**Remaining scope:** The S02 adoption fixture is one tank/party combination. The existing equipment migration fixtures do not form the required v2–v5 × party-v1 × economy-envelope matrix. There is no single adoption fixture simultaneously containing R&D, transit, a receivable and refit escrow for each applicable version. These separately tested lifecycle paths must not be described as exhaustive S02 envelope qualification.

## S02-EF-03 — Funding-only construction

**Implemented assertions:**

- [`funded_construction_ignores_empty_raw_goods_power_and_workforce`](../../../spheres-sim/tests/s02_industry.rs) starts Office, Advanced Industry and Warehouse projects with empty stocks and zero filled workforce. Real plans and settlement agree on progress/cash; required inputs are zero, stocks stay unchanged, and repeated same-day construction does nothing.
- [`production::tests::daily_priority_reserves_one_cash_budget_across_different_project_kinds`](../../../spheres-sim/src/production.rs) checks a fully funded high-priority project and a partly funded lower-priority project against one cash budget and pure previews.
- [`production::tests::daily_completion_charges_exact_frozen_cash_once_and_survives_midday_reload`](../../../spheres-sim/src/production.rs) runs a Warehouse to completion around a saved partial timeline, compares its exact total cost against an idle control and prevents a second asset/bill.
- [`production::tests::daily_cancellation_preserves_paid_cash_without_refunding_or_granting_assets`](../../../spheres-sim/src/production.rs) cancels paid work without a refund or commissioned asset.
- [`industrial_modules::tests::missing_construction_inputs_do_not_block_work_and_operating_output_is_fractional`](../../../spheres-sim/src/industrial_modules.rs) exercises a fractional Starter Industry build with missing copper and checks fractional operating output after commissioning. [`unfinished_module_replays_identically_after_reload_and_keeps_frozen_price`](../../../spheres-sim/src/industrial_modules.rs) preserves fractional size and price through a mid-build save.

**Remaining scope:** Substantial composed coverage of the financial contract. The new S02 test does not place all sixteen kinds, a fractional Starter Industry and a near-complete project in the same priority queue; those boundaries are separate fixtures.

## S02-EF-04 — Small-country construction

**Implemented assertions:**

- [`all_sixteen_construction_contracts_are_financial_only_even_for_tonga`](../../../spheres-sim/tests/s02_industry.rs) exercises the actual allocator for France and Tonga, each of sixteen project identities, zero raw stocks and fractional Starter Industry. Paid work advances with no raw/goods requirement or raw debit.
- The normal project starts in EF-03 establish that zero workforce/power does not block funded construction in the France fixture.
- [`new_construction_previews_explain_conditional_operations_without_mutating_state`](../../../spheres-sim/tests/s02_industry.rs) checks the three new buildings' stated costs, minimum days, provincial/national effects and separate operating requirements, with exact save purity.

**Remaining scope:** The sixteen-kind Tonga test deliberately inserts frozen project records directly, bypassing siting/start eligibility. It proves allocation of existing legal work, not that every listed project can legally start in Tonga. The commissioned/absent power × workforce matrix and an end-to-end legal small-country start through completion remain uncovered as one country-comparison scenario.

## S02-EF-05 — Existing naval orders and leases

**Implemented assertions:**

- [`old_naval_line_and_company_lease_keep_their_arms_plant_entitlements`](../../../spheres-sim/tests/s02_industry.rs) starts a legacy naval line and company lease in two Arms Plant slots, enables the rebuild rule, and checks unchanged line ownership, no new dock claim and no blocker. A new Shipyard line uses its separate slot ledger. Stopping it preserves the old lease/slots and the existing order vector.
- [`dock_without_operating_inputs_cannot_spend_or_order_even_with_equipment_raw_inputs`](../../../spheres-sim/tests/s02_industry.rs) forces zero qualified staff and verifies no new dock spending, throughput or order despite equipment raw inputs.
- [`company_rights_share_the_existing_factory_and_preserve_paid_public_work`](../../../spheres-sim/tests/companies_integration.rs) separately exercises the retained shared factory reservation/public work contract.

- [Combined paid-entitlement fixture](../../../spheres-sim/tests/s02_entitlements.rs) reserves three existing Arms Plant slots in one province for a legacy naval line, a partly finished public custom batch and a supplier lease. Real paid naval work and a seven-day custom delivery survive connected adoption/reload; the arriving tank settles once and cannot release another owner's slot.

**Remaining scope:** The combined fixture uses explicit opening endowments and isolated fiscal settlement. The naval order remains paid and pending; it does not skip its 36-month lead time. The full naval-arrival lifecycle and company migration matrix remain S05 qualification.

## S02-EF-06 — New operating chain

**Implemented assertions:**

- [`commissioned_operations_consume_real_complete_bundles_once_and_survive_save`](../../../spheres-sim/tests/s02_industry.rs) operates installed Office/Advanced sites with supplied power. It checks exact advanced-component output (0.2), intermediate stock (9.59), copper/rare-earth/coal use, operating/energy cash receipts, both GDP receipt identities and unchanged treasury before fiscal settlement. Same-day rerun and reload are exact.
- [`missing_rare_earths_never_consumes_an_incomplete_operating_bundle`](../../../spheres-sim/tests/s02_industry.rs) checks no partial stock, cash or GDP debit/award when that input is absent. [`qualified_worker_shortage_caps_operations_without_inventing_employment`](../../../spheres-sim/tests/s02_industry.rs) forces zero filled jobs and verifies zero operation without changing the population book.
- [`commerce::tests::advanced_components_counteroffer_escrow_freight_and_delivery_conserve_separate_inventory`](../../../spheres-sim/src/commerce.rs) checks a component counteroffer, acceptance, escrow, dispatch and dated delivery with save continuation; one inventory is conserved and delivery cannot duplicate components or award resale GDP.
- The preview fixture in EF-04 checks read-only operation requirements.

**Remaining scope / intentionally deferred functionality:** There is no complete shortage-and-replenishment matrix for each skill, cash, power, intermediate input, demand and storage. More importantly, retained supplier fabrication does **not** consume advanced components in this S02 candidate. [`manufacturing::advanced_components_demand_daily`](../../../spheres-sim/src/manufacturing.rs) returns zero, and the new preview test explicitly asserts zero to avoid a retroactive supplier input bill. Production and trade are implemented; the requested company consumer belongs to the later supplier integration contract. EF-06's full consumption requirement is therefore not satisfied by these checks.

## S02-EF-07 — GDP chain reconciliation

**Implemented assertions:**

- [`gdp_projects::tests::actual_factory_chain_conserves_value_added_and_internal_power`](../../../spheres-sim/src/gdp_projects.rs) asserts final old-chain value added after raw/intermediate deductions, shared internal power, no direct treasury/GDP write and exact repeated/save behavior.
- [`province_economy::tests::opening_covers_every_live_nation_and_reconciles_without_repricing`](../../../spheres-sim/src/province_economy.rs), [`annual_output_replaces_itself_and_only_inherited_output_compounds`](../../../spheres-sim/src/province_economy.rs) and [`first_observed_existing_asset_is_absorbed_but_subsequent_upgrades_are_not`](../../../spheres-sim/src/province_economy.rs) check national/province totals, replacement of actual project output and inherited-asset baselines.
- [`gdp_projects::tests::completion_keeps_actual_work_receipt_and_legacy_work_is_not_incremental`](../../../spheres-sim/src/gdp_projects.rs) and [`legacy_and_rebuilt_construction_kinds_use_paid_work_not_completion_bonuses`](../../../spheres-sim/src/gdp_projects.rs) check actual paid construction receipts, all thirteen/sixteen catalogue identities, no legacy completion bonus and no treasury/debt change. [`military_order_is_a_payment_not_fabricated_delivered_production`](../../../spheres-sim/src/gdp_projects.rs) checks the ordering boundary.
- EF-06 checks new Office/Advanced receipt identities and exact physical inputs; EF-08 checks that enrollment/training does not directly reseed GDP or cash.

**Remaining scope:** The old-chain value-added fixture does not contain the new Office/Advanced operations. The new operation fixture checks receipt presence, not the complete new-chain GDP arithmetic and national/province reconciliation with inherited generation/processors in the same world. That joined inherited/new-asset scenario remains a useful regression; existing GDP module tests are not a substitute for asserting its final formula.

## S02-EF-08 — Population enrollment and training

**Implemented assertions:**

- `legacy_peace_resident_estimates_reconcile_only_on_initial_enrollment` reproduces the archived Kuwait ownership/population mismatch, preserving every field outside the explicit population adoption changes. It verifies raw-load inertia, proportional mapped shares, the separate France residual, dated replay and strict refusal of corrupt enabled books. The paired invalid-estimate test refuses negative, nonfinite and overflowing inputs atomically.
- [`tiny_resident_accounts_keep_each_sector_within_its_job_demand`](../../../spheres-sim/src/population.rs) reproduces China's positive year-30 rounding residual and verifies bounded sector staffing for zero, tiny and ordinary populations. [`archived_legacy_checkpoint_adoption_preserves_accounts_and_replays`](../../../spheres-sim/tests/s02_population.rs) accepts an explicitly supplied unchanged archive, checks retained positive residuals/property and repeats two actual daily save/resume steps. This external-input test is ignored by the default suite and must be invoked separately with recorded input provenance.

- [`france_japan_india_enroll_current_midcampaign_residents_without_reseeding_money`](../../../spheres-sim/tests/s02_population.rs) modifies current residents, mapped district population, GDP and balances at 2005-06-15 for France/Japan/India. Enrollment preserves every nation's population/GDP/cash/debt/political capital and RNG; outcomes match current residents, no graduates/courses appear, and repeated enable/tick/views preserve serialization. A saved next-day continuation is exact.
- [`training_requires_paid_calendar_days_and_keeps_qualifications_on_save_resume`](../../../spheres-sim/tests/s02_population.rs) compares funded/unfunded courses over 548 calendar days, saves on days 2 and 300, prevents same-date progress, checks no early qualification, conserved residents and better qualified staffing only after completion, with unchanged direct GDP/treasury.
- [`japan_and_india_training_cross_year_save_and_policy_dates_without_early_award`](../../../spheres-sim/tests/s02_population.rs) preserves course and policy cooldown dates across the year boundary. [`actual_daily_pipeline_keeps_districts_nations_and_outcomes_coherent`](../../../spheres-sim/tests/s02_population.rs) uses full daily ticks and validates/reloads France/Japan/India.
- [`inconsistent_enrollment_is_atomic_and_loaded_books_fail_closed`](../../../spheres-sim/tests/s02_population.rs) rejects inconsistent residents/outcomes, future dates and nonfinite/corrupt books. [`funding_only_construction_does_not_fabricate_domestic_workforce_demand`](../../../spheres-sim/tests/s02_population.rs) checks that construction queues create no fabricated construction employment.

**Remaining scope:** The France/Japan/India comparison uses deliberately altered mid-campaign states; the separate legacy archive is not a played-save matrix for all three countries. Training uses controlled cohort/job demand and funding inputs. Validation checks coherent employment totals; the fixtures are not an exhaustive adversarial allocation matrix over every skill and sector. They cover same-date population continuation, not every possible interrupted full-system phase with a simultaneously maturing course.

## S02-EF-09 — Succession and ownership

**Implemented assertions:**

- [`mapped_transfer_preserves_residents_qualifications_and_course_progress`](../../../spheres-sim/tests/s02_population.rs) uses a real France-to-Japan district transfer. [`unallocated_successor_transfer_preserves_distinct_training_progress_and_people`](../../../spheres-sim/tests/s02_population.rs) checks unallocated residents and distinct courses with different funded progress.
- [`authored_succession_moves_existing_cohorts_without_reseeding_successor_totals`](../../../spheres-sim/tests/s02_population.rs) constructs successor government records then calls real USSR district dissolution to Russia/Ukraine. It conserves living residents, preserves mapped cohort/course records, clears the dead parent's outcomes, sets each successor's opening population to actual transferred residents and makes repeat reconciliation inert. It checks that population reconciliation leaves the fixture's existing financial/GDP fields untouched.
- [`province_economy::tests::consent_reattributes_project_output_without_transferring_gdp_twice`](../../../spheres-sim/src/province_economy.rs) separately checks district GDP ownership.
- [`s02_fiscal_save_validation_is_pure_and_refuses_corrupt_or_future_receipts`](../../../spheres-sim/tests/s02_fiscal.rs) accepts a newly created Russia with authored debt ratio and unopened stock fields without letting validation open its books. [`s02_dissolved_government_retains_valid_history_without_reopening_its_books`](../../../spheres-sim/tests/s02_fiscal.rs) preserves a dead government's valid historical fiscal record.

- [Combined authored-succession fixture](../../../spheres-sim/tests/s02_succession.rs) creates all fifteen republics through the real politics path. It preserves mapped/unallocated residents and course progress, paid construction, completed-site location and historical stock/financial property, then compares two resumed daily steps. Successors open at zero cash and their authored debt obligations.

**Remaining scope:** Opening sites, component stock and courses are explicit controlled fixtures. The inherited game keeps the dead government's national stock, nominal accounts and unfinished-project payer as historical property; it does not distribute them to successors. The old paid project becomes blocked on ownership loss. This proves retention under existing ownership rules, not a new settlement or inheritance policy. Any later redistribution belongs to S04/S05 and must preserve the same property explicitly.

## S02-EF-10 — Fiscal and company money

**Implemented assertions:**

- [`s02_enrollment_preserves_existing_stocks_and_creates_no_reserve`](../../../spheres-sim/tests/s02_fiscal.rs) preserves open balances and opens previously unopened accounts at zero cash plus existing GDP/debt-ratio obligations, with idempotent enrollment.
- [`s02_construction_and_company_capital_share_one_public_bill`](../../../spheres-sim/tests/s02_fiscal.rs) combines a real company capitalization with recorded construction spending. It checks one public bill, one company settlement, exact final cash/debt and fiscal spending, cleared receivables and no second close.
- [`s02_receipt_and_final_cash_are_observed_once`](../../../spheres-sim/tests/s02_fiscal.rs) reads an actual paid receipt plus separately paid cash activity without another bill. [`s02_partial_months_and_calendar_jumps_do_not_invent_paid_years`](../../../spheres-sim/tests/s02_fiscal.rs) checks only observed daily exposure rather than invented intervening years.
- [`s02_late_first_program_enrollment_preserves_the_paid_ordinary_bill_and_resumes`](../../../spheres-sim/tests/s02_fiscal.rs) performs real ordinary settlement, then a same-date `SetProgramBudget`, reloads before observation and checks that the paid ordinary receipt survives exactly once through following daily ticks.
- Existing [`finite_stock_sale_is_atomic_stale_safe_and_fields_only_after_settled_delivery`](../../../spheres-sim/tests/companies_integration.rs), [`service_advance_is_locked_until_return_and_fresh_or_prepaid_refund_posts_once`](../../../spheres-sim/tests/company_refits.rs) and [`prior_year_service_refund_returns_cash_without_recreating_expired_or_current_authority`](../../../spheres-sim/tests/company_refits.rs) separately exercise purchase and refit money ownership.

**Remaining scope:** The new combined fiscal/company fixture uses capitalization, not a simultaneously settling stock purchase and refit service fee. Existing purchase/refit tests do not themselves enroll the new fiscal observer. Several fiscal observer tests inject controlled receipts and explicitly charge them, so they verify bookkeeping/exposure rather than complete campaign revenue calibration. The purchase/refit-plus-observer combination remains partial.

## S02-EF-11 — Rule adoption and read-only views

**Implemented assertions:**

- [`explicit_adoption_preserves_paid_work_and_current_accounts`](../../../spheres-sim/tests/s02_integration.rs) checks explicit/idempotent adoption. [`refused_enrollment_is_atomic_and_player_scoped`](../../../spheres-sim/tests/s02_integration.rs) checks wrong-player, invalid population, monthly-world and production-disabled refusal without mutation. The direct API and command give the same production prerequisite reason; adoption cannot silently enable unrelated production rules.
- [`versioned_economy_refuses_downgrade_and_separate_master_property`](../../../spheres-sim/tests/s02_integration.rs) preserves a supported raw legacy save without enrollment and rejects missing/forged economy-envelope ownership, unsupported master company/warfare property and malformed known campaign namespaces.
- Browser API tests [`s02_parser_binds_enrollment_and_training_to_the_player`](../../../spheres-web/src/main.rs), [`s02_new_campaign_and_loaded_legacy_have_distinct_economy_enrollment`](../../../spheres-web/src/main.rs) and [`s02_connected_reading_is_pure_and_exposes_the_same_snapshot_in_cash_flow_and_state`](../../../spheres-web/src/main.rs) assert actor binding, explicit fresh-versus-loaded behavior, exact save purity and matching connected snapshots in both player views.
- EF-02/04/08/10 include separate company, construction, population and fiscal view/validation purity assertions.

**Remaining scope / supported boundary:** S02 does not migrate M's separate contractor/campaign-war property. It refuses recognized unsupported property before it can disappear. The refusal tests construct those shapes; they are not a successful import of a complete archived master save. Nor is there an all-readers repeated sweep for every historical A envelope. These API assertions do not constitute an interactive browser usability check.

## S02-EF-12 — Replay and date guards

**Implemented assertions:**

- [`integrated_daily_batch_and_resume_have_one_identical_timeline`](../../../spheres-sim/tests/s02_integration.rs) enables the connected economy on 2000-02-15 and compares a leap-February batch with individual full daily ticks and load/validation after each date. Serialized end states must match. Re-entering population on the settled date cannot age residents again.
- [`s02_pending_company_payment_and_fiscal_close_survive_reload_exactly`](../../../spheres-sim/tests/s02_fiscal.rs) saves before a company/public close, compares settlement hashes, reloads after close, repeats it and compares following full daily ticks. [`s02_pending_ordinary_receipt_and_partial_year_continue_deterministically`](../../../spheres-sim/tests/s02_fiscal.rs) covers a pending ordinary receipt across the year boundary.
- EF-03's construction/module completion and saved continuation tests, EF-06's same-date operations/component freight tests, EF-08's course saves and EF-02's supplier adoption continuation cover their own active owner boundaries.
- [`new_sparse_save_records_cannot_disappear_or_hide_behind_disabled_rules`](../../../spheres-sim/tests/s02_industry.rs) and the population/fiscal validation fixtures reject future dates, nonfinite state or records whose enabled owner has disappeared.

**Remaining scope:** The full-driver leap-February fixture does not simultaneously populate an active new operating chain, paid supplier transit/refit and construction completion. Separate owner-boundary replay checks provide composed evidence, but the complete multi-owner before/after timeline requested in S01 is not one implemented fixture. Save equivalence for these selected paths does not establish determinism for arbitrary commands or the full campaign lifetime.

## Evidence boundaries and follow-up

The largest functional deferral is EF-06's company component consumer. The largest combined-fixture gaps are the envelope/party cross-product (EF-02), full naval arrival after its real lead time (EF-05), new/inherited GDP reconciliation in one chain (EF-07), and supplier/refit property through succession (EF-09). Explicit successor fiscal opening is covered by the controlled succession fixture. These remaining gaps should stay visible in later supplier/warfare integration work rather than be silently treated as completed S02 acceptance.

The source defines each fixture's setup and command sequence. Optional output from `SPHERES_S02_EVIDENCE` records the S02 adoption snapshot and direct/resumed leap-February hashes; optional `SPHERES_S02_UI_FIXTURE` records real France browser-rule snapshots. These are not a complete twelve-fixture provenance manifest: source-envelope inputs and input/output hashes are not individually recorded for every composed check. This document does not invent missing hashes, results or archived input provenance.

Country coverage here is bounded: France/Japan/India population cases, France/Tonga construction allocation and controlled USSR/Russia/Ukraine succession, plus technical fixtures inherited from other modules. USA use is technical regression coverage and does not create a USA campaign requirement. The larger S01 country matrix, all-country startup smoke, long-run balance and 1990–2035 campaign qualification remain separate acceptance work.
