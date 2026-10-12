# ravel-session-control-k7

## Goal

Give every Ravel-launched agent a generic controlled-session notice and explicit
continue/force-stop instructions, with both outcomes observable to any caller.

## Context

- Requires grove-extraction-release-k13's verified extraction release and k3's
  isolated Ravel store/rename contract. This is the second product increment in
  the same grove; its live driver may still be original Grove/dispatch 22.1.0.

- Work in `/Users/antony/Development/Ravel` against its extracted supervision,
  exit verb, prompt delivery, ending report and record seams.
- The existing policy builds the agent's argv using `request.prompt`. Notice
  injection must happen where the policy receives the prompt it delivers, not
  by guessing a selected harness's argv format. Preserve the caller's task text.
- Existing channel appearance carries no content; run ending and observation
  validation will need one coherent generic extension. Keep implementation
  mechanics focused on this behavior rather than inventing a session protocol.

## Done when

- Public CLI documentation specifies bare `ravel exit` as continue and the
  explicit force-stop spelling. Generic injected prose names those two endings,
  states that termination/reaping belong to Ravel, and knows no Grove conventions.
  A fake agent sees the notice, both commands and the intact supplied task text.
  Use `ravel exit --force-stop` for the explicit spelling. Continue retains
  schema 1/source ravel/observed exit_signal. Force-stop uses a distinct observed
  ending value `force_stop`, never exit_signal plus a field an older reader
  ignores; do not bump the envelope for normal continue. Document the new
  Ravel version supplying the notice and force-stop contract.
- Real-worker/fake-agent command tests distinguish continue from force-stop in
  the public caller result and stored observation. Bare exit preserves existing
  continuation behavior for Grove before its next leaf migrates the new case.
  Feed both reports through the actual lifecycle/standalone readers of the
  extraction release using k5's frozen fixtures: continue is accepted and
  force-stop fails the old continuation/publication predicate. This is a public
  compatibility test, not a Ravel Rust dependency. Include a report with an
  extra field beside exit_signal to prove why that shape cannot mean force-stop.
- Nested/stale signals remain isolated; repeated exits, outside-supervisor no-op,
  malformed/unsupported inputs and cancellation precedence have observable
  checks. Both endings retain group cleanup, terminal recovery and confinement.
  No caller receives a successful ending before reaping/cleanup confirmation.
- Old records remain readable under k3's snapshot/legacy-store rule. Do not
  upgrade the live old supervisor's store or rewrite its immutable documents.
  Generic inspection/run reporting exposes no
  caller adapter semantics. The private worker stays paired within Ravel only.
  Run Ravel's root check and installed smoke tasks with fake agents.
- Independently release/install the tested Ravel update through its existing
  tasks, recording the CLI/report contract and version for
  grove-session-control-k8. Retain all cargo/test authority guards.
  Preserve the original driver/keg/PATH compatibility link through this Ravel
  upgrade and test the old temporary-channel path still works. Do not issue a
  force-stop or new-channel signal to this session's legacy supervisor.

## Notes

Continue returns to a caller's normal decision; it does not guarantee another
launch. Force-stop requests that caller's sequence stop. Ravel implements no
Grove loop or teardown decision. Neither outcome authorizes a daemon, dashboard,
communication service or agent-server design.
