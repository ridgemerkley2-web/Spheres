"""Record the independent human-readable content rulings, not an automatic fact checker."""
import hashlib
import json
from pathlib import Path

EV = Path(__file__).resolve().parent
ROOT = EV.parents[5]
RAW = Path('D:/spheres-offload/codex-next-20260928/c01-41-originals')
packet = json.loads((ROOT / 'docs/campaign-certification/C01/research/ussr.json').read_text('utf8'))
sources = packet['sources'][56:]

def save(name, value):
    (EV / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf8', newline='\n')

notes = [
    'The dated plenum communique names M. S. Gorbachev in the exact office on 5 February. Full name is corroborated by the July biography; no January-opening or continuous-term inference.',
    'The 10 July communique lists Avaliani and Gorbachev on the secret ballot; nomination/ballot evidence only.',
    'The communique separately reports voting during the evening session of 10 July, without converting it into an office boundary.',
    'The result and approval of the counting-commission protocol are printed for 10 July. An election event, not an assumed start.',
    'The final paragraph styles Gorbachev as General Secretary while reporting his address after the result. Supports the dated observation, not a complete term.',
    'The adjacent biography prints his full name and dates re-election to 10 July. Retained as a separate biographical election claim.',
    'The 11 July communique styles Gorbachev as General Secretary while reporting that he presided after the break. Separate intermediate attestation.',
    'The 11 July communique lists Dudyrev, Ivashko and Ligachev as deputy candidates. No election result inferred from candidacy.',
    'The 11 July communique reports voting for deputy and control-commission chair. Distinguished from the next-day announcement.',
    'The 12 July communique reports Ivashko elected and the protocols approved. No effective-day claim or holder start inferred.',
    'The Pravda correspondents attribute the tally to the counting-commission chair: 150/4268, 3109/1309 and 776/3642. Printed thousands separators retained. Reported figures, not a recovered protocol.',
    'The biography prints Vladimir Antonovich Ivashko and states election on 11 July, independently of the 12 July announcement. Both remain separate event claims.',
    'The 13 July communique dates formation of the programme commission; its list names Gorbachev with the General Secretary title. A list attestation, not a new term.',
    'The programme-commission list styles V. A. Ivashko as Deputy General Secretary; his full name is supported by the previous issue biography. Supports 13 July without resolving old Ivashkov spelling.',
    'The editorial-board list supplies the deputy title. The imprint says signed to press 10 July; the cover says August. Retain a claim dated by the imprint with this disclosed limitation, not a start.',
    'The page 2 article is signed TASS and reports Dzasokhov referring to Gorbachev in office at the 21 August press conference. Reprinted reportage, not a signed party statement; attribution clarified.',
    'The same TASS-attributed report names V. Ivashko with the exact deputy title and says he flew that day. The opening dates the conference 21 August. Accept only the attributed observation, not an appointment or verbatim transcript.',
    'The statement is signed by the Secretariat and names Gorbachev in the exact office. No separate date; 22 August is the newspaper issue date and remains a publication-date attestation.',
    'The speaker is Gorbachev; pages 32 and 34 state he laid down General Secretary duties. Neither gives that act a date. The 26 August speech does not prove an August 24 until.',
    'The same speech proposes Central Committee self-dissolution. A proposal is not a decision or accomplished dissolution.',
    'Gazette article 1024 orders safeguarding party property and measures for workers of committees ceasing activity; signed 24 August. Does not itself dissolve the party or end a party office.',
    'Medvedev is identified on page 11 and refers to Gorbachev resignation. Another deputy reference dated by the 3 September sitting, without a resignation day.',
    'The sentence continues onto page 12 and states the Central Committee did not decide self-dissolution. Accepted as Medvedev statement, not a recovered committee resolution.',
    'Gazette article 1038 point 7 suspends CPSU activity throughout USSR; dated 29 August. Suspension is not dissolution and yields no office-end or lifecycle date.',
    'Both the decoded Kremlin capture and independently retrieved original-edition legal-portal page print point 1 and the 6 November signature. RSFSR jurisdiction only; 1992 qualifications are outside this 1991 claim.',
    'Both editions print point 3 concerning party property in RSFSR territory. The current edition also contains a 1992 court qualification after point 3; provenance corrected. Original wording does not assert present validity.',
]
claims = [(source, claim) for source in sources for claim in source['claims']]
assert len(claims) == len(notes) == 26
save('claim-review.json', [dict(claim_id=c['id'], source_id=s['id'], original_url=s['url'],
    locator=c['locator'], attested_on=c['attested_on'], decision='accept_bounded_attributed_claim',
    independent_content_finding=note, reviewed_original_body=str(RAW / (s['id'] + '.body')))
    for (s, c), note in zip(claims, notes)])

reviews = []
for i, s in enumerate(sources):
    e = json.loads((ROOT / s['snapshot']['path']).read_text('utf8'))
    body = (RAW / (s['id'] + '.body')).read_bytes()
    row = dict(source_id=s['id'], original_response_reproduced=True, byte_count=len(body),
        sha256=hashlib.sha256(body).hexdigest(), claims_reviewed=len(s['claims']),
        decision='accept_as_attributed_reproduced_record', printed_authorship=s['publisher'],
        hosting_classification=('private-account scan hosted by Internet Archive' if i < 7 else
            'nonofficial SSSR.SU scan, Internet Archive capture' if i < 11 else
            'Internet Archive capture of official Kremlin text; original edition corroborated on official legal portal'),
        independent_paper_original_authentication=False,
        reviewed_pdf_pages_one_based=sorted(set(e['visual_review']['pdf_pages_one_based'] + ([1] if 7 <= i <= 10 else []))),
        limits='No portrait rights or complete chronology accepted. Host status and byte identity do not independently authenticate the paper original.')
    if 'stored_file' in e:
        row['stored_file_sha1_agrees_with_submission'] = hashlib.sha1(body).hexdigest() == e['stored_file']['archive_org_file_sha1']
        row['archive_metadata_independently_retrieved'] = False
    reviews.append(row)
save('source-review.json', reviews)

holders = []
for role in packet['organizations'][0]['roles']:
    for h in role['holder_claims'][1:]:
        holders.append(dict(role_id=role['id'], name=h['name'], attested_on=h['attested_on'],
            **{'from': None, 'until': None}, claim_ids=h['claim_ids'], decision='accept_dated_attributed_observation',
            limit=('TASS-reported named party officer statement as printed by Pravda, not a transcript or appointment.'
                if h['attested_on'] == '1991-08-21' else 'Undated Secretariat statement dated by publication, not an effective day.'
                if h['attested_on'] == '1991-08-22' else 'Dated source attestation, not a tenure boundary.')))
assert len(holders) == 5
save('holder-review.json', holders)

renders = []
for f in sorted((RAW / 'renders').glob('*.png')):
    raw = f.read_bytes()
    renders.append(dict(path=str(f), bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest(), visually_inspected=True))
save('visual-review.json', dict(method='Poppler page-image rendering, visually read by Codex. Pypdf extraction used only for navigation; rulings checked against images. Both HTML editions decoded and read independently.',
    unique_pdf_pages_visually_reviewed=22, renders=renders,
    initial_tool_issue='First recipe attempted unavailable pdftotext before any render; used available pypdf for navigation and Poppler for rendering. Not a source access failure.'))
save('review-limits.json', dict(decision='accept_bounded_research_intake_after_precision_corrections',
    reviewed_exact_claude_tip='a989ddb4d67657236292a35443049554cc9b19e7', accepted_sources=12,
    accepted_claims=26, accepted_new_holder_observations=5, held_sources=0, held_claims=0,
    held_new_holder_observations=0, original_responses_bytes=sum(x['byte_count'] for x in reviews),
    separate_original_edition_bytes=30481,
    rulings=[
        'Private scan hosts remain private; acceptance concerns attributed content in reproduced party/official publications, consistent with C01-35. No paper-original authentication.',
        'The TASS report of a named party officer styles the deputy expressly; retain one dated attributed observation, not a signed party act.',
        'No election or later resignation reference supplies from/until. No exact resignation day inferred.',
        'The old Ivashkov observation remains unreconciled. Acting General Secretary service and union-level dissolution remain unestablished.'],
    outside_scope=[
        'Four submitted live SSSR.SU downloads and Claude paired-download timing were not repeated.',
        'Internet Archive item metadata/uploader identity was not independently retrieved. Publication mastheads were visually checked.',
        'No runtime, game mapping, portrait rights, exhaustive country coverage or C01/C06/S23/CP1 closure.'],
    web_tool_attempts=[
        dict(url='https://archive.org/details/199037_9972', result='Web open returned inaccessible; ordinary direct PDF curl succeeded exactly.'),
        dict(url='https://web.archive.org/web/20260830202901id_/http://kremlin.ru/acts/bank/385', result='Web open returned inaccessible; ordinary direct original curl succeeded exactly.')]))
raw = (ROOT / 'docs/campaign-certification/C01/research-index.json').read_bytes()
save('temporary-index.json', dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest(),
    validation='Temporarily generated for branch-local checks. Not an integration payload; regenerate after combining packets.'))
print('Recorded', len(claims), 'claim decisions;', sum(x['byte_count'] for x in reviews), 'original bytes;', len(renders), 'render images.')
