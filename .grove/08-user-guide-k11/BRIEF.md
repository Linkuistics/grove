# user-guide-k11 — brief

## Goal

Bring `docs/USAGE.md` to complete coverage of the human's surface onto grove,
against a coverage inventory written down and committed **before** the guide is
edited, so that "complete" names a checkable set rather than a judgement.

## Done when

- A coverage inventory exists, committed in its own change, listing every command
  and flag the guide is obliged to cover and every user journey from scaffolding
  a grove to teardown.
- `docs/USAGE.md` covers every row of that inventory, each command with a worked
  invocation.
- The guide still owns exactly its row in `docs/ARCHITECTURE.md`'s
  *Documentation ownership* table — *Human workflow and commands* — and takes no
  subject another row owns.
- An adversarial read against the inventory has closed it, or the producing
  session has recorded why one was not warranted.

## Decomposition

Two leaves, and the order between them is the point.

1. `usage-inventory-k16` — write and commit the inventory.
2. `usage-guide-k23` — edit the guide against it.

They are separate leaves so the inventory's commit provably precedes the guide's.
An inventory written in the same session as the guide is a description of what
was written, not a standard the writing was held to, and the root brief asks for
the second.

## Pointers

- `docs/USAGE.md` today: *Start, resume, and finish*, *The task tree*, *What
  happens in a session*, *Two habits for the human in the loop*, *Review
  composition and escalation*, *Finish*.
- `docs/ARCHITECTURE.md`, *Documentation ownership*, fixes what may sit directly
  under `docs/` and which subject each file owns. `CONFIGURATION.md` owns session
  configuration and launch policy; do not absorb it.
- The command surfaces themselves: `grove --help`, and `grove-llm --help` plus
  each verb's own `--help`. The help text is the authority on what exists.
- `crates/grove/tests/reference_navigation.rs` holds
  `user_documentation_references_resolve` and the repository-wide sweep; the
  guide is inside the curated user surface the first of those checks.

## Notes

**This node is placed before the books deliberately.** Decision 7 of `plan-k1`
makes the user guide the audience's entry point, so every walkthrough will link
into it; a guide restructured after the books are written moves anchors those
books already cite. The cost is that the campaign's critical path — the validator
— finishes one node earlier, which is the trade recorded at `walkthroughs-k3`
decision 5.

**The link that justifies that placement is a stated contract, not an
expectation.** As `walkthroughs-k3` cut it, no book and no spec required a link
into the guide at all, so two sessions sat on the critical path for an
anchor-stability property nothing checked. `walkthrough-books-spec-k20` now
carries the guide-link contract and every book's `Done when` requires its links
to resolve — so the anchors `usage-guide-k23` leaves behind are a published
surface, and the placement is earned. If that contract is ever dropped, this node
has no reason to sit ahead of the pilot and should move.

**The guide is one document, expanded in place.** Decision 2 of `plan-k1`: a
second document on the same subject would violate the ownership table.
