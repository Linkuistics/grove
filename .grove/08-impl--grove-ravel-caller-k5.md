# grove-ravel-caller-k5

## Goal

Make Grove's lifecycle and standalone invocations consume installed Ravel only
through its documented CLI, and make Grove own its policy adapter and examples.

## Context

- Requires ravel-publication-k11's independently installed release and k3's
  documented rename/report/layout contract.
- `crates/grove/src/dispatch.rs::locate` currently canonicalizes Grove's path
  and selects its sibling `harness-dispatch`; lifecycle and standalone callers
  both use it. `crates/grove-loop/src/launch_directory.rs` validates the ending
  report's schema/source/observed value. Prompt and confinement/runtime-read
  construction must follow the resolved Ravel executable.
  Resolve `ravel` from the launching process's PATH, canonicalize the selected
  executable once, and use that same path throughout launch/confinement. Do not
  search the cwd, repository, Grove sibling or equal-version keg as a fallback.
  Document the PATH trust decision and its costs in Grove's existing caller ADR.
- Preserve the current adapter, Grove examples/sample, fixtures and command
  tests from the old dispatch package. The adapter uses the generic SDK and
  public task conventions; deliver it through Grove-owned readable assets and
  ordinary documented policy imports, never Ravel's embedded module registry.

## Done when

- Lifecycle and standalone paths resolve the installed `ravel` independently
  of Grove's real executable directory/version, with an actionable missing-tool
  remedy. Resolve once and confine that executable and its documented runtime
  layout. Grove reads only the public ending report, not worker protocol/records.
  Check the documented Ravel CLI version against the minimum delivered contract
  before launching; unknown/older versions refuse with an upgrade remedy.
  Homebrew depends_on ensures presence, not this runtime contract check.
- Caller-owned adapter/sample/examples and their typecheck/runtime fixtures work
  with Ravel's public SDK. Review Creator/Reviews handling and provider lookup
  retain their existing behavior. Ravel contains no Grove module registration.
- Extend existing lifecycle/standalone fixtures to execute the real installed
  Ravel and worker, with fake agents and temporary policies. Put Grove and Ravel
  in separate prefixes with different versions; a decoy old sibling must not
  win. Missing Ravel reports its install remedy. No host Bun/Node is required.
  Replace test support's old sibling symlink seam with isolated PATH/prefix
  fixtures; exercise a decoy older Ravel, malformed version output and a cwd
  decoy. Freeze the k5 public continue report fixture and verify both actual
  Grove readers accept schema 1/source ravel/observed exit_signal and reject
  unrelated sources and malformed reports. k7 reuses this contract fixture.
- Existing continuation, completion, refusal/resume, cancellation, process-group
  cleanup, PTY recovery, confinement, epoch admission and stale-session checks
  pass. This leaf retains bare exit's existing continue semantics.
  Flip every current Grove command, environment, SDK/import and report-source
  spelling together: prompt/signalling text, tree-lifecycle/standalone prose,
  grove-llm CLI, finish skill and Creator instructions, assertions, and the
  loop/standalone/testing-support/cargo scrub lists. Scrub both old and new
  control sets. Creator instructions use RAVEL_RUN_ID with the explicit legacy
  HARNESS_DISPATCH_RUN_ID fallback for sessions still launched by the old driver.
  Enumerate/classify the complete Grove surfaces, including dotfiles, with the
  spine's observed positive/cross-tree controls; keep only named compatibility
  and historical exceptions. Source changes do not change the live prompt.
- Grove's runtime and caller documentation describe the executable boundary.
  Preserve its local keyed-launch implementation and authority guards; no Rust
  dependency points into Ravel. Run the repository `task check` after migration.
  Update source-pinned walkthrough blocks/manifests/indexes in this same leaf,
  wherever its source changed (overview, grove-loop and any other affected
  corpus). Update release-notes.sh's old dispatch:build prerequisite and
  release-prepare.test.sh's command/build assertions with the caller migration.
  Keep old package archive checks working until k6; k12 owns broader prose.

## Notes

Keep the old bundled package/build recipe temporarily where needed to leave
current archive checks green; grove-ravel-delivery-k6 removes that delivery.
Do not make new source prompts or SDK names alter this session's original
installed driver/exit contract. Record the Ravel version used for integration.
