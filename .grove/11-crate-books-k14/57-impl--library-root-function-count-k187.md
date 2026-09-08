# library-root-function-count-k187

## Goal

Correct the count in `crates/grove-loop/src/lib.rs` line 15 — *one binary per
audience, **each three functions long*** — which is wrong for both binaries, and
the `grove-loop` book's chapter 1 paragraph that repeats it. Land it as one
corpus change the book contract permits.

## Context

- Found at `manifest-function-count-k82` while removing the same defect's twin
  from `crates/grove/Cargo.toml` (*the reason it is three functions long*). Same
  class, different frozen root, different book — so it was externalised here
  rather than folded into that leaf, whose one commit could not have carried a
  green `book-check` over a book it had no business editing.

- **Measured, both binaries.** `crates/grove` defines two functions outside its
  test module — `main` (`src/main.rs:11`) and `run` (`src/cli.rs:42`); counting
  the test helper `undescribed` makes three and the two `#[test]`s make five.
  `crates/grove-llm` is nowhere near any of those numbers: `src/main.rs` defines
  `main`, `src/lib.rs` defines none, and `src/cli.rs` defines over twenty —
  `parse_kind`, `run`, `render_resolution`, and one `cmd_*` per verb. So *each
  three* is wrong for `grove` under every reading a reader takes first, and wrong
  for `grove-llm` under all of them. The word *each* is what makes it
  unrescuable: no single number is true of both.

- **The book asserts it in its own voice**, unadjudicated, at
  `docs/walkthroughs/grove-loop/01-orientation.md:286` — *"What is left outside is
  one binary per audience, each three functions long, and those binaries are the
  overview's book and the `grove-llm` book rather than this one."* The fragment
  reproducing the source bytes is `library-root-and-the-driver`
  (`lib.rs:11-15`) immediately below it. Both must change with the source.

- Prefer a wording with **no count at all**, on the rule in the grove spine's
  `references/execute.md`: a claim documented by a count of itself goes stale the
  moment the thing it counts changes. `k82` set the shape to reuse — the twin
  comment now reads *the reason there is nothing else in it* — and the two
  comments should not end up disagreeing about how grove describes a thin binary.

## Done when

- Line 15 states something the source bears out, and the chapter-1 paragraph and
  the `library-root-and-the-driver` fragment both match it.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, the affected fragment and page, and a green `book-check
  --final` over the `grove-loop` book. `source-library-root` is 377 lines and the
  single block `library-root` is `1-377`, so a reword that keeps the line count
  moves no boundary; a reword that changes it moves every block below it in
  `docs/walkthroughs/grove-loop/walkthrough.toml` and must re-prove the whole
  root.
- `bash scripts/check.sh` passes.

## Notes

**`book-check` is blind to the prose half.** It was green while the sentence was
wrong and will be green whatever number replaces it — it proves the fragment
reproduces the source bytes and nothing about whether a sentence describing them
is true. The chapter-1 paragraph has to be re-derived by hand.

**Placed after every crate book deliberately**, beside `manifest-function-count-k82`
and its neighbours: editing a byte of a frozen root while a book that quotes it is
being written invalidates the ranges the freeze protects.
