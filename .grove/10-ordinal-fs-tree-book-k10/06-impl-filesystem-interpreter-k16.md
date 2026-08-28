# filesystem-interpreter-k16

## Goal

Explain the public write guard and the single filesystem interpreter, including
locking, effect application, intermediate states, rollback, partial rollback,
and the error taxonomy.

## Context

- Inputs: `mutation-algebra-k15`, `book-system-k6`, and this subtree's brief.
- Primary source emphasis: `src/fs/mod.rs`, `src/fs/apply.rs`, `src/fs/lock.rs`,
  and `src/error.rs` as assigned by the design ledger.

## Done when

- Shared/exclusive locking, snapshot timing, and one-operation guard ownership
  establish the concurrency contract and its limits.
- The path from public mutation method to pure decision to plan validation to
  ordered application and `Report` is explicit.
- Name-to-path checks, rename/create/move effects, highest-first shifts, and
  promotion's non-atomic intermediate states are connected to their invariants.
- Forward failure, successful unwind, failed unwind, retry advice, and the
  difference between refusal and error are explained through a concrete trace.
- Assigned fragments tangle exactly and scoped source, Markdown/link, relevant
  model, fault-injection, and crate checks pass.

## Notes

Do not call the operation atomic. State exactly which intermediate filesystem
states can exist under the exclusive lock and what rollback promises after each
failure class.
