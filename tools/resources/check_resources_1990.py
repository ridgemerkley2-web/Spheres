#!/usr/bin/env python3
"""
tools/resources/check_resources_1990.py — verify the sim's 1990 resource table.

Run from the repo root, after `make_resources_1990.py`:

    python tools/resources/check_resources_1990.py [--fast]

Mirrors `check_resources.py`: it does not trust the generator, it re-reads the
committed `spheres-sim/data/resources_1990.json` and holds it against the
inputs on disk and the rules it declares (SPEC-RESOURCE-SYSTEM.md section 2.4).
Five groups —

  1. THE JOIN. Every `national_1990` key is a 1990 start nation of the roster;
     the only keys not seated are the listed `dropped_keys` (Namibia, a roster
     seat that comes alive later); no start nation's figure was dropped.
  2. SHARES. Every located list sums to one within 1e-9, every share is at
     least 1e-3, every located district belongs to that nation in 1990, and no
     Soviet coal district under 1e-3 survived (the pruned count is printed).
  3. THE TRANSCRIPTION GUARDS. The USSR's iron reads 236,000,000 exactly; the
     USA has no located oil and no bauxite row; every price row carries a
     positive figure and a source, and every mined line without one is named
     in `prices_omitted`; presence has 1,471 keys and quality is 0 exactly
     where the bit is 0.
  4. PROVENANCE. `source_sha256` matches the three inputs on disk, and
     `upstream` matches the artifact's own meta.
  5. DETERMINISM. Regenerates twice to temporary files and requires all three
     byte-identical. Skipped with --fast.

Every check prints PASS or FAIL and the script exits non-zero on any failure.
"""

import hashlib
import json
import os
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
if HERE not in sys.path:
    sys.path.insert(0, HERE)

import make_resources_1990 as G                             # noqa: E402

FAILURES = []
CHECKS = [0]


def ok(cond, label, detail=""):
    CHECKS[0] += 1
    if cond:
        print(f"  PASS  {label}")
    else:
        print(f"  FAIL  {label}   {detail}")
        FAILURES.append(label)


