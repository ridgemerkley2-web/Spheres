"""Assemble the resumed, manually reviewed C01-31 acceptance recommendation."""
import copy,hashlib,json,pathlib,subprocess
HERE=pathlib.Path(__file__).resolve().parent; ROOT=HERE.parents[4]; PRIOR=HERE.parent/'CLAUDE-C01-31'
BASE='f3e18e8306a0a7b1098b91f53f00efbb30da7990'; IMPORT='b74f4fa565429b63052d94cc8ec10337b0b573ed'; CORRECTION='aed12059c63545c3e6d6c5cacaa696a8db53c918'
sha=lambda b:hashlib.sha256(b).hexdigest()
def read(p):return json.loads(p.read_text(encoding='utf8'))
def write(name,data):(HERE/name).write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf8',newline='\n')
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT)
def at(rev,path):return git('show',rev+':'+path)
path='docs/campaign-certification/C01/research/japan.json'
packet=json.loads(at(CORRECTION,path)); original=json.loads(at(IMPORT,path)); base=json.loads(at(BASE,path))
assert at('509bd289',path)==at(BASE,path), 'Current integration changed prior Japan; require combined review'
assert packet['sources'][:472]==base['sources']
new=packet['sources'][472:]; assert len(new)==31
old_claims={c['id']:c for s in original['sources'][472:] for c in s['claims']}
claims={c['id']:c for s in new for c in s['claims']}; assert len(claims)==64
for cid,c in claims.items():
 assert {k:v for k,v in c.items() if k not in ['locator','uncertainty']}=={k:v for k,v in old_claims[cid].items() if k not in ['locator','uncertainty']}
# Restore only the declared appended ranges; all earlier packet values must be exact.
r=copy.deepcopy(packet); r['sources']=r['sources'][:472]
org=next(o for o in r['organizations'] if o['id']=='jp_sangiin_pr_2025_15'); oldorg=next(o for o in base['organizations'] if o['id']==org['id'])
for k in ['sources','claim_ids']:org[k]=org[k][:len(oldorg[k])]
org['coverage']['unresolved']=org['coverage']['unresolved'][:-1]; org['roles']=[]
r['coverage']['unresolved']=r['coverage']['unresolved'][:-1]
assert r==base
retrieval=read(HERE/'retrieval.json'); recovered={r['id']:r for r in retrieval['results']}; assert len(recovered)==7 and all(r['exact'] for r in recovered.values())
readable={r['id']:r for r in read(HERE/'readable-manifest.json')}
notes={
'jp_komeito_kanzaki_fifth_convention_address_20041101':'Read the 1 November 2004 publication line, introduction dated the 31st, guest greeting and following continuation paragraph. The source states reappointment and renewed duty on 31 October; it supplies an observation, not an independently explicit new term boundary. The self-continuation passage is article paragraph 3, not 2.',
'jp_komeito_ota_sixth_convention_20061001':'Read the 1 October 2006 date, 30 September photo caption, opening report, representative address and sole-candidate rule-20 confidence passage. Election and office attestation match. Mixed p/br markup makes old numeric election locators ambiguous, so two unique passage starts identify them. The separately verified explicit Ota assumption statement retains its start.',
'jp_komeito_yamaguchi_selection_greeting_20090909':'Read the 9 September 2009 publication, 8th-meeting introduction and first four article paragraphs. Selection, current new-representative address and former-Ota wording match. The greeting on taking office does not separately date an effective legal start, and former status does not date Ota\'s end.',
'jp_komeito_yamaguchi_ninth_convention_20120923':'Read the 23 September 2012 publication, 22nd-convention caption and opening/reappointed-address passages. Reappointment and subsequent representative speech support the 22 September observation. No effective term boundary is supplied.',
'jp_komeito_ishii_new_representative_interview_20240930':'Read the 30 September 2024 date, 28th-convention executive-launch introduction, and first answer in the assumption-ambitions section. Ishii is named representative in the launched executive; his appointment and former-Yamaguchi eight-term/fifteen-year references are retrospective and undated separately. Retain the 28 September observation without deriving an assumption or predecessor end.',
'jp_komeito_ishii_resignation_intent_20241101':'Read the 1 November 2024 date, 31st-morning report and following explanation, future convention decision and conditional rule. The party reports resignation intention; it does not state an accepted/effective departure day. Keep the 31 October intention claim and no holder end.',
'jp_komeito_saito_recommended_20241108':'Read the 8 November 2024 date and first two paragraphs: the 7th committee recommendation is separate from conditional confidence at the future 9th convention. The source supports a candidate recommendation only. Add paragraph 2 to the locator so the conditional clause is cited; no start is inferred from ministry title or recommendation.'}
assert set(notes)==set(recovered)
sources=read(PRIOR/'source-verification.json')
for source in sources:
 sid=source['source_id']
 if sid in recovered:
  row=recovered[sid]; d=readable[sid]
  source.update(retrieval_decision='exact_original_reproduced',attempt_receipts=source['attempt_receipts']+['../CLAUDE-C01-31-resumed-20261001/retrieval.json'],content_review=notes[sid],body_external=row['body_external'],reproduced_bytes=row['bytes'],reproduced_sha256=row['sha256'],readable_external=d['readable_external'],readable_sha256=d['readable_sha256'],decoding=d['encoding'],content_review_checkpoint='resumed-20261001')
 else:
  source['attempt_receipts']=['../CLAUDE-C01-31/'+p for p in source['attempt_receipts']]
  source['content_review_checkpoint']='prior-review-reused'
 data=pathlib.Path(source['body_external']).read_bytes()
 assert len(data)==source['expected_bytes'] and sha(data)==source['expected_sha256']
 assert sha(pathlib.Path(source['readable_external']).read_bytes())==source['readable_sha256']
