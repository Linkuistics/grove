# release-toml-member-quantifier-k205

## Goal

Correct `release.toml`'s claim that *every other member* takes
`version.workspace = true` — line 102 as at `release-cut-member-comments-k188`'s
commit, re-derive it — and decide, once, whether `CHANGELOG.md`'s copy of the
same false predicate is corrected or left as the record of a past release.

## Context

- Found at `release-cut-member-comments-k188`, which was fixing the same false
  predicate in four crate manifests. `every-member-version-comment-k84` owed and
  ran the enumeration behind both leaves, but scoped it to `crates/`, `docs/`,
  `scripts/` and the **root manifest** — and `release.toml` is a repository-root
  file that is not the root manifest, so no token in it was ever classified.
  k188 found it only by grepping the surrounding phrase case-insensitively.

- The sentence, in the comment above `pre-release-commit-message`:

      # Keep the release commit per-crate so `{{version}}` is unambiguous in the
      # commit message. Grove has one released crate — `crates/grove`, the human's
      # binary and the thing the tag names. Every other member takes
      # `version.workspace = true`, so a cut rewrites one field and moves all six
      # versions together (`docs/specs/module-decomposition.md`, decision 1: one
      # workspace, one release version); what `release = false` buys those members
      # is no tag, no changelog section and no publish of their own, not a frozen
      # version.

  It is false and **self-contradicting in its own sentence**: members other than
  `crates/grove` number six, so "every other member" plus `grove` is seven, while
  the same clause says a cut moves *all six* versions. `crates/book-validation`
  sets `publish = false` and carries a `version = "0.1.0"` of its own, so it
  neither inherits nor moves. The later "those members" is right as it stands —
  five crates carry `[package.metadata.release] release = false`, and
  `book-validation` has no such line (`docs/RELEASING.md`, *One release, six
  packages, one tag*).

- **k84's and k188's wording is the precedent.** k84 moved the version-inheritance
  predicate's domain to *every crate an operator installs*; k188 moved the
  release-cut predicate's to *every other crate the release ships*. Either fits
  here — the sentence is about the cut, so k188's is the closer match — and
  neither needs a new explanation, because the workspace root's `Cargo.toml`
  already carries the one paragraph naming `crates/book-validation` as the member
  that deliberately does not inherit.

- **`release.toml` is not a frozen book root.** Roots are each crate's
  `src/**/*.rs` plus that crate's own `Cargo.toml` (`.grove/BRIEF.md`,
  *Pointers*), so this file is quoted by no fragment and no book page. Confirmed
  at k188 by grepping `one release version` and `cut moves` across `docs/`. That
  makes this a plain source edit with no ledger, no page and no re-proof — which
  is exactly why it was cheap to externalise rather than absorb.

- **The second instance is `CHANGELOG.md` line 300**, in the `20.1.0`-era entry
  *One release version is a manifest fact*: "every member takes
  `version.workspace = true`, so a `cargo release` cut rewrites one field and
  moves all six together" — the same predicate, contradicting itself the same
  way. This leaf owes the **decision**, not a reflex: `docs/adr/entries-are-never-removed.md`
  and the repository's practice on amending shipped changelog entries decide
  whether a released entry is corrected in place, footnoted, or left standing as
  what was believed at that release. Record which, and why, so the next
  enumeration does not re-litigate it.

- **Classified and deliberately out of scope**, both checked at k188 and true:
  `Cargo.toml` line 111, *hold the clippy baseline at zero, for every member* —
  all seven members carry `[lints] workspace = true`, `book-validation`
  included; and the two book paragraphs restating it (`grove-loop` chapter 1,
  `keyed-launch` chapter 1). Classify each token; do not sweep the phrase.

## Done when

- `release.toml`'s comment states something the manifests bear out, and its
  "all six" and its quantifier agree.
- The `CHANGELOG.md` question is decided and the decision is recorded in the
  commit message — the entry either corrected or explicitly left standing.
- `bash scripts/check.sh` passes. No book is touched, so no `book-check` re-proof
  is owed; if that turns out to be wrong, the corpus-freeze rule in `.grove/BRIEF.md`
  applies in full and this leaf lands the whole set or defers behind the books.

## Notes

**A defect leaf inherits the blind spot of the enumeration that found it.** k84
enumerated four scopes and this file sat in none of them; the fix here is one
line, but the reusable finding is that a repository-root file which is not
`Cargo.toml` is invisible to that scoping. Any future *classify every token of
phrase P* leaf should enumerate the repository and partition, rather than
enumerate a directory list.
