# core-fallible-construct-counts-k179

## Goal

Re-derive, and then correct or justify, the two supporting numbers in chapter 19
of the `grove-loop` book — *the next lowest is three, and `driver_lease.rs` has
215* — which no counting rule tried so far reproduces. The claim they support is
sound and is not in question.

## Context

`docs/walkthroughs/grove-loop/19-the-core.md`, under `## What could not move`,
reads:

> **On the way through — the preconditions.** None. Enumerated across the crate's
> twelve Rust roots, `prompt.rs` is the **only one with no fallible construct at
> all** — no `Result`, no `?`, no `bail!`, no `unwrap`, no `expect`, no `panic!`;
> the next lowest is three, and `driver_lease.rs` has 215.

**The load-bearing half is confirmed.** `prompt.rs` really is the only one of the
twelve Rust roots with zero occurrences of those six constructs, and it is a
zero rather than a minimum. Nothing in chapter 19's argument — or in chapter 21's
restatement of it — depends on the two numbers after the semicolon.

**Neither number reproduces.** Counting lines that match
`\bResult\b|\?|bail!|unwrap|expect|panic!` over `crates/grove-loop/src/*.rs`,
with `//` comments stripped, gives:

| Root | Count |
|---|---:|
| `prompt.rs` | **0** |
| `complete.rs` | 4 |
| `driver.rs` | 5 |
| `lib.rs` | 10 |
| `session_config.rs` | 25 |
| `verbs.rs` | 30 |
| `loop_driver.rs` | 33 |
| `task_grow.rs` | 33 |
| `task_name.rs` | 71 |
| `task_tree.rs` | 178 |
| `driver_lease.rs` | 257 |
| `tree_lifecycle.rs` | 257 |

So the next lowest is **4**, not three — three only if the `use anyhow::Result`
line is silently dropped — and `driver_lease.rs` is **257** by matching line,
also 257 by occurrence, 132 in the production half chapter 16 owns and 125 in the
test half chapter 17 owns. None of those is 215. **And `driver_lease.rs` is not
uniquely the maximum**: `tree_lifecycle.rs` ties it at 257 under this rule, which
the sentence's *and `driver_lease.rs` has 215* implies it is not.

**The chapter states no counting rule**, which is why this is a *cannot
reproduce* rather than a proven error: a rule exists under which the author got 3
and 215, or the numbers are wrong. Either way the page owes the reader the rule
or the corrected figures — the node's own promoted finding is that *a claim about
a measurement is worth exactly the re-run*, and this is the re-run failing to
land on the published number.

Found by `what-could-not-move-k130`, whose chapter 21 restates chapter 19's
answer to question 2. That page now carries only the confirmed half — *the only
one of the crate's twelve with no fallible construct anywhere in it … the only
zero rather than merely the lowest* — and no derived number, so **nothing
downstream depends on this leaf's outcome**; it is chapter 19's own page that
carries the two figures.

## Done when

- The counting rule is stated, or the two figures are corrected, in
  `docs/walkthroughs/grove-loop/19-the-core.md`. Prefer stating the rule: the
  neighbouring sentence *Enumerated across the crate's twelve Rust roots* already
  promises an enumeration, and a figure without its rule is what produced this
  leaf.
- If the figures change, check whether *`driver_lease.rs` has N* still wants to
  name that root at all, given the tie above — the node's promoted guidance is to
  **prefer a characterisation to an ordinal**, and two chapters of Part V have
  already had a size claim corrected (`lease-size-ranking-k171`,
  `chapter-eighteen-size-claim-k175`).
- `book-check --repo . --book docs/walkthroughs/grove-loop --final --check all`
  is still valid at 13 files, 10,533 resolved lines, `final=true`, and
  `bash scripts/check.sh` passes.

## Notes

**Prose only, and the corpus is frozen.** This leaf edits one paragraph of one
book page. It changes nothing under `crates/`, and it must not touch a literal
fragment — `book-check` compares those byte for byte.

**Do not weaken the surviving claim.** *`prompt.rs` is the only root with no
fallible construct at all* is measured, reproduced and load-bearing for chapter
19's answer to the second question. Only the clause after the semicolon is in
doubt.
