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

## Decisions (running log)

- F1: noise raised for want of runtime evidence. The real dispatch test now
  tries creating and writing a hard link as well as the literal path. Seatbelt
  denies the link on this Mac, and the original credential stays unchanged;
  retain the regression test and the read-only contract.
- F4: visible trade-off accepted for this increment. Docker Linux reports a
  soft limit and kernel `nr_open` of 1,048,576. A Python loop performing the same
  `fcntl` calls (plus exception handling) took 0.611 s; raising the limit to the
  claimed 1,073,741,816 refused. This confirms linear cost, not the alleged
  minutes on this host. Keep the finite-limit requirement and full sweep, which
  covers descriptors std opens after inspection and inherited descriptors above
  a lowered limit. Reopen for measured launch latency on a supported Linux host
  or a requirement to support unbounded limits; the proposed highest-open-only
  fallback would miss concurrent/newly opened descriptors.
- Graph evidence unavailable: `list_projects` refused because an unverified
  active generation owns the service. Exact source and command tests supply
  the evidence here; no exhaustive graph claim is made.
- F2: real issue. Protect the canonical ending-file path alongside owner
  resources; direct and symlinked writable destinations now refuse before
  evaluation or recording. Both cases failed against the original code and
  pass with the overlap check.
- F3: real issue, reproduced under an outer Seatbelt policy denying the
  Sandbox system call: the original run selected, recorded execution and
  ended with launcher exit 71. Probe an empty sandbox around `/usr/bin/true`
  before selection, capture the launcher's diagnostics, and refuse with
  `confinement_unusable`. The same command test now refuses and records nothing.
  This is a capability check, not proof of later grant setup; an intervening
  filesystem or platform change can still fail a real launch after preflight.
- F5: real coverage gaps. Add real `grove run` tests for a denied forged
  report followed by an unacknowledged zero exit, cancellation while copying
  publication after dispatch ended, and a second-output collision reporting
  the already-published first output. Repair the staging replacement test to
  retain its acknowledgement helper, tolerate denied rename/link operations,
  and assert the held-directory output read is reached. All 20 standalone
  command tests pass on macOS.
- F6: real documentation drift. Correct the book-structure ledger here;
  keep decision 7's complete API migration with `current-state-docs-k20`,
  explicitly notifying that existing leaf of the committed review's pointer.
- F7: restore both cargo and helper guards for the installed v22 standalone
  signal, and name the dispatch group accurately. Accept transcript relay
  failure as a visible fail-closed trade-off: display is part of an invocation,
  so failed display prevents publication while retaining the transcript. This
  is documented in the standalone contract and overview book.
- The additional assumptions are accepted explicitly: Grove's uncatchable
  death can orphan its detached dispatch and harness, but publishes nothing;
  the nested cancellation bound depends on dispatch completing its record
  work inside Grove's grace; confinement resolves argv[0] to the executable's
  canonical path, including multicall aliases, as the existing spec states.
- The narrow fresh-context source review found no issue in the preflight's
  signal handling or captured diagnostics. A real command test also covers
  preflight with SIGCHLD ignored and SIGUSR1 blocked.
- The first full check caught an unclassified `GROVE_RUN_SIGNAL_FILE` in the
  removed-surface inventory. Classify it as a retained legacy control guard,
  not a newly supported runtime input; the inventory still requires an actual
  occurrence outside its own table, and the guard test fails when the guard is
  removed. Rerun the failed seam and the whole check after this correction.
- Final verification: `task check` (the Taskfile entry for
  `bash scripts/check.sh`) passed all 12 principal checks, including the
  workspace tests and final validation of all six books. SHA-256 digests of
  all 1,924 tracked file subjects matched before and after this successful
  run. Linux bubblewrap was not exercised; the Linux evidence here is the
  descriptor-sweep measurement only.
