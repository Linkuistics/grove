# skill-words-k9

## Goal

Add the rules for the standard's word-level sections to the skill, restated in
our own words: section 1 (words), section 2 (multi-word nouns) and section 3
(verbs). Vocabulary is controlled only approximately, because the skill ships
no dictionary.

## Context

- `skill-sentences-k8` created the skill and set the layout and the way rules
  are restated. Follow both. The spec and ADRs from `skill-design-k5` bind here
  as they do there.
- The standard is `~/Downloads/ASD-STE100_ISSUE9.pdf`, read as page images with
  the `Read` tool's `pages` option, working notes in the scratchpad only. The
  same ban applies: no text, no examples and no word lists from the standard.
- Sections 1 to 3 hold twenty-three rules. Rule 1.12 admits technical verbs for
  computer processes; the model writes about code, commands and tools, so
  say how that admission applies to its text. This repository's own prose is
  the case to think about.
- The human accepted approximate vocabulary control as a minor cost of shipping
  no dictionary. State the principles behind the word rules instead of lists,
  and invent the examples.

## Done when

- Rules for sections 1, 2 and 3 are in the skill, restated in our own words,
  with the same checks and the same running-log note as `skill-sentences-k8`.
- The checker-enforceable subset is marked, with matching rule identifiers.
- The skill's text still passes the document checker, and `bash scripts/check.sh`
  is green.

## Notes

- If twenty-three rules do not fit one session, decompose and finish the first
  part.
