# ravel-session-control-k7

## Goal

Give every Ravel-launched agent a generic controlled-session notice and explicit
continue/force-stop instructions, with both outcomes observable to any caller.

## Context

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
- Real-worker/fake-agent command tests distinguish continue from force-stop in
  the public caller result and stored observation. Bare exit preserves existing
  continuation behavior for Grove before its next leaf migrates the new case.
- Nested/stale signals remain isolated; repeated exits, outside-supervisor no-op,
  malformed/unsupported inputs and cancellation precedence have observable
  checks. Both endings retain group cleanup, terminal recovery and confinement.
  No caller receives a successful ending before reaping/cleanup confirmation.
- Old records remain readable. Generic inspection/run reporting exposes no
  caller adapter semantics. The private worker stays paired within Ravel only.
  Run Ravel's root check and installed smoke tasks with fake agents.
- Independently release/install the tested Ravel update through its existing
  tasks, recording the CLI/report contract and version for
  grove-session-control-k8. Retain all cargo/test authority guards.

## Notes

Continue returns to a caller's normal decision; it does not guarantee another
launch. Force-stop requests that caller's sequence stop. Ravel implements no
Grove loop or teardown decision. Neither outcome authorizes a daemon, dashboard,
communication service or agent-server design.
