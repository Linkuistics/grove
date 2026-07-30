# confirmation-prose-integrate-k17

**Kind:** integrate-review-impl

## Goal

Triage `confirmation-prose-review-k16`'s findings and apply the real ones.

## Context

Read `docs/adr/confirmation-boundary.md`, then `k15`'s diff, then `k16`'s findings
— in that order. The ADR is the authority on what the prose should say; `k16` is
evidence about whether it says it.

## Done when

- Every finding is **reproduced before it is touched** — run the grep, run the
  test, read the section. `chain-node-integrate-k11` accepted nothing on assertion
  and that is the standard here.
- Every finding is either applied or **explicitly upheld as rejected**, with the
  reason. A rejected finding that is worth remembering goes in the commit message;
  one that changes a recorded decision goes in the ADR set.
- A finding against the **ADR** itself (see `k16`'s Notes) is reworked **in place**
  in `docs/adr/confirmation-boundary.md` — never appended as a superseding record —
  and every citation of it is reconciled: `docs/adr/pruning.md`,
  `docs/adr/task-tree-scheme.md`, `docs/adr/task-kind-taxonomy.md`,
  `docs/adr/in-session-finish-cycle.md`, `docs/specs/task-kind-taxonomy.md`,
  `CONTEXT.md`. Grep the slug; do not walk that list.
- The claim grep is **re-run after the fixes** and comes back clean. Applying a fix
  is not evidence the claim is gone.

## Notes

**If the triage decides the whole decision is wrong**, that is a legitimate
outcome and it is HITL — say so and stop rather than reverting the ADR set
unilaterally. Reversing a decision three ADRs and the glossary now cite is a
commitment-shaped call, and this leaf is AFK.

**Do not absorb adjacent work.** `CHANGELOG.md` is `changelog-unreleased-k13`;
five stale `src/` module headers are `stale-module-headers-k14`. If this leaf
surfaces something new, `leaf-add` it.
