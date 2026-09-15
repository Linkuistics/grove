# live-viewer-k5 — brief

## Goal

Make the formatted browser a live monitor: external changes appear automatically
while the selected item's identity, reading position, expansion and focus stay
stable wherever the current tree and content support them. Complete the root's
full observable contract and delivery verification.

## Context

- `tree-viewer-k3` supplies the runnable application and temporary-tree seam;
  `responsive-viewer-k8` supplies quiet try-read and `viewer-interaction-k9`
  supplies the full interaction and terminal-restoration evidence.
- `markdown-viewer-k4` supplies formatted layout with source ranges and scroll
  anchors. Extend that application's observation and reconciliation behavior.
- The root brief specifies polling, root lifetime, duplicate keys, disappearance,
  decomposition, recovery and the exceptions to selection/expansion retention.
- Grove's generic snapshot can contain duplicate keys and its key lookup chooses
  a match. Validate key uniqueness before accepting a viewer snapshot; do not
  infer a unique item from that lookup alone.

## Done when

- The production event loop polls every 500 ms with at most one pending refresh;
  it reads selected bytes even when file size/mtime appear unchanged. On local
  temporary fixtures changes reach the display within one second. UI input,
  resize and quit remain usable while another process holds the tree lock.
- Permanent keys preserve selection across insertion before the item, renumber,
  slug edits, moves, retirement, abandonment and leaf-to-node decomposition.
  The new node brief becomes selected on decomposition. Focus, horizontal
  offset and reading anchor persist where applicable; unchanged refreshes do
  not jump, and changed content maps/clamps the source anchor coherently.
- Expansion persists by node key, with the selected item's new ancestors
  revealed after a move. A missing selected item falls back to its nearest
  surviving old ancestor or root, with a visible disappearance notice. Removed
  state is discarded. Reused keys from a replaced root never inherit selection
  or reading positions, even when replacement occurs entirely between polls.
- Busy, malformed/unreadable trees, unreadable/disappearing selected files,
  duplicate keys, root removal/recreation and interrupted external edits obey
  the root's visible stale/error/waiting rules and recover without user input.
  A root brief edited or atomically replaced is still the same root lifetime.
- Deterministic application tests drive refresh actions after real fixture edits
  and assert displayed results for each case above, including a moved selected
  item under a collapsed branch, a retired selected item, a selected leaf that
  becomes a node, unchanged ticks and content inserted above a visible marker.
- Separate integration tests use the real production clock/change-detection path
  against temporary files: do not call refresh directly in these tests. Exercise
  nested additions/deletions, same-length selected-file edits, rapid bursts,
  invalid-then-repaired trees, root replacement between polls and removal with
  delayed recreation. Bound waits with deadlines and assert the expected screen.
- After each external edit, compare the recursive filesystem names/bytes against
  the expected external mutation, establishing that the viewer contributed no
  writes. A real Grove mutation also completes while the monitor is open.
- The actual terminal smoke check covers live updates, navigation, wide Markdown,
  resize including the too-small view, quit, Ctrl-c/SIGTERM and terminal recovery.
  Confirm the shipped `grove` executable has the viewer command through the
  existing package/build/install path. Record the commands and observed results.
- README, usage examples/key help, command coverage, architecture and delivery
  documentation describe the completed monitor. Review the root's Done when
  against the delivered evidence and add a precise follow-up only for a real gap.
- Update CHANGELOG.md under Unreleased. Source edits in the human crate carry
  docs/walkthroughs/overview/ updates; reader-facade edits carry
  docs/walkthroughs/grove-loop/ updates; fs changes carry the ordinal-fs-tree
  book. Change affected manifests, ownership ranges, fragments, explanations
  and corpus assertions together, with final validation of each affected book.
  Preserve grove-tui's workspace-version/release=false policy and reconcile
  release inventory only if delivered package membership changes.

## Decomposition

Inspection found refresh resets the entire view, positions use paths, and the
Markdown anchor has no source edit mapping. live-observation-k12 delivers the
first runnable live monitor; live-reading-k13 completes content edit mapping and
the full real-clock recovery/delivery matrix. The parent contract remains whole.

## Notes

The first child remains a runnable live monitor with its temporary source-offset
reading-position limitation documented; the second completes the full contract.

The clock-triggered refresh and scripted refresh must enter the same application
operation. Polling is the production change detector, so a test of an injected
notification alone cannot establish live observation. Never hold a read guard
between ticks, across input polling or in saved display/scroll state.

Keep a descriptor to the observed root for lifetime identity without keeping its
parent's tree lock. Changes by editors outside Grove's advisory lock can race a
capture; discard inconsistent results and retry, rather than claiming an atomic
filesystem snapshot or treating partially observed data as authoritative.

Run the focused observation tests, `cargo test --workspace`,
`cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets`, the
locked Rust 1.85 workspace check, and a real executable build before the final
terminal smoke. A passing headless frame test is not terminal-lifetime evidence.
