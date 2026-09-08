# default-root-slug-two-spellings-k159

## Goal

Reconcile the two independent spellings of the default root slug — grove-loop's
`DEFAULT_ROOT_SLUG` and the CLI's `#[arg(default_value = "plan")]` — so their
agreement is held by something, and correct `default_root_slug`'s doc comment,
which names a caller it does not have. Inside both files' frozen line counts.

## Context

- **Two literals, each pinned, their agreement pinned by nothing.**
  `crates/grove-loop/src/tree_lifecycle.rs` line 56 is
  `const DEFAULT_ROOT_SLUG: &str = "plan";`.
  `crates/grove-llm/src/cli.rs` line 327 is `#[arg(default_value = "plan")]`,
  with line 326's doc comment saying *Default: `plan`* a third time in prose.
- **Established by mutation**, not by reading: changing the constant to
  `"mutant"` in a workspace copy reddens exactly one test of 558 —
  `tree_lifecycle::tests::transition_initializes_an_absent_grove_under_one_exclusive_guard`,
  which asserts the picked leaf is `01-requirements--plan-k1.md` — while
  `root_init_default_slug_is_plan` in `crates/grove-llm/tests/root_init.rs`
  drives the binary and stays green. Diffed against an 11-failure control run of
  the same copy; the harness is in `no-word-for-k127`'s brief.
- **The two defaults are reached by different doors and both are user-visible.**
  The constant is read only by `transition_to_current` (line 82), the driver's
  own scaffold, which creates a grove when the loop starts against a worktree
  that has none. The clap default is what `grove-llm root-init` with no argument
  uses. A grove scaffolded either way should carry the same first leaf, and
  today that holds by coincidence.
- **The doc comment names a caller it does not have.** Lines 351–352 open *The
  slug `root-init` uses when nobody supplied one, and the only slug the driver's
  own scaffold can use.* `root_init` takes `slug: &Slug` and never falls back;
  `grep -rn --include='*.rs' 'default_root_slug\|DEFAULT_ROOT_SLUG' crates/`
  returns the definition, the constant and **one** call site, line 82. The second
  clause is exactly right and the first is not.
- **Found by `a-grove-begins-k155`** while drafting chapter 11, which adjudicates
  both on the page at `11-a-grove-begins.md#the-value-nothing-holds`.

## Done when

- The agreement is held by something a change would break — the CLI reading the
  crate's constant, or a test asserting the two are equal — rather than by two
  literals that happen to match. A mutation of whichever spelling survives
  reddens at least one test on **both** paths.
- `default_root_slug`'s doc comment names `transition_to_current` as its caller
  and drops the `root-init` clause, or states where the CLI's default actually
  lives.
- **`crates/grove-loop/src/tree_lifecycle.rs` is still exactly 2,725 lines** and
  `crates/grove-llm/src/cli.rs` its own count, so no ownership range, manifest
  `lines` value or fragment range moves. If a fix cannot fit, this leaf says so
  and the ledgers move in the same commit.
- `11-a-grove-begins.md` reproduces the new bytes and its adjudication is
  rewritten to the repaired text; any `concept-index.md` entry naming the defect
  follows.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  at whatever slice the book is proved at when this runs, and every other book
  the commit touched is green.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Deferred behind the `grove-loop` book**, with `unresolved-doc-links-k151`,
`unreachable-root-clause-k152` and `grow-header-stale-helper-k154`, and for the
same reason: the bytes are reproduced by a finished page and the freeze rule
wants one commit carrying source, ledgers, pages and a green validator. Until
then chapter 11's adjudication stands as the record.

**This defect straddles two books, and both are already written over it.**
`crates/grove-llm/src/cli.rs` is the `grove-llm` book's corpus, and line 327 is
inside fragment `«args-root-init»` (`lines="323-330"`, owner `before-the-lock`)
at `docs/walkthroughs/grove-llm/04-growing-the-tree.md:261`. So a commit that
touches the clap default carries **two** books' pages, ledgers and validator
runs, and `grove-llm`'s is already at green `--final`. Prefer the fix that
changes only `tree_lifecycle.rs` if one exists — a test asserting the two
spellings agree can live in `grove-llm`'s test directory, which is evidence and
not corpus in either book.

## Decisions (running log)

