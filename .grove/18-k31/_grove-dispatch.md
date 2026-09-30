# grove-dispatch-k31 — brief

## Goal

Let a Grove owner launch lifecycle sessions through `harness-dispatch`, by
pointing a personal command definition at `harness-dispatch run` with the
`kind`, `task_file`, `task_id` and `prompt` slots. Grove keeps its foreground-job,
wrapper-exec and completion-authority contracts. Direct-harness configurations
keep working. Both inspection surfaces, and the boundary between them, are
documented where an owner configures Grove.

## Done when

- Grove's existing launch-boundary suite runs a real dispatch route against a
  fake harness. The original prompt and the authoritative kind, task path and
  handle arrive as native data. The final harness receives
  `HARNESS_DISPATCH_RUN_ID` and Grove's fresh completion channel. The policy
  worker receives neither the channel nor any other completion value.
- Task authoring succeeds with a valid wrapper whose delegated policy is
  incomplete. The launch then refuses actionably and leaves the leaf live and
  resumable. The tests and documentation state that Grove's pre-authoring
  guarantee covers only the configured command.
- The controlling-PTY suite shows the dispatched harness keeping Grove's PID and
  process group, the cwd, the terminal and native exits. The entry signal mask
  and dispositions, SIGPIPE included, arrive unchanged. The helper has null
  stdin and a scrubbed environment. Signal cancellation during selection and
  during execution, and descendant escalation, behave as Grove's job contract
  requires.
- Direct-harness compatibility coverage is retained unchanged.
- The configure-grove skill and the Grove usage and configuration documentation
  explain the division of responsibility and activation. Grove's configuration
  owns the wrapper, and dispatch policy owns selection. `grove config show` and
  `harness-dispatch inspect` are the two inspection surfaces. Delegated policy
  is validated at launch, and the documents give the exact remedy for an
  incomplete mapping. `harness-dispatch` help carries a Grove example.

## Decomposition

1. `dispatched-launch-k32`: the launch-boundary cases, completion authority,
   direct-harness compatibility, and authoring-succeeds-launch-refuses.
2. `dispatched-terminal-k33`: the controlling-PTY cases.
3. `grove-dispatch-guidance-k34`: configure-grove, usage and configuration
   documentation, and the Grove help example.

## Pointers

- Spec sections: `#grove-integration`, `#execution-contract` and the two Grove
  rows of `#test-seams`.
- ADRs: `docs/adr/the-launched-child-is-a-job.md`,
  `docs/adr/complete-session-configuration.md` and
  `docs/adr/untracked-configuration-delta.md`.
- `.cargo/config.toml` explains why cargo-run tests clear `GROVE_SIGNAL_FILE`.
  Tests that assert completion-channel delivery must set their own channel
  explicitly and never rely on the ambient one.
- The creator-line lifecycle cases are `creator-reference-k38`'s, not this
  node's.
