# refusal-k60

## Goal

Draft `06-refusal.md`, slice `no-remedy-of-its-own`, and take the book to
`book-check --through no-remedy-of-its-own --check all`.

## Context

- Responsibilities: [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md),
  *6 · Refusal*. Thesis: **the crate has no consumer to speak for**, so it never
  says what the caller should do; it says what jj offers.
- One block, 230 lines — the largest in the book: `refusal-source`, the whole of
  `crates/jj-workspace/src/refusal.rs`.
- Required worked-example anchor `worked-refusal`: the **identical** attempt
  chapter 5 traced, where `jj commit` declines — `CommandFailed` wrapped as
  `CommitNotRecorded`, the rendered message, and the `source()` chain a consumer
  walks. Reuse chapter 5's tree, paths and message unchanged; this is a variation
  on a trace the reader already holds, not a second scenario.
- **Walk the ten kinds at the sites that create them, not as a catalogue.** The
  chapter is placed sixth rather than second precisely so the reader has met each
  refusal at the site that creates it before reading the variants as a set.
- Two carry remedies and get their full weight: the gate's refusal, whose
  two-line repair is stated unconditionally rather than chosen by probing for a
  `.git`; and `CommitNotRecorded`, the only refusal that has to say something
  about *state* rather than about a command.
- It reserves `docs/USAGE.md#undoing-a-mistake`, beside `CommitNotRecorded`'s
  `jj undo` / `jj op log` remedy.
- Close the loop with chapter 1's empty dependency table: the error is the
  crate's own type and leaks no error crate.

## Done when

- `06-refusal.md` exists; the block is resolved, its defer replaced, ledger row
  moved to `resolved`, fragment index rows added, navigation and contents
  updated, concept-index entries curated.
- The `Refusal` early-use row and the `Refusal::not_a_workspace` /
  `Refusal::unresolvable_path` row move to `explained`.
- `book-check --through no-remedy-of-its-own --check all` exits 0.
- The draft stage record's `## Provenance` names this commit.
- `cargo test --locked --workspace`, `cargo clippy` and `cargo fmt --all --check`
  pass. `scripts/check.sh` is red on `book-check` alone.

## Notes

**Most of this file's non-comment bytes are user-facing remedy text inside string
literals**, so the fragment graph already puts the messages on the page verbatim.
The prose's job is the adjudication the message cannot perform: why the type is
opaque, what `Display` and `Error::source` give a consumer in place of matchable
variants, and which alternative was rejected at each choice.
