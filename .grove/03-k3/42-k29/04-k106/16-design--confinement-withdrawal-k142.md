# confinement-withdrawal-k142

## Goal

Design the removal of OS confinement from Ravel 2.0.0 and from Grove's
standalone caller. After the removal, the harness's own sandbox is the only
boundary of a managed process.

## Context

The human decided this on 2026-11-03, in stage-evidence-reconciliation-k125.
Ravel removes `--confine` and its backends before the 2.0.0 cut, because a
removal after the cut needs 3.0.0. Grove stops requesting confinement. The
decision follows the human's choice in ravel-token-handoff-k141 to rely on the
harness sandboxes. The decision is settled. Design how each product states it.

The evidence behind the decision:

- Claude Code 2.1.317 has no sign-in under Ravel's macOS profile, because the
  profile denies the Keychain.
- Codex completes a confined run only through a wrapper that turns its own
  sandbox off with `--sandbox danger-full-access`.
- The "Confined harness repair" and "Token handoff withdrawal" sections of
  `docs/common-loading-acceptance-record.md` hold the measurements.

Ravel, in the `Ravel.skill-directory-delivery` workspace on the unpushed
bookmark `skill-directory-delivery`. Follow that repository's VCS discipline.

- `crates/ravel/src/confinement.rs`, `crates/keyed-launch/src/confinement.rs`
  and their callers, such as `cli.rs`, `run.rs`, `skills/launch.rs` and
  `rhai_worker/harness.rs`.
- `docs/specs/session-execution.md`, `skill-discovery.md`,
  `configuration-evaluation.md` and `result-job-completion.md`, and the
  decision record `configuration-evaluation-precedes-the-launch`.
- The book chapters that name confinement, such as
  `09-jobs-signals-and-the-terminal.md`, with the two limits that commit
  `d16976730248` added.
- The Linux test environments and the native-entry Linux campaign evidence.

Grove, in this workspace:

- `crates/grove-loop/src/session_command.rs`, `crates/grove/src/standalone.rs`
  and `crates/grove/src/cli.rs`.
- `docs/specs/standalone-invocations.md`,
  `docs/specs/harness-selection-and-execution.md`, the delivery specification
  `docs/specs/codex-skill-provisioning.md` and
  `docs/specs/coordinated-release.md`.
- `docs/USAGE.md`, `docs/RELEASING.md`, `docs/linux-floor-native.md`,
  `docs/linux-floor-qemu.md` and `docs/linux-floor-backend.md`.
- `scripts/release-notes/codex-headless.sh`, the floor and release smoke
  scripts, and the harness trial runner.

## Done when

- Ravel's specifications, decision records and book state that Ravel 2.0.0
  has no confinement. They state what a 2.0 front does with `--confine` and
  `--runtime-read`, and the remedy that it gives.
- The design states what replaces each confined guarantee of skill delivery.
  Examples are the snapshot read grants, the read-only seal by file mode and
  the protected configuration reads. Each guarantee is kept, moved to the
  harness or withdrawn, with its reason.
- The design states whether a configuration script's harness run loses its
  confinement too.
- Grove's specifications state the standalone contract without confinement.
  They keep the private staged directory and the export of declared outputs.
  They state what `grove run` does with `--runtime-read`.
- The design states who configures the harness sandbox of a standalone kind.
  It states the release-notes route with Codex's own sandbox.
- The design lists each test, script, floor case and acceptance row to remove
  or change in each repository. It states which Linux floor evidence stays
  for the glibc link and which goes with bubblewrap.
- The acceptance criteria that name confinement have a stated disposition.
  These are in the delivery specification, Ravel's acceptance table and the
  briefs of common-loading-acceptance-k29 and acceptance-handoff-k106.
- The design states the version consequence for Grove of removing the
  `--runtime-read` flag. If the consequence is a major release, the design
  asks the human, because the project's default is a minor release.

## Notes

This is a design leaf. Write no product code. confinement-withdrawal-k143
reviews the design before ravel-confinement-removal-k145 and
grove-confinement-removal-k146 implement it.

Grove's admission accepts Ravel 1.1.0, and 1.1.0 keeps `--confine`. A
standalone request without `--confine` works on both majors, so state that
Grove's request is the same for both.

The live driver's ending executable is the 1.1.0 keg. Do not change it.
