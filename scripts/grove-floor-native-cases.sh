#!/usr/bin/env bash
# Actual separate-prefix callers, mandatory-backend refusals and filesystem denials.
#
#   grove-floor-native-cases.sh grove-prefix version ravel-1.1-prefix [ravel-2.0-prefix]
#
# Each Ravel prefix is one runtime lane in its own HOME. The 1.1 lane writes
# `policy.ts` with schema 2. The 2.0 lane writes `config.rhai` with
# SCHEMA_VERSION 4, and in the supported environment it also runs the
# confined directory case.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "grove-floor-native: $*" >&2; exit 1; }
[[ $# == 3 || $# == 4 ]] || fail 'expected Grove prefix/version, the Ravel 1.1 prefix and an optional Ravel 2.0 prefix'
prefix="$1"
version="$2"
mode="${FLOOR_MODE:?native runner must name its environment}"
if [[ "${FLOOR_KIND:-native}" == qemu ]]; then
  /usr/libexec/floor-security || fail 'outer QEMU kernel security contract failed'
  [[ -f /usr/libexec/floor-cpu-probe ]] || fail 'missing sandbox CPU control'
else
  /smoke/security || fail 'outer container security contract failed'
fi
grep -q '^CapBnd:[[:space:]]*0000000000000000$' /proc/self/status || fail 'nonempty bounding capability set'
grep -q ' /smoke ro[, ]' /proc/self/mountinfo || fail 'smoke inputs are not mounted read-only'
if [[ "$mode" != missing ]]; then
  [[ "$(/usr/bin/bwrap --version)" == 'bubblewrap 0.11.0' ]] || fail 'wrong real backend'
  [[ ! -u /usr/bin/bwrap && ! -g /usr/bin/bwrap ]] || fail 'privileged backend'
  if [[ "${FLOOR_KIND:-native}" == qemu && "$mode" == supported ]]; then
    cmp /smoke/native/libcap.so.2 /usr/libexec/floor-backend/libcap.so.2 || fail 'wrong native floor libcap'
    /usr/libexec/floor-backend/ld-linux-aarch64.so.1 --list /usr/libexec/floor-backend/bwrap
  else
    cmp /smoke/libcap.so.2 /usr/lib64/libcap.so.2.22 || fail 'wrong floor libcap'
  fi
else
  [[ ! -e /usr/bin/bwrap && ! -e /bin/bwrap ]] || fail 'missing-backend control has a backend'
fi
if [[ "$mode" == supported ]]; then
  /bin/bash /smoke/grove-installed-smoke.sh "$prefix" "$version" "$3" ${4:+"$4"}
fi
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

# The denial task and the native guidance case of RUNTIME, in a HOME of its
# own, through the Ravel prefix RAVEL_PREFIX. The body keeps column 0, because
# its here-documents are written at column 0.
run_lane() {
lane_runtime="$1"
ravel="$2/bin/ravel"
work="$scratch/ravel-$lane_runtime"
mkdir -p "$work/home/.config/ravel" "$work/tmp" "$work/tools" "$work/project"
for utility in cat chmod cmp cp dirname env grep ln ls mkdir rm touch; do
  resolved="$(PATH=/usr/bin:/bin command -v "$utility")" || fail "missing utility $utility"
  ln -s "$resolved" "$work/tools/$utility"
done
ln -s "$ravel" "$work/tools/ravel"
for runtime in bun node harness-dispatch; do
  ! PATH="$work/tools" command -v "$runtime" >/dev/null 2>&1 || fail "unexpected runtime $runtime"
done
printf owner-secret >"$work/secret"
printf sentinel >"$work/project/existing.txt"
cat >"$work/agent.sh" <<'EOF'
#!/bin/sh
set -eu
secret="$2"
tool="$1"
floor_kind="$3"
/bin/cat /etc/passwd >/dev/null || exit 80
[ "$(/usr/bin/id -u)" = 1000 ] || exit 80
[ -f /usr/bin/true ] && [ -f /proc/sys/kernel/hostname ] || exit 93
for protected in / /usr/bin /proc /dev; do
  [ -d "$protected" ] || exit 93
  /bin/grep -q " $protected ro[, ]" /proc/self/mountinfo || exit 92
done
if /bin/cat "$secret" >/dev/null 2>&1; then exit 81; fi
if (printf hacked >"$secret") 2>/dev/null; then exit 82; fi
if /bin/sh -c '/bin/cat "$1" >/dev/null 2>&1' child "$secret"; then exit 83; fi
/bin/ln -s "$secret" alias
if /bin/cat alias >/dev/null 2>&1; then exit 84; fi
if (printf hacked >/escape) 2>/dev/null; then exit 85; fi
if (printf hacked >/usr/bin/true) 2>/dev/null; then exit 86; fi
if (printf hacked >>"$tool") 2>/dev/null; then exit 87; fi
if (printf hacked >/dev/escape) 2>/dev/null; then exit 88; fi
if (printf hacked >/proc/sys/kernel/hostname) 2>/dev/null; then exit 89; fi
/bin/grep -q '^CapEff:[[:space:]]*0000000000000000$' /proc/self/status || exit 90
/bin/grep -q '^CapBnd:[[:space:]]*0000000000000000$' /proc/self/status || exit 90
/bin/grep -q '^NoNewPrivs:[[:space:]]*1$' /proc/self/status || exit 91
if [ "$floor_kind" = qemu ]; then
  [ -f /usr/libexec/floor-cpu-probe ] || exit 98
  status=0
  /bin/sh -c 'exec /usr/libexec/floor-cpu-probe' >cpu.stdout 2>cpu.stderr || status=$?
  [ "$status" = 132 ] || exit 94
  /bin/grep -q 'cpu probe started' cpu.stdout || exit 95
  if /bin/grep -q 'executed its instruction' cpu.stdout; then exit 96; fi
  [ ! -e /usr/libexec/floor-qemu/interpreter-max ] || exit 97
  printf 'sandbox descendant CPU refusal passed (132)\n'
fi
printf 'read/write, descendant/symlink, root/system/runtime/proc/dev and payload capability denials passed\n'
printf denied-and-published >published.txt
exec "$tool" end-session grove-continue
EOF
chmod +x "$work/agent.sh"
# The positive selection control: the 1.1 policy writes a file when it loads,
# and the 2.0 configuration, which cannot write a file, prints a marker from
# select() to stderr.
selected() {
  if [[ "$lane_runtime" == 1.1 ]]; then
    [[ -f "$work/selected" ]]
  else
    grep -q 'floor selection: selected' "$work/stderr"
  fi
}
if [[ "$lane_runtime" == 1.1 ]]; then
cat >"$work/home/.config/ravel/policy.ts" <<EOF
import { definePolicy } from "ravel/sdk";
import { writeFileSync } from "node:fs";
import { withProvenance } from "$prefix/libexec/ravel-policy/provenance.ts";
import { groveReviewSelector } from "$prefix/libexec/ravel-policy/examples/grove-review.ts";
writeFileSync("$work/selected", "selected");
export const policy = withProvenance(definePolicy({ schemaVersion: 2, version: "native-denials",
  ...groveReviewSelector({ routes: { check: () => ({ program: "/bin/sh", args: ["agent.sh", "$ravel", "$work/secret", "${FLOOR_KIND:-native}"], provider: "fixture", model: "none", effort: "none" }) }, reviews: {} }) }));
EOF
else
cat >"$work/home/.config/ravel/config.rhai" <<EOF
import "$prefix/libexec/ravel-policy/examples/grove-review" as grove_review;
import "$prefix/libexec/ravel-policy/provenance" as provenance;
export const SCHEMA_VERSION = 4;
export const VERSION = "native-denials";
fn route() {
    |request| #{ program: "/bin/sh", args: ["agent.sh", "$ravel", "$work/secret", "${FLOOR_KIND:-native}"],
        provider: "fixture", model: "none", effort: "none" }
}
const RULES = #{ routes: #{ check: route() }, reviews: #{} };
fn load_context(request) { grove_review::load_context_with(global::RULES, request) }
fn select(request, context) {
    print("floor selection: selected");
    provenance::with_provenance(grove_review::select_with(global::RULES, request, context), global::VERSION, request)
}
EOF
fi
status=0
(cd "$work/project" && env -i HOME="$work/home" TMPDIR="$work/tmp" PATH="$work/tools" \
  "$prefix/bin/grove" run check Write --ui=inline --input "$work/agent.sh" \
  --output "$work/project/published.txt") \
  >"$work/stdout" 2>"$work/stderr" || status=$?
cat "$work/stderr"
if [[ "$mode" == supported ]]; then
  ((status == 0)) || fail "$lane_runtime actual denial task failed ($status)"
  selected || fail "$lane_runtime positive policy-selection control did not fire"
  [[ "$(cat "$work/project/published.txt")" == denied-and-published ]] || fail 'output not published'
  grep -q 'payload capability denials passed' "$work/stderr" || fail 'lost agent denial evidence'
  ! grep -q grove-continue "$work/stdout" || fail 'leaked lifecycle result'
else
  ((status != 0)) || fail "$mode unexpectedly succeeded"
  grep -q confinement_unusable "$work/stderr" || fail "$lane_runtime wrong refusal reason"
  ! selected || fail "$lane_runtime refused backend reached policy selection"
  [[ ! -e "$work/project/published.txt" ]] || fail 'refused backend published new output'
  [[ "$(cat "$work/project/existing.txt")" == sentinel ]] || fail 'refused backend changed existing output'
  case "$mode" in
    missing) grep -q 'requires bubblewrap' "$work/stderr" ;;
    default-seccomp) grep -q 'namespace.*Operation not permitted\|No permissions.*namespace' "$work/stderr" ;;
    masked-proc) grep -q "Can't mount proc.*Operation not permitted" "$work/stderr" ;;
    qemu-unusable) grep -q 'namespace.*Invalid argument\|Creating new namespace failed' "$work/stderr" ;;
    *) fail "unknown mode $mode" ;;
  esac
