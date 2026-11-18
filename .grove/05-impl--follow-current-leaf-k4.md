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
