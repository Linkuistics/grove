# book-validation-diagnostic-fixtures-k111


## Goal

Refresh the diagnostic-contract fixtures that still describe the pre-rebase
94-line library root so the complete `book-validation` test suite exercises the
current fixed corpus.

## Context

- `cargo test -p book-validation` currently fails in
  `byte_mismatch_has_complete_source_and_path_evidence` and
  `an_invalid_root_suppresses_only_its_byte_cascade`.
- The failures are in `crates/book-validation/tests/diagnostic_contract.rs` and
  are independent of the CLI source-root refresh in `syllabus-cli-k109`.
- The fixed inventory now assigns `source-library` 103 lines; the affected
  synthetic ledgers and fragment bodies still declare 94.

## Done when

- Both diagnostic fixtures describe the current fixed `source-library` root
  without weakening their intended mismatch and suppression assertions.
- `cargo test -p book-validation` passes in full.

## Notes

Keep the tests focused on diagnostic behavior. Do not change production
validation semantics merely to preserve the obsolete fixture size.
