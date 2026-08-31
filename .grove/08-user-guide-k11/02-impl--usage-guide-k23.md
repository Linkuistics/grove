# usage-guide-k23

## Goal

Bring `docs/USAGE.md` to complete coverage of every row of the inventory
`usage-inventory-k16` committed, each command with a worked invocation.

## Context

- The inventory is the specification. Read it from its own commit; if a row is
  wrong, say so and fix the inventory in this session's commit rather than
  quietly covering something else.
- Decision 2 of `plan-k1`: one document, expanded in place. The guide owns
  *Human workflow and commands* in `docs/ARCHITECTURE.md`'s ownership table and
  nothing else. Session configuration and launch policy are
  `docs/CONFIGURATION.md`'s; grove's vocabulary is `CONTEXT.md`'s and is linked,
  not restated.
- Decision 7 of `plan-k1` fixes the audience: a reader who knows Rust and jj and
  has driven a grove. The guide is their entry point, and every walkthrough will
  link into it — so the anchors this session leaves behind are anchors later
  books will cite.
- A worked invocation is a command a reader can run, with the output they should
  see. `grove-llm`'s verbs print paths and diagnostics with a stable shape;
  quote them rather than paraphrasing.

## Done when

- Every inventory row is covered, and the coverage is checkable row by row rather
  than asserted in aggregate.
- Every command in the guide carries a worked invocation.
- Every stable entry point the inventory names carries an explicit
  `<a id="…"></a>` line immediately preceding its heading. This is the anchor
  set walkthrough books reserve from, and the walkthrough specification requires
  each book to cite one from its `README.md`, so an anchorless guide leaves the
  next five books unable to conform. A renderer-generated heading slug does not
  satisfy it: the explicit form is the whole mechanism, because a slug changes
  silently when a heading is retitled.
- `bash scripts/check.sh` passes, `user_documentation_references_resolve` and
  `every_repository_markdown_reference_resolves` included.

## Notes

**The root brief says an adversarial read against the inventory is what closes
this.** Cut `review-impl` as your last act unless you can say concretely why the
guide does not need one — the inventory makes a reviewer's job mechanical, which
is most of the argument for paying for it. This leaf is the last entry in its
own directory, so a plain `leaf-add` already places the review, and any
integration it cuts, ahead of everything that follows; no `leaf-insert` is
needed here.

**Watch the summary layer.** A guide is a document with an overview, a table of
contents and section bodies; a correction to one section does not reach the
roll-up above it. When you change what a section says, sweep the guide's opening
and any list that summarises it.
