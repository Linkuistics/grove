# 030-inbox-edit-verb

**Kind:** work

## Goal

Add a `grove-llm inbox-edit` verb (with an `inboxes::edit` backing) that
rewrites the body of an existing observation file on `grove-meta`, then wire a
TUI Ctrl-E path that edits the selected entry's body via `$EDITOR` and calls
the new verb. Keeps the "TUI never touches `grove-meta` git directly" rule.

## Context

Depends on `010` (a selected inbox entry exists). There is no edit verb today;
the only writes are `inbox-add` (`capture`) and `inbox-drain` (`drain_finalize`).
`capture` is the template to mirror: write the file, stage + commit via
`with_index_lock_retry(|| stage_and_commit(...))`, `push_best_effort`. Input
plumbing reuses `read_body` + the `--body`/`--body-file`/`--body-stdin`
convention (`src/llm_cli.rs`); add an `InboxEditArgs` in `src/cli.rs` alongside
`InboxAddArgs` / `InboxDrainArgs`.

**The filename content-hash invariant — the one real decision here.**
Filenames are `<stamp>Z-<slug>-<hash8>.md`; `hash8 = content_hash_8(body)`
backs `find_by_hash` capture-dedup (ADR-0004). Editing in place desyncs it.
Recommended contract (from the node BRIEF): recompute the hash suffix (rename
so dedup stays correct), **preserve the original capture timestamp** (an edit
doesn't change capture order), **keep the slug**. Use `git mv` so the rename is
one staged change. Confirm or revise this when implementing; weigh a short ADR
if it turns out later work leans on it.

## Done when

- `grove-llm inbox-edit <path> --body|--body-file|--body-stdin` rewrites the
  named observation's body, applies the filename-hash contract above, commits
  (`inbox: edit <name>/<entry>`), and pushes best-effort. Path validation
  reuses / mirrors `validate_path_inside_inbox` (must be a `.md` inside
  `inboxes/<name>/`, never `.gitkeep`).
- `inboxes::edit` has unit coverage: body rewritten, file renamed to the new
  hash with the timestamp preserved, a commit created; rejects out-of-inbox
  paths and `.gitkeep`.
- In the TUI, Ctrl-E on the focused inbox selection seeds `$EDITOR` with the
  entry's current body (reuse `shell_editor`), then on non-empty change runs
  `grove-llm inbox-edit --body-file` via the `suspended` + `PendingAction`
  pattern; errors surface on the status line. View refreshes after.
- A `handle_key`-level test asserts Ctrl-E on a selected entry sets the
  edit `PendingAction` with the right path.

## Notes

- Distinguish from the capture modal's Ctrl-E (which edits an in-memory draft
  body): this Ctrl-E edits an *existing committed observation* and round-trips
  through the new verb. Different `PendingAction` variant.
- An empty edited body should be rejected (mirror `capture`'s empty-body
  guard) rather than producing an empty observation.
- This is the last child; on its retirement the node `090-tui-inbox-triage`
  empties — promote any surviving design note (esp. the hash-invariant
  decision, as an ADR if warranted) upward before retiring the node.
