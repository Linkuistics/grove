# Ordinal filesystem tree — book structure brief

## Status and provenance

This is the structure brief for the book at
[`docs/walkthroughs/ordinal-fs-tree/`](../walkthroughs/ordinal-fs-tree/README.md),
in the form [`walkthrough-books.md`](walkthrough-books.md) requires under *The
structure brief*. It settles what that specification deliberately does not: the
chapter sequence, the mapping of that sequence onto the corpus, each chapter's
worked example, the early uses the order forces, and what the book does not
cover.

**This document is recovered, not authored.** The `ordinal-fs-tree` book's
responsibilities were chosen and written down once, in the one-book
specification `docs/specs/ordinal-fs-tree-book.md`, and
`walkthrough-books-spec-k20` rewrote that document into a specification of the
book *system* — moving the per-book responsibilities out and putting them
nowhere. Every statement below is traceable to that deleted text, readable at
`docs/specs/ordinal-fs-tree-book.md@d2d839bb-`, and specifically to its
*Concept sequence and page responsibilities*, *Book location and pages*,
*Source and ownership ledger*, *Early-use ledger*, *Worked examples*, *Audience
boundary* and *Rejected alternatives and limits* sections.

Nothing here is derived from the finished pages. The finished Markdown shows
what was written, never which responsibilities were binding, and a brief
reconstructed from the pages would record the former while claiming to be the
latter. Where the deleted text is silent, this document says so under
*What the deleted text did not settle* rather than filling the gap in.

**This brief is the human contract; the manifest is its machine-readable form.**
The chapter sequence and ownership mapping below are what
`docs/walkthroughs/ordinal-fs-tree/walkthrough.toml` records as its `[[page]]`
and `[[block]]` groups. Where the two disagree, that is a defect in one of them,
not a licence to prefer either.

## Audience and intended outcome

The book is a self-contained walkthrough for a reader **already proficient in
Rust, common crates, and operating-system APIs**. It introduces the code in
reader-dependency order rather than filesystem order, and reconstructs every
in-scope source file byte for byte from fragments embedded in the Markdown.

Two of the four deliberate properties the deleted specification named are
properties of the *reading order* and so belong to this brief: the numbered
pages form one canonical reading path, and source fragments sit beside the
explanation that gives them meaning. The remaining two — raw Markdown exposing
the fragment graph, and the scoped/final split of the checks — are properties of
the book system and stay in [`walkthrough-books.md`](walkthrough-books.md).

## Chapter sequence

Eight chapters, in this order. `README.md`, `concept-index.md` and
`source-index.md` are the contents page and the two lookup surfaces; they are
not chapters and not alternate explanatory paths.

Slice IDs are opaque book-system tokens. They were derived from the producing
work items, and they remain valid after `.grove/` is removed: slice identity is
deliberately distinct from page ID so a page rename does not migrate fragment
ownership, ledger rows and `--through` values with it.

| Order | File | Page ID | Title | Slice |
|---:|---|---|---|---|
| 1 | `01-orientation.md` | `orientation` | Orientation | `orientation-k11` |
| 2 | `02-name-seam.md` | `name-seam` | Name seam | `name-seam-k12` |
| 3 | `03-reference-domain.md` | `reference-domain` | Reference domain | `reference-domain-k13` |
| 4 | `04-read-path.md` | `read-path` | Read path | `read-path-k14` |
| 5 | `05-mutation-algebra.md` | `mutation-algebra` | Mutation algebra | `mutation-algebra-k15` |
| 6 | `06-filesystem-interpreter.md` | `filesystem-interpreter` | Filesystem interpreter | `filesystem-interpreter-k16` |
| 7 | `07-syllabus-cli.md` | `syllabus-cli` | Syllabus CLI | `syllabus-cli-k17` |
| 8 | `08-invariants-and-trade-offs.md` | `invariants-and-trade-offs` | Invariants and trade-offs | `book-assembly-k18` |

`book-assembly-k18` owns no production source and is therefore final-only: it
has no scoped prefix to prove.

## Concept and seam responsibilities

### `README.md` — reader contract

State the audience, scope, exclusions, source-authority rule, exact-fragment
claim, canonical page order, lookup paths, and the distinction between scoped
and final completeness. Explain how to recognise a fragment definition,
insertion, source root and deferred hole without duplicating the full grammar.

