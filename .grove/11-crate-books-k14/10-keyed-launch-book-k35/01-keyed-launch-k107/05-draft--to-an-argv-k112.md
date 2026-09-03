# to-an-argv-k112

## Goal

Draft chapter 5 of the `keyed-launch` book — *From a template to an argv*,
`05-to-an-argv.md`, slice `whole-word-or-nothing` — owning `src/templates.rs`
lines 146–275 (130) and 661–670 (10), and `src/argv.rs` whole (48): 188 lines.

## Context

- Draft stage, child 5 of 10 of `keyed-launch-k107`. Responsibilities are the
  structure brief's *5 · From a template to an argv*.
- `source`, and why `None` for a key the primary does not declare whatever the
  overlay says; `require` as the obligation a consumer discharges *before* it
  commits to a key, stated once so the refusal's wording has one owner;
  **`expand`'s single remaining obligation and why it is stated over the
  *vocabulary* rather than over this template's own words** — so a consumer cannot
  have a call that works for one key and fails for its neighbour purely because
  the two templates mention different optional slots; `match_values` as that
  check; `unresolved` and its two wordings, **which is where the
  untracked-delta rule is actually kept**; `keys` as the conformance kit's one
  window and nothing more; and `Argv` — no constructor, no shell, `words()`.
- **The seam is proved here.** `Argv::new` is `pub(crate)` and `Templates::expand`
  is its only caller, so *nothing reaches a spawn that a template did not author*
  is a fact about the types. Chapter 1 stated it; this chapter shows the two lines
  that make it true. Verify the `pub(crate)` and the caller count against the
  source rather than against chapter 1's claim.
- Prose obligation 3 is **supply the argument** — the last of the three chapters
  it points at. From chapter 6 onward the instruction reverses.
- Required example anchor: `four-words` — `expand("impl", …)` through to
  `["claude", "--model", "opus", "<the prompt>"]`, the prompt one argument however
  many spaces it holds.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  whole-word-or-nothing --check all` is valid: 958 resolved lines, 1,115 deferred,
  `final=false`.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

This chapter closes `src/templates.rs`: after it, all eight blocks of that root
are resolved. It owns the 146–275 that sits between chapter 3's two blocks, and
the ownership ledger is where a reader sees that.
