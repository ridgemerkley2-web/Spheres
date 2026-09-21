"""Validate retained S16 evidence; --apply publishes only S16/G3 completion.
Usage: python publish-s16-closure.py CONFIG.json [--apply]
CONFIG requires candidate_revision and actual visual_review. Paths are replaceable.
No Cargo, Git, server, campaign, collection or staging command runs here.
"""
import argparse, copy, datetime as dt, hashlib, json, math, os, pathlib, re, zlib
BASE=pathlib.Path(__file__).resolve().parent
REPO=BASE/'integration'
DEST=REPO/'docs/campaign-certification/S16'
DRIVER='tools/ui/ci-air-defense.cjs'
RUNTIME_PATHS=['spheres-sim','spheres-web','Cargo.toml','Cargo.lock']
LANES=('web','sim','integration','node','binary','fixture')
TARGETS=['ground_equipment_integration','military_operations','campaign_operations','equipment_integration','company_refits','equipment_supply_automation','company_ammunition','ammunition_reserves','aviation_ammunition','campaign_movement_audit','campaign_peace_audit','companies_integration','s08_supplier_lifecycle']
DEFAULTS={
 'windows':{k:{'proof':f'evidence/S16-final1-{k}.json','log':f'evidence/S16-final1-{k}.log'} for k in LANES},
 'browser_proof':'evidence/S16-final1-browser.json','browser_log':'evidence/S16-final1-browser.log',
 'linux_result':'evidence/S16-final1-linux/result.json','fixture_manifest':'evidence/S16-fixture-final1/manifest.json',
 'linux_runner':'run-s16-linux.py',
 'review_launch':'S16-review-launch.json','preservation':'evidence/S16-preservation-final.json',
 'inventory':'integration/docs/campaign-certification/S16/evidence/inventory.json','selection':'S16-evidence-selection.json',
}
DIRECT={
 'sim':[
  's16_fighter_research_is_prerequisite_gated_dated_and_grants_no_property',
  's16_fighter_profile_has_only_interception_and_rejects_bomber_hardware',
  's16_fighter_v8_air_profiles_remain_sparse_and_frozen_claims_are_checked',
  's16_company_fighter_and_missiles_complete_paid_delivery_upkeep_and_assignment',
  's16_defend_skies_requires_fighter_friendly_geography_and_real_readiness',
  's16_paid_interception_changes_the_real_hostile_strike_and_depleted_defense_loses_it',
  's16_fighter_and_ground_air_defense_have_distinct_paid_contributions',
  's16_multiple_interceptors_and_ground_defense_cannot_write_off_one_target_twice',
  's16_both_sides_and_two_conflicts_share_one_finite_national_store_plan',
  's16_defensive_patrol_has_no_ground_attack_or_hostile_ground_defense_exposure',
  's16_saved_defense_effects_cannot_invent_contacts_suppression_or_losses',
  'air_support_opening_day_edits_use_calendar_time_and_preserve_fiscal_receipts',
  's16_legacy_raid_losses_preserve_parked_fighters_and_their_loss_carry',
 ],
 'web':[
  's16_fighters_offer_only_defense_and_distinguish_friendly_contested_and_enemy_areas',
  's16_defense_review_is_pure_and_maps_a_patrol_area_with_no_ground_attack_claim',
  's16_reports_separate_own_expected_interception_and_ground_defense_losses_from_actual_total',
  's16_fighter_design_library_guidance_and_comparison_use_interception',
 ],
}

def need(condition, message):
    if not condition:
        raise ValueError(message)


def read(p):
    return json.loads(pathlib.Path(p).read_text(encoding='utf-8-sig'),
                      parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value)))


def text_json(value):
    return json.dumps(value, ensure_ascii=False, indent=2, allow_nan=False) + '\n'


def path(value, parent=BASE):
    value = str(value)
    if os.name == 'nt' and re.match(r'^/mnt/[a-z]/', value):
        value = value[5].upper() + ':/' + value[7:]
    p = pathlib.Path(value)
    return (p if p.is_absolute() else parent / p).resolve()


def sha(p):
    with pathlib.Path(p).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def digest(value):
    return isinstance(value, str) and re.fullmatch('[a-f0-9]{64}', value) is not None


def integer(value, minimum=0):
    return type(value) is int and value >= minimum


def finite(value, minimum=0):
    return type(value) in (int, float) and math.isfinite(value) and value >= minimum


def record(p):
    p = path(p)
    need(p.is_file() and not p.is_symlink(), f'Missing or linked evidence: {p}')
    return {'path': str(p), 'bytes': p.stat().st_size, 'sha256': sha(p)}


def exact(row, parent=BASE):
    p = path(row.get('path', row.get('file')), parent)
    found = record(p)
    need(found['sha256'] == row['sha256'], f'Changed evidence: {p}')
    need('bytes' not in row or row['bytes'] == found['bytes'], f'Changed evidence size: {p}')
    return p


def inventory_check(file, selection):
    inv = read(file)
    need(inv['selection_sha256'] == sha(selection) and inv['selection'] == read(selection),
         'Retained selection does not match its exact original')
    index, logical, names, stored = {}, set(), set(), 0
    for row in inv['files']:
        need(row['reconstruction_verified'] is True and row['encoding'] in ('raw', 'gzip')
             and row['storage'], 'Unverified retention row')
        need(row['logical_path'].casefold() not in logical, 'Duplicate logical evidence path')
        logical.add(row['logical_path'].casefold())
        size, result = 0, hashlib.sha256()
        decoder = zlib.decompressobj(31) if row['encoding'] == 'gzip' else None
        for part in row['storage']:
            rel = pathlib.PurePosixPath(part['file'])
            need(not rel.is_absolute() and '..' not in rel.parts and '\\' not in str(rel)
                 and ':' not in str(rel), 'Unsafe retained path')
            need(str(rel).casefold() not in names, 'Duplicate retained storage path')
            names.add(str(rel).casefold())
            p = exact(part, file.parent)
            need(p.is_relative_to(file.parent), 'Retained evidence escaped its directory')
            stored += part['bytes']
            with p.open('rb') as stream:
                while block := stream.read(65536):
                    raw = decoder.decompress(block) if decoder else block
                    result.update(raw)
                    size += len(raw)
        if decoder:
            raw = decoder.flush()
            result.update(raw)
            size += len(raw)
            need(decoder.eof and not decoder.unused_data and not decoder.unconsumed_tail,
                 'Incomplete or concatenated retained gzip stream')
        need(size == row['bytes'] and result.hexdigest() == row['sha256'],
             'Retained bytes cannot reconstruct ' + row['logical_path'])
        index.setdefault(str(path(row['source'])).casefold(), []).append(row)
    need(inv['files'] and stored == inv['stored_bytes'], 'Incorrect retention totals')
    actual = {p.relative_to(file.parent).as_posix().casefold()
              for p in file.parent.rglob('*') if p.is_file()}
    need(actual == names | {'inventory.json'}, 'Unlisted or missing retained storage files')
    return inv, index


