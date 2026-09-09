# grove-llm — book structure brief

## Status and provenance

This is the structure brief for the book at `docs/walkthroughs/grove-llm/`,
which does not exist yet and which this document is written ahead of, in the
form [`walkthrough-books.md`](walkthrough-books.md) requires under *The
structure brief*. It settles what that specification deliberately does not: the
chapter sequence, the mapping of that sequence onto the corpus, each chapter's
worked example, the early uses the order forces, and what the book does not
cover.

**This document is authored, not recovered, and it precedes its book.** Every
decision below was settled in the `grove-llm-structure-k32` interview and is
recorded, with its rejected alternatives, in that leaf's decision log. It follows
[`jj-workspace-book-structure.md`](jj-workspace-book-structure.md) and
[`overview-book-structure.md`](overview-book-structure.md), the two briefs
elicited before their books, and is uniform with them.

**This brief is the human contract; the manifest is its machine-readable form.**
The chapter sequence and ownership mapping below are what
`docs/walkthroughs/grove-llm/walkthrough.toml` records as its `[[page]]` and
`[[block]]` groups. Where the two disagree, that is a defect in one of them, not
a licence to prefer either.

**What is different about this corpus.** Four roots and 1,017 lines, of which
`src/cli.rs` is 944: the book's structural problem is how to divide one large
file, and the answer below divides it by concept rather than by file order, at
the price of twenty-two ownership blocks in that one root. There is no inline
test module — `crates/grove-llm/tests/` is nine times the corpus and all of it
is evidence — and `docs/ARCHITECTURE.md` keeps no descriptive passage this book
makes redundant: every residue mark in that document names another crate's
book, so `architecture-residue-k75` owes nothing on this book's account.

## Audience and intended outcome

The audience is settled by decision 7 of `plan-k1` and is not re-opened here: a
reader who knows Rust and Jujutsu and has driven a grove, for whom grove's
vocabulary is linked to the glossary and never re-taught, and whose entry point
to the system is [`USAGE.md`](../USAGE.md). This reader has already run most of
these twelve verbs, and the guide has already told them what each does to the
tree; what they have not read is the code.

**The intended outcome is the what-is-left test.** At the end the reader can
take a thin command surface over a library in their own code, ask *what is left
here that is not rendering*, and answer in three parts: which text is parsed
before which lock, which check runs before which mutation, which admission
precedes which signal. For each part they can say what reversing the order costs
— a process blocking against itself, a tree mutated by a command that was
refused, a signal sent to a loop that did not launch this session — and name the
test that would catch it. All three are provable inside this book's 1,017 lines,
and each has a named test:

- the read before the write opening in `cmd_leaf_decompose`, held by
  `no_production_lock_grove_takes_for_itself_ever_blocks` in
  `crates/grove-llm/tests/tree_lock.rs`;
- the presence rule before the exclusive opening in every grow verb, held by
  `leaf_add_refuses_a_kind_no_template_resolves_for_and_mutates_nothing` in
  `crates/grove-llm/tests/session_kind_presence.rs`; and
- the admission before dispatch in `run`, and the channel check before the
  write in `cmd_complete`, held by
  `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` in
  `crates/grove-loop/tests/driver_lease.rs`, which drives the real binary.

The closing chapter states the test and applies it to all twelve verbs.

**The stream contract is the rendering half of the same thesis, not a second
outcome.** Data on stdout, advice on stderr, exit zero for information, every
refusal carrying its remedy: the book teaches all of it, and chapter 7 tabulates
it, but as the complement of *order is what is left* rather than as a
transferable claim of its own.

Three candidate outcomes were rejected. **The maintainer outcome** — add a
thirteenth verb without breaking admission, text-before-lock,
presence-before-mutation or stdout-is-data — follows for free from
source-exactness plus chapter 7's table and is therefore not the stated goal,
as both precedents found for theirs. **A reference card for the twelve verbs** is
already [`USAGE.md`](../USAGE.md)'s row for the human and `--help`'s for the
LLM; a source-exact book is a poor way to deliver a third copy. **Take it as a
dependency** is ruled out by fact: [`RELEASING.md`](../RELEASING.md), *One
release, six packages, one tag*, settles that this package ships inside grove's
cut, and it is a binary in any case.

## The spine: order is what is left

**Every chapter opens on the one thing a thin binary still has to get right at
that point in a session.** The spine is recovered from the source rather than
imposed on it. `src/cli.rs`'s own header, lines 10–23, states it: every verb is
one `grove_loop::verbs::` call plus rendering, and what is left that is not
rendering is *order* — the operator's text read by the type that owns it before
a lock is taken, the just-in-time presence rule asked before the mutation, and
the session admitted against the completion channel before it is written to.
The chapters follow the order a session meets the verbs, and each carries the
rule that family enforces:

| The family | The rule it opens on | Chapter |
|---|---|---:|
| the manifest, the library root, the entry point, the header | thin is held by the crate boundary, and order and rendering are what thin leaves behind | 1 |
| the grammar, dispatch and the two openings | every verb is admitted before it is dispatched, and a grove that is not there is not a grove that is finished | 2 |
| `pick`, `brief-chain`, `kind`, `resolve` | an absent answer is information, not an error | 3 |
| `root-init`, `leaf-add`, `leaf-insert`, `leaf-decompose` | text before lock, presence before mutation | 4 |
| `leaf-retire`, `leaf-prune` | the last grove verbs a session runs say on stderr what remains | 5 |
| `finish-commit`, `complete` | admit against the channel before writing to it | 6 |

Three alternatives were rejected. **The session's verb order with no
per-chapter thesis** — the guide's four groups with fragments attached — would
make the book [`USAGE.md`](../USAGE.md)'s account of what each verb does to the
tree, restated with code; that account is the guide's row and the book does not
own it. **Thin, held by the compiler** as the whole spine is the overview's
chapter-1 mechanism and its stated outcome, which this book would re-prove over
twelve verbs; it is stated here once, in chapter 1, as the ground the rest of
the book stands on. **The seam** — organising by what each verb opens: a shared
read, an exclusive write, the vacancy, no tree at all — surfaces the lock
discipline but sorts twelve verbs by an implementation detail.

