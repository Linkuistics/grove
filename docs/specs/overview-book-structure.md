# System overview — book structure brief

## Status and provenance

This is the structure brief for the book at `docs/walkthroughs/overview/`, which
does not exist yet and which this document is written ahead of, in the form
[`walkthrough-books.md`](walkthrough-books.md) requires under *The structure
brief*. It settles what that specification deliberately does not: the chapter
sequence, the mapping of that sequence onto the corpus, each chapter's worked
example, the early uses the order forces, and what the book does not cover.

It settles one thing no other brief in this campaign has to. The overview is the
only book that **absorbs prose from another document**: decision 5 of `plan-k1`
moves [`ARCHITECTURE.md`](../ARCHITECTURE.md)'s descriptive account into it. So
this brief also carries the rule `architecture-move-k31` applies clause by clause,
the fate of that document's anchors, and what is left behind for whom — and it is
written to be specific enough that k31 needs no second interview.

**This document is authored, not recovered, and it precedes its book.** Every
decision below was settled in the `overview-structure-k29` interview and is
recorded, with its rejected alternatives, in that leaf's decision log. It follows
[`jj-workspace-book-structure.md`](jj-workspace-book-structure.md), which is the
first brief elicited before its book and the shape this one is uniform with.

**This brief is the human contract; the manifest is its machine-readable form.**
The chapter sequence and ownership mapping below are what
`docs/walkthroughs/overview/walkthrough.toml` records as its `[[page]]` and
`[[block]]` groups. Where the two disagree, that is a defect in one of them, not
a licence to prefer either.

## Audience and intended outcome

The audience is settled by decision 7 of `plan-k1` and is not re-opened here: a
reader who knows Rust and Jujutsu and has driven a grove, for whom grove's
vocabulary is linked to the glossary and never re-taught, and whose entry point
to the system is [`USAGE.md`](../USAGE.md).

**The intended outcome is the thin-entry-point test.** At the end the reader can
take an entry point in their own code, ask what is left for an argument to
select, and tell a binary that is genuinely thin from one that merely looks thin
— naming which of three mechanisms holds it: a package boundary the compiler
enforces, a closure property a test asserts, or a convention nobody checks.

That outcome was chosen because all three mechanisms are provable inside this
book's own 204 lines, and no other book in the set could make the same claim:

- `Cargo.toml` argues the crate-not-a-`[[bin]]` boundary, and the compiler holds
  it — everything `main.rs` can reach is something `grove-loop` chose to publish;
- `the_human_command_surface_has_nothing_left_to_select` asserts a **closure
  property** rather than a list of rejected verbs, so it fails on the next flag
  too; and
- `the_human_facing_binary_describes_every_option_it_lists` catches the third
  case — a convention that had already been broken once, at
  `retire-no-launch-help-k21`, and rendered as a padded blank row.

**The locate-and-route outcome follows for free and is therefore not the stated
one.** Source-exactness plus chapter 5's module map already let a reader name the
package and module owning any behaviour. Naming it as *the* outcome would make
this a reference card, which
[`ordinal-fs-tree-book-structure.md`](ordinal-fs-tree-book-structure.md) and its
successor both reject.

**A third candidate was ruled out by fact rather than by preference.** *Read one
`grove` invocation end to end* is not an available outcome: the description of
what happens inside `grove_loop::run` stays in `ARCHITECTURE.md` until the
`grove-loop` book lands (see *What this book absorbs*), and the outbound-link
contract forbids this book citing it. The promise would exceed what the book can
deliver.

## The spine: the call, and what hangs off it

**Each chapter opens at one of the binary's own steps, and the absorbed
architecture description is the body of the chapter whose step reaches it.**

The spine is recovered from the source rather than imposed on it. Ninety-seven of
the corpus's 204 lines are comment prose, and every thesis in them is about
something the binary declines to do: `Cargo.toml` argues why `grove` is a crate
rather than a `[[bin]]` target inside `grove-loop`, why there is no `[lib]`, and
why the repository-surface tests live in this package; `cli.rs` argues that
launch policy has one home and it is not the command line; `main.rs` states that
its three steps are exactly the things the loop cannot do for itself.