def totals(value, log, node=False):
    if node:
        observed = {k: int(re.findall(r'(?:ℹ|#)\s+' + k + r'\s+(\d+)', log)[-1])
                    for k in ('tests', 'pass', 'fail', 'skipped')}
        normalized = {'tests': value['tests'], 'pass': value.get('pass', value.get('passed')),
                      'fail': value.get('fail', value.get('failed')), 'skipped': value['skipped']}
        need(observed == normalized and observed['pass'] > 0 and observed['fail'] == 0
             and observed['tests'] == observed['pass'] + observed['skipped'], 'Node totals differ from log')
    else:
        rows = [tuple(map(int, row)) for row in re.findall(
            r'test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', log)]
        observed = {'passed': sum(r[0] for r in rows), 'failed': sum(r[1] for r in rows),
                    'ignored': sum(r[2] for r in rows), 'completed_targets': len(rows)}
        need(observed == value and observed['passed'] > 0 and observed['failed'] == 0,
             'Native totals differ from log')
    return value



def integration_targets(command):
    return [command[i+1] for i,x in enumerate(command) if x == '--test']


def successful_names(log, names):
    for name in names:
        need(re.search(r'^test [^\n]*\b' + re.escape(name) + r' \.\.\. ok$', log, re.M),
             'Missing passing acceptance regression: '+name)


def qualified_sources(cfg, kept):
    pin=cfg['candidate_revision']
    need(isinstance(pin,str) and re.fullmatch('[a-f0-9]{40}',pin), 'Supply the final complete runtime revision')
    refs,windows,logs,ignored={},{},{},{}
    runner=kept(BASE/'run-s16-check.py')
    need(set(cfg['windows'])==set(LANES),'All six final Windows lanes are required')
    for lane in LANES:
        pp,lp=path(cfg['windows'][lane]['proof']),path(cfg['windows'][lane]['log'])
        proof,log=read(pp),lp.read_text(encoding='utf-8-sig')
        need(proof['lane']==lane and proof['passed'] is True and proof['exit_code']==0, 'Windows '+lane+' did not pass')
        need(proof['revision']==proof['revision_after']==pin and proof['clean_before'] is True and proof['clean_after'] is True,
             'Windows '+lane+' is not the clean intended source')
        need(proof['runner_sha256']==runner['sha256'] and proof['log_sha256']==sha(lp), 'Changed Windows runner/log')
        if lane!='binary': totals(proof['totals'],log,lane=='node')
        if lane=='integration':
            need(integration_targets(proof['command'])==TARGETS and proof['totals']['completed_targets']==len(TARGETS),
                 'Windows integration coverage differs from the thirteen intended targets')
        if lane=='fixture':
            need('s16_export_disposable_air_defense_fixture' in proof['command'] and '--ignored' in proof['command'],
                 'Wrong native fixture export command')
        if lane in DIRECT: successful_names(log,DIRECT[lane])
        if lane in ('sim','web','integration'):
            ignored[lane]=re.findall(r'^test (.+?) \.\.\. ignored([^\n]*)$',log,re.M)
        refs['windows_'+lane]={'proof':kept(pp),'log':kept(lp)}
        windows[lane],logs[lane]=proof,log
    binary=windows['binary']['binary'];exact(binary)
    refs['windows_runner']=runner
    lf=path(cfg['linux_result']);linux=read(lf);lr=kept(path(cfg['linux_runner']))
    expected={'head':pin,'status':''}
    need(linux['format']=='spheres-s16-linux/v1' and linux['revision']==pin and linux['passed'] is True
         and linux['status']=='passed' and linux['source_before']==linux['source_after']==expected,
         'Linux qualification did not pass on the clean runtime')
    need(linux['runner_sha256']==linux['runner_sha256_after']==lr['sha256'],'Linux runner changed')
    need(len(linux['checks'])==4 and {c['name'] for c in linux['checks']}=={'sim','web','integration','node'},'Missing Linux lane')
    for check in linux['checks']:
        need(check['passed'] is True and check['exit_code']==0 and check['source_before']==check['source_after']==expected,
             'Linux lane is incomplete or has changed source')
        lp=path(check['log'],lf.parent);log=lp.read_text(encoding='utf-8-sig')
        need(sha(lp)==check['log_sha256'],'Linux log changed')
        totals(check['totals'],log,check['name']=='node')
        if check['name'] in DIRECT: successful_names(log,DIRECT[check['name']])
        if check['name']=='integration':
            need(integration_targets(check['command'])==TARGETS and check['totals']['completed_targets']==len(TARGETS),
                 'Linux integration coverage differs')
        refs['linux_'+check['name']]=kept(lp)
    toolchain=lf.parent/'toolchain.json';need(sha(toolchain)==linux['toolchain_sha256'],'Linux toolchain record changed')
    refs.update(linux_result=kept(lf),linux_runner=lr,linux_toolchain=kept(toolchain))
    return windows,linux,logs,ignored,refs,binary


def qualified_browser(cfg,kept,binary,windows):
    pin=cfg['candidate_revision'];pp,lp=path(cfg['browser_proof']),path(cfg['browser_log'])
    proof=read(pp);runner=kept(BASE/'run-s16-check.py');driver=kept(REPO/DRIVER)
    need(proof['passed'] is True and proof['exit_code']==0 and proof['log_sha256']==sha(lp),'Browser wrapper did not pass or log changed')
    if 'runtime_revision' in proof:
        # Optional tools-only final driver wrapper. It must disclose exact
        # changed paths and compare all native/runtime trees against the pin.
        driver_pin=proof['driver_revision'];runner=kept(path(cfg['browser_runner']))
        need(proof['runtime_revision']==pin and proof['changed_paths']==[DRIVER]
             and proof['runtime_paths']==RUNTIME_PATHS and proof['runtime_diff']==[], 'Driver changed runtime or unrelated files')
        need(proof['head_after']==driver_pin and proof['clean_before'] is True and proof['clean_after'] is True,
             'Final driver was not clean and stable')
        need(proof['binary_sha256']==binary['sha256'] and proof['driver_sha256']==driver['sha256'],'Final driver/binary changed')
    else:
        driver_pin=pin
        need(proof['lane']=='browser' and proof['revision']==proof['revision_after']==pin
             and proof['clean_before'] is True and proof['clean_after'] is True, 'Browser source changed')
        need(proof['binary_sha256_before']==proof['binary']['sha256']==binary['sha256'], 'Browser executable differs from qualified build')
        exact(proof['binary'])
        need(path(proof['fixture'])==path(windows['fixture']['fixture']), 'Browser used a different native fixture')
    need(re.fullmatch('[a-f0-9]{40}',driver_pin) and proof['runner_sha256']==runner['sha256'],'Browser runner changed')
    emitted=[]
    for line in lp.read_text(encoding='utf-8-sig').splitlines():
        try: row=json.loads(line)
        except ValueError: continue
        if isinstance(row,dict) and row.get('passed') is True and isinstance(row.get('result'),str):
            emitted.append(path(row['result'],REPO))
    need(len(emitted)==1,'Expected exactly one successful browser result in the wrapper log')
    bf=emitted[0]
    if 'browser_result' in cfg: need(bf==path(cfg['browser_result']),'Selected browser result differs from qualified output')
    if 'result' in proof: need(exact(proof['result'])==bf,'Final wrapper result digest differs')
    browser=read(bf)
    need(browser['passed'] is True and browser['errors']==[] and not browser.get('diagnostic_only'),'Browser journey is incomplete')
    need(browser['source']=={'revision':driver_pin,'driver_sha256':driver['sha256']},'Browser result has a different source/driver')
    build=browser['build']
    need(build['revision']==pin and build['binary_sha256']==binary['sha256'] and build['build']['revision'] in (pin,pin[:12]),
         'Browser served a different runtime')
    need(build['assets'],'No served asset verification')
    for name,asset in build['assets'].items():
        need('/' not in name and '\\' not in name and name not in ('.','..'),'Unsafe asset name')
        af=REPO/'spheres-web/ui'/name;raw=af.read_bytes()
        need(asset['served_sha256']==asset['checkout_sha256']==sha(af) and asset['served_bytes']==asset['checkout_bytes']==len(raw),
             'Served asset differs from checkout: '+name)
        need(hashlib.sha256(raw.replace(b'\r\n',b'\n')).hexdigest()==asset['canonical_checkout_sha256']==asset['committed_sha256'],
             'Asset differs from committed canonical bytes: '+name)
    return browser,bf,driver_pin,{'browser_proof':kept(pp),'browser_log':kept(lp),'browser_result':kept(bf),
                                'browser_driver':driver,'browser_runner':runner}


