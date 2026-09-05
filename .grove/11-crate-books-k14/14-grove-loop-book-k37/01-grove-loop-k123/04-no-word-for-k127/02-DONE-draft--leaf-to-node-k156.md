# leaf-to-node-k156

## Goal

Draft chapter 12 of the `grove-loop` book, *A leaf becomes a node*, over
`tree_lifecycle.rs`'s two `the-key-survives` blocks — `490-695` and `1666-2234`,
775 lines — and prove the prefix through slice `the-key-survives`.

## Context

- Child 2 of 4 of `no-word-for-k127`. The rule is **the key is preserved, because
  the entity that was the leaf becomes the node**: the leaf *file*
  `NN-<kind>--<slug>-k<key>.md` becomes the node *directory* `NN-<slug>-k<key>/`,
  its body renamed in as `BRIEF.md`, and a first child grown atomically so a node
  is never childless.
- Blocks: `decompose-production` (`490-695`: `leaf_decompose`, `decomposable`,
  `promoted`) and `decompose-tests` (`1666-2234`), whose two labelled sections are
  *leaf-decompose* at `1666` and *leaf-decompose: the seam* at `1971`.
- **The test support this block leans on is chapter 11's**, at `1077-1261`, and
  the two sections at `1666` and `1971` are the first consumers of `mknode`,
  `touch_body` and `list`. Chapter 11 reproduces and explains them; this chapter
  names them and does not re-explain.
- **`append_brief_suffix_in_file` is chapter 11's source and chapter 12 is its
  only caller anywhere** — `tree_lifecycle.rs:587`, inside `leaf_decompose`.
  Chapter 11 reproduces it and points forward; the account of *why a freshly
  decomposed brief is retitled at all* is this chapter's.
- **Two of its claims have no observer, established by mutation in
  `a-grove-begins-k155`.** Panicking on the conservative
  `return Ok(())` branch — the *already suffixed, or a custom title* arm — leaves
  all 558 `grove-loop` and `grove-llm` tests green against an 11-failure control,
  so neither the idempotence the doc comment claims nor the *never clobbers a
  custom title* promise is pinned by anything. State it where the helper is
  consumed.
- **The four refusals the structure brief names are a claim about attribution,
  not a count**, and chapter 8's precedent is that three of four refusal tests
  refused somewhere other than their names say. Read the `bail!` texts, match each
  assertion's substring to exactly one, then mutate each arm to a **panic** in a
  workspace copy and diff against an unmutated control run of the same copy. The
  harness `a-grove-begins-k155` used is described in this node's brief.
