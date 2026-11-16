# E1 current arm, attempt 1: infrastructure failure

The first E1 run of the current arm started at about 02:00 on 2026-11-17. It did not finish.

- In turn 1, the session followed the visual discussion step of the `grove-design` skill. It wrote
  a viewer under `.design/retry-policy/` and started `python3 -m http.server 8765` in the
  background, so that the human could open the page. Both arms contain this step.
- `claude -p` waits for its background tasks before it exits. The server never ends, so the call
  stayed open for about 16 minutes.
- The operator stopped the server at 02:30. A shell quoting error in the operator's own command then
  started a second server on port 8765 from a different directory. The session requested
  `/diagrams.json` from that server and received a 404. The session then started a new server on
  port 8766.
- The operator stopped the run at about 02:33. That run made no proxy turn and wrote no spec. The
  runner stored no output for it.

The operator's server answered a request from the session, so the run is not clean. Under the
brief's rule, the run is repeated once. Before the repeat, `tools/run.py` gained a watchdog. When a
call has emitted its result and stays open for 300 seconds, the runner stops the call's process
group. Each call records `stopped_after_result` in `meta.json`. The watchdog does not change the
prompt or the skills.