def canonical_comparisons(browser,bf,fixture,ff,kept):
    canonical={}
    for auditfile in (bf.parent/'archive-audit').glob('*.json'):
        a=read(auditfile)
        if 'input' not in a or 'canonical' not in a: continue
        exact(a['input']);exact(a['canonical'])
        need(a['canonical']['ignored_paths']==[],'A native archive audit omitted world fields')
        key,value=a['input']['sha256'],a['canonical']['sha256']
        need(key not in canonical or canonical[key]==value,'Conflicting canonical archive results')
        canonical[key]=value
    need(canonical and browser['world_comparisons'] and browser['envelope_comparisons'],'Missing native save comparisons')
    for c in browser['world_comparisons']:
        need(c['ignored_paths']==[] and digest(c['sha256']),'Invalid world comparison')
        for side in ('left','right'):
            kept(exact(c[side]));need(canonical.get(c[side]['sha256'])==c['sha256'],'World comparison lacks exact retained canonical bytes')
    for c in browser['envelope_comparisons']:
        a,b=c['left'],c['right']
        need(digest(a['sha256']) and a['sha256']==b['sha256'] and a['bytes']==b['bytes']
             and integer(a['saved_unix']) and integer(b['saved_unix']), 'Historical envelope fields differ')
    for phrase in ('Ordinary Load','Save/Load preserves','Continue preserves'):
        need(any(c['message'].startswith(phrase) for c in browser['world_comparisons']),'Missing '+phrase+' comparison')
    for checkpoint,target in fixture['expected_stages'].items():
        found=[c for c in browser['world_comparisons'] if c['message']=='Exact native checkpoint '+checkpoint]
        need(len(found)==1 and path(found[0]['right']['path'])==path(target,ff.parent),'Wrong native checkpoint oracle: '+checkpoint)
    return canonical


def mission_report(flight, key):
    found=[o for o in (flight.get('missions') or {}).get('orders',[]) if o['id']==key]
    need(len(found)==1 and found[0]['status']=='flown' and found[0].get('report'),'Missing flown mission '+str(key))
    r=found[0]['report']
    need(integer(r['aircraft'],1) and integer(r['aircraft_lost']) and r['aircraft_lost']<=r['aircraft']
         and finite(r['sorties']) and finite(r['stores_used']) and finite(r['applied_power']), 'Invalid actual aircraft report')
    return found[0],r


