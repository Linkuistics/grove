# read-path-fragment-boundary-k192

## Goal

Move the `read-path-fragment-boundary` off-by-one in
`docs/walkthroughs/ordinal-fs-tree/04-read-path.md` onto the right line, and
decide whether `book-check` should have caught it.

Two adjacent fragments over `crates/ordinal-fs-tree/src/fs/read.rs` each
declare a range one line off from the bytes they actually carry:

- `«read-tree-discovery»` (`04-read-path.md:89`) declares `lines="1-81"`; its
  fence body (`04-read-path.md:91-172`) is 82 lines and equals source `1-82`.
- `«read-directory-listing»` (`04-read-path.md:197`) declares `lines="82-155"`;
  its body (`199-271`) is 73 lines and equals source `83-155`.

Source line 82 is blank, and the first fragment carries it.

## Context

- Found at `complete-help-grove-do-k101` by that leaf's one in-session
  reviewer, which was pointed at the `grove-llm` book and swept every book's
  fragments as its cross-tree control. Nothing to do with that leaf's change to
  `crates/grove-llm/src/cli.rs`; it is pre-existing, and it is in the relocated
  `ordinal-fs-tree` book rather than any book this node authored.
- Reproduced independently before this leaf was cut: `sed -n '91,172p'` of the
  page is byte-identical to `sed -n '1,82p'` of the root and differs from
  `1,81p`; `sed -n '199,271p'` is byte-identical to `83,155p` and differs from
  `82,155p`.
- **`book-check --final --check all` is green on that book** (`8720 resolved
  lines`, exit `0`), so this is not a red check to fix but a claim no check
  makes. The two errors cancel: the declared ranges still tile `1-155` with no
  gap or overlap, and the concatenated bytes still reconstruct the file, because
  the misplaced line is blank and sits at the boundary. Whether a *non-blank*
  misplaced line would also have passed is the open question below, and it is
  not answered here: it turns on whether the validator compares the assembled
  corpus or each fragment against its own declared range, and this leaf found
  the symptom without reading the validator.

## Done when

- Each fragment's declared range names the lines its fence actually carries —
  either by moving the declarations to `1-82` and `83-155`, or by moving the
  blank line into the second fence and leaving the declarations alone. Whichever
  is chosen, say which and why in the running log; the page's prose around both
  fragments must still be true of the fence it introduces.
- The question the reviewer's remark raises is answered rather than left: does
  `book-check` verify each fragment against *its own* declared range, or only
  the assembled reconstruction? Read the validator and state the answer. If it
  is the latter, that is a gap the whole campaign leans on — every book's
  per-fragment `lines="A-B"` is then an unchecked annotation — and it earns its
  own leaf beside this one rather than being fixed here.
- `book-check --repo . --book docs/walkthroughs/ordinal-fs-tree --final --check
  all` is green, and `bash scripts/check.sh` passes.

## Notes

The frozen-corpus rule does not bind here: nothing in `crates/` changes, so this
is a page-only edit and no ledger, line count or other book's transcript moves.
That is what makes it safe to place after the books rather than before them.

## Decisions (running log)

**Reproduced both halves before touching anything.** `sed -n '91,172p'` of
`04-read-path.md` is byte-identical to `sed -n '1,82p'` of
`crates/ordinal-fs-tree/src/fs/read.rs` and differs from `1,81p`;
`sed -n '199,271p'` is byte-identical to `83,155p` and differs from `82,155p`.
Source line 82 is blank; line 81 is the `}` closing `snapshot`, and line 83 opens
the doc comment on `Unlistable`. The third sibling, `read-lock-location`
(`156-407`), is unaffected — only the first of the two internal boundaries is
off.

**The repair moves the declarations, not the bytes.** Of the two options the
brief allows, `1-82` / `83-155` is taken. Three grounds. (1) The fence is
*evidence* — bytes reproduced from the source — and the `lines="A-B"` attribute
is the *claim about them*; the claim is what is false, so the claim is what
moves. Relocating a reproduced source line so a wrong annotation comes true
edits the evidence to fit the claim. (2) It keeps *Done when*'s prose clause
true by construction: neither fence's contents change, so no sentence
introducing either fence can have been falsified. Moving the blank line would
change what each fence carries and put every surrounding paragraph back in
question. (3) Neither choice is settled by convention — across all six books the
blank-line boundary falls both ways (leading-blank / trailing-blank per book:
overview 11/2, `jj-workspace` 23/21, `grove-loop` 91/359, `ordinal-fs-tree`
32/38, `grove-llm` 9/51, `keyed-launch` 101/19), and
`docs/specs/walkthrough-books.md` states no rule beyond *every boundary is a
source-line boundary*. So the tie-break is blast radius and evidence-integrity,
not house style.

**The fan-out is two files, and it was enumerated rather than assumed.**
Grepping `1-81` and `82-155` across `docs/` and `.grove/` returns exactly four
live surfaces for these two fragments: the two directives in `04-read-path.md`
(lines 89 and 197) and the two ledger rows in
`docs/walkthroughs/ordinal-fs-tree/source-index.md` (lines 238 and 240). Every
other hit belongs to an unrelated fragment in another book
(`keyed-launch/error.rs 1-81`, `jj-workspace/jj.rs 1-81`). The ledger rows carry
no line-count column, so no derived number moves.

**`book-check` does not verify a fragment against its own declared range once
the book is final; it verifies only the assembled reconstruction.** Read at
`crates/book-validation/src/validator.rs:743` (`check_bytes`), which branches on
whether the root has any `Child::Defer`:

- *With* a defer under the root, it iterates `reachable_literals` and compares
  each literal's bytes against `source_range(bytes, fragment.range)` — the
  fragment's **own** declared range (`validator.rs:766-782`).
- *Without* one — the state every finished book is in, and the state `--final`
  demands — it concatenates all children via `expand_children` and compares that
  single stream against **the root's** range (`validator.rs:786-805`). Fragment
  ranges are not consulted at all on this path.

The only other consumer of a fragment's declared range is
`check_child_partition` (`validator.rs:554`), which checks that children tile
the parent contiguously and in order — an arithmetic property of the
*declarations*, never compared against the fence bytes.

So a redistribution of lines across an internal boundary is invisible whenever
the declarations still tile and the concatenation still reconstructs the root.
The blankness of line 82 is incidental to *why* this passed; it only explains why
the misplacement was hard to see by eye.

**Demonstrated, not merely read — with the instrument seen dirty first.**
Baseline: `book-check --repo . --book docs/walkthroughs/ordinal-fs-tree --final
--check all` → `valid: 17 files, 8720 resolved lines`, exit 0. Positive control:
appending ` X` to one line inside the first fence → `F008 … expected 0x0a, actual
0x20; emitting fragment 'read-tree-discovery'`, exit 1. Gap test: moving page
lines 171-172 (source line 81, the non-blank `}`, together with the blank 82)
from the tail of the first fence to the head of the second — leaving the
declarations `1-81` / `82-155` untouched, so both are then wrong by one and the
misplaced content is **not** blank — → exit 0, `valid: 17 files, 8720 resolved
lines`. The file was restored to green after each mutation. A non-blank
misplaced line passes exactly as the blank one did.

**That gap earns its own leaf, per *Done when*.** Cut as
`fragment-range-unchecked-when-final-k206` beside this one. It is not fixed here:
the fix is a change to the validator plus, potentially, a re-audit of every
per-fragment `lines="A-B"` in six books — 985 fragment directives — which is
tree-sized work, and this leaf is a two-file annotation repair.
