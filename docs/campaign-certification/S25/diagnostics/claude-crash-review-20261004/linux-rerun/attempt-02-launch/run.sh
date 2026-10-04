#!/bin/bash
# Diagnostic only: Linux rerun of Japan/seed 7 at the crashed candidate, with memory sampling.
S=/tmp/claude-0/-home-user-Spheres/e3d5a311-d2fe-504d-bb9f-1264447cf21c/scratchpad
D=$S/s25-linux-japan7; WT=$S/wt-68ba
cd $WT
export CARGO_TARGET_DIR=$S/target-68ba
date -u +%FT%TZ > $D/build-start.txt
cargo test --locked --release -p spheres-web --no-run --message-format=json > $D/build.json 2> $D/build.stderr
echo "build exit $?" >> $D/build-start.txt
BIN=$(python3 -c "
import json,sys
for l in open('$D/build.json'):
    try: d=json.loads(l)
    except: continue
    if d.get('reason')=='compiler-artifact' and d.get('executable') and d['target']['name']=='spheres-web' and d['profile']['test']:
        print(d['executable'])" | tail -1)
echo "$BIN" > $D/binary.txt; sha256sum "$BIN" >> $D/binary.txt
rm -rf $D/native  # the test requires a NEW output directory
cat > $D/request.json <<R
{
  "format": "spheres-stability-cell/v1",
  "id": "japan-7",
  "country": "Japan",
  "seed": 7,
  "through": "2035-12-31",
  "revision": "68ba0622ec709b78617aadd1f9198d18f532bb32"
}
R
cd $D
date -u +%FT%TZ > $D/run-start.txt
SPHERES_S25_REQUEST=$D/request.json SPHERES_S25_OUT=$D/native "$BIN" --exact s25_stability_tests::s25_stability_cell --ignored --nocapture --test-threads=1 > $D/stdout.log 2> $D/stderr.log &
PID=$!
echo $PID > $D/pid.txt
echo "utc,elapsed_s,pid,rss_kb,hwm_kb,vmsize_kb,threads,native_files" > $D/memory.csv
T0=$(date +%s)
while kill -0 $PID 2>/dev/null; do
  C=$(pgrep -P $PID | head -1); P=${C:-$PID}
  if [ -r /proc/$P/status ]; then
    R=$(awk '/VmRSS/{print $2}' /proc/$P/status); H=$(awk '/VmHWM/{print $2}' /proc/$P/status); V=$(awk '/VmSize/{print $2}' /proc/$P/status); TH=$(awk '/Threads/{print $2}' /proc/$P/status)
    echo "$(date -u +%FT%TZ),$(( $(date +%s)-T0 )),$P,$R,$H,$V,$TH,$(find $D/native -type f 2>/dev/null | wc -l)" >> $D/memory.csv
  fi
  sleep 30
done
wait $PID; echo "exit $?" > $D/exit.txt; date -u +%FT%TZ >> $D/exit.txt
