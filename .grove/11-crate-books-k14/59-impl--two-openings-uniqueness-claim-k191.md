# two-openings-uniqueness-claim-k191

## Goal

Correct the uniqueness claim at
`docs/walkthroughs/grove-llm/04-growing-the-tree.md` lines 245–247 — *the one
grow verb that has to open the tree twice* — which the same chapter refutes
sixty lines further on.

## Context

- Found by `root-init-drop-order-comment-k99`'s in-session reviewer. The
  sentence introduces *`leaf-decompose`: two openings, in order* as reading
  **the one** grow verb that has to open the tree twice.
- **`leaf-insert` also opens twice**, and this chapter says so at lines 664–669:
  `verbs::stale_cross_refs` *needs its own reading opening to scan the tree for
  stale references*, and first calls `relinquish` on the `TreeWrite` before
  taking it. So one `leaf-insert` run holds a write opening and then a read
  opening.
- What is actually true of `leaf-decompose` and of no other verb is narrower and
  is about **`cli.rs`**: `cmd_leaf_decompose` is the only handler in this book's
  corpus that takes the second opening *itself* — `inherited_kind` reads through
  `grove_loop::read` before `writable` — where `leaf-insert`'s second opening is
  taken inside `grove-loop`, below the surface this book owns, and is a
  relinquish-then-read rather than two live openings.
- The count is not wrong; the **domain** is. The fix is almost certainly to change
  the quantifier's domain — *the one grow verb* → *the one handler on this
  page* — rather than to weaken it to *a* verb, which would lose what the
  sentence is for.

## Done when

- The sentence states a quantifier that holds against the chapter's own later
  passage, with the domain named rather than the set narrowed.
- The neighbouring sentence at lines 244–246 — *the one verb that reads through
  its own opening gives up any guard it still holds first* — is checked against
  the same enumeration; it may carry the same defect, and one reason stated
  for two verbs is usually load-bearing for only one of them.
- `07-what-order-holds.md` is swept for the same claim: line 125's table cell
  and the boundary prose both discuss the second opening, and a finding against
  a section does not reach the summary layer.
- `bash scripts/check.sh` passes, and `book-check --final` is green over
  `grove-llm`. This is prose only — no source byte moves, so no fragment range
  changes.

## Notes

Cut separately from `lock-scan-blind-to-contention-probe-k190` even though both
sit in the same closing passage of chapter 4: that one changes a test and the
sentence reporting it, this one is a quantifier over the chapter's own
enumeration, and each is verifiable without the other.
