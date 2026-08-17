# plugin-skills-k7

## Goal

Rationalise the bundled `linkuistics` plugin skills: compress the three that
sprawl, turn two into short routers, fold one into its parent, and either delete
or sharply narrow the generic coding-style skill — with language defaults made
repo-config-first and the universal mandates in `cli-tool-design` softened.

## Context

This leaf is in the **skills** bounded context (`plugins/CONTEXT.md`), not grove's.
Its house rules — description shape, progressive disclosure, source citation,
symlink install — are that glossary's, and they apply here.

Words at the start of this grove:

| skill | words | disposition |
|---|---|---|
| `using-testanyware` | 643 | **the exemplar** — the shape the others move toward |
| `doubt-driven-development` | 2,355 | not named for change; leave unless the exemplar makes an obvious case |
| `using-codebase-memory` | 2,609 | → short router |
| `using-jujutsu` | 1,908 | → short router, absorbing `git-to-jj-mapping` |
| `git-to-jj-mapping` | 1,421 | → folded under `using-jujutsu` |
| `codebase-design` | 1,536 | compress |
| `decision-records` | 1,345 | compress |
| `simplify-project` | 1,191 | compress |
| `cli-tool-design` | 1,403 | soften universal mandates |
| `coding-style` (generic) | 295 | delete or sharply narrow |
| `coding-style-*` (7 languages) | 189–350 each | make defaults repo-config-first |
| `guardrail` | 1,191 | not named for change |
| `authoring-conventions` | 1,266 | not named for change |

**Router** means the skill states when to reach for it and routes to a reference
file for the detail — not a summary of what it dropped. `using-testanyware` at 643
words is the target shape; read it before writing any of the others.

**Repo-config-first** for language defaults: a skill should read the repository's
own configuration (formatter, linter, toolchain files) as the authority and state
its own defaults as the fallback for a repo that has none. Today they assert
house defaults unconditionally, which is wrong in any repo that has already
decided.

**Softening `cli-tool-design`** means its universal mandates become guidance with
stated applicability. A CLI design rule asserted for every tool in every language
is either trivially true or frequently wrong.

**Two skills grove depends on are in this list.** `decision-records` and
`codebase-design` are cited from `content/` in 8 of the 14 deferrals. Compressing
them changes what grove's deferral resolves to. Do not remove the ADR **AND**
test or the seam-judgement material those deferrals rely on;
`plugin-fallback-k9` audits the result immediately after this leaf, and its job is
easier if this one records what moved.

## Done when

- Each skill above has its named disposition executed, with word counts recorded
  before and after.
- The two routers route — each has a reference file carrying the detail, and the
  `SKILL.md` is short enough to be worth loading speculatively.
- `git-to-jj-mapping` no longer exists as a separate skill, and nothing references
  it by name — check `content/`, `docs/`, `plugins/`, and the installed symlink
  set.
- Generic `coding-style` is deleted, or narrowed to a scope that does not overlap
  the seven language skills; the choice is recorded.
- Language defaults read repo configuration first in each of the seven.
- `cli-tool-design`'s mandates state their applicability.
- Every skill still satisfies the skills context's description-shape house rule,
  so model-invocation triggering is not degraded by the compression.
- No skill's *subject matter* leaks into either glossary (`CONTEXT-MAP.md`).

## Notes

- Skill count matters to installation: `plugins/install.sh` symlinks each
  `linkuistics` skill into three harness directories. Deleting or folding a skill
  changes that set, and a stale symlink is left behind by the removal — the
  script re-links on re-run, but say so and check.
- `harness-compat-k8` adds the metadata that decides *where* each of these
  installs. Keep the two leaves' concerns apart: this one is about what a skill
  says, that one about who receives it.
- Compression is not the deliverable — a skill that is shorter and now says the
  wrong thing has failed. Prefer removing a passage outright to paraphrasing it
  smaller.
