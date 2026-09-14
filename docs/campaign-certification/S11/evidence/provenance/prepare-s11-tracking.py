import json, pathlib
base=pathlib.Path(__file__).resolve().parent
repo=base/'integration'
p=repo/'docs/planning/campaign-pathway.json'
d=json.loads(p.read_text(encoding='utf8'))
s=next(s for s in d['sessions'] if s['id']=='S11')
s['status']='in_progress'
s['evidence']='../campaign-certification/S11/README.md'
d['execution'].update(authorized_through='S11',stop_boundary='S11',status='running',previous_stop='S10 gameplay closure',instruction='Next',instruction_date='2026-09-13',next_session_requires_instruction=True)
p.write_text(json.dumps(d,indent=2,ensure_ascii=False)+'\n',encoding='utf8')
p=repo/'docs/CERTIFIED_CAMPAIGN_PATHWAY.md'
t=p.read_text(encoding='utf8').replace('S01–S10 complete; S11–S30 planned.','S01–S10 complete; S11 in progress; S12–S30 planned.').replace('S01–S10 are complete; S11–S30 remain planned.','S01–S10 are complete; S11 is in progress; S12–S30 remain planned.')
t=t.replace('stops after S10; S11 is planned.','stopped after S10. The subsequent “Next” instruction authorizes S11.')
t=t.replace('**Status:** Planned · **Requires:** S05, S09','**Status:** In progress · **Requires:** S05, S09\n\nThe [S11 work record](campaign-certification/S11/README.md) tracks implementation and qualification. S12 remains planned.')
p.write_text(t,encoding='utf8')
out=repo/'docs/campaign-certification/S11';out.mkdir(exist_ok=False)
(out/'README.md').write_text('''# S11 — Ground equipment and operations

Status: in progress. The user’s “Next” instruction authorizes this session after S10 closed. S12 has not started.

This session connects delivered ground equipment to the existing shared national force, maintenance, physical ammunition, manufacturer refits, purchases and retirement. It does not create a second combat strength or vehicle damage system. Maintenance and condition determine supported capability; a manufacturer conversion preserves vehicle age.

Implementation adds the ground equipment desk and role explanations in Operations, dated model-by-model national inventory settlements across simultaneous conflicts, and corrects unsupported ground weapons consuming/firing physical stores. Receipts preserve historical counts after later deliveries, conversions and retirement. Refit reservations remain protected; orders are not delivered stock.

Qualification is pending. Native accounting tests, an authored France player-control journey, full saved-world and history comparisons, relevant regressions and desktop/narrow browser checks will be recorded before closure. Authored fleet/company/conflict preconditions are test setup, not historical evidence or a long campaign.

C01–C07 remain open. S10/G2 gameplay closure is unchanged; G3 and CP1 are not earned by S11 alone.
''',encoding='utf8')
