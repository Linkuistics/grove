# config-resolution-k5

**Integrates:** config-resolution-k4

## Goal

Integrate the actionable findings from the adversarial design review before
`local-config-kdl-k3` implements the configuration delta. Rework the ADR set and
the implementation contract; no production code in this leaf.

## Context

Read `config-resolution-k4`'s `## Review result` in full. Its cold read found
the two-stage resolution rule clear, all four new delta alternatives worth
recording, present-tense ADR language legitimate ahead of implementation, both
inbound citations from `one-build-owns-a-session` still valid, and the brief
handoff correct. Five findings need action:

1. `complete-session-configuration:37`-`42` says an untrusted repository cannot
   ship `.grove.kdl`, but neither an ignore instruction nor the planned
   `SessionConfig::load` seam establishes that the candidate is untracked. A
   tracked file at the searched path would select an executable. This conflicts
   with settled requirement 1 and the ADR's claimed security boundary.
2. Whole-template opacity and the delta's source/placement/security policy are
   independently reversible decisions. Split them into the surviving
   `complete-session-configuration` record and a slug-named delta record rather
   than keeping one 144-line record with two grains.
3. The inline-methodology option at
   `complete-session-configuration:136`-`144` still describes a kind-selected
   prompt slice. The current `skill-delivers-the-methodology` decision says the
   skill carries the methodology and the prompt carries only the guaranteed
   core; that owner also holds the surviving inline rejection.
4. `content/references/driver.md:23`-`24` still says personal configuration
   lives at the home path. It does not bridge cleanly to a design where the
   worktree/repository delta is personal launch policy and can supply the
   selected kind. The nearby fallback-to-inheritance edit is correct.
5. The placement rationale restates finish-transaction mechanics without citing
   `task-tree-transactions-fail-closed`, while its first
   `supported-workspace-layouts` citation attributes configuration-root
   semantics to a record about working-tree/control-directory topology. The
   second citation's no-advisory-channel argument is sound.

## Done when

- The tracked-file case has an explicit, honest design outcome. If satisfying
  requirement 1 means refusing tracked candidates, record the VCS-aware seam and
  extend `local-config-kdl-k3`'s contract and test seam accordingly. If the
  security claim or requirement must instead be weakened, stop and ask the
  human; do not silently turn it into a documentation convention.
- The ADR set is a minimum coherent current-state set at one-decision grain.
  `complete-session-configuration` owns opaque complete launch values; the new
  delta record owns lookup, per-kind selection, trackedness/security,
  fail-closed behavior, placement, and the four delta alternatives. Reconcile
  `CONTEXT-MAP.md`, every citation, and the root brief by slug.
- The stale inline-methodology option is deleted or reduced to an accurate
  current citation owned by `skill-delivers-the-methodology`.
- `content/references/driver.md` remains true now and has explicit ownership for
  its post-implementation wording; `local-config-kdl-k3` cannot finish while
  leaving the paragraph stale.
- The placement rationale cites `task-tree-transactions-fail-closed` directly,
  and every `supported-workspace-layouts` citation supports the claim attached
  to it.
- Post-fix corpus verification is recorded. No production or test code is
  implemented here.

## Notes

The two existing `one-build-owns-a-session` citations depend only on command
opacity and should remain on `complete-session-configuration`. A single direct
cross-citation between the two configuration ADRs for the personal-file
completeness invariant is coherent and preferable to overlapping records.

## Decisions (running log)

**F1 resolves by enforcement, not by weakening: the delta must be untracked in
the VCS that owns it, and a tracked candidate fails closed.** The task file
allows weakening the claim only by stopping and asking the human, and the
enforcing design turned out to be buildable inside conventions grove already
has, so there was nothing to escalate. Two facts were established empirically in
throwaway repositories rather than argued. In git, a committed `.grove.kdl`
stays tracked after an ignore rule is added and `git check-ignore` then reports
nothing for it, so an ignore rule is not evidence of untrackedness and
`git ls-files --error-unmatch` is the probe that is. In jj, an unignored
`.grove.kdl` is snapshotted into the working-copy commit automatically — it was
in `jj file list` and in the colocated git index without ever being added —
while a `.gitignore`d sibling was not. So the ignore line the documentation
already had to name is load-bearing on the jj lane rather than hygiene, and
trackedness is answerable in both lanes by a read-only probe of the kind
`src/repo.rs` already runs anchored. The probe runs only when a candidate file
exists, so a checkout with no delta pays nothing.

