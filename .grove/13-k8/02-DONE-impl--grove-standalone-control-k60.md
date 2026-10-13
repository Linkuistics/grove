# grove-standalone-control-k60

## Goal

Extend standalone Grove instructions and interpretation to caller-owned
force-stop, without publishing an artifact or claiming success on that result.

## Context

Consume k59's typed Ravel result and exact command helpers. Ravel 0.1.0 needs
no new feature or release; k8's root mandate and live-driver boundary apply.

## Done when

- Standalone prompts supply the selected canonical `end-session` commands for
  successful completion and force-stop, and explain each result's meaning.
- Real installed fake-agent tests show continue publishes checked outputs,
  force-stop leaves destinations absent and reports failure, and enclosing
  lifecycle/standalone channels remain untouched.
- Retain refusal, cancellation, natural exit, malformed/missing/unsupported
  result and confinement/publication tests. Remove the temporary continuation
  adapter if no caller needs it; reconcile affected source-pinned pages.
- Relevant nextest and installed standalone suites plus `task check` pass.
- No install/release/teardown; retain the live prompt's original ending contract.

## Implementation plan

1. Extend the existing real installed Ravel/shell fixture to execute both
   canonical commands from the actual standalone prompt. Assert continue's
   checked publication and force-stop's failed status, absent whole output set,
   group cleanup and untouched enclosing channels; watch the new cases fail.
2. Supply both commands and their meanings, consume `SessionResult::read`, and
   reject `ForceStop` explicitly before validation/publication. Keep cancellation,
   surviving-group and supervisor-status checks. Remove `read_continue` after
   its last caller migrates, retaining exact-byte reader tests.
3. Extend nested ending coverage and invalid-result publication coverage. Update
   current standalone specs/ADR and affected source-exact walkthrough fragments.
4. Run focused nextest through the project tasks, installed command/result and
   authority tasks, then one frozen `task check`. Retire and seal this leaf;
   k61 retains parent acceptance and final surface/archive reconciliation.

## Decisions (running log)

- The existing k8 brief settles the design and real installed fake-agent seam;
  no new agreement point or standalone design artifact is needed. Force-stop
  returns failure with `failed` transcript status and publishes no artifacts,
  even when the entire output set is valid. Continue alone reaches publication.
- Preserve the standalone ordering of group/cancellation/supervisor checks.
  No teardown authority exists in this caller. Both canonical commands use
  k59's quoting helpers, and all meaning stays with Grove, not Ravel.
- Removed the temporary continuation-only adapter; both production callers
  consume typed dispositions, and remaining transport fixtures use the same
  reader. Exact-byte, sparse-tail, installed two-runner and refusal coverage
  stays in place. The graph's inbound trace omitted two test consumers; a
  literal source sweep found and migrated them before the full gate.
- The real prompt-command regression supplies the adversarial instrument:
  it failed against the missing command, then passed both endings, whole-set
  publication rules, descendant cleanup and untouched enclosing controls.
  A fresh read of this same narrow branch would buy less evidence; k61 already
  owns broader contract acceptance. No in-session reviewer is materialised.
- The first full gate passed runtime/docs but refused the host's newly selected
  Bun 1.4.3 at the existing 1.4.2 policy pin. Reuse the already present,
  previously frozen `/Users/antony/Development/Ravel/target/tools/bun` via this
  verification shell's PATH; change no installation, pin, policy or owner config.
  Preserve the failed gate and its matching digests, then repeat the gate on
  final bytes (including a qualification that only a continue result can remain
  successful after a later managed nonzero exit).

## Verification

- Final `task check` passed all 18 principal checks with existing pinned Bun
  1.4.2 selected on this shell's PATH: 1,385 workspace nextest cases (31 skipped),
  nine doctests and six books with final/all validation. `target/k60/check.log`
  holds the complete log. The earlier host-Bun refusal is retained in
  `target/k60/check-system-bun.log`; its other 17 checks passed. Both runs' 1,385
  runtime identities and outcomes agree individually (`runtime-outcomes.json`).
- `task test:grove RAVEL=/opt/homebrew/bin/ravel CLI_ARGS='--test standalone'`
  passed all 24 cases through real installed Ravel and deterministic confined
  shell agents. Both prompt commands, valid whole-set completion, force-stop
  with no destinations and failed status, descendant cleanup, outer sentinels
  and nested completion/force-stop/cancellation pass. Natural/nonzero exit,
  refusal, timeout, invalid results, runtime grants and publication races remain
  covered. The new prompt-command case's failing baseline and green run are
  `target/k60/commands-{red,green}.log`; the full suite is `standalone.log`.
- Explicit `task ravel:commands`, `task ravel:result` and `task test:authority`
  passed two regular/two installed command cases, five regular/three installed
  reader cases (both runner APIs and both dispositions), and nine authority
  cases. Complete logs: `target/k60/installed-{commands,result,authority}.log`.
- All 2,983 frozen files matched before/after both full gates: tracked source,
  docs, task notes and configs, installed Ravel 0.1.0, policy node_modules and
  pinned Bun. `target/k60/check-inputs{,-after}.json` records final per-file
  digests; `check-system-bun-inputs{,-after}.json` records the first run. This
  does not freeze every host/toolchain file read. No subject was edited during
  a measurement. Post-verification changes are task evidence and retirement.
- `task legacy:verify` with the applied original-driver plan returned `ok`
  (`target/k60/legacy-verify.log`). Ravel's checkout remained clean at published
  main 3916a4a1; installed Ravel stays 0.1.0. No install, release or teardown ran.
  No new framework API or version decision was needed.

## Handoff

k60 supplies explicit standalone force-stop through the unchanged public Ravel
CLI and removes k59's temporary adapter. Current standalone specs/ADR and both
affected source-exact books agree. k61 remains live and owns the full parent's
active-surface, separate-version/archive and lifecycle acceptance. The k8 node
is not closed; the original driver's launch and final signalling contract remain.
