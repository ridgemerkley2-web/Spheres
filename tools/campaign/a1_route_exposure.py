"""Audit route exposure in the retained A1 seed-0 trace without running the game.

Trigger and later AI-funding observations are deliberately kept separate: a
government can change between those two sites in the same monthly tick.
"""
import argparse
from collections import Counter
import datetime as dt
import gzip
import hashlib
import json
from pathlib import Path
import re
import subprocess

OBSERVER_REVISION = "6818e4f0d94b01c86d7a9acc4252260947d13504"
RAW_SHA256 = "13676236845d5c814e640520d322227769d4a7e16c685a237a6ce99a2ca46b49"
RESULT_SHA256 = "28b9e9177d800e78566c2bec8a878245515932e342c5304ea2f1bcaf6bfca6f6"
ANCHORS = ("Thailand", "Haiti", "Algeria", "Peru", "SierraLeone", "Gambia", "Niger", "Pakistan", "Honduras")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}


def verify_firings(firings, result):
    headlines = result["elected_coup_headlines"]
    require(type(headlines) is list and firings == result["firing_cases"] and len(firings) == len(headlines),
            "Actual firing evidence mismatch")
    for firing, headline in zip(firings, headlines):
        year, month, _ = firing["date"]
        require(headline["month"] == (year - 1990) * 12 + month and
                headline["headline"].endswith("removes the elected government."), "Firing chronology/headline mismatch")


def roster_codes(source):
    # Match roster constructors, not comments, aliases, or neighbouring names.
    return set(re.findall(r'^\s*row\("([^"\n]+)",', source, flags=re.MULTILINE))


def trigger_blockers(row):
    g, a, limits = row["government"], row["army"], row["thresholds"]
    require(g["electoral"] is True, "Trigger observation is not electoral")
    require(limits == {"army": 0.35, "discontent": 0.25, "settled_months": 12, "pressure": 1.0},
            "Unexpected observed trigger limits")
    return [name for name, blocked in (
        ("no_army", not g["has_army"]),
        ("unsettled", g["settled_months"] < limits["settled_months"]),
        ("awaiting_first_election", g["awaiting_first_election"]),
        ("army_not_hostile", a["effective_loyalty"] >= limits["army"]),
        ("no_current_crisis", row["discontent"]["total"] < limits["discontent"]),
        ("pressure_not_ready", a["pressure"] < limits["pressure"]),
    ) if blocked]


def fresh_country(country):
    return {"country": country, "funding_stage_months": 0, "electoral_at_funding": 0,
            "non_electoral_at_funding": 0, "trigger_checks": 0, "eligible_trigger_checks": 0,
            "eligible_crisis_checks": 0, "eligible_hostile_crisis_checks": 0, "firings": 0,
            "trigger_branches": Counter(), "trigger_blockers_nonexclusive": Counter(),
            "funding_stage_regime_intervals": [], "same_month_exit_witnesses": []}


class Exposure:
    def __init__(self):
        self.countries = {}
        self.stages = Counter()
        self.seen = set()
        self.triggers = {}
        self.firings = []
        self.rows = 0

    def add(self, row, line):
        country, stage, date = row["country"], row["stage"], tuple(row["date"])
        dt.date(*date)
        key = (country, date, stage)
        require(key not in self.seen, "Duplicate country/date/stage observation")
        self.seen.add(key)
        self.rows += 1
        self.stages[stage] += 1
        record = self.countries.setdefault(country, fresh_country(country))
        if stage.startswith("trigger_"):
            require((country, date) not in self.triggers, "Multiple trigger outcomes in one month")
            blockers = trigger_blockers(row)
            expected_branch = ("trigger_unsettled_interim_or_no_army" if
                any(x in blockers for x in ("no_army", "unsettled", "awaiting_first_election")) else
                "trigger_live_conditions_inactive" if any(x in blockers for x in ("army_not_hostile", "no_current_crisis")) else
                "trigger_pressure_not_ready" if "pressure_not_ready" in blockers else "trigger_firing")
            require(stage == expected_branch, "Recorded branch disagrees with contemporaneous guards")
            self.triggers[country, date] = {"line": line, "stage": stage}
            record["trigger_checks"] += 1
            record["trigger_branches"][stage] += 1
            record["trigger_blockers_nonexclusive"].update(blockers)
            eligible = not any(x in blockers for x in ("no_army", "unsettled", "awaiting_first_election"))
            crisis = "no_current_crisis" not in blockers
            record["eligible_trigger_checks"] += eligible
            record["eligible_crisis_checks"] += eligible and crisis
            record["eligible_hostile_crisis_checks"] += eligible and crisis and "army_not_hostile" not in blockers
            if stage == "trigger_firing":
                record["firings"] += 1
                self.firings.append(row)
        if stage != "ai_before_funding":
            return
        electoral = row["government"]["electoral"]
        require(type(electoral) is bool, "Electoral state is not a boolean")
        record["funding_stage_months"] += 1
        record["electoral_at_funding" if electoral else "non_electoral_at_funding"] += 1
        intervals = record["funding_stage_regime_intervals"]
        if intervals:
            last = intervals[-1]["last_date"]
            require(tuple(last) < date, "Funding observations are not chronological")
            consecutive = date[2] == last[2] == 1 and date[0] * 12 + date[1] == last[0] * 12 + last[1] + 1
        else:
            consecutive = False
        if intervals and consecutive and intervals[-1]["electoral"] == electoral:
            intervals[-1]["last_date"] = list(date)
            intervals[-1]["observations"] += 1
        else:
            intervals.append({"electoral": electoral, "first_date": list(date), "last_date": list(date), "observations": 1})
        trigger = self.triggers.get((country, date))
        if trigger is not None and not electoral:
            record["same_month_exit_witnesses"].append({"date": list(date), "trigger": trigger,
                "later_funding_line": line,
                "interpretation": "Electoral at the trigger, non-electoral at later funding; this pair alone does not identify every possible intervening event."})

    def summary(self, roster):
        rows = []
        for country in sorted(set(self.countries) | set(ANCHORS)):
            record = dict(self.countries.get(country, fresh_country(country)))
            record["in_reviewed_roster"] = country in roster
            for key in ("trigger_branches", "trigger_blockers_nonexclusive"):
                record[key] = dict(sorted(record[key].items()))
            record["classification"] = (
                "outside_roster" if country not in roster else
                "no_observations" if country not in self.countries else
                "fired" if record["firings"] else
                "no_electoral_trigger_observed" if not record["trigger_checks"] else
                "no_eligible_trigger_observed" if not record["eligible_trigger_checks"] else
                "no_eligible_crisis_observed" if not record["eligible_crisis_checks"] else
                "no_eligible_hostile_crisis_observed" if not record["eligible_hostile_crisis_checks"] else
                "live_conditions_observed_but_no_firing")
            rows.append(record)
        return rows