Two alternatives were rejected. A **descriptive system account with the source as
one chapter**, and a **two-part book** with source-exact chapters followed by
source-less ones, both put the absorbed prose in chapters owning no `[[block]]`.
A slice owning no block is not a `--through` value — the specification derives the
scoped domain from block ownership — so under either shape the whole descriptive
half would have had no scoped proof checkpoint and would be proved, if at all,
only by the final run.

## What this book absorbs from `ARCHITECTURE.md`

This section is `architecture-move-k31`'s charter. It is stated here rather than
there because the boundary is a structure decision about this book, and k31 is an
`impl` leaf that should be executing a settled rule rather than eliciting one.

### The rule

**A clause moves iff it states what the system does, is shaped like, or reaches —
something checkable against the code as it stands. Everything else stays,
including anything the session cannot classify confidently.** A sentence that
describes and justifies at once is split where the halves stand alone, and stays
whole where they do not.

The ground is **asymmetric reversal cost**, not taxonomy. A clause moved into the
overview acquires a page ID and an ownership block in a fragment-validated book;
pulling it back later means editing a proved book, its ledger and its manifest. A
clause left in `ARCHITECTURE.md` moves later for the price of an edit. Under
genuine uncertainty the clause belongs where reversal is cheap, and the default
therefore points at staying.

The mirror-image rule — *stay iff it names a rejected alternative, a ground, a
constraint or a measurement* — reaches the decisions-only end state faster and was
rejected for putting every uncertain clause in the expensive document. Paragraph
granularity was rejected because this document's paragraphs are long and often
mixed, so every mixed paragraph would be wrong in one direction.

### What is in play, and what is not

**Three sections are in play**, and no others:

| Section | Lines | Why |
|---|---:|---|
| *Runtime flow* | 41 | The path one invocation takes; system-level by construction |
| *Command surfaces* | 42 | Both binaries' grammars, one of which is this book's corpus |
| *Main module seams* | 65 | The workspace map — six packages, not one crate's internals |

**Six sections are not in play, and stay whole for now** — *Task-tree data model*
(374), *Task kinds and composition* (447), *How the methodology reaches a session*
(315), *Lifecycle and resumption* (68), *Human authority and completion* (67) and
*Version-control seam* (62). Each describes `grove-loop`'s or `jj-workspace`'s
internals, whose canonical source is that crate's own book. Moving them here would
make the overview canonical for a subject another book is about, which the
*Documentation ownership* rule does not allow, and the outbound-link contract
forbids the later book citing the overview to repair it.

The remaining sections — *Documentation ownership*, *Repository products*,
*Session configuration*, *Process ownership* and *Verification* — were never
named as descriptive and are untouched, beyond the ownership table's own row (see
*The book's row in the ownership table*).

**k31's task file overstates its case, and this brief corrects it.** It calls the
three sections in play "descriptive throughout". Under the rule above each keeps
a real residue, and k31 should expect to leave text behind in all three:

- *Runtime flow* keeps **"Two advisory steps used to open every iteration and are
  gone"** and its account of what went with `delete-provisioning-k19`. The
  diagram, the `--help` / `--version` sentence and *the driver stays in the
  foreground* move.
- *Command surfaces* keeps **"Twelve, not thirteen"** and `open-kind-k20`'s
  reason; **"Two entries are gone rather than renamed"** and both retirements; the
  ground that the split *keeps a discoverable human API without forcing the agent
  to reproduce filesystem mutations from prose*; and the clause that *the binary
  is thin* is compiler-enforced rather than reviewed. The description of each
  surface, the verb list itself, and the thin-entry-point sentences move.
- *Main module seams* keeps the module-visibility constraint, the ground for
  file-sized modules with no service layer, the record that there is no `harness`,
  `methodology`, `provision` or `leaf` module, and the `loop-crate-verbs-k21`
  observation that a crate boundary is a reachability boundary. **The table
  moves**, and it is chapter 5's whole subject.

### Anchors, and the forward pointer

