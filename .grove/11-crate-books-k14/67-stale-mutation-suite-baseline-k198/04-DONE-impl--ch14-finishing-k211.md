# ch14-finishing-k211

## Goal

Re-derive `docs/walkthroughs/grove-loop/14-finishing.md`'s *What the refusals are
worth, measured* under the node brief's control, and make its two `558` citations
and every sentence that rests on that fifteen-arm table say what the suite
actually shows. Its table restates four of chapter 11's arms from the other side,
so it must agree with `ch11-a-grove-begins-k208` **test for test**.

## Context

- **One row is already known to be stale, and it is not a total.** Row 2 is
  `transition_to_current`'s `RootShape::ATree` arm — chapter 11's row 6, the same
  mutation from the other side of the file inversion. `ch11-a-grove-begins-k208`
  measured it at **ten reddened, two observing**, against the page's *Newly
  failing: 2*: eight of the ten are out-of-process fixtures in
  `crates/grove-loop/tests/driver_lease.rs` and
  `crates/grove-llm/tests/removed_surface.rs` that drive the loop against a
  worktree already holding a grove, so they die in the fixture on any panic in the
  driver's pre-launch path and observe nothing about this arm. Chapter 11 chose to
  report the distinction — *reached* versus *observing* — rather than a new
  number, because the page already draws exactly that line for its entry probe.
  **Adopt the same distinction here** and say so, or the two chapters state
  different things about one mutation.
- **The rest of the table should hold, and a row that moves is the finding.**
  This chapter's procedure paragraph already names the relink step — *build the
  `grove` binary first so `CARGO_BIN_EXE_grove` is set* — so like chapters 12 and
  13 nothing here was measured through an unrelinked binary. Chapter 12
  reproduced nineteen for nineteen and chapter 13 twenty-four for twenty-four.
