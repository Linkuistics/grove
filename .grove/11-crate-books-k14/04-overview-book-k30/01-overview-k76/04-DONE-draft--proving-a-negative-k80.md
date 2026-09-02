# proving-a-negative-k80

## Goal

Draft chapter 4 of the overview: slice `closure-proved`,
`04-proving-a-negative.md`, owning `surface-closure-tests` —
`crates/grove/src/cli.rs` lines 54–137, the whole `#[cfg(test)] mod tests`.

## Context

- Draft stage, child 4 of 5 of `overview-k76`. Responsibilities are the
  structure brief's *4 · Proving a negative* section: how a negative surface is
  held; why the assertion is a closure property rather than a list of rejected
  verbs, and that it fails on the next flag too; why both tests read clap's own
  model rather than rendered help; why the `undescribed` walk exists twice
  across two packages (`crates/grove-llm/tests/help_surfaces.rs` is the other
  copy — evidence, cited by path) and what the alternatives would have cost;
  and the empty-description case, where `#[arg(help = "")]` renders exactly
  like the missing doc comment the check rejects.
- The required example anchor is `worked-assertion`: the two tests run against
  a `Cli` that has grown one flag — what `undescribed` collects, what
  `get_arguments` reports once `help` and `version` are filtered out, and the
  exact text each assertion prints. **Measure it**: copy the crate to a scratch
  directory outside the repository, add one undocumented flag to `Cli`, run
  `cargo test`, and quote the real assertion output. Nothing in the repository
  is touched.
- This chapter completes the outcome's three mechanisms: the compiler's
  boundary (chapter 1), the asserted property, and the described-option
  convention, which had already been broken once at `retire-no-launch-help-k21`.
- The brief's *Stated limits* says the prose should not pretend this is the
  chapter for a reader who wants the system rather than the technique.

## Done when

- The block's fragments are defined on the page, its defer is replaced by an
  insert, its ownership row reads `resolved`, and the fragment index has the
  rows.
- Contents, navigation and the concept index are updated.
- `book-check --through closure-proved --check all` is valid: 204 resolved
  lines, 0 deferred, `final=false`. The repository Markdown sweep passes.
  `scripts/check.sh` stays red on `book-check` alone until chapter 5 exists.

## Decisions (running log)

**1 · The block is partitioned into eleven literals along the file's own
seams.** `tests-module-opening` (54–57), the walk's doc comment split at its
three paragraphs — `undescribed-doc-purpose` (58–61, the blank line and the
contract), `undescribed-doc-twice` (62–71), `undescribed-doc-empty` (72–74) —
the walk's two loops `undescribed-arguments` (75–84) and
`undescribed-subcommands` (85–95), and each test's doc comment and body:
`describes-test-doc` (96–101), `describes-test` (102–111), `closure-test-doc`
(112–117), `closure-test-subcommands` (118–125) and `closure-test-arguments`
(126–137, which carries the two closing braces). Blank lines lead the fragment
that follows them, as in the earlier chapters. The doc comment is split three
ways so that each paragraph sits beside the section that argues it.

**2 · Reader order is argument order, not file order.** The module head, then
the closure test (mechanism 2), then the model-not-text reason, then the
described-option test (mechanism 3) and the walk it calls, then the twice
paragraph with its alternatives, then the empty-description paragraph, then
the worked example, then the three mechanisms closed. The second test in the
file is read first because it is the chapter's headline; the walk is read after
the test that calls it because its contract is meaningful only once the
assertion is known.

**3 · The model the tests inspect is unbuilt, and the page says so rather than
repeating the comfortable reading.** Read from clap 4.6.1's source: the
derive-generated `command()` (`clap_derive-4.6.0/src/derives/into_app.rs`)
does not call `build()`, and the `help` and `version` args are pushed only in
`_check_help_and_version`, reached from `_build_self`. So at this checkout the
closure test's filter removes nothing and the convention test walks an empty
list. Measured (decision 5): the unbuilt model of the grown struct reports
`["harness"]`, the built one `["harness", "help", "version"]`. The page states
the observable — the filter makes the assertion read the same whether or not
the model has been built — and attributes no intent the comment does not
state. Chapter 2's sentence about the filter (*they are the parser's, and the
property is about what this crate declared*) stands as written.

**4 · The five names the closure test's doc comment subsumes are recorded as
what they were.** `CHANGELOG.md` v17.0.0 *Removed* lists the `--harness` and
`--no-launch` flags with the legacy launch routing, and the `do`, `migrate`
and `retire` subcommands, so the page states them as the human binary's
former surface removed in one release. The k20/k21 handles are named in no
document but the two test files; the page says the test is the record.

