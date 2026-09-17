# modular-parser-k13


## Goal
Delete the unreachable flat parser and template scanner, folding unsupported
top-level rejection into modular capture while preserving diagnostics.



## Context

## Done when
- Flat node validation, duplicate aggregation and legacy word parsing are gone.
- Modular capture rejects flat/mixed documents with the same source spans and
  actionable remedy. The shared shell-comment scanner remains.
- The keyed-launch book reconstructs and explains the changed parser source.
- Public configuration load/expansion tests and `bash scripts/check.sh` pass.

## Notes
The representation child owns CapturedDocument.templates, literal route folding,
inspection type cleanup, final whole-book narrative reconciliation and the
repository-wide forms audit. Do not add a new CLI acceptance suite.

## Decisions (running log)

The parser is a green boundary independent of the public inspection types.
Remove its dead validation and move top-level rejection into named::parse;
temporarily retain an empty captured template map for the existing resolver.
The existing rejection and modular expansion tests are the behavioral seam.

Top-level rejection stays an early phase inside modular capture: collect all
unsupported nodes and return before wrapper contents, preserving the prior
mixed-input diagnostic precedence. The generic at_node remedy now describes
modular input; modular inner-shape and duplicate helpers retain their remedies.

The parser change uses the existing public rejection, diagnostics, hash and
expansion tests rather than a new acceptance suite. In-session review is declined
because executable assertions cover the moved rejection boundary and exact-source
validation covers fragment reconstruction. The remaining subsystem review decision
belongs to modular-representation-k14 once its resolver/public-type artifact exists.

## Validation

`cargo test --locked -p keyed-launch` passed before and after the refactor.
`bash scripts/check.sh` passed all eight principal checks, including the full
workspace suite and all six final walkthrough validators. The keyed-launch book
reconstructs 3,825 source lines across 11 files with no deferred lines.
SHA-256 digests of all 1,797 tracked files (source, tests, scripts, manifests,
documentation and task files) were identical before and after the check run.
The retained shell-comment enum and scanner are byte-identical to the parent.
