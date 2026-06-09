# 030-aux-panes

**Kind:** work

## Goal

Wire the three aux tools (plain term / yazi / lazygit-vcs) into the working set:
lazy-spawn on toggle, `t`/`y`/`v` in the leader gate + footer, per-grove
visibility state, hide-not-close. Ties 010 (keying) + 020 (layout) into a usable
per-grove working set.

## Context

010 gave the composite **(grove, role)** key; 020 gave the side-column layout that
stacks visible members. This leaf adds the real aux panes as members of that
column and the toggle spine that shows/hides them.

## Done when

- The leader gate (`src/tui/focus.rs` `arbitrate_pending`) dispatches `t`→Term,
  `y`→Yazi, `v`→Vcs. Each is **toggle-with-focus-follow** (Q6): hidden →
  lazy-spawn (if first time) + show + focus the pane; visible → hide + return
  focus to the harness. The footer (`src/tui/footer.rs`) `LeaderPending` menu
  lists the new keys (`… t term · y yazi · v vcs …`).
- Lazy spawn opens a detached rmux window per aux pane (mirroring `open_or_focus`)
  with **cwd = the grove's worktree** and argv: term = `$SHELL`, yazi = `yazi`,
  vcs = **lazygit** (hardcoded; leave a `// jj detection is a follow-up` seam).
  Resolved below the seam. A missing binary just fails visibly *inside* the pane
  (no PATH pre-check, no toast).
- **Per-grove visibility state**: which aux roles are shown is tracked per grove
  (part of the working set), ephemeral per session. Switching groves restores
  *that* grove's toggled-on set. Toggle-off hides (stops drawing); the pty stays
  warm. Panes close only at TUI exit.
- Aux panes render into their 020 side-column slots via `render_pane`, sized to
  their slot `Rect` on show (resize-on-show). Background aux render pushes do not
  force a redraw unless that pane is currently visible (mirror the existing
  `surface_shows_pane` gate).
- Tests: gate dispatch for `t`/`y`/`v` (pure focus-table tests); visibility toggle
  state transitions; footer menu includes the aux keys. Spawn itself is I/O —
  exercise it via the existing pane-map plumbing, not a live daemon.

## Notes

- **Accepted gap (Q6):** toggle hides a visible-but-unfocused aux instead of
  focusing it. The real fix — click-to-focus hit-testing across side-column rects
  — is a seeded follow-up, out of scope here.
- Esc stays with the embedded tool (`Focus::Pane` forwards all but the leader);
  you leave an aux pane via the leader, not Esc.
- After this leaf the 050-working-set node is complete; its retirement feeds the
  ADR-0022/0023/0024 markings into the 070 teardown (verdicts already in the node
  brief's running log).
