#!/usr/bin/env bash
#
# The installed-layout smoke test: exercise an installed harness-dispatch pair
# as its owner would, from the installation alone.
#
#   installed-smoke.sh PREFIX VERSION
#
# PREFIX holds bin/harness-dispatch and libexec/harness-dispatch/, as a release
# archive and `dispatch.sh install` lay them out. VERSION is the package
# version the front and its worker must both report. Run it where the
# installation is meant to run, with no Bun or Node on PATH. It refuses to
# start otherwise, because a host runtime could then stand in for the one
# inside the worker. Grove's scripts/release-smoke.sh runs it for each release
# target: natively on macOS arm64, and in a glibc-2.17 userland on Linux.
#
# It needs only bash 3.2, coreutils, grep and cmp, so one file runs under
# macOS's /bin/bash and under CentOS 7's. Its policies, fake harnesses and run
# records live in one fresh directory under TMPDIR, removed on exit. It writes
# them with `write_lines`, never a heredoc: under Docker Desktop's x86_64
# emulation (QEMU 8.1.5), CentOS 7's `cat` segfaults reading a heredoc and
# bash's own `read` hangs on one.
#
# Every case runs twice: through PREFIX/bin/harness-dispatch, and through a
# relative symlink to it from another directory, as Homebrew links a keg's
# bin/ into its prefix. Both must find the worker under PREFIX's libexec/.
#
# ADDING A CASE. Write `case_<name> FRONT DIR` and add <name> to CASES. A case
# works only inside DIR, a fresh directory of its own, and stops the smoke test
# through `fail`.

set -euo pipefail
IFS=$'\n\t'

CASES=(table_typescript parameters_typescript child_answer declared_package signal_state)

fail() {
  echo "installed-smoke: FAIL: $*" >&2
  exit 1
}

note() {
  echo "installed-smoke: $*"
}

# FILE must contain FRAGMENT verbatim. The reports are compact JSON with sorted
# keys, so a fragment is a stable, exact assertion without a JSON parser, which
# a glibc-2.17 userland does not have.
expect_json() {
  local file="$1" fragment="$2"
  grep -Fq -- "$fragment" "$file" || fail "$file lacks $fragment; it holds: $(cat "$file")"
}

# The ID KEY names in FILE's JSON.
json_id() {
  local file="$1" key="$2"
  sed -n "s/.*\"$key\":\"\([0-9a-f-]*\)\".*/\1/p" "$file" | head -n 1
}

# The table case's arguments as the reports spell them.
args_json() {
  local task_file="$1" prompt_json="$2"
  printf '"--kind","smoke","--task-file","%s","--task-id","smoke-task","--model","smoke-model","--effort","high","%s"' \
    "$task_file" "$prompt_json"
}

# The relative path from the canonical directory DIR up to /.
up_to_root() {
  local dir="$1" up=""
  while [[ "$dir" != / ]]; do
    up="../$up"
    dir="$(dirname "$dir")"
  done
  printf '%s' "$up"
}

# Write LINES as FILE, each ending in a newline.
write_lines() {
  local file="$1"
  shift
  printf '%s\n' "$@" >"$file"
}

# A fake harness that records beside itself, in received/, its argv, the run
# identity and state directory `run` exported, and its cwd; then exits 42, a
# status none of harness-dispatch's own exits share, so `run`'s status can
# only be the harness's.
write_fake_harness() {
  local path="$1"
  mkdir -p "$(dirname "$path")/received"
  # shellcheck disable=SC2016 # the harness's own code, expanded when it runs
  write_lines "$path" \
    '#!/bin/sh' \
    'received="$(dirname "$0")/received"' \
    'i=0' \
    'for arg in "$0" "$@"; do' \
    '  printf "%s" "$arg" >"$received/arg.$i"' \
    '  i=$((i + 1))' \
    'done' \
    'printf "%s" "$#" >"$received/argc"' \
    'printf "%s" "${HARNESS_DISPATCH_RUN_ID-unset}" >"$received/run-id"' \
    'printf "%s" "${HARNESS_DISPATCH_STATE_DIR-unset}" >"$received/state-dir"' \
    'pwd -P >"$received/cwd"' \
    'exit 42'
  chmod +x "$path"
}

