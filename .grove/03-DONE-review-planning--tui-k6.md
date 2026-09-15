# tui-k6

**Reviews:** tui-k2

## Goal

Adversarially review the read-only TUI design outline and its ordered working
increments against the human-approved root contract. Produce actionable
findings, or establish that no substantive findings remain, before implementation.

## Context

- Read the committed root brief and `tui-k2`, then `tree-viewer-k3`,
  `markdown-viewer-k4` and `live-viewer-k5` as a single planned subtree.
- Follow the source pointers in those leaves. Their parent supplied Tier 2
  evidence at graph generation 2026-09-15T07:00:01Z; reconfirm current coverage
  and check actual source for every load-bearing claim.
- The human explicitly chose these as ordered leaves in this grove, overriding
  the planning skill's separate-grove default. Requirements and the application
  test seam are settled; review how this plan satisfies them.

## Done when

- Contest whether each implementation leaf fits a focused session and delivers
  a runnable behavior on its own, including dependencies, documentation and
  verification. Look for a missing working increment or a hidden design project
  in the first browser leaf, which also adds the quiet try-read consumer.
- Trace the proposed read path against actual shared-lock acquisition, snapshot
  ownership, path composition and diagnostics. Find any way it could freeze the
  UI, retain a guard while idle, read through the wrong lock, duplicate grammar,
  or invoke driver/session mutation from the viewer command.
- Challenge the root-lifetime and selection rules with moves under collapsed
  ancestors, leaf-to-node changes, duplicate keys, selected-file races, brief
  replacement, and removal/recreation entirely between polls. Check that stale
  views, missing-item fallback and saved anchors have unambiguous recovery.
- Check formatted Markdown, source-anchor preservation, terminal restoration
  including partial initialization/signals, actual filesystem-change detection,
  Rust-floor dependency evidence and executable delivery against the proposed
  verification. Flag required behavior that only an injected refresh test or
  a widget-state assertion would appear to prove.
- Findings cite the producer's committed artifact and the relevant contract or
  exact source. Apply the review skill's disposition and integration procedure;
  do not implement fixes in this review.

## Notes

This review is deliberately before `tree-viewer-k3`. If actionable findings
earn an integration step, insert it ahead of that first implementation sibling
so the plan is reconciled before consumers build on it. The integration body
must point to this review's handle rather than copy its findings as obligations.

## Findings

Reviewed at the committed planning artifact `tui-k2` (change `lrursnqsyrwm`,
commit `130a07c99563`): the root brief's design outline and the three
implementation leaves `tree-viewer-k3`, `markdown-viewer-k4`, `live-viewer-k5`.
Every load-bearing claim below was checked against current source, not the
graph. Severity: **high** blocks a leaf from landing green as written;
**medium** leaves a required behavior unspecified or a recorded decision
silently reversed; **low** is a wording or sizing risk.

### F1 (high) — `tree-viewer-k3` is at least two sessions, not one

The leaf's seven *Done when* bullets ask one session for: a new workspace crate
and its manifest; a clap subcommand plus the rewrite of
`crates/grove/src/cli.rs`'s two command-surface tests; a nonblocking shared
acquisition spanning `crates/ordinal-fs-tree/src/fs/lock.rs`,
`crates/ordinal-fs-tree/src/fs/mod.rs`, `crates/grove-loop/src/task_tree.rs`
and `crates/grove-loop/src/lib.rs`, with a public path-composition affordance;
the application seam, tree adapter and terminal lifetime (raw mode, alternate
screen, panic hook, termination-signal handler, partial-initialisation cleanup);
the whole navigation, help, resize and too-small contract; four visible failure
states with manual retry; application-seam tests with a recursive manifest
comparison; lock-contention tests with deadlines; a non-TTY process test; a real
PTY smoke covering SIGTERM and panic; eight named documents; and the full
validation battery including the locked 1.85 check. F2 adds three walkthrough
books and the changelog on top. That is a horizontal stack, and the task file
itself asked this review to look for "a hidden design project in the first
browser leaf". It is one.

Recommended cut, each slice runnable on its own:

- **k3a — runnable browser on the existing blocking reader.** `grove view`,
  root row + task rows with outcome labels, plain-text pane, j/k/Enter/q,
  missing/malformed states, non-TTY refusal, the overview book, USAGE/README,
  coverage inventory, changelog. Contention blocks the UI in this slice and the
  waiting diagnostic prints; documented as the slice's known limit.
- **k3b — quiet try-read and the busy state.** The store/loop extension, its
  narrow store test, the `tree_lock.rs` update (F3), the two library books, and
  its consumer: busy indicator, `r` retry, contention tests. Vertical — the
  user-visible behavior is *a running `grove-llm` verb no longer freezes the
  viewer*. This keeps `tui-k2`'s decision that there is no reader-only leaf.
- **k3c — the rest of the interaction contract and terminal hardening.** Tab
  focus, help overlay, resize/too-small, Home/End, horizontal scroll, panic
  hook, signal restoration, the PTY check (F5).