# Normalize the recovered source old attempt links to the preserved prior receipt too.
for source in sources:
 source['attempt_receipts']=[('../CLAUDE-C01-31/'+p if p.startswith('retrieval-') else p) for p in source['attempt_receipts']]
write('source-verification.json',sources)
claim_notes={
'jp_komeito_kanzaki_reelected_20041031':'Introduction calls Kanzaki reappointed at the convention held on the 31st; November publication resolves 31 October, supported by the dated article.',
'jp_komeito_kanzaki_self_stated_again_20041031':'Self-address says he again bears the duty after confidence. This is the third article paragraph after introduction and guest greeting; keep attestation only.',
'jp_komeito_ota_selected_20060930':'Opening names the sixth convention on the 30th; the later rule-20 passage states unanimous standing confidence for the sole candidate. The caption explicitly says 30 September.',
'jp_komeito_ota_in_office_convention_20060930':'The report names representative Ota rising to address after selection; this supports the same-day office observation independently of his ministry or Diet role.',
'jp_komeito_yamaguchi_selected_20090908':'Introduction dates the meeting to the 8th; the new representative says he has just been selected with delegates\' confidence. Keep selection separate from tenure.',
'jp_komeito_yamaguchi_in_office_greeting_20090908':'Heading and paragraphs 2/4 name the new representative and his greeting on taking office. Current-office attestation is supported without a separately explicit effective date.',
'jp_komeito_ota_former_20090908':'Paragraph 3 calls Ota former representative while thanking former leadership. Former status supplies no outgoing effective date.',
'jp_komeito_yamaguchi_reelected_20120922':'Opening dates ninth convention to the 22nd and calls Yamaguchi reappointed; the 23 September publication/caption support this calendar day.',
'jp_komeito_yamaguchi_in_office_convention_20120922':'The reappointed party representative rises to address at the same convention. Current duty supports observation, not a new term boundary.',
'jp_komeito_ishii_in_office_convention_20240928':'Opening says the new executive led by representative Ishii launched at the convention on the 28th. Preserve the observation; no separately stated individual assumption date is promoted.',
'jp_komeito_ishii_appointment_recalled':'First answer recalls accepting the party representative appointment after Yamaguchi. No day occurs in that answer, so its structured date stays null.',
'jp_komeito_yamaguchi_former_eight_terms_recalled':'Same answer calls Yamaguchi former representative over eight terms/fifteen years. It proves the bounded retrospective statement without an outgoing date.',
'jp_komeito_ishii_resignation_intent_20241031':'Opening expressly reports intention to resign at the 31st-morning executive meeting. Later future succession procedure does not state effective resignation; no end recorded.',
'jp_komeito_saito_recommended_20241107':'Paragraph 1 reports the 7th recommendation; paragraph 2 expressly makes selection conditional on majority confidence at the 9th convention. No assumption inferred.'}
assert set(claim_notes)==set(read(PRIOR/'held-dependencies.json')['claims'])
cr=read(PRIOR/'claim-review.json')
for row in cr:
 cid=row['claim_id']; row['locator']=claims[cid]['locator']
 if cid in claim_notes:row.update(decision='supported_within_stated_scope',reviewer_observation=claim_notes[cid],content_review_checkpoint='resumed-20261001')
 else:row['content_review_checkpoint']='prior-review-reused'