- **There is exactly one duplicated test name in the whole suite, and it is
  *this* chapter's row 12.** Reading a mutant as `comm -13` over **bare** test
  names merges two tests that share a name in different binaries. Measured at
  `ch13-outcomes-k210`: the 560-test control carries **559** distinct bare names,
  and the single duplicate is
  `finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, present in
  both `crates/grove-llm/tests/finish_commit.rs` and
  `crates/grove-loop/tests/verbs.rs` — which the page's row 12 cell names in
  **both** files and counts as part of its *3*. Qualify every failure name by its
  binary before diffing, or row 12 reads 2 and looks like a moved cell when
  nothing moved. It bit no earlier chapter: it appears in no newly-failing set of
  chapters 11, 12 or 13.
- **Six cells are zeros, and a zero means untested *or* dead**
  (`mutation-cannot-see-unreachable`). Rows 5, 6, 8, 10, 13 and 14. The page
  already argues the split — two unreachable (5, 6), three needing an environment
  failure no fixture builds (8, 13, 14), and row 10 — so the re-derivation owes
  each zero a re-measurement, not a re-reading, and owes the *arguments* a check
  that they still describe the code.
- **Five arms are held only from outside the crate, and here that is literally
  true.** Rows 7, 9, 11, 12 and 15 are `finish_commit`, `delete_and_commit` and
  `require_recoverable_grove`, and the page says not one observer is an inline
  test — they live in `crates/grove-llm/tests/finish_commit.rs`, which runs the
  binary and reads its stderr. Row 12's second file is `grove-loop`'s own
  `tests/verbs.rs`, so *outside the crate* is right for four of the five and
  wrong for row 12; `ch13-outcomes-k210` hit the same over-claim on its own page
  and repaired the boundary rather than the count.
- **Two roll-ups sit on the table**: *Nine arms are held and six are not*, and
  *four of the eight arms with observers belong to `transition_to_current`, and
  of the five tests holding them, three sit in chapter 11's block and two in this
  one*. Both are derived from the column, so both are falsified by a cell that
  moves — and *nine held* versus *eight with observers* in adjacent paragraphs is
  worth checking against the runs rather than against each other.

## Done when

- All fifteen runs are taken against the node brief's control, each confirmed to
  have reported 560, each read as a `comm -13` over **binary-qualified** failure
  names, and any mutant whose newly-failing set names a `driver_lease.rs` or
  `prompt.rs` test re-run before it is believed.
- The *Newly failing* and *Where the observer lives* columns state what the runs
  show, row 2 carries chapter 11's reached-versus-observing distinction in the
  same terms chapter 11 used, and both roll-ups above agree with the column.
- The two `558` citations — the *control is not zero* bullet and the *Every
  mutant below ran 558 tests* check — each state the re-derived number. The first
  also gives one cause for eleven failures (*the scratch copy is not a jj
  repository*) where the run's own stderr shows ten on `NotAWorkspace` and the
  eleventh on the missing `.claude-plugin/marketplace.json`; chapters 11, 12 and
  13 all name both causes.
- Every prose attribution of an arm to a named test — including the *`Taskless`
  reddens one, `Unrecognised` reddens one, `ATree` reddens two* summary about two
  hundred lines earlier in the chapter — states what the runs show.
- Chapter 14's last act cuts the next leaf: chapters 15 and 16 travel together
  (`--kind impl`, slug `ch15-16-configured-command`), per the node brief's
  decomposition.
- `book-check --final --check all` is green, and `bash scripts/check.sh` is no
  worse than before.

## Notes

**No source change.** If a mutant exposes a defect in the crate, cut a leaf for
it; the corpus freeze is the parent node's rule.

**The control, reproduced three times now.** The node brief's *Pointers* carries
the copy recipe and it has reproduced exactly at `ch11-a-grove-begins-k208`,
`ch12-leaf-to-node-k209` and `ch13-outcomes-k210` — 560 tests, 549 passed, 11
failed, matched set for set, about 55 seconds per mutant on a shared
`CARGO_TARGET_DIR`. Two traps that both read as clean results: a mutant that fails
to compile prints no per-test lines, and a mutant mutated at the *function* rather
than at the call site a verb uses reddens every verb's tests. Pre-checking all
mutants with `cargo check -p <crate> --tests` before the study closes the first
trap ahead of the runs rather than after them; `ch13-outcomes-k210` did that and
it cost one pass.

## Decisions (running log)

**The fifteen cited arms all still name their cited arms.** Located line by line
in `crates/grove-loop/src/tree_lifecycle.rs` before anything was run, as the node
brief requires: 79 (`Vacancy`), 88 (`ATree`), 89 (`Taskless`), 98
(`Unrecognised`), 135 (`materialize_finish`'s `.context`), 166 (`finish_slug`'s
`map_err`), 212, 218, 222, 228, 230, 245 (`finish_commit`'s six), 285, 298
(`delete_and_commit`'s two) and 322 (`require_recoverable_grove`). Fifteen arms,
fifteen lines, no drift — so a count that moves is the harness or a new test.

**The control reproduces exactly, and its eleven split ten and one.** The node
brief's copy recipe, then `cargo build -p grove --bins`, then
`cargo test --no-fail-fast -p grove-loop -p grove-llm` on a shared
`CARGO_TARGET_DIR`: **560 tests, 549 passed, 11 failed**, the eleven matching the
brief's set name for name. The run's own stderr splits the cause exactly as the
task file predicted — **ten** on `Refusal(NotAWorkspace { … })` and
`the_namespace_is_the_shipped_plugin_entrys_declared_name` on the marketplace
manifest being `NotFound`. Re-confirmed rather than re-derived against a ten.

**Binary qualification was set up before the first reading, not after.** Every
mutant is `comm -13 control mutant` over failure names of the form
`<test file>@<crate>::<test name>`, with the compilation hash stripped so the
label survives a source edit. The control's 560 test lines carry **559** distinct
bare names and the single duplicate is
`finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, exactly as
`ch13-outcomes-k210` measured. Unqualified, row 12 would have read **2** and
looked like a moved cell.

