#!/usr/bin/env python3
"""Import reviewed party offices into empty catalogue rows, preserving incumbents.

This is deliberately separate from identity ingestion. It cannot replace an
existing chronology or grant a party leader eligibility for national office.
Review sources and component save migrations before using --apply.
"""
from __future__ import annotations
import argparse
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import sys
import unittest
from import_historical_people import ROOT, REGISTRY, MANIFEST
import person_art_pipeline as art
from research_import_io import import_paths, life, window, read_snapshot, write_transaction

PARTY_FIELDS = {'nation','party','kind','coverage','sources','identity_note','components','terms','gaps','founded','dissolved'}
TERM_FIELDS = {'id','person','component','role','kind','from','until','affiliation_from','affiliation_until','sources','note'}
COMPONENT_FIELDS = {'id','name','sources','founded','dissolved'}
GAP_FIELDS = {'from','until','reason','sources'}
# These are the exact inventories supported by the narrow native save upgrade.
REVIEWED_COMPONENTS = {
    ('Japan','jp_jsp'): ['jp_jsp_1945','jp_sdp_1996'],
    ('Japan','jp_komeito'): ['jp_komeito_1964','jp_komei_1994','jp_komeito_1998'],
    ('France','fr_udf'): ['fr_udf_federation','fr_udf_pr','fr_udf_dl','fr_udf_cds','fr_udf_fd','fr_udf_radical','fr_udf_psd','fr_udf_perspectives','fr_udf_ppdf','fr_udf_direct','fr_udf_pril'],
    ('Germany','de_union'): ['de_union_cdu','de_union_csu'],
}


def fields(record, allowed, required, label):
    if not isinstance(record,dict) or set(record)-allowed or required-set(record):
        raise ValueError(f'Invalid {label} fields')


def sources(value, required=True):
    if not isinstance(value,list) or (required and not value) or any(not art.web_url(s) for s in value):
        raise ValueError('Sourced research URLs required')


def canonical(row):
    result=deepcopy(row)
    for key in ['identity_note','founded','dissolved']: result.setdefault(key,None)
    for c in result['components']:
        for key in ['founded','dissolved']: c.setdefault(key,None)
    for t in result['terms']:
        for key in ['component','note']: t.setdefault(key,None)
    for g in result['gaps']: g.setdefault('sources',[])
    return result


def validate_row(row, known_people):
    fields(row,PARTY_FIELDS,{'nation','party','kind','coverage','sources','components','terms','gaps'},'Party')
    if row['kind'] not in {'party','coalition','collective','unknown'} or row['coverage']!='partial':
        raise ValueError('This staged importer accepts explicitly partial party research only')
    if not all(isinstance(row[k],list) for k in ['components','terms','gaps']):
        raise ValueError('Components, terms and gaps must be arrays')
    sources(row['sources']); life(row.get('founded'),row.get('dissolved'))
    if row.get('identity_note') is not None and not isinstance(row['identity_note'],str): raise ValueError('Invalid identity note')
    key=row['nation'],row['party']
    components=set()
    for c in row['components']:
        fields(c,COMPONENT_FIELDS,{'id','name','sources'},'Component')
        if not isinstance(c['id'],str) or not art.ID_RE.fullmatch(c['id']) or not art.nonempty(c['name']) or c['id'] in components:
            raise ValueError('Invalid or duplicate component identity')
        components.add(c['id']); sources(c['sources']); life(c.get('founded'),c.get('dissolved'))
    if row['components'] and [c['id'] for c in row['components']] != REVIEWED_COMPONENTS.get(key):
        raise ValueError('Component import requires its exact reviewed native save-migration schema')
    ids=set()
    for t in row['terms']:
        fields(t,TERM_FIELDS,{'id','person','role','kind','from','until','affiliation_from','affiliation_until','sources'},'Term')
        if not isinstance(t['id'],str) or not art.ID_RE.fullmatch(t['id']) or t['id'] in ids:
            raise ValueError('Invalid or duplicate term ID')
        ids.add(t['id'])
        if t['person'] not in known_people: raise ValueError('Import sourced identities first')
        if not art.nonempty(t['role']) or t['kind'] not in {'leader','co_leader','acting','candidate'}:
            raise ValueError('Invalid researched party role/kind')
        if (components and t.get('component') not in components) or (not components and t.get('component') is not None):
            raise ValueError('Missing or unknown term component')
        if t.get('note') is not None and not isinstance(t['note'],str): raise ValueError('Invalid term note')
        sources(t['sources']); window(t['from'],t['until']); window(t['affiliation_from'],t['affiliation_until'])
    for g in row['gaps']:
        fields(g,GAP_FIELDS,{'from','until','reason'},'Gap')
        if not art.nonempty(g['reason']): raise ValueError('Coverage gap needs an explanation')
        sources(g.get('sources',[]),False); window(g['from'],g['until'])

