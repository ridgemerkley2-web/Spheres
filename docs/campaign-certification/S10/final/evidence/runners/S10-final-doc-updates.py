"""Draft or apply the explicitly approved S10 gameplay/content scope amendment.

Default execution is a read-only proposed diff. Applying requires the exact user
answer, its recorded receipt, a passed final manifest, and the final inventory.
This helper does not qualify a build, create evidence, change game data, or
complete any character/content/release milestone. Earlier evidence is immutable.
"""
import argparse
import copy
import datetime
import difflib
import hashlib
import json
import pathlib
import re
import subprocess

BASE = pathlib.Path(__file__).resolve().parent
REPO = BASE / "integration"
CANDIDATE = "bcdf72bcbe8947966359deb95f715cb526a68def"
CHOICE = "Finish S10 gameplay; keep content certification separate (Recommended)"
FINAL = "docs/campaign-certification/S10/final/manifest.json"
OWNED = (
    "docs/planning/campaign-pathway.json",
    "docs/CERTIFIED_CAMPAIGN_PATHWAY.md",
    "docs/campaign-certification/C01/README.md",
    "docs/campaign-certification/S10/README.md",
)
OUTPUT = (
    "G2: qualified government and diplomatic gameplay with explicit historical "
    "coverage gaps."
)
G2_REASON = (
    "Earned for the qualified finance, construction, supplier, research, government "
    "and diplomacy paths under the explicitly approved S10 scope amendment. "
    "Historical coverage remains incomplete and is required through C01–C06 and "
    "S23 before CP1 certification. Earlier and current qualification retain their "
    "exact source, build and scenario scopes."
)
CONTENT_OBLIGATIONS = [
    "C01–C07 retain the worldwide organization, institution, leadership and cartoon work.",
    "C06 and S23 must close the eight certified country casts and their historical/future coverage before CP1 qualification.",
    "Current discovery packets remain partial; S10/G2 gameplay closure establishes no exhaustive census, complete leadership chain or new portrait coverage.",
]


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def git(*args):
    return subprocess.check_output(
        ["git", "-c", "core.longpaths=true", *args], cwd=REPO
    )


def once(text, old, new):
    assert text.count(old) == 1, f"Expected exactly one unchanged anchor: {old!r}"
    return text.replace(old, new, 1)


def positive(value, label):
    assert isinstance(value, int) and not isinstance(value, bool) and value > 0, label


def check_qualified(manifest, path, approval_choice):
    assert path == (REPO / FINAL).resolve(), "Unexpected final manifest path"
    assert manifest["format"] == "spheres-s10-final/v1"
    assert manifest["status"] == "s10_gameplay_complete"
    assert manifest["candidate_revision"] == CANDIDATE
    assert manifest["s10_complete"] is True and manifest["g2_earned"] is True
    assert manifest["c01_complete"] is False and manifest["s11_started"] is False
    approval = manifest["approval"]
    assert approval_choice == CHOICE, "The exact recorded approval choice is required"
    assert approval["answer"] == CHOICE, "Manifest must retain the actual user answer"
    assert isinstance(approval["question"], str) and approval["question"].strip()
    received = datetime.datetime.fromisoformat(approval["received_utc"].replace("Z", "+00:00"))
    assert received.tzinfo is not None, "Approval receipt must carry its time zone"
    q = manifest["qualification"]
    assert q["passed"] is True
    for name in ("windows_web", "windows_agency", "windows_leadership"):
        positive(q[name]["passed"], name)
        assert q[name]["failed"] == 0, name
        assert q[name].get("ignored", 0) >= 0
    positive(q["windows_interface"]["pass"], "Windows interface")
    assert q["windows_interface"]["fail"] == 0
    assert q["windows_interface"]["tests"] == (
        q["windows_interface"]["pass"] + q["windows_interface"]["skipped"]
    )
    linux = {row["name"]: row["totals"] for row in q["linux_checks"]}
    assert len(linux) == len(q["linux_checks"]), "Duplicate Linux lane"
    assert {"web", "agency"}.issubset(linux)
    for name, totals in linux.items():
        positive(totals["passed"], "Linux " + name)
        assert totals["failed"] == 0, name
    assert q["content_tests"] == 102
    assert q["browser_countries"] == 8 and q["browser_elapsed_days"] == 0
    assert q["ordinary_browser_passed"] is True
    inventory = path.parent / "evidence/inventory.json"
    digest = manifest["evidence"]["inventory_sha256"]
    assert re.fullmatch(r"[0-9a-f]{64}", digest)
    assert sha(inventory) == digest, "Final inventory does not match the manifest"
    return approval


