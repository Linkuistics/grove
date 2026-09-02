# what-the-call-reaches-k81

## Goal

Draft chapter 5 of the overview — slice `assembly`, `05-what-the-call-reaches.md`,
owning no source — and take the book to green **final** validation and a green
`bash scripts/check.sh`.

## Context

- Draft stage, child 5 of 5 of `overview-k76`, and the only child whose `Done
  when` carries the final run and the umbrella script. Responsibilities are the
  structure brief's *5 · What the call reaches* section: the map of what the
  call reaches; the statement of this book's boundary — everything behind
  `grove_loop::run` is named and explained nowhere here, said once rather than
  per row; and the transferable thin-entry-point test applied back across the
  three mechanisms. It has no worked-example section; its job is synthesis, as
  `jj-workspace`'s seventh chapter's is.
- **The module table is `architecture-move-k31`'s to bring, and this session
  moves nothing.** What this chapter can carry from the corpus and the
  workspace as they stand is a package-level map: the six workspace packages
  (`Cargo.toml` at the root names the members), which of them `grove` reaches
  and through what — every one only as a `grove-loop` re-export — and the one
  row that is a module this book explained, `grove::cli`. Shape the page so
  the twelve-module table from *Main module seams* lands under its own heading
  at k31, and do not name a book per row (brief, decision 11).
- The glossary anchor `guaranteed-core` is declared for this page, beside the
  `prompt` module's row. That row arrives with k31; if this session cites the
  term elsewhere, cite it at the first use of *guaranteed core* and nowhere
  twice. A declared anchor no page cites is not a finding.
- The catalogue rule allows the map to open the page because the chapter has
  no example section.
- Close the indexes: the concept index's last entries, the owned-source totals'
  role sentence, and the final-verification section that records the exact
  commands run and what each proved, as `07-what-jj-owns.md` does.

## Done when

- `book-check --book docs/walkthroughs/overview --final --check all` is valid:
  3 files, 204 resolved lines, 0 deferred, `final=true`.
- `bash scripts/check.sh` passes, the overview gated by discovery.
- **Last act**: `grove-llm leaf-add overview-book-k30 overview --kind copy-edit`,
  unless a live later sibling under `overview-book-k30` already holds that
  stage — read that off the node's live entries. Write into the new leaf's body
  the specific things this draft hands to the copy edit, and put anything a
  later stage owns under `## Handed forward` in `overview-book-k30`'s brief.

## Decisions (running log)

**1 · The structure brief is `docs/specs/overview-book-structure.md`, and it
was read before anything else.** Its *5 · What the call reaches* section states
the three responsibilities — the map, the boundary said once, the test applied
back — and its *Worked examples* section releases this chapter from carrying
one, which is why the map may open the page under the catalogue rule.

**2 · The map is seven members, not six.** The root manifest names seven
`[workspace] members`, and *The surface* had already said so at its version
paragraph; a table of six would have contradicted a page in the same book. The
seventh row is `book-validation`, stated as the authoring tool and as the
dev-dependency *Orientation* read. Dependency edges were read from each
crate's `[dependencies]` table: `grove` → `grove-loop`; `grove-llm` →
`grove-loop`, `jj-workspace`; `grove-loop` → `jj-workspace`, `keyed-launch`,
`ordinal-fs-tree`; the three domain-free crates and `book-validation` depend
on no member.

**3 · The seven names are every distinct `grove_loop::` path in the two Rust
files**, one of them (`run` at `main.rs` line 6) a documentation link rather
than a use. Each is mapped to the `lib.rs` line that publishes it: 73, 80, 81,
87, 88 and 91. Four of the five loop-owned names come from modules `lib.rs`
declares without `pub`, which is what makes the re-export lines load-bearing.

**4 · Mechanism 1's negative side was measured, not read.** A scratch package
in the session scratchpad (`reach/`), depending on `grove-loop` by path with
`CARGO_TARGET_DIR` outside the repository, named `DriverLease` once through
`grove_loop::driver_lease` and once through the root; `cargo check --offline`
refused the first with `E0603: module driver_lease is private` and the note
pointing at `lib.rs` line 54, and accepted the second. The repository was not
touched. The page quotes the diagnostic with the note and the two trailing
lines elided, and says so.

**5 · The modules section is shaped as `architecture-move-k31`'s destination
and moves nothing.** It names the four places in the loop's module tree the
binary's five loop-owned names come from — the root, `loop_driver`,
`driver_lease`, `session_config` — states the root's eleven modules and which
four are `pub`, and names `docs/ARCHITECTURE.md`'s *Main module seams* by path
as where the module-by-module account is *today*. That word, and the
boundary section's *the description a reader can open today*, are two
sentences k31 must revisit when the table lands; recorded in the node brief's
*Pointers*.

**6 · `guaranteed-core` is not cited on this page; `task-tree-scheme` is.**
The task file places the first beside the `prompt` row, which arrives with
k31, and says a declared anchor no page cites is not a finding. The second is
cited in the boundary table at the task tree's row: the brief's outbound-link
table names ch. 5 for it, and the node brief's decision 7 is the precedent for
a second citation where the brief places one.

