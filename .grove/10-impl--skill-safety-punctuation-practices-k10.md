# skill-safety-punctuation-practices-k10

## Goal

Complete the skill's coverage of Part 1 with the last three sections, restated
in our own words: section 7 (safety instructions), section 8 (punctuation and
word count) and section 9 (writing practices, including the general
recommendations). After this leaf the skill covers all nine sections.

## Context

- `skill-sentences-k8` set the layout and the way rules are restated;
  `skill-words-k9` followed it. Follow both. The spec and ADRs from
  `skill-design-k5` bind here.
- The standard is `~/Downloads/ASD-STE100_ISSUE9.pdf`, read as page images with
  the `Read` tool's `pages` option, working notes in the scratchpad only. The
  same ban applies: no text and no examples from the standard.
- Sections 7 to 9 hold three, seven and four rules, plus eight general
  recommendations. Say how the safety rules apply to a model's text: warnings
  about destructive commands, irreversible steps and data loss in the documents
  and replies it writes. Say what a word count means for a reply or a document.
- The standard places itself under a house style guide and does not regulate
  formatting, abbreviations or units. The precedence order in `SKILL.md`
  already assigns those to house style; keep this section consistent with it.

## Done when

- Rules for sections 7, 8 and 9 are in the skill, restated in our own words,
  with the same checks and the same running-log note as `skill-sentences-k8`.
  All nine sections of Part 1 are now covered.
- The checker-enforceable subset is marked, with matching rule identifiers.
- The skill's text still passes the document checker, and `bash scripts/check.sh`
  is green.
- Because this is the last of the three rules leaves, read the skill as a whole
  for contradictions between sections and for any text that sits too close to
  the standard. Record what you found.

## Notes

- If the sections do not fit one session, decompose and finish the first part.
