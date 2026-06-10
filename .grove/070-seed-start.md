# 070-seed-start

**Kind:** work

## Goal

Enter on a seed row starts the grove behind a y/n confirm (010-plan Q8): a
confirm modal over the existing `Focus::Modal` machinery, then the same
`grove do <name>` spawn the nav uses for live groves — the seed becomes a
live grove with its harness pane open and focused.

## Context

- `src/tui/focus.rs` `ModalKind` — add a `Confirm` kind (y/Enter confirms,
  n/Esc cancels back to Nav); the modal owns focus while up, like capture.
- `src/tui/launch.rs` / the `NavSelect` path in `app.rs` — for a seed the
  worktree doesn't exist yet; `grove do` creates worktree + branch + opens
  the bootstrap session. Confirm copy should say that ("start grove <name>?
  creates worktree + branch").
- After the spawn, the fleet re-scan flips the row Seed → Live; selection
  should land on the new live grove (rebuild preserves by name).

## Done when

- Enter on a seed opens the confirm; confirm spawns and focuses the new
  harness pane; cancel returns to Nav with nothing spawned.
- Enter on a live grove is unchanged.
- Headless tests: the arbitrate transitions for the confirm modal, and the
  seed-row Enter → confirm wiring.

## Notes

This retires the "Enter silently dead on seed rows" interim state allowed by
030/060.
