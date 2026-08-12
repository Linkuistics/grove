# one-build-owns-a-session-integrate-k21

**Integrates:** one-build-owns-a-session-review-k20

## Goal

Apply the actionable findings from the adversarial implementation review of
`one-build-owns-a-session-k17`. Preserve the ADR's advisory boundary: pairing
diagnostics report and launch; they never become a gate.

## Context

The producer recorded `cargo test` (963 passing), `cargo clippy --all-targets`,
`cargo fmt --check`, and mutation checks. The review was inspection-only and ran
none of them. It checked the committed diff, current source, durable documents,
the release path, and the recorded evidence.

### F1 — P2 — Resolve relative `PATH` entries from the session's cwd

`src/loop_driver.rs:500-518` resolves an empty or relative `PATH` entry against
the driver's process cwd. Bare `grove` deliberately accepts an invocation from
any directory inside the working tree and retains that cwd while resolving the
root (`src/launch.rs:16-18`), but the configured session is spawned with the
worktree root as its cwd (`src/loop_driver.rs:231-232`). Therefore a session that
inherits `PATH` can still resolve a different `grove-llm` from the one the
driver probes. For example, from `<worktree>/subdir`, `PATH=:/usr/bin` makes the
probe inspect `<worktree>/subdir/grove-llm` while the session resolves
`<worktree>/grove-llm`; any other relative entry has the same defect.

This violates the exact case the docs claim is reliable (an inherited
environment), can print a false missing/mismatch report, and can execute an
unrelated repository-local helper during the advisory probe. Resolve and run
the probe relative to the configured session's worktree cwd. Add a black-box
test that launches bare `grove` from a nested directory with an empty or relative
`PATH` entry and proves the root-level binary is the one inspected. Keep the
report advisory.

### F2 — P2 — Make the diagnostic contract state only knowable values

The durable record promises that missing, unidentifiable, and mismatched cases
all name a resolved path and both identities
(`docs/adr/one-build-owns-a-session.md:36-40,70-78`; the release note repeats it
at `CHANGELOG.md:118-119`). The producer leaf repeated the same impossible Done
when. The implementation correctly cannot do that: `Pairing::Missing` has no
path or peer identity (`src/loop_driver.rs:434-436`), and
`Pairing::Unidentifiable` has a path but no peer identity
(`src/loop_driver.rs:438-440`). The missing diagnostic is nevertheless
actionable: it names the driver's `PATH`, this build's identity, and the exact
requirement.

Rework the ADR, architecture/spec/usage wording where necessary, and changelog
to promise the values available in each branch: missing = own identity and no
resolved path; unidentifiable = resolved path, own identity, and reason;
mismatch = resolved path and both identities. Do not fabricate a path or peer
identity and do not turn either case into a stop.

### F3 — P3 — Verify the invariant on the artifacts that ship

`tests/provision.rs:41-60` establishes that only `grove` carries the embed by
scanning `CARGO_BIN_EXE_*`, i.e. binaries from the profile and host target used
for that test run. The prescribed release checks run `cargo test --locked`
(`docs/RELEASING.md:45-52`), then `scripts/release-build.sh:50-68` independently
builds three release artifacts and packages them without asserting the marker.
The producer manually checked debug and release on one machine, but a later
release-only/target-specific linker or codegen setting can make a shipped
`grove-llm` retain `CONTENT` while the test-profile binary stays clean.

Keep the binary-artifact grain, but place a marker-presence/absence assertion on
each release pair before it is archived (or otherwise make the release path run
the same invariant against each artifact). Retain the ordinary integration test
as the fast local guard.

## Done when

- The pairing probe resolves relative and empty `PATH` entries from the
  worktree cwd used for the configured session, with regression coverage for a
  nested bare-`grove` invocation.
- Durable docs and the changelog describe the knowable diagnostic fields per
  branch, and the missing branch remains actionable and non-fatal.
- Every release artifact pair is checked so `grove` contains the methodology
  marker and `grove-llm` does not.
- Existing recorded behavior remains intact: stamp re-verification may still
  propagate a real provisioning refusal, agent-side warnings remain best-effort
  stderr before the epoch guard, identity parsing accepts formatting whitespace
  but rejects chatter, and the pair report never gates a launch.
- `cargo test`, `cargo clippy --all-targets`, and `cargo fmt --check` pass after
  integration.

## Notes

Review conclusions that do **not** require changes:

- A foreign unstamped skill directory appearing between loop iterations is a
  provisioning refusal, not a pairing stop. The error names the directory,
  preserves it, leaves the tree resumable, and is the same safety boundary as
  the start-of-run sweep.
- `warn_on_foreign_skill_dirs` widens every verb to `$HOME`, but deliberately
  swallows lookup/read failures and writes only stderr; no verb's stdout or exit
  status changes, including `complete`.
- The two hash traversals agree for the three shipped Unix targets: build-script
  and embedded paths use `/`, both sort paths and hash the same little-endian
  length-prefixed path/byte records, and the in-crate equality test catches
  drift on the build host.
- Trimming leading/trailing whitespace and accepting CRLF does not turn chatter
  into an identity; trailing comments/non-empty lines and stderr-only output
  remain unidentifiable.
