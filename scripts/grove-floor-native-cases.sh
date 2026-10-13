#!/usr/bin/env bash
# Actual separate-prefix callers, mandatory-backend refusals and filesystem denials.
set -euo pipefail
IFS=$'\n\t'
fail() { echo "grove-floor-native: $*" >&2; exit 1; }
[[ $# == 3 ]] || fail 'expected Grove prefix/version and independent Ravel prefix'
prefix="$1"
version="$2"
ravel="$3/bin/ravel"
mode="${FLOOR_MODE:?native runner must name its environment}"
/smoke/security || fail 'outer container security contract failed'
grep -q ' /smoke ro[, ]' /proc/self/mountinfo || fail 'smoke inputs are not mounted read-only'
if [[ "$mode" != missing ]]; then
  [[ "$(/usr/bin/bwrap --version)" == 'bubblewrap 0.11.0' ]] || fail 'wrong real backend'
  [[ ! -u /usr/bin/bwrap && ! -g /usr/bin/bwrap ]] || fail 'privileged backend'
  cmp /smoke/libcap.so.2 /usr/lib64/libcap.so.2.22 || fail 'wrong floor libcap'
else
  [[ ! -e /usr/bin/bwrap && ! -e /bin/bwrap ]] || fail 'missing-backend control has a backend'
fi
if [[ "$mode" == supported ]]; then
  /bin/bash /smoke/grove-installed-smoke.sh "$prefix" "$version" "$3"
fi
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
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
/bin/grep -q '^NoNewPrivs:[[:space:]]*1$' /proc/self/status || exit 91
printf 'read/write, descendant/symlink, root/system/runtime/proc/dev and payload capability denials passed\n'
printf denied-and-published >published.txt
exec "$tool" end-session grove-continue
EOF
chmod +x "$work/agent.sh"
cat >"$work/home/.config/ravel/policy.ts" <<EOF
import { definePolicy } from "ravel/sdk";
import { writeFileSync } from "node:fs";
import { withProvenance } from "$prefix/libexec/ravel-policy/provenance.ts";
import { groveReviewSelector } from "$prefix/libexec/ravel-policy/examples/grove-review.ts";
writeFileSync("$work/selected", "selected");
export const policy = withProvenance(definePolicy({ schemaVersion: 2, version: "native-denials",
  ...groveReviewSelector({ routes: { check: () => ({ program: "/bin/sh", args: ["agent.sh", "$ravel", "$work/secret"], provider: "fixture", model: "none", effort: "none" }) }, reviews: {} }) }));
EOF
status=0
(cd "$work/project" && env -i HOME="$work/home" TMPDIR="$work/tmp" PATH="$work/tools" \
  "$prefix/bin/grove" run check Write --ui=inline --input "$work/agent.sh" \
  --output "$work/project/published.txt") \
  >"$work/stdout" 2>"$work/stderr" || status=$?
cat "$work/stderr"
if [[ "$mode" == supported ]]; then
  ((status == 0)) || fail "actual denial task failed ($status)"
  [[ -f "$work/selected" ]] || fail 'positive policy-selection control did not fire'
  [[ "$(cat "$work/project/published.txt")" == denied-and-published ]] || fail 'output not published'
  grep -q 'payload capability denials passed' "$work/stderr" || fail 'lost agent denial evidence'
  ! grep -q grove-continue "$work/stdout" || fail 'leaked lifecycle result'
else
  ((status != 0)) || fail "$mode unexpectedly succeeded"
  grep -q confinement_unusable "$work/stderr" || fail 'wrong refusal reason'
  [[ ! -e "$work/selected" ]] || fail 'refused backend reached policy selection'
  [[ ! -e "$work/project/published.txt" ]] || fail 'refused backend published new output'
  [[ "$(cat "$work/project/existing.txt")" == sentinel ]] || fail 'refused backend changed existing output'
  case "$mode" in
    missing) grep -q 'requires bubblewrap' "$work/stderr" ;;
    default-seccomp) grep -q 'namespace.*Operation not permitted\|No permissions.*namespace' "$work/stderr" ;;
    masked-proc) grep -q "Can't mount proc.*Operation not permitted" "$work/stderr" ;;
    *) fail "unknown mode $mode" ;;
  esac
fi
[[ "$(cat "$work/secret")" == owner-secret ]] || fail 'owner secret changed'
[[ "$(cat "$work/project/existing.txt")" == sentinel ]] || fail 'adjacent existing output changed'
echo "grove-floor-native: $mode actual caller/selection/publication controls passed"
