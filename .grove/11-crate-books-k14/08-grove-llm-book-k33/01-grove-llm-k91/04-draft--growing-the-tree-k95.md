# growing-the-tree-k95

## Goal

Draft chapter 4 of the `grove-llm` book: slice `before-the-lock`,
`04-growing-the-tree.md`, owning `verb-root-init` (`cli.rs` 66–74),
`verbs-growing` (125–216), `args-root-init` (323–330),
`kind-help-and-parse-kind` (331–358), `args-growing` (359–399),
`handler-root-init` (484–513), `handlers-growing` (637–767) and
`presence-rule-and-slug` (826–862) — 376 lines, the heaviest chapter.

## Context

- Draft stage, child 4 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *4 · Growing the tree* section. Thesis: **text before
  lock, presence before mutation.** Walk the four verbs in the brief's order,
  because each adds one lock-scope argument: `root-init` (the vacancy, the
  `match`-not-`let … else` drop order), `leaf-add` (`print_paths` **after** the
  call), `leaf-insert` (`report_insert`, the lint through `writeln!(…).ok()`
  outside the lock), `leaf-decompose` (the inherited kind read through its own
  opening **before** `writable`, because two file descriptions do not share an
  `flock`).
- State what the tree access lock is as this chapter's premise — one paragraph,
  linked to the glossary at `tree-access-lock`. State `flock(2)`'s one relevant
  fact once. Explain `KIND_HELP` as help that lists nothing, `require_declared`
  as the just-in-time presence rule and why it still loads the whole
  configuration, and `--kind` required with no default.
- The methodology in `leaf-add`'s and `leaf-insert`'s help — review chains,
  vendor pairs, integrate placement — is linked to the guide at
  `usage-review-composition` and not explained; the page says what the code
  keeps of it: the atomic list, the silence on failure, and that placement is
  the caller's.
- The required example anchor is `worked-leaf-add`: `leaf-add . rate-limit
  --kind review-impl` — the kinds, the slug and the parent read by their types,
  the presence rule answered from the configuration, the exclusive opening, the
  run landed and `02-review-impl--rate-limit-k4.md` printed; then the same argv
  with `--kind prototype`, which the carried configuration does not declare —
  refused naming the kind and the file that must declare it, no exclusive lock
  taken, the tree byte-identical. `root-init` on a vacancy is a second, shorter
  trace. Measure both refusals against the built binary.
- The self-deadlock is not an ending: `tree_lock.rs` proves it cannot happen,
  and the page says so instead of tracing it. Evidence:
  `session_kind_presence.rs`, `tree_lock.rs`, `composition_verbs.rs`, `leaf.rs`,
  `leaf_ops.rs`, `root_init.rs`. Mark the rows this slice owns (`Kind`, `Slug`;
  `SessionConfig`; the growing handlers) `explained`.

## Done when

- The fragments for the eight blocks are defined on the page, the defers are
  replaced, the ownership rows read `resolved`, the fragment index has their
  rows, and navigation, contents and the concept index are updated.
- `book-check --through before-the-lock --check all` is valid: 816 resolved
  lines, 201 deferred. The repository Markdown sweep passes. `scripts/check.sh`
  stays red on `book-check` alone, by design.
