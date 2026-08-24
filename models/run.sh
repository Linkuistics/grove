#!/usr/bin/env bash
# The one repository model runner.
#
# It has three obligations beyond running commands, and they are what make it a
# test seam rather than a convenience script (`docs/specs/semantic-contract.md`,
# *Model paths and the runner*):
#
#   1. ABORT ON A DEAD TOOL.  A tool that failed to launch reports what a tool
#      that found nothing reports.  Launch-failure output is a runner error,
#      never a result.
#   2. FAIL ON ZERO WORK.  A model file no runner reaches, an empty command set,
#      a witness that never lands, and a skipped verification step are each a
#      runner failure that names itself.
#   3. ASSERT OBLIGATION COVERAGE IN BOTH DIRECTIONS, PER FAMILY.  Every
#      obligation the catalogue defines must be answered by each family, and
#      every TT_/FN_/SY_-prefixed command must name an obligation the catalogue
#      defines.
#
# The catalogue IS the manifest: the obligation list is read out of
# `docs/specs/semantic-contract.md` rather than transcribed here, so a claim
# added there with no command anywhere fails the run.
#
# COMMAND CONVENTIONS.  Alloy's are the two `docs/ordinal-fs-tree/models/`
# runners', extended with two controls the assumption table needs:
#
#   check <OB>_<mnemonic>              must find NO counterexample
#   run   witness_<OB>_<mnemonic>      must find an instance
#   check expect_fail_<EN>_<OB>_<m>    must find a counterexample   (premise-break)
#   run   expect_unreachable_<EN>_<m>  must find NO instance        (exercise-removal)
#
# where <OB> is an obligation spelled without its separators: `TT-02.b` is
# `TT_02b`, and a claim with no sub-identities is `TT_03`.  The two `expect_`
# forms are inverted deliberately: an assumption mutation whose control is "this
# named obligation fails" cannot be reported by a runner that treats every
# failing check as a defect.
#
# Usage:
#   models/run.sh [--scope task-tree|finish|lifecycle|ordinal]...
#                 [--family alloy|quint] [--no-coverage] [--list] [--quiet]
#
# With no --scope the run is the whole repository and coverage is asserted over
# the whole catalogue; a run that names a subset asserts coverage over exactly
# that subset.  --no-coverage runs the commands and reports the matrix without
# making an empty cell fatal: that is what a scope still being built uses, and
# the model README says which obligations it claims so far.
set -euo pipefail

# Associative arrays and `mapfile` are bash 4; macOS ships 3.2 as /bin/bash. A
# 3.2 run does not fail, it MISBEHAVES — `declare -A` is a syntax error in some
# positions and a silent scalar in others — which is the class of thing this
# runner exists to refuse rather than to suffer.
[[ ${BASH_VERSINFO[0]:-0} -ge 4 ]] || {
  echo "models/run.sh needs bash 4+; found ${BASH_VERSION:-unknown}" >&2; exit 2; }

here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/.." && pwd)"
catalogue="$repo/docs/specs/semantic-contract.md"

# scope -> directory holding both families' models for that scope
scope_dir() {
  case "$1" in
    task-tree) echo "crates/grove-task-tree/models" ;;
    finish)    echo "crates/grove-finish/models" ;;
    lifecycle) echo "models/system" ;;
    ordinal)   echo "docs/ordinal-fs-tree/models" ;;
    *) return 1 ;;
  esac
}
# obligation prefix -> scope, which is also the placement rule the root brief states
prefix_scope() {
  case "$1" in TT) echo task-tree ;; FN) echo finish ;; SY) echo lifecycle ;; esac
}

