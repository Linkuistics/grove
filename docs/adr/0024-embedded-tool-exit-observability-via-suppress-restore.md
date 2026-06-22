# 24. Embedded-tool exit observability reuses suppress/restore + a host-pane signal

- Status: **superseded** (was accepted) — Superseded by
  [ADR-0031](0031-shed-machinery-keep-self-extension-core-and-methodology.md) (grove
  sheds its machinery to a self-extension core) and
  [ADR-0032](0032-loop-substrate-is-a-self-driving-shell-loop-not-archon.md) (the loop
  substrate is a self-driving shell loop). The rmux/ratatui TUI + Fleet tower this ADR
  belongs to is **deleted** in leaf `080-shed-tui`; its runtime lives only in git
  history. The decision is retained here as record. (Prior status: superseded by
  [ADR-0028](0028-rmux-substrate.md), rmux substrate, 2026-06-10, 070-teardown D4,
  mechanism-dissolved — suppress/restore exit-observability evaporates with the park
  machinery, ADR-0023. The *concern* — knowing when a wrapped tool exits — survives via
  the SDK, `render_stream` closing / `Pane::wait_exit()`, not suppress/restore.)
- Date: 2026-06-04
- Deciders: Antony Blakey (with grove 060/040/130/030 build)
- Builds on: ADR-0020 §6 (first-class observability of wrapped tools — this is its
  *first slice*), ADR-0021 (the host-pane / `HostSurface` / `HostDriver` seam),
  ADR-0023 (`suppressed_panes` as the alive-but-hidden park primitive, and
  in-place `replace_pane`).

## Context

grove's native detail surface (ADR-0023) needs the v1 `$EDITOR` drop (`Ctrl-E`):
edit a capture draft or a committed inbox observation in `$EDITOR`, then read the
result back. v1 ran `$EDITOR` by suspending the tty (`ratatui::restore()` +
spawn). That premise is gone: a [[host surface]] renders **inside the trellis
server daemon, which has no tty** (ADR-0021). So the editor must run as a **real
trellis terminal pane** (vim/etc fully emulated) and the surface must learn when
its child **exits** to read the tempfile back.

ADR-0020 §6 framed a broad facility — observe and drive embedded tools (screen,
cursor, scrollback, exit, mode requests, input injection). This leaf needed only
the **exit slice**, and `150-working-set` will need the same slice (it embeds
harness/terminal/yazi/lazygit). The question this ADR settles: **what is the
minimum shape of "observe an embedded tool's exit", and does it justify a new
framework mechanism?**

## Evidence

trellis already runs `$EDITOR <tempfile>` as a terminal pane and observes its exit
— its built-in **scrollback editor** (`Tab::edit_scrollback`). The lifecycle, traced
through the vendored 0.44.3 source:

1. **Spawn** `PtyInstruction::OpenInPlaceEditor` runs `$EDITOR <tempfile>` on the
   pty thread; the per-child wait thread fires `quit_cb` on exit, which posts
   `ScreenInstruction::ClosePane`.
2. **Suppress in place** `replace_active_pane_with_editor_pane` swaps the editor
   terminal pane into the **focused** pane's slot and parks the displaced pane in
   `suppressed_panes`, keyed by the editor pid, tagged `is_scrollback_editor`.
3. **Restore on exit** `Tab::close_pane` sees the closing pid is in
   `suppressed_panes` and routes to `replace_pane_with_suppressed_pane`, which puts
   the original pane back in its slot (focus follows) — the editor pane closes
   itself, no stray pane.

The **one thing the scrollback editor does not do is read the tempfile back** — it
is view-only. That readback, plus a signal to the right host surface, is the entire
delta this leaf adds.

A `Box<dyn HostSurface>` value cannot ride a `Clone + Debug` `ScreenInstruction`
(ADR-0023 Evidence #4), but here it need not: the surface is *already mounted* in
its host pane; the editor pane only needs to find that pane on exit.

## Decision

**Reuse trellis's scrollback-editor suppress/restore machinery verbatim; add only a
host-editor tag and an exit signal. No bespoke spawn/observe path.**

1. **`HostDriver::open_editor(tempfile)`** posts `ScreenInstruction::OpenHostEditor
   { host_pane_id, tempfile, client_id }`. The screen thread forwards it to the
   **existing** `PtyInstruction::OpenInPlaceEditor` targeted at the focused pane
   (`ClientId`). Because a host surface only receives keys while its pane is
   focused, "the focused pane" **is** the calling host pane — so
   `replace_active_pane_with_editor_pane` suppresses exactly it and **focus follows
   to the editor for free** (no by-id refocus).

2. **One tag distinguishes a host `$EDITOR` drop** from a real scrollback edit
   (restored pane is a terminal) and a content-swap park (`is_scrollback_editor`
   false): a `Tab::host_editor_panes: HashSet<PaneId>` recorded when the suppressed
   pane is a `PaneId::Host`. On `close_pane`, if the closing pid is tagged, restore
   as usual, then call the restored host pane's `host_editor_exited(exit_status)` →
   `HostSurface::editor_exited`.

3. **The surface owns the tempfile and the readback.** It holds the `NamedTempFile`
   from `open_editor` until `editor_exited`, then reads it and applies the edit (or
   treats a non-zero/signalled exit as "abort, leave unchanged", faithful to v1's
   `shell_editor`). trellis never interprets the file — the framework→host seam
   (ADR-0020 §4) stays one-way: trellis defines `open_editor` / `editor_exited`; the
   host implements the meaning.

The addition is deliberately **only the exit slice** (constraint 4): exit signal +
read-back, *not* screen/cursor/scrollback access or input injection. The broader
ADR-0020 §6 observability API is still owed by a later leaf, designed by its real
consumer.

## Consequences

- **`150-working-set` reuses this slice** for its embedded tools' exit/lifecycle;
  it widens §6 toward screen/scrollback/input-injection only as it needs them, on
  top of `open_editor`/`editor_exited`, not by re-deriving the suppress/restore
  path.
- **No new framework concept and no new spawn path.** The mechanism is the proven
  scrollback editor plus a `HashSet` tag and one `Pane`/`HostSurface` method; the
  full `zellij-server` suite (incl. the scrollback-editor + close-pane tests) stays
  green. The one-way crate seam holds — `zellij-server` owns
  `OpenInPlaceEditor`/`suppressed_panes`/`close_pane`; grove drives them.
- **Tempfile stays local-filesystem** (the v1 assumption: surface and editor pane
  share the daemon's fs). A future remote/web client would need a different edit
  affordance — out of scope (root brief).
- **The focus invariant is load-bearing**: `open_editor` replaces the *active* pane,
  correct only because a host surface is focused whenever it receives the `Ctrl-E`
  that calls it. If a host ever drives `open_editor` from an unfocused surface, the
  wrong pane would be edited; the verb's contract documents "must be the focused
  pane."

## Notes

- The `is_scrollback_editor` bool in `suppressed_panes` was *almost* enough to
  distinguish a host edit (it is true for the host case), but it is also true for a
  real scrollback edit of a terminal pane; gating additionally on "the restored
  pane is a `PaneId::Host`" — via the explicit `host_editor_panes` tag — keeps the
  two intents from ever crossing, rather than overloading that bool.
- The exit status is passed through (`None` = signalled) so the surface can mirror
  v1's "editor exited non-zero ⇒ abort" without the framework deciding policy.
