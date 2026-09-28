from pathlib import Path
import json,hashlib,collections,datetime
out=Path(r'D:/spheres-offload/codex-next-20260928/a1-firing-analysis-01')
review=json.loads((out/'trigger-confidence-check.json').read_text())
mechanical=json.loads((out/'mechanical-check.json').read_text())
source=Path(r'D:/spheres-offload/codex-next-20260928/a1-firing-observer/spheres-sim/src')
lines=[
'# A1 exact-firing diagnostic review',
'',
'Reviewer: Codex `/root/review_gap_submission`. Source-only and retained-output analysis of observer commit `6818e4f0d94b01c86d7a9acc4252260947d13504`; no Cargo, simulation, holdout, acceptance, threshold, or model change. This packet does not certify A1.',
'',
'**Outcome: no additional functional defect established.** The ten recorded elected-government coups across six countries are consistent with the implemented funding, smoothing, pressure, confidence, and trigger rules. A1 remains open. This is not a claim that those design assumptions satisfy calibration.',
'',
'## Evidence and checks',
'',
'- Streamed all 146,525 original JSONL rows. Their stage counts and all ten firing snapshots equal result.json exactly.',
'- The native result reports 252 full-world/RNG/headline comparisons equal; every retained comparison says equal. I did not rerun those simulations or independently reconstruct their intermediate worlds. The two retained final worlds are independently byte-identical: 2,137,811 bytes, SHA-256 `8ff7ce538fc47b0a22a13d25e6af73c04bb35cdc416e710fd82bfe1600150431`.',
'- Recomputed all 21,797 electoral loyalty/pressure updates, 146,525 confidence penalties, 133,647 resource-based targets, and 25,769 recorded trigger branch decisions. No discrepancy above 1e-14 for floating computations. These are checks against recorded inputs, not an independent world replay.',
'- Matched all 38,581 before/after funding pairs: 151 commands were eligible, useful under the existing hysteresis, and affordable; every one applied its exact quoted military share and political-capital charge. The remaining 36,403 had no low-loyalty action condition and 2,027 failed the useful-increase condition. No eligible funding attempt failed for standing in this run.',
'- Each firing had actual hostile loyalty below .35, discontent at least .25, pressure at least 1, a present Army, and at least 12 settled months. The following same-country AI snapshot confirms pressure 0, Army loyalty .90, months-in-office 0, and elected false after all ten breaks.',
'',
'## Causal observations',
'',
'1. Nine firings have zero available allocation under the no-new-borrowing calculation, not zero existing military spending. All twelve preceding monthly funding quotes for each of these nine also capped the proposed allocation at zero. `standing_affordable: true` on such a quote is not an affordable increase: zero would be a priced reduction, and the increase-only guard correctly rejects it. The fiscal constraint, not missing PC or a missed AI review, prevented an increase.',
'2. Myanmar on 2001-12-01 is the only funded-target exception. Funding increased on seven of the preceding twelve reviews. On 2001-06-01 the ordinary command raised military share .04704849127069362 -> .049273248814340156 and target .38431622086852896 -> .39770870200354197. By September the target exceeded .40. At the December walk, actual loyalty rose .3017406363908067 -> .30620961390172247 toward target .40105124774448986; discontent remained .31361289370984735, so pressure rose .9861996805724245 -> 1.0315577803443452. The live trigger correctly reads actual smoothed loyalty rather than the desired target. Immediate forgiveness on buying a target, faster recovery, or a funding-before-trigger reorder would be design changes; this trace does not justify them as repairs.',
'3. All ten firing-time funding quotes were below the existing useful-increase threshold. Thus none is a direct case of a pending eligible appropriation being skipped because the trigger occurs before ai_government. Earlier affordable proposals were applied. The observed order is drift_support -> Army walk -> live coup check -> term limits -> AI funding; politics consolidation follows. This is the explicit production order, not an observer artifact.',
'4. Confidence penalties are positive at all ten firings and are applied once. They use recorded sustained crisis, current mandate and executive leverage. Recomputing those formulas found no double subtraction, stale-month arithmetic, or mismatch with the recorded resource target. Whether the resource and confidence model gives a historically satisfactory country distribution remains a separate unresolved model question.',
'5. Repeated elected-government firings are separated by 34 months (Sao Tome), 35 (Guyana), 65 (Myanmar), and 90 (Guatemala). All reset pressure and loyalty after the earlier break. Sao Tome March 1993 already held pressure above 1 before its 12-month grace elapsed, and then fired on reaching the existing gate; this is accumulated pressure during protected months, not an observed failure to clear pressure at a coup.',
'',
'## Source references and bounded next checks',
'',
'- `government.rs:8765` documents slow-to-buy/quick-to-lose smoothing; `:8874` uses post-walk loyalty for pressure; `:8948` checks live conditions before firing; `:9306` resets the actual break; `:9365` caps funding by existing civilian spending and fiscal headroom; `:9414` uses the existing cadence/hysteresis/paid command; `:9512` and `:9596` show scheduling.',
'- `government.rs:8566` is the current confidence formula. `economy.rs:367` supplies the same fiscal spending and revenue read; `lib.rs:374/:406` supplies standing/price and `:1049` the applied share. `politics.rs:155` protects existing budget ownership and limits consolidation cuts.',
'- Existing tests `electoral_coup_requires_a_current_hostile_army_and_crisis`, `ai_army_funding_is_priced_affordable_and_does_not_override_owned_budgets`, and `ai_can_fund_a_midyear_army_crisis_once_without_free_loyalty_or_tiny_churn` cover the core contracts. I did not run them. If coverage is extended, a useful unchanged-contract fixture is target >= .40 with actual loyalty still < .35 and a live crisis: prove target purchase grants neither instant loyalty nor immunity, and only actual recovery deactivates the trigger. A second fixture can state explicitly that a zero fiscal-cap quote must not be interpreted as an affordable increase.',
'- This diagnostic uses legacy monthly simulation, fixed development seed 0, and 252 months. It does not exercise daily timing, all seeds, reserve cohorts, explicit player/department budgets, or prove the overall calibration gate.',
'',
'## Files and provenance',
'',
'`mechanical-check.json` pins every original diagnostic file and records the first-pass counts. `trigger-confidence-check.json` adds exact source raw/Git-byte pins, the second-pass counts and firing lines. `firing-windows.json` retains 650 selected original snapshots (the firing month and twelve preceding months per firing, with line numbers); `firing-country-funding.json` retains compact funding decisions for the six firing countries. The original 240,585,752-byte JSONL remains untouched.',
'',
'`analyze.py` and `check_trigger.py` are the read-only analysis scripts; their logs are retained. The supplied coordinator log reports one successful ignored diagnostic test in 9.43 seconds; that is not a new execution or a performance measurement by this reviewer. This packet pins the reviewed source and original outputs, but does not invent a compiled-binary/source binding absent from the supplied diagnostic directory.',
'',
'Original observations SHA-256: `13676236845d5c814e640520d322227769d4a7e16c685a237a6ce99a2ca46b49`.',
'Original result SHA-256: `28b9e9177d800e78566c2bec8a878245515932e342c5304ea2f1bcaf6bfca6f6`.',
]
out.joinpath('README.md').write_text('\n'.join(lines)+'\n',encoding='utf8',newline='\n')
log=out.parent/'a1-firing-observer-run-01.log';b=log.read_bytes()
manifest={'format':'spheres-a1-readonly-review-manifest/v1','reviewer':'Codex /root/review_gap_submission','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'qualification':False,'a1_pass_claimed':False,'no_simulation_or_source_edits':True,'input_files':mechanical['input_files']+[{'path':str(log),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}],'source_files':review['source_files'],'artifacts':[]}
for p in sorted(out.iterdir()):
 if p.is_file() and p.name!='manifest.json':
  b=p.read_bytes();manifest['artifacts'].append({'path':p.name,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()})
out.joinpath('manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf8',newline='\n')
for r in manifest['artifacts']:
 b=(out/r['path']).read_bytes();assert len(b)==r['bytes'] and hashlib.sha256(b).hexdigest()==r['sha256']
print('Artifacts verified:',len(manifest['artifacts']));print('README:',hashlib.sha256((out/'README.md').read_bytes()).hexdigest());print('Manifest:',hashlib.sha256((out/'manifest.json').read_bytes()).hexdigest());print('Total compact bytes:',sum(r['bytes'] for r in manifest['artifacts']))
