#!/usr/bin/env bash
# Run every command in an Alloy model and report pass/fail.
#
# The convention this script keys on: a command named `witness_*` must find an
# instance (it exhibits a defect or a deliberately admitted structure); every
# other command — every `check` — must find none.
#
# Alloy is found via $ALLOY_JAR, else ~/.local/share/alloy/org.alloytools.alloy.dist.jar.
# Java 17+ is required; set $JAVA if the default `java` is older.
set -euo pipefail

model="${1:-$(dirname "$0")/structure.als}"
jar="${ALLOY_JAR:-$HOME/.local/share/alloy/org.alloytools.alloy.dist.jar}"
java_bin="${JAVA:-java}"

[[ -f "$jar" ]] || { echo "alloy jar not found: $jar (set ALLOY_JAR)" >&2; exit 2; }
[[ -f "$model" ]] || { echo "model not found: $model" >&2; exit 2; }

commands=$(grep -oE '^(check|run) +[A-Za-z_][A-Za-z0-9_]*' "$model" | awk '{print $2}')
[[ -n "$commands" ]] || { echo "no commands found in $model" >&2; exit 2; }

fail=0
while read -r name; do
  out=$("$java_bin" -jar "$jar" exec -q -c "$name" -o - "$model" 2>&1) || true
  if grep -q "^Command " <<<"$out"; then found=yes; else found=no; fi
  if [[ "$name" == witness_* ]]; then want=yes; label="instance"; else want=no; label="counterexample"; fi
  if [[ "$found" == "$want" ]]; then
    printf 'PASS  %-52s %s\n' "$name" "$([[ $want == yes ]] && echo "$label found" || echo "no $label")"
  else
    printf 'FAIL  %-52s %s\n' "$name" "$([[ $want == yes ]] && echo "no $label" || echo "$label found")"
    fail=1
  fi
done <<<"$commands"

exit "$fail"
