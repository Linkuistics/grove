#!/usr/bin/env bash
# The one repository model runner.
#
# It has four obligations beyond running commands, and they are what make it a
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
#      defines.  THE SECOND DIRECTION IS CHECKED AGAINST THE MANIFEST, not
#      merely against the obligation SHAPE: `inv_TT_99_invented` is
#      syntactically perfect and answers nothing, and a runner that credits it
#      to a matrix key nothing ever reads has asserted nothing in that
#      direction at all.  A command may cite a CLAIM rather than one of its
#      sub-identities (`TT_24` where the manifest carries `TT-24.a` and
#      `TT-24.b`),
#      which is the same relaxation obligation 4 makes for a matrix row; such a
#      command is real but credits NO cell, and the run reports it rather than
#      losing it silently.
#
#      AND IT REPORTS CONTESTED CELLS, WHICH IS A STATEMENT ABOUT EVIDENCE
#      RATHER THAN ABOUT COVERAGE.  A cell one family ANSWERS while the other
#      DECLARES A GAP is where a transcription hides: the answering family may
#      have imported the machinery the declining family refused to import, in
#      which case its property restates the import and no mutation can kill it.
#      That happened — `TT-24.c`, entry 048 — and the matrix printed
#      `alloy:gap quint:ok`, which reads as the declining family being behind
#      when the declining family was right.  So the run additionally says
#      whether the ANSWERING family carries a control naming that obligation.
#      It is REPORTED, NEVER FATAL: a family may honestly answer what another
#      cannot express, and a control is not always available.  What it buys is
#      that the next reader of the coverage matrix meets the fact.
#   4. ASSERT Q4'S REMOVAL MATRIX IN BOTH DIRECTIONS, PER FAMILY.  The
#      catalogue calls the artifact/transition removal matrix "a runner
#      obligation like any other: a removable artifact with no row fails the
#      run" (*Q4 needs a matrix, not a claim*).  Every artifact the catalogue
#      names has a row; every row names an artifact the catalogue names and
#      cites an obligation the catalogue defines, or `none`, or is declared
#      `abstracted`; and every row's evidence citation RESOLVES.  See *Q4's
#      removal matrix* below for the row shape and for what this cannot check.
#
# The catalogue IS the manifest: the obligation list is read out of
# `docs/specs/semantic-contract.md` rather than transcribed here, so a claim
# added there with no command anywhere fails the run.  Obligation 4 is the same
# principle at a second grain: the ten removable artifacts, and the README that
# holds their matrix, are read out of the catalogue's Q4 paragraph too.
#
# THE MATRIX OBLIGATION RIDES WITH COVERAGE ASSERTION; IT HAS NO FLAG OF ITS
# OWN.  A matrix is owed only once a family's column has closed, and
# `--no-coverage` is already the statement "this family is still being built".
# Splitting the two would let a run assert coverage while excusing the matrix,
# which is the same hole in a second place.  The split that IS made is the one
# the coverage matrix already makes: the catalogue->row direction (a named
# artifact with no row) rides with coverage, and the row->catalogue directions
# (an invented artifact, an invented obligation, a dangling citation) are fatal
# always, exactly as a command naming no obligation is fatal always.  A broken
# row is not an empty cell.
#
# COMMAND CONVENTIONS.  Each family keeps the dialect of the
# `docs/ordinal-fs-tree/models/` runner it inherits from, extended with the two
# controls the assumption table needs.  Alloy:
#
#   check <OB>_<mnemonic>              must find NO counterexample
#   run   witness_<OB>_<mnemonic>      must find an instance
#   check expect_fail_<EN>_<OB>_<m>    must find a counterexample   (premise-break)
#   run   expect_unreachable_<EN>_<m>  must find NO instance        (exercise-removal)
#
# Quint, whose runner spells the same two things `inv`/`wit`:
#
#   val inv_<OB>_<mnemonic>            must HOLD
#   val wit_<OB>_<mnemonic>            must be REACHED
#   val inv_fail_<EN>_<OB>_<m>         must be VIOLATED             (premise-break)
#   val wit_unreach_<EN>_<m>           must NOT be reached          (exercise-removal)
#
# where <OB> is an obligation spelled without its separators: `TT-02.b` is
# `TT_02b`, and a claim with no sub-identities is `TT_03`.  The two inverted
# forms are inverted deliberately: an assumption mutation whose control is "this
# named obligation fails" cannot be reported by a runner that treats every
# failing check as a defect.
#
# WHICH MODULE A QUINT COMMAND RUNS IN, because `quint run` needs one and a
# `.qnt` file holds several.  A module carrying `const` declarations is a
# LIBRARY — it cannot be run, it is instantiated.  Every other module is an
# instance, and a command runs in the module it is TEXTUALLY defined in.
#
# THE MODULE RULE, in one statement.  This is the only place it is DEFINED; the
# model headers and each scope README cite it rather than restating it, because
# a convention stated in four places is a convention with four versions.
#
#   - a `relax_`, `mutant_` or `scenario_` instance carries ONLY the commands
#     written inside it.  Each exists precisely because some obligation behaves
#     differently there, so inheriting the library's would assert the opposite
#     of what the instance is for.
#   - a `verify_` instance is MODEL-CHECKED (see QUINT VERIFICATION below) and
#     inherits the library's PROPERTY commands only.  It is the one prefix that
#     inherits, which is why a `verify_` module declaring no commands of its own
#     is correct rather than zero-work.
#   - every other instance inherits ALL the library's commands and must satisfy
#     every one of them.
#
# QUINT VERIFICATION.  A module named `verify_<something>` is MODEL-CHECKED with
# `quint verify` (Apalache) rather than simulated, and inherits the library's
# property commands only — a witness is a reachability question, and a reduced
# verification world is the wrong place to ask one.  It is OFF by default
# (`QUINT_VERIFY=0`), because on this repository's subject it is reachable and
# not affordable; every line then says SKIP rather than passing silently, and
# `QUINT_VERIFY=1` runs it.  Whatever a scope's Quint models can and cannot
# check is declared in its README's `VERIFY` line, which
# this runner READS AND PRINTS on every run, and a scope whose Quint models
# exist and which declares nothing is a runner failure — so a limit on model
# checking names itself instead of passing as silence.
#
# An out-of-heap, a backend that never started and a reporter that crashed all
# print what a violated invariant prints if nobody looks.  They abort the run as
# DEAD TOOL rather than being recorded as verdicts.  THE DEFAULT IS DEATH, NOT
# GREEN: a non-zero `quint verify` becomes a verdict only when the output
# carries Apalache's own counterexample report, and every other non-zero exit
# — a JVM that could not read its jar, an OOM whose wording nobody predicted,
# a reporter that blew up — aborts.  A list of known-fatal strings is the wrong
# shape for this: it makes the UNRECOGNISED failure the green one.
#
# Usage:
#   models/run.sh [--scope task-tree|finish|lifecycle|ordinal]...
#                 [--family alloy|quint] [--no-coverage] [--list] [--quiet]
#
# Quint knobs, all env: QUINT_SAMPLES (default 8000), QUINT_STEPS (default 24),
# QUINT_SEED (default fixed, so a run is replayable), QUINT_VERIFY (default 0),
# QUINT_VERIFY_STEPS (default 4), JVM_ARGS (default -Xmx16G, for Apalache).
#
# With no --scope the run is the whole repository and coverage is asserted over
# the whole catalogue; a run that names a subset asserts coverage over exactly
# that subset.  --no-coverage runs the commands and reports both matrices — the
# coverage matrix and Q4's removal matrix — without making an empty cell or an
# unrowed artifact fatal: that is what a scope still being built uses, and the
# model README says which obligations it claims so far.  It never excuses a
# BROKEN row; see obligation 4.
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
    -h|--help) awk 'NR > 1 { if (!/^#/) exit; print }' "$0"; exit 0 ;;
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

