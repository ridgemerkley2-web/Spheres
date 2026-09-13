"""Record declared timing comparisons for existing diagnostics, never qualification."""
import datetime
import hashlib
import json
import math
import pathlib
import sys

base = pathlib.Path(__file__).resolve().parent
plan_path = base / 'evidence/S08-performance-plan.json'
plan_bytes = plan_path.read_bytes()
assert hashlib.sha256(plan_bytes).hexdigest() == '8d2cb23fe996ba9dc35da0aed5859072a83c2e61b55d4881cf047bc1ac195a28'
bars = json.loads(plan_bytes)['s08_feature_case']
mapping = [
    ('simulation_and_history_recording', 'p95_ms', 'simulation_p95_limit_ms'),
    ('whole_server_turn', 'p95_ms', 'whole_turn_p95_limit_ms'),
    ('whole_server_turn', 'max_ms', 'whole_turn_max_limit_ms'),
    ('equipment_market_read_and_serialization', 'p95_ms', 'equipment_board_read_and_serialization_p95_limit_ms'),
    ('purchase_quote', 'p95_ms', 'purchase_quote_p95_limit_ms'),
]
for name in sys.argv[1:]:
    folder = pathlib.Path(name).resolve()
    profile_bytes = (folder / 'diagnosis.json').read_bytes()
    report = json.loads(profile_bytes)
    checks = []
    for key, field, bar in mapping:
        row = report[key]
        value = row[field]
        assert row['samples'] > 0 and math.isfinite(value) and value >= 0
        checks.append({'metric': key + '.' + field, 'actual_ms': value,
                       'limit_ms': bars[bar], 'within_limit': value <= bars[bar]})
    assessment = {
        'scope': 'Developmental comparison only. Dirty-source diagnostic using an older genuine input; does not qualify a replacement campaign or build.',
        'created_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'profile_sha256': hashlib.sha256(profile_bytes).hexdigest(),
        'plan_sha256': hashlib.sha256(plan_bytes).hexdigest(),
        'runner_sha256': hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest(),
        'checks': checks,
        'all_timing_limits_met': all(check['within_limit'] for check in checks),
    }
    with (folder / 'development-timing-assessment.json').open('x', encoding='utf-8') as handle:
        json.dump(assessment, handle, indent=2)
        handle.write('\n')
    print(json.dumps({'folder': str(folder), 'all_timing_limits_met': assessment['all_timing_limits_met']}))
