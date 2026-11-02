#!/usr/bin/env bash
# Run every claim of the owner-switch model and report pass or fail.
#
#   inv NAME   must HOLD in that instance       (quint run --invariants)
#   wit NAME   must be REACHED in that instance (quint run --witnesses)
#   dead       every mutant witness must stay UNREACHED in a corrected world
#
# A mutant instance removes one mechanism.  Its witness writes out the
# violation inline, and the dead check shows that the same predicate is not
# reached while the mechanism is present.  The two together make each
# mechanism's control isolating: it fails for the reason that it names.
#
# The sweep at the end runs every invariant against every mutant and prints
# what it finds.  It is information, not a pass condition.
#
# Results are sampled, not proved.  Each line quotes the step bound and the
# sample count.
#
# Env: SAMPLES (default 4000), STEPS (default 60), SWEEP (default 1)
set -euo pipefail

model="$(dirname "$0")/owner-switch.qnt"
samples="${SAMPLES:-4000}"
steps="${STEPS:-60}"
sweep="${SWEEP:-1}"

[[ -f "$model" ]] || { echo "model not found: $model" >&2; exit 2; }
command -v quint >/dev/null || { echo "quint not on PATH" >&2; exit 2; }

ALWAYS=(
  inv_noSilentLoss
  inv_exactlyOneState
  inv_stateMatchesTable
  inv_interruptedOnlyWhenOpen
  inv_settledOnlyWhenClosed
  inv_receiptRecordsSwitched
  inv_receiptMatchesProbes
  inv_completeOnlySwitched
  inv_onlyRehearsedApplied
  inv_acceptedOnlyClassified
)

MUTANT_WITNESSES=(
  wit_m_writeLostSilently
  wit_m_interruptionWithoutRun
  wit_m_settledWhileOpen
  wit_m_receiptOnUnswitched
  wit_m_unrehearsedApplied
  wit_m_thirdValueLost
  wit_m_mergedDeleteLost
  wit_m_acceptedUnclassified
)

CORRECTED=(crashes writers both classify_world coincidence after_receipt during_run
           merge_return entries)
MUTANTS=(m_no_expect m_no_run_record m_early_end m_receipt_no_recheck
         m_no_rehearsal_check m_inverse_resets_third m_merge_no_authority
         m_accept_unchecked)

witnesses() {
  case "$1" in
    crashes)        echo wit_complete wit_completeAfterCrash wit_resumedToNative \
                         wit_probeFailedToNative wit_ownerInverseToNative wit_activeAfterCrash ;;
    writers)        echo wit_complete wit_conflictFlagged wit_lateWriteFlagged \
                         wit_cellConflictFlagged wit_mergedAndComplete \
                         wit_driftedDuringRun wit_changedNeedsPlan wit_deleteFlagged ;;
    both)           echo wit_complete wit_completeAfterCrash wit_conflictFlagged \
                         wit_resumedToNative ;;
    classify_world) echo wit_changedNeedsPlan wit_secondPlanComplete ;;
    coincidence)    echo wit_coincidenceReset ;;
    after_receipt)  echo wit_driftedAfterReceipt wit_ownerInverseToNative ;;
    during_run)     echo wit_conflictFlagged wit_cellConflictFlagged \
                         wit_lateWriteFlagged wit_driftedDuringRun ;;
    merge_return)   echo wit_mergedReturnFlagged wit_mergedAndComplete ;;
    entries)        echo wit_resumedCopied wit_resumedStoppedChanged \
                         wit_secondPlanComplete ;;
    m_no_expect)            echo wit_m_writeLostSilently ;;
    m_no_run_record)        echo wit_m_interruptionWithoutRun ;;
    m_early_end)            echo wit_m_settledWhileOpen ;;
    m_receipt_no_recheck)   echo wit_m_receiptOnUnswitched ;;
    m_no_rehearsal_check)   echo wit_m_unrehearsedApplied ;;
    m_inverse_resets_third) echo wit_m_thirdValueLost ;;
    m_merge_no_authority)   echo wit_m_mergedDeleteLost ;;
    m_accept_unchecked)     echo wit_m_acceptedUnclassified ;;
  esac
}

