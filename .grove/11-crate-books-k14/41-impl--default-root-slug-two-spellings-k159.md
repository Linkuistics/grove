# default-root-slug-two-spellings-k159

## Goal

Reconcile the two independent spellings of the default root slug — grove-loop's
`DEFAULT_ROOT_SLUG` and the CLI's `#[arg(default_value = "plan")]` — so their
agreement is held by something, and correct `default_root_slug`'s doc comment,
which names a caller it does not have. Inside both files' frozen line counts.

## Context

- **Two literals, each pinned, their agreement pinned by nothing.**
  `crates/grove-loop/src/tree_lifecycle.rs` line 56 is
  `const DEFAULT_ROOT_SLUG: &str = "plan";`.
  `crates/grove-llm/src/cli.rs` line 327 is `#[arg(default_value = "plan")]`,
  with line 326's doc comment saying *Default: `plan`* a third time in prose.
- **Established by mutation**, not by reading: changing the constant to
  `"mutant"` in a workspace copy reddens exactly one test of 558 —
  `tree_lifecycle::tests::transition_initializes_an_absent_grove_under_one_exclusive_guard`,
  which asserts the picked leaf is `01-requirements--plan-k1.md` — while
  `root_init_default_slug_is_plan` in `crates/grove-llm/tests/root_init.rs`
  drives the binary and stays green. Diffed against an 11-failure control run of
  the same copy; the harness is in `no-word-for-k127`'s brief.
- **The two defaults are reached by different doors and both are user-visible.**
  The constant is read only by `transition_to_current` (line 82), the driver's
  own scaffold, which creates a grove when the loop starts against a worktree
  that has none. The clap default is what `grove-llm root-init` with no argument
  uses. A grove scaffolded either way should carry the same first leaf, and
  today that holds by coincidence.
- **The doc comment names a caller it does not have.** Lines 351–352 open *The
  slug `root-init` uses when nobody supplied one, and the only slug the driver's
  own scaffold can use.* `root_init` takes `slug: &Slug` and never falls back;
  `grep -rn --include='*.rs' 'default_root_slug\|DEFAULT_ROOT_SLUG' crates/`
  returns the definition, the constant and **one** call site, line 82. The second
  clause is exactly right and the first is not.
- **Found by `a-grove-begins-k155`** while drafting chapter 11, which adjudicates
  both on the page at `11-a-grove-begins.md#the-value-nothing-holds`.

## Done when

- The agreement is held by something a change would break — the CLI reading the
  crate's constant, or a test asserting the two are equal — rather than by two
  literals that happen to match. A mutation of whichever spelling survives
  reddens at least one test on **both** paths.
- `default_root_slug`'s doc comment names `transition_to_current` as its caller
  and drops the `root-init` clause, or states where the CLI's default actually
  lives.
- **`crates/grove-loop/src/tree_lifecycle.rs` is still exactly 2,725 lines** and
  `crates/grove-llm/src/cli.rs` its own count, so no ownership range, manifest
  `lines` value or fragment range moves. If a fix cannot fit, this leaf says so
  and the ledgers move in the same commit.
- `11-a-grove-begins.md` reproduces the new bytes and its adjudication is
  rewritten to the repaired text; any `concept-index.md` entry naming the defect
  follows.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  at whatever slice the book is proved at when this runs, and every other book
  the commit touched is green.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**Deferred behind the `grove-loop` book**, with `unresolved-doc-links-k151`,
`unreachable-root-clause-k152` and `grow-header-stale-helper-k154`, and for the
same reason: the bytes are reproduced by a finished page and the freeze rule
wants one commit carrying source, ledgers, pages and a green validator. Until
then chapter 11's adjudication stands as the record.

**This defect straddles two books, and both are already written over it.**
`crates/grove-llm/src/cli.rs` is the `grove-llm` book's corpus, and line 327 is
inside fragment `«args-root-init»` (`lines="323-330"`, owner `before-the-lock`)
at `docs/walkthroughs/grove-llm/04-growing-the-tree.md:261`. So a commit that
touches the clap default carries **two** books' pages, ledgers and validator
runs, and `grove-llm`'s is already at green `--final`. Prefer the fix that
changes only `tree_lifecycle.rs` if one exists — a test asserting the two
spellings agree can live in `grove-llm`'s test directory, which is evidence and
not corpus in either book.

## Decisions (running log)
