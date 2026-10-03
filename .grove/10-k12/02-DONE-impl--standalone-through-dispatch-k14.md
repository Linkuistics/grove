# standalone-through-dispatch-k14

## Goal

`grove run KIND` launches its confined harness through dispatch:

- **The launch.** Grove stages the invocation's private working directory and
  runs `harness-dispatch run --confine` there. It passes the kind, the
  invocation's whole prompt, each `--runtime-read` grant, and an
  `--ending-file` in Grove's own private directory outside the sandbox. It
  passes no task file, task identity or parameter.
- **How dispatch runs.** Noninteractively, through the runner: in a session of
  its own, with null stdin, its output (the harness's included) going to the
  invocation's transcript. The environment is Grove's own with the loop-control
  variables removed, so the owner's grants and bounds apply to selection.
- **The acknowledgement.** The prompt tells the harness to run
  `harness-dispatch exit`, by the canonical path of the dispatch beside Grove,
  once every required output is written and checked.
- **Cancellation.** Grove forwards its own cancellation to dispatch and waits
  for it.
- **Publication.** Grove publishes the declared outputs only on an
  `exit_signal` ending, read from the ending file, with dispatch's exit 0 and
  no cancellation of its own. Any other end publishes nothing.

Grove's `inspect`-then-confine path goes. So do its own completion channel
(`GROVE_RUN_SIGNAL_FILE`) and its reading of `grove-llm complete --done`. The
runner's file-output confined launch goes with this, its last caller.

## Context

- The spec's *A standalone invocation* under *Grove integration*, and all of
  `docs/specs/standalone-invocations.md`.
- `crates/grove/src/standalone.rs` today runs `inspect`, launches the reported
  file with `keyed_launch::run_confined`, and publishes when the helper
  `grove-llm` writes `done` into its channel. Its suite is
  `crates/grove/tests/standalone.rs`, and
  `crates/grove-llm/tests/standalone_completion.rs` covers the helper's side.
- Release preparation runs `grove run release-notes`. See
  `scripts/release-prepare.test.sh` and `scripts/release-notes/`.

## Done when

- Seam 4:
  - The policy selects the standalone kind outside the sandbox, and a variable
    the owner granted is visible to it.
  - The harness runs inside the sandbox with the prompt and a recorded run ID.
    It cannot read the personal policy, the owner settings or the record
    store, and cannot write outside its working directory and dispatch's run
    directory.
  - The file dispatch resolved is the file confined and run.
  - Outputs publish on the exit signal with a clean end, and never on the
    harness's exit alone, nor when it fails after signalling.
  - A writer the harness left in its group after a clean acknowledgement is
    stopped before any output publishes.
  - A refused selection publishes nothing. A signal during selection or the run
    launches or publishes nothing and leaves no confined process running.
- The verification obligations in `docs/specs/standalone-invocations.md` hold,
  including that a nested exit signal and nested cancellation leave an
  enclosing session's run intact.
- No standalone code or test path writes or reads the `done` token.
- Release preparation's tests pass with a deterministic harness that
  acknowledges through `harness-dispatch exit`.
- The `overview` and `keyed-launch` books follow (P2), and `## Unreleased`
  records that `grove run` launches through dispatch with a run record.
- `bash scripts/check.sh` passes.

## Notes

- This leaf's retirement closes `confined-run-k12`. Run that node's close,
  and judge its review as its brief describes, before retiring.
- `grove-llm complete` remains for lifecycle sessions until `launch-cutover`
  removes it. Its standalone-completion tests go or move here, with the path
  they covered.

## Implementation steps

1. Migrate the real-confinement fixtures to dispatch exit; require a recorded run, protected owner resources and isolated enclosing controls. Run the seam red against the current adapter.
2. Launch dispatch with the noninteractive runner and read only its ending observation; retain staging and publication validation. Remove the obsolete standalone helper and confined file-output runner, migrating runner probes to its remaining confined interface.
3. Exercise acknowledged failures, survivor writers, run/selection cancellation, input isolation and output races. Validate release preparation with a deterministic dispatch-acknowledging harness.
4. Update the changed crates' books and Unreleased notes, run task check, settle the node's review, retire and seal the focused jj change.

## Decisions (running log)

- K14.1: The approved standalone and area specs are the implementation contract. Grove's launch keeps its environment except enclosing completion/launch controls; dispatch owns the confined environment. The ending file stays outside both writable sandbox roots.
- K14.2: Graph access failed before project discovery because an incompatible generation is active. Use targeted source fallback; no negative claim rests on the graph.

- K14.3: A dispatch supervisor needs cancellation and reap, not another completion channel. Add `NoninteractiveLaunch` without a channel; the runner shares its spawn/watch core through a private job descriptor with an optional completion capability. This removes Grove standalone's channel allocation entirely.
- K14.4: Refused selection now retains a transcript and failed status because dispatch's selection runs inside the recorded invocation. The existing missing-sibling diagnostic still happens before a transcript is opened.

- K14.5: The confinement node earns a scheduled review before launch-cutover. Its contract joins canonical grant refusal, environment isolation and acknowledgement-only publication across two supervisors. The executable seam supplies behavioural evidence; the future review will contest that boundary and the changed books in fresh context. No in-session reviewer duplicates it.

- K14.6: The sibling dispatch can itself be a symlink. The new regression reproduced an alias in the acknowledgement prompt; canonicalize after the sibling lookup so both launch and the sandbox's exit command use the executable's resolved path. Keep the existing missing-sibling diagnostic.

## Validation

- The initial `task check` passed all 12 principal checks and all six books;
  all 1,922 tracked subject hashes were unchanged throughout the run.
- The canonical-sibling regression then failed with the alias in the prompt;
  after canonicalization, all 17 standalone tests and both directly affected
  book checks passed. Final whole-repository verification follows below.
- Scheduled `confined-run-k23` reviews the closing node before launch-cutover.
  The root brief preserves the channel-free supervisor distinction for its
  later siblings. No ADR change is needed: this implements the approved set.

- Final `task check` passed all 12 principal checks without warnings or failures,
  including all 17 standalone cases, the real deterministic release-notes run,
  process/PTY and confinement suites, and all six final book validations.
  All 1,923 tracked subjects and their hashes were unchanged during that run.
- The changed books reconstruct overview's 1,129 lines, keyed-launch's 2,369
  lines and grove-llm's 949 lines, with none deferred. The standalone adapter
  and test sweep found no legacy completion channel or command; positive,
  cross-tree and deliberately dirty/clean scratch controls behaved as expected.
- Native confinement was exercised on macOS Seatbelt. Linux bubblewrap was not
  exercised on this host and is called out in the scheduled review.
- `confined-run-k12` meets its Done when: the two dispatch/standalone increments
  implement the agreed seams and verification obligations, and standalone no
  longer uses Grove's completion token. Its last leaf retires in this change;
  the root still has live review, cutover, documentation and release work.