## Chapter sequence

Seven pages, six of which own source. `README.md`, `concept-index.md` and
`source-index.md` are the contents page and the two lookup surfaces; they are
not chapters and not alternate explanatory paths.

| Order | File | Page ID | Title | Slice |
|---:|---|---|---|---|
| 1 | `01-orientation.md` | `orientation` | Orientation | `one-call-plus-rendering` |
| 2 | `02-the-grammar.md` | `the-grammar` | The grammar and the openings | `admitted-before-dispatch` |
| 3 | `03-reading-the-tree.md` | `reading-the-tree` | Reading the tree | `information-not-error` |
| 4 | `04-growing-the-tree.md` | `growing-the-tree` | Growing the tree | `before-the-lock` |
| 5 | `05-ending-work.md` | `ending-work` | Ending work | `two-steps-remain` |
| 6 | `06-leaving-the-loop.md` | `leaving-the-loop` | Leaving the loop | `admit-before-signal` |
| 7 | `07-what-order-holds.md` | `what-order-holds` | What order holds | `assembly` |

`assembly` owns no production source and is therefore final-only: it has no
scoped prefix to prove.

**Slice IDs are named for the rule each chapter carries**, so the slice list
reads as the book's spine, and no slice token equals any page ID. That
difference is by construction, and it is the property the specification's
separate slice domain exists for: a presentation-level page rename must not
become a fragment-ownership, ledger, manifest and CLI migration. Slice IDs carry
no Grove task key, for the reason recorded in `jj-workspace-structure-k17`'s
decision 8, applied here rather than re-elicited.

**The pages are cut by verb family, not by the file's own order.** `cli.rs` is
ordered by Rust convention — the command enum, then the argument structs, then
the handlers, then the helpers — so a family's help text sits two hundred lines
from its arguments and four hundred from its handler. Each chapter gathers its
family's variants, argument structs, handlers and helpers on one page, beside
the rule they enforce. That is what the spine asks for, and it is what makes
*hold the help to the handler* (below) possible on a single page.

**Chapter 4 is one chapter, at 37% of the corpus, rather than two.** It is one
rule with four instances, and each instance adds one lock-scope argument in
order: `root-init`'s drop order for the live `TreeWrite`, `leaf-add` printing
only after the run landed, `leaf-insert`'s cross-reference lint outside the
lock, and `leaf-decompose`'s read of the inherited kind before the write
opening. Splitting `leaf-decompose` into a 78-line chapter of its own would
carry the last step of an argument the other three build; keeping the whole
`Command` enum in chapter 2 as *the contract as `--help` renders it* would put
204 lines of help text before any handler and make the catalogue rule bind a
whole chapter; file order would give the fewest blocks and no chapter a rule to
open on. The precedent for a heavy chapter is `jj-workspace`'s 230-line
`refusal`, kept whole on the same ground.

## Concept and seam responsibilities

### `README.md` — reader contract

State the audience, scope, exclusions, source-authority rule, exact-fragment
claim, canonical page order, lookup paths, and the distinction between scoped
and final completeness. Explain how to recognise a fragment definition,
insertion, source root and deferred hole without duplicating the full grammar.
State the what-is-left outcome. Cite [`USAGE.md`](../USAGE.md) at
`usage-tree-verbs` — the one guide link the contract permits and requires from
this page, and the section whose subject is exactly this crate's surface. State
the book's boundary: it explains one module, `grove_llm::cli`, and stops at
`grove_loop::verbs`.

### 1 · Orientation — one call plus rendering

Owns `Cargo.toml`, `src/lib.rs` and `src/main.rs` whole, and `src/cli.rs` lines
1–34: the header comment and the import block. Responsible for: what `grove-llm`
is and who drives it — the LLM inside a session, deterministically, which is the
audience-split decision the header cites; the manifest's crate-not-a-target
argument, stated once as the ground every later chapter stands on; why there is
a library target as well as the binary — the clap surface can be inspected by
this crate's own tests, and the library costs the guarantee nothing because the
code the binary must not reimplement is in another crate either way; the
three-line `main`; the header's thesis — one call plus rendering, and the three
orders — which is the book's map; the import block read as the evidence of what
the binary reaches, fourteen `grove_loop` items and one `jj_workspace` type; the
four dev-dependencies and which test needs each — `keyed-launch` for the
completion channel's own framing, `libc` for holding the tree lock from outside
the binary, `ordinal-fs-tree` for reading a filename back through the seam
production uses, `tempfile`; and `release = false` as an answered question.

This chapter reads the clauses recorded under *Known in advance* below — all
of them corrected in the source since drafting — beside the fragments that
carry them. It carries the worked example at
low resolution.

### 2 · The grammar and the openings — admitted before dispatch

Owns `cli.rs` lines 35–65, 290–310, 412–437 and 863–903: the `Cli` struct and
the command enum's head, the enum's close and `operation_label`, `run`, and the
four shared openings. Responsible for: the `#[command]` attributes — one version
constant read by both binaries, `arg_required_else_help`, and why `command` is
an `Option` that is never `None` past `parse`; the enum head and
`operation_label`, the label each verb is admitted under; `run` — parse, the
unreachable bare-invocation branch, the current directory, and
`admit_ambient_session` **before** the exhaustive dispatch, so every verb,
`complete` included, is admitted against the live session epoch, and
`--version` is exempt only because clap answers it before `run` is entered;
`worktree` resolving the working tree through `Workspace::resolve` so the grove
root is spelled in one place; `readable` and `writable` as the shared and the
exclusive opening; and `absent` — *a grove that is not there is not a grove that
is finished* — one wording carrying its remedy, so a session run one directory
off is refused rather than told its work is done.

