#!/usr/bin/env bash
# Controls on THE RUNNER, as against controls on the models.
#
# `models/run.sh` is a test seam with four obligations (its own header lists
# them), and every one of them is a claim that the runner goes RED in a
# situation nobody normally creates.  A suite that has never been shown to fail
# is not evidence, and a runner that has never been shown to fail is a green
# tick over nothing — which is the same hazard the model's own `mutant_`
# instances exist to close, one level up.
#
# Each control below MUTATES A COPY of the repository — never the working tree —
# and asserts that the runner emits its named fatal diagnostic.  The copy holds
# the real catalogue and the real task-tree models, so what is exercised is the
# actual accounting path and not a miniature of it.
#
# WHAT THIS DOES AND DOES NOT SHOW, stated plainly.  The controls run at
# `QUINT_SAMPLES=1 QUINT_STEPS=1`, which makes a full accounting pass take about
# a minute instead of twenty.  At that budget the WITNESSES of the unmutated
# suite do not land, so a control's non-zero exit is not by itself the
# interesting fact — the named diagnostic is, and each assertion is on the exact
# line.  The unmutated baseline is the full-budget run recorded beside these in
# `crates/grove-task-tree/models/README.md`, which exits 0 and prints none of
# these lines.
#
# Usage:  models/run-controls.sh [name]...     (default: all)
set -euo pipefail

[[ ${BASH_VERSINFO[0]:-0} -ge 4 ]] || {
  echo "models/run-controls.sh needs bash 4+; found ${BASH_VERSION:-unknown}" >&2; exit 2; }

here="$(cd "$(dirname "$0")" && pwd)"
repo="$(cd "$here/.." && pwd)"
work="$(mktemp -d "${TMPDIR:-/tmp}/grove-run-controls.XXXXXX")"
trap 'rm -rf "$work"' EXIT

pass=0; failed=0

# A copy of exactly what a `--scope task-tree` run reads.
fresh_repo() {
  local d="$work/$1"
  rm -rf "$d"
  mkdir -p "$d/models" "$d/docs/specs" "$d/crates/grove-task-tree/models"
  cp "$repo/models/run.sh" "$d/models/"
  cp "$repo/docs/specs/semantic-contract.md" "$d/docs/specs/"
  cp "$repo/crates/grove-task-tree/models/"* "$d/crates/grove-task-tree/models/"
  echo "$d"
}

# Run the copy's runner and capture everything.  Never fatal here: a control's
# whole subject is a non-zero exit.
run_copy() {
  local d="$1"; shift
  set +e
  QUINT_SAMPLES=1 QUINT_STEPS=1 "$@" bash "$d/models/run.sh" \
    --scope task-tree --family quint --quiet >"$d/out" 2>&1
  echo $? >"$d/rc"
  set -e
}

expect() {
  local name="$1" d="$2" want_rc="$3" pattern="$4"
  local rc; rc=$(<"$d/rc")
  local ok=1
  [[ "$want_rc" == any || "$rc" == "$want_rc" ]] || ok=0
  grep -qE -- "$pattern" "$d/out" || ok=0
  if [[ "$ok" == 1 ]]; then
    printf 'PASS  %-26s exit %s, and says so\n' "$name" "$rc"; pass=$((pass + 1))
  else
    printf 'FAIL  %-26s exit %s (wanted %s), pattern not found: %s\n' \
      "$name" "$rc" "$want_rc" "$pattern"; failed=$((failed + 1))
    sed -n '$p;/runner error/p;/naming no obligation/p' "$d/out" | head -5
  fi
}

# A `quint` that is on PATH and cannot do its job.  `mode` decides how it dies.
shim_path() {
  local mode="$1"
  local d="$work/shim-$mode"
  mkdir -p "$d"
  cat >"$d/quint" <<SHIM
#!/usr/bin/env bash
case "\$1" in
  --version) [[ "$mode" == launch ]] && { echo "dyld: symbol not found" >&2; exit 1; }
             echo "0.32.0" ;;
  *)         echo "FATAL: the backend went away" >&2; exit 1 ;;
esac
SHIM
  chmod +x "$d/quint"
  echo "$d"
}

