#!/usr/bin/env python3
"""Merge reviewed historical identities without changing saved identity facts.

This imports people and their source URLs only. Party offices, affiliations,
executive eligibility, saves and artwork are never inferred or overwritten.
Review chronology separately before any party-term import.
"""
from __future__ import annotations
import argparse
from copy import deepcopy
from datetime import date
import hashlib
import json
from pathlib import Path
import sys
import unittest
import person_art_pipeline as art
from research_import_io import import_paths, life, read_snapshot, write_transaction

ROOT=Path(__file__).resolve().parents[2]
REGISTRY=ROOT/"spheres-sim/data/party_leaders.json"
MANIFEST=ROOT/"spheres-web/data/person_portraits.json"
PERSON_FIELDS={"id","name","native","born","died","sources"}
IDENTITY_FIELDS=("name","native","born","died")

def merge(registry:dict, manifest:dict, incoming:dict):
    result, portraits=deepcopy(registry),deepcopy(manifest)
    art.people_from_registry(registry)
    people={p["id"]:p for p in result["people"]}
    additions, preserved, enriched=[],[],[]
    proposed=incoming.get("people")
    if not isinstance(proposed,list) or not proposed:
        raise ValueError("A nonempty researched people array is required")
    seen=set()
    for candidate in proposed:
        if not isinstance(candidate,dict) or set(candidate)-PERSON_FIELDS:
            raise ValueError("Only Person-compatible fields can be imported")
        known=art.people_from_registry({"people":[candidate]})
        pid=next(iter(known))
        if pid in seen: raise ValueError(f"Duplicate incoming identity: {pid}")
        seen.add(pid)
        sources=candidate.get("sources")
        if not isinstance(sources,list) or not sources or any(not art.web_url(s) for s in sources):
            raise ValueError(f"{pid}: sourced identity URLs required")
        if candidate.get("native") is not None and not art.nonempty(candidate["native"]):
            raise ValueError(f"{pid}: native name must be nonempty or absent")
        for field in ("born","died"):
            value=candidate.get(field)
            if value is None: continue
            if not isinstance(value,dict) or set(value)-{"kind","value"}:
                raise ValueError(f"{pid}: invalid {field} date bound")
            kind,text=value.get("kind"),value.get("value")
            if kind=="unknown" and text is None: continue
            if kind=="day": art.iso_day(text)
            elif kind=="month" and isinstance(text,str) and len(text)==7:
                art.iso_day(text+"-01")
            elif kind=="year" and isinstance(text,str) and len(text)==4 and text.isascii() and text.isdigit():
                date(int(text),1,1)
            else: raise ValueError(f"{pid}: invalid {field} date bound")
        life(candidate.get('born'),candidate.get('died'))
        if pid in people:
            existing=people[pid]
            differences=[key for key in IDENTITY_FIELDS if key in candidate and candidate[key]!=existing.get(key)]
            if differences:
                preserved.append({"person_id":pid,"fields":differences,
                    "action":"Preserved existing saved-identity facts; proposed corrections require separate review/migration."})
            added=[s for s in sources if s not in existing["sources"]]
            existing["sources"].extend(added)
            if added: enriched.append(pid)
        else:
            existing=deepcopy(candidate)
            existing["sources"]=list(dict.fromkeys(sources))
            result["people"].append(existing); people[pid]=existing; additions.append(pid)
        entry=portraits["people"].setdefault(pid,{"name":existing["name"],"identity_sources":[],"portraits":[]})
        if entry.get("name")!=existing["name"]:
            raise ValueError(f"{pid}: existing portrait name does not match its exact identity")
        entry["identity_sources"]=list(dict.fromkeys(entry.get("identity_sources",[])+existing["sources"]))
    art.people_from_registry(result)
    return result,portraits,{"added_person_ids":additions,"source_enriched_ids":enriched,
        "existing_fact_conflicts_preserved":preserved,"party_terms_changed":0,
        "office_links_changed":0,"portraits_generated":0}

def write_json(path:Path,value):
    write_transaction([(path,value)])

