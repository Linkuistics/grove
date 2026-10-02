# two-orders-k20

## Goal

The `grove-llm` book states the two orders the command surface still holds:
text before lock, and admission before signalling. It stops stating the
presence rule as a third.

## Context

- `lifecycle-launch-k12` deleted `require_declared`, `inherited_kind` and the
  `SessionConfig` import from `crates/grove-llm/src/cli.rs`. No tree verb asks
  whether a kind can be launched (`docs/specs/harness-selection-and-execution.md`,
  *A refusal*).
- That leaf's first child rebased the book's fragments, rewrote
  `04-growing-the-tree.md` and its index entries, and left the book passing
  `book-check`. Everything else in the book still says three orders.
- `crates/grove-llm/tests/session_kind_presence.rs` is gone.
  `crates/grove-llm/tests/no_kind_admission.rs` holds what replaced it.

## Done when

- `01-orientation.md` carries a session with no Grove configuration: its import
  ledger, its worked trace and its verb table say two orders, and the
  `SessionConfig` early-use entry is gone from the manifest, the chapter and
  the source index.
- `07-what-order-holds.md` assembles two orders: the twelve-verb table has no
  presence column to fill, the orders table has no second row, and the
  observations, the boundary table and the recorded test run follow.
- `README.md`, `02-the-grammar.md`, `05-ending-work.md` and `concept-index.md`
  no longer name the presence rule as something the binary does.
- `docs/specs/grove-llm-book-structure.md` describes the book as it stands.
- `book-check` validates the book, and `bash scripts/check.sh` passes.

## Notes

- A sentence saying the rule existed and what replaced it is current state and
  can stay, as the growing chapter's opening has it.
- The chapters carry measured figures: test counts, mutation tables, a recorded
  test run. Re-measure one only where the text is being rewritten anyway, and
  say when a figure is carried over unmeasured.
- The source is not expected to change. If it does, rebase the fragments rather
  than patching ranges by hand.

## Decisions (running log)

**The first order's cost changed with the deletion, and the book says so.** The
old cost of reversing text-before-lock was a self-deadlock in `leaf-decompose`,
whose handler took a shared opening before the exclusive one. That read went
with the presence rule, so no handler opens the tree twice. What is left is
contention: a lock over the whole grove taken, and waited for, to refuse a
typo.

**No test holds the first order, and none was added.** The refusal tests
require an untouched tree, which holds whether the text is read before or after
the opening. The book says *no test does*, and records one measurement: with
another process holding the exclusive lock on a scratch grove's root, a
malformed argument to each of the six text-reading handlers was refused at
once, and well-formed `leaf-add` and `resolve` waited until killed. Whether a
test is worth cutting is put to the node's review.

**The twelve-verb table lost a column, not just its entries.** `complete`'s
channel check was the only thing left in *asked before the mutation*, so it
moved into that verb's opening cell.

**One word of source changed.** The manifest's comment on the removed `grove`
dependency said the leaf-writing verbs *make* a launch-template check. It now
says *made*. The line count is unchanged, so only that fragment's bytes moved.
Left as it was, `current-state-documents-k15`'s sweep would have had to rebase
this book for it.

**Two sentences of the growing chapter were repaired.** Its opening gave the
first order the presence rule's old cost, a tree changed by a refused command,
and its last section still spoke of `leaf-decompose`'s two openings.

**The structure specification follows the manifest.** Its ownership table, line
ranges and totals were regenerated from `walkthrough.toml`, and its thesis,
chapter 4 entry, worked-example row and early-use ledger say two orders. It no
longer says the book does not exist yet.

**Figures.** Measured here: 971 corpus lines and 898 in `cli.rs`; 426 comment
lines in the corpus, 389 of them in `cli.rs`; thirteen imported `grove_loop`
items and five reached by path; 233 integration tests in 24 files, 43 of them
in the four methodology files; 9,865 lines under `tests/`. The recorded
`cargo test -p grove-llm` run is this session's. Carried over unmeasured: the
line numbers chapter 2's opening gives for its own blocks, and every mutation
table and test count in chapters 2 to 6.

**Left as found.** The specification's *Outbound links* section still calls two
glossary anchors *must be created* and assigns the work to a leaf long retired.
The concept index still calls `finish-commit`'s handle lenient where chapter 6
and the manifest say canonical. Neither concerns the orders.

**The node gets a review, cut ahead of `grove-configuration-k13`.** The cutover
is the launch path of every session in a major release, and its commit removes
about 1,400 more test lines than it adds. `lifecycle-launch-k21` carries the
doubts. This session's in-session reviewer was not spent.