def merge(registry, incoming, policy):
    if policy.get('version') != 1 or policy.get('historical_default')!='party_only' or policy.get('reviewed_at')!='2026-09-07':
        raise ValueError('Reviewed party-only executive eligibility policy version 1 is required')
    if incoming.get('reference_through')!='2026-09-07': raise ValueError('Research must retain the exact historical cutoff')
    result = deepcopy(registry)
    by_key = {(p['nation'], p['party']): p for p in result['parties']}
    if len(by_key)!=len(result['parties']): raise ValueError('Duplicate existing party identity')
    known_people = {p['id'] for p in result['people']}
    art.people_from_registry(registry)
    term_ids = {t['id'] for p in result['parties'] for t in p['terms']}
    if len(term_ids)!=sum(len(p['terms']) for p in result['parties']): raise ValueError('Duplicate existing term ID')
    additions = []
    seen = set()
    if not isinstance(incoming.get('parties'),list) or not incoming['parties']: raise ValueError('No reviewed parties supplied')
    for row in incoming['parties']:
        validate_row(row,known_people)
        key = row['nation'], row['party']
        if key in seen: raise ValueError('Duplicate incoming party')
        seen.add(key)
        original = by_key.get(key)
        if not original: raise ValueError(f'Unknown simulation party: {key}')
        if canonical(original) == canonical(row): continue
        if original['terms'] or original['components'] or original['coverage'] != 'gap':
            raise ValueError(f'Existing chronology requires an explicit migration: {key}')
        components = row.get('components', [])
        for term in row.get('terms', []):
            if term['id'] in term_ids: raise ValueError('Existing/duplicate term ID')
            term_ids.add(term['id'])
        original.clear()
        original.update(deepcopy(row))
        additions.append({'nation': key[0], 'party': key[1], 'terms': len(row['terms']),
                          'components': [c['id'] for c in components]})
    return result, {'imported_parties': additions, 'people_changed': 0,
                    'office_links_changed': 0, 'executive_grants_added': 0,
                    'save_files_changed': 0}

