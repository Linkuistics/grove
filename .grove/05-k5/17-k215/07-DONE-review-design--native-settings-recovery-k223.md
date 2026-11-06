# native-settings-recovery-k223

**Reviews:** native-settings-recovery-k221, hooks-installation-writes-k222

## Goal

Read the redesigned preludes of the owner switch adversarially before
`homebrew-delivery-plan-k217` cuts implementation leaves from them. The
design is the work of both producers together. Produce findings, and no
fixes.

## Context

- Grove: the commit of `native-settings-recovery-k221` and the commit of
  `hooks-installation-writes-k222`. Each commit message names its handle. The
  running logs of both task files hold the options, the decisions and the
  model results.
- The amended documents: `docs/specs/owner-skill-delivery.md`,
  `docs/models/owner-switch.qnt`, `docs/models/run-owner-switch.sh`,
  `docs/owner-skill-delivery-evidence.md` and the owner views in
  `docs/design/harness-selection-and-execution/`.
- The cases came from findings F1 and F2 of
  `homebrew-delivery-design-k219`. `homebrew-delivery-design-k220` judged both
  real and gave them to k221.
- The owner design runs its switch against the real HOME inside finish.

## Done when

- Each finding names the document and section, the defect and its
  consequence. A finding states whether it concerns k221's work, k222's or
  both.
- If a finding is worth acting on, an `integrate-review-design` leaf is cut
  as the review's last act. Its body names this review's handle.

## Notes

The producers' specific doubts. Read each against the artifacts rather than
taking it as established:

- **The gate.** Step 3 runs the preludes only while the latest complete
  backup's run is not open. Look for a layout or run record where that
  condition is wrong. One example is an incomplete backup kept aside. Another
  is a run closed by "inverse ended" with a third value left. A third is a
  prepared plan that differs from an open run's plan.
- **The settings record.** Check that each later run finds it, also before
  the first backup. Step 8, step 13 and `mac-check` must find it too. Check
  its order against the exchange, its interruption cases and its purge.
- **The accepted case.** Absence is now an earlier value of a prelude cell,
  so a prelude replaces an erased `skills` key or an uninstalled plugin.
  Judge whether that case is bounded and reported as the design claims.
- **The resumption of the hooks installation.** It disables an enabled plugin
  while no run is open. Check the claim that no other route reaches the
  native state. Check also that the program reads the plugin again before the
  disabling. k222 made the disabling an exchange after a settings record.
- **The model.** One `enter` stands for steps 1 and 2, and a stop at `ready`
  stands for any stop in steps 4 to 8. Both prelude cells are absent at first
  in every world. Property 6 uses the writer of a cell to tell a prelude's
  own value from another program's. Judge whether these abstractions hide a
  case, and whether each focused world and mutant isolates its mechanism.
- **Claude Code's own writes.** Read k222's decision against its measurement
  and against "Concurrent writers". k222's specific doubts:
  - A held link keeps only the inode at the path when the program takes it.
    A command that renamed twice would lose a write in place between its two
    renames. The evidence measured one rename per command, and only a
    rehearsal seam rechecks it. Judge whether the program must check it too.
  - The accepted case leans on the finish session's own process. Its agent
    waits on the program, and finish tells the owner to change no setting.
    Judge whether that evidence and the bound of two intervals of 30 to 38
    ms are enough, and whether another kept process can rename the file.
  - In the model, one command stands for the registration and the
    installation. The program takes the held link and decides in one step,
    and an outside rename hits only part `HOOKS`. Judge whether these
    abstractions hide a loss.
  - A write in place before a command's read flags the held copy although
    nothing is lost. Judge whether that report blocks progress more than it
    should.
- **One owner per rule.** The coordinated release design's recovery row says
  that the third step of the switch finds the state. Look for a sentence in
  any document that still runs the native settings outside step 3.

## Findings

Reviewed Grove commits `2d1df7c400ad` (k221) and `2c17bc993d19` (k222)
together, against the requirements and the current documents. Locations
below refer to `2c17bc993d19`. This is an inspection-only review. No test,
build, lint, format or model command ran, and no design, model, production
code or test code was changed.

### F1 — P1: metadata added before the held-link digest is silently dropped

**Producer:** k222. **Location:** `docs/specs/owner-skill-delivery.md`,
"Claude Code's own writes", lines 735–749; compare the metadata observation
at lines 732–733 and the control at lines 1548–1551.

