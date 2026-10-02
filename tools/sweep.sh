#!/bin/bash
# usage: tools/sweep.sh "<bench args 1>" "<bench args 2>" ...
# prints one line per config: total, number of games reaching 131072
cd "$(dirname "$0")/.."
B=./target/release/cg_2048.exe
for cfg in "$@"; do
  out=$($B bench --threads 12 $cfg 2>&1)
  total=$(echo "$out" | grep TOTAL | sed -E 's/.*\(([0-9.]+M)\).*/\1/')
  wins=$(echo "$out" | grep -c "tile 131072")
  capped=$(echo "$out" | grep -c CAPPED)
  echo "$total wins=$wins capped=$capped | $cfg"
done
