# the-grammar-k93

## Goal

Draft chapter 2 of the `grove-llm` book: slice `admitted-before-dispatch`,
`02-the-grammar.md`, owning `grammar-cli-and-enum-head` (`cli.rs` 35–65),
`enum-close-and-operation-label` (290–310), `run-admission-and-dispatch`
(412–437) and `openings` (863–903).

## Context

- Draft stage, child 2 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *2 · The grammar and the openings* section: the
  `#[command]` attributes — one version constant read by both binaries,
  `arg_required_else_help`, and why `command` is an `Option` that is never
  `None` past `parse`; `operation_label`; `run` — parse, the unreachable bare
  branch, the current directory, and `admit_ambient_session` **before** the
  exhaustive dispatch, with `--version` exempt only because clap answers it
  first; `worktree` through `Workspace::resolve`; `readable` and `writable`;
  and `absent` — one wording carrying its remedy.
- State what a session epoch is as this chapter's premise — one paragraph,
  linked to the glossary at `session-epoch` and the guide at
  `usage-driver-lease`, never a primer. Cite `driver-lease` beside admission and
  `stated-vcs` beside `Workspace::resolve`.
- The required example anchor is `worked-dispatch`: `grove-llm resolve
  rate-limit-k3` parsed, its operation label, the epoch admitted, the handler
  dispatched; the same argv one directory off (a directory with no `.grove/`),
  refused with `root-init` named as the remedy; and `grove-llm --version`
  answered before admission is reached. **Measure the refusal and the version
  line against the built binary**, as `the-surface-k78` did for the overview.
- **This page must carry `<a id="worked-dispatch"></a>` on its example** and
  state the four handler-family rows' minimum statements there: one handler per
  reading verb, per growing verb, per terminal mark, and the two that open no
  tree. Leave those rows `pending`; mark the three rows this slice owns
  (`SessionEpochGuard`; `Reading`, `Tree`, `Writing`, `TreeWrite`; `Workspace`)
  `explained`.
- **Known in advance**: `cli.rs` lines 39–43 say the package carries a `0.1.0`
  of its own; `Cargo.toml` line 3 is `version.workspace = true`. State the
  checkable fact beside the fragment; `grove-llm-version-comment-k83` owns the
  comment.
- This chapter owns none of the twelve variants. If it names the twelve, it does
  so after the worked example — the catalogue rule binds.

## Done when

- The fragments for the four blocks are defined on the page, the four
  source-index defers are replaced by inserts, their ownership rows read
  `resolved`, and the fragment index has their rows.
- `README.md` links the page, chapter 1's navigation gains a Next, and
  `concept-index.md` has this chapter's entries.
- `book-check --through admitted-before-dispatch --check all` is valid: 226
  resolved lines, 791 deferred. The repository Markdown sweep passes.
  `scripts/check.sh` stays red on `book-check` alone, by design.
