# no-word-for-k127 — brief

## Goal

Draft Part III of the `grove-loop` book — chapters 11 to 14, the whole of
`crates/grove-loop/src/tree_lifecycle.rs` (2,725 lines, nine blocks) — and prove
the prefix through slice `the-tree-deletes-itself`.

## Context

- Draft stage, child 4 of 7 of `grove-loop-k123`. The structure brief is
  `docs/specs/grove-loop-book-structure.md`; the four chapters are its sections
  *11 · A grove begins* through *14 · Finishing*, and the mapping is its
  *Top-level ownership blocks* and *`tree_lifecycle.rs` four ways in nine
  blocks*.
- **Expect this to decompose, one child per chapter.** Four chapters at 612, 775,
  808 and 530 lines, over the largest root in the corpus.
- The blocks, in file order and with their owning chapter: `1-331` (14),
  `332-489` (11), `490-695` (12), `696-1012` (13), `1013-1076` (11),
  `1077-1466` (11), `1467-1665` (14), `1666-2234` (12), `2235-2725` (13). **The
  file opens on finishing and the book closes on it**: chapter 14 owns `1-331`
  and chapter 11 owns `1013-1076`. `1467-1665` — the `materialize_finish` and
  `transition_to_current` tests — is chapter 14's, not chapter 11's.
- **The prose obligation is *supply the claim*, at the largest scale in the
  book.** 1,649 of these lines are the inline test module at 15% prose. Every
  reproduced test states the property it establishes **and what would have to be
  true for it to pass while the property was broken**.
- Chapter 11 owns the shared test-support block the next two chapters' tests use,
  and the three body-writing helpers shared with chapter 12.
- The brief names the pairs that carry each chapter's rule:
  `root_init_creates_the_whole_grove_through_one_store_operation` with
  `a_refused_grove_leaves_no_root_behind` (11);
  `decompose_converts_leaf_file_to_node_dir_preserving_the_key` and the four
  refusals (12); `retire_adds_done_infix_keeping_position_and_key` with
  `retire_does_not_rewrite_the_header_or_body`, and
  `pruning_a_node_takes_one_guard_per_mark` as the cost the atomicity is *not*
  paid with (13); `materialize_finish_writes_a_handle_that_matches_its_own_filename`
  and `a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping` (14).
- **Chapter 14's observable end is the grove ceasing to exist**, which is why the
  book is not illustrated from a live `.grove/`: `finish-commit` removes it.
- Chapter 13's thesis is *the tree's shape is the only state* — the spine the
  whole brief rejected as the book's, kept as this chapter's and contrasted with
  chapter 16's in a sentence chapter 1 has already written.
