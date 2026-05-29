# 090-tui-inbox-triage — brief

## Goal

Bring inbox **view / modify / delete** into the TUI. Today the TUI only
*views* a flat list of observation filenames (`RightPane::Inbox`) and
*captures* new ones (`c`); acting on an existing entry still requires the
CLI (`grove-llm inbox-drain …`) or hand-editing files on `grove-meta`.

Deferred from an inbox observation (2026-05-29), verbatim:

> The TUI needs to allow the inbox entries to be viewed/modified/deleted

## Done when

- The inbox right-pane is **navigable**: the user selects an individual
  pending observation and reads its body, not just a filename list.
- A selected observation can be **dispositioned** — incorporated / deferred
  / rejected — persisted through the same `inboxes::drain_finalize` path the
  CLI uses (all three delete the file; the choice only sets the commit-message
  bucket, faithful to [[Drain]]).
- A selected observation's **body can be edited** from the TUI, via a new
  `grove-llm inbox-edit` verb and the `$EDITOR` escape-hatch pattern (Ctrl-E),
  so the TUI never touches `grove-meta` git directly.
- After any write the view **refreshes** (the fs-watch path already does this;
  confirm it fires) so the change is reflected immediately.
- Key handling is covered by `handle_key`-level tests at each leaf.

## Decomposition

Split by surface, in dependency order — each is one focused commit:

- `010-inbox-pane-selection-and-view` — make the inbox pane a selectable list
  with its own `ListState` and render the selected entry's body. Pure
  read-side; the foundation the two write leaves build on. No new verb.
- `020-inbox-disposition-delete` — wire incorporate/defer/reject from the
  selected entry through `drain_finalize` (shell out to `grove-llm
  inbox-drain`, consistent with how capture shells `inbox-add`).
- `030-inbox-edit-verb` — add the `grove-llm inbox-edit` verb +
  `inboxes::edit` backing, then the TUI Ctrl-E-on-selected-entry path. The one
  leaf carrying a genuine design decision (the filename content-hash
  invariant — see Notes); the others are mechanical.

## Pointers

- `src/tui.rs` — `RightPane::Inbox` rendering (`right_pane_content`, ~l.1030)
  and `render_grove_detail`; `DetailState` holds per-pane state; `handle_key`
  routes detail-screen keys. `PendingAction` + `process_pending_action` +
  `suspended()` is the "keystroke decides an external action, loop runs it
  after suspending the terminal" pattern to extend. `shell_capture` /
  `shell_editor` are the existing shell-out helpers; `find_grove_llm` locates
  the sibling binary.
- `src/repo_view.rs` — `GroveDetail.inbox: Vec<PathBuf>` (sorted observation
  paths); `read_path` reads a body on demand. Scan loads no bodies by design.
- `src/inboxes.rs` — `drain_finalize` (the delete/disposition backing),
  `capture` (the write+commit+push template a new `edit` mirrors),
  `content_hash_8` / `derive_slug` / `utc_iso8601_seconds` (the filename
  components), `list_observations`, `with_index_lock_retry`,
  `push_best_effort`.
- `src/llm_cli.rs` — verb dispatch; `cmd_inbox_drain` shows the
  enumerate-vs-finalize two-phase shape; `read_body` shows the
  `--body`/`--body-file`/`--body-stdin` input convention a new `inbox-edit`
  reuses. `src/cli.rs` holds the `InboxAddArgs` / `InboxDrainArgs` structs
  (an `InboxEditArgs` joins them).
- Glossary terms in play: [[Inbox]], [[Drain]], `grove-meta branch` (see
  `CONTEXT.md`). ADR-0004 (inbox file shape — the content-hash filename),
  ADR-0005 (sync/push semantics), ADR-0006 (grove / grove-llm binary split).

## Notes

**The edit filename-hash invariant (decide in `030`).** Observation
filenames are `<stamp>Z-<slug>-<hash8>.md`, where `hash8` is
`content_hash_8(body)` and is used by `find_by_hash` to make capture
idempotent (ADR-0004). Editing a body in place desyncs that suffix. Recommended
contract for `inboxes::edit`: **recompute the hash suffix** (rename the file so
dedup stays correct), **preserve the original capture timestamp** (chronological
ordering is capture order, which an edit does not change), and **keep the slug**
(a cosmetic human hint — re-slugging on every edit is churn). Commit message
`inbox: edit <name>/<entry>`; push best-effort like capture. If this proves
load-bearing for later work, the `030` session should weigh a short ADR rather
than leaving it only in this brief.

**Selection UX (decide in `010`).** When `RightPane::Inbox` is active, j/k
should drive a dedicated inbox `ListState` (add to `DetailState`) rather than
the task tree, and the selected entry's body renders below the list (or the
list + a body view share the right pane). Keep the existing Tab-cycles-pane and
PgUp/PgDn-scroll model; the disposition/edit keys (`020`/`030`) act on the
inbox selection and are live only while the Inbox pane is focused.

**Sequencing.** Relates to the now-retired `080-tui-capture-multiline-paste`;
both touch the TUI inbox/capture surface. `010` is a hard prerequisite for
`020`/`030` (they need a selected entry to act on); `020` and `030` are
independent of each other.
