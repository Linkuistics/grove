# unreachable-root-clause-k152

## Goal

Decide what to do about `leaf_entry`'s grove-root clause in
`crates/grove-loop/src/task_tree.rs` — six lines that cannot execute while the
tree is open — and, if it is removed, carry every ledger and page the removal
shifts.

## Context

- **The defect.** `crates/grove-loop/src/task_tree.rs` lines 717 to 722 read
  `if target == root_real { bail!("leaf path {} is the grove root, not a leaf", …) }`.
  Reaching it requires an argument that passed `candidate.is_file()` at line 687
  **and** canonicalises to the same path as the grove root. Chapter 5's opening
  refuses a root that is not a directory (`task_tree.rs` line 276, *grove root not
  found*), so while a `Tree` exists its root is a directory; a regular file and a
  directory cannot canonicalise to one path. The clause is unreachable.
- **It has a test named after it that does not reach it.**
  `brief_chain_errors_when_given_the_grove_root_itself` (line 1502) passes the
  grove root directory, which fails `is_file` twenty-nine lines earlier, and
  asserts on *Grove leaf not found* — the message from line 688. Found and
  adjudicated by `kind-and-briefs-k144` at
  `docs/walkthroughs/grove-loop/08-kind-and-briefs.md#twenty-two-tests`.
- **Measured, not inferred.** Deleting lines 717 to 729 — this clause and the
  `starts_with` containment check beside it — leaves all 245 of `grove-loop`'s
  inline tests green and all twenty-five of `grove-llm`'s test targets green.
  Performed in a copy of the workspace, which is the instrument a claim about a
  measurement is worth.
- **The containment clause beside it is a different case.** Lines 723 to 729 are
  reachable in principle — a task-shaped file outside the grove root reaches
  them — and merely untested. **Do not remove it on the strength of the same
  measurement**; the right repair there is a test, and this leaf should say which
  fixture would build one.
- **Removal changes line counts, and that is the expensive part.** Six lines
  deleted shifts every later line of a 2,023-line root: chapter 8's own test
  blocks (`1361-1652`, `1997-2023`), chapter 9's two blocks (`747-1015`,
  `1653-1996`), and every fragment range and ledger row derived from them.
  Keeping the clause and correcting only the test's name changes no counts and is
  the cheaper option; the leaf decides between them rather than assuming.

## Done when

- A decision is recorded, with its reason: remove the clause and re-ledger, or
  keep it and repair the misleading test name, or keep both and add the
  containment fixture.
- Whatever is chosen, `docs/walkthroughs/grove-loop/08-kind-and-briefs.md`'s
  adjudication reads correctly against the resulting source — it currently states
  the clause *cannot fire*, which stops being the finding if the clause goes.
- If any line count moved: one commit carries the source change, every affected
  manifest block range, ledger row, fragment range and page, and a green
  `book-check --final` over the `grove-loop` book.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is deferred behind the whole `grove-loop` book and says so.** A line
count moving inside `task_tree.rs` invalidates chapters 9 and 10, which are not
written, as well as chapters 5 to 8, which are. Run it after `grove-loop-k123`'s
last child reaches green `--final` validation, so the re-ledgering is done once
against a complete book instead of racing pages still being drafted.

**The corpus is frozen and this leaf is the exception the root brief describes**,
not a licence to edit `crates/grove-loop/` freely: it lands one source change with
every artifact it invalidates, in one commit, or it does not land.

## Decisions (running log)
