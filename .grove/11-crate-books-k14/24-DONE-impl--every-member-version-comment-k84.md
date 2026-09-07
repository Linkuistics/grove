# every-member-version-comment-k84

## Goal

Correct the claim, made in three places, that *every member takes
`version.workspace = true`* — `crates/grove/src/cli.rs` line 12,
`crates/grove-loop/src/lib.rs` line 68, and the workspace root `Cargo.toml`
line 58 — and land the two frozen-root edits as corpus changes the book contract
permits.

## Context

- Observed at `the-surface-k78` under its in-session technical review.
  `crates/book-validation/Cargo.toml` line 3 is `version = "0.1.0"`, and
  `book-validation` is a workspace member (root `Cargo.toml`, `[workspace]
  members`). Six of seven members inherit the workspace version; the claim as
  written is false.
- The overview's chapter 2 adjudicates the claim beside the fragment that
  reproduces line 12: the invariant the comment needs is that every crate on the
  path from `grove` to `grove-loop` inherits one version, and that holds.
  Chapter 1's sentence about `version.workspace = true` was narrowed to the
  same statement in the same session. Once the comment changes, chapter 2's
  adjudicating paragraph in `docs/walkthroughs/overview/02-the-surface.md` is
  wrong the other way and must be rewritten in the same commit.
- Two of the three sites are frozen roots: line 12 of `cli.rs` is the overview's
  (`surface-clap-attributes`, lines 8–18), and line 68 of `grove-loop`'s crate
  root belongs to the `grove-loop` book (`grove-loop-book-k37`). The root
  `Cargo.toml` is in no corpus.
- Two fixes are available and the second is better. Either `book-validation`
  takes `version.workspace = true` too, making the comment true — but the root
  brief earmarks that crate to leave the workspace, and tying its version to the
  release is the wrong direction for a crate on its way out. Or the three
  comments say what is actually held: every crate an operator installs
  inherits one version. Prefer the wording with no universal quantifier over a
  set that is about to change.

## Done when

- The three comments state something the manifests bear out, and the chapter-2
  paragraph that adjudicates line 12 is rewritten to match.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit**
  carries the source change, every affected fragment and page of the overview
  and `grove-loop` books, and a green `book-check --final` over each. A
  rewording that keeps each line count moves no boundary; one that changes it
  re-proves every range below it in that root.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately**, beside
`manifest-function-count-k82` and `grove-llm-version-comment-k83`: editing a
byte of a frozen root while a book that quotes it is being written invalidates
the ranges the freeze protects.

## Decisions (running log)

