# decompose-help-separator-k222

## Goal

`grove-llm leaf-decompose --help` tells an operator that the first child it grows
is named `01-<kind>-<first-child-slug>-k<new>.md`. **Grove cannot write that
name.** Since `grammar-separator-k15` a leaf's kind and slug are separated by
`--`, and a leaf name without one has no reading at all rather than two
(`docs/adr/task-names-are-canonical.md`). The user-facing help spells a name the
binary would refuse.

Fix the spelling, and fix the reason the guard did not catch it.

## Context

Found by `loop-construct-k7` while reading the verb surface, not while looking
for defects.

**The occurrences partition, and only one half is wrong.** Eleven hits for
`first-child-slug` outside `target/` and `.jj/`:

- **Six are the defective *filename* form** — `01-<kind>-<first-child-slug>-k<new>.md`,
  one dash where the grammar requires `--`:
  `crates/grove-llm/src/cli.rs:211`, `crates/grove-loop/src/tree_lifecycle.rs:494`,
  `crates/grove-llm/tests/leaf_ops.rs:8`,
  `crates/grove-llm/tests/session_kind_guidance.rs:598`, and the two book pages
  that reproduce those bytes — `docs/walkthroughs/grove-llm/04-growing-the-tree.md:1030`
  and `docs/walkthroughs/grove-loop/12-leaf-to-node.md:101`.
- **Four are the *argument* form and are correct** —
  `leaf-decompose <leaf-path> <first-child-slug>` names the CLI positional, not a
  filename: `leaf_ops.rs:4`, `tree_lifecycle.rs:490`,
  `docs/walkthroughs/grove-loop/12-leaf-to-node.md:97`, and
  `docs/walkthroughs/grove-loop/source-index.md:721`. **Do not touch these.**
- One is in a retired task file under `.grove/`, which dies at the finish cycle.

**The deeper half: a guard that exempts the case it was written for.**
`crates/grove-llm/tests/session_kind_guidance.rs` asserts the shipped parser
decides — but only for a *concrete* name. `is_sketch` returns true for anything
containing `<` or `[`, and `classify_example` then routes it to the hand-rolled
`classify_shape` **without ever calling `TaskName::parse`**. So line 598's sketch
is asserted to be "a well-formed grammar sketch" by a check that never applies
the separator rule. That is the shape
`docs/adr/a-closed-partition-is-over-outcomes-not-states.md` names in general — *a
check that exempts its own declared counterexamples tests nothing where the claim
is hardest* — and it is why this survived a leaf that renamed every entry in the
repo's own tree.

**A one-character fix is still a frozen-corpus change.** `cli.rs` and
`tree_lifecycle.rs` are book roots, so the root brief's rule binds: **one commit
carries the source change, every affected ledger and page, and a green run of the
validator over every book it touched.** Measured to help scope it, and to be
re-checked rather than trusted: adding one character leaves every affected line
in place (55→56, 82→83, 89→90, 57→58 characters), so **no line count moves and no
fragment range should shift**. If a range does shift, something reflowed and the
`grove-llm` and `grove-loop` books both need their ledgers re-derived, not
patched.

## Done when

- All six defective occurrences spell `--`, and the four correct ones are
  unchanged.
- The guard can see it: a sketch is checked against the separator rule rather
  than exempted from it — either by routing sketches through a check that knows
  about `--`, or by narrowing what `is_sketch` exempts. **Watch the new check
  fail** on the pre-fix spelling before crediting it; a check that passes both
  spellings is the defect wearing a repair.
- `grove-llm leaf-decompose --help` prints a name grove would write.
- `book-check --final --check all` is green over both affected books, and
  `scripts/check.sh` is green.

## Notes

The two book pages reproduce source bytes, so they change **because** the source
did — they are not independent edits and must not be corrected on their own. Read
`docs/specs/walkthrough-books.md` on an accepted source change before starting:
the ownership ranges and fragments change with the bytes, then final validation
runs against the new bytes.
