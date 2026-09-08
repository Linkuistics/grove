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
  `docs/walkthroughs/grove-loop/08-kind-and-briefs.md#twenty-three-tests`
  (the anchor this leaf renamed with the section).
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

**1 — The clause stays. Keep it, and repair the tests around it.**
Three grounds, in order of weight.

*Cost against benefit, measured.* Deleting lines 717–722 removes six lines of
code that cannot execute and shifts every later line of a 2,023-line root:
**69** fragment ranges carrying `source="crates/grove-loop/src/task_tree.rs"`
with a start above 722, and **7** of the manifest's 10 blocks for that root
(`kind-and-brief-chain`'s end, then `resolution`, `path-composition-tests`,
`pick-tests`, `brief-chain-and-kind-tests`, `resolve-tests`,
`pick-with-brief-chain-tests`), plus the root's own `lines = 2023`. Six lines is
not worth that blast radius.

*Unreachability is conditional, not absolute, and the clause is the guard for
the condition — demonstrated, not argued.* `candidate.is_file()` (line 687) and
`candidate.canonicalize()` (line 711) are **two by-name resolutions of the same
path**, twenty-four lines apart, and neither holds a descriptor, so nothing pins
what the name denotes between them. The book's premise — *a regular file and a
directory cannot canonicalise to one path* — is true at one instant, and the
function does not act at one instant. Reproduced in a standalone program that
performs exactly those two calls with the path rebound to the grove root in
between:

    is_file() at 687      : true
    canonicalize() at 711 : /private/var/.../tmp.72mD2MDKYx/.grove
    root_real             : /private/var/.../tmp.72mD2MDKYx/.grove
    target == root_real   : true

The filename never changes, so `TaskName::parse` still yields
`Positioned { parts: Leaf }`. The clause is therefore what a deliberate
re-resolution owes: without it that argument passes `starts_with` (a path starts
with itself), matches no walked entry, and is refused by the closing `bail!`,
whose advice — *every level above it must be a node directory* — is wrong for it.
This was the one claim an in-session reviewer was commissioned for; the reviewer
stalled without reporting and the measurement above replaced it, which is the
stronger evidence anyway.

*The finding is worth more standing than removed.* Chapter 8's adjudication is
the chapter's principal measured result. Deleting the clause deletes the result;
keeping it leaves the result true and moves the repair to where the defect
actually is — the test names.

**2 — Both misleading test names are repaired, not just the one this leaf is
named for.** `brief_chain_errors_when_given_the_grove_root_itself` and
`brief_chain_errors_when_leaf_outside_grove_root` are the two the book measures
as refusing somewhere other than their names describe. Renaming one and leaving
the other would leave a test called *outside grove root* sitting beside the new
containment test that actually is one — a worse arrangement than today's.
`brief_chain_errors_when_grove_root_absent` is **not** renamed: its name claims
an absent root and an absent root is what it exercises; that it is refused by
chapter 5's opening rather than by the resolver is a fact about composition, not
a misnomer. A rename is a byte change on one line and moves no line count.

**3 — The containment fixture is built here, not deferred.** Lines 723–729 are
reachable and unobserved; the fixture that reaches them is a **task-shaped file
outside the grove root** — `<tmp>/01-impl--a-k1.md`, a sibling of `.grove`
rather than a child, passed absolute to `brief_chain_at`. It clears `is_file`,
clears the grammar arm that today's misnamed test stops at, and fails
`starts_with`. Building it now costs one re-ledger (32 ranges, 3 blocks) that a
later leaf would otherwise pay for the same lines; the rename already obliges
this leaf to re-edit the same block and the same chapter.

**4 — Out of scope, recorded rather than fixed: the operator-facing half.**
`grove-llm brief-chain .grove` answers *Grove leaf not found: …/.grove*, which is
false — the path was found; it is a directory. The clause that would say so
correctly is ordered after the one that pre-empts it. Repairing that means
reordering the clauses or adding an `is_dir` arm, both of which change line counts
at 687–722 and neither of which is in this leaf's option set.

**5 — An off-by-one this change closes by accident, recorded so it is not read as
deliberate.** Chapter 8's closing residue paragraph already opened *What the
twenty-three tests do not hold* while its own section heading said **twenty-two**,
and the block carried twenty-two: counted against the parent revision,
`sed -n '1361,1652p' | grep -c '#\[test\]'` returns **22**. The closing sentence
was wrong by one before this leaf and is right by one after it. The heading, the
block description and the group table were each corrected on the measurement
(eleven `brief-chain` and twelve `kind`, against the `// ---- kind` label at line
1538), not on the closing sentence's authority.

