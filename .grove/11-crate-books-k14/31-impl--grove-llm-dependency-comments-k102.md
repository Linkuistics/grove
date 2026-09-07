# grove-llm-dependency-comments-k102

## Goal

Correct the three claims in `crates/grove-llm` that the crate's source does not
bear out — `Cargo.toml`'s *A binary target can reach its own library's private
items* and *everything this binary can reach is something `grove-loop` chose to
publish* (both in lines 9–15) and `src/lib.rs`'s *something `grove-loop` or
`grove` chose to publish* (lines 7–8) — and land them as one corpus change the
book contract permits.

## Context

- Both were adjudicated on the page at `orientation-k92` while drafting the
  `grove-llm` book's chapter 1, which owns `manifest-thin-by-crate` and
  `library-root` and reproduces both comments verbatim. That session's decision 7
  states the fix and defers it: *no leaf is cut this session … it belongs after
  every book with k83*. No leaf was ever cut for it, so the finding lived only in
  `grove-llm-k91`'s decision log until `what-order-holds-k98` closed the book and
  externalised it here. Placed beside `grove-llm-version-comment-k83`,
  `root-init-drop-order-comment-k99`, `next-steps-comment-lane-k100` and
  `complete-help-grove-do-k101`, and ahead of `architecture-residue-k75`, which
  stays last in this node.
- **The manifest claim.** Every `grove_loop::` name the binary uses is a `pub`
  item of that crate, and `Workspace` is re-exported at
  `crates/grove-loop/src/lib.rs` line 81 — so nothing the binary *reaches* is
  unpublished by `grove-loop`. But `crates/grove-llm/Cargo.toml` declares
  `jj-workspace` as a second direct dependency, and a declared dependency makes
  that crate's whole public surface *reachable*: a `use jj_workspace::` of any
  item it publishes compiles today. The sentence is about what can be reached,
  and as written it does not hold.
- **The privacy claim, added by `bin-target-privacy-claim-k87`.** Lines 11–12
  restate decision 1 of `docs/specs/module-decomposition.md` as a universal: *A
  binary target can reach its own library's private items*. It is true only of a
  target that compiles the library's modules as its own, and false of the
  ordinary shape — a target that depends on the library beside it is refused
  with `E0603`, measured twice, at `proving-a-negative-k80` and again at k87.
  k87 corrected the same clause in `crates/grove/Cargo.toml`, in decision 1
  itself, and in `docs/ARCHITECTURE.md`; this site was left here because it sits
  **inside the same fragment** — `manifest-crate-not-a-target`, lines 9–15 —
  that this leaf must rewrite anyway, and two leaves editing seven lines in
  succession would make the second re-derive the first's ranges. Decision 1 now
  carries the long form, so this comment needs only the shape named, in the room
  it has. Chapter 1 is where the book adjudicates it — *the comment's own
  sentence states the clause without naming the shape it holds for, and is
  reproduced as written* — and that paragraph moves with the comment.
  `07-what-order-holds.md` only **points at** that reading (*Orientation read
  the alternative that clause holds for and the shape it does not*), so it
  survives unchanged; check it rather than assume it.
- **The library-root claim.** *`grove-loop` or `grove`* was true while this
  package depended on both. The dependency table now names `grove-loop` and
  `jj-workspace`, no `use grove::` appears in the crate, and the manifest's own
  third comment records the `grove` edge as removed at `loop-crate-driver-k22`.
  The manifest is the current fact and this doc comment is the stale one.
- **Two fixes are available for the manifest claim and they are not equal.**
  Dropping the `jj-workspace` dependency and importing `grove_loop::Workspace`
  makes the sentence true as written, and changes `Cargo.toml` and `cli.rs` line
  32. Rewording the sentence to the narrower fact — that nothing reached is
  unpublished by `grove-loop` — changes one comment. The first is the stronger
  end state and the more invasive change; choose deliberately and say which.
- The corpus is frozen and both files are roots of the `grove-llm` book
  (`grove-llm-book-k33`), which reconstructs every byte of them. Chapter 1 owns
  `Cargo.toml` 1–54 and `lib.rs` 1–16 in eight and two literal fragments; a
  rewording that changes the line count of either file moves every range below it
  in that root and the ledger with it.

## Done when

- All three sentences state something the source bears out, and chapter 1 of
  `docs/walkthroughs/grove-llm/` has been reconciled: the affected literal
  fragments carry the new bytes, any changed line counts are reflected in the
  ownership blocks, the manifest and the source index, and the paragraphs
  that adjudicate the stale claims are rewritten to describe what the comments
  now say — chapter 1 owns all three, and `07-what-order-holds.md`'s pointer at
  chapter 1's reading is re-read rather than assumed.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, every affected fragment, ledger row and page, and a green
  `book-check --final --check all` over the `grove-llm` book.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, for the reason
`grove-llm-version-comment-k83` and `manifest-function-count-k82` are: editing a
byte of a frozen root while a book that quotes it is being written invalidates
the ranges the freeze protects. If a later book quotes either comment, this leaf
reconciles that book too, in the same commit.