Whether k3b precedes or follows k3c is the integration's call; k3a first is
not.

### F2 (high) — three walkthrough books, the changelog and release metadata are missing obligations

`docs/specs/walkthrough-books.md` (*An accepted source change requires the
affected ownership ranges and fragments to change … one commit carrying the
source change, every affected manifest and page*) binds every file a book
reconstructs byte for byte. The sources this plan edits are all reconstructed:

- `docs/walkthroughs/overview/walkthrough.toml` — `crates/grove/Cargo.toml`,
  `crates/grove/src/main.rs`, `crates/grove/src/cli.rs`.
- `docs/walkthroughs/grove-loop/walkthrough.toml` — `crates/grove-loop/Cargo.toml`,
  `crates/grove-loop/src/lib.rs`, `crates/grove-loop/src/task_tree.rs`.
- `docs/walkthroughs/ordinal-fs-tree/walkthrough.toml` —
  `crates/ordinal-fs-tree/src/fs/mod.rs`, `crates/ordinal-fs-tree/src/fs/lock.rs`.

This is not only `scripts/check.sh`'s `book-check` pass:
`crates/book-validation/tests/corpus_validation.rs::the_frozen_seventeen_file_corpus_expands_byte_for_byte`
reads the **live** repository sources (`tests/support/mod.rs::repository()` is
`CARGO_MANIFEST_DIR/../..`) and pins 17 files / 8,845 resolved lines, so the
first byte changed in `fs/mod.rs` or `fs/lock.rs` fails `cargo test --workspace`
until the book pages, manifest and that test's counts move together. No leaf
names a book.

Also unnamed: `CHANGELOG.md` (*A session logs its change when it makes it*,
under `## Unreleased`); `docs/RELEASING.md` *One release, six packages, one tag*
and `release.toml`'s *the six crates the release ships* — a seventh inheriting
crate needs `[package.metadata.release] release = false` beside
`version.workspace = true`, or the next cut bumps it separately and writes a
second heading into the changelog (the `book-validation` precedent recorded in
`release.toml`).

### F3 (medium) — the try-read reverses a twice-recorded rule, and the plan does not say so

The rule: `docs/ordinal-fs-tree/ARCHITECTURE.md` *Locking is invisible … Consumers
never mention locking*; `crates/ordinal-fs-tree/src/fs/mod.rs` header *no `try`
variant and no timeout — an API offering any of those would be an API that
mentions locking*; `fs/lock.rs::take` *Blocking, with no way to ask for a
refusal instead*; `docs/ARCHITECTURE.md` §Tree access lock *Locking is invisible
in the library's interface by design — no try-variant, no timeout*. A `Busy`
answer is that API. The plan lists both architecture documents for
reconciliation but presents the extension as an addition, so the implementing
session meets the rule mid-task with no recorded reason to override it.

The extension is still the right route, and the review confirms why the
alternative fails: a viewer-held `LOCK_SH|LOCK_NB` on its own description
followed by the library's blocking `read` would make the library's shared
request wait only on a *waiting* exclusive contender, which is platform-defined
(Linux `flock` grants it; macOS derives `flock` from `lockf` and this was not
verified here), and it would be a second `flock` in Grove, which
`crates/grove-llm/tests/tree_lock.rs` forbids (*Grove never waits on a lock of
its own*). So: record the reversal — amend both documents with the new rule
(*one try-variant exists, for an observer that must never block; it answers
busy and hands back no snapshot*), or an ADR if it clears `ADR-FORMAT.md`'s bar
— and name it in the leaf.

