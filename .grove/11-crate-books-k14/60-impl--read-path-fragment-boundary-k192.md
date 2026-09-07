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