def self_test():
    class Checks(unittest.TestCase):
        def setUp(self):
            self.empty = dict(nation='Example', party='ex_party', kind='unknown', coverage='gap',
                              sources=[], identity_note='', components=[], terms=[], gaps=[])
            self.registry = dict(people=[{'id':'person','name':'Test person','sources':['https://example.org/person']}], parties=[self.empty], office_links=[{'person':'incumbent'}])
            self.row = dict(self.empty, kind='party', coverage='partial', sources=['https://example.org/party'],
                            terms=[dict(id='term', person='person', role='Party chair',kind='leader',
                                **{'from':{'kind':'day','value':'1990-01-01'},'until':{'kind':'open'}},
                                affiliation_from={'kind':'day','value':'1990-01-01'},affiliation_until={'kind':'open'},
                                sources=['https://example.org/term'])])
            self.policy={'version':1,'historical_default':'party_only','reviewed_at':'2026-09-07'}
        def incoming(self,rows): return {'parties':rows,'reference_through':'2026-09-07'}
        def test_offices_and_people_preserved(self):
            result,audit = merge(self.registry, self.incoming([self.row]), self.policy)
            self.assertEqual(result['people'],self.registry['people'])
            self.assertEqual(result['office_links'],self.registry['office_links'])
            self.assertEqual(audit['executive_grants_added'],0)
            self.assertEqual(self.registry['parties'][0]['terms'],[])
        def test_existing_chronology_and_duplicate_rejected(self):
            result,_ = merge(self.registry, self.incoming([self.row]), self.policy)
            changed = dict(self.row, identity_note='changed')
            with self.assertRaises(ValueError): merge(result, self.incoming([changed]), self.policy)
            with self.assertRaises(ValueError): merge(self.registry, self.incoming([self.row,self.row]), self.policy)
        def test_identical_import_is_idempotent(self):
            result,_ = merge(self.registry, self.incoming([self.row]), self.policy)
            normalized=canonical(self.row)
            same,audit = merge(result, self.incoming([normalized]), self.policy)
            self.assertEqual(same,result)
            self.assertEqual(audit['imported_parties'],[])
        def test_missing_policy_and_unknown_person_fail(self):
            with self.assertRaises(ValueError): merge(self.registry, self.incoming([self.row]), {})
            missing=deepcopy(self.registry);missing['people']=[]
            with self.assertRaises(ValueError): merge(missing, self.incoming([self.row]), self.policy)
        def test_bad_dates_unknown_fields_and_unreviewed_components_fail_without_mutation(self):
            rows=[]
            bad=deepcopy(self.row);bad['terms'][0]['until']={'kind':'day','value':'1989-01-01'};rows.append(bad)
            bad=deepcopy(self.row);bad['terms'][0]['from']={'kind':'day','value':'1990-02-30'};rows.append(bad)
            bad=deepcopy(self.row);bad['terms'][0]['executive']=True;rows.append(bad)
            bad=deepcopy(self.row);bad['components']=[{'id':'invented','name':'Invented','sources':['https://example.org/x']}];rows.append(bad)
            before=deepcopy(self.registry)
            for row in rows:
                with self.assertRaises(ValueError):merge(self.registry,self.incoming([row]),self.policy)
                self.assertEqual(self.registry,before)
        def test_reviewed_components_require_every_term_binding_and_exact_order(self):
            base=deepcopy(self.registry);base['parties'][0].update(nation='Germany',party='de_union')
            row=deepcopy(self.row);row.update(nation='Germany',party='de_union',kind='coalition',components=[
                {'id':c,'name':c,'sources':['https://example.org/component']} for c in REVIEWED_COMPONENTS[('Germany','de_union')]])
            with self.assertRaises(ValueError):merge(base,self.incoming([row]),self.policy)
            row['terms'][0]['component']='de_union_cdu'
            merge(base,self.incoming([row]),self.policy)
            row['components'].reverse()
            with self.assertRaises(ValueError):merge(base,self.incoming([row]),self.policy)
    run=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Checks))
    return int(not run.wasSuccessful())

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', nargs='?', type=Path)
    parser.add_argument('--apply', action='store_true')
    parser.add_argument('--receipt', type=Path)
    parser.add_argument('--self-test', action='store_true')
    args=parser.parse_args()
    if args.self_test: return self_test()
    if not args.source: parser.error('Reviewed staging file required')
    source=args.source.resolve()
    receipt=args.receipt.resolve() if args.receipt else None
    policy_path=ROOT/'spheres-sim/data/party_executive_eligibility.json'
    import_paths(source,receipt,ROOT,{REGISTRY,MANIFEST,policy_path})
    before,registry_raw=read_snapshot(REGISTRY)
    incoming,source_raw=read_snapshot(source)
    policy,policy_raw=read_snapshot(policy_path)
    result,audit=merge(before,incoming,policy)
    audit.update(source_file=source.relative_to(ROOT).as_posix(),
                 source_sha256=hashlib.sha256(source_raw).hexdigest(),
                 status='party_terms_imported' if args.apply else 'review_preview',
                 validation='Run native registry, succession and saved-campaign checks after import.')
    outputs=[(REGISTRY,result)] if args.apply else []
    if receipt: outputs.append((receipt,audit))
    write_transaction(outputs,{REGISTRY:registry_raw,source:source_raw,policy_path:policy_raw})
    print(json.dumps(audit,ensure_ascii=True,indent=2))
    return 0

if __name__=='__main__':
    try: raise SystemExit(main())
    except (ValueError,OSError,KeyError,RuntimeError) as error:
        print(f'Party-term import failed: {error}',file=sys.stderr)
        raise SystemExit(1)
