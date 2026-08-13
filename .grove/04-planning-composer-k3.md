# composer-k3

## Goal

Cut the mandate composer increment into vertical slices and grow the tree. The
deliverable is more tree, not code.

## Context

Read the root `BRIEF.md` and the spec as `specialised-ending-k2` and its review
chain leave it — the design is settled by then, so this leaf plans against amended
text and must not re-open D1–D4.

The increment's parts, as they stand at grilling time. Treat this as the input to
slicing, not as the slicing:

- A composition function over `(units, kind)` on the existing `methodology`
  module seam — the spec agrees this seam and explicitly rejects a separate
  composer seam.
- The file-ordering comment directive, which the spec sequences to arrive *with*
  the composer and not before, plus its duplicate-position build error.
- `content/prompts/continue.md` → `content/MANDATE.md`, reduced to a framing-only
  unit (D1). `content/prompts/` goes with it.
- Re-scoping `skill-signal` and `skill-finish`, and writing the reopened-`finish`
  ending (D2, D4).
- `src/loop_driver.rs:195` `mandate_prompt` composing by kind instead of calling
  `provision::continue_prompt()`. The kind is already in hand at the call site as
  `selection.kind` and is currently unused there.
- Tests: byte-exactness, the completeness invariant over composed mandates, the
  golden per-kind snapshots, the 64 KiB per-kind alarm, and the all-nineteen
  ending guard (D3).

## Done when

- Each slice leaves the product working and delivers something verifiable for
  its successor. The natural fault line is that composition can land and be
  proven by test *before* the driver consumes it, so a slice that changes what
  sessions receive is separable from one that only adds a function.
- The `content/` prose edits are sequenced against the composer, not before it.
  Splitting `skill-signal` while the launcher is still emitted whole and nothing
  selects by kind would deliver **both** endings to a `finish` session — strictly
  worse than the single conditional sentence it replaces. Whichever slice cuts
  the unit boundaries must be the one that also selects by kind, or must follow
  it.
- Provisioning is untouched in every slice.

## Notes

**The behavioural check is deferred and that is expected.** Provisioning stays
live through this whole increment, so every session also receives the unsliced
`SKILL.md` as a harness skill. No slice here can be verified by watching a
session behave differently; each is verified by the composed mandates themselves.
Do not cut a slice whose `Done when` depends on observed session behaviour.

**Review chains are cut lazily, by the sessions that need them** — not planned
here. The spec's own classification pass earned a review leaf; a slice that
rewrites `content/` prose that ships into every mandate is the likely candidate,
and the producer decides that at the end of its own session.
