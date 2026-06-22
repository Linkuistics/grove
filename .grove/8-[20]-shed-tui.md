# 8-[20]-shed-tui

**Kind:** work

## Goal

Delete the rmux/ratatui **TUI + Fleet** tower — the single largest machinery
shed (ADR-0031). The self-driving loop (ADR-0032) + Archon's-replacement-is-a-shell-loop
make the TUI's whole reason-for-being (driving harness sessions, multi-grove nav)
obsolete.

## Context

Read **ADR-0031** (machinery, not wisdom, is what sheds) first. The TUI tower is
ADR-0013–0030 and `src/tui/`; it embeds harness panes via `rmux`/`ratatui-rmux`/
`rmux-sdk`, draws nav/detail/capture/whichkey, and resolves a multi-repo Fleet.
None of it survives — `grove do <name>` is now a foreground loop, not a TUI.

## Done when

- `src/tui/` and the `rmux`/`ratatui`/`tokio` dependencies it pulled in are
  removed; the async runtime confined to `src/tui/` (ADR-0028 E1) goes with it,
  returning the core to fully synchronous.
- The `grove tui` / Fleet entry points and their config are removed.
- ADR-0013–0030 are marked **Superseded** (the per-ADR sweep ADR-0031 deferred to
  this leaf), pointing at ADR-0031/0032 and git history.
- `CONTEXT.md`'s "TUI (rmux substrate)" section is removed/condensed (the live
  terms it defines no longer exist) — coordinate with the glossary so it stays a
  truthful map.
- The build is green with the TUI gone.

## Notes

- Sequence **after** 040 proves the new loop works — never delete the old runtime
  before the new one runs (ADR-0031 consequences).
- This is a large deletion; it may decompose (code removal / ADR-sweep / glossary
  reconciliation) if too big for one session.
- Preserve nothing of the TUI UX model — unlike ADR-0028 (which kept the UX,
  replaced the realisation), here the UX itself is retired.
