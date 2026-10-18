# publication-k17

**Reviews:** publication-k16

## Goal

Read the restated rules of the `technical-communication` skill against the
standard they are modelled on, and find any place that still reproduces it or
follows it too closely. The risk is licence risk: the standard may not be
redistributed, and everything in this repository becomes public history at the
release that follows. Produce findings and no fixes.

## Context

- The contract is the spec `docs/specs/technical-communication.md`, section
  *Restatement discipline*, and the ADR
  `docs/adr/technical-communication-carries-no-standard-text.md`. Ideas may
  overlap; wording, rule titles, example scenarios, an example's structure with
  a software noun swapped in, a list in the standard's order and a phrase of
  five or more words that is not ordinary technical English may not.
- The standard is `~/Downloads/ASD-STE100_ISSUE9.pdf`. Part 1 is PDF pages 45 to
  128; section 1 starts on page 45. Read it only as page images with the `Read`
  tool's `pages` option. Never extract its text, and never copy any of it into
  the repository or `.grove/`. Report by page number and by file and line, and
  describe a match in your own words.
- The files under test are in
  `plugins/linkuistics/skills/technical-communication/`: the nine files in
  `references/`, `SKILL.md` and `always-on/instruction.md`, and the user guide
  `docs/TECHNICAL-COMMUNICATION.md`.
- Review the committed state of `publication-k16` (find its commit by that
  handle) against the current tree.

## The doubt to test

`publication-k16` compared each file with its section and then received one
in-session adversarial read, which found adapted examples (the standard's example
with software nouns swapped in) and close paraphrase in several files. The
producer rewrote the examples and sentences it judged valid. **Do the rewrites
now clear the contract, or do the new examples and sentences still follow the
standard's skeletons?** Re-read every rewritten passage, and read the rest of the
files as if you had not seen the first review.

The producer left some items as they were, calling each a visible trade-off.
Test each of these calls, and say which you would change:

- An example that demonstrates a rule must share the rule's skeleton (a
  condition moved to the front of an instruction, an article added before a
  noun, "that" added after a verb, an English word for a Latin abbreviation).
  Is that skeleton shared by necessity, or does the example also keep the
  standard's scenario structure?
- The coverage tables in each file give rule numbers, and each file mirrors the
  order of one section's rules. The spec allows the numbers. Does the mirroring
  of order go further than the spec allows?
- Some identifiers are single generic terms that the standard also uses as
  headings (for example `word-count`, `vertical-list`, `false-friend`,
  `no-latin-abbreviation`). Are they titles in the contract's sense?

## Done when

- Every file under test has a verdict (clean, or the findings against it), and
  each finding gives file, line, page, kind and seriousness.
- Each of the three trade-off calls above has your verdict.
- The report names the single change that would reduce the risk most.

## Notes

- Do not edit the skill. A finding that needs a fix goes to an
  `integrate-review-impl` leaf.
- The finished leaves' logs in `.grove/` name the standard because they record
  the human's own decisions. The producer left them as they are and reports that
  as a finding for the human, so it is not yours to fix.

## Decisions (running log)

- **Review boundary.** The producer is commit `30fdcaa70734`. The working copy
  started empty on that commit, and the twelve files under review have no later
  edits. I read PDF pages 45 to 128 as images, without text extraction or OCR.
  This harness has no PDF `Read` tool, so `pdftoppm` rendered temporary page
  images outside the repository. No test, build, lint or format command ran.
- **The three trade-offs.** Numeric coverage tables and generic grammatical
  identifiers are acceptable. A necessary grammatical operation is acceptable
  too, but it does not excuse a discretionary scenario or a source-shaped
  sequence of examples. Preserving a reference file's whole teaching order is
  separate from the permitted coverage numbers. The report records that
  distinction and the places where it matters.
- **Findings.** Three items warrant integration triage: a retained instrument
  example pattern, a retained timed start-up note, and teaching outlines that
  still follow the source's topic order. The findings, all twelve file verdicts,
  the rewritten-passage check and the risk-reduction recommendation are in
  `docs/technical-communication-publication-review.md`. The implementation is
  unchanged. These findings apply the existing contract; they change no ADR.
