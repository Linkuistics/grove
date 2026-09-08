# release-cut-member-comments-k188

## Goal

Correct the claim, made in four crate manifests, that a release cut moves that
crate *with every other member* — `crates/grove-loop/Cargo.toml` line 49,
`crates/jj-workspace/Cargo.toml` line 34, `crates/keyed-launch/Cargo.toml`
line 37 and `crates/ordinal-fs-tree/Cargo.toml` line 102 — and land all four as
corpus changes the book contract permits.

## Context

- Found at `every-member-version-comment-k84` by the enumeration that leaf owed:
  every `every member` / `every other member` token in `crates/`, `docs/`,
  `scripts/` and the root manifest, classified one at a time. k84 corrected the
  *version-inheritance* predicate over the workspace's members; this is a
  **different predicate over the same set**, and it is false for the same reason.
  `crates/book-validation` is a workspace member, sets `publish = false` and
  carries a `version = "0.1.0"` of its own, so a `cargo release` cut does not
  move it. Six of the seven members move; the sentence says all six others plus
  it, which is seven.
- The four comments are near-identical, each in the crate's own
  `[package.metadata.release]` block:

      # `version.workspace = true`, so a cut moves it with every other member — one
      # workspace, one release version (`docs/specs/module-decomposition.md`,
      # decision 1).

- **All four manifests are frozen roots**, one per book: `grove-loop-book-k37`,
  `jj-workspace` (the pilot's book), `keyed-launch-book-k35`, and the relocated
  `ordinal-fs-tree` book. So this leaf touches four books in one commit — more
  than k84's two.
- **Three of the four books also restate the claim in their own prose**, as true
  rather than adjudicated, and those paragraphs change with the comments:
  `docs/walkthroughs/grove-loop/01-orientation.md` line 196,
  `docs/walkthroughs/keyed-launch/01-orientation.md` line 161, and
  `docs/walkthroughs/jj-workspace/01-orientation.md` line 145. The
  `ordinal-fs-tree` book reproduces the comment without restating it, so only
  its fragment moves. Line numbers are as at k84's commit and are re-derived,
  not trusted: every live sibling leaf between k84 and this one may move them
  first, and there are thirty-one of them.
- **k84's wording is the precedent and the fix should be uniform with it.** The
  three version comments now quantify over *every crate an operator installs*
  rather than *every member*, and the workspace root's `Cargo.toml` carries the
  one paragraph that names `crates/book-validation` as the member that
  deliberately does not inherit. The natural form here is the same move — *a cut
  moves it with every other crate the release ships* — and it needs no new
  explanation, because the root manifest already carries one.
- **`docs/RELEASING.md` is already correct** and is the cross-check: k84 amended
  it to say every claim on that page is about the six shipped crates, not about
  workspace membership. Its *One release, six packages, one tag* heading is
  cited by `crates/grove-llm/Cargo.toml` line 52 and must not be renamed.

## Done when

- The four comments state something the manifests bear out, and the three book
  paragraphs that restate the claim are rewritten to match.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  all four source changes, every affected fragment and page of the `grove-loop`,
  `jj-workspace`, `keyed-launch` and `ordinal-fs-tree` books, and a green
  `book-check --final --check all` over each of the four. Prefer a
  line-count-preserving rewrite in each manifest; one that changes a count
  re-proves every range below it in that root.
- The sibling claim two lines above each of these — *the same lints and rustc
  settings as every other member* (`grove-loop` chapter 1 line 179,
  `keyed-launch` chapter 1 line 143) — is **left alone**. It was checked at k84
  and it is true: all seven members, `book-validation` included, carry
  `[lints] workspace = true`. Classify each token; do not sweep the phrase.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside `manifest-function-count-k82`,
`grove-llm-version-comment-k83` and `every-member-version-comment-k84`: editing a
byte of a frozen root while a book that quotes it is being written invalidates the
ranges the freeze protects.