def qualified_journey(cfg,browser,bf,windows,kept):
    pin=cfg['candidate_revision'];ff=path(cfg['fixture_manifest']);fixture=read(ff)
    need(fixture['version']==1 and fixture['fixture']=='s16-authored-air-defense'
         and fixture['compiled_revision'] in (pin,pin[:12]) and fixture['player']=='France','Wrong native S16 fixture')
    need(path(windows['fixture']['fixture'])==ff.parent and path(browser['fixture']['manifest_path'])==ff
         and browser['fixture']['manifest_sha256']==sha(ff),'Fixture binding differs')
    need(exact(browser['fixture'])==ff.parent/fixture['before_file'] and browser['fixture']['scope']==fixture['scope']
         and fixture['authored_preconditions'],'Fixture opening/preconditions differ')
    for directory in (ff.parent,bf.parent):
        for f in directory.rglob('*'):
            if f.is_file() and f.suffix.lower() not in ('.exe','.dll'): kept(f)
    steps=fixture['steps'];commands=[];checkpoints=[];imports=[];refusals=[];days=0
    for s in steps:
        if set(s)=={'command'}: commands.append(s['command'])
        elif set(s)=={'days'}:
            need(integer(s['days'],1),'Invalid native day count');days+=s['days']
        elif set(s)=={'checkpoint'}: checkpoints.append(s['checkpoint'])
        elif set(s)=={'load','file'}:
            kept(ff.parent/s['file']);imports.append(s)
        elif set(s)=={'refusal','command','reason'}: refusals.append(s)
        else: raise ValueError('Unsupported authored step schema: '+str(s))
    need(days==fixture['days_advanced'] and days>0 and len(set(checkpoints))==len(checkpoints)
         and set(checkpoints)==set(fixture['expected_stages']), 'Native journey counts/checkpoints differ')
    need([s['id'] for s in browser['stages']]==['loaded']+checkpoints+['funded_restored','reloaded','continued'], 'Browser omitted/reordered a native checkpoint')
    need(len(browser['commands'])==len(commands) and len(fixture['native_confirmations'])==len(commands),'Command count differs')
    for actual,name,confirmation in zip(browser['commands'],commands,fixture['native_confirmations']):
        payload=actual['payload'];need(actual['stage']==name and len(payload['commands'])==1,'Wrong command sequence')
        native={k:v for k,v in payload['commands'][0].items() if k!='quote'}
        expected={k:v for k,v in confirmation['command'].items() if k!='quote'}
        need(confirmation['id']==name and native==fixture['commands'][name]==expected,'Browser command differs from native confirmation')
        need(isinstance(payload.get('session_id'),str) and payload['session_id'] and isinstance(payload.get('client_id'),str)
             and payload['client_id'] and integer(payload.get('request_seq'),1),'Command lacks protected request identity')
    need(len(browser['advances'])==days and all(a['payload']['days']==1 for a in browser['advances']), 'Wrong ordinary one-day advance count')
    canonical=canonical_comparisons(browser,bf,fixture,ff,kept)
    by={s['id']:s for s in browser['stages']}
    for stage in by.values():
        archive=exact(stage['archive']);kept(archive)
        need(stage['flight']==native_facts(archive),'Browser stage projection differs from its actual save: '+stage['id'])
    funded=by['defense_flown']
    for name in ('funded_restored','reloaded','continued'):
        need(canonical.get(by[name]['archive']['sha256'])==canonical.get(funded['archive']['sha256'])
             and digest(canonical.get(funded['archive']['sha256'])),'Final restoration does not match the funded native checkpoint')
        need(by[name]['date']==funded['date'] and by[name]['flight']['envelope']['sha256']==funded['flight']['envelope']['sha256'],
             'Funded restoration changed its date or historical envelope')
    need(by['loaded']['flight']['as_of_day']==fixture['opening_day'],'Opening native date differs')
    # Imports explicitly rewind to authored saved branches. Check each segment
    # against its native load/checkpoint, not by subtracting the last date from
    # the first and pretending this was one uninterrupted campaign.
    need([s['load'] for s in imports]==['intercept_before','depleted_before'],'Unexpected authored scenario imports')
    need(len(browser['imported_fixtures'])==len(imports)+1,'Missing ordinary imported scenarios or funded-result restoration')
    need(len(browser['refusal_checks'])==len(refusals) and refusals,'Missing depleted-defense refusal')
    # The driver-specific import/refusal schemas are checked by the adapter
    # below; all imported bytes also have exact native checkpoint comparisons.
    validate_imports_and_refusals(browser,imports,refusals,ff,kept)
    r=fixture['required_outcomes'];comparison=fixture['comparison']
    quiet_order,quiet=mission_report(by['quiet_flown']['flight'],r['quiet_mission'])
    defense_order,defense=mission_report(by['defense_flown']['flight'],r['defense_mission'])
    hostile_order,hostile=mission_report(by['defense_flown']['flight'],r['hostile_mission'])
    baseline_order,baseline=mission_report(by['depleted_flown']['flight'],r['hostile_mission'])
    need(comparison=={'quiet':quiet,'defended':defense,'hostile_with_defense':hostile,'unopposed_hostile_baseline':baseline},
         'Observed reports differ from native scenario comparison')
    need(quiet_order['kind']==defense_order['kind']=='defend_skies' and quiet['family']==defense['family']=='air_missile_short_range'
         and quiet['stores_used']>0 and quiet['contacted'] is False and quiet['applied_power']==0
         and quiet['defense']['opposing_missions']==0, 'Quiet patrol has invented contact or free stores')
    need(defense['contacted'] is True and defense['stores_used']>0 and defense['defense']['opposing_missions']==1
         and defense['defense']['prevented_power']>0 and defense['defense']['ground_defense_expected_loss']==0,
         'Fighter did not produce a distinct paid interception result')
    need(hostile_order['kind']==baseline_order['kind']=='strike_target' and hostile['contacted'] is True
         and baseline['contacted'] is True and hostile['stores_used']>0 and baseline['stores_used']>0
         and hostile['defense']['air_combat_expected_loss']>0 and baseline.get('defense') is None
         and 0<=hostile['applied_power']<baseline['applied_power'],'Interception did not reduce the native hostile strike')
    depleted=by['depleted_before']['flight']
    need((depleted.get('ammunition') or {}).get('stocks',{}).get('air_missile_short_range',0)==0,'Depleted comparison has available missiles')
    need(all(o['id']!=r['defense_mission'] for o in by['depleted_flown']['flight']['missions']['orders']), 'Refused dry defense nevertheless launched')
    first,last=by['loaded']['flight'],by['continued']['flight']
    held=lambda f:sum(h['units'] for h in f['holdings'] if h.get('design_id')==r['aircraft_revision'])
    need(held(first)==0 and first.get('aviation') is None,'The browser starts with granted national fighters/squadrons')
    purchase=fixture['commands']['purchase_fighters'];quantity=purchase['quantity']
    deliveries=[d for d in last['deliveries'] if d['company']==r['company'] and d['product']==r['aircraft_product']
                and d['quantity']==quantity and d['purchased_day']>=fixture['opening_day']]
    need(len(deliveries)==1 and deliveries[0]['total_price_bn']>0 and deliveries[0]['settled_day'] is not None
         and deliveries[0]['delivered_day']>deliveries[0]['purchased_day'],'No paid fighter delivery')
    need(held(last)==quantity-quiet['aircraft_lost']-defense['aircraft_lost'], 'Final aircraft holdings do not reconcile to paid delivery and actual losses')
    need(any(s.get('transit') for s in by['delivery_transit']['flight']['aviation']['squadrons']), 'Ordinary geographic transfer was not observed')
    sq=[s for s in (last.get('aviation') or {}).get('squadrons',[]) if s['id']==r['squadron']]
    need(len(sq)==1 and sq[0]['revision']==r['aircraft_revision'] and sq[0]['base']==r['home_base']
         and not sq[0].get('transit'),'Purchased fighters did not reach the defended base')
    need(any(d['family']=='air_missile_short_range' and d['total_price_bn']>0 and d['delivered_day'] is not None
             and d['purchased_day']>=fixture['opening_day']
             for d in last['ammunition_deliveries']),'No delivered paid fighter stores')
    cancelled=[o for o in last['missions']['orders'] if o['id']==r['cancelled_mission']]
    need(len(cancelled)==1 and cancelled[0]['status']=='cancelled','Ordinary patrol cancellation missing')
    expected_outcomes={'paid_fighter_delivery':deliveries[0],'quiet_patrol':quiet,'funded_defense':defense,
        'hostile_with_defense':hostile,'unopposed_hostile_baseline':baseline,
        'prevented_hostile_power':baseline['applied_power']-hostile['applied_power'],
        'cancelled_mission':r['cancelled_mission'],'refusal_checks':browser['refusal_checks'],
        'days_advanced':days,'final_restored_checkpoint':'defense_flown','final_save_slot':'s16-air-defense'}
    need(browser['required_outcomes']==expected_outcomes,'Browser outcome summary differs from retained native observations')
    # Reports belong to separate branches. Do not sum the same hostile mission
    # across alternatives and label that a single campaign casualty total.
    observations={name:{'own_aircraft_lost':row['aircraft_lost'],'stores_used':row['stores_used'],
                        'applied_power':row['applied_power'],'defense':row.get('defense')}
                  for name,row in comparison.items()}
    return fixture,ff,{
        'days_advanced_across_scenarios':days,'ordinary_commands':len(commands),'native_checkpoints':len(checkpoints),
        'authored_imports':len(imports),'final_funded_restorations':1,'refusal_checks':len(refusals),'browser_capture_stages':len(browser['stages']),
        'world_comparisons':len(browser['world_comparisons']),'envelope_comparisons':len(browser['envelope_comparisons']),
        'paid_fighter_delivery':deliveries[0],'native_comparison':comparison,'observed_reports':observations,
        'opening_date':by['loaded']['date'],'final_date':by['continued']['date'],
        'prevented_hostile_power':baseline['applied_power']-hostile['applied_power'],
    }


