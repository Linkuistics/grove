# admission-fixtures-k8


## Goal
Migrate grove-llm admission/shared helpers and grove-tui witnessed fixtures to
modular form; finish the downstream consumer inventory.



## Context
Inspect `crates/grove-llm/tests/` including support, session_kind_tree, leaf and
removed_surface helpers, plus `crates/grove-tui/`. Earlier children own grove-loop
and grove CLI. Enumerate any other consumers beyond these starting points.

## Done when
Retained fixtures are modular with unchanged admission and execution assertions;
compatibility-specific cases are identified for k3; downstream tests and
`bash scripts/check.sh` pass. Reconcile source-exact walkthroughs if touched.

## Notes
Preserve fake executables. Local files cannot define commands; personal policy
must admit every kind. Escape literal dollars according to the named scanner.

## Decisions (running log)

- Convert the shared `testing/support.rs` admission policy and the independent
  presence/removed-surface helpers to one modular command and binding with
  explicit routes for their existing kind lists. Keep fake executables and argv
  assertions unchanged.
- In `session_kind_tree`, put base authority inside the modular wrapper and
  express local target overrides as routes to the personal binding. The
  missing personal target and inactive personal authority must still fail for
  their original semantic reasons, not for unsupported syntax.
- TUI witnessed fixtures need only modular declarations around their existing
  harness commands; no production behavior or new testing seam is needed.
- The one in-session reviewer found no issues in the fixture diff, including
  the missing-target and inactive admission cases. Existing assertions and
  command templates remain intact.

## Consumer inventory

Enumerated `.kdl` paths and Catalog/Templates/SessionConfig consumers across
repository Rust, shell and KDL files, including hidden paths, then inspected
fixture writers and their templates. The additional shared writer is
`testing/support.rs`; `root_init.rs` consumes it, and `leaf.rs` already uses
modular policy. Driver-lease inline fixtures in `driver_lease.rs`, `observation.rs`
and `witnesses.rs` were migrated by k6. The excluded graph subtree
`crates/ordinal-fs-tree/bin` was checked directly and has no configuration-loader
consumer. Graph coverage is best effort; stale driver-lease metadata and excluded
scripts/docs were supplemented by source reads.

No additional retained executable consumer fixtures were found outside the
completed runner/loop/CLI/admission batches. The remaining cases are the k4
runner compatibility inventory and k6/k7 handoffs, plus the packaged legacy
example and documentation owned by k3. In particular,
`scripts/release-publish.sh` contains a commented smoke-test recipe generating
flat policy; include that recipe in k3's current-documentation sweep.

The literal flat-impl search was checked against the known remaining config-show
compatibility cases and packaged legacy sample; the modular-route search found
the changed consumers. These are cross-checks of the writer inventory, not its
sole evidence. All six walkthrough manifests cover production source rather
than the changed integration tests or `testing/support.rs`.

## Validation

- `cargo test --locked -p grove-llm -p grove-tui` passed.
- `bash scripts/check.sh` passed all eight principal checks, including the full
  workspace test run and all six final walkthrough validations.
- SHA-256 manifests of every tracked non-`.grove/` file matched before and
  after the full check. Subjects include code, tests, fixtures, manifests,
  scripts, skills and documentation.
