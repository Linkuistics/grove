# stale-slug-precondition-comment-k162

## Goal

Rewrite the four-line comment at `crates/grove-loop/src/tree_lifecycle.rs` lines
538 to 541, which justifies a slug precondition `leaf_decompose` no longer
performs, inside the file's frozen 2,725-line count — and reconcile the page that
reproduces the changed bytes.

## Context

- **The defect.** Lines 538 to 542 are:

      // Grove's own precondition, before the tree is even observed, so a bad slug
      // leaves the leaf un-decomposed. It could sit inside the guard with the rest
      // of them; it stays out here because it needs nothing from the tree, and
      // refusing without taking an exclusive lock is strictly kinder.
      let child_slug = first_child_slug;

  Line 542 refuses nothing. It is a rebinding of a parameter already typed
  `&Slug`, and a `Slug` cannot be built from text the grammar disclaims. There is
  no check here to sit inside the guard or outside it.
- **The file is its own refutation, 1,386 lines further down.**
  `decompose_cannot_be_reached_with_a_bad_child_slug`'s doc comment (lines 1,924
  to 1,931) records the change: *the claim used to be about ordering: the slug was
  validated before the rename … Since `loop-crate-verbs-k21` the verb takes a
  [`Slug`], so the text is read by the type that owns it and a bad slug never
  reaches a tree at all.* Two passages of one file disagree about which mechanism
  holds one claim, and the later one is right.
- **The claim itself is still true and must survive the rewrite.** A bad child
  slug does leave the leaf un-decomposed, and it does cost no exclusive lock —
  because the `Slug` was constructed by the caller before `write` was ever called.
  What has to go is *Grove's own precondition* and *refusing without taking an
  exclusive lock*, which describe code that is not there.
- **No instrument reports it.** The comment is `//`, not `///`, so
  `cargo doc --no-deps --document-private-items` never sees it — thirty warnings
  across the crate, none for `tree_lifecycle.rs`. This is
  `grow-header-stale-helper-k154`'s blind spot in a third form: not a `//` header
  with a bodyless heading and not a welded summary, but a `//` rationale outliving
  its code.
- **A fix fits in five lines**, which is what keeps the count frozen: lines 538 to
  542 stay five lines and nothing after them moves. One shape that fits is to say
  that the slug was validated by its type at the caller, so the refusal costs no
  lock — dropping the *it could sit inside the guard* clause, which is about a
  choice nobody makes any more.
- **Found by `leaf-to-node-k156`** while drafting chapter 12, which reproduces the
  block and adjudicates it on the page under *A precondition that moved into the
  type, and the comment left behind*.

## Done when

- Lines 538 to 542 describe the code that is there: the precondition is the
  `Slug` type's, discharged at the caller, and no lock is taken to refuse.
- **`crates/grove-loop/src/tree_lifecycle.rs` is still exactly 2,725 lines**, so
  no ownership range, manifest `lines` value or fragment range moves.
- `12-leaf-to-node.md` reproduces the new bytes and its adjudication is rewritten
  to describe the repaired comment; the two `concept-index.md` entries naming the
  defect follow.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  at whatever slice the book is proved at when this runs.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Deferred behind the `grove-loop` book**, with `unresolved-doc-links-k151`,
`unreachable-root-clause-k152`, `grow-header-stale-helper-k154`,
`default-root-slug-two-spellings-k159`, `welded-grove-name-summary-k160` and
`refused-grove-test-overclaims-k161`, for the same reason: the bytes are
reproduced by a finished page.

**Do not delete the comment.** Its second clause is the true shape of the
function and chapter 12 leans on it: the slug is refused before the tree is
observed, which is what makes a bad spelling cost no exclusive lock. What is
stale is the mechanism, not the claim.

## Decisions (running log)

1. **Four comment lines replace four, and the rebinding stays.** Lines 538 to 542
   remain five lines, so `tree_lifecycle.rs` is still 2,725 and no ownership
   range, manifest `lines` value or fragment range moved. The new text opens by
   saying no check happens here, names `Slug` as the precondition, and keeps the
   two clauses the task file protects — *un-decomposed* and *no exclusive lock* —
   attributing both to the caller's construction rather than to this function.
