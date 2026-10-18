# document-checks-k6

## Goal

Extend the vendored checker so that it can gate whole prose documents, as the
spec `docs/specs/technical-communication.md` defines in *The checker's
contract*. This is the half the document gate runs.

## Context

- The spec's section *The checker's contract* defines the contract. It covers:
  - the text types and their limits: the `auto` test for an instruction, 20
    and 25 words, and six sentences in a paragraph
  - what counts as prose in Markdown, commit messages and plain text
  - British spelling without a dictionary
  - the marker grammar (`exempt`, `depart`, `off`, `pending`, `on`), with the
    `marker` finding
  - the rule identifiers and levels, the `--markers` mode, the output formats
    and the exit codes
- The vendored checker's behaviour and its surprises are in
  `vendored-checker-k3`'s running log. The spec settles four of the surprises.
  They are the fence's kind and length, link targets and URLs, front matter,
  and the exit code for a file that is not UTF-8.
- Which files the gate checks, and its message, belong to the hook core in
  `claude-code-always-on-k11`. The `--log-start` and `--log-reply` modes belong
  to `reply-scoring-k7`.
- The checker ships with its own self-test and nothing else is tested here. The
  self-test's assertions on level names and messages follow the renames.
- This is more than the leaf's first brief named. If it does not fit one
  session, decompose and finish the first part (text types, the paragraph check
  and the markers), as `references/decompose.md` directs.

## Done when

- The checker takes a whole file, applies the limits for the text type, honours
  the markers, and reports and exits as the spec says. A justified departure is
  accepted, and a bare one is a `marker` finding.
- `spelling` flags American forms, and nothing that British usage also accepts,
  on this repository's documents.
- Messages name this skill's rules and never cite the standard's numbering or
  name. `nominalization` is `nominalisation`, and the levels are `hard` and
  `advisory`.
- Every modification is marked as ours, so the vendored base stays identifiable.
- The shipped self-test still passes under `scripts/check.sh`, and
  `bash scripts/check.sh` is green.
- Run on a few real documents from this repository (in the scratchpad, as an
  ad hoc check), it produces findings a human judges sensible. Record the
  false alarms you saw in the running log. The human logs failures in use.
- Where the code meets a defect in the spec's contract, the spec is fixed in
  place, and the running log says why.

## Notes

- Leave reply scoring to `reply-scoring-k7`.


## Decisions (running log)

**The leaf fits one session, so it is not decomposed.** The checker is one file
and the pieces depend on each other (the paragraph check needs the block model,
the markers need the same block ids). The whole contract in `The checker's
contract` is built, except `--log-start` and `--log-reply`, which stay with
`reply-scoring-k7`, and the gate's file selection and message, which stay with
`claude-code-always-on-k11`.

**Structure.** `scripts/ste-lint.py` has the same shape as upstream plus four
new parts. Each changed or added hunk starts with a `# ours:` comment (a
`begin` and `end` pair for a block). The header says so.

- `_prepare` is a masking pass that runs before any rule. It blanks what is not
  prose with spaces, so columns hold: front matter (except a `description`
  value), fences (by kind and length), code spans (any backtick count, keeping
  the outer backticks so the vendored dangling-conjunction check still sees a
  code span), HTML comments (also across lines), HTML tags and autolinks, link
  and image targets, reference definitions, bare URLs, and quotations on one
  line. It also collects the HTML comments for the markers.
- `_lint_lines` is upstream's `lint` loop on those masked lines. It assigns an
  id to every block (paragraph, list item, table row, heading), which the
  `depart` marker uses as its scope.
- `analyse(text, filename, text_type, reply)` is the whole-text entry point. It
  parses and resolves the markers, masks `off` and `pending` regions, runs the
  rules, applies the departures, and returns a dict (the spec lists the keys).
  `lint()` is a thin wrapper that keeps upstream's signature.
- Spelling is a dictionary-free table: suffix patterns for -ize and -yze, and
  stems with fixed suffix sets for -our, -re, doubled l and a few single words.
  It is built once at import.

**Decisions where the spec left room.**

- *Quotations keep their marks.* A masked quotation keeps the quotation marks,
  and keeps a final `.`, `?` or `!` when a new sentence follows. Without that,
  `He asked "why?" Then ...` runs two sentences into one.
- *Bare tokens are not words.* `#`, `-`, `|`, `---` and so on are not words, and
  list markers and heading marks are blanked before counting (`1.` would
  otherwise count as a word).
- *A capital inside a sentence is a name.* The spelling rule skips a capitalised
  word unless it starts a sentence or a line.
