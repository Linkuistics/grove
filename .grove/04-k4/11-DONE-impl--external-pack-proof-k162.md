# external-pack-proof-k162

## Goal

Close the research-pack proof. Record the proven contract and the evidence
that repository-moves-k5 needs. Audit the Done when of
external-research-pack-k4 one criterion at a time.

## Context

- `docs/specs/external-pack-contract.md` and its section for proven facts.
- The records of grove-research-removal-k158, research-directory-trials-k160
  and research-native-namespace-k161.
- The "Completion audit" of `docs/common-loading-acceptance-record.md`, as a
  model for the audit.

## Done when

- The contract's section "Proven facts" fills each row of its table. It
  states the proven layout, resource references, checkout boundary and
  conformance boundary. It names the source
  directory and roots inside any checkout of the pack. It names the reusable
  pack tooling: the conformance invocation, the Taskfile tasks, the plugin
  manifest and the trial-runner fields.
- The section names the evidence that repository-moves-k5 needs before it
  starts, with commits. The node brief lists that evidence.
- ResearchSkills' own checks pass from a clean checkout of the published pack
  commit that records the removal commit. The run uses the committed public
  `GROVE_REPOSITORY` and no override. Record the remote bookmark that holds the
  recorded Grove commit. Grove's `bash scripts/check.sh` passes on the
  integrated line with the Ravel 2.0 build. Record both build identities.
- Each criterion of external-research-pack-k4 has a verdict and evidence. A
  gap has repair work, a covering case or a recorded human decision.
- Any missing work is a new leaf in this node before the node closes.

## Notes

Where a criterion fails and you cannot name the work, stop and ask the human.

## Decisions (running log)

- The Ravel 2.0 build of this leaf is `task install` from the Ravel
  workspace at `71f70fcd3f92` into a scratch prefix. Its `bin/ravel` SHA-256
  is `26b487afb957…`, the value of `builds.json`, and the prefix has no
  `libexec`.
- From a fresh plain Git checkout of the public ResearchSkills `main`,
  `task check` exited 0. The checkout was at `1c18a90945a0`, with the tree ID
  `79dc2a530cd2` for `plugins/research`. It used the committed public
  `GROVE_REPOSITORY` and `GROVE_COMMIT` `57414e00cd87`, with no override.
  `check:plugin` passed with the icon and version warnings. The pack alone
  gave 14 loaded-path checks and 3 external edges. The aggregate gave 116,
  2783 and 72. The checkout was clean after the run.
- The remote bookmark `gh-issue-25` is at `2328c7f0d45a`. `git merge-base
  --is-ancestor` shows that `57414e00cd87` is its ancestor, so the bookmark
  holds the recorded Grove commit.
- `ravel skills resolve --json` with that build, over the fresh pack checkout
  and the pack task's Grove checkout at `57414e00cd87`, gave `resolved`. The
  digests were `ea34f8afb116…` for `grove` and `7fb3d473f66f…` for
  `research`, the values of the trials.
- The audit goes into the acceptance record, as the section "Research pack
  audit", beside the research trials that it cites. The contract's "Proven
  facts" holds the facts and links to the audit. The node brief keeps its
  evidence list and points to the section for the commits.
- Gate with 2.0 on `f26e93064c73`, before any edit: `mise exec bun@1.4.3 --
  bash scripts/check.sh` passed all 21 principal checks, from 06:43Z to
  06:48Z, under a scratch HOME with the guard first on PATH. No log line holds
  `host-ravel-reached`. `check:packs` passed the pinned `dc3a4812db3b` alone
  and in the aggregate. The digest of the tracked files outside `.grove/` was
  the same after the run.
- The same gate on the final tree of this leaf passed all 21 principal checks
  again, from 06:50Z to 06:56Z, with 1343 Rust tests, no guard line and the
  same digest after the run.
- Every criterion of external-research-pack-k4 is Met, so the node closes with
  no new leaf. The pointer to the proven facts goes into the brief of
  repository-moves-k5.
