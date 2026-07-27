# status-surface-live-k23

**Kind:** impl

## Goal

Make the pane-state surface **actually live on this machine** and prove it end to
end, so that `herdr-turn-hooks-k4` and `herdr-grove-plugin-k5` are refining a
surface they can watch, not one that exists only in the source tree and the test
suite.

## Context

Surfaced by `herdr-notes-reverify-k17`, which found the surface inert in
production for **two independent reasons**, neither of them a defect:

- **The shipped `grove` has no reporter.** `/opt/homebrew/bin/grove` is 15.0.0,
  and the binary contains no `HERDR_SOCKET_PATH`, no `HERDR_PANE_ID` and no
  `pane.report_agent` — the reporter exists only in `src/herdr.rs` at repo HEAD.
  The repo's `Cargo.toml` is *also* 15.0.0, so HEAD is a pile of unreleased work
  sitting at the released version number. Same reason the shipped `grove-llm`
  refuses `--kind impl` and work here needs `./target/debug/grove-llm`.
- **The running herdr server predates the patched build.** The patch commit is
  dated after the server process started, so the server cannot contain it. This
  is the documented restart trap in `docs/specs/herdr-fork-maintenance.md`, not a
  new problem — but it is currently firing.

Observable right now on a `grove do` pane: `agent: codex`, `agent_status: idle`,
with `agent_session` owned by `herdr:claude`. That is the pre-reporter world
exactly — grove is reporting nothing, and the pane is falling back to screen
detection against the wrong agent's manifests (`herdr-pane-misdetection-k11`).

**Restarting herdr kills every pane**, so that step is the human's call, never an
agent's. Ask; do not do it unprompted.

## Done when

- A grove release ships carrying the reporter and the seventeen-kind taxonomy —
  version bumped off 15.0.0, tap updated, `brew upgrade` clean. Check whether
  anything else at HEAD is unreleased and should ride along rather than shipping
  a second time next week.
- The installed `grove` binary demonstrably contains the reporter (the three
  strings above are the cheap check).
- With the human's agreement, herdr is restarted so the patched server is the one
  running.
- The acceptance test in `docs/specs/herdr-fork-maintenance.md` passes against a
  real `grove do` pane, not a synthetic one: the pane reads `agent: grove` with
  `agent_status` tracking the loop, a *second* differing report lands too, and
  `agent_session` stays exactly as the harness left it.
- `blocked` is observed for a real no-signal stop, and release is observed on
  `complete --done` — the two rows of ADR *herdr-optional-ui*'s table that only a
  live pane can show.

## Notes

The reporter and the patch are both already tested in their own suites; this leaf
is about the **integration being live**, not about re-testing either. If the
acceptance test fails once both halves are current, that is a genuine finding and
belongs in a new leaf, not in this one.

Sequenced ahead of `herdr-turn-hooks-k4` deliberately: turn hooks are a
refinement of a surface, and refining one you cannot observe is how the wrong
thing gets built.
