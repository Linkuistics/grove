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

1. **The *two of the ten arms* count is gone, and the paragraph kept its job.**
   The paragraph heads two *methodological* traps — a non-compiling mutant that
   prints no per-test lines and reads as clean, and a call-site panic that
   measures reachability — and neither is a property of any row. It now opens
   **"Two more ways to misread the run, and both of them are silent"** and closes
   by saying so explicitly: neither is a property of a particular arm, both are
   traps in the procedure, and they are why the ten rows can be read at all.
   *More* rather than a bare *two* because the section already carries two other
   traps ahead of it — the set-vs-count control and the relink — so a section-wide
   "two traps" would have been the same kind of false quantifier, one scope up.

2. **The five call sites were re-grepped, not taken from this Context, and the
   list held.** `grep -rn root_init crates/` over `.rs`: `grove-llm/src/cli.rs:508`
   (through `verbs::root_init`), `grove-loop/src/tree_lifecycle.rs:1217`,
   `grove-loop/src/task_grow/tests.rs:1598`, `grove-loop/tests/verbs.rs:80` and
   `grove/tests/lifecycle_cutover.rs:1135`. `grove-loop/src/verbs.rs:38` is the
   public door the prose already excludes as a site rather than a caller, and
   `tree_lifecycle.rs:1217` is inside the file's single `#[cfg(test)]` module
   (line 1077), so **four of the five are test code** and the production caller is
   the one that binds.

3. **The evidence was repaired and the conclusion kept.** Row 1's sentence now
   says every site *supplies `Kind::requirements()` and nothing else*, and a new
   short paragraph — *What the five sites do, exactly* — names the four literal
   sites and then `cmd_root_init`, which binds `let kind = Kind::requirements();`
   at `cli.rs:490` **so the same value can go to `require_declared` before the
   lock**, and passes `&kind` at 508. Stated as the leaf asked: the binding is as
   conclusive as the literals, because nothing between the two lines reassigns it
   and `root-init` exposes no flag that could put another kind in it. The closing
   line makes the reason explicit — row 1 is unobserved because of the *argument*
   every caller supplies, not the *syntax* any one of them supplies it in.

4. **The same false clause was live twice in this chapter, and both were fixed.**
   Line 128 — the chapter's own opening account of the refusal — carried
   *every one of which passes `Kind::requirements()` as a literal*, the identical
   claim the Context located only in the measured section. Repairing one and
   leaving its twin would have made the chapter contradict itself, so this is the
   same repair rather than a widening: 128 now reads *supplies
   `Kind::requirements()` and nothing else*, deferring the per-site detail to the
   measurement section that owns it. Its own enumeration — `cmd_root_init`, two
   integration fixtures, the excluded `task_grow/tests.rs`, this file's
   `root_init_at` — was checked against the grep and is correct as it stands.

5. **Green.** `book-check --final --check all` over `docs/walkthroughs/grove-loop`
   is `valid: 13 files, 10557 resolved lines, 0 deferred lines, final=true`, and
   `bash scripts/check.sh` (with `GROVE_SIGNAL_FILE` unset) reports **all 8
   principal checks pass**, six books checked and none failing. No source changed,
   so no fragment range moved and the freeze was never in play.
