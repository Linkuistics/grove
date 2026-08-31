# architecture-anchors-k19

## Goal

Widen the repository's link-integrity suite so that a
`docs/ARCHITECTURE.md#<anchor>` citation is resolved wherever it appears — in
Rust sources as well as in Markdown. Land it green against today's tree.

## Context

- `plan-k2` found, and `plan-k8` accepted, that decision 5 of `plan-k1` claimed
  the architecture move is guarded by red tests when it is not.
  `every_repository_markdown_reference_resolves` is **Markdown-only**, and
  `every_adr_citation_names_a_decision_record` — the one check that reads `.rs` —
  recognises the `ADR <slug>` form, not a raw
  `docs/ARCHITECTURE.md#<anchor>` link. Raw citations of that shape live in Rust
  sources today, concentrated in `crates/grove-loop/src/tree_lifecycle.rs` and
  `task_tree.rs`, and nothing checks them.
- The machinery to extend is already in
  `crates/grove/tests/reference_navigation.rs`: `architecture_slugs()` collects
  the document's explicit `<a id="…">` anchors, `github_heading_anchor()` derives
  the heading namespace, `repository_files(&[".md"])` is the sweep whose
  extension list is the narrowing, and `uncommented()` already exists for reading
  a `.rs` line.
- `docs/ARCHITECTURE.md`'s own *Documentation ownership* section states the
  convention: the former decision-record slugs are explicit HTML anchors, source
  comments and tests use those stable slugs as compact design references, and
  changing a section title does not change the anchor. The check must permit the
  bare parenthetical form the repository already uses and catch the raw-link form
  that resolves nowhere once a section moves.

## Done when

- A test resolves every `docs/ARCHITECTURE.md#<anchor>` citation in Rust sources
  against the document's anchor namespace, and fails loudly on one that does not
  resolve.
- The check is seen to fail before it is credited: point a citation at an anchor
  that does not exist, watch it go red, and put that back.
- The check's own reach is asserted, in the idiom the neighbouring tests already
  use — a sweep that silently stopped reaching `crates/grove-loop/src/` would
  read exactly like a clean tree.
- `bash scripts/check.sh` passes with nothing else changed.

## Notes

**This lands green today and is useful on its own.** It closes a real hole in the
link-integrity suite whether or not `architecture-move-k31` ever happens; that is
why it is its own leaf and not the first half of the move.

**Do not re-point or move anything.** If a citation is already wrong, that is a
finding: report it, and fix it here only if the fix is the citation and not the
document. Moving prose is `architecture-move-k31`'s work, and mixing the two
makes the guard and the change under guard land in one commit.

**Count nothing in prose.** The number of such citations was measured while this
subtree was cut and will have moved by the time you read this; state the
structural fact and let the test enumerate.