### 1 · Orientation — purpose and one complete operation

Establish the package's purpose and minimum vocabulary: root, entry, leaf, node,
level, ordinal, key, distinguished child, consumer, and operator. Explain the
library/consumer boundary, the feature/dependency boundary, and the public crate
surface from the non-CLI manifest ranges and `src/lib.rs`.

Then follow one `lesson-insert` operation at low resolution through this exact
control path:

```text
CLI parse and run
→ CLI insert helper
→ fs::write
→ WriteGuard::insert with its captured Snapshot
→ ops::insert, which builds the effects and returns
  Plan::of(effects).guarded(snapshot) as a Decision
→ WriteGuard::run
→ fs::apply::apply
→ Run::step and, on failure, Run::unwind
→ Report or Error
→ CLI stdout/stderr rendering and process exit status
```

**The operation is `insert` for a stated reason.** It exposes mutable ordinals,
stable keys, highest-first sibling shifts, ordered effects, guarded planning,
rollback and observable reporting without requiring promotion's exceptional
transient duplicate ordinal/key state.

The trace uses an actual command, an explicit before/after tree, real names and
values, the planned effect order, the resulting report order, stream ownership,
and exit category. It names the exact functions and types above, but it does not
copy source owned by later slices; each later-owned type receives the minimum
local statement recorded in *Early uses* below. That preserves the orientation
slice's fragment ownership while keeping the path technically exact.

### 2 · Name seam — filename algebra and the consumer seam

Explain ordinal versus key, parsing and composition, `Found`, `Verdict`,
`Species`, `EntryName::Parts`, `Triple`, one-component rendering, and
`EntryName`. Connect the seam to the surrounding read and mutation flow rather
than presenting an API catalogue. Show concrete accepted, foreign, malformed and
reserved names without yet treating the syllabus vocabulary as a library
default.

### 3 · Reference domain — one worked consumer

Explain `SyllabusName`, labels, status, lessons, modules, overviews,
distinguished names, positioning, parsing, formatting, recovery advice, and the
conformance kit. Separate assumptions checked by the type system from properties
exercised by reusable conformance checks. Establish examples stable enough for
the remaining pages to reuse with small local restatements.

### 4 · Read path — filesystem discovery to immutable views

Follow a complete read from unfollowed directory entries through filename
classification, recursion, deterministic sibling ordering, snapshot building,
and public `ReadGuard` behaviour. Explain foreign-name skipping versus
malformed/reserved halting, root versus entry views, levels, traversal, key
lookup, predicates, ancestors, distinguished chains, shared locking, and the
snapshot lifetime. State read errors and their recovery meaning locally.

### 5 · Mutation algebra — total decisions without a filesystem