1. **The agreement is held by a test, not by deleting the second spelling.** The
   Notes' preferred route was taken: `crates/grove-llm/src/cli.rs` is byte-for-byte
   unchanged, so the `grove-llm` book's pages, ledger and `--final` validation are
   untouched and the commit carries one book. The observer is
   `both_scaffolding_doors_name_the_first_leaf_the_same` in
   `crates/grove-llm/tests/root_init.rs` — evidence in neither book's corpus. It
   runs `root-init` with no argument against the file's existing jj fixture, calls
   `grove_loop::driver::transition_to_current` against a bare temp directory, and
   asserts the two first leaves carry the same filename. `grove-loop` is a normal
   dependency of `grove-llm`, and `driver` is `pub`, so no visibility changed
   either.
2. **Measured, not asserted, and the numbers moved since the book was written.**
   In a workspace copy (`crates/`, `Cargo.*`, `.cargo/`, `testing/`, `plugins/`,
   `.claude-plugin/`, `docs/`, `scripts/` and the root markdown `include_str!`
   targets — without the last group `composition_guidance.rs` fails to *compile*
   and the run reads clean), `cargo test -p grove-loop -p grove-llm` is **559**
   tests before this leaf and **560** after, with **10** environmental failures,
   all `crates/grove-loop/tests/prompt.rs`, because the copy is not a jj
   repository. That is ten and not the eleven chapter 11 records: the copy carries
   the marketplace, so `the_namespace_is_the_shipped_plugin_entrys_declared_name`
   passes. Against that control, `DEFAULT_ROOT_SLUG` → `"mutant"` reddens **two**
   (`transition_initializes_an_absent_grove_under_one_exclusive_guard` and the new
   test) and `#[arg(default_value)]` → `"mutant"` reddens **eight** (the new test,
   `root_init_default_slug_is_plan`,
   `after_root_init_pick_returns_the_new_leaf_not_done`,
   `root_init_creates_root_brief_and_first_requirements_leaf`,
   `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`,
   `concurrent_root_initializers_wait_before_observing_or_creating_the_grove`,
   `reader_through_a_symlink_alias_waits_for_the_same_worktree_identity`,
   `root_init_scaffolds_a_grove_in_a_jj_native_tree`). **Their intersection is one
   test and it is the new one** — which is the *Done when* clause, and also the
   measurement behind the page's uniqueness claim.
3. **The fan-out was five surfaces, not the one the task file named.** Enumerating
   from `walkthrough.toml` rather than from the task file found the second fragment
   over the same function's neighbourhood. The changed doc comment is inside
   `«grove-beginning-default-slug»` (`351-356`, chapter 11) only — lines 54–57's
   `«finishing-default-slug»` was left alone, because the *constant's* comment was
   never the defect — but chapter 14's prose **quotes the old opening** and states
   the agreement is unheld, so it changed anyway. Five surfaces in all: chapter
   11's fragment bytes, its `#the-value-nothing-holds` section, its *only observer*
   paragraph and its *What could not move* policy paragraph; chapter 14's
   *transition* prose; chapter 21's `#stated-twice` table row, ladder sentence and
   *three of the four are held by a test* tally; and five `concept-index.md`
   entries (four rewritten, one added).
4. **Chapter 21's ladder shortened, and the page says so rather than restating a
   range it no longer has.** The row's last column was **Nothing**; it is now *One
   test, and it postdates the reading*, and *nothing, a comment, one test, nine
   tests* is now a floor of one comment. The neighbouring tally moved from two of
   four held by a test to three.
5. **The `558` figures across seven chapters were left alone.** Each is a record
   of a mutation run performed when its chapter was drafted, not a claim about the
   suite's current size, and chapter 11's sentence keeps its own 558 in the past
   tense beside the re-run's 560 with the difference stated.
6. **The heading was renamed and its anchor kept.** `### The value nothing holds`
   is now *The value chosen twice, and what holds it*; `<a id="the-value-nothing-holds">`
   stands, so chapter 14's citation and the four `concept-index.md` entries still
   resolve. `book-check` matches the explicit anchor rather than a heading slug,
   and this book already carries anchors that differ from their headings.
7. **`tree_lifecycle.rs` is still exactly 2,725 lines** and
   `crates/grove-llm/src/cli.rs` still exactly 944, so no ownership range, manifest
   `lines` value or fragment range moved. All six books are green at
   `--check all --final`.
8. **The two new prose links are wrapped, so `book-check` does not check them —
   and that is `wrapped-link-labels-unchecked-k186`, already cut.** Writing the
   first of them on one line turned the book red (`M201 … resolves to missing
   repository file docs/walkthroughs/grove-loop`), which is k186's second defect:
   a bare `#anchor` resolves to the book directory. Both new links were therefore
   written in the qualified `page.md#anchor` form, which is correct under either
   answer k186 lands on. The blind spot was re-confirmed with a control while
   here — breaking `01-orientation.md:178`'s wrapped `#the-two-openings` to
   `#no-such-anchor-at-all` leaves the book **green** at `--check all --final` —
   and enumerating the whole set finds **17** wrapped-label links across five
   books, not the two k186's *Context* names. No leaf was added: k186 owns it, and
   its `Done when` already covers the wider set. The two anchors this leaf wrote
   were verified by hand, since no instrument will.
