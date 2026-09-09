# colocated-fixture-control-claim-k204

## Goal

Correct what `docs/walkthroughs/jj-workspace/03-subprocess-seam.md` says about the
colocated fixture being a control on
`resolution_ignores_repository_selection_and_temporary_directory_environment`.
The conclusion is right and the mechanism it states is false, measured. And
record the fact that makes the `--colocate` flag load-bearing for a reason the
page does not give.

## Context

Both findings are from `env-guard-set-var-soundness-k185`, which measured them
while re-verifying that leaf's own controls; neither is caused by that leaf's
change, and both predate it.

- **The false clause.** `03-subprocess-seam.md` (the paragraph beginning *The
  claim that this works is asserted directly*, around line 386) says: *"The
  fixture is colocated on purpose, and the test says so: in a tree with no
  `.git` the selectors point at nothing and **the test would pass whether or not
  the scrub existed**."* Measured against jj 0.45.1, with `--colocate` replaced
  by `--no-colocate`, one mutation at a time:

  | fixture | scrub | result |
  | --- | --- | --- |
  | `--no-colocate` | intact | **red**, at `intended/.git/index` |
  | `--no-colocate` | `GIT_INDEX_FILE` dropped | **red**, at `intended/.git/index` |

  It does not pass either way. `env-selector-coverage-k68` added the assertion
  that the colocated index was exported into the intended repository, and that
  assertion **presupposes** colocation — so a non-colocated fixture makes the
  test vacuous by turning it permanently red rather than permanently green.
  The conclusion the paragraph draws (colocation is what makes the green result
  evidence) survives; the sentence supporting it does not, and it is the kind of
  clause that reads as true only in the company of the one beside it.
- **`--colocate` is the default, and the page says nothing about why the flag is
  still there.** `jj git init --help` on 0.45.1: *"**This is the default**, and
  this option has no effect, unless the `git.colocate` config is set to
  `false`."* So passing it changes nothing for most readers — and it is exactly
  what defends the fixture against a developer whose own `git.colocate` is
  `false`. That makes it the **same kind of control as the pinned `JJ_CONFIG`**:
  a defence against the reader's configuration, not against jj's behaviour. The
  page presents colocation as a property of the fixture; neither it nor
  `crates/jj-workspace/tests/environment.rs`'s own comment states that a config
  could take it away or that the flag is what stops that.
  Verified: `jj git init --quiet .` in an empty directory leaves a real `.git`
  **directory** in the working tree.

## Done when

- The false clause is replaced by what a non-colocated fixture actually does,
  and the paragraph still reaches its conclusion — the replacement is a
  statement about the test as it stands, not a weaker restatement of the old
  one.
- Whatever `--colocate`'s real role is decided to be, it is stated once in the
  place that owns it, and the two places that mention colocation
  (`03-subprocess-seam.md` and the `colocated()` doc comment in
  `crates/jj-workspace/tests/environment.rs`) do not disagree.
- Chapter 2's own colocation sentence (*"on a colocated tree the walk answers
  alone, so those variables have nothing to act on and that half of the test
  would pass with the seam's scrub deleted"*, around line 99) is checked against
  the same measurement rather than assumed correct: it is about the **resolution
  half** and looks true, but it is the summary layer for the same fact and a
  finding against a section does not reach it on its own.
- `bash scripts/check.sh` passes, `book-check --final` is green over the
  `jj-workspace` book, and the crate's suite is unchanged at thirty-two tests.

## Notes

**No fragment or ledger is at risk, but confirm it rather than assume it.** The
sentences are prose around fragments, and `tests/environment.rs` is outside the
book's corpus (`docs/specs/jj-workspace-book-structure.md`). A source-comment
change in `crates/jj-workspace/tests/environment.rs` is likewise outside every
ledger. If the fix reaches a **source root** instead, the campaign's freeze rule
binds: one commit carries the source change, every affected ledger and page, and
a green validator run over every book it touched.

**This is the pilot's book.** If the correction turns out to need the paragraph
re-argued rather than one clause replaced, that is editorial work and the
campaign's answer to it is a contiguous run of stage leaves from the owning stage
through `proof` (`docs/adr/a-feedback-edge-is-forward-tree-growth.md`), not an
inline rewrite here. Decide which of the two this is before editing.