# The harness must have received exactly ARGS as its argv, byte for byte.
expect_received() {
  local received="$1" i=0 arg
  shift
  [[ "$(cat "$received/argc")" == "$(($# - 1))" ]] ||
    fail "the fake harness received $(cat "$received/argc") arguments after its program, not $(($# - 1))"
  for arg in "$@"; do
    printf '%s' "$arg" | cmp -s - "$received/arg.$i" ||
      fail "the fake harness's argument $i is [$(cat "$received/arg.$i")], not [$arg]"
    i=$((i + 1))
  done
}

# A policy in TypeScript whose `select` consults a table by kind: an interface,
# annotated bindings and a type-only import, which only a TypeScript loader
# accepts, across a relative import, with its SDK from the embedded
# `harness-dispatch/sdk`. Its command places every caller input the request
# carries, so each one is checked end to end.
case_table_typescript() {
  local front="$1" dir="$2"
  local harness="$dir/harness/fake-harness" state="$dir/state"
  local prompt=$'installed smoke; $HOME stays literal\n'
  # shellcheck disable=SC2016 # the same text as JSON spells it, unexpanded
  local prompt_json='installed smoke; $HOME stays literal\n'
  mkdir -p "$dir/policy" "$dir/cwd"
  write_fake_harness "$harness"
  write_lines "$dir/policy/table.ts" \
    'import type { Selected, SelectionRequest } from "harness-dispatch/sdk";' \
    '' \
    'export interface Entry {' \
    '  readonly model: string;' \
    '  readonly effort: string;' \
    '}' \
    '' \
    'export const table: Readonly<Record<string, Entry>> = {' \
    '  smoke: { model: "smoke-model", effort: "high" },' \
    '};' \
    '' \
    'export function command(entry: Entry, request: SelectionRequest): Omit<Selected, "status" | "reason"> {' \
    '  return {' \
    "    program: \"$harness\"," \
    '    args: [' \
    '      "--kind", request.kind,' \
    '      "--task-file", request.taskFile ?? "none",' \
    '      "--task-id", request.taskId ?? "none",' \
    '      "--model", entry.model,' \
    '      "--effort", entry.effort,' \
    '      request.prompt,' \
    '    ],' \
    '    provider: "smoke-provider",' \
    '    model: entry.model,' \
    '    effort: entry.effort,' \
    '  };' \
    '}'
  # shellcheck disable=SC2016 # TypeScript template literals, not shell expansions
  write_lines "$dir/policy/policy.ts" \
    'import { definePolicy } from "harness-dispatch/sdk";' \
    'import { command, table, type Entry } from "./table.ts";' \
    '' \
    'export const policy = definePolicy({' \
    '  schemaVersion: 2,' \
    '  version: "installed-smoke",' \
    '  select(request) {' \
    '    const entry: Entry | undefined = table[request.kind];' \
    '    if (entry === undefined) {' \
    '      return { status: "refused", code: "smoke_unrouted", message: `no entry for ${request.kind}`, remedy: "add one" };' \
    '    }' \
    '    return { status: "selected", ...command(entry, request), reason: `table[${request.kind}]` };' \
    '  },' \
    '});'
  local selection=(--kind smoke --config "$dir/policy/policy.ts" --task-file task.md
    --task-id smoke-task --state-dir "$state")
  local args
  args="$(args_json "$dir/cwd/task.md" "$prompt_json")"

  (cd "$dir/cwd" && "$front" inspect "${selection[@]}" --prompt "$prompt" --json) \
    >"$dir/inspect.json" || fail "inspect exited $?"
  expect_json "$dir/inspect.json" '"evidence":"proposal"'
  expect_json "$dir/inspect.json" '"authority":"explicit"'
  expect_json "$dir/inspect.json" '"version":"installed-smoke"'
  expect_json "$dir/inspect.json" \
    '"selection":{"effort":"high","model":"smoke-model","provider":"smoke-provider","reason":"table[smoke]"}'
  expect_json "$dir/inspect.json" \
    "\"command\":{\"args\":[$args],\"executable\":\"$harness\",\"program\":\"$harness\"}"
  expect_json "$dir/inspect.json" \
    "\"packageVersion\":\"$VERSION\",\"path\":\"$PREFIX/libexec/harness-dispatch/harness-dispatch-policy\"}"

  local status=0
  (cd "$dir/cwd" && "$front" run "${selection[@]}" --prompt "$prompt" --json) \
    >"$dir/run.stdout" 2>"$dir/run.stderr" || status=$?
  [[ "$status" == 42 ]] ||
    fail "run exited $status, not the fake harness's 42; its stderr: $(cat "$dir/run.stderr")"
  local run_id received="$dir/harness/received"
  run_id="$(json_id "$dir/run.stderr" runId)"
  [[ -n "$run_id" ]] || fail "run reported no run ID: $(cat "$dir/run.stderr")"
  expect_json "$dir/run.stderr" "\"handoff\":{\"effort\":\"high\",\"executable\":\"$harness\",\"kind\":\"smoke\",\"model\":\"smoke-model\",\"provider\":\"smoke-provider\""
  expect_received "$received" "$harness" --kind smoke --task-file "$dir/cwd/task.md" \
    --task-id smoke-task --model smoke-model --effort high "$prompt"
  [[ "$(cat "$received/run-id")" == "$run_id" ]] ||
    fail "the harness's HARNESS_DISPATCH_RUN_ID is $(cat "$received/run-id"), not the run's $run_id"
  [[ "$(cat "$received/state-dir")" == "$state" ]] ||
    fail "the harness's HARNESS_DISPATCH_STATE_DIR is $(cat "$received/state-dir"), not $state"
  [[ "$(cat "$received/cwd")" == "$dir/cwd" ]] ||
    fail "the harness ran in $(cat "$received/cwd"), not the caller's $dir/cwd"

  # Reading the committed run back executes the bundled SQLite both ways.
  [[ -f "$state/records.sqlite3" ]] || fail "run left no record store in $state"
  "$front" record show --run "$run_id" --state-dir "$state" --json >"$dir/record.json" ||
    fail "record show of run $run_id exited $?"
  expect_json "$dir/record.json" "\"runId\":\"$run_id\""
  expect_json "$dir/record.json" '"evidence":"handoff_attempt"'
  expect_json "$dir/record.json" '"execution":"unknown"'
  expect_json "$dir/record.json" '"provider":"smoke-provider"'
  expect_json "$dir/record.json" '"taskId":"smoke-task"'
  expect_json "$dir/record.json" "\"argv\":[\"$harness\",$args]"
}

