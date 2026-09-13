"""Build only the outside draft from small, completed qualification proofs."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json

STAGING = Path(__file__).resolve().parent
BASE = STAGING.parent
PIN = 'd770592aeead51f1313d507edd26b02d75a69bba'
proofs = {}

def read(relative):
    path = BASE / relative
    assert path.stat().st_size <= 2_000_000, 'Do not read/hash large archives in this draft task.'
    return json.loads(path.read_text(encoding='utf-8-sig'))

def proof(key, relative, scope, outcome=None):
    path = BASE / relative
    size = path.stat().st_size
    assert size <= 2_000_000, 'Only small proof/source files may be hashed here.'
    proofs[key] = {'original_path': relative, 'bytes': size, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'collected_path': None, 'scope': scope}
    if outcome is not None:
        proofs[key]['outcome'] = outcome
    return key

rows = [
 ('windows_native','evidence/S08-final-native-route-pool-1.json','Completed full Windows native suite','passed'),
 ('windows_native_log','evidence/S08-final-native-route-pool-1.log','Original Windows native log'),
 ('windows_node','evidence/S08-final-node-route-pool-1.json','Completed full Windows Node suite','passed'),
 ('windows_node_log','evidence/S08-final-node-route-pool-1.log','Node count evidence:1507 passed,0 failed,1 skipped'),
 ('linux_qualification','evidence/S08-linux-route-pool-1/runner-result.json','Linux native,Node and three external archive checks','passed'),
 ('windows_external','evidence/S08-external-route-pool-1/result.json','Three original ignored archive checks explicitly executed','passed'),
 ('windows_game_build','evidence/S08-final-binary-route-pool-1.json','Clean pinned release game build','passed'),
 ('general_browser','evidence/S08-final-browser-installed-route-pool-3.json','General browser acceptance with outside observation only','passed'),
 ('general_browser_inner','evidence/S08-general-browser-d770592/result.json','Original completed general UI assertions and served assets','passed'),
 ('general_observer_metadata','evidence/S08-general-navigation-observed-route-pool-3/metadata.json','Executed observational preload and exact source hashes'),
 ('general_observer_events','evidence/S08-general-navigation-observed-route-pool-3/events.jsonl','Real request,console and lifecycle timeline'),
 ('general_observer_review','evidence/S08-general-navigation-observed-route-pool-3-review.md','Startup,warning and retained failure interpretation'),
 ('money_browser','evidence/S08-final-money-browser-route-pool-1.json','Completed original Money UI assertions','passed'),
 ('money_browser_inner','evidence/S08-money-browser-d770592/result.json','Actual Money UI receipt and screenshots','passed'),
 ('genuine_export','evidence/S08-genuine-supplier-route-pool-1/runner-result.json','Fresh campaign ordinary paid supplier/APC journey','passed'),
 ('genuine_provenance','evidence/S08-genuine-supplier-route-pool-1/export/provenance.json','Native source identity and no-endowment disclosure'),
 ('genuine_native_outcome','evidence/S08-genuine-supplier-route-pool-1/export/result.json','Actual native contract and funding record','passed'),
 ('genuine_purchase_review','evidence/S08-genuine-supplier-route-pool-1/export/purchase-review.json','Original exact reviewed purchase'),
 ('performance_plan','evidence/S08-performance-plan.json','Original unchanged declared performance limits'),
 ('performance_binding','evidence/S08-performance-protocol-route-pool-1.json','Original declaration rebound to the current qualified test binary'),
 ('legacy_performance','evidence/S08-performance-route-pool-1/acceptance.json','Six unchanged legacy workloads and memory acceptance','passed'),
 ('legacy_performance_runner','evidence/S08-performance-route-pool-1/runner-result.json','Original run and executable/source preservation','passed'),
 ('legacy_performance_profile','evidence/S08-performance-route-pool-1/profile.json','Measured six-workload profile'),
 ('supplier_performance','evidence/S08-feature-performance-route-pool-1/runner-result.json','Fresh same-candidate original purchased input and five fixed bars','passed'),
 ('supplier_performance_profile','evidence/S08-feature-performance-route-pool-1/profile.json','Measured supplier31-day profile'),
 ('failed_general_1','evidence/S08-final-browser-installed-route-pool-1.json','Original30-second startup timeout; cause unknown','failed'),
 ('failed_general_1_log','evidence/S08-final-browser-installed-route-pool-1.log','Original timeout before UI assertions'),
 ('failed_general_2','evidence/S08-final-browser-installed-route-pool-2.json','Outside preload path setup error; no browser launch','failed'),
 ('failed_general_2_log','evidence/S08-final-browser-installed-route-pool-2.log','Retained MODULE_NOT_FOUND error'),
 ('failed_supplier_browser','evidence/S08-final-supplier-browser-route-pool-1.json','Real supplier journey stopped at stale-review cleanup','failed'),
 ('failed_supplier_browser_log','evidence/S08-final-supplier-browser-route-pool-1.log','Retained already-handled route error'),
 ('original_supplier_harness','integration/tools/ui/ci-supplier-imports.cjs','Frozen committed test source; separate from runtime binary'),
 ('corrected_supplier_harness','s08-staging/ci-supplier-imports.cleanup-order.cjs','Prepared exact corrected test source; full journey still pending'),
 ('supplier_harness_patch','s08-staging/supplier-preview-cleanup-order.patch','One-block cleanup correction only'),
 ('supplier_harness_vm','s08-staging/supplier-preview-cleanup-order-vm-evidence.json','13 bounded checks,including required failing original control','passed'),
 ('supplier_harness_vm_log','s08-staging/supplier-preview-cleanup-order-vm-evidence.log','All13 checks retained'),
 ('supplier_harness_vm_source','s08-staging/verify-supplier-preview-cleanup-order.cjs','Exact VM verifier source'),
 ('supplier_harness_review','s08-staging/supplier-preview-cleanup-order-review.md','Local Playwright cause,scope and provenance'),
 ('corrected_browser_runner','run-s08-supplier-browser-corrected.py','Explicit outside runtime/test-harness binding runner'),
 ('corrected_browser_overlay','s08-supplier-harness-overlay.cjs','Hash-checked Module._compile at original harness filename'),
]
for row in rows:
    proof(*row)
native = read(proofs['windows_native']['original_path'])
linux = read(proofs['linux_qualification']['original_path'])
export = read(proofs['genuine_export']['original_path'])
legacy = read(proofs['legacy_performance']['original_path'])
feature = read(proofs['supplier_performance']['original_path'])
assert native['passed'] and native['revision_after'] == PIN
assert linux['status'] == 'passed' and linux['candidate'] == PIN
assert export['passed'] and export['integrity_passed'] and export['candidate'] == PIN
assert legacy['passed'] and legacy['candidate'] == PIN
assert feature['passed'] and feature['integrity_passed'] and feature['candidate'] == PIN
assert proofs['performance_plan']['sha256'] == feature['plan_sha256_before'] == legacy['declared_plan_sha256']
assert proofs['original_supplier_harness']['sha256'] == 'de8a13611ce1617133710b252253cd82e41d6a4b74d39799f7d29eb2b97ce7f1'
assert proofs['corrected_supplier_harness']['sha256'] == 'c74f8a8d1f1ee090113179a5a34acfada0ffa28e8689de4e3aba8fa766e21b9c'
contract = export['native_outcome']['contract']
document = {
 'format':'spheres-s08-publication-draft','version':1,'session':'S08',
 'status':'qualification_in_progress','publication_status':'draft_not_collected','completed_date':None,
 'prepared_utc':datetime.now(timezone.utc).isoformat(),
 'runtime_candidate':PIN,
 'final_test_documentation_commit':None,
 'source_path_base':'external campaign-certification directory; collector remapping pending',
 'qualification_scope':'S08 supplier choice/reviewed imports only; no S09 advancement or later campaign/content/release certificate',
 'runtime_binary':{'sha256':'c5852894069c797c1cbf6b15a302f73f85a0f0b4a4edfee63eb1ca6b367f8435','identity_source':'windows_game_build','freshly_hashed_by_this_manifest_builder':False},
 'test_binaries':{'windows':native['binary_sha256'],'linux':linux['web_test_binary']['sha256'],'freshly_hashed_by_this_manifest_builder':False},
 'test_harness_distinction':{
   'runtime_unchanged':True,'committed_original_proof':'original_supplier_harness','executed_corrected_source_proof':'corrected_supplier_harness',
   'patch_proof':'supplier_harness_patch','vm_proof':'supplier_harness_vm','runner_proof':'corrected_browser_runner','overlay_proof':'corrected_browser_overlay',
   'change':'Release hold; await existing bounded fetch/fulfill/dispose completion; then unroute exact handler. No runtime,assets,commands,timeouts or assertions changed.',
   'final_commit_requirement':'Apply the exact corrected source actually executed,with runtime d770 and test/docs commit distinctly identified.',
 },
 'completed_checks':{
   'windows_native':native['totals'],
   'linux_native':next(row['totals'] for row in linux['checks'] if row['name']=='native'),
   'windows_node':{'passed':1507,'failed':0,'skipped':1,'total':1508,'count_proof':'windows_node_log'},
   'linux_node':{'passed':1507,'failed':0,'skipped':1,'total':1508,'count_proof':'linux_qualification'},
   'external_archives':{'windows_passed_tests':3,'linux_passed_tests':3,'scope':'Pinned master;28 active supplier fixtures;party v1/equipment versions2–5','ignored_suite_tests_count_as_passes':False},
   'general_browser':{'passed':True,'proof':'general_browser','inner_proof':'general_browser_inner','observation_preserved_original_assertions_timeouts_routes':True},
   'money_browser':{'passed':True,'proof':'money_browser','inner_proof':'money_browser_inner'},
 },
 'genuine_native_journey':{
   'passed':True,'proof':'genuine_export','actual_days_advanced':export['native_outcome']['actual_days_advanced'],
   'post_tick_date':export['native_outcome']['date'],
   'contract':{key:contract[key] for key in ['id','buyer','seller','company','product','quantity','buyer_revision','purchased_day','settled_day','delivered_day','total_price_bn','escrow_bn','refunded_bn','status']},
   'platform':'APC','purchase_date':'29 Oct 1995','post_tick_delivered_day':2155,
   'budget_policy':export['native_provenance']['declared_policy'],'no_synthetic_endowments':True,
   'archive_inventory':{'count':len(export['archive_phases']),'hashes_copied_from_completed_export_proof':True,'archives_freshly_hashed_by_this_builder':False,'phases':export['archive_phases']},
 },
 'performance':{
   'declared_plan_proof':'performance_plan','binding_proof':'performance_binding','limits_unchanged':True,
   'limits':{'simulation_history_p95_ms':300,'whole_turn_p95_ms':400,'whole_turn_max_ms':750,'legacy_sampled_private_bytes':1073741824,'supplier_equipment_board_p95_ms':300,'supplier_purchase_quote_p95_ms':250,'supplier_memory_limit':None},
   'legacy':{'passed':True,'proof':'legacy_performance','days_per_case':31,'cases':legacy['rows'],'sampled_private_bytes':legacy['sampled_private_bytes']},
   'supplier':{'passed':True,'proof':'supplier_performance','days':31,'checks':feature['acceptance_checks'],'original_purchased_input_sha256':feature['input_sha256_before'],'profile_sha256':proofs['supplier_performance_profile']['sha256'],'sampled_private_bytes':feature['memory']['maximum_private_bytes'],'memory_scope':'Observation only;no supplier-memory acceptance limit was declared'},
   'limits_of_claim':'Finite measured workloads;excludes network,disk autosave and browser rendering; supplier simulation p95 is close to300ms.',
 },
 'corrected_supplier_browser':{
   'status':'pending','passed':None,'outer_proof':None,'inner_proof':None,
   'expected_output_directory':'evidence/S08-supplier-browser-corrected-route-pool-1',
   'active_run_directory':'evidence/S08-supplier-browser-corrected-route-pool-1/supplier-v6VJe7',
   'receipt':None,'paid_maintenance':None,'save_load_continue':None,'commands':None,'errors':None,'screenshots':None,
   'note':'Await completed bound outer and inner results; running progress is not acceptance.',
 },
 'scope_disclosures':[
   'Seven opt-in modeled supplier programmes earn their inputs/stock through ordinary decisions;player skipped;no opening-fleet or first-stock-date guarantee.',
   'Fresh authentic availability evidence is one French APC purchased by Tonga with a disclosed fixed-total budget tradeoff and annual renewal.',
   'A developmental default-budget run did not produce an affordable purchase within3000days.',
   'Separate eleven-platform lifecycle matrix uses synthetic opening company capital,plant,raw inputs and buyer appropriation;not fresh availability qualification.',
   'Domestic ammunition tests cover23recipes;representative foreign branches do not prove genuine availability of all23.',
   'Imported exact specifications grant no component research or domestic manufacturing licence.',
 ],
 'retained_failures':{
   'current_pin_proofs':['failed_general_1','failed_general_1_log','failed_general_2','failed_general_2_log','failed_supplier_browser','failed_supplier_browser_log'],
   'general_first_timeout_cause':'unknown','general_second_failure':'Outside Node preload path spelling;no browser launch',
   'supplier_first_failure':'Last-interceptor cleanup can continue held route before fulfillment;Route is already handled',
   'initial_candidate':'e6187df8044556a8262154e7a6c8baa32b283799',
   'must_retain':['Initial candidate passes attached only to their source','evidence/S08-feature-performance-final-1/ failed bars','Both initial supplier browser attempts,including0xC0000409 and termination-result.json','All intermediate diagnostics,failed fixtures,source snapshots and superseded drafts'],
   'initial_passes_qualify_current_runtime':False,
 },
 'remaining_publication_gates':{
   'corrected_supplier_browser':{'status':'pending','evidence':None},
   'collector_inventory':{'status':'pending','path':None,'sha256':None,'reconstruction_verified':None},
   'final_preservation':{'status':'pending','proof':None,'eight_protected_saves':None,'protected_source_heads':None,'original_fixture_producers':None},
   'isolated_review_launch':{'status':'pending','url':None,'runtime_revision':None,'binary_sha256':None,'save':None,'proof':None},
   'final_test_docs_commit':{'status':'pending','revision':None,'exact_executed_patch_verified':None},
   'git_disposition':{'status':'pending','remote_push':None,'verified_bundle':None,'proof':None},
   'pathways':{'status':'not_updated','required_after_acceptance':'Update both representations consistently;S09 remainsPlanned'},
 },
 'proofs':proofs,
}
target = STAGING/'manifest-publication-d770.json'
target.write_text(json.dumps(document,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
print(json.dumps({'written':str(target),'small_proofs_hashed':len(proofs),'total_proof_bytes':sum(row['bytes'] for row in proofs.values()),'status':document['status']}))