1. **The quantifier's domain changes; the set does not.** Both fixes offered in
   *Context* were live, and the second is taken: `book-validation` keeps its
   `version = "0.1.0"` and the three comments quantify over *every crate an
   operator installs* instead of *every member*. Verified by enumeration rather
   than by grep — the seven `[workspace] members` rows in the root `Cargo.toml`
   against each member manifest's own `version` line: `book-validation` is
   `version = "0.1.0"`, the other six are `version.workspace = true`. The
   installed set is the six, so the new wording is true today and stays true when
   `book-validation` leaves, which is the direction `plan-k1` decision 6 records
   for it ("not part of the grove system and its home is with the walkthrough
   skill"). Editing `book-validation` to inherit would have made the old sentence
   true by coupling a departing crate to the release.

2. **Both frozen-root rewrites are line-count-preserving.** `crates/grove/src/cli.rs`
   lines 11–15 are five comment lines in and five out, so `surface-clap-attributes`
   (`lines="8-18"`), `surface-empty-struct` (`19-19`) and `surface-grammar`
   (`1-19`) keep their boundaries and the file stays 137 lines.
   `crates/grove-loop/src/lib.rs` lines 67–72 are six in and six out, so
   `library-root-version` (`lines="64-73"`) and every fragment below it in that
   root are unmoved and the file stays 377 lines. Widest new line is 83 columns
   against the file's existing 95, and neither file has a `rustfmt.toml` setting
   that touches comments.

3. **The root `Cargo.toml` names the exception, and it is the only site that
   does.** It is in no corpus (root brief, *The corpus, exactly*), so it pays no
   freeze cost for the extra paragraph, and it is the manifest that *lists* the
   members — the one place a reader meets the seven-member set and needs to know
   why six of them inherit. The two comments inside frozen roots state only the
   invariant they depend on; repeating the exception in each would put the same
   paragraph in three roots and three books.

4. **Both adjudicating paragraphs become explanations of the quantifier, not
   restatements of a settled fact.** With nothing left to adjudicate, deleting
   them would have lost the reason the sentence is worded oddly — a reader who
   counts the workspace's members finds seven and the comment's set is six.
   `docs/walkthroughs/overview/02-the-surface.md` and
   `docs/walkthroughs/grove-loop/01-orientation.md` now each say which set the
   comment quantifies over, name `book-validation` as the member outside it and
   why, and keep the point each page owned: the overview's is the narrower
   path-from-`grove`-to-`grove-loop` invariant its version argument rests on, and
   the `grove-loop` book's is that this is what makes `VERSION` the only version
   an operator can install. The two are uniform, as they were when both
   adjudicated, and the clause they read is now byte-identical in both roots.

5. **The `grove-loop` book's assembly chapter and concept index both carried the
   adjudication and both move.** `21-what-could-not-move.md` said *Two were known
   false before drafting began and are adjudicated on the page*; one now is, so
   the sentence splits — one still standing (`session_config.rs`'s read count,
   `template-source-read-count-k86`) and one corrected at source here. The
   concept-index row *The `version.workspace` claim, adjudicated* names a
   paragraph that no longer adjudicates and became *Which crates take
   `version.workspace = true`, and the member that does not*; its `#the-cast`
   target is unchanged. Found by sweeping the book for the claim rather than by
   reading the changed chapter — a finding against a section does not reach the
   book's summary layer on its own.

6. **`docs/specs/grove-loop-book-structure.md` is edited in place.**
   `SPEC-FORMAT.md` makes `docs/specs/` a current-state set, so its *Known in
   advance: the two claims this book adjudicates* section is not left as a record
   of what was true at drafting: the heading drops the count, the summary bullet
   at the top of the brief drops it too, item 1 is rewritten as a closed claim
   naming the wording that replaced it, and the pointer at chapter 1's entry
   changes from *It adjudicates the `version.workspace` claim* to what the
   paragraph now does. Item 2 and *No third stale claim was found* are untouched.

7. **`docs/RELEASING.md` carried the identical false claim three more times, and
   is corrected here rather than deferred.** It is in no corpus, so it costs no
   freeze; it is the same sentence about the same set, so a leaf for it would
   have split one claim across two commits; and it was already internally
   inconsistent — its heading is *One release, six packages, one tag* and its
   body said *Every member of the workspace takes `version.workspace = true` …
   moves all six*, which is six and seven in one sentence. The three sites now
   quantify over the six shipped crates, and one new paragraph names
   `book-validation` as the seventh member a cut does not touch, so the page
   states its own scope once instead of implying it three times. The cited
   heading is unchanged — `crates/grove-llm/Cargo.toml` line 52 names it.

8. **A second, distinct defect over the same set is externalised, not absorbed.**
   Four crate manifests say a release cut moves that crate *with every other
   member*, which is false for the same reason and is a different predicate:
   `crates/grove-loop` line 49, `crates/jj-workspace` line 34,
   `crates/keyed-launch` line 37, `crates/ordinal-fs-tree` line 102. All four are
   frozen roots of four different books and three of those books restate the
   claim in prose, so it is four books in one commit against this leaf's two —
   its own session, cut as `release-cut-member-comments-k188` at the end of this
   node with the sites and the k84 wording written into it.

9. **The sweep was an enumeration, and four of its tokens survive it untouched.**
   Every `every member` / `every other member` / `all members` / `each member`
   occurrence in `crates/`, `docs/`, `scripts/` and the root manifest was
   extracted and classified one at a time rather than a pattern list swept.
   Twenty-two tokens before this leaf's edits, in five classes. **Four** are this
   leaf's three comments — the root manifest's carries two, *every member takes*
   and *which every member reads*. **Three** are `docs/RELEASING.md`'s, taken in
   decision 7. **Eleven** are `release-cut-member-comments-k188`'s: four
   manifests, the four book fragments reproducing them, and three book paragraphs
   restating them. **One** is `docs/specs/module-decomposition.md` line 635, a
   different sense entirely — a skill family's members, not the workspace's. And
   **three** are the lints-and-settings sense, which is a **true** universal over
   all seven and is deliberately left as written: the root manifest's *Hold the
   clippy baseline at zero, for every member*, and the same claim in the
   `grove-loop` and `keyed-launch` books' chapter 1. All seven member manifests
   carry `[lints] workspace = true`, and `book-validation`'s literal
   `edition = "2021"` / `rust-version = "1.85"` are the workspace's own values,
   so it does hold. That last class is the cross-tree control the sweep needed:
   a pattern coming back clean everywhere reads the same whether or not the
   instrument works, and this is the class still legitimately present. It sits
   two lines from k188's in two of the same files, which is why the tokens were
   classified rather than the phrase swept.

   **One of k188's eleven is invisible to a line-oriented grep.**
   `docs/walkthroughs/jj-workspace/01-orientation.md` line 145 ends *…means a cut
   moves this crate with every* and carries `other member` onto line 146, so the
   token search that found the other ten does not report it. It was found by
   enumerating the four books' `release = false` sections instead. A count taken
   from the line grep alone would have been ten, and k188's body would have
   under-listed its own work.

10. **The in-session review allowance is not spent.** Every claim this leaf makes
    is settled by an enumeration a reader can re-run — the seven member manifests
    against their `version` lines, the token sweep above, the thirty-one live
    siblings counted for k188's body — or by `book-check --final --check all`
    over all six books and `bash scripts/check.sh`, both green with the corpus
    totals unchanged at 204 lines for the overview and 10,533 for `grove-loop`.

11. **`book-check` was seen to fail before it was trusted.** Green over both
    books proves nothing on its own — a validator that never reads the corpus is
    green everywhere. With the source left correct and one line of each changed
    reproduction mutated by a single word, `--final --check all` reported `F008`
    at source byte 575 line 12 naming `surface-clap-attributes` for the overview,
    and at source byte 3379 line 69 naming `library-root-version` for
    `grove-loop` — the two fragments this leaf touched and no others. Restored,
    both are green again at 204 and 10,533 lines. Both directions observed on the
    two fragments that carry the change, so the clean read is evidence rather
    than an absence of one.
