# 010-plan

**Kind:** planning

## Goal

**Eliminate local mode.** The grove name "grove-always-starts-in-local-mode"
and the founding observation were a *bug report*, not a goal: the shipped grove
binary builds with `trellis-seam` **off**, so `grove tui` always falls into the
legacy single-repo in-terminal dashboard (`tui::run`) — which is why it ignored
`~/.config/grove/fleet.toml` (local is fleet-blind). The fix is to make trellis
the one and only TUI: remove the legacy local dashboard, make trellis
unconditional, and stop trellis from inheriting the user's zellij config.

## Context

Founding observation (drained, incorporated): *"Grove always starts in local
mode"* — re-read mid-grilling as the **symptom** the user wants gone.

Two further observations surfaced during grilling:
- `grove tui` didn't pick up `~/.config/grove/fleet.toml` — root cause: the
  shipped binary runs the local path, which is single-repo and never calls
  `fleet::resolve()` (`src/tui.rs:55-66`). The trellis path *does*
  (`src/tui.rs:4516`). Eliminating local **fixes this for free**.
- "We should not be reading the global zellij config. This is not zellij." —
  the trellis path sources `~/.config/zellij` as its config base
  (`src/trellis_host.rs:171-179`); grove embeds a vendored fork, not zellij.

`grove tui` is purely a convenience surface (the actual work happens in
`grove do`, which execs the harness directly; `grove status` is the
non-interactive view). So eliminating local with **no in-terminal fallback** is
acceptable — trellis or nothing.

## Done when

- `grove tui` launches trellis unconditionally; there is no `--local` flag and
  no `tui::run` in-terminal event loop.
- `trellis-seam` cargo feature is gone; `trellis*` crates are unconditional
  deps; no `#[cfg(feature = "trellis-seam")]` remains; build/release config
  carries no `--features trellis-seam`.
- A default `cargo build` produces a binary whose `grove tui` runs trellis (so
  the released binary surfaces `fleet.toml`).
- The trellis path does not read `~/.config/zellij` / `$ZELLIJ_CONFIG_DIR` /
  user zellij themes/layouts; its config is grove-owned only.
- Docs/CHANGELOG/glossary updated to drop the local-vs-trellis split.

## Decomposition

Grown into two work leaves (+ an ADR written during 020):
- `020-eliminate-local-mode` — delete `tui::run` / `--local` / `TuiArgs.local` /
  the `cfg(not)` fallback; make trellis deps unconditional and un-gate every
  `#[cfg(feature = "trellis-seam")]` (the `--server` intercept and the tui
  dispatch both simplify); drop `--features trellis-seam` from build/release;
  update README / CONTEXT.md (`--local` references) / CHANGELOG; write the ADR.
  Keep `tui::dashboard_surface` + the `App`/ratatui rendering — trellis uses it
  as a host surface.
- `030-trellis-ignore-user-zellij-config` — build the trellis config from
  trellis defaults + grove's bundled `GROVE_TUI_CONFIG` only; stop sourcing the
  user's `~/.config/zellij`.

## Decisions (running log)

> **Turnaround note.** Q1/Q2/Q-fleet below were settled on a *wrong reading* of
> the intent (that "starts in local mode" was the goal → flip the default *to*
> local). The user's "what is the point of local mode?" → "let's eliminate
> local mode" reversed that: the title was the bug. The earlier Qs are kept for
> the audit trail but are **superseded** by the Final decision.

### Q1 (superseded) — flip default to local
Originally chose "make local the default, trellis opt-in." Reversed: local is
being eliminated, not promoted.

### Q2 (superseded) — add `--trellis`, drop `--local`
Moot: with local gone there is one mode, so no `--trellis`/`--local` axis at all.

### Q3 (stands) — `trellis-seam` feature gate removed
"We should always have trellis-seam. That shouldn't be a feature." Confirmed and
strengthened: the gate is removed and trellis is *unconditional*. This is also
the root cause of the shipped-local bug (gate off by default in releases).

### Q-fleet (resolved differently) — fleet.toml
Earlier resolution "keep local single-repo" is moot once local is gone. The
trellis path already reads `fleet.toml`, so eliminating local **fixes** the
report rather than route around it.

### Q-zellij (stands) — trellis ignores user zellij config
Trellis config is grove-owned only; do not source `~/.config/zellij`.

### Final — **eliminate local mode; trellis is the only, always-compiled TUI**
`grove tui` = trellis, unconditionally. No local dashboard, no `--local`, no
`trellis-seam` feature. `grove tui` is convenience-only, so no fallback is
owed. Trade-off accepted: every build compiles the forked zellij (build time /
binary size) in exchange for one coherent always-trellis binary that surfaces
`fleet.toml` and owns its own config.

## Notes

ADR candidate (write in 020): "trellis is the only TUI — local dashboard and
the `trellis-seam` feature gate removed." Amends the ADR-0020/0021 framing
(trellis as the supported path) by removing the `--local` escape hatch and the
build-feature gate, and records the convenience-only / no-fallback rationale.
