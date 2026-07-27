# status-surface-live-k23 — brief

## Goal

Make the pane-state surface **actually live on this machine** and prove it end to
end, so that `herdr-turn-hooks-k4` and `herdr-grove-plugin-k5` are refining a
surface they can watch, not one that exists only in the source tree and the test
suite.

## Context

Surfaced by `herdr-notes-reverify-k17`, which found the surface inert in
production for **two independent reasons**, neither of them a defect. Both were
re-confirmed live at the start of `ship-release-k25`:

- **The shipped `grove` has no reporter.** `/opt/homebrew/bin/grove` is 15.0.0,
  and the binary contains none of `HERDR_SOCKET_PATH`, `HERDR_PANE_ID`,
  `pane.report_agent` — the reporter exists only in `src/herdr.rs` at repo HEAD.
  The repo's `Cargo.toml` is *also* 15.0.0, so HEAD is a pile of unreleased work
  sitting at the released version number. Same reason the shipped `grove-llm`
  refuses `--kind impl` and work here needs `./target/debug/grove-llm`.
- **The running herdr server predates the patched build.** Server PID 3825
  started `Fri 24 Jul 09:55`; the patched Cellar build was installed
  `27 Jul 01:01`. A process cannot contain code that did not exist when it
  started. This is the documented restart trap in
  `docs/specs/herdr-fork-maintenance.md`, not a new problem — but it is firing.

## Decomposition

Two children, split at a boundary that is physical rather than a matter of
effort. A `grove do` pane's process ancestry is `herdr → shell → grove →
harness`, so the session doing the work is a **grandchild of both things that
have to be replaced**. Getting the reporter live means replacing the `grove`
driver process; getting the patch live means replacing the herdr server. Neither
can be observed from a session descended from the old ones.

- **ship-release-k25** — cut and ship the release, then prove the installed
  binary carries the reporter. Ends by handing the restart decision to the human,
  which is what closes this session.
- **observe-live-surface-k26** — everything that needs a *new* driver under a
  *new* server: the fork-maintenance acceptance test on a real `grove do` pane,
  plus the `blocked` and release rows of ADR *herdr-optional-ui*'s table.

## Pointers

- `docs/specs/herdr-fork-maintenance.md` — the acceptance test, and the two CLI
  traps for driving `herdr pane` by hand (positional pane id first; exit 0 on
  protocol error).
- ADR *herdr-optional-ui* — the report/release table the observation half checks
  row by row.

## Notes

The reporter and the patch are both already tested in their own suites; this node
is about the **integration being live**, not about re-testing either. If the
acceptance test fails once both halves are current, that is a genuine finding and
belongs in a new leaf, not in either child here.

Sequenced ahead of `herdr-turn-hooks-k4` deliberately: turn hooks are a
refinement of a surface, and refining one you cannot observe is how the wrong
thing gets built.
