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

## Notes

**Verified rather than reconstructed.** Every transcript line, refusal and lock
timing on the page was measured against the built binary at `20.1.0`, without a
driver, on scratch Jujutsu workspaces shaped like the carried tree. Measured and
stated: `leaf-add --kind review-impl` landing `02-review-impl--rate-limit-k4.md`;
the same argv with `--kind prototype` refused by `require_declared` naming the
kind and `config.kdl`, tree byte-identical, no lock taken; `--kind Impl` and
`--kind a--b` refused by shape naming `'I'` and the separator; a missing `--kind`
as clap's exit-`2` usage error; `root-init` on a vacancy writing the charter and
`01-requirements--plan-k1.md`, refusing `requirements` when undeclared and
leaving no `.grove/`, and refusing *grove root already exists* on a live grove;
`finish` refused as driver-reserved by all three grow verbs even when the
configuration declares it; `leaf-insert` shifting `02→03`, printing the renumber
summary and the cross-reference lint; `leaf-decompose` inheriting `review-impl`
and overriding it, refusing a brief/retired/node with the verb's own message. The
whole-configuration-load half was measured too: a malformed unrelated entry, and
a duplicate key, fail a declared `leaf-add` before the one kind is asked about.

**Lock timing measured from outside the process.** With an external `flock` held
on the worktree root: an undeclared or ill-formed `leaf-add` returns its refusal
at once (no lock taken), a declared one blocks and prints *waiting for active
Grove tree operation*, `leaf-decompose` with an undeclared override refuses at
once, and `leaf-decompose` inheriting its kind blocks on the shared read then
completes on release — the read and write openings are sequential, never nested.

## Decisions (running log)

**1 · The structure brief is the precondition and was read first.**
`docs/specs/grove-llm-book-structure.md`, section *4 · Growing the tree*, states
this chapter's reader outcome, section plan and emphasis. `grove-draft` and the
editorial family file were read from `plugins/grove/skills/` in the working tree;
the plugin cache still has no `grove:grove-draft`.

**2 · The page reads its eight blocks in argument order, not file order:** the
opening (thesis + lock premise), the worked example (`leaf-add`'s two endings and
`root-init`'s trace), then the four verbs each with its handler and args —
`root-init`, the shared presence machinery (`require_declared`, `slug`,
`KIND_HELP`/`parse_kind`), `leaf-add`, `leaf-insert`, `leaf-decompose` — and the
four help variants last, as the catalogue, under the example-before-catalogue
rule.

**3 · The fragment partition is 22 literals under six composites plus two
bare-literal blocks.** `verb-root-init` (66-74) and `args-root-init` (323-330)
are single-item blocks and are literals directly under `source-command-surface`;
the six composites (`verbs-growing`, `kind-help-and-parse-kind`, `args-growing`,
`handler-root-init`, `handlers-growing`, `presence-rule-and-slug`) each partition
their range. `handler-root-init` splits `cmd_root_init` at the vacancy `match`
so the drop-order argument sits with the code it is about. Bodies were spliced
from the source with exact byte ranges, not retyped.

**4 · The `match`-versus-`let … else` comment is adjudicated as a claim the
source does not bear out.** The comment (`cli.rs` 495-500) argues `match` drops
the unmatched value on entry to the arm and `let … else` holds it after the else
block. Compiled and run under the workspace toolchain in editions 2021 and 2024,
the compiler does the reverse: a `match` scrutinee lives to the end of the
enclosing `let` statement (guard alive through the `Tree(_)` arm body, dropped
after), while a `let … else` initializer's value drops before the else block. So
the lock is held through the `match` failure arm the code uses and released
before a `let … else` failure block — a tree read added to name the live leaf
would deadlock in the form the code chose, not the one the comment rejects. The
code is nonetheless correct today because the arm reads no tree. The page states
the checkable fact and reproduces the comment as written; the rewrite is a defect
leaf's under the freeze, cut beside `grove-llm-version-comment-k83` and ahead of
`architecture-residue-k75`.

**5 · The self-deadlock is stated as a property a test holds, not traced.**
`no_production_lock_grove_takes_for_itself_ever_blocks` scans every production
`flock` and requires each non-blocking; `leaf-decompose`'s inherited-kind read is
structured before `writable` so the two openings are sequential. The page says so
instead of provoking a deadlock.

**6 · The one in-session review was spent, and it paid.** A fresh context was
given the page, the corpus, the loop's verbs/lib/task_name/session_config/
task_grow/tree_lifecycle, the cited tests, the contract and the brief with a
*disprove it* brief and the built binary. It independently reproduced the
drop-order finding in editions 2021 and 2024 and confirmed it, adding one
reinforcement (`TreeWrite::relinquish` is private to `grove-loop`, so `cli.rs`
could not use the safe path even if it wanted to). Eight findings were valid and
fixed: *a test proves it cannot arise* overclaimed — `no_production_lock…` scans
only the five grove packages and not the store's deliberately-blocking `flock`,
so the page now states the property as two facts (grove takes no blocking lock of
its own; the self-deadlock is ruled out structurally, by guard consumption and
`relinquish`); *the lint is the one output written outside the lock* was false
because a mutation releases its guard on return, so every stdout line runs
unlocked — the page now says what is actually distinctive is the second shared
opening and the value-return; `relinquish` drops nothing on the leaf-insert path
because the insert already spent the guard, so the page names it as the
discipline rather than an active release here; the `the_librarys_tree_lock…`
citation for the silent-lint property was a non sequitur and is dropped;
*three streams* is two streams and three outputs; *a bad kind, slug or target is
refused before the lock* narrowed to kind and slug, with the target parsed before
and resolved under the lock; and two *pins it* / *requires it* claims were
softened to what the named test actually asserts. Visible trade-off left as is:
the worked example shows `~/.config/grove/config.kdl` where the binary prints the
absolute path, disclosed in the section's preamble. After the fixes
`book-check --through before-the-lock --check all` is valid (816 resolved, 201
deferred) and `reference_navigation`'s thirteen tests pass.

**7 · The drop-order comment defect is leafed.** A new defect leaf is cut in
`crate-books-k14` beside the other frozen-corpus comment defects and ahead of
`architecture-residue-k75`, to reword `cli.rs` 495-500 in one commit with this
book's `handler-root-init-vacancy` fragment once the book has landed. The page
adjudicates the stale claim now and names no leaf, as `grove-llm-version-comment-k83`'s
does.

**8 · `bash scripts/check.sh` ran over the page and the new leaf, and is red on
`book-check` alone, by design.** Seven of eight checks pass — `cargo fmt`,
`shellcheck`, `cargo clippy`, `plugin install`, `conformance`, the conformance
suite, and `cargo test --locked --workspace`. The three finished books report
`final=true`; the `grove-llm` final run rejects the deferred prefix, reporting
the blocks chapters 5 and 6 own (`verbs-ending`, `args-ending`, `handlers-ending`,
`verbs-leaving`, `args-complete`, `handlers-leaving`), the three pages that do not
yet exist (05, 06, 07), and the forward navigation that wants a `Next` to a page
5 the book has not reached. Every finding is under `docs/walkthroughs/grove-llm`;
nothing else in the repository is touched.
