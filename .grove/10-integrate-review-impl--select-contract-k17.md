# select-contract-k17

**Integrates:** select-contract-k16

## Goal

Triage the implementation review `select-contract-k16`, apply findings that
hold against the contract, and verify the resulting increment before
`owner-settings-k9` builds on it.

## Context

- The review's committed task file carries its findings, evidence, rulings and
  verification limits. Read it rather than treating its conclusions as the
  integration's charter.
- The reviewed producer is `select-contract-k8`, commit `086fd972`.
- `docs/specs/harness-selection-and-execution.md` is the contract; the root
  brief and the producer delimit what later implementation leaves own.

## Done when

- Every review finding has an evidence-backed disposition in this task's log.
- Any accepted fixes and meaningful regression coverage are implemented, and
  the increment's installed-layout smoke test and `bash scripts/check.sh`
  have current verification evidence.

## Notes

The review is inspection-only and ran no tests or builds. Integration owns
runtime confirmation and all post-fix verification.
