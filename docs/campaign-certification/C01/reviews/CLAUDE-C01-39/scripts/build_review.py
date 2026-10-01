"""Record the single independently read claim and all explicit original-access holds."""
import json
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[6]
OUT = pathlib.Path(__file__).resolve().parents[1]
BASE = '509bd2890f71850c97215d304ff16b757c7d49c2'
PACKET = 'docs/campaign-certification/C01/research/south-africa.json'
old = json.loads(subprocess.check_output(['git', 'show', BASE + ':' + PACKET], cwd=ROOT))
new = json.loads((ROOT / PACKET).read_text(encoding='utf-8'))
old_ids = {s['id'] for s in old['sources']}
sources = [s for s in new['sources'] if s['id'] not in old_ids]
journal = json.loads((OUT / 'source-attempts.json').read_text(encoding='utf-8'))
attempts = {a['source_id']: a for a in journal['attempts']}
read_id = 'za_da_fedex_informed_maimane_resignation_decision_20191024'
read_source = 'za_da_fedcouncil_chair_leadership_vacancies_20191024'
source_rows, claim_rows = [], []
for source in sources:
    attempt = attempts.get(source['id'])
    state = 'exact_original_content_read' if attempt and attempt['exact'] else 'held_rate_limited' if attempt else 'held_not_attempted_after_rate_limit'
    source_rows.append({'source_id': source['id'], 'url': source['url'], 'state': state,
                        'http_status': attempt['http_status'] if attempt else None,
                        'independently_reproduced': bool(attempt and attempt['exact']),
                        'claim_ids': [c['id'] for c in source['claims']]})
    for claim in source['claims']:
        row = {'claim_id': claim['id'], 'source_id': source['id'],
               'submitted_attested_on': claim.get('attested_on'), 'submitted_locator': claim['locator'],
               'decision': 'held_no_independently_available_original', 'source_state': state,
               'review_limit': 'Submitted extract inspected for navigation and structure only. No independent original-content decision is made.'}
        if claim['id'] == read_id:
            assert source['id'] == read_source and attempt['exact']
            row.update(decision='supported_bounded_attributed_statement; packet_not_accepted',
                       reading_aid_line_spans=[21, 28],
                       original_sha256=attempt['sha256'],
                       content_review='The preserved party page is issued by Helen Zille in her Federal Council role on 24 October 2019. Its opening paragraph names Maimane as Federal Leader and says the executive learned on Wednesday of his personal resignation decision. Later paragraphs describe a Thursday meeting and both leadership posts already vacant. This supports the reported statement; the claim date is the issue date, not a substituted effective resignation date.',
                       review_limit='The raw capture is from November 2021 and includes later site furniture. Only the dated article body is used. A calendar resolution of the relative weekday is interpretation, not a separately printed numeric effective date. The next-day vacancy source was rate-limited; no exact end is installed.')
        claim_rows.append(row)
assert len(source_rows) == 24 and len(claim_rows) == 29
assert sum(s['independently_reproduced'] for s in source_rows) == 1
assert sum(c['claim_id'] == read_id for c in claim_rows) == 1
org = next(o for o in new['organizations'] if o['id'] == 'za_iec_n2024_027')
role = next(r for r in org['roles'] if r['id'] == 'za_da_federal_leader')
old_role = next(r for o in old['organizations'] if o['id'] == org['id'] for r in o['roles'] if r['id'] == role['id'])
holder_rows = []
for holder in role['holder_claims']:
    if holder in old_role['holder_claims']:
        continue
    assert all(cid != read_id for cid in holder['claim_ids'])
    holder_rows.append({'name': holder['name'], 'attested_on': holder['attested_on'],
                        'from': holder['from'], 'until': holder['until'],
                        'sources': holder['sources'], 'claim_ids': holder['claim_ids'],
                        'decision': 'held_all_cited_originals_unattempted_after_rate_limit'})
assert len(holder_rows) == 11
for name, data in [
    ('source-review.json', {'sources': source_rows, 'exact': 1, 'rate_limited': 1, 'not_attempted': 22,
                            'unverified_total': 23, 'note': 'Twenty-three unverified originals are not twenty-three failed requests.'}),
    ('claim-review.json', {'claims': claim_rows, 'independently_read': 1, 'held': 28,
                           'accessible_claims_deferred': 0, 'packet_accepted': False}),
    ('holder-review.json', {'new_holder_observations': holder_rows, 'new_supported': 0, 'new_held': 11,
                            'existing_s10h_objects_unchanged': old_role['holder_claims'],
                            'note': 'The two prior observations are preserved, not newly verified by this review.'}),
]:
    (OUT / name).write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8', newline='\n')
print('Held: one original and one claim reviewed; one429;22not attempted;28claims/11newobservations remain held.')
