# responsive-viewer-k8

## Goal

A running Grove mutation no longer freezes the browser. Deliver quiet,
nonblocking reads with a visible busy state and retries, consumed by the
existing runnable `grove view` command in this same slice.

## Context

`tree-viewer-k3` supplies the basic application seam and command. Extend
`crates/ordinal-fs-tree/src/fs/{mod,lock}.rs`,
`crates/grove-loop/src/{task_tree,lib}.rs` and the viewer adapter. Preserve the
existing guarded presence/parser/snapshot path and path-composition ownership.

The root records the deliberate reversal of the no-try/no-timeout rule. Amend
`docs/ordinal-fs-tree/ARCHITECTURE.md`, `docs/ARCHITECTURE.md`'s Tree access lock
section and the library's module/lock comments in this source commit: one
observer try-read may return Busy without a snapshot; ordinary blocking
reads/writes and CLI waiting diagnostics retain their existing behavior.

## Done when

- Attempt the real shared lock without waiting. On success, presence checking,
  canonical parsing and snapshot construction use that same held descriptor.
  Busy is distinct from vacant and malformed/unreadable. Failed acquisition
  captures no tree/selected bytes and emits no output. A preliminary probe
  followed by a blocking read is not sufficient.
- Startup, selection and `r` use this path. Busy retains the last good display
  with a waiting indicator (or an empty waiting view); retry every 500 ms while
  busy without an event backlog. Input/quit remain usable. Missing/malformed/file
  errors retain the root's distinctions; non-busy repair still uses manual
  refresh until `live-viewer-k5`. Drop read guards before layout, input polling
  or another acquisition. Remove the documented blocking limit from usage.
- Narrow store tests cover acquisition, Busy and successful retry. Application
  tests show waiting, navigation/quit responsiveness and content after contention
  ends. Hold a real exclusive lock using an independent open file description,
  in-process or in a helper child, with deadlines. Confirm the fixture on
  supported macOS/Linux targets; no external flock executable is required.
  A separate real Grove mutator completes while the viewer is idle.
- Production fs acquisition remains solely in task_tree.rs. Update
  `crates/grove-llm/tests/tree_lock.rs`'s fs-module mention count and explanation
  for the actual facade, retaining its enforcement across package sources.
  grove-tui never calls flock or uses ordinal_fs_tree::fs directly.
- Update `docs/walkthroughs/grove-loop/` and
  `docs/walkthroughs/ordinal-fs-tree/`: affected manifests, ownership ranges,
  fragments and prose, in the source commit. Update relevant assertions in
  `crates/book-validation/tests/corpus_validation.rs` from the validated corpus.
  If human CLI sources change, reconcile the overview book too. Update
  CHANGELOG.md under Unreleased, usage and affected architecture/module docs.

## Notes

No reader-only intermediate delivery or new release lane. Retain grove-tui's
workspace inheritance and release=false metadata. Demonstrate the command under
contention, compare fixture names/bytes, and record commands/results here.

Run focused store/loop/viewer/lock tests, final validation of affected books,
`cargo test --workspace`, format and clippy checks, the locked Rust 1.85
workspace check, and a real executable build with a contention smoke check.
