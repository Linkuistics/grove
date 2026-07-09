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

## Notes
