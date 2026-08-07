# config-driven-sessions-review-k7

**Kind:** review-design
**Reviews:** config-driven-sessions-k6

## Goal

Adversarially review `config-driven-sessions-k6` and record concrete findings for its integration step.

## Context

- Attempt to disprove the design against `.grove/BRIEF.md`, especially direct
  execution, restart/finish recovery, configuration completeness, migration,
  wrapper commands, Git/jj parity, and removal of harness knowledge.

## Done when

- Findings identify contract violations, hidden harness coupling, ambiguous
  substitution or parsing behavior, unsafe recovery windows, and missing test
  coverage; no design fixes are applied.

## Notes
