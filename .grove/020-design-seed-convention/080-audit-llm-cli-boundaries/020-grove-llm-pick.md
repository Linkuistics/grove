# 020-grove-llm-pick

**Kind:** work

## Goal

Implement `grove-llm pick`: deterministic depth-first walk of
`.grove/` skipping `done/`, printing the absolute path of the next
live `.md` leaf in numeric-prefix order. Replaces the prose-coded
"Pick" step in `content/SKILL.md`'s loop.

## Context

- Today's prose in `content/SKILL.md` Pick paragraph: "From the
  grove root, depth-first in numeric-prefix order, skipping
  `done/`: descend into directories; the first `.md` leaf reached
  is the next task."
- The walk's input is the grove's worktree root (the same
  worktree the binary runs in — `git rev-parse --show-toplevel`
  identifies it; the grove root is `<worktree>/.grove/`).
- The walk's output is a single absolute path, or a clear "no
  live leaves; this grove is done" message on stderr + non-zero
  exit if the tree is empty modulo `done/`. The exact exit code
  is decided in this leaf; `0` with empty stdout vs `1` with a
  message are both defensible. Recommend `0` with a single
  diagnostic line on stderr so the LLM can branch on stdout
  presence without parsing exit codes.
- Ordering rule: numeric prefix sort lexicographically across
  the zero-padded `NNN-` prefixes (the established convention).
  Files without a numeric prefix go to the end. The grove root's
  `BRIEF.md` is not a leaf — the walk starts at the root's
  children.
- Node vs leaf at the root level: a directory at `NNN-name/` is
  descended into; a file at `NNN-name.md` is a leaf candidate.
  Inside a node, the same rule applies — descend into nested
  nodes first or pick the first leaf at the current level? Per
  SKILL.md "depth-first in numeric-prefix order" — siblings are
  visited in prefix order; for each sibling, if it's a directory
  descend (depth-first), if it's a file return it. So a node
  `010-foo/` containing only `done/` is effectively empty and
  the walk falls through to `020-bar.md`.

## Done when

- `grove-llm pick` prints the absolute path of the next live
  leaf to stdout, or exits cleanly with a diagnostic on stderr
  when the grove has no live leaves.
- The walk handles all the shapes the existing grove has produced:
  root-level leaves, nested nodes, empty-but-for-`done/` nodes,
  and a fully-retired grove. Tests cover each shape with
  fixtures.
- `content/SKILL.md` Pick paragraph is rewritten to direct the
  LLM to invoke `grove-llm pick` rather than to perform the walk
  by prose. The walk's *semantics* (depth-first, numeric prefix,
  skip `done/`) stay in the prose for human readers and as the
  verb's spec.
- The materialised `.claude/skills/grove/SKILL.md` is
  regenerated.
- This leaf is committed as one focused commit and retired into
  `done/`.

## Pointers

- The implementation can lean on the existing worktree-discovery
  helpers used by `grove start|continue` to locate the grove
  root.
- Parent BRIEF (`../BRIEF.md`) "Decisions (running log)" Q1
  records the determinism principle that motivates this
  promotion; Q4 records why the verb lives on `grove-llm`.

## Notes

- **No flags.** A `--format` or `--all` flag would be feature
  creep — `pick` returns *the next* leaf, not an enumeration.
  If a future leaf needs enumeration (e.g. the TUI in 090),
  introduce a separate verb (`grove-llm walk` or similar).
- **No I/O beyond the answer.** The verb does not call
  `grove-llm brief-chain` for the user; that is a separate verb
  for a separate concern. Composability over conflation.
