# Complete session configuration

Every Grove session kind resolves to one complete command-template string, read
whole out of a single file; Grove executes the expanded argv directly and does
not infer a harness, model, defaults, or hidden harness-specific arguments. Two
files can supply that string and never more than two. The personal
`~/.config/grove/config.kdl` declares all nineteen kinds exactly once; an
optional **configuration delta** — `.grove.kdl` at the worktree root, or failing
that the main repository root — declares any subset of them, and each kind it
names wins outright. Search order is not merge order: the first of the two
candidate paths that holds a file is *the* delta, the two are never merged with
each other, and a kind the delta does not name comes from the personal file
untouched. Within a kind nothing is merged either — a delta replaces that kind's
whole template or says nothing about it. So resolution is two deep and flat, one
kind's command still has exactly one author, and no lattice decides which words
of a launch come from where.

This binds because launch policy includes choices Grove cannot own — model,
reasoning effort, approval, sandboxing, wrappers, and harness-specific
behavior — while *splitting one command* across environment precedence,
repository stamps, task metadata, and a built-in harness registry made the
effective command impossible to inspect or validate as one value. Selecting a
whole command from one of two files is not that split: a reader of either file
sees the entire launch on one line, and Grove still understands nothing about
the words it expands.

The personal file's completeness is what makes a partial delta safe. It is
validated in full whether or not a delta exists, so a newly added session kind
still fails visibly in every stale personal config and can never be supplied by
a delta that does not mention it — a delta narrows *who runs* a kind, never
*which kinds exist*. An unreadable, unparseable, or invalid delta fails closed
exactly as the personal file does, at both load points — before every tree
mutation and again before every launch — with the same aggregate rather than
first-error diagnostics, reported against the delta's own path and location.
Grove creates and edits neither file.

The delta is untracked, and it sits beside `.grove/` rather than inside it.
Untracked because launch policy is personal — which harness balances whose
account is not a property of the project — so it never enters the repository's
history and no clone carries one. That is also the whole security story: a
repository cannot ship a `.grove.kdl`, and therefore cannot choose which
executable Grove spawns. Beside, because `.grove/` is claimed wholesale by
mechanisms that would each do the wrong thing to a policy file: the finish
commit stages the task root as one pathspec with the transaction witness as its
only exclusion, and teardown evacuates and recursively unlinks every ordinary
entry it finds there with no regard for trackedness. Beside `.grove/` the file
survives `finish`, which is correct — the policy belongs to the checkout, not to
one grove. A delta at the repository root is inherited by every workspace of
that project, which is what makes this per-project rather than per-grove; one in
a workspace's own worktree shadows it for a one-off. The two roots are the ones
[supported workspace layouts](supported-workspace-layouts.md) distinguishes, and
in a single-worktree repository they coincide.

Before spawning the configured foreground command, Grove removes stale Grove
control values and grants its fresh signal path; it otherwise preserves the
caller's environment, including Git repository selectors. Driver-internal VCS
commands follow the opposite rule: they scrub repository selectors and anchor
Git explicitly to the leased working tree, so personal launch policy cannot
redirect lifecycle mutations.

## Considered options

- **Keep a primary harness and layer kind/family overrides over it.** Rejected
  because the result depends on a precedence lattice and still asks Grove to
  understand harness model flags and defaults. The delta is not that lattice
  returning quietly, and the difference is worth stating because it is the one a
  reader will assume away: an override replaces a kind's whole template or
  nothing, so no rule has to decide which words win, and the two candidate delta
  paths are *searched* rather than merged, so a third candidate location would
  still yield exactly one delta. Depth is two by construction rather than by
  policy. Reopen only if Grove becomes a harness-aware model router rather than
  a direct foreground launcher.