State here, as this chapter's premise, what a session epoch is — one paragraph,
linked to the glossary at `session-epoch` and to the guide at
`usage-driver-lease`, never a primer. This chapter owns none of the twelve
variants; if it names the twelve, it does so after its worked example, and the
catalogue rule binds.

### 3 · Reading the tree — information, not error

Owns `cli.rs` lines 75–124, 514–636 and 904–944: the `pick`, `brief-chain`,
`kind` and `resolve` variants, their handlers with `leaf_in` and
`render_resolution`, and the path and label helpers. Thesis: an absent answer is
information, so `pick` on a finished grove prints its diagnostic on stderr and
exits zero, `resolve` reports not-found and ambiguity the same way, and a `DONE`
or `ABANDONED` match prints its path **and** a note, so a resolved dead end
never looks live. Responsible for: the four contracts as their help states
them; `leaf_in` — the named leaf, or `pick`'s next; `normalize_leaf_path` and
its three cases; `label` and `no_live_leaves` — one diagnostic, printed once;
the root as the one answer with no entry behind it, whose path is the caller's
own spelling; and `render_resolution` — the crate's one pure function, `pub` and
`#[must_use]` because it is unit-tested through the library target, and living
here rather than in `grove-loop` because it is presentation and nothing else.
Link the glossary at `task-tree-scheme` beside the reference grammar. Evidence:
`pick.rs`, `brief_chain.rs`, `kind.rs`, `resolve.rs` and `resolve_rendering.rs`
in this crate's tests.

### 4 · Growing the tree — before the lock

Owns `cli.rs` lines 66–74, 125–216, 323–330, 331–358, 359–399, 484–513,
637–767 and 826–862: the `root-init` variant and the three grow variants, their
argument structs, the two `--kind` help constants with `parse_kind`, the
handlers with `print_paths`, `report_insert` and `inherited_kind`, and
`require_declared` with `slug`. Thesis: **text before lock, presence before
mutation.** Every grow verb reads its text by the grammar's own types — `slug`,
`parse_kind`, `Reference::parse` — and asks `require_declared` whether each kind
resolves to a template, and only then opens the tree for writing; a refusal at
either step leaves the tree byte-identical and takes no exclusive lock.

Walk the four verbs in this order, because each adds one lock-scope argument:

1. **`root-init`**, the vacancy case and the rule in its simplest form —
   `Kind::requirements()` fixed, the slug read before the lock, and the
   drop-order fact about how long the live `TreeWrite` — and its exclusive
   lock — outlives the `match` arm that refuses.
2. **`leaf-add`** — the ordered list of kinds as one atomic run, and
   `print_paths` **after** the call, never per leaf, because a run that fails is
   rolled back and stdout must not describe files that are no longer there.
3. **`leaf-insert`** — the same order, then `report_insert`: the path on
   stdout, the renumber summary on stderr, and the cross-reference lint written
   through `writeln!(…).ok()` after the insert has landed and **outside** the
   lock, so a stderr that has stopped draining blocks this process and no other.
4. **`leaf-decompose`** — the inherited kind read through its own opening
   **before** `writable`, because two file descriptions do not share an
   `flock` and reading under the write opening would block this process against
   itself; and the verb's own refusals — a brief, a retired leaf, a malformed
   name — left to the verb rather than pre-empted by a configuration complaint.

Explain `KIND_HELP` as help that teaches the shape and lists nothing, because
grove holds no list of kinds; `require_declared` as the just-in-time presence
rule that replaced an all-kinds completeness check grove can no longer make, and
why it still loads the whole configuration to ask about one kind; and the
`--kind` flag being required with no default, because a default is a kind
literal under a friendlier name and the one that would produce a *wrong* leaf
rather than an error.

State here, as this chapter's premise, what the tree access lock is — one
paragraph, linked to the glossary at `tree-access-lock`. The methodology in
`leaf-add`'s and `leaf-insert`'s help — review chains, vendor pairs, where an
integration is placed — is linked to the guide at `usage-review-composition`
and not explained; the page says what the code keeps of it, which is the atomic
list, the silence on failure, and that placement is the caller's. Evidence:
`session_kind_presence.rs`, `tree_lock.rs`, `composition_verbs.rs`, `leaf.rs`,
`leaf_ops.rs` and `root_init.rs`.

### 5 · Ending work — two steps remain

Owns `cli.rs` lines 217–247, 400–411 and 768–825: the `leaf-retire` and
`leaf-prune` variants, their argument structs, and the handlers with
`eprint_next_steps`. Thesis: retire and prune are the terminal-marking pair and
the **last grove verbs a session runs** — Retire precedes Commit, and the commit
is jj's — so their stderr names the two steps that remain, at the moment of
decision rather than a whole session earlier in the mandate; stdout stays data.
Responsible for: `eprint_next_steps` and its plural; retire's refusals — a
brief, a `DONE` leaf, an `ABANDONED` leaf — as the verb's, and its promise that
task bodies are byte-identical; prune's **HITL** rule as help the code cannot
enforce, since grove guides and does not gate; prune's node case, which marks
every live leaf, leaves `DONE` ones alone and says so on stderr; and the reminder
printed last and only when something was marked, since a no-op prune leaves no
session to close. Link the glossary at `task-commit-boundary` beside the
reminder. Evidence: `leaf_ops.rs` and `reviewed_producer_lifecycle.rs`.

### 6 · Leaving the loop — admit before signal

Owns `cli.rs` lines 248–289, 311–322 and 438–483: the `finish-commit` and
`complete` variants, `CompleteArgs`, and the two handlers. Thesis: the
completion channel is resolved and checked against the admitted epoch
**before** it is written, because an answer that came back with the signal
would come back too late; and `finish-commit` reads the operator's handle by the
type that owns the grammar, lenient on the key, and quotes the operator's own
spelling once, in the last frame that still has it. Responsible for:
`CompleteArgs` — `--done` and `--signal-file` defaulting from the environment;
`cmd_complete`'s order and its two endings, `Wrote` and `NoLoop`; the help's
argument that the verb writes a flag and returns because ending the session is
the driver's job and an in-agent self-kill is something a sandbox may silently
deny; `cmd_finish_commit` — `Handle::parse`, `Workspace::resolve`, the change id
on stderr — and the help's account of teardown as a deletion and a path-scoped
commit with jj's own `restore` and `undo` as the remedy. These are the two verbs
that open no tree, and the chapter says so. Link the glossary at
`loop-control-channel` beside the channel and the guide at `usage-finish` beside
`finish-commit`. Evidence: `complete.rs` and `finish_commit.rs` here, and
`grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` in
`crates/grove-loop/tests/driver_lease.rs`.