Explain `Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, guarded
planning, and `Report`. Derive append, append-many, insert, promote and rewrite
from one immutable snapshot. Follow the orientation insert in full through
target resolution, hole refusal, highest-first shifting, maximum-key allocation,
effect construction, guarding, and report meaning. Locate each refusal at the
decision that creates it, and state that refusal performs no effects. Keep the
algebra/filesystem seam explicit.

### 6 · Filesystem interpreter — locks, effects, and recovery

Explain presence-shaped read and write openings, shared and exclusive locks,
snapshot timing, the consuming write guard, and vacancy initialisation. Cover
root creation, application, root rollback, name-to-path validation, ordered
effect-specific steps and unwind, successful rollback, partial rollback,
whole-tree deletion, and the `Removed` and `RemovalStopped` reports. State the
error taxonomy, concurrency limits, and crash-visible intermediate states.
Follow both a successful insert and a forward failure.

**Do not call a multi-effect operation atomic.** State what the exclusive lock
hides from cooperating processes and what it cannot protect against.

### 7 · Syllabus CLI — the external consumer and full operation

Explain the demonstration consumer outside `src/`, its CLI-only dependency and
binary declaration, every verb, target syntax, filters, record output,
advisories, refusals, errors, exit categories, idempotency, and omitted
features. Keep operator vocabulary distinct from consumer vocabulary.

Return to the exact command and starting tree used by orientation. Resolve the
same insert at full source resolution through parsing, parts construction,
guard, snapshot, algebra, plan, interpreter, report, stdout/stderr, and exit
status. Use links and brief restatements to revisit earlier source; do not
duplicate fragments.

### 8 · Invariants and trade-offs — assembly and synthesis

Assemble the cross-cutting invariants, model evidence, failure/refusal map,
architecture summary, concurrency and recovery limits, and explicit design
trade-offs. This page owns no production source. It closes the early-use and
source ledgers, confirms every deferral has become an insertion, and records the
final verification commands and results.

## The mapping onto the corpus

The corpus is seventeen source roots, in this fixed order. The order is the one
the source index presents and the one the manifest's `[[root]]` array follows.

| Root ID | Source path | Lines |
|---|---|---:|
| `source-crate-manifest` | `crates/ordinal-fs-tree/Cargo.toml` | 112 |
| `source-syllabus-cli` | `crates/ordinal-fs-tree/bin/syllabus.rs` | 1,738 |
| `source-library` | `crates/ordinal-fs-tree/src/lib.rs` | 103 |
| `source-conformance` | `crates/ordinal-fs-tree/src/conformance.rs` | 667 |
| `source-error` | `crates/ordinal-fs-tree/src/error.rs` | 510 |
| `source-name` | `crates/ordinal-fs-tree/src/name.rs` | 717 |
| `source-operations` | `crates/ordinal-fs-tree/src/ops.rs` | 634 |
| `source-plan` | `crates/ordinal-fs-tree/src/plan.rs` | 597 |
| `source-reference` | `crates/ordinal-fs-tree/src/reference.rs` | 559 |
| `source-report` | `crates/ordinal-fs-tree/src/report.rs` | 186 |
| `source-snapshot` | `crates/ordinal-fs-tree/src/snapshot.rs` | 677 |
| `source-sought` | `crates/ordinal-fs-tree/src/sought.rs` | 132 |
| `source-filesystem-module` | `crates/ordinal-fs-tree/src/fs/mod.rs` | 827 |
| `source-filesystem-read` | `crates/ordinal-fs-tree/src/fs/read.rs` | 407 |
| `source-filesystem-apply` | `crates/ordinal-fs-tree/src/fs/apply.rs` | 488 |
| `source-filesystem-remove` | `crates/ordinal-fs-tree/src/fs/remove.rs` | 275 |
| `source-filesystem-lock` | `crates/ordinal-fs-tree/src/fs/lock.rs` | 91 |

### Top-level ownership blocks

Thirty-three blocks. The State column is the required state after the
orientation slice; a later owner changes only its own rows from `deferred` to
`resolved` when it replaces the matching defer with an insert and adds the
definition.

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-package-and-library-dependency` | `source-crate-manifest` | `orientation-k11` | `1-42` | 42 | `resolved` |
| `manifest-cli-feature` | `source-crate-manifest` | `syllabus-cli-k17` | `43-45` | 3 | `deferred` |
| `manifest-library-cli-boundary` | `source-crate-manifest` | `orientation-k11` | `46-61` | 16 | `resolved` |
| `manifest-cli-binary` | `source-crate-manifest` | `syllabus-cli-k17` | `62-65` | 4 | `deferred` |
| `manifest-development-and-release` | `source-crate-manifest` | `orientation-k11` | `66-112` | 47 | `resolved` |
| `syllabus-cli-source` | `source-syllabus-cli` | `syllabus-cli-k17` | `1-1738` | 1,738 | `deferred` |
| `library-crate-surface` | `source-library` | `orientation-k11` | `1-103` | 103 | `resolved` |
| `reference-conformance-source` | `source-conformance` | `reference-domain-k13` | `1-667` | 667 | `deferred` |
| `filesystem-error-source` | `source-error` | `filesystem-interpreter-k16` | `1-510` | 510 | `deferred` |
| `name-seam-source` | `source-name` | `name-seam-k12` | `1-717` | 717 | `deferred` |
| `mutation-operations-source` | `source-operations` | `mutation-algebra-k15` | `1-634` | 634 | `deferred` |
| `mutation-plan-source` | `source-plan` | `mutation-algebra-k15` | `1-597` | 597 | `deferred` |
| `reference-domain-source` | `source-reference` | `reference-domain-k13` | `1-559` | 559 | `deferred` |
| `mutation-report-source` | `source-report` | `mutation-algebra-k15` | `1-186` | 186 | `deferred` |
| `read-snapshot-source` | `source-snapshot` | `read-path-k14` | `1-677` | 677 | `deferred` |
| `sought-object-answer` | `source-sought` | `name-seam-k12` | `1-132` | 132 | `deferred` |
| `filesystem-read-opening` | `source-filesystem-module` | `read-path-k14` | `1-128` | 128 | `deferred` |
| `filesystem-write-acquire` | `source-filesystem-module` | `filesystem-interpreter-k16` | `129-155` | 27 | `deferred` |
| `filesystem-read-acquire-and-guard` | `source-filesystem-module` | `read-path-k14` | `156-202` | 47 | `deferred` |
| `filesystem-writing-shape` | `source-filesystem-module` | `filesystem-interpreter-k16` | `203-215` | 13 | `deferred` |
| `filesystem-reading-api` | `source-filesystem-module` | `read-path-k14` | `216-248` | 33 | `deferred` |
| `filesystem-writing-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `249-290` | 42 | `deferred` |
| `filesystem-read-guard` | `source-filesystem-module` | `read-path-k14` | `291-304` | 14 | `deferred` |
| `filesystem-write-guard` | `source-filesystem-module` | `filesystem-interpreter-k16` | `305-388` | 84 | `deferred` |
| `filesystem-vacancy-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `389-520` | 132 | `deferred` |
| `filesystem-read-guard-api` | `source-filesystem-module` | `read-path-k14` | `521-534` | 14 | `deferred` |
| `filesystem-write-guard-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `535-812` | 278 | `deferred` |
| `filesystem-read-deref` | `source-filesystem-module` | `read-path-k14` | `813-820` | 8 | `deferred` |
| `filesystem-write-deref` | `source-filesystem-module` | `filesystem-interpreter-k16` | `821-827` | 7 | `deferred` |
| `read-filesystem-source` | `source-filesystem-read` | `read-path-k14` | `1-407` | 407 | `deferred` |
| `filesystem-interpreter-source` | `source-filesystem-apply` | `filesystem-interpreter-k16` | `1-488` | 488 | `deferred` |
| `filesystem-removal-source` | `source-filesystem-remove` | `filesystem-interpreter-k16` | `1-275` | 275 | `deferred` |
| `filesystem-lock-source` | `source-filesystem-lock` | `filesystem-interpreter-k16` | `1-91` | 91 | `deferred` |

### Where a file's concerns split across chapters

**Exactly two roots have more than one owner**, and both splits are deliberate.
Whole-file ownership with no planned holes was considered and rejected: it
minimises author coordination but conflicts with the settled conceptual split.

- **`Cargo.toml`** carries both library and CLI concerns. Orientation owns the
  package, library-dependency, library/CLI-boundary and development/release
  ranges; the syllabus CLI chapter owns the optional CLI feature (`43-45`) and
  the binary declaration (`62-65`).
- **`src/fs/mod.rs`** interleaves read and write/interpreter concerns across
  thirteen alternating blocks. The read path owns the read opening, the read
  acquire and guard, the reading API, the read guard and its API, and the read
  `Deref`; the filesystem interpreter owns the write acquire, the writing shape,
  the writing API, the write guard and its API, the vacancy API, and the write
  `Deref`.

Explicit top-level blocks preserve reader order and make those two real handoffs
visible. Every other root is one full-file top-level block.

### Owned-source totals

Derived from the ownership blocks above, which the deleted specification named
as the design authority for the corpus. The totals count each source line once
at its top-level block; they do not count composite ancestors or lookup prose.

| Slice | Page | Owned lines |
|---|---|---:|
| `orientation-k11` | `01-orientation.md` | 208 |
| `name-seam-k12` | `02-name-seam.md` | 849 |
| `reference-domain-k13` | `03-reference-domain.md` | 1,226 |
| `read-path-k14` | `04-read-path.md` | 1,328 |
| `mutation-algebra-k15` | `05-mutation-algebra.md` | 1,417 |
| `filesystem-interpreter-k16` | `06-filesystem-interpreter.md` | 1,947 |
| `syllabus-cli-k17` | `07-syllabus-cli.md` | 1,745 |
| `book-assembly-k18` | `08-invariants-and-trade-offs.md` | 0 |
| **Total** | 17 source roots | **8,720** |

**One figure is derived rather than transcribed, and deliberately so.** The
deleted specification's own totals table credited `syllabus-cli-k17` with 1,446
lines and the book with 8,421 — but its ownership blocks give that slice
1,738 + 3 + 4 = 1,745, its roots table gives `bin/syllabus.rs` 1,738 lines, and
the file on disk has 1,738 lines. The totals table had drifted against the
blocks table beside it, by 299 lines in exactly that one row; every other row
agrees. The blocks are the authority the deleted text itself nominated, and the
finished book records 8,720, so the drift is in the superseded totals table and
nowhere else. It is recorded here rather than silently corrected, because a
recovered document that quietly repairs its source stops being evidence of what
was decided.

## Worked examples

Seven chapters carry the named example section. The eighth does not: it owns no
production source and its job is synthesis.

| Chapter | Required anchor | Start and observable end |
|---|---|---|
| `01-orientation.md` | `insert-tour` | One command from before-tree through exit status at low resolution. |
| `02-name-seam.md` | `worked-names` | At least one accepted name and one foreign, malformed, or reserved name through observation, parse verdict, and rendering/recovery consequence. |
| `03-reference-domain.md` | `worked-reference-name` | One syllabus entry through parts, formatting, parsing, species, and relevant conformance obligation. |
| `04-read-path.md` | `worked-read` | One directory tree from unfollowed entries through snapshot construction and one public query result. |
| `05-mutation-algebra.md` | `worked-insert-decision` | The orientation insert from target resolution through refusal-or-guarded-plan and report meaning. |
| `06-filesystem-interpreter.md` | `worked-apply-and-unwind` | The same plan through successful application, and a second trace through forward failure and unwind outcome. |
| `07-syllabus-cli.md` | `worked-cli-insert` | The orientation command and starting tree through every source layer, streams, and exit status at full resolution. |

**One example, carried through the book.** The orientation and CLI rows use the
identical command and starting tree — the same `lesson-insert` at low and then
full resolution. The read, mutation and interpreter rows reuse that domain with
only the local setup their own boundaries need.

Each section shows concrete filenames, ordinals, keys, parts, paths, effects,
reports, streams and outcomes when that chapter's boundary uses them. Vocabulary
needed for the trace may precede it.

**The example comes before the catalogue.** No section earlier than a chapter's
worked example may primarily enumerate three or more public queries, operations,
verbs, error variants or conformance obligations; those catalogues follow the
complete example. A reviewer checks that the stated start, the transition steps,
and the observable end are all present.

## Early uses the order forces

Reader-dependency order puts orientation first, and orientation traces an
operation through every layer. Every codebase-specific type it names is
therefore owned by a later chapter. These nine rows are the minimum ledger: each
records the symbol family, the first-use page and anchor, the owning slice, and
the minimum local statement the earlier page must make. Authors add a row before
introducing any additional later-owned type.

| Symbol family | First use | Owner | Minimum local statement |
|---|---|---|---|
| `Ordinal`, `Key`, `Found`, `Verdict`, `Species`, `EntryName` | `01-orientation.md#working-vocabulary` | `name-seam-k12` | Ordinal is mutable sibling position, key is stable tree identity, observed file kind is not followed, verdict separates foreign, accepted, and refused names, species controls file versus directory shape, and EntryName is the consumer parsing and composition seam. |
| `manifest-cli-binary` | `01-orientation.md#package-contract` | `syllabus-cli-k17` | The binary declaration is CLI-owned and deferred; it maps the demonstration executable to its external consumer source and requires the CLI feature. |
| `manifest-cli-feature` | `01-orientation.md#package-contract` | `syllabus-cli-k17` | The optional parser dependency is activated by a later CLI-owned feature range, enabled by default while library consumers may disable default features. |
| `Sought` | `01-orientation.md#public-surface` | `name-seam-k12` | Sought distinguishes a search match from a completed search that matched nothing; nothing is neither a mutation refusal nor an error, while accessors retain Option. |
| `Label`, `Status`, `reference::Parts`, `SyllabusName` | `01-orientation.md#insert-tour` | `reference-domain-k13` | These values are the syllabus consumer's vocabulary and seam implementation, not library defaults. |
| `Snapshot`, `Entry`, `ReadGuard` | `01-orientation.md#insert-tour` | `read-path-k14` | A snapshot is the immutable parsed tree captured under a guard, entries are borrowed views, and a read guard couples a shared lock, caller-spelled root, and snapshot. |
| `Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, `Report` | `01-orientation.md#insert-tour` | `mutation-algebra-k15` | Target names the root or a stable key, new entry carries opaque parts and bytes that may be empty, every input yields refusal or a guarded ordered plan, and the report records landed effects in its documented orders. |
| `WriteGuard`, `Error`, `apply::Faults`, `apply::Run` | `01-orientation.md#insert-tour` | `filesystem-interpreter-k16` | A write guard couples an exclusive lock and snapshot and is consumed by one mutation, errors distinguish refusal, clean rollback, partial rollback, and boundary failure, Faults is a test seam, and Run owns per-plan forward and undo state. |
| `Cli`, `Verb`, `Streams`, `Failure` | `01-orientation.md#insert-tour` | `syllabus-cli-k17` | Parsed verbs drive dispatch, stdout is result data, stderr carries advisories and errors, and failure pairs operator-facing text with an exit category. |

