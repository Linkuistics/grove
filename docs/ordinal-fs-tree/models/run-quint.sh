#!/usr/bin/env bash
# Run every claim in the behavioural model and report pass/fail.
#
# The convention this script keys on, matching `run-alloy.sh`:
#
#   inv NAME   must HOLD in that instance      (quint run --invariant)
#   wit NAME   must be REACHED in that instance (quint run --witnesses)
#
# A property that holds in one instance and fails in another is not a
# contradiction — it is the point.  Each instance fixes one question, and a
# deliberately admitted violation is written as the WITNESS that reaches it,
# never as an invariant expected to fail.  So every row below is a positive
# claim, and the table is the model's own account of what each instance shows.
#
# Env: SAMPLES (default 2000), STEPS (default 24), MODEL (default operations.qnt)
set -euo pipefail

model="${MODEL:-$(dirname "$0")/operations.qnt}"
samples="${SAMPLES:-2000}"
steps="${STEPS:-24}"

[[ -f "$model" ]] || { echo "model not found: $model" >&2; exit 2; }
command -v quint >/dev/null || { echo "quint not on PATH" >&2; exit 2; }

# --- the claims -------------------------------------------------------------
# Structural claims every instance must satisfy, whatever else it is exploring.
ALWAYS=(
  inv_siblingNamesUnique
  inv_onlyNodesHaveChildren
  inv_noFloatingObjects
  inv_freshKeysAreFresh
  inv_atomicity
  inv_blockedIsAtomicToo
  inv_rollbackRemovesOnlyItsOwn
  inv_insertOnlyShifts
  inv_promoteKeepsIdentity
  inv_rewriteKeepsPlace
  inv_appendOnlyAdds
  inv_interpreterNeverFindsADestinationTaken
)

claims() {
  local inst=$1 c
  for c in "${ALWAYS[@]}"; do echo "inv $c"; done
  case "$inst" in
    pristine)
      # Only the library writes.  Everything holds, density included: this is
      # the `init = empty tree` answer.
      echo "inv inv_ordinalsDistinctAtRest"
      echo "inv inv_keysUniqueAtRest"
      echo "inv inv_denseAtRest"
      echo "inv inv_ordinalsDistinctThroughout"
      echo "inv inv_destinationNeverOccupied"
      echo "wit wit_succeeded"
      echo "wit wit_appendManySucceeded"
      echo "wit wit_initializeSucceeded"
      echo "wit wit_initializeWritesADistinguishedChild"
      echo "wit wit_promoteWithChild"
      echo "wit wit_rewriteToSameParts"
      echo "wit wit_refusedTargetMissing"
      echo "wit wit_refusedTargetNotNode"
      echo "wit wit_refusedNoOccupantAtOrdinal"
      echo "wit wit_insertPastTheEnd"
      echo "wit wit_refusedPromoteNotLeaf"
      echo "wit wit_refusedPromotePartsNotNode"
      echo "wit wit_refusedRewriteSpeciesChange"
      echo "wit wit_promoteTransientlyDuplicatesAKey"
      echo "wit wit_promoteTransientlyDuplicatesAnOrdinal"
      ;;
    hand_edited)
      # A human edits between operations.  Density FAILS — reached as a
      # witness, which is the `init = arbitrary well-formed tree` answer.
      echo "inv inv_ordinalsDistinctAtRest"
      echo "inv inv_keysUniqueAtRest"
      echo "inv inv_ordinalsDistinctThroughout"
      echo "inv inv_destinationNeverOccupied"
      echo "wit wit_gappedLevel"
      echo "wit wit_insertIntoAGap"
      echo "wit wit_shiftPartiallyApplied"
      ;;
    corrupted)
      # A duplicated key, which the library admits and never checks.  Even
      # here, highest-first neither collides nor transiently duplicates.
      echo "inv inv_ordinalsDistinctAtRest"
      echo "inv inv_ordinalsDistinctThroughout"
      echo "inv inv_destinationNeverOccupied"
      echo "wit wit_duplicateKeysAdmitted"
      ;;
    lowest_first)
      # The same trees, shifted the other way.  Both payoffs of the ordering
      # rule are reached here and nowhere else.
      echo "inv inv_ordinalsDistinctAtRest"
      echo "wit wit_duplicateKeysAdmitted"
      echo "wit wit_shiftOrderRefusesTheInsert"
      echo "wit wit_shiftTransientlyDuplicatesAnOrdinal"
      ;;
    no_distinguished)
      echo "inv inv_ordinalsDistinctAtRest"
      echo "inv inv_ordinalsDistinctThroughout"
      echo "inv inv_destinationNeverOccupied"
      echo "wit wit_refusedNoDistinguishedChild"
      echo "wit wit_refusedNoDistinguishedChildOnInitialize"
      ;;
    unparseable)
      echo "inv inv_ordinalsDistinctAtRest"
      echo "inv inv_ordinalsDistinctThroughout"
      echo "inv inv_destinationNeverOccupied"
      echo "wit wit_haltedUnparseable"
      ;;
    failures)
      echo "inv inv_ordinalsDistinctAtRest"
      echo "inv inv_keysUniqueAtRest"
      echo "inv inv_ordinalsDistinctThroughout"
      echo "inv inv_destinationNeverOccupied"
      echo "wit wit_failedRolledBack"
      echo "wit wit_appendManySucceeded"
      ;;
    rollback_fails)
      # The only instance that does NOT claim ordinal distinctness or key
      # uniqueness at rest — a rollback that fails is exactly what breaks them,
      # and the witnesses below are that failure, reached deliberately.
      echo "wit wit_failedPartiallyRolledBack"
      echo "wit wit_partialRollbackLeavesNeitherState"
      echo "wit wit_partialRollbackLeavesADuplicateKey"
      echo "wit wit_partialRollbackLeavesADuplicateOrdinal"
      echo "wit wit_damagedTreeStrandsALaterOperation"
      ;;
  esac
}