def analyze(packet, repo):
    packet, repo = Path(packet), Path(repo)
    result_bytes = (packet / "original-run/result.json").read_bytes()
    require(digest(result_bytes)["sha256"] == RESULT_SHA256, "Original result changed")
    result = json.loads(result_bytes)
    require(result["seed"] == 0 and result["months_requested"] == result["months_compared"] == 252
            and result["passed"] is True and result["a1_pass_claimed"] is False, "Wrong diagnostic scope")
    require(len(result["comparisons"]) == 252 and all(x["equal"] is True for x in result["comparisons"]),
            "Observer equivalence receipt is incomplete")
    manifest_bytes = (packet / "manifest.json").read_bytes()
    manifest = json.loads(manifest_bytes)
    require(manifest["candidate_revision"] == OBSERVER_REVISION, "Wrong observer revision")
    name = "original-run/observations.jsonl.gz"
    entry, = [x for x in manifest["files"] if x["path"] == name]
    packed = packet / name
    require(digest(packed.read_bytes()) == {k: entry[k] for k in ("bytes", "sha256")}, "Compressed trace changed")
    exposure = Exposure()
    raw_hash, raw_size = hashlib.sha256(), 0
    with gzip.open(packed, "rb") as stream:
        for line, raw in enumerate(stream, 1):
            raw_hash.update(raw)
            raw_size += len(raw)
            exposure.add(json.loads(raw), line)
    require((raw_size, raw_hash.hexdigest()) == (240585752, RAW_SHA256), "Decoded trace changed")
    require(exposure.rows == 146525 and dict(exposure.stages) == result["stage_counts"], "Stage coverage mismatch")
    verify_firings(exposure.firings, result)
    source_checks = []
    for name in ("spheres-sim/src/nations.rs", "spheres-sim/src/government.rs", "spheres-sim/tests/bloc_census.rs"):
        observed = subprocess.check_output(["git", "show", f"{OBSERVER_REVISION}:{name}"], cwd=repo)
        current = subprocess.check_output(["git", "show", f"HEAD:{name}"], cwd=repo)
        require(current == observed, f"Political source changed; assess applicability before using this audit: {name}")
        source_checks.append({"path": name, **digest(current), "matches_observer_git_bytes": True})
        if name.endswith("nations.rs"):
            roster = roster_codes(current.decode("utf-8"))
    rows = exposure.summary(roster)
    return {"format": "spheres-a1-route-exposure/v1", "observed_revision": OBSERVER_REVISION,
            "reviewed_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip(),
            "seed": 0, "months": 252, "rows": exposure.rows, "stage_counts": dict(exposure.stages),
            "inputs": {"manifest": digest(manifest_bytes), "native_result": digest(result_bytes),
                       "decoded_observations": {"bytes": raw_size, "sha256": raw_hash.hexdigest()}, "source_checks": source_checks},
            "anchor_countries": [next(row for row in rows if row["country"] == c) for c in ANCHORS],
            "countries": rows, "native_reexecuted": False, "reserved_holdouts_used": False,
            "a1_pass_claimed": False, "qualification": False,
            "limits": "One existing development seed. Observed country-month exposures are not independent samples or counterfactual effects. Funding follows trigger/election processing; its state must not replace trigger-time state. Anchor names come from the existing A1 comment, not a new historical event classification."}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--packet", type=Path, required=True)
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    require(not args.out.exists(), "Refusing to overwrite an existing report")
    for source in (args.packet.resolve(), args.repo.resolve() / "spheres-sim"):
        require(not args.out.resolve().is_relative_to(source), "Report cannot be written inside source evidence or simulation inputs")
    report = analyze(args.packet, args.repo)
    with args.out.open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(report, stream, indent=2, ensure_ascii=False, allow_nan=False)
        stream.write("\n")
    print(json.dumps({"rows_verified": report["rows"], "firings": sum(x["firings"] for x in report["countries"]),
                      "anchor_classifications": {x["country"]: x["classification"] for x in report["anchor_countries"]},
                      "a1_pass_claimed": False}, indent=2))


if __name__ == "__main__":
    main()
