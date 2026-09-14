"""Publish qualification progress without altering the approved completion boundary."""
import json,pathlib
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
manifest=json.loads((repo/'docs/campaign-certification/S10/final/manifest.json').read_text(encoding='utf-8'))
assert manifest['status']=='gameplay_qualified_scope_pending' and manifest['qualification']['passed']
assert not any(manifest[k] for k in ('s10_complete','c01_complete','g2_earned','s11_started'))
p=repo/'docs/planning/campaign-pathway.json';data=json.loads(p.read_text(encoding='utf-8'))
session=next(s for s in data['sessions'] if s['id']=='S10')
assert session['status']=='in_progress' and 'C01' in session['depends']
session['gameplay_qualification']={'status':'passed_scope_pending','candidate_revision':manifest['candidate_revision'],'evidence':'docs/campaign-certification/S10/final/manifest.json','remaining_closure':'C01 remains an approved prerequisite. The separate content-scope question has no recorded answer.'}
data['execution'].update(stop_boundary='S10 gameplay qualification',status='stopped',previous_stop='S10.h',instruction='Continue. Finish S10',instruction_date='2026-09-13')
assert data['last_completed_session']=='S09' and data['next_session']=='S10'
assert data['gate_decisions']['G2']['status']=='not_earned'
data['gate_decisions']['G2'].update(evidence='docs/campaign-certification/S10/final/manifest.json',reason='All three S10 gameplay acceptance clauses passed final qualification. C01 remains incomplete and an approved S10 prerequisite; no answer to the explicit scope amendment has been received. G2 remains unearned.')
with p.open('w',encoding='utf-8',newline='\n') as f:json.dump(data,f,indent=2,ensure_ascii=False);f.write('\n')
p=repo/'docs/campaign-certification/S10/README.md';old=p.read_text(encoding='utf-8')
head='''# S10 — Government, succession and diplomacy

Status: **Gameplay acceptance passed; S10 closure awaits its C01 dependency decision**.

The [final qualification report](final/README.md) records a clean final candidate,
all eight ordinary country journeys and a separate France two-tab/save journey.
Windows and Linux each pass 387 web, 12 agency/succession, 49 leadership and
1,565 interface tests; the recorded ignored/skipped cases remain disclosed.
102 content tests and five reproduction checks pass. Browser journeys advance
zero campaign days. Five current screenshots were manually reviewed.

C01's worldwide census and leadership histories remain incomplete. The approved
S10 dependency is unchanged while the explicit gameplay/content scope question
awaits an answer. S10 and C01 remain open, G2 is unearned, and S11 is not started.
Latest instruction: “Continue. Finish S10”.

## Historical increment records

The following records preserve the state and claims of their own increments.

'''
assert '## Historical increment records' not in old
rest=old[old.index('## S10.h —'):]
with p.open('w',encoding='utf-8',newline='\n') as f:f.write(head+rest)
p=repo/'docs/CERTIFIED_CAMPAIGN_PATHWAY.md';old=p.read_text(encoding='utf-8')
needle='**Approved pathway · 10 September 2026 · S01–S09 complete; S10 in progress.**'
assert old.count(needle)==1
addition='\n\nS10’s [final gameplay qualification](campaign-certification/S10/final/README.md)\npasses all three gameplay clauses. C01 remains an approved prerequisite and is\nincomplete; the explicit scope amendment is awaiting an answer. S10 and G2\nremain open. Full campaign certification is still later work.'
with p.open('w',encoding='utf-8',newline='\n') as f:f.write(old.replace(needle,needle+addition))
print('Recorded passed gameplay qualification; no completion dependency or gate changed.')
