# Complete session configuration

Every Grove session kind maps to one complete command-template string in the
personal `~/.config/grove/config.kdl`; Grove executes the expanded argv directly
and does not infer a harness, model, defaults, or hidden harness-specific
arguments. This binds because launch policy includes choices Grove cannot own —
model, reasoning effort, approval, sandboxing, wrappers, and harness-specific
Herdr behavior — while splitting those choices across environment precedence,
repository stamps, task metadata, and a built-in harness registry made the
effective command impossible to inspect or validate as one value. The
configuration deliberately repeats all nineteen targets so every session is
complete in isolation and adding a kind fails visibly in every old config.

## Considered options

- **Keep a primary harness and layer kind/family overrides over it.** Rejected
  because the result depends on a precedence lattice and still asks Grove to
  understand harness model flags and defaults. Reopen only if Grove becomes a
  harness-aware model router rather than a direct foreground launcher.
- **Provide defaults, families, inheritance, or profiles inside KDL.** Rejected
  because deduplication makes one kind's target partial and allows a new kind to
  inherit policy its owner never reviewed. Reopen if the session-kind set grows
  large enough that explicit targets are no longer auditable and a replacement
  can preserve fail-on-kind-addition behavior.
- **Store launch policy in each task leaf.** Rejected because task trees should
  describe work and remain portable, while executable, model, permission, and
  sandbox policy is personal and may change between sessions. Reopen only if a
  work item must carry a reproducible execution environment as part of its
  durable contract.
- **Execute templates through a shell.** Rejected because shell evaluation
  turns quoting, environment expansion, pipelines, and redirection into a
  second configuration language and obscures the direct foreground child Grove
  must supervise. A wrapper script supplies that power explicitly and can
  preserve ownership with `exec`. Reopen only if direct argv cannot express a
  required launch and wrappers cease to be viable.
- **Infer optional Herdr and agent-hint arguments from the selected command.**
  Rejected because target identity is intentionally opaque. The visible
  `${herdr_settings}` splice and configured `HERDR_AGENT` assignment let policy
  opt in without inference. Reopen only if every supported command shares one
  verified argument contract.