**Every explicit `<a id="…"></a>` anchor stays where it is, above the argument
that survives. No citation is re-pointed, in Rust or in Markdown.**

This was settled by measurement. Of the 39 `ARCHITECTURE.md#…` citations in Rust
sources, `library-refusals` (12) and `tree-access-lock` (8) sit in sections that
are not in play at all; `task-kind-taxonomy` (2), `symmetric-vcs-rule` (2),
`cli-binary-split`, `no-migration` and `pruning` each sit above surviving
argument — `cli-binary-split`'s single citation, in `crates/grove-llm/src/cli.rs`,
cites the audience-split *decision*. The two that appear to cite description,
`#runtime-flow` and one `#task-kind-taxonomy`, are positive controls inside
`crates/grove/tests/reference_navigation.rs` asserting the resolver works.

**Not one real design citation points at description.** Moving anchors with the
description would therefore re-point citations that are not pointing at it, and
would move them into a namespace no check resolves:
`every_architecture_anchor_citation_in_a_source_resolves` reads
`docs/ARCHITECTURE.md#` only, and `every_repository_markdown_reference_resolves`
is Markdown-only. Leaving anchors in place keeps both green through the move
without a single source edit.

**Each stripped section opens with exactly one forward pointer** — a link to the
overview page now carrying its description. A link from `ARCHITECTURE.md` into a
book is inbound, so the book's outbound-link contract does not bind it, and
`every_repository_markdown_reference_resolves` checks it. Without the pointer, a
reader arriving from a source comment lands on an argument with its subject
removed and no local signal that the subject moved.

### What is left behind, and who removes it

Because the six crate-internal sections stay, `architecture-move-k31` **cannot**
reach the end state both the root brief and `crate-books-k14`'s brief state —
that `ARCHITECTURE.md` carries decisions, constraints and measurement records
only. It reaches that state for the three sections in play and no further.

The mechanism for the rest is **deletion, not a second move**. The `grove-llm`,
`keyed-launch` and `grove-loop` books re-derive their crates' accounts from the
source they reconstruct; the surviving description in `ARCHITECTURE.md` becomes
redundant rather than relocatable. `architecture-residue-k75` owns removing it,
runs last in `crate-books-k14`, and is the leaf on which the campaign's
decisions-only *Done when* actually falls due. `k31` marks each surviving
descriptive passage with the book that will make it redundant, so that leaf
inherits a list rather than a judgement.

## Chapter sequence

Five chapters, four of them source-owning, cut so that **each absorbed section
has exactly one home**.

| # | Page | Slice ID | What the chapter is for |
|---|---|---|---|
| 1 | `01-orientation.md` | `compiler-held` | What the binary is, and what being a separate crate buys that a `[[bin]]` target could not. |
| 2 | `02-the-surface.md` | `no-arguments` | The human grammar, which selects nothing, beside the agent grammar, which is twelve verbs. Absorbs *Command surfaces*. |
| 3 | `03-three-steps.md` | `one-call` | Resolve, lease, run — and the signal path, which is the only interesting thing a three-step `main` can still get wrong. Absorbs *Runtime flow*. |
| 4 | `04-proving-a-negative.md` | `closure-proved` | How the surface is held closed: a property asserted against clap's own model, not a list of rejected argument vectors. |
| 5 | `05-what-the-call-reaches.md` | `assembly` | Where the work happens, the transferable test, and the boundary of this book. Absorbs *Main module seams*. Owns no source. |

**Slice IDs are named for the mechanism each chapter establishes**, so the slice
list reads as the book's spine — `compiler-held`, `no-arguments`, `one-call`,
`closure-proved`, `assembly` — and no slice token equals any page ID. That
difference is by construction, and it is the property the specification's separate
slice domain exists for: a presentation-level page rename must not become a
fragment-ownership, ledger, manifest and CLI migration. Slice IDs carry no Grove
task key, for the reason recorded in `jj-workspace-structure-k17`'s decision 8.

