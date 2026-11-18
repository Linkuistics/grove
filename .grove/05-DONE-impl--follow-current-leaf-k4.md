# follow-current-leaf-k4

## Goal

Make the tree view follow the current leaf, as the root brief's *Following and
centring* section settles. This leaf adds the cursor rule, the centring
moments, the startup position and the `c` key.

## Context

- `Viewer` in `crates/grove-tui/src/lib.rs`: selection, refresh and the RUNNING
  row binding. NEXT and activity come from the typed observation.
- `docs/specs/item-status.md`: this leaf changes two of its rules, the cursor's
  independence from RUNNING and the minimal viewport movement.
- **Current leaf** in `CONTEXT.md`.
- The fixture in `crates/grove-tui/tests/witnessed.rs` that runs a real driver
  and session.

## Done when

- Every point of the brief's *Following and centring* section holds.
- Application-seam tests show four things. A resting cursor follows a change of
  NEXT. A cursor elsewhere stays. A WAITING interval does not break following.
  The current leaf sits where the brief says, with clamping at both ends.
- One test with the real driver shows the cursor on the RUNNING leaf. When the
  session ends, the cursor moves to NEXT.
- The spec and the usage guide describe following, centring, the startup
  position and `c`.
- `bash scripts/check.sh` passes.

## Notes

- The first observation selects the root. The viewer learns the current leaf
  from that observation, so it then moves the selection and loads that leaf's
  file.
- A tree replacement clears the remembered current leaf with the other saved
  state.

## Decisions (running log)

The viewer remembers the last current leaf that it saw. It observes RUNNING
when an accepted row binds a same-tree session. It observes NEXT otherwise,
which covers idle and a session on a previous tree. A same-tree session
with no bound row is unobservable, so NEXT does not stand in for it.

Before the viewer learns its first current leaf, the cursor sits at the
startup position. It goes to the first current leaf only if the human has
not moved it since startup or the last tree replacement. So a busy startup
never moves a cursor that the human has already placed.

`c` works in both views, because the views share one selection. In File view
the file changes to the current leaf, and the tree centres it at the next
tree render. With no current leaf, `c` shows a notice that stays until the
selection moves or a current leaf appears.

A following move needs a second capture, because a capture reads the file of
the selection that it starts from. The viewer captures at most twice per
refresh. A change that the second capture shows waits for the next poll.

Old tests that assumed a start on the root now open with Home, as a human
can. The PTY tests of the real binary send `gg` for the same reason. The tree-replacement tests keep their intent with a retired reused key,
or they assert the startup position on the new tree's current leaf.
