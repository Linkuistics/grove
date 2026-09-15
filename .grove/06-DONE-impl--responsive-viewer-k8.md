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

## Decisions (running log)

- Use `TryReading::{Ready(Reading), Busy}` for observer acquisition; retain
  the existing blocking `Reading` contract. Both paths construct the snapshot
  through the same helper while holding the acquired descriptor.
- Keep one pending request in the viewer (whole refresh or latest selection),
  with an absolute retry deadline. Navigation can replace the requested file;
  a pending whole refresh remains pending. Rendering never acquires a guard.
- The leaf's one fresh reviewer found an inherited missing-worktree case:
  removing the observation directory retained STALE content after lock-open
  failed. Classified valid/actionable; a failing application regression now
  covers it. Check absence only after acquisition fails, then clear the view.
- Linux's Rust 1.85 compiler rejected the inherited `instability 0.3.13` and
  `darling 0.24.1` lockfile resolutions (Rust 1.88 required). The initial local
  rustup-cargo command actually drove Homebrew rustc through PATH; an explicit
  `RUSTC` reproduced the refusal. Resolve `instability` to 0.3.7 in Cargo.lock:
  its published manifest declares Rust 1.64 and darling 0.20.10. Keep Ratatui
  0.29/Crossterm 0.28.1 and the workspace floor unchanged. Source inspected:
  https://static.crates.io/crates/instability/instability-0.3.7.crate.

## Implementation plan

1. Add real-lock regression tests for Busy, guarded success, vacancy/error
   distinctions and retry; implement the store and Grove facade together.
2. Add application contention tests; consume the quiet facade on startup,
   refresh and selection, and poll input against one 500 ms retry deadline.
3. Reconcile lock enforcement, usage/architecture/changelog and both affected
   source-exact books, then run the leaf's required validation and binary smoke.

## Notes

No reader-only intermediate delivery or new release lane. Retain grove-tui's
workspace inheritance and release=false metadata. Demonstrate the command under
contention, compare fixture names/bytes, and record commands/results here.

Run focused store/loop/viewer/lock tests, final validation of affected books,
`cargo test --workspace`, format and clippy checks, the locked Rust 1.85
workspace check, and a real executable build with a contention smoke check.

## Validation

- `cargo test -p ordinal-fs-tree --test reading_on_disk observer_try_read`:
  passed on macOS. Independent descriptor contention returns Busy before
  parsing; release permits a guarded retry, and vacancy/error remain distinct.
- `cargo test -p grove-tui --test browser`: all 10 application tests passed,
  including startup, latest selection, manual refresh, coalesced retry,
  non-busy repair, missing worktree and unchanged fixture names/bytes.
- `cargo test --locked -p grove-loop task_tree::tests`: 64 passed.
  `cargo test --locked -p grove-llm --test tree_lock`: all 6 passed, including
  the package-source enforcement of acquisition ownership.
- Linux arm64, official `rust:1.85` container, repository mounted read-only and
  `CARGO_TARGET_DIR=/tmp/grove-target`: `cargo test --locked -p ordinal-fs-tree
  --test reading_on_disk` passed all 17 tests; `cargo test --locked -p grove-tui
  --test browser` passed all 10 after the compatible dependency resolution.
- `cargo test --workspace --locked`: 1,188 passed across 88 result groups,
  zero failures or ignored tests, with the final lockfile.
  `cargo clippy --workspace --all-targets --locked -- -D warnings` and
  `cargo fmt --all --check`: passed.
- Actual Rust 1.85.1: set `RUSTC` to the 1.85 toolchain's absolute rustc path
  and invoke that toolchain's absolute cargo path with
  `check --workspace --locked`; passed. Merely running cargo via rustup was
  insufficient on this machine, as recorded above.
- `target/debug/book-check --repo . --book docs/walkthroughs/grove-loop
  --final --check all`: 13 files, 10,489 lines, no deferred lines.
  The corresponding `ordinal-fs-tree` command: 17 files, 8,896 lines, no
  deferred lines. Updated the fixed test corpus and both total/deferred count
  assertions; byte-exact checks remain enabled.
- `cargo build --locked -p grove -p grove-llm`: passed. The rebuilt binary's
  PTY smoke (`uv run --with pyte python /tmp/grove-k8-smoke.py`) reconstructed
  the actual terminal screen, observed WAITING under a real exclusive lock,
  verified quit within one second and silent stderr, then exercised startup,
  selection and manual-refresh recovery after release. An independent
  `grove-llm leaf-retire 01-impl--first-k1.md` completed within two seconds
  while the viewer was idle in a temporary jj repository. Fixture contents and
  names matched exactly the explicit external edit and retirement; viewing
  caused no changes. Terminal attributes matched the saved attributes on quit.

The one fresh review finding is fixed and covered by an executable regression;
no further actionable findings were reported. Full navigation, terminal
hardening, formatted Markdown and automatic observation remain the next leaves.
