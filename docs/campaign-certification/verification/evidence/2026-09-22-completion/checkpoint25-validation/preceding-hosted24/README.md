Checkpoint24 hosted CI: completed failure

Run 35789452696 checks b51b7333b862d60fc3357b6d54d2ca5960b1543c.
All ten jobs concluded: four passed, six failed. JavaScript/tools and town-browser
passed on both platforms. Each native workspace run reports 1,881 passed, 3 failed,
89 ignored and 1 filtered across 66 targets: exactly the same three local checkpoint24 failures,
now repaired separately in checkpoint25. Political jobs each pass 10 tests and
fail A1 only (median 7 coups, top-three share 0.59); one diagnostic is filtered.
Both aggregates correctly fail because native and political dependencies failed.

result.json and latest-jobs.json retain conclusions and step summaries. All six
failed-job logs are exact decoded UTF-8 connector outputs with original platform
line endings. run.json verifies the head SHA and completed failure. No source
edits, reruns, pushes or workflow cancellations were performed. Later fixes do
not retroactively turn this checkpoint green.
