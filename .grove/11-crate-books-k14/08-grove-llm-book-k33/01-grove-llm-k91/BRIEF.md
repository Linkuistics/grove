# grove-llm-k91 — brief

## Goal

Draft the `grove-llm` book to green final validation: a complete, source-exact
walkthrough of the crate's four roots and 1,017 lines under
`docs/walkthroughs/grove-llm/`, in the seven-chapter shape the structure brief
settled, uniform with the `overview`, `jj-workspace` and `ordinal-fs-tree`
books.

## Context

- **The structure brief is `docs/specs/grove-llm-book-structure.md`**, and it is
  the precondition `grove-draft` requires: it states who the reader is and the
  what-is-left outcome (*Audience and intended outcome*), the ordered section
  plan (*Chapter sequence* and *Concept and seam responsibilities*), and what
  deserves emphasis and what the book does not cover (*What each chapter's
  prose owes*, *What the book deliberately does not cover*). Its ownership
  mapping is the manifest's `[[block]]` table; where the two disagree it is a
  defect in one of them.
- The corpus, exactly: `crates/grove-llm/Cargo.toml` (54), `src/lib.rs` (16),
  `src/main.rs` (3), `src/cli.rs` (944). There is no inline test module;
  `crates/grove-llm/tests/` is evidence, and this book cites it more heavily
  than either precedent because *hold the help to the handler* names a test for
  every promise.
- The contract is `docs/specs/walkthrough-books.md`; the validator is
  `book-check`, run through `--through <slice>` per child and `--final` by the
  last.
- Method: `linkuistics:writing-code-walkthroughs`. The draft owns structure and
  technical truth as obligations: every claim about the binary is checked
  against the binary and its tests, not against memory, the guide, or an
  earlier page of the same book.
- **The standing risk is paraphrase.** Roughly half the corpus is comment prose
  and a fifth of `cli.rs` is the twelve verbs' `--help` text, which the guide
  already paraphrases for the human. Each chapter's prose owes the three things
  a comment cannot do — adjudicate the claim, carry the through-line, hold the
  help to the handler — and nothing else.

## Done when

- `docs/walkthroughs/grove-llm/` holds the book and final validation over it
  passes with no deferred holes: 4 files, 1,017 resolved lines, `final=true`.
- `README.md` cites `docs/USAGE.md#usage-tree-verbs`, and every anchor the
  manifest declares exists in its target in the explicit form.
- `every_repository_markdown_reference_resolves`,
  `every_book_root_has_a_documentation_ownership_row` and the corpus-inventory
  tests pass; `bash scripts/check.sh` passes, the book gated by discovery.
