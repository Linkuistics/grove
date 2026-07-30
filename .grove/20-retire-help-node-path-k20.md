# retire-help-node-path-k20

**Kind:** impl

## Goal

Replace `grove retire --help`'s original-scheme node-path example with the
current stable-keyed directory grammar.

## Context

Executing generated help during `confirmation-prose-review-k16` printed
`003-session-store` as the example `<PATH>`. Current nodes are named
`NN-<slug>-k<key>/`; the old three-digit, keyless form was removed by ADR
*task-tree-scheme*. This is help text a human copies at the terminal, not tagged
history.

## Done when

- The source argument help demonstrates a current node path (or a stable handle
  if the command accepts one) and no longer teaches `003-<slug>`.
- A CLI test pins the current example or grammar so the generated help cannot
  regress silently.
- The relevant test suite passes and `grove retire --help` is executed to verify
  the rendered text.

## Notes
