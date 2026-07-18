# harness-spawn-preflight-k8

**Kind:** work

## Goal

Widen the pre-flight harness-binary check to cover per-kind rerouted harnesses,
so a missing `pi` fails before the loop starts rather than aborting it mid-run.

## Context

Found by the adversarial review in review-k3. driver-side-kill-k2 changed the
failure mode for a missing harness binary, as a side effect of dropping the
`sh -c` wrapper: `sh` used to absorb the failed exec (exiting 126), so
`cmd.status()` returned `Ok` and the loop reported `LoopOutcome::Stopped` with
the friendly "re-run `grove do` to resume". Now `cmd.spawn()` propagates
`ENOENT` as `Err`, aborting all of `grove do`:

    Err(launching the harness session: No such file or directory (os error 2))

Loud-over-silent is the right call and worth keeping — a silent 126 that reads
as "the human exited" is worse. The problem is *where* it fires. The pre-flight
PATH check in `src/launch.rs` validates only `harness.exec_bin` for the
**stamped** harness. This trial's central configuration is
`GROVE_REVIEW_HARNESS=pi` against a codex-stamped grove: with `pi` not
installed, the pre-check passes, the loop runs happily for hours, and then dies
on the first review leaf it reaches.

## Done when

A grove configured with a per-kind harness override whose binary is missing
fails at pre-flight with a diagnostic naming the override var and the missing
binary — not mid-loop on the first leaf of that kind. The abort-vs-stop change
for a genuinely unspawnable harness is documented (CHANGELOG at least; ADR only
if it clears the when-to-write bar).

## Notes

Check the same widening covers the `GROVE_HARNESS_BIN_<NAME>` per-harness test
seam, which can also point at a nonexistent path.
