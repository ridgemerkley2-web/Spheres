#!/usr/bin/env python3
"""Rebuild the 1990–2035 leadership production inventory without inventing history.

The board is an audit, not a roster generator. All countries remain visible,
including those with no simulation party rows. Physical, validated cartoon files
are counted separately from research requirements, pending jobs and future fiction.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
from datetime import date, timedelta
import hashlib
import json
from pathlib import Path
import re
import sys
import unittest

import person_art_pipeline as art
import fictional_art_pipeline as fictional_art

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "spheres-web/data/leadership_production_2035.json"
START = date(1990, 1, 1)
CUTOFF = date(2026, 9, 7)
FUTURE = CUTOFF + timedelta(days=1)
END = date(2036, 1, 1)
STYLE = "spheres-web/ui/person-portraits/margaret-thatcher-cartoon-1990-v3.png"


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical(value) -> str:
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def nation_roster(root: Path) -> dict:
    source = (root / "spheres-sim/src/nations.rs").read_text(encoding="utf-8")
    block = source.split("pub const ROSTER:", 1)[1].split("\n];", 1)[0]
    rows = re.findall(r'\brow\("([A-Za-z0-9]+)",\s*"([^"]+)"', block)
    if not rows or len(dict(rows)) != len(rows):
        raise ValueError("Could not unambiguously read the complete nation roster")
    figures = load(root / "spheres-web/data/nation_figures.json")["nations"]
    if set(dict(rows)) != set(figures):
        raise ValueError("Nation roster and selector inventory disagree; no country may disappear")
    return {code: {"name": name, "region": figures[code]["region"],
                   "start_1990": figures[code]["start_1990"]} for code, name in rows}


def simulation_parties(root: Path) -> dict:
    """Read the two authored polity tables, retaining minor and alternate rows."""
    source = (root / "spheres-sim/src/government.rs").read_text(encoding="utf-8")
    result = {}
    for table in ("POLITIES", "D4_POLITIES"):
        block = source.split(f"pub const {table}:", 1)[1].split("\n];", 1)[0]
        for polity in block.split("Polity {")[1:]:
            match = re.search(r"nation:\s*NationId::(\w+)", polity)
            if not match:
                raise ValueError("Unrecognized polity nation")
            nation = match[1]
            for party, name, native, family in re.findall(
                    r'\b(?:p|pariah)\("([^"]+)",\s*"([^"]*)",\s*"([^"]*)",\s*Family::(\w+)', polity):
                key = nation, party
                row = {"name": name, "native": native, "family": family}
                if key in result and result[key] != row:
                    raise ValueError(f"Conflicting simulation party identity: {key}")
                result[key] = row
    return result


def interval(first: date, until: date) -> dict:
    return {"from": first.isoformat(), "to": until.isoformat()}


def safe_term_window(term: dict, person: dict) -> tuple[date, date] | None:
    """Use the known interior of uncertain terms, clipped to life and history.

    An unknown start/end never authorizes a whole-period portrait. Open end means
    known ongoing at the research cutoff; it cannot make a future historical fact.
    """
    begin, finish = term.get("from"), term.get("until")
    if not isinstance(begin, dict) or begin.get("kind") not in {"day", "month", "year"}:
        return None
    if not isinstance(finish, dict) or finish.get("kind") not in {"day", "month", "year", "open"}:
        return None
    _, start, _ = art.bound_range(begin, START, FUTURE)
    end = FUTURE if finish["kind"] == "open" else art.bound_range(finish, START, FUTURE)[0]
    start, end = max(START, start), min(FUTURE, end)
    born, died = person.get("born"), person.get("died")
    if isinstance(born, dict) and born.get("kind") in {"day", "month", "year"}:
        start = max(start, art.bound_range(born, START, FUTURE)[1])
    if isinstance(died, dict) and died.get("kind") in {"day", "month", "year"}:
        end = min(end, art.bound_range(died, START, FUTURE)[0])
    return (start, end) if start < end else None


def merge_windows(windows: list[tuple[date, date]]) -> list[tuple[date, date]]:
    merged = []
    for begin, end in sorted(windows):
        if begin >= end:
            continue
        if merged and begin <= merged[-1][1]:
            merged[-1] = merged[-1][0], max(end, merged[-1][1])
        else:
            merged.append((begin, end))
    return merged


def appearance_jobs(person_id: str, person: dict, windows: list, ready: list) -> list:
    jobs = []
    for start, end in merge_windows(windows):
        for begin, until in art.missing_intervals(ready, start, end):
            cursor = begin
            while cursor < until:
                boundary = date((cursor.year // 5 + 1) * 5, 1, 1)
                stop = min(until, boundary)
                jobs.append({"id": f"cartoon:{person_id}:{cursor}:{stop}:v1",
                             "person_id": person_id, "name": person["name"],
                             **interval(cursor, stop), "period_kind": "historical",
                             "status": "pending_source_and_visual_review",
                             "identity_sources": person.get("sources", []),
                             "requirements": ["dated likeness reference", "approved cartoon style",
                                              "physical image and hash", "identity, era and visual QA"],
                             "note": "Requested appearance window only; it does not extend an office, affiliation or life."})
                cursor = stop
    return jobs


def future_inventory(root: Path, ready=None) -> dict:
    ready = ready or {}
    path = root / "spheres-sim/data/future_party_leadership.json"
    result = {"from": FUTURE.isoformat(), "to": END.isoformat(), "period_kind": "fictional",
              "status": "research_and_runtime_catalog_pending", "validated_candidate_count": None,
              "rendered_avatar_count": None,
              "note": "Future candidates are gameplay fiction, not predicted winners. Profile templates and name pools are not completed characters or artwork."}
    if path.exists():
        profiles = load(path)
        if (profiles.get("historical_reference_through"), profiles.get("fictional_from"),
                profiles.get("fictional_until_exclusive")) != (CUTOFF.isoformat(), FUTURE.isoformat(), END.isoformat()):
            raise ValueError("Future profile dates disagree with the historical/fictional boundary")
        nation_ids = set(nation_roster(root))
        pool_mapping = profiles.get("country_name_pools", {})
        if set(pool_mapping) != nation_ids:
            raise ValueError("Future name-pool inventory excludes or invents nation identities")
        if set(pool_mapping.values()) - set(profiles.get("name_pools", {})):
            raise ValueError("Future countries reference unknown name pools")
        result.update(status="profile_catalog_present_runtime_candidates_not_audited",
                      source_path=path.relative_to(root).as_posix(), source_sha256=sha(path),
                      declared_profile_status=profiles.get("status"), declared_scope=profiles.get("scope"),
                      profile_count=len(profiles.get("profiles", [])),
                      configured_country_name_pools=len(pool_mapping),
                      authored_name_pool_count=len(profiles.get("name_pools", {})),
                      research=profiles.get("research", []), assumptions=profiles.get("assumptions", []))
    exported = root / "spheres-web/data/future_candidates_2035.json"
    if exported.exists():
        data = load(exported)
        if (data.get("historical_reference_through"), data.get("from"), data.get("until_exclusive")) != (CUTOFF.isoformat(), FUTURE.isoformat(), END.isoformat()):
            raise ValueError("Exported fictional candidate dates cross the history boundary")
        candidates = data.get("candidates", [])
        ids = [c.get("person_id") for c in candidates]
        if not all(isinstance(pid, str) and pid.startswith("fictional_") for pid in ids) or len(set(ids)) != len(ids):
            raise ValueError("Fictional candidates require unique explicitly fictional IDs")
        parties = {(c.get("nation"), c.get("party")) for c in candidates}
        if parties != set(simulation_parties(root)):
            raise ValueError("Fictional candidate export excludes or invents a simulation party")
        registry = load(root / "spheres-sim/data/party_leaders.json")
        component_ids = {(p["nation"], p["party"], c["id"]) for p in registry["parties"] for c in p.get("components", [])}
        actual_components = {(c["nation"], c["party"], c["component"]) for c in candidates if c.get("component")}
        if actual_components != component_ids:
            raise ValueError("Fictional candidate export excludes or invents a coalition component")
        if any(c.get("origin") != "fictional_successor" or not c.get("name") for c in candidates):
            raise ValueError("Exported future candidates must be named and explicitly fictional")
        counts = (len(candidates), len(parties), len({c["nation"] for c in candidates}))
        if counts != (data.get("candidate_count"), data.get("party_count"), data.get("country_count")):
            raise ValueError("Reported future counts disagree with actual exported records")
        if set(ids) & {p["id"] for p in registry["people"]}:
            raise ValueError("A fictional candidate ID appears in the historical person registry")
        result.update(status="exported_fictional_templates_audited_art_and_editorial_review_incomplete",
                      validated_candidate_count=counts[0], represented_party_count=counts[1], represented_country_count=counts[2],
                      available_component_templates=sum(c.get("component_available") is not False for c in candidates),
                      archived_component_templates=sum(c.get("component_available") is False for c in candidates),
                      rendered_person_count=len(set(ids) & set(ready)),
                      rendered_avatar_count=sum(len(records) for pid, records in ready.items() if pid in set(ids)),
                      validated_cartoon_assets=ready,
                      export_path=exported.relative_to(root).as_posix(), export_sha256=sha(exported),
                      candidates=candidates,
                      note="Counts verify exported fictional IDs, country/party/component coverage and dates. They do not establish complete research, editorial review or finished cartoon art.")
    return result


def build(root: Path = ROOT) -> dict:
    roster = nation_roster(root)
    simulation = simulation_parties(root)
    registry = load(root / "spheres-sim/data/party_leaders.json")
    manifest = load(root / "spheres-web/data/person_portraits.json")
    known = art.people_from_registry(registry)
    checked = art.validate_manifest(manifest, root, known)
    if not checked["valid"]:
        raise ValueError("Portrait manifest fails physical validation: " + "; ".join(checked["errors"]))
    ready = {pid: [p for p in portraits if p.get("style") == "cartoon"]
             for pid, portraits in checked["ready"].items()}
    ready = {pid: portraits for pid, portraits in ready.items() if portraits}
    authored = {(p["nation"], p["party"]): p for p in registry["parties"]}
    if len(authored) != len(registry["parties"]):
        raise ValueError("Duplicate historical party rows")
    if set(authored) != set(simulation):
        raise ValueError(f"Simulation/history party coverage differs: missing={set(simulation)-set(authored)}, extra={set(authored)-set(simulation)}")
    if {n for n, _ in authored} - set(roster):
        raise ValueError("A historical party references a nation outside the roster")

    needs, reasons = defaultdict(list), defaultdict(list)
    countries = []
    for nation, info in roster.items():
        parties = []
        for (code, party_id), party in authored.items():
            if code != nation:
                continue
            terms = []
            for term in party.get("terms", []):
                pid = term.get("person") or term.get("person_id")
                if pid not in known:
                    raise ValueError(f"Unknown term person: {pid}")
                window = safe_term_window(term, known[pid])
                if window:
                    needs[pid].append(window)
                    reasons[pid].append({"kind": "sourced_party_term", "nation": nation,
                                         "party": party_id, "component": term.get("component"),
                                         "term_id": term["id"], **interval(*window)})
                gaps = art.missing_intervals(ready.get(pid, []), *window) if window else []
                terms.append({"id": term["id"], "person_id": pid, "name": known[pid]["name"],
                              "component": term.get("component"), "role": term.get("role"),
                              "source_from": term.get("from"), "source_until": term.get("until"),
                              "safe_art_window": interval(*window) if window else None,
                              "art_status": "needs_date_research" if not window else "missing" if gaps else "covered",
                              "missing_art_windows": [interval(*g) for g in gaps]})
            components = [{"id": c["id"], "name": c["name"], "sources": c.get("sources", []),
                           "known_term_ids": [t["id"] for t in terms if t["component"] == c["id"]],
                           "history_status": "partial" if any(t["component"] == c["id"] for t in terms) else "missing",
                           "future_status": "requires_distinct_fictional_candidates"}
                          for c in party.get("components", [])]
            parties.append({"id": party_id, **simulation[(nation, party_id)], "kind": party.get("kind"),
                            "history_status": party.get("coverage", "gap"),
                            "sources": party.get("sources", []), "declared_history_gaps": party.get("gaps", []),
                            "components": components, "terms": terms,
                            "history_requirements": ["Verify party identity and lifespan", "Research every leader, acting leader and co-leader",
                                                     "Preserve minority and coalition component identities", "Source exact or explicitly uncertain term dates"],
                            "future_status": "requires_research_informed_fictional_successors"})
        countries.append({"id": nation, **info, "party_rows": len(parties),
                          "inventory_status": "simulation_rows_inventoried" if parties else "no_simulation_party_rows",
                          "all_real_parties_status": "incomplete_research",
                          "party_inventory_requirement": "Audit political organizations for the full historical period, including small, unrepresented, dissolved and successor parties; existing simulation rows are not an exhaustive real-world census.",
                          "institutional_requirement": "Research institutions, independent factions and political organizations; zero simulation party rows does not mean the country had no political organizations." if not parties else None,
                          "parties": parties})

    for link in registry.get("office_links", []):
        pid = link["person"]
        if pid not in known or link["nation"] not in roster:
            raise ValueError("Unknown original executive identity or nation")
        point = (START, START + timedelta(days=1))
        born = known[pid].get("born")
        died = known[pid].get("died")
        if born and art.bound_range(born, START, FUTURE)[1] > START:
            continue
        if died and art.bound_range(died, START, FUTURE)[0] <= START:
            continue
        needs[pid].append(point)
        reasons[pid].append({"kind": "1990_original_executive_observation", "nation": link["nation"], **interval(*point),
                             "note": "The seed confirms identity on 1 January 1990 only; no unsourced office end or later eligibility is inferred."})

    people, jobs = [], []
    for pid, person in sorted(known.items()):
        windows = merge_windows(needs[pid])
        pending = appearance_jobs(pid, person, windows, ready.get(pid, []))
        jobs.extend(pending)
        people.append({"id": pid, "name": person["name"], "period_kind": "historical",
                       "born": person.get("born"), "died": person.get("died"),
                       "required_art_windows": [interval(*w) for w in windows],
                       "reasons": reasons[pid], "validated_cartoon_assets": ready.get(pid, []),
                       "pending_art_job_ids": [j["id"] for j in pending],
                       "status": "eligibility_research_required" if not windows else "art_pending" if pending else "known_windows_covered"})
    fiction_manifest = load(root / "spheres-web/data/fictional_portraits.json")
    export_path = root / "spheres-web/data/future_candidates_2035.json"
    if export_path.exists():
        fiction_checked = fictional_art.validate(fiction_manifest, load(export_path), root, manifest)
    elif fiction_manifest.get("people"):
        raise ValueError("A populated fictional portrait manifest requires the exported candidate catalogue")
    else:
        fiction_checked = {"valid": True, "errors": [], "ready": {}}
    if not fiction_checked["valid"]:
        raise ValueError("Fictional cartoon manifest fails physical validation: " + "; ".join(fiction_checked["errors"]))
    fiction_ready = fiction_checked["ready"]
    future = future_inventory(root, fiction_ready)
    future_by_party = defaultdict(list)
    for candidate in future.get("candidates", []):
        future_by_party[(candidate["nation"], candidate["party"])].append(candidate["person_id"])
    for country in countries:
        for party in country["parties"]:
            party["future_candidate_ids"] = future_by_party[(country["id"], party["id"])]
            if party["future_candidate_ids"]:
                party["future_status"] = "fictional_templates_present_art_and_editorial_review_incomplete"
    flat_parties = [p for c in countries for p in c["parties"]]
    count = Counter(p["history_status"] for p in flat_parties)
    return {"version": 1, "production_version": "global-cartoon-1990-2035-v1",
            "source_cutoff": CUTOFF.isoformat(), "historical_period": interval(START, FUTURE),
            "future_period": interval(FUTURE, END), "date_convention": "from inclusive, to exclusive",
            "style": {"kind": "fixed_2d_cartoon", "anchor": STYLE, "anchor_sha256": sha(root / STYLE),
                      "user_style_approval": {"date": "2026-09-07", "statement": "Looks good please do them for all parties for every country until 2035",
                                              "scope": "Visual style only; this does not approve unmade art, historical research or fictional biographies."}},
            "input_hashes": {p: sha(root / p) for p in ("spheres-sim/src/nations.rs", "spheres-sim/src/government.rs",
                              "spheres-sim/data/party_leaders.json", "spheres-web/data/person_portraits.json", "spheres-web/data/fictional_portraits.json")},
            "counts": {"nation_identities": len(countries), "starting_nations": sum(c["start_1990"] for c in countries),
                       "successor_nations": sum(not c["start_1990"] for c in countries),
                       "nations_with_simulation_parties": sum(bool(c["parties"]) for c in countries),
                       "nations_without_simulation_parties": sum(not c["parties"] for c in countries),
                       "simulation_party_rows": len(flat_parties), "explicit_components": sum(len(p["components"]) for p in flat_parties),
                       "parties_verified_history": count["verified"], "parties_partial_history": count["partial"],
                       "parties_with_declared_history_gaps": sum(bool(p["declared_history_gaps"]) for p in flat_parties),
                       "known_historical_people": len(people), "known_party_terms": sum(len(p["terms"]) for p in flat_parties),
                       "validated_cartoon_people": len(ready), "validated_cartoon_assets": sum(map(len, ready.values())),
                       "validated_fictional_cartoon_people": len(fiction_ready), "validated_fictional_cartoon_assets": sum(map(len, fiction_ready.values())),
                       "validated_cartoon_total_assets": sum(map(len, ready.values())) + sum(map(len, fiction_ready.values())),
                       "pending_historical_art_jobs": len(jobs), "people_without_researched_eligibility": sum(not p["required_art_windows"] for p in people),
                       "countries_with_complete_all_party_history": 0, "images_generated_by_this_tool": 0},
            "completion_policy": "Inventory, templates and queued jobs are never reported as completed research, characters or artwork. Whole-world completion requires an independently reviewed exhaustive party census for every country.",
            "future": future, "countries": countries, "people": people, "historical_art_jobs": jobs}


def self_test() -> int:
    class Checks(unittest.TestCase):
        def test_global_inventory_excludes_no_nations_or_parties(self):
            board = build()
            self.assertEqual({c["id"] for c in board["countries"]}, set(nation_roster(ROOT)))
            self.assertEqual({(c["id"], p["id"]) for c in board["countries"] for p in c["parties"]}, set(simulation_parties(ROOT)))
            self.assertEqual(board["counts"]["nation_identities"], len(nation_roster(ROOT)))
            self.assertEqual(board["counts"]["simulation_party_rows"], len(simulation_parties(ROOT)))
            self.assertIn(("Belgium", "be_vb"), simulation_parties(ROOT))

        def test_small_coalition_components_stay_distinct(self):
            board = build()
            party = next(p for c in board["countries"] if c["id"] == "UK" for p in c["parties"] if p["id"] == "uk_nat")
            self.assertEqual({c["id"] for c in party["components"]}, {"snp", "plaid_cymru"})
            self.assertTrue(all(c["known_term_ids"] for c in party["components"]))

        def test_terms_clip_to_life_and_cutoff(self):
            term = {"from": {"kind": "day", "value": "1980-01-01"}, "until": {"kind": "open"}}
            self.assertEqual(safe_term_window(term, {}), (START, FUTURE))
            person = {"died": {"kind": "day", "value": "2010-06-01"}}
            self.assertEqual(safe_term_window(term, person), (START, date(2010, 6, 1)))
            self.assertIsNone(safe_term_window({"from": {"kind": "unknown"}, "until": {"kind": "open"}}, {}))

        def test_uncertain_boundaries_do_not_invent_exact_terms(self):
            term = {"from": {"kind": "month", "value": "2000-09"}, "until": {"kind": "year", "value": "2005"}}
            self.assertEqual(safe_term_window(term, {}), (date(2000, 9, 30), date(2005, 1, 1)))

        def test_jobs_use_only_required_windows_and_never_future_history(self):
            jobs = appearance_jobs("example", {"name": "Example"}, [(date(2018, 7, 1), FUTURE)], [])
            self.assertEqual(jobs[0]["from"], "2018-07-01")
            self.assertEqual(jobs[-1]["to"], FUTURE.isoformat())
            self.assertTrue(all(j["period_kind"] == "historical" and j["status"].startswith("pending") for j in jobs))
            self.assertEqual(appearance_jobs("example", {"name": "Example"}, [], []), [])

        def test_existing_art_covers_only_its_reviewed_era(self):
            jobs = appearance_jobs("example", {"name": "Example"}, [(START, date(2000, 1, 1))], [{"from": "1990-01-01", "to": "1995-01-01"}])
            self.assertEqual([(j["from"], j["to"]) for j in jobs], [("1995-01-01", "2000-01-01")])

        def test_future_is_separate_and_queue_does_not_count_as_completed(self):
            board = build()
            self.assertEqual(board["historical_period"]["to"], board["future_period"]["from"])
            self.assertEqual(board["future_period"]["to"], "2036-01-01")
            self.assertEqual(board["future"]["period_kind"], "fictional")
            self.assertEqual(board["counts"]["images_generated_by_this_tool"], 0)
            self.assertEqual(board["counts"]["pending_historical_art_jobs"], len(board["historical_art_jobs"]))
            self.assertEqual(board["counts"]["countries_with_complete_all_party_history"], 0)

        def test_exported_future_records_are_complete_templates_not_counted_as_art(self):
            board = build()
            future = board["future"]
            if "candidates" not in future:
                self.skipTest("No exported fictional candidate catalogue yet")
            self.assertEqual(future["validated_candidate_count"], len(future["candidates"]))
            self.assertEqual(future["represented_party_count"], len(simulation_parties(ROOT)))
            self.assertTrue(all(c["origin"] == "fictional_successor" for c in future["candidates"]))
            self.assertLessEqual(future["rendered_person_count"], future["validated_candidate_count"])
            self.assertEqual(future["rendered_avatar_count"], sum(map(len, future["validated_cartoon_assets"].values())))
    result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Checks))
    return int(not result.wasSuccessful())


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("build", "check", "summary", "self-test"))
    args = parser.parse_args()
    if args.command == "self-test":
        return self_test()
    board = build()
    if args.command == "build":
        OUTPUT.write_text(canonical(board), encoding="utf-8", newline="\n")
    elif args.command == "check":
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != canonical(board):
            print("Production board is stale. Run: python tools/avatars/leadership_production.py build", file=sys.stderr)
            return 1
    print(json.dumps(board["counts"], indent=2))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, KeyError) as error:
        print(f"Leadership production audit failed: {error}", file=sys.stderr)
        raise SystemExit(1)
