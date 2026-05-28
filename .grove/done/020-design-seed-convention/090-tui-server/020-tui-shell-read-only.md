# 020-tui-shell-read-only

**Kind:** work

## Goal

Ship the read-only TUI shell: the `grove tui` subcommand, the Ratatui +
crossterm event loop, both screens (grove list, grove detail) with
master/detail layout, and the navigation keybindings agreed in the
parent BRIEF. No writes. No filesystem-watch yet — refresh on a manual
keypress is fine for this leaf, fs-watch lands in leaf 030.

After this leaf, `grove tui` is a usable read-only navigator. A user
can run it, see all groves in the repo, drill in, browse the task tree,
read leaves and inboxes and briefs, and exit.

## Context

Parent BRIEF settled:

- Subcommand: `grove tui` on the existing `grove` binary.
- Stack: Ratatui + crossterm, sync event loop.
- Two screens, master/detail framing.
- Navigation: arrow keys + j/k, Enter drills, Esc/q pops, `/` filters,
  Tab cycles right-pane mode, `?` for help.

The data layer from leaf 010 (`RepoView` or equivalent) is the only
state source — render directly off it, do not invent a parallel model.

Wire the `grove tui` subcommand into `cli.rs` alongside the existing
verbs. If launching from inside a worktree (`<repo>/.grove-worktrees/<name>/`),
pre-select that grove on the grove-list screen. Detection lives in
`repo.rs` already — reuse.

## Done when

- `cargo run --bin grove -- tui` launches the TUI from any worktree
  and produces the grove-list screen.
- Grove-list screen: one row per grove, columns for name,
  live/retired leaf counts, inbox pending count, lifecycle badge.
  Current grove (if launched from a worktree) is pre-selected.
- Grove-detail screen: left pane shows the task tree as a navigable
  list (depth-first walk, retired under `done/` greyed); right pane
  toggles between (a) selected leaf body, (b) inbox listing, (c)
  current node's `BRIEF.md`. Tab cycles the right pane.
- Navigation keys behave as specified in the parent BRIEF.
- `/` opens a filter input that narrows the current pane's list by
  substring.
- `?` shows a help overlay listing all keybindings.
- `q` from grove-list quits; `Esc` from grove-detail returns to
  grove-list.
- Ratatui + crossterm are added as dependencies; the `grove` binary
  still builds with no warnings.
- Manual snapshot test: a fixture repo with one live grove and one
  seed renders both screens correctly. Snapshot via
  `insta` + `ratatui::backend::TestBackend` if the rest of the
  codebase already uses `insta`; otherwise a single hand-eyeballed
  walkthrough recorded in a brief docstring is acceptable for v1.

## Notes

- **No state mutation in this leaf.** Every keybind either changes
  the selected row, switches panes, opens help, or quits. No CLI
  shell-outs. The `c` keybind is introduced in leaf 030 alongside
  the actual shell-out, not stubbed here.
- **Pre-selection from worktree** is a small but high-leverage UX
  detail. The user runs `grove tui` from inside the work they care
  about; landing on a sorted list with their grove pre-selected
  beats forcing them to find it.
- **Filter (`/`)** is per-pane. On grove-list it filters groves; on
  grove-detail's left pane it filters leaves. Don't try to make it
  smart — substring match is fine.
- **Right-pane content rendering:** markdown can be rendered with
  `tui-markdown` or shown as plain text. Plain text is cheaper and
  fine for v1 — the leaves and briefs are short. Pick whichever the
  implementer finds less intrusive; mention the choice in the commit
  message so the future grove can revisit.
