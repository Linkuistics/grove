# Modular configuration examples

These are design examples for the [modular configuration
contract](../../specs/modular-configuration.md). The current reader does not yet
accept the modular form. The implementation must test these exact files through
its reader before shipping them or installing them into the personal directory.
Remove this delivery-status paragraph when that acceptance passes.

The planned `grove-llm config-examples` verb places the files below in
`~/.config/grove/` and installs these instructions as
`CONFIGURATION.examples.md`. The verb never overwrites `config.kdl`, any active
`.grove.kdl`, or a different existing file. Running it again leaves matching
files unchanged. A conflict names the files the owner must move or reconcile.

| File | Demonstrates |
|---|---|
| `config.modular.example.kdl` | Two commands, explicit routes, two lead arrangements, includes, a global default, and an inactive unfinished profile |
| `grove.codex-led.example.kdl` | Local selection replacing the global default with the first arrangement |
| `grove.claude-led.example.kdl` | The opposite arrangement with no copied templates or route map |
| `grove.high-effort.example.kdl` | A parameter-only experiment composed after `daily` |
| `grove.local-override.example.kdl` | Per-kind local values after profiles, and removal of a route override |
| `grove.legacy-override.example.kdl` | An unchanged flat local template replacing one route |

`my-codex-policy`, `my-claude-policy`, and `my-other-policy` are illustrative
executables supplied by the owner. No such wrapper is shipped. Their flags are
an example of an author-defined interface, not a claim about a particular
harness release. Supply your executables or wrappers, model values, approval,
permissions, and sandbox policy. A wrapper should `exec` its final program so
the supervised foreground process remains the real harness.

The first two commands deliberately express effort differently: one puts it
inside `model_reasoning_effort=...`; the other receives it as a whole argument.
Grove treats both as opaque words. The example names two familiar vendors, but
neither name has meaning to the resolver and both can be replaced.

After the feature is delivered, copy or adapt the personal sample into
`config.kdl` yourself. To activate a local example, first add `/.grove.kdl` to
the workspace's ignore rules, then copy the chosen example into `.grove.kdl`
beside `.grove/`. An already tracked delta must be ignored first and then
untracked with `jj file untrack .grove.kdl`. Grove writes no ignore rule and
refuses tracked, unreadable or invalid selected candidates.

The global personal file is always read. Grove uses the worktree's local file
when it exists, otherwise the main repository's; it never merges both local
files. A local `select` is the complete selected list. To extend `daily`, write
`select "daily" "high-effort"`, as the experiment sample does. `select` with no
arguments disables profiles while retaining any personal base entries.

Run `grove-llm config-show --kind impl` to inspect a route, or add `--json` for
structured words and origins. The report leaves runtime slots identified rather
than inventing a prompt. Inspection changes no configuration or workspace file;
its jj trackedness query may snapshot metadata, just as the launch check does.
Inspect before launching a real session.

Expected example outcomes: `daily` sends `impl` to the Codex wrapper and
`review-impl` to the Claude wrapper. `routes` plus `claude-led` reverses them.
Adding `high-effort` changes shared effort; removing it restores `medium`.
`proof` retains its explicit `high` override until an `unset` removes it. The
local override sample sends `impl` with `low` despite the selected shared
`high`, and the legacy sample sends it to `my-other-policy`.

Selecting `daily` followed by `unfinished` must fail with an unknown command
reference and launch nothing. Leaving `unfinished` unselected must succeed.
The implementation tests these outcomes with fake executables, without running
Codex or Claude Code.
