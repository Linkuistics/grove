# orientation-k108

## Goal

Create the `keyed-launch` book and prove its first slice: `understands-neither`,
`01-orientation.md`, owning the whole of `crates/keyed-launch/Cargo.toml`,
`src/lib.rs` and `src/error.rs` — 196 lines.

## Context

- Draft stage, child 1 of 10 of `keyed-launch-k107`. The structure brief is
  `docs/specs/keyed-launch-book-structure.md`, and chapter 1's responsibilities
  are its *1 · Orientation — understands neither* section: what the crate is and
  what it refuses to be, with grove's mapping stated **once** as *a session kind
  is a key*; the three dependencies read as the evidence of that claim; the
  manifest's opaque-error rule and the two other crates that state it; `lib.rs`
  as the book's map, its six headed theses named against the chapters that own
  them; **the two halves and the seam** — `Templates::expand` authors an `Argv`,
  `run` consumes one, neither module names the other, and the compiler holds it —
  stated here as the ground chapter 5 stands on; the two error types read as the
  seam's evidence, since `error.rs` lines 47–52 give *keeping the two halves
  usable apart* as the reason there are two; the opacity argument and the
  obligation that replaces it; and `release = false` as an answered question.
- **One sentence says how this book's spine differs from `jj-workspace`'s**, and
  no later chapter returns to it: that book's refusals each have an address, and
  this crate names no other owner because the meaning does not exist inside it to
  delegate.
- The first slice carries the book's scaffolding under
  `docs/specs/walkthrough-books.md`, *Authoring workflow and scoped proof*: the
  complete manifest, `README.md`, both lookup indexes, all nine source-root
  directives, the full ownership ledger with a defer for each of the seventeen
  later-owned blocks, and every early-use row `pending`.
- The required example anchor is `the-launch-in-outline`: the two published
  `config.kdl` lines on disk through to a running child and a token, named but
  not traced. It fixes the concrete values every later chapter reuses — the
  prompt string, the paths and the token — as all three precedents left them to
  the orientation slice.
- **The one obligation outside the book is this child's**: `docs/ARCHITECTURE.md`'s
  *Documentation ownership* table gains its `keyed-launch` row, in the wording the
  brief's *The book's row in the ownership table* carries. No glossary promotion
  is owed — all three reserved anchors already exist in explicit form.
- `docs/specs/module-decomposition.md`'s decision 7 sketch is stale against this
  crate's real surface. It is not adjudicated at a fragment, because there is no
  fragment to adjudicate it against, and it is `runner-sketch-drift-k106`'s.

## Done when

- `book-check --repo . --book docs/walkthroughs/keyed-launch --through
  understands-neither --check all` is valid: 9 files, 196 resolved lines, 1,877
  deferred, `final=false`.
- `every_repository_markdown_reference_resolves`,
  `every_book_root_has_a_documentation_ownership_row` and the corpus-inventory
  tests pass with the new book root present.
- `scripts/check.sh` is red on `book-check` alone, and this file says so.

## Notes

The worked example fixes the values every later chapter reuses; they are recorded
in `keyed-launch-book-k35`'s brief under *Pointers*, and the carried
configuration is grove's own, told strictly from the crate's side.

## Decisions (running log)

**1 · The structure brief is `docs/specs/keyed-launch-book-structure.md`, and it
was read before anything else.** It states the reader and the pass-through
outcome (*Audience and intended outcome*), the ordered plan and each chapter's
responsibilities (*Chapter sequence*, *Concept and seam responsibilities*), and
the emphasis and exclusions (*What each chapter's prose owes*, *What the book
deliberately does not cover*), so the `grove-draft` precondition is met from a
named artifact rather than inferred from the node's own brief.

**2 · The harness did not have `grove:grove-draft`.** The Claude Code plugin
cache this session loaded predates the four stage skills, as it did at
`orientation-k92`. The skill was read from
`plugins/grove/skills/grove-draft/SKILL.md` and the editorial family file from
`plugins/grove/skills/grove/references/editorial.md` in the working tree, with
`grove:grove` from the cache for the spine. Not escalated: `overview-book-k30`
decision 1 already flagged it to the human.

