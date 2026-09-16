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
  runner and Grove. Its review and integration are `modular-configuration-k3`
  and `modular-configuration-k5`.
- `modular-configuration-k4` plans the complete reviewed feature. Its planning
  review `modular-configuration-k13` precedes the implementation work below;
  any review integration must also precede the first implementation entry.
- `configuration-engine-k6` delivers the generic Catalog/Templates increment:
  captured legacy configuration (`captured-configuration-k7`), flat inspection
  and provenance (`flat-provenance-k15`), reusable base
  commands (`reusable-commands-k8`), then selected profile composition
  (`profile-composition-k9`). Each child supplies working public behavior.
  Grove gains wrapper base behavior in `k8`; `k9` captures Catalog at its adapter
  and temporarily refuses either source's selection declaration rather than
  silently resolving the base. Without declarations it resolves an empty list.
- `workspace-configuration-k10` makes Grove selection, admission, tree mutation
  and launches use selected profiles, replacing `k9`'s declaration guard with
  local/default/empty selection policy, with isolation and next-session reload.
- `configuration-inspection-k11` adds human/JSON inspection of that same result.
- `configuration-examples-k12` validates and packages the repository examples,
  adds safe installation, and performs actual personal-directory delivery.

These are dependency-ordered working increments inside the approved complete
feature scope. The generic engine is useful through its public API before the
Grove selection policy lands; Grove launches work before the inspection command;
inspection works before example installation. Later work must not be needed to
repair a predecessor's checks or source-exact documentation. The planning leaf's
running log records why the complete root contract remains in this grove.

## Implementation verification and documentation

Every producer owns observable tests at the agreed seam and documentation for
its delivered behavior. Use Rust and walkthrough-authoring skills when writing
those artifacts, and the CLI-design skill for command surfaces. Prefer existing
test seams. Keep all affected crate docs, the user reference, usage and
architecture prose, durable specs/ADRs and glossary consistent at each landing;
narrow pending-implementation notices to what remains rather than claiming a
partially delivered contract is complete.

Production Rust and crate manifests belong to source-exact walkthrough corpora.
Inspect every affected book manifest, update fragments, explanatory prose,
indices, concept rows, root lengths and corpus inventory where required, and
run final book validation in the same task. New production modules are included
by recursive corpus rules; a new file does not escape this obligation. Discover
the complete affected set instead of assuming the named book is the only one.
Any added/changed `[[corpus.add]]` or `[[corpus.exclude]]` entry also requires
its matching exception inventory row in `docs/specs/walkthrough-books.md` and
the repository corpus-exception check. This includes new inline test files;
production modules stay inside the recursive corpus.

Each human-command producer updates `docs/specs/user-guide-coverage.md` in the
same leaf that changes help, with matching `docs/USAGE.md` anchors, coverage-test
updates and CLI surface assertions. `configuration-inspection-k11` owns
`config show`; `configuration-examples-k12` owns `config examples`.

Run focused public-seam acceptance as appropriate, then `bash scripts/check.sh`
before committing each implementation task. It is the principal check list,
including workspace tests and final validation of every book. Keep measurement
inputs fixed during a run and preserve `.cargo/config.toml`'s signal guard.
Invoke actual Grove tree verbs with `grove-llm` directly, never through cargo.

## Acceptance ownership

The rows below cover the modular spec's complete acceptance table. Handles are
stable even if review inserts work or a producer further decomposes a leaf.
Shared rows name both the semantic owner and the composed Grove observation;
each named leaf must carry its part, not assume another owner tested it.

| Spec acceptance case | Implementation/testing owner and observation |
|---|---|
| Shared command | `reusable-commands-k8`: public expansion changes all shared users while preserving explicit route overrides; Grove wrapper-base/local-override launch and mutation acceptance |
| Lead arrangements | `profile-composition-k9`: unchanged route map reaches swapped bindings; `workspace-configuration-k10`: opposite launches and workspace isolation |
| Small experiment | `profile-composition-k9`: add/remove profile changes embedded parameter contents with stable word count; `workspace-configuration-k10`: exact launched argv |
| Ordering | `profile-composition-k9`: within-scope last write, cross-scope specificity, includes before own patch, repeats/diamonds, full cycle chain |
| Selection | `profile-composition-k9`: declaration capture and explicit generic selection, plus Grove refusal of any declaration before mutation/launch while policy is pending; `workspace-configuration-k10`: replace guard with absent/replacement/empty local policy using global definitions |
| Source discovery | `workspace-configuration-k10`: worktree/repository precedence, tracked/unreadable/unprobeable refusal without fallback; `configuration-inspection-k11`: same admission |
| Inactive work | `profile-composition-k9`: inactive success versus surviving selected errors; `configuration-examples-k12`: packaged unfinished profile and no launch on failure |
| Partial building blocks | `profile-composition-k9`: split routes/bindings/values and local completion; `workspace-configuration-k10`: production adapter uses the result |
| Missing personal target | `reusable-commands-k8` and `profile-composition-k9`: global `missing_target`, real origins/occurrences, local target cannot repair; `workspace-configuration-k10` and `configuration-inspection-k11`: no mutation/launch/snapshot even for another requested kind |
| Loader convenience | `captured-configuration-k7`: legacy/vocabulary equivalence; `reusable-commands-k8`: wrapper base equivalence; `profile-composition-k9`: captured selections ignored by the empty-selection convenience |
| Local authorization | `reusable-commands-k8`: local-only non-admission; `profile-composition-k9`: inactive versus selected personal target; `workspace-configuration-k10`: refusal before use |
| Legacy compatibility | `captured-configuration-k7`: unchanged flat argv and eager checks; `reusable-commands-k8` and `profile-composition-k9`: shape-disambiguated grammar words, mixed forms and overrides; `workspace-configuration-k10`: existing launch fixtures |
| Parameter safety | `reusable-commands-k8`: exact words for adversarial strings, empty values, NUL rejection, native runtime values and named/legacy scanner differences; `workspace-configuration-k10`: fake executable receives those boundaries |
| Provenance | `captured-configuration-k7`: capture original sources/declarations; `flat-provenance-k15`: flat histories, origins and inspection/expansion equality after source removal; `reusable-commands-k8`: multi-origin words, winners, removals/resets; `profile-composition-k9`: repeated occurrence histories; `configuration-inspection-k11`: encoded/displayed records |
| Inspection | `configuration-inspection-k11`: shared compiled words equal captured argv with fixed inputs/context, full validation, read-only bytes, no launch/lease/epoch dependence, JSON and usage contract |
| Reload | `workspace-configuration-k10`: edits change the next child and preserve the running child |
| Delivery | `configuration-examples-k12`: all exact repository examples through the reader and fake launch, safe installer success/conflict/failure/races, actual personal-directory files and preserved active policy |

