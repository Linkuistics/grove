# supervised-run-k22

**Integrates:** supervised-run-k11

## Goal

Triage `supervised-run-k11`'s seven findings against the source and apply those
that hold, so that `confined-run-k12` builds on an agreed supervisor. This
session owns the corrections to `runner-job-k8`, `dispatch-supervises-k9` and
`run-ending-k10` and their verification, not the confinement work that follows.

## Context

- Read the findings in `supervised-run-k11`'s committed review artifact. Classify
  each on evidence (a contract stated unclearly, a real issue, a visible
  trade-off, or noise), rather than treating the list as agreed work.
- The reviewed producer is `supervised-run-k7`: `runner-job-k8` (`75af05fd`),
  `dispatch-supervises-k9` (`464499fe`) and `run-ending-k10` (`f6fc19aa`).
- F1 (a caller that ignores SIGCHLD breaks selection) and F2 (handlers dropped
  before the end is recorded) rest on reading. Reproduce each before fixing it.
- F3 and F4 each pick between two corrections, one in code and one in the spec.
  Settle which, and keep the spec, the glossary and the walkthrough books that
  describe it in step. A leaf that changes a crate's source updates that crate's
  walkthrough book in the same commit, and logs the change under `## Unreleased`
  in `CHANGELOG.md`.
- The owner's mid-session request about `harness-dispatch init` (`supervised-run-k11`'s
  R2) is not a finding. It needs the owner, so leave it for them.

## Done when

- Each finding is classified, and each real one is fixed with a test through
  dispatch's command or the runner, or accepted visibly.
- F6's missing seam cases exist through `harness-dispatch run`, or the spec's seam
  row says which are runner-level.
- `bash scripts/check.sh` passes.

## Notes

- This is a meta-grove: end the session with the installed v22 `grove-llm`, never
  `./target/debug/grove-llm`, as the root brief says.
