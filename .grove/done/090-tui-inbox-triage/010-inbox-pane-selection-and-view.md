# 010-inbox-pane-selection-and-view

**Kind:** work

## Goal

Make the detail screen's inbox right-pane a **navigable, selectable list** of
pending observations, and render the **body** of the selected entry — replacing
today's static, non-selectable join of filenames. This is the read-side
foundation the disposition (`020`) and edit (`030`) leaves act on.

## Context

Today `right_pane_content` for `RightPane::Inbox` (`src/tui.rs`, ~l.1030)
returns a single `String` made of observation filenames joined by `\n`,
rendered as a non-interactive `Paragraph`. There is no per-entry selection and
no way to read a body without leaving the TUI. `GroveDetail.inbox:
Vec<PathBuf>` (already sorted chronologically) is the data; `repo_view::read_path`
reads a body on demand.

## Done when

- `DetailState` gains a dedicated inbox `ListState` (e.g. `inbox: ListState`),
  kept distinct from the task-tree `tree` state.
- While `RightPane::Inbox` is focused, `j`/`k` (and `Up`/`Down`) move the
  inbox selection rather than the task tree; switching panes via `Tab` leaves
  each pane's selection intact. Selection clamps when the inbox shrinks
  (mirror the existing tree/list clamp logic).
- The inbox pane renders as a selectable list (highlight style like the tree)
  **plus** the selected entry's body — e.g. split the right pane into a short
  list region and a body region, or list-with-body-below. Keep PgUp/PgDn
  scrolling working for the body.
- Empty inbox still renders the existing "(no pending observations)" message.
- The footer/help reflects any new in-pane navigation if it differs from the
  current hints.
- A `handle_key`-level test drives `Tab` to the inbox pane and `j`/`k`,
  asserting the inbox selection moves (and the tree selection does not), plus a
  render-to-buffer test asserting a selected entry's body text appears.

## Notes

- Selection UX recommendation is in the node BRIEF ("Selection UX"). Don't
  overload j/k globally — gate inbox-selection movement on
  `d.right == RightPane::Inbox`.
- Reading bodies on selection is on-demand (`read_path`); the scan still loads
  no bodies. Don't add bodies to the `RepoView` snapshot.
- Pure read-side: no `grove-llm` verb, no `grove-meta` write. One focused
  commit.