def draft_json(text, approval, date, qualified):
    plan = json.loads(text)
    before = copy.deepcopy(plan)
    sessions = {s["id"]: s for s in plan["sessions"]}
    s10 = sessions["S10"]
    assert s10["status"] == "in_progress" and s10["depends"] == ["S05", "C01"]
    assert [row["id"] for row in s10["increments"]] == [f"S10.{x}" for x in "abcdefgh"]
    assert all(row["status"] == "complete" for row in s10["increments"])
    assert sessions["C01"]["status"] == "in_progress"
    assert all(sessions[f"C{i:02}"]["status"] == "planned" for i in range(2, 8))
    assert sessions["S23"]["depends"] == ["C06", "S10", "S20"]
    assert sessions["C06"]["depends"] == ["C05"]
    assert sessions["C05"]["depends"] == ["C03", "C04", "S10"]
    assert "S23" in sessions["S24"]["depends"] and sessions["S25"]["depends"] == ["S24"]
    amendment_id = "S10-G2-CONTENT-SEPARATION"
    amendment = {
        "id": amendment_id,
        "date": date,
        "question": approval["question"],
        "answer": approval["answer"],
        "received_utc": approval["received_utc"],
        "status": "approved" if qualified else "DRAFT_PENDING_USER_ANSWER_AND_QUALIFICATION",
        "scope": "S10/G2 gameplay closure is independent of completing the C01 world census.",
        "previous_s10_dependencies": copy.deepcopy(s10["depends"]),
        "previous_s10_remaining": copy.deepcopy(s10["remaining"]),
        "retained_content_obligations": CONTENT_OBLIGATIONS,
        "unchanged_release_requirements": ["C06", "S23", "S24", "S25", "G5", "CP1", "WC1"],
        "evidence": FINAL,
    }
    assert not any(x.get("id") == amendment_id for x in plan["approval"]["scope_changes"])
    plan["approval"]["scope_changes"].append(amendment)
    s10["depends"] = ["S05"]
    s10["output"] = OUTPUT
    s10["status"] = "complete"
    s10["completed_date"] = date
    s10["evidence"] = "../campaign-certification/S10/final/README.md"
    s10["scope_amendment"] = amendment_id
    s10["remaining"] = []
    s10["content_obligations"] = CONTENT_OBLIGATIONS
    s10["closure"] = {"evidence": FINAL, "candidate_revision": CANDIDATE}
    plan["date"] = date
    plan["last_completed_session"] = "S10"
    plan["next_session"] = "S11"
    plan["execution"].update({
        "authorized_through": "S10", "stop_boundary": "S10", "status": "stopped",
        "previous_stop": "S10.h", "instruction": "Continue. Finish S10",
        "instruction_date": date, "next_session_requires_instruction": True,
    })
    plan["gate_decisions"]["G2"] = {
        "status": "earned", "date": date, "candidate": CANDIDATE,
        "evidence": FINAL, "reason": G2_REASON, "scope_amendment": amendment_id,
    }
    nation = next(p for p in plan["phases"] if p["id"] == "nation")
    assert nation["gate"] == "G2 · Connected national economy"
    nation["gate"] = "G2 · Governable country ✓"
    # No acceptance checks, earlier increments, downstream sessions, content
    # status, release scope, country list or prior gate decision may be changed.
    assert s10["accept"] == next(s for s in before["sessions"] if s["id"] == "S10")["accept"]
    assert s10["increments"] == next(s for s in before["sessions"] if s["id"] == "S10")["increments"]
    assert [s for s in plan["sessions"] if s["id"] != "S10"] == [s for s in before["sessions"] if s["id"] != "S10"]
    assert plan["countries"] == before["countries"] and plan["target"] == before["target"]
    assert plan["gate_decisions"]["G1"] == before["gate_decisions"]["G1"]
    return json.dumps(plan, ensure_ascii=False, indent=2) + "\n"


