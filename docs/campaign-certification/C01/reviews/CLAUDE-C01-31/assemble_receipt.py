"""Assemble the explicitly reviewed C01-31 held receipt; never infer acceptance from hashes."""
import copy
import hashlib
import json
import pathlib
import re
import subprocess

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[4]
EXTERNAL = pathlib.Path(r'D:\spheres-offload\codex-next-20260928\c01-31-original-responses-20261001')
BASE = 'f3e18e8306a0a7b1098b91f53f00efbb30da7990'
TIP = '696937ba3d31280aab569cbd78418e9c3a0c6880'
IMPORT = 'b74f4fa5'
REPAIR = '988d537934b4d36e2a251c4b5ca498f19dc7cf06'
PATH = 'docs/campaign-certification/C01/research/japan.json'
sha = lambda data: hashlib.sha256(data).hexdigest()
def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT)
def at(rev, path):
    return git('show', rev + ':' + path)
def write(name, value):
    (HERE / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf8', newline='\n')

base = json.loads(at(BASE, PATH))
original = json.loads(at(IMPORT, PATH))
packet = json.loads(at(REPAIR, PATH))
assert original == json.loads(at(TIP, PATH)), 'Imported Japan differs from exact delivery'
assert packet['sources'][:472] == base['sources']
new = packet['sources'][472:]
assert len(new) == 31 and sum(len(s['claims']) for s in new) == 64
org = next(o for o in packet['organizations'] if o['id'] == 'jp_sangiin_pr_2025_15')
old_org = next(o for o in base['organizations'] if o['id'] == org['id'])
restored = copy.deepcopy(packet)
restored['sources'] = restored['sources'][:472]
target = next(o for o in restored['organizations'] if o['id'] == org['id'])
assert target['sources'][:len(old_org['sources'])] == old_org['sources']
assert target['claim_ids'][:len(old_org['claim_ids'])] == old_org['claim_ids']
assert target['coverage']['unresolved'][:-1] == old_org['coverage']['unresolved']
assert old_org['roles'] == [] and len(target['roles']) == 1
target['sources'] = target['sources'][:len(old_org['sources'])]
target['claim_ids'] = target['claim_ids'][:len(old_org['claim_ids'])]
target['coverage']['unresolved'] = target['coverage']['unresolved'][:-1]
target['roles'] = []
restored['coverage']['unresolved'] = restored['coverage']['unresolved'][:-1]
assert restored == base, 'Prior Japan research was modified'
source_ids = {s['id'] for s in new}
assert all(s.startswith('jp_komeito_') for s in source_ids)
submitted = {s['id']: s for s in original['sources'][-31:]}
for source in new:
    old = submitted[source['id']]
    assert source['url'] == old['url']
    for old_c, new_c in zip(old['claims'], source['claims'], strict=True):
        assert {k:v for k,v in old_c.items() if k != 'uncertainty'} == {k:v for k,v in new_c.items() if k != 'uncertainty'}

notes = {
'jp_komeito_diet_hr_budget_19930129': 'Read meeting metadata and speeches25,43,71. The replies identify Ichikawa as secretary-general; his housing question names the party chair Ishida. This establishes the29January1993 observation, not continuous service since1990.',
'jp_komeito_diet_hr_political_reform_19931105': 'Read speech117, including the May1989 recollection and its approximate newspaper days. Month-only recollection is retained without a structured date or holder boundary; ministerial title is not used as party-office proof.',
'jp_komeito_diet_hc_budget_19940616': 'Read speech3. Ishida explicitly says he remains chair while an unnamed acting chair operates the party. Both continuation and acting-arrangement claims match; neither invents an outgoing end or acting-holder identity.',
'jp_komeito_komei_representative_message_retrospective': 'Read the complete English representative message and personal-history list. Formation and Fujii representative history belong to Komei, not the later Komeito observation; retrospective dates remain unstructured.',
'jp_komeito_history_2003_retrospective': 'Read the named old-party, division, New Frontier Party, Reimei/Heiwa and reformation sections. The six paraphrases match the retrospective page; no organizational identity, date boundary or cabinet office is inferred.',
'jp_komeito_founding_convention_report_19981108': 'Read the complete1998 report. October24 naming decision, surviving-party merger form, confidence procedure and Kanzaki address are separate facts. The page gives November8 as its publication line, not a convention day; the holder remains a publication-date observation.',
'jp_komeito_diet_hr_plenary_19981130': 'Read speech15 opening. Kanzaki dates the organizational merger convention to November7 but introduces a parliamentary-group representative question; the packet correctly does not turn that into party-holder proof.',
'jp_komeito_kanzaki_reappointed_20021103': 'Read date line and officer-selection paragraphs1,4,5. Sole-candidate confidence and subsequent naming of officers match. The report supplies November3 publication, without an explicit convention day or assumption boundary.',
'jp_komeito_kanzaki_fourth_convention_address_20021103': 'Read the November3 date line, representative heading and second address paragraph. The speaker explicitly continues in the duty after confidence; this supports an observation, not a new effective start.',
'jp_komeito_ota_address_sixth_convention_20061001': 'Read October1 publication, introduction dating the convention to the30th, and first two address paragraphs. Ota explicitly says he took office at today\'s convention; September30 assumption is supported. Kanzaki four-term/eight-year predecessor reference does not provide an end.',
'jp_komeito_diet_hr_plenary_20061003': 'Read speech3 opening and metadata. Ota introduces himself as recently having taken the party representative office; October3 is a continuation, not a new start.',
'jp_komeito_diet_hc_plenary_20091030': 'Read speech2 opening, September8 selection recollection and since-taking-office passage. Selection date and October30 current-office attestation remain separate; neither substitutes for unavailable contemporaneous address content.',
'jp_komeito_yamaguchi_tenth_convention_20140922': 'Read September22 publication,21st convention paragraph and confidence/officer-nomination paragraph. Re-election by all standing delegates and the subsequent representative action match; no effective term boundary is stated.',
'jp_komeito_yamaguchi_eleventh_convention_20160918': 'Read September18 publication,17th convention/caption and confidence procedure. Re-election and named in-office address are separately supported; no effective term boundary is stated.',
'jp_komeito_yamaguchi_twelfth_convention_20181001': 'Read October1 publication,30th convention/caption and sole-candidate confidence procedure. September30 follows the explicitly reported prior-day convention; two-year term language supplies no effective boundary.',
'jp_komeito_yamaguchi_thirteenth_convention_20200928': 'Read September28 publication,27th convention and sole-candidate confidence/officer nomination. Both claims match, with election and subsequent office naming kept distinct.',
'jp_komeito_yamaguchi_fourteenth_convention_20220926': 'Read September26 publication,25th convention and confidence/officer nomination. Both claims match; no successor, outgoing end or precise term start is derived.',
'jp_komeito_saito_extraordinary_convention_20241110': 'Read November10 publication,9th extraordinary convention, rules-based selection, subsequent Saito address and former-Ishii reference. The three claims match; former status does not date Ishii\'s end.',
'jp_komeito_diet_hr_plenary_20241203': 'Read speech10 metadata and nuclear-disarmament passage expressly identifying the speaker as party representative. This is December3 continuation evidence, independent of any ministerial title.',
'jp_komeito_takeya_interim_20260123': 'Read January23 publication and first two paragraphs. January22 approval is explicitly representative-proxy service styled representative pending a convention; predecessor thanks do not supply Saito\'s effective last day.',
'jp_komeito_chudo_founding_convention_20260125': 'Read January25 publication,22nd founding convention and same-day Takeya reference. Chudo co-representatives are other-organization offices; Takeya\'s styled title is constrained by the verified interim clarification.',
'jp_komeito_takeya_recommended_20260312': 'Read March12 publication and March11 committee decision. Recommendation and conditional future convention confidence remain a proposal/event, never an effective start.',
'jp_komeito_takeya_address_20260315': 'Read March15 publication,14th caption and first address paragraph. Confidence and acceptance of representative duty support the office observation; they do not separately state an effective assumption day. Preserve claim wording/date/ID, correct holder to attested_on.',
'jp_komeito_takeya_extraordinary_convention_20260315': 'Read March15 publication,14th convention, selection and subsequent representative address, and January vacancy recollection. Selection/attestation are distinct; the vacancy is month-only and cannot date Saito\'s end.',
}
notes = {sid: re.sub(r'(?<=[A-Za-z])(?=\d)|(?<=\d)(?=[A-Z])', ' ', note) for sid, note in notes.items()}

attempts = {}
for receipt in sorted(HERE.glob('retrieval-*/retrieval.json')):
    for row in json.loads(receipt.read_text(encoding='utf8'))['results']:
        attempts.setdefault(row['id'], []).append(dict(row, receipt=receipt.relative_to(HERE).as_posix()))
verified = {sid: next((r for r in rows if r['exact']), None) for sid, rows in attempts.items()}
assert set(notes) == {sid for sid, row in verified.items() if row}
assert len(notes) == 24
decoded = {r['id']: r for r in json.loads((EXTERNAL / 'readable-manifest.json').read_text(encoding='utf8'))}
source_reviews, claim_reviews = [], []
for source in new:
    sid = source['id']
    exact = verified.get(sid)
    extract = json.loads((ROOT / source['snapshot']['path']).read_text(encoding='utf8'))
    original_extract = json.loads(at(IMPORT, source['snapshot']['path']))
    for key in ['source_url', 'source_response_bytes', 'source_response_sha256', 'source_response_content_encoding']:
        assert extract[key] == original_extract[key]
    row_map = {r['claim_id']: r for r in extract['rows']}
    record = {'source_id': sid, 'url': source['url'], 'original_url': extract.get('original_url', source['url']), 'delivery_host': 'web.archive.org' if 'web.archive.org' in source['url'] else 'kokkai.ndl.go.jp', 'authored_provenance': 'Party publication replayed by third-party archive; the archive is the delivery host, not the party author.' if 'web.archive.org' in source['url'] else 'Official parliamentary minutes API served by the National Diet Library.', 'archive_capture_utc': extract.get('archive_capture_utc'), 'expected_bytes': extract['source_response_bytes'], 'expected_sha256': extract['source_response_sha256'], 'retrieval_decision': 'exact_original_reproduced' if exact else 'held_original_unavailable', 'attempt_receipts': [r['receipt'] for r in attempts[sid]], 'claim_ids': [c['id'] for c in source['claims']], 'content_review': notes.get(sid, 'Not independently read: original response remains unavailable; no historical contradiction or acceptance is inferred from access failure.')}
    if exact:
        data = pathlib.Path(exact['body_external']).read_bytes()
        assert len(data) == record['expected_bytes'] and sha(data) == record['expected_sha256']
        record.update(body_external=exact['body_external'], reproduced_bytes=len(data), reproduced_sha256=sha(data), readable_external=decoded[sid]['readable_external'], readable_sha256=decoded[sid]['readable_sha256'], decoding=decoded[sid]['encoding'])
    source_reviews.append(record)
    for claim in source['claims']:
        cid = claim['id']
        corrected = cid == 'jp_komeito_takeya_assumption_stated_20260314'
        claim_reviews.append({'claim_id': cid, 'source_id': sid, 'decision': ('supported_as_attestation_after_boundary_correction' if corrected else 'supported_within_stated_scope') if exact else 'held_original_unavailable', 'event_kind': row_map[cid]['event_kind'], 'structured_attested_on': claim.get('attested_on'), 'locator': claim['locator'], 'claim_text_sha256': sha(claim['text'].encode()), 'reviewer_observation': record['content_review'], 'effective_boundary': 'Explicit Ota assumption statement supports 2006-09-30.' if cid == 'jp_komeito_ota_assumption_stated_20060930' else 'No new effective start/end derived from this claim.'})
held_sources = [s['source_id'] for s in source_reviews if s['retrieval_decision'].startswith('held')]
held_claims = [c['claim_id'] for c in claim_reviews if c['decision'].startswith('held')]
role = org['roles'][0]
submitted_role = next(o for o in original['organizations'] if o['id'] == org['id'])['roles'][0]
holders = []
for before, after in zip(submitted_role['holder_claims'], role['holder_claims'], strict=True):
    dependencies = sorted(set(after['sources']) & set(held_sources))
    holders.append({'name': after['name'], 'date': after.get('attested_on') or after['from'], 'decision': 'held_additional_cited_original_unavailable' if dependencies else ('supported_attestation_after_boundary_correction' if before != after else 'supported_bounded_observation'), 'missing_source_ids': dependencies, 'claim_ids': after['claim_ids'], 'submitted': before, 'reviewed': after, 'note': 'The verified Ota address independently states assumption; this row still has an unavailable convention-report citation.' if after['name'] == '太田昭宏' else 'No continuous tenure or outgoing end inferred.'})
assert len(held_sources) == 7 and len(held_claims) == 14
assert sum(bool(h['missing_source_ids']) for h in holders) == 5
write('source-verification.json', source_reviews)
write('claim-review.json', claim_reviews)
write('holder-review.json', holders)
write('held-dependencies.json', {'decision': 'held_not_accepted', 'sources': held_sources, 'claims': held_claims, 'holder_dependencies': [{k:h[k] for k in ['name', 'date', 'missing_source_ids', 'claim_ids']} for h in holders if h['missing_source_ids']], 'totals': {'originals_reproduced':24, 'originals_unavailable':7, 'claims_supported_in_bounded_scope':50, 'claims_held':14, 'holder_observations_fully_reviewed':10, 'holder_observations_with_missing_dependency':5}})
paths = git('diff', '--name-only', BASE, IMPORT).decode().splitlines()
assert len(paths) == 40 and 'docs/campaign-certification/C01/research-index.json' not in paths
pins = []
for name in paths:
    authored = at(IMPORT, name)
    assert authored == at(TIP, name), ('Authored file changed during import', name)
    reviewed = at(REPAIR, name)
    pins.append({'path':name, 'authored_git_blob': git('rev-parse', IMPORT + ':' + name).decode().strip(), 'authored_bytes':len(authored), 'authored_sha256':sha(authored), 'reviewed_git_blob':git('rev-parse', REPAIR + ':' + name).decode().strip(), 'reviewed_bytes':len(reviewed), 'reviewed_sha256':sha(reviewed)})
write('scope-audit.json', {'submission':TIP, 'merge_base':'02d2c5a25c9a70bc9ba9503c8dc45f3ccdc1f0cf', 'integration_base':BASE, 'authored_import':git('rev-parse', IMPORT).decode().strip(), 'correction':REPAIR, 'authored_paths':pins, 'prior_japan_values_restored_exactly':True, 'existing_sources_unchanged':472, 'existing_claims_unchanged':799, 'added_sources':31, 'added_claims':64, 'added_roles':1, 'added_holder_observations':15, 'distinct_people':7, 'all_original_response_identities_unchanged':True, 'all_claim_ids_texts_dates_locators_unchanged':True, 'runtime_art_other_country_paths_changed':False, 'generated_index_excluded':True, 'correction_paths':git('diff', '--name-only', IMPORT, REPAIR).decode().splitlines()})

# Keep original headers externally and publish only cookie-redacted derivatives.
header_records = []
for header in sorted(HERE.glob('retrieval-*/*.headers.txt')):
    raw = header.read_bytes()
    external = EXTERNAL / header.parent.name / header.name
    if external.exists():
        raw = external.read_bytes()
    else:
        external.write_bytes(raw)
    sanitized, count = re.subn(rb'(?im)^(set-cookie|authorization|proxy-authorization):[^\r\n]*', rb'\1: [response value redacted]', raw)
    header.write_bytes(sanitized)
    header_records.append({'checked_in':header.relative_to(HERE).as_posix(), 'checked_in_sha256':sha(sanitized), 'original_external':str(external), 'original_bytes':len(raw), 'original_sha256':sha(raw), 'redacted_header_values':count})
write('header-provenance.json', header_records)
print('Receipt assembled:24/31 originals,50supported/14held claims,10complete/5dependent holder rows. Packet remains held.')
