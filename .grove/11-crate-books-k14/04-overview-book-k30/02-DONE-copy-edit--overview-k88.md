# overview-k88

## Goal

Copy-edit the system overview at `docs/walkthroughs/overview/` as the draft
left it: sentences, terminology, and consistency with the prose contract in
`docs/specs/walkthrough-books.md` and with the precedent it names,
`docs/walkthroughs/jj-workspace/`. No restructuring: a structural finding is a
`draft` re-run leaf, never a fix here.

## Context

- The document: `README.md`, five chapters, two lookup indexes and
  `walkthrough.toml`, at `final=true` over 3 files and 204 lines. Its
  structure brief is `docs/specs/overview-book-structure.md`; its glossary is
  `CONTEXT.md`.
- **What the draft hands to this stage** is under `## Handed forward` in this
  node's `BRIEF.md`, per chapter: the figurative, emotive and persuasive
  wording each drafting session left in place because style was not its
  charter, including phrases that are the structure brief's own (*centre of
  gravity*, *through-line*, *the argument in miniature*) and the precedent's
  (*what goes red*). Decide once whether those stand as the book's vocabulary,
  and apply the decision across all five chapters rather than per occurrence.
- **What this stage must not undo** is under *Pointers* in the same brief:
  six claims the draft adjudicated against the frozen corpus, where a page
  states the checkable fact beside a comment that says otherwise (the function
  count, two version-inheritance clauses, the `TemplateSource` read count, the
  `[[bin]]` privacy shape, the comment-line count); the names the manifest
  depends on (`01-orientation.md#the-binary`, `02-the-surface.md#the-imports`,
  every chapter's H1); and two sentences in chapter 5 naming
  `docs/ARCHITECTURE.md` by path, which are `architecture-move-k31`'s to
  reword when the module table lands, not this stage's.
- The one term spelled two ways that the draft found and aligned is the third
  mechanism (*a convention nobody checks* in `README.md` against *a convention
  a test checks* on three chapters); the README's new sentence is the first
  place to read for a second such pair.

## Done when

- The whole document has been read against the prose contract and the
  precedent, and every change is within this stage's charter.
- `book-check --repo . --book docs/walkthroughs/overview --final --check all`
  is valid: 3 files, 204 resolved lines, 0 deferred, `final=true`; `bash
  scripts/check.sh` passes.
- Entries this stage closes are cleared from `## Handed forward`; anything the
  `art` or `proof` stage owns is added there.
- **Last act**: `grove-llm leaf-add overview-book-k30 overview --kind art`,
  unless a live later sibling under `overview-book-k30` already holds that
  stage — read that off the node's live entries.

## Notes

The chain is lazy and its membership is not optional: cut `art` even if this
read found little. A defect the `draft` owns is sent back as a contiguous run
of re-run leaves from `draft` through `proof`, in place of the ordinary last
act (`docs/adr/a-feedback-edge-is-forward-tree-growth.md`); a second such run
against this document is an escalation, not a third leaf.