**Chapter 4 is a chapter rather than a subsection** because the test module is
84 lines — 41% of the whole corpus — and carries the book's thesis. Two rejected
shapes: four chapters folding the surface into the call, where chapter 2 would
carry roughly 55 lines of moved prose from two different sections and lose the
one-section-one-home property; and six chapters splitting the tests to sit beside
what each constrains, where chapter 2 would own two non-adjacent blocks and send
a reader backwards through the file once.

## Concept and seam responsibilities

### `README.md` — reader contract

Names the audience, states the thin-entry-point outcome, cites
[`USAGE.md`](../USAGE.md) at `usage-running-grove` — the one guide link the
contract permits and requires — and states the book's boundary: it explains one
module, `grove::cli`, and names the rest.

### 1 · Orientation — the binary that selects nothing

Owns `crates/grove/Cargo.toml` whole. Responsible for: what `grove` is and what
it is for; the two products the repository ships and the fact that it installs
neither into the other; and the crate-not-a-`[[bin]]` argument, which is the
first of the outcome's three mechanisms and the only one the compiler holds.

The chapter's thesis is the manifest's own: *a binary target inside `grove-loop`
could reach that library's private items, so* the binary is thin *would stop
being compiler-enforced*. It also carries the absence of `[lib]` — which is why
`cli.rs`'s clap model is asserted by the binary's own unit tests rather than by an
integration test — and the repository-surface tests' residence here, which is
where the reader first meets *this is the package that is grove-the-product*.

### 2 · The surface — two binaries, one of them empty

Owns `crates/grove/src/cli.rs` lines 1–19: the imports, the `Cli` struct, and the
clap attributes. Absorbs *Command surfaces*. Responsible for: the audience split
between a human binary and an agent binary; the human grammar having nothing left
to select and why — the driver reads the task tree for what to do and
`~/.config/grove/config.kdl` for how to launch it; the twelve `grove-llm` verbs as
a flat surface; and one version constant read by both binaries so `grove
--version` and `grove-llm --version` cannot skew.

The catalogue rule binds here: the twelve verbs are a catalogue and follow this
chapter's worked example rather than opening the page.

### 3 · Three steps — resolve, lease, run

Owns `crates/grove/src/cli.rs` lines 20–53 and the whole of
`crates/grove/src/main.rs`. Absorbs *Runtime flow*. Responsible for: the three
things the loop cannot do for itself; the workspace being resolved **once** here
and handed to both the lease and the loop, and what that seam replaced; the shape
of one foreground iteration; and the signal path.

**The signal path is the chapter's centre of gravity.** A driver that was killed
does not exit 0: the loop reports *why* it stopped, one reason is that this
process was sent `SIGTERM` or `SIGHUP` mid-grove, and the only way to say so
through a wait status is to die of the same signal after cleanup. That is the
most argued and least obvious claim in the corpus, and a reader would not guess it.

### 4 · Proving a negative — the closure tests

Owns `crates/grove/src/cli.rs` lines 54–137. Responsible for: how a *negative*
surface is held; why the assertion is a closure property rather than a list of
rejected verbs, and that it fails on the next flag too; why both tests read
clap's own model rather than rendered help text; why the `undescribed` walk exists
twice across two packages and what the alternatives to that duplication would have
cost; and the empty-description case, where `#[arg(help = "")]` renders exactly
like the missing doc comment the check exists to reject.

This chapter completes the outcome's three mechanisms: the compiler's boundary
(chapter 1), the asserted property, and the described-option convention.

### 5 · What the call reaches — the module map

Owns no source; final-only. Absorbs *Main module seams*. Responsible for: the map
of six packages and twelve `grove-loop` modules; the statement of this book's
boundary; and the transferable test applied back across the three mechanisms.

**The map names; it does not explain.** Its three columns are the ones
*Main module seams* already uses, and the chapter states once — not per row — that
each row's *how* belongs to that crate's own book, three of which are not
written. Exactly one row is a module this book explained: `grove::cli`.

Naming a book per row was rejected: three of them do not exist, book paths in
inline code are opaque to the link scanner, and the result would be forward
references that nothing reports when they stay wrong. Dropping the responsibility
column was also rejected — absorbing *Main module seams* down to its column
headings is close to not absorbing it, and it would cost the locate-and-route
outcome this chapter delivers for free.

