# issues-substrate-brainstorm-k5

**Kind:** planning

## Goal

Brainstorm, on merit: should grove's `.grove/` task-tree structure move onto
**GitHub issues** (or grow a tracker-backed mode)? Outcome is a genuinely open
decision — stay / hybrid / move — recorded durably (likely an ADR either way,
since it either reaffirms or rewrites grove's substrate bets).

## Context

- Trigger: upstream `wayfinder` (see report Part 1 + non-actions table) proves
  grove's exact shape — lazy fog-of-war decomposition, one unit per fresh
  session, permanent names over positions — **works on a tracker substrate**:
  a `wayfinder:map` issue with child tickets, native blocking rendering the
  frontier visually in the tracker UI, claim-by-assignment for multi-agent
  concurrency, sub-issues for hierarchy.
- The tension to engage: grove's spine constraints 1 & 6 (artifacts-not-state,
  git-is-history, walk-away-able, read-don't-run bootstrap) vs what a tracker
  buys (team visibility at a URL, native blocking UI, assignment-based
  claiming, inbound-issue integration). Issues live on GitHub's server, not in
  the repo; `gh` CLI becomes a bootstrap dependency; offline work changes.
- **Read first:** `docs/research/in-repo-issue-tracker-postmortems.md` — grove
  already surveyed in-repo vs hosted trackers for the seed/inbox work; its
  failure-mode evidence (and what has changed since) is the evidence base this
  brainstorm must engage rather than re-derive.
- No sunk-cost defence of the current substrate (memory: no-sunk-cost); no
  novelty bias toward the shiny option either. Candidate hybrid shapes worth
  putting on the table: mirror/sync (tree canonical, issues as a view);
  issues-native grove variant for team repos; status quo + better `find
  .grove` visibility.

## Questions to grill

1. What problem would issues actually solve *for the user's groves today*
   (solo, local, TUI-driven)? Team visibility? Multi-agent claiming? Inbound
   capture?
2. Which spine constraints are negotiable, and what would each cost concretely
   (walk-away test per option, per driving.md research discipline)?
3. Sequencing of downstream leaves: k6 (where `kind` lives) and k7 (craft
   prose targets) queue behind this decision.

## Done when

A recorded decision (stay / hybrid / move) with rationale; ADR raised or
edited if the decision meets the when-to-write test; follow-on leaves grown if
the decision implies work.

## Decisions (running log)

**Decision: stay.** `.grove/` remains a git-tracked directory tree in the
grove's worktree. No tracker-backed mode, no mirror/sync hybrid. Settled with
the user 2026-07-09 after one round of framing; the user's stated motive for
raising it was that mattpocock's `wayfinder` drives its workstream on issues —
"that is the only reason" — and there was no independent pull toward a tracker.

Rationale, in the order it decided the question:

1. **The three tracker affordances answer problems grove does not have.**
   Facts checked, not assumed: `github` appears nowhere in `src/` (7,437
   lines); the repo has one author across 479 commits; the loop runs one
   foreground `claude` against one worktree. *Team visibility* has no team.
   *Claim-by-assignment* and the *blocking DAG* are multi-writer coordination
   primitives, and a grove tree is **single-writer by construction**
   (*task-tree-scheme*: "single-worktree, single-writer") — so they lock
   against contention that cannot arise. `find .grove` already renders the
   frontier, and `pick` already *is* the blocking rule.
2. **The spine costs are real and immediate.** Constraint 1 (state on GitHub's
   server, not the tree; history in their database, not git), constraint 2
   (bootstrap would need `gh` to *succeed* before work begins — the read-don't-run
   rule is not a preference, it is the reason a grove survives a dead network),
   constraint 6 (walk away and `.grove/` is not a folder of notes — it is
   nothing). Three constraints paid, zero affordances collected.
3. **Inbound capture is separable, and the user closed it.** The one affordance
   that named a real gap — grove shed its inbox (`self-extension-core-and-methodology`)
   and `inbox` now survives only as a fixture string in `tree_migrate.rs` — is
   not a gap the user wants closed by machinery: incoming GitHub issues are
   processed **ad-hoc by a user-directed grove**, which needs no substrate
   change and no capture subsystem.

**No ADR raised, and none edited.** The decision fails the when-to-write test
(reversible, unsurprising, no live trade-off) *and* is already durably recorded:
the spine constraints in `content/SKILL.md` state the reasoning, and
`task-tree-scheme` states the substrate. Per `driving.md` — "a research finding
that *confirmed* an existing decision … stays in the research doc with a forward
pointer" — the record lands in the incorporation report's non-actions table,
sharpened with the single-writer argument this grilling added. Writing an ADR
would restate the spine, and a restated rule is one that will later disagree
with its original.

**No follow-on leaves.** k5 was sequenced ahead of `task-kinds-model-selection-k6`
and `decomposition-craft-k7` because a substrate change could have reshaped their
implementation surface. It did not; both proceed exactly as briefed.

**What would reopen this:** grove gaining real contributors (team visibility
stops being hypothetical), or the loop gaining genuine multi-agent concurrency
(claim-by-assignment stops being a lock against nothing). Absent one of those,
this is settled.

## Notes
