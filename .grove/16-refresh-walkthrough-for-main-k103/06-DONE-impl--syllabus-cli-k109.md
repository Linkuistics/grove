# syllabus-cli-k109


## Goal

Refresh the demonstration CLI chapter against the current
`bin/syllabus.rs`, including destructive behavior and stream contracts.


## Context

- Source root: `crates/ordinal-fs-tree/bin/syllabus.rs`.
- Book surfaces: `07-syllabus-cli.md` and `source-index.md`.
- The prior doubt pass identified stale claims that the CLI has no removal
  command or destructive verb.

## Done when

- The CLI root tangles byte-for-byte and its inventory entry is current.
- Commands, deletion behavior, terminal record encoding, stdout/stderr
  boundaries, failures, and exit behavior match the current implementation.
- Superseded non-destructive-CLI claims are absent from the owned chapter.
- Full validation has no mismatch for the CLI root.

## Notes

Preserve the rebased percent-encoded terminal-record behavior.