all_scopes=(task-tree finish lifecycle ordinal)
scopes=()
families=(alloy quint)
coverage=1
list_only=0
quiet=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --scope)  scope_dir "$2" >/dev/null || { echo "unknown scope: $2" >&2; exit 2; }
              scopes+=("$2"); shift 2 ;;
    --family) case "$2" in alloy|quint) families=("$2") ;; *) echo "unknown family: $2" >&2; exit 2 ;; esac
              shift 2 ;;
    --no-coverage) coverage=0; shift ;;
    --list)   list_only=1; shift ;;
    --quiet)  quiet=1; shift ;;
    -h|--help) sed -n '2,50p' "$0"; exit 0 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[[ ${#scopes[@]} -gt 0 ]] || scopes=("${all_scopes[@]}")

[[ -f "$catalogue" ]] || { echo "catalogue not found: $catalogue" >&2; exit 2; }

# ---------------------------------------------------------------------------
# The manifest, read out of the catalogue.
#
# Fenced blocks are skipped: the catalogue documents the obligation shape by
# showing it, and its own example is otherwise indistinguishable from a real
# obligation.  Extracting it is a match rather than a parse, but only outside
# the fences.
# ---------------------------------------------------------------------------
manifest=$(awk '
  /^```/            { fence = !fence; next }
  fence             { next }
  /^\*\*`(TT|FN|SY)-[0-9][0-9]` —/ {
      match($0, /(TT|FN|SY)-[0-9][0-9]/); c = substr($0, RSTART, RLENGTH)
      order[++n] = c; next }
  /^- `(TT|FN|SY)-[0-9][0-9]\.[a-z]` —/ {
      match($0, /(TT|FN|SY)-[0-9][0-9]\.[a-z]/); ob = substr($0, RSTART, RLENGTH)
      split(ob, p, "."); sc[p[1]]++; subs[p[1]] = subs[p[1]] " " ob; next }
  END {
      for (i = 1; i <= n; i++) {
        c = order[i]
        if (sc[c] == 0) print c
        else { k = split(subs[c], a, " "); for (j = 1; j <= k; j++) if (a[j] != "") print a[j] }
      }
  }' "$catalogue")
[[ -n "$manifest" ]] || { echo "runner error: the catalogue yielded no obligations" >&2; exit 2; }

# obligation identifier -> model spelling: TT-02.b -> TT_02b
spell() { echo "${1//-/_}" | tr -d '.'; }

selected_manifest=""
for ob in $manifest; do
  s=$(prefix_scope "${ob:0:2}")
  for want in "${scopes[@]}"; do [[ "$want" == "$s" ]] && selected_manifest+="$ob "$'\n' && break; done
done
selected_manifest=$(echo "$selected_manifest" | tr -d ' ' | grep -v '^$' || true)

if [[ "$list_only" == 1 ]]; then
  echo "$selected_manifest"
  echo "-- $(echo "$selected_manifest" | grep -c .) obligations in scope"
  exit 0
fi

# ---------------------------------------------------------------------------
# Alloy: find a Java 17+, exactly as `docs/ordinal-fs-tree/models/run-alloy.sh`
# does.  The measurement host's default `java` is below Alloy 6's floor, so this
# probe is the difference between a suite and a broken instrument.
# ---------------------------------------------------------------------------
jar="${ALLOY_JAR:-$HOME/.local/share/alloy/org.alloytools.alloy.dist.jar}"
java_major() { [[ -x "$1" ]] || return 1; "$1" -version 2>&1 | sed -n '1s/.*version "\([0-9]*\).*/\1/p'; }
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

fail=0
ran=0
declare -a rows=()          # "family obligation kind command outcome"
declare -a bad_commands=()  # commands naming no obligation
declare -A covered_prop=() covered_wit=()

note() { [[ "$quiet" == 1 ]] || echo "$@"; }

# obligation token out of a command name, or empty.
# Strips the outcome prefix, then reads a leading TT_nn[a]/FN_../SY_.. token.
ob_of() {
  local n="$1"
  n="${n#witness_}"; n="${n#expect_fail_}"; n="${n#expect_unreachable_}"
  [[ "$n" =~ ^(TT|FN|SY)_([0-9][0-9])([a-z]?)(_|$) ]] || { echo ""; return; }
  local id="${BASH_REMATCH[1]}-${BASH_REMATCH[2]}"
  [[ -n "${BASH_REMATCH[3]}" ]] && id="$id.${BASH_REMATCH[3]}"
  echo "$id"
}
is_control() { local n="$1"; n="${n#witness_}"; n="${n#expect_fail_}"; n="${n#expect_unreachable_}"; [[ "$n" == EN_* ]]; }

run_alloy_file() {
  local file="$1" scope="$2" java_bin="$3"
  local commands
  commands=$(grep -oE '^(check|run) +[A-Za-z_][A-Za-z0-9_]*' "$file" | awk '{print $2}') || true
  if [[ -z "$commands" ]]; then
    echo "runner error: no commands in $file (zero work)" >&2; fail=1; return
  fi
  while read -r name; do
    [[ -n "$name" ]] || continue
    local out found want label
    # -n excludes models in which an arithmetic overflow occurred: position and
    # key allocation are `+ 1` over Int, and an overflow counterexample is a
    # fact about the bitwidth rather than about the claim.
    # -t text because the default table renders a temporal trace as an empty
    # grid — the tool reports a counterexample and shows nothing of it.
    out=$("$java_bin" -jar "$jar" exec -q -n -t text -c "$name" -o - "$file" 2>&1) || true
    if grep -qE '^(Error|Exception)|UnsupportedClassVersionError|LinkageError' <<<"$out"; then
      echo "alloy failed to run ($java_bin) on $file / $name:" >&2; echo "$out" >&2; exit 2
    fi
    if grep -q "^---Trace---" <<<"$out"; then found=yes; else found=no; fi
    case "$name" in
      witness_*)             want=yes; label="instance" ;;
      expect_fail_*)         want=yes; label="counterexample" ;;
      expect_unreachable_*)  want=no;  label="instance" ;;
      *)                     want=no;  label="counterexample" ;;
    esac
    ran=$((ran + 1))
    local ob; ob=$(ob_of "$name")
    if [[ -z "$ob" ]] && ! is_control "$name" && [[ "$name" =~ ^(witness_|expect_fail_|expect_unreachable_)?(TT|FN|SY)_ ]]; then
      bad_commands+=("alloy $file $name")
    fi
    if [[ -n "$ob" ]]; then
      local obscope; obscope=$(prefix_scope "${ob:0:2}")
      if [[ "$obscope" != "$scope" ]]; then
        echo "placement error: $file ($scope) carries $name, which answers $ob (scope $obscope)" >&2
        fail=1
      fi
      case "$name" in
        witness_*)                       covered_wit["alloy $ob"]=1 ;;
        expect_fail_*|expect_unreachable_*) : ;;   # controls prove nothing about coverage
        *)                               covered_prop["alloy $ob"]=1 ;;
      esac
    fi
    if [[ "$found" == "$want" ]]; then
      note "$(printf 'PASS  %-58s %s' "$name" "$([[ $want == yes ]] && echo "$label found" || echo "no $label")")"
    else
      printf 'FAIL  %-58s %s\n' "$name" "$([[ $want == yes ]] && echo "no $label" || echo "$label found")"
      fail=1
    fi
  done <<<"$commands"
}

# ---------------------------------------------------------------------------
# Run the models
# ---------------------------------------------------------------------------
java_bin=""
for scope in "${scopes[@]}"; do
  dir="$repo/$(scope_dir "$scope")"

  if [[ "$scope" == ordinal ]]; then
    # The delegated boundary, and this runner's positive control: those suites
    # are known green, so a repository run that reports them clean while finding
    # nothing anywhere else is reporting a broken instrument.
    [[ -d "$dir" ]] || { echo "runner error: $dir missing" >&2; exit 2; }
    for fam in "${families[@]}"; do
      case "$fam" in
        alloy) note "== ordinal-fs-tree (delegated, alloy)"; "$dir/run-alloy.sh" || fail=1; ran=$((ran + 1)) ;;
        quint) note "== ordinal-fs-tree (delegated, quint)"; "$dir/run-quint.sh" || fail=1; ran=$((ran + 1)) ;;
      esac
    done
    continue
  fi

  if [[ ! -d "$dir" ]]; then
    echo "MISSING SCOPE  $scope — no model directory at $(scope_dir "$scope")"
    [[ "$coverage" == 1 ]] && fail=1
    continue
  fi

  for fam in "${families[@]}"; do
    case "$fam" in
      alloy)
        mapfile -t files < <(find "$dir" -maxdepth 1 -name '*.als' | sort)
        if [[ ${#files[@]} -eq 0 ]]; then
          echo "MISSING MODEL  $scope/alloy — no .als in $(scope_dir "$scope")"
          [[ "$coverage" == 1 ]] && fail=1
          continue
        fi
        [[ -n "$java_bin" ]] || { [[ -f "$jar" ]] || { echo "alloy jar not found: $jar (set ALLOY_JAR)" >&2; exit 2; }
                                  java_bin=$(pick_java) || { echo "no Java 17+ found; set \$JAVA" >&2; exit 2; }; }
        for f in "${files[@]}"; do note "== $scope/alloy $(basename "$f")"; run_alloy_file "$f" "$scope" "$java_bin"; done ;;
      quint)
        mapfile -t files < <(find "$dir" -maxdepth 1 -name '*.qnt' | sort)
        if [[ ${#files[@]} -eq 0 ]]; then
          echo "MISSING MODEL  $scope/quint — no .qnt in $(scope_dir "$scope")"
          [[ "$coverage" == 1 ]] && fail=1
          continue
        fi
        echo "runner error: the quint driver is not built yet — quint-models-k10 extends this runner" >&2
        fail=1 ;;
    esac
  done
done

# ---------------------------------------------------------------------------
# A model file no runner reaches is a runner defect, asserted as such.
# ---------------------------------------------------------------------------
known_dirs=$(for s in "${all_scopes[@]}"; do echo "$repo/$(scope_dir "$s")"; done)
while read -r stray; do
  [[ -n "$stray" ]] || continue
  d=$(dirname "$stray")
  grep -qxF "$d" <<<"$known_dirs" || { echo "runner error: $stray is in no known scope" >&2; fail=1; }
done < <(find "$repo" \( -name target -o -name .jj -o -name .git \) -prune -o \( -name '*.als' -o -name '*.qnt' \) -print)

# ---------------------------------------------------------------------------
# Declared gaps, read out of each scope's README.  Shape:
#   - **GAP** <family> `<obligation>` (<reason-class>) — <reason>
# A declared gap counts as covered for the family that declared it, and is
# reported; a gap declared on BOTH families is a finding about the catalogue and
# is counted separately.
# ---------------------------------------------------------------------------
declare -A gap=()
for scope in "${scopes[@]}"; do
  rm_file="$repo/$(scope_dir "$scope")/README.md"
  [[ -f "$rm_file" ]] || continue
  # Fenced blocks are skipped here for the same reason they are in the
  # catalogue: a README that is also a manifest documents its own line shape by
  # showing it, and the example is otherwise indistinguishable from a real
  # declaration.
  while read -r famname obid; do
    [[ -n "$obid" ]] || continue
    gap["$famname $obid"]=1
  done < <(awk '
      /^```/ { fence = !fence; next }
      fence  { next }
      /^- \*\*GAP\*\* [a-z]+ `[A-Z][A-Z]-[0-9][0-9](\.[a-z])?`/ {
        match($0, /[a-z]+ `[A-Z][A-Z]-[0-9][0-9](\.[a-z])?`/)
        t = substr($0, RSTART, RLENGTH); gsub(/`/, "", t); print t }
    ' "$rm_file")
done

# ---------------------------------------------------------------------------
# The coverage matrix, per (family, obligation), in both directions.
# ---------------------------------------------------------------------------
missing=0; declared=0; both_gap=0; complete=0
echo
echo "-- coverage, per (family, obligation)"
while read -r ob; do
  [[ -n "$ob" ]] || continue
  line=""
  gaps_here=0
  for fam in "${families[@]}"; do
    if [[ -n "${gap[$fam $ob]:-}" ]]; then line+=" $fam:gap"; declared=$((declared + 1)); gaps_here=$((gaps_here + 1))
    elif [[ -n "${covered_prop[$fam $ob]:-}" && -n "${covered_wit[$fam $ob]:-}" ]]; then line+=" $fam:ok"; complete=$((complete + 1))
    elif [[ -n "${covered_prop[$fam $ob]:-}" ]]; then line+=" $fam:NO-WITNESS"; missing=$((missing + 1))
    elif [[ -n "${covered_wit[$fam $ob]:-}" ]]; then line+=" $fam:NO-CHECK"; missing=$((missing + 1))
    else line+=" $fam:MISSING"; missing=$((missing + 1)); fi
  done
  [[ "$gaps_here" -eq ${#families[@]} && ${#families[@]} -gt 1 ]] && both_gap=$((both_gap + 1))
  [[ "$line" == *MISSING* || "$line" == *NO-* || "$line" == *gap* ]] && printf '  %-10s%s\n' "$ob" "$line"
done <<<"$selected_manifest"

total_cells=$(( $(echo "$selected_manifest" | grep -c .) * ${#families[@]} ))
echo
echo "-- commands run: $ran"
echo "-- cells: $complete complete, $declared declared gaps, $missing empty, of $total_cells"
[[ "$both_gap" -gt 0 ]] && echo "-- $both_gap obligations are declared gaps in BOTH families: a finding about the catalogue"
if [[ ${#bad_commands[@]} -gt 0 ]]; then
  echo "-- commands naming no obligation the catalogue defines:"
  printf '     %s\n' "${bad_commands[@]}"
  fail=1
fi
[[ "$ran" -gt 0 ]] || { echo "runner error: zero work" >&2; exit 2; }
if [[ "$missing" -gt 0 ]]; then
  if [[ "$coverage" == 1 ]]; then fail=1
  else echo "-- coverage not asserted (--no-coverage); $missing empty cells are the phase's remaining work"; fi
fi

exit "$fail"
