# viewer-interaction-k9

## Goal

Complete the manual browser's interaction contract and terminal cleanup, with
evidence that it restores the terminal on normal and exceptional exits.

## Context

`tree-viewer-k3` supplies basic navigation and terminal ownership;
`responsive-viewer-k8` supplies responsive reads. Extend their application and
terminal adapter. The root owns exact keys, focus, resize and help rules.
Markdown and automatic observation remain subsequent increments.

## Done when

- Tab focus, full arrow/hjkl navigation, Enter/Space expansion, Home/End, file
  line/page/horizontal scrolling, key help and Escape satisfy the root. Keep
  selection visible and focus legible without color. q/Ctrl-c work from either
  pane and help. Resize clamps plain-text scroll without changing selection;
  below 60 columns or 10 rows preserve state behind the resize message while
  accepting quit/refresh, restoring it when panes fit.
- Application tests use scripted keys/sizes to verify displayed behavior,
  including collapsed branches, long Unicode lines and zero-sized frames.
  Compare tree names/bytes; all viewer state remains memory-only.
- Cleanup covers partial setup, input/draw errors, handled termination signals
  and unwinding panic, with restoration before fatal diagnostics. Do not do
  unsafe terminal work inside a raw signal handler or rely on Drop alone for
  process termination.
- An integration test in crates/grove/tests launches the actual grove executable
  in a PTY using openpty or an equivalent supported fixture. Compare slave
  termios with pre-launch values (including ICANON/ECHO), and capture
  leave-alternate-screen/show-cursor output for q, Ctrl-c and SIGTERM. Include
  navigation and resize; bound child waits and reap every child.
- A test-only child calls the production terminal lifetime/cleanup code and
  injects failures after setup stages, input/draw errors and an actual unwinding
  panic. Check restoration with a live PTY endpoint and diagnostic ordering.
  Distinguish this from shipped-binary evidence; no product fault flag. A
  closed-master test proves bounded exit/reaping only where attributes/output
  can no longer be observed. Hook registration alone cannot prove cleanup.
  Record commands and observations here.
- Update key help, README, docs/USAGE.md, command coverage and CHANGELOG.md under
  Unreleased. Changes to human crate sources carry docs/walkthroughs/overview/
  manifest/pages; reader changes carry docs/walkthroughs/grove-loop/ (and
  ordinal-fs-tree's book if touched). Validate affected books byte-exactly,
  updating ranges/fragments/prose and changed corpus assertions together.
  Retain release=false/workspace-version policy; no new package is needed.

## Notes

Run focused application/terminal/CLI tests, affected books' final validation,
`cargo test --workspace`, format/clippy checks, the locked Rust 1.85 workspace
check after dependency changes, and a real executable build/terminal smoke.
Headless frame assertions do not prove terminal cleanup.
