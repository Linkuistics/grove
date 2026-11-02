#!/bin/sh
# The route program of a harness trial (scripts/harness_trial.py). The
# configuration script of the trial selects this wrapper with these arguments:
#
#   trial-launch.sh RECORDS VIEW PROGRAM ARG... PROMPT
#
# RECORDS is the launch-record directory of the trial, VIEW the JSON of the
# request fields that the script observed, and PROGRAM the harness. The wrapper
# claims the next numbered record, writes the view, the prompt and the
# environment that the harness receives, and then execs the harness with its
# standard output and standard error in the record. The harness keeps the
# process identity of the wrapper, so Ravel manages the harness itself.
set -eu
records=$1
view=$2
program=$3
shift 3

n=0
until mkdir "$records/$n" 2>/dev/null; do
  n=$((n + 1))
  [ "$n" -lt 100 ] || exit 90
done
record="$records/$n"
printf '%s' "$view" >"$record/view.json"
for argument in "$@"; do prompt=$argument; done
printf '%s' "${prompt-}" >"$record/prompt"
env >"$record/env"
date -u +%Y-%m-%dT%H:%M:%SZ >"$record/started"
exec "$program" "$@" >"$record/stream.jsonl" 2>"$record/stderr"
