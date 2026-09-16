# workspace-reload-k30


## Goal

Prove composed workspace isolation, exact launched argv and next-session reload
using the selected-profile adapter delivered by workspace-selection-k29.

## Done when

- Temporary jj workspaces and fake executables demonstrate opposite lead/review
  arrangements, repository/worktree precedence, local replacement/default/empty
  selection, and parameter-only experiments without cross-workspace leakage.
- Parameter values containing spaces, quotes, dollar/slot text, empty strings and
  shell punctuation reach the fake child with exact argument boundaries.
- Editing a selection and a shared value while a child runs leaves that child's
  argv/process intact and changes the next session's command. Exercise the real
  driver and existing readiness/recording seams, without real agent sessions.
- Invalid active configuration at the pre-launch reload refuses a launch while
  preserving the applicable tree boundary; inspect existing coverage before
  adding redundant cases. Retain legacy/source-admission regression coverage.
- Update affected documentation and source-exact walkthroughs as necessary and
  run bash scripts/check.sh. Check and close workspace-configuration-k10 against
  its complete brief, carrying any named gap as work before closing it.

## Context

The parent brief owns the complete contract and acceptance ownership. k29 owns
selection, structured diagnostics and focused mutation/launch behavior; this leaf
owns the broader composed observations. Reuse lifecycle_cutover.rs and
loop_driver.rs fixtures. Preserve .cargo/config.toml's signal guard; execute new
builds in acceptance and invoke actual tree verbs directly with grove-llm.

## Implementation plan

- Extend lifecycle_cutover's real jj/driver fixture with two secondary workspaces,
  opposite bindings, a parameter-only profile, local selection replacement,
  inheritance and empty selection, recording exact NUL-delimited child argv.
- Extend loop_driver's bounded process/readiness fixture with a live-child
  handshake around selection and shared-value edits, and a tree-lock handshake
  that makes configuration invalid between transition admission and launch.
- Run focused acceptance and mutation controls, reconcile the parent brief and
  documentation, then run the fixed-input principal checks before retirement.

## Decisions (running log)

- The reviewed design and existing adapter need acceptance evidence, not new
  configuration behavior. Tests remain in the existing process suites and reuse
  their real jj setup, child recording, readiness and cleanup seams.
- Graph generation 2026-09-16T11:54:39Z has stale metadata for lifecycle_cutover
  and the production driver; direct source reads supply current evidence there.
  Existing legacy reload and pre-transition invalid-policy cases remain; the
  new lock-controlled case specifically exercises the second load.
- The late-load mutation (reuse pre_transition_config) makes the new refusal
  test fail by launching the old command. Giving personal selection precedence
  over local selection makes the workspace test fail on the empty-list case.
  Both controls are restored before final verification.
- The single adversarial reviewer found an unbounded wait after next-child
  readiness. Add the adjacent fixture's bounded try_wait pattern before finish;
  readiness proves recording, not process termination. No production change or
  resolver decision is required.

## Verification and parent close

- All three modular process tests pass, including 20 exact-argv launches across
  two secondary workspaces. Both deliberate production mutations fail their
  targeted tests; restored production passes the focused rerun and full suite.
- The single reviewer reported no other actionable findings. The bounded final
  wait passes through the same real-driver seam; no further review is needed.
- `bash scripts/check.sh` passes all eight checks, including workspace tests and
  final validation of all six source-exact books. All 1,809 versioned source,
  manifest, documentation, script and fixture paths have identical SHA-256
  digests before and after the run. Only this verification/retirement record
  follows that measurement.
- All six book manifests were inspected: these integration tests are evidence,
  outside their source corpora. The loop chapter now describes the external-edit
  test and records the previously surviving late-load mutation as caught. No
  production bytes, corpus entries, source indices or root lengths changed.
- Parent k10's selection/local-completion/authority and structured diagnostic
  requirements are covered by k29's public SessionConfig and leaf-add/launch
  acceptance. Existing discovery tests retain tracked, unreadable, unparseable,
  unprobeable and actively invalid refusals without fallback; all 24 adapter
  tests pass. k31 covers initial-kind authority before fresh-root creation.
- This leaf supplies k10's composed workspace, parameter-boundary and live-child
  reload evidence, plus invalid policy at the second load. Existing legacy and
  source-admission regressions pass. The parent has no remaining named gap;
  close workspace-configuration-k10 and promote its handoff to the root brief.
  The ADRs already state the delivered contract. Human inspection and example
  delivery remain live as k11/k12, so the root stays open.