**6 — Line counts moved, and the carried set was enumerated rather than listed
from memory.** `task_tree.rs` 2,023 → 2,038; fifteen lines inserted at old line
1501, so every range with a start at or above it shifts by fifteen. Carried: 35
fragment ranges across `08`, `09` and `source-index`; three manifest blocks
(`brief-chain-and-kind-tests`, `resolve-tests`, `pick-with-brief-chain-tests`) and
the root's own `lines`; four `source-index` ledgers including the owned-source
total (10,542 → 10,557) and `root-to-leaf`'s 428 → 443; the reproduced bytes of
`chain-tests-refusals`; and prose in `05`, `06`, `07`, `08`, `09`, `10`, `14`,
`16`, `21`, `README` and `concept-index`. Outside this book: the corpus table and
two clauses in `docs/specs/grove-loop-book-structure.md`, and **a console
transcript in another book** — `docs/walkthroughs/jj-workspace/07-what-jj-owns.md`
reproduces `valid: 13 files, 10542 resolved lines`, which no validator would have
caught. The clean sweep for `2,023` and `10,542` was credited only after a
positive control (`2,038` found at eleven sites) and a cross-tree control
(`2,725`, an untouched root, still found by the same command).

**7 — One adjacency claim was replaced rather than re-counted.** Chapter 8 said
the relative-path chain test's *twin for the other verb appears eight tests
later*. Enumerated against the parent revision it was **ten**, not eight — a
pre-existing error — and inserting the containment test between the two endpoints
would have made it eleven. A count of position is falsified by any insertion
anywhere between its ends and no validator can see it, so the sentence now names
`kind_accepts_a_grove_root_relative_path` instead of counting to it. This is the
spine's *state the structural fact, not a count of itself* rule applied to an
ordinal.

**8 — `03-the-walk-k126`'s brief was left naming the old test.** It records what
was known when that leaf was chartered — including *a 2,023-line root* — and it
is a completed node's charter rather than a live one. Correcting it would replace
an accurate historical record with a claim about a tree that did not exist then;
k152's own task file carries the outcome, and that is where a reader is sent.

**9 — The specification's tables were *not* shifted, and the first attempt to
shift them was wrong.** The +15 sweep was applied to
`docs/specs/grove-loop-book-structure.md`'s chapter-mapping and owned-lines
tables and then reverted, because the document states its own rule four lines
above them: *the tables here are left as written because they are what the
chapter cut was decided against, and rewriting them would make them false about
the decision they record*, with the drift disclosed instead in a following *As
of …* sentence — which is why they already read `Cargo.toml` 59 and chapter 1
436 against the book's 68 and 445. Their apparent staleness is load-bearing. The
same rule kills a second edit that looked purely mechanical: line 1209 says
`canonicalisation-sites-k149` landed its fix with *`task_tree.rs` still 2,023*,
a statement about what was true when **that** leaf landed, which 2,038 would make
false. So two hunks survive in this file — the *As of* sentence, re-dated to this
leaf and extended with `task_tree.rs` 2,038 and chapter 8's 443; and the prose at
line 714, which is an argument about the corpus rather than a frozen table and
would otherwise claim sixty-three tests where there are now sixty-four.

**10 — One suspected count survived its check and no leaf was cut for it.**
Chapter 9 says the `parse_ref` mutation ran against *thirty test targets*; the
pair now yields **32** `test result:` lines. Two of those are doctest targets
(`grove-loop` one, `grove-llm` one), so under the reading the number plainly
carries — compiled test binaries — it is thirty exactly. The instrument was
wrong, not the page.

**11 — Three numbers were swept forward and then put back, because they record a
run rather than the corpus.** A count in this book comes in two shapes that read
identically, and only one of them may be made current: a claim about the corpus
*now*, and a claim about what a past leaf or mutation measured. The tell is a
named handle or a past run in the same sentence.

- `06-paths.md` (and the specification at line 1209) say
  `canonicalisation-sites-k149` reworded a header *inside its own line count …
  and `task_tree.rs` still 2,023*. That is a statement about what k149 landed;
  2,038 would make it false. Reverted, and the book's sentence now reads *still
  at the 2,023 lines it then had*.
- `09-resolve.md`'s `parse_ref` mutation and `10-growing.md`'s eleven-arm
  mutation each state their instrument — 245 inline tests, 42 binaries and 626
  tests. Those were the sizes when the mutations ran. Reverted, and each sentence
  now dates itself; chapter 9's also says that none of its arms is reachable from
  the test this leaf added, which is why the reported failure sets stand.

`14-finishing.md`'s *none of the 245 inline tests in this crate does* is the
other shape — a present-tense claim about the crate — and became 246. So did the
new mutation paragraph in chapter 8, which reports a run performed in this
session.
