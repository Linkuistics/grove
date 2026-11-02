# ravel-local-sources-k78

## Goal

Narrow the skill-discovery contract of Ravel to local sources. Record why
Ravel does not fetch, install or update a source. Settle the permanent outcome
of a settings record with `type: git`.

## Context

- During ravel-git-acquire-k77 on 2026-11-02, the human chose to narrow Ravel
  to local sources. Their words: "The process of 'installing' skills and
  keeping them up to date should not be part of Ravel."
- Work in the workspace `Ravel.skill-directory-delivery`. The unpushed bookmark
  `skill-directory-delivery` is at `c2e24d371db2`. That commit backs out the
  Git acquisition of ravel-git-acquire-k70, and its tree equals the local
  stage `1d50dae5d29e`.
- `docs/specs/skill-discovery.md` still states the pinned-Git contract. The
  affected parts are the `git` settings record and "Exact Git acquisition and
  offline reuse". They also include `skill_revision_unavailable`, the `git`
  descriptor record and the `reused` and `acquired` dispositions. The Git
  acceptance rows and the Homebrew Git decision are affected too.
- The local build refuses a Git source with status 3 and no stable code. The
  local stage made that refusal temporary.
- The running log of ravel-git-acquire-k77 holds the six reproduced defects of
  the Git path. They are evidence for the decision record.

## Done when

- The specification, its design evidence and `CONTEXT.md` describe local
  sources only. No clause requires Ravel to run Git.
- The specification states the public outcome of a `type: git` record: the
  code, the stage, the exit status and the remedy.
- A Ravel ADR records the decision, the rejected Git acquisition and what
  would reopen it.
- The Ravel change is committed through jj, and the bookmark advances without
  a push. Record the output commit here and in the Grove commit.

## Notes

This leaf is design only. skill-loading-replan-k80 grows the implementation.
That implementation removes the temporary refusal, its fixture and the Git
parts of the book and `CHANGELOG.md`. Earn a focused review if the change of
the specification is not mechanical.
