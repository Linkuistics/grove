# cli-k16

## Goal

Build the CLI that `10-design-cli-shape-k15.md` settled. This is the leaf that
closes increment 1: after it, `ordinal-fs-tree` stands alone with a driver, and
the library has its first end-to-end consumer.

## Context

Beyond the brief chain:

- Whatever `10-design-cli-shape-k15.md` produced. It is the specification for
  this leaf, the way the models are for the operation leaves.
- `linkuistics:cli-tool-design`, again, at the point where help text, exit codes
  and error text are actually written.
- The reference domain from `01-impl-seam-k8.md`, which is what the shipped
  binary drives.

## Done when

- The CLI drives a conforming tree end to end: create, read, and every mutation
  the design exposes, against a real directory.
- Refusals reach the operator as the domain wrote them, recovery advice
  included. A refusal that reaches a human as a bare exit code has discarded the
  one thing the design went out of its way to preserve.
- Its tests are contract tests over the binary, not unit tests behind it — the
  point of this leaf is exercising the library from outside.
- `cargo test` and `cargo clippy --all-targets` are green for both crates.
- The node brief's `Done when` is met. This leaf is the last live one under
  `crate-k7`, so retiring it closes the node: check the brief's conditions
  against what the subtree actually delivered, `leaf-add` any nameable gap,
  escalate a gap you cannot name, and promote anything still live in the brief
  up to the root brief before the close.

## Notes

**This is where the seam gets its first honest test.** Every test before this
leaf was written by someone who had read the architecture document. A CLI forces
a real `Display`, real error text, and a real domain implementation through the
same surface, and any awkwardness that shows up here is a finding about the
seam, not about the CLI.

**Increment 2 starts after this.** It is not cut yet, and cutting it is not this
leaf's job. If this leaf learns something the flip will need, it goes in the root
brief, which is the brief chain every increment-2 session will read.
