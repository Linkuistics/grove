# skill-words-k9

## Goal

Add the rules for the standard's word-level sections to the skill, restated in
our own words: section 1 (words), section 2 (multi-word nouns) and section 3
(verbs), in `references/words.md`, `noun-clusters.md` and `verbs.md`, with
their lines in `SKILL.md`'s rule index. Vocabulary is controlled only approximately, because the skill ships
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

## Decisions (running log)

**Layout.** `references/words.md` (280 lines), `noun-clusters.md` (95) and `verbs.md`
(about 225) sit beside the three files from `skill-sentences-k8`. `SKILL.md` gained
three index rows and one sentence saying when to read each file. It stays near 220
lines. Twenty-three rules became twenty-one identifiers. The enforced ones keep the
checker's identifiers (`synonym-rotation`, `marketing-adjective`, `spelling`,
`passive-voice`, `present-perfect`, `nominalisation`, `phrasal-verb`). Each file ends
with a coverage table of rule numbers only.

**Merged and split.** Rules 1.2, 1.7 and 1.13 became `word-class`. Rules 1.5, 1.6 and
1.8 became `domain-terms`. Rule 1.11 is `synonym-rotation`, with `repeat-key-terms`
(`descriptions.md`) as the neighbouring rule for adjacent sentences, so no rule has two
owners. Rules 3.1, 3.2 and 3.4 became `simple-tenses`, and `present-perfect` is its
enforced slice. Rule 3.7 became `verb-for-action`, with `nominalisation` as its
enforced slice. `phrasal-verb` and `marketing-adjective` come from the checker and have
no rule number. I use "domain term" and "domain verb" and not the standard's names, and
I renamed the first draft's `technical-verbs` to `domain-verbs` for that reason.

**Rule 1.12 (verbs for computer processes).** The skill admits a verb when three tests
pass: the field names the process with this verb, no common verb says it as exactly,
and the text uses it as a verb in its domain sense. General business verbs and soft
phrasal verbs are not admitted. A command name is code, so it stays outside the rules.
For this repository's prose, a verb that the project's glossary defines (for example
"retire a leaf") is a domain verb if the text uses the defined sense only and a
document for outsiders defines it at first use. Where the glossary has no verb, use
plain words and do not invent one. Chosen over a closed list of software verbs, because
the skill ships no dictionary.

**Departures from the standard, on purpose.** Spelling stays British (the human's
decision). `passive-voice` allows the passive when the actor is unknown or does not
matter, as `SKILL.md` already says. `verb-for-action` and `nominalisation` use the plain
verb ("check"), where the standard's weak-verb pattern differs. The skill makes no
conformance claim, so these are design choices.

**How each rule was checked against the standard.** I read pages 45 to 76 as page
images only (no text extraction), kept idea-only notes in the scratchpad, and wrote each
rule from its idea with the pages closed. Every example is invented and is about
software or model work. After the first draft I re-read my text against my memory of
the pages and rewrote six passages that followed the standard's structure too closely:
the preposition list in `noun-cluster`, the abbreviation-only sentence in
`short-form-after-full`, the order of the four passive rewrites, the order of the tense
list, the three example sentences for the "-ing" jobs, and the noun-for-action example
that matched a pattern of the standard. I also rewrote the passive test question,
which the probe found verbatim. Then I probed the whole surface (54 files: the diff
against `main`, ignored files under `plugins/`, `docs/` and `.grove/`, and every file
under `.grove/`) with 37 phrases of eight words at most each, case-insensitive with any
whitespace. The positive control (a phrase planted in a scratch copy, split across a
line) came back with one hit, the negative control (one word changed) with none, and
the real surface after the fix has hits only in `.grove/` (the phrase "technical verbs"
in `plan-k1`, `skill-sentences-k8` and this task file, all from the human's own brief
wording, not from the skill). None is in the skill. `publication-k16` should weigh those
three. The trademark sweep (`Simplified Technical English`, `STE100`, `ASD-STE`, whole
words `ASD` and `STE`, case-sensitive) finds only "modelled on ASD-STE100" in
`SKILL.md`, twice, and one `STE` in a self-test assertion of the checker from an earlier
leaf. A planted `STE` in a scratch copy was found, so the sweep works. The phrase list
lived in the scratchpad and is not kept. No fresh-context read was spent. The commit
message asks `publication-k16` to run one over `words.md`, `noun-clusters.md` and
`verbs.md`.

**Own text passes the checker.** `ste-lint.py` reports no hard finding on any file in
the skill. `SKILL.md` has one advisory finding, `is modelled`, kept for the sanctioned
phrase. `--markers` is clean on all files. Two examples show text with a sales word and
other faults, and they sit in fenced blocks, which the checker skips.

**`bash scripts/check.sh` is 18 of 19 green, not fully green.** The one failure is
"Ravel caller assets and provenance": `scripts/ravel-policy-typecheck.sh` stops on the
pinned Bun 1.4.2, and this machine has 1.4.3. It is the same failure as in
`skill-sentences-k8`, and it never reads a file this leaf touched, so I judge it
environmental. `plugins/install.test.sh` passes (21).
