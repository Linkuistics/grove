# grove-run-job-k8

## Goal

Move `grove run` to the shared function in its detached shape. This is the
first caller to migrate. It is the smaller one, and it needs no lease.

## Context

- The function of `ravel-job-k7`. Read its running log for the chosen names.
- `crates/grove/src/standalone.rs` is the caller. It uses
  `run_noninteractive_with_stdout`, `End`, `Escalation`, `Group`,
  `take_interrupt` and `regular_file_at` of `keyed-launch`.
- The root brief: *Behaviour that stays* (the `grove run` item), *Residues*
  and *Tests* (the standalone case under a pseudo-terminal).
- `docs/specs/standalone-invocations.md` and the section *Standalone
  interpretation* of the caller contract.
- The book `overview` has the whole of `crates/grove/src/**/*.rs` as its
  corpus.

## Done when

- `grove run` starts `ravel` through the shared function in the detached shape.
  It cancels on Ctrl-C, TERM or HUP. It publishes its outputs only after a
  successful Ravel status with `grove-continue`. Its transcript relay works as
  it does now.
- The survivor guard and the check of the group are gone. A failed run exits 1.
- The copy of the scrub list in `standalone.rs` is gone. `grove run` and the
  version probe of `dispatch/ravel.rs` use the list and the helper of the new
  module.
- `regular_file_at` is a private function of the standalone module. The
  observer is a no-op there.
- `crates/grove/Cargo.toml` no longer depends on `keyed-launch`.
- A new standalone case runs under a pseudo-terminal. A Ctrl-C typed at
  `grove run` cancels it and publishes nothing. The foreground stays with the
  group of `grove run`.
- Chapters 01, 05 and 06 of the `overview` book match the changed code and the
  changed `Cargo.toml`. The conversions of those chapters land first as their
  own changes.
- `bash scripts/check.sh` passes. These tasks pass with `RAVEL` set:
  `test:grove`, `ravel:activation` and `test:authority`.

## Notes

- The loop driver still uses `keyed-launch`. Two latches exist in the source
  tree until `lifecycle-job-k9`. They never run in one process, because
  `grove run` and `grove do` are separate invocations.
- Remove the conversion of `Argv` at the call site of `grove run` that
  `ravel-job-k7` left.
