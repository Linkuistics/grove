#!/usr/bin/env bash
# Run the eight comparison runs, four at a time. Logs go to $COMPARISON_SCRATCH/logs.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
: "${COMPARISON_SCRATCH:?set COMPARISON_SCRATCH to a scratch directory}"
mkdir -p "$COMPARISON_SCRATCH/logs"

run_one() {
  local log="$COMPARISON_SCRATCH/logs/$(IFS=-; echo "$*").log"
  if python3 "$here/run.py" "$@" >"$log" 2>&1; then
    echo "$* exit 0"
  else
    echo "$* exit $?"
  fi
}

# Two waves of four. Each wave holds both arms of the same cases, so the two
# arms of a case run in the same time window. Bash 3.2 has no `wait -n`.
for wave in "d1-draft d2-copy-edit" "d3-proof exchange"; do
  for item in $wave; do
    for arm in current revised; do
      if [[ $item == exchange ]]; then run_one exchange "$arm" &
      else run_one doc "$item" "$arm" &
      fi
    done
  done
  wait
done
