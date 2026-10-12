# grove-ravel-caller-k5

## Goal

Make Grove's lifecycle and standalone invocations consume installed Ravel only
through its documented CLI, and make Grove own its policy adapter and examples.

## Context

- Requires ravel-delivery-k4's independently installed release.
- `crates/grove/src/dispatch.rs::locate` currently canonicalizes Grove's path
  and selects its sibling `harness-dispatch`; lifecycle and standalone callers
  both use it. `crates/grove-loop/src/launch_directory.rs` validates the ending
  report's schema/source/observed value. Prompt and confinement/runtime-read
  construction must follow the resolved Ravel executable.
- Preserve the current adapter, Grove examples/sample, fixtures and command
  tests from the old dispatch package. The adapter uses the generic SDK and
  public task conventions; deliver it through Grove-owned readable assets and
  ordinary documented policy imports, never Ravel's embedded module registry.

## Done when

- Lifecycle and standalone paths resolve the installed `ravel` independently
  of Grove's real executable directory/version, with an actionable missing-tool
  remedy. Resolve once and confine that executable and its documented runtime
  layout. Grove reads only the public ending report, not worker protocol/records.
- Caller-owned adapter/sample/examples and their typecheck/runtime fixtures work
  with Ravel's public SDK. Review Creator/Reviews handling and provider lookup
  retain their existing behavior. Ravel contains no Grove module registration.
- Extend existing lifecycle/standalone fixtures to execute the real installed
  Ravel and worker, with fake agents and temporary policies. Put Grove and Ravel
  in separate prefixes with different versions; a decoy old sibling must not
  win. Missing Ravel reports its install remedy. No host Bun/Node is required.
- Existing continuation, completion, refusal/resume, cancellation, process-group
  cleanup, PTY recovery, confinement, epoch admission and stale-session checks
  pass. This leaf retains bare exit's existing continue semantics.
- Grove's runtime and caller documentation describe the executable boundary.
  Preserve its local keyed-launch implementation and authority guards; no Rust
  dependency points into Ravel. Run the repository `task check` after migration.

## Notes

Keep the old bundled package/build recipe temporarily where needed to leave
current archive checks green; grove-ravel-delivery-k6 removes that delivery.
Do not make new source prompts or SDK names alter this session's original
installed driver/exit contract. Record the Ravel version used for integration.