Two mechanical consequences the leaf must name:
`tree_lock.rs::the_librarys_tree_lock_is_taken_from_exactly_one_module` pins
`task_tree.rs` at exactly five non-comment mentions of `ordinal_fs_tree::fs`,
so the new acquisition lives there and the count moves; and the new crate must
never spell `ordinal_fs_tree::fs` or call `flock` (`support::grove_sources()`
enumerates every package's `src/`).

### F4 (medium) — a node's outcome label is unspecified

Root brief: *Nodes are labeled branches; `LIVE` means unfinished work*. Root
*Done when*: *clearly labels live, completed and abandoned work*. A node
carries no outcome — `task_tree.rs::entry_outcome` answers `Outcome::Live` for
every node, and `retire.md` says a node's done-ness *is the absence of a live
leaf anywhere in its subtree* — so a node whose subtree is all `DONE`, all
`ABANDONED`, mixed, or (after a hand edit) empty has no defined label. Define
the derivation once, e.g. `LIVE` if any live leaf beneath; else `DONE` if any
`DONE` leaf beneath; else `ABANDONED`; and say what an empty node shows.

### F5 (medium) — k3's terminal-restoration evidence has no stated mechanism

*A real PTY smoke check covers … SIGTERM and error/panic restoration … Capture
reproducible observations in this leaf.* A panic cannot be provoked in the
shipped binary without test scaffolding, so *observed* panic restoration is
unobtainable as written. Restoration after `q`, Ctrl-c, SIGTERM and a closed
terminal *is* checkable from outside: an integration test in
`crates/grove/tests` spawning `CARGO_BIN_EXE_grove view` under `libc::openpty`
(the crate already takes `libc` for `kill(2)`), then asserting the slave's
termios (ICANON/ECHO restored) and the leave-alternate-screen / show-cursor
sequences in the captured output; closing the master induces the input-error
path. Scope the panic claim to a unit test that the hook is installed and calls
the same restoration function, and say that is the evidence.

### F6 (low) — the contention test does not need a second process

k3: *A separate process holding the real exclusive tree lock … must not
deadlock a test process on its own lock*. `flock` attaches to an open file
description, not a process (`docs/ARCHITECTURE.md` §One lock, and it is the
library's; `tree_lock.rs::lock_worktree` already holds `LOCK_EX` in-process
before spawning). A second `File::open` of the worktree in the test process
plus `LOCK_EX` is the real lock, and the `LOCK_NB` path cannot deadlock against
it. Drop the process requirement; `flock(1)` on this machine is a Homebrew
install, not stock macOS, so a shell helper is not portable either.

### F7 (low) — the "nothing to select" closure property should be restated, not deleted

`cli.rs::the_human_command_surface_has_nothing_left_to_select`,
`docs/specs/user-guide-coverage.md` row G4 (*The working directory as the only
selector*) and `docs/USAGE.md` (*There are no subcommands*) all state one
property: no argument chooses a workstream or launch policy. `grove view
[WORKTREE]` chooses neither. Keep the property — assert the subcommand set is
exactly `{view}` and that bare `grove` still takes no argument — and add the
inventory row; the leaf currently says only "change both".

### F8 (low) — say that viewer state is in-process memory only

*Save reading positions per item for revisits* names no store. The manifest
comparison proves nothing about writes outside the observed tree, so state the
rule: no file, no config, no cache — everything dies with the process.

### F9 (low) — `live-viewer-k5` is also heavy; name its seam now

Polling, lifetime identity, selection/expansion preservation, anchor
reconciliation on edit, duplicate keys, five recovery states, eight
deterministic cases, six real-clock cases, manifest checks, smoke, delivery
confirmation, docs and the root review. Nothing structural is missing, so this
is a sizing note: if it decomposes, the seam is *observation + lifetime +
selection/expansion preservation + missing-item fallback* first, then
*content-change anchor reconciliation + the real-clock recovery matrix*.

### F10 (low) — `WORKTREE` resolves nothing upward while bare `grove` does

Bare `grove` resolves the jj workspace by searching upward; `grove view` run
from a subdirectory observes `<subdir>/.grove` and waits forever. Deliberate
and recorded; the help text should say it in one line.

### Verified, no finding

- Dependency floors as stated: Ratatui 0.29.0 `rust-version = "1.74.0"`,
  Crossterm 0.28.1 `1.63.0`, pulldown-cmark 0.13.0 `1.71.1` (crate archives
  read). `TestBackend::resize`, `crossterm::event::poll(Duration)`,
  `Parser::new_ext`, `Options::ENABLE_TABLES`, `into_offset_iter` all exist.
  `Cargo.lock` holds `unicode-width 0.1.14` and `rustix 1.1.4`; Ratatui pins
  `unicode-width = "=0.2.0"` and Crossterm wants `rustix ^0.38.34` — different
  majors, so two copies each and no resolution conflict.
- Ratatui's `Buffer::set_stringn` already drops graphemes containing control
  characters, so the inert-control-character requirement is bought for every
  rendered cell; only bytes written outside the buffer need care.
- The lock is on the directory *containing* `.grove/` (`fs/lock.rs`,
  `fs/read.rs::containing_directory`), so an open descriptor of `.grove/`
  itself holds no tree lock, as the brief claims. `rmdir` of an open directory
  succeeds on both targets, so the descriptor cannot block a finish teardown.
- The lifetime, moved-under-collapsed-ancestor, leaf-to-node, duplicate-key,
  selected-file-race and between-polls-replacement rules are coherent and each
  has one recovery. One visible trade-off, accepted: a jj checkout of another
  commit keeps `.grove/`'s inode, so keys may name different artifacts inside
  one lifetime; byte comparison and the missing-key fallback bound it.
- The chosen crate placement (a `grove-tui` library behind the thin binary)
  is consistent with `docs/specs/module-decomposition.md` decision 1 once its
  table gains a row and *the one library crate allowed to be domain-bound*
  (root `Cargo.toml`, `crates/grove-loop/Cargo.toml`) is reworded; k3 lists
  the spec.

## Decisions (running log)

F1–F5 are actionable and change how the first implementation leaf is cut, so
this review cuts `integrate-review-planning` ahead of `tree-viewer-k3`, the
first later sibling holding live work, as its last act. The integration body
names this handle and copies no finding. No in-session reviewer was spent: a
`review-*` session is the adversarial read.
