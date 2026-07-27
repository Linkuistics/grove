# observe-mid-turn-live-k31

**Kind:** impl

## Goal

Watch a real `grove do` pane read **`blocked`** while it sits on a real
permission prompt, and read `working` again once the permission is granted —
plus the three rows of ADR *herdr-optional-ui*'s state table that nobody has
ever observed. Everything here needs a live herdr pane; none of it is reachable
from a test.

## Context

**herdr-mid-turn-blockers-k30** shipped the mid-turn pair (`Notification` ⇒
`blocked`, `PostToolUse` ⇒ `working`) and verified everything a subprocess can
reach — see its Notes for exactly what was measured and how. What it could not
reach is the `Notification` half end to end: the notification behind it is
raised by a **TUI dialog component**, so `claude -p` never fires it, and four
attempts at driving an interactive claude under `expect` failed to get the model
as far as a permission prompt (nested-session environment; `auto mode
unavailable for this model`). That path is a rabbit hole, not a missing step —
a human at a real pane sees it in seconds.

This is the same shape as **observe-live-surface-k26**: the implementation is
done and tested; what is missing is one measurement that only production has.

## Done when

- A `grove do` pane parked on a permission prompt for more than six seconds
  reads **`blocked`** in herdr's sidebar (not `working`, not `done`).
- Granting the permission returns it to `working` **without** waiting for the
  turn to end.
- The three unobserved rows of *herdr-optional-ui*'s table are checked on the
  same pane while it is up: **SIGTERM/SIGHUP** (release, report nothing), a
  **version-skew stop** (`blocked`, authority held), and a **relaunch** (silent;
  the next launch re-reports `working`).
- Anything that turns out false is fixed, or recorded — in the ADR if it changes
  the decision, in this grove's brief Notes if it is just a fact.

## Notes

**Ship first, then observe, and the loop already knows how.** The turn hooks are
injected by the *driver*, so the observing session must have been launched by a
binary carrying the mid-turn pair — which the installed `grove` will not be until
v16.1.0 is cut (the CHANGELOG entry is written; `Cargo.toml` is still 16.0.0).
A session cannot upgrade the driver that launched it, but it does not need to:
ship the release, and the **version-skew guard** stops the loop with
"re-run `grove do`". This leaf is not retired at that point, so the re-run
re-picks it and the *next* session — under the new driver — does the observing.
`restart ≡ continuation`, used on purpose.

`release-doctor-toolchain-gap-k27` is the known snag in that release path
(the doctor asks rustup what targets are installed; the build asks whatever
`cargo` is on `PATH`). It is still a live leaf, so expect to hit it.

**Do not use `herdr update --handoff`** to get the patched server running — that
fetches *upstream* herdr and clobbers the fork. `herdr server live-handoff
--import-exe <path>` is the route, and it preserves every pane including the one
that issues it.

**Assert on resting state, never on report count.** With the mid-turn pair the
pane now gets a report at *every tool call*, so the count is not even bounded by
the number of turns (`docs/specs/herdr-fork-maintenance.md`).

**Two known-uncovered cases, so they are not mistaken for defects**: a parallel
tool batch whose sibling outlives the six-second timer lifts the block early, and
a tool that renders its own dialog (`AskUserQuestion`) raises no notification at
all. Both are in ADR *herdr-turn-boundary-hooks* under *Consequences*.