def self_test():
    class Checks(unittest.TestCase):
        def setUp(self):
            self.base={"people":[{"id":"existing","name":"Original","born":{"kind":"day","value":"1940-01-01"},"sources":["https://example.org/original"]}],"parties":[{"party":"unchanged"}],"office_links":[{"person":"existing"}]}
            self.art={"version":1,"people":{"existing":{"name":"Original","identity_sources":["https://example.org/original"],"portraits":[{"asset":"original.png"}]}}}
        def test_new_sourced_identity_does_not_create_offices_or_art(self):
            r,m,a=merge(self.base,self.art,{"people":[{"id":"new_person","name":"New Person","sources":["https://example.org/new"]}]})
            self.assertEqual(r["parties"],self.base["parties"])
            self.assertEqual(r["office_links"],self.base["office_links"])
            self.assertEqual(m["people"]["new_person"]["portraits"],[])
            self.assertEqual(a["added_person_ids"],["new_person"])
            self.assertEqual(self.base["people"][0]["name"],"Original")
        def test_existing_identity_facts_and_images_never_change(self):
            incoming={"people":[{"id":"existing","name":"Corrected name","born":{"kind":"day","value":"1941-01-01"},"sources":["https://example.org/extra"]}]}
            r,m,a=merge(self.base,self.art,incoming)
            self.assertEqual(r["people"][0]["name"],"Original")
            self.assertEqual(r["people"][0]["born"],self.base["people"][0]["born"])
            self.assertEqual(m["people"]["existing"]["portraits"],self.art["people"]["existing"]["portraits"])
            self.assertEqual(len(a["existing_fact_conflicts_preserved"]),1)
        def test_duplicate_unknown_fields_and_unsourced_identity_fail(self):
            p={"id":"x","name":"X","sources":["https://example.org/x"]}
            for people in ([p,p],[dict(p,fiction=True)],[dict(p,sources=[])],[dict(p,sources=["file:///bad"])]):
                with self.assertRaises(ValueError): merge(self.base,self.art,{"people":people})
        def test_repeated_import_is_idempotent(self):
            incoming={"people":[{"id":"new_person","name":"New Person","sources":["https://example.org/new"]}]}
            r,m,_=merge(self.base,self.art,incoming)
            r2,m2,a=merge(r,m,incoming)
            self.assertEqual((r,m),(r2,m2))
            self.assertEqual(a["added_person_ids"],[])
        def test_invalid_life_dates_fail_before_import(self):
            for bound in ({"kind":"day","value":"1990-02-30"},{"kind":"year","value":"0000"},{"kind":"open"}):
                with self.assertRaises(ValueError):
                    merge(self.base,self.art,{"people":[{"id":"new","name":"New","born":bound,"sources":["https://example.org/new"]}]})
            with self.assertRaises(ValueError):
                merge(self.base,self.art,{'people':[{'id':'new','name':'New','born':{'kind':'year','value':'2000'},
                    'died':{'kind':'year','value':'1990'},'sources':['https://example.org/new']}]})
    run=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Checks))
    return int(not run.wasSuccessful())

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source",nargs="?",type=Path)
    parser.add_argument("--apply",action="store_true",help="Write the reviewed people-only merge")
    parser.add_argument("--receipt",type=Path,help="Save provenance and exact preserved conflicts")
    parser.add_argument("--self-test",action="store_true")
    args=parser.parse_args()
    if args.self_test:return self_test()
    if args.source is None:parser.error("A reviewed research file is required")
    source=args.source.resolve()
    receipt=args.receipt.resolve() if args.receipt else None
    import_paths(source,receipt,ROOT,{REGISTRY,MANIFEST,ROOT/'spheres-sim/data/party_executive_eligibility.json'})
    data,source_raw=read_snapshot(source)
    before_registry,registry_raw=read_snapshot(REGISTRY)
    before_manifest,manifest_raw=read_snapshot(MANIFEST)
    registry,manifest,audit=merge(before_registry,before_manifest,data)
    checked=art.validate_manifest(manifest,ROOT,art.people_from_registry(registry))
    if not checked["valid"]:raise ValueError("; ".join(checked["errors"]))
    audit.update(source_file=source.relative_to(ROOT).as_posix(),
        source_sha256=hashlib.sha256(source_raw).hexdigest(),
        status="identities_imported" if args.apply else "people_only_preview",
        known_people=len(registry["people"]))
    outputs=[(REGISTRY,registry),(MANIFEST,manifest)] if args.apply else []
    if receipt: outputs.append((receipt,audit))
    write_transaction(outputs,{source:source_raw,REGISTRY:registry_raw,MANIFEST:manifest_raw})
    print(json.dumps(audit,ensure_ascii=True,indent=2))
    return 0

if __name__=="__main__":
    try:raise SystemExit(main())
    except (ValueError,OSError,KeyError,RuntimeError) as error:
        print(f"Historical identity import failed: {error}",file=sys.stderr)
        raise SystemExit(1)
