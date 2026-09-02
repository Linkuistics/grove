# architecture-residue-k75

## Goal

Reduce `docs/ARCHITECTURE.md` to decisions, constraints and measurement records
only, by deleting the crate-internal description that every landed crate book has
made redundant — reaching the end state the root brief and `crate-books-k14`'s
brief both state, which no earlier leaf in this node can reach.

## Context

- Cut at `overview-structure-k29` (decision 9), when decision 2 of that interview
  settled that the overview takes system-level description only. Six sections
  survive `architecture-move-k31` because they describe `grove-loop`'s and
  `jj-workspace`'s internals rather than the system: *Task-tree data model*,
  *Task kinds and composition*, *How the methodology reaches a session*,
  *Lifecycle and resumption*, *Human authority and completion* and
  *Version-control seam*.
- **The mechanism is deletion, not a second move.** A crate book re-derives its
  crate's account from the source it reconstructs; it does not absorb this
  document's prose, and the outbound-link contract would forbid it citing this
  document anyway. So a passage here leaves when its book exists and covers it,
  and VCS holds the text.
- k31 marks each surviving descriptive passage with the book that owes it. This
  leaf inherits that list; it does not re-derive the boundary. Where a marked
  passage looks wrong, the rule is `docs/specs/overview-book-structure.md`, *The
  rule* — the same positive move test, applied here as a positive delete test
  with the same default: when in doubt, it stays.
- Anchors: every explicit `<a id="…"></a>` stays above surviving argument (k29
  decision 4). Twelve Rust citations of `library-refusals` and eight of
  `tree-access-lock` sit inside the sections this leaf edits; both anchors
  precede argument, not description, and must still resolve when this leaf is
  done.
- This runs after `grove-loop-book-k37` and after every other book leaf in this
  node, and is appended last for that reason. An insert that later lands ahead of
  it is the thing that is wrong, not this position.

## Done when

- `docs/ARCHITECTURE.md` carries decisions, constraints and measurement records
  only. Every deleted passage is covered by a landed book, and the commit message
  names which book covers which deletion.
- No explicit anchor is removed, and
  `every_architecture_anchor_citation_in_a_source_resolves` is green with no
  Rust edit.
- `every_repository_markdown_reference_resolves` is green: k31's forward pointers
  still resolve, and a pointer whose target section this leaf emptied is kept
  where the section's decisions survive and re-pointed where they do not.
- The *Documentation ownership* table, this document's opening paragraphs, and
  `CONTEXT-MAP.md`'s account of what `docs/ARCHITECTURE.md` holds all describe
  what is then true.
- `bash scripts/check.sh` passes.

## Notes

**A finding against a section does not reach the summary layer.** The opening
paragraphs of `docs/ARCHITECTURE.md` describe the document; the ownership table's
own row describes it; `CONTEXT-MAP.md` describes it. Deleting a section leaves all
three stale and reading as descriptions unless each is swept.

**Do not delete what no book covers.** A marked passage whose book did not land,
or landed without covering it, stays — marked, with the reason — rather than
being deleted on the strength of the mark. "Made redundant" is a claim about a
book that exists, and a deletion is only as safe as that claim.

**The `include_str!` assertions in `crates/grove-llm/tests/composition_guidance.rs`
read this document** and may go red on any deletion here. Turning them green is
by re-pointing the content or the path, never by loosening the assertion.