9. **One test-binary stall was observed and is not this leaf's.** A `check.sh` run
   sat on `Running tests/root_init.rs` for seven minutes; so did one direct run of
   the same binary. In both, the harness's own `running 10 tests` header never
   printed, so nothing had begun executing — the stall is before the first test,
   not inside one. It did not recur across twelve direct runs of that binary, a
   full `cargo test --locked --workspace` (exit 0, no failures, `root_init.rs` ten
   passed in 1.07s), or the final gate. Both occurrences were the first exec of a
   freshly linked binary on a machine also building another workspace.
10. **The new intra-doc link costs no rustdoc warning.** `cargo doc --no-deps
    --document-private-items -p grove-loop` still reports **26**, the count
    chapters 11, 15 and 16 state, and none of them names `default_root_slug` or
    `transition_to_current`. A private item linking to a `pub(crate)` one in the
    same module resolves; the figure did not have to move. `bash scripts/check.sh`
    is `all 8 principal checks pass`, exit 0 — the same as before this leaf — and
    all six books are green at `--check all --final`.
11. **The leaf's one in-session reviewer was spent on the changed pages, and it
    was right about the instrument.** Decision 2's mutation numbers were wrong:
    `cargo build -p grove --bins` ran *before* the mutations and not after, so
    every mutated run reused a `grove` binary linked from unmutated source. The
    constant mutation reddens **three**, not two — the third is
    `bare_scaffolding_is_anchored_before_the_configured_command_inherits_git_context`
    in `crates/grove-loop/tests/driver_lease.rs`, which drives that binary against
    a worktree holding no grove. Re-measured on a **freshly rebuilt copy** (the
    first was found dirty, its `cli.rs` still mutated, so it was discarded rather
    than reverted) with `cargo build -p grove --bins` after each edit: control 560
    / 10 environmental failures; constant → 3 newly red; clap → 8 newly red;
    **intersection exactly `both_scaffolding_doors_name_the_first_leaf_the_same`**,
    and empty with that test removed. The *Done when* clause holds; only the
    counts had to move, and chapter 11 now states the rebuild step as part of the
    harness because omitting it is what produced the original *exactly one test*.
12. **The chapter-11 claim that `DEFAULT_ROOT_SLUG` has one observer in this crate
    was false, and predates this leaf.** It has two — the lock-count test and the
    driver-lease one above. The page now names both and says both are blind to the
    CLI's literal, which is the same finding stated over a larger set.
13. **The fan-out was wider again: nine surfaces, and the `grove-llm` book is in
    the commit after all.** Beyond decision 3's five, the reviewer found four more
    that no validator reads. `15-the-verbs.md` said
    `driver::transition_to_current` has *exactly one call site in the whole
    workspace* and that the module's publicity is *earned by `materialize_finish`
    alone* — the new test is a second call site, and one from **outside the
    crate**, which is what that `pub` buys; the section is rewritten rather than
    rewritten — and rewritten twice: the first attempt claimed the header's
    justification *now holds for both operations*, which overclaims. The header's
    reason is that **the verb suite** needs the state; a second package's test is
    not that suite, so the reason is still earned by `materialize_finish` alone.
    What changed is that the `pub` is spent at all. `20-the-loop.md` said *called at line 243 and nowhere else in the
    workspace*, and now carries the same *evidence and not a root* shape it
    already used for `materialize_finish`. And
    `docs/walkthroughs/grove-llm/07-what-order-holds.md` **reads
    `crates/grove-llm/tests/` twice** — a console transcript giving `root_init.rs`
    nine tests, and a sentence totalling two hundred and twenty-nine integration
    tests — so both moved by one, to ten and two hundred and thirty. The
    transcript's binary hash was left alone: it identifies a build, not a claim,
    and its sibling hashes are all from the original capture. **This falsifies the
    premise that a test in that directory is free**, so chapter 11's and chapter
    21's cost paragraphs were rewritten to state the price rather than deny it.
14. **One pre-existing error was corrected because this leaf rewrote its
    sentence.** `14-finishing.md` said `default_root_slug` is *sixty lines below
    in the file*; it is at line 351 and the constant at 56, so 295. Replaced with
    the line number, which does not have to be maintained as a distance.