Additional interface obligations have explicit owners: the generic non-Grove
consumer and captured-source independence start in `captured-configuration-k7`
and gain inspection equality in `flat-provenance-k15`, then are extended through
`profile-composition-k9`; Catalog/Selection conformance
migrates in the first child and tests modular resolution in the last. CLI help,
streams, exit codes, native path encoding and stale-epoch independence belong
to the two human-command leaves. Complete example delivery remains root work
until the intended files are actually present or a live leaf carries the gap.

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
implementation planning; integration `modular-configuration-k5` repaired that
review before planning. The design and ADRs describe the intended modular
contract; at the planning handoff, production code and the flat user reference
remain at the pre-implementation boundary. Each implementation producer owns
updating that boundary as its working increment lands.

The user explicitly approved inspection reusing the existing jj trackedness
check. It edits no configuration or working-tree files and launches no session;
jj may snapshot metadata. This preserves the same admission decision as launch.

## Generic engine handoff

`selected-profile-fold-k28` closes `profile-occurrences-k26`,
`profile-composition-k9` and `configuration-engine-k6`. The generic Catalog now
implements the reviewed language: explicit selections expand every include and
selected occurrence, fold personal patches before local patches, validate the
active result globally, and retain occurrence-specific diagnostics and inspection
histories. Captured sources, conformance and snapshots require no subsequent file
I/O. The public profile tests include diamonds, repeated selections, specificity,
literal resets, personal authority and a consumer with non-Grove keys/vocabulary.
All eight principal checks and all six source-exact books pass.

`workspace-configuration-k10` should replace SessionConfig's declaration guard
with local/default/empty selection policy through this Catalog API. Generic
`Templates::load` deliberately continues to ignore captured declarations and
resolve an empty list. Preserve admission before use, global missing-target
failure, local-only non-admission and the existing source protections. k11 owns
the human/JSON inspector and k12 owns validated example delivery; neither is
claimed by the completed engine. The current configuration reference and module
spec distinguish these remaining consumer features from implemented composition.

## Workspace configuration handoff

`workspace-reload-k30` completes `workspace-configuration-k10`, following the
selection/diagnostic adapter in k29 and initial-kind admission repair in k31.
Real driver acceptance covers opposite lead/review arrangements in sibling jj
workspaces, repository fallback and worktree shadowing, local replacement,
default inheritance and empty selection, direct overrides and parameter-only
experiments. NUL-delimited child records retain adversarial parameter contents
and empty words. A live-child handshake covers selection/shared-value reload
without changing that child's PID/argv, and a lock-controlled external edit
tests invalid configuration at the second load with no launch or tree change.

Inspection k11 should continue through SessionConfig's existing discovery,
selection and diagnostic surface. Human inspection and example delivery remain
owned by k11 and k12. No new resolver, load point or source-admission policy was
needed for the composed acceptance. The ADR set already matches this boundary.

## Implementation constraints carried from design

- Implement the reviewed KDL grammar and parameter/reference namespaces without baking
  harnesses or the methodology's set of kinds into the binary.
- Preserve the specified named settings, override granularity and order for
  nested profiles. Preserve the ability to change a single
  value without copying an entire command or mapping.
- Preserve the interaction of profiles with legacy flat entries and direct local
  overrides, including the requirement that a local delta cannot independently
  introduce a session kind absent from personal policy.
- Distinguish document-wide parse/structural errors from selected-combination
  semantic errors, so partial profiles can be useful building blocks and unused
  experiments do not break working configurations.
- Deliver safe parameter expansion, required-parameter errors, source attribution,
  and the specified inspection surface. Keep inspection and launch on one resolver.
- Deliver the shipped examples through the specified safe installer without
  replacing personal files, and keep example contents checked against the reader.
- The configuration ADRs now permit profiles and require complete resolved
  commands. Their original flat-only prohibition is historical evidence of a
  trade-off, not a current restriction to restore.
- Preserve the generic runner's independence from Grove-specific paths, kinds,
  profile-selection policy and VCS checks. The reviewed Catalog/Templates and
  SessionConfig boundary assigns those responsibilities; keep its output
  records coherent with the module contract as implementation lands.
