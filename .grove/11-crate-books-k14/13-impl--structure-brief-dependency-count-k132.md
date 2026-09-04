# structure-brief-dependency-count-k132

## Goal

Correct the dependency count in `docs/specs/grove-loop-book-structure.md`, which
says four where `crates/grove-loop/Cargo.toml` declares five, before the count is
propagated into the eighteen chapters of the `grove-loop` book that have not been
drafted yet.

## Context

- **The defect, exactly.** The brief's *1 · Orientation — `allowed-to-mean`*
  section says *the manifest's four dependencies each carry their reason in
  situ* and then names four: `anyhow`, `libc`, `keyed-launch` and
  `ordinal-fs-tree`. The `[dependencies]` table declares **five** —
  `crates/grove-loop/Cargo.toml` lines 27, 28, 29, 30 and 38 — and the fifth is
  `jj-workspace`, which the manifest's own comment accounts for as one of *the
  three modules it composes* without giving it a clause of its own. The brief's
  *Worked examples* table repeats the number in chapter 1's observable end: *the
  twelve verbs, four dependencies and one error, named*. Both say four.
- **The book already says five**, and states which four carry a reason in situ.
  `orientation-k124` checked the count against the manifest rather than against
  the brief, under the draft stage's technical-truth charter, and recorded the
  disagreement in its decision log. So the page is right and the brief is wrong;
  nothing is red, and nothing will go red — which is exactly why this is worth a
  leaf rather than a note. A `copy-edit`, `proof` or assembly session reading the
  brief against the book would find a page that departs from its own contract and
  could correct it in the wrong direction.
- The specification's rule is that a brief and its book disagreeing *is a defect
  in one of them, not a licence to prefer either*
  (`docs/specs/walkthrough-books.md`, and the brief's own opening). This leaf
  settles which.
- **`jj-workspace` is not an incidental dependency.** Five modules of the crate
  `use` it directly — `tree_lifecycle.rs`, `driver_lease.rs`, `prompt.rs`,
  `session_config.rs` and `loop_driver.rs` — and `lib.rs` republishes `Commit`
  and `Workspace`, so the
  clause the manifest does not carry for it is a genuine gap in the manifest's
  own account, and the corrected brief should say so rather than merely bumping
  four to five.

## Done when

- `docs/specs/grove-loop-book-structure.md` states five dependencies in both
  places, names which four carry a reason in situ, and says what `jj-workspace`
  is reached for.
- The wording matches what `docs/walkthroughs/grove-loop/01-orientation.md`
  already says, so the two agree in the direction the source supports.
- `bash scripts/check.sh` is no worse than it was: red on `book-check` alone
  while the book is a prefix.

## Notes

**The corpus is frozen and this leaf does not touch it.** The manifest comment is
not wrong — it names three composed modules plus two libraries, which is five —
so there is no source defect here and no fix is owed in `crates/grove-loop/`.

## Decisions (running log)
