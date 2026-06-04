# 020-detail-surface — brief

**Kind:** node (work)

## Goal

Build 130's headline acceptance (minus `$EDITOR`): selecting a grove in the
**constant nav** mounts that grove's **detail** (task tree + inbox + capture)
beside its `grove do <name>` **harness** in the **content region** (ADR-0022),
two groves selectable in turn with no cross-talk, parked-alive when not selected,
restored on switch-back.

## Why this is a node (decomposed from a leaf)

The sibling `010-content-swap-spike` only **decided** the park/mount mechanism
(ADR-0023, throwaway not retained — "the build leaf writes the real swap verb").
The substrate does **not yet exist**: the current code is still the **tab model**
(nav = home tab, `Enter` → `new_command_tab` opens a [[workspace]] tab). Reaching
the acceptance therefore means two distinct, sequential bodies of work — a deep
**trellis** content-swap substrate, then a **grove** detail surface mounted
through it — each independently testable. Bundling them into one leaf would land a
half-built substrate and half-built surface in one commit. Split per the
substrate→surface seam (matches ADR-0023's own evidence shape).

## Decomposition (this node)

```
010-swap-substrate  (work) trellis: the constant-nav + content-slot layout; keyed
                    host-surface registry + id-only MountHostSurface; HostDriver
                    swap verb (replace_pane into the slot, displaced pane parked in
                    suppressed_panes); harness `grove do <name>` opened into the
                    content slot (NewPane Tiled, not a tab); nav Enter → swap
                    (retiring the OpenHarness→new_command_tab path); retire the
                    GoToTab/Alt-1..9 binds. Acceptance: two groves' HARNESSES swap
                    in/out of the content slot beside the constant nav, parked alive
                    (scrollback intact) and restored on switch-back. No detail yet.
020-mount-detail    (work) grove: per-grove DetailSurface (scoped App, detail-only)
                    mounted beside the harness — the content region becomes a split;
                    park/restore the harness+detail PAIR as a unit; per-grove
                    fs-watch; triage (`d`) + capture (`c`/Ctrl-S) in-process;
                    Ctrl-E shows the "lands in 030" pointer. Two groves, own detail,
                    no cross-talk. Closes 130's headline acceptance.
```

The two child leaves carry the detailed "Done when" + build notes; this brief
holds only the seam between them. The hard pair-park/restore (a flat
`suppressed_panes: HashMap<PaneId,…>` parking a harness+detail *pair*) lands in
`020-mount-detail`, once `010` has proven single-pane swap/park/restore.

## Decisions carried in (ADR-0023, unchanged)

- **Swap = in-place `TiledPanes::replace_pane` on the content slot** — incoming
  pane inherits the slot geom; displaced pane parked in the tab's
  `suppressed_panes`, kept alive by the existing pty/resize routing (~15 sites).
  The park primitive `suppress_pane_and_replace_with_other_pane` already exists.
- **The host-pane seam widens one-shot → N via a keyed registry + an id-only
  `MountHostSurface { key }`** — *not* a surface carried in a `ScreenInstruction`
  (the enum is `Clone+Debug`; a `Box<dyn HostSurface>` is neither). `HostDriver`
  gains the swap verb. The one-way crate seam (ADR-0020 §4) holds: `zellij-server`
  defines the registry / `replace_pane` / `MountHostSurface`; grove drives them.
- **Reuse the v1 `App` detail rendering** (detail-locked mode), reuse the
  `process_action` `shell_capture`/`shell_drain` shell-outs; the only deferral is
  `$EDITOR` (→ 030).

## Notes

- **Scope boundary with 150-working-set:** the detail/harness split here is a
  minimal fixed choice; the *responsive* multi-pane content region
  (terminal/yazi/lazygit, pack-vs-degrade, toggles) is 150's.
- Depends on **110** (host API) and **120** (the nav surface + leader), both done.