def prior_and_preservation(cfg,kept):
    road=read(REPO/'docs/planning/campaign-pathway.json')
    need(road['gate_decisions']['G2']['status']=='earned','G2 prerequisite is not earned')
    paths={'G2':REPO/'docs/campaign-certification/S10/final/manifest.json',
           'S11':REPO/'docs/campaign-certification/S11/manifest.json',
           'S12-S15':REPO/'docs/campaign-certification/S15/manifest.json'}
    prior={k:read(p) for k,p in paths.items()}
    need(prior['G2']['g2_earned'] is True and prior['G2']['s10_complete'] is True
         and prior['G2']['c01_complete'] is False and prior['G2']['candidate_revision']==road['gate_decisions']['G2']['candidate'],
         'G2 inherited scope differs')
    need(prior['S11']['status']=='complete' and prior['S11']['candidate_revision']=='d9afd1604219e160cc9deaa4b3999df7db5cde57',
         'S11 inherited qualification differs')
    need(prior['S12-S15']['status']=='complete' and prior['S12-S15']['candidate_revision']=='4c4129abeeec4e9e2987f121e875f5efc54b4c7f'
         and prior['S12-S15']['driver_revision']=='1653638fc050b4b4796875ffdeb7fbff9e7b6d7e', 'S12-S15 inherited qualification differs')
    inherited={k:{'manifest':kept(paths[k]),'candidate_revision':v['candidate_revision'],
                  'driver_revision':v.get('driver_revision'),'scope':v['scope']} for k,v in prior.items()}
    pf=path(cfg['preservation']);p=read(pf);baseline=BASE/'evidence/S08-preservation-before.json';b=read(baseline)
    need(p['passed'] is True and p['baseline_sha256']==sha(baseline),'Original preservation did not pass')
    worktree_file=BASE/'evidence/S16-preservation-before.json';wb=read(worktree_file)
    need(wb['passed'] is True and wb['baseline_sha256']==sha(baseline),'Opening worktree baseline did not pass')
    need(len(p['files'])==len(b['files']) and len(p['worktrees'])==len(wb['worktrees'])
         and {str(path(x['path'])) for x in p['files']}=={str(path(x['path'])) for x in b['files']}, 'Preservation coverage changed')
    baseline_files={str(path(x['path'])):(x['bytes'],x['sha256']) for x in b['files']}
    baseline_heads={str(path(x['path'])):x['head'] for x in wb['worktrees']}
    need({str(path(x['path'])):(x['bytes'],x['sha256']) for x in p['files']}==baseline_files
         and {str(path(x['path'])):x['head'] for x in p['worktrees']}==baseline_heads,'Preservation baseline values changed')
    need(all(x['unchanged'] is True for x in p['files']+p['worktrees'])
         and all(x['head']==x['expected'] for x in p['worktrees']),'Original campaign/worktree preservation changed')
    for x in p['files']: exact(x)
    return inherited,{'record':kept(pf),'baseline':kept(baseline),'worktree_baseline':kept(worktree_file),'files':len(p['files']),'worktree_heads':len(p['worktrees'])}