**5 · The worked example is measured, not rendered from convention.** The
whole workspace was copied (rsync, excluding `target`, `.jj`, `.git` and
`.grove`) to the session scratchpad and built there; line 19 of the copy's
`cli.rs` was replaced three ways — an undocumented `--harness`, one with
`help = ""`, and one with a doc comment — and `cargo test -p grove --bin
grove` and the grown binary's `--help` were captured for each. A fourth run
added a probe test printing `get_arguments` ids unbuilt and built, and
`Arg::help("")` versus no help. SHA-256 of the repository's `cli.rs` was taken
before and after and is unchanged
(`1eafc402…5119`); the scratch copy was restored to the same digest. Results:
A and B fail both tests with identical messages and render a byte-identical
padded blank row (two trailing spaces after `<HARNESS>`); C passes the
convention test and fails the closure test alone. The page quotes the
assertion text as printed and notes that the panic line numbers are the grown
file's, three lines past the corpus's.

**6 · Counts stated on the page were checked against the bytes.** The two
`undescribed` bodies (`cli.rs` 75–95 and `help_surfaces.rs` 40–60) differ only
in the closure parameter names, by `diff` after dedenting; both are 21 lines,
and the other copy's `assert_fully_described` is 10, so the comment's *thirty
lines* is read as the walk plus that helper and not restated as exact. The
derive's doc-comment rule — short help from the first paragraph, long help the
whole comment only when there is more than one paragraph — was read from
`clap_derive-4.6.0/src/utils/doc_comments.rs` before the walk section named
it.

**7 · Proof, before review.** `book-check --through closure-proved --check
all` is valid: 3 files, 204 resolved lines, 0 deferred, `final=false`.
`reference_navigation` (5 tests) and `corpus_exception_inventory` (12 tests)
pass.

**8 · `bash scripts/check.sh` ran to completion under a ten-minute limit, and
is red on `book-check` alone.** `cargo fmt`, `shellcheck`, `cargo clippy`,
`plugin install`, both conformance stages and `cargo test` are green — the two
`driver_lease` fixtures `the-surface-k78` recorded as overrunning their
readiness limit under workspace load passed in this run. The `--final` check
over this book reports exactly the shape the node brief predicts: the missing
chapter 5, the contents entry it will link, and the Next this page will gain;
the `jj-workspace` and `ordinal-fs-tree` books are `final=true`. No fragment,
coverage or byte finding is reported in final mode, so all 204 lines already
reconstruct.

**9 · The one in-session review was spent, and it paid.** A fresh context was
given the page, the corpus, the other copy of the walk, the earlier chapters,
the structure brief, the prose contract, the measurement record and the clap
4.6 source with a *disprove it* brief, and returned twenty-one technical
findings and fourteen style findings; every one was classified. It confirmed
all eleven literals byte for byte and the `--help` transcript including its
trailing spaces. Wrong or unsupported, and fixed: the transcript was elided
without saying so (the elisions are now named); *nothing else records* the
k21 fix (the `16.3.0` changelog entry does); k21 was attached to the fix
rather than the shipping; the `[lib]` row said a same-package binary could
name unpublished items. Imprecise, and tightened: the fragment enumeration
missed the closure body's split; *second import* for the first; *anywhere
else* for *outside this binary target*; *thirty lines* glossed as exact; the
two copies' differences (indentation and doc comments too); the `help`
subcommand's absence, now explained by clap's gate on existing subcommands;
the build paths named as exhaustive; the long-help rule's attribute
exception and the dropped trailing period; *the one place* clap's behaviour
is relied on; the re-aligned help rows; `--no-launch` having been boolean;
the two flags belonging to subcommands; `/work/atlas/` for the directory
`grove` is typed in; chapter 1's table having six rows; *every test passed*.
Contract, and fixed: six fragment introductions did not state their role in
the worked example, and one stated no input or output; *the k21 shape* and
*twelve verbs* were not self-contained. Fourteen style findings are copy-edit's
and are under `## Handed forward` in `overview-book-k30`'s brief.

**10 · One finding reached the corpus and chapter 1, and was measured before
anything was changed.** A scratch package with a `[lib]` and a `[[bin]]` beside
it: naming a `pub(crate)` item through the library's path fails with `E0603`;
including the library's source file as a module by `#[path]` compiles and
reaches it. So the manifest's clause holds for a binary target that compiles
the loop's modules as its own crate and not for one that depends on the
library. Chapter 1's `pub(crate)` sentence was narrowed to state both shapes
in this commit, under the draft's technical-truth charter and the precedent of
`three-steps-k79`'s decision 10; chapter 4's `[lib]` row says the same; and
the three prose sites of the clause — the manifest comment (frozen corpus),
`docs/specs/module-decomposition.md` decision 1 and `docs/ARCHITECTURE.md` —
are `bin-target-privacy-claim-k87`, inserted ahead of
`architecture-residue-k75` under the root brief's cross-book rule. Recorded
in the node brief as decisions 18 and 19.