fi
[[ "$(cat "$work/secret")" == owner-secret ]] || fail 'owner secret changed'
[[ "$(cat "$work/project/existing.txt")" == sentinel ]] || fail 'adjacent existing output changed'
if [[ "$mode" == supported ]]; then
  # Native guidance outside every grant. No skill is injected, and native
  # copies lie under HOME. The confined agent cannot read them. It reports
  # the missing guidance and force-stops, and Grove publishes nothing and
  # launches once. The control grants each copy as a regular file. The same
  # agent then reads them and Grove publishes.
  native=()
  for tree in .claude/skills .codex/skills .agents/skills; do
    mkdir -p "$work/home/$tree/grove"
    printf '# grove spine (native copy)\n' >"$work/home/$tree/grove/SKILL.md"
    cat "$work/home/$tree/grove/SKILL.md" >/dev/null || fail 'unreadable native copy outside the sandbox'
    native+=("$work/home/$tree/grove/SKILL.md")
  done
  cat >"$work/native.sh" <<'EOF'
#!/bin/sh
tool="$1"
shift
printf 'fake agent: started\n' >&2
unreadable=
for file in "$@"; do
  if /bin/cat "$file" >/dev/null 2>&1; then printf 'fake agent: read %s\n' "$file" >&2
  else printf 'fake agent: cannot read guidance %s\n' "$file" >&2; unreadable=yes; fi
