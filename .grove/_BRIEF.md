# grove.modular-configuration — brief

## Goal

Make Grove configuration easy to reuse, vary, and switch. Users should be able
to keep multiple configurations available and try small differences without
repeating almost identical configuration entries or editing many entries to
switch setups.

The motivating examples are Codex-led and Claude Code-led workflows, but the
scope is arbitrary configuration experiments, including small variations.

## Settled requirements

### Reuse and composition

- Define named, combinable configuration profiles in the personal
  configuration. Keep multiple alternatives available at the same time.
- Shared configuration remains live: changing a shared value changes its users
  unless they explicitly override that value. Profiles record differences rather
  than independent copies.
- Profiles may include other profiles, so a common combination can have a
  reusable name such as `daily`.
- Selected profiles compose left to right. A later value overrides an earlier
  value; intentional overlap is not a conflict error.
- Reusable commands expose parameters defined by their author. Grove does not
  interpret harness-specific model, reasoning-effort, permission, or other flags.
  Multiple session kinds can share a command without repeating its text.
- Small experiments must support both changing which commands kinds use and
  changing individual argument values. The motivating effort change includes a
  value inside an argument such as `model_reasoning_effort=medium`, not just an
  argument whose entire contents are the value.

### Selection and validation

- Select profiles per worktree in the existing `.grove.kdl`, with a global
  default selection in the personal configuration.
- An explicit workspace selection replaces the global default list; it does not
  append implicitly. To extend `daily`, select `daily` followed by the experiment
  profiles explicitly.
- Unfinished, unselected profiles with missing parameters or unresolved
  references do not block a working selection. The whole document must still be
  valid KDL. Validate the selected combination and the profiles it includes.
- Existing flat personal configurations and workspace overrides keep working
  unchanged; adopting profiles must not require a one-time rewrite.

### Examples

- Provide a sample configuration using the new form and workspace override
  examples in `~/.config/grove/`, under separate filenames from the user's
  active configuration. The examples must be usable with the delivered grammar.
- Demonstrate two different lead/review arrangements, a small parameter-only
  variation, a named combination that includes other profiles, workspace
  selection replacing the global default, and a local override.
- Use illustrative commands and explain where users supply their own policy.
  The feature must not depend on today's model names or two particular vendors.

## Supporting behavior

- Expand a profile's included profiles in their listed order, then apply that
  profile's own values. Apply explicit workspace values after the selected
  profiles, so a one-off experiment can stay in the workspace file.
- Always read the global personal configuration at
  `~/.config/grove/config.kdl`. It supplies reusable definitions and the default
  profile selection even when a workspace selects other profiles.
- Select at most one local override file: use the worktree's `.grove.kdl` when
  present, otherwise look for the main repository's `.grove.kdl`. Apply the
  selected local file over the global personal configuration. The two local
  candidates are alternatives; neither replaces the global file. Preserve the
  untracked-file requirement and refusal of tracked or unreadable candidates.
- Configuration changes affect the next session. They do not restart or mutate
  the command already running.
- Reject unknown selected profiles, reachable include cycles, unresolved required
  parameters, and invalid resolved commands before launching. Keep errors
  actionable and identify the relevant configuration source. A failure must not
  silently fall back to another selection.
- Preserve explicit session-kind routing and refusal of unconfigured kinds;
  profile composition must not introduce an implicit catch-all route.
- Keep direct execution without a shell and preserve argument boundaries when
  expanding parameter values. A parameter value must not inject extra arguments.
- Provide a read-only way to inspect the selected profiles and resolved commands,
  including where overridden values came from, without launching a session.

## Acceptance scenarios

1. Define a reusable command once and use it for several kinds. Changing one
   shared value updates every use except uses with an explicit override.
2. Keep `codex-led` and `claude-led` configurations together. Switch one workspace
   between them by editing only its selection; other workspaces keep their own
   selections.
3. Select `daily` plus `high-effort`. Only the settings supplied by `high-effort`
   change. Removing it restores the values inherited through `daily`.
4. A later selected profile overrides the same setting from an earlier profile.
   Nested includes obey the agreed order and a profile's own values can override
   values supplied by its includes.
5. The global personal configuration is read for every selection. With no local
   selection, use its default profile list. An explicit local selection replaces
   that list while continuing to use the global definitions; direct local values
   take precedence over the selected profiles. If both local candidate files
   exist, only the worktree's is used alongside the global file.
