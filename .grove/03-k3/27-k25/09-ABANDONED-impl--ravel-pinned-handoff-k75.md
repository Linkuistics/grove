# ravel-pinned-handoff-k75

## Goal

Close the pinned stage with a reproducible matched temporary installation, a
reconciled documentation set and recorded commits. Grove's dual-Ravel stage
and the acceptance stage then use one known build of the complete source
contract.

## Context

- The `## Integration handoff` of `ravel-local-delivery-k23` and the Settled
  inputs of `ravel-pinned-delivery-k25`.
- Ravel `docs/skill-discovery-design-evidence.md`: "Feature line and stage
  handoff" and "Local stage record".
- The precedent `ravel-local-handoff-k61`.

## Done when

- The session fetches Ravel. When `main` moved, it rebases the line, reconciles
  upstream changes and reruns the affected checks. It records the exact base.
- A row-by-row record maps every case of the public acceptance table to its
  fixtures, with the Git part named where a row has one. A row that belongs
  to `common-loading-acceptance-k29` names that owner. A gap earns a leaf
  inserted before this one.
- The documentation set describes the complete source contract and no
  temporary refusal. That set is the specifications, `CONTEXT.md`, the book,
  the examples and `CHANGELOG.md`. All book tasks pass, with
  `task book:integrity`.
- `task check`, `task test:cli`, `task harness:smoke:test` and
  `task test:native-linux` pass on the final commit.
- `task install PREFIX=<temporary prefix>` installs the build, and the
  installed checks pass with an isolated `HOME`. The record names the commit,
  build identity, toolchain and platform. The build reports `ravel 1.1.0`.
- `task smoke:pair OLD=<1.1 executable> NEW=<prefix>` passes. Its directory
  launch includes a local Git source. Otherwise the record states why the pair
  smoke keeps a local source, and which installed check covers Git.
- A "Pinned stage record" in the Ravel evidence document records the base,
  the output commit, the build inputs and the platform facts. The briefs of
  `ravel-pinned-delivery-k25` and `common-skill-loading-k3` name the same
  commit.
- Nobody pushes the line, and no release consumes it.

## Notes

When a producer of this stage cuts a `review-impl` leaf, it inserts that leaf
before this one with `grove-llm leaf-insert`. The review and its integration
then precede the handoff.