## The mapping onto the corpus

Three source roots, in this fixed order — the order the source index presents and
the manifest's `[[root]]` array follows.

| Root ID | Source path | Lines |
|---|---|---:|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 54 |
| `source-entry-point` | `crates/grove/src/main.rs` | 13 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 137 |
| **total** | | **204** |

**The corpus rule needs no exceptions.** `include` is the two base patterns fixed
by `[book].subject` — the exact path `crates/grove/Cargo.toml` and the pattern
`crates/grove/src/**/*.rs` — and that pair yields exactly these three roots. The
manifest carries no `[[corpus.add]]` and no `[[corpus.exclude]]`, which is what
`walkthrough-books.md`'s exception inventory already records by giving this book
no row.

**`cli.rs`'s test module is inside a root and is reconstructed.** Lines 54–137 are
a `#[cfg(test)] mod tests`, and the `inline-test-module` exclusion class applies
to a whole file named `tests.rs`, which this is not. The 84 lines are owned,
explained and proved like any other block — which is what makes chapter 4
possible rather than merely permitted.

**`[book].subject` is `crates/grove`**, fixed by the subject inventory in
[`walkthrough-books.md`](walkthrough-books.md) and not this book's to choose.
`[book].title` is `System overview`, and `[book].id` — hence the directory name —
is `overview`.

### Top-level ownership blocks

Five blocks. The State column is the required state after the `compiler-held`
slice; a later owner changes only its own row from `deferred` to `resolved` when
it replaces the matching defer with an insert and adds the definition.

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---:|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-54` | 54 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-13` | 13 | `deferred` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-19` | 19 | `deferred` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `20-53` | 34 | `deferred` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `54-137` | 84 | `deferred` |

The blocks of `source-command-surface`, in array order, are `1-19`, `20-53` and
`54-137`: ordered, adjacent, non-overlapping, and covering 1 to 137 exactly. The
other two roots are single full-file blocks.

### Where a file's concerns split across chapters

**Exactly one root has more than one owner.** `src/cli.rs` carries three of the
book's five concerns, and the boundaries fall on structural lines rather than on
convenience:

- **the surface** owns the imports, the `Cli` struct and its clap attributes
  (`1-19`), ending at `pub struct Cli {}`;
- **one call** owns `run`'s doc comment and body (`20-53`), which is where
  resolve, lease and the signal path are all argued; and
- **closure proved** owns the whole `#[cfg(test)] mod tests` (`54-137`), which
  begins at the `#[cfg(test)]` attribute line.

**One deliberate consequence, accepted rather than engineered around.** The `use
grove_loop::{DriverLease, LoopOutcome, TemplateSource, Workspace};` line is line 2
and therefore belongs to chapter 2, four of whose named types chapter 3 owns.
Giving chapter 3 a second block of `1-2` would have removed four early-use rows at
the cost of separating `use clap::Parser;` from the `#[derive(Parser)]` that needs
it, and of a chapter owning two non-adjacent blocks to save an early-use table
that the specification expects every book to have. The rows are in *Early uses the
order forces* below.

`main.rs` is a full-file block owned by chapter 3 rather than by chapter 1,
because its seven-line module documentation is the three-steps argument in
miniature and belongs beside the body that performs them.

### Owned-source totals

| Slice | Page | Owned lines |
|---|---|---:|
| `compiler-held` | `01-orientation.md` | 54 |
| `no-arguments` | `02-the-surface.md` | 19 |
| `one-call` | `03-three-steps.md` | 47 |
| `closure-proved` | `04-proving-a-negative.md` | 84 |
| `assembly` | `05-what-the-call-reaches.md` | 0 |
| **Total** | 3 source roots | **204** |

The scoped-slice domain derived from these blocks is `compiler-held`,
`no-arguments`, `one-call`, `closure-proved` — four accepted `--through` values,
with `assembly` final-only.

## What each chapter's prose owes

