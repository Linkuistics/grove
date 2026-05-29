# 090-tui-inbox-triage

**Kind:** work

## Goal

Let inbox entries be viewed, modified, and deleted from within the TUI —
not just viewed.

## Context

Deferred from an inbox observation (2026-05-29), verbatim:

> The TUI needs to allow the inbox entries to be viewed/modified/deleted

Today the TUI can *view* pending observations (the detail screen's
`RightPane::Inbox`, fed by `src/repo_view.rs` `GroveDetail.inbox`) and
*capture* new ones (the `c` modal), but editing or removing an entry requires
the CLI (`grove-llm inbox-drain --for=… --incorporated/--deferred/--rejected`,
or hand-editing files on the `grove-meta` branch). This leaf brings
modify/delete (and, by extension, triage disposition) into the TUI.

Pointers:
- `src/tui.rs` — `RightPane::Inbox` rendering (~`render_grove_detail`,
  line 830) and the key handler; `PendingAction` is the existing "a keystroke
  decides an external action, the loop then runs it" pattern to extend.
- `src/repo_view.rs` — `GroveDetail.inbox: Vec<PathBuf>` is the list of
  pending observation files to act on.
- `src/inboxes.rs` / `src/llm_cli.rs` — `drain_finalize` is the backing
  delete/disposition operation; decide whether the TUI shells out to
  `grove-llm` (consistent with capture) or calls the library directly.

## Done when

- From the TUI inbox view, a pending observation can be deleted/dispositioned
  (incorporated / deferred / rejected) and persisted via the same
  `drain_finalize` path the CLI uses.
- Editing an entry's body is possible (likely via the `$EDITOR` escape hatch,
  reusing the capture modal's Ctrl-E pattern).
- The view refreshes (re-`scan`) so the change is reflected immediately.
- Key handling is covered by a `handle_key`-level test.

## Notes

- Relates to [[080-tui-capture-multiline-paste]] — both touch the TUI's
  inbox/capture surface; consider sequencing them together.
- Larger than a single session may allow; if so, this leaf becomes a node via
  `leaf-decompose` when picked.
