# configuration-examples-k12


## Goal

Deliver `grove config examples`, validate its packaged repository bytes through
the production reader and fake launches, and install that tested set into the
user's requested `~/.config/grove/` directory without replacing active policy
or conflicting files. Close the feature's complete acceptance boundary.



## Context

Depends on `configuration-inspection-k11`. The authoritative package is
`docs/examples/modular-configuration/`: six KDL examples and its README installed
as `CONFIGURATION.examples.md`. Read the current files; do not maintain a second
handwritten copy in Rust or tests. The spec's delivery section defines the
collision and partial-failure behavior.

The user asked for actual example files in the personal directory. Implementing
and testing the installer alone does not satisfy this leaf or the root. Use the
newly built human binary for delivery, not an older installed command.

## Done when

- `config examples` is documented in help and dispatches before workspace,
  active policy, epoch or lease setup. It succeeds outside any workspace with
  broken or absent active configuration and ignores a stale ambient signal.
  Destinations and filenames are fixed, separate from all active policy, and
  reported. No force option or ignore-file edit is introduced.
- Package the exact repository example bytes, including the self-contained
  README. Production-reader tests resolve the personal sample alone and with
  every local sample, checking both arrangements, effort changes, per-kind
  overrides/unset, legacy literal replacement, includes and local replacement.
  The inactive unfinished profile succeeds unselected and fails actionably when
  selected. Fake executables capture the promised exact argv without harnesses.
- Preflight the entire destination set before writing: matching regular files
  are untouched; different contents, symlinks, directories and unreadable entries
  are conflicts. Missing files are exclusively created so a race cannot replace
  a new occupant. A later create/write failure reports created paths and the
  failure accurately; do not claim atomic installation or delete preexisting
  contents as cleanup. Repeating a successful installation leaves matching files
  unchanged. Exit codes are 0 success, 1 failure/conflicts, 2 usage.
- Temporary-directory acceptance covers a mixed missing/conflicting set with
  no premature writes, matching files, symlink/directory refusal, exclusive-create
  races and late failures using a meaningful I/O seam where needed. Active
  `config.kdl`, workspace deltas and unrelated files retain their bytes.
- After the final build and tests, run that implementation against the actual
  requested personal directory and verify that every intended example is present
  with the repository bytes. Record the command and delivery result in this
  task's running log, with no active-policy contents. Do not change the user's
  HOME or activate a sample. If conflicts prevent completion, preserve them and
  report their paths; obtain user direction or leave precise live delivery work
  rather than retiring the root as delivered.
- The root acceptance matrix is accounted for by executed checks across this
  tree. Configuration/user/CLI/architecture/module docs, ADRs, glossary and
  release-facing descriptions describe the delivered contract. Remove obsolete
  implementation/delivery notices, preserving the genuine limitations: opaque
  commands, runtime placeholders, no cross-file transaction or live-child change.

## Verification and documentation

Use production Catalog/SessionConfig and fake executable acceptance on the exact
packaged fixtures, installer tests in temporary destinations and the final actual
directory byte comparison. Inspection of each valid example combination agrees
with its captured argv under the same context. Preserve `.cargo/config.toml`'s
meta-grove signal guard.

Update `docs/CONFIGURATION.md`, `docs/USAGE.md`, the modular spec's delivery status,
sample instructions where needed and all affected crate/source-exact books.
Extend `docs/specs/user-guide-coverage.md` for `config examples`, its invocation,
destinations, conflicts/partial failures and exit codes, with matching
`docs/USAGE.md` anchors and coverage-test updates in
`crates/grove/tests/user_guide_coverage.rs` as required. Update CLI surface/help
assertions for the newly working command in this leaf.
Ensure installed instructions contain no repository-relative links or transient
status text. Discover new modules through manifests rather than extending only
a remembered file list, and pair any corpus exception with its
`docs/specs/walkthrough-books.md` inventory row. Run `bash scripts/check.sh`
with final source/books; the complete feature must finish with all principal
checks green.

## Notes

No new documentation-only catch-up task should be necessary: earlier producers
own documentation at each landing. This leaf reconciles the final feature claim
and real delivery evidence. Driver-owned finish/teardown still belongs to the
later finish session, not to this installer or this leaf.
