# 030-grove-llm-brief-chain

**Kind:** work

## Goal

Implement `grove-llm brief-chain [<leaf-path>]`: walk ancestors from
the given leaf (or `pick`'s output if no argument) up to the grove
root, printing each `BRIEF.md` path one per line in root→leaf
order. Replaces the prose-coded brief-chain enumeration in the
Bootstrap step.

## Context

- Today's prose in `content/SKILL.md` Bootstrap paragraph reads:
  "the BRIEF chain root→leaf; the task file." The enumeration is
  mechanical — from a leaf path, walk parent directories up,
  collecting any `BRIEF.md` found at each level, until reaching
  `.grove/` (which itself contains a root `BRIEF.md` — also
  included in the chain).
- Input: an optional leaf path (relative to grove root or
  absolute). If absent, the verb invokes the same internal
  function as `grove-llm pick` to obtain the next leaf.
- Output: one absolute path per line, ordered root→leaf. For a
  leaf at `.grove/020-design-seed-convention/080-audit-llm-cli-boundaries/010-scaffold-grove-llm-and-migrate-inbox.md`,
  the output is:
  ```
  /…/.grove/BRIEF.md
  /…/.grove/020-design-seed-convention/BRIEF.md
  /…/.grove/020-design-seed-convention/080-audit-llm-cli-boundaries/BRIEF.md
  ```
  The leaf file itself is *not* included — that's the LLM's
  separate read, not part of the brief chain.
- ADRs cited *by* the briefs are not in scope — that's
  judgement-shaped reading and stays prose. A future flag
  (`--with-cited-adrs`) was discussed in grilling and deferred;
  do not implement it in this leaf.

## Done when

- `grove-llm brief-chain` with no argument resolves to the
  current pick and prints the chain.
- `grove-llm brief-chain <path>` with an explicit leaf path
  prints the chain for that leaf. Accepts both absolute and
  grove-root-relative paths.
- Edge cases handled with tests: leaf at the grove root (only
  `.grove/BRIEF.md` returned); leaf two levels deep (root +
  middle + node BRIEFs); leaf whose intermediate directory has
  no `BRIEF.md` (skip that level, do not error — some nodes may
  not yet have a brief).
- `content/SKILL.md` Bootstrap paragraph is rewritten to direct
  the LLM to invoke `grove-llm brief-chain` rather than to walk
  ancestors by prose. The semantics (root→leaf, one path per
  line) stay in the prose as the verb's spec.
- The materialised `.claude/skills/grove/SKILL.md` is
  regenerated.
- This leaf is committed as one focused commit and retired into
  `done/`.

## Pointers

- Implementation can share the grove-root-discovery helper with
  `grove-llm pick`.
- Parent BRIEF (`../BRIEF.md`) inventory row B1 records why this
  step is a "promote"; B2 (ADR citation extraction) is the
  related-but-out-of-scope concern.

## Notes

- **No `BRIEF.md` at a level is not an error.** Some nodes may
  not yet carry a brief (e.g. mid-decomposition transient
  state). Skip silently; the LLM gets the chain it would have
  got from prose execution.
- **Composability with `pick`.** `grove-llm brief-chain` with no
  argument == `grove-llm brief-chain "$(grove-llm pick)"`. The
  no-arg form is sugar; the two-step form must continue to work
  unchanged.
