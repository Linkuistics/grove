# subprocess-seam-k57

## Goal

Draft `03-subprocess-seam.md`, slice `nothing-ambient`, and take the book to
`book-check --through nothing-ambient --check all`.

## Context

- Responsibilities: [`docs/specs/jj-workspace-book-structure.md`](../../../docs/specs/jj-workspace-book-structure.md),
  *3 · The subprocess seam*. Thesis: **no ambient state selects the repository.**
- One block, 81 lines: `subprocess-seam-source`, the whole of
  `crates/jj-workspace/src/jj.rs`.
- Required worked-example anchor `worked-invocation`: one invocation built end to
  end — the exact argv, the working directory, the four removed variables, and
  both failure endings (jj absent, and jj declining).
- The argued **absence** is as much of the chapter as the presence: there is no
  `JJ_*` counterpart to remove, because jj selects its repository by walking up
  from the working directory and its own variables configure the *user*, so
  stripping them would change who a commit is attributed to. State that jj
  behaviour as this chapter's premise, with a link to jj's documentation.
- `crates/jj-workspace/tests/environment.rs` is the evidence for the hygiene
  claims — cite it, do not reproduce it.

## Done when

- `03-subprocess-seam.md` exists; the block is resolved, its defer replaced,
  ledger row moved to `resolved`, fragment index rows added, navigation and
  contents updated, concept-index entries curated.
- `book-check --through nothing-ambient --check all` exits 0.
- The draft stage record's `## Provenance` names this commit.
- `cargo test --locked --workspace`, `cargo clippy` and `cargo fmt --all --check`
  pass. `scripts/check.sh` is red on `book-check` alone.

## Notes

**A `.git` beside a `.jj` is jj's business.** The chapter states once why four
`GIT_*` variables are nevertheless removed — a Git-aware child following an
inherited foreign repository is a real hazard where the backend is real — and
does not otherwise explore colocation.