- Chapter 1's cast rows owned by `never-mistaken-for-finished` move to
  `explained` when chapter 11 lands.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  the-tree-deletes-itself --check all` is valid: 13 files, 7,416 resolved lines,
  3,117 deferred, `final=false`.
- Chapters 11–14 exist, contents and navigation are updated, and the nine
  `tree_lifecycle.rs` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Decomposition

**Four children, one per chapter.** The manifest's nine `tree_lifecycle.rs`
blocks already partition the root along exactly the chapter boundary, so the seam
is the chapter boundary and nothing else. The order is forced: `--through` proves
a canonical prefix, so a child cannot prove its slice before its predecessor's
page exists.

| # | Child | Chapter | Slice | Blocks | Owned lines | Cumulative resolved | Deferred |
|---:|---|---:|---|---|---:|---:|---:|
| 01 | `a-grove-begins-k155` | 11 | `never-mistaken-for-finished` | `grove-beginning`, `body-helpers`, `root-init-tests` | 612 | 5,303 | 5,230 |
| 02 | `leaf-to-node-k156` | 12 | `the-key-survives` | `decompose-production`, `decompose-tests` | 775 | 6,078 | 4,455 |
| 03 | `outcomes-k157` | 13 | `marked-in-place` | `outcomes-in-place`, `retire-and-prune-tests` | 808 | 6,886 | 3,647 |
| 04 | `finishing-k158` | 14 | `the-tree-deletes-itself` | `finish-transition`, `finish-tests` | 530 | 7,416 | 3,117 |

Each child's slug is its page id, which is the convention `orientation-k124` set
and both `the-grammar-k125` and `the-walk-k126` kept. None of the four collides
with an existing entry, checked with `resolve` before the cut. Every child leaves
`scripts/check.sh` red on `book-check` alone and says so.

**Every child owns production its own chapter does not consume, and that is this
part's shape rather than a mis-cut.** The file opens on finishing and the book
closes on it, so chapter 11's `RootShape` is read only by chapter 14's
`transition_to_current`, chapter 11's `append_brief_suffix_in_file` is called only
by chapter 12's `leaf_decompose`, and chapter 11's `default_root_slug` has exactly
one caller anywhere and it is chapter 14's. Each page owes a forward pointer, and
a child that silently narrows its block to what its own verb calls will leave
lines unexplained.

## Found while drafting

**The mutation harness, and the environment baseline it needs.** Established by
`a-grove-begins-k155`; the next three children run the same procedure over their
own arms. Copy the workspace to a scratch directory — `crates/`, `.cargo/`,
`testing/`, `plugins/`, `scripts/`, `docs/` **and every root-level file**, because
`crates/grove-llm/tests/composition_guidance.rs` does `include_str!` on
`../../../CONTEXT.md` and the copy will not compile without it. Run
`cargo test --no-fail-fast -p grove-loop -p grove-llm`, keep the **per-test**
lines rather than the totals, and sort them: that is the control. The copy is not
a jj repository, so **eleven tests fail in the control before any mutation** — all
of `crates/grove-loop/tests/prompt.rs` — and a mutant is read as the `comm`
difference against that control, never against zero. 558 tests run in all.

**Three corrections to that harness, all found by `leaf-to-node-k156`, and each
one changes what a reading means.**

- **`cargo build -p grove --bins` before the control run.** Without it
  `CARGO_BIN_EXE_grove` is unset — the `grove` binary belongs to a third package —
  and six further `grove-llm` tests fail on a missing binary, making the control
  seventeen rather than eleven. A control that is wrong in that direction **hides
  observers**, because a test already red cannot go redder.
- **A message-preserving `panic!` is not a mutation for an out-of-process
  observer.** `bail!(…)` → `panic!(…)` keeps the format string, so a `grove-llm`
  integration test that shells out and asserts on *stderr substrings* stays green:
  the panic prints the same words the refusal did and the exit status is non-zero
  either way. Replace the **whole macro call** with `panic!("MUTANT-<arm>")`
  instead. Doing so found three observers the message-preserving pass had recorded
  as absent — `decompose_rejects_a_brief` and `decompose_rejects_a_retired_leaf`
  (`crates/grove-llm/tests/leaf_ops.rs`) and
  `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind`
  (`crates/grove-llm/tests/session_kind_tree.rs`). This is the dual of *a reworded
  `bail!` is not a mutation*, and it bites harder, because the whole `grove-llm`
  suite is out of process.
- **A flaky test reads as a newly attributed observer, and only a re-run separates
  them.** The first reading of chapter 12's grove-root arm credited
  `a_second_driver_refuses_before_tree_access_or_launch` and
  `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`
  (`crates/grove-loop/tests/driver_lease.rs`) to it. Both are 120-second
  wedged-producer timeouts under machine load; the re-run attributed the arm to
  one test. **Re-run any mutant whose newly-failing set names a `driver_lease.rs`
  or `prompt.rs` test** before writing the attribution down.

**And the whole workspace suite can wedge under load without failing.**
`crates/grove/tests/loop_driver.rs` sat for thirty-one minutes at 0.2 seconds of
CPU, blocked on a pipe read with three unreaped children, while an unrelated
`cargo` build saturated the machine; run alone it passes eleven tests in forty
seconds. A `scripts/check.sh` read through that is not a reading — kill it and
re-run.

**Two things that make a mutation reading a lie, and both were hit here.** A
mutant that fails to compile prints no per-test lines at all, so it reads as *0
newly failing* — exactly like a clean result; **check the mutant's test count
equals the control's** before crediting it. And a panic inserted at the *call*
rather than on the *arm* measures reachability instead of observation: panicking
unconditionally in `root_init` reddens 41 tests and says nothing about whether
its refusal is held. The arm form is
`if <refusal>.is_err() { panic!(...) }`, or a panic in place of the `bail!`.

**Seven of ten refusal and fallback arms in chapter 11's production are held by
nothing**, against a control mutation of the three that are. Unobserved:
`refuse_finish_kind` in `root_init`; the store-`initialize` failure; both
`initialize_grove` `bail!`s; the `task_grow::allocated` key-disagreement guard;
`grove_name`'s `"grove"` fallback; and `append_brief_suffix_in_file`'s
conservative *already suffixed, or a custom title* return. Observed:
`RootShape::Taskless` (one test), `ATree` (two) and `Unrecognised` (one). **Two of
those four observers sit in chapter 11's block and two in chapter 14's**, so the
evidence is split rather than exported: `ATree` is pinned from both sides,
`Taskless` only from chapter 11, and `Unrecognised` only from chapter 14.

**Two independent spellings of the default root slug, and nothing holds them to
each other.** `DEFAULT_ROOT_SLUG` is `"plan"` at `tree_lifecycle.rs:56`;
`crates/grove-llm/src/cli.rs:327` writes `#[arg(default_value = "plan")]`.
Changing the constant to `"mutant"` reddens exactly one test —
`transition_initializes_an_absent_grove_under_one_exclusive_guard` — while
`root_init_default_slug_is_plan`, which drives the CLI, stays green. So each
literal is pinned by its own test and their **agreement** is pinned by none.
`default-root-slug-two-spellings-k159` holds the decision and is deferred behind
the book.

