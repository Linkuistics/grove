# confined-run-k12 — brief

## Goal

`grove run` launches through `harness-dispatch run --confine` (D6). Dispatch
selects outside the sandbox, with the owner's settings, grants and bounds, then
confines and supervises the harness it spawns, which gets a run record and a
run ID. Grove keeps what is a standalone invocation's own: staging, the inputs,
the declared outputs, and publication, which happens only on an `exit_signal`
ending with dispatch's exit 0.

## Done when

- Seam 4 passes.
- Seam 1's confinement refusals and seam 2's confinement record pass.
- The verification obligations of `docs/specs/standalone-invocations.md` hold.
- Nothing but a lifecycle session still uses `grove-llm complete`, and no
  standalone path reads its `done` token, so `launch-cutover` can remove it.
- `bash scripts/check.sh` passes at every leaf's commit.

## Decomposition

1. `dispatch-confines` gives dispatch `run --confine`, `--runtime-read`, the
   refusal of a grant that reaches owner data, the confined harness's minimal
   environment, and the confinement in the run record. Any caller can use it,
   and Grove's `grove run` is unchanged by it.
2. `standalone-through-dispatch` moves `grove run` onto that, removing Grove's
   inspect-then-confine path, its channel and its use of the `done` token.

Dispatch's half is usable on its own. Grove's half needs it.

## Pointers

- Spec: *Confinement*, and *A standalone invocation* under *Grove
  integration*. All of `docs/specs/standalone-invocations.md`.
- `harness-wrapper-k5`'s I2 (the refusal by canonical overlap) and
  `harness-wrapper-k2`'s W13.
- ADR `docs/adr/harness-selection-is-owned-by-policy.md` (*Have the executable
  launch a confined invocation too*, now reopened and answered).

## Notes

- Whether this node earns a `review-impl` is the closing producer's judgement
  (P4). Weigh three doubts:
  - the protected-path refusal, compared canonically, including a symlinked
    alias, and the two exempt executables;
  - the scrubbed environment moving behind selection;
  - publication resting only on an acknowledged, group-confirmed, exit-0 end.

  If the producer cuts a review, it `leaf-insert`s it before
  `launch-cutover`'s node so the lifecycle cutover builds on reviewed
  confinement, and names its run on it.
- `grove run` is also how release preparation produces its notes, through the
  `release-notes` kind. Its deterministic harness in the release scripts' tests
  must acknowledge the new way.
