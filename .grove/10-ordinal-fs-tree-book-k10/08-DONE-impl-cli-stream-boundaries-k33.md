# cli-stream-boundaries-k33

## Goal

Make the syllabus CLI's record encoding and terminal-write failures match an
explicit, testable stream contract, then reconcile every affected book
fragment and claim with the accepted source change.

## Context

The adversarial read of `syllabus-cli-k17` found two source-level gaps:

- `Streams::records` writes `Path::display()` unescaped. A Unix root containing
  a newline breaks the promised one-physical-line record shape, and a non-UTF-8
  root is rendered lossily, so the printed path is not generally round-trippable.
- `Streams::records` treats every stdout write or flush error like a benign
  closed pipe and still exits 0, while `eprintln!` on stderr can panic outside
  the documented exit taxonomy.

The book node freezes its fifteen source files during ordinary authoring. This
leaf is the explicit source-defect boundary required by the node brief. Relevant
artifacts include `crates/ordinal-fs-tree/bin/syllabus.rs`,
`tests/driving_a_tree.rs`, `docs/ordinal-fs-tree/CLI.md`, and the book's CLI
page, source index, fragments, and source counts.

## Done when

- The stdout record encoding has a stated domain for arbitrary platform paths,
  including record delimiters and non-UTF-8 names, and contract tests exercise
  the supported round trip rather than only a tab in the root.
- Only the intended closed-pipe case is benign. Other stdout failures and
  stderr failures have deliberate process behavior consistent with the CLI's
  documented exit categories, with contract coverage at a controllable I/O
  seam.
- `CLI.md`, help text, and the book state the same stream and recovery contract.
- Every source fragment and ledger row affected by the accepted source change
  tangles exactly; source counts and ranges are reconciled rather than patched
  around.
- Scoped or final book validation as appropriate, CLI contract tests, and the
  complete `ordinal-fs-tree` crate verification pass.

## Notes

Preserve the distinction between stdout result data, suppressible stderr
advice, and unsuppressible failure reporting. Do not widen `EntryName` or add
paths to the algebra to solve a terminal encoding problem.