assert all(r['decision']!='held_original_unavailable' for r in cr)
write('claim-review.json',cr)
hr=read(PRIOR/'holder-review.json')
for row in hr:
 row['prior_missing_source_ids']=row['missing_source_ids']; row['missing_source_ids']=[]
 row['prior_decision']=row['decision']; row['decision']='supported_bounded_holder_observation'
 row['content_review_checkpoint']='resumed-20261001' if row['prior_missing_source_ids'] else 'prior-review-reused'
write('holder-review.json',hr)
write('decision.json',{'recommendation':'accept_bounded_research_with_corrections','submission':'696937ba3d31280aab569cbd78418e9c3a0c6880','integration_base_at_resume':retrieval['current_integration'],'authored_import':IMPORT,'takeya_boundary_correction':'988d537934b4d36e2a251c4b5ca498f19dc7cf06','locator_correction':CORRECTION,'originals_exact':31,'claims_reviewed':64,'holder_observations_reviewed':15,'people':7,'new_originals_this_resume':7,'new_claims_reviewed_this_resume':14,'released_holder_dependencies':5,'remaining_missing_sources':[],'remaining_held_claims':[],'remaining_held_holder_dependencies':[],'parent_gates_closed':[],'limits':'Bounded research only. No complete period, organization identity mapping, runtime installation, artwork/likeness permission, C01, C06, S23, WC1 or CP1 acceptance. Root owns final integration/queue. Prior50claim/10holdercontentreview reused; originalbytes freshly rehashed.'})
prior_scope=read(PRIOR/'scope-audit.json')
paths=[]
for row in prior_scope['authored_paths']:
 p=row['path']; blob=at(CORRECTION,p)
 paths.append({'path':p,'bytes':len(blob),'sha256':sha(blob),'git_blob':git('rev-parse',CORRECTION+':'+p).decode().strip()})
oldpins=[]
for p in sorted(PRIOR.rglob('*')):
 if p.is_file() and '__pycache__' not in p.parts:
  rel=p.relative_to(ROOT).as_posix(); blob=at('d3b4e6da',rel)
  assert p.read_bytes()==blob
  oldpins.append({'path':rel,'bytes':len(blob),'sha256':sha(blob),'git_blob':git('rev-parse','d3b4e6da:'+rel).decode().strip()})
write('scope-audit.json',{'reviewed_source':CORRECTION,'prior_checkpoint':'d3b4e6daede2bc9fc571a88de1b0ba1049505b70','authored_import':IMPORT,'base':BASE,'current_integration_japan_equals_base':True,'earlier_sources_preserved':472,'earlier_claims_preserved':799,'earlier_packet_restored_exactly_by_removing_declared_appends':True,'claim_ids_texts_dates_original_response_identities_preserved':True,'new_locator_changes':{cid:{'before':old_claims[cid]['locator'],'after':claims[cid]['locator']} for cid in claims if old_claims[cid]['locator']!=claims[cid]['locator']},'authored_reviewed_paths':paths,'preserved_prior_receipt':oldpins})
(HERE/'locator-correction.patch').write_bytes(git('diff','--binary','d3b4e6da',CORRECTION,'--','docs/campaign-certification/C01/research','tools/avatars/test_japan_komeito_representatives_c01_31.py'))
print('31 originals rehashed; 64 claims/15 holders reviewed; prior receipt and earlier Japan unchanged. Bounded acceptance recommendation prepared.')
