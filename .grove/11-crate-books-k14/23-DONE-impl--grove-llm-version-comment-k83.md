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

## Decisions (running log)

1. **The rewrite is line-count-preserving: five comment lines in, five out.**
   The book's `grammar-command-attributes` fragment carries `lines="35-50"` into
   `crates/grove-llm/src/cli.rs`, and every later fragment in that root is
   anchored below it. A five-line replacement moves no boundary and leaves the
   1,017-line frozen total intact, which is what `book-check --final` proves
   byte for byte. Each new line is ≤ 78 columns, inside the 84 the file already
   carries, so `cargo fmt` is untouched.

2. **The comment keeps only the argument, and states no fact about either
   manifest.** The stale half was *`crates/grove-llm` carries a `0.1.0`*; the
   sound half was the reason to read one constant. The replacement says both
   binaries read one constant instead of each reading its own
   `env!("CARGO_PKG_VERSION")`, so agreement is a fact about a single definition
   rather than about two manifests staying in step — and keeps the last clause
   about an operator diagnosing a skew. Verified against the source: both
   `crates/grove/src/cli.rs` and this file set `version = grove_loop::VERSION`,
   and `crates/grove-loop/src/lib.rs` line 73 declares
   `pub const VERSION: &str = env!("CARGO_PKG_VERSION")`. Deliberately no claim
   about *every member* inheriting — that is `every-member-version-comment-k84`'s
   comment, in the other binary, and duplicating it here would put the same
   defect in two roots.

3. **Chapter 2's paragraph becomes an argument rather than an adjudication.**
   With no stale claim left there is nothing to adjudicate, so the paragraph now
   says why the constant is worth having *given* that inheritance would already
   make the numbers agree: `crates/grove-llm/Cargo.toml` line 3 and
   `crates/grove-loop/Cargo.toml` line 3 are both `version.workspace = true`, so
   two independent `CARGO_PKG_VERSION` reads would agree anyway, and the constant
   is what survives a manifest that stopped inheriting. The `0.1.0` survives only
   as what `the_two_binaries_report_one_version` asserts against, attributed to
   that test's own comment rather than asserted by the page.

4. **The assembly's tally drops to five, and its known-before count to two.**
   `07-what-order-holds.md` said *Six were judged worth a source change … Three
   of the six were known before drafting began*. Chapter 2 no longer judges this
   comment wanting, so it leaves both counts. Re-enumerated against live leaves
   rather than decremented: the two known-before claims are the manifest's
   reachability clause and `lib.rs`'s *or `grove`*, both carried by
   `grove-llm-dependency-comments-k102`; the three found while drafting are
   `root-init-drop-order-comment-k99`, `next-steps-comment-lane-k100` and
   `complete-help-grove-do-k101`. 2 + 3 = 5, and *each now has a leaf to carry
   one* still holds.

5. **`docs/specs/grove-llm-book-structure.md` is edited in place, not left as a
   record of what was true at drafting.** `SPEC-FORMAT.md` makes `docs/specs/`
   a current-state set, and the brief's *Known in advance* section claimed three
   surviving stale clauses and said chapter 2 adjudicates one of them — both now
   false. The third item is removed and the counts corrected, with one paragraph
   recording that the comment was corrected here so a re-draft does not
   re-adjudicate a claim the corpus no longer carries.

6. **`crates/grove-llm/tests/llm_cli.rs` is left alone.** Its doc comment
   describes the history — a bare `version` attribute answering `0.1.0` when the
   crate was split out at `loop-crate-verbs-k21` — which is still true as
   history, and both of its assertions still hold against the new comment. It is
   in a `tests/` directory, which the root brief calls evidence rather than a
   root, so no book range depends on it.

7. **The in-session review allowance is not spent.** The only claim the compiler
   cannot settle is the count in decision 4, and that was closed by enumerating
   the five leaves above rather than by a second opinion. Everything else is
   proved by `book-check --final` over the book (byte-exact reconstruction) and
   by `scripts/check.sh`.