Measured before deciding: 48% of this corpus is comment prose — `Cargo.toml` 54%,
`main.rs` 54%, `cli.rs` 45% — and the executable production Rust outside the test
module is about 27 lines. As with `jj-workspace`, the standing risk is therefore
**paraphrase, not obscurity**: the fragment graph quotes the argued comment
verbatim on the page anyway. Three things a comment structurally cannot do are
what each chapter's prose owes, and the third is this book's alone.

- **Adjudicate.** For every argued claim, name the behaviour it rests on, the test
  that proves it, and the alternative rejected with what it would have cost. A doc
  comment cannot cite a test in another package, and two of this corpus's three
  mechanisms are proved by tests in the same file the comment sits in — which
  makes the third, the compiler's, the one a reader is most likely to take on
  trust.
- **Carry the through-line.** Trace one invocation across all three roots and show
  where a decision in one file is only explicable by a decision in another: the
  manifest's refusal of a `[lib]` is *why* the closure tests are a `mod tests`
  inside the binary rather than an integration test, and the manifest's choice of
  a separate crate is *why* chapter 5's map can say that everything `main.rs`
  reaches is something `grove-loop` chose to publish.
- **Say what is behind the call without explaining it.** This corpus ends at
  `grove_loop::run`. The prose owes the reader an honest statement of where the
  book stops — not a summary of the loop, and not silence.

## Worked examples

Four chapters carry the named example section. The fifth does not: it owns no
production source and its job is synthesis.

| Chapter | Required anchor | Start and observable end |
|---|---|---|
| `01-orientation.md` | `one-invocation` | The carried invocation at low resolution: `grove` typed at a grove working tree's root, through the lease to one launched session, the loop continuing, and the process exiting 0. |
| `02-the-surface.md` | `worked-argv` | The same invocation's argv: `grove` alone; `grove --help` and `grove --version` stopping before the flow, discovering no repository and acquiring no lease; and `grove --harness claude` refused, with the text clap renders. The twelve-verb catalogue follows. |
| `03-three-steps.md` | `worked-run` | The same invocation at full resolution: `current_dir`, `Workspace::resolve`, `DriverLease::acquire`, `TemplateSource::from_env`, `grove_loop::run` — and both endings. `Finished` and `Stopped` reach `Ok(())`; `Interrupted(SIGTERM)` reaches `reraise`, and whoever started `grove` reads a wait status of `128 + 15`. The two error endings `run`'s own doc comment names are the same trace stopping earlier. |
| `04-proving-a-negative.md` | `worked-assertion` | The two tests run against a `Cli` that has grown one flag: what `undescribed` collects, what `get_arguments` reports once `help` and `version` are filtered out, and the exact text each assertion prints. |

**One invocation, carried through the book, with two endings.** Chapters 1 and 3
are the same invocation at low and then full resolution. Chapter 2 is that
invocation's argv, plus the two arguments that stop before the flow and one that
is refused. Chapter 4 shows what guarantees the argv can never grow, so the tests
are read as the guard on a trace the reader already holds rather than as a second
scenario to learn.

Rejected: **two refusals** as the carried trace — a working tree that is not a jj
workspace, and a lease another driver holds — which would make the happy path the
aside and would break the carry at chapter 4, since neither closure test is about
an error path. Also rejected: **the self-referential run** against this grove's
own tree, whose named task file is retired and renamed within days of the book
landing, so the book would carry a value that goes stale by design.

**The example comes before the catalogue.** No section earlier than a chapter's
worked example may primarily enumerate three or more public verbs, operations,
options or obligations. This binds chapter 2, whose twelve `grove-llm` verbs
follow its example, and chapter 5, whose twelve-module table may open the page
because that chapter has no example section.

## Early uses the order forces

The order forces five rows. All five are `grove-loop`'s public items named by
chapter 2's import line or by chapter 1's manifest comment, before chapter 3
explains what the binary does with them.

