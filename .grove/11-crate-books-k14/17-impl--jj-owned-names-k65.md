# jj-owned-names-k65

## Goal

Add `.gitignore` to `JJ_OWNED_NAMES` so a consumer asking for a name Jujutsu
already writes inside `.jj/` gets the refusal that names the remedy, and land it
as a corpus change the book contract permits.

## Context

- `crates/jj-workspace/src/lib.rs:60` reserves two names — `repo` and
  `working_copy`. Observed while drafting `namespace-k58`, on jj 0.44.0: a
  workspace created by `jj git init` **and** one created by
  `jj git init --colocate` both contain a third entry, `.jj/.gitignore`, whose
  contents are the single line `/*`.
- The consequence is a wrong remedy rather than a broken guarantee, and it was
  observed against this crate rather than reasoned about.
  `control_dir(".gitignore")` passes `validated_namespace` — the name is
  non-empty, contains no separator, is not `.` or `..`, and is not in the list —
  so the refusal comes from `fs::create_dir_all` failing with `AlreadyExists` on
  an existing *file*:

  ```text
  the control directory /work/atlas/.jj/.gitignore is not usable: File exists (os error 17)

  It must exist and be writable before anything can coordinate through it.
  Check the permissions on the workspace's `.jj` directory.
  ```

  The consumer is told to check permissions that are fine. The refusal it should
  get is `cannot reserve the control namespace `.gitignore`: Jujutsu owns that
  name inside `.jj``.
- **This is the reserved list's own argument coming due, not a surprise.** The
  comment at `lib.rs:55-59` prices exactly this: the cost of a name jj adds that
  the list has not heard of is a collision. Chapter 4 of the `jj-workspace` book
  (`docs/walkthroughs/jj-workspace/04-namespace.md`, *Worked example* and *The
  two names jj owns*) states the gap as an outstanding observable, so the page
  changes when the code does.
- No test in `crates/jj-workspace/tests/` covers a jj-owned name outside the
  list; `a_namespace_jujutsu_owns_is_refused` covers `"repo"` only.
- **Do not widen the scope to discovering what jj owns.** jj documents no
  interface that enumerates the names it writes inside `.jj/`, and a crate that
  shelled out to find them would depend on a private layout jj has never
  promised. The hand-maintained list is the decided shape; this leaf only
  corrects its membership.

## Done when

- `JJ_OWNED_NAMES` includes `.gitignore`, and the array's declared length moves
  with it.
- A test asserts the new member is refused as a namespace and that jj's own
  `.jj/.gitignore` is undisturbed, in the shape
  `a_namespace_jujutsu_owns_is_refused` already uses.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, every affected book ledger and page, and a green validator
  run over every book it touched. `lib.rs:55-61` is the `namespace-reserved-names`
  block, owned by chapter 4 and split into two literal fragments — the change
  moves a line boundary inside `namespace-owned-names-list`, so the fragment
  fence, its line range, the fragment-index rows and the prose that counts the
  names all move together.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately, for the reason `jj-docs-url-k64`
carries.** Editing a byte of a frozen root while a book that quotes it is being
written invalidates the ranges the freeze protects. Run this once the books that
own `lib.rs` exist and can be re-proved in the same commit.

**Whether a third name is enough is not settled here.** The list is a bound on
damage rather than a claim of completeness, and this leaf makes it current on jj
0.44.0. A future jj release can open the same gap again, and the argued position
is that this is acceptable because the cost is one-directional.
