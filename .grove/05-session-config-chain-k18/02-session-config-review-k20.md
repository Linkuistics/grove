# session-config-review-k20

**Kind:** review-impl
**Reviews:** session-config-k19

## Goal

Adversarially review `session-config-k19` and record concrete findings for its integration step.

## Context

- Review the committed `session-config-k19` change against
  `docs/specs/config-driven-sessions.md` and
  `docs/adr/complete-session-configuration.md`.
- Concentrate on grammar ambiguity, exact nineteen-kind completeness,
  diagnostic aggregation and spans, argv injection, accidental shell
  evaluation, Rust 1.74 compatibility, and whether the module's interface is
  deep enough to keep those rules local.

## Done when

- Findings are recorded in this task file, each with severity, evidence,
  affected contract, and a concrete correction or explicit no-finding result.
- The review independently exercises representative valid and invalid configs
  and distinguishes implementation defects from choices already settled by the
  spec.
- No production or test code is changed.

## Notes

The reviewer produces findings only; `session-config-integrate-k21` owns fixes.