def draft_pathway(text, approval, date):
    text = once(text,
        "**Approved pathway · 10 September 2026 · S01–S09 complete; S10 in progress.**",
        "**Approved pathway · 10 September 2026 · S01–S10 complete; S11–S30 planned.**\n\n"
        "S10 closes its qualified gameplay scope under the explicit amendment below.\n"
        "C01 and worldwide character work remain incomplete; CP1 is not earned.")
    text = once(text,
        "Thirty core development, qualification and release work sessions. S01–S09 are complete; S10 is in progress; S11–S30 remain planned.",
        "Thirty core development, qualification and release work sessions. S01–S10 are complete; S11–S30 remain planned.")
    text = once(text,
        "| [S10](#s10) · Qualify government, succession and diplomacy | G2: governable nations with trustworthy political decisions, leaders and diplomatic consequences. | S05, C01 |",
        f"| [S10](#s10) · Qualify government, succession and diplomacy | {OUTPUT} | S05 |")
    text = once(text,
        "| G2 | G1 + S06–S10 | Complete national finance/construction/supplier/research/government paths. |",
        "| G2 | G1 + S06–S10 | Qualified national finance/construction/supplier/research/government paths; historical content remains required by C06/S23 before CP1. |")
    amendment = (
        f"## S10 scope amendment · {date}\n\n"
        f"The user selected: “{approval['answer']}”. The recorded question, answer and\n"
        "receipt are retained in the [S10 final manifest](campaign-certification/S10/final/manifest.json).\n"
        "This removes C01 as a prerequisite for closing S10's government, succession\n"
        "and diplomacy gameplay. The S10 acceptance checks remain unchanged.\n\n"
        "C01–C07 retain the organization census, historical leadership, fictional\n"
        "successor and cartoon work. C06 and S23 still require the eight certified\n"
        "country casts and their sourced historical/future coverage before later\n"
        "campaign qualification. S24, S25, G5, CP1 and WC1 requirements are unchanged.\n"
        "No missing identity, historical interval or portrait is waived or counted\n"
        "as complete. G2 is a gameplay gate and does not certify that content.\n\n"
    )
    text = once(text, "## Detailed session cards\n", amendment + "## Detailed session cards\n")
    text = once(text,
        "**Status:** In progress · **Requires:** S05, C01\n\n**Completion marker:** G2: governable nations with trustworthy political decisions, leaders and diplomatic consequences.\n",
        f"**Status:** Complete · **Requires:** S05\n\n**Completion marker:** {OUTPUT}\n\n"
        "The [final gameplay qualification](campaign-certification/S10/final/README.md)\n"
        "records the exact candidate, scope amendment and checks supporting G2.\n"
        "C01 remains open and feeds the required C06/S23 content gates. Execution\n"
        "stops after S10; S11 is planned.\n\n"
        "##### Historical increment records\n\n"
        "The following S10.a–h notes retain the scope and open/closed decisions\n"
        "at each original checkpoint. They do not override the final status above.\n")
    start = text.index("#### S10 — Qualify government, succession and diplomacy")
    stop = text.index("### 3 · A military you can use", start)
    section = text[start:stop]
    checks = [
        "Test parliamentary, presidential, authoritarian and monarchical/institutional cases through review, confirmation and dated result.",
        "Preserve saved incumbents; historical browsing and future candidate eligibility do not replace an officeholder automatically.",
        "Validate offers, deadlines, standing policies, sanctions, party/executive roles, unaffordable/stale reviews and foreign inspection.",
    ]
    for check in checks:
        section = once(section, "- [ ] " + check, "- [x] " + check)
    section = once(section, "- [x] " + checks[0],
        "##### Final gameplay acceptance\n\n- [x] " + checks[0])
    text = text[:start] + section + text[stop:]
    text = once(text,
        "Unrepresented real organizations remain unknown; this partial inventory does\nnot close C01 or the S10/G2 dependency.",
        "Unrepresented real organizations remain unknown; this partial inventory does\n"
        "not close C01. Under the explicit S10 scope amendment, gameplay closure is\n"
        "separate from the still-required C06/S23 historical-content gates.")
    old = (
        "The subsequent Continue authorized S10. S10.a through S10.h are complete as bounded increments; "
        "S10 and C01 remain incomplete. Execution stopped after S10.h; S11 and later sessions require a new instruction. "
        "No later campaign, content or release certificate is awarded."
    )
    new = (
        "The subsequent Continue authorized S10. S10.a through S10.h completed as bounded increments while "
        "S10 and C01 were still open. The later instruction “Continue. Finish S10” and the recorded scope "
        "amendment authorized closure of S10's qualified gameplay scope. S10 is complete and G2 is earned "
        "on the final report's exact evidence. C01 remains incomplete, and C06/S23 content requirements "
        "still block CP1 certification. Execution stopped after S10; S11 and later sessions require a new "
        "instruction. No later campaign, content or release certificate is awarded."
    )
    return once(text, old, new)


def draft_c01(text, date):
    text = once(text, "# C01 — worldwide party and institution census\n",
        "# C01 — worldwide party and institution census\n\n"
        f"**Current scope · {date}: C01 remains incomplete.** The explicitly approved\n"
        "[S10 gameplay scope amendment](../S10/final/README.md) separates S10/G2\n"
        "gameplay closure from this census. Character work continues through C01–C07;\n"
        "C06 and S23 still require the certified country casts and historical/future\n"
        "coverage before CP1 qualification. No census, identity, term or portrait\n"
        "requirement is waived. Earlier increment descriptions below are historical.\n")
    text = once(text,
        "**Partial inventory recorded; C01 remains incomplete.** This is a reproducible\nprerequisite audit for S10, originally recorded at\n",
        "**Partial inventory recorded; C01 remains incomplete.** This reproducible\n"
        "audit was originally an S10 prerequisite under the earlier scope, recorded at\n")
    text = once(text,
        "nor sufficient to establish worldwide coverage. S10/G2 retain their unchanged\nC01 prerequisite while this census is open.",
        "nor sufficient to establish worldwide coverage. The original S10/G2 C01\n"
        "prerequisite is superseded by the explicit gameplay scope amendment above.\n"
        "C01 remains required for the character workstream; C06 and S23 retain the\n"
        "certified-country coverage requirements before CP1 qualification.")
    return text