A row is `pending` while its owner is outside the current scoped prefix and
`explained` from the owner's scoped increment onward. The owning slice changes
only its own rows.

**The local statement is judged, not matched.** The validator checks row
structure, owner order, page and anchor existence, and resolution status;
whether a statement is present and sufficient is a technical-review judgement,
and keyword matching is not a substitute for it.

## What the book deliberately does not cover

### Rust and operating-system fundamentals

The book explains surprising uses of Rust or operating-system behaviour **in
this crate**, and does not teach ownership, traits, iterators, filesystem paths,
directories, advisory locks, command-line parsing, or standard test mechanics
from first principles. It does explain codebase-specific lifetime choices, guard
consumption, unfollowed file kinds, filename identity, locking scope, effect
order, and rollback semantics.

### Source outside the seventeen roots

The corpus is exactly the seventeen roots tabulated above. The crate's
`src/fixtures.rs`, its inline `tests.rs` modules, and everything under
`crates/ordinal-fs-tree/tests/` are outside it: no chapter owns them and no
fragment reconstructs them. Tests remain **evidence** the book cites, not source
it reproduces.

### Cross-layer source in orientation

Orientation deliberately owns no source belonging to a later layer. Copying
narrow fragments from every layer into it would make its low-resolution trace
visibly source-backed, but it would violate the settled chapter boundary and
create seven additional cross-slice ownership joins. Exact identifiers, concrete
values, the early-use ledger, and source-backed later chapters give the same
technical trace with a smaller fragment graph.

