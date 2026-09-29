# Harness selection and execution — brief

## Goal

Build a separate executable that selects and executes an agent harness with the
appropriate model and effort. Grove supplies the selected task and session
prompt; the executable evaluates user-owned selection policy and is useful
outside Grove.

## Human requirements already established

- Keep the executable in a separate package in this repository for development,
  testing and initial deployment. Ship it with Grove's installation and release.
  Preserve a boundary that allows eventual extraction to a separate repository.
- Pass the session kind as an explicit argument. The executable must not parse
  Grove filenames to discover it or depend on Grove for ordinary use.
- Pass the original prompt unchanged, plus the selected task file when one
  exists. Other callers must be able to supply sufficient generic task context.
- Support configured selection and dynamic per-invocation selection. Prefer
  TypeScript configuration so policy can perform arbitrary computation without
  growing a declarative policy language in Grove.
- Choose the harness, model and effort together, then execute the harness.
- For the first version, every review must use a different provider from the
  artifact's original creator. This includes explicit choices and retries.
  Use recorded execution provenance; missing provenance must not
  silently permit a same-provider review. Keep this simple: no separate model
  comparison or multi-author accounting is required for this increment.
- Encode that review rule in static configuration or TypeScript policy, rather
  than hard-coding it into the generic executable. Artifact-associated execution
  provenance must be discoverable by TypeScript when selecting a reviewer.
- Load personal policy by default. Repository TypeScript runs only when
  explicitly selected through personal configuration or a `--config` argument;
  cloning or entering a repository must not execute its policy code.
- Higher abstraction and/or higher consequences of error justify more effort.
  Consider uncertainty, downstream rework, reversibility and available checks.
- Start with useful static policy and a testable dynamic selection interface.
  A learned selector needs representative outcomes and calibration before it
  controls launches. General web findings inform priors; task-specific evidence
  must establish value across model/effort combinations.
- The agreed first release is static routing plus a programmable TypeScript
  selection interface, with execution and outcome records ready for evaluation.
  A working local LLM selector and its evaluation pilot are follow-up work, not
  required for this first release.
- Incomplete mappings stop the launch with an actionable diagnostic. The human
  rejected automatic fallback; the executable must not fill a missing or invalid
  selection by inventing a default or substituting another candidate.

## Done when

- An independently usable executable can inspect a proposed choice and execute
  a configured harness using static or computed policy, with actionable errors.
- Grove passes the selected kind, task and prompt through a small generic seam;
  current direct-harness configurations continue to work.
- The interface supplies enough bounded context for dynamic choices and exposes
  artifact-associated producer identity to configuration. The configured review
  policy applies the agreed original-creator provider separation rule.
- Execution preserves terminal, working directory, cancellation, exit status and
  Grove completion authority correctly. A selector cannot complete the task.
- Configuration ownership and trust, overrides, unavailable candidates and
  fallback behavior are explicit and tested at their external boundaries.
- Records support later evaluation of acceptance, missed defects, false findings,
  downstream repair, human work, latency and total usage, retaining unknown or
  unobserved outcomes explicitly. Launch success is not task acceptance. Any
  probability fields distinguish choices from calibrated success probabilities.
- Durable usage/design documentation and the configure-grove skill explain the
  resulting division of responsibility. Required checks pass and the completed
  work is ready for the repository's usual confirmed finish/release sequence.

## Acceptance cases

- A caller without Grove supplies a kind, prompt and sufficient generic task
  context and can inspect or run one configured joint harness/model/effort
  choice. A task file is optional. TypeScript can consume caller context without
  needing Grove filenames or a closed enumeration of kinds.
- Static mapping and computed TypeScript policy can each select a configured
  candidate. Inspection explains the effective policy source, choice, reason,
  context sources and expanded arguments without launching the harness. It must
  not promise that evaluating trusted TypeScript is free of side effects.
- An incomplete mapping, unreadable or invalid selected configuration, invalid
  selection result, or unavailable selected executable launches nothing and
  reports the missing or invalid input. No automatic default fills the gap.
  Explicit choice inputs are visible to policy and do not silently become some
  other candidate when they cannot be satisfied.
- A repository containing TypeScript policy causes no execution merely because
  the caller runs there. Personal policy or an explicit `--config` selection is
  the authority to load it; inspection identifies the selected source.
- A review can discover the original creator's recorded provider through
  artifact-associated provenance. Changing today's producer mapping does not
  change that record. The supplied configuration chooses another provider;
  missing original-creator provenance is an incomplete mapping, not permission
  to launch a same-provider review. Keep the first representation simple.
- Grove passes the kind and task from its authoritative selection along with the
  original prompt. Spaces, quotes and shell punctuation in the prompt or paths
  remain data. Existing direct-harness templates remain valid. The adapter does
  not select another leaf or recover its kind from the prompt.
- The selected harness keeps the intended cwd, terminal, cancellation and exit
  behavior. Selection helpers receive no Grove completion authority; the final
  harness receives the completion channel. A selected joint choice stays fixed
  for that harness session.
- Execution records distinguish a proposed choice, launch failure and actual
  execution evidence. Outcome records can associate later acceptance, findings,
  repair and human work with the run without treating absent measurements as
  zero. Records identify the policy/candidate and context versions needed to
  interpret them. No local-selector benchmark is required for first release.
- Grove's existing release/install route delivers the new command on its
  supported targets. Usage documentation and configure-grove explain which
  configuration owns selection, how to inspect it, and how to remedy incomplete
  mappings. Reusable package checks join the repository's Taskfile workflow.

## Agreed test seams

The human agreed these boundaries during requirements:

- Exercise the new command with temporary TypeScript policies and fake harness
  executables. Verify selection, generic context delivery, inspection,
  provenance, trust admission and refusal on incomplete mappings.
- Extend Grove's existing launch-boundary integration tests. Verify unchanged
  prompts, selected kind/task context, terminal and cancellation behavior, exit
  status and completion authority reaching only the final harness. Retain
  direct-harness compatibility coverage.

## Starting evidence and existing contracts

Read `docs/research/grove-model-effort-routing.md` for the dated model research,
local inference investigation and proposed interface. It is research and a
starting proposal, not an implemented API or a substitute for requirements.
The durable boundary decision is
`docs/adr/harness-selection-is-owned-by-policy.md`.
Read `docs/CONFIGURATION.md`, `docs/adr/complete-session-configuration.md`,
`docs/adr/a-kind-is-an-open-token.md`, `docs/adr/the-launched-child-is-a-job.md`,
and `docs/adr/untracked-configuration-delta.md` when defining the integration.
Use the repository Taskfile for reusable development workflows.

## Next design and planning work

The human confirmed the consolidated requirements and the next sessions: an
independent requirements review (`harness-selection-and-execution-k2`), followed
by design (`harness-selection-and-execution-k3`). Design synthesizes
the running decisions in `harness-selection-and-execution-k1` and this brief; it
does not repeat the interview. It owns the executable/package name, runtime and
delivery choice, policy/request/selection protocol, configuration precedence,
bounded generic context, simple provenance association and record format. It
must account for execution and completion authority before choosing a TypeScript
hosting strategy, and preserve the agreed process test seams.

The design produces the enduring area spec, reconciles the ADR set and schedules
review when needed. Planning then cuts independently useful increments against
that design. Do not pre-build implementation leaves before those decisions.

## Beyond the first release

The research records Jev-style decision models and open-source local alternatives
for a later selector pilot. The user has oMLX on an Apple M4 Max with 128 GiB
memory; availability is not evidence of routing quality. A working local selector,
calibration campaign and repository extraction are outside this grove's first
release scope.
