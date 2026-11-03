# confinement-withdrawal-k144

**Integrates:** confinement-withdrawal-k143

## Goal

Triage the confinement withdrawal design review and apply the real findings
to the design before ravel-confinement-removal-k145 implements it.

## Context

- Review confinement-withdrawal-k143 and its committed findings.
- The design commits of confinement-withdrawal-k142, in this repository and on
  the Ravel bookmark `skill-directory-delivery`. Follow the VCS discipline of
  each repository.

The human's decision to withdraw confinement stays settled.

## Done when

- Each finding has a recorded disposition and its reason.
- The real findings are applied in the repository that owns each artifact.

## Decisions (running log)

**F1 is a real issue, and its class is wider than the finding.**
`scripts/check.sh` passes the removed scripts to ShellCheck at lines 109–114.
It runs `release:floor:controls` and `release:floor:qemu:controls` at lines
133 and 135. `release:floor:seccomp:controls` stays, because
`release-smoke.sh:449` uses the `--qemu` profile.

An enumeration of every reference to the removed floor names found more
unlisted consumers. They are `docs/RELEASING.md`,
`docs/ravel-target-inputs.md`, the links of `docs/linux-floor-qemu.md` and
`scripts/floor-seccomp/README.md`. The test
`every_repository_markdown_reference_resolves` reads every Markdown file. A
link to a removed document therefore fails `cargo nextest`. The correction
adds one list of these consumers to Grove's handoff.

**F2 is a real issue.** The directory case is the only check that publishes
skill snapshots from the release archive at the glibc 2.17 floor. Ravel's
release smoke publishes no snapshot. Snapshot publication needed the
`renameat2` repair of grove-linux-two-lane-k103 to link at that floor. Both
general routes run the cases as uid 1000, so file modes still refuse a
snapshot write.

The case therefore stays on the 2.0 lane of the general route. It loses its
confinement probes, its grants and its unconfined control. The earlier native
floor case needed the sandbox to make the native copies unreadable, so that
the agent force-stopped. `grove-installed-smoke.sh` still runs the standalone
caller on each lane, and `standalone.rs` checks the force-stop. The native
part therefore has no floor case. The delivery specification and the
dispositions table state both.

**No second design review.** The corrections repair lists and one floor
case. The `task release:smoke` check of the handoff measures the retained
case through an executable seam. A second review would repeat what that check
proves.

**F3 is a real issue.** `--config PATH` resolves against the working
directory, and a launch from HOME puts the configuration inside it. The old
`confinement_overlap` refusal at `crates/ravel/src/confinement.rs:193` goes
with the removal. The skill sources of this project live in the checkout that
is the working directory of each lifecycle session. Rows 621 and 622 of
Ravel's evidence table now state the write protection as conditional on the
harness sandbox, with an overlapping example. The decision record and the
skill discovery specification already state the condition, so they do not
change.
