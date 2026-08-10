# finish-transaction-handoff-hardening-k138

**Kind:** impl

## Goal

Keep a committed finish disposition exact and fail closed through atomic
quarantine handoff and best-effort cleanup.

## Context

- Consume validated, recoverable transaction state from the preceding
  integrity and transition leaves.
- Repository adapters own Git/native-jj/colocated-jj topology classification;
  this leaf owns how the transaction consumes and revalidates that proof.

## Done when

- Tests cover proof changes before and after handoff, quarantine collisions and
  rename failure, atomic restoration when disposition changes, tracked witness
  refusal, and unexpected repository topology without resurrecting old work.
- Successful handoff leaves either the complete in-tree witness or the complete
  quarantine; no intermediate task-root shape is runnable.
- Disposal unlinks without following symlinks, and immediate or later cleanup
  failure retains attempt-bound retry evidence without changing lifecycle
  classification.

## Notes

Reuse the existing `finish_cleanup` identity and retry seams; do not rebuild
their race machinery inside the transaction module.