# ---------------------------------------------------------------------------
# Q4's removal-matrix manifest, read out of the catalogue for the same reason
# the obligation manifest is: the catalogue names the ten removable artifacts in
# one sentence, and names the README that holds their matrix in the sentence
# before it (*Q4 needs a matrix, not a claim*).  Transcribing either here would
# make an artifact the catalogue added invisible to the runner, which is the
# whole failure this obligation exists to catch.
#
# Extraction is a match rather than a parse, outside the fences, over the
# catalogue joined into one line — the list spans a line break.
# ---------------------------------------------------------------------------
catalogue_flat=$(awk '
  /^```/ { fence = !fence; next }
  fence  { next }
  { buf = buf " " $0 }
  END    { print buf }' "$catalogue")

q4_file=$(grep -oE 'recorded in `[^`]+/README\.md`' <<<"$catalogue_flat" | head -1 | tr -d '`' | sed 's/^recorded in //')
q4_list=$(grep -oE 'one row per removable artifact or transition — [^—]+ —' <<<"$catalogue_flat" | head -1 |
          sed -E 's/^one row per removable artifact or transition — //; s/ —$//')
[[ -n "$q4_file" && -n "$q4_list" ]] || {
  echo "runner error: the catalogue yielded no removal matrix (no artifact list, or no README named)" >&2; exit 2; }

IFS=',' read -r -a q4_artifacts <<<"$q4_list"
for i in "${!q4_artifacts[@]}"; do
  a="${q4_artifacts[$i]}"; a="${a#"${a%%[![:space:]]*}"}"; a="${a%"${a##*[![:space:]]}"}"
  q4_artifacts[$i]="$a"
done
[[ ${#q4_artifacts[@]} -ge 2 ]] || { echo "runner error: the catalogue's removal-matrix list did not split" >&2; exit 2; }

# The scope that owes the matrix is whichever one owns the README the catalogue
# named; a matrix recorded somewhere no scope reaches is a runner error.
q4_scope=""
for s_ in "${all_scopes[@]}"; do
  [[ "$(scope_dir "$s_")/README.md" == "$q4_file" ]] && { q4_scope="$s_"; break; }
done
[[ -n "$q4_scope" ]] || { echo "runner error: the catalogue records the removal matrix in $q4_file, which is in no known scope" >&2; exit 2; }

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
declare -a bad_commands=()  # commands naming no obligation the catalogue defines
declare -a claim_level=()   # commands citing a claim rather than an obligation
declare -A covered_prop=() covered_wit=() has_control=()

note() { [[ "$quiet" == 1 ]] || echo "$@"; }

# obligation token out of a command name, or empty.
# Strips the outcome prefix, then reads a leading TT_nn[a]/FN_../SY_.. token.
strip_outcome() {
  local n="$1"
  # Order matters: the compound prefixes must come off before the bare ones.
  n="${n#witness_}"; n="${n#expect_fail_}"; n="${n#expect_unreachable_}"
  n="${n#inv_fail_}"; n="${n#wit_unreach_}"; n="${n#inv_}"; n="${n#wit_}"
  echo "$n"
}

ob_of() {
  local n
  n=$(strip_outcome "$1")
  [[ "$n" =~ ^(TT|FN|SY)_([0-9][0-9])([a-z]?)(_|$) ]] || { echo ""; return; }
  local id="${BASH_REMATCH[1]}-${BASH_REMATCH[2]}"
  [[ -n "${BASH_REMATCH[3]}" ]] && id="$id.${BASH_REMATCH[3]}"
  echo "$id"
}
is_control() { [[ "$(strip_outcome "$1")" == EN_* ]]; }

# THE OBLIGATION A CONTROL NAMES, or empty.  A control is not coverage and never
# credits a cell; what it is evidence of is that the obligation it names can be
# made to FAIL, which is the one thing a green property command cannot say about
# itself.  Two shapes carry an obligation — `<EN_nn>_<OB>_<m>` for a premise
# break and `MUT_<OB>_<m>` for a model mutation — and two do not
# (`expect_unreachable_<EN>_<m>`, `wit_unreach_<EN>_<m>`), which is correct:
# those are stated over a removed dimension rather than over one obligation.
control_ob() {
  local n
  n=$(strip_outcome "$1")
  [[ "$n" =~ ^(EN_[0-9][0-9]|MUT)_(TT|FN|SY)_([0-9][0-9])([a-z]?)(_|$) ]] || { echo ""; return; }
  local id="${BASH_REMATCH[2]}-${BASH_REMATCH[3]}"
  [[ -n "${BASH_REMATCH[4]}" ]] && id="$id.${BASH_REMATCH[4]}"
  echo "$id"
}

note_control() {
  local fam="$1" cob
  cob=$(control_ob "$2")
  [[ -n "$cob" ]] && has_control["$fam $cob"]=1
  return 0
}

# THE SECOND DIRECTION, and the whole of it.  `ob_of` reads a well-SHAPED
# obligation out of a command name; the catalogue decides whether that
# obligation EXISTS.  Sets `RESOLVED_OB` to the obligation this command credits,
# or to the empty string, and records every rejection where the run can see it —
# by assignment rather than by `echo`, because `$( )` runs a subshell and every
# `bad_commands+=` inside one is discarded the moment it returns.
RESOLVED_OB=""
resolve_ob() {
  local fam="$1" file="$2" name="$3" ob stripped
  RESOLVED_OB=""
  ob=$(ob_of "$name")
  stripped=$(strip_outcome "$name")
  if [[ -z "$ob" ]]; then
    if ! is_control "$name" && [[ "$stripped" =~ ^(TT|FN|SY)_ ]]; then
      bad_commands+=("$fam $file $name — names no obligation")
    fi
    return 0
  fi
  if grep -qxF "$ob" <<<"$manifest"; then RESOLVED_OB="$ob"; return 0; fi
  # A claim rather than one of its sub-identities: real, and credits no cell.
  if grep -q "^${ob}\.[a-z]$" <<<"$manifest"; then
    claim_level+=("$fam $file $name — cites the claim $ob, not one of its obligations")
    return 0
  fi
  bad_commands+=("$fam $file $name — names $ob, which the catalogue does not define")
  return 0
}

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
    note_control alloy "$name"
    local ob; resolve_ob alloy "$file" "$name"; ob="$RESOLVED_OB"
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
# Quint.  Same two obligations as the Alloy driver, in Quint's own dialect.
#
# ABORT ON A DEAD TOOL is the reason for `quint_probe`: a `quint` that cannot
# launch prints to stderr and exits non-zero, which is indistinguishable from a
# model whose witness never landed unless someone asks the tool first.
# ---------------------------------------------------------------------------

quint_samples="${QUINT_SAMPLES:-8000}"
quint_steps="${QUINT_STEPS:-24}"
# A FIXED default seed, so a green run is replayable and a red one is
# reproducible from the line the runner prints rather than from a screenshot.
quint_seed="${QUINT_SEED:-0x5e0a51d3c0ffee01}"
quint_verify="${QUINT_VERIFY:-0}"
quint_verify_steps="${QUINT_VERIFY_STEPS:-4}"
# Apalache is given a heap by default because the default one is not enough for
# this subject and an OOM is a DEAD TOOL, not a verdict.
quint_jvm_args="${JVM_ARGS:--Xmx16G}"
quint_probed=0

quint_probe() {
  [[ "$quint_probed" == 1 ]] && return 0
  command -v quint >/dev/null || { echo "quint not on PATH" >&2; exit 2; }
  local out
  out=$(quint --version 2>&1) || { echo "quint failed to launch:" >&2; echo "$out" >&2; exit 2; }
  note "   quint $out, samples=$quint_samples steps=$quint_steps seed=$quint_seed"
  quint_probed=1
}

# A skipped verification must NAME ITSELF.  The declaration lives in the scope
# README beside the models it is about, in one fixed shape:
#
#   - **VERIFY** quint (<reason-class>) — <reason>
#
# so that a scope whose Quint column is complete and which declares nothing
# fails the run rather than quietly checking nothing but simulation.
report_verify_declaration() {
  local scope="$1" rm_file line
  rm_file="$repo/$(scope_dir "$scope")/README.md"
  line=$(awk '
      /^```/ { fence = !fence; next }
      fence  { next }
      /^- \*\*VERIFY\*\* quint / { print; exit }
    ' "$rm_file" 2>/dev/null || true)
  if [[ -n "$line" ]]; then
    echo "   VERIFICATION, declared: ${line#- \*\*VERIFY\*\* quint }"
    [[ "$quint_verify" == 0 ]] && echo "   (QUINT_VERIFY=0: model checking SKIPPED this invocation)"
    true
  else
    echo "runner error: $scope/quint declares no VERIFY line in $(scope_dir "$scope")/README.md;" >&2
    echo "              a skipped verification step must name itself" >&2
    fail=1
  fi
}

# A `verify_` instance is model-checked rather than simulated, and inherits the
# library's PROPERTY commands only: a witness is a reachability question and a
# reduced verification world is the wrong place to ask it.
quint_run_verify() {
  local file="$1" mod="$2"; shift 2
  local -a names=("$@") n
  (( ${#names[@]} )) || return 0
  if [[ "$quint_verify" == 0 ]]; then
    for n in "${names[@]}"; do
      printf 'SKIP  %-58s %s\n' "$n" "model checking skipped (QUINT_VERIFY=0)"
    done
    return 0
  fi
  local out rc=0
  out=$(JVM_ARGS="$quint_jvm_args" quint verify "$file" --main="$mod" \
          --invariants "${names[@]}" --max-steps="$quint_verify_steps" 2>&1) || rc=$?

  # A DEAD TOOL IS NOT A RESULT, AND AN UNRECOGNISED FAILURE IS DEATH.
  #
  # Measured against quint 0.32.0 / Apalache: a run that genuinely found a
  # counterexample exits non-zero AND prints its own report — `[violation] Found
  # an issue`, one `❌ <invariant>` line per violated invariant, and
  # `error: found a counterexample`.  A run whose backend died prints something
  # else entirely: a JVM stack trace out of `LauncherHelper` when the heap is
  # too small to read the jar, an OOM, a reporter `RangeError`, a socket that
  # was never answered.
  #
  # So the test is for the VERDICT, not for a list of known deaths.  A list of
  # known-fatal strings makes the failure nobody predicted the GREEN one, which
  # is the exact defect this replaces: `JVM_ARGS=-Xmx6m` matched none of the
  # five strings it used to look for, exited 1, and every invariant in the batch
  # was recorded "model-checked … no counterexample".
  local verdict=0
  grep -qE '\[violation\] Found an issue|error: found a counterexample' <<<"$out" && verdict=1
  # A zero exit with no verdict line is a completed check with nothing found;
  # a zero exit that printed a violation report is a contradiction, so it is
  # also refused rather than being read either way.
  if [[ "$rc" == 0 && "$verdict" == 1 ]]; then
    echo "quint verify on $file / $mod exited 0 while reporting a counterexample; refusing to guess:" >&2
    grep -E '\[violation\]|❌|^error' <<<"$out" | head -5 >&2
    exit 2
  fi
  if [[ "$rc" != 0 && "$verdict" == 0 ]]; then
    echo "quint verify failed to complete on $file / $mod (tool failure, not a result; exit $rc):" >&2
    tail -8 <<<"$out" >&2
    echo "      replay: JVM_ARGS=$quint_jvm_args quint verify $file --main=$mod --invariants ${names[*]} --max-steps=$quint_verify_steps" >&2
    exit 2
  fi

  ran=$((ran + ${#names[@]}))
  if [[ "$verdict" == 0 ]]; then
    for n in "${names[@]}"; do
      quint_pass "$n" "model-checked to depth $quint_verify_steps, no counterexample"
    done
  else
    # Attribution is read off Apalache's own `❌ <name>` lines, anchored, rather
    # than by asking whether the name appears anywhere in the output: one
    # command name is a substring of another the moment two mnemonics share a
    # stem, and a substring match would then fail the wrong command.
    for n in "${names[@]}"; do
      if grep -qE "^[[:space:]]*❌[[:space:]]+${n}[[:space:]]*$" <<<"$out"; then
        quint_fail "$n" "counterexample found by quint verify"
      else quint_pass "$n" "model-checked to depth $quint_verify_steps, no counterexample"; fi
    done
    echo "      replay: JVM_ARGS=$quint_jvm_args quint verify $file --main=$mod --invariants <name> --max-steps=$quint_verify_steps"
  fi
}

quint_kind() {
  case "$1" in
    inv_fail_*)    echo violate ;;
    wit_unreach_*) echo unreached ;;
    inv_*)         echo hold ;;
    wit_*)         echo reached ;;
    *)             echo unknown ;;
  esac
}

# Record one command against the coverage matrix and the placement rule.
quint_account() {
  local name="$1" file="$2" scope="$3" ob obscope
  note_control quint "$name"
  resolve_ob quint "$file" "$name"; ob="$RESOLVED_OB"
  [[ -n "$ob" ]] || return 0
  obscope=$(prefix_scope "${ob:0:2}")
  if [[ "$obscope" != "$scope" ]]; then
    echo "placement error: $file ($scope) carries $name, which answers $ob (scope $obscope)" >&2
    fail=1
  fi
  case "$(quint_kind "$name")" in
    hold)     covered_prop["quint $ob"]=1 ;;
    reached)  covered_wit["quint $ob"]=1 ;;
    *)        : ;;   # controls prove nothing about coverage
  esac
}

