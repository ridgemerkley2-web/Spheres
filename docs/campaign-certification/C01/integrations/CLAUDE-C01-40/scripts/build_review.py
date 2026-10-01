"""Materialize hand-reviewed decisions and independent scope comparisons, offline."""
import copy
import hashlib
import json
import pathlib
import subprocess

RECEIPT = pathlib.Path(__file__).resolve().parents[1]
ROOT = pathlib.Path(__file__).resolve().parents[6]
BASE = 'f3e18e8306a0a7b1098b91f53f00efbb30da7990'
SUBMISSION = '54680e39910804b3864a3e973c452f2db2ac5e6f'
IMPORT = '5d5935c36aa108e9b5d4b9ba1977f8341d63ea43'
PACKET = 'docs/campaign-certification/C01/research/india.json'


def git_json(ref, path):
    return json.loads(subprocess.check_output(['git', 'show', ref + ':' + path], cwd=ROOT))


def write(name, value):
    (RECEIPT / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


packet = json.loads((ROOT / PACKET).read_text(encoding='utf-8'))
base = git_json(BASE, PACKET)
received = git_json(IMPORT, PACKET)
base_ids = {s['id'] for s in base['sources']}
sources = [s for s in packet['sources'] if s['id'] not in base_ids]
texts = {s['source_id']: s for s in json.loads((RECEIPT / 'source-texts.json').read_text(encoding='utf-8'))['sources']}
attempts = json.loads((RECEIPT / 'source-attempts.json').read_text(encoding='utf-8'))
exact = {r['source_id']: r for r in attempts['attempts'] if r['exact']}
assert len(sources) == len(texts) == len(exact) == 18

# Each entry is an independently read material passage and a bounded decision,
# not a conclusion computed from a matching checksum or the author's extract.
decisions = [
    ('in_cpim_site_ems_former_general_secretary_caption', [2, 6],
     'The caption names EMS and describes a former general secretary. Its death date is not an office date. No dated in-period observation or 1990 opening holder is established.'),
    ('in_cpim_biodata_surjeet_elected_gs_1992_14th_congress', [35, 42],
     'The biography recalls a 1992 election at the 14th Congress and says he continues. Only a year is supplied; neither the modification stamp nor capture date dates an assumption or an in-office observation.'),
    ('in_cpim_site_cc_elected_surjeet_gs_recalled_19981011', [4, 8],
     'The committee page retrospectively dates the CC meeting to 11 October 1998 and names Surjeet as general secretary. Accept the recalled election date, not a new term start.'),
    ('in_pd_surjeet_gs_letter_to_cec_20040302', [8, 16],
     'The March 7 issue introduces a March 2 letter and explicitly styles its author Surjeet as party general secretary. Contemporary 2004 context supplies the year. Accept the March 2 in-office observation, not assumption.'),
    ('in_pd_karat_unanimously_elected_gs_18th_congress', [8, 14],
     'The April 17, 2005 issue names Karat and unanimous election by the new CC. The misleading 04172004 filename does not override the printed 2005 issue date. No election day or effective start is stated.'),
    ('in_pd_rally_surjeet_former_gs_20050411', [14, 35],
     'The rally report styles Surjeet former general secretary and places the New Delhi meeting on April 11. Former status does not give the day his office ended.'),
    ('in_pd_rally_newly_elected_gs_karat_20050411', [19, 22, 46, 66],
     'The same April 11 meeting describes Karat escorting Surjeet and speaking as newly elected general secretary. Accept an in-office observation on that day; no effective assumption date is inferred.'),
    ('in_pd_surjeet_general_secretary_since_1992_recalled', [169, 179],
     'The first substantive paragraph under the personal-pledge heading recalls service as general secretary since 1992. A retrospective year is not an exact dated boundary.'),
    ('in_pd_surjeet_says_karat_took_up_mantle_2005', [185, 188],
     'The third substantive pledge paragraph describes Karat taking over while Surjeet remains in the Polit Bureau. The submitted first-paragraph locator was wrong and is repaired. No day of handover is supplied.'),
    ('in_pd_rally_newly_elected_gs_karat_took_salute_20120409', [13, 19],
     'The report gives April 9 at Kozhikode and identifies Karat taking the salute as newly elected general secretary. Named text anchors replace ambiguous paragraph counting in legacy HTML. Later observation only; no exact election or assumption day.'),
    ('in_pd_karat_gs_inaugurated_21st_congress_20150414', [29, 30],
     'The opening-session section gives April 14 and styles Karat general secretary while inaugurating the congress. This is a continuation observation, not a new term.'),
    ('in_pd_21st_congress_elected_general_secretary_20150419', [32, 32],
     'The last congress paragraph dates election of the listed bodies and general secretary to April 19 but names no officeholder in that sentence. Retain an unnamed election claim.'),
    ('in_pd_rally_karat_outgoing_gs_20150419', [19, 19, 32, 32],
     'The Karat paragraph calls him outgoing; the final congress paragraph dates that afternoon meeting to April 19. The former ninth-paragraph pointer was displaced; named anchor repaired. Outgoing does not establish an effective end.'),
    ('in_pd_rally_newly_elected_gs_yechury_20150419', [20, 20, 32, 32],
     'The Yechury paragraph identifies him as newly elected general secretary speaking at the same April 19 meeting. The former tenth-paragraph pointer was displaced; named anchor repaired. Accept the dated observation, not a start.'),
    ('in_pd_pb_list_yechury_general_secretary_21st_congress', [116, 118],
     'The recovered original lists Yechury first with the general-secretary title under the elected Polit Bureau heading. No election day is printed in that list; the weekly issue date is not substituted.'),
    ('in_cpim_cc_elected_yechury_gs_22nd_congress_20180422', [136, 137],
     'The concluding sentence says the CC elected Yechury as general secretary. April 22, 2018 is the current REST record dateline, with a later 2024 modification stamp. Accept an attributed election statement; no immutable contemporary capture or new term start.'),
    ('in_cpim_cc_reelected_yechury_gs_23rd_congress_20220410', [112, 120],
     'After the Polit Bureau list the post explicitly reports Yechury re-elected; a separate control-commission passage says it met today. April 10, 2022 comes from the current record dateline, not an assumption clause.'),
    ('in_cpim_pb_yechury_general_secretary_died_20240912', [2, 4],
     'The party statement explicitly names Yechury, the general-secretary office and death on September 12, 2024 in the same sentence. Accept the stated death-in-office end; its date is in the body, not merely post metadata.'),
    ('in_cpim_pb_yechury_elected_gs_21st_congress_recalled', [8, 8],
     'The tribute recalls election at the 21st Congress in 2015 and continued service. The submitted third-paragraph pointer led to student biography, so a named anchor is substituted. A retrospective year does not establish an exact start.'),
    ('in_cpim_cc_karat_coordinator_interim_arrangement_20240929', [1, 2],
     'The CC decision describes Karat coordinating the Polit Bureau and CC temporarily until the 24th Congress. It does not appoint or style him general secretary. Keep only the attributed coordinator decision, dated by current post metadata.'),
    ('in_cpim_karat_styled_coordinator_inaugural_speech_20250402', [1, 3],
     'Both title and speech heading identify Karat as coordinator. April 2, 2025 is the current post dateline. This supports interim-service styling only, never a general-secretary holder or exact interim end.'),
    ('in_cpim_cc_elected_baby_gs_24th_congress_20250406', [120, 122],
     'After the Polit Bureau list the body reports the CC elected Baby general secretary. April 6, 2025 is current record metadata. No independent assumption clause; keep the election claim separate from his later observation.'),
    ('in_cpim_baby_gs_letter_to_pm_20250512', [2, 2, 13, 14],
     'The release names Baby and the general-secretary office, dates the letter as today, and retains his signature with that title. Accept May 12, 2025 as the party record reported in-office observation; no assumption date, with current REST metadata limits.'),
    ('in_cpim_baby_gs_memo_census_20260824', [2, 2, 30, 31],
     'The release introduction and memo signature both identify Baby as general secretary. August 24, 2026 is the current post dateline. Retain a continuation claim within cutoff, not proof of every intervening or subsequent day.'),
]
notes = {cid: (lines, note) for cid, lines, note in decisions}
assert len(notes) == sum(len(s['claims']) for s in sources) == 24
claims = []
source_rows = []
old_claims = {c['id']: c for s in received['sources'] for c in s['claims']}
locator_repairs = []
for source in sources:
    text = texts[source['id']]
    raw = exact[source['id']]
    extract = json.loads((ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
    original_extract = git_json(IMPORT, source['snapshot']['path'])
    for key in ('source_response_bytes', 'source_response_sha256', 'source_url', 'accessed_date'):
        assert extract[key] == original_extract[key], (source['id'], key)
    rows = {r['claim_id']: r for r in extract['rows']}
    for claim in source['claims']:
        line_spans, rationale = notes[claim['id']]
        assert max(line_spans) <= text['lines']
        claims.append({
            'claim_id': claim['id'], 'source_id': source['id'],
            'decision': 'accepted_bounded_attributed_claim', 'event_kind': rows[claim['id']]['event_kind'],
            'attested_on': claim.get('attested_on'), 'locator': claim['locator'],
            'reading_aid_line_spans': line_spans, 'content_review': rationale,
            'source_response_sha256': raw['sha256'],
            'date_basis': 'current_party_record_dateline_or_explicit_body_date' if text['post_metadata'] else 'archived_body_or_issue_context; capture_is_not_event_date',
        })
        if old_claims[claim['id']]['locator'] != claim['locator']:
            locator_repairs.append({'claim_id': claim['id'], 'before': old_claims[claim['id']]['locator'], 'after': claim['locator'], 'rationale': rationale})
    source_rows.append({
        'source_id': source['id'], 'url': source['url'], 'decision': 'original_reproduced_and_material_content_read',
        'bytes': raw['bytes'], 'sha256': raw['sha256'], 'successful_attempt': raw['attempt'],
        'claim_ids': [c['id'] for c in source['claims']], 'post_metadata': text['post_metadata'],
        'limits': ('Current party REST record, not an independently timestamped historical capture. Metadata is reported by the publisher and can be revised. Exact reproduction establishes retrieval identity, not immutable past publication. No revision history or unrelated policy assertions authenticated.'
                   if text['post_metadata'] else 'Material text inspected in the pinned raw archived response. Capture and server modification dates are not office dates. No portrait rights or complete chronology inferred.'),
    })
write('claim-review.json', {'method': 'Independent reading of original response material passages; numbering refers to hashed external reading aids. These decisions were authored by the reviewer, not inferred from tests or matching author extracts.', 'claims': claims})
write('source-review.json', {'sources': source_rows, 'originals_exact': 18, 'claims_read': 24, 'held_originals': [], 'attempt_count': len(attempts['attempts'])})
write('locator-repairs.json', {'repairs': locator_repairs})

org_id = 'in_eci_20240323_np_04'
org = next(o for o in packet['organizations'] if o['id'] == org_id)
old_org = next(o for o in base['organizations'] if o['id'] == org_id)
role = next(r for r in org['roles'] if r['id'] == 'in_cpm_general_secretary')
received_role = next(r for o in received['organizations'] if o['id'] == org_id for r in o['roles'] if r['id'] == role['id'])
assert role['holder_claims'] == received_role['holder_claims']
write('holder-review.json', {
    'decision': 'Four dated source observations accepted, not a continuous succession timeline.',
    'holders': role['holder_claims'], 'unchanged_from_submission': True,
    'unknown_from_count': 4, 'explicit_end_count': 1,
    'ems': 'Not established as an in-period or opening holder; undated former-office caption only.',
    'karat_interim': 'Coordinator statements only, not a second general-secretary tenure.',
    'party_identity': 'Existing ECI recognition observation only; historical organizational continuity and game mapping not approved.',
    'runtime_or_art_permission': False,
})

trimmed = copy.deepcopy(packet)
trimmed['sources'] = [s for s in trimmed['sources'] if s['id'] in base_ids]
trim_org = next(o for o in trimmed['organizations'] if o['id'] == org_id)
trim_org['roles'] = [r for r in trim_org['roles'] if r['id'] != role['id']]
trim_org['sources'] = [sid for sid in trim_org['sources'] if sid in base_ids]
trim_org['claim_ids'] = [cid for cid in trim_org['claim_ids'] if cid not in notes]
trim_org['coverage']['unresolved'] = [u for u in trim_org['coverage']['unresolved'] if 'CLAUDE-C01-40' not in u]
trimmed['coverage']['unresolved'] = [u for u in trimmed['coverage']['unresolved'] if 'CLAUDE-C01-40' not in u]
assert trimmed == base, 'Unexpected change to pre-existing accepted India content'
assert git_json('c70c10eb^', PACKET) == base, 'Submitted packet starting India differs from accepted base'
paths = subprocess.check_output(['git', 'diff-tree', '--no-commit-id', '--name-only', '-r', IMPORT], cwd=ROOT, text=True).splitlines()
exact_import = []
for path in paths:
    before = subprocess.check_output(['git', 'show', SUBMISSION + ':' + path], cwd=ROOT)
    imported = subprocess.check_output(['git', 'show', IMPORT + ':' + path], cwd=ROOT)
    assert before == imported, path
    exact_import.append({'path': path, 'sha256': hashlib.sha256(imported).hexdigest(), 'bytes': len(imported)})
assert len(paths) == 28
assert all('research-index.json' not in path for path in paths)
write('scope-audit.json', {
    'base': BASE, 'submission': SUBMISSION, 'scoped_import_commit': IMPORT,
    'import_paths': exact_import, 'generated_research_index_imported': False,
    'author_pre_packet_india_equals_accepted_base': True,
    'all_preexisting_india_content_equal_after_removing_bounded_additions': True,
    'institutions_unchanged': packet['institutions'] == base['institutions'],
    'other_organizations_unchanged': [o for o in packet['organizations'] if o['id'] != org_id] == [o for o in base['organizations'] if o['id'] != org_id],
    'holders_unchanged_from_submission': True,
    'source_response_pins_unchanged': True,
    'new_sources': 18, 'new_claims': 24, 'new_role': role['id'], 'holder_observations': 4,
    'locator_repairs': len(locator_repairs), 'parent_gates_closed': [],
})
print('Reviewed 18 original sources, 24 claims, four holder observations; scope isolation verified.')
