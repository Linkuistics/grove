# wrapped-link-labels-unchecked-k186

## Goal

Make `book-check` see the Markdown links it currently skips: a link whose **label
is hard-wrapped across a line** is dropped by the scanner without a diagnostic, so
it is never link-checked in any book.

## Context

- `crates/book-validation/src/markdown.rs:686-689`: after finding `](`,
  `scan_links` does `if markdown[index..separator].contains('\n') { index =
  label_start; continue; }`. The link is neither scanned nor reported — it simply
  leaves the corpus the checker sees.
- The guard is defensible in intent (it stops a `[` in one paragraph pairing with
  a `](` in the next), but its effect is silent under-coverage, and hard-wrapped
  prose is this repository's house style — so the excluded case is the *common*
  case, not an edge one.
- Two live examples, both in a book that passes `--final --check all`:
  `docs/walkthroughs/jj-workspace/06-refusal.md:230` and `:463` — cited at `:216`
  and `:445` when this leaf was cut, and moved down by
  `jj-workspace-method-counts-k184`, which rewrote prose above both. Both are
  same-page `#anchor` links with wrapped labels. They are unchecked, and — see
  below — the resolver would reject them if it saw them.
- **A second, separable defect the first one hides.** `resolve_local`
  (`markdown.rs:835-860`) pops the filename off the source path and, for an empty
  path component, returns the *directory*. So a bare `#anchor` link resolves to
  `docs/walkthroughs/<book>`, which is in none of the three file maps, and is
  reported as `M201 … resolves to missing repository file`. Measured directly:
  the same link written on one line fails `book-check`; wrapped across two, it
  passes. Whether bare `#anchor` should be legal at all is the decision this leaf
  owes — the two examples above assume it is.
- Found while correcting book prose in `env-selector-coverage-k68`, whose
  one-line same-page link went red where the book's existing wrapped ones do not.

## Done when

- A wrapped-label link is scanned like any other, or is reported — silence is the
  one outcome ruled out. If the newline guard is kept in some form, it is narrowed
  to the pairing hazard it was written for (a blank line between `[` and `](`)
  rather than to any newline.
- Bare `#anchor` links have a stated answer: either `resolve_local` returns the
  source file for an empty path and they are legal, or they are rejected with a
  message that names the remedy. Whichever is chosen, the two occurrences in
  `06-refusal.md` end up conforming rather than exempt.
- The gap has a test that has been **seen to fail**: a fixture whose only link is
  a wrapped one pointing at a target that does not exist must go red. A test
  asserting the current behaviour proves nothing about the change.
- `bash scripts/check.sh` passes over all six books.

## Notes

**This is a validator change, so expect it to find things.** Every book's prose
becomes visible to the link checker for the first time, and a wrapped link with a
stale destination is exactly what has been able to hide. Re-running `book-check`
across all six books before writing any prose fix is the cheapest way to size it —
and if the fan-out is larger than one session, that sizing is the decomposition,
not a reason to narrow the fix.

**`book-validation` is deliberately outside the documented corpus** (root brief,
*P1*): it is the authoring tool, not the system being documented, so no book root,
fragment or ledger is at stake here and the freeze rule does not bind. Only the
prose fixes the change forces would touch a book.

## Decisions (running log)

**The newline guard is narrowed, not removed.** `scan_links` now rejects a
candidate label only when the span from `[` to `](` crosses a **blank line**,
which is exactly the pairing hazard the guard was written for. A label is inline
content, and CommonMark 0.31.2 §4.8 has it that *paragraphs can contain multiple
lines, but no blank lines*
(https://spec.commonmark.org/0.31.2/#paragraphs) — so a blank line is the point
past which a `[` left open in one paragraph can no longer find a `](` in the
next. Verified against a live implementation rather than inferred from the
prose: cmark-gfm, through GitHub's `/markdown` API, renders
`[label\nwrapped](url)` as a single anchor and leaves `[ bracket\n\nlater](url)`
as two paragraphs of plain text. Any-newline was the wrong boundary because hard-wrapped prose is this repository's house style, which made
the excluded case the common one. `crosses_blank_line` inspects only the
*interior* lines of the span — the first element is the tail of the line the
bracket opened on and the last runs up to the separator, so neither is a whole
line and `[label\n](target)` stays a link.

**Bare `#anchor` links are legal, and `resolve_local` returns the source page for
an empty path.** The alternative — rejecting them with a named remedy — was
declined on three grounds. `crates/grove/tests/reference_navigation.rs:178-192`
already states the repository's position in a doc comment (*bare fragments …
resolve within the rendering page*) and `unresolved_reason` at :204 resolves an
empty path part against the source file, so rejecting them in `book-check` would
have put the two checkers in contradiction. `docs/USAGE.md` and
`docs/ARCHITECTURE.md` use the form throughout. And the old behaviour — popping
the filename and returning the *directory*, which is in none of the three file
maps — was not a policy anyone chose; it was an unreached branch that could only
ever report `M201 … resolves to missing repository file`.

**The stated answer lives in the spec, because that is where the link contract
is stated.** `docs/specs/walkthrough-books.md`, *The link contract*, said local
links use `[descriptive label](relative/path)` with an optional
`#explicit-anchor`, which under-determines both questions this leaf settles: it
neither permits nor forbids an empty path, and says nothing about a label that
wraps. Two sentences were added there — the path may be empty and means the page
that carries the link; a label may be hard-wrapped but not span a blank line,
with CommonMark's reason. Nothing else in the section changed, no heading moved,
and the spec set stays current-state rather than gaining a superseding document
(`SPEC-FORMAT.md`).

