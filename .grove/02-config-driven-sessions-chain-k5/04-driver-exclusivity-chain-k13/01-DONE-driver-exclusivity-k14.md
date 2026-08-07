# driver-exclusivity-k14

**Kind:** design

## Goal

Design the process-ownership rule that prevents concurrent bare `grove`
drivers or stale launched sessions from issuing duplicate or cross-generation
mandates in one working tree.

## Context

- `config-driven-sessions-integrate-k8` deliberately releases the universal
  tree lock before foreground launch so the mandated session can mutate the
  tree; that lock cannot also prove driver exclusivity.
- Preserve tree-only durable workflow state, direct foreground-child ownership,
  restart ≡ continuation, no hidden user configuration, and agent access to
  ordinary tree mutations during a session.
- Decide whether a second driver is refused through process-scoped coordination,
  how descriptors behave across exec, and whether a stable handle needs a grove
  generation binding after finish deletion and later root initialization.

## Done when

- The durable design states exactly when driver ownership begins and ends, how
  concurrent starts and stale sessions fail, and which state is ephemeral versus
  part of `.grove/`.
- Git/jj and crash/restart behavior are explicit and observable test seams cover
  duplicate launch, process death, exec inheritance, and same-worktree grove
  recreation.
- The minimum coherent ADR/spec/glossary set is reworked in place without
  reopening confirmed command-template or task-selection requirements.

## Notes