# A policy in TypeScript that builds its command from the caller's parameters:
# an asynchronous `select`, annotated with the SDK's types, that waits on a
# timer, reads the versioned request and imports the embedded static example by
# its specifier. A parameter value sits inside one argument, and its reason
# carries what it read, so the reports show the request reached it. Without the
# parameter it refuses, so a refused run is seen to start nothing, and without
# a prompt it sees the marker the SDK names.
case_parameters_typescript() {
  local front="$1" dir="$2"
  local harness="$dir/harness/fake-harness" state="$dir/state"
  local prompt=$'parameters smoke; $HOME stays literal\n'
  mkdir -p "$dir/policy" "$dir/cwd"
  write_fake_harness "$harness"
  # shellcheck disable=SC2016 # TypeScript template literals, not shell expansions
  write_lines "$dir/policy/policy.ts" \
    'import { definePolicy, PROMPT_NOT_SUPPLIED, type SelectionRequest, type SelectionResult } from "harness-dispatch/sdk";' \
    'import { policy as example } from "harness-dispatch/examples/static";' \
    '' \
    'async function select(request: SelectionRequest): Promise<SelectionResult> {' \
    '  await new Promise((resolve) => setTimeout(resolve, 10));' \
    '  const session = request.params["session"];' \
    '  if (session === undefined) {' \
    '    return { status: "refused", code: "smoke_no_session", message: "the smoke policy needs a session", remedy: "pass --param session=NAME" };' \
    '  }' \
    '  const prompted = request.prompt === PROMPT_NOT_SUPPLIED ? "unprompted" : "prompted";' \
    '  return {' \
    '    status: "selected",' \
    "    program: \"$harness\"," \
    '    args: [`--session=${session}`, request.prompt],' \
    '    provider: "smoke-provider",' \
    '    model: "smoke-model",' \
    '    effort: "high",' \
    '    reason: `${prompted} for ${request.kind}/${request.taskId} beside ${example.version}`,' \
    '  };' \
    '}' \
    '' \
    'export const policy = definePolicy({' \
    '  schemaVersion: 2,' \
    '  version: "installed-smoke-parameters",' \
    '  select,' \
    '});'
  local selection=(--kind smoke --config "$dir/policy/policy.ts" --task-id smoke-task
    --state-dir "$state")
  local reason='for smoke/smoke-task beside harness-dispatch/examples/static 2"'

  (cd "$dir/cwd" && "$front" inspect "${selection[@]}" --param 'session=smoke one' --json) \
    >"$dir/inspect.json" || fail "inspect exited $?"
  expect_json "$dir/inspect.json" '"version":"installed-smoke-parameters"'
  expect_json "$dir/inspect.json" '"params":{"session":"smoke one"}'
  expect_json "$dir/inspect.json" '"prompt":{"marker":"<harness-dispatch inspect: no prompt was supplied>","supplied":false}'
  expect_json "$dir/inspect.json" \
    "\"command\":{\"args\":[\"--session=smoke one\",\"<harness-dispatch inspect: no prompt was supplied>\"],\"executable\":\"$harness\",\"program\":\"$harness\"}"
  expect_json "$dir/inspect.json" "\"reason\":\"unprompted $reason"

  local status=0 received="$dir/harness/received"
  (cd "$dir/cwd" && "$front" run "${selection[@]}" --prompt "$prompt" --json) \
    >"$dir/refused.stdout" 2>"$dir/refused.stderr" || status=$?
  [[ "$status" == 3 ]] ||
    fail "a run the policy refuses exited $status, not 3; its stderr: $(cat "$dir/refused.stderr")"
  expect_json "$dir/refused.stderr" '"code":"policy_refused"'
  expect_json "$dir/refused.stderr" '"policyCode":"smoke_no_session"'
  [[ ! -e "$received/argc" ]] || fail "a refused run started the fake harness"

  status=0
  (cd "$dir/cwd" && "$front" run "${selection[@]}" --param 'session=smoke one' --prompt "$prompt" --json) \
    >"$dir/run.stdout" 2>"$dir/run.stderr" || status=$?
  [[ "$status" == 42 ]] ||
    fail "run exited $status, not the fake harness's 42; its stderr: $(cat "$dir/run.stderr")"
  local run_id
  run_id="$(json_id "$dir/run.stderr" runId)"
  [[ -n "$run_id" ]] || fail "run reported no run ID: $(cat "$dir/run.stderr")"
  expect_json "$dir/run.stderr" "\"reason\":\"prompted $reason"
  expect_received "$received" "$harness" '--session=smoke one' "$prompt"

  "$front" record show --run "$run_id" --state-dir "$state" --json >"$dir/record.json" ||
    fail "record show of run $run_id exited $?"
  expect_json "$dir/record.json" '"params":{"session":"smoke one"}'
  expect_json "$dir/record.json" "\"reason\":\"prompted $reason"
}

