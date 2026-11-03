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
#
# A standalone launch gives `-` as RECORDS. The wrapper then prints the view
# and the environment to standard output as `trial-*` lines, and the harness
# writes its stream there too. Grove's run log keeps both.
set -eu

records=$1
view=$2
program=$3
shift 3

for argument in "$@"; do prompt=$argument; done

if [ "$records" = - ]; then
  printf 'trial-view-b64 %s\n' "$(printf '%s' "$view" | base64 | tr -d '\n')"
  printf 'trial-env-b64 %s\n' "$(env | base64 | tr -d '\n')"
  printf 'trial-started %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  exec "$program" "$@"
fi

n=0
until mkdir "$records/$n" 2>/dev/null; do
  n=$((n + 1))
  [ "$n" -lt 100 ] || exit 90
done
record="$records/$n"
printf '%s' "$view" >"$record/view.json"
printf '%s' "${prompt-}" >"$record/prompt"
env >"$record/env"
date -u +%Y-%m-%dT%H:%M:%SZ >"$record/started"
exec "$program" "$@" >"$record/stream.jsonl" 2>"$record/stderr"
