# 020-rip-out

**Kind:** work

## Goal

Rip the old TUI substrate out for a clean slate (010-plan D5/D6: replace-in-place,
trellis deleted first). Delete `crates/trellis/` (the vendored zellij fork) and
`crates/harness-pane/` (the shelved in-process-pty fallback) and every reference to
them, leaving the workspace building with `grove tui` disabled until 030 lands.

## Context

trellis is currently the one, always-on, unconditional TUI (ADR-0026) — an unconditional
path dependency, so `grove tui` and any `trellis`/`harness-pane` import must be removed
together. `harness-pane` is already shelved (ADR-0015 fallback, not built on) so it goes
in the same sweep. `grove tui` being down is accepted (D5): groves are driven via
`grove-llm`/`grove status` meanwhile.

## Done when

- `crates/trellis/` and `crates/harness-pane/` are gone (directories + workspace
  `exclude`/members + any `build.rs` embeds).
- All `use`/path-dep references removed; `cargo build`/`cargo test` green.
- `grove tui` is a clean disabled stub (clear "TUI is being rebuilt on rmux" message),
  not a broken/panicking path.
- Bundled config/layout/KDL + install paths that only existed for the zellij substrate
  are removed (or stubbed for 030 to repopulate).
- ADR bookkeeping is NOT done here — the dissolution sweep is the teardown leaf (per D4);
  this leaf only deletes code. Leave the old ADRs untouched for now.

## Notes

Watch for: the `bugs` grove's committed-but-broken trellis floating-pane change is moot
(superseded). Capture anything trellis-specific worth remembering before deleting (most
is superseded glossary already). The grove→trellis edge being unconditional means the
build won't go green until every reference is excised — expect a sweep, not a snip.
