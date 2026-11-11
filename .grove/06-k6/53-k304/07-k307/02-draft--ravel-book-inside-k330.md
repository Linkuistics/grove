# ravel-book-inside-k330

## Goal

Draft chapter 8 of the Ravel book for 2.0.0, “Follow a session through the
code”. A reader follows a successful, a refused and a cancelled session
through the code that exists now.

## Context

- Read the node brief `_ravel-book-inside.md` and its pointers. The structure
  brief is Ravel `docs/book/contract.md`, sections 1 and 2.
- Chapter 7, as drafted by `ravel-book-inside-k329`, sets the words for the
  parts. Restate them in chapter 8 and use the same concrete example.
- The draft of 2026-11-11 found these stale claims in chapter 8:
  - “Evaluate and check the configuration script” names `evaluate_and_resolve`,
    `choice::judge` and `worker::evaluate`. The Rhai path in
    `crates/ravel/src/choice.rs` is `choose_prepared`, then
    `evaluate_and_resolve_rhai`, then `rhai_worker::front::evaluate`. Its
    checks are the closures `judge` (`rhai_context`) and `reply_judge`
    (`config::judge_result`).
  - The exchange table names `crates/ravel/worker/src/main.ts`, which does not
    exist. The worker is `crates/ravel/src/rhai_worker/role.rs`, chosen by
    `arg0` in `main`. The frames are in the module comment of
    `crates/ravel/src/rhai_worker/mod.rs`: `hello`, `evaluate`, `host`,
    `reply`, `context`, `delivered`, `selection` and `failure`.
  - The front, not the worker, reads the entry. The source travels in the
    `evaluate` frame. Host calls are answered on a Tokio current-thread
    runtime in `rhai_worker/serve.rs`.
  - “The worker supplies `undefined`” is TypeScript. Check what `select`
    receives without a context in Rhai.
  - The refusal section and its failure tables name `choice::judge`,
    `Validator::validate`, `choice::deliver` and `worker::expired`. Check each
    one against the source.
- Every excerpt is whole lines of a tracked file. Read `docs/book/excerpts.md`.
  The 12 current excerpts pass, but passing proves only that the lines exist.
  Check that each excerpt is still on the path that the text describes.

## Done when

- Chapter 8 follows the Rhai path from `main` to the result write, then the
  refusal and cancellation paths. Each claim is checked against the source.
- `task docs:excerpts`, `task docs:examples`, `task docs:ste` and
  `task docs:breaks` pass for chapter 8.
- The chapter follows each Read 1 pattern of contract section 2.
- The leaf lists figure needs for the art step in the parent node brief's
  `## Handed forward`, with `session-code-path` and `selection-cancellation`
  checked against the new text.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`. If a
heading changes, reconcile every inbound link in the book (L65). Chapter 9
links to “Finish the managed job” and “Choose the final outcome”.
