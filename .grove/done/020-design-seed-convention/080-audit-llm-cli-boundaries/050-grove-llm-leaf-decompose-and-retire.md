# 050-grove-llm-leaf-decompose-and-retire

**Kind:** work

## Goal

Implement two `mv`-flavored verbs that operate on a single leaf:

- `grove-llm leaf-decompose <leaf-path>` converts a leaf file
  `NNN-x.md` into a node directory `NNN-x/` containing
  `BRIEF.md` seeded from the prior leaf's body. The leaf's
  content becomes the new node's brief; child work leaves are
  added afterwards via `leaf-add`.
- `grove-llm leaf-retire <leaf-path>` performs the mechanical
  `git mv` of a leaf into `.grove/done/` preserving its
  relative path. Pure mechanics — no cascade walk, no user
  prompting (those stay prose).

## Context

- `leaf-decompose` today is prose in the SKILL.md "Decompose"
  paragraph: "When a leaf is too big for one focused session, a
  planning task replaces the leaf `NNN-x.md` with a node
  `NNN-x/` holding a `BRIEF.md` (`BRIEF-FORMAT.md`) and ordered
  child leaves." The mechanical step is: create the directory
  alongside the leaf, `git mv` the leaf into it as `BRIEF.md`,
  optionally reshape the title and front matter to be brief-
  shaped. The reshape is judgement (a planning leaf's `## Goal`
  may want to become the node's `## Goal` verbatim, or may want
  rewording). Recommend: the verb does the `git mv` and
  retitles the first heading from `# NNN-x` to `# NNN-x — brief`
  while leaving body content unchanged; further reshape is the
  LLM's call.
- `leaf-retire` today is prose in the SKILL.md "Retire"
  paragraph: "After committing the task, `mv` the just-finished
  leaf into `.grove/done/`, preserving its relative path —
  mechanical bookkeeping, no need to ask." The verb is exactly
  this: `git mv <path> .grove/done/<same-relative-path>`,
  creating intermediate directories as needed.
- Parent-chain cascade after retire is **out of scope** for this
  verb — that's the "ask user before retiring each empty node,
  promote brief content upward" step, which is judgement-shaped
  and stays prose. The verb retires *one* leaf; the cascade is
  the LLM's responsibility per session.
- Existing `grove retire <name>/<node-path>` (top-level, on the
  human binary) targets *nodes* and is launcher-shaped (opens a
  retire session). `grove-llm leaf-retire <leaf-path>` is the
  mechanical leaf-level counterpart. The two coexist; SKILL.md
  must clarify when to call which.

## Done when

- `grove-llm leaf-decompose <path>` exists. It:
  1. Errors if `<path>` is not an extant `.md` file inside the
     grove's worktree.
  2. Errors if the corresponding directory `<path-without-.md>/`
     already exists.
  3. Creates the directory and `git mv`s the file into it as
     `BRIEF.md`, retitling the first-line `# NNN-x` to
     `# NNN-x — brief`.
- `grove-llm leaf-retire <path>` exists. It:
  1. Errors if `<path>` is not an extant leaf inside the
     grove's worktree.
  2. Computes the destination as
     `.grove/done/<path-relative-to-.grove>`, creating
     intermediate directories.
  3. Performs `git mv` of the source to the destination.
- Both verbs accept absolute and grove-root-relative paths.
  Both produce working-tree changes only — no `git commit`.
- Tests cover: `leaf-decompose` on a freshly-created leaf;
  `leaf-decompose` collision (directory already exists);
  `leaf-retire` of a leaf at the root level; `leaf-retire` of a
  leaf nested two levels deep (creates `done/<a>/<b>/leaf.md`).
- `content/SKILL.md`'s Decompose and Retire paragraphs are
  rewritten to direct the LLM to invoke these verbs. The
  cascade-walk and the brief-promotion-upward (both judgement)
  remain prose, with explicit "this stays prose" framing so
  future readers don't try to verb-ify them by reflex.
- The materialised `.claude/skills/grove/SKILL.md` is
  regenerated.
- This leaf is committed as one focused commit and retired into
  `done/`.

## Pointers

- Existing node-retire flow: `grove retire <name>/<node-path>`
  on the human binary. Read its implementation
  (`src/commands/retire.rs` or similar) to confirm the path
  arithmetic is consistent with the new leaf-level verb.
- Parent BRIEF inventory rows D1 (decompose) and R1
  (leaf-retire) record the audit's classification; R2
  (cascade) records why the cascade stays prose.

## Notes

- **`leaf-retire` is the simplest verb in this audit.** A
  `git mv` and a `mkdir -p`. The complexity is in the docs:
  SKILL.md must be precise about *when* the LLM calls it (after
  the task's commit) and *when* it doesn't (it never retires
  the node — the node-retire cascade is the human's call).
- **`leaf-decompose` is a one-shot.** It does not seed child
  leaves; that's the LLM's job afterwards via `leaf-add`. This
  preserves the "decomposition is lazy" principle (SKILL.md
  constraint 4).
- **No `--cascade` flag.** The cascade walk is prose (R2).
  Resist the gravity to add it as a flag here.