quint_pass() { note "$(printf 'PASS  %-58s %s' "$1" "$2")"; }
quint_fail() { printf 'FAIL  %-58s %s\n' "$1" "$2"; fail=1; }

# ONE SIMULATION, CLASSIFIED.  Runs `quint run` for one invariant batch and
# answers only "held" (0) or "violated" (1); anything else ABORTS, because the
# third answer a shell sees — a non-zero exit — is the same one a dead tool
# gives, and `quint_run_violations` below records a non-zero exit as a control
# PASSING.  Measured against quint 0.32.0: a real violation prints
# `error: Invariant violated` and exits 1; a tool that could not run prints its
# own error (`[QNT405] Main module not found`, a parse error, a missing file)
# and exits 1 too.  Only the first is a result.
quint_simulate() {
  local file="$1" mod="$2"; shift 2
  local -a names=("$@")
  local out rc=0 flag=--invariants
  (( ${#names[@]} == 1 )) && flag=--invariant
  out=$(quint run "$file" --main="$mod" "$flag" "${names[@]}" \
          --max-steps="$quint_steps" --max-samples="$quint_samples" \
          --seed="$quint_seed" --verbosity=0 2>&1) || rc=$?
  [[ "$rc" == 0 ]] && return 0
  if grep -qE '^error: Invariant violated' <<<"$out"; then return 1; fi
  echo "quint run failed to complete on $file / $mod (tool failure, not a result; exit $rc):" >&2
  tail -5 <<<"$out" >&2
  echo "      replay: quint run $file --main=$mod $flag ${names[*]} --max-steps=$quint_steps --max-samples=$quint_samples --seed=$quint_seed --verbosity=3" >&2
  exit 2
}

# One `quint run` per (module, mode).  Invariants are batched — that is what
# makes an 8000-sample suite finish — and attributed individually only when the
# batch goes red, exactly as `docs/ordinal-fs-tree/models/run-quint.sh` does.
quint_run_invariants() {
  local file="$1" mod="$2"; shift 2
  local -a names=("$@") n
  (( ${#names[@]} )) || return 0
  if quint_simulate "$file" "$mod" "${names[@]}"; then
    for n in "${names[@]}"; do quint_pass "$n" "holds"; ran=$((ran + 1)); done
  else
    for n in "${names[@]}"; do
      ran=$((ran + 1))
      if quint_simulate "$file" "$mod" "$n"; then
        quint_pass "$n" "holds"
      else
        quint_fail "$n" "violated"
        echo "      replay: quint run $file --main=$mod --invariant $n --max-steps=$quint_steps --max-samples=$quint_samples --seed=$quint_seed --verbosity=3"
      fi
    done
  fi
}

# A premise-break control is inverted: it must go RED, and a green one means the
# assumption was carrying no weight — which is the finding, not a pass.
quint_run_violations() {
  local file="$1" mod="$2"; shift 2
  local -a names=("$@") n
  for n in "${names[@]}"; do
    ran=$((ran + 1))
    if quint_simulate "$file" "$mod" "$n"; then
      quint_fail "$n" "HELD — the mutated assumption broke no obligation"
    else
      quint_pass "$n" "violated, as the control requires"
    fi
  done
}

# Witnesses and unreachability controls share one run: both are read off the
# per-name trace counts, and the only difference is which count is a pass.
quint_run_witnesses() {
  local file="$1" mod="$2"; shift 2
  local -a names=("$@") n
  (( ${#names[@]} )) || return 0
  local out line count
  out=$(quint run "$file" --main="$mod" --witnesses "${names[@]}" \
          --max-steps="$quint_steps" --max-samples="$quint_samples" \
          --seed="$quint_seed" --verbosity=1 2>&1) || true
  if grep -qE '^(error|Error)' <<<"$out" && ! grep -q 'was witnessed' <<<"$out"; then
    echo "quint failed to run on $file / $mod:" >&2; echo "$out" >&2; exit 2
  fi
  for n in "${names[@]}"; do
    ran=$((ran + 1))
    line=$(grep -E "^$n was witnessed" <<<"$out" || true)
    count=$(sed -E 's/.* in ([0-9]+) trace.*/\1/' <<<"$line")
    [[ -n "$line" ]] || { quint_fail "$n" "not reported by the run"; continue; }
    if [[ "$(quint_kind "$n")" == unreached ]]; then
      if [[ "${count:-1}" -eq 0 ]]; then
        quint_pass "$n" "unreached in $quint_samples samples, as the control requires"
      else
        quint_fail "$n" "REACHED in $count trace(s) — the removed dimension was not the one exercising it"
      fi
    else
      if [[ "${count:-0}" -gt 0 ]]; then
        quint_pass "$n" "reached in $count trace(s)"
      else
        quint_fail "$n" "never reached in $quint_samples samples"
        echo "      replay: quint run $file --main=$mod --witnesses $n --max-steps=$quint_steps --max-samples=$quint_samples --seed=$quint_seed --verbosity=1"
      fi
    fi
  done
}

run_quint_file() {
  local file="$1" scope="$2"
  local parsed lib="" name mod kind
  # Modules, their `const`-ness, and the commands textually inside each.
  parsed=$(awk '
    /^module [A-Za-z_][A-Za-z0-9_]*/ {
      mod = $2; sub(/\{$/, "", mod); order[++n] = mod; hasconst[mod] = hasconst[mod] + 0; next }
    /^  const / { hasconst[mod] = 1; next }
    /^  val (inv|wit)_[A-Za-z0-9_]*:/ {
      cmd = $2; sub(/:$/, "", cmd); print "CMD " mod " " cmd; next }
    END { for (i = 1; i <= n; i++) print "MOD " order[i] " " hasconst[order[i]] }
  ' "$file")

  local -a instances=()
  while read -r _ mod flag; do
    [[ -n "$mod" ]] || continue
    if [[ "$flag" == 1 ]]; then lib="$mod"; else instances+=("$mod"); fi
  done < <(grep '^MOD ' <<<"$parsed")

  (( ${#instances[@]} )) || { echo "runner error: $file declares no runnable instance (zero work)" >&2; fail=1; return; }

  # A file MAY carry no library — the controls for a model live in a file of
  # their own so that `quint verify` is not asked to serialise them, and they
  # import their library across files.  Such a file's instances carry only their
  # own commands, which is what they would carry anyway: every one of them is
  # prefixed, and a prefixed instance never inherits.
  local -a lib_cmds=()
  if [[ -n "$lib" ]]; then
    while read -r _ mod name; do
      [[ "$mod" == "$lib" ]] && lib_cmds+=("$name")
    done < <(grep '^CMD ' <<<"$parsed")
    (( ${#lib_cmds[@]} )) || { echo "runner error: $file's library declares no commands (zero work)" >&2; fail=1; return; }
  fi

  for mod in "${instances[@]}"; do
    local -a own=() cmds=() invs=() wits=() viol=()
    while read -r _ m name; do [[ "$m" == "$mod" ]] && own+=("$name"); done < <(grep '^CMD ' <<<"$parsed")
    if [[ "$mod" =~ ^verify_ ]]; then
      (( ${#lib_cmds[@]} )) || { echo "runner error: $file / $mod is a verification instance in a file with no library" >&2; fail=1; continue; }
      local -a props=()
      for name in "${lib_cmds[@]}"; do
        [[ "$(quint_kind "$name")" == hold ]] && props+=("$name")
      done
      note "-- $(basename "$file")::$mod (${#props[@]} properties, model-checked)"
      quint_run_verify "$file" "$mod" "${props[@]}"
      unset props
      continue
    fi
    if [[ "$mod" =~ ^(relax_|mutant_|scenario_) ]]; then
      cmds=("${own[@]}")
    elif (( ${#lib_cmds[@]} )); then
      cmds=("${lib_cmds[@]}" "${own[@]}")
    else
      echo "runner error: $file / $mod is an inheriting instance in a file with no library" >&2
      fail=1; continue
    fi
    (( ${#cmds[@]} )) || { echo "runner error: $file / $mod has no commands (zero work)" >&2; fail=1; continue; }
    note "-- $(basename "$file")::$mod (${#cmds[@]} commands)"
    for name in "${cmds[@]}"; do
      quint_account "$name" "$file" "$scope"
      case "$(quint_kind "$name")" in
        hold)                 invs+=("$name") ;;
        violate)              viol+=("$name") ;;
        reached|unreached)    wits+=("$name") ;;
        *) echo "runner error: $file / $mod carries $name, which is neither inv_ nor wit_" >&2; fail=1 ;;
      esac
    done
    quint_run_invariants "$file" "$mod" "${invs[@]}"
    quint_run_violations "$file" "$mod" "${viol[@]}"
    quint_run_witnesses  "$file" "$mod" "${wits[@]}"
    unset own cmds invs wits viol
  done
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
        quint_probe
        report_verify_declaration "$scope"
        for f in "${files[@]}"; do note "== $scope/quint $(basename "$f")"; run_quint_file "$f" "$scope"; done ;;
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
# Q4's removal matrix, read out of the scope README the catalogue named.  Shape,
# beside the GAP shape above — one row per (family, removable artifact):
#
#   | Q4-<n> | <family> | the **<artifact>**[, <gloss>] | <obligation> | <evidence> |
#
#   <obligation>  a backticked obligation the catalogue defines, or `none`, or
#                 empty (an em dash) when the row is `abstracted`
#   <evidence>    opens with its class — `mutation`, `argument` or `abstracted`
#                 — and a `mutation` names the mutation-matrix row it fired:
#                 "mutation — row 17", "mutation — row x2"
#
# The artifact cell is keyed on its leading `the`/`its` plus the bolded name, so
# the gloss after the comma is free prose; the key is what the catalogue's own
# sentence says, character for character.  Rows of families this run did not
# select are ignored, exactly as their commands were not run.
#
# WHAT THIS CANNOT CHECK, STATED PLAINLY.  A row naming the wrong BUT REAL
# obligation reports identically to a right one.  No runner can decide *first
# broken*; that is what the mutation discipline is for.  What a reader reaches
# instead is the citation — it can check that the cited mutation-matrix row
# EXISTS, never that its kill was the first.
# ---------------------------------------------------------------------------
declare -A q4_ob=() q4_class=() q4_rowid=()
declare -a q4_errors=()
q4_readme=""

q4_in_scope=0
for want in "${scopes[@]}"; do [[ "$want" == "$q4_scope" ]] && q4_in_scope=1 && break; done

if [[ "$q4_in_scope" == 1 ]]; then
  q4_readme="$repo/$q4_file"
  if [[ ! -f "$q4_readme" ]]; then
    echo "removal matrix: $q4_file does not exist, so no family has one" >&2
    [[ "$coverage" == 1 ]] && fail=1
  else
    # The mutation-matrix row ids a `mutation` citation may resolve to: the
    # leading cell of every table row under the README's *mutation matrix*
    # heading, up to the next same-level heading.
    mutation_rows=$(awk '
      /^```/                 { fence = !fence; next }
      fence                  { next }
      /^## /                 { inmm = (tolower($0) ~ /mutation matrix/) ; next }
      !inmm                  { next }
      /^\| *x?[0-9]+ *\|/    { match($0, /x?[0-9]+/); print substr($0, RSTART, RLENGTH) }
    ' "$q4_readme")

    has_abstractions=0
    grep -qiE '^#+ .*abstractions' "$q4_readme" && has_abstractions=1

    # \037 rather than a tab: TAB is IFS whitespace, so `read` collapses a run
    # of them and an empty obligation cell — which is exactly what an
    # `abstracted` row has — silently shifts every later field left.
    while IFS=$'\037' read -r rid rfam rart rob rclass rcite; do
      [[ -n "$rid" ]] || continue
      sel=0; for f in "${families[@]}"; do [[ "$f" == "$rfam" ]] && sel=1 && break; done
      if [[ "$sel" == 0 ]]; then
        case "$rfam" in
          alloy|quint) continue ;;   # a real family this run did not select
          *) q4_errors+=("row $rid names family '$rfam', which is not a model family"); continue ;;
        esac
      fi
      if [[ -n "${q4_rowid[$rfam $rart]:-}" ]]; then
        q4_errors+=("row $rid repeats $rfam's row for '$rart' (already ${q4_rowid[$rfam $rart]})")
        continue
      fi
      q4_rowid["$rfam $rart"]="$rid"
      q4_ob["$rfam $rart"]="$rob"
      q4_class["$rfam $rart"]="$rclass"

      # direction: every row names an artifact the catalogue names
      known=0
      for a in "${q4_artifacts[@]}"; do [[ "$a" == "$rart" ]] && known=1 && break; done
      [[ "$known" == 1 ]] || q4_errors+=("row $rid ($rfam) names '$rart', which the catalogue does not name as removable")

      # direction: every row's cited obligation is one the catalogue defines,
      # or `none`, or the row is a declared `abstracted`
      case "$rob" in
        none) : ;;
        "")   [[ "$rclass" == abstracted ]] ||
                q4_errors+=("row $rid ($rfam, '$rart') names no obligation and is not declared abstracted") ;;
        *)    # A row may cite a CLAIM rather than one of its sub-identities: the
              # register's shared-safety list names `TT-24`, and the manifest —
              # whose unit is the pair `(family, obligation)` — carries only
              # its lettered sub-identities.  `Q4-6` was that case; it now cites
              # `TT-24.a` directly, and the relaxation stays for the next one.
              grep -qxF "$rob" <<<"$manifest" || grep -q "^${rob}\\.[a-z]$" <<<"$manifest" ||
                q4_errors+=("row $rid ($rfam, '$rart') cites $rob, which the catalogue does not define") ;;
      esac

      # direction: every row's evidence citation resolves
      case "$rclass" in
        mutation)
          if [[ -z "$rcite" ]]; then
            q4_errors+=("row $rid ($rfam, '$rart') is evidenced by a mutation and cites no mutation-matrix row")
          elif ! grep -qxF "$rcite" <<<"$mutation_rows"; then
            q4_errors+=("row $rid ($rfam, '$rart') cites mutation row $rcite, which $q4_file does not have")
          fi ;;
        argument)   : ;;
        abstracted)
          [[ "$has_abstractions" == 1 ]] ||
            q4_errors+=("row $rid ($rfam, '$rart') is declared abstracted and $q4_file has no Abstractions section") ;;
        *)
          q4_errors+=("row $rid ($rfam, '$rart') carries no evidence class (mutation, argument or abstracted)") ;;
      esac
    done < <(awk -F'|' '
      BEGIN { US = sprintf("%c", 31) }
      function trim(x) { gsub(/^[[:space:]]+|[[:space:]]+$/, "", x); return x }
      /^```/ { fence = !fence; next }
      fence  { next }
      /^\| *Q4-[0-9]+ *\| *[A-Za-z]+ *\|/ {
        id = trim($2); fam = trim($3); art = $4; ob = $5; ev = $6
        akey = ""
        if (match(art, /(the|its) \*\*[^*]+\*\*/)) { akey = substr(art, RSTART, RLENGTH); gsub(/\*/, "", akey) }
        okey = ""
        if (match(ob, /`(TT|FN|SY)-[0-9][0-9](\.[a-z])?`/)) { okey = substr(ob, RSTART, RLENGTH); gsub(/`/, "", okey) }
        else if (ob ~ /`none`/) { okey = "none" }
        cls = ""; cite = ""
        if (match(ev, /mutation|argument|abstracted/)) cls = substr(ev, RSTART, RLENGTH)
        if (cls == "mutation" && match(ev, /row x?[0-9]+/)) { cite = substr(ev, RSTART + 4, RLENGTH - 4) }
        print id US fam US akey US okey US cls US cite
      }' "$q4_readme")

    # direction: every artifact the catalogue names has a row, per family
    echo
    echo "-- Q4 removal matrix ($q4_scope), per (family, artifact) — ${#q4_artifacts[@]} artifacts named by the catalogue"
    for fam in "${families[@]}"; do
      have=0; none=0; abst=0
      declare -a absent=()
      for a in "${q4_artifacts[@]}"; do
        if [[ -n "${q4_rowid[$fam $a]:-}" ]]; then
          have=$((have + 1))
          [[ "${q4_ob[$fam $a]}" == none ]] && none=$((none + 1))
          [[ "${q4_class[$fam $a]}" == abstracted ]] && abst=$((abst + 1))
        else
          absent+=("$a")
        fi
      done
      printf '   %-6s %d of %d rows' "$fam" "$have" "${#q4_artifacts[@]}"
      [[ "$none" -gt 0 ]] && printf ', %d `none`' "$none"
      [[ "$abst" -gt 0 ]] && printf ', %d abstracted' "$abst"
      echo
      if [[ ${#absent[@]} -gt 0 ]]; then
        for a in "${absent[@]}"; do echo "      NO ROW  $fam — $a"; done
        if [[ "$coverage" == 1 ]]; then fail=1
        else echo "      (matrix not asserted (--no-coverage); ${#absent[@]} unrowed artifacts are this family's remaining work)"; fi
      fi
      unset absent
    done
    if [[ ${#q4_errors[@]} -gt 0 ]]; then
      echo "   removal matrix, rows that do not resolve:"
      printf '      %s\n' "${q4_errors[@]}"
      fail=1
    fi
  fi
fi

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

# CONTESTED CELLS.  One family answered, another declared a gap.  Reported with
# whether the answering family can make its own answer fail; never fatal.
if [[ ${#families[@]} -gt 1 ]]; then
  contested=0
  contested_uncontrolled=0
  contested_lines=""
  while read -r ob; do
    [[ -n "$ob" ]] || continue
    # NOT `declared`/`answered`: those are the coverage counters above, and this
    # block runs BEFORE the line that prints them.  Reusing either name blanks
    # the gap count in every run — which is how this comment came to exist.
    gapped_by=""; answered_by=""
    for fam in "${families[@]}"; do
      if [[ -n "${gap[$fam $ob]:-}" ]]; then gapped_by+="$fam "
      elif [[ -n "${covered_prop[$fam $ob]:-}" ]]; then answered_by+="$fam "; fi
    done
    [[ -n "$gapped_by" && -n "$answered_by" ]] || continue
    contested=$((contested + 1))
    for fam in $answered_by; do
      if [[ -n "${has_control[$fam $ob]:-}" ]]; then
        contested_lines+="  $ob  ${gapped_by% } declared a gap; $fam answered, and carries a control"$'\n'
      else
        contested_lines+="  $ob  ${gapped_by% } declared a gap; $fam answered with NO CONTROL"$'\n'
        contested_uncontrolled=$((contested_uncontrolled + 1))
      fi
    done
  done <<<"$selected_manifest"
  if [[ "$contested" -gt 0 ]]; then
    echo
    echo "-- contested cells: one family answered what another declared out of reach."
    echo "   Not a failure. An answer no control can kill is a transcription of the"
    echo "   machinery it imported, and that is what this line exists to show."
    printf '%s' "$contested_lines"
    echo "-- $contested contested, of which $contested_uncontrolled have no control on the answering side"
  fi
fi

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
if [[ ${#claim_level[@]} -gt 0 ]]; then
  echo "-- commands citing a claim rather than one of its obligations (credit no cell):"
  printf '     %s\n' "${claim_level[@]}"
fi
[[ "$ran" -gt 0 ]] || { echo "runner error: zero work" >&2; exit 2; }
if [[ "$missing" -gt 0 ]]; then
  if [[ "$coverage" == 1 ]]; then fail=1
  else echo "-- coverage not asserted (--no-coverage); $missing empty cells are the phase's remaining work"; fi
fi

exit "$fail"