2. **The no-lock claim is the caller's, and it was measured rather than
   inherited.** `crates/grove-llm/src/cli.rs` builds the child slug
   (`slug(&args.first_child_slug)?`) before it opens the tree for writing
   (`writable(&worktree)?`), so a refused spelling really does cost no exclusive
   lock. The comment says *the refusal is the caller's* rather than asserting an
   ordering `leaf_decompose` cannot enforce, since the verb is handed an
   already-taken `Guard`.
3. **Plain backticks, not `[`Slug`]`.** The block is `//`, not `///`, so an
   intra-doc link would render nowhere. That invisibility is also the defect's
   own cause and the chapter now says so.
4. **The section keeps its anchor and loses its framing.** `<a
   id="the-precondition-that-moved-into-the-type"></a>` is unchanged so both
   `concept-index.md` links still resolve; the heading, the opening sentence and
   the *file is its own refutation* paragraph are rewritten to the past tense,
   and `12-leaf-to-node.md`'s later sentence calling the test doc comment a
   refutation of the production comment now says the two agree.
5. **Four surfaces, and the line count froze the rest.** Source literal, the
   `decompose-verb-body` fragment fence, the indented quote in the adjudication,
   and `concept-index.md` line 306 (*A file that is its own refutation* → *A
   comment brought back level with its code*). Line 305 still reads true and is
   untouched. Because no line count moved, the cross-book `scripts/check.sh`
   transcript class does not fire.
6. **The reviewer allowance was spent once, and it caught the leaf reintroducing
   its own defect class.** Decision 2's wording — *refusing it costs no exclusive
   lock — the refusal is the caller's, before this verb is ever reached* — stated
   the no-lock claim of **every** caller. Rust evaluates arguments left to right
   and `tree: Guard` is argument one, so at `crates/grove-loop/tests/verbs.rs:393`
   and at every inline call site (`tree_lifecycle.rs:1855`, `1870`, `1880`, …)
   the guard is taken *before* the slug is built. Only `grove-llm`'s
   `cmd_leaf_decompose` has the order the sentence asserted. That is the same
   over-claim shape as the comment this leaf removed — a property of one call
   site written as a property of the function — so the comment now names
   `grove-llm` explicitly, and the chapter gained a paragraph stating the
   asymmetry and why the signature cannot enforce it.
7. **Findings classified.** Valid and actionable, fixed: the unconditional
   no-lock claim (source and page); the self-contradicting `concept-index.md`
   entry (*brought back level with its code* → *two passages … brought back into
   agreement*, since the comment is level with its code and it is the other
   passage that is 1,386 lines away); the section opening inviting an
   adjudication its close reports as finished; an unparseable three-deep relative
   clause at the test's paragraph; one 111-character line in an ~80-column file.
   Visible trade-off, not fixed: the italic quotation of the test doc comment is
   lower-cased and elided without an ellipsis — that is the chapter's established
   quoting convention and this leaf's task file quotes it identically.
   Externalised, not fixed: chapter 21's landed-leaf tally.
8. **Chapter 21's tally is a pre-existing defect and became
   `landed-leaf-tally-chapter-21-k200`.** *Four of that second group have since
   landed* (`21-what-could-not-move.md:463`) omits `grow-header-stale-helper-k154`,
   `default-root-slug-two-spellings-k159`, `welded-grove-name-summary-k160` and
   `refused-grove-test-overclaims-k161` — all landed before this leaf was picked —
   so it was already wrong independently of k162 and will be wrong again after the
   next deferred leaf lands. Correcting it means enumerating every landed leaf in
   that group and probably replacing the count with a structural claim, which is
   another session's work, not a clause inside this one. Chapter 21 owns no source
   roots, so `book-check` expands nothing there and the whole page is unchecked
   claims about other chapters.
