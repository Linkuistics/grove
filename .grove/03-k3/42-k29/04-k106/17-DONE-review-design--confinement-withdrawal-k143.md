# confinement-withdrawal-k143

**Reviews:** confinement-withdrawal-k142

## Goal

Adversarially review the confinement withdrawal design of
confinement-withdrawal-k142 before ravel-confinement-removal-k145 and
grove-confinement-removal-k146 implement it.

## Context

- The design commits of confinement-withdrawal-k142, in this repository and on
  the Ravel bookmark `skill-directory-delivery` in
  `Ravel.skill-directory-delivery`. Follow that repository's VCS discipline.
- The decision log of confinement-withdrawal-k142.

The human decided the withdrawal on 2026-11-03. The decision itself is
closed. Review how the design states it and what the design leaves out.

## Done when

- Each finding names its file, its line and a smallest useful correction.

## Notes

Look in particular for these failures:

- A dependent of confinement that the design does not list, in either
  repository. Examples are a test, a script, a book excerpt, a fixture, a
  floor case and an acceptance row.
- A guarantee that the design silently drops, such as a protection of the
  owner's configuration or of a skill snapshot.
- A remaining promise in either product that a sandbox denies an access.
- A refusal or a remedy for a removed flag that the design does not specify.

## Decisions (running log)

The human's withdrawal decision is closed. This review checks the guarantees,
the migration work and the remaining acceptance contract. Findings belong in
this task so that confinement-withdrawal-k144 can give each one a disposition.
The review changes no design or product file and runs no test, build, lint or
format command.

## Findings

### F1 — P2: Include the check script in the backend removal

**Location:** `docs/confinement-withdrawal-evidence.md:283`, the list of floor
scripts and tasks to remove.

The handoff removes the backend scripts, their tests and the tasks
`release:floor:controls` and `release:floor:qemu:controls`, but it omits
`scripts/check.sh`. That script passes the removed files directly to
ShellCheck at lines 109–114. It also invokes both removed tasks at lines 133
and 135. Following the handoff therefore leaves a required check that fails
on missing files and tasks, on both Ravel lanes. This is separate from a
failure of the code being checked.

**Smallest useful correction:** Add `scripts/check.sh` to the implementation
handoff. Remove its references to the deleted files and tasks, and retain
`release:floor:seccomp:controls`, which still checks the general QEMU helper.
Keep the dual-Ravel full-check obligation at line 266.

### F2 — P2: Preserve the directory-delivery part of Linux floor acceptance

**Location:** `docs/confinement-withdrawal-evidence.md:285`, which deletes
`grove-floor-directory-case.sh`, and line 303, which substitutes
`grove-installed-smoke.sh`.

The directory case contains required behavior beyond sandbox denials. It
reads the descriptor, the mandatory spine, a supporting resource and a kind
from retained snapshots outside the working directory. It checks snapshot
file and directory modes and successful output publication. The assertions
are in `scripts/grove-floor-directory-case.sh:133` and line 152.
The replacement `scripts/grove-installed-smoke.sh` writes no skill settings
and supplies no skill source. Its two runtime lanes exercise native delivery
only, so they do not replace those directory checks.

The edited delivery specification still requires the directory case on the
Linux floor at `docs/specs/codex-skill-provisioning.md:495`. It also links to
`linux-floor-native.md`, which this handoff deletes. The design therefore
removes the evidence for a retained criterion and leaves its normative
description inconsistent with the new route.

**Smallest useful correction:** Move the directory reads, mode checks and
publication check into the general floor route for the 2.0 archive. Drop the
confinement probes and their control. Update the delivery specification's
floor paragraph and link to name the retained route. Treat earlier backend
results as historical without retiring the directory-delivery requirement.

### F3 — P2: Make configuration write protection conditional

**Location in Ravel:** `docs/skill-discovery-design-evidence.md:621` in
`Ravel.skill-directory-delivery`, commit `3642a5d628fd`.

The table says that protection against configuration and settings writes
moves to the harness because both paths lie outside the working directory.
That separation is not guaranteed after this design removes the overlap
refusal. Ravel accepts an explicit `--config ./config.rhai`, as
`docs/specs/configuration-evaluation.md:103` states. A run from HOME also puts
the default settings under the working directory. These are writable under
the design's Codex `workspace-write` example. Configuration below `TMPDIR`
or an additional writable root has the same problem. Mutable sources in the
next table row can also lie inside a writable root.

The old `Confined::prepare_for` rejects writable grants that overlap the
configuration directory or settings (`crates/ravel/src/confinement.rs:193`).
The proposed removal deletes that check. The retained source-overlap check
only keeps protected files out of published skill content; it does not keep
the harness from writing those files. The table's reason thus suggests that
an old protection survives in cases where it does not.

**Smallest useful correction:** State that configuration, settings and source
write protection is conditional on the owner's harness sandbox excluding
those paths from every writable root, including temporary and added roots.
State that Ravel no longer enforces the separation. Give an overlapping
`--config` example in the disposition or acceptance guidance so that a
successful `workspace-write` launch is not credited as proof of this
protection. This correction does not require restoring Ravel confinement.

## Review evidence and limits

The inspected design subjects are Grove `d218f3d8f660` (with its preceding
ADR conversion `7a49d252`) and Ravel `3642a5d628fd` (with evidence conversion
`74885e37`). Both workspaces were clean at entry. The review read the producer
task, the root-to-leaf briefs, the changed contracts and decisions, and the
implementation handoffs. It followed the standalone request through
`execute` and `session_command::confined`, and inspected the existing
configuration-overlap check and the floor consumers.

Graph discovery used Tier 2 verification. Grove's coverage metadata reports
generation `2026-11-03T12:31:57Z`; the cited paths have matching metadata.
The reported partial range of `release-smoke-qemu.sh:219` and the trial
launcher's partial range at line 90 were read directly. Ravel's graph is the
older fast generation `2026-11-02T05:30:19Z`: its design documents are
excluded and its relevant source metadata has changed. The review read the
relevant current document passages and source functions directly instead of
treating that graph as current source evidence. Coverage metadata is a
best-effort signal, and this review makes no exhaustive repository-wide
absence claim.

No product code, design contract or other task was edited. No test, build,
lint, format or real-harness command ran. The implementation leaves own fixes
and verification. In particular, the proposed real Codex release-notes run
remains unmeasured, as the design already states.
