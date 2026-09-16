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

## Decisions (running log)

- Follow the approved delivery design without reopening configuration grammar.
  Embed the seven authoritative files with `include_bytes!`; the human binary
  owns installation and dispatches it before workspace or active-policy setup.
- Preflight the complete fixed set, retain matching regular files, and create
  missing entries with exclusive opens. Keep completed and partial creations
  visible in failure reports; never roll back by deleting user paths. Exercise
  race and I/O failure branches through a filesystem seam.
- Implement in test-first increments, then update the complete affected overview
  book and command/reference coverage. Run the principal checks on fixed inputs,
  build the human binary, deliver to the actual personal directory, and compare
  bytes before retirement and the parent/root acceptance reconciliation.
- The graph generation is 2026-09-16T11:54:39Z and its CLI ranges are stale.
  Source reads, including the current dispatch and acceptance tests, are the
  evidence for this leaf rather than the obsolete graph call edges.
- A single fresh-context adversarial reviewer inspected installer preservation,
  preflight, exclusive creation, partial reports, early dispatch and installed
  fixture tests against the delivery contract. It returned no findings. The
  review used direct source because graph metadata is stale; it did not run tests.
- The initial process tests failed on the missing command, then passed after
  implementation. Focused checks now pass for filesystem conflicts/idempotence,
  injected race/partial-write/directory/read failures, CLI coverage and installed
  example reader/inspection/fake-launch behavior. The overview book is the sole
  affected source corpus; its recursive rule includes the new module and inline
  tests without corpus exceptions.
- The first principal-check attempt found two test-only Clippy errors (octal
  permission notation and a cloned reference in an assertion). Stopped that
  failed run before editing; its tracked-file digests were unchanged. Corrected
  both mechanically, mirrored the inline test in the book, and reran Clippy and
  final overview validation successfully. No production behavior changed.

## Verification and delivery

- `bash scripts/check.sh`: all 8 principal checks passed, including the locked
  workspace test suite and final validation of all 6 source-exact books. The
  overview reconstructs 6 roots and 974 lines with no deferred bytes. SHA-256
  manifests over every tracked regular file (source, tests, manifests, fixtures,
  docs, plugin inputs and scripts) matched before and after this complete run.
- `cargo build --locked -p grove --bin grove`: passed after the final tests.
- `./target/debug/grove config examples`: exited 0 using the existing HOME and
  ambient session environment; created all 7 files in
  `/Users/antony/.config/grove/`. No sample was activated. Every destination was
  compared byte-for-byte to its authoritative repository source, including
  README.md installed as CONFIGURATION.examples.md.
- A preservation snapshot covered all preexisting non-directory entries beneath
  the personal configuration directory plus existing workspace delta/ignore paths
  in this checkout and the main-repository delta. All 3 preexisting entries kept
  their contents, file identity, mode and modification time; no policy contents
  were printed or recorded. Exactly the 7 intended files were added.
- Repeated `./target/debug/grove config examples`: exited 0 and reported every
  path unchanged. A second preservation snapshot equalled the post-install
  snapshot, including the newly installed files.
- Parent acceptance is met: exact package and delivered-fixture execution,
  complete preflight and failure/race behavior, early CLI dispatch, command/help
  and guide coverage, final source-exact books, and real delivery are verified.
  Root acceptance is reconciled in the promoted handoff below its brief. The
  configuration ADRs already state the delivered authority/composition contract;
  no new decision or compatibility exception was introduced. Finish/teardown
  remains driver-owned.