def native_facts(file):
    original=read(file);world=original;wrappers=[]
    while isinstance(world,dict) and isinstance(world.get('world'),dict):
        wrappers.append({k:v for k,v in world.items() if k!='world'});world=world['world']
    need(isinstance(world,dict) and isinstance(world.get('nations'),list),'Invalid native world')
    nation=next(n for n in world['nations'] if n['id']=='France');equipment=nation.get('equipment') or {};companies=world.get('companies') or {}
    calendar={'year':world['year'],'month':world['month'],'day':world.get('day',1)}
    facts={'calendar':calendar,'as_of_day':(dt.date(calendar['year'],calendar['month'],max(1,calendar['day']))-dt.date(1990,1,1)).days,
        'holdings':nation['arsenal']['held'],'aviation':nation.get('aviation'),'airbases':world.get('airbases'),
        'missions':world.get('air_missions'),'ammunition':equipment.get('ammunition'),'support':equipment.get('air_support'),
        'construction':nation.get('program_budget'),'firms':[f for f in companies.get('firms',[]) if f['nation']=='France'],
        'deliveries':[x for x in companies.get('deliveries',[]) if x.get('buyer')=='France'],
        'ammunition_deliveries':[x for x in companies.get('ammunition_deliveries',[]) if x.get('buyer')=='France'],'envelope':None}
    if 'world' in original:
        need(set(original)=={'format','version','world','history','log','history_epoch','saved_date','player','saved_unix'}
             and original['format']=='spheres-campaign' and original['version']==1 and integer(original['saved_unix']), 'Invalid historical campaign envelope')
        timestamp=wrappers[0].pop('saved_unix')
        raw=json.dumps(wrappers,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode('utf-8')
        facts['envelope']={'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'saved_unix':timestamp,
                           'history_points':len(original['history']),'dispatches':len(original['log'])}
    return facts


def validate_imports_and_refusals(browser,imports,refusals,ff,kept):
    expected=[('s16-'+x['load'],ff.parent/x['file'],True) for x in imports]
    expected.append(('s16-funded-result',ff.parent/read(ff)['expected_stages']['defense_flown'],False))
    for row,(slot,file,loaded) in zip(browser['imported_fixtures'],expected):
        need(row['slot']==slot and exact(row)==file and (not loaded or row.get('loaded') is True),'Wrong authored imported save')
        kept(file)
    for row,step in zip(browser['refusal_checks'],refusals):
        need(row['id']==step['refusal'] and row['valid'] is False and row['intended_command']==step['command']
             and row['native_reason']==step['reason'] and row['blockers'],'Wrong native depleted-defense refusal')
        details=[x if isinstance(x,str) else x.get('detail',x.get('reason','')) for x in row['blockers']]
        need(any(step['reason'] in text for text in details),'Displayed refusal omitted the native reason')


def qualified_review(cfg,browser,bf,binary,kept):
    pin=cfg['candidate_revision'];rf=path(cfg['review_launch']);review=read(rf)
    stage_name=cfg.get('review_stage','continued')
    stages={s['id']:s for s in browser['stages']};need(stage_name in stages,'Unknown final review stage')
    stage=stages[stage_name]
    need(review['runtime_revision']==pin and review['executable_sha256']==binary['sha256']
         and review['build']['revision'] in (pin,pin[:12]) and review['player']=='France'
         and review['date']==stage['date'] and review['required_outcomes']==browser['required_outcomes'],'Review campaign differs from final qualified result')
    need(isinstance(review['url'],str) and re.match(r'^http://127\.0\.0\.1:\d+/?$',review['url']), 'Review URL is not the separate local campaign')
    need(len(review['evidence'])==2 and {str(exact(x)) for x in review['evidence']}==
         {str(path(cfg['windows']['binary']['proof'])),str(bf)},'Review proof links differ')
    directory=path(review['directory']);need(sha(directory/'spheres-web.exe')==binary['sha256'],'Copied review executable changed')
    source,copied=path(review['save_copy']['source']),path(review['save_copy']['copy'])
    default_source=path(browser['run'])/'saves/s16-air-defense.json'
    need(source==path(cfg.get('review_source_save',default_source)) and copied.is_relative_to(directory)
         and sha(source)==sha(copied)==review['save_copy']['sha256'],'Review save copy differs')
    native=review['native_verification'];matches=[x for x in browser['world_comparisons']
        if any(x[side]['sha256']==stage['archive']['sha256'] for side in ('left','right'))]
    need(matches and len({x['sha256'] for x in matches})==1 and native['ignored_paths']==[]
         and native['exact_bytes_match_final_browser'] is True and native['canonical_sha256']==matches[0]['sha256'],
         'Review native world differs from selected final browser stage')
    af=directory/'native-verification/copied-save.json';audit=read(af)
    need(exact(audit['input'])==copied and audit['input']['sha256']==review['save_copy']['sha256']
         and audit['canonical']['ignored_paths']==[] and audit['canonical']['sha256']==native['canonical_sha256']
         and audit['canonical']['bytes']==native['bytes'],'Copied save native audit differs')
    worker=kept(REPO/'tools/ui/archive-worker.py');need(worker['sha256']==native['worker_sha256'],'Native archive worker changed')
    return {'url':review['url'],'stage':stage_name,'launch':kept(rf),'native_verification':native}, {
        'review_launch':kept(rf),'review_save':kept(copied),'review_source_save':kept(source),
        'review_audit':kept(af),'review_canonical':kept(exact(audit['canonical'])),'archive_worker':worker}


def qualified_visual(cfg,browser,bf,kept):
    visual=cfg.get('visual_review')
    need(isinstance(visual,dict) and visual.get('passed') is True and set(visual.get('widths',[]))=={1440,390}
         and visual.get('screenshots') and isinstance(visual.get('notes'),str) and visual['notes'].strip()
         and 'REPLACE' not in visual['notes'],'Supply actual final desktop/mobile visual inspection findings')
    need(len(set(visual['screenshots']))==len(visual['screenshots']),'Repeated visual-review capture')
    for name in visual['screenshots']:
        rel=pathlib.PurePosixPath(name)
        need(not rel.is_absolute() and '..' not in rel.parts and name in browser['screenshots'],'Unknown visual-review capture')
        kept(bf.parent/name)
    need(all(any(str(width) in name for name in visual['screenshots']) for width in (1440,390)),
         'The final inspected desktop and mobile captures are required')
    inspection=browser['fighter_inspection']
    need(inspection['spec']['platform']=='air_fighter' and inspection['canvas'] is True
         and isinstance(inspection['status'],str) and inspection['status']
         and not re.search(r'could not|unavailable|Preparing',inspection['status'],re.I),'Fighter inspection asset was not available')
    readings=browser['map_readings']
    need({r['width'] for r in readings}=={1440,390},'Missing final map reading at the two supported widths')
    for row in readings:
        need(row['backVisible'] is True and finite(row['camera']['k']) and row['camera']['k']>1.2
             and row['rangeKm']==900 and row['target'] and row['screenshot'] in browser['screenshots'],
             'Patrol map focus, range or return control failed')
        kept(bf.parent/row['screenshot'])
    return visual


def qualify(config,config_file):
    cfg=copy.deepcopy(DEFAULTS)
    if 'proof_label' in config:
        label=config['proof_label'];need(isinstance(label,str) and re.fullmatch('[A-Za-z0-9_-]+',label),'Unsafe final proof label')
        cfg['windows']={k:{'proof':f'evidence/S16-{label}-{k}.json','log':f'evidence/S16-{label}-{k}.log'} for k in LANES}
        cfg.update(browser_proof=f'evidence/S16-{label}-browser.json',browser_log=f'evidence/S16-{label}-browser.log',
                   linux_result=f'evidence/S16-{label}-linux/result.json',fixture_manifest=f'evidence/S16-fixture-{label}/manifest.json')
    cfg.update(config)
    invfile,selection=path(cfg['inventory']),path(cfg['selection'])
    need(invfile==DEST/'evidence/inventory.json','Only the new S16 evidence tree is eligible')
    inv,index=inventory_check(invfile,selection)
    def kept(file):
        p=path(file);r=record(p)
        found=[x for x in index.get(str(p).casefold(),[]) if x['sha256']==r['sha256'] and x['bytes']==r['bytes']]
        need(found,'Required complete source was not retained: '+str(p))
        r.update(logical_path=found[0]['logical_path'],storage=found[0]['storage']);return r
    windows,linux,logs,ignored,refs,binary=qualified_sources(cfg,kept)
    browser,bf,driver_pin,br=qualified_browser(cfg,kept,binary,windows);refs.update(br)
    fixture,ff,journey=qualified_journey(cfg,browser,bf,windows,kept);refs['fixture_manifest']=kept(ff)
    inherited,preservation=prior_and_preservation(cfg,kept)
    review,rr=qualified_review(cfg,browser,bf,binary,kept);refs.update(rr)
    visual=qualified_visual(cfg,browser,bf,kept)
    refs.update(inventory=record(invfile),selection=kept(selection),publication_config=kept(config_file),publisher=kept(pathlib.Path(__file__)),
                collector=kept(BASE/'collect-s16-evidence.py'))
    # Retain these exact source bytes as well as their source revision. This is
    # intentionally bounded to the changed feature and its gate regressions.
    sources=[DRIVER,'spheres-sim/src/equipment.rs','spheres-sim/src/equipment_aviation.rs',
        'spheres-sim/src/equipment_air_support.rs','spheres-sim/src/equipment_air_support_tests.rs',
        'spheres-sim/src/operations.rs','spheres-sim/src/operations_aviation_loss_tests.rs',
        'spheres-sim/src/equipment_fighter_tests.rs','spheres-sim/src/airmissions.rs',
        'spheres-sim/src/airmissions_defense_tests.rs','spheres-web/src/s16_fixture_tests.rs',
        'spheres-web/src/equipment_flight_view.rs','tools/ui/archive-worker.py']
    source_refs={name:kept(REPO/name) for name in sources}
    extras=[kept(path(x)) for x in cfg.get('extra_evidence',[])]
    completed=dt.datetime.fromisoformat(browser['finished_utc'].replace('Z','+00:00'))
    need(completed.tzinfo is not None,'Browser completion timestamp lacks timezone')
    scope=('Bounded authored France fighter lifecycle and three saved air-defense scenarios, with explicit Italian opposing orders. '
           'G3 joins this S16 proof to the exact inherited G2 and S11–S15 evidence; it does not certify an unassisted campaign or S17 AI.')
    return {
        'format':'spheres-s16-closure/v1','sessions':['S16'],'status':'complete','g3_earned':True,'cp1_earned':False,'s17_started':False,
        'candidate_revision':cfg['candidate_revision'],'driver_revision':driver_pin,'completed_utc':browser['finished_utc'],
        'completed_date':completed.astimezone(dt.timezone.utc).date().isoformat(),'scope':scope,
        'gate':{'id':'G3','status':'earned','criterion':'G2 + S11–S16: Ground and three supported air missions use real campaign forces and results.',
                'inherited':inherited,'s16_acceptance_tests':DIRECT},
        'qualification':{'passed':True,'windows':{k:windows[k].get('totals',{'passed':True}) for k in LANES},
            'linux':{c['name']:c['totals'] for c in linux['checks']},'native_ignored_tests':ignored,
            'browser':{'passed':True,'runtime_revision':cfg['candidate_revision'],'driver_revision':driver_pin,
                       'driver_only_change':driver_pin!=cfg['candidate_revision'],'runtime_diff':[]},
            'direct_regressions':DIRECT,'ignored_world_paths':[],'history_timestamp_exception':'saved_unix only, retained and validated separately',
            **{k:v for k,v in journey.items() if k not in ('native_comparison','observed_reports','paid_fighter_delivery')}},
        'build':browser['build'],'scenario':{'country':'France','scope':fixture['scope'],
            'authored_preconditions':fixture['authored_preconditions'],'opening_date':journey['opening_date'],'final_date':journey['final_date'],
            'final_restored_checkpoint':'defense_flown','review_stage':review['stage'],'final_save_slot':'s16-air-defense'},
        'outcomes':browser['required_outcomes'],'observed_reports_by_scenario':journey['observed_reports'],
        'review':review,'visual_review':visual,'preservation':preservation,'source_files':source_refs,'proofs':refs,'extra_evidence':extras,
        'retention':{'files':len(inv['files']),'stored_bytes':inv['stored_bytes'],'all_reconstructions_verified':True,
                     'inventory_sha256':sha(invfile),'scope':inv['scope']},
        'limits':[
            'The opening manufacturer capital/resources, researched fighter integrations, bases, conflict and opposing aircraft/stores are authored preconditions, not historical opening allocations.',
            'Research, component legality, paid development, production, delivery, upkeep and automatic stores have direct native regressions; the browser starts after the manufacturer and legal fighter integrations are ready.',
            'Interception covers explicit supported Support army and Strike target orders. The hostile browser order is authored; autonomous opponent choices remain S17 work.',
            'Three authored saved branches share history. Their alternative hostile results and casualty counts must not be summed into one uninterrupted campaign total.',
            'Browser comparisons cover the named native checkpoints and funded-result restore/Save/Load/Continue, not every intermediate day or an injected lost network response.',
            'Coordinated ground-air defense, multiple fighters, two countries/conflicts, finite shared stores and forged saved effects have separate native regressions; the browser claims only its recorded contacts and losses.',
            'Desktop 1440px and mobile 390px were inspected. No 320px result, independent human playtest, finished S18 aircraft-art gate or new performance certificate is claimed.',
            'G3 inherits exact earlier gate/session evidence with its original source and authored scope. It does not re-run every prior browser journey on this runtime.',
            'Preservation covers only the baseline named files and worktree HEADs, whose observed counts are recorded in this manifest.',
            'S17 and later sessions remain planned. Historical/content coverage, long-run campaigns, release qualification and CP1 remain open.'],
    }


def render(manifest):
    roadmapfile=REPO/'docs/planning/campaign-pathway.json';pathwayfile=REPO/'docs/CERTIFIED_CAMPAIGN_PATHWAY.md'
    original=read(roadmapfile);data=copy.deepcopy(original)
    sessions={s['id']:s for s in data['sessions']}
    need(sessions['S16']['status'] in ('in_progress','complete') and sessions['S17']['status']=='planned',
         'S16 is not the active closure boundary or S17 has started')
    need(all(sessions[f'S{i:02d}']['status']=='complete' for i in range(1,16)), 'An inherited core session is not complete')
    need(data['execution']['authorized_through']==data['execution']['stop_boundary']=='S16','Unexpected execution authorization')
    sessions['S16'].update(status='complete',evidence='../campaign-certification/S16/README.md',completed_date=manifest['completed_date'],
        closure={'evidence':'docs/campaign-certification/S16/manifest.json','candidate_revision':manifest['candidate_revision'],
                 'driver_revision':manifest['driver_revision'],'completed_utc':manifest['completed_utc']})
    data['date']=manifest['completed_date'];data['last_completed_session']='S16';data['next_session']='S17'
    data['execution'].update(status='stopped',next_session_requires_instruction=True)
    decision={'status':'earned','date':manifest['completed_date'],'candidate':manifest['candidate_revision'],
        'evidence':'docs/campaign-certification/S16/manifest.json',
        'reason':'Ground and the three supported air missions use real campaign forces and results. G3 combines the exact inherited G2 and S11–S15 records with the pinned S16 fighter lifecycle, coordinated interception and depleted-defense regressions, plus the authored France browser comparison. Explicit opposing orders are authored; S17 AI, later content/campaign/performance qualification and CP1 remain open.'}
    if 'G3' in data['gate_decisions']:
        need(data['gate_decisions']['G3']==decision,'A different G3 decision already exists')
    data['gate_decisions']['G3']=decision
    phase=next(x for x in data['phases'] if x['id']=='military')
    need(phase['gate'] in ('G3 · Complete military loop','G3 · Complete military loop ✓'),'Unexpected military gate')
    phase['gate']='G3 · Complete military loop ✓'
    need({k:v for k,v in data['gate_decisions'].items() if k!='G3'}==
         {k:v for k,v in original['gate_decisions'].items() if k!='G3'},'An unrelated gate decision changed')
    need([s for s in data['sessions'] if s['id']!='S16']==[s for s in original['sessions'] if s['id']!='S16'],
         'An unrelated session changed')
    need([p for p in data['phases'] if p['id']!='military']==[p for p in original['phases'] if p['id']!='military'],
         'An unrelated phase changed')
    pathway=pathwayfile.read_text(encoding='utf-8-sig')
    old_heading='S01–S15 complete; S16 in progress; S17–S30 planned.'
    new_heading='S01–S16 complete; S17–S30 planned.'
    need(old_heading in pathway or new_heading in pathway,'Cannot locate current pathway status')
    pathway=pathway.replace(old_heading,new_heading)
    pathway=pathway.replace('S01–S15 are complete; S16–S30 remain planned.','S01–S16 are complete; S17–S30 remain planned.')
    start=pathway.index('#### S16 —');end=pathway.index('\n### 4 ·',start)
    section=pathway[start:end]
    need(section.count('- [ ]') in (0,3) and section.count('- [x]') in (0,3),'Unexpected S16 acceptance section')
    section=re.sub(r'\*\*Status:\*\* (?:In progress|Complete)','**Status:** Complete',section,count=1).replace('- [ ]','- [x]')
    evidence=('**Evidence and gate decision:** [S16 fighter and defensive aviation qualification](campaign-certification/S16/README.md). '
              '**G3 earned** with the exact inherited G2 and S11–S15 evidence. S17 and CP1 remain open.')
    if evidence not in section: section=section.rstrip()+'\n\n'+evidence+'\n\n'
    pathway=pathway[:start]+section+pathway[end:]
    old=('On 14 September 2026, “continue” authorized S16. Fighter research, supplier delivery and Defend skies are in development; '
         'S17 remains outside this instruction. G3 has not yet been earned.')
    new=('On 14 September 2026, “continue” authorized S16. S16 is complete on the exact source and evidence in its '
         '[qualification record](campaign-certification/S16/README.md). **G3 is earned** for ground and the three supported air missions, '
         'combining the retained earlier qualification with the authored fighter/interception comparison and direct native regressions. '
         'Opposing orders in this journey are authored. Execution stopped after S16; S17 requires a new instruction. '
         'CP1 and later campaign, content, human-usability and performance requirements remain unearned.')
    need(old in pathway or new in pathway,'Cannot locate current S16 authorization paragraph')
    pathway=pathway.replace(old,new)
    q=manifest['qualification'];rows=[]
    for platform,checks in [('Windows',q['windows']),('Linux',q['linux'])]:
        for lane in ('web','sim','integration','node'):
            t=checks[lane];rows.append(f"| {platform} {lane} | {t.get('pass',t.get('passed'))} | {t.get('skipped',t.get('ignored',0))} |")
    outcomes=manifest['outcomes'];comparison=manifest['observed_reports_by_scenario'];result_rows=[]
    for name,title in [('quiet','Quiet patrol'),('defended','Funded fighter defense'),('hostile_with_defense','Hostile strike with interception'),('unopposed_hostile_baseline','Alternative strike after missile depletion')]:
        r=comparison[name];result_rows.append(f"| {title} | {r['stores_used']:.6g} | {r['applied_power']:.6g} | {r['own_aircraft_lost']} |")
    same_driver=manifest['driver_revision']==manifest['candidate_revision']
    driver_note=('The native, Node and browser proofs use the same clean source revision.' if same_driver else
        f"The final browser driver revision is `{manifest['driver_revision']}`. Its only change from the runtime candidate is `{DRIVER}`; native/web/Cargo diffs are empty. The native suites and executable qualify the unchanged runtime, and the final browser qualifies the driver.")
    readme=f"""# S16 — Fighters and defensive aviation

Status: **complete** on runtime `{manifest['candidate_revision']}`. **G3 is earned**; execution stops after S16. S17 remains planned and CP1 remains unearned.

The researched `air_fighter` platform has company development and fabrication, paid national purchase and dated delivery, exact squadron assignment, geographic basing, paid upkeep and finite `air_missile_short_range` stores. **Defend skies** uses its interception profile to confront explicit hostile Support army or Strike target orders. Fighter and ground-air-defense contributions remain separate, with one coordinated physical aircraft-loss settlement. Research grants no aircraft or stores, and fighters do not acquire an implicit bombing attack.

The authored France journey completed **{q['ordinary_commands']} ordinary reviewed commands**, **{q['days_advanced_across_scenarios']} one-day advances across saved scenarios**, **{q['native_checkpoints']} native checkpoints**, and **{q['refusal_checks']} visible native refusal**. It purchased and received **{outcomes['paid_fighter_delivery']['quantity']} fighters**, purchased compatible finished missiles, formed and transferred a squadron, cancelled a patrol, flew a funded quiet patrol, and intercepted the authored hostile strike. Loading the explicitly depleted alternative visibly refused defense and allowed the stronger baseline strike. The final ordinary restore selected **defense_flown**, then completed Save/Load/Continue using `s16-air-defense` for review.

| Observed scenario result | Finished stores used | Applied power | Own whole-aircraft losses |
|---|---:|---:|---:|
{chr(10).join(result_rows)}

The hostile strike's applied power was reduced by **{outcomes['prevented_hostile_power']:.6g}** in the funded comparison. These are alternative authored saved branches; their hostile outcomes and losses are not summed into an uninterrupted campaign total. The [manifest](manifest.json) preserves exact reports, authored preconditions and every observed whole-aircraft loss.

| Qualification | Passed | Ignored / skipped |
|---|---:|---:|
{chr(10).join(rows)}

The separate fixture export passed. Both native platforms include all thirteen selected integration targets and every named S16 acceptance regression. Existing ignored native tests and skipped Node cases are listed with the observed counts; they are not passes. Direct native coverage separately checks legal research and paid supplier development, depleted defenses, distinct ground-air defense, multiple interceptors, two countries/conflicts sharing finite stores, no double destruction and invalid saved-effect rejection.

The final browser completed **{q['world_comparisons']} full native-world comparisons** and **{q['envelope_comparisons']} historical-envelope comparisons**, with no ignored world fields. Only the separately validated save timestamp may differ in an envelope. Comparisons cover the named checkpoints and funded restoration/Save/Load/Continue; they do not claim every intermediate day or an injected network failure. {driver_note}

Desktop **1440px** and mobile **390px** captures were inspected: {manifest['visual_review']['notes']}

G3's exact criterion is **G2 + S11–S16: Ground and three supported air missions use real campaign forces and results.** Its prerequisites retain their original pinned scopes: [G2](../S10/final/manifest.json), [S11 ground operations](../S11/manifest.json), and [S12–S15 flight lifecycle and tactical missions](../S15/manifest.json). This S16 record adds the third supported air mission. Earlier browser journeys are inherited evidence on their original runtimes, not asserted reruns on this candidate.

The [inventory](evidence/inventory.json) retains **{manifest['retention']['files']} complete selected evidence files**, each with exact original/storage hashes and verified byte reconstruction. Large files use lossless gzip and may have numbered pieces; concatenate pieces before decompression. Full qualifying logs, raw saves, native oracles, canonical audits, screenshots, runners and bounded source copies are retained. Executables are identified by source and SHA-256 rather than repackaged. Prior session evidence remains preserved.

[Open the separate review campaign]({manifest['review']['url']}). Its copied final save and native audit match the **{manifest['review']['stage']}** browser stage. Preservation checks retain the baseline's **{manifest['preservation']['files']} named files** and **{manifest['preservation']['worktree_heads']} worktree HEADs**.

The native manufacturer, fighter research, authored bases/conflict and Italian aircraft/stores are disclosed preconditions. This is not historical opening equipment, an autonomous opponent or an unassisted campaign. The browser begins after manufacturer preparation; native tests separately cover research and company development. No 320px visual check, independent human playtest, finished S18 art gate, new performance qualification or complete 1990–2035 campaign certificate is awarded. **Next: S17**, requiring a new instruction. Historical/content, later campaign and release obligations remain open; **CP1 is unearned**.
"""
    return {DEST/'manifest.json':text_json(manifest),DEST/'README.md':readme,
            roadmapfile:text_json(data),pathwayfile:pathway}


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('config',type=pathlib.Path);parser.add_argument('--apply',action='store_true');args=parser.parse_args()
    manifest=qualify(read(args.config),args.config.resolve());outputs=render(manifest)
    allowed={DEST/'manifest.json',DEST/'README.md',REPO/'docs/planning/campaign-pathway.json',REPO/'docs/CERTIFIED_CAMPAIGN_PATHWAY.md'}
    need(set(outputs)==allowed and all(p.is_relative_to(REPO) for p in outputs),'Unexpected closure write target')
    before={p:p.read_bytes() if p.exists() else None for p in outputs}
    changes=[{'path':p.relative_to(REPO).as_posix(),'before_sha256':hashlib.sha256(before[p]).hexdigest() if before[p] is not None else None,
              'after_sha256':hashlib.sha256(value.encode('utf-8')).hexdigest(),'changed':before[p]!=value.encode('utf-8')}
             for p,value in outputs.items()]
    if args.apply:
        for p in outputs:
            need(p.parent.is_dir() and not p.is_symlink(),'Missing or linked completion document')
            need((p.read_bytes() if p.exists() else None)==before[p],'Completion document changed during validation')
        for p,value in outputs.items(): p.write_text(value,encoding='utf-8',newline='\n')
    print(text_json({'passed':True,'applied':args.apply,'candidate_revision':manifest['candidate_revision'],
        'driver_revision':manifest['driver_revision'],'g3_earned':manifest['g3_earned'],'cp1_earned':False,
        'qualification':manifest['qualification'],'changes':changes}))


if __name__=='__main__':
    main()