# A policy that starts a child and uses its answer, as one that hands the
# prompt to a deciding agent does. A script stands in for the agent: it records
# its cwd and the prompt it was given beside itself, and answers `quick`. The
# policy starts it with the runtime's own process API in the caller's
# directory, under the host's signal, reaps it, and looks the answer up among
# the commands it wrote itself. So the answer is seen to choose the command,
# and the child to have run where the caller is, which the worker never is.
case_child_answer() {
  local front="$1" dir="$2"
  local harness="$dir/harness/fake-harness" state="$dir/state" agent="$dir/agent/decide"
  local prompt=$'child smoke; $HOME stays literal\n'
  mkdir -p "$dir/policy" "$dir/cwd" "$dir/agent"
  write_fake_harness "$harness"
  # shellcheck disable=SC2016 # the stand-in's own code, expanded when it runs
  write_lines "$agent" \
    '#!/bin/sh' \
    'here="$(dirname "$0")"' \
    'pwd -P >"$here/cwd"' \
    'printf "%s" "$1" >"$here/prompt"' \
    'echo quick'
  chmod +x "$agent"
  # shellcheck disable=SC2016 # TypeScript template literals, not shell expansions
  write_lines "$dir/policy/policy.ts" \
    'import { definePolicy, type SelectHost, type SelectionRequest, type SelectionResult } from "harness-dispatch/sdk";' \
    '' \
    'const efforts: Readonly<Record<string, string>> = { deep: "high", quick: "low" };' \
    '' \
    'async function select(request: SelectionRequest, _context: unknown, host: SelectHost): Promise<SelectionResult> {' \
    "  const agent = Bun.spawn([\"$agent\", request.prompt], {" \
    '    cwd: request.cwd, stdin: "ignore", stdout: "pipe", stderr: "ignore", signal: host.signal,' \
    '  });' \
    '  const answer = (await new Response(agent.stdout).text()).trim();' \
    '  await agent.exited;' \
    '  const effort = efforts[answer];' \
    '  if (effort === undefined) {' \
    '    return { status: "refused", code: "smoke_answer_unknown", message: `the child answered ${JSON.stringify(answer)}`, remedy: "answer deep or quick" };' \
    '  }' \
    '  return {' \
    '    status: "selected",' \
    "    program: \"$harness\"," \
    '    args: ["--effort", effort, request.prompt],' \
    '    provider: "smoke-provider",' \
    '    model: "smoke-model",' \
    '    effort,' \
    '    reason: `the child answered ${answer}`,' \
    '  };' \
    '}' \
    '' \
    'export const policy = definePolicy({' \
    '  schemaVersion: 2,' \
    '  version: "installed-smoke-child",' \
    '  select,' \
    '});'

  local status=0 received="$dir/harness/received"
  (cd "$dir/cwd" && "$front" run --kind smoke --config "$dir/policy/policy.ts" \
    --state-dir "$state" --prompt "$prompt" --json) \
    >"$dir/run.stdout" 2>"$dir/run.stderr" || status=$?
  [[ "$status" == 42 ]] ||
    fail "run exited $status, not the fake harness's 42; its stderr: $(cat "$dir/run.stderr")"
  expect_json "$dir/run.stderr" '"reason":"the child answered quick"'
  expect_json "$dir/run.stderr" '"effort":"low"'
  expect_received "$received" "$harness" --effort low "$prompt"
  [[ "$(cat "$dir/agent/cwd")" == "$dir/cwd" ]] ||
    fail "the policy's child ran in $(cat "$dir/agent/cwd"), not the caller's $dir/cwd"
  printf '%s' "$prompt" | cmp -s - "$dir/agent/prompt" ||
    fail "the policy's child was given [$(cat "$dir/agent/prompt")], not the prompt"
}

