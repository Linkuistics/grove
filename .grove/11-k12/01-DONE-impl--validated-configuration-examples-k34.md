# validated-configuration-examples-k34


## Goal
Validate the exact repository configuration examples through the production
reader, inspection and fake-executable seams, before they become installed bytes.



## Context
Use `docs/examples/modular-configuration/` directly, without copied policy strings.
`crates/grove/tests/config_show.rs` demonstrates SessionConfig and JSON-to-argv
acceptance. The parent owns the complete feature contract.

## Done when
- The personal sample alone and with every local sample resolves and launches
  the documented lead/review/proof commands with exact argument boundaries.
- Inspection agrees with captured argv under a fixed runtime context. Includes,
  replacement selection, shared effort, per-kind values/unset and legacy literal
  replacement are observable; removing a variation restores the inherited value.
- Unfinished policy succeeds inactive; selecting it yields actionable diagnostics
  and a real driver refusal without launching a fake executable or changing work.
- The reference/spec distinguish validated examples from the still-pending
  installer and actual delivery. The self-contained installed README stays free
  of temporary status prose. Focused tests and `bash scripts/check.sh` pass.

## Notes
No production API or command is added here. The next leaf owns the installer,
packaging byte equality, CLI docs/books, actual personal-directory delivery and
root reconciliation; this child must not claim that delivery has happened.

## Decisions (running log)

Use a new grove integration test that reads the repository examples unchanged,
loads them through SessionConfig and inspects them through the human binary.
Resolve illustrative executable names through temporary PATH entries so fake
launches exercise the exact templates, including embedded effort words. Compare
against literal documented argv as well as filled inspection records; agreement
between two resolver consumers alone would miss a shared semantic defect.

The new tests pass against all six unchanged KDL files. A deliberate removal
of `high-effort` from its workspace selection made the table fail on that exact
case; restoring the file restored green. The executable assertions are the
chosen doubt instrument for this fixture-validation boundary rather than a
second reader of the same examples. No resolver or runtime behavior changed.
The graph's 2026-09-16 generation is stale for SessionConfig and omits newer
integration tests, so current source reads supplied the evidence at that seam.

No source-exact corpus changes are needed: the only Rust addition is under
`crates/grove/tests/`, outside the books' manifest/production-source corpora.
No corpus add/exclude entry or crate manifest changed. The reference/spec now
distinguish validated repository examples from pending packaging and delivery;
the installed README and KDL bytes remain unchanged.

The first principal run passed seven checks, including all six books, but
workspace tests stopped on the existing TUI test
`witnessed_launch_identity_change_is_compared_even_when_tree_capture_fails`:
its shared fixture's initial Running assertion failed at observation.rs:250.
That production/test file is unchanged. An immediate `cargo test -p grove-tui
--lib` passed all seven tests; no fix or root cause is claimed. Before/after
SHA-256 lists matched for all 1,816 discovered repository files (including
manifests, sources, fixtures, scripts and docs; VCS/build directories excluded).
A complete principal rerun will verify the final state rather than treating the
isolated retry as a green workspace suite.

Final verification: `cargo test -p grove --test config_examples` passes all three
acceptance tests. The complete second `bash scripts/check.sh` run exits 0 with
all eight principal checks and all six final book validations green, including
the previously failing TUI test. Its before/after repository SHA-256 lists are
identical. Comparing named test results confirms every test reported by the
first run is present and passing in the second; the second also completes the
workspace tests the first failure prevented from running. No source edits were
made between these runs. The initial intermittent TUI failure is recorded above
rather than attributed to a fix.

The child is complete. `configuration-example-installation-k35` remains live,
so neither configuration-examples-k12 nor the root closes here. No files have
been delivered to the real personal directory in this child.