**`default_root_slug`'s doc comment names a caller it does not have.** It opens
*The slug `root-init` uses when nobody supplied one*, and `root_init` never reads
it — the verb takes `slug: &Slug` and the fallback lives in clap. Its one caller
anywhere is `transition_to_current` at line `82`, which is the comment's second
clause and is correct. Chapter 11 adjudicates on the page; the fix rides with
k159.

**`grove_name` carries two summary sentences welded into one paragraph, and
`cargo doc` reports nothing.** Lines `1013-1017` are a single `///` run with no
blank line between an older summary and a newer one, so rustdoc renders both as
the first paragraph and the module index prints the pair as the function's
one-line description — confirmed from the rendered
`target/doc/grove_loop/tree_lifecycle/fn.grove_name.html`, which has two `<p>`
elements where every neighbour has a summary and a body. **`cargo doc --no-deps
--document-private-items` emits no warning for `tree_lifecycle.rs` at all**,
against thirty elsewhere in the crate, so this is the sixth instrument's blind
spot in a new form: not a `//` header this time but a *correctly attached* doc
comment with two summaries. A five-line reflow fixes it **inside the existing
line count**, which is what `welded-grove-name-summary-k160` holds.

**A cited pair where one member is the other member with a different name.**
`a_refused_grove_leaves_no_root_behind` (1430) and
`root_init_rejects_a_bad_slug_without_leaving_a_grove_behind` (1318) call the same
helper with the same `"Bad Slug"` and make the same two assertions; only the
failure messages differ. The structure brief names the first as the negative of
`root_init_creates_the_whole_grove_through_one_store_operation`, and its doc
comment claims the store's takedown — but `root_init_at` validates the slug on
line 1211, before `write_or_vacancy` on 1213, so neither test enters
`initialize_grove` at all. **The unwinding is asserted by nothing in this
workspace**, confirmed by the mutation study above: every error path inside
`initialize_grove` is unobserved. `refused-grove-test-overclaims-k161` holds the
decision — earn the claim with a test that makes `initialize` fail after the root
exists, or narrow the comment and reconcile the structure brief.

**This is the third instance of one lesson in this book, and it is now the
node's.** Chapter 10 met it as *a cited test can pin a narrower claim than its
citation, and the helper is where that hides*; chapter 11 met it twice more, in
the same helper. **`root_init_at`'s own doc comment states the order that causes
it** — *read the slug, resolve the vacancy, then scaffold, in that order, which is
what the tests below about a refused slug are actually asserting* — so the fixture
told the truth and three test names did not. Every remaining child reading a test
that refuses **must read the helper's order before believing what the refusal
proves**, and chapters 12 to 14 all lean on the same support block.

**A test belongs to the block its line number falls in, not to the verb it
exercises.** The single worst error in chapter 11's draft, caught by its reviewer:
`one_process_creating_and_reading_a_grove_never_waits_on_itself` calls
`transition_to_current` and asserts `CurrentTransition::AlreadyCurrent`, so the
draft filed it under chapter 14 — but it sits at line 1385, inside `root-init-tests`
(`1077-1466`), which is chapter 11's. That single mis-filing produced three wrong
sentences, including an *only* and a *three of four*. **Every child owes a line-number
check against `walkthrough.toml`'s `[[block]]` ranges before attributing a test to a
chapter**, and the four chapters of this part are the ones where it is easiest to get
wrong, because the file's concerns and the book's chapters are deliberately
inverted.

**`refuse_token` reports the first offending character, so a refusal message names
the first violation and not the interesting one.** The draft wrote that `"Bad Slug"`
is refused *for its space*; it is refused for its capital `B`, because the
character-class arm uses `.chars().find(...)` and the space is never reached. The
arms run in order — reserved set, leading or trailing dash, `--` separator, then the
character class — and chapters 12 to 14 all reproduce tests that assert on refusal
substrings. **Read which arm fires, not which rule the fixture was chosen to
break.**

**A classification table must state what its columns mean, or the author
miscounts them.** Chapter 11's *five of eighteen* table was right in its membership
and wrong in its heading: a column labelled *first called by a later chapter* held
three helpers whose first caller is inside the support block itself. The membership
question that is actually load-bearing — *which helpers does this chapter's own test
section call?* — is answerable by enumeration; *first caller* is a different question
and was never the one being asked.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline.

**The two glossaries collide here.** `ordinal-fs-tree`'s *leaf* and *node* are
not grove's, and its *ordinal* is grove's *position* (`CONTEXT-MAP.md`). These
four chapters speak of both trees in nearly every paragraph; say which tree is
meant, sentence by sentence.

## Decisions (running log)

1. **Decomposed one child per chapter rather than drafting all four.** 2,725
   source lines over the largest root in the corpus, against the 436 that became
   chapter 1's 1,039 markdown lines in one session; `the-grammar-k125` decomposed
   at 1,714 over three chapters and `the-walk-k126` at 2,541 over six, so this is
   the largest of the three and the precedent is unanimous. The manifest's nine
   blocks already partition the root along the chapter boundary, so any other cut
   would leave a child unable to prove itself with `--through`.
