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
