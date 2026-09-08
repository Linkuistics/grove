# stale-mutation-suite-baseline-k198 — brief

## Goal

Re-derive the `grove-loop` book's mutation studies against the suite as it now
stands. Twenty citations across eight chapters state a control of **558 tests**
and it is **560** — but the totals are the cheap half. The counts derived from
those runs are stale for a second, larger reason the leaf that cut this node did
not know when it was written, and every table of observing tests is stale with
them.

## Done when

- Each of the eight chapters states the baseline the suite actually has, and
  every count derived from a mutation run on those pages has been **re-measured**
  under the corrected harness below — never adjusted arithmetically, and never
  copied from a neighbouring page.
- Every table naming individual observing tests is re-derived from the same runs.
  A new observer joins a column as easily as a total, and a page's list of the
  tests holding an arm is a measurement, not a citation
  (`a-cited-test-list-is-a-measurement`).
- Chapter 19's shared-baseline paragraph — the one that speaks for all seven
  chapters at once — agrees with what the seven now say, and the
  mechanical-check question is decided there, once, with
  `ledger-rollup-check-k207` named.
- `book-check --final --check all` is green over every book touched, and
  `bash scripts/check.sh` is no worse than before.

## Decomposition

One child per chapter, in book order, because a chapter's measurement section
plus its own citations is the smallest independently verifiable slice: its
numbers are internally consistent on their own, `book-check` closes over it, and
no child waits on a sibling. Chapters 15 and 16 travel together because 16 cites
15's measurement rather than taking its own, and chapter 19 comes last because
it is the only page that speaks for the others.

1. `ch11-a-grove-begins` — ten arms, the `root_init` entry probe, five citations.
   The chapter whose table the corrected harness falsifies hardest.
2. chapter 12 — sixteen arms, two arms outside the block measured for
   attribution, one whole-line deletion, four citations.
3. chapter 13 — twenty-four arms, three citations.
4. chapter 14 — fifteen arms, two citations. Its table restates four of chapter
   11's arms from the other side, so it must agree with child 1 test for test.
5. chapters 15 and 16 — three mutants, four citations across the two pages.
6. chapter 17 — eight rungs, one citation.
7. chapter 19 — the shared baseline paragraph, the ten-versus-eleven
   adjudication, one citation, and the mechanical-check decision.

Each child cuts the next as its last act, so a chapter that turns out to hold
more than its share does not have to be split against a plan written before it
was measured.

## Pointers

**The control, reproduced.** Copy `crates docs testing plugins scripts
Cargo.toml Cargo.lock .cargo CHANGELOG.md CONTEXT.md CONTEXT-MAP.md README.md
LICENSE release.toml` to a scratch directory, plus `CLAUDE.md` and a plain copy
of it as `AGENTS.md` — without those two `every_repository_markdown_reference_resolves`
fails on `docs/ARCHITECTURE.md:34` and the control is twelve. Then
`cargo build -p grove --bins`, then
`cargo test --no-fail-fast -p grove-loop -p grove-llm`, sharing one
`CARGO_TARGET_DIR` across the control and every arm: **560 tests, 549 passed, 11
failed**, about 55 seconds per mutant after the first.

The eleven are the ten `crates/grove-loop/tests/prompt.rs` composition tests plus
`the_namespace_is_the_shipped_plugin_entrys_declared_name` — the split chapter 19
already states. **Match the set, not the count**: two copies can agree on eleven
and disagree about which eleven, and a control that is red where it should be
green hides observers, because a test already failing cannot fail again.

**Report each mutant as `comm -13 control mutant` over the sorted failure names**,
and confirm the run reported 560 before reading it: a mutant that fails to compile
prints no per-test lines at all and reads exactly like a clean result.

**The eleven, not chapter 19's ten.** Chapter 19 argues for copying
`.claude-plugin/` so the baseline is a clean ten of one cause, and it is right
that a flat eleven can hide an observer of `PLUGIN`. Seven chapters' controls are
nonetheless the eleven, and the eleventh's separate cause is already stated on
chapter 19's own page; re-deriving against a ten would change what each of those
paragraphs is *about* on top of changing its number. Re-confirm the eleven, and
leave chapter 19's recommendation standing as a recommendation.

## Notes

**The premise the parent leaf was cut on is false, and this is why the node
exists.** That leaf assumed each derived count moved by at most the two tests
`unreachable-root-clause-k152` and `default-root-slug-two-spellings-k159` added.
Chapter 11's `RootShape::ATree` arm — recorded as observed by **two** tests in
`11-a-grove-begins.md`'s table and as *Newly failing: 2* in `14-finishing.md`'s —
reddens **ten** against the reproduced control, measured twice, identically.
Neither new test is among the eight extra.

**The cause is a harness step chapter 11's original run did not take.** The
mutant must be relinked into the `grove` binary — `cargo build -p grove --bins`
*after* the edit, not before it — or the suite runs a `grove` built from
unmutated source and every out-of-process observer in
`crates/grove-loop/tests/driver_lease.rs` and `crates/grove/tests/` stays green.
Chapter 11 already names this step, in a later section describing k159's own
re-run; its table predates it. Chapters 12, 13 and 14 state it in their procedure
paragraphs. So a small count or a zero on a page whose run predates the step is a
**lower bound**, not a measurement.

**The doubt is bounded, which is what makes this enumerable rather than endless.**
An arm the `grove` binary cannot reach keeps its recorded count: `RootShape::Taskless`
(`tree_lifecycle.rs:484`) re-measures at exactly one observer, unchanged. An arm
it can reach does not. And the `root_init` entry probe re-measures at **43**
against the page's 41, matching the parent leaf's own re-derivation — so the
instrument agrees with the one number that leaf had already checked.

**Every cited mutation line still names its cited arm.** Checked line by line
across chapters 11, 12 and 13 against `crates/grove-loop/src/tree_lifecycle.rs`
as it now stands: the source under these studies has not moved, so a re-measured
count that differs is the harness or a new test, never a drifted line number.

**Eighty mutation judgements, not eighty commands.** A `bail!` is replaced whole
— `panic!("MUTANT-…")` with **no message**, because every out-of-process observer
asserts on stderr substrings and cannot tell a panic from a refusal when both
print the same words. A `.context` or `.with_context` arm needs a `map_err` that
panics only on the error path, or the types stop agreeing. A `refuse_finish_kind`
call site must panic on the *refusal*, not on entry, or it measures reachability
instead of observation. That per-arm judgement, not machine time, is what sizes a
child.

**The suite is frozen for the whole node, and no child changes source.** These
are prose pages against a measurement. If a child finds a source defect it cuts a
leaf for it rather than fixing it inline, and says so — the corpus freeze is the
parent node's rule and it still holds.

**Re-run any mutant whose newly-failing set names a `driver_lease.rs` or
`prompt.rs` test.** Both files wedge on 120-second producer timeouts under load,
and a timeout reads exactly like a newly attributed observer. Check `ps` for
orphaned `configured-command.sh` children before believing a failure.
