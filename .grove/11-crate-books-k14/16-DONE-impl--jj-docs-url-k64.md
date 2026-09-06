# jj-docs-url-k64

## Goal

Re-point the one Jujutsu documentation URL in production source at jj's current
documentation host, and land it as a corpus change the book contract permits.

## Context

- `crates/jj-workspace/src/refusal.rs:184` prints
  `https://jj-vcs.github.io/jj/latest/install-and-setup/` in the `NotRunnable`
  remedy, which is the URL a user is told to open when jj is not installed.
- Observed while drafting `the-gate-k56`: that host now answers `301 Moved
  Permanently` to `http://docs.jj-vcs.dev/latest/...`. The link still resolves
  through the redirect, so this is staleness rather than a broken remedy — and
  it is the only external URL the crate emits, so it breaks silently and
  user-facing on the day the redirect is retired.
- **Do not widen the scope to a link checker.** Whether the repository should
  test outbound external URLs at all is a separate question with a separate cost
  (network in CI), and nothing here settles it.

## Done when

- The URL names jj's current documentation host, verified by a `200` rather than
  a redirect at the time the change lands.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, every affected book ledger and page, and a green validator
  run over every book it touched. `refusal.rs` is one root of the `jj-workspace`
  book, wholly owned by chapter 6; the change is inside a string literal and
  moves no line boundary, so the affected set is the fragment whose fence quotes
  that line.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately.** Editing a byte of a frozen root
while a book that quotes it is being written invalidates the very ranges the
freeze protects; run this once the books that own `refusal.rs` exist and can be
re-proved in the same commit.

## Decisions (running log)

1. **The current host is `https://docs.jj-vcs.dev/latest/install-and-setup/`,
   verified by measurement rather than by the redirect target.** `curl` on
   2026-09-06: the old URL answers `301` to `http://docs.jj-vcs.dev/latest/...`
   and the `https` form of that target answers `200` with no further redirect.
   The `https` spelling is taken rather than the redirect's literal `http` one —
   it is what already answers 200, and it is the spelling the other six jj
   citations in this book use.

2. **Four sites carry the URL, not one, and only one of them is checked.** The
   source literal (`refusal.rs:184`), the fragment fence reproducing it
   (`06-refusal.md:739`), the rendered-output sample in `03-subprocess-seam.md`,
   and the concept-index entry naming the defect. `book-check` sees only the
   fragment; the other three are unchecked copy — which is the concession
   chapter 7 already makes about this file, so the fix and the book's own
   argument agree rather than conflict.

3. **The line count did not move**, so no ledger edit was owed.
   `refusal.rs` stays at 230 lines and block `refusal-source` at `1-230`; the
   change is inside a string literal on line 184. The affected set was therefore
   the fragment plus the prose that argued *from* the defect, and nothing in
   `walkthrough.toml`.

4. **Two prose passages argued from the redirect being live and were rewritten,
   not merely re-pointed.** `06-refusal.md`'s *`NotRunnable`'s remedy* paragraph
   and `07-what-jj-owns.md`'s checkability-clause paragraph both cited the `301`
   as present-tense evidence. Both now state it as decay that happened and was
   repaired by hand, which preserves the argument they were making — a string
   literal quoting another tool is unchecked copy — while making it true. The
   concept-index entry "The stale installation URL in production source" was the
   summary-layer leak: correcting the section did not reach it.

5. **The clean validator read was earned with a control.** Mutating one byte of
   the source URL and re-running `book-check` with the page unchanged produced
   `F008 … expected 0x58, actual 0x2f` at source byte 7560, line 184, against
   fragment `refusal-display-seam`. Restored, the book is green. Without that
   dirty read the green one would not distinguish a correct page from a fragment
   the validator never expands.

6. **Scope held: no link checker.** Nothing here tests outbound URLs, and the
   next decay of this literal will be as silent as this one was. That is stated
   in chapter 7 rather than fixed, per the task's explicit instruction.

7. **The pilot's stage records were left alone.** `docs/evaluations/…/stages/`
   quotes the old URL in `0-draft.md` and `2-technical-edit.md`; those are frozen
   measurement records of what a stage reported at the time, and editing them
   would falsify the evaluation rather than update it.