**All fifteen runs are taken, every one reporting 560 and every one building
clean.** All fifteen were pre-checked with `cargo check -p grove-loop --tests`
before the study began.

| row | arm | line | page says | re-measured |
|---:|---|---:|---|---|
| 1 | `Opening::Vacancy` scaffolds | 79 | 1 | **3** |
| 2 | `RootShape::ATree` | 88 | 2 | **10 reached, 2 observing** |
| 3 | `RootShape::Taskless` | 89 | 1 | same, alone |
| 4 | `RootShape::Unrecognised` | 98 | 1 | same, alone |
| 5 | `materialize_finish` — sentinel without a key | 135 | **0** | **nothing** |
| 6 | `finish_slug` — `"finish"` invalid | 166 | **0** | **nothing** |
| 7 | no `.grove` at all | 212 | 1 | same, alone |
| 8 | `symlink_metadata` failed otherwise | 218 | **0** | **nothing** |
| 9 | root is not a directory | 222 | 1 | same, alone |
| 10 | no live leaf remains | 228 | **0** | **nothing** |
| 11 | live work remains | 230 | 1 | same, alone |
| 12 | handle is not the live finish leaf | 245 | 3 | **same three** |
| 13 | the store's `delete` failed | 285 | **0** | **nothing** |
| 14 | the commit failed | 298 | **0** | **nothing** |
| 15 | jj tracks nothing under the root | 322 | 1 | same, alone |

**Thirteen of fifteen reproduce; the two that moved moved for two different
reasons, and neither is arithmetic.**

1. **Row 2 agrees with `ch11-a-grove-begins-k208` test for test, which is what
   this child was cut to check.** Ten reddened: the two the page names, plus the
   five `driver_lease.rs` and three `removed_surface.rs` fixtures chapter 11
   enumerates — the same ten names, from the other side of the file inversion.
   Reported in chapter 11's own terms, **ten reached, two observing**, because
   the eight drive the loop against a worktree that already holds a grove and die
   in their fixtures rather than in an assertion about what the arm answered.
   Adopting a new number here instead would have made the two chapters state
   different things about one mutation.
