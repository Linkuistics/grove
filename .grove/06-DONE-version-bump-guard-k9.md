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

## Decisions (running log)

- **The pin goes; there is no guard because there is no discipline left to
  guard.** The leaf's frame (guard-or-accept) assumed `plugin-versioning-k5`'s
  explicit `version` stands. Reopened and reversed: drop `version` from both
  manifests and return to the commit-SHA fallback. Deleting a silent failure mode
  beats guarding one when what you give up is cosmetic. Evidence: (a) the churn
  `k5` removed is conceded cosmetic in `k5`'s own notes, while the staleness it
  bought is silent and unbounded; (b) the vendor's guidance points the same way —
  [Version management][vm] recommends the commit-SHA approach for *"internal or
  team plugins under active development"* and reserves explicit versions for
  *"published plugins with stable release cycles"*, warning *"if you're iterating
  quickly, leave `version` unset"*; (c) no guard site exists that fires on the
  path a forgotten bump takes — this repo has no CI at all, and jj has no
  `pre-commit` hook because the working copy is snapshotted rather than staged.

  [vm]: https://code.claude.com/docs/en/plugins-reference#version-management

- **The record is the rewritten consequence bullet in
  `docs/adr/skills-monorepo.md`, not a new ADR.** The delivery choice exists *only
  because* the plugins share a repo whose commit rate is grove's, so it is entailed
  by the monorepo decision and unreadable without it; a standalone ADR would have
  had to restate that premise, against `linkuistics:decision-records`'
  minimum-coherent-set rule. Rewritten in place as current state — no "we used to
  pin" — with `_Reopens if_` naming consumers beyond this machine.

- **Nothing records that a guard was considered, deliberately.** The leaf's third
  done-when expected the bullet to say enforcement was declined and why. That is
  moot under the reversal: the guard question only existed to protect the bump
  rule, and the bump rule is gone. A reader cannot ask "why is this rule
  unenforced?" about a rule that no longer exists, so recording the deliberation
  would be changelog, not current state.

- **The unpinning landed in this same session** rather than being handed to a work
  leaf, so no commit exists in which the ADR describes a manifest state the repo is
  not in. A record that contradicts the tree is the same defect class as a dangling
  citation.

- **The marketplace's missing `description`** — surfaced here, owned by nothing —
  went to `cutover-k6` as a done-when, the leaf that publishes and verifies the
  marketplace, rather than being fixed inline or grown into a leaf of its own.

## Findings

Established by inspection this session, not inherited from the briefs:

- **The SHA fallback is repo-HEAD-wide, not per-subdirectory.** Both plugins are
  installed as version `e0ba6f40f6e8` — the *same* string for two different
  source subdirectories. `k5`'s churn premise was therefore right: in a monorepo
  every plugin's version moves with every unrelated commit.
- **Neither piece of prior art the brief flagged covers this.** `claude plugin
  validate` reads a manifest and has no history access, so staleness is invisible
  to it (it passes on `plugins/linkuistics` as pinned today). `claude plugin tag`
  creates a **git** tag, and this repo is jj-native — no `.git`, and jj cannot
  create git tags — so it cannot run here at all.
- **`validate --strict` warns on a *missing* version** ("Consider adding a
  version following semver"), so unpinning makes strict validation fail and the
  tooling nudges a future reader toward re-pinning. Recording the decision where
  that reader will find it is the mitigation. (`--strict` is already failing on
  the marketplace manifest for a missing `description` — separate concern.)
- **`plugins/<name>/` contains only `.claude-plugin/` and `skills/`**, so
  everything under it ships and the retired bump rule's scope was exact:
  `plugins/CONTEXT.md` and `plugins/README.md` sit outside it, which is why this
  grove's two earlier `plugins/`-touching commits correctly needed no bump.

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
