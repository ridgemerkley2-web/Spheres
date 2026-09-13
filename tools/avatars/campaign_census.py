#!/usr/bin/env python3
"""Freeze a reproducible *partial* C01 inventory; never invent missing parties.

Only reads existing catalogues. No network, image generation, roster writes or
game commands. Physical art validation is explicitly the older production audit,
not a new visual review. Run normally to write C01 outputs, or --check to compare.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path

import leadership_production as production

ROOT = Path(__file__).resolve().parents[2]
DEST = Path("docs/campaign-certification/C01")
CERTIFIED = ("France", "Japan", "India", "Brazil", "SouthAfrica", "Tonga",
             "SaudiArabia", "USSR", "Russia")
BASELINE = "872442d7411d9986321248b41ccadf79e2851d1c"
INPUTS = (
    "spheres-sim/src/nations.rs", "spheres-sim/src/government.rs",
    "spheres-sim/data/party_leaders.json",
    "spheres-sim/data/party_executive_eligibility.json",
    "spheres-sim/data/future_party_leadership.json",
    "spheres-sim/data/future_party_leadership_seats.json",
    "spheres-sim/data/future_party_continuation.json",
    "spheres-web/data/nation_figures.json",
    "spheres-web/data/future_candidates_2035.json",
    "spheres-web/data/person_portraits.json",
    "spheres-web/data/fictional_portraits.json",
    "spheres-web/data/leadership_production_2035.json",
    "tools/avatars/leadership_production.py",
    "tools/avatars/person_art_pipeline.py", "tools/avatars/fictional_art_pipeline.py",
    "tools/avatars/campaign_census.py",
)


def load(root, name):
    return json.loads((root / name).read_text(encoding="utf-8-sig"))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical(data):
    return json.dumps(data, ensure_ascii=False, indent=2) + "\n"


def unique(rows, key, label):
    result = {}
    for row in rows:
        identity = key(row)
        if identity in result:
            raise ValueError(f"Duplicate {label}: {identity}")
        result[identity] = row
    return result


def batches(prefix, nation, members, phase, dependencies):
    """Explicit bounded sessions: at most ten known records, not ten countries."""
    deliverables = {
        "known_historical_chains": "Source exact or explicitly uncertain role/affiliation/life intervals; distinguish leader, chair, acting and collective seats. Split long chains into additional numbered sessions rather than claiming their people are reviewed.",
        "fictional_editorial_review": "Review each exact fictional ID, authored biography/name/appearance, role eligibility, institution seats and organizational continuation. Prepare and visually review its physical fixed 2D cartoon; existing reviewed art may be reused within its documented window. A template is not an illustrated character.",
        "historical_cartoon_jobs": "First verify the named person's dated identity/likeness and safe appearance window, then create/review a physical fixed 2D cartoon with provenance. A requested art interval does not extend an office, affiliation or life.",
    }
    return [{"id": f"{prefix}-{nation}-B{offset // 10 + 1:03d}",
             "nation": nation, "priority": "certified_first" if nation in CERTIFIED else "worldwide",
             "phase": phase, "status": "open", "depends_on": dependencies,
             "members": members[offset:offset + 10], "maximum_reviewed_records": 10,
             "deliverable": deliverables[phase]}
            for offset in range(0, len(members), 10)]


def org_identity(nation, party, component=None):
    return f"{nation}/{party}" + (f"/{component}" if component else "")


def build(root=ROOT):
    registry = load(root, "spheres-sim/data/party_leaders.json")
    board = load(root, "spheres-web/data/leadership_production_2035.json")
    candidates = load(root, "spheres-web/data/future_candidates_2035.json")
    eligibility = load(root, "spheres-sim/data/party_executive_eligibility.json")
    seats = load(root, "spheres-sim/data/future_party_leadership_seats.json")
    continuation = load(root, "spheres-sim/data/future_party_continuation.json")
    roster, live = production.nation_roster(root), production.simulation_parties(root)
    parties = unique(registry["parties"], lambda p: (p["nation"], p["party"]), "party")
    people = unique(registry["people"], lambda p: p["id"], "person")
    future = unique(candidates["candidates"], lambda p: p["person_id"], "fictional person")
    if set(live) != set(parties) or set(CERTIFIED) - set(roster):
        raise ValueError("Live party rows/registry or certified identities disagree")
    if set(future) & set(people) or any(not pid.startswith("fictional_") for pid in future):
        raise ValueError("Historical and fictional identities must remain separate")
    if {(p["nation"], p["party"]) for p in future.values()} != set(live):
        raise ValueError("Fictional export must cover exactly the represented party rows")
    boundary = (registry["reference_from"], registry["reference_through"],
                candidates["from"], candidates["until_exclusive"])
    if boundary != ("1990-01-01", "2026-09-07", "2026-09-08", "2036-01-01"):
        raise ValueError("Cutoff changed: review the explicit C01 policy before regeneration")
    if candidates["historical_reference_through"] != boundary[1] or board["source_cutoff"] != boundary[1]:
        raise ValueError("Historical/fictional boundary differs between sources")

    stale = [{"path": name, "recorded_sha256": old, "current_sha256": digest(root / name)}
             for name, old in board["input_hashes"].items() if digest(root / name) != old]
    old_rows = {(c["id"], p["id"]): {k: p[k] for k in ("name", "native", "family")}
                for c in board["countries"] for p in c["parties"]}
    # Snapshot jobs cannot be silently attached to a changed person/party roster.
    jobs = unique(board["historical_art_jobs"], lambda j: j["id"], "art job")
    job_inputs_unchanged = all(x["path"] not in {
        "spheres-sim/data/party_leaders.json", "spheres-web/data/person_portraits.json"}
        for x in stale)
    if not job_inputs_unchanged or set(j["person_id"] for j in jobs.values()) - set(people):
        raise ValueError("Historical artwork job inputs changed; rebuild/review their source audit first")

    terms = []
    organizations = []
    lifecycle = []
    for (nation, party), row in sorted(parties.items()):
        identity = org_identity(nation, party)
        organizations.append({"id": identity, "nation": nation, "party": party,
            "representation": "simulation_party_row", **live[nation, party],
            "organization_kind": row["kind"], "history_status": row["coverage"],
            "identity_note": row.get("identity_note"), "sources": row.get("sources", []),
            "declared_gaps": row.get("gaps", []),
            "historical_term_ids": [t["id"] for t in row["terms"] if not t.get("component")],
            "component_ids": [org_identity(nation, party, c["id"]) for c in row.get("components", [])]})
        for entity, entity_id in [(row, identity)] + [
                (c, org_identity(nation, party, c["id"])) for c in row.get("components", [])]:
            dates = {key: entity[key] for key in ("founded", "dissolved") if key in entity}
            if dates:
                lifecycle.append({"organization_id": entity_id, "kind": "existing_registry_bounds",
                                  "bounds": dates, "sources": entity.get("sources", [])})
        for component in row.get("components", []):
            cid = component["id"]
            organizations.append({"id": org_identity(nation, party, cid), "nation": nation,
                "party": party, "component": cid, "name": component["name"],
                "representation": "registered_component_or_historical_name_phase",
                "organization_kind": "not_separately_typed",
                "history_status": "partial" if any(t.get("component") == cid for t in row["terms"]) else "missing",
                "sources": component.get("sources", []),
                "historical_term_ids": [t["id"] for t in row["terms"] if t.get("component") == cid]})
        for term in row["terms"]:
            if term["person"] not in people:
                raise ValueError(f"Term references absent person: {term['id']}")
            terms.append({"nation": nation, "party": party,
                          "organization_id": org_identity(nation, party, term.get("component")), **term})
    unique(organizations, lambda o: o["id"], "organization")
    term_ids = unique(terms, lambda t: t["id"], "term")
    org_ids = {o["id"] for o in organizations}
    if any(t["organization_id"] not in org_ids for t in terms):
        raise ValueError("Term references an unregistered component")
    expected_components = {(p["nation"], p["party"], c["id"]) for p in parties.values() for c in p.get("components", [])}
    if {(p["nation"], p["party"], p["component"]) for p in future.values() if p.get("component")} != expected_components:
        raise ValueError("Future component export differs from registered components")
    for grant in eligibility["historical_grants"]:
        target = term_ids.get(grant["term"])
        if target is None or any(target.get(k) != grant.get(k) for k in ("nation", "party", "person", "component")):
            raise ValueError("Executive grant points to an absent or different party office holder")
    for row in continuation["parties"]:
        lifecycle.append({"organization_id": org_identity(row["nation"], row["party"]),
                          "kind": "future_editorial_continuation_disclosure", **row})

    links = registry["office_links"]
    countries = []
    work = []
    historical_members = defaultdict(list)
    future_members = defaultdict(list)
    art_members = defaultdict(list)
    nation_order = {n: i for i, n in enumerate(CERTIFIED)}
    for org in organizations:
        historical_members[org["nation"]].append(org["id"])
    for candidate in sorted(future.values(), key=lambda c: c["person_id"]):
        future_members[candidate["nation"]].append(candidate["person_id"])
    board_people = {p["id"]: p for p in board["people"]}
    for job in jobs.values():
        nations = {r["nation"] for r in board_people[job["person_id"]]["reasons"]}
        if not nations or nations - set(roster):
            raise ValueError("Art job needs a known country association")
        # One physical-person job, even when the person belongs to several offices.
        owner = min(nations, key=lambda n: (nation_order.get(n, len(CERTIFIED)), n))
        art_members[owner].append({"job_id": job["id"], "person_id": job["person_id"],
                                   "associated_nations": sorted(nations), "from": job["from"], "to": job["to"]})
    for nation in sorted(roster, key=lambda n: (nation_order.get(n, len(CERTIFIED)), n)):
        cparties = [p for p in parties.values() if p["nation"] == nation]
        country_work = f"C01-{nation}-ORG-001"
        countries.append({"id": nation, **roster[nation], "certified_case": nation in CERTIFIED,
            "party_rows": len(cparties), "registered_components": sum(len(p.get("components", [])) for p in cparties),
            "partial_party_histories": sum(p["coverage"] == "partial" for p in cparties),
            "known_party_terms": sum(len(p["terms"]) for p in cparties),
            "seed_executive_observations": sum(x["nation"] == nation for x in links),
            "institution_policy_records": sum(x["nation"] == nation for x in eligibility["institutions"]),
            "fictional_templates": len(future_members[nation]),
            "all_real_organizations_status": "unreviewed_exhaustiveness",
            "unrepresented_organization_count": None, "unknown_coverage_is_not_zero": True,
            "census_work_order": country_work})
        work.append({"id": country_work, "nation": nation,
            "priority": "certified_first" if nation in CERTIFIED else "worldwide", "phase": "organization_census",
            "status": "open", "known_rows_are_exhaustive": False,
            "required_discovery": ["unrepresented/minor organizations", "dissolved and merged organizations",
                                   "successor and renamed organizations", "coalition membership intervals",
                                   "independent factions and collective/governing institutions"],
            "deliverable": "Cited organization IDs, type, lifespan/uncertainty, country jurisdiction interval, and explicit unresolved discoveries. Split into further numbered sessions as needed; no inferred zero-party waiver."})
        work.append({"id": f"C01-{nation}-ROLE-001", "nation": nation,
            "priority": "certified_first" if nation in CERTIFIED else "worldwide", "phase": "institution_and_office_roles",
            "status": "open", "depends_on": [country_work],
            "deliverable": "Separate party leader/chair, parliamentary leader, head of state/government and collective seats; source nomination/appointment and shared-office rules. Existing seed links and gameplay grants do not close historical office chains."})
        work += batches("CH", nation, historical_members[nation], "known_historical_chains", [country_work, f"C01-{nation}-ROLE-001"])
        work += batches("CF", nation, future_members[nation], "fictional_editorial_review", [country_work, f"C01-{nation}-ROLE-001"])
        work += batches("CA", nation, art_members[nation], "historical_cartoon_jobs", [country_work, f"C01-{nation}-ROLE-001"])
    unique(work, lambda w: w["id"], "work order")
    manifests = [load(root, p)["people"] for p in (
        "spheres-web/data/person_portraits.json", "spheres-web/data/fictional_portraits.json")]
    counts = {"nation_identities": len(roster), "starting_nations": sum(c["start_1990"] for c in roster.values()),
        "successor_nations": sum(not c["start_1990"] for c in roster.values()),
        "represented_party_rows": len(live), "countries_with_party_rows": len({n for n, _ in live}),
        "countries_without_party_rows": len(roster) - len({n for n, _ in live}),
        "party_row_kind": dict(sorted(Counter(p["kind"] for p in parties.values()).items())),
        "party_history_status": dict(sorted(Counter(p["coverage"] for p in parties.values()).items())),
        "registered_components_or_name_phases": len(expected_components),
        "historical_people": len(people), "historical_terms": len(terms),
        "component_term_records": sum(bool(t.get("component")) for t in terms),
        "historical_term_kind": dict(sorted(Counter(t["kind"] for t in terms).items())),
        "executive_seed_observations": len(links), "institution_policy_records": len(eligibility["institutions"]),
        "historical_executive_gameplay_grants": len(eligibility["historical_grants"]),
        "future_seat_policies": len(seats["parties"]),
        "existing_lifecycle_records": len(lifecycle), "future_templates": len(future),
        "historical_manifest_assets": sum(len(v["portraits"]) for v in manifests[0].values()),
        "fictional_manifest_assets": sum(len(v["portraits"]) for v in manifests[1].values()),
        "historical_art_snapshot_pending_jobs": len(jobs),
        "people_with_unknown_appearance_eligibility": sum(p["status"] == "eligibility_research_required" for p in board["people"]),
        "exhaustive_country_censuses": 0, "unrepresented_organizations": None,
        "work_orders_by_phase": dict(sorted(Counter(w["phase"] for w in work).items()))}
    summary = {"format": "spheres-c01-partial-census/v1", "status": "partial_baseline_recorded_census_open",
        "source_baseline_revision": BASELINE, "c01_complete": False, "g2_prerequisite_satisfied": False,
        "historical_from": boundary[0], "existing_research_cutoff": boundary[1],
        "fictional_from": boundary[2], "until_exclusive": boundary[3],
        "cutoff_note": "Frozen existing data boundary, not new present-day research. Reassess the cutoff before content acceptance; fictional templates are not real-world forecasts.",
        "certified_country_cases": list(CERTIFIED[:-2]) + ["USSR -> Russia"],
        "certified_identity_ids": list(CERTIFIED), "counts": counts,
        "production_snapshot": {"all_declared_inputs_current": not stale, "stale_inputs": stale,
            "current_party_rows_match_snapshot_exactly": live == old_rows,
            "job_person_and_portrait_inputs_current": job_inputs_unchanged,
            "physical_art_revalidated_this_run": False,
            "historical_jobs_are_known_backlog_not_worldwide_total": True},
        "source_files": [{"path": name, "bytes": (root / name).stat().st_size, "sha256": digest(root / name)} for name in INPUTS],
        "acceptance_remaining": ["Independently enumerate real organizations for every country and jurisdiction period, including unrepresented/minor/dissolved/successor bodies.",
            "Resolve leaders, chairs, parliamentary and executive offices, coalition components and collective institutions without substituting one role for another.",
            "Review certified cases first, explicitly track unknowns worldwide, and approve the research cutoff.",
            "Expand uniquely numbered historical/future work orders with discoveries. C01 inventories the work; subsequent character sessions complete historical chains and physical cartoon art. Neither an artwork count nor this partial recount establishes C01 closure."]}
    return {"census.json": summary, "countries.json": countries,
        "represented-organizations.json": organizations,
        "roles-and-lifecycle.json": {"term_records": terms, "seed_executive_observations": links,
            "historical_default_executive_policy": eligibility["historical_default"],
            "historical_executive_gameplay_grants": eligibility["historical_grants"],
            "institution_policy_records": eligibility["institutions"],
            "future_seat_policies": seats["parties"], "lifecycle_disclosures": lifecycle,
            "appearance_eligibility_research_backlog": [{"person_id": p["id"], "name": p["name"],
                "status": p["status"], "known_associations": p["reasons"],
                "required_action": "Research identity, life and office intervals before assigning an artwork window; an empty association list is unknown, not permission to invent a country/role."}
                for p in board["people"] if p["status"] == "eligibility_research_required"],
            "interpretation": "Exact existing role labels, dates, uncertainty and sources retained. Components may be historical name phases, not simultaneous coalition members. Policy records/grants and seed executive observations do not establish complete historical office chains or a collective-institution census."},
        "work-orders.json": work}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Compare generated evidence without writing")
    parser.add_argument("--output-dir", type=Path, default=ROOT / DEST)
    args = parser.parse_args()
    outputs = build()
    for name, value in outputs.items():
        path, content = args.output_dir / name, canonical(value)
        if args.check:
            if not path.exists() or path.read_text(encoding="utf-8") != content:
                raise SystemExit(f"C01 evidence differs: {path}")
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8", newline="\n")
    print(canonical({"check": args.check, "status": outputs["census.json"]["status"],
                     "counts": outputs["census.json"]["counts"]}), end="")


if __name__ == "__main__":
    main()