### 7 · What order holds — assembly and the transferable test

Owns no production source. Assemble the twelve verbs into one table — which
text each parses, which opening it takes, which check precedes its mutation,
what its stdout carries, what its stderr carries, and the test that holds it —
and read the table as the what-is-left test with the stream contract as its
rendering half. State what the compiler holds (the crate boundary), what order
holds (the three), and what tests hold (the flat surface, the described options,
the instructed set). State where the book stops — `grove_loop::verbs` — and
that this crate's test directory also carries the suite that compares the
shipped methodology against the binary, which the book cites and does not
explain. Close the early-use and source ledgers, confirm every deferral has
become an insertion, and record the final verification commands and results.

## The mapping onto the corpus

Four source roots, in this fixed order — the order the source index presents
and the manifest's `[[root]]` array follows.

| Root ID | Source path | Lines |
|---|---|---:|
| `source-crate-manifest` | `crates/grove-llm/Cargo.toml` | 54 |
| `source-library-root` | `crates/grove-llm/src/lib.rs` | 16 |
| `source-entry-point` | `crates/grove-llm/src/main.rs` | 3 |
| `source-command-surface` | `crates/grove-llm/src/cli.rs` | 944 |
| **total** | | **1,017** |

**The corpus rule needs no exceptions.** `include` is the two base patterns
fixed by `[book].subject` — the exact path `crates/grove-llm/Cargo.toml` and
the pattern `crates/grove-llm/src/**/*.rs` — and that pair yields exactly these
four roots: there is no `bin/`, no test-support module under `src/` and no
inline `tests.rs`. The manifest carries no `[[corpus.add]]` and no
`[[corpus.exclude]]`, which is what
[`walkthrough-books.md`](walkthrough-books.md)'s exception inventory already
records by giving this book no row.

**`[book].subject` is `crates/grove-llm`**, fixed by the subject inventory in
[`walkthrough-books.md`](walkthrough-books.md) and not this book's to choose.
`[book].title` is `grove-llm`, and `[book].id` — hence the directory name — is
`grove-llm`.

### Top-level ownership blocks

Twenty-five blocks. The State column is the required state after the
`one-call-plus-rendering` slice; a later owner changes only its own rows from
`deferred` to `resolved` when it replaces the matching defer with an insert and
adds the definition.

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---:|---|
| `manifest-thin-by-crate` | `source-crate-manifest` | `one-call-plus-rendering` | `1-54` | 54 | `resolved` |
| `library-root` | `source-library-root` | `one-call-plus-rendering` | `1-16` | 16 | `resolved` |
| `entry-point` | `source-entry-point` | `one-call-plus-rendering` | `1-3` | 3 | `resolved` |
| `surface-thesis-and-imports` | `source-command-surface` | `one-call-plus-rendering` | `1-34` | 34 | `resolved` |
| `grammar-cli-and-enum-head` | `source-command-surface` | `admitted-before-dispatch` | `35-65` | 31 | `deferred` |
| `verb-root-init` | `source-command-surface` | `before-the-lock` | `66-74` | 9 | `deferred` |
| `verbs-reading` | `source-command-surface` | `information-not-error` | `75-124` | 50 | `deferred` |
| `verbs-growing` | `source-command-surface` | `before-the-lock` | `125-216` | 92 | `deferred` |
| `verbs-ending` | `source-command-surface` | `two-steps-remain` | `217-247` | 31 | `deferred` |
| `verbs-leaving` | `source-command-surface` | `admit-before-signal` | `248-289` | 42 | `deferred` |
| `enum-close-and-operation-label` | `source-command-surface` | `admitted-before-dispatch` | `290-310` | 21 | `deferred` |
| `args-complete` | `source-command-surface` | `admit-before-signal` | `311-322` | 12 | `deferred` |
| `args-root-init` | `source-command-surface` | `before-the-lock` | `323-330` | 8 | `deferred` |
| `kind-help-and-parse-kind` | `source-command-surface` | `before-the-lock` | `331-358` | 28 | `deferred` |
| `args-growing` | `source-command-surface` | `before-the-lock` | `359-399` | 41 | `deferred` |
| `args-ending` | `source-command-surface` | `two-steps-remain` | `400-411` | 12 | `deferred` |
| `run-admission-and-dispatch` | `source-command-surface` | `admitted-before-dispatch` | `412-437` | 26 | `deferred` |
| `handlers-leaving` | `source-command-surface` | `admit-before-signal` | `438-483` | 46 | `deferred` |
| `handler-root-init` | `source-command-surface` | `before-the-lock` | `484-513` | 30 | `deferred` |
| `handlers-reading-and-rendering` | `source-command-surface` | `information-not-error` | `514-636` | 123 | `deferred` |
| `handlers-growing` | `source-command-surface` | `before-the-lock` | `637-767` | 131 | `deferred` |
| `handlers-ending` | `source-command-surface` | `two-steps-remain` | `768-825` | 58 | `deferred` |
| `presence-rule-and-slug` | `source-command-surface` | `before-the-lock` | `826-862` | 37 | `deferred` |
| `openings` | `source-command-surface` | `admitted-before-dispatch` | `863-903` | 41 | `deferred` |
| `path-and-label-helpers` | `source-command-surface` | `information-not-error` | `904-944` | 41 | `deferred` |

