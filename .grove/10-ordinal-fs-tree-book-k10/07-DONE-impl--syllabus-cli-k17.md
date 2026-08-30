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
  snapshot, algebra, plan, interpreter, report, stdout/stderr, and exit status,
  resolving in full the same operation `orientation-k11` introduced at low
  resolution.
- All CLI source fragments are owned exactly once and the chapter remains
  intelligible without requiring the repository CLI design document.
- Assigned fragments tangle exactly and scoped source, Markdown/link, CLI
  contract tests, and relevant crate checks pass.

## Notes

Keep operator terminology distinct from the library's consumer terminology.

## Decisions (running log)

The CLI file is reconstructed by one composite over seven contiguous literal
ranges aligned with its command-line, parsing/failure, streams/paths,
mutation-output, dispatch, read, and mutation sections. The two CLI-owned
manifest ranges remain separate literals because the manifest root crosses
slice ownership.

The chapter completes the orientation insert before cataloguing the remaining
verbs. It repeats the exact command and starting tree locally, then follows the
same values through CLI parsing and dispatch before briefly restating the
already-explained library layers.

The source and CLI contract required no production change. Consumer-side
filtering, path construction, refusal rendering, stream policy, and exit
classification are explained as properties of the demonstration binary rather
than additions to the library interface.

The fresh-context review found that arbitrary platform paths and terminal write
failures exceed the binary's advertised record and exit contracts. This chapter
states the current limitations without changing frozen source, and
`cli-stream-boundaries-k33` is inserted before assembly to repair the source,
contract tests, CLI design document, and every affected book fragment together.
