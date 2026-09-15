# tree-viewer-k3

## Goal

Deliver a basic runnable, permanently read-only plain-text browser through
`grove view [WORKTREE]`: navigate the tree, read tasks/briefs, and manually
refresh. This is the first working slice of the root's final contract.

## Context

- Add the `grove-tui` library with the root's application seam and dependencies.
  `crates/grove/src/cli.rs` dispatches view before jj, driver lease or launch
  configuration resolution.
- Use the existing blocking grove-loop Reading/Tree API and typed snapshot.
  Reuse canonical path composition; expose only the existing helper if needed.
  `responsive-viewer-k8` owns quiet try-read. Copy display data/selected bytes
  and release the guard before layout, input waits and idle time.
- `docs/walkthroughs/overview/walkthrough.toml` reconstructs the human crate's
  manifest/main/CLI. Carry affected pages and ranges with source changes. If
  exposing the path helper changes grove-loop, update its walkthrough too.

## Done when

- `grove view` observes `./.grove`; its optional argument is the containing
  directory. Help states there is no upward search and gives both examples.
  Temporary non-jj directories and concurrent viewers work. Bare grove keeps
  its lifecycle and no selector arguments; assert exactly `{view}` as the
  subcommand set and preserve help/version/description coverage.
- A selectable root and ordered nested branch/task rows include every outcome
  and open-token kind. Use the root's derived branch labels/counts, including
  all-done, all-abandoned, mixed terminal and empty subtrees. Initially select
  root and expand branches; Up/Down or j/k selects rows and Enter toggles a
  branch. Selection
  opens the corresponding file/brief as inert plain text. PageUp/PageDown
  scroll the file so long documents remain readable. q/Ctrl-c exit.
- `r` reloads tree/content; selection may reset to root. Missing/malformed trees
  and selected-file errors have visible states and recover on manual refresh.
  A retained last-good tree is visibly stale after a failed reload. No repair or
  scaffolding occurs. Usage states the temporary blocking-read/diagnostic limit
  on startup, selection and reload; this slice does not claim quit remains
  responsive while waiting for a writer.
- Non-TTY input/output fails before terminal setup. Basic terminal ownership
  restores modes, alternate screen and cursor on ordinary quit and returned
  errors, with fatal messages afterward. An actual-terminal smoke demonstrates
  navigation, file scrolling, refresh and q/Ctrl-c restoration. Full fault and
  signal evidence belongs to `viewer-interaction-k9`.
- Application-seam tests use real temporary trees for nested navigation,
  collapse/expand, root/node brief selection, labels, safe control characters,
  empty documents and long-file reading. Rendering is safe at zero/small sizes.
  Compare names/bytes before and after interactions; viewer state is memory-only.
- README, docs/USAGE.md and docs/specs/user-guide-coverage.md describe this
  subset; G4 retains the bare lifecycle's lack of launch-policy selectors.
  Reconcile docs/specs/module-decomposition.md, docs/ARCHITECTURE.md and
  CONTEXT-MAP.md for the viewer crate, including touched manifests' obsolete
  claim that only grove-loop can be domain-bound.
- The library inherits workspace version, Rust floor and lints, and sets
  `[package.metadata.release] release = false`. Reconcile docs/RELEASING.md and
  release.toml's shipped-package inventory. Check scripts/release-build.sh's
  two-binary delivery: building/installing grove includes the viewer transitively.
  grove-loop and grove-llm acquire no terminal dependencies.

## Decisions (running log)

- Use the approved root design directly. The implementation sequence is:
  application-seam fixtures and CLI contract tests; owned tree capture and
  plain-text rendering; terminal dispatch/lifetime; documentation and byte-exact
  walkthrough reconciliation; required build, test and PTY verification.
- Re-export the existing canonical `entry_path` helper through grove-loop.
  Keep all acquisitions in its current reader, and copy only selected bytes
  while guarded. The viewer never holds a tree guard between actions.
- This human interactive command does not emit structured data. CLI-design's
  human help and non-TTY error guidance apply; a JSON mode does not.
- The one bounded fresh-context review inspected capture and terminal cleanup
  against the contract and reported no material findings. Its source inspection
  supplements the executable application and PTY checks; it is not full
  signal/panic hardening, which remains owned by viewer-interaction-k9.