The blocks of `source-command-surface`, in array order, are ordered, adjacent,
non-overlapping, and cover 1 to 944 exactly. Every boundary falls on a
structural line: a block ends on the blank line after an item or on an item's
last line, and the next begins on the next item's first attribute, doc comment
or head. The other three roots are single full-file blocks.

### Where a file's concerns split across chapters

**Exactly one root has more than one owner, and it has six.** `src/cli.rs`
carries every chapter's concern except the close's, in four passes the file
makes in Rust's conventional order — the command enum, the argument structs,
the handlers, the helpers — while the book gathers each verb family from all
four:

- **the grammar and the openings** owns the `Cli` struct and the enum's head
  (`35-65`), the enum's close and `operation_label` (`290-310`), `run`
  (`412-437`), and `worktree`, `readable`, `writable` and `absent` (`863-903`);
- **reading the tree** owns its four variants (`75-124`), its four handlers with
  `leaf_in` and `render_resolution` (`514-636`), and `normalize_leaf_path`,
  `label` and `no_live_leaves` (`904-944`);
- **growing the tree** owns the `root-init` variant (`66-74`) and the three
  grow variants (`125-216`), `RootInitArgs` (`323-330`), the two `--kind`
  constants with `parse_kind` (`331-358`), the three grow argument structs
  (`359-399`), `cmd_root_init` (`484-513`), the three grow handlers with
  `print_paths`, `report_insert` and `inherited_kind` (`637-767`), and
  `require_declared` with `slug` (`826-862`);
- **ending work** owns its two variants (`217-247`), its two argument structs
  (`400-411`), and its two handlers with `eprint_next_steps` (`768-825`);
- **leaving the loop** owns its two variants (`248-289`), `CompleteArgs`
  (`311-322`), and its two handlers (`438-483`);
- **orientation** owns the header and the imports (`1-34`).

Two consequences are accepted rather than engineered around. **`RootInit` is
the enum's first variant and a grow verb**, so chapter 4's first block is nine
lines inside the enum ahead of chapter 3's fifty; moving `root-init` to chapter
2 would put a mutation in the chapter that opens on admission and the openings.
**The `--kind` help constants sit between `RootInitArgs` and `LeafAddArgs`**
and go to chapter 4 with both, so `KIND_OVERRIDE_HELP` is owned three verbs
before `leaf-decompose` reads it — a backward reference in the ordinary
direction. Whole-file ownership, and keeping the enum whole in chapter 2, were
both considered and are rejected under *Chapter sequence*.

### Owned-source totals

| Slice | Page | Owned lines |
|---|---|---:|
| `one-call-plus-rendering` | `01-orientation.md` | 107 |
| `admitted-before-dispatch` | `02-the-grammar.md` | 119 |
| `information-not-error` | `03-reading-the-tree.md` | 214 |
| `before-the-lock` | `04-growing-the-tree.md` | 376 |
| `two-steps-remain` | `05-ending-work.md` | 101 |
| `admit-before-signal` | `06-leaving-the-loop.md` | 100 |
| `assembly` | `07-what-order-holds.md` | 0 |
| **Total** | 4 source roots | **1,017** |

The scoped-slice domain derived from these blocks is `one-call-plus-rendering`,
`admitted-before-dispatch`, `information-not-error`, `before-the-lock`,
`two-steps-remain`, `admit-before-signal` — six accepted `--through` values,
with `assembly` final-only.

## What each chapter's prose owes

Measured before deciding: **45% of the corpus is comment prose** — `cli.rs` 426
of 944 lines, `Cargo.toml` 23 of 54, `lib.rs` 14 of 16 — and **204 of `cli.rs`'s
comment lines are the doc comments on the twelve `Command` variants**, which are
the `--help` text the LLM reads and the guide paraphrases for the human. The
fragment graph quotes them verbatim on the page anyway. The standing risk for
this book is therefore paraphrase, and a sharper form of it than either
precedent faced: three documents about the same twelve verbs. Three things a
comment structurally cannot do are what each chapter's prose owes, and a
technical review checks for them:

1. **Adjudicate the claim.** For every argued claim, name the behaviour it
   rests on, the test that proves it, and the alternative rejected with what it
   would have cost. A doc comment cannot cite an integration test in another
   file, and this crate's evidence is nine times its corpus.
2. **Carry the through-line.** Show where a decision in one root is only
   explicable by a decision in another: the manifest's separate-crate argument
   is *why* `lib.rs` exists at all, which is *why* `render_resolution` is `pub`
   and `#[must_use]`; `worktree` resolving through `Workspace::resolve` is *why*
   `normalize_leaf_path` may pass a bare grove-relative name through for the
   verb to join; and `absent`'s one wording is *why* `readable` and `writable`
   can refuse the same way. No single comment holds any of those.
3. **Hold the help to the handler.** For every promise a verb's doc comment
   makes, name the line in the handler that keeps it and the test that would
   catch its breach. `leaf-add` promises to print nothing at all if the run
   could not be created, and `print_paths` after the call is what keeps it;
   `leaf-prune` promises to write no producer receipt, and
   `reviewed_producer_lifecycle.rs` holds it; `resolve` promises to exit zero on
   a reference it cannot place, and `cmd_resolve`'s `Ok(())` after `eprint!` is
   the whole mechanism. Where a doc comment carries **methodology** rather than
   mechanism — the fifty lines on review chains, vendor pairs and integrate
   placement under `leaf-add` and `leaf-insert` — the page links the guide and
   explains only what the code keeps of it.

The third is this book's own. Rejected: supplying the loop's half at the seam —
what each `grove_loop::verbs` call takes, returns and does not promise — which is
the overview's third obligation restated and belongs under *What the book
deliberately does not cover*; naming the session's half — what the methodology
does with each output — which the doc comments already do for the LLM and which
would restate the guide; and no third obligation, which leaves the paraphrase
risk unaddressed for a corpus where the help text is a fifth of the whole.

## Worked examples

Six chapters carry the named example section. The seventh does not: it owns no
production source and its job is synthesis.