### An outbound guide link

`ordinal-fs-tree` is a bounded context of its own (`CONTEXT-MAP.md`), and its
walkthrough is the self-contained reader-facing account of the crate: its reader
need not have driven a grove, and the `grove` binary's user guide is not an
entry point for them. A mandatory link from the book into that guide would
assert a dependency the context map denies. The book's manifest carries that as
a `[guide] omitted` reason rather than as an absence, and everything else in
[`walkthrough-books.md`](walkthrough-books.md) binds it unchanged.

## What the deleted text did not settle

Recorded as unsettled rather than filled in, so a later reader can tell a
decision from an invention.

- **The exclusion reasons for non-root files.** The deleted specification fixed
  the seventeen roots and never said *why* `src/fixtures.rs` and the inline
  `tests.rs` modules are outside them. The classes and reasons now in the book's
  manifest (`test-support`, `inline-test-module`, `production-outside-src`) were
  settled later, by the shared specification's *The corpus rule and its
  witness*; they are not recovered from this book's own design record.
- **The CLI's omitted features.** Chapter 7 is required to state the
  demonstration consumer's omitted features, and the deleted text never
  enumerated them. What that list contains is the chapter's to establish from
  the source.
- **A named exclusions section.** The deleted specification had no single
  statement of what the book does not cover. The section above is assembled from
  its audience boundary, its root list, and its rejection of cross-layer
  ownership in orientation — which is why each entry carries its own source
  rather than a common one.
