# ch11-measurement-prose-claims-k199

## Goal

Correct two claims in `11-a-grove-begins.md`'s *What the refusals are worth,
measured* that the section's own evidence does not support: a count with no
referent, and a statement about how a call site passes its argument.

## Context

- **A count with nothing to count.** `docs/walkthroughs/grove-loop/11-a-grove-begins.md`
  (search *make the reading a lie*) opens a paragraph with **"Two of the ten arms
  make the reading a lie if you skip a check."** Nothing in the paragraph
  identifies two arms. What follows are two *methodological* traps — a mutant that
  fails to compile prints no per-test lines and reads exactly like a clean result,
  and a panic at a call site measures reachability rather than observation — and
  both apply to the procedure as a whole, not to two of the ten rows in the table.
  The sentence borrows the table's shape for a claim that is not about the table.
- **`as a literal` is not true of the one call site that is not a test.** The same
  section says all five call sites that reach `root_init` *pass
  `Kind::requirements()` as a literal*. The five are
  `crates/grove-llm/src/cli.rs:508`, `crates/grove-loop/src/tree_lifecycle.rs:1217`,
  `crates/grove-loop/src/task_grow/tests.rs:1598`,
  `crates/grove-loop/tests/verbs.rs:80` and
  `crates/grove/tests/lifecycle_cutover.rs:1135` — the count is right — but
  `cli.rs` binds `let kind = Kind::requirements();` at line 490 and passes `&kind`
  at 508. The **conclusion** holds (the reserved kind cannot reach the refusal, so
  row 1 is unobserved); the stated evidence does not, as phrased, and it is the
  production call site rather than one of the tests.
- **Neither is caused by `refused-grove-test-overclaims-k161`**, which found them
  while re-deriving the section's table. That leaf moved row 2 and the three
  roll-ups over it, and left these two alone rather than widening its own scope.

## Done when

- The *two of the ten arms* sentence either names two arms and is true of them, or
  is restated as what it is about — the procedure's two traps — with no count
  borrowed from the table.
- The `root_init` call-site sentence says what each of the five sites actually
  does, keeping the conclusion and repairing the evidence: a binding is as
  conclusive as a literal here, and saying so is shorter than pretending it is a
  literal.
- Both are re-derived by enumeration from the sources rather than from the
  sentence being corrected, and the five call sites are re-enumerated rather than
  taken from this Context — the list above was read, not re-grepped, at the time
  this leaf was cut.
- `book-check --final --check all` is green over `docs/walkthroughs/grove-loop`,
  and `bash scripts/check.sh` is no worse than before.

## Notes

**No source change, so the freeze is not in play**, and the leaf is deferred
behind nothing.

**Check whether the neighbouring clause is load-bearing before deleting either
one.** The *two of the ten arms* sentence introduces the two traps that justify
the whole procedure; the count is wrong but the paragraph it heads is not, so the
repair is to the quantifier and not to the paragraph.

## Decisions (running log)