- The last child's last act is `grove-llm leaf-add grove-llm-book-k33 grove-llm
  --kind copy-edit`, unless a live later sibling under `grove-llm-book-k33`
  already holds that stage.

## Decomposition

Seven children, one per slice, in canonical page order — the only order the
scoped validator accepts, since `--through` proves a prefix. Each child's
`--through` figures are cumulative resolved and deferred lines.

1. `orientation-k92` — slice `one-call-plus-rendering`, `01-orientation.md`,
   owns `manifest-thin-by-crate`, `library-root`, `entry-point` and
   `surface-thesis-and-imports` (107 lines). Carries the whole book's
   scaffolding: the complete manifest, `README.md`, both indexes, every
   source-root directive, the full ownership ledger with twenty-one defers, all
   fourteen early-use rows `pending`, and the two obligations outside the book.
   107 resolved, 910 deferred.
2. `the-grammar-k93` — slice `admitted-before-dispatch`, `02-the-grammar.md`,
   owns `grammar-cli-and-enum-head`, `enum-close-and-operation-label`,
   `run-admission-and-dispatch` and `openings` (119). 226 resolved, 791 deferred.
3. `reading-the-tree-k94` — slice `information-not-error`,
   `03-reading-the-tree.md`, owns `verbs-reading`,
   `handlers-reading-and-rendering` and `path-and-label-helpers` (214). 440
   resolved, 577 deferred.
4. `growing-the-tree-k95` — slice `before-the-lock`, `04-growing-the-tree.md`,
   owns `verb-root-init`, `verbs-growing`, `args-root-init`,
   `kind-help-and-parse-kind`, `args-growing`, `handler-root-init`,
   `handlers-growing` and `presence-rule-and-slug` (376). 816 resolved, 201
   deferred. The heaviest chapter, kept whole on the brief's argument.
5. `ending-work-k96` — slice `two-steps-remain`, `05-ending-work.md`, owns
   `verbs-ending`, `args-ending` and `handlers-ending` (101). 917 resolved, 100
   deferred.
6. `leaving-the-loop-k97` — slice `admit-before-signal`,
   `06-leaving-the-loop.md`, owns `verbs-leaving`, `args-complete` and
   `handlers-leaving` (100). 1,017 resolved, 0 deferred, still `final=false`.
7. `what-order-holds-k98` — slice `assembly`, `07-what-order-holds.md`. Owns
   no source and is final-only: it takes the book to green **final** validation
   and a green `bash scripts/check.sh`, and is the only child whose `Done when`
   carries either.

**Every child but the last leaves `scripts/check.sh` red on `book-check`, and
that is the shape rather than a lapse.** The script runs `--final` over every
book root by discovery, so the book is inside the gate from the moment child 1
created it, while a prefix deliberately leaves later blocks deferred. Each child
proves itself with `book-check --through <its slice> --check all` and the rest
of the script's checks, and says so.

## Notes

**This is the draft stage only.** Copy edit, art and proof are the later stages
under `grove-llm-book-k33`, cut lazily; a draft that has been polished leaves
the next stage's empty result unreadable. Figures are drawn where the prose
contract requires a relation to be drawn, and left to `art` otherwise.

**Cite `docs/ARCHITECTURE.md`, ADRs and specs by backticked path, never by
link**: the outbound-link contract permits only the guide and the glossary.
`crates/grove-loop` is named where a call crosses into it and explained nowhere;
the book stops at `grove_loop::verbs` and says so once, in chapter 7.

**The corpus is frozen.** Do not edit `crates/grove-llm/`. A defect found here
becomes its own leaf under the root brief's cross-book rule, beside
`grove-llm-version-comment-k83` and ahead of `architecture-residue-k75`.

## Decisions (running log)

**1 · One child per slice, and this session did the first.** The corpus is
1,017 lines and chapter 4 alone owns 376 of them; the structure brief requires
every chapter to adjudicate, carry the through-line and hold the help to the
handler, so the prose-to-source ratio is at least the overview's. Seven sessions
cost less than one long one that degrades by chapter 4. Rejected: one session
for the whole draft.

**2 · Page titles are the short form from the brief's chapter table** —
*Orientation*, *The grammar and the openings*, *Reading the tree*, *Growing the
tree*, *Ending work*, *Leaving the loop*, *What order holds* — and each
chapter's subtitle (*one call plus rendering*, *admitted before dispatch*, …)
is its opening section's heading, under an anchor equal to the slice id. The H1
must equal the manifest `title` and the navigation labels reuse it.

**3 · Chapter 1 partitions the manifest into eight literals along its own
blocks**: the package block; the crate-not-a-target comment; the
grove-dependency-removed comment; the `[[bin]]` table; the dependencies; the
dev-dependencies whole, with a table beside them saying which test needs each;
the lints; and the release block. Blank lines lead the fragment that follows
them, as in the overview's manifest chapter. `lib.rs` is two literals along its
two paragraphs, `main.rs` one, and `cli.rs` 1–34 three: the audience comment
(1–9), the thin-and-order comment (10–23), and the imports (24–34).

**4 · The early-use anchors are fixed now, four of them on a page that does
not exist.** Ten rows have their first use at `01-orientation.md#the-imports`
and are stated there in one table; four have it at
`02-the-grammar.md#worked-dispatch`, so `the-grammar-k93` must carry an
explicit anchor of exactly that name on its worked example and state the four
handler families' minimum statements there. The manifest is complete from the
first slice and the ledger carries all fourteen rows `pending`.

**5 · The two new glossary anchors are taken minimally, as `overview-k76` took
`guaranteed-core`**: `session-epoch` and `tree-access-lock` each promote a
bold paragraph to a `###` heading with the anchor line before it, keeping the
phrase as the heading text and changing nothing else. Chapter 1 cites
`driver-lease`, `session-epoch` and `loop-control-channel` at their first use
in the carried trace; chapters 2, 4 and 6 cite them again where the brief
places them, under the precedent of `overview-k76`'s decision 7.

**6 · The import block is evidence of what the binary reaches, and the page
says what it is not evidence of.** Lines 27–31 name the `verbs` module and
fourteen `grove_loop` items, and line 32 one `jj_workspace` type; four more
`grove_loop` items — `VERSION`, `admit_ambient_session`, `read` and `write` —
are reached by path at their use sites and never imported. The page names all
four so the count on the page is the count in the file.

**7 · The first stale claim is adjudicated as two facts, not one verdict.**
The manifest's *everything this binary can reach is something `grove-loop`
chose to publish* is checked two ways: every `grove_loop::` name the binary
uses is a `pub` item of that crate's root, and `Workspace` is re-exported there
(`crates/grove-loop/src/lib.rs`, line 81), so the binary *reaches* nothing
`grove-loop` did not also publish — but the direct `jj-workspace` dependency
makes that crate's whole public surface *reachable*, and the sentence is about
what can be reached. The page states both and names the fix — drop the line
and import `grove_loop::Workspace` — as a defect leaf's, deferred behind the
book. No leaf is cut this session: the fix invalidates chapter 1's manifest
fragments and the ledger, and belongs after every book with k83.

