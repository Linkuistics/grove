# skill-sentences-k8

## Goal

Create the `technical-communication` skill as a working whole: its `SKILL.md`,
the cross-cutting rules, the always-on instruction and reminder text, and the
rules for the standard's sentence-level sections (4 sentences, 5 procedural
writing, 6 descriptive writing), restated in our own words. After this leaf the
skill loads and a model can follow it; `skill-words-k9` and
`skill-safety-punctuation-practices-k10` complete the coverage.

## Context

- Read the spec `docs/specs/technical-communication.md` and the ADRs it cites
  first. *Layers and layout* fixes the files: `SKILL.md`,
  `always-on/instruction.md` and `always-on/reminder.md`, and `references/` with
  one file for each section. This leaf writes `references/sentences.md`,
  `procedures.md` and `descriptions.md`. The same section fixes the size
  budgets, the description's conditional trigger, the instruction's content and
  order, and the rule identifiers. *The checker's contract* holds the markers
  and *The document gate and conversion* holds the conversion procedure that
  `SKILL.md` states. *Restatement discipline* binds every rule and example.
- The standard is `~/Downloads/ASD-STE100_ISSUE9.pdf`. It is licensed to the
  human and may not be redistributed. Read sections 4 to 6 as page images with
  the `Read` tool's `pages` option; the file disallows text copying. Keep your
  working notes in the scratchpad, never in the repository or `.grove/`, which
  become public history.
- The skill's shape: `plugins/linkuistics/skills/technical-communication/`, with
  `harnesses: [any]` and a description in the house shape (capability plus "Use
  when"; read the `authoring-conventions` skill). Do not set
  `disable-model-invocation`: this skill must load on its own.
- The checker is already in `scripts/` from `vendored-checker-k3` and
  `document-checks-k6`. The spec maps its rules to the skill's rules.

## Done when

- `SKILL.md` carries the precedence order (the human's request, then meaning,
  then this skill's rules, then house style), British spelling, the exclusions,
  the text types and limits, the markers, the lazy-conversion procedure with its
  fallback for large documents and its landing step, how to run the checker
  (including on a commit message, a pull request or an issue text before it is
  sent), and the switch.
- The description stays within 400 characters and says to use the skill when
  the always-on instruction is in context or when the human asks for
  controlled English.
- The always-on instruction (2,500 characters at most) and the reminder (200
  at most) exist as harness-neutral files that follow the spec's content
  contract. The instruction states facts, not commands, and its core rules carry
  their identifiers and state no rule that `SKILL.md` lacks. The harness leaves
  inject them.
- Rules for sections 4, 5 and 6 are restated in our own words. No sentence, rule
  wording, explanatory text or example comes from the standard; every example
  is invented. Say in the running log how each rule was checked against that.
- Each rule that the checker enforces is marked as such, and the checker's rule
  identifiers match.
- The skill's own text passes the document checker, apart from justified
  departures. `plugins/install.sh` treats the skill as eligible on all four
  harnesses, and `bash scripts/check.sh` is green.
- The text states no conformance claim and carries no trademarked name beyond
  "modelled on ASD-STE100" where the spec places it.

## Notes

- Sections 4 to 6 hold sixteen rules. If they do not fit one session, decompose
  and finish the first half, as `references/decompose.md` directs.
- Section 1.12's admission of technical verbs for computer processes belongs to
  `skill-words-k9`, not here.
- The restated rules are the licence risk. If you judge a read by a fresh
  context necessary, say so in the commit message so `publication-k16` weighs it;
  do not spend more than the one in-session reviewer.
