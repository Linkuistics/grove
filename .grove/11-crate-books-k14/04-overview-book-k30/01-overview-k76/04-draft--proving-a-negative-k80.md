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
