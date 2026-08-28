# syllabus-cli-k17

## Goal

Explain the demonstration CLI as a concrete external consumer of the library,
including every verb, output stream, exit category, and end-to-end operation.

## Context

- Inputs: all preceding book slices, `docs/ordinal-fs-tree/CLI.md`,
  `book-system-k6`, and this subtree's brief.
- Primary source emphasis: `bin/syllabus.rs` and the manifest's CLI feature and
  binary declaration.

## Done when

- The reader understands why the CLI is a syllabus-specific consumer outside
  `src/`, not a generic library interface.
- Parsing and target naming, read verbs, mutation verbs, filters, record output,
  advisory traces, refusal/error rendering, exit codes, idempotency, and omitted
  features are covered without restating clap or standard terminal concepts.
- One command is traced from clap dispatch through reference parts, guard,
  snapshot, algebra, plan, interpreter, report, stdout/stderr, and exit status.
- All CLI source fragments are owned exactly once and the chapter remains
  intelligible without requiring the repository CLI design document.
- Assigned fragments tangle exactly and scoped source, Markdown/link, CLI
  contract tests, and relevant crate checks pass.

## Notes

Keep operator terminology distinct from the library's consumer terminology.
