# 040-grove-llm-leaf-add-and-insert

**Kind:** work

## Goal

Implement two related verbs that grow the grove tree:

- `grove-llm leaf-add <slug> [--prefix NNN] [--kind work|planning]`
  appends a new leaf file at the next available numeric prefix
  in the target node (or at an explicit prefix if free), no
  renumber.
- `grove-llm leaf-insert <prefix>-<slug> [--kind ...]` inserts a
  new leaf at a specific prefix, shifting every existing sibling
  at or after that prefix up by 10, rewriting each renamed file's
  `# NNN-...` header line accordingly, and surfacing any numeric
  cross-references in the renamed files (and their siblings) so
  the operator can review which (if any) need updating.

The two verbs ship in one leaf because `insert` is "add then
renumber siblings" — they share templating, prefix arithmetic, and
file-creation logic. The renumber side is the harder part and the
featured CLI candidate that motivated this whole audit (the 020
subtree's planning history renumbered four times by hand).

## Context

- The leaf-templating convention: a new task file opens with
  `# <prefix>-<slug>` then `**Kind:** work|planning` then the
  standard sections (`## Goal`, `## Context`, `## Done when`,
  `## Pointers`, `## Notes`). See `content/TASK-FORMAT.md` for
  the format spec — the verb must produce a file consistent with
  it. The brief at `../BRIEF.md` and existing leaves under this
  node are concrete reference shapes.
- Prefix arithmetic: prefixes are three-digit zero-padded
  (`010`, `020`, …, `990`). The next available prefix at append
  is the highest existing + 10 (or `010` if the node is empty
  modulo `BRIEF.md`). The verb errors clearly if 990 is already
  occupied and a renumber would overflow.
- Insert semantics: `grove-llm leaf-insert 050-foo` in a node
  containing `040-a.md`, `050-b.md`, `060-c.md` produces:
  ```
  040-a.md            (unchanged)
  050-foo.md          (new)
  060-b.md            (was 050)
  070-c.md            (was 060)
  ```
  Each renumbered file's first-line header `# 050-b` becomes
  `# 060-b`. The `git mv` preserves history; the header rewrite
  is a separate edit committed in the same operation.
- Cross-reference surfacing: after the renumber, scan the
  renumbered files *and* the sibling files (including
  `BRIEF.md`) for occurrences of any old prefix in
  `NNN-`-shaped tokens (regex: `\b\d{3}-`). Print each
  occurrence with file:line for the operator to review. Do
  *not* auto-rewrite — cross-references may be intentional
  (e.g. "the previously-considered leaf 040" in a brief is a
  historical reference, not a live pointer). The verb's job is
  to surface; the operator decides per occurrence.
- Target-node disambiguation: by default the verb operates on
  the node directory it's invoked from (cwd-relative). An
  explicit `--node <path>` flag lets the LLM target a node
  without `cd`. Recommend the cwd default for ergonomic LLM use
  in a session that's already chosen the working node.
- Add-vs-insert decision rule for the LLM (lives in SKILL.md):
  use `leaf-add` when growing the tree at the end (the common
  case), `leaf-insert` only when a new concern surfaces that
  must sequence ahead of existing leaves.

## Done when

- `grove-llm leaf-add` and `grove-llm leaf-insert` exist with
  the flag shapes specified above. Both produce templated leaf
  files consistent with `content/TASK-FORMAT.md`.
- `leaf-insert` performs the renumber correctly: `git mv` each
  affected sibling, edit each renamed file's header line, and
  print cross-reference candidates to stderr (so stdout stays
  parseable if the LLM wants the new leaf's path on stdout).
- Tests cover: empty-node add; non-empty-node add at next
  prefix; explicit-prefix add to a free slot; collision error
  on explicit-prefix add to an occupied slot; insert at the
  start; insert in the middle; insert at the end (degenerates
  to add); insert that would overflow `990`; cross-ref
  surfacing on a real fixture.
- `content/SKILL.md` Decompose paragraph is rewritten to direct
  the LLM to invoke `leaf-add`/`leaf-insert` rather than to
  perform the `git mv` and header-rewrite dance by prose. The
  decision rule (add at end vs insert with renumber) lives in
  the prose; the mechanics live in the verb.
- The materialised `.claude/skills/grove/SKILL.md` is
  regenerated.
- This leaf is committed as one focused commit and retired into
  `done/`.

## Pointers

- Featured failure mode documented in the parent BRIEF
  (`../BRIEF.md`) inventory row D2 and in the
  `020-design-seed-convention/BRIEF.md` notes §2 — four manual
  renumbers in this subtree's planning history.
- Existing `git mv` invocations in the codebase
  (`src/commands/*.rs`) show the project's preferred git
  plumbing patterns.

## Notes

- **If cross-reference surfacing turns out to need design**
  (e.g. the regex catches too many false positives, or the
  rewrite-or-not decision deserves an interactive prompt),
  split this leaf into `040-grove-llm-leaf-add.md` and
  `045-grove-llm-leaf-insert.md`. The simpler `add` verb
  ships first either way.
- **Both verbs operate on the working tree only.** No
  `grove-meta` interaction, no inbox interaction. Pure
  `.grove/` mechanics inside the grove's worktree.
- **The verb does not run `git commit`.** It produces a
  working-tree change (new file + renames + edits) and prints a
  summary. The LLM (or operator) commits as part of the
  enclosing task's commit.
