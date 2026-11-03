#!/bin/sh
# The route program of a harness trial (scripts/harness_trial.py). The
# configuration script of the trial selects this wrapper with these arguments:
#
#   trial-launch.sh RECORDS VIEW PROGRAM [--probe OP PATH | --tmp-env NAME SUB |
#                   --tmp-link SUB TARGET | --env NAME VALUE]... ARG... PROMPT
#
# RECORDS is the launch-record directory of the trial, VIEW the JSON of the
# request fields that the script observed, and PROGRAM the harness. The wrapper
# claims the next numbered record, writes the view, the prompt and the
# environment that the harness receives, and then execs the harness with its
# standard output and standard error in the record. The harness keeps the
# process identity of the wrapper, so Ravel manages the harness itself.
#
# A confined launch can write only inside its invocation directory. RECORDS is
# then `-`: the wrapper prints the view, the environment and each probe to
# standard output as `trial-*` lines, and the harness writes its stream there
# too. Grove's run log keeps both.
#
# Each `--probe read PATH` or `--probe write PATH` tries one access from inside
# the launch, before the harness starts, and reports `ok` or `failed`. A write
# probe removes the file that it made. `trial-launch.sh --probes-only` followed
# by probes runs the same probes, unconfined, for the runner's contrast.
#
# A confined harness can write only in the working directory and in the
# private run directory that Ravel names in TMPDIR. `--tmp-env NAME SUB`
# makes the directory `$TMPDIR/SUB` and exports NAME with its path, and SUB
# `.` names TMPDIR itself. `--tmp-link SUB TARGET` makes `$TMPDIR/SUB` a
# symbolic link to TARGET, such as a sign-in that a `--runtime-read` grant
# admits read-only. `--env NAME VALUE` exports NAME with a fixed VALUE. Ravel
# removes the private run directory after the run.
# The configuration script gives the set-up after the probes, and the wrapper
# runs both in order, before it records the environment.
set -eu

probe() {
  if [ "$1" = read ]; then
    if cat -- "$2" >/dev/null 2>&1; then status=ok; else status=failed; fi
  elif [ "$1" = write ]; then
    if (: >"$2") 2>/dev/null; then
      status=ok
      rm -f -- "$2"
    else
      status=failed
    fi
  else
    exit 91
  fi
  printf 'trial-probe %s %s %s\n' "$1" "$status" "$2"
}

if [ "$1" = --probes-only ]; then
  shift
  while [ "$#" -ge 3 ] && [ "$1" = --probe ]; do
    probe "$2" "$3"
    shift 3
  done
  exit 0
fi

records=$1
view=$2
program=$3
shift 3

probes=
while [ "$#" -ge 3 ]; do
  case $1 in
  --probe)
    probes="${probes}$(probe "$2" "$3")
"
    ;;
  --tmp-env)
    if [ "$3" = . ]; then value=$TMPDIR; else value=$TMPDIR/$3; fi
    mkdir -p -- "$value"
    export "$2=$value"
    ;;
  --tmp-link)
    ln -s -- "$3" "$TMPDIR/$2"
    ;;
  --env)
    export "$2=$3"
    ;;
  *) break ;;
  esac
  shift 3
done
for argument in "$@"; do prompt=$argument; done

if [ "$records" = - ]; then
  printf 'trial-view-b64 %s\n' "$(printf '%s' "$view" | base64 | tr -d '\n')"
  printf 'trial-env-b64 %s\n' "$(env | base64 | tr -d '\n')"
  printf '%s' "$probes"
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
printf '%s' "$probes" >"$record/probes"
env >"$record/env"
date -u +%Y-%m-%dT%H:%M:%SZ >"$record/started"
exec "$program" "$@" >"$record/stream.jsonl" 2>"$record/stderr"