INSTANCES=(pristine hand_edited corrupted lowest_first
           no_distinguished unparseable failures rollback_fails)

# Some witnesses are rare under random simulation — `rollback_fails` needs two
# independent failures in one trace, and one of its witnesses lands in roughly
# 0.07% of them.  Sampling it at the common budget would make this suite
# flaky, which is worse than slow, so that instance gets its own.
samples_for() {
  case "$1" in
    rollback_fails) echo $(( samples * 6 )) ;;
    *)              echo "$samples" ;;
  esac
}

fail=0
for inst in "${INSTANCES[@]}"; do
  echo "── $inst ──────────────────────────────────────────────"
  nsamples=$(samples_for "$inst"); invs=(); wits=()
  while read -r kind name; do
    [[ -z "$kind" ]] && continue
    case "$kind" in inv) invs+=("$name");; wit) wits+=("$name");; esac
  done < <(claims "$inst")

  # Invariants: batched first (fast), attributed individually only on failure.
  if quint run "$model" --main="$inst" --invariants "${invs[@]}" \
        --max-steps "$steps" --max-samples "$nsamples" --verbosity=0 >/dev/null 2>&1; then
    for n in "${invs[@]}"; do printf 'PASS  %-52s holds\n' "$n"; done
  else
    for n in "${invs[@]}"; do
      if quint run "$model" --main="$inst" --invariant "$n" \
            --max-steps "$steps" --max-samples "$nsamples" --verbosity=0 >/dev/null 2>&1; then
        printf 'PASS  %-52s holds\n' "$n"
      else
        printf 'FAIL  %-52s violated\n' "$n"; fail=1
      fi
    done
  fi

  # Witnesses: one run reports all of them, with a trace count each.
  if (( ${#wits[@]} )); then
    out=$(quint run "$model" --main="$inst" --witnesses "${wits[@]}" \
            --max-steps "$steps" --max-samples "$nsamples" --verbosity=1 2>&1) || true
    for n in "${wits[@]}"; do
      line=$(grep -E "^$n was witnessed" <<<"$out" || true)
      count=$(sed -E 's/.* in ([0-9]+) trace.*/\1/' <<<"$line")
      if [[ -n "$line" && "${count:-0}" -gt 0 ]]; then
        printf 'PASS  %-52s reached in %s trace(s)\n' "$n" "$count"
      else
        printf 'FAIL  %-52s never reached\n' "$n"; fail=1
      fi
    done
  fi
done

echo
(( fail )) && echo "FAILED" || echo "all claims hold"
exit "$fail"
