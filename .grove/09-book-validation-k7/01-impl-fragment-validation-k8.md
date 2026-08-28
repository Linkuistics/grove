# fragment-validation-k8

## Goal

Implement and test exact fragment tangling and complete source-coverage checks
for the book format settled by `book-system-k6`.

## Context

- Read `book-system-k6` and this subtree's brief.
- Follow the repository's relevant language style and test-driven-development
  guidance before implementation.

## Done when

- Tests first demonstrate failures for duplicate IDs, unresolved references,
  cycles, unreachable fragments, missing source bytes, duplicated source bytes,
  unknown roots, and newline/whitespace drift.
- Recursive expansion of every declared source root can be compared byte for
  byte with its repository source file.
- Scoped mode verifies only the fragment/source ownership claimed by an
  authoring leaf. It accepts a hole declared as deferred to a named later slice
  and reports that hole as deferred rather than unresolved. Exhaustive mode
  requires exactly the fifteen files in the book brief, zero deferred holes,
  and rejects extras or omissions.
- Failure fixtures include a deferred hole whose named later slice never fills
  it, distinct from an ordinary unresolved reference.
- Diagnostics identify the fragment ID, source root, and relevant book location
  deterministically.
- The design's committed example or fixture book passes the scoped check, all
  validator tests pass, and invocation is documented for later leaves.

## Notes

Do not normalize line endings, trailing spaces, blank lines, or final newlines:
the criterion is source identity, not semantic equivalence.
