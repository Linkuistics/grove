# 020-inbox-disposition-delete

**Kind:** work

## Goal

Let the user disposition the selected inbox observation from the TUI —
**incorporated / deferred / rejected** — persisting through the same
`inboxes::drain_finalize` path the CLI uses. All three delete the file; the
choice only sets the commit-message bucket (faithful to the [[Drain]] model).

## Context

Depends on `010` (a selected inbox entry exists). `drain_finalize` already
takes three path slices (`incorporated`, `deferred`, `rejected`) and deletes +
commits with the `drain <name>: N incorporated, M deferred, K rejected`
message (`src/inboxes.rs`). The TUI should **shell out** to `grove-llm
inbox-drain --for=<name> --<disposition>=<path>` — consistent with how capture
shells `inbox-add` (`shell_capture`) and so the TUI never touches `grove-meta`
git directly.

## Done when

- From the focused inbox pane, a key (e.g. `d` for disposition) on the
  selected entry prompts for one of incorporated / deferred / rejected (a small
  picker/sub-modal, or three direct keys — pick the lighter option).
- The chosen disposition triggers a `PendingAction` that, after suspending the
  terminal, runs `grove-llm inbox-drain --for=<grove> --<bucket>=<abs-path>`
  via the `find_grove_llm` + `suspended` + status-line-on-error pattern
  (mirror `shell_capture`'s error handling: surface stderr, no silent retry).
- After the shell-out the view refreshes so the entry disappears (the fs-watch
  path on `.grove-meta/inboxes` already triggers a debounced rescan — confirm
  it fires; the inbox `ListState` clamps via `010`'s logic).
- A `handle_key`-level test asserts the disposition keystroke on a selected
  entry sets the expected `PendingAction` (with the right bucket + path); a
  test asserting no-op when the inbox is empty.

## Notes

- Single-entry disposition is enough for v1; batch/multi-select is out of
  scope unless it falls out cheaply.
- The picker may want its own small modal-state flag on `App` (like
  `capture.open` / `show_help`) so `handle_key` routes its keys; keep it
  testable without a real terminal (decision happens in `handle_key`, the
  shell-out runs in the loop — the existing split).
- `PendingAction` is currently `Submit` / `EditBody`; add a
  `Drain { path, disposition }`-style variant (or similar) rather than
  overloading the capture variants.
