# static-dispatch-k46

**Reviews:** static-dispatch-k12

## Goal

An adversarial, inspection-only read of the whole `static-dispatch-k12` node:
the standalone `harness-dispatch` walking skeleton that every later increment
extends. Produce findings, not fixes.

## What to doubt

The node brief names these as the foundation of every later increment and of
the trust boundary, so cheaper to correct now than after later increments build
on them:

- **The protocol.** The framed channel on descriptor 3, the hello and evaluate
  exchange, the 1 MiB frame cap, and whether any frame, diagnostic or policy
  output can reach stdout, the report or another frame.
- **The worker's location and identity check.** Found only relative to the
  real front executable, never PATH, cwd or environment. The protocol, package
  version and source digest are checked before any entry is handed over. Can
  `build.rs` and `scripts/dispatch.sh` compute different digests for the same
  source set without a test failing?
- **Policy authority.** Only the personal default or an explicit `--config`.
  Is there any path by which a repository's code, or an environment variable,
  selects or shadows an entry? That includes the embedded specifiers
  (`harness-dispatch/sdk` and the two `harness-dispatch/examples/…`) against a
  `node_modules/harness-dispatch` shadow.
- **The deadline's hard kill and reaping.** It holds from worker start, the TERM
  grace is at most one second, and then KILL. Is the worker always reaped, on
  every path, including a result that arrives just as the bound expires, a
  worker that exits early, and one whose descendants hold its streams?
- **The durability of the pre-exec commit.** One exclusive transaction at
  `synchronous = EXTRA` with `fullfsync`. No lock is held across evaluation.
  Failures exit 4 and launch nothing, and committed launch fields never change.
  Is there any path that execs without a committed attempt, or turns an
  attempt into a success?

The last leaf, `choice-and-refusals-k15`, added `--choice`, the refusal
contract and the static starter examples. Doubt these as well:

- Does an explicit choice ever select anything but the named candidate, or
  fall back to a route when the ID is unknown?
- Does the equivalent `inspect` invocation a refused `run` prints really
  reproduce the selection? Consider quoting, hyphen-leading values, relative
  paths with the cwd, and non-UTF-8 inputs, which it renders lossily. Can it
  ever leak the prompt?
- Does every refusal name an input or a source? `support::Run::refusal`
  enforces that only for refusals some test reads.
- Do the examples' effort explanations stay out of model ranking, and is the
  Grove example's kind list exactly Grove's?

## Done when

The findings are written into this leaf, each with its evidence, and the leaf
is retired.