- The manifest's early-use rows are a floor; enumerate this chapter's own bytes,
  in both the Rust-path and the hyphenated-verb spelling.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  the-key-survives --check all` is valid: 13 files, 6,078 resolved lines, 4,455
  deferred, `final=false`.
- `12-leaf-to-node.md` exists, `README.md` and chapter 11's navigation are
  updated, and the two `the-key-survives` ownership rows read `resolved`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf.

**Say which tree.** `ordinal-fs-tree`'s *leaf* and *node* are not grove's, and
this chapter's whole subject is one word in each glossary meaning different
things at once.

## Decisions (running log)

1. **The structure brief precondition is met and was checked by content, not by
   filename.** `docs/specs/grove-loop-book-structure.md` states all three
   required things quotably: the audience and outcome (*Audience and intended
   outcome*, the what-could-not-move test), the ordered section plan (*Chapter
   sequence*, twenty-one pages, and *Concept and seam responsibilities* §12), and
   what deserves emphasis and what is excluded (*What each chapter's prose owes*,
   *What the book deliberately does not cover*).

2. **The mutation harness needs `cargo build -p grove --bins` before the control
   run, and the node brief's harness does not say so.** Without it
   `CARGO_BIN_EXE_grove` is unset and six further `grove-llm` tests fail in the
   control — 17, not the 11 `a-grove-begins-k155` recorded. With the build the
   control reproduces exactly: 558 tests, 11 failures, all in
   `crates/grove-loop/tests/prompt.rs`. Promoted to the node brief.

3. **A message-preserving `panic!` is not a mutation for an out-of-process
   observer, and the first pass over all sixteen arms was void because of it.**
   `bail!(…)` → `panic!(…)` keeps the format string, so a `grove-llm` integration
   test that shells out and asserts on *stderr substrings* stays green: the panic
   prints the same words the refusal did. Replacing the **whole macro call** with
   `panic!("MUTANT-<arm>")` found three observers the first pass missed —
   `decompose_rejects_a_brief` and `decompose_rejects_a_retired_leaf`
   (`crates/grove-llm/tests/leaf_ops.rs`) and
   `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind`
   (`crates/grove-llm/tests/session_kind_tree.rs`). This is the dual of the
   node's *a reworded `bail!` is not a mutation* and is promoted with it.

4. **A flaky test reads as a newly-attributed observer, and only a re-run
   separates them.** The first `a01` reading credited
   `a_second_driver_refuses_before_tree_access_or_launch` and
   `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`
   (`crates/grove-loop/tests/driver_lease.rs`) to the grove-root refusal. Both are
   120-second wedged-producer timeouts under load; the re-run of the same mutant
   attributes the arm to the sweep alone. Every observer named on the page
   survived a second run of its mutant, or is named by a test whose fixture
   reaches the arm.

5. **The four refusals the structure brief names each refuse where their name
   says**, which is the opposite of chapter 8's result and is stated as such.
   The attribution defect in this block is elsewhere: `decompose_refuses_a_foreign_file`
   never enters `decomposable` — it is refused in `task_tree::target`, chapter 6's
   function — and `an_absent_grove_root_is_refused_by_the_opening` says in its own
   name that it never reaches the verb.

6. **One floor row is owed and it is `verbs::leaf_decompose`.** Enumerating both
   spellings over `490-695` and `1666-2234` against every symbol declared in a
   later chapter's blocks leaves exactly one uncovered later-owned referent: the
   public verb, named in this block's own doc comment in the hyphenated
   `leaf-decompose <leaf-path> <first-child-slug>` spelling, which is chapter 10's
   `verbs::leaf_add` precedent exactly. `tree_lifecycle::leaf_decompose` already
   has a row (first use chapter 6) and moves to `explained` here.

7. **`cargo doc` is clean over this block, and the block's summaries are single.**
   Thirty warnings for the crate, none naming `tree_lifecycle.rs`; the five
   intra-doc links in `490-695` all resolve. Checked against the render rather
   than the source: the module index's one-line description for `leaf_decompose`,
   `decomposable` and `promoted` each equals that item's first rendered
   paragraph, where `grove_name`'s — chapter 11's, held by
   `welded-grove-name-summary-k160` — visibly does not.

8. **One defect leafed, one adjudicated in place.** `crates/grove-loop/src/tree_lifecycle.rs`
   lines 538–541 justify a slug precondition the verb no longer performs — the
   line they annotate is a bare rebinding of an already-typed `&Slug` — and the
   same file refutes them 1,386 lines below, in
   `decompose_cannot_be_reached_with_a_bad_child_slug`'s own doc comment.
   `stale-slug-precondition-comment-k162` holds the fix, `leaf-insert`ed ahead of
   `architecture-residue-k75` beside the node's other deferred comment fixes; the
   page adjudicates it under *A precondition that moved into the type*. Nothing
   else in the block needed a leaf: `cargo doc` is clean over it and the three
   items' rendered summaries are single paragraphs.

9. **Sixteen arms measured; six observed, ten held by nothing.** Six of the ten
   are `promoted`'s library-contract checks and two are I/O failures on the
   retitle — neither a gap a fixture could close. The tenth is not:
   `refuse_finish_kind` at line 560 guards `--kind finish` as an *override*, is a
   different arm from `decomposable`'s finish-leaf refusal at 622, is reachable
   through `grove-llm`'s `cmd_leaf_decompose` (whose only prior check,
   `require_declared`, asks about the launch configuration and not about
   reservation), and survives being deleted outright with all 558 tests green.

10. **Three uniqueness claims were written, then failed enumeration and were
    rewritten.** *The only workspace call to the library's `promote` outside
    `leaf_decompose`* was false — eleven `.promote(` call sites exist, nine of
    them `ordinal-fs-tree`'s own binary and tests; the true claim is two of
    eleven are grove's. *The largest inline-test block outside `tree_lifecycle.rs`'s
    neighbours* was an unnecessary hedge — 569 lines is the largest in the book,
    against chapter 17's 564 and chapter 13's 491. And *`promote` refuses three
    ways* conflated the three refusals promotion **owns** with the six its
    decision can raise. Each was caught by counting rather than by reading.

11. **The support-block callback is an enumeration, not an impression.** Chapter
    11's *Five of eighteen* has a counterpart here: this block calls twelve of the
    eighteen, nine of them ones chapter 11's own tests never touched, and only
    `worktree`, `root_init_at`, `grow_leaf` and `guard_at` are named nowhere in
    the 569 lines.

12. **`scripts/check.sh` is red on `book-check` alone, and this is what that
    means here.** Seven of eight pass — `cargo fmt`, `shellcheck`,
    `cargo clippy`, `plugin install`, `conformance`, `conformance suite` and
    `cargo test --locked --workspace`; the script reports `FAILED — 1 of 8`.
    Five of the six books are green at `final=true`; `grove-loop` is the one
    failing, and every one of its diagnostics is the prefix shape rather than a
    defect: nine `M101` for the unwritten chapters 13 to 21, twelve `F003` for
    the `defer` directives a `--final` run rejects by design, one `F009` for a
    `deferred` ownership row, sixteen `F009` for `pending` early-use rows whose
    owners are unwritten, and two `M103` for the navigation and contents that a
    scoped prefix deliberately leaves in final-page form. **No diagnostic names a
    row this leaf wrote as `resolved` or `explained`**, checked by mapping every
    flagged `source-index.md` line back to its state.

13. **The workspace suite wedged for thirty-one minutes and it was contention,
    not a defect.** `crates/grove/tests/loop_driver.rs` sat at 0.2 seconds of CPU
    with three unreaped children, blocked on a pipe read, while an unrelated
    `cargo` build saturated the machine. Run alone it passes eleven tests in
    forty seconds. The first `check.sh` run was killed and re-run clean rather
    than read through the wedge — an instrument adjusted mid-reading has not read
    anything, and one adjusted by the machine around it has not either.

14. **No `copy-edit` leaf is cut here, and the condition that says so is in the
    brief chain rather than in a judgement.** The draft stage is the node
    `grove-loop-k123`, not this leaf; it has five live children left — `03` and
    `04` under `no-word-for-k127`, and `05`, `06` and `07` under `k123` itself —
    so the stage has not ended. `grove-loop-k123`'s *Done when* assigns the act
    explicitly: *the last child's last act is `grove-llm leaf-add
    grove-loop-book-k37 grove-loop --kind copy-edit`, unless a live later sibling
    under `grove-loop-book-k37` already holds that stage*. This is not a stage
    skipped on a judgement that it would find nothing; it is a stage that is not
    yet due.
