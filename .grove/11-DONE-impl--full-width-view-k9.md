# full-width-view-k9


## Goal

Ship switchable full-width Tree and File views without losing either reading
position. This is the first working increment: a reader can use the terminal's
width immediately, before lifecycle styling or activity observation lands.



## Context

Implement the View switching and chrome contract in `docs/specs/item-status.md`
for navigation, active-view naming and saved positions. Activity summary lines
arrive with idle-next-k11; this increment need not show empty placeholders.

Start with `Viewer::act`, `render`, `save_position`, `restore_position` and
`refresh_at` in `crates/grove-tui/src/lib.rs`. The current renderer always draws
both panes and clamps file state against the file rectangle. Preserve the
Markdown source-anchor behavior in `crates/grove-tui/src/markdown.rs` while
rendering only the active view. Keep hidden-view dimensions and state intact.

Use `crates/grove-tui/tests/browser.rs` through the public Viewer application
seam. Its reflow, revisits, edited files, root replacement and production-clock
tests already cover the important state transitions; adapt their interaction
to the new visible view. Key mapping and terminal cleanup belong to
`crates/grove-tui/src/terminal.rs` and `terminal_tests.rs`.

## Done when

- Tree starts active and occupies the full body width. Tab opens the selected
  root brief, branch brief or leaf task at full width, then returns to Tree.
  Chrome, footer and help name the active view and Tab destination.
- Existing tree selection, parent/child and folding keys, including Enter and
  Space, retain their meanings. File line/page/Home/End/horizontal movement
  applies only in File. Help pauses navigation; refresh and quit stay global.
- Repeated switching preserves selection, expansion and tree viewport, plus
  each file's source anchor and horizontal offset. A hidden view is never
  rendered or clamped against a zero-sized or other-view rectangle.
- At 60 × 10, after shrinking and growing, and after hidden-file edits,
  reflow follows the saved source anchor and Tree keeps the selection visible.
  Revisited files and file errors keep their existing restoration behavior.
- Automatic observation continues in both views, help and undersized frames.
  Switching neither forces a reload nor resets the 500 ms deadline. Key-based
  movement, decomposition, nearest-ancestor fallback and root replacement
  continue to work.
- Application tests inspect visible text and positions at minimum and wide
  sizes; terminal tests cover changed key/help behavior and cleanup. Temporary
  tree filesystem snapshots remain unchanged by viewing.
- Update Viewing a tree in `docs/USAGE.md` and Read-only viewer in
  `docs/ARCHITECTURE.md` for the shipped interaction, retaining the pointer to
  future activity work. Reconcile G6 in `docs/specs/user-guide-coverage.md` for
  the shipped switching behavior, retaining lifecycle/activity obligations.
  Run `cargo test --locked -p grove-tui`, then the root
  brief's principal verification command after all edits.

## Notes

Keep this increment independent of runtime metadata. Lifecycle words remain
visible under the existing format until lifecycle-rows-k10. If a substantive
viewport doubt remains after validation, cut a review-impl leaf with the bare
stem full-width-view after producing the artifact; do not pre-create integration.

## Implementation plan

1. Add public Viewer regression tests for full-width switching, independent
   viewports, hidden-file edits and unchanged polling deadlines; observe failures.
2. Gate body rendering and file page actions on the existing active-view state
   in `crates/grove-tui/src/lib.rs`; preserve observation and Markdown anchors.
3. Adapt browser/key tests to explicitly open File and return to Tree, retaining
   their existing filesystem, source-position and terminal-cleanup assertions.
4. Update the usage, architecture and G6 coverage documents; run the focused
   crate suite and `bash scripts/check.sh`, then retire and seal this leaf.

## Decisions (running log)

- Keep the existing boolean view state and independent ListState/file reading
  state. Render only the active body; hidden File retains its last dimensions
  until it is displayed, when existing source-anchor reflow applies. Switching
  itself performs no observation and leaves the poll deadline untouched.
- Browser assertions now visit File explicitly; production PTY fixtures use
  Tab for root/task reading and return to Tree to observe retirement. The long
  link fixture keeps enough trailing content for its marker to reach the top
  at full width, preserving the existing transformed-source anchor assertion.
- The single in-session adversarial review found a test adaptation inspecting
  an old WAITING frame instead of the current STALE frame. Classified actionable
  and restored the current-frame binding; no production defect was reported.
