#!/bin/bash
# Diagnostic only (Claude subagent): short Linux paired-cell run with fine RSS sampling.
# Usage: run_cell.sh <tag> <Country> <seed> <through> <interval_s>
P=/tmp/claude-0/-home-user-Spheres/e3d5a311-d2fe-504d-bb9f-1264447cf21c/scratchpad/s25-source-probe
BIN=/tmp/claude-0/-home-user-Spheres/e3d5a311-d2fe-504d-bb9f-1264447cf21c/scratchpad/target-68ba/release/deps/spheres_web-7f381178d9133e63
T=$1; D=$P/$T; mkdir -p $D
cat > $D/request.json <<R
{"format":"spheres-stability-cell/v1","id":"$T","country":"$2","seed":$3,"through":"$4","revision":"68ba0622ec709b78617aadd1f9198d18f532bb32"}
R
cd $D
date -u +%FT%TZ > $D/start.txt
SPHERES_S25_REQUEST=$D/request.json SPHERES_S25_OUT=$D/native "$BIN" --exact s25_stability_tests::s25_stability_cell --ignored --nocapture --test-threads=1 > $D/stdout.log 2> $D/stderr.log &
PID=$!; echo $PID > $D/pid.txt
echo "elapsed_s,rss_kb,hwm_kb,vmsize_kb,threads,last_progress" > $D/memory.csv
T0=$(date +%s.%N)
while kill -0 $PID 2>/dev/null; do
  if [ -r /proc/$PID/status ]; then
    S=$(awk '/VmRSS/{r=$2}/VmHWM/{h=$2}/VmSize/{v=$2}/Threads/{t=$2}END{print r","h","v","t}' /proc/$PID/status)
    L=$(tail -c 200 $D/native/progress.jsonl 2>/dev/null | grep -o '"end_native_date":"[0-9-]*"' | tail -1 | cut -d'"' -f4)
    echo "$(echo "$(date +%s.%N)-$T0" | bc),$S,$L" >> $D/memory.csv
  fi
  sleep $5
done
wait $PID; echo "exit $?" > $D/exit.txt; date -u +%FT%TZ >> $D/exit.txt
