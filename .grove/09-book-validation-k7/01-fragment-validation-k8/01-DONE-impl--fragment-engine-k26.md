# fragment-engine-k26


## Goal

Land the byte-level fragment lexer, recursive expansion engine, fixed-corpus
coverage checks, and fragments-only read-only CLI as a coherent foundation for
the remaining validator contracts.

## Context

- Parent contract: `fragment-validation-k8`'s brief and
  `docs/specs/ordinal-fs-tree-book.md`.
- The producer's adversarial pass found that the first implementation mixed
  the engine with ledger, page-placement, and final diagnostic-envelope work;
  those now have their own children.
- Follow Rust style, CLI design, and test-driven-development guidance.

## Done when

- Exact directive forms, ordinary fences, literal fences, LF/CR preservation,
  composite grammar, and safe malformed-input recovery are tested.
- Unique fragment/root identity, unresolved inserts, canonical cycles,
  per-root reachability, parent/source/owner/range partitions, and early or
  overdue deferrals have their required fragment diagnostic classes.
- Scoped ownership states and final expansion cover exactly the fifteen frozen
  source files and 6,618 lines, byte for byte, with a successful recursive
  composite fixture.
- The CLI exposes only the fragment check this increment actually implements,
  loads from explicit `--repo` and normalized `--book`, is non-interactive and
  read-only, and documents scoped and final fragment invocations.
- Crate tests and Clippy pass without warnings. Repository-wide verification is
  run, with unrelated flaky failures isolated and reported.

## Notes

Do not claim ledger tables, canonical page placement, Markdown checking, or the
complete stable diagnostic schema in this child. `ledger-and-pages-k27`,
`diagnostic-contract-k28`, and `markdown-validation-k9` own those increments.

## Decisions (running log)

The public/CLI check surface is fragment-only until `markdown-validation-k9`
lands. Silently treating `all` or `markdown` as fragments was rejected because
it can report false success.

The live frozen sources remain the authority for the exhaustive byte identity
test. A separate committed composite fixture proves recursive success without
deriving the expected expansion through validator code.

Repository-wide verification reached and passed all 497 `grove` library tests,
then the sandbox stalled in `brief_chain::grove_help_does_not_list_brief_chain`
when that integration test launched a freshly built executable. The validator
suite, formatting, and workspace Clippy complete normally; this executable
launch limitation is isolated to the harness rather than a validator failure.
