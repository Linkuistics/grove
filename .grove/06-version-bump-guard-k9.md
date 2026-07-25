# version-bump-guard-k9

**Kind:** planning

## Goal

Decide whether the plugin version bump gets a mechanical guard, or whether the
written rule is accepted as sufficient — and record that decision either way.

## Context

Grown from `plugin-versioning-k5`, which pinned an explicit `version` in both
`plugins/*/.claude-plugin/plugin.json`. That stopped grove commits from
re-versioning the plugins, but the mechanism is a **pin**: the version is the
cache key Claude Code decides an update on, so a commit that changes a skill
without bumping its plugin's version reaches no consumer, `/plugin update`
reports "already at the latest version", and **nothing surfaces an error**
([Version management](https://code.claude.com/docs/en/plugins-reference#version-management)).

So the churn is gone and a new silent-failure mode is in its place. That is the
same hazard class this grove already refused to leave unmitigated once — the root
brief's *"Archiving fails silently"* note, and `cutover-k6`'s announcement
requirement, both exist because `autoUpdate: true` keeps succeeding while content
freezes. A discipline that fails silently is either guarded or knowingly accepted;
what it must not be is unexamined.

The rule itself is already written down in `CHANGELOG.md`'s policy header (who
bumps, on what, and how MAJOR / MINOR / PATCH are graded). This leaf is not about
restating it — it is about whether anything checks it.

## Done when

- The trade is settled with the human: is the exposure real enough to mechanise?
  The honest case for *no* is strong and should be argued, not skipped — the
  consumer set is effectively one machine (`docs/adr/skills-monorepo.md`), the
  author is the only person who can forget, and an unnecessary guard is a
  maintenance cost with a false-positive rate.
- If a guard is built, it is one leaf's worth of work and it is named. The
  shape to establish first is **where a check can even run**, because the obvious
  answer is wrong here: this repo is jj-native, and jj has no `pre-commit` hook —
  the working copy is snapshotted, not staged, so there is no commit-time gate to
  hang one on. That pushes the options toward CI on push, a `grove` subcommand run
  by hand or by the loop, or `cargo test` asserting a property over the tree. Pick
  by which one actually fires on the path a forgotten bump takes.
- If no guard is built, the decision is recorded where a future reader will hit
  it — the existing consequence bullet in `docs/adr/skills-monorepo.md` (which
  currently ends *"The rule is written down, not enforced"*) is the natural home,
  edited in place to say enforcement was considered and declined, and why. That
  bullet is a live loose end either way: a bare statement that nothing enforces the
  rule invites the next reader to re-open the question this leaf closed.

## Notes

- **Beware of designing the guard before deciding it is wanted.** The interesting
  question is the first bullet; the mechanism is easy once the answer is yes.
- A guard that only checks "did `version` change" is weaker than it looks: it
  cannot tell a real bump from a wrong-grade one (a removed skill bumped as
  PATCH). Deciding whether the grade is in scope is part of the shape.
- Cheap prior art to look at before inventing anything: `claude plugin validate`
  already warns on a *missing* version, and `claude plugin tag` exists for cutting
  release tags (`plugins-reference`). Check whether either already covers part of
  this before writing new tooling.
