# observe-live-surface-k26

**Kind:** impl

## Goal

Prove the status surface works in production, by watching a real `grove do` pane
under a new driver and a restarted server — the two rows of ADR
*herdr-optional-ui*'s table that only a live pane can show, plus the
fork-maintenance acceptance test.

## Context

Runs only once **both** halves are current: `ship-release-k25` put the reporter
in the installed binary, and the human has restarted herdr so the running server
carries the authority patch. A session that finds either half stale should say so
and stop rather than testing against the pre-reporter world — the whole point of
`herdr-notes-reverify-k17`'s finding is that two independent silences produce the
same symptom as a broken patch.

Cheap pre-flight, both from `docs/specs/herdr-fork-maintenance.md`:

- `strings "$(readlink -f "$(which grove)")" | grep pane.report_agent` — the
  driver can report at all.
- `ps -eo pid,lstart,args | grep '[h]erdr server'` against the Cellar install
  time — the server postdates the patched build.

**The restart need not kill every pane.** The spec says it does; that is true of
`herdr server stop` and false in general. herdr carries a live-handoff path —
`server.live_handoff` over the socket, taking an `import_exe`, spawning
`herdr server --handoff-import <sock> <token>` and passing pane fds across — and
`platform::capabilities()` reports `live_handoff: cfg!(unix)`, so it is available
here. The currently-running server was itself started with `--handoff-import`,
which is direct evidence the path works on this machine. Whichever route the
human took, **the spec's flat claim needs correcting** — see Done when.

There is **no CLI subcommand** for handing off into an already-installed binary;
`herdr update --handoff` is the only user-facing path and it fetches *upstream*
herdr, which would clobber the fork. Do not use it. The raw call is one
newline-delimited JSON request to `$HERDR_SOCKET_PATH`, params per
`src/api/schema/server.rs` (all three fields optional):

```json
{"id":"1","method":"server.live_handoff",
 "params":{"import_exe":"/opt/homebrew/bin/herdr","expected_version":"0.7.5"}}
```

Unproven for this particular swap; herdr has rollback logic
(`cleanup_failed_import_child`, `recover_failed_live_handoff_for_update`) if the
import server fails to come up.

**A handoff alone is not sufficient.** It preserves pane *processes* — which
means the old `grove` driver keeps running in every existing pane, and the old
driver is the one with no reporter. This leaf needs a pane whose `grove do` was
started **after** the v16.0.0 upgrade, whatever route the server restart took.

## Done when

- The acceptance test in `docs/specs/herdr-fork-maintenance.md` passes against a
  real `grove do` pane, not a synthetic one: the pane reads `agent: grove` with
  `agent_status` tracking the loop, a *second* differing report lands too, and
  `agent_session` stays exactly as the harness left it.
- `blocked` is observed for a real no-signal stop (`/exit` or Ctrl-C out of a
  grove session), and release is observed on `complete --done`.
- `docs/specs/herdr-fork-maintenance.md`'s restart section is corrected to
  distinguish `server stop` (kills panes) from live handoff (does not), with
  whatever the restart actually demonstrated. It stays a human's call either way,
  but "kills every pane" is not a property of restarting.
- Anything the observation contradicts is written down. A failing acceptance test
  is a genuine finding and belongs in a **new leaf**, not absorbed here.

## Notes

Read `agent` and `agent_status` with `herdr pane current` / `herdr pane get`.
**Not `revision`** — it does not move for a state report, and it *does* move for
a terminal-title change, which a `grove do` pane produces on its own. Two CLI
traps: the pane id is positional and **first**, and the CLI exits 0 on a protocol
error with the failure in the JSON body.