# Two packages in a node_modules beside the policy, each found only through
# its package.json: one names its entry with `main`, the other with `exports`.
# Neither has an index.js, so a worker that read no package.json would refuse
# the import. What they export becomes the policy's version, which inspection
# reports.
case_declared_package() {
  local front="$1" dir="$2"
  local packages="$dir/policy/node_modules"
  mkdir -p "$packages/smoke-main/lib" "$packages/smoke-exports/dist" "$dir/cwd"
  write_lines "$packages/smoke-main/package.json" \
    '{ "name": "smoke-main", "main": "./lib/entry.js" }'
  write_lines "$packages/smoke-main/lib/entry.js" \
    'export const which = "main";'
  write_lines "$packages/smoke-exports/package.json" \
    '{ "name": "smoke-exports", "type": "module", "exports": { ".": "./dist/entry.js" } }'
  write_lines "$packages/smoke-exports/dist/entry.js" \
    'export const which = "exports";'
  # shellcheck disable=SC2016 # a TypeScript template literal, not a shell expansion
  write_lines "$dir/policy/policy.ts" \
    'import { which as main } from "smoke-main";' \
    'import { which as exported } from "smoke-exports";' \
    '' \
    'export const policy = {' \
    '  schemaVersion: 2,' \
    '  version: `installed-smoke-${main}-${exported}`,' \
    '  select: (request) => ({ status: "selected", program: "true", args: [request.prompt], provider: "smoke-provider", model: "smoke-model", effort: "low", reason: "smoke" }),' \
    '};'

  (cd "$dir/cwd" && "$front" inspect --kind smoke --config "$dir/policy/policy.ts" \
    --state-dir "$dir/state" --json) >"$dir/inspect.json" ||
    fail "inspect of a policy importing packages declared by main and exports exited $?"
  expect_json "$dir/inspect.json" '"version":"installed-smoke-main-exports"'
  expect_json "$dir/inspect.json" '"command":{"args":["<harness-dispatch inspect: no prompt was supplied>"],"executable":"'
}

