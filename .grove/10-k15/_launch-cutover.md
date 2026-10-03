# launch-cutover-k15 — brief

## Goal

Grove launches every lifecycle session through the supervised run:

- **The launch directory.** Each launch gets one, published as
  `GROVE_LAUNCH_DIR` and bound by the session epoch. It holds dispatch's exit
  channel and ending file.
- **The teardown record.** Grove writes it itself, with `grove-llm
  record-teardown`.
- **The loop decides** from the run ending and that record.

The old contract goes: `grove-llm complete`, `GROVE_SIGNAL_FILE` and the
runner's token. The methodology's signal contract names `harness-dispatch exit`,
with `grove-llm record-teardown` before it for a finish (D4, D5).

## Done when

- Seam 3 passes:
  - the endings;
  - staleness;
  - each launch directory removed after its launch, and an abandoned one by a
    replacement driver;
  - the terminal after dispatch's death, and a driver started in the
    background;
  - the driver's own TERM through dispatch;
  - the creator cases, with fakes that end the new way.
- The prompt, the shipped skills, the conformance rows and the Codex-provisioned
  skills state the same signal contract as the binary.
- `bash scripts/check.sh` passes at every leaf's commit.

## Decomposition

Expand → migrate → contract, each step green (P3):

1. `launch-directory` **(expand)**. The launch directory replaces the signal
   path as what the session epoch binds and what admission compares.
   Abandoned launch directories are removed, `grove-llm record-teardown`
   arrives, and the loop finishes on a teardown record. The old channel and
   `complete` still work, so the loop finishes on either.
2. `dispatch-ending` **(migrate)**. The driver runs dispatch with `--exit-dir`
   and `--ending-file`, allocates no channel and escalates nothing, and reads
   the launch by the spec's table. The prompt names the new verbs.
3. `signal-contract` **(contract)**. `complete`, `GROVE_SIGNAL_FILE` and the
   token go, and the methodology states the new contract. The
   `instructed_verbs` test ties the shipped skills to the CLI, so the skills
   change in the leaf that removes the verb.

## Pointers

- Spec, under *Grove integration*: *A lifecycle session*, *The launch
  directory*, *The process and terminal chain*, *Ending a session*, *The
  runner* and *A meta-grove*. Also *When dispatch dies*, under *Supervision*.
- Decision 9 of `docs/specs/module-decomposition.md` (`record_teardown`, and the
  signalling contract's own gap). Decision 7 (Grove's launch: no channel, a
  kill-grace longer than dispatch's).
- ADRs: `docs/adr/one-live-driver-per-working-tree.md` (the session epoch,
  abandoned-directory cleanup reading nothing),
  `docs/adr/dispatch-supervises-the-harness.md`.
- `harness-wrapper-k2`'s W8, W10, W11 and W12, and `harness-wrapper-k5`'s I3.
- `docs/specs/item-status.md`: the session witness rides the runner's
  launch events for dispatch.

## Notes

- **This grove's own sessions stay on v22 throughout** (root brief, P5). They
  end with the installed `grove-llm complete`, as their v22 prompt says, and
  use the installed `grove-llm` for every tree verb. Nothing this node builds
  reaches them before the release.
- The cargo guard keeps clearing `GROVE_SIGNAL_FILE`, beside
  `GROVE_LAUNCH_DIR` and `HARNESS_DISPATCH_EXIT_FILE`, for as long as this
  grove runs, and says why.
- `launch-cutover` (`review-impl`), cut beside this node, reviews the whole
  node. The session whose retirement closes it names its run on it.
