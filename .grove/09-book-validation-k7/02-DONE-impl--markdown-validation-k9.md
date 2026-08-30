# markdown-validation-k9

## Goal

Implement and test deterministic Markdown-structure and local-link validation
for the multi-page book.

## Context

- Read `book-system-k6`, `fragment-validation-k8`, and this subtree's brief.
- The fragment validator remains the sole owner of source reconstruction; this
  leaf validates document structure and navigation.

## Done when

- Failure fixtures cover missing/duplicate page identities, malformed required
  structure, broken relative file links, missing heading anchors, and links that
  escape the permitted book/repository scope.
- Validation follows the design's page ordering and navigation contract without
  imposing prose judgments that belong to editorial review.
- External URLs are syntax-checked only; the command performs no network fetch.
- Diagnostics are deterministic and name the source page and link or structure
  at fault.
- The design's committed example or fixture book passes, tests pass, and one
  documented command runs both the Markdown/link checks and fragment checks.

## Notes

Use the repository's actual Markdown conventions for anchor generation rather
than inventing a subtly different renderer model.

Repository-wide verification currently fails
`reference_navigation::every_repository_markdown_reference_resolves` because
the specification's illustrative inline-code destination `relative/path` is
classified as a repository reference. This predates `fragment-engine-k30` and
belongs at this Markdown/link-validation seam; reconcile the example and the
repository-wide scanner while implementing the shared link contract.

## Decisions

- The fragment parser's opaque ranges are also the authority for Markdown
  headings, anchors, links, and production-looking fences. A public shared link
  scanner applies the same fence and inline-code rules to repository docs.
- All filesystem inputs are canonicalized beneath the explicitly supplied
  repository root. Fragment-only runs do not load Markdown link targets.
- `--check all` is the documented and default combined validation mode; the
  narrower `fragments` and `markdown` modes remain available for diagnosis.
