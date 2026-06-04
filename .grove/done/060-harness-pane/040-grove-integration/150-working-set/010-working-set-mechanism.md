# 010-working-set-mechanism

**Kind:** work (trellis-only — the framework change, independently reviewable)

## Goal

Generalise trellis's content region from a **fixed harness+detail pair** to an
**ordered, variable-membership working set** per key, parked/restored as a unit,
with a verb to **toggle a single member** (suppress/restore + re-tile) without
disturbing the rest. This is the foundation the grove side (020–040) composes the
real tools onto.

## Context

- Today `ContentSwap` (screen.rs:1421) holds `primary_slot` + optional
  `secondary_slot`; `SwapContent`/`ContentSpawned` mount/park exactly that pair;
  `content_restore` (tab/mod.rs:2425) restores one slot, parking its displaced
  occupant. The whole pair is parked/restored as a unit across grove-switch.
- The primitives are already proven and ADR'd (ADR-0023): `suppressed_panes` as
  the alive-but-hidden park, in-place `TiledPanes::replace_pane` to mount into a
  fixed slot, the keyed host-surface registry for secondary surfaces that can't
  ride a `Clone+Debug` `ScreenInstruction`.
- This leaf **reuses those primitives for N members** — it does not invent a new
  mechanism. The risk is the layout/re-tile axis: hiding/showing one member must
  re-tile the *visible* members of the working set without touching the constant
  nav or the whichkey bar (the swap's invariant: siblings outside the content
  region are byte-stable).

## Done when

- `ContentSwap` carries an **ordered working set** per key (a list of member panes
  with roles, not just primary+secondary); a grove-switch parks/restores the whole
  set as a unit, harness focused.
- A `HostDriver` verb **toggles one member** of the *currently-mounted* set:
  hide = suppress that member alive + re-tile the remaining visible members; show =
  restore it into the set + re-tile. Fire-and-forget, id/key-addressed (no surface
  rides the instruction — ADR-0023 Evidence #4).
- The constant nav and whichkey bar are **untouched** by a member toggle or a set
  swap (siblings stable, per ADR-0023's sibling-untouched assertion).
- Trellis `zellij-server` suite green, including the existing swap / editor /
  close-pane tests; new tests cover: park/restore a 3+-member set as a unit;
  toggle one member alive (its scrollback survives a hide→show); siblings stable.
- The one-way crate seam holds: `zellij-server` owns the mechanism; the `key` and
  member roles are opaque; trellis never names grove or a grove tool.

## Notes

- The single-`replace_pane`-into-a-fixed-slot model fits a *fixed* set of slots;
  a variable working set means the layout of the content **subtree** changes when
  membership changes. Decide during the build whether the cleanest realisation is
  (a) a content **subtree** grove re-lays-out on toggle, or (b) a small fixed slot
  pool with show/hide. Whichever — keep it the suppress/restore primitive, not a
  new park concept (constraint 4). If a surprising constraint surfaces (à la
  ADR-0023's instruction-can't-carry-a-surface), that earns an ADR.
- Membership/ordering is grove's call (passed in via the driver verb); trellis
  stores and applies it. Responsive *defaults* (which members start visible) are
  040's concern — this leaf only needs show/hide to work for any subset.
- Do not build deeper ADR-0020 §6 observability here — exit observability already
  exists (ADR-0024); screen/scrollback/input-injection stay later/lazy.