2. **Row 1 moved from 1 to 3, and the page's own table did not predict it.** The
   two extras are genuine observers, not merely reached, and they arrive by
   different routes. `both_scaffolding_doors_name_the_first_leaf_the_same`
   (`grove-llm/tests/root_init.rs`) calls `transition_to_current` **in process**
   against a bare temp directory and compares the leaf the arm scaffolds against
   `root-init`'s — no harness variant could have missed it, so it is a **new
   test**: `default-root-slug-two-spellings-k159` added it (that leaf's own
   decision 1 says so, and chapter 15 reads the same fact from the verb's side).
   `bare_scaffolding_is_anchored_before_the_configured_command_inherits_git_context`
   (`grove-loop/tests/driver_lease.rs`) runs the driver against a worktree with
   **no** `.grove/` — its fixture comment says the transition "has a mutation to
   anchor: creating the grove" — and asserts the scaffolded leaf lands in the
   intended tree and nowhere else. That one is the **harness**.

**Why an unrelinked run loses exactly those observers, proved from source rather
than from history.** `testing/support.rs:434`'s `workspace_binary` rebuilds
`target/<profile>/grove` **only if the file does not exist**, and
`cargo test -p grove-loop -p grove-llm` never rebuilds the `grove` package at
all. So a harness that builds `grove` once and then mutates leaves every observer
that shells out to the driver binary green — `driver_lease.rs` and
`removed_surface.rs`, and nothing else. `grove-llm`'s binary is rebuilt by the
test command itself, which is why rows 7, 9, 11, 12 and 15 — all
`finish_commit.rs` — reproduce untouched. The page's procedure sentence said
*build the `grove` binary first so `CARGO_BIN_EXE_grove` is set*, which is wrong
twice: the build belongs after each edit, and these tests do not use
`CARGO_BIN_EXE_grove` — `testing/support.rs`'s own doc comment explains why they
cannot.

**The message-preserving claim was measured, and it is right for four arms of
five.** The page asserted that a `panic!` carrying each `bail!`'s own format
string would have left all five observers of the ending green. Re-run as a second
five-mutant study with the words preserved: arms 7, 9, 11 and 15 redden
**nothing**, exactly as claimed. Arm 12 reddens **two** —
`verbs.rs`'s copy, which calls `verbs::finish_commit` in process where a panic
reddens whatever it says, and `a_refused_handle_is_quoted_as_the_operator_wrote_it`,
which fails on a missing `other-k007`. That spelling is not in this arm's message
at all: the arm prints the canonical `other-k7`, and the operator's own spelling
is added a layer up by `crates/grove-llm/src/cli.rs:454`'s
`.with_context(|| format!("`grove-llm finish-commit {finish_handle}`"))`, which a
panic unwinds straight past. So the claim's *shape* survives and its scope was
over-stated (`joint-justifications-split`); the repair names the exception.

**Two roll-ups checked against the runs, and one was arithmetic.** *Nine arms are
held and six are not* stands — held are rows 1, 2, 3, 4, 7, 9, 11, 12 and 15;
unheld are 5, 6, 8, 10, 13 and 14. *Four of the **eight** arms with observers* was
simply wrong beside it, and the task file was right to flag the adjacent
eight-versus-nine as the tell: it is nine. The companion clause — *of the five
tests holding them, three sit in chapter 11's block and two in this one* — is now
seven tests, five of them in the two blocks (three and two, unchanged) and two in
`tests/` directories, which this book treats as evidence rather than as roots and
which therefore belong to no chapter's block (`a-test-belongs-to-its-line-range`).
That also unseats *the `Vacancy` arm … is held **only** from chapter 11*.

**A third count claim, not flagged in advance, was wrong in two ways.** The arm-10
paragraph read *five of `finish_commit`'s six operator-facing refusals have a
test*, and listed *the untracked tree* among *its neighbours in the same
function*. `require_recoverable_grove` is a different function, reached through
`delete_and_commit`; and `finish_commit`'s own operator-facing refusals are
**five**, of which four have a test. The sixth `bail`-shaped arm in that function
is arm 8, which the page has already placed in the environment-failure group. The
finding the paragraph exists for — one untested refusal among siblings that are
all tested, for no reason — is unchanged and now counts correctly
(`uniqueness-and-count-claims-need-enumeration`).

**Every zero re-measured as a zero, and each argument still describes the code.**
Arm 5's dead `.context` is dead for the reason given: `new_finish_leaf(None)`
builds an empty entry, `tree.append` is refused by the library first, and
`a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping`
(`tree_lifecycle.rs:1501`) asserts on the library's *greatest a key can be*
rather than on grove's wording. Arm 6's error arm is unreachable, and the mutant
is its own positive control: a panic on `Slug::new("finish")`'s error path leaves
all 549 passing tests green, including every test that builds a finish leaf. Arms
8, 13 and 14 need a filesystem or a version-control failure nothing here can
arrange; arm 10 needs only a fixture nobody wrote.

**No source defect surfaced, so no defect leaf is owed**, and the corpus freeze
held: no file under `crates/` was changed in this leaf.

**Both re-runs were owed and both were taken.** Arms 1 and 2 name
`driver_lease.rs` tests, so each was run twice; the two pairs of failure sets are
byte-identical, `ps` showed no orphaned `configured-command.sh` children, and no
run came near the 120-second producer timeout.

**Both gates are green.** `book-check --repo . --book docs/walkthroughs/grove-loop
--final --check all` reports *valid: 13 files, 10557 resolved lines, 0 deferred
lines, final=true*, and `bash scripts/check.sh` exits 0 with *all 8 principal
checks pass* over six books, 0 failing — no worse than before, since this leaf
changed no source and no fragment range.

**Chapters 15 and 16's leaf is cut** — `--kind impl`, slug
`ch15-16-configured-command`, per the node brief's decomposition — and its Context
carries what this study learned that it cannot re-derive cheaply.
