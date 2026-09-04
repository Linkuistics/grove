# what-a-runner-cannot-k129

## Goal

Draft Part V of the `grove-loop` book — chapters 16 to 20, owning
`crates/grove-loop/src/driver_lease.rs` (1,383 in two blocks),
`src/session_config.rs` (358), `src/prompt.rs` (245) and `src/loop_driver.rs`
(615), 2,601 lines in all — and prove the prefix through slice
`four-things-a-runner-cannot-choose`, which resolves the last deferred block in
the book.

## Context

- Draft stage, child 6 of 7 of `grove-loop-k123`. The structure brief is
  `docs/specs/grove-loop-book-structure.md`; the five chapters are its sections
  *16 · One live driver per working tree* through *20 · The loop*.
- **Expect this to decompose, one child per chapter.** Five chapters at 819, 564,
  358, 245 and 615 lines.
- **This part carries both halves of the prose obligation, and they are
  opposite.** `driver_lease.rs` splits at its `#[cfg(test)]` line into chapter 16
  (1–819, **12% comment prose**) and chapter 17 (820–1383, **3%**) — the only
  split in the book that is not by concept, made because the two halves need
  opposite treatments and one chapter cannot carry both instructions. Chapter 16
  takes *supply the argument*: per mechanism, the line that enforces it, the
  failure it prevents, and the record clause it keeps. Chapter 17 takes *supply
  the claim*: what each scenario establishes and what it would still pass under.
  Chapters 18, 19 and 20's production halves — 44%, 69% and 51% — take *do not
  restate*.
- **Chapter 16 is the book's deliberate counterexample.** Everything else in the
  crate is re-derived from the tree; the lease and the epoch cannot be, and
  chapter 1 has already said so in one sentence. Do not re-argue it, and do not
  let the chapter open by contradicting the spine.
- **Chapter 18 adjudicates the read-count claim and chapter 20 refutes it.**
  `src/session_config.rs` line 89 says the loop re-reads the configuration once
  per iteration; `src/loop_driver.rs` calls `templates.load(&delta_roots)` twice,
  at lines 241 and 260, for two different reasons the brief names. This is the
  campaign's only adjudication whose claim and refutation are both inside one
  book's corpus. State it beside the fragment that reproduces the comment; never
  repeat it as true and never silently correct it. `template-source-read-count-k86`
  holds the source fix and lands after this book.
- Chapter 19's rule is the too-late test, and its closure on the word *fact*: a
  driver fact is a launch-varying value the methodology cannot know at authoring
  time, and its static meaning stays in the skill.
- Chapter 20's rule is that all the spawning, watching and escalating is
  `keyed-launch`'s; what stays is the four things a loop must choose — the
  channel's directory, the variable that publishes it, the variables scrubbed,
  and the two graces. It also carries *restart ≡ continuation* and the shell
  sketch the header keeps, which is still the whole loop *because a boundary is
  not a step*.
- Chapter 1's cast rows owned by `one-per-working-tree`, `whose-file-and-whether`,
  `too-late-to-say-later` and `four-things-a-runner-cannot-choose` move to
  `explained` as those chapters land — the last of them closing every row in the
  ledger.
- Guide anchor `usage-driver-lease` and glossary anchors `driver-lease`,
  `session-epoch`, `guaranteed-core`, `stated-vcs` and `loop-control-channel` are
  this part's; all exist today.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  four-things-a-runner-cannot-choose --check all` is valid: 13 files, 10,533
  resolved lines, 0 deferred, still `final=false` because chapter 21 does not
  exist yet.
- Chapters 16–20 exist, contents and navigation are updated, and every ownership
  row in the ledger reads `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline — and `session_config.rs`'s stale sentence in particular is
adjudicated on the page rather than corrected.

## Decisions (running log)
