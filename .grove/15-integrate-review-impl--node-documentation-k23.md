# node-documentation-k23

**Integrates:** node-documentation-k22

## Goal

Triage the review's findings against the release-facing documentation and
apply the real ones, so that `node-cutover-k10` consumes a guide, books,
changelog and architecture summary that state one grammar and one refusal
story with no self-contradiction.

## Context

Read the findings from the review's own commit, by its handle above. The
artifact is `node-documentation-k9`'s corpus: `docs/USAGE.md`, the walkthrough
books under `docs/walkthroughs/`, `docs/ARCHITECTURE.md`, `CHANGELOG.md`'s
Unreleased section, and the test-module comments and fixtures the producer
touched. Two of the findings name source text (a help string in `cli.rs`, two
fixture strings in a `grove-llm` test); any source change taken carries its
matching book fragment, ledger and explanation in the same commit, as
`docs/specs/walkthrough-books.md` requires. Prose line citations in a book are
not checked by `book-check`; only the fragments are.

The installed 20.2.0 binary and this live tree's naming stay as they are.
This leaf authorises no release, plugin install or live-tree conversion.

## Done when

- Each finding is classified — accepted, a trade-off, or rejected with the
  reason recorded — and the accepted ones are applied at the owning document,
  with the summary layer swept for the same claim, and no naming history
  introduced anywhere.
- Glossary anchors, deliberate malformed-input fixtures and model witnesses
  are preserved; a fixture is changed only where the finding shows it no
  longer witnesses what its comment says.
- `bash scripts/check.sh` is green, every touched book's final validation
  included, before committing.

## Notes

Reject a finding by writing why, not by silence; the review's list is the only
one there is, and a rejection recorded here is what a later reader can check.
