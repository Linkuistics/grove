# lifecycle-methodology-k79

**Kind:** impl

## Goal

Reconcile the provisioned lifecycle guidance and launcher with the shipped bare
configured driver, authoritative mandate, and resumable finish session.

## Context

- Depends on `legacy-review-removal-integrate-k65`.
- Primary artifacts: `content/SKILL.md`, `content/prompts/`, provisioning and
  reference-navigation tests.
- Keep session-kind taxonomy and review-ownership detail for the next two
  increments unless this lifecycle slice must adjust a shared sentence.

## Done when

- Bootstrap resolves the prompt-mandated stable handle without picking; bare
  `grove` is the sole lifecycle entry; configuration is the only launch policy;
  finish is a resumable HITL session.
- `content/prompts/continue.md` is the only embedded launcher; obsolete
  `start.md` and `retire.md` are deleted and not embedded or provisioned.
- Canonical lifecycle, prompt, provisioning, `cargo fmt --check`, and
  `cargo test --locked` checks pass.

## Notes

This increment leaves a complete usable lifecycle description before taxonomy
and review-policy language are contracted.