- The overview book now explains the observation/lifecycle dispatch split.
  Reconciled fragment ownership at the new enum, viewer dependency and public
  path helper; preserved byte-exact validation. Release inventory references
  propagated through manifests and their book fragments.

## Notes

Full pane focus, arrow/hjkl expansion, Home/End, line/horizontal scrolling,
key help and the too-small/resize state arrive in `viewer-interaction-k9`.
Markdown and live observation follow. Keep this slice usable without absorbing
those later leaves to satisfy the root prematurely.

Update CHANGELOG.md under Unreleased. Run focused viewer/CLI and changed loop
tests, affected books' final validation (including ownership/fragments and any
changed corpus assertions), `cargo test --workspace`,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets`,
`rustup run 1.85 cargo check --locked --workspace --all-targets`, and a real
grove executable build/smoke. Record commands and results here.

## Verification

- `cargo test -p grove-tui --test browser`: 6 application-seam tests pass.
  Fixtures cover root/branch/task selection, collapse, all outcomes and branch
  aggregates, manual recovery, selected-file disappearance, control characters,
  empty/long documents, zero/small frames, unchanged names/bytes, and an
  exclusive lock succeeding while two viewers remain idle.
- `cargo test -p grove --test view_command --bin grove`: 4 tests pass. The new
  CLI process tests first failed because view was absent; application fixtures
  first failed against the empty renderer. The existing lifecycle metadata
  assertion now accepts view, while the parser asserts exactly `{view}` and no
  bare launch-policy arguments.
- Final `cargo test --workspace`: **1,183 passed, 0 failed, 0 ignored**, across
  88 test targets, including the changed reader's tests and existing lifecycle
  behavior. `cargo fmt --all -- --check` and
  `cargo clippy --workspace --all-targets`: pass.
- `rustup run 1.85 cargo check --locked --workspace --all-targets`: pass;
  `rustup run 1.85 rustc --version` reports **1.85.1**. The resolved Ratatui
  0.29.0/Crossterm 0.28.1 graph preserves the floor without extra pins.
- `cargo build -p grove -p grove-llm`: pass. `cargo metadata --no-deps` confirms
  seven shipped packages at 21.0.0, only grove released, and grove-tui a library
  with Rust 1.85. `cargo tree -p grove-loop -e normal --prefix none` and the same
  command for grove-llm contain no viewer/Ratatui/Crossterm; grove is the positive
  control containing all three. Read `scripts/release-build.sh`: its explicit
  two-package build and two-binary archive include the viewer transitively.
- `./target/debug/book-check --repo . --book docs/walkthroughs/<book> --final
  --check all`: every book passes with zero deferred lines. Corpus results:
  overview 3 files/224 lines; grove-loop 13/10,458; grove-llm 4/1,015;
  jj-workspace 4/752; keyed-launch 9/2,073; ordinal-fs-tree 17/8,845.
  Ownership ranges, literal fragments and ledgers remain byte-exact. Final
  lookup-label cleanup was followed by another successful overview validation
  and `cargo test -p grove --test reference_navigation --test user_guide_coverage`.
- Actual macOS PTY smoke, using the built grove executable:
  `uv run --with pyte python -u /tmp/grove-view-pty-smoke.py` passed. Two
  concurrent viewers in a non-jj temporary directory exercised default/explicit
  paths, root/branch/task selection, folding, PageDown/PageUp, and r after an
  external edit. A subdirectory invocation showed Missing instead of searching
  upward. q and Ctrl-c both exited 0, restored exact saved termios attributes,
  and emitted leave-alternate-screen/show-cursor output. Names and byte digests
  matched the expected external edit. This was a session-local smoke harness,
  not a shipped fault-injection interface. The supervising shell remained alive
  until attributes were inspected; a separate pipe acknowledged its exit,
  avoiding macOS invalidating the PTY when its session leader exits.
- The final combined run hashed all 1,750 tracked subject files before and after
  the test/build/book/smoke sequence: none changed during measurement. Logs and
  the digest inventory were kept under `/tmp/grove-final-*` for this session.

Delivered limits are the ones this leaf chartered: inert plain text, manual
refresh with root reset, clipped wide lines, and existing blocking reads and
contention diagnostics. Responsive contention, complete interaction/signal
hardening, Markdown and automatic observation remain in their existing leaves.
