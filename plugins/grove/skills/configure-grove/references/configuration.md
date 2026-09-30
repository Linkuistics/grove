# Configuration mechanics

Source: [Grove configuration reference](https://github.com/Linkuistics/grove/blob/main/docs/CONFIGURATION.md).
Check the reference for the installed Grove version when its grammar differs.

## Sources and admission

The personal source is `~/.config/grove/config.kdl`. A lifecycle workspace may
also use one **untracked** `.grove.kdl`: the worktree-root file wins over the
default repository-root file. Those two local candidates are never merged.
The file sits beside `.grove/`, so finishing a workstream leaves it in place.

Ignore `/.grove.kdl` before creating it: ordinary jj commands snapshot files.
Establish coverage in the workspace holding the file, not just the caller's.
Prefer existing coverage or a local untracked exclude source honored by the
installed jj. Report any required tracked ignore-file edit as part of the change.
An ignore rule does not untrack an existing file. For the user's accidentally
tracked delta, inspect and preserve its contents, establish ignore coverage,
then use `jj file untrack .grove.kdl` from its workspace and verify admission.
Report the resulting tracked deletion. Do not adopt an upstream-supplied delta
by untracking it without explicit authorization for its executable policy.

Only active personal routes admit kinds. Local-only kinds do not run. Commands
and profiles are personal definitions; local files can patch values, bindings,
routes and selection, but cannot declare commands or profiles.
`grove run KIND` uses personal policy only: local deltas never affect standalone
invocations such as release-note generation.

## Reuse and precedence

Both files use `config { ... }`. A command declares its parameters; a binding
names a command; a route names a binding. These namespaces are distinct.

- Command `param` declarations supply defaults or required parameters.
- `values "command"` changes shared parameter values.
- Route `param` values override shared values, even shared assignments applied
  later. `unset "effort"` on a route removes that exception and exposes the
  effective shared value; it is not an empty string.
- Personal profiles may `include` other personal profiles and patch values,
  bindings and routes. Includes apply left to right before the profile's patch.
- Local `select` **replaces** the personal selection list. Omit it to inherit;
  `select;` selects no profiles. A profile containing only bindings may need a
  separate routes profile in the selection.
- The local delta applies after the selected profiles. Use a route patch to
  change one kind; changing a binding or shared value may affect many kinds.

This example assumes the effective personal policy already admits `impl` and
`review-impl`, and their final commands declare an `effort` parameter:

```kdl
config {
    route "impl" { param "effort" "high"; }
    route "review-impl" { param "effort" "high"; }
}
```

To change models, patch the actual declared model parameter in the same route.
Its name is user-defined; `model` and `effort` are conventions, not built-ins.
Use a full model ID supported by that route's harness. Cross-harness changes
also require redirecting the route or binding to a compatible personal command.
Compatibility includes the execution context: a standalone-only headless helper
may depend on staged files and Grove's outer confinement, and must not be reused
for lifecycle launches. Standalone commands must be noninteractive and may need
updated runtime-read grants outside this configuration. Compare approval,
permission and sandbox posture before redirecting; do not introduce an
unauthorized relaxation. Reconcile every carried route parameter with the new
harness, removing or replacing incompatible values rather than trusting shared
parameter names. Grove validates parameter structure, not model availability.
Keep one declaration per name in each scope: edit an existing patch rather
than appending a duplicate.

## Execution and inspection

Templates are split into argv and executed directly, without a shell. Runtime
slots are `${prompt}`, `${repo}`, `${worktree}` and `${session_name}`, plus the
lifecycle-only `${kind}`, `${task_file}` and `${task_id}`: the selected leaf's
kind token, absolute task path and `<slug>-k<key>` handle, taken from the
selection that composes the prompt. `${prompt}` is required exactly once and
every other slot may appear at most once. Runtime slots occupy whole arguments.
Declared `${param.name}` substitutions can also fill part of an argument. A
wrapper must `exec` its foreground harness to preserve process ownership.
`grove run` refuses a routed command that uses a lifecycle-only slot, so do not
route a standalone kind to such a command. Inspection shows these slots as
`slot <kind>`, `slot <task_file>` and `slot <task_id>` and fills in nothing.

Use `grove config show --json` for whole-policy validation and provenance, and
`grove config show --kind KIND` for a focused human-readable view. Even the
filtered command validates the entire active policy. Inspection launches no
model and may snapshot jj metadata while checking local trackedness.

The JSON `commands` entries contain `key`, `binding`, `command`, `parameters`
and `words`; parameters are name/value entries. `sources`, `selection` and
`non_admitted_keys` expose which policy actually took effect. Respect the report's
schema version rather than treating this summary as a permanent API schema.

For alternate-profile inspection, use an isolated scratch jj repository and an
ignored local delta containing only the intended `select` declaration. For
personal-only or standalone-policy inspection, use that scratch repository
without a delta. Do not temporarily change personal selection or move an active
workspace's delta aside. The personal file still participates; this technique
does not validate an unpublished replacement personal file. If no supported
staging validator exists, publish the complete candidate atomically, validate
immediately, and atomically restore only your change if validation fails.
