# walkthrough-books-spec-k47

**Integrates:** walkthrough-books-spec-k46

## Goal

Triage the adversarial review of the walkthrough-book system design and rework
the agreement point wherever the findings survive, before either validator leaf
implements against it.

## Context

- Read `walkthrough-books-spec-k46` from its committed review task; its findings
  are the input to triage, not obligations copied into this charter.
- The producer is `walkthrough-books-spec-k20`; its durable artifacts are
  `docs/specs/walkthrough-books.md` and
  `docs/adr/a-book-cannot-witness-its-own-corpus.md`, and its running log records
  the decisions the review attacked.
- This leaf is inserted immediately ahead of `validator-structure-k21`; preserve
  that ordering so neither validator implementation lands against a design still
  under integration.

## Done when

- Every review finding is classified against current source and requirements as
  accepted, narrowed, or rejected, with the evidence and resulting decision
  recorded in this task's running log.
- Every surviving design correction is applied to the current spec and ADR set,
  and any newly explicit work is placed on the live leaf that owns it.
- The resulting manifest and assurance contract can be implemented by
  `validator-structure-k21` and `validator-fragments-k22` without inventing a
  policy or weakening the root brief's exact-corpus requirement.
- Repository documentation checks pass.

## Notes

This is design integration, not validator implementation. Do not absorb either
consumer leaf or edit the frozen walkthrough source corpus.

## Decisions (running log)

**1 · F1 accepted, as a correction to the recorded reasoning and not to the
design.** Verified the historical clause at the producer's parent revision:
"duplicates parents, children, ownership, and ranges already visible in
Markdown" (`docs/specs/ordinal-fs-tree-book.md@d2d839bb-:1059-1065`). The
ownership blocks and their ranges *were* visible in Markdown — `source-index.md`
carries the Ownership blocks table — so the clause does reach top-level ownership
and ranges, and the manifest now duplicates exactly those rows. The ADR's
"the obligation was never in Markdown to be duplicated" is false for that half,
and the spec's *What changed* paragraph inherits the same error. The design is
unaffected: the spec already justifies the one restatement it accepts, because
the tables are checked against the manifest in both directions under a stated
trust order. So the split verdict is narrowed rather than upheld — the
fragment-graph clauses stand, the ownership/ranges clause is admitted as
reversed under a new trust order — and both documents are reworked to say so.

**2 · F2 accepted; the derivation check is kept and the hole beside it is
closed.** Verified: `[book].subject` was diagnostic only, `[corpus] include` was
unconstrained, and `reason` is never validated — so a manifest could declare
`include = ["<subject>/Cargo.toml"]`, declare that one root, and satisfy
set-equality against a tree it had narrowed to fit. Derivation is genuinely
stronger in one direction (a file added to the crate and forgotten by the book
fails) and weaker in another (an authored exception is unchecked), and property 4,
the ADR and the bridge's doc comment all claimed only the first half. Three
corrections: `include` must contain the base patterns derived from `subject`;
each `add`/`exclude` carries a `class` from a closed list, with the one
mechanically checkable class (`inline-test-module`) checked; and the complete
per-book exception set is restated as a normative inventory in the specification
and compared against every manifest by a repository test. Rejected the reviewer's
alternative of a second full inventory: what scales with the corpus stays derived,
and only the six exceptions — precisely what derivation cannot check — are stated
twice. The inventory was verified against the tree rather than copied: `src/**/*.rs`
counts are `grove` 2, `grove-llm` 3, `jj-workspace` 3, `keyed-launch` 8,
`grove-loop` 13, and the only `tests.rs` under any campaign `src/` is
`grove-loop/src/task_grow/tests.rs`, which reproduces the root brief's per-crate
root counts with `Cargo.toml` added.

**3 · F3 accepted in full and widened.** The spec derived the ordered `--through`
domain from chapter `slice` values and required no uniqueness anywhere, so two
chapters could share a slice, the accepted-value list could carry a duplicate, and
`F010` would have no single required page to name. The fix is a schema rule rather
than a validator convention: page `file`, page `id`, chapter `slice`, root `id`,
root `path` and block `id` are each unique within a manifest, a duplicate is
`U002`, and a `[[block]]`'s `owner` and `root` must resolve to a declared chapter
slice and a declared root. Widened beyond the reported `slice` case because the
same class of ambiguity is available on every identity the manifest declares, and
`validator-structure-k21` is forbidden to invent any of them.

**4 · F4 accepted, resolved on the first of the two forks the review offered.**
Verified the gap end to end: the spec permitted an anchorless guide citation
unconditionally and required no non-empty `[guide] anchors`; the five book leaves
require only that links to `docs/USAGE.md` *resolve*, which a file-only link
satisfies; `usage-guide-k23` owed no anchors at all; and `user-guide-k11`'s brief
still justifies its position ahead of the pilot by anchor stability. So the
campaign could have closed green with the guide-first ordering asserting nothing.
Taken the fork that keeps the ordering rather than the one that abandons it,
because two sessions are already placed on its argument and the anchor set is
cheap: a book declaring a `[guide] path` must declare at least one anchor and
must cite one from its `README.md` reader contract (`M201` otherwise). The two
target-document edits are now placed rather than noted — the guide's stable anchor
set is named by `usage-inventory-k16` and published by `usage-guide-k23`, and
`CONTEXT.md`'s anchors land with `jj-workspace-book-k25`, the first book to cite
the glossary. Rejected making every page's guide citation anchored: the coupling
this bounds is one line per book, which decision 8 already settled correctly.

**5 · F5 accepted; recovery placed as a leaf, not performed here.** Confirmed the
loss — the deleted per-page concept responsibilities and worked-example table
exist only at `docs/specs/ordinal-fs-tree-book.md@d2d839bb-`, the tree carries a
`*-structure-k*` requirements leaf for each of the five new deliverables and none
for the relocated book, and `validator-structure-k21` authors that book's manifest
without restoring the human contract behind it. The specification now states that
a structure brief is durable, lives at `docs/specs/<book-id>-book-structure.md`,
and exists for every book including the relocated one. The recovery itself is
`ordinal-structure-brief-k48`, `leaf-insert`ed ahead of `validator-structure-k21`
so the manifest is written against a stated contract rather than beside a missing
one. Cut as `impl` rather than `requirements`: the human decisions were made and
written down once, so this is recovery from version control, and a HITL
elicitation leaf would re-ask questions that already have answers.

**6 · No finding was rejected, and nothing was externalised as redesign.** All
five survived triage against the current source. Each fix repairs the artifact
under a decision k20 already made — the manifest, the derivation check, the
guide-link contract and the structure brief all stand — so none required a new
producer review chain beside this leaf. F2 came closest: had the honest account
of derivation forced the manifest's shape to change, that would have been a
redesign for a new `design` leaf rather than for this session. It did not; the
manifest keeps its schema and gains two constraints and a second, much smaller,
independently authored statement.
