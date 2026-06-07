# 020-diagnose-and-fix

**Kind:** work

## Goal
Root-cause and fix the blank render of grove's native host surfaces (the
`grove-nav` pane and the `grove-whichkey` bar) in `grove tui`, and land a
regression test that would have caught it.

## Context
Read the root `BRIEF.md` first — it carries the full pointer set. Beyond it:

- Apply **systematic-debugging**: form hypotheses and confirm the *actual* root
  cause before writing the fix. Drive it from a **failing reproduction test**
  rather than a speculative patch.
- The fault is **downstream of grove's off-screen `Buffer`**. Proven facts from
  the observation: `App::render` TestBackend tests pass; embedded *terminal*
  panes render fine (so trellis pane compositing broadly works); blank persists
  with the fs-watch stubbed; server alive at ~0% CPU (a render gap, not a hang).
- Prime suspect: `crates/trellis/zellij-server/src/panes/host_pane.rs` —
  `HostPane` render → `CharacterChunk` compositing, and how grove's surface
  `Buffer` reaches it. Compare grove's `DashboardSurface::draw` / `whichkey`
  copy-into-`buf` (`src/tui.rs` `mod native`) against what `HostPane` actually
  reads back and emits as chunks. Also check `Tab::inject_host_pane` /
  `inject_whichkey_pane` geometry (`tab/mod.rs`) — a zero/degenerate render rect
  would produce blank output with no error.
- A likely shape: the off-screen buffer is populated but `HostPane` reads the
  wrong region/origin, never ticks an initial draw, or emits empty chunks
  (e.g. selective-redraw / dirty-tracking only emitting on *change*, with no
  initial full paint). Confirm, don't assume.

## Done when
- Root cause identified and stated (in this leaf or, if durable/surprising, an
  ADR under `docs/adr/`).
- A trellis-level regression test renders a host surface through the
  `HostPane` → `CharacterChunk` path and asserts non-blank cells reach the
  composited output. It fails before the fix, passes after.
- The fix lands (in `crates/trellis/` and/or `src/`); `cargo test` green.
- Manual tmux confirmation: `grove tui` shows the nav list and the whichkey
  hint line populated. (Confirm both surfaces — verify the single-root-cause
  hypothesis; if they have *separate* causes, fix both or split a follow-up
  leaf.)

## Notes
- Trellis edits are in scope (grove-owned hard fork, ADR-0020/0021).
- If diagnosis reveals the fix is large or multi-part, stop and decompose this
  leaf rather than forcing it into one session (grove stays lazy — constraint 4).
- Verification: use the project's TUI run path for the tmux check; see
  CONTEXT.md `host surface / host driver / host tick` and
  `crates/trellis/HOST_API.md` for the contract.
