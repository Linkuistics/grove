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

## Decisions (running log)

**1. F1, F2 and F5 accepted as unclear or contradictory contract prose.**
The required node file is `_<slug>.md`; `_BRIEF.md` is root-only.
`transition_to_current` refuses `RootShape::Taskless` before driver selection,
whereas helper selection can return no live leaf. Correct the initialization
rationale throughout the owning prose, CLI help and source comment, and state
the helper/driver distinction. The changelog should explicitly state refusal
and no automatic conversion. These are documentation repairs, not behavior
changes; the ADR contract remains unchanged. Graph startup failed because an
incompatible generation is active; exact source reads supply the evidence.

**2. F3 accepted; use symbols for the affected prose citations.**
The cited locations disagree with `task_name.rs`, `tree_lifecycle.rs` and
`loop_driver.rs`. Replace affected prose coordinates with the functions and
constants being discussed, including their repeated explanations and ledger
rows. Fragment ranges remain exact and validator-owned.

**3. F4 accepted; F6 accepted as trivia.**
The exclusion examples must witness current node directories (`07-k28/` and
`NN-k<key>/`). Preserve the deliberately malformed leaf example. Consolidate
the three missing-file paragraphs into one that keeps the depth and guarded
opening claims. Test files are outside the book production corpus; the changed
CLI and lifecycle source comments must retain matching book fragments.

**4. Summary sweep extends F2 to the chapter 11 introduction.**
Its first-leaf rationale repeated the same false driver-finish implication.
The source header and chapter 14 fragment now say the first leaf is ready for
the first session. Both source edits preserve their line counts and ownership
ranges, so the existing manifests and fragment-ledger rows remain exact. The
chapter 4 CLI explanation is updated with its help fragment. F3's repeated
`finish_slug` citation and neighboring stale kind-call coordinates are also
replaced with their verified symbols; this does not claim an audit of all
source-coordinate prose in chapters 5–21.

**5. Verification passed; no redesign or additional review is needed.**
`bash scripts/check.sh` passed all eight principal checks, including workspace
tests and final validation of all six books. Final prose clarification and
wrapping were then checked by both touched books' `--final --check all` runs
and all 13 `reference_navigation` tests. SHA-256 digests of every tracked regular
file outside `.grove/` were unchanged across that final validation, including
source, documentation, manifests, fixtures, scripts and plugin rules. An isolated
jj fixture confirmed the three untargeted helper verbs return exit 0 and the
no-live-leaves diagnostic on a charter-only root. The rationale sweep detected
the known-bad parent chapter and retained unrelated legitimate comparisons.
All six review findings are applied. No ADR decision changed, no in-session
reviewer was needed, and cutover and migration remain live at the root, so there
is no parent-node close to cascade.