**3 · The carried example uses the primary document alone, and the overlay is
named rather than used.** The structure brief fixes the primary's two published
lines, an overlay declaring `impl` and nothing else, and grove's four-slot
vocabulary; it also gives chapter 3's observable end as `impl` resolving *from
the overlay* and chapter 5's as `["claude", "--model", "opus", …]`, which is the
*primary*'s `impl` template. Both hold only if chapter 1 loads the primary alone,
so step 1 of the outline passes `None` for the overlay and says the second
document is chapter 3's. That also matches the brief's own start point for this
chapter — *the two lines on disk* — and leaves chapter 3 the whole of the
second ending. The fixed values are `/work/atlas` as the working tree,
`/work/atlas/.jj/grove` as the control directory, a channel named
`signal-` plus 32 hex characters, `GROVE_SIGNAL_FILE` as the channel variable,
and `relaunch` as the token.

**4 · Chapter 1's three roots are partitioned along the files' own blank-line
blocks, with a blank line leading the fragment that follows it.** Seventeen
literals under three composites: the manifest in five (package identity 1–10,
dependencies 11–26, dev-dependencies 27–29, lints 30–32, release 33–47);
`lib.rs` in seven, six of them the doc comment's own paragraph breaks (1–9,
10–22, 23–27, 28–34, 35–47, 48–52) and one the modules and exports (53–68); and
`error.rs` in five (the import 1–1, then each type's definition and its three
trait implementations, 2–25, 26–42, 43–66, 67–81). The `error.rs` split is by
*type* rather than by *kind of item* so the two types read as two decisions,
which is what the file's own comments argue.

**5 · The worked example precedes `#the-cast`, which is a departure from the
`grove-llm` chapter 1's order and is deliberate.** The prose contract forbids any
section *earlier* than the complete example from primarily enumerating three or
more public operations, and `#the-cast` is exactly that enumeration — nine public
names including `run`, `signal` and `conformance::check`. Reading the manifest
and the library-root doc comment first, then the example, then the export list,
keeps the catalogue after the example without disturbing source-authority order
inside each file.

**6 · Chapter 1 cites no glossary anchor.** The structure brief places the single
`CONTEXT.md` citation at chapter 6, at its first use of *channel* for the thing
grove's glossary already names. The contract permits a glossary citation at first
use but does not require one, and the *channel* chapter 1 names is this crate's
own `Channel`, which the book owns. `grove-llm`'s chapter 1 took the other
reading; this book follows its brief.

**7 · The one obligation outside the book is discharged, and no glossary
promotion was owed.** `docs/ARCHITECTURE.md`'s *Documentation ownership* table
gains the `keyed-launch` row in the wording the brief's *The book's row in the
ownership table* carries, placed after the `grove-llm` row.
`every_book_root_has_a_documentation_ownership_row` passes. All three declared
anchors — `usage-running-grove` (`docs/USAGE.md` line 29),
`usage-session-lifecycle` (line 242) and `loop-control-channel` (`CONTEXT.md`
line 359) — already exist as explicit `<a id="…"></a>` lines, so `M201` was green
from the first run.

**8 · One `M103` on the first scoped run, and nothing else.** Chapter 1 carried
the first-page navigation form naming chapter 2; the last page of a scoped prefix
takes the final-page form even where the manifest gives it a successor. After the
fix, `book-check --through understands-neither --check all` is valid: 9 files,
196 resolved lines, 1,877 deferred, `final=false`. `reference_navigation` (13
tests, including `every_book_root_has_a_documentation_ownership_row`) and
`corpus_exception_inventory` (5) pass with the new root present.

**9 · The leaf's one in-session reviewer was spent on the page's factual core,
and it paid.** `book-check` proves the bytes the page reproduces and proves
nothing about the claims around them, so a fresh context was given the five book
files, the nine source roots, the tests, the shared contract and the structure
brief with a *find what is wrong* brief, explicitly barred from re-checking the
fenced source bytes. It returned twenty findings, every one classified and every
one re-verified against the source before anything was changed.