# The caller's SIGPIPE and HUP reach the harness as they were: the front's
# initializer records them before the Rust runtime ignores SIGPIPE, and its
# pre-exec hook reinstates them after std resets it. The harness sends itself
# the signal. An inherited ignore lets it run on and exit 42; the default kills
# it, and `run`'s status is that death's. A shell cannot reset a signal it
# started with ignored, so the default direction first checks that this
# script's own children start with the default.
case_signal_state() {
  local front="$1" dir="$2"
  local harness="$dir/harness/self-signalling" state="$dir/state"
  mkdir -p "$dir/policy" "$dir/harness"
  # shellcheck disable=SC2016 # the harness's own code, expanded when it runs
  write_lines "$harness" \
    '#!/bin/sh' \
    'kill -s "$SMOKE_SIGNAL" $$' \
    'exit 42'
  chmod +x "$harness"
  write_lines "$dir/policy/policy.ts" \
    'export const policy = {' \
    '  schemaVersion: 2,' \
    '  version: "installed-smoke-signals",' \
    "  select: (request) => ({ status: \"selected\", program: \"$harness\", args: [request.prompt], provider: \"smoke-provider\", model: \"smoke-model\", effort: \"low\", reason: \"smoke\" })," \
    '};'
  local selection=(run --kind smoke --config "$dir/policy/policy.ts" --state-dir "$state"
    --prompt p --json)
  local signal number status
  for signal in PIPE:13 HUP:1; do
    number="${signal#*:}"
    signal="${signal%:*}"

    status=0
    (trap '' "$signal" && SMOKE_SIGNAL="$signal" exec "$front" "${selection[@]}") \
      >"$dir/ignored-$signal.stdout" 2>"$dir/ignored-$signal.stderr" || status=$?
    [[ "$status" == 42 ]] ||
      fail "with SIG$signal ignored by the caller, run exited $status, not 42: the harness did not inherit the ignore; its stderr: $(cat "$dir/ignored-$signal.stderr")"

    # Bash reports a child's death by HUP on its own stderr; the braces'
    # redirection hushes that report of a death the case asks for.
    status=0
    { SMOKE_SIGNAL="$signal" "$harness"; } 2>/dev/null || status=$?
    [[ "$status" == $((128 + number)) ]] ||
      fail "this script's children do not start with SIG$signal at its default (the harness alone exited $status); run the smoke test from a caller that leaves SIG$signal at its default"
    status=0
    { SMOKE_SIGNAL="$signal" "$front" "${selection[@]}" \
      >"$dir/default-$signal.stdout" 2>"$dir/default-$signal.stderr"; } 2>/dev/null || status=$?
    [[ "$status" == $((128 + number)) ]] ||
      fail "with SIG$signal at its default, run exited $status, not $((128 + number)): the harness did not inherit the default; its stderr: $(cat "$dir/default-$signal.stderr")"
  done
}

main() {
  (($# == 2)) || fail "usage: installed-smoke.sh PREFIX VERSION"
  [[ -x "$1/bin/harness-dispatch" ]] || fail "$1/bin/harness-dispatch is not an executable"
  PREFIX="$(cd "$1" && pwd -P)"
  VERSION="$2"
  local runtime found
  for runtime in bun node; do
    if found="$(command -v "$runtime")"; then
      fail "PATH holds $runtime at $found; run with a PATH that has no Bun or Node, so only the worker's own runtime can evaluate TypeScript"
    fi
  done

  local work
  work="$(mktemp -d)"
  # shellcheck disable=SC2064 # expand now: the trap must remove this directory
  trap "rm -rf '$work'" EXIT
  work="$(cd "$work" && pwd -P)"
  # Policies embed these paths in TypeScript strings, and the assertions in
  # JSON ones, both unescaped.
  local path
  for path in "$PREFIX" "$work"; do
    [[ "$path" =~ ^[A-Za-z0-9/._-]+$ ]] ||
      fail "$path holds a character the fixtures would have to escape; use a plainer TMPDIR or prefix"
  done

  local direct="$PREFIX/bin/harness-dispatch" linked="$work/linked/bin/harness-dispatch"
  mkdir -p "$work/linked/bin"
  ln -s "$(up_to_root "$work/linked/bin")${PREFIX#/}/bin/harness-dispatch" "$linked"
  local front reported
  for front in "$direct" "$linked"; do
    reported="$("$front" --version)" || fail "$front --version exited $?"
    [[ "$reported" == "harness-dispatch $VERSION" ]] ||
      fail "$front reports '$reported', not 'harness-dispatch $VERSION'"
  done
  note "no bun or node on PATH ($PATH); $direct and a relative symlink to it report $VERSION"

  local name label
  for name in "${CASES[@]}"; do
    for front in "$direct" "$linked"; do
      label=direct
      [[ "$front" == "$linked" ]] && label=symlink
      "case_$name" "$front" "$work/$name-$label"
      note "$name through the $label front: passed"
    done
  done
  note "worker $(sed -n 's/.*"worker":\({[^}]*}\).*/\1/p' "$work/${CASES[0]}-direct/inspect.json")"
  note "all ${#CASES[@]} case(s) passed through both fronts"
}

main "$@"
