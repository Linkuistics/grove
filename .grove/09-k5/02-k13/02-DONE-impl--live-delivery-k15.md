# live-delivery-k15

## Goal

Establish the complete live monitor recovery and delivery evidence required by
live-viewer-k5 and the root; close the branch only when their contracts hold.

## Context

live-observation-k12 supplies polling/identity; edit-anchors-k14 adds edit mapping.
Read their retired tasks for evidence, then inspect the current application,
browser tests and PTY support. Do not assume scripted ticks prove real timing.

## Done when

- Add separate real production-clock tests for nested additions/deletions,
  same-length edits, rapid bursts, invalid-then-repaired trees, root replacement
  between polls and removal/delayed recreation. Bound waits and inspect screens;
  no direct Refresh or injected notification substitutes. Compare recursive
  names/bytes after every external mutation to establish no viewer writes.
- A real Grove mutation completes while the monitor stays open. Exercise busy,
  unreadable and interrupted-edit recovery against the root's precise rules;
  add targeted fixes and regressions for actual gaps.
- Build the executable and run real binary PTY smoke for live changes, navigation,
  wide Markdown, ordinary and too-small resize, quit, Ctrl-c/SIGTERM and recovery.
- Confirm view through the existing package/build/install path; reconcile README,
  usage/key help, command coverage, architecture and delivery docs as needed.
- Run focused observation tests, cargo test --workspace, cargo fmt --all -- --check,
  cargo clippy --workspace --all-targets, locked Rust 1.85 workspace check,
  executable build and required book validations. Record commands/results.
- Check every parent and root acceptance clause against evidence before closure;
  create precise follow-up only for actual gaps. Update Unreleased for new changes.

## Notes

Preserve terminal lifetime boundaries and workspace Rust 1.85; no cargo-launched
grove-llm verbs. Source changes in book-owned crates require synchronized fragments,
ownership ranges/manifests/explanations and final affected-book validation.
On this machine the installed floor toolchain is named `1.85` and `cargo` is not
the rustup shim: use `rustup run 1.85 cargo check --workspace --locked`.

## Decisions (running log)

Use the existing approved viewer design and application seam. The graph's
2026-09-15T07:00:01Z generation does not track viewer files; direct source reads
cover the application, observation, terminal loop and browser/PTY tests.
Add separate real-clock application tests using only `tick(Instant::now())`
and production `retry_after` deadlines, with one-second screen assertions and
recursive names/bytes comparisons after external edits. Extend actual-binary
PTY coverage for wide Markdown and recovery while a real Grove mutation runs.
Then run the full workspace, floor, book and package/install checks and reconcile
the parent contracts against the resulting evidence.

The single reviewer found two valid/actionable test issues: successful frames
were accepted before checking the one-second deadline, and concurrent PTY
expected snapshots could absorb viewer writes. Time from before the external
mutation and check the deadline before returning success; derive PTY expected
maps from the known initial fixture and exact external edits. Both changes are
at executable test seams. No second review is needed. The PTY stream contains
Ratatui cell deltas (LIVE→DONE writes only DON); wait for that automatic change,
then resize for a complete readable frame assertion.

The release candidate uses the existing two-package release build and two-binary
archive layout, extracted and installed into a temporary prefix. A test-only
`TEST_GROVE_VIEW_BINARY` override lets the existing PTY suite exercise that exact
installed binary. The initial `GROVE_*` spelling failed the repository's launch
surface classification test; use the test namespace instead of adding a product
configuration variable. Clippy also required `0o0` for two permission literals.
No product source, dependency or book-owned corpus changed.

## Validation

Final commands and results on macOS arm64:

- `cargo test -p grove-tui --test browser production_clock`: all seven passed.
  Each uses real elapsed time, starting before its external mutation, with a
  one-second deadline checked before accepting the rendered frame. There is no
  direct Refresh action or injected notification in these tests.
