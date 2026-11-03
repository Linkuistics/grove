# ravel-token-handoff-k139

**Integrates:** ravel-token-handoff-k138


## Goal

Triage the token handoff design review and apply the supported corrections to
Ravel's design before ravel-token-handoff-k133 implements it.



## Context

- Review ravel-token-handoff-k138 and its committed findings in the "Token
  handoff design review" section of
  `docs/common-loading-acceptance-record.md`.
- Ravel's reviewed artifact is commit
  `7c5b6cdd38427b21dac8a5307dadeac3cebace1a` on the unpushed bookmark
  `skill-directory-delivery`, in `Ravel.skill-directory-delivery`. Follow that
  repository's VCS discipline.
- The owner chose the minted-token/socket route in ravel-token-handoff-k132.
  That route remains settled.


## Done when

- Each review finding has an evidence-based disposition. Apply valid findings
  to the owning specifications, ADRs and views, and explain rejected findings.
- The repaired design is a coherent contract for implementation, with its
  verification limits explicit. Record the resulting Ravel commit in Grove's
  acceptance record so ravel-token-handoff-k133 can consume it.


## Notes

This task owns design integration, not implementation. Read no credential and
make no model call. The opt-in credential check remains with
ravel-token-handoff-k133 and needs the human's approval.


## Decisions (running log)

**Triage evidence.** The triage reads the findings from review commit
`511ae4e4b1d`. It checks each against Ravel commit `7c5b6cdd3842` and the
source of the same workspace, whose working copy is empty above that commit.
It reads no credential and runs no harness.

**F1 is a real issue.** `Confined::prepare` resolves the protected paths
before `choose_prepared` runs the selection (`crates/ravel/src/run.rs`). The
specification then checks and reads the file by its path, with links
admitted. Nothing binds the read to the protected path.

The repair: each delivery resolves the declared path again and requires the
protected real path. It opens that path without following a last link, checks
the open file and bounds the read. The protection also covers the declared
path itself, as `Protected::new` does for skill paths. A link count of one
closes a second name that a path check cannot see. The specification states
the limit that remains: processes outside confinement are trusted.

**F2 is a real issue.** `Protected::new` in `crates/ravel/src/skills/mod.rs`
lists the configuration directory, the settings file, the cache and the
inherited channel, and no token file. Capture copies the whole source. The
repair adds every declared token path to that set, for `skills resolve` and for
each launch, with or without confinement.

**F3 is a real issue.** `run_with_output` in
`crates/keyed-launch/src/run.rs` marks every descriptor from 3 up
close-on-exec in each detached mode, and the confined and captured modes are
detached. The repair states a runner-owned list of socket descriptors that stay
open, with ownership through the spawn and its failure.

**F4 is a real issue.** A harness run returns captured `stdout` to the worker
(`rhai_worker/harness.rs`), and managed output goes to the caller's stderr.
The repair limits the promise to what Ravel itself adds. The fixture reports a
digest, not the bytes.

**F5 is a real issue.** The profile denies by default, and grants decide
reads. A runtime read of a sign-in file is readable, as the Codex route uses.
The repair limits the claims to what this feature adds, in the specification
and in the ADR. The same scope applies to the sentence about rotation of the
owner's sign-in, which depends on the same grants.

**The other doubts.** The review's dispositions stand. The refusal table now
names both overlap codes, which the review showed to differ. The socket
bullets follow the review's static reading: Claude Code closes the descriptor
after its read, so a later descendant has none.

**The read sequence.** Each delivery runs nine ordered steps. It checks the
expiry, then resolves the declared path again, which must give the protected
real path. It opens that path with no link in the last component and no
block. It checks type, owner, mode, one link and size on the open descriptor.
Then it reads at most 4,097 bytes.

A harness run has its own protection, because each run calls
`Confined::prepare_for`. The interval between the new resolution and the open
stays open, but only a process outside confinement can use it. An ordinary
rename of the token file keeps its real path and needs no refusal.

**Limits stated, not closed.** Protection works by path. A second hard link
is outside it, so the read refuses a link count above one, and the owner keeps
one name. An intermediate symbolic link between the declared path and the
real path is not protected. The owner keeps such links outside every grant.
A wider rule, such as a refusal of every link or a link-count check at the
protection, would change the contract beyond the findings.

**Diagrams.** `launch.mmd` gains the runner as a participant and the new
order. `route.mmd` and `reader.mmd` follow the closed descriptor and the
source protection. The manifest and the views README describe the change.

**One narrow reviewer.** This session spends its one reviewer on the
protection, read and runner sections, against the current code, with the
threat model and without this log.

**Reviewer triage.** The reviewer returned twelve findings. Each is checked
against the source, and two are measured with dummy files under
`scripts/harness-trials/profile-copy.py` and the profile of
`crates/keyed-launch/src/confinement.rs`:

- R1, a grant resolved again at spawn, is a real issue that predates this
  feature. `command_with_grants` canonicalises each writable root and runtime
  read again, after the check and the selection. It also exposes the settings
  file today. The repair is a confinement rule in `session-execution.md`. The
  runner grants the canonical form that Ravel checked. It refuses a path that
  now resolves elsewhere, as for a changed skill directory.
- R2, a hard link made by a confined process, is measured. Under the profile
  copy, `ln` from a dummy outside the grants fails with "Operation not
  permitted", from the real path and from the `/System/Volumes/Data` alias.
  The specification states the measurement and adds a fixture case.
- R3, a second link inside a skill source, is a real issue. The protection
  now refuses a declared file with more than one link, in both checks. This
  replaces the earlier decision to leave the link count to the read.
- R4, the read order of Claude Code, is a visible verification limit. Hooks or
  MCP servers that Claude Code launches before the read would inherit the
  unread socket. No
  run measured the order.
- R5, a dangling declared link, is a real gap. Its real path becomes the
  prospective path of the link's target.
- R6, the volume alias, is measured. The alias read fails under the profile
  copy, for a scratch file and for a file under `/Users`. A fixture case
  keeps it.
- R7, the program and executable reads, is a visible trade-off. The trusted
  script selects the program, and the claim now names both exceptions.
- R8, the ambiguity of step 2, is a contract stated unclearly. Each launch
  resolves the protected paths once, and every check uses that resolution.
- R9, R10, R11 and R12 are contracts stated unclearly, with wording repairs.

**A second review.** R1, R3, R5 and R8 are substantive repairs after the one
in-session review. A `review-design` leaf therefore goes before
ravel-token-handoff-k133, with the specific doubts in its body.

**Result.** The integrated design is Ravel commit
`9084e0dcfcd733a1185ca1bb26a312b53fb25549`, with the bookmark
`skill-directory-delivery` moved to it and still unpushed. Grove's acceptance
record holds each disposition, the probe results and the handoff to
ravel-token-handoff-k133. ravel-token-handoff-k140 reviews the commit first.
A separate commit converts the review's record section to controlled English,
with no change of meaning.
