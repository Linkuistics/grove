# evaluation-boundary-k27 — brief

## Goal

Make dispatch safe to run unattended, in any directory. A stuck or interrupted
selection launches nothing and leaves no worker behind. The harness receives
exactly the signal state its caller had. Ambient repository and environment
inputs can neither run code nor gain authority, and each such claim is proved
against a configuration that has been seen to fire.

## Done when

- The whole-selection bound holds: 30 seconds by default, 1 to 120 through
  `--timeout-ms`. It counts from worker start and covers imports, context and
  the callback. A hard wall-clock deadline kills a worker stuck in module
  import or a synchronous loop, after a cleanup grace of at most one second.
  Timeout exits 124 and launches nothing.
- INT, TERM and HUP are handled during evaluation unless the caller ignored them
  at entry. The worker is stopped and reaped, nothing launches, and the process
  ends by restoring and re-raising the signal. Cancellation is checked at every
  post-result boundary.
- The handoff blocks the handled signals after the record commit and checks
  once more for pending cancellation. On cancellation it launches nothing,
  appends a best-effort not-executed detail and re-raises. Otherwise it
  restores the entry mask and dispositions and execs. SIGPIPE and caller-ignored
  HUP reach the harness exactly as they were at entry.
- The spec lists five hostile classes, each with a firing configuration: cwd
  policy entry, cwd dotenv and bunfig preload, `BUN_OPTIONS`, the
  `node_modules/harness-dispatch` shadow, and tsconfig `paths`. Each stays
  inert through the public launcher, and each firing configuration is seen to
  fire. Classes with no known firing configuration are reported as such, not
  counted.
- The worker environment is the base set plus exact `--policy-env` grants.
  `BUN_*`, `NODE_OPTIONS`, `NODE_PATH`, loader injection and the private
  protocol variables are always excluded, and grant values are never printed.
  Worker and nested child environments lack caller completion values unless the
  owner explicitly grants them.

## Decomposition

1. `selection-cancellation-k28`: the deadline, hard kill, cleanup grace and
   signal cancellation during evaluation.
2. `signal-transparent-handoff-k29`: the linearization point, the not-executed
   append, and entry-state transparency including SIGPIPE.
3. `ambient-authority-k30`: `--policy-env`, environment scrubbing, worker-location
   spoofing, the hostile fixtures with their firing controls, and updated
   runtime evidence.

## Pointers

- Spec sections: `#execution-contract`, `#policy-authority`, `#diagnostics`
  (exits and re-raise), `#test-seams`, the lifecycle and authority rows, and
  the firing-configuration table.
- Evidence: `docs/design/harness-selection-and-execution/runtime-evidence.md`
  gives the probe builds that fired: the default-autoload build, the
  unregistered build, and the tsconfig/package-json-enabled build. Probe builds
  are test instruments and never ship.
- The keyed-launch interrupt and re-raise suites show this repository's existing
  idiom for signal tests.

## Review

This node is expected to end with a `review-impl` naming this node's handle.
Its last leaf cuts that review inside this node. Signal races, ordering around
the linearization point, pre-main SIGPIPE capture and environment authority
are claims the compiler cannot check. A mistake here silently ends Grove
sessions or grants completion authority.
