# paused-grove-handoffs-k320

## Goal

Write the brief-update text for the three paused workstreams. Each one can
then rebase onto this grove's results when it resumes.

## Context

- The parent brief, Done when, sixth and seventh items.
- `docs/repository-moves-record.md`, section "The handoff to the owner
  cutover": the references in paused task trees, such as the TestSubject
  path in `rhai-book-use-k74`.
- `docs/coordinated-release-evidence.md`: the release receipts and the row
  "Rollout, for the paused Rhai workstream".
- The node `ravel-two-release-k304`: the book work this grove did.

## Done when

- A durable document in Grove's `docs/` holds one section per workstream. The
  section names each workstream's workspace and its affected leaves.
- The editorial grove (`grove.gh-issue-14-documentation-skills-for-editorial-pipeline`)
  and the design and decomposition grove
  (`grove.improve-initial-design-decomposition-skill`) receive their text. It
  names the new skill locations and the Homebrew and Ravel delivery. It keeps
  craft guidance separate from kind charters.
- The Rhai workstream (`Ravel.gh-issue-2`) receives text with the 2.0.0
  release receipts and the owner-conversion receipt of
  owner-policy-conversion-k245. It directs that workstream to prune or
  retarget `release-ravel-2-0-0-k37`, `homebrew-upgrade-k38` and
  `owner-policy-conversion-k39`. A resumed k37 would otherwise cut 3.0.0.
- The Rhai text states that this grove ran draft, copy-edit, art and proof
  over chapters 7 to 11 before the cut. The owner's Read 1 and Read 2 stay
  with that workstream. It directs that workstream to reconcile
  `rhai-book-inside-k35` and `-k36` with the released text.
- The document has a named place for the promotion and switch receipts.
  Finish fills it, and the finish handoff names it.

## Notes

Do not edit those workstreams' task trees, and do not signal their sessions.

## Decisions (running log)

- 2026-11-13: The document is `docs/paused-grove-handoffs.md`. It has shared
  facts, one section per workstream and a receipts section. Each workstream
  section ends in "Text for the root brief", which the human copies into that
  workstream's `_BRIEF.md`. The text stays checked prose, not a code block.
- 2026-11-13: A repeated sweep of the live entries of the three trees found the
  references of the moves record's table and no other. This grove's own tree
  was the positive control. A mutated pattern found none.
- 2026-11-13: All VCS reads of the paused workspaces used
  `jj --ignore-working-copy`, so no snapshot or operation was recorded there.
- 2026-11-13: Ravel `main` and the tag `v2.0.0` carry the Rhai tree. No commit
  after `24a545236185` changes `.grove/`, so the Rhai rebase meets no conflict.
- 2026-11-13: The design grove's pinned trial revision `c14276be9401` is on no
  remote. The bookmark `grove-approved-history-k702` keeps it reachable, so its
  text says to keep that bookmark through the rebase.
- 2026-11-13: The owner's installed `config.rhai` has an unrecorded digest,
  `21ca2e51…`, modified 2026-11-11. The Rhai section reports it as an
  observation. A note in owner-final-inventory-k322 carries the question.
- 2026-11-13: A note in finish-handoff-k324 names the section "Promotion and
  switch receipts". The same edit split an earlier paragraph of that leaf for
  the writing rules, with no change of meaning.
- 2026-11-13: No in-session reviewer. Each claim cites its record, and the
  human reads the text before applying it.
