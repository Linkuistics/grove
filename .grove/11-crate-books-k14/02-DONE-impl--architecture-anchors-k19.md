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

## Decisions (running log)

**The scan is over `.rs` sources, and the Markdown half is left where it already
works.** The goal says a citation must resolve "wherever it appears", but
`every_repository_markdown_reference_resolves` already resolves the Markdown
occurrences — every one of them is written as a link destination, which that
sweep reads. Scanning `.md` again for the same text would report a broken link
twice, which `repository_relative`'s own doc comment warns against. The one
non-link Markdown citation in the tree is in `CHANGELOG.md`, which both checks
exclude by contract because history keeps the past. So the new check takes
`repository_files(&[".rs"])` and nothing else.

**Only the repository-relative prefix `docs/ARCHITECTURE.md#` is a citation.** A
Rust source is not rendered, so a directory-relative form has nothing to resolve
against and does not occur in `src/`; it *does* occur inside
`crates/book-validation/tests/`' synthetic Markdown fixtures, where the anchor
(`#boundary`) is invented on purpose. Matching one literal prefix keeps those
fixtures out without a second exception list.

**An empty anchor is not a citation.** `docs/ARCHITECTURE.md#<anchor>` is how
this repository writes the *shape* of a citation when describing the convention
rather than using it; metasyntax names no anchor, so there is nothing to resolve.

**This file joins the scan's exceptions, in the idiom already there.**
`THIS_FILE` was already excluded from the ADR citation scan because the file
quotes the defect it rejects; the control test for this check quotes an anchor
that resolves nowhere, for the same reason. The constant's doc comment now covers
both scans rather than one.

**Reach is asserted in the two idioms the neighbouring tests already use**: the
enumeration must reach `crates/grove-loop/src/tree_lifecycle.rs` and
`task_tree.rs` (the `every_repository_markdown_reference_resolves` idiom), and at
least one citation must have been read (the `every_adr_citation_names_a_decision_record`
idiom). No count of citations is asserted anywhere.

**Seen red before credited.** Repointing one real citation in
`crates/grove-loop/src/task_tree.rs` at `#library-refusals-moved-away` failed the
new test with the file, line and reason; the edit was reverted and the tree is
clean.

**Finding, not fixed here: two cited anchors are heading anchors, not explicit
ones.** `tree-access-lock` and `the-boundary-is-a-build-not-a-commit` are cited
from Rust sources and resolve today only because `architecture_slugs()` also
carries generated heading anchors — retitling either section breaks every
citation of it, which is exactly what the *Documentation ownership* convention's
explicit `<a id="…">` anchors exist to prevent. The other cited slugs are
explicit anchors. Reporting it is this leaf's business; adding the anchors is a
change to `docs/ARCHITECTURE.md` and belongs to `architecture-move-k31`, which
this leaf is forbidden to pre-empt. The new check is what makes that breakage
loud rather than silent whenever it happens.

**The leaf's one in-session reviewer was spent, and it changed the design.** A
fresh context was given the artifact and the four contract clauses with the
conclusion stripped, and asked to break it. Five findings, classified:

- **Valid, fixed here — this file does not need excluding from its own scan.**
  Excluding `THIS_FILE` looked like matching the ADR scan's idiom, but the two
  scans are not symmetric: the ADR scan cannot avoid quoting mislabelled forms,
  whereas the only citation here that must resolve nowhere is exercised through
  `unresolved_architecture_anchor` with a bare anchor and no path in front of it.
  The reviewer showed a probe citation inside this file staying green. The
  exclusion is gone, the `THIS_FILE` doc comment is back to describing one scan,
  and the same probe now fails at `reference_navigation.rs:1131`. The unicode
  fixture interpolates its anchor for the same reason — a literal would be a real
  dangling citation.
- **Valid, fixed here — the anchor character class disagreed with the namespace.**
  `github_heading_anchor` builds anchors with unicode-aware `is_alphanumeric()`,
  so an ASCII-only reader could not spell an anchor the namespace can legitimately
  hold, and would report the truncated stem (`caf` for `café-notes`) as the broken
  name. One token; the two halves now agree.
- **Valid, fixed here — the prefix doc comment overclaimed.** It said matching one
  literal prefix *keeps* `book-validation`'s Markdown fixtures out; it is a
  consequence of how those fixtures are spelled today, and a fixture written
  repository-relative would be read as a citation. Now stated as that, with the
  fix named (spell the fixture the way its Markdown file actually sits).
- **Valid, externalised — `explicit_anchors` admits anchors the renderer does not
  publish.** A fenced `<a id="…">` joins the namespace (no fence tracking, unlike
  `markdown_headings` beside it), and a single-quoted `<a id='…'>` is excluded from
  it. Both are pre-existing, both are latent today — the document carries neither
  form — and both belong to a helper shared with two existing checks, so fixing
  them here would silently redefine the namespace of checks outside this leaf's
  `Done when`. Cut as `explicit-anchor-namespace-k74`, inserted ahead of
  `architecture-move-k31` because that is when a false namespace is most dangerous.
- **Noise — the working copy also changes this task file.** Grove bookkeeping;
  the leaf's `Done when` is about the repository.

No second review was needed: every fix above is covered by an executable seam in
the same file, and each was seen red before it was credited.