done
printf valid >native.txt
if [ -n "$unreadable" ]; then
  printf 'fake agent: the mandated guidance is missing or unreadable\n' >&2
  exec "$tool" end-session grove-force-stop
fi
exec "$tool" end-session grove-continue
EOF
  if [[ "$lane_runtime" == 1.1 ]]; then
  cat >"$work/home/.config/ravel/policy.ts" <<EOF
export const policy = { schemaVersion: 2, version: "native-guidance",
  select() { return { status: "selected", program: "/bin/sh", args: ["native.sh", "$ravel", "${native[0]}", "${native[1]}", "${native[2]}"], provider: "fixture", model: "none", effort: "none", reason: "native guidance" }; } };
EOF
  else
  cat >"$work/home/.config/ravel/config.rhai" <<EOF
export const SCHEMA_VERSION = 4;
export const VERSION = "native-guidance";
fn select(request, context) {
    #{ status: "selected", program: "/bin/sh", args: ["native.sh", "$ravel", "${native[0]}", "${native[1]}", "${native[2]}"],
        provider: "fixture", model: "none", effort: "none", reason: "native guidance" }
}
EOF
  fi
  native_run() {
    (cd "$work/project" && env -i HOME="$work/home" TMPDIR="$work/tmp" PATH="$work/tools" \
      "$prefix/bin/grove" run check Read --ui=inline --input "$work/native.sh" \
      --output "$work/project/native.txt" "$@") >"$work/native.stdout" 2>"$work/native.stderr"
  }
  status=0
  native_run || status=$?
  cat "$work/native.stderr"
  ((status != 0)) || fail 'unreadable native guidance published'
  [[ ! -e "$work/project/native.txt" ]] || fail 'force-stopped native case published its output'
  grep -q 'Grove force-stop requested; outputs were not published' "$work/native.stderr" ||
    fail 'Grove did not report the force-stop'
  [[ "$(grep -c 'fake agent: started' "$work/native.stderr")" == 1 ]] || fail 'the agent started more than once'
  for copy in "${native[@]}"; do
    grep -qF "fake agent: cannot read guidance $copy" "$work/native.stderr" || fail "no unreadable report for $copy"
  done
  ! grep -q 'fake agent: read ' "$work/native.stderr" || fail 'the confined agent read native guidance'
  native_run --runtime-read "${native[0]}" --runtime-read "${native[1]}" --runtime-read "${native[2]}" ||
    fail 'granted native control failed'
  [[ "$(cat "$work/project/native.txt")" == valid ]] || fail 'granted native control did not publish'
  echo "grove-floor-native: $lane_runtime unreadable native guidance force-stopped without publication; granted control published"
fi
echo "grove-floor-native: $lane_runtime $mode actual caller/selection/publication controls passed"
}

run_lane 1.1 "$3"
if [[ $# == 4 ]]; then
  run_lane 2.0 "$4"
  if [[ "$mode" == supported ]]; then
    /bin/bash /smoke/grove-floor-directory-case.sh "$prefix" "$4"
  fi
fi
