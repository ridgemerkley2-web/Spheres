import datetime,hashlib,json,pathlib,subprocess
P=pathlib.Path(__file__).parent
ROOT=P.parent/'review-in27'
D=json.loads((P/'all-inputs.json').read_text(encoding='utf-8'))
INDEX=P.parent/'in27-source-preparation/combined-index.json'
ANCHOR=P.parent/'in27-source-preparation/date-anchor-retrieval.json'
research=ROOT/'docs/campaign-certification/C01/research/india.json'
data=json.loads(research.read_text(encoding='utf-8'))
role=next(r for o in data['organizations'] for r in o.get('roles',[]) if r['id']=='in_bjp_president')
def pin(path):
    raw=path.read_bytes()
    return {'path':str(path),'bytes':len(raw),'sha256':hashlib.sha256(raw).hexdigest()}
visual={0:[202,203,246,268],2:[1],3:[28,31,85],7:[1,2],8:[1],9:[1,2],10:[310,438],51:[1],63:[6,7,8,9,12],65:[8],68:[2,6,36],69:[1],70:[1],72:[6,7,36]}
notes=[
 'All 74 original response bodies were rehashed locally against the parent retrieval ledger before reading; this reviewer did not conduct the India HTTP retrievals.',
 'All 167 added claim texts and their material office/date propositions were checked against the relevant original passages. HTML body text and cited PDF text layers were read; 30 cited PDF pages plus both pages of the auxiliary date anchor were rendered and visually inspected. This is not a claim to have read every unrelated page in the six long party-document volumes.',
 'All 19 holder observations were checked, including the three express assumption dates and the one immediate-effect resignation end. No additional holder, boundary, government-office equivalence, avatar permission, or game mapping is approved.',
 'The Jan 1991 Advani source appositive may describe the 1990 narrated event rather than incumbency on the answer date. The packet explicitly preserves that uncertainty and gives only an attestation, not a term interval or opening-day holder.',
 'The auxiliary Rajya Sabha file repeats columns 323-324 and continues the same question on column 325 with the 8 January 1991 date. The source date anchor is supported; no source timestamp or filename alone was used as proof.',
 'Conflicting 1991 compilation headings, retrospective years, 2004 Council/Executive naming, Gadkari 2009 page/list dates, 2016 versus 2017 retrospective re-election years, and January 2026 sole-nominee/result dates remain explicit claims or uncertainties; they do not fabricate starts or ends.',
 'Jana Krishnamurthi acting service and Nadda/Nabin working presidencies remain claim-only under the submission scope. Later substantive stylings and predecessor references are retained rather than erased.',
 'Naidu 1 July 2002 is stated in the contemporaneous party journal. Nadda 20 January 2020 is stated on journal page 6. Nabin 20 January 2026 is stated both in the same-day release and journal page 6; the 19 January notice announces only one proposed name. Laxman resignation acceptance explicitly takes immediate effect on 14 March 2001.',
 'The December 2025 and January 2026 issue publication dates differ from their fortnight labels; pages 36 support 19 December 2025 and 24 January 2026. The August 2026 page publication/text/modification discrepancy is disclosed and not converted into a term boundary.',
 'No actionable material content defect found. Recommend bounded research acceptance only, subject to parent technical/isolation review. C01, C06, S23, WC1 and CP1 remain open; no runtime or portrait acceptance.',
 'No India tracked files edited, no India tests or native/browser commands executed by this reviewer. Parent owns those independent checks and final ordered acceptance.'
]
rows=[]
for i,s in enumerate(D):
    body=pathlib.Path(s['body']); assert pin(body)['sha256']==s['sha256']
    rows.append({'source_id':s['id'],'original':pin(body),'claim_ids_reviewed':[c['id'] for c in s['claims']],
      'material_claim_review':'no_actionable_discrepancy_with_disclosed_scope',
      'cited_pdf_text_pages_read':s['pdf_pages'],'pdf_pages_visually_inspected':visual.get(i,[])})
anchor_body=P.parent/'in27-source-preparation/bodies/advani-date-anchor.pdf'
a=json.loads(ANCHOR.read_text(encoding='utf-8')); assert pin(anchor_body)['sha256']==a['expected_sha256'] and pin(anchor_body)['bytes']==a['expected_bytes']
report={'format':'spheres-independent-research-content-review/v1','task':'CLAUDE-C01-27',
 'reviewer':'Codex /root/review_gap_submission','reviewed_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
 'reviewed_commit':'0765c590c120d3aed3291dccec9d391670110543','research_cutoff':'2026-09-07',
 'recommendation':'accept_bounded_research_subject_to_parent_technical_and_isolation_review',
 'counts':{'exact_source_bodies_rehashed':74,'auxiliary_date_anchor_bodies_rehashed':1,'claims_reviewed':167,'holder_observations_reviewed':19,'express_starts':3,'express_ends':1,'cited_pdf_pages_visually_inspected':30,'auxiliary_pdf_pages_visually_inspected':2,'actionable_content_findings':0},
 'qualification':False,'runtime_acceptance':False,'avatar_acceptance':False,'parent_scope_complete':False,
 'input_ledgers':[pin(INDEX),pin(ANCHOR)],'research_file_raw_worktree_pin':pin(research),
 'research_file_git_blob':subprocess.check_output(['git','rev-parse','HEAD:docs/campaign-certification/C01/research/india.json'],cwd=ROOT,text=True).strip(),
 'notes':notes,'sources':rows,'auxiliary_date_anchor':{'original':pin(anchor_body),'pages_visually_inspected':[1,2]},
 'holder_observations':role['holder_claims'],'technical_tests_run_by_this_reviewer':[],'tracked_edits':[]}
(P/'content-review.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
md='# India BJP presidents: independent content review\n\nReviewer: `Codex /root/review_gap_submission`. Reviewed submission: `0765c590c120d3aed3291dccec9d391670110543`.\n\nRecommendation: **accept the bounded research content**, subject to the parent\'s separate technical and isolation review. This is not task closure, runtime data acceptance, portrait permission, or certification.\n\n74/74 exact original bodies and the separate date-anchor body rehashed; 167 added claims and 19 holder observations reviewed. Thirty cited PDF pages and both date-anchor pages visually inspected. No actionable material content defect found; no India files changed.\n\n'+'\n\n'.join(notes)+'\n\n`content-review.json` records the individual source identities, original SHA-256/byte counts, all reviewed claim IDs, visually inspected pages, and the exact holder observations. Originals and rendered pages remain local; this note does not republish them.\n'
(P/'content-review.md').write_text(md,encoding='utf-8')
print(json.dumps([pin(P/'content-review.json'),pin(P/'content-review.md')],indent=2))