| Chapter | Required anchor | Start and observable end |
|---|---|---|
| `01-orientation.md` | `one-session` | The carried session at low resolution: the mandated handle resolved, the brief chain printed, the work done, a review leaf added, the session's own leaf retired, the commit, and `complete` writing the relaunch flag — one tree, one live leaf, every verb named, no handler shown. |
| `02-the-grammar.md` | `worked-dispatch` | The session's first verb as argv: `grove-llm resolve <handle>` parsed, its operation label, the epoch admitted, the handler dispatched; the same argv one directory off, refused with `root-init` named as the remedy; and `grove-llm --version` answered before admission is reached. |
| `03-reading-the-tree.md` | `worked-resolve` | `resolve` at full resolution with three renderings: the live leaf's path on stdout and nothing on stderr; the same handle after retirement — the path, and the note; and a bare slug two entries share — empty stdout, the keys on stderr, exit zero. `brief-chain` for the same leaf follows on the same values. |
| `04-growing-the-tree.md` | `worked-leaf-add` | `leaf-add . <stem> --kind review-impl`: the kinds, the slug and the parent read by their types, the presence rule answered from the configuration, the exclusive opening, the run landed and its path printed; then the same argv with a kind no template declares — refused naming the kind and the file that must declare it, no exclusive lock taken, the tree byte-identical. `root-init` on a vacancy is a second, shorter trace. |
| `05-ending-work.md` | `worked-retire` | `leaf-retire` of the session's own leaf: the renamed path on stdout and the two remaining steps on stderr; `leaf-prune` of a node shown once, with its untouched `DONE` leaf reported. |
| `06-leaving-the-loop.md` | `worked-complete` | `complete` with the loop's channel in the environment: the channel resolved, checked against the admitted epoch, written, and *the loop will start the next task* on stderr; the same verb with no channel — `NoLoop`, and the instruction to exit manually. `finish-commit finish-k0001` is a second, shorter trace: accepted leniently, committed as `finish-k1`, the change id on stderr. |

**One session, carried through the book, with the presence refusal as its
second ending.** The carried example is grove's real usage of this binary: a
session over one fixed tree with one live leaf, running the verbs in the order
the chapters run them. Chapter 1 traces it whole at low resolution; chapters 2
to 6 each take the verb that chapter owns at full resolution, on the same tree
and the same names. The second ending is chapter 4's refusal before the lock,
because it is the spine's second order made visible and
`session_kind_presence.rs` pins it — where the self-deadlock, the file's most
argued claim, is not an ending at all: `tree_lock.rs` proves it cannot happen,
and chapter 4 says so instead of tracing it. The two verbs a session never runs,
`root-init` and `finish-commit`, are the driver's, and each gets one short trace
in the chapter that owns it.

Rejected: **`leaf-decompose` alone** carried from orientation to chapter 4 with
its two endings, which breaks the carry after chapter 4 and leaves three
chapters with unrelated examples; **two refusals** as the carry — the
wrong-working-tree admission and the undeclared kind — which makes the happy
path the aside, as both precedents found; and **this grove's own tree**, whose
named leaf is retired and renamed within days of the book landing.

**The example comes before the catalogue.** No section earlier than a chapter's
worked example may primarily enumerate three or more public verbs, operations,
flags, refusals or obligations; those catalogues follow the complete example.
This binds chapter 2 if it names the twelve verbs, chapter 3's four contracts
and chapter 4's four, and chapter 7's table, which may open that page because
the chapter has no example section. A reviewer checks that the stated start,
the transition steps, and the observable end are all present.

## Early uses the order forces

Two costs are paid here. Orientation owns the import block, so every
`grove-loop` type the binary names is named before the chapter that explains
what the binary does with it; and chapter 2 owns `run`, whose exhaustive match
names every handler four chapters own. These rows are the minimum ledger.
Authors add a row before introducing any additional later-owned name.

| Symbol family | First use | Owner | Minimum local statement |
|---|---|---|---|
| `SessionEpochGuard` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | The guard `run` obtains when it admits this process against a live session epoch — present under a driver, absent for a manual command — alive through the verb, and consulted only by `complete`. |
| `Reading`, `Tree`, `Writing`, `TreeWrite` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | The two openings of a grove — a shared read and an exclusive write — each answering *vacant* as a value rather than an error. |
| `Workspace` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | A resolved jj working tree; every verb but `complete` resolves it from the current directory, and the grove root is spelled from it in one place. |
| `Sought`, `Resolution` | `01-orientation.md#the-imports` | `information-not-error` | `Sought` is a found-or-nothing answer; `Resolution` is what `resolve` found — the root, one entry, or an ambiguity. |
| `Reference` | `01-orientation.md#the-imports` | `information-not-error` | A parsed spelling of a tree entry — key, handle or slug — read by its own type before any tree is opened. |
| `Outcome` | `01-orientation.md#the-imports` | `information-not-error` | Live, retired or abandoned — the infix a filename carries, rendered as a stderr note so a dead end never looks live. |
| `Kind`, `Slug` | `01-orientation.md#the-imports` | `before-the-lock` | The grammar's own types for a `--kind` token and a slug; malformed text is refused by them, before any lock. |
| `SessionConfig` | `01-orientation.md#the-imports` | `before-the-lock` | The launch configuration, loaded whole and asked whether one kind resolves to a template. |
| `Handle` | `01-orientation.md#the-imports` | `admit-before-signal` | A `<slug>-k<key>` handle, parsed leniently on the key and spoken canonically thereafter. |
| `Signalled` | `01-orientation.md#the-imports` | `admit-before-signal` | Whether `complete` wrote the disposition to a channel or found no loop to signal. |
| `cmd_pick`, `cmd_brief_chain`, `cmd_kind`, `cmd_resolve` | `02-the-grammar.md#worked-dispatch` | `information-not-error` | One handler per reading verb: the shared opening, one `grove_loop::verbs` call, and rendering. |
| `cmd_root_init`, `cmd_leaf_add`, `cmd_leaf_insert`, `cmd_leaf_decompose` | `02-the-grammar.md#worked-dispatch` | `before-the-lock` | One handler per growing verb: text parsed, presence asked, then the exclusive opening, then one call. |
| `cmd_leaf_retire`, `cmd_leaf_prune` | `02-the-grammar.md#worked-dispatch` | `two-steps-remain` | One handler per terminal mark: the exclusive opening, one call, the marked paths, and the two remaining steps on stderr. |
| `cmd_finish_commit`, `cmd_complete` | `02-the-grammar.md#worked-dispatch` | `admit-before-signal` | The two handlers that open no tree: one commits through the workspace, one writes the completion channel. |