The command's metadata check precedes the held link and its recorded digest.
Consider a file with no ACL or extra extended attribute. Item 1 passes.
Another program then adds an extended attribute in place, before item 2
takes the link and records the inode's digest. The command drops that
attribute, as the recorded measurement establishes. The held copy still
equals its recorded digest, so item 6 flags nothing. Later checks also find
no difference. The outside write is absent from the live file and has no
report, although metadata is expressly part of the detection contract.

This case needs no outside rename, and it is neither accepted case of
"Concurrent writers". The held bytes preserve the attribute, but an
unflagged kept copy is eligible for purge. The existing control supplies
the attribute before the installation starts, so it does not exercise this
ordering. The model explicitly leaves metadata to the rehearsal
(`docs/models/owner-switch.qnt`, lines 144–151).

Validate the metadata of the same held inode and recorded content that
authorise the command. A metadata write after that recorded observation
must then flag the held copy. Add a rehearsal control that inserts an ACL
or extended attribute between the current metadata check and the held-link
digest, and require refusal or a reported kept copy. Do not admit an
unreported loss merely because the pre-command digest already includes it.

### F2 — P2: the accepted-loss bound does not cover retries or scheduling delays

**Producer:** k222. **Location:** `docs/specs/owner-skill-delivery.md`,
"Claude Code's own writes", lines 770–780, and the rehearsal condition at
lines 1557–1561. Supporting evidence is
`docs/homebrew-skill-delivery-evidence.md`, "The interval between the read
and the rename", lines 236–268.

The design accepts silent loss within a stated bound of two intervals of
30 to 38 ms and says that a later run has none. Its own interruption paths
contradict the latter claim. Stop after registration but before installation:
the plugin is still absent, and the next run must execute the installation
command. Stop before that command's rename, and another retry can execute
it again. No durable attempt limit bounds the total number of exposed
intervals. An accepted outside uninstallation can also make a later run
install the plugin again.

The duration is an observation from an idle machine, not an upper bound
that the program enforces. A command paused after its settings read and
before its rename has the same loss mechanism for the whole pause. Rerunning
the measurement during rehearsal cannot bound the scheduling of the later
real-HOME command. The model deliberately abstracts both duration and the
number of commands, so its sampled results establish neither limit.

State the accepted case with its actual retry semantics and separate the
measured durations from a guaranteed bound. If a fixed bound is necessary
to accept the risk, provide a mechanism that enforces it; otherwise have
the acceptance explicitly cover an unbounded delay and repeated attempts.
Include interruption after registration and before a command's rename in
the evidence used for that decision. The existing staging observation of
kept processes remains useful, but supplies no timing or retry guarantee.

## Review evidence and limits

The inspected gate skips both preludes for open runs, including Active and
Complete continuation. The settings record precedes its exchange, survives
before the first backup and participates in later reporting and purge. The
inspected recovery rules also leave an incomplete backup aside, select the
open backup's plan and defer a different prepared plan until the inverse.
These source observations support no further finding in those mechanisms.
The coordinated-release recovery row directs the caller through switch
step 3, and the current owner views show that gate.

The held link has a concrete purpose: it retains writes through the earlier
inode. The extra report for an in-place write before the command's read is
an explicit trade-off, not a finding. Registration and installation each
need their own held link. The single-command model is useful for that
mechanism but does not establish their combined exposure bound. The
rehearsal must still verify the actual command's one-rename behaviour and
the writes of the signed-in finish process. Existing evidence does not
establish which user-settings writes that process can make without a user
action; the design's staging seam must resolve that uncertainty before use.

The SHA-256 digests of the model, its runner and `Taskfile.yml` match the
k222 run of record. The four measurement scripts also match their recorded
digests. Their reported results are producer evidence, not checks rerun by
this review. No model property is claimed proved.

Graph evidence used Tier 2 with project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-06T08:42:26Z`. Both pages of the bounded measurement-source search
were read. Exact snippets and direct source supplied the evidence; the
graph's unrelated cross-language call edges were not used. Coverage metadata
matched the checked documents and measurement scripts. The Quint file had
no tracked freshness, and `owner-cutover.mmd` had a recorded parse gap at
lines 1–70; both were read directly. Coverage remains a best-effort signal.
