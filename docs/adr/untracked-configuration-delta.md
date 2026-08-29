# The untracked configuration delta

Launch policy may be overridden per session kind by a **configuration delta**: a
`.grove.kdl` searched at the leased worktree root and, failing that, at the main
repository root — the same two roots `${worktree}` and `${repo}` expand to. The
first of the two paths that holds a file is *the* delta; the other is not read,
and the two are never merged with each other. Each kind the delta declares wins
outright and every kind it does not declare comes from the personal file
untouched, so search order is never merge order and resolution is two deep and
flat. What a delta may supply is unchanged by its existence: one kind's whole
template, read whole, exactly as [complete session
configuration](complete-session-configuration.md) requires, and that record's
completeness rule still binds the personal file whatever a delta says.

The delta is **untracked, and Grove enforces it** rather than asking for it. A
tracked candidate is refused and the session fails closed. The property is worth
enforcing because the file names a program to execute: an untrusted repository
that could ship a `.grove.kdl` would choose what Grove spawns in any checkout of
it, which is arbitrary code execution its operator never selected. Documentation
cannot establish that boundary, and neither can an ignore rule — a file already
committed stays tracked when an ignore line is added, so the rule is evidence of
nothing. Trackedness is the question, it is answerable by asking the workspace that owns
the candidate, and it is asked only when a candidate file exists, so a checkout
with no delta pays nothing for it. A probe Grove cannot complete fails closed
like any other unresolved validation. Grove does not run the probe: the
**version control seam** does (`crates/jj-workspace`), and the answer it gives is
about the working tree as it is on disk rather than as it was at the last
snapshot.

Asking through that seam is not a hygiene point but the seam itself: the ambient
environment must not decide the answer to the one question standing between an
untrusted repository and arbitrary code execution. The seam removes the
repository selectors from every child it spawns and pins the question by working
directory, which is what keeps it answering about the workspace Grove selected —
and it is *its* guarantee to make, so no call site can be written without it.

That enforcement is what makes the ignore line the documentation names a
requirement rather than hygiene, and the current-state answer sharpens it: an
unignored delta is refused from the moment it exists, not from the next command
that happens to snapshot the working copy. jj snapshots automatically, so an
unignored delta would be in the working-copy commit within one command and would
ride into history from there; ignored, it never enters that commit at all. The
ordering is forced and the documentation says so: `jj file untrack` refuses a
path that is not already ignored, so a delta committed by accident is ignored
first and untracked second.

An unreadable, unparseable, or otherwise invalid delta likewise fails closed —
at both load points, before every tree mutation and again before every launch,
with the same aggregate rather than first-error diagnostics the personal file
gets, reported against the delta's own path and location. Trackedness is
validated on the delta, never used to choose it: a tracked file at the first
searched path is a refusal, not a reason to read the second path. Selection is
held to the same rule one step earlier: only a candidate Grove positively
establishes to be **absent** lets the search move on, so a candidate whose state
cannot be determined at all is a refusal naming that path — never an absence
that hands the decision to the second path, or from there back to the personal
file.

The delta sits **beside `.grove/` rather than inside it**, because teardown
claims that directory wholesale: `finish-commit` recursively deletes every entry
beneath the task root with no regard for trackedness, and commits the result as
one `.grove/`-scoped fileset. Beside `.grove/` the
file survives `finish`, which is correct — the policy belongs to the checkout,
not to one grove. A delta at the main repository root is inherited by every
workspace of that project, which is what makes this per-project rather than
per-grove; one in a workspace's own worktree shadows it for a one-off. The two
roots coincide in a single-worktree repository and diverge in the
secondary-workspace family. Grove creates and edits no configuration file and
writes no ignore rule.

## Considered options

- **Let a second file replace the personal one rather than override it per
  kind.** Rejected because a replacement must itself be complete, so it
  duplicates eighteen unchanged templates in order to change one and gives
  personal policy two places to drift; and completeness would then guard only
  whichever file is in play, so a newly added kind would stop failing visibly in
  the file that was not consulted. The partial delta buys the override without
  moving the completeness rule. Reopen only if a supported machine can have no
  personal configuration at all, which would leave a project-supplied file as
  the only complete one.
- **Track the delta so a project can share its launch policy.** Rejected because
  the file names a program to execute, so a tracked one would let any repository
  — including one merely cloned to read — choose what Grove spawns, and no
  amount of validation makes that anything but arbitrary code execution the
  operator never chose. Untrackedness is also what keeps the delta a property of
  the checkout rather than of the project's history, which is what a per-account
  balancing policy actually is. Reopen only if Grove gains a launch form it can
  constrain rather than execute verbatim.
- **Document the ignore line and trust it, without checking trackedness.**
  Rejected because it makes the security boundary a convention: nothing stops a
  repository committing the searched path, an ignore rule does not untrack an
  already-tracked file, and the operator who most needs the guarantee is the one
  who cloned the repository without reading it. The cheap version of this — read
  the ignore rules instead of the index — answers a different question and
  answers it wrongly for exactly the tracked file that matters. Reopen only if
  Grove can no longer ask a VCS about a path, which would remove the check
  rather than make trusting the line correct.
- **Treat a tracked delta as absent and fall through to the personal file.**
  Rejected because it is safe against the hostile repository and silent against
  the owner: someone whose own delta had been committed by accident would keep
  launching the very policy they were moving work away from, with nothing said.
  Refusing is the only outcome that reaches the person who can fix it. Reopen
  only if Grove gains an advisory surface a human reads between sessions.
- **Put the delta inside `.grove/`.** Rejected because the finish commit would
  commit a file whose whole point is to stay untracked, and teardown would
  destroy personal policy when a workstream ends. Reopen only if `.grove/` gains
  a reserved region that is both exempt from teardown and excluded from the task
  root's pathspec, which no current mechanism provides.
- **Warn and fall back to the personal file when a delta is invalid.** Rejected
  because the fallback is silent in the way that matters: the session still
  launches, on exactly the policy its owner was moving work away from, and the
  warning arrives after the launch it should have prevented. Grove has no
  advisory channel either, and a warning re-emitted on every iteration of a
  self-relaunching loop is noise ignored by construction. Reopen only if Grove gains an advisory surface a human reads
  between sessions.
- **Store launch policy in each task leaf.** Rejected because task trees should
  describe work and remain portable, while executable, model, permission, and
  sandbox policy is personal and may change between sessions. The delta does not
  reopen this: it is untracked and outside `.grove/`, so no task artifact
  carries it and a clone reproduces none of it. Reopen only if a work item must
  carry a reproducible execution environment as part of its durable contract.
