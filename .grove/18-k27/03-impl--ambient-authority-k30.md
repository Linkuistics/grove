# ambient-authority-k30

## Goal

Prove, through the public launcher, that running dispatch in a hostile
directory or environment executes no repository code and grants no authority.
The owner's explicit choices must still be admitted.

## Context

The spec's `#policy-authority` owns the environment rules and the `--policy-env`
contract. The spec's firing-configuration table and the runtime evidence name
which probe build makes each hostile class fire. The skeleton built the
controls. This leaf proves them and adds the explicit grant.

## Done when

- `--policy-env NAME` is repeatable and grants exact names beyond the base
  environment. The names below are always excluded from grants:
  - `BUN_*`, `NODE_OPTIONS` and `NODE_PATH`;
  - dynamic-loader injection variables;
  - the private protocol variables.
  Inspection shows granted names, never values.
- Tests show the worker, and a normal child it spawns, lacking a caller's
  `GROVE_SIGNAL_FILE` and other completion values unless the value is
  explicitly granted. The usage documentation warns against granting
  `GROVE_SIGNAL_FILE`.
- A worker-shaped executable on PATH or in the cwd is never used, and
  installation control never comes from ambient input.
- Each hostile class is tested beside its firing configuration, with a probe
  build the Taskfile makes for tests only:
  - a cwd policy entry;
  - a cwd `.env` and bunfig preload;
  - a `BUN_OPTIONS` preload;
  - a `node_modules/harness-dispatch` shadow beside an admitted entry;
  - tsconfig `paths` beside an admitted entry.
  The class stays inert through the public launcher, and its firing
  configuration is seen to fire in the same run. The HOME bunfig and cwd
  tsconfig classes are reported as having no known firing configuration.
- An explicit relative `--config` and a personal policy that imports a repository
  entry are admitted. Inspection identifies the authority. Each documented
  specifier resolves to its embedded module.
- A policy that floods stdout and stderr leaves `--json` output and the private
  protocol intact.
- `runtime-evidence.md` records which controls were seen to fire against the
  shipped launcher, on which host and Bun version. The archive assertions
  confirm that no probe build ships.
- This node's `Done when` holds. As this leaf's last act, cut the node's
  `review-impl`: run `grove-llm leaf-add evaluation-boundary-k27
  evaluation-boundary --kind review-impl`, with `**Reviews:**
  evaluation-boundary-k27`. Write the node brief's review doubts into its body.