def main():
    fast = "--fast" in sys.argv
    with open(G.OUT, encoding="utf-8") as f:
        T = json.load(f)
    art = G.load_json(G.ART)
    dj = G.load_json(G.DISTRICTS_JSON)
    codes = G.roster_codes()
    start = G.start_nations(codes)
    owner = G.owners_1990(dj, start)
    roster_districts = {d["id"] for lst in dj["nations"].values() for d in lst}

    M, N, L = T["meta"], T["national_1990"], T["located"]

    # --- 1. the join ------------------------------------------------------
    print("CHECK 1: the join")
    ok(T["commodities"] == G.COMMODITIES, "twelve commodities, alphabetical")
    ok(T["tracked"] == G.TRACKED, "six tracked lines")
    ok(sorted(N) == G.TRACKED and sorted(L) == G.TRACKED,
       "national_1990 and located are keyed by the tracked lines")
    stray = sorted({n for c in N for n in N[c] if n not in start})
    ok(not stray, "every national_1990 key is a 1990 start nation", f"{stray[:6]}")
    ok(M["dropped_keys"] == ["Namibia"], "dropped_keys is exactly [Namibia]",
       f"{M['dropped_keys']}")
    ok(all(k in codes and k not in start for k in M["dropped_keys"]),
       "every dropped key is a roster seat that is not a 1990 start nation")
    upstream_non_start = sorted({n for c in G.TRACKED for n in art["national"][c]
                                 if n not in start})
    ok(upstream_non_start == M["dropped_keys"],
       "the dropped keys are exactly the artifact's non-start keys",
       f"{upstream_non_start}")
    lost = [(c, n) for c in G.TRACKED for n, r in art["national"][c].items()
            if n in start and float(r["value"]) > 0 and n not in N[c]]
    ok(not lost, "no start nation's positive figure was dropped", f"{lost[:6]}")
    zero_kept = [(c, n) for c in N for n, r in N[c].items() if r["value"] <= 0]
    ok(not zero_kept, "no zero row is carried", f"{zero_kept[:6]}")
    verbatim = [(c, n) for c in N for n, r in N[c].items()
                if float(art["national"][c][n]["value"]) != r["value"]
                or art["national"][c][n]["source"] != r["source"]]
    ok(not verbatim, "every national figure and source is the artifact's, verbatim",
       f"{verbatim[:6]}")
    ok(len(start) == 137, "137 start nations on the roster", f"{len(start)}")

    # --- 2. shares --------------------------------------------------------
    print("\nCHECK 2: shares")
    bad_sum, bad_min, bad_sort, bad_owner, dup, empty, unseated = [], [], [], [], [], [], []
    for c in G.TRACKED:
        for n, lst in L[c].items():
            if n not in N[c]:
                unseated.append((c, n))
            if not lst:
                empty.append((c, n))
                continue
            s = sum(x for _, x in lst)
            if abs(s - 1.0) > 1e-9:
                bad_sum.append((c, n, s))
            if any(x < G.PRUNE for _, x in lst):
                bad_min.append((c, n))
            if lst != sorted(lst, key=lambda t: (-t[1], t[0])):
                bad_sort.append((c, n))
            ids = [d for d, _ in lst]
            if len(set(ids)) != len(ids):
                dup.append((c, n))
            if any(owner.get(d) != n for d in ids):
                bad_owner.append((c, n))
    ok(not unseated, "every located nation has a national figure", f"{unseated[:6]}")
    ok(not empty, "no located list is empty", f"{empty[:6]}")
    ok(not bad_sum, "every located list sums to 1 within 1e-9", f"{bad_sum[:4]}")
    ok(not bad_min, f"every located share is at least {G.PRUNE}", f"{bad_min[:6]}")
    ok(not bad_sort, "every located list is sorted share desc then id", f"{bad_sort[:6]}")
    ok(not dup, "no district is listed twice in one list", f"{dup[:6]}")
    ok(not bad_owner, "every located district belongs to that nation in 1990",
       f"{bad_owner[:6]}")
    ussr_coal = L["coal"].get("USSR", [])
    ok(ussr_coal and all(x >= G.PRUNE for _, x in ussr_coal),
       f"no Soviet coal district under {G.PRUNE} survives "
       f"({len(ussr_coal)} located, {M['counts']['pruned']['coal'].get('USSR', 0)} pruned)")
    ok(M["counts"]["pruned"]["coal"].get("USSR", 0) > 0,
       "the Soviet coalfield tail was actually pruned (count above is non-zero)")
    counts_ok = all(M["counts"]["located_rows"][c] == sum(len(v) for v in L[c].values())
                    and M["counts"]["located_nations"][c] == len(L[c])
                    and M["counts"]["national_rows"][c] == len(N[c]) for c in G.TRACKED)
    ok(counts_ok, "meta.counts describe the file")
    unl_ok = all(sorted(set(N[c]) - set(L[c])) == M["counts"]["unlocated_producers"][c]
                 for c in G.TRACKED)
    ok(unl_ok, "unlocated_producers is exactly national minus located, per line")

    # --- 3. the transcription guards --------------------------------------
    print("\nCHECK 3: the transcription guards")
    ok(N["iron"].get("USSR", {}).get("value") == 236000000.0,
       "USSR iron is 236,000,000 exactly (DS-896)", f"{N['iron'].get('USSR')}")
    ok(N["iron"].get("USSR", {}).get("source") == "ds896_iron", "USSR iron cites ds896_iron")
    ok("USA" not in L["oil"], "USA has no located.oil entry (0 located)")
    ok("USA" in N["oil"], "USA keeps its national oil figure (unlocated)")
    ok("USA" not in N["bauxite"], "USA has no national_1990.bauxite entry")
    ok("USA" in M["counts"]["unlocated_producers"]["oil"], "USA is listed as an unlocated oil producer")
    P = T["price_1990"]
    mined = [c for c in G.TRACKED if c != "oil"]
    ok("oil" not in P, "oil has no price row (the sim prices it at w.oil_price)")
    ok(sorted(set(P) | set(M["prices_omitted"])) == mined and not (set(P) & set(M["prices_omitted"])),
       "every mined line is either priced or named in prices_omitted, never both",
       f"priced={sorted(P)} omitted={sorted(M['prices_omitted'])}")
    bad_price = [c for c, r in P.items()
                 if not (r["usd_per_unit"] > 0 and r["source"].strip()
                         and r["as_printed"].strip() and r["conversion"].strip())]
    ok(not bad_price, "every price row carries a positive figure, as_printed, conversion and source",
       f"{bad_price}")
    ok(sorted(T["units"]) == G.TRACKED and all(T["units"][c] == G.UNITS[c] for c in G.TRACKED),
       "units name the sim's unit per tracked line")
    ok(len(T["presence"]) == 1471, "presence has 1,471 keys", f"{len(T['presence'])}")
    ok(len(T["quality"]) == len(T["presence"]) and set(T["quality"]) == set(T["presence"]),
       "quality is keyed exactly like presence")
    ok(set(T["presence"]) == set(art["districts"]),
       "presence keys are exactly the artifact's districts")
    ok(all(0 < m < 4096 for m in T["presence"].values()), "every presence mask is a non-empty 12-bit mask")
    qbad = [d for d, q in T["quality"].items()
            if len(q) != 12 or any((q[i] == 0) != ((T["presence"][d] >> i) & 1 == 0) or not 0 <= q[i] <= 3
                                   for i in range(12))]
    ok(not qbad, "quality is 0 exactly where the presence bit is 0, and 1..3 where it is set", f"{qbad[:6]}")
    ok(all(d in roster_districts for d in T["presence"]), "every presence district is a roster district")
    ok(sorted(T["pop_share"]) == sorted(owner) and sorted(T["pop_1990"]) == sorted(owner),
       "pop_share and pop_1990 cover exactly the districts held in 1990",
       f"{len(T['pop_share'])} vs {len(owner)}")
    ok(all(0.0 <= s <= 1.0 for s in T["pop_share"].values()), "every pop_share is in 0..=1")
    nation_sums = {}
    for d, s in T["pop_share"].items():
        nation_sums[owner[d]] = nation_sums.get(owner[d], 0.0) + s
    off = {n: s for n, s in nation_sums.items() if abs(s - 1.0) > 1e-6}
    ok(not off, "each 1990 owner's pop_share sums to 1 within 1e-6", f"{list(off.items())[:4]}")
    located_ids = {d for c in L for lst in L[c].values() for d, _ in lst}
    ok(located_ids <= set(T["presence"]), "every located district carries a presence bit")

    # --- 4. provenance ----------------------------------------------------
    print("\nCHECK 4: provenance")
    for name, path in (("district_resources.json", G.ART),
                       ("district_population.json", G.POP),
                       ("districts.json", G.DISTRICTS_JSON)):
        with open(path, "rb") as f:
            h = hashlib.sha256(f.read()).hexdigest()
        ok(M["source_sha256"].get(name) == h, f"source_sha256[{name}] matches the file on disk")
    ok(M["upstream"]["generator"] == art["meta"]["generator"]
       and M["upstream"]["vintage"] == art["meta"]["vintage"],
       "upstream generator and vintage are the artifact's own")
    used = sorted({r["source"] for c in N for r in N[c].values()})
    ok(sorted(M["upstream"]["sources"]) == used
       and all(M["upstream"]["sources"][k]["url"] == art["sources"][k]["url"] for k in used),
       "every cited national source is described in upstream.sources with the artifact's URL")
    ok(M["generator"] == "tools/resources/make_resources_1990.py", "meta.generator names this tool")

    # --- 5. determinism ---------------------------------------------------
    print("\nCHECK 5: determinism")
    if fast:
        print("  SKIP  (--fast)")
    else:
        with open(G.OUT, "rb") as f:
            committed = f.read()
        outs = []
        with tempfile.TemporaryDirectory() as tmp:
            for i in (1, 2):
                p = os.path.join(tmp, f"run{i}.json")
                subprocess.run([sys.executable, os.path.join(HERE, "make_resources_1990.py"),
                                "--out", p], cwd=G.ROOT, check=True, stdout=subprocess.DEVNULL)
                with open(p, "rb") as f:
                    outs.append(f.read())
        ok(outs[0] == outs[1], "two regenerations are byte-identical")
        ok(outs[0] == committed, "regeneration reproduces the committed file byte-for-byte",
           f"{hashlib.sha256(outs[0]).hexdigest()[:12]} != {hashlib.sha256(committed).hexdigest()[:12]}")
        ok(b"\r" not in committed, "the committed file uses LF newlines only")

    print(f"\n{CHECKS[0]} checks, {len(FAILURES)} failed")
    if FAILURES:
        for f in FAILURES:
            print("  FAILED:", f)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