| Symbol family | First use | Owner | Minimum local statement |
|---|---|---|---|
| `grove_loop::run` | ch. 1, `Cargo.toml` comment | ch. 3 | The loop's single entry point; everything the binary does after its three steps is behind this call. |
| `Workspace` | ch. 2, `cli.rs` line 2 | ch. 3 | A resolved jj working tree, produced once here and handed to both the lease and the loop. |
| `DriverLease` | ch. 2, `cli.rs` line 2 | ch. 3 | The one-driver-per-working-tree claim, taken for the life of the process. Linked to the glossary at `driver-lease`. |
| `TemplateSource` | ch. 2, `cli.rs` line 2 | ch. 3 | Where launch policy is read from; the loop re-reads it once per iteration. |
| `LoopOutcome` | ch. 2, `cli.rs` line 2 | ch. 3 | Why the loop stopped — the value that decides whether this process exits 0 or dies of a signal. |

`grove_loop::VERSION` is named and owned in chapter 2 and is not an early use.
`Cli` is named in chapter 2 and used in chapter 4, which is forward reference in
the ordinary direction and needs no row.

## Outbound links

**The guide.** `docs/USAGE.md`, declared with four anchors.

| Anchor | Where it is cited |
|---|---|
| `usage-running-grove` | `README.md` reader contract — the required citation, and the anchor whose subject is exactly this book's |
| `usage-tree-verbs` | ch. 2, beside the twelve `grove-llm` verbs |
| `usage-driver-lease` | ch. 3, beside `DriverLease::acquire` |
| `usage-session-lifecycle` | ch. 3, beside `grove_loop::run` |

All four already exist in `docs/USAGE.md` in the explicit `<a id="…"></a>` form.

**The glossary.** `CONTEXT.md`, declared with five anchors.

| Anchor | Status | Where it is cited |
|---|---|---|
| `driver-lease` | exists | ch. 3 |
| `stated-vcs` | exists | ch. 3, beside `Workspace::resolve` |
| `loop-control-channel` | exists | ch. 3, beside the completion signal |
| `guaranteed-core` | **must be created** | ch. 5, beside the `prompt` module's row |
| `task-tree-scheme` | **must be created** | ch. 1 and ch. 5 |

**Two obligations fall outside the book and are `overview-book-k30`'s.**
`CONTEXT.md` carries its terms as bold paragraphs; only four have been promoted to
`###` headings with explicit anchors, and those four are the ones
`jj-workspace-book-k25` needed. So `guaranteed-core` requires promoting the bold
term to a heading and anchoring it, and `task-tree-scheme` requires only the
anchor line, its heading already existing. **Both must land before the book
validates**: a declared anchor the target does not carry is `M201` against the
manifest, reported whether or not any page cites it.

The minimal set — four guide anchors and the three existing glossary anchors, with
no work outside the book — was rejected because the two terms the book most
depends on would have been the two it navigated to worst, each a bare link into a
1,046-line document. The wide set, adding `usage-workspace-layouts`,
`usage-finish`, `session-epoch` and `session-kind`, was rejected as navigation
these five chapters do not need at the price of three more promotions.

**Nothing else is citable, and no book cites this one.** The permitted local
targets are this book's own pages, its three source roots, the guide path and the
glossary path. `crate-books-k14`'s brief said the overview goes first partly
because the other books link into it; they cannot, and that ground has been struck
from it. The overview is a book, not a hub. A reader moves between books through
[`ARCHITECTURE.md`](../ARCHITECTURE.md)'s *Documentation ownership* table, which
every book root is already owed a row in and which
`every_book_root_has_a_documentation_ownership_row` holds.

## The book's row in the ownership table

The overview's row is owed by `every_book_root_has_a_documentation_ownership_row`
and must not collide with the row `ARCHITECTURE.md` keeps for itself. After the
move those two rows read as a split of one subject rather than as two claims on it:

| Subject | Canonical source |
|---|---|
| Runtime and repository design — the decisions, the constraints, and the measurement records | this document |
| The `grove` binary's source, read page by page, and the system description its call reaches | `walkthroughs/overview/README.md` |

Which leaf writes which half: `overview-book-k30` adds the overview's row, because
the check goes red the moment the book root exists; `architecture-move-k31` amends
this document's own row, because that is the leaf that makes the narrowing true.

