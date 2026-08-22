#!/usr/bin/env bash
# Run every command in an Alloy model and report pass/fail.
#
# The convention this script keys on: a command named `witness_*` must find an
# instance (it exhibits a defect or a deliberately admitted structure); every
# other command — every `check` — must find none.
#
# Alloy is found via $ALLOY_JAR, else ~/.local/share/alloy/org.alloytools.alloy.dist.jar.
#
# Java 17+ is required, and finding it is this script's job rather than the
# caller's: a machine whose default `java` is older makes Alloy fail to start,
# and Alloy's not-found-and-succeeded reporting then turns a dead tool into a
# suite where every `check` passes and every witness fails. That reads as a
# broken model and is a broken instrument. $JAVA is tried first, but is
# ignored if it too is older than 17 — a stale override should not disable the
# suite it was set to enable.
set -euo pipefail

model="${1:-$(dirname "$0")/structure.als}"
jar="${ALLOY_JAR:-$HOME/.local/share/alloy/org.alloytools.alloy.dist.jar}"

# --- find a Java 17+ --------------------------------------------------------
java_major() {
  [[ -x "$1" ]] || return 1
  "$1" -version 2>&1 | sed -n '1s/.*version "\([0-9]*\).*/\1/p'
}
pick_java() {
  local c major
  for c in "${JAVA:-}" "$(command -v java || true)" \
           "$HOME"/.local/share/jdk/*/Contents/Home/bin/java \
           "$HOME"/.local/share/jdk/*/bin/java; do
    [[ -n "$c" ]] || continue
    major=$(java_major "$c") || continue
    if [[ -n "$major" && "$major" -ge 17 ]]; then echo "$c"; return 0; fi
  done
  return 1
}

[[ -f "$jar" ]] || { echo "alloy jar not found: $jar (set ALLOY_JAR)" >&2; exit 2; }
[[ -f "$model" ]] || { echo "model not found: $model" >&2; exit 2; }
java_bin=$(pick_java) || {
  echo "no Java 17+ found (tried \$JAVA, PATH, ~/.local/share/jdk/*); set \$JAVA" >&2
  exit 2
}

commands=$(grep -oE '^(check|run) +[A-Za-z_][A-Za-z0-9_]*' "$model" | awk '{print $2}')
[[ -n "$commands" ]] || { echo "no commands found in $model" >&2; exit 2; }

fail=0
while read -r name; do
  out=$("$java_bin" -jar "$jar" exec -q -c "$name" -o - "$model" 2>&1) || true
  # A JVM that never reached Alloy reports nothing, which is indistinguishable
  # from "no instance".  Abort rather than record it as a result.
  if grep -qE '^(Error|Exception)|UnsupportedClassVersionError|LinkageError' <<<"$out"; then
    echo "alloy failed to run ($java_bin):" >&2; echo "$out" >&2; exit 2
  fi
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
