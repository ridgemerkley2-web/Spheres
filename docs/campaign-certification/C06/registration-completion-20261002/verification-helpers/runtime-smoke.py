"""Opt-in, GET-only smoke of the 31 registered portraits plus France's seven.

No build is performed. Launch requires --run. A new campaign cwd and an ephemeral
loopback listener belong exclusively to this invocation; no existing server is
contacted. This is HTTP verification, not browser, gameplay or country acceptance.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import socket
import struct
import subprocess
import sys
import tempfile
import time
import traceback
import urllib.error
import urllib.request


BASE = Path(__file__).resolve().parent
DEFAULT_REPO = BASE.parent / "claude-c06-review-20261002"
REGISTRATION = "docs/campaign-certification/C06/registration-completion-20261002/registration.json"
FRANCE = "tools/avatars/person-prompts/france-cast-batch-01.json"
MANIFEST = "spheres-web/data/person_portraits.json"
ROSTER = "spheres-sim/data/party_leaders.json"
CHECKS = "docs/campaign-certification/C06/registration-completion-20261002/checks"
ASSET_PREFIX = "spheres-web/ui/person-portraits/"

# These are reference queries within accepted party-holder intervals. The dates
# select artwork; this helper makes no new claim about terms or country coverage.
REFERENCE_CASES = [
    ("Brazil", "2009-11-11", "br_pdt", "carlos_lupi", "carlos-lupi-cartoon-2004-v1.png"),
    ("Brazil", "2014-12-31", "br_pdt", "carlos_lupi", "carlos-lupi-cartoon-2004-v1.png"),
    ("Brazil", "2015-01-01", "br_pdt", "carlos_lupi", "carlos-lupi-cartoon-2015-v2.png"),
    ("Brazil", "2019-12-31", "br_pdt", "carlos_lupi", "carlos-lupi-cartoon-2015-v2.png"),
    ("Brazil", "2020-01-01", "br_pdt", "carlos_lupi", "carlos-lupi-cartoon-2020-v1.png"),
    ("Brazil", "2026-09-07", "br_pdt", "carlos_lupi", "carlos-lupi-cartoon-2020-v1.png"),
    ("Japan", "2014-12-31", "jp_komeito", "natsuo_yamaguchi", "natsuo-yamaguchi-cartoon-2009-v2.png"),
    ("Japan", "2015-01-01", "jp_komeito", "natsuo_yamaguchi", "natsuo-yamaguchi-cartoon-2015-v1.png"),
    ("Japan", "2024-09-27", "jp_komeito", "natsuo_yamaguchi", "natsuo-yamaguchi-cartoon-2015-v1.png"),
    ("France", "2007-05-29", "fr_ps", "francois_hollande", "francois-hollande-cartoon-1997-v1.png"),
    ("India", "2021-01-01", "in_bjp", "jagat_prakash_nadda", "jagat-prakash-nadda-cartoon-2020-v1.png"),
    ("SouthAfrica", "2013-09-16", "za_ff", "pieter_mulder", "pieter-mulder-cartoon-2001-v1.png"),
    ("SouthAfrica", "2026-03-25", "za_ff", "corne_mulder", "corne-mulder-cartoon-2025-v1.png"),
    # The party API does not manufacture a monarchy or a missing party holder.
    # These two queries check country/date/read-only context, not named coverage.
    ("SaudiArabia", "1996-01-01", None, None, None),
    ("USSR", "1991-01-01", None, None, None),
]


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def utc():
    return dt.datetime.now(dt.timezone.utc).isoformat()


def git(repo, *args):
    result = subprocess.run(["git", "-C", str(repo), *args], check=True,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
    return result.stdout


def pin(repo, revision, relative):
    path = repo / relative
    data = path.read_bytes()
    committed = git(repo, "show", f"{revision}:{relative}")
    require(data == committed, f"Worktree bytes differ from expected source: {relative}")
    return {"path": relative, "sha256": sha(data), "bytes": len(data)}, data


def png_size(data):
    require(data[:8] == b"\x89PNG\r\n\x1a\n" and data[12:16] == b"IHDR", "Not a PNG with IHDR")
    return struct.unpack(">II", data[16:24])


def load_inputs(repo, revision):
    pins, docs = [], {}
    for relative in (REGISTRATION, FRANCE, MANIFEST, ROSTER,
                     "spheres-web/src/main.rs", "spheres-web/src/person_portraits.rs",
                     "spheres-web/src/person_avatar_assets.rs"):
        record, data = pin(repo, revision, relative)
        pins.append(record)
        if relative.endswith(".json"):
            docs[relative] = json.loads(data)
    registration, france, manifest, roster = (docs[p] for p in (REGISTRATION, FRANCE, MANIFEST, ROSTER))
    require(registration["portraits_added"] == 31 and len(registration["added_portraits"]) == 31,
            "Registration no longer describes exactly 31 additions")
    require(len(france["generations"]) == 7, "France batch no longer contains seven portraits")
    require(sha((repo / MANIFEST).read_bytes()) == registration["manifest_after_sha256"],
            "Registration's final manifest pin is stale")
    require((roster["reference_from"], roster["reference_through"]) == ("1990-01-01", "2026-09-07"),
            "Reference cutoff contract changed")
    jobs = list(registration["added_portraits"]) + [dict(g["job"], country="france") for g in france["generations"]]
    require(len({job["asset"] for job in jobs}) == 38, "Expected 38 distinct selected PNGs")
    selected = []
    for job in jobs:
        matches = [p for p in manifest["people"][job["person_id"]]["portraits"]
                   if all(p[k] == job[k] for k in ("asset", "from", "to", "sha256"))]
        require(len(matches) == 1, f"No exact registered portrait for {job['asset']}")
        portrait = matches[0]
        require(all(portrait["review"].get(k) is True for k in ("identity", "likeness", "era", "visual")),
                f"Unreviewed selected portrait: {job['asset']}")
        basename = job["asset"].removeprefix(ASSET_PREFIX)
        require(job["asset"].startswith(ASSET_PREFIX) and PurePosixPath(basename).name == basename,
                "Asset is not a direct portrait basename")
        record, data = pin(repo, revision, job["asset"])
        require(record["sha256"] == job["sha256"] == portrait["sha256"], f"PNG SHA mismatch: {basename}")
        require(len(data) == portrait["bytes"], f"PNG byte count mismatch: {basename}")
        require(png_size(data) == (portrait["width"], portrait["height"]) == (1024, 1536),
                f"PNG dimensions mismatch: {basename}")
        selected.append(dict(record, country=job["country"], person_id=job["person_id"],
                             **{"from": job["from"], "to": job["to"]},
                             width=portrait["width"], height=portrait["height"],
                             url="/art/people/" + basename))
    return pins, selected, manifest


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        raise AssertionError(f"Unexpected redirect from owned localhost server: {newurl}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run", action="store_true", help="Explicitly authorize this invocation to launch its own server")
    parser.add_argument("--repo", type=Path, default=DEFAULT_REPO)
    parser.add_argument("--binary", type=Path, default=BASE / "native-target/release/spheres-web.exe")
    parser.add_argument("--expected-revision", default="HEAD", help="Exact commit or unique revision; resolved once before launch")
    parser.add_argument("--out", type=Path, help="Receipt parent, default registration-completion-20261002/checks")
    parser.add_argument("--startup-timeout", type=float, default=60)
    args = parser.parse_args()
    if not args.run:
        parser.error("Prepared only. Pass --run after the release build and launch authorization are ready.")
    repo, binary = args.repo.resolve(), args.binary.resolve()
    out = (args.out or repo / CHECKS).resolve()
    out.mkdir(parents=True, exist_ok=True)
    evidence = Path(tempfile.mkdtemp(prefix="runtime-smoke-", dir=out))
    disposable = Path(tempfile.mkdtemp(prefix="http-smoke-", dir=BASE))
    campaign = disposable / "campaign"
    campaign.mkdir()
    receipt_path = evidence / "receipt.json"
    receipt = {"version": 1, "started_at": utc(), "passed": False,
               "scope": "Actual-game HTTP PNG bytes and selected historical party-reference dates only",
               "human_approval": False, "country_completion": False,
               "tonga_browser_acceptance": False, "gameplay_or_save_mutations_requested": False,
               "repo": str(repo), "binary": str(binary), "helper_sha256": sha(Path(__file__).read_bytes()),
               "evidence_directory": str(evidence), "disposable_campaign_directory": str(campaign),
               "requests": [], "portraits": [], "reference_cases": [], "negative_cases": [],
               "limitations": ["All 38 PNG URLs are checked. The party endpoint is not an arbitrary person/date API.",
                               "Saudi monarchs and USSR's unpopulated party holders are not invented as party leaders.",
                               "No POST, campaign advance, save/load, user server, or browser is used.",
                               "Appearance intervals do not establish office tenures or complete historical coverage."]}
    process = None
    log = None
    try:
        revision = git(repo, "rev-parse", f"{args.expected_revision}^{{commit}}").decode().strip()
        require(git(repo, "rev-parse", "HEAD").decode().strip() == revision, "Expected source is not checkout HEAD")
        receipt["source_revision"] = revision
        receipt["source_pins"], selected, manifest = load_inputs(repo, revision)
        receipt["expected_portrait_count"] = len(selected)
        receipt["binary_sha256"] = sha(binary.read_bytes())
        receipt["binary_bytes"] = binary.stat().st_size
        require(not list(campaign.iterdir()), "Disposable campaign cwd is not empty")
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as probe:
            probe.bind(("127.0.0.1", 0))
            port = probe.getsockname()[1]
        require(port != 8796, "Refusing the user's active server port")
        base_url = f"http://127.0.0.1:{port}"
        receipt["base_url"] = base_url
        command = [str(binary), "--port", str(port), "--no-open"]
        receipt["command"] = command
        log = (evidence / "server.log").open("wb")
        process = subprocess.Popen(command, cwd=campaign, stdin=subprocess.DEVNULL,
                                   stdout=log, stderr=subprocess.STDOUT,
                                   creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
        receipt["owned_pid"] = process.pid
        opener = urllib.request.build_opener(urllib.request.ProxyHandler({}), NoRedirect())

        def get(route, expected=200, save_as=None):
            require(route.startswith(("/api/build", "/api/state", "/api/party-leadership?", "/art/people/")),
                    f"Unexpected smoke route: {route}")
            require(process.poll() is None, "Owned server exited before request")
            request = urllib.request.Request(base_url + route, method="GET")
            try:
                response = opener.open(request, timeout=10)
            except urllib.error.HTTPError as error:
                response = error
            with response:
                data = response.read()
                item = {"method": "GET", "route": route, "status": response.code,
                        "mime": response.headers.get_content_type(), "bytes": len(data), "sha256": sha(data)}
            receipt["requests"].append(item)
            require(item["status"] == expected, f"{route}: HTTP {item['status']}, expected {expected}")
            if save_as:
                (evidence / save_as).write_bytes(data)
                item["body_file"] = save_as
            return item, data

        deadline = time.monotonic() + args.startup_timeout
        while True:
            require(process.poll() is None, "Owned server exited during startup; see server.log")
            try:
                _, data = get("/api/build", save_as="build.json")
                break
            except (urllib.error.URLError, TimeoutError, ConnectionError) as error:
                require(time.monotonic() < deadline, f"Startup timeout: {error}")
                time.sleep(0.2)
        build = json.loads(data)
        receipt["build"] = build
        require(build["full_revision"] == revision, f"Served build source differs: {build['full_revision']}")
        require(build["revision"] == revision[:12], "Served short revision differs or build is modified")
        require(Path(build["save_directory"]).resolve() == campaign.resolve(), "Server is using a different save directory")
        require(isinstance(build["built_at_unix_seconds"], int) and build["built_at_unix_seconds"] > 0,
                "Missing build timestamp")
        _, before = get("/api/state", save_as="state-before.json")
        for expected in selected:
            actual, data = get(expected["url"])
            item = dict(expected, http=actual)
            receipt["portraits"].append(item)
            require(actual["mime"] == "image/png", f"Wrong PNG MIME: {expected['url']}")
            require(actual["sha256"] == expected["sha256"] and actual["bytes"] == expected["bytes"],
                    f"Served bytes differ from committed PNG: {expected['url']}")
            require(png_size(data) == (expected["width"], expected["height"]), "Served PNG dimensions differ")
            item["passed"] = True

        for index, (nation, date, party_id, person_id, filename) in enumerate(REFERENCE_CASES):
            route = f"/api/party-leadership?nation={nation}&date={date}"
            http, data = get(route, save_as=f"reference-{index:02d}-{nation}-{date}.json")
            value = json.loads(data)
            item = {"nation": nation, "date": date, "party_id": party_id, "person_id": person_id,
                    "expected_url": "/art/people/" + filename if filename else None, "http": http}
            receipt["reference_cases"].append(item)
            require(value["nation"] == nation and value["date"] == date, "Reference country/date differs")
            require(value["campaign_date"] == "1990-01-01", "Reference lookup changed campaign date")
            require(value["enabled"] is False and value["eligibility_context"] == "historical_reference"
                    and value["executive_person"] is None, "Reference query acquired campaign authority")
            require((value["reference_from"], value["reference_through"]) == ("1990-01-01", "2026-09-07"),
                    "Served historical reference bounds differ")
            for party in value["parties"]:
                require(party["status"] == "reference_only" and
                        all(party[k] == [] for k in ("campaign", "future_candidates", "future_preview")),
                        "Reference query returned campaign holders or future candidates")
            if person_id:
                parties = [p for p in value["parties"] if p["party_id"] == party_id]
                require(len(parties) == 1, f"Expected party missing: {party_id}")
                people = [h["person"] for h in parties[0]["historical"] if h["person"]["id"] == person_id]
                require(bool(people), f"Expected historical party holder missing: {person_id} on {date}")
                expected_portraits = [p for p in manifest["people"][person_id]["portraits"]
                                      if p["from"] <= date and (p["to"] is None or date < p["to"])]
                require(len(expected_portraits) == 1, "Expected selected appearance is ambiguous")
                expected_portrait = expected_portraits[0]
                require(expected_portrait["asset"] == ASSET_PREFIX + filename, "Case no longer matches manifest")
                for person in people:
                    portrait = person["portrait"]
                    require(portrait is not None and portrait["url"] == item["expected_url"],
                            f"Wrong dated portrait for {person_id} on {date}")
                    require((portrait["from"], portrait["to"]) == (expected_portrait["from"], expected_portrait["to"]),
                            "Served appearance interval differs")
                item["observed_urls"] = [p["portrait"]["url"] for p in people]
            item["passed"] = True

        for index, (date, expected_error) in enumerate([
            ("2026-02-30", "Choose a valid calendar date."),
            ("1990-1-01", "Choose a valid calendar date."),
            ("1989-12-31", "Historical reference dates run from 1990-01-01 through 2026-09-07."),
            ("2026-09-08", "Historical reference dates run from 1990-01-01 through 2026-09-07."),
        ]):
            http, data = get(f"/api/party-leadership?nation=Brazil&date={date}", expected=400,
                             save_as=f"rejected-date-{index:02d}.json")
            require(json.loads(data).get("error") == expected_error, "Unexpected invalid-date response")
            receipt["negative_cases"].append({"date": date, "http": http, "error": expected_error, "passed": True})
        _, after = get("/api/state", save_as="state-after.json")
        require(json.loads(before) == json.loads(after), "Campaign state changed during GET-only smoke")
        require(not list(campaign.iterdir()), "Server wrote files in disposable campaign cwd")
        require(process.poll() is None, "Owned server unexpectedly exited before checks finished")
        receipt["state_unchanged"] = True
        receipt["save_directory_remained_empty"] = True
        receipt["passed"] = True
    except BaseException as error:
        receipt["error"] = str(error)
        receipt["traceback"] = traceback.format_exc()
    finally:
        if process is not None:
            receipt["server_exit_before_cleanup"] = process.poll()
            try:
                if process.poll() is None:
                    process.terminate()
                    try:
                        process.wait(timeout=10)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait(timeout=10)
                receipt["owned_process_stopped"] = process.poll() is not None
                receipt["server_exit_after_cleanup"] = process.returncode
            except BaseException as error:
                receipt["passed"] = False
                receipt["cleanup_error"] = repr(error)
        if log is not None:
            log.close()
        receipt["campaign_files_after_cleanup"] = sorted(str(p.relative_to(campaign)) for p in campaign.rglob("*"))
        if receipt["campaign_files_after_cleanup"]:
            receipt["passed"] = False
            receipt["save_directory_remained_empty"] = False
        receipt["finished_at"] = utc()
        receipt_path.write_text(json.dumps(receipt, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
        print(json.dumps({"passed": receipt["passed"], "receipt": str(receipt_path),
                          "portraits_checked": len(receipt["portraits"]), "error": receipt.get("error")}, ensure_ascii=False))
    return 0 if receipt["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
