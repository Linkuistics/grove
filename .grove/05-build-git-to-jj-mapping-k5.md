# build-git-to-jj-mapping-k5

**Kind:** work

## Goal

Create `plugins/linkuistics/skills/git-to-jj-mapping/SKILL.md` — the
on-demand git→jj reference: command and concept translation, loaded only
when translation is needed.

## Context

Settled decisions (rationale in `03-DONE-skill-design-k3.md`'s running log):

- **Model-invoked, tight description** (settled wording to adapt): "git→jj
  command and concept mapping. Use when translating a specific git command
  or concept to jj." One description line of standing context; portable
  across harnesses; `using-jujutsu` references it by name.
- **Direction is git→jj only** — the reader arrives knowing the git verb
  and needs the jj one.
- **Seed** from jaredramirez/codex-jj-plugin's compact table (MIT —
  attribute per house convention), cross-checked against mtaran/jj-guide's
  references (MIT). It covers the newer verbs worth keeping: `absorb`,
  `parallelize`, `metaedit`, `jj bisect run`, `jj file search`.
- **Concept mappings**, not just commands: staging area/index → none
  (working copy is the commit); `HEAD` → `@`; branch → bookmark (with the
  never-auto-advance caveat); stash → just `jj new` (changes stay in the
  abandoned-in-place change); `commit --amend` → keep editing `@` /
  `jj squash`; `rebase -i` → `jj squash`/`jj split`/`jj rebase`; detached
  HEAD → normal jj state; `.gitignore` honoured as-is.
- Known prior-art errors to avoid: `jj tug` is a user alias, not builtin;
  `jj split` accepts filesets non-interactively; `--limit`/`-n` is not a
  `jj log` flag (revsets instead).

## Done when

- The skill exists, follows `authoring-conventions` (capability + "Use
  when" description; attribution comments; body lean — this is a reference
  table with terse notes, not an essay).
- Every mapping row verified against local jj 0.43 (`jj <cmd> --help`);
  unverifiable rows flagged `UNVERIFIED` or dropped.
- One focused commit naming `build-git-to-jj-mapping-k5`; leaf retired.

## Notes

AFK. README/plugin.json/CHANGELOG untouched — `reconcile-and-announce-k6`'s
job. If `using-jujutsu` exists by the time this runs, keep the two
non-overlapping: workflow/behaviour lives there, translation lives here;
where the table needs a behavioural caveat, one short parenthetical plus
deference to `using-jujutsu` beats duplicating its prose.
