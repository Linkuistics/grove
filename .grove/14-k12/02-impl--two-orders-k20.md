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
