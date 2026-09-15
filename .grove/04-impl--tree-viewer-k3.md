# tree-viewer-k3

## Goal

Deliver a runnable, permanently read-only tree and file browser through
`grove view [WORKTREE]`. This first increment is useful on its own: navigate
every task and branch, read their files as plain text, and request a refresh.
The root brief owns the final UI contract; formatted Markdown and continuous
observation arrive in the following increments.

## Context

- `crates/grove/src/cli.rs`: dispatch currently resolves jj, acquires a lease
  and loads configuration unconditionally. Its command-surface assertion also
  assumes no subcommands. Change both for the explicit viewer route.
- `crates/grove-loop/src/lib.rs`: the public `Reading`/`Tree` interface.
- `crates/grove-loop/src/task_tree.rs`: the existing reader, contention
  diagnostic, `entry_path` helper and Grove error restatement.
- `crates/ordinal-fs-tree/src/fs/{mod,lock}.rs`: shared guard acquisition and
  snapshot creation. `read` blocks today; keep its behavior for existing users.
- `crates/ordinal-fs-tree/src/snapshot.rs`: ordered children, node contents and
  distinguished files. The application must not use `pick` as its tree view.
- Add the `grove-tui` workspace library, its application/terminal adapter and
  interface tests. The root brief owns the seam and dependency choices.
- Reconcile `docs/specs/module-decomposition.md`,
  `docs/ordinal-fs-tree/ARCHITECTURE.md`, `docs/ARCHITECTURE.md`, `CONTEXT-MAP.md`,
  `README.md`, `docs/USAGE.md`, `docs/specs/user-guide-coverage.md` and the
  corresponding command/documentation tests where this increment changes their
  claims. Check release manifests/docs and `scripts/release-build.sh` for the
  new library's place in the existing executable delivery.

## Done when

- `grove view` opens `./.grove`; an explicit directory selects that directory's
  `.grove`. It works without jj metadata or launch configuration and alongside
  a real Grove driver. Bare `grove`, help and version retain their contracts.
- A root row, all nested nodes and all live/terminal leaves appear in tree
  order with the agreed labels. Selection opens the correct file or node brief.
  Tree expansion, pane focus, scrolling, key help, resize and quit work as
  specified in the root brief. This increment displays literal text safely.
- A real nonblocking shared acquisition returns busy without printing to the
  terminal, and uses the existing guarded parser when acquired. Expose it
  through Grove's public reader and expose/reuse its existing path composition.
  No snapshot or selected-file bytes are taken on a failed acquisition, and
  every successful read releases its guard before input waits or rendering.
- `r` reloads the tree and file; initial/manual busy reads retry without
  freezing navigation or quit. Show the missing, malformed and file-error
  states from the root brief, with manual recovery after external repair.
  This slice may reset selection to the root on a successful manual reload;
  key-based refresh preservation belongs to `live-viewer-k5`.
- Through the application seam, temporary-tree fixtures demonstrate nested
  navigation, all outcomes, open-token kinds, empty documents, brief selection,
  collapse/expand, scrolling and narrow/zero-sized frames. Compare a recursive
  manifest of names and file bytes before/after all viewer interactions.
- A separate process holding the real exclusive tree lock causes a visible busy
  state while keys/quit still work; release allows a successful read. Conversely
  a real mutator completes while the browser is idle. These tests must have
  deadlines and must not deadlock a test process on its own lock.
- Non-TTY invocation fails before terminal setup. A real PTY smoke check covers
  keyboard navigation, resize, quit, Ctrl-c, SIGTERM and error/panic restoration.
  Include cleanup of partial initialization; do not assume a Drop guard handles
  process termination signals. Capture reproducible observations in this leaf.
- Usage/help/README describe the currently delivered manual-refresh browser.
  The new crate inherits workspace version, Rust floor and lint settings and
  is included transitively when the existing `grove` package is built/installed.
  `grove-loop` and `grove-llm` keep terminal dependencies out of their dependency
  graphs.

## Notes

Use the root's one application seam for behavior tests and only a narrow
additional store test for the new shared-lock acquisition contract. Set up the
failing end-to-end browser example first; land the library work together with
the working browser rather than leaving a reader-only intermediate commit.

Validate with targeted store/loop/viewer/CLI tests, `cargo test --workspace`,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets`,
`rustup run 1.85 cargo check --locked --workspace --all-targets`, and an actual
`grove` executable build. Inspect dependency resolution for the Rust floor;
upstream manifest floors alone do not prove the lockfile builds. Use installed
or directly built `grove-llm`, never `cargo run`, for tree verbs.