- `cargo test --workspace`: passed, 1,223 tests across 89 successful test/doc-test
  summaries; includes all 38 browser tests, two terminal fault tests, five actual
  binary PTY tests and command-surface/launch-environment assertions.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets`: passed after the octal-literal fix.
- `rustup run 1.85 cargo check --workspace --locked`: passed. The stronger
  `rustup run 1.85 cargo check --workspace --all-targets --locked` also passed,
  including the new tests.
- `cargo build --locked --release -p grove -p grove-llm`: passed. Both artifacts
  are arm64 Mach-O executables reporting 21.0.0.
- `./target/debug/book-check --repo . --book <book> --final --check all`, for
  every directory in `docs/walkthroughs/`: all six passed with zero deferred
  lines. Grove-llm: 4 files/1,015 lines; grove-loop: 13/10,489; jj-workspace:
  4/752; keyed-launch: 9/2,073; ordinal-fs-tree: 17/8,896; overview: 3/224.
  No corpus or ownership ranges changed.

Package/install verification followed the host branch of
`scripts/release-build.sh`'s build/stage/tar layout and
`scripts/templates/grove.rb.tmpl`'s two-binary installation. Staged the release
pair in `grove-v21.0.0-aarch64-apple-darwin/`, archived with `tar -cJf`, extracted
with `tar -xJf`, and installed with `install -m 755` into a temporary `prefix/bin`.
Archive listing contains only the directory and `grove`/`grove-llm`.
SHA-256 of both installed executables equals the corresponding release build.
The installed `grove view --help` describes automatic read-only observation and
no upward search. Ran:

```sh
TEST_GROVE_VIEW_BINARY=/tmp/grove-delivery-k15.6CWLdH/prefix/bin/grove \
  cargo test -p grove --test view_terminal
```

All five tests passed against that installed pair. This is a local candidate
installation, not a published release or a global Homebrew upgrade. Linux
cross-builds were not run in this host delivery check.

SHA-256 before/after comparisons froze 1,738 inputs: every discovered file under
`crates/`, `docs/`, `testing/`, `scripts/`, `plugins/` and `.cargo/`, plus root
Cargo.toml/Cargo.lock, CONTEXT.md, README.md, CHANGELOG.md and release.toml.
Both the initial run and corrected final run had identical inputs within their
own measurement. Logs and digests are in `/tmp/grove-delivery-k15.6CWLdH/`.
The initial workspace run failed only on the override's former product-prefixed
name; the final full run passed. No production fix was required.

## Parent acceptance and closure

- **Live observation:** the seven new production-clock tests independently
  cover nested additions/deletions, unchanged size/mtime edits, rapid bursts,
  interrupted decomposition and duplicate-key repair, replacement between polls,
  removal/delayed recreation, busy reads and unreadable file/tree recovery.
  Recursive names/bytes comparisons surround observations. Permission-denial
  assertions run when the host actually denies reads; this host did. An open
  descriptor checks selected-file bytes while fresh pathname opens are denied.
- **Identity and reading:** existing application tests cover moved selection
  under collapsed ancestors, rename/retirement, decomposition, root lifetime,
  duplicate rejection, ancestor fallback and unchanged ticks. edit-anchors-k14's
  tests cover insertion above the visible marker, repeated passages, deleted
  anchors, empty content, error recovery, reflow, horizontal offsets and revisits.
  Reconciliation uses keys rather than positions/slugs/outcomes, retaining live,
  done and abandoned items through the same path. All run in the final suite.
- **Reader/navigation:** existing application tests cover task/root/node briefs,
  all outcomes and descendant counts, collapse/expand, focus, visible-row movement,
  Home/End, line/page/horizontal scrolling, help, small/zero viewports and formatted
  headings/lists/tables/code. The new PTY test also reaches a previously hidden
  wide-code marker using real keyboard input.
- **Terminal and mutation:** the real binary recovers from a missing root brief
  while resizing from too-small to normal. Grove's actual write admission and
  leaf retirement finish within one second while the viewer process stays open;
  automatic output changes to DONE and the selected content remains readable.
  Expected filesystem maps derive from the intended external edits. The existing
  PTY suite verifies q, Ctrl-c, SIGTERM/SIGINT/SIGHUP, termios/descriptor flags,
  cursor and alternate-screen restoration, partial input and closed endpoints;
  the separate fault child covers setup/input/draw failure and unwinding panic.
- **Delivery:** README, usage/key help and architecture match the completed live
  monitor and source-edit anchors. Updated release verification and Unreleased.
  The viewer retains workspace version/Rust floor and release=false, ships inside
  the existing human executable, and adds no standalone binary or release lane.

These results satisfy live-reading-k13 and live-viewer-k5, and the root's full
Done when. No additional implementation gap was identified. Promote the delivery
evidence to the root brief; close both parent nodes with this leaf. Whole-grove
finish and teardown remain the driver's subsequent finish session.
