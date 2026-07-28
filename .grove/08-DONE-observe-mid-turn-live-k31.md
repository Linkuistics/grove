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

## Progress — session 2 (headline observed; skew observed; two rows in flight)

Preconditions re-checked and both held: the driver injects hooks (parent argv
carries `--settings` with all four rows), and the patched server is still PID
77248, so no second live-handoff was needed.

**The headline is observed**, on claude 2.1.220 — the same version
`herdr-mid-turn-blockers-k30` derived the matcher behaviour against. From
`target/k31-permission-observation.log`:

```
18:34:25  agent=grove status=blocked   ← real permission dialog, up ~10s untouched
18:34:44  agent=grove status=working   ← granted; PostToolUse restored it
```

Both Done-when points, and neither is an artefact of hand-invoking `report-turn`:
the `blocked` came from a real dialog raising a real `Notification`, and the
`working` landed *mid-turn* (the same turn made a further tool call at 18:34:56).
The sidebar rendering was confirmed too — the human saw a red dot, and
`~/Development/herdr/src/ui/status.rs:195-227` maps `Blocked → red` with **red
unique to blocked** across all five rows (`working` yellow, `done` teal, `idle`
green, `unknown` grey).

**The version-skew row is observed** — read from the previous session's
`target/k31-skew-observation.log`: `blocked` 14s after `complete`, held for the
remaining 5½ min, `agent=grove` and `session=herdr:claude` intact throughout. No
release. Exactly `plan_for(Stop::VersionSkew)`.

**`PostToolUse ⇒ working` was also isolated on its own**, before the headline: a
hand-invoked `report-turn waiting` inside a tool call put the pane at `blocked`
(18:31:59), the `herdr pane get` *inside that same call* still read `blocked`,
and it flipped to `working` at 18:32:00 with no report command of mine running.
Nothing else could have done it — the driver reports only at launch and stop.

Four findings that cost time and should not be re-derived:

- **A permissive permission mode raises no dialog at all**, so the mid-turn row
  is unreachable in it. Under `defaultMode: "auto"` plus
  `skipDangerousModePermissionPrompt`, an `rm -rf`, an explicit sandbox override,
  and an un-allowlisted MCP call *all* ran with no prompt. The measurement needed
  the human to Shift+Tab into ask mode. Now a bound in ADR
  *herdr-turn-boundary-hooks*.
- **The six-second timer is gated on human inattention, not elapsed dialog
  time.** Dialogs held several seconds with the human present did not fire it;
  the one that did had sat ~10s untouched. Do not expect to provoke it on demand
  while watching.
- **`herdr pane get`'s `revision` is not a report discriminator** — it stayed at
  9 across three state changes. It tracks pane lifecycle, not agent state, so it
  cannot distinguish "reported the same state" from "reported nothing".
- **`pgrep -P` returns nothing under this sandbox**; `ps -axo pid=,ppid=,command=
  | awk '$2==<pid>'` is what works for child detection.

## Progress — session 3 (both rows recorded; leaf complete)

The detached observer fired cleanly — no `ABORT`, and the `RELAUNCH` marker is
there. Both rows in one run, transcribed here because the log lives under
`target/` and does not survive a `cargo clean`:

```
18:43:26  [0000]  watch start — pane wQ:p1, driver 39105, outgoing session 39119
18:43:26  [0000]  agent=grove status=working session=herdr:claude
18:43:55  [0095]  RELAUNCH — driver 39105 has new harness child 79090 (was 39119)
18:43:55  [0095]             relaunch row asserts: no agent=null above this line
18:44:02  [0119]  SIGTERM -> driver 39105 (grove do)
18:44:02  [0119]             interrupt row asserts: agent=null appears below this line
18:44:04  [0126]  agent=null status=unknown session=herdr:claude
18:44:04  [0126]  RELEASE observed — interrupt row confirmed
18:44:04  [0127]  agent=null status=idle session=herdr:claude
18:44:04  [0128]  agent=null status=unknown session=herdr:claude
18:54:42  [2400]  watch end — phase=done
```

- **Relaunch** — one sample above the marker, `agent=grove status=working`, and
  no `agent=null` anywhere in the phase: the pane was never released across the
  session swap. `plan_for(Stop::Signal(Some(Relaunch)))` exactly.
- **Interrupt** — `agent=null` 2s after the SIGTERM. Released, reporting nothing:
  `plan_for(Stop::Interrupted)` exactly. The 2s is the child's kill grace — the
  driver checks `SessionEnd::Interrupted` and applies the plan only after the
  session it is tearing down is reaped (`src/loop_driver.rs:228-236`).
- **SIGHUP was not separately provoked** and does not need to be: it shares one
  handler and one `Stop::Interrupted` with SIGTERM
  (`src/loop_driver.rs:1259-1278`).

`agent_session` stayed `herdr:claude` throughout, including across the release —
the patch's second hunk, holding under the one path that legitimately releases.

**The silent half of the relaunch row stays unobserved on purpose.** With the
pane already at `working`, "reported nothing" and "re-reported `working`" are
byte-identical from the socket (`revision` is not a discriminator), so proving it
needs a *third* cross-session observer arranging a different pre-state — for a
distinction no operator can see. The policy table is unit-pinned
(`src/herdr.rs:426-463`); the measurement covers the operational claim, which is
that the pane is never released mid-loop.

All four Done-when points are met and the two rows are promoted into the brief
Notes. Nothing is left; the leaf retires here.
