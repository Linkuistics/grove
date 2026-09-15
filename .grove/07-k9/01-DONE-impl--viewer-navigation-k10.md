# viewer-navigation-k10


## Goal
Complete keyboard navigation, pane focus, help and resize in the runnable
manual plain-text browser.



## Context

## Done when
The parent's first two Done when bullets hold through production key handling
and TestBackend frames. Update README, usage, command coverage and changelog;
run focused tests, workspace tests, format/clippy and an executable smoke.
Terminal restoration evidence belongs to the following child.

## Notes
Implementation plan: add failing scripted-key/frame tests; extend the existing
Viewer action seam and terminal key mapping; implement focus-aware navigation,
Unicode horizontal clipping, help, and small-frame state retention; update
usage and run validation. No new dependency or human-crate source edit needed.

## Decisions (running log)

Keep the agreed root interaction design. Key decoding becomes an Action
constructor used by production and tests, avoiding a second test key map.
Plain text stays unwrapped until Markdown; scroll offsets use terminal columns
and source lines, clamped only at usable viewport sizes. Help is a modal page;
quit and refresh remain global, Escape dismisses it, navigation is suspended.

The behavioral doubt check uses the production key decoder and TestBackend
against real temporary trees. The new tests failed for absent focus/help before
implementation and now pass; this executable disproof attempt is more useful
than another read of the same control-flow code. No in-session reviewer spent.
The graph generation 2026-09-15T07:00:01Z reported viewer files as not tracked;
source fallback covered lib.rs, terminal.rs, browser.rs and the viewer manifest.

## Validation

- `cargo test -p grove-tui`: 13 application tests pass, including parent/child
  navigation, collapsed branches, selection visibility, focus, line/page scroll,
  combining and double-width Unicode, resize clamping, help, zero-sized frames,
  global quit/refresh, and names/bytes unchanged after interaction.
- `cargo test --workspace`: passed. Afterward clippy's mechanical replacement
  of `repeat().take()` with `repeat_n()` was verified by rerunning the full
  viewer suite and the checks below.
- `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets`:
  passed. `rustup run 1.85 cargo check --locked -p grove-tui --all-targets`:
  passed. No manifest or lockfile changed.
- SHA-256 comparisons of Cargo.toml, Cargo.lock, the viewer manifest, all viewer
  sources and its browser tests matched before/after final focused validation.
- `cargo build -p grove --bin grove` passed. The actual executable in a 120×24
  PTY displayed the tree, accepted j, Tab, file j and ?, then exited 0 on q;
  captured output included leave-alternate-screen and show-cursor sequences.
  This smoke does not establish termios equality or exceptional restoration:
  `viewer-terminal-k11` owns that evidence and actual PTY resize checks.
- No walkthrough-owned source changed; the human crate, reader and filesystem
  books require no reconstruction edits for this increment.
