# grove-llm-version-comment-k83

## Goal

Correct the one claim in `crates/grove-llm/src/cli.rs` that the source does not
bear out — the `#[command]` attribute comment saying *`crates/grove-llm` carries
a `0.1.0` that names nothing an operator can install*, lines 39–43 — and land it
as a corpus change the book contract permits.

## Context

- Observed at `the-surface-k78` while drafting the overview's chapter 2, which
  owns the matching `version = grove_loop::VERSION` attribute in
  `crates/grove/src/cli.rs` and reads the `grove-llm` attribute as evidence
  for the claim that both binaries report one number. `crates/grove-llm/Cargo.toml`
  line 3 is `version.workspace = true`, so the package carries the workspace's
  `20.1.0` and no `0.1.0` of its own; the comment describes a manifest that no
  longer exists.
- The overview page does not repeat the stale claim. It states the two
  mechanisms that hold the numbers equal today — both manifests inherit the
  workspace version, and both clap models read one constant — and argues for
  the second on the ground the comment still states correctly: one definition
  rather than two manifests staying in step. Nothing in the overview needs to
  change when the comment does.
- This file is a root of the `grove-llm` book (`grove-llm-book-k33`), which
  will own lines 39–43 and must reconstruct them. A rewording that keeps the
  line count moves no boundary; one that changes it moves every range below
  line 43 in that root.

## Done when

- The comment states something the source bears out: the reason to read the
  loop's constant rather than `env!("CARGO_PKG_VERSION")` in each binary, with
  no claim about a package version the manifest does not carry.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, every affected fragment and page of the
  `grove-llm` book, and a green `book-check --final` over that book.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside
`manifest-function-count-k82`: editing a byte of a frozen root while the book
that quotes it is being written invalidates the ranges the freeze protects. If
`grove-llm-book-k33` lands first and quotes the comment as written, its page
adjudicates the stale claim the way the overview's chapter 1 adjudicates the
function count, and this leaf rewrites that paragraph in the same commit.
