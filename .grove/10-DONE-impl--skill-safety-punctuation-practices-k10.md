# skill-safety-punctuation-practices-k10

## Goal

Complete the skill's coverage of Part 1 with the last three sections, restated
in our own words: section 7 (safety instructions), section 8 (punctuation and
word count) and section 9 (writing practices, including the general
recommendations), in `references/safety.md`, `punctuation.md` and
`practices.md`, with their lines in `SKILL.md`'s rule index. After this leaf the
skill covers all nine sections.

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

## Decisions (running log)

**Layout.** `references/safety.md` (130 lines), `punctuation.md` (132) and
`practices.md` (299) sit beside the six earlier files. `SKILL.md` gained three index rows
and one paragraph that says when to read each file. It is 227 lines. All nine sections of
Part 1 are now covered. Each new file ends with a coverage table of rule numbers.

**Seventeen identifiers for 22 numbered items.** Section 7 became `risk-label`,
`command-first` and `state-the-risk`. Section 8 became `semicolon` (the checker's
identifier), `hyphen-compound`, `parentheses` and `word-count`, where rules 8.4 to 8.7
merged because one counting method serves all four. Section 9 became `rewrite-not-swap`
(9.1), `literal-sense` (9.2), `consistent-wording` (9.4) and one identifier for each
recommendation, with GR-3 and GR-4 merged into `clear-pronoun`. Rule 9.2 is split: its
sense and word-class halves already live in `one-meaning` and `word-class`, so
`words.md`'s coverage table gained 9.2 on those rows. Rule 9.3 is `phrasal-verb` in
`verbs.md`, whose coverage line now says so. No rule has two owners. The enforced marks
match the checker's list (`semicolon` is the only one this leaf adds).

**How the safety rules apply to a model's text.** Two levels, mapped to software: a
warning is harm that cannot be undone or that lands on other people or their things (a
deletion with no backup, a leaked secret, a live system, a payment), and a caution is
repairable damage. Both levels in one step give a warning. The skill governs the words
of a warning only. Whether the model asks first belongs to the human's instructions and
the harness permissions, and `safety.md` says so. It also covers the report after an
irreversible step (what, what changed, whether it can be undone, how) and a prompt for
another model. A warning is never a note (`notes-inform-only` already said that).

**What a word count means for a reply or a document.** The limits are per sentence. A
reply or a document has no word limit, and meaning still comes first. The reply log's
`words` and `per_100` use the counting method that `word-count` states. I probed the
checker for it instead of assuming: a token with a letter or digit is a word, a
hyphenated compound is one word, a code span and a double-quoted span count nothing,
parenthesised words count in their sentence, "10 ms" is two words, a list lead-in ends
at its colon and each item is its own count.

**Departures from the standard, on purpose.** Parentheses text counts in its sentence
(the standard counts it as one word and then counts it again as a sentence), and a
number and its unit count as two words. Both follow the checker, which this leaf does
not change, and both are the stricter reading. Safety wording and the two risk levels
are recast for software. `neutral-people` allows the singular "they" and stated
pronouns. No conformance claim follows.

**Precedence and house style.** The standard places itself under a house style guide.
`SKILL.md` said house style governs "abbreviations and units", but `short-form-after-full`
and `no-latin-abbreviation` are rules that outrank it. I reworded the item to "which units
and abbreviations to use", so house style picks them and the rules still require a full
name at first use and no Latin abbreviation. `safety.md` leaves the look of a label to
house style, and `word-count` leaves the way to write a unit to it.

**How each rule was checked against the standard.** I read pages 103 to 129 as page images
only (no text extraction), kept idea-only notes in the scratchpad, and wrote each rule
from its idea with the pages closed. Every example is invented and is about software or
model work. After the first draft I re-read my text against my memory of the pages and
changed what followed the standard too closely: the order of the hyphen list and of the
parenthesis list (both now reordered), a swap example that echoed an original (the
negated-antonym trick) and was replaced, a "shows that" example whose shape matched an
original, and a "use X to do Y" pair that I reduced to one differently shaped example.
Then I probed the whole surface (50 files: the diff against `main`, ignored files under
`plugins/`, `docs/` and `.grove/`, and every file under `.grove/`, SVG and bytecode
excluded) with 55 phrases of eight words at most each, case-insensitive with any
whitespace. The positive control (a phrase planted in a scratch copy, split across a
line) came back with one hit, the negative control (one word changed) with none. The
first real run found "general recommendations" (a section name) in `practices.md`
twice and "the condition first" in `safety.md`. I reworded all three. The only remaining
hit is "general recommendations" in this task file, from the human's own brief wording,
as in earlier leaves. `publication-k16` should weigh it. The trademark sweep
(`Simplified Technical English`, `STE100`, `ASD-STE`, whole words `ASD` and `STE`,
case-sensitive) finds only "modelled on ASD-STE100" in `SKILL.md`, twice. A planted `STE`
in a scratch copy was found, so the sweep works. The phrase list lived in the scratchpad
and is not kept. No fresh-context read was spent. The commit message asks
`publication-k16` to run one over `safety.md`, `punctuation.md` and `practices.md`, as it
does for the earlier files.

**Own text passes the checker.** `ste-lint.py` reports no hard finding on any file in the
skill, `--markers` is clean, and `--selftest` passes. `SKILL.md` has one advisory finding,
`is modelled`, kept for the sanctioned phrase. The "After" lines of the new examples pass
as text. `plugins/install.test.sh` passes (21). The three new files are not `pending` and
carry no marker.

**Whole-skill read, as this leaf asked.** I read all nine reference files, `SKILL.md`
and the two always-on files together. I found:

- One real contradiction, now fixed: the precedence item about abbreviations (above).
- No other clash between sections. The limits agree (`safety.md`, `sentences.md` and
  `procedures.md` all say 20 words for safety text). `notes-inform-only` and `safety.md`
  agree that a warning is a step, not a note. `condition-first`, `hyphen-unit`,
  `short-form-after-full`, `articles` ("this" with a noun) and `clear-pronoun` fit
  together. `full-words` forbids contractions, and `possessive-care` says "its" and not
  the contraction. `semicolon` was already in the core table and is now owned by
  `punctuation.md`.
- No gendered pronoun and no Latin abbreviation in the earlier files (grep, and the
  grep pattern was seen to match in `practices.md`, where the rules mention them).
- Text close to the standard: nothing beyond the items fixed above. The ideas overlap
  with the standard wherever a rule's idea has one natural shape, such as the warning
  label, the "with" readings and the "use X to do Y" rewrite. Each such example is
  invented and in another domain. `publication-k16` should still read these in pairs.
- One checker gap, not fixed here: a sentence that starts with a label ("Warning: Do
  not ...") is read as a description, so the checker allows 25 words where the skill
  allows 20. `safety.md` tells the model to count it itself. A later grove can teach
  the checker to skip a leading label.

**`bash scripts/check.sh` is 18 of 19 green, not fully green.** The one failure is
"Ravel caller assets and provenance": `scripts/ravel-policy-typecheck.sh` stops with
"Bun 1.4.2 is pinned; install it and retry", and this machine has Bun 1.4.3. It is the
same failure as in `skill-sentences-k8` and `skill-words-k9`, and it never reads a file
this leaf touched, so I judge it environmental. Someone with the pinned Bun should
rerun that check.
