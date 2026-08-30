# Complete session configuration

Every Grove session kind resolves to one complete command-template string, read
whole out of a single file; Grove executes the expanded argv directly and does
not infer a harness, model, defaults, or hidden harness-specific arguments.
Nothing is merged within a kind — a template is authored entire or not at all —
so one kind's command has exactly one author and no rule decides which words of
a launch come from where. The personal `~/.config/grove/config.kdl` declares
each kind it supplies exactly once. A second source may supply a kind's whole
template in place of the personal file's, and that is all it may do; where such a
file comes from and what makes it admissible belong to [the untracked
configuration delta](untracked-configuration-delta.md), which leaves this record
intact precisely because what it selects is still one complete string read whole
out of one file.

This binds because launch policy includes choices Grove cannot own — model,
reasoning effort, approval, sandboxing, wrappers, and harness-specific
behavior — while *splitting one command* across environment precedence,
repository stamps, task metadata, and a built-in harness registry made the
effective command impossible to inspect or validate as one value. Selecting a
whole command from one file rather than another is not that split: a reader of
whichever file supplies a kind sees the entire launch on one line, and Grove
still understands nothing about the words it expands.

**Presence is per-kind and just-in-time; everything else about a document is
eager.** The whole of the personal file — and of any second source — is read and
validated before every tree mutation and again before every launch, for syntax,
duplicates, node shape and every template rule, so a malformed entry for a kind
this iteration will not reach still fails before anything is spawned. What is
checked at the moment of use is *presence*: before Grove writes a leaf of kind K,
and before it launches kind K, K must resolve to exactly one complete template
read whole out of one file.

The quantifier moved because Grove can no longer state it. It holds no set of
session kinds, writes no skill directory and keeps no registry, so it cannot
enumerate what a methodology declares, and a rule quantified over a set nobody
can produce is not a rule. **What is lost is exactly one thing: the early warning
for a kind not yet reached.** A stale personal configuration now fails at the
first `leaf-add` of that kind rather than at the next tree mutation of any kind.
Nothing else goes — a kind that is used still fails visibly, and it fails before
the tree is mutated rather than after. What is bought is that adding a kind no
longer wedges every operation in every stale configuration until each owner edits
their file.

The property the old quantifier was really carrying is restated per kind rather
than dropped: **a key resolves only if the primary file declares it**. A second
source may replace such a key's template and may not introduce one, so it can
never name a program the operator has not already chosen for themselves — see
[the untracked configuration delta](untracked-configuration-delta.md), which now
states that as its own property. Grove creates and edits no configuration file.

Before spawning the configured foreground command, Grove removes stale Grove
control values and grants its fresh signal path; it otherwise preserves the
caller's environment, including Git repository selectors. Driver-internal VCS
commands follow the opposite rule: they scrub repository selectors and anchor
Git explicitly to the leased working tree, so personal launch policy cannot
redirect lifecycle mutations.

## Considered options

- **Keep a primary harness and layer kind/family overrides over it.** Rejected
  because the result depends on a precedence lattice and still asks Grove to
  understand harness model flags and defaults. Selecting one file's whole
  template over another's is not that lattice returning quietly, and the
  difference is worth stating because it is the one a reader will assume away:
  an override replaces a kind's whole template or nothing, so no rule has to
  decide which words win, and candidate sources are *searched* rather than
  merged. Depth is bounded by construction rather than by policy. Reopen only if
  Grove becomes a harness-aware model router rather than a direct foreground
  launcher.
- **Provide defaults, families, inheritance, or profiles inside KDL.** Rejected
  because deduplication makes one kind's target partial: the launch would have
  to be assembled from a base and an override instead of read, which is the
  property this record defends, and it is why no source may say "the same
  command with a different model". The other half of the original objection —
  that a new kind could inherit policy its owner never reviewed — is **retired
  rather than upheld**: an unlisted kind resolves to nothing and is refused where
  it is used, never to an inherited default. Reopen if the session-kind set grows
  large enough that explicit targets are no longer auditable and a replacement
  can preserve refusal-on-unlisted-kind behaviour.
- **Execute templates through a shell.** Rejected because shell evaluation
  turns quoting, environment expansion, pipelines, and redirection into a
  second configuration language and obscures the direct foreground child Grove
  must supervise. A wrapper script supplies that power explicitly and can
  preserve ownership with `exec`. Reopen only if direct argv cannot express a
  required launch and wrappers cease to be viable.
- **Compare `research-a` and `research-b` targets to enforce vendor diversity.**
  Rejected because opaque command strings do not expose a stable harness or
  model identity: different wrappers may reach the same target and equal words
  may still produce independent corpora. A research pair — `leaf-add` given
  `--kind research-a --kind research-b --kind combine-research` — therefore
  records two research sessions while material target diversity remains
  configuration-owner policy. Reopen only if target identity becomes an explicit comparable part of
  configuration without reintroducing harness inference.