def draft_s10(text, date):
    original = (
        "Status: **S10.a through S10.h complete; S10 and C01 remain in progress**.\n"
        "The [S10.b report](b/README.md) records source-backed country discoveries,\n"
        "Linux qualification and all eight ordinary government browser journeys.\n"
        "Parent source: `872442d7411d9986321248b41ccadf79e2851d1c`.\n"
        "Latest user instruction: Continue. Work is limited to S10; S11 is not started.\n"
    )
    revised = (
        f"Status: **S10 gameplay complete; G2 earned · {date}**. C01 remains in\n"
        "progress and S11 is planned. See the [final qualification](final/README.md)\n"
        "for the exact candidate, checks and explicitly approved scope amendment.\n\n"
        "S10 closes the government, succession and diplomacy gameplay acceptance\n"
        "checks. C01–C07 retain worldwide historical and cartoon work. C06 and S23\n"
        "still require the certified-country content before CP1 qualification.\n"
        "This does not complete a country census, leadership history or portrait\n"
        "backlog. Latest instruction: “Continue. Finish S10”. Execution stops after\n"
        "S10; S11 is not started.\n\n"
        "## Historical increment records\n\n"
        "The following notes preserve each increment's original source, evidence\n"
        "and open/closed decisions. Their then-current S10/G2 status is historical;\n"
        "the final report above records the later scope amendment and closure.\n"
        "The [S10.b report](b/README.md) records source-backed country discoveries,\n"
        "Linux qualification and all eight ordinary government browser journeys.\n"
        "Original parent source: `872442d7411d9986321248b41ccadf79e2851d1c`.\n"
    )
    return once(text, original, revised)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=pathlib.Path, default=REPO / FINAL)
    parser.add_argument("--approval-choice")
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    manifest_path = args.manifest.resolve()
    qualified = manifest_path.exists() and args.approval_choice is not None
    if qualified:
        approval = check_qualified(read_json(manifest_path), manifest_path, args.approval_choice)
        date = approval["received_utc"][:10]
    else:
        assert not args.apply, "Cannot apply: final qualification and actual approval receipt are required"
        assert args.approval_choice is None, "No approval may be inferred without the final receipt"
        approval = {
            "question": "[DRAFT: actual question pending recorded receipt]",
            "answer": CHOICE,
            "received_utc": "[DRAFT: pending actual user answer]",
        }
        date = "[approval date pending]"
    assert git("rev-parse", "HEAD").decode().strip() == CANDIDATE, "Candidate changed; review helper before using"
    original, texts = {}, {}
    for name in OWNED:
        raw = (REPO / name).read_bytes()
        pinned = git("show", f"{CANDIDATE}:{name}")
        assert raw.replace(b"\r\n", b"\n") == pinned.replace(b"\r\n", b"\n"), f"Concurrent change: {name}"
        original[name] = raw
        texts[name] = raw.decode("utf-8").replace("\r\n", "\n")
    updated = {
        OWNED[0]: draft_json(texts[OWNED[0]], approval, date, qualified),
        OWNED[1]: draft_pathway(texts[OWNED[1]], approval, date),
        OWNED[2]: draft_c01(texts[OWNED[2]], date),
        OWNED[3]: draft_s10(texts[OWNED[3]], date),
    }
    print("QUALIFIED APPROVED DIFF" if qualified else "PROPOSAL ONLY — NO APPROVAL OR COMPLETION CLAIM")
    for name in OWNED:
        print("".join(difflib.unified_diff(texts[name].splitlines(True), updated[name].splitlines(True),
            fromfile=name, tofile=name + " (proposed)")), end="")
    # Check every original again before the first write; no partial concurrent
    # overwrite is permitted. The only mutations are these four document files.
    if args.apply:
        assert qualified
        for name in OWNED:
            assert (REPO / name).read_bytes() == original[name], f"Concurrent change: {name}"
        for name in OWNED:
            (REPO / name).write_text(updated[name], encoding="utf-8", newline="\n")
    print(json.dumps({"applied": args.apply, "qualified_manifest": qualified,
        "candidate_revision": CANDIDATE, "files": list(OWNED),
        "c01_complete": False, "s11_started": False,
        "historical_increment_files_modified": False}, indent=2))


if __name__ == "__main__":
    main()