## What the book deliberately does not cover

### Rust, clap, and operating-system fundamentals

The audience knows Rust. `clap`'s derive macro, its `CommandFactory` model and its
help renderer are named where the source names them and explained only where an
argument rests on their behaviour — chapter 4's account of why
`#[arg(help = "")]` renders like a missing doc comment is that case. Signal
semantics are stated once in chapter 3, as that chapter's premise, with the
`128 + N` convention named rather than derived.

### Everything behind `grove_loop::run`

The largest omission, and the one the book states out loud. The loop, the task
tree, the kinds, the lifecycle and the session epoch are named — a reader cannot
follow chapter 3 otherwise — and none is explained. That description is
`ARCHITECTURE.md`'s until the `grove-loop` book lands, and the outbound-link
contract forbids this book citing it. Chapter 5 says so in one place rather than
apologising per row.

### `crates/grove`'s own tests directory

`Cargo.toml` argues at length for the repository-surface tests living in this
package — `reference_navigation.rs`, `plugin_fallback.rs`, `commit_guidance.rs`
and `retire_guidance.rs` — and chapter 1 owns and explains that argument. Their
bytes are outside the corpus: `tests/` directories are evidence, not roots. They
are cited as evidence throughout and never reproduced.

### `crates/grove-llm`'s source

Chapter 2 describes the agent surface because *Command surfaces* describes both,
and the twelve verbs are half of the audience-split argument. `crates/grove-llm/src/cli.rs`
is another book's corpus and no byte of it appears here.

### Launch configuration

`~/.config/grove/config.kdl`, the four template slots and the `.grove.kdl` delta
are named in chapters 2 and 3 as the reason the command line has nothing to
select. [`CONFIGURATION.md`](../CONFIGURATION.md) owns them, and the ownership
table's one-canonical-source rule is what keeps this book from re-describing them.

### The workspace root manifest, and the methodology plugin

The root `Cargo.toml` belongs to no crate and is excluded from every book's corpus
by the campaign's frozen-corpus rule. `plugins/grove/` is in no crate and
therefore in no book; chapter 5 names it as one of the repository's two products
and stops.

## What this brief does not settle

- **Prose, headings and figures within a chapter.** The responsibilities above are
  binding; the wording is the `draft` stage's, and the figures are `art`'s under
  the prose contract's *Figures* convention.
- **Fragment partitioning below the top-level block.** Intent-named literal and
  composite fragments inside each block are the owning slice's, subject to the
  ledger and the line-alignment rules.
- **Concept-index curation.** Which terms earn an entry is the authoring slices',
  as it was for both existing books.
- **The exact `README.md` H1 wording** beyond `[book].title` being `System
  overview`, which the manifest fixes and the page table must match.
- **Which clause of the three in-play sections is which.** The rule is settled and
  worked examples of each residue are given; applying it line by line is
  `architecture-move-k31`'s, and that is deliberate — this brief exists so k31
  applies a rule rather than eliciting one.

## Stated limits

**Nothing mechanically checks the move rule.** Whether a clause that moved was
description is a judgement, and no test distinguishes a well-applied move from a
badly-applied one. `book-check` proves the 204 lines reconstruct; the link suites
prove every citation resolves; neither can see that a decision was carried into a
book or that a description was left behind. The rule above and an adversarial read
of `architecture-move-k31` are the only controls, and this is the one place in the
campaign where a documented obligation has no machine holding it.

**Chapter 5 carries absorbed prose with no scoped checkpoint.** `assembly` owns no
block, so it is not a `--through` value and the module map is proved by the final
run or not at all. That is inherent to a final-only chapter and is the price of
giving *Main module seams* a home of its own rather than distributing it.

**The book's largest block is its least transferable.** Chapter 4 owns 84 lines,
41% of the corpus, and they are unit tests for a struct with no fields. The
chapter earns its place through the outcome — a closure property is the second of
the three mechanisms — but a reader who wanted the system and not the technique
will find this the chapter to skip, and the prose should not pretend otherwise.
