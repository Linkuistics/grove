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

## Decisions (running log)

**Layout.** `SKILL.md`, `always-on/instruction.md` (1,889 characters),
`always-on/reminder.md` (119) and `references/{sentences,procedures,descriptions}.md`
exist under `plugins/linkuistics/skills/technical-communication/`. `harnesses:
[any]`; no `disable-model-invocation`. The description is 219 characters and
carries the conditional trigger. `SKILL.md` is 215 lines; the references run from
99 to 188 lines.

**Sixteen rules from sections 4 to 6 became sixteen identifiers, merged and split
for clarity.** Rules 4.1, 5.1 and 6.3 share `long-sentence`, because one checker
rule enforces all three limits. The idea of 4.1's two halves splits into
`one-idea-per-sentence` and `say-what-happens`. Rules 6.4 and 6.5 merge into
`paragraph-topic`. Rule 6.1 becomes `gradual-order`, rule 6.2 `repeat-key-terms`
(its connector half lives in `connect-sentences`). The enforced rules carry the
checker's identifiers: `long-sentence`, `long-paragraph`, `dangling-conjunction`.
No rule title from the standard is used.

**`SKILL.md` states the enforced rules of the word, verb and punctuation
sections now** (`passive-voice`, `present-perfect`, `nominalisation`,
`phrasal-verb`, `marketing-adjective`, `synonym-rotation`, `semicolon`,
`spelling`, `marker`), in its core-rules table, because the checker already
enforces them and the instruction must name them. `skill-words-k9` and
`skill-safety-punctuation-practices-k10` restate the full rules and add their
index rows. The index table lists only the files that exist.

**The model stops applying the rules in a session where the human turns the
switch off.** The spec said text already in a session stays; the human's request
is precedence 1, so `SKILL.md` says to stop, and the spec's switch table now says
so. Chosen over "keep following the rules until the session ends".

**The model cannot always know that a gate runs.** `SKILL.md` says: run the checker
yourself on each prose file you write in any session where no gate message has
appeared, and always for a file that a shell command wrote.

**How each rule was checked against the standard.** I read pages 77 to 102 as page
images only (no text extraction), kept idea-only notes in the scratchpad, and wrote
each rule from its idea with the pages closed. Every example is invented and is about
software or model work. After the first draft I re-read my text against my memory of
the pages and rewrote six passages whose wording, order or example pattern followed the
standard too closely: the list-mechanics order in `vertical-list`, the opening
sentences of `full-words`, the three-way ambiguity set in `imperative-steps`, the
comma-and-condition sentence and the notes test, and four examples whose scenario
or sentence pattern echoed an original (a limit-after-measurement step, a
hold-and-act pair, a tag-the-parts connector, a no-X-permitted sentence). Then I probed
the three reference files, `SKILL.md` and the always-on files with 22 distinctive
phrases (eight words at most each), case-insensitive with any whitespace. The
positive control (a phrase planted in a scratch copy) came back with one extra hit,
the negative control (one word changed) with none, and the real surface came back
with none. The one hit class before the rewrite was "topic sentence", a general
writing term, and the near-copy of one explanatory sentence, which I rewrote. A
trademark sweep (`Simplified Technical English`, `STE100`, `ASD-STE`, whole words
`ASD` and `STE`, case-sensitive) finds only "modelled on ASD-STE100", twice
(the description and the opening paragraph); a planted `STE` in a scratch copy was
found, so the sweep works. The phrase list lived in the scratchpad and is not kept.
No fresh-context read was spent; the commit message asks `publication-k16` to run
one over the three reference files.

**Own text passes the checker.** `ste-lint.py` reports no hard finding on any new file.
`SKILL.md` has one advisory finding, `is modelled`, kept because "modelled on
ASD-STE100" is the one sanctioned phrase. `ste-lint.py --markers` is clean on all
files. `plugins/install.test.sh` passes (21), and a scratch-home run of
`install.sh --force` linked the skill into the Codex, Gemini and Pi skill folders.

**`bash scripts/check.sh` is 18 of 19 green, not fully green.** The one failure is
"Ravel caller assets and provenance": `scripts/ravel-policy-typecheck.sh` stops with
"Bun 1.4.2 is pinned; install it and retry", and this machine has Bun 1.4.3. It
stops before it reads any file this leaf touched, so I judge it environmental and
left it. Someone with the pinned Bun should rerun that check.