`verbs` — the module — is named in chapter 1 and explained there as *one call
per verb*; it is `grove-loop`'s and no chapter of this book owns it, so it
needs no row. `worktree`, `readable`, `writable`, `absent`, `slug`,
`parse_kind`, `require_declared` and `normalize_leaf_path` are each owned by a
chapter earlier than or equal to every chapter that names them, which is
forward reference in the ordinary direction and needs no row.

A row is `pending` while its owner is outside the current scoped prefix and
`explained` from the owner's scoped increment onward. The owning slice changes
only its own rows. **The local statement is judged, not matched.**

## Outbound links

The book is a Grove runtime crate's book, so
[`walkthrough-books.md`](walkthrough-books.md)'s *Outbound links* binds it
unchanged: it declares a `[guide]` path with a non-empty anchor array, and its
`README.md` reader contract cites one of those anchors. The `[guide] omitted`
shape is not available to it.

**The guide.** [`USAGE.md`](../USAGE.md), declared with four anchors, all of
which already exist in the explicit `<a id="…"></a>` form.

| Anchor | Where it is cited | Why this book reserves it |
|---|---|---|
| `usage-tree-verbs` | `README.md` reader contract | The required citation, and the section whose subject is exactly this crate's surface. |
| `usage-driver-lease` | ch. 2, beside `admit_ambient_session` | The lease is what installs the epoch a verb is admitted against. |
| `usage-review-composition` | ch. 4, beside `leaf-add`'s chain and pair help | The methodology the help text carries, linked rather than explained. |
| `usage-finish` | ch. 6, beside `finish-commit` | The finish cycle this verb is one step of. |

**The glossary.** [`CONTEXT.md`](../../CONTEXT.md), declared with seven anchors.

| Anchor | Status | Where it is cited |
|---|---|---|
| `driver-lease` | exists | ch. 2, beside admission |
| `stated-vcs` | exists | ch. 2, beside `Workspace::resolve` in `worktree` |
| `task-tree-scheme` | exists | ch. 3, beside the reference grammar |
| `task-commit-boundary` | exists | ch. 5, beside the two-step reminder |
| `loop-control-channel` | exists | ch. 6, beside the signal file |
| `session-epoch` | **must be created** | ch. 2, the admission's own term |
| `tree-access-lock` | **must be created** | ch. 4, the lock every lock-scope argument is about |

**Two obligations fall outside the book and are `grove-llm-book-k33`'s.**
`CONTEXT.md` carries *Session epoch* and *Tree access lock* as bold paragraphs
today; each must be promoted to a `###` heading with an explicit anchor line
before the book validates, because a declared anchor the target does not carry
is `M201` against the manifest whether or not any page cites it. The promotion
is the same move `overview-book-k30` made for `guaranteed-core`: keep the
phrase as the heading text, add the anchor line above it, change nothing else.
No test pins either bold form — `composition_guidance.rs`'s pin on the phrase
*Tree access lock* is against `docs/ARCHITECTURE.md`, not the glossary.

The minimal set — one guide anchor and the five existing glossary anchors, with
no work outside the book — was rejected because the two terms the spine's orders
are named in would become bare links into a 1,049-line document. The wide set —
adding `usage-session-lifecycle`, `usage-task-tree` and `undoing-a-mistake`, and
`work-item-handle`, `session-kind` and `pruning` — was rejected as navigation
seven chapters do not need at the price of three more promotions.

**Nothing else is citable, and no book cites this one.** The permitted local
targets are this book's own pages, its four source roots, the guide path and the
glossary path. The overview names the twelve verbs and this crate's test files
and reproduces no byte of this corpus; this book names `grove_loop::verbs` and
reproduces no byte of that one. A reader moves between books through
[`ARCHITECTURE.md`](../ARCHITECTURE.md)'s *Documentation ownership* table.

## The book's row in the ownership table

The row is owed by `every_book_root_has_a_documentation_ownership_row`, goes red
the moment the book root exists, and is `grove-llm-book-k33`'s to add:

| Subject | Canonical source |
|---|---|
| `grove-llm` source, read page by page | `walkthroughs/grove-llm/README.md` — the code walkthrough: a book whose every chapter opens on the one thing a thin binary still has to get right, and whose fragments reconstruct every byte of the crate's frozen corpus |

It collides with no existing row: the guide's row is *human workflow and
commands*, and the overview's is the `grove` binary's source and the system
description its call reaches.

## What the book deliberately does not cover

### Rust and clap fundamentals

The audience knows Rust. `clap`'s derive macros, `Subcommand`, and
`arg_required_else_help` are named where the source names them and explained
only where an argument rests on their behaviour — chapter 2's account of why
`command` is an `Option` is that case. `flock(2)` semantics are stated once, in
chapter 4, as that chapter's premise: two file descriptions do not share an
advisory lock, which is the whole of what the self-deadlock argument needs.

### Everything behind `grove_loop::verbs`

The largest omission, and the one the book states out loud. What a verb does to
the tree — the walk `pick` performs, the renumber `leaf-insert` performs, the
atomic run `leaf-add` lands or rolls back, the epoch record `admit_ambient_session`
reads — is `grove-loop`'s, whose book is not written, and the outbound-link
contract forbids this book citing where that description lives today. This book
states, for each call, what the binary passes in, what comes back, and what the
binary does with it, and stops. The prose says where the book stops; it does not
summarise the loop, and it is not silent about it.

