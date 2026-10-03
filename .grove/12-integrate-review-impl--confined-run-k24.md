# confined-run-k24

**Integrates:** confined-run-k23

## Goal

Triage `confined-run-k23`'s findings against the source and apply those that
hold, so that `launch-cutover-k15` builds on agreed confinement. This session
owns the corrections to `dispatch-confines-k13` and
`standalone-through-dispatch-k14` and their verification, not the lifecycle
cutover that follows.

## Context

- Read the findings in `confined-run-k23`'s committed review artifact. Classify
  each on evidence (a contract stated unclearly, a real issue, a visible
  trade-off, or noise), rather than treating the list as agreed work.
- The reviewed producer is `confined-run-k12`: `dispatch-confines-k13` and
  `standalone-through-dispatch-k14`. Each commit message names its handle.
- The review marks which findings rest on reading (PLAUSIBLE) and which on the
  source (CONFIRMED). Reproduce each PLAUSIBLE one before fixing it. Linux
  bubblewrap was never exercised by the producer or the reviewer, so a Linux
  finding is reasoning until someone runs it.
- Where a finding offers a code correction and a contract correction, settle
  which, and keep the spec, the glossary and the walkthrough books that describe
  it in step. A leaf that changes a crate's source updates that crate's
  walkthrough book in the same commit, and logs the change under `## Unreleased`
  in `CHANGELOG.md`.

## Done when

- Each finding is classified, and each real one is fixed with a test through
  dispatch's command, the runner or `grove run`, or accepted visibly.
- `bash scripts/check.sh` passes.

## Notes

- This is a meta-grove: end the session with the installed v22 `grove-llm`,
  never `./target/debug/grove-llm`, as the root brief says.
