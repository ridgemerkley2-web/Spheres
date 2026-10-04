#!/bin/bash
# Copies the resumed monthly archive once per simulated year (Jan 1) from MY probe run, runs breakdown, deletes copy.
P=/tmp/claude-0/-home-user-Spheres/e3d5a311-d2fe-504d-bb9f-1264447cf21c/scratchpad/s25-source-probe
D=$P/japan7-to1995; PID=$(cat $D/pid.txt); done_years=""
while kill -0 $PID 2>/dev/null; do
  L=$(tail -c 300 $D/native/progress.jsonl 2>/dev/null | grep -o '"end_native_date":"[0-9-]*"' | tail -1 | cut -d'"' -f4)
  case "$L" in
    *-01-01|*-07-01)
      if ! echo "$done_years" | grep -q "$L"; then
        sleep 2
        cp $D/native/resumed/saves/monthly.json $P/snap-$L.json 2>/dev/null && python3 $P/breakdown.py $P/snap-$L.json $P/breakdown-$L.json > /dev/null && python3 $P/analyze_depth.py $P/snap-$L.json > $P/depth-$L.txt; rm -f $P/snap-$L.json
        done_years="$done_years $L"
      fi;;
  esac
  sleep 3
done