# ---------------------------------------------------------------------------
# 1. An INVENTED obligation.  Syntactically perfect, answers nothing, and the
#    catalogue has never heard of it.  This is the direction the coverage
#    matrix promised and did not check: before the fix it was counted under a
#    key nothing reads and the run stayed green.
# ---------------------------------------------------------------------------
ctl_invented_obligation() {
  local d; d=$(fresh_repo invented)
  cat >>"$d/crates/grove-task-tree/models/task-tree-controls.qnt" <<'PROBE'

module scenario_probe {
  import taskTree(
    FOCUS = 0,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_TT_99_invented_obligation: bool = true
}
PROBE
  run_copy "$d"
  expect invented-obligation "$d" 1 'names TT-99, which the catalogue does not define'
}

# ---------------------------------------------------------------------------
# 2. A CLAIM-LEVEL citation — `TT_24` where the manifest carries `TT-24.a` and
#    `TT-24.b`.  Real, not invented, and it credits no cell.  The relaxation is
#    obligation 4's, and it has to be VISIBLE or it is the first hole again.
# ---------------------------------------------------------------------------
ctl_claim_level() {
  local d; d=$(fresh_repo claimlevel)
  cat >>"$d/crates/grove-task-tree/models/task-tree-controls.qnt" <<'PROBE'

module scenario_claim_probe {
  import taskTree(
    FOCUS = 0,
    RENAME_ATOMIC = true, FOREIGN_WRITES = true, CRASHES = true,
    ENTRIES_REMOVABLE = false, HAND_EDITS = true, REAPER_SWEEPS_RESERVED = false,
    ONE_SNAPSHOT = true,
    BULK_TARGET_IDEMPOTENT = true,
    MAX_OBJECTS = 14, MAX_DEPTH = 6, MAX_POS = 6
  ).* from "./task-tree"

  val inv_TT_24_the_whole_claim: bool = true
}
PROBE
  run_copy "$d"
  expect claim-level-citation "$d" any 'cites the claim TT-24, not one of its obligations'
  # `if`, not `&&` — a trailing `&&` whose left side fails returns 1 from the
  # function, and under `set -e` that ends the whole control run at the one
  # control that went right.
  if grep -q 'TT-24 .*does not define' "$d/out"; then
    echo "FAIL  claim-level-citation      reported as invented"; failed=$((failed + 1))
  fi
}

# ---------------------------------------------------------------------------
# 3/4. The FORWARD direction, which was already asserted and is re-asserted
#      here so the pair is one instrument: delete the last property, and the
#      last witness, for one real obligation.
# ---------------------------------------------------------------------------
ctl_deleted_witness() {
  local d; d=$(fresh_repo delwit)
  local f="$d/crates/grove-task-tree/models/task-tree.qnt"
  perl -0pi -e 's/^  val wit_TT_19_a_preparing_tree_that_looks_perfectly_walkable: bool =\n(?:.+\n)+?\n/\n/m' "$f"
  if grep -q 'wit_TT_19' "$f"; then
    echo "FAIL  deleted-witness            mutation did not apply"; failed=$((failed + 1)); return
  fi
  run_copy "$d"
  expect deleted-witness "$d" 1 'TT-19 +quint:NO-WITNESS'
}

ctl_deleted_property() {
  local d; d=$(fresh_repo delinv)
  local f="$d/crates/grove-task-tree/models/task-tree.qnt"
  perl -0pi -e 's/^  val inv_TT_19_a_reserved_witness_refuses_everything_else: bool =\n(?:.+\n)+?\n/\n/m' "$f"
  if grep -q 'val inv_TT_19' "$f"; then
    echo "FAIL  deleted-property           mutation did not apply"; failed=$((failed + 1)); return
  fi
  run_copy "$d"
  expect deleted-property "$d" 1 'TT-19 +quint:NO-CHECK'
}

# ---------------------------------------------------------------------------
# 5. A DEAD TOOL at the front door: `quint` is on PATH and cannot launch.
# ---------------------------------------------------------------------------
ctl_dead_quint_launch() {
  local d shim; d=$(fresh_repo deadlaunch); shim=$(shim_path launch)
  run_copy "$d" env "PATH=$shim:$PATH"
  expect dead-quint-launch "$d" 2 'quint failed to launch'
}

# ---------------------------------------------------------------------------
# 6. A DEAD TOOL mid-run: `quint --version` answers, and every `quint run`
#    dies.  Before the fix a non-zero `quint run` was read as a verdict —
#    "violated" for an invariant, and, worse, "violated, AS THE CONTROL
#    REQUIRES" for a premise-break control, which is a dead tool reported as a
#    PASS.
# ---------------------------------------------------------------------------
ctl_dead_quint_run() {
  local d shim; d=$(fresh_repo deadrun); shim=$(shim_path run)
  run_copy "$d" env "PATH=$shim:$PATH"
  expect dead-quint-run "$d" 2 'quint run failed to complete .*tool failure, not a result'
}

# ---------------------------------------------------------------------------
# 7. A DEAD BACKEND: Apalache with a heap too small to read its own jar.  The
#    JVM prints a `LauncherHelper` stack trace and exits 1, which matched none
#    of the five strings the runner used to look for — so every invariant in
#    the batch was recorded "model-checked … no counterexample".  A green
#    verdict from a backend that never started.
# ---------------------------------------------------------------------------
ctl_dead_backend() {
  local d; d=$(fresh_repo deadbackend)
  run_copy "$d" env QUINT_VERIFY=1 QUINT_VERIFY_STEPS=1 JVM_ARGS=-Xmx6m
  expect dead-backend "$d" 2 'quint verify failed to complete .*tool failure, not a result'
}

all=(invented_obligation claim_level deleted_witness deleted_property
     dead_quint_launch dead_quint_run dead_backend)
want=("${@:-}")
[[ -n "${want[0]:-}" ]] || want=("${all[@]}")

for c in "${want[@]}"; do
  c="${c//-/_}"
  declare -F "ctl_$c" >/dev/null || { echo "unknown control: $c" >&2; exit 2; }
  "ctl_$c"
done

echo
echo "-- runner controls: $pass passed, $failed failed"
[[ "$failed" == 0 ]]
