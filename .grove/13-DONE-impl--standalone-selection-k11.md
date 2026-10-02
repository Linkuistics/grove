# standalone-selection-k11

## Goal

`grove run KIND` selects its command with `harness-dispatch inspect --json` and
the runner launches the reported file under confinement. No Grove configuration
is read for it.

## Context

- `docs/specs/harness-selection-and-execution.md`, *Grove integration*:
  *Finding it*, *A standalone invocation*, *A refusal* and *The runner*; and
  the confinement and runner rows of the test seams.
- `docs/specs/standalone-invocations.md`, *Selection*.
- Root brief, requirements 6, 10 and 14.
- `direct-dispatch-k4`'s note *Where the design lands in the code*, the *Runner*
  and *Grove* entries.

## Done when

- Grove finds `harness-dispatch` beside its own executable, from its own real
  path, and reports a missing one with that path.
- `grove run KIND` stages, selects outside the sandbox and launches inside it,
  as the specification states. A refused, cancelled or timed-out selection
  launches nothing and publishes nothing.
- The runner's argv has a public constructor. A confined launch takes an
  absolute program path, refuses any other, and does no PATH lookup of its own.
- The cases of the `grove run` confinement seam hold, and a `config.kdl` or
  `.grove.kdl` on disk changes nothing about `grove run`.
- The release tooling's tests that run `grove run release-notes` work from a
  policy.
- The overview and `keyed-launch` books are valid for what changed.
- `bash scripts/check.sh` passes.

## Notes

- Lifecycle sessions still launch from configuration after this leaf.
  `lifecycle-launch-k12` moves them, and reuses the lookup and the constructor
  this leaf adds.
- A standalone invocation gets no run record and no launcher on `run`.

## Decisions (running log)

**The lookup lives in the `grove` binary, not in the loop.** `crates/grove/src/dispatch.rs`
finds the sibling from the binary's own real path. `lifecycle-launch-k12` hands
that path to `grove_loop::run` in place of the `TemplateSource` it loses. The
loop crate then needs no opinion about where its caller is installed, and its
book is untouched here. The completion helper `grove-llm` is now taken from the
same directory, so both come from Grove's real path and not from a symlink's.

**A confined program is still canonicalised before it is granted and run.** The
sandbox grants a literal path, so a symlinked program needs its target's path.
It is the same file inspection reported.

**Selection removes only the two live channels from its environment**,
`GROVE_SIGNAL_FILE` and `GROVE_RUN_SIGNAL_FILE`. The retired PID handles stay
named in the loop's scrub module alone: nothing reads them, and the worker
receives only what its owner grants.

**A signal during selection ends `grove` by its default disposition.** Grove
installs no handler for the selection: the signal reaches the group, dispatch
stops its worker, and Grove dies of the signal with nothing launched. The staged
temporary directory is left behind in that case, as it already was for a signal
before the launch. Cleaning it would need a handler and a wait that the
specification does not ask for.

**The `grove run` cases moved to an integration test** that drives the real
binary (`crates/grove/tests/standalone.rs`). The policy is found from HOME, and
a test cannot set HOME for an in-process call. The two ignored native smoke
tests went with the template they took: an owner's real policy is now exercised
by running `grove run` itself.

**The release-notes script builds the pair it runs.** `scripts/release-notes.sh`
runs `task dispatch:build` before it builds `grove`, because cargo does not build
dispatch's policy worker and the freshly built `grove run` finds dispatch beside
itself. The preparation test fakes that task as it already fakes the build. Its
`grove` is a fixture script, so no policy is read there: the real route from a
policy to a confined `release-notes` harness is the confinement seam's, whose
cases all run that kind.

**Three passages outside the specifications followed the code.** The usage
guide's standalone paragraph, the releasing guide's release-notes paragraph and
the headless Codex helper's header named `config.kdl` as what routes
`grove run`. Each now names the policy. The rest of those documents is
`current-state-documents-k15`'s.

**The in-session reviewer was not spent.** No claim here was both unexpected and
beyond what the seam's cases observe. The one property with no earlier
observation, that selection never holds an outer session's channel, has a case
that was seen to fail with the removal taken out.
