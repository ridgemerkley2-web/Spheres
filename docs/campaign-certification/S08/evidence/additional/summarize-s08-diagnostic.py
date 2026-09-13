"""Read-only summary of an existing developmental diagnostic; no acceptance claim."""
import collections
import json
import pathlib
import re
import sys

report = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8-sig"))
summaries = report["subsystem_summaries"]
main = {key: value for key, value in summaries.items() if not key.startswith("detail.")}
print("Developmental subsystem medians / p95 / maxima (ms)")
for key, value in sorted(main.items(), key=lambda row: -row[1]["max_ms"])[:14]:
    print(f'{key}: {value["median_ms"]:.3f} / {value["p95_ms"]:.3f} / {value["max_ms"]:.3f}')

groups = collections.defaultdict(float)
for row in report["rows"]:
    for stage in row["stages"]:
        name = stage["name"]
        if name.startswith("detail.economic_ai."):
            groups[".".join(name.split(".")[2:-1])] += stage["elapsed_ms"]
print("AI stage totals (nested review.total is inclusive)")
print(json.dumps(dict(sorted(groups.items(), key=lambda row: -row[1])), indent=2))

for group in ["dispatch_plan", "dispatch_source_search", "dispatch_source_setup",
              "dispatch_source_traversal", "dispatch_source_fallback", "dispatch_assembly", "dispatch_cache_validity"]:
    selected = [(key, value) for key, value in summaries.items() if key.startswith("detail.market." + group + ".")]
    print(group, {key.rsplit(".", 1)[-1]: round(value["p95_ms"], 3)
        for key, value in selected if value["p95_ms"] > 0.1})
print("Worst normal driver days:", [(row["date_before"], row["normal_game_ms"])
    for row in sorted(report["rows"], key=lambda row: -row["normal_game_ms"])[:5]])
heap_names = ['pushes', 'decreases', 'pops', 'stale_pops', 'expanded_nodes', 'examined_edges', 'max_queue']
for row in sorted(report['rows'], key=lambda row: -row['normal_game_ms'])[:3]:
    totals = dict.fromkeys(heap_names, 0)
    found = False
    for stage in row['stages']:
        if stage['name'].startswith('detail.market.dispatch_heap_counts.'):
            values = [int(value) for value in re.findall(r'(\d+)-', stage['name'])]
            assert len(values) == len(heap_names)
            for name, value in zip(heap_names, values):
                totals[name] = max(totals[name], value) if name == 'max_queue' else totals[name] + value
            found = True
    if found:
        totals['stale_share_of_pops'] = totals['stale_pops'] / max(1, totals['pops'])
        print('Heap work', row['date_before'], totals)
print("All days match normal native state:", len(report["rows"]) == 31
    and all(row["exact_native_world"] for row in report["rows"]))
