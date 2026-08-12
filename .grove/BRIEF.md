# grove.grove-loop-should-detect-vcs-not-the-llm — brief

## Goal

Have the loop driver **state** the working tree's VCS to every session it
launches, so no session ever detects it.

The fact is deterministic and the driver already owns it: `repo::vcs_of` resolves
jj-first from the closest marker walking up, and every tree-mutation verb already
branches on the result. Only the *session* re-derives it — and re-derives it
badly:

- **The harness banner lies.** It is computed from `.git` alone and cannot see
  jj ([claude-code#41435](https://github.com/anthropics/claude-code/issues/41435)).
  The requirements session that produced this brief ran in a native jj workspace
  under a banner reading `Is a git repository: false`.
- **Detection is instruction-driven, so it is skippable.** A session that never
  loads `using-jujutsu` commits with git in a jj tree, bypassing the operation
  log.
- **It costs context, every task, forever** — to learn something the driver knew
  before the session existed.

## Done when

Every session launched by bare `grove` receives, in its mandate, the VCS the
driver resolved and the root it resolved for — and is told not to probe. Proven
at the driver seam over both lanes.

## Decomposition

- `mandate-states-vcs-k2` — the grove context: driver, tests, and the two durable
  records.
- `probe-carve-out-k3` — the skills context: reconcile `using-jujutsu`'s "First
  action: probe the repo" with a mandate that says not to.

Split by **bounded context**, not by artifact type, so each commit has one owner
(`CONTEXT-MAP.md`, *A durable record has one owner*).

## Pointers

- `src/loop_driver.rs:195` `mandate_prompt` — the seam. Already appends the
  authoritative handle to the embedded launcher; the VCS line joins it there.
- `src/repo.rs:102` `vcs_of` — the named authority. Two variants only, never
  three: grove cannot run without a marker, because the driver lease lives in the
  VCS-administration directory. So the mandate never says "no VCS".
- `Vcs::Git` is a unit variant carrying no root, but the driver already holds
  `worktree` (canonicalized from the marker), and for jj that *is* the workspace
  root. The driver needs the discriminant, not a new type.
- `tests/loop_driver.rs:224` — the mandate-capture pattern: the real driver, a
  configured command that appends `$1` to a file, the text parsed back out.
- `tests/jj_tree_verbs.rs:42,66` — `jj_native` and `colocated` fixtures. `jj` is
  already a hard requirement of the suite.
- `docs/ARCHITECTURE.md` — *Version-control seam* (`#symmetric-vcs-rule`).
- `CONTEXT.md` — **Kind routing** describes the mandate today; the new term is
  its sibling.

## Notes

**Settled by grilling in `plan-k1`; treat as decided, not as open scope.**

- **Rejected mechanisms.** A `${vcs}` template word (staleness: every existing
  config silently gains nothing, and grove cannot tell whether the target used
  it). A `grove-llm vcs` verb (still a probe, still a turn, still skippable —
  fails the save-context test outright). `grove-llm` owning the commit boundary
  (strongest reading of "not the LLM", but it inherits scope, staging and message
  policy for a commit that covers artifacts, not just `.grove/` — far beyond
  "detect").
- **Line content.** Identity, the resolved root, and an explicit
  do-not-probe / disregard-the-banner instruction. Deliberately **not** the marker
  kind, and **not** the commit-boundary commands — those live in the skill's
  Commit step, and copying them into a driver-computed prompt creates a second
  source of truth that drifts across the build boundary.
- **No ADR.** The when-to-write test does not clear: this is not a real
  trade-off, it is the driver telling the truth about something it already
  resolved. The architecture doc and the glossary carry it.
- **No `content/SKILL.md` change.** Its Commit step is already lane-conditional
  prose — it says what git does and what jj does, never that the session should
  work out which. The mandate supplies the lane; the skill needs no edit.
- **Non-goal: nested-layout divergence.** grove walks to the *closest* marker,
  `jj root` walks up unboundedly, so the two can in principle disagree. Explicitly
  out of scope — this workstream is about not asking twice, not about
  reconciling two answers.
- **Meta-grove build boundary.** A driver change reaches no session in this loop
  until the binary is rebuilt and installed. The `using-jujutsu` edit is a
  symlinked plugin skill and lands sooner; do not read either as immediately
  live.
