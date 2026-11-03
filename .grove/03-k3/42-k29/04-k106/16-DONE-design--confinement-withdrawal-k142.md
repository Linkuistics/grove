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

## Decisions (running log)

**A 2.0 front refuses the removed flags with a remedy.** `--confine` and
`--runtime-read` refuse as `confinement_withdrawn`, stage `cli`, exit 2. The
check runs after the command line parses and before every other check, so
nothing is read or launched. The remedy names the harness's own sandbox and
Ravel 1.1. A plain unknown-flag error was rejected, because it gives no
remedy. Acceptance without effect was rejected, because it would launch a
caller's request for a boundary without that boundary.

**`grove run --runtime-read` stays accepted and has no effect.** Grove
prints one notice and passes nothing to Ravel. Without confinement every file
that the owner can read is readable, so the flag loses no meaning. The
change breaks no command, so it fits the project's default minor release, and
the design asks the human no version question. Removal of the flag was
rejected, because it breaks existing commands and so needs a major release.

**Grove moves two environment guarantees into its own scrub.** Ravel's
confined environment kept only seven names, so a standalone harness received
no repository selector and no multiplexer variable. Grove now removes the
four repository selectors of `jj-workspace` and the multiplexer variables
from the standalone job. The removal of the multiplexer variables removes a
pointer, not a capability, and the specification states that limit.

**The design artifacts are edited now, and the book changes with the code.**
The Ravel decision record, the four Ravel specifications and the skill
discovery views state the 2.0.0 contract in this leaf. The book stays for
ravel-confinement-removal-k145, because `task docs:examples` runs each book
example against the built CLI and `task docs:excerpts` checks each quoted
line against the source. A book that stated the 2.0.0 refusal before the
code would fail both checks on the design commit. The design lists each book
change for k145.

**The skill delivery guarantees each have one disposition.** The source
overlap refusal and the read-only seal by file mode stay, because they govern
what Ravel publishes, not what a sandbox denies. The snapshot read grants and
the write protection of the configuration move to the harness sandbox. The
protected configuration reads, the writable-grant overlap, the backend
listing probe and the held identity of a granted tree are withdrawn.

**A configuration script's harness run loses Ravel's confinement too.** A
harness run is never confined by Ravel. The script chooses the sandbox of
each harness run through its arguments, such as `codex exec --sandbox
read-only` for an LLM call.

**The release-notes route runs Codex under `--sandbox workspace-write`.** It
uses the owner's own Codex home. It copies no sign-in file and sets no
certificate bundle, because both existed only for Ravel's sandbox.

**A nested standalone cancellation no longer ends at once.** A confined
managed process ran detached, and its group took KILL at once on
cancellation. After the removal, Ravel forwards the signal and sends KILL
after its kill-grace of 5 seconds. The standalone specification states that
a nested run can outlast a shorter grace of an outer supervisor.

**Four passages stay with the removal leaves.** The confined directory of
Ravel's `result-job-completion.md` and the row G9 of Grove's
`user-guide-coverage.md` lie in `pending` regions of the controlled-English
conversion. Grove's decision record `a-review-carries-its-creator-reference`
has ten checker findings. Each describes code or a user guide that changes
with the removal, so k145 and k146 convert and change them.

**The measurement instruments stay.** `profile-copy.py` and
`descriptor-probe.py` read the Ravel source of a recorded commit, so they
still reproduce the record's measurements. This follows the decision of
ravel-token-handoff-k141. The two confined trial cases go, because no build
after the removal can run them.

**The design commits.** Ravel commit `74885e37` converts the design evidence,
and Ravel commit `3642a5d628fdc9c53db2f46837ccf8f719242f04` holds the design.
The bookmark `skill-directory-delivery` points at the design commit, and
nobody pushed it. Grove's conversion of `harness-selection-is-owned-by-policy`
precedes this leaf's commit.