Note that this was **not** a case of following the corpus's house style, because
there is no house style to follow: enumerated across `docs/walkthroughs/`, seven
same-page links name the page's own filename and four are bare, and
`jj-workspace/06-refusal.md` uses both. The decision rests on the three grounds
above, and the spec now records that both forms are permitted and neither is
preferred.

**The four occurrences in the books conform unedited.** The task file named two
(`jj-workspace/06-refusal.md`); a scan over the whole of `docs/` found two more
(`grove-loop/01-orientation.md:178` and `:359`), also wrapped, also bare
`#anchor`. All four now resolve against their own page and are anchor-checked.
No book prose changed.

**Fan-out: zero.** All six books pass `--final --check all` unedited, and
`scripts/check.sh` reports *all 8 principal checks pass* — 6 books checked, 0
failing — over a tree frozen for the run and digest-checked unmoved after it.
The widening reaches further than the books: `scan_markdown_links` is also the
scanner behind `every_repository_markdown_reference_resolves`, which sweeps every
Markdown file in the repository outside `UNSWEPT_DIRECTORIES`, and that test is
green on the widened scan too. Measured with `scan_markdown_links` over every
Markdown file outside `UNSWEPT_DIRECTORIES`, against the final scanner: 364
files, 3,020 links, of which **49** have hard-wrapped labels and were previously
invisible, and **0** are malformed. **18** of the 49 are inside
`docs/walkthroughs/` — 12 in `grove-loop`, 4 in `jj-workspace`, 1 each in
`ordinal-fs-tree` and `overview`. The malformed count is the one to watch: the
intermediate version that only narrowed the guard produced **9**.
That every one of the eighteen was already correct is the measurement, not an
assumption: the check was confirmed live by breaking `06-refusal.md`'s wrapped
`#the-cause-chain` to `#no-such-anchor`, which now reports `M201` at `233:28`
where before the change it reported nothing.

**Narrowing the guard was not sufficient, and the adversarial pass is what
showed it.** The in-session reviewer this leaf spent its allowance on
(`references/execute.md`) found that the widening had introduced a defect class
of its own, in files no book check reads. `scan_links` pairs the **outermost**
`[` with the **first** `](`; while a label could not cross a line that was
contained, but once it can, one unmatched bracket in prose swallows everything
up to the next real link. Reproduced before believing it:
`docs/formalism-findings.md:10197` produced a **28-line** label that consumed the
genuine link at `:10224`, and `CONTEXT.md:528` a 29-line one out of the
`[[Glossary term]]` syntax — each reporting a destination against a line its
author never wrote it on.

`scripts/check.sh` was green throughout that, which is the part worth keeping.
The repository-wide sweep filters on neither `valid_syntax` nor the label, and
the swallowed destination survives, so the instrument that should have caught
this could not — a clean read from a blinded control, exactly the failure
`references/execute.md` describes.

**Three further changes make the widening safe**, each pinned by a test seen to
fail against its own negation:

- **Brackets pair innermost-first**, as CommonMark pairs them. `scan_links`
  restarts at the last unescaped `[` inside a candidate label instead of
  reporting a label that contains one. This is what kills the whole class:
  after it, every genuine link above is back at its own line, valid.
  (`a_stray_bracket_does_not_swallow_a_later_link_on_another_line`)
- **A separator inside an opaque range cannot pair.** `find("](")` scans raw
  bytes, so it reached into fenced code the parser had already marked opaque;
  only the cursor was ever checked against those ranges.
  (`a_separator_inside_a_fence_does_not_pair_with_a_bracket_above_it`)
- **`blank_line` is CommonMark's, not `str::trim`'s.** `trim` strips every
  Unicode `White_Space` character, so a line holding one non-breaking space read
  as a paragraph break and dropped a real link in silence — the very outcome
  this leaf exists to rule out. Now space, tab, or CR only.
  (`a_line_of_unicode_whitespace_is_not_a_paragraph_break`)

**An empty anchor now names its remedy.** `#` and `page.md#` were rejected as
*escapes the repository scope*, which describes a `../..` climb and says nothing
to an author who typed a trailing `#` — and the two are far easier to confuse now
that an empty *path* is legal. This is the second half of the *Done when*'s
"rejected with a message that names the remedy", for the one bare-anchor form
that stays illegal.

**Every finding classified, as the four-step pass requires.** Nine came back.
Seven were valid and actionable and are fixed above or in the spec. One was a
**visible trade-off**, now stated in the spec rather than left implicit: the
blank line is the *only* block boundary the scanner recognises — a bare `>`
inside a block quote, a new list item and a heading are not breaks to it — and
innermost-first pairing is what carries those cases instead. One was **noise**:
the claim that the old `resolve_local` returned `None` and reported *escapes the
repository scope* was measured against a revert that dropped my early return
without restoring the `if !path.is_empty()` wrapper around the loop, so it
described a code state that never existed. The parent commit
(`jj file show -r @-`) and this session's own first run both show the original
returning the **directory** and reporting `resolves to missing repository file
docs/walkthroughs/ordinal-fs-tree`. The finding's second clause was fair, and the
comment was tightened for it: no bare-anchor link was ever *visible* to be
reported, because a wrapped label hid all four.

**Two of the original tests asserted absence, which the old code also produced.**
Both were strengthened rather than defended: the wrapped-live-target test now
asserts positively against `scan_markdown_links` — one link, its exact label and
destination — because the skipped link produced a clean report too, and `valid`
alone cannot tell coverage from silence; and the bare-anchor test now asserts the
diagnostic **names the page**, since resolving to the directory raised an `M201`
as well, just a different one.
