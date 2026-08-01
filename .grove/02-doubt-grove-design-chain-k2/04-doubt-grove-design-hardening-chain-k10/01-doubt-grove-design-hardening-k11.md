# doubt-grove-design-hardening-k11

**Kind:** design

## Goal

Rework the doubt/Grove composition design to close the second-pass
crash-consistency and receipt-correctness findings without changing the
confirmed orchestration boundary or advisory-diversity requirement.

## Context

Read the root brief, `docs/specs/doubt-grove-review-mechanics.md`, and its three
cited ADRs. The narrow doubt pass run by `doubt-grove-design-integrate-k5`
surfaced seven issues that must be independently classified before editing:

1. Promotion has no serialization across concurrent mutators, so two commands
   can allocate and land duplicate key runs before either sees `PROMOTING-`.
2. The design uses power-loss language without specifying durability ordering
   (`fsync`) or narrowing the guarantee to process interruption.
3. A prior session's valid receipt can survive when a later finisher cannot
   overwrite it but retirement must still proceed.
4. Worktree plus current-pick checks do not bind inherited launch context to the
   leaf that the driver routed.
5. Harness-scoped defaults leave `null`-model equality ambiguous across
   different harnesses.
6. The staging move is VCS-aware but the landing rename's Git/Jujutsu metadata
   behavior is unspecified.
7. An absent/malformed relationship cannot support a warning that names a
   producer handle without inventing a forbidden positional inference.

## Done when

- Every finding is classified against the confirmed root contract and each real
  issue is fixed in the spec/ADR set.
- The promotion interface has explicit concurrency and interruption/durability
  semantics, including Git/Jujutsu behavior, that implementation can test.
- The receipt protocol cannot preserve a stale authoritative target and binds
  session context to the actual launched leaf without overriding the session's
  factual pick.
- Default-model identity and every uncheckable warning payload have one
  implementable test oracle.
- The ADR set remains a minimum coherent current-state set.

## Notes

This is design hardening, not implementation. If satisfying a finding changes a
human-owned requirement, stop and ask rather than silently weakening it.