**8 · The comment-line counts are not stated on the page.** The brief's 45%
and 426 are its own measurement, and the overview's *97 comment lines* was
wrong by five; a count is a claim the source does not carry and the page
reproduces no figure it cannot make the reader check in one command. The page
says *most of the file is comment* and points at the header instead.

**9 · Chapter 2 adjudicates the `0.1.0` comment as a stale fact with a live
reason, and names no leaf on the page.** `Cargo.toml` line 3 inherits the
workspace version, so a bare `version` attribute would also answer `20.1.0`
today; the comment describes the crate as created at `loop-crate-verbs-k21`.
The page states the manifest fact beside the fragment and cites
`the_two_binaries_report_one_version`; the rewrite stays with
`grove-llm-version-comment-k83`.

**10 · The wrong working tree has two endings, and chapter 2 shows both.**
Under the driver's channel a session one working tree off is refused at
admission — *wrong working tree for grove-llm resolve* — before any handler
runs; without the channel it is a manual command and reaches `absent`'s
*grove root not found … Scaffold one with `grove-llm root-init`*. Both were
measured against the built binary, the first under a live `grove` driver.
Later chapters that show a refusal from an opening should say which of the
two environments the trace assumes.

**11 · `worktree`'s comment holds for calls and not for text, and the page
says so.** The module joins `.grove` three times for display — `readable`'s
refusal (chapter 2), `resolve`'s root answer (chapter 3) and `root-init`'s
already-exists refusal (chapter 4) — and passes the working-tree root to every
loop call. Chapters 3 and 4 should state their spelling as display rather
than as a second source of the root. Under a driver the working tree is also
resolved twice per verb, by admission and by `worktree`, from the same
directory; the refusal's *command resolved* clause reports the first.

**12 · Every measurement of the binary's streams so far agrees with clap's
documented behaviour, and two are worth reusing.** A bare `grove-llm` prints
the short help on stderr with exit `2`; `--help` prints the long form on
stdout with exit `0`. Chapter 7's stream table can take those rows from
chapter 2 rather than re-measuring.

**13 · `readable`'s comment gives the driver's scaffold as the reason the
loop answers vacancy on the read side, and that reason was checked against
the driver.** The driver scaffolds through the exclusive opening's vacancy
(`task_tree::write_or_vacancy`), and its one `read` call treats *vacant* as
nothing to pick; the read-side value is the library's shape. Chapter 2 states
that beside the fragment; chapter 4, which reads `root-init` taking the same
vacancy, should not repeat the read-side gloss as the mechanism.

**14 · The structure brief's section 2 says clap answers `--version`
*before `run` is entered*; the source has `Cli::parse()` as `run`'s first
statement.** The page states the position, not the brief's phrase. A copy of
the brief's wording is a defect in the brief, not the book, and is left there.

**15 · Chapter 3 adjudicates five comment claims beside their fragments and
names no leaf.** `no_live_leaves`'s *spelled four times* has three callers;
`kind`'s help refuses a *missing or unknown* kind where, since
`open-kind-k20`, only a malformed token can refuse; `label`'s *equals the
grove name* is a convention the binary checks nothing of; `resolve`'s help
omits `.`, which the handler answers and `resolve_dot_prints_the_grove_root`
pins; and `Reference::parse`'s refusal offers *a path under `.grove/`*, a
form the grow verbs' `<parent>`/`<target>` accept and `resolve` does not
(measured: not-found). The last is `grove-loop`'s corpus and that book's to
own, on the footing of decision 7. **Chapter 4 should say, beside its
`<parent>` argument, that the path form is the grow verbs' and not
`resolve`'s**, and should treat `root-init`'s `.grove` as the third display
spelling, chapter 3 having claimed the second.

**16 · Chapter 3 names every promise no test holds rather than leaving it
implied**, and chapter 7's table should carry the same discipline: the
finished-grove diagnostic of `brief-chain` (handler alone), `pick` skipping an
`ABANDONED` leaf (no test in this crate), the abandoned `resolve` note (unit
test only), `[n]-slug` as decorative, `normalize_leaf_path`'s pass-through
branch, `render_resolution`'s root arm, and the `absent` remedy's second line
for every verb but `pick`. Chapter 7's stream rows for the four reading
verbs can be taken from chapter 3's absent-answer table: stderr for every
absent answer, exit `0`; refusals exit `1`.

**17 · The worked example may look forward in the carried session.**
Chapter 3's second and third renderings use the tree as chapters 4 and 5
leave it (`k3` retired, `k4` live), with the transcript marking the step, and
its finished-grove transcript uses the state after the review leaf's own
session; later chapters may do the same rather than invent values.