**Eighteen were valid and are fixed.** Three were wrong about the code: `libc`
supplies far more than *two system calls* — `run.rs` reaches `signal`,
`sigprocmask`, `tcgetpgrp`, `tcsetpgrp`, `setpgid`, `open`, `kill`, `raise`,
`getpgrp` and `getpid`, and the manifest comment names `kill(2)` and `signal(2)`
as what the *escalation* needs rather than as libc's whole role; `tempfile` is
used by four of the five test files, not five, because `tests/reraise.rs`
re-executes itself and needs no directory; and the error module reaches
`std::error::Error` as well as `std::fmt`. Seven were counts or line references
that did not survive recomputation: *ten chapters* open on the refusal where nine
do, *six headed theses* where `lib.rs` has four `#` headings, two bold paragraphs
and one unheaded opening, the vocabulary section is third in the file rather than
fourth, and the map table's *nine chapters* names eight. Three were contract
violations: the `jj-workspace` comparison ran to four sentences where the brief
requires one and contained an idiom the prose contract forbids, and the lints
paragraph asserted a causal claim (*the reason no `#![allow]` appears*) that the
inheritance does not support. Two were early-use failures and are the most
consequential class, because the ledger is a machine-checked contract the *page*
has to satisfy in prose: the minimum statements for `run`/`Launch`/`Ended`/`End`/
`Escalation` and for `reraise`/`take_interrupt` were paraphrased rather than
stated, so the second did not distinguish the two names at all. All seven rows at
`#the-cast` now carry the ledger's wording **verbatim**, which is checkable by
string containment rather than by reading. Three were precision: the overlay is
`Option`, so a consumer hands over one path and optionally a second; grove maps
onto the crate rather than onto this chapter's 196 lines; and it is `SlotRule`,
not `Vocabulary`, that carries a name and a `Requirement`.

**One was a defect the reviewer found in a claim the source itself makes, and it
is adjudicated rather than fixed.** `error.rs` lines 51–52 say the two halves are
*the whole claim `Templates` and `run` make by not referring to each other*, and
`run.rs` line 53 does name `Templates::expand` — in a doc-comment link on
`Launch::argv`. This is not a stale claim and no leaf is cut: neither module
`use`s or compiles against the other (`run.rs` reaches `crate::channel` and
`crate::error`; `templates.rs` reaches `crate::argv`, `crate::error` and
`crate::vocabulary`), so the source's claim about the code holds and it was the
page's paraphrase — *neither module names the other* — that was stronger than the
source. The page now states the dependency claim and adjudicates the one mention
beside it.

**One was half valid and half a visible trade-off.** The two published lines the
worked example uses as the primary document are `docs/CONFIGURATION.md`'s
**delta** example, not its personal-file example. The trade-off is the structure
brief's, whose *Worked examples* assigns those exact lines to the primary and
calls them real and published; the two documents share one grammar, so the lines
are valid in either, and the book may not link that document to say so. Kept.
The valid half is fixed: the figure carried a `# ~/.config/grove/config.kdl`
banner inside the fence, and a `#` line is not a KDL comment — printing one as
document body, in a book one of whose chapters is about the crate refusing an
unquoted `#`, is a hazard on this book's own subject. The path moved into the
prose and the fence is now the two lines alone.

**None was noise.** The reviewer additionally listed thirty-odd claims it checked
and found correct — every line range in the map table, `Argv::new`'s single
caller, the seven-module and six-export counts, every signature and field name in
the trace, the channel path's 32 hex characters, the token framing, the four-slot
table against `crates/grove-loop/src/session_config.rs`, and the figure, fragment
and link contracts — which is the part of the page the fragment validator had
already made cheap to get right.

**10 · `bash scripts/check.sh` ran after every edit was finished, and is red on
`book-check` alone, by design.** Seven of the eight principal checks pass. The
four existing books report `final=true`; the `keyed-launch` final run reports the
seventeen deferred blocks, the nine pages that do not yet exist, and the `pending`
ledger rows a final scope rejects. Scoped proof over this slice —
`book-check --through understands-neither --check all` — is valid: 9 files, 196
resolved lines, 1,877 deferred, `final=false`.
