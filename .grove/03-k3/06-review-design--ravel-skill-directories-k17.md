# ravel-skill-directories-k17

**Reviews:** ravel-skill-directories-k15

## Goal

Adversarially review the corrected Ravel directory-injection design before Grove
adoption review and implementation planning consume it.

## Context

Read the producer's Grove commit by its stable handle and Ravel commit
`4c16f8e09d58c8f649a375029c8e8fa2933de2b4` in
`/Users/antony/Development/AgenticOrchestration/Ravel`. Follow that repository's
instructions. Its skill/session specs, skill ADRs, glossary, design evidence and
three visual views are the artifact. The root/parent briefs and human corrections
in grove-skill-delivery-k8 are the contract. Earlier catalog proposals are
historical input, not requirements to restore.

The user supplies skills natively or through Ravel. Ravel acquires configured
local/exact pinned sources and injects directory locations; the agent finds and
loads guidance. Grove names its required skill without resolving its location.
The corrected design retains immutable source resources, direct mandatory-file
references and a directory-only recovery descriptor. Challenge that design
against the requirements rather than assuming these choices are sufficient.

## Done when

- Check that the complete artifact set has one coherent directory contract,
  without skill-name resolution, metadata admission, duplicate-name gating or
  a generated skill catalog reappearing through another interface.
- Challenge mandatory file references, agent enumeration, explicit native/path
  requests, ambiguous matches and already injected native guidance. Identify
  any contradiction or missing observation needed to establish implicit skill
  discovery in both real harnesses.
- Challenge local capture, exact Git/offline reuse, descriptor publication,
  source provenance, resource lifetime, compaction recovery and delegated reads.
  Distinguish retained paths from actual permission and instructions from
  removal of inherited ending authority.
- Verify disabled/no-source behavior, source-specific failure remedies and
  original prompt/ending preservation. Check the source-acquisition CLI and
  policy interface for enough precision to implement without introducing a
  caller-side lookup protocol.
- Challenge the Ravel 2/schema-3 migration against the existing caller, SDK and
  worker boundaries. Preserve Grove's separate native-only 1.1 route and require
  explicit 2.x admission, policy examples and acceptance before shared activation.
- Assess whether the ADR set and public acceptance seams carry the real trade-offs
  and distinguish document validation from unimplemented product behavior.
  Produce findings only; classify visible costs separately from contract defects.

## Notes

The producer used Verify-tier graph evidence with direct source fallback for
excluded Ravel docs. Its evidence file names source coordinates and limitations;
refresh coverage before relying on them. No product or real-harness acceptance
was performed. No current creator receipt was present in the producing session.

This is the upstream review, immediately before grove-skill-delivery-k16. For
findings worth acting on, insert integrate-review-design immediately before that
next live sibling, using this review's handle and no transcribed finding list.
Spend no in-session reviewer. Implementation remains common-skill-loading-k9's
planning responsibility after both contracts settle.