- *`pending` is a count of regions*, and the JSON gains `exempt` (the spec
  listed no field for it). `analyse()` also returns `markers`,
  `first_prose_line` and `unclosed`, so the gate in `claude-code-always-on-k11`
  can apply "first marker before any prose" without parsing again.
- *A reply (`reply=True`) ignores every verb except `exempt`*, faulty or not,
  and reports a faulty `exempt` as a `marker` finding. `reply-scoring-k7` may
  change this if it needs to.
- *Two more marker faults*: a `depart` alone on its line with no block after it,
  and an `off` or `pending` opened while a region is open. Both are `marker`
  findings. The spec now says so.
- *A valid `exempt` anywhere makes the whole document unchecked*, with 0 words.
- *Front matter is recognised only in `.md` and `.markdown` files*, and only when
  the first non-blank line after `---` looks like `key:`, so a document that
  opens with a thematic break is not swallowed.
- *Imperative list.* 167 base-form verbs in `IMPERATIVE_VERBS`, plus
  "do not", "don't", "never" and "always", after skipping a leading
  if/when/before/after/once clause. It is a starting list for field use to tune.
- *Messages* name the rule in this skill's words, carry the matched phrase for
  the phrase rules, and never cite the standard. The upstream "Rule 8.1" is gone.
- *Spec fixes in place*: `exempt` and `pending` in the JSON, the `--markers`
  output, the function the hook core calls, the extra marker faults, front matter
  by file type, and what the spelling rule leaves alone.

## Checks and what they showed

- The shipped self-test passes on Python 3.15 and on the system Python 3.9.6
  (the spec's floor). It grew by 92 assert statements (several run in loops): text types, the paragraph
  limit, every kind of masked text, front matter, commit messages, each spelling
  family and its exceptions, markers (valid, faulty, scope, regions, exempt,
  reply), the exit codes, `--markers` and `--json`.
- A fuzz run (6,000 random Markdown fragment mixes, three text types, three file
  names, reply on and off) raised nothing and put no finding outside the text.
  It lives in the scratchpad and is not kept. A pathological-input run (50,000
  backticks, quotes, comment openers, brackets, one 100,000-line paragraph) took
  at most 1.5 s.
- Every finding that carries a matched phrase points at that phrase in the
  source: 0 of 14,803 were misplaced on this repository's documents.
- All 819 Markdown and text files in the repository outside `.grove/`, `target`
  and `LICENSES` (857,910 words) take about 5.4 s. They give 18,731 hard and
  7,515 advisory findings (2.2 hard per 100 words). Of the hard ones, 10,415
  are `long-sentence`, 6,472 `semicolon`, 809 `spelling` and 369 `long-paragraph`.
  This repository is written in American English with long sentences, so the
  count is a baseline, not a defect.
- `docs/specs/technical-communication.md` itself gets 7 hard findings (3
  paragraphs and 4 sentences), all genuine.

**False alarms I saw** (the human logs more in use):

- `spelling` on names: a capitalised tool name right after a bold lead-in
  (`**Type analysis:** Dialyzer`) counts as a sentence start, and a heading that
  is a name (`## Harbor`) is checked. Both are rare. No lower-case false alarm
  appeared in the 809 spelling findings; every one is a real American form
  ("behavior" and its forms 293, "initialization" 51, "labeled" 24, and so on).
- `long-sentence`, `auto` mode: every sentence of an ordered-list item is held to
  20 words, including a descriptive second sentence. A sentence that starts with
  a word that is both a noun and a verb ("Test", "Report", "Update", "Record") is
  taken as an instruction. A skill's `description:` usually starts "Use when ...",
  so it gets the 20-word limit.
- `marketing-adjective` (upstream) flags "robust" in its technical sense ("robust
  to a foreign write") and in all-capital text.
- `dangling-conjunction` (upstream) flags a list item that ends "; and", which is
  the ordinary shape of a list inside one sentence.
- `synonym-rotation` (upstream) gave 620 findings on ordinary variety ("fetch"
  and "obtain"). It is a design choice, not a defect.

**`bash scripts/check.sh`: 18 of 19 pass, and "technical-communication checker
self-test" is among them.** The one failure is the one `vendored-checker-k3`
reported, and it is not from this leaf: `ravel:policy:typecheck` stops with
"Bun 1.4.2 is pinned; install it and retry" while this machine has Bun 1.4.3. I
did not change the pin: that is a repository decision. The finish cycle's
`check.sh` will meet it again, so the human needs to decide before then.
