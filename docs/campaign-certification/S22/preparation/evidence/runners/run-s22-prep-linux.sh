#!/usr/bin/env bash
set -euo pipefail
source /home/ridge/.cache/spheres-s05-tools/env.sh
export PATH="/home/ridge/.cache/spheres-s21-git:$PATH"
runner=/mnt/c/Users/ridge/Documents/Codex/2026-09-05/pick-up-the-spheres-game-on/work/campaign-certification/run-s22-prep-check.py
python3 "$runner" focused linux1
python3 "$runner" manifest linux1
python3 "$runner" records linux1
