# walkthroughs-k38

**Integrates:** walkthroughs-k9

## Goal

Triage the findings from `walkthroughs-k9` and apply the ones that survive
against the current planned subtree.

## Context

Read the findings from the review's own committed task artifact. Do not treat
the finding list as this leaf's charter: each finding may be accepted, narrowed,
or rejected against the evidence.

## Done when

Every finding in `walkthroughs-k9` is classified, every accepted change is
applied to the planned subtree, and the pre-order walk and parent briefs are
reconciled with the result.

## Notes

The review is the source of the findings; this body deliberately carries only
its handle.

## Decisions (running log)

**1 · All five findings survive triage; one is narrowed.** F1, F2, F3 and F5 are
accepted as stated. F4 is accepted as to its evidence and narrowed as to its
remedy: the two-leaf validator split survives, and what is wrong is the *reason*
recorded for it. Nothing here demanded that an artifact be rethought rather than
repaired, so no new producer review chain is cut beside `walkthroughs-k3`.

**2 · F1 accepted — the three reviews are placed, not merely promised.** Verified
against the walk: `walkthrough-books-spec-k20` sits at position 02 of a node whose
positions 03 and 04 consume it, `pilot-preregistration-k24` at 02 of a node whose
03 and 04 consume it, `pipeline-kinds-k27` at 02 ahead of `pipeline-skills-k28`.
`leaf-add` appends at the parent's end, so each review would have landed behind
every consumer. Each of the three bodies now names the `leaf-insert` target
explicitly, and says the same placement binds the `integrate-review-*` step.
`usage-guide-k23`'s `review-impl` is the one that does not need it — it is last in
its directory — and its body now says so, so a later session does not copy the
wrong habit.

**3 · F2 accepted in full — `pilot-measure-k26` is now a node of six.** One leaf
requiring five stage commits plus a committed report is not one focused commit,
and the old body's "decompose if it proves too large" already named the proof. Cut
`developmental-edit-k40`, `technical-edit-k41`, `copy-edit-k42`, `art-k43`,
`proof-k44` and `measurement-report-k45`, which also gives the report the owner
"one child per stage" left it without. This is not speculative decomposition: the
stage list is fixed by decision 11 of `plan-k1` and the sequence was already
known. The node brief records that the per-stage commit boundary is the
attribution rule's instrument rather than bookkeeping.

**4 · F3 accepted — `book-assurance-surface-k39` cut at the end of
`walkthrough-machinery-k10`.** Confirmed against the source: `USER_DOCS` in
`crates/grove/tests/reference_navigation.rs` holds five hand-enumerated top-level
guides and no book, and `docs/ARCHITECTURE.md`'s *Documentation ownership* table
has no row for the overview or any book. Decision 8 of `plan-k1` requires both and
no leaf carried either. The new leaf makes both surfaces *discover* book roots, so
the obligation is met once rather than forgotten five times; `architecture-move-k31`
still owes the overview's row, which is content and not the check. The obligation
is now stated in the root brief, this node's brief and `crate-books-k14`'s.

**5 · F4 accepted as to evidence, narrowed as to remedy.** The finding is right
that the seam is misdescribed and wrong to imply the split may not be workable.
Read at source: `crate::ledger::check` runs only under `Fragments | All`
(`crates/book-validation/src/validator.rs`), and `src/ledger.rs` is where
`PAGE_BY_OWNER` and `SOURCE_INDEX` are defined and `SLICE_ORDER` imported, while
`src/markdown.rs` reads `SLICE_ORDER` too — so every constant `validator-structure-k21`
touches is read on the fragment path. The two-leaf split still lands independently,
because the two *constant sets* are disjoint; what was false is `walkthroughs-k3`
decision 4's claim that the readers divide along `--check`. Both leaf bodies and
the node brief now describe the real seam and require `--check all` of each half.
The retired decision log is left as it stands — a retired leaf's record is history,
and the correction belongs in the artifacts future sessions read.

**6 · F5 accepted — decision 5 is made true rather than reversed.** No book leaf
and no spec acceptance condition required a link into the guide, so two sessions
sat ahead of the pilot for an anchor-stability property nothing checked. Chosen
over moving `user-guide-k11` off the path, because decision 7 of `plan-k1` already
makes the guide the reader's entry point and the link is wanted on its own terms;
reversing the order would have left the anchor risk real. `walkthrough-books-spec-k20`
now owes the guide-link contract, each of the five book leaves owes resolving
links, and `user-guide-k11`'s brief records that the placement is earned only while
that contract stands.

**7 · No in-session reviewer spent.** Every finding was decided against a source
read or the walk itself, and nothing turned on a claim a fresh context could
settle better.