- **Provide defaults, families, inheritance, or profiles inside KDL.** Rejected
  because deduplication makes one kind's target partial: the launch would have
  to be assembled from a base and an override instead of read, which is the
  property this whole record defends, and it is why a delta may not say "the
  same command with a different model". The other half of the original
  objection — that a new kind could inherit policy its owner never reviewed — is
  **retired rather than upheld**: the personal file is complete and fully
  validated whatever a delta says, so an unlisted kind is a validation failure
  and never an inherited default. Reopen if the session-kind set grows large
  enough that explicit targets are no longer auditable and a replacement can
  preserve fail-on-kind-addition behavior.
- **Let a second file replace the personal one rather than override it per
  kind.** Rejected because a replacement must itself be complete, so it
  duplicates eighteen unchanged templates in order to change one and gives
  personal policy two places to drift; and completeness would then guard only
  whichever file is in play, so a newly added kind would stop failing visibly in
  the file that was not consulted. The partial delta buys the override without
  moving the completeness rule. Reopen only if a supported machine can have no
  personal configuration at all, which would leave a project-supplied file as
  the only complete one.
- **Store launch policy in each task leaf.** Rejected because task trees should
  describe work and remain portable, while executable, model, permission, and
  sandbox policy is personal and may change between sessions. The delta does not
  reopen this: it is untracked and outside `.grove/`, so no task artifact
  carries it and a clone reproduces none of it. Reopen only if a work item must
  carry a reproducible execution environment as part of its durable contract.
- **Track the delta so a project can share its launch policy.** Rejected because
  the file names a program to execute, so a tracked one would let any repository
  — including one merely cloned to read — choose what Grove spawns, and no
  amount of validation makes that anything but arbitrary code execution the
  operator never chose. Untrackedness is also what keeps the delta a property of
  the checkout rather than of the project's history, which is what a per-account
  balancing policy actually is. Reopen only if Grove gains a launch form it can
  constrain rather than execute verbatim.
- **Put the delta inside `.grove/`.** Rejected because two wholesale-scoped
  mechanisms already own that directory: the finish commit stages `.grove` as
  one pathspec, which would commit a file whose whole point is to stay
  untracked, and teardown recursively unlinks every ordinary entry beneath it,
  which would destroy personal policy when a workstream ends. Reopen only if
  `.grove/` gains a reserved region that is both exempt from teardown and
  excluded from the task root's pathspec, which no current mechanism provides.
- **Warn and fall back to the personal file when a delta is invalid.** Rejected
  because the fallback is silent in the way that matters: the session still
  launches, on exactly the policy its owner was moving work away from, and the
  warning arrives after the launch it should have prevented. Grove has no
  advisory channel either — the same fact that rejected *warn and continue* in
  [supported workspace layouts](supported-workspace-layouts.md) — and a warning
  re-emitted on every iteration of a self-relaunching loop is noise ignored by
  construction. Reopen only if Grove gains an advisory surface a human reads
  between sessions.
- **Execute templates through a shell.** Rejected because shell evaluation
  turns quoting, environment expansion, pipelines, and redirection into a
  second configuration language and obscures the direct foreground child Grove
  must supervise. A wrapper script supplies that power explicitly and can
  preserve ownership with `exec`. Reopen only if direct argv cannot express a
  required launch and wrappers cease to be viable.
- **Compare `research-a` and `research-b` targets to enforce vendor diversity.**
  Rejected because opaque command strings do not expose a stable harness or
  model identity: different wrappers may reach the same target and equal words
  may still produce independent corpora. `leaf-add-pair` therefore records two
  research sessions while material target diversity remains configuration-owner
  policy. Reopen only if target identity becomes an explicit comparable part of
  configuration without reintroducing harness inference.
- **Inline the complete Grove methodology into every `${prompt}`.** Rejected on
  specificity rather than size: a session handed all of `content/` performs
  exactly the selection a mandate exists to have already made for it. What
  `${prompt}` does carry is the kind-selected slice set settled by [the skill
  delivers the methodology](skill-delivers-the-methodology.md), which is a
  decision about delivery and classification and not about launch policy —
  nothing about it asks Grove to infer a harness, a model, or a default. Reopen
  never; the complete inline is refused there, and the partial one is that
  decision's to revise.
