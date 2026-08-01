# doubt-grove-design-hardening-integrate-k13

**Kind:** integrate-review-design

## Goal

Classify and integrate the hardening review so the spec and ADR set are safe to
drive implementation.

## Context

Read the artifact from `doubt-grove-design-hardening-k11` and findings in
`doubt-grove-design-hardening-review-k12`. Classify each as contract misread,
actionable issue, accepted visible trade-off, or noise before editing.

## Done when

- Every review finding is classified with evidence and each real issue is fixed.
- Promotion, receipts, nullable-model comparison, warning payloads, and VCS
  semantics require no implementation-time design invention.
- The spec/ADR set remains a minimum coherent current-state set and the root
  brief/glossary are reconciled if the contract sharpened.

## Notes

If a finding changes a human-owned requirement rather than clarifying it, stop
and ask rather than silently rewriting the baseline.

## Integration

| Finding | Classification | Evidence and disposition |
|---|---|---|
| `doubt-grove-design-hardening-review-k12` H1 | Actionable issue | `CONTEXT.md` contradicted the receipt ADR on both write ordering and identity checks. Reconciled it to `DONE` first, then unconditional receipt replacement, gated by worktree + routed handle + factual pick. |
| `doubt-grove-design-hardening-review-k12` H2 | Actionable issue | The measured `git mv` interruption separates the directory rename from Git's index update. The spec and *promotion-transactions-fail-closed* ADR now prepare final index paths while `PROMOTING-*` remains visible and make one plain filesystem rename the final landing seam. |
| `doubt-grove-design-hardening-review-k12` H3 | Actionable issue | `tree_read::pick` requires the Grove root directory, while `brief_chain` deliberately skips absent briefs. The lock target is now the open `.grove/` directory descriptor, not optional `BRIEF.md`. |
| `doubt-grove-design-hardening-review-k12` H4 | Actionable issue | A moved producer is intentionally invisible to ordinary `pick`/`resolve`. Promotion now detects and enters matching recovery before path, liveness, kind, or current-pick validation. |
| `doubt-grove-design-hardening-review-k12` H5 | Actionable issue | A generic reader cannot derive a producer handle without task-content or positional inference. Diagnostics now name the exact reserved path and use that path as the recovery argument. |
| `doubt-grove-design-hardening-review-k12` H6 | Actionable issue | Current `resolve_kind` receives only kind plus optional harness, while `routed_leaf` performs another pick. The canonical routing peek is now one structured `kind --with-harness --json` result containing path, handle, kind, and harness; malformed or handle-free non-empty output stops before launch. |
| `doubt-grove-design-hardening-review-k12` H7 | Actionable issue | Current path resolution canonicalises before recovering identity, so a serialized waiter loses the moved path. Promotion now reduces an accepted path to its stable handle and specifies `changed: false` for the stale-path waiter even if retirement or an insert wins the next lock. |
| `doubt-grove-design-hardening-review-k12` H8 | Actionable issue | Historical chains were flat, so the brief-less-parent compatibility claim was too broad. Promotion now scans the whole sibling level for stable `Reviews` metadata; the spec explicitly exposes metadata-free flat chains as unidentifiable without forbidden suffix/position grammar and requires annotation before promotion. |
| `doubt-grove-design-hardening-review-k12` H9 | Actionable issue | Grove now unconditionally replaces a pre-existing receipt after `DONE` and tests that path. The spec narrows freshness to Grove's cooperative writes: manually restoring a live producer while retaining its receipt creates an accepted visible generation ambiguity, diagnosed as possibly stale if replacement fails rather than turning advisory metadata into a retirement gate. |
| `doubt-grove-design-hardening-review-k12` H10 | Actionable issue | Receipt/relationship disagreement now yields `uncheckable(reason=receipt-producer-mismatch)` and names only the relationship producer. Decomposing a review leaf is documented to drop the relation to the brief and make the new child uncheckable unless explicitly reattached; Grove does not infer it from position. |
| `doubt-grove-design-hardening-review-k12` H11 | Actionable issue | The lock is now a canonical glossary surface, emits one contention diagnostic before waiting, and the structured peek removes the launch/readiness re-picks that currently swallow `tree_read::pick` errors. Pending-transaction failure remains a surfaced routing error. |

The review's attempted counterexamples for nullable-model identity, rollback,
happy-path Git landing, symlinked worktree identity, hidden-key reuse, and
`leaf-insert` serialization remain closed as recorded; no artifact change was
needed for them.

## Doubt cycle

- **Root brief discriminator — valid and actionable.** A missing optional root
  brief accidentally matched the metadata-free composition-node refusal. The
  structural signal now applies only to non-root parents, with a root/no-brief
  executable seam.
- **Retry after intervening retirement — valid and actionable.** Advisory-lock
  fairness cannot guarantee a waiting second promoter runs before retirement.
  Completed-shape recognition now precedes liveness/current-pick gates and
  returns `changed: false` for live, terminal, or pre-empted producers, with the
  competing-retirement seam made explicit.

Both fixes are narrow state-ordering clarifications covered by executable test
seams, so they meet Grove's exception to escalating a second review need. The
leaf-wide reviewer allowance is exhausted; no second reviewer was run.
