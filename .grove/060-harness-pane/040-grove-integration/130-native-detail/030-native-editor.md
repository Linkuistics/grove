# 030-native-editor

**Kind:** work

## Goal

Make the detail surface's **`$EDITOR` drop** (Ctrl-E) work natively, by running
`$EDITOR <tempfile>` as a **real trellis terminal pane** and **observing its exit**
— the first slice of ADR-0020 §6 **embedded-tool observability**, which
`150-working-set` also needs.

## Context

- **Why not v1's `suspended()`:** host surfaces render in the server daemon, which
  has no tty (ADR-0021; the thin client owns the tty). So `ratatui::restore()` +
  spawn-`$EDITOR` is impossible. Settled at the 130 decompose grilling: editor =
  a trellis terminal pane (vim/etc fully emulated), exit observed.
- **The two v1 editor flows** (`PendingAction` in `tui.rs`):
  - `EditBody` — edit the in-memory capture draft before submit; on exit, the
    edited text becomes `capture.body`.
  - `EditObservation { path }` — edit a *committed* inbox observation; on exit, if
    changed-and-non-empty, round-trip through `grove-llm inbox-edit` (see
    `decide_observation_edit` / `inbox_edit_verb`). The fs-watch then rescans.
- **What trellis must add (the §6 first slice):** a `HostDriver` way to open a
  short-lived command pane and get a signal (tick / callback) when its child
  **exits**, so the surface can read the tempfile back. trellis already tracks pty
  child exit (it draws the red exit-code frame); expose the minimum the host needs.

## Done when

- `Ctrl-E` on a capture draft opens `$EDITOR` in a trellis pane; on exit the edited
  body is read back into the capture modal.
- `Ctrl-E` on an inbox entry opens `$EDITOR` seeded with the entry body; on a
  non-empty change it runs `grove-llm inbox-edit` and the detail rescans; unchanged
  is a no-op with a status line (faithful to `EditOutcome`).
- The editor pane closes itself on exit (no stray pane left in the content region).
- `cargo build`/`cargo test` green. The trellis exit-observability addition has a
  unit test; grove core stays `ratatui`-free below the seam.

## Notes

- **The observability seam may earn an ADR.** It is a reusable framework facility
  (observe + drive an embedded tool), not grove-specific glue — if its shape
  stabilises here, raise the ADR (per `driving.md`); otherwise leave a `BRIEF`
  pointer for 150 to formalise. Keep this leaf's addition the *minimum* the
  `$EDITOR` flow exercises (constraint 4) — exit signal + read-back, not a full
  scrollback/screen/input-injection API.
- **Tempfile is local-filesystem** (the v1 `$EDITOR` assumption holds; the surface
  and the editor pane share the host's fs). A future remote/web client would need a
  different edit affordance — out of scope.
- **Concurrency:** the editor child runs while the surface lives on the screen
  thread; the exit signal arrives as a tick-like wake, mirroring fs-watch. Don't
  block the screen thread waiting on the editor.
