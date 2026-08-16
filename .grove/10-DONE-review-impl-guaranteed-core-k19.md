# guaranteed-core-k19

**Reviews:** guaranteed-core-k9

## Why this chain exists

The cutover is large and load-bearing: it swaps the delivery path every session
depends on, adds a module seam, deletes a composer and reworks two records. It
also made **one design call the leaf did not settle**, and that call is the first
thing to read.

It is placed **before** `mandate-machinery-k10` deliberately. That leaf deletes
the marker grammar, the readers, the build gate and `grove-llm methodology`, which
would leave this review reconciling a historical diff against a tree that had
moved underneath it.

## What to read

The producer's commit, by handle, against the current source. Then:

- `src/prompt.rs` — the seam, the wording, and the two exhaustive matches.
- `tests/prompt.rs` — every claim about the core, and the controls.
- `docs/adr/skill-delivers-the-methodology.md` — reworked in place and renamed.
- `docs/adr/one-build-owns-a-session.md` — the targeted rework.
- The producer's **Running log** in `.grove/09-…-k9.md`, which records what was
  decided rather than specified.

## The doubt this carries

- **The `finish` exception.** The spec said three parts for a session of any kind;
  the producer found `content/SIGNAL.md` states an ending that is wrong for two of
  a `finish` session's three outcomes, and made the third part absent for that one
  kind — amending the spec's scenario rather than departing from it silently. Is
  the argument right, and is the amendment the smallest one that carries it? The
  alternative not taken was a second embedded ending file for `finish`, which
  would keep the shape fixed at the cost of duplicating the outcomes table.
- **The too-late test, applied to what actually shipped.** The core carries five
  things. Check each against the rule, and check the *absences* too: the two
  normative tails left the prompt, and `tests/prompt.rs` asserts both that they
  are gone and that `content/SKILL.md` states them. Is that pair sufficient, or
  can a rule now be stated nowhere?
- **What the checks establish, against what they are cited for.** The suite
  cannot check what the wording *says*; the spec is explicit about that, and the
  producer's test module says so in its header. Is any claim in the code, the
  records or the changelog leaning on a test that does not carry it?
- **`content/MANDATE.md`'s deletion and the `order=` renumber.** In scope per
  `corpus-rewrite-k7`'s brief, but it touched every corpus file's first line. Is
  the contiguity convention intact, and did anything else move with it?
- **The deletions.** `methodology::compose`, the composition golden, and
  `tests/session_kind_guidance.rs`'s per-kind ending section. Each was a real
  claim; confirm the surviving `tests/prompt.rs` claims are the same claims
  narrowed, not weaker ones — particularly the recency claim, which is now
  *ends_with* against a fixed template rather than a total file ordering.

## Not in scope

Inspection only. Do not run the build, the suite, or a formatter; do not edit
code. `guaranteed-core-k20` owns every fix and all post-fix verification.