**A tracked candidate fails closed rather than being skipped or ignored.**
Treating it as absent would be safe against the hostile-repository case but
silent in exactly the way requirement 4 rejects: an owner whose own delta had
been committed by accident would keep launching the policy they were moving work
away from. Requirement 6 also survives intact, because trackedness is not a
selection filter — found-ness still selects *the* delta, and trackedness is
validated on the selection.

**The record splits, against `config-resolution-k2`'s judgement.** `k2` argued
opacity and two-file resolution are one claim seen from two sides; the review's
independent-reversibility test is the better one, and F1 settles it — the
trackedness gate is a whole enforcement mechanism that can change without
touching whether a launch value is opaque, and vice versa. The surviving
`complete-session-configuration` owns the opaque complete value, the personal
file's completeness, direct argv execution and the environment rules that follow
from opacity. The new `untracked-configuration-delta` owns where launch policy
may come from: lookup, per-kind selection, trackedness and its enforcement,
fail-closed validation, and placement.

**Slug: `untracked-configuration-delta`, not the review's illustrative
`worktree-local-configuration-delta`.** The delta is also honoured at the main
repository root, so *worktree-local* would misname it in the one place a slug is
read. Untrackedness is the property the record now enforces and the one a reader
is most likely to want to reopen.

**`Store launch policy in each task leaf` moves to the delta record with the
four new options.** The review named only the four, but that entry is a
where-may-policy-live rejection, which is the new record's axis; leaving it
behind would keep the axis in both records, which is what the split exists to
end. The two `one-build-owns-a-session` citations and the `docs/ARCHITECTURE.md`
citation stay on `complete-session-configuration`, as the review confirmed —
each leans on opacity alone.

**F3, F4 and F5 applied as the review specified.** The inline-methodology option
is deleted rather than reduced to a citation: `skill-delivers-the-methodology`
owns both the current delivery design and the surviving rejection of a complete
inline, and nothing cites the configuration record's copy, so keeping a pointer
would buy a second search path for a decision that is not about launch policy.
`content/references/driver.md` is left untouched — F4's own verdict is that the
sentence is true today and false after the code lands — and the reconciliation is
now a `Done when` bullet in `local-config-kdl-k3`, which cannot finish while
leaving it stale. The placement rationale cites
`task-tree-transactions-fail-closed` directly. `supported-workspace-layouts` is
now cited for the one thing it establishes — the linked-worktree and
secondary-workspace families in which the two roots differ — and the brief's
pointer, which credited it with defining `${worktree}` and `${repo}`, is
corrected to say `src/repo.rs` and the delta record define those. Its second
citation, for *no advisory channel*, is unchanged and still supports its claim.

**Verification.** `cargo test` is green across the whole suite (0 failures), and
the citation surface is machine-checked rather than eyeballed:
`tests/reference_navigation.rs` passes, including
`every_repository_markdown_reference_resolves` and
`every_adr_citation_names_a_decision_record`. That instrument was controlled
rather than trusted — moving `docs/adr/untracked-configuration-delta.md` out of
the tree turned the link test `FAILED` and restoring it turned it green again, so
the pass is evidence and not a vacuous scan. `tests/rule_ownership.rs` and
`tests/loaded_path_budgets.rs` are unaffected because no `content/` file was
edited. No production or test code was written here.

**No in-session reviewer was spent and no new review chain is cut.** The
enforcing seam is the shape `config-resolution-k4`'s own F1 prescribed, not a
redesign this session invented past its findings, and the two facts it rests on
were settled empirically rather than argued. The next reader of this design is
`local-config-kdl-k3`, whose body already carries the seam, the probe idioms and
the fixture cost, and whose last act is to decide on a `review-impl` leaf.
