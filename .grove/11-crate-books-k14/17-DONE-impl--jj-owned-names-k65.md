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

## Decisions (running log)

1. **The gap is still open on jj 0.45.1, not only on the 0.44.0 the book
   measured.** Re-measured today in a scratch tree: `jj git init` with stock
   config and `--config git.colocate=true` both write `.jj/.gitignore` holding
   the single line `/*`; `--config git.colocate=false` writes `repo` and
   `working_copy` and no third entry. So the book's two-shape account survives
   a jj minor release, and the new prose can say *0.44.0 and again on 0.45.1*
   rather than re-pinning every measurement the book took.

2. **`.gitignore` is appended last, and no line boundary moved.** The array is
   `["repo", "working_copy", ".gitignore"]` — jj's own `.jj/` listing order,
   which is the order chapter 4's tree diagram already draws. The declaration is
   73 columns, under rustfmt's default `max_width`, so `cargo fmt` leaves it on
   one line and `lib.rs` stays at 343 lines. **The task file's prediction that
   the change moves a line boundary inside `namespace-owned-names-list` is
   therefore wrong**: every block range in `walkthrough.toml` and every fragment
   range in the pages is unchanged, and nothing in the ledger is owed an edit.
   The affected set is the one fragment reproducing line 60, plus the prose that
   counts the names or argues from the gap being open — the same shape
   `jj-docs-url-k64` found.

3. **The test is `the_git_ignore_jujutsu_writes_is_refused`, on the `colocated`
   fixture, and its control was watched failing.** `a_namespace_jujutsu_owns_is_refused`
   uses `native`, which is the one shape that has no `.jj/.gitignore` — and the
   point of this test is that the refusal comes from the reserved list rather
   than from `create_dir_all` tripping over an existing entry, which only a tree
   holding the entry can distinguish. With the array reverted to two names the
   test fails with exactly the `ControlDir` message the task file quotes; with
   three it passes and jj's own file is byte-identical afterwards. Without the
   dirty read the green one would not distinguish a reserved-list refusal from a
   filesystem one.

4. **The affected set was five surfaces, and only one of them is checked.** The
   fragment reproducing line 60 in `04-namespace.md` is the whole of what
   `book-check` sees. The other four are prose that counted the names or argued
   from the gap being open: chapter 4's premise, worked example, reserved-list
   section, guard table and closing verdict; chapter 6's `ControlDir` paragraph,
   whose worked defect was exactly this call reaching the wrong arm; chapter 5's
   count of the suite; and chapter 7 in four places — the fork paragraph, the
   uncomfortable-row argument, the untested-claims table and the suite's test
   counts. The concept index was the fifth and, as at `jj-docs-url-k64`, the one
   a correction to a section does not reach on its own.

5. **The book keeps the defect rather than erasing it, because chapter 7's
   argument is built on it.** The pages could have been rewritten to read as
   though the list always held three names. That would have deleted the crate's
   only worked proof that a compiled-in fork of jj's layout actually drifts —
   which is chapter 7's refusal-4 verdict, and which the `jj-docs-url-k64` repair
   supplies for refusal 6 in exactly the same shape. So every rewritten passage
   states the gap as something that happened, was paid for by a consumer's wrong
   remedy, and was repaired by a human reading `.jj/` rather than by anything
   going red. The verdict is unchanged and better evidenced: a corrected copy is
   still a copy.

6. **Chapter 7's untested-claims table kept nine rows, and row 5 changed hands
   rather than closing.** The old row 5 — `control_dir(".gitignore")` collides in
   a colocated tree and succeeds in a native one — is no longer true of the code,
   but the repair created its own unasserted claim in its place: the name is now
   refused in a **native** workspace too, where nothing would have collided, and
   `the_git_ignore_jujutsu_writes_is_refused` stands in the colocated shape only.
   Deleting the row would have renumbered four rows and five cross-references for
   no gain; replacing it keeps the count honest and the arithmetic below the table
   ("two of the nine have leaves", "the other seven stand") re-derived by counting
   rather than by editing the old numbers.

7. **A false uniqueness claim was caught in the new test's own doc comment.** It
   read *the only one jj writes as a file*; `.jj/repo` is a pointer file in a
   secondary workspace and a directory in one that holds its own repository,
   which chapter 2 of this very book explains. Restated as *the only one of the
   three that is never a directory*, which is what the sentence was actually
   for — it is why `create_dir_all` reported this collision instead of silently
   accepting it.

8. **The leaf's one in-session reviewer was spent on the prose, and it found
   fourteen things — all real, none noise.** The claim put to it was that every
   sentence in the changed pages is true of the current source; the contract it
   was given was the book's own (byte-exact fragments, counts enumerated rather
   than trusted, cited tests read for their assertions rather than their names).
   Classified: **twelve valid and actionable**, all applied — two stale
   `two-element array` claims in chapter 7 that the first sweep never reached
   (the *verdict* table at the head of the chapter, and the closing paragraph of
   *Taking the test to a boundary of your own*); a `still reproducible on 0.45.1`
   clause whose subject read as the collision when it meant jj's behaviour; a
   conditional turned into a flat assertion (`a directory it shared with jj` — in
   a native tree the consumer shared the *name*, never a file); an incomplete
   characterisation of the new test; a cross-reference naming the wrong section
   of chapter 4; a guard-table row whose second example broke the arithmetic of
   the sentence under it; a console block that read as current behaviour when its
   output is now unreachable; three re-wrap defects; and a mis-dating of when the
   third name was added. **One a stated trade-off**: the book stays pinned to jj
   0.44.0 and now carries exactly one 0.45.1 citation, which says so in place.
   **One a finding I would not have reached** — see the next entry. Nothing was
   classified as an unclear contract or as noise.

9. **The new test left a check behind, in the cheap direction only, and chapter
   7 said the opposite.** The reviewer read the test's assertions rather than its
   name and found that its first one — the fixture really holds a
   `.jj/.gitignore` — goes red if jj ever *stops* writing the entry. So the
   sentence *nothing went red, because there is nothing that could have* was
   false as written the moment the test landed. It now says what is true: the
   suite goes red if jj drops the name and stays silent if jj adds a fourth,
   which is the constant's own one-directional argument showing up in the one
   place the suite can observe it. This is the finding the leaf-wide review
   allowance exists for: it is a claim about a negative, the compiler cannot
   establish it, and the sentence had been made *more* wrong by the change that
   was supposed to make it current.
