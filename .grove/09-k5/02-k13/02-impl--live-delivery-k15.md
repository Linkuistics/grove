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
