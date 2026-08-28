# reference-navigation-literal-k31


## Goal

Restore repository-wide Markdown reference validation without weakening checks
for real relative links.

## Context

`cargo test --workspace` fails in
`reference_navigation::every_repository_markdown_reference_resolves` because
the inline-code placeholder `relative/path` at
`docs/specs/ordinal-fs-tree-book.md:882` is interpreted as a repository link.
The book spec predates `ledger-and-pages-k27`; that leaf did not modify it.

## Done when

- The scanner distinguishes literal path examples from actual Markdown link
  targets, or the specification expresses the placeholder in an unambiguous
  form without weakening the example.
- A focused regression test proves real broken relative links still fail.
- `cargo test -p grove --test reference_navigation` and
  `cargo test --workspace` pass.

## Notes

Do not broaden into the book's local-link validator; `markdown-validation-k9`
owns that implementation.
