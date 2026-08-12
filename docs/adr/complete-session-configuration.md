# Complete session configuration

Every Grove session kind maps to one complete command-template string in the
personal `~/.config/grove/config.kdl`; Grove executes the expanded argv directly
and does not infer a harness, model, defaults, or hidden harness-specific
arguments. This binds because launch policy includes choices Grove cannot own —
model, reasoning effort, approval, sandboxing, wrappers, and harness-specific
behavior — while splitting those choices across environment precedence,
repository stamps, task metadata, and a built-in harness registry made the
effective command impossible to inspect or validate as one value. The
configuration deliberately repeats all nineteen targets so every session is
complete in isolation and adding a kind fails visibly in every old config.
Before spawning the configured foreground command, Grove removes stale Grove
control values and grants its fresh signal path; it otherwise preserves the
caller's environment, including Git repository selectors. Driver-internal VCS
commands follow the opposite rule: they scrub repository selectors and anchor
Git explicitly to the leased working tree, so personal launch policy cannot
redirect lifecycle mutations.

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
- **Compare `research-a` and `research-b` targets to enforce vendor diversity.**
  Rejected because opaque command strings do not expose a stable harness or
  model identity: different wrappers may reach the same target and equal words
  may still produce independent corpora. `leaf-add-pair` therefore records two
  research sessions while material target diversity remains configuration-owner
  policy. Reopen only if target identity becomes an explicit comparable part of
  configuration without reintroducing harness inference.
- **Inline the complete Grove methodology into every `${prompt}`.** Rejected on
  specificity rather than size: a session handed all of `content/` performs
  exactly the selection a mandate exists to have already made for it. What
  `${prompt}` does carry is the kind-selected slice set settled by [the mandate
  delivers the methodology](mandate-delivers-the-methodology.md), which is a
  decision about delivery and classification and not about launch policy —
  nothing about it asks Grove to infer a harness, a model, or a default. Reopen
  never; the complete inline is refused there, and the partial one is that
  decision's to revise.
