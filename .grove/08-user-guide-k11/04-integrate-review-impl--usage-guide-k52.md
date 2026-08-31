# usage-guide-k52

**Integrates:** usage-guide-k51

## Goal

Triage the findings in `usage-guide-k51`, apply the valid ones, and restore the
user guide's claim to complete, source-exact coverage of its inventory.

## Context

- Read the review from its committed handle. Its findings are inputs to triage,
  not obligations accepted in advance.
- `docs/specs/user-guide-coverage.md` remains the standard; if triage changes the
  standard, make that change explicit rather than silently fitting it to the
  guide.
- The review was inspection-only and ran no tests. Post-fix verification belongs
  to this leaf.

## Done when

- Every review finding is classified and each valid one is fixed at its real
  source.
- The guide and coverage inventory agree without crossing their documented
  ownership boundary.
- The relevant focused tests and `bash scripts/check.sh` pass.

## Notes

Keep the guide's eight explicit `usage-*` anchors stable unless triage proves a
contract change is necessary; future walkthrough books reserve from that set.