**7 · Chapter 1's comment count was corrected under the draft's
technical-truth charter.** Counted with a script: 29 comment lines in
`Cargo.toml`, 7 in `main.rs`, 56 in `cli.rs` — 92, not 97. The 97 is
reproducible only by adding the `about` string and the four assertion-message
lines, which are not comments. Chapter 1 now reads *92 of those lines are
comments*. The structure brief's *48%* under *What each chapter's prose owes*
is the brief's own measurement, not a book claim, and is left as written.

**8 · The evidence table classifies ten claims, and the fixtures were read
before being cited.** `cli_metadata_exposes_only_the_bare_entrypoint_and_writes_no_skill_directory`
asserts `--help` and `--version` succeed and `--version`'s exact stdout, from
the crate directory inside this workspace, so the *before the flow* half is
unheld; it asserts `grove do` fails without text or status.
`a_second_driver_refuses_before_tree_access_or_launch` spawns the `grove`
binary through the fixtures' `support` module as a second driver and asserts a
failed status and *existing Grove driver must stop* on stderr, not the `Error:`
prefix or the status `1`. No fixture removes `$HOME`, signals the session
rather than the driver, sends `SIGHUP`, or runs the binary outside a jj
workspace (`grep` over `crates/*/tests`, with `UNSWEPT_DIRECTORIES` read to
confirm the sweep's own scope). Row 4 names `driver-lease-fixture-timing-k85`
as the nearest fixture leaf and says it is not this.

**9 · The test is applied to `grove-llm` from three facts only**: its manifest
lists two workspace dependencies; its `lib.rs` declares the library target and
states why; and the two tests *The surface* and *Proving a negative* already
named. Its manifest's clause that *everything this binary can reach is
something `grove-loop` chose to publish* is not adjudicated here — the binary
also depends on `jj-workspace` directly — because it is another book's corpus;
promoted to `crate-books-k14`'s brief for `grove-llm-structure-k32`.

**10 · Proof, before review.** `book-check --repo . --book
docs/walkthroughs/overview --final --check all` is valid on the first run: 3
files, 204 resolved lines, 0 deferred, `final=true`. `bash scripts/check.sh`
was then started over the finished page, and the one in-session reviewer was
given the page, the corpus, `lib.rs`, the seven manifests, the named fixtures,
the earlier chapters, the brief and the prose contract with a *disprove it*
brief. Any page edit that follows the review is re-proved by `book-check
--final` and by `reference_navigation`, which are the two instruments that
read the page; the umbrella's other six checks do not.

**11 · `bash scripts/check.sh` passed over the finished page: all 8 principal
checks, the `book-check` block reporting the three books `final=true`, and
`cargo test -p grove` 2 unit and 67 integration tests green.** Both transcripts
on the page are quoted from those runs, with the elisions the page names.
Chapter 1's re-export sentence was narrowed after the run — *what the binary
takes from two of them arrives as a `grove-loop` re-export, and it takes
nothing from the third* — and the page's transcript was pasted after it; both
were re-proved by `book-check --final` and `reference_navigation`.

**12 · The one in-session review was spent, and it paid.** Twenty-six
findings, every one classified. Wrong, and fixed: `session_config` is a `pub
mod`, so three loop-owned names come from private modules, not four, and
`TemplateSource` has a second spelling the binary does not use; *every member
reads* the release version (six do); *linked against five* (four); the
`grove` row listed a dev-dependency while the `grove-llm` row did not (the
column now excludes dev-dependencies); the `grove-llm` paragraph said its crate
calls the library-beside-binary shape *weaker* (its root says it costs the
guarantee nothing) and named one boundary where the map shows two; the
boundary table claimed a first glossary citation that *Orientation* already
made; the evidence table's row 4 sentence asserted a leaf that does not exist;
the `E0603` transcript had a one-digit gutter where the elided note makes it
two; `root-init` was dropped from the manifest's clause; the `Error:` prefix
was attributed to this crate where *Three steps* attributes it to the standard
library; row 9 said *workspace* where chapter 4 says *crate*; the opaque-error
row linked the section that names the type and not the one that reads the
clause. One book-wide inconsistency, fixed under the draft's charter:
`README.md` called the third mechanism *a convention nobody checks* where
chapters 1, 4 and 5 call it a convention a test checks; the README now reads
*a convention a test checks, which nobody otherwise would*. Visible trade-offs,
unchanged: the module table is `architecture-move-k31`'s and the page names
`docs/ARCHITECTURE.md` for it (decision 5); `guaranteed-core` is uncited until
that row lands (decision 6); the page's seven members and eleven modules are
checkable against the manifests and `lib.rs` where the brief's *six* and
*twelve* are that document's counts. Noise: three claims the reviewer could
not confirm from the final state alone, each of which the precedent chapter
makes in the same form. Eleven style findings are copy-edit's and are under
`## Handed forward` in `overview-book-k30`'s brief. After the fixes,
`book-check --final` is valid and `reference_navigation`'s twelve tests pass.