6. An incomplete, unselected experiment coexists with a working selection.
   Selecting that experiment produces a useful error and launches nothing.
7. Existing flat configurations and local deltas still resolve and launch the
   same commands. The existing worktree/repository search and trackedness
   behavior continue to hold.
8. Example files in the configuration directory demonstrate the feature, are
   validated by the implementation's own reader, and do not overwrite active
   policy. Inspection shows the same command that a launch would use.

## Agreed test seams

- **Configuration load and expansion:** exercise the public configuration
  interface with temporary personal and workspace files. Assert selected profile
  order, includes, inheritance, parameter substitution, exact argument vectors,
  origin information, compatibility, and diagnostics. Prefer the existing
  configuration seam rather than tests coupled to parser internals.
- **Grove launch acceptance:** use temporary jj workspaces and fake executables
  that record the arguments received. Check workspace isolation, configuration
  reload at the next session, and refusal before launch or tree mutation when the
  active configuration is invalid. Preserve the meta-grove signal guard.

Real Codex/Claude sessions are not needed for these tests: the observable contract
is which executable and exact arguments Grove launches.

## Done when

- The agreed configuration behavior is implemented and its examples can be used
  from the Grove configuration directory.
- The acceptance scenarios pass through the agreed test seams, including legacy
  compatibility and the existing local configuration protections.
- Configuration documentation, affected ADRs, glossary entries, and affected
  interface documentation describe one coherent current contract.
- The resolved configuration is inspectable before launch, and the active
  personal configuration is preserved when examples are provided.

## Decomposition

- `plan-k1` establishes the approved human-facing behavior and test seams.
- `modular-configuration-k2` designs the grammar, resolution semantics,
  diagnostics, inspection, example delivery, and interfaces between the generic
  runner and Grove. It synthesizes these decisions rather than repeating the
  interview; implementation planning follows that design.

## Pointers

- Requirements decisions: `plan-k1`.
- Existing contracts to revisit: `docs/adr/complete-session-configuration.md` and
  `docs/adr/untracked-configuration-delta.md`.
- Current user-facing reference: `docs/CONFIGURATION.md`.
- Module contract: the runner and configuration sections of
  `docs/specs/module-decomposition.md`.
- Design contract: `docs/specs/modular-configuration.md`; examples under
  `docs/examples/modular-configuration/` and the visual document under
  `docs/design/modular-configuration/` are reviewed with it.
- Glossary terms: Grove configuration, Configuration delta, Configuration
  profile, Session kind, Kind routing, and Loop control channel.
- Existing evidence and source locations are in the requirements leaf's notes.

## Design handoff

`modular-configuration-k2` owns the design. The review is
`modular-configuration-k3`, followed by `modular-configuration-k4` for
implementation planning; any review integration goes before planning.
The design and ADRs describe the intended modular contract; production code and
the flat user reference remain at the pre-implementation boundary.

The user explicitly approved inspection reusing the existing jj trackedness
check. It edits no configuration or working-tree files and launches no session;
jj may snapshot metadata. This preserves the same admission decision as launch.

## Design obligations

- Choose a clear KDL grammar and parameter/reference namespaces without baking
  harnesses or the methodology's set of kinds into the binary.
- Specify which named settings and mappings compose, their override granularity,
  and the order for nested profiles. Preserve the ability to change a single
  value without copying an entire command or mapping.
- Specify the interaction of profiles with legacy flat entries and direct local
  overrides, including the requirement that a local delta cannot independently
  introduce a session kind absent from personal policy.
- Distinguish document-wide parse/structural errors from selected-combination
  semantic errors, so partial profiles can be useful building blocks and unused
  experiments do not break working configurations.
- Define safe parameter expansion, required-parameter errors, source attribution,
  and a useful inspection surface. Keep inspection and launch on one resolver.
- Choose how the shipped examples reach the configuration directory without
  replacing personal files, and keep example contents checked against the reader.
- Reconcile the current ADR's rejection of profiles and partial commands with
  the user's explicit decision to support them. The old prohibition is evidence
  of a trade-off to address, not grounds to refuse this feature.
- Preserve the generic runner's independence from Grove-specific paths, kinds,
  profile-selection policy, and VCS checks where those remain the consumer's
  responsibilities. Reassess the exact seam during design rather than deciding
  the implementation here.
