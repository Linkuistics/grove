# what-order-holds-k98

## Goal

Draft chapter 7 of the `grove-llm` book — slice `assembly`,
`07-what-order-holds.md`, owning no source — and take the book to green
**final** validation and a green `bash scripts/check.sh`.

## Context

- Draft stage, child 7 of 7 of `grove-llm-k91`, and the only child whose `Done
  when` carries the final run and the umbrella script. Responsibilities are the
  structure brief's *7 · What order holds* section: assemble the twelve verbs
  into one table — which text each parses, which opening it takes, which check
  precedes its mutation, what its stdout carries, what its stderr carries, and
  the test that holds it — and read the table as the what-is-left test with the
  stream contract as its rendering half. State what the compiler holds, what
  order holds, and what tests hold. State where the book stops —
  `grove_loop::verbs` — and that this crate's test directory also carries the
  suite that compares the shipped methodology against the binary
  (`instructed_verbs.rs`, `composition_guidance.rs`,
  `session_kind_guidance.rs`, `removed_surface.rs`), named once and not
  explained.
- It has no worked-example section; the catalogue rule allows the table to
  open the page. Close the early-use and source ledgers, confirm every deferral
  has become an insertion, and record the final verification commands and
  results, as the overview's fifth chapter does.

## Done when

- `book-check --book docs/walkthroughs/grove-llm --final --check all` is
  valid: 4 files, 1,017 resolved lines, 0 deferred, `final=true`.
- `bash scripts/check.sh` passes, the book gated by discovery.
- **Last act**: `grove-llm leaf-add grove-llm-book-k33 grove-llm --kind
  copy-edit`, unless a live later sibling under `grove-llm-book-k33` already
  holds that stage — read that off the node's live entries. Write into the new
  leaf's body the specific things this draft hands to the copy edit, and put
  anything a later stage owns under `## Handed forward` in
  `grove-llm-book-k33`'s brief.
