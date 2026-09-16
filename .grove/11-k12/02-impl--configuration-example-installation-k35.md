# configuration-example-installation-k35


## Goal
Implement and deliver `grove config examples`: safely install the validated
repository examples and self-contained README, then close the parent's complete
delivery and root acceptance boundary.



## Context
Depends on `validated-configuration-examples-k34`. Read the parent's full done
criteria and delivery spec; those remain authoritative. Reuse the exact example
bytes tested by `crates/grove/tests/config_examples.rs`, without a second policy
copy. Package the README as `CONFIGURATION.examples.md`.

## Done when
- All parent installer criteria pass: early dispatch outside a workspace with
  broken/absent policy and stale signal; fixed reported paths; full preflight;
  matching regular files untouched; conflict refusal before writes; exclusive
  creation; accurate partial-failure reports; no overwrite/force/ignore editing.
- Temporary-destination acceptance exercises success, idempotence, mixed
  conflicts, symlinks, directories, unreadable files, races and late failures
  through a meaningful I/O seam. Active/unrelated files retain their bytes.
- Packaged bytes equal every repository example and README. Preserve and rerun
  k34's reader/inspection/fake-launch acceptance against the delivered set.
- Help/usage/exit-code assertions, user-guide coverage, configuration/architecture/
  module/reference/release prose and affected source-exact books describe the
  implemented command. Reconcile the root acceptance matrix and remaining status
  notices, preserving genuine limitations. Run `bash scripts/check.sh`.
- After final build/tests, invoke the newly built human binary against the real
  requested `~/.config/grove/` without changing HOME or activating a sample.
  Compare each destination to repository bytes and record command/result here,
  never active-policy contents. Preserve conflicts and retain precise live work
  if delivery cannot complete; do not close the parent/root as delivered.

## Notes
This is a complete command-and-delivery increment, not an implementation followed
by a documentation catch-up. Read the Rust, CLI-design and walkthrough skills
for the surfaces it changes. Driver-owned finish remains outside this task.
