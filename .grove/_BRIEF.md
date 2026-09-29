# Harness selection and execution — brief

## Goal

Build a separate executable that selects and executes an agent harness with the
appropriate model and effort. Grove supplies the selected task and session
prompt; selection policy belongs to the executable and is useful outside Grove.

## Human requirements already established

- Pass the task kind as an explicit argument. The executable must not parse
  Grove filenames to discover it or depend on Grove for ordinary use.
- Pass the original prompt unchanged, plus the selected task file when one
  exists. Other callers must be able to supply sufficient generic task context.
- Support configured selection and dynamic per-invocation selection. Prefer
  TypeScript configuration so policy can perform arbitrary computation without
  growing a declarative policy language in Grove.
- Choose the harness, model and effort together, then execute the harness.
- Every review must use a different model provider from the producer of the
  artifact being reviewed. This includes explicit choices, retries and fallbacks.
  Use actual execution provenance; missing provenance must not silently permit
  a same-provider review. Establish how multi-provider authorship is handled.
- Higher abstraction and/or higher consequences of error justify more effort.
  Consider uncertainty, downstream rework, reversibility and available checks.
- Investigate Jev-style decision models and open-source local alternatives.
  The user has oMLX installed on an Apple M4 Max with 128 GiB memory. Local
  inference is an option to evaluate, not evidence of routing quality.
- Start with useful static policy and a testable dynamic selection interface.
  A learned selector needs representative outcomes and calibration before it
  controls launches. General web findings inform priors; task-specific evidence
  must establish value across model/effort combinations.

## Done when

- An independently usable executable can inspect a proposed choice and execute
  a configured harness using static or computed policy, with actionable errors.
- Grove passes the selected kind, task and prompt through a small generic seam;
  current direct-harness configurations continue to work.
- The interface supplies enough bounded context for dynamic choices and enforces
  provider separation using recorded producer identity.
- Execution preserves terminal, working directory, cancellation, exit status and
  Grove completion authority correctly. A selector cannot complete the task.
- Configuration ownership and trust, overrides, unavailable candidates and
  fallback behavior are explicit and tested at their external boundaries.
- Evaluation captures acceptance, missed defects, false findings, downstream
  repair, human work, latency and total usage. The implementation distinguishes
  choice probabilities from calibrated success probabilities.
- Durable usage/design documentation and the configure-grove skill explain the
  resulting division of responsibility. Required checks pass and the completed
  work is ready for the repository's usual confirmed finish/release sequence.

## Starting evidence and existing contracts

Read `docs/research/grove-model-effort-routing.md` for the dated model research,
local inference investigation and proposed interface. It is research and a
starting proposal, not an implemented API or a substitute for requirements.
Read `docs/CONFIGURATION.md`, `docs/adr/complete-session-configuration.md`,
`docs/adr/a-kind-is-an-open-token.md`, `docs/adr/the-launched-child-is-a-job.md`,
and `docs/adr/untracked-configuration-delta.md` when defining the integration.
Use the repository Taskfile for reusable development workflows.

## Questions for the requirements session

Settle the separate tool's repository/package boundary and delivery, the smallest
useful first increment, configuration trust/precedence, the generic context and
provenance contract, and test seams. Distinguish the human's settled constraints
above from proposed details in the research. Do not re-interview settled choices
or pre-build a speculative task tree before the remaining dependencies are clear.