# The second plan needs a classification, a rehearsal and a whole switch.
# The unchecked receipt needs an outside write inside the probe window.
samples_for() {
  case "$1" in
    classify_world)       echo $(( samples * 3 )) ;;
    m_receipt_no_recheck) echo $(( samples * 5 )) ;;
    *)              echo "$samples" ;;
  esac
}

run_q() { quint run "$model" --main="$1" --max-steps "$steps" --max-samples "$2" "${@:3}"; }

check_invariants() {
  local inst=$1 n=$2; shift 2
  local names=("$@") name
  if run_q "$inst" "$n" --invariants "${names[@]}" --verbosity=0 >/dev/null 2>&1; then
    for name in "${names[@]}"; do printf 'PASS  %-34s holds (%s samples, %s steps)\n' "$name" "$n" "$steps"; done
    return 0
  fi
  local bad=0
  for name in "${names[@]}"; do
    if run_q "$inst" "$n" --invariant "$name" --verbosity=0 >/dev/null 2>&1; then
      printf 'PASS  %-34s holds (%s samples, %s steps)\n' "$name" "$n" "$steps"
    else
      printf 'FAIL  %-34s violated\n' "$name"; bad=1
    fi
  done
  return "$bad"
}

check_witnesses() {
  local inst=$1 n=$2; shift 2
  local out name line count bad=0
  out=$(run_q "$inst" "$n" --witnesses "$@" --verbosity=1 2>&1) || true
  for name in "$@"; do
    line=$(grep -E "^$name was witnessed" <<<"$out" || true)
    count=$(sed -E 's/.* in ([0-9]+) trace.*/\1/' <<<"$line")
    if [[ -n "$line" && "${count:-0}" -gt 0 ]]; then
      printf 'PASS  %-34s reached in %s of %s traces\n' "$name" "$count" "$n"
    else
      printf 'FAIL  %-34s never reached in %s traces\n' "$name" "$n"; bad=1
    fi
  done
  return "$bad"
}

check_dead() {
  local inst=$1 n=$2 expr="" w bad=0
  for w in "${MUTANT_WITNESSES[@]}"; do expr+="${expr:+ and }not($w)"; done
  if run_q "$inst" "$n" --invariant "$expr" --verbosity=0 >/dev/null 2>&1; then
    printf 'PASS  %-34s every mutant witness unreached\n' "dead"
    return 0
  fi
  for w in "${MUTANT_WITNESSES[@]}"; do
    if ! run_q "$inst" "$n" --invariant "not($w)" --verbosity=0 >/dev/null 2>&1; then
      printf 'FAIL  %-34s reached in a corrected world\n' "dead:$w"; bad=1
    fi
  done
  return "$bad"
}

fail=0
for inst in "${CORRECTED[@]}"; do
  echo "── $inst ──────────────────────────────────────────────"
  n=$(samples_for "$inst")
  check_invariants "$inst" "$n" "${ALWAYS[@]}" || fail=1
  read -r -a wits <<<"$(witnesses "$inst")"
  check_witnesses "$inst" "$n" "${wits[@]}" || fail=1
  check_dead "$inst" "$n" || fail=1
done

for inst in "${MUTANTS[@]}"; do
  echo "── $inst ──────────────────────────────────────────────"
  n=$(samples_for "$inst")
  read -r -a wits <<<"$(witnesses "$inst")"
  check_witnesses "$inst" "$n" "${wits[@]}" || fail=1
done

if (( sweep )); then
  echo
  echo "── sweep: every invariant against every mutant (information) ──"
  for inst in "${MUTANTS[@]}"; do
    violated=()
    for name in "${ALWAYS[@]}"; do
      run_q "$inst" "$samples" --invariant "$name" --verbosity=0 >/dev/null 2>&1 || violated+=("$name")
    done
    printf '%-24s %s\n' "$inst" "${violated[*]:-no invariant violated}"
  done
fi

echo
(( fail )) && echo "FAILED" || echo "all claims hold"
exit "$fail"
