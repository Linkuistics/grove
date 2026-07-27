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

## Progress — session 1 (ship half done, observation not started)

**v16.1.0 is shipped and installed.** Tagged, three targets built, GitHub Release
created, tap formula pushed, `brew upgrade` clean; `grove --version` reads
16.1.0. The installed binary carries `report-turn`, `UserPromptSubmit`,
`PostToolUse`, `Notification`, `permission_prompt` and `elicitation_dialog`, and
`grove-llm report-turn --help` renders — the same cheap `strings` check
`ship-release-k25` used, extended to the mid-turn pair. Green baseline before
cutting: `cargo test` — 558 passed, 0 failed.

Two release-path frictions, **both folded into
`release-doctor-toolchain-gap-k27`** rather than fixed here:
`PATH="$HOME/.cargo/bin:$PATH"` is still required (the known doctor gap — with it
set, the doctor passed *and* all three targets built), and `cargo release`
refuses jj's detached HEAD, so the cut needs `--allow-branch HEAD`.

**A version-skew stop is being captured across this session's end.** Shipping
16.1.0 under a 16.0.0 driver *creates* one of the three unobserved rows: at the
next loop iteration the guard sees the skew, and per `plan_for` should report
**`blocked`** and **not** release. Nothing in-session can watch that — it happens
after this session is dead — so a detached poller was started immediately before
the completion signal:

- watcher: `target/k31-pane-watch.sh` (samples `herdr pane get` every 250ms,
  logs only on change)
- log: `target/k31-skew-observation.log`

**Read that log first.** It is the version-skew row, already collected. Both
files are under `target/`, so they are gitignored and would not survive a
`cargo clean` — do not run one before reading.

## What is left

- The headline: a real permission prompt, held past six seconds, reading
  **`blocked`**, and back to `working` on grant. This session's driver injected
  no hooks (its argv had no `--settings`); a session under the 16.1.0 driver
  will. Confirm that first — `ps -o command= -p $PPID` should now show
  `--settings`.
- Same detached-watcher trick works for the two remaining rows, and both are
  cheapest at the very end of a session: **relaunch** (silent, next launch
  re-reports `working`) rides on a normal `grove-llm complete`, and
  **SIGTERM/SIGHUP** (release, report nothing) means signalling the driver — the
  grandparent process, `ps -o ppid= -p $PPID`.
- The pane is `wQ:p1` on workspace `wQ`; the patched server is PID 77248
  (`linkuistics-herdr 0.7.5-linkuistics.1`, live-handoff import). Re-check both
  before trusting a reading.