### The tests

`crates/grove-llm/tests/` — twenty-two test files and a support module, 9,379
lines — is outside the corpus: no chapter owns it and no fragment reconstructs
it. Tests remain **evidence** the book cites, and this book cites them more
heavily than either precedent, because *hold the help to the handler* names a
test for every promise. The only traces of them inside the corpus are the four
dev-dependencies whose manifest comments say which test needs each, which
chapter 1 owns and explains. Four of the files — `instructed_verbs.rs`,
`composition_guidance.rs`, `session_kind_guidance.rs` and `removed_surface.rs` —
test the shipped methodology and the repository's documents against this binary
rather than the binary itself; chapter 7 names that fact once and the book does
not explain the skills.

### The guide's account, and the methodology inside the help text

[`USAGE.md`](../USAGE.md)'s row is *what each verb does to the tree, what it
prints, and whether it commits*, for a human. The book does not restate it. The
help text this corpus carries goes further than mechanism in two places — the
fifty lines under `leaf-add` and `leaf-insert` on how review chains and vendor
pairs are cut and where an integration is placed, and `leaf-prune`'s HITL rule —
and the book reproduces those bytes, links the guide at
`usage-review-composition`, and explains only what the code keeps of them.

### Jujutsu, beyond two calls

The reader knows jj. The binary makes two calls into it — `Workspace::resolve`
in `worktree`, and the path-scoped commit behind `verbs::finish_commit` — and
each is stated at the point of dependence, in chapters 2 and 6, with the
glossary's `stated-vcs` linked beside the first. The book does not otherwise
explore workspaces, snapshotting or the operation log; `finish-commit`'s help
names jj's `restore` and `undo` as remedies, and the page says why the crate
offers none of its own without teaching either command.

### The human binary

`grove` is the overview's corpus. The audience-split decision the header cites is
stated in chapter 1 as a decision, and the other binary is named, not described.

### Cross-layer source in orientation

Orientation owns no source belonging to a later chapter. Copying a handler into
it would make its low-resolution trace visibly source-backed, but it would
violate the settled chapter boundary and add ownership joins. Exact identifiers,
concrete values, the early-use ledger and source-backed later chapters give the
same technical trace with a smaller fragment graph.

## Known in advance: what chapter 1 once adjudicated

Two claims in the frozen corpus did not survive a check against the source
beside them. Each page stated the checkable fact beside the fragment that
carried the stale one, as the overview's pages do for theirs; neither licensed
a code change from inside the book, under the root brief's freeze rule. Both
have since been corrected by `grove-llm-dependency-comments-k102`, which
carried the source change, the two fragments and chapter 1's paragraphs in one
commit. The corpus no longer carries either, and a re-draft should not
re-adjudicate them.

A third claim never reached a page as a live one. `cli.rs` lines 39–43 once
said `crates/grove-llm` carries a `0.1.0` of its own, against a manifest whose
line 3 is `version.workspace = true`; `grove-llm-version-comment-k83` rewrote
the comment to argue only for reading one constant, and rewrote chapter 2's
paragraph in the same commit. The corpus no longer carries that claim, and a
re-draft should not re-adjudicate it.

1. **The manifest's reachability claim, and the direct `jj-workspace`
   dependency.** `Cargo.toml` line 15 said everything this binary can reach is
   something `grove-loop` chose to publish; line 31 declares `jj-workspace` as a
   second dependency, and `cli.rs` line 32 imports `Workspace` from it directly.
   `grove-loop` re-exports that same type (`crates/grove-loop/src/lib.rs`, line
   81), so the binary reaches nothing `grove-loop` did not also publish — but
   the dependency line makes the whole of `jj-workspace`'s public surface
   reachable, and the claim as written was a claim about what *can* be reached.
   k102 narrowed the sentence to `grove-loop` — of that crate, only the
   published surface is reachable, which is the half the compiler holds
   outright — rather than dropping the dependency and importing
   `grove_loop::Workspace`, so `cli.rs` is untouched and the direct edge stays.
2. **The module documentation's second publisher.** `lib.rs` lines 7–8 said
   *something `grove-loop` or `grove` chose to publish*. The manifest's own
   comment, lines 17–22, records that the `grove` dependency went at
   `loop-crate-driver-k22`. The manifest was right and the module doc was
   stale; k102 dropped the second publisher and scoped the clause the same way.

k102 rewrote a further clause in the same seven lines, one not known before
drafting: `Cargo.toml`'s *a binary target can reach its own library's private
items*, which holds only for a target that compiles the library's modules into
itself. `bin-target-privacy-claim-k87` had corrected that wording in decision 1,
in `crates/grove/Cargo.toml` and in `docs/ARCHITECTURE.md`, and left this site
inside the fragment k102 had to rewrite anyway.

The draft's technical-truth charter found three more, carried by
`root-init-drop-order-comment-k99`, `next-steps-comment-lane-k100` and
`complete-help-grove-do-k101`; *What order holds* keeps that tally.

## What this brief does not settle

Recorded as open rather than filled in, so a later reader can tell a decision
from an invention.

- **The concept-index entries.** The two lookup surfaces are required by
  [`walkthrough-books.md`](walkthrough-books.md) and their contents are the
  authoring slices' to curate, not this brief's to enumerate.
- **The fragment partition inside each block.** This brief fixes top-level
  ownership; how a block divides into intent-named literal and composite
  fragments is the owning slice's work.
- **The exact values of the carried session.** The shape is settled — one
  tree, one live leaf, a session running its verbs in chapter order — but the
  concrete names, paths, handle and change id are the orientation slice's to
  fix, and every later chapter then reuses them unchanged. The slice may reuse
  the overview's tree, which is already fixed in that book.
- **Section anchors other than the ones named here.** `the-imports` in chapter
  1 and the six worked-example anchors are required because ledger rows and the
  contract name them; every other heading and anchor is the draft's.
