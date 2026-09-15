# jj-workspace — book structure brief

## Status and provenance

This is the structure brief for the book at
`docs/walkthroughs/jj-workspace/`, which does not exist yet and which this
document is written ahead of, in the form [`walkthrough-books.md`](walkthrough-books.md) requires under *The
structure brief*. It settles what that specification deliberately does not: the
chapter sequence, the mapping of that sequence onto the corpus, each chapter's
worked example, the early uses the order forces, and what the book does not
cover.

**This document is authored, not recovered, and it precedes its book.**
[`ordinal-fs-tree-book-structure.md`](ordinal-fs-tree-book-structure.md) was
reconstructed from a deleted specification after its book already existed; this
one was elicited from the human before a page of the book was drafted, which is
the order decision 15 of `plan-k1` intends and the order every later book
follows. Every decision below was settled in the `jj-workspace-structure-k17`
interview and is recorded, with its rejected alternatives, in that leaf's
decision log.

**This brief is the human contract; the manifest is its machine-readable form.**
The chapter sequence and ownership mapping below are what
`docs/walkthroughs/jj-workspace/walkthrough.toml` records as its `[[page]]` and
`[[block]]` groups. Where the two disagree, that is a defect in one of them, not
a licence to prefer either.

## Audience and intended outcome

The audience is settled by decision 7 of `plan-k1` and is not re-opened here: a
reader who knows Rust and Jujutsu and has driven a grove, for whom grove's
vocabulary is linked to the glossary and never re-taught, and whose entry point
to the system is [`docs/USAGE.md`](../USAGE.md).

**The intended outcome is a transferable test, not a reference card.** At the end
the reader can take a boundary in their own code, ask *what does the tool
underneath already own*, and tell a justified subtraction from an abdication. The
closing chapter states that test and applies it to all six of this crate's
refusals.

Two candidate outcomes were rejected, and one of them by fact rather than by
preference. *Take this crate as a dependency* is not available:
[`RELEASING.md`](../RELEASING.md), *One release, seven packages, one tag*, settles
that only `crates/grove` is released and that the five library members ship
inside grove's cut with no release lane of their own, so this crate has exactly
one consumer and it is in this repository. *Rely on it without reading it* was
rejected because a source-exact book is a poor way to deliver a reference card
for a fourteen-line public surface. The maintainer's outcome — locate any
behaviour, name the invariant a change would break, name the test that catches
it — follows for free from source-exactness and is therefore not the stated goal.

## The spine: what the crate refuses to own

**Every chapter opens on something this crate declines to own, and names who owns
it instead.** The spine is recovered from the source rather than imposed on it:
`lib.rs` carries three headed theses (*It knows nothing about its consumer*, *It
takes commits; it does not implement transactions*, *Reads add no history*) over
an opening that says "no repository abstraction here and no second lane behind
it"; `Cargo.toml` carries a fourth ("**No dependencies, and that is the
deliverable rather than an accident**"); `jj.rs` carries two more; and
`refusal.rs` carries the last ("The remedies named here are **jj's**").

**Row 6 does not repeat that comment's wording, and the book does not take it
at its word.** The module comment overstates its own file: only two of
`refusal.rs`'s eleven kinds name a jj command, so *the remedies are jj's* is
where the sixth thesis was recovered from rather than what it claims. What
holds across all eleven is the negative — no remedy it names is a statement
about one consumer's policy — and that is what row 6, the early-use statement for
`Refusal` below, and chapter 6 all say
(`refusal-remedies-are-jjs-overclaim-k202`).

| # | The refusal | Who owns it instead | Chapter |
|---:|---|---|---|
| 1 | No dependencies | `std` spawns a process and reads a directory | 1 |
| 2 | No second lane, and no repository abstraction | jj is the version control system | 2 |
| 3 | Nothing ambient chooses the repository | `current_dir`, with the selectors removed | 3 |
| 4 | No vocabulary for its consumer | the consumer, which supplies the namespace | 4 |
| 5 | No transactions, and no history added by a read | jj's snapshot and its operation log | 5 |
| 6 | No remedy of its own to offer | sometimes jj, elsewhere the filesystem, the environment or this crate's own rules — never the caller | 6 |

[`CONTEXT-MAP.md`](../../CONTEXT-MAP.md) argues that `jj-workspace` is
deliberately **not** a bounded context, because every term in it is Jujutsu's and
what the crate adds is a namespace it will not name for its consumer. That
argument is refusal 4 — chapter 4's thesis, restated in the close — rather than
the whole book's proposition. Taken as the whole book's proposition it would
leave `resolve`, `main_repo_of`, the fileset algebra and the subprocess hygiene,
roughly half the corpus, to be bent to fit.

## Chapter sequence

Seven pages, six of which own source. `README.md`, `concept-index.md` and
`source-index.md` are the contents page and the two lookup surfaces; they are not
chapters and not alternate explanatory paths.

Slice IDs are opaque book-system tokens, enumerated by the manifest and
meaningful because it enumerates them. They are deliberately distinct from page
IDs so a page rename does not migrate fragment ownership, ledger rows and
`--through` values with it.

**They carry no Grove task key, and the precedent's `-kN` spelling is not
followed.** Those keys belong to the task tree that happened to produce the first
book; they are not the book system's, and
[`walkthrough-books.md`](walkthrough-books.md) already records the spelling as
historical and the values as meaningful because the manifest enumerates them.
This book's slices are named for the **refusal each chapter carries** while its
pages are named for their subject, so the two vocabularies differ by
construction — which is the property the separate slice domain exists for, bought
without a token that looks like a work-item handle `resolve` could find.

| Order | File | Page ID | Title | Slice |
|---:|---|---|---|---|
| 1 | `01-orientation.md` | `orientation` | Orientation | `no-dependencies` |
| 2 | `02-the-gate.md` | `the-gate` | The gate | `one-lane` |
| 3 | `03-subprocess-seam.md` | `subprocess-seam` | The subprocess seam | `nothing-ambient` |
| 4 | `04-namespace.md` | `namespace` | The namespace it will not name | `no-consumer-vocabulary` |
| 5 | `05-scope-and-commit.md` | `scope-and-commit` | Scope and commit | `no-transactions` |
| 6 | `06-refusal.md` | `refusal` | Refusal | `no-remedy-of-its-own` |
| 7 | `07-what-jj-owns.md` | `what-jj-owns` | What jj owns | `assembly` |

`assembly` owns no production source and is therefore final-only: it has no
scoped prefix to prove.

**The order is narrative, not strictly bottom-up, and that is a decision rather
than an oversight.** Read strictly by reader-dependency, `refusal.rs` would come
second — every function in the crate returns it — and `jj.rs` third. It is placed
sixth instead, because reading ten remedy messages for cases the reader has not
met is exactly the *catalogue before the worked example* failure the precedent
forbids; that precedent placed its own `error.rs` sixth of eight for the same
reason. The price is paid in the early-use ledger below, and it is two rows.

## Concept and seam responsibilities

### `README.md` — reader contract

State the audience, scope, exclusions, source-authority rule, exact-fragment
claim, canonical page order, lookup paths, and the distinction between scoped and
final completeness. Explain how to recognise a fragment definition, insertion,
source root and deferred hole without duplicating the full grammar. Carry the
book's one guide citation, naming a declared anchor.

### 1 · Orientation — a crate defined by what it declines

Establish the crate's purpose in its own one sentence, the six refusals as the
book's map, and the public surface: `Workspace`, `Commit`, `Refusal`, and the
four operations. Own the whole manifest, and read it as the first refusal — an
empty `[dependencies]` table is the deliverable, `tempfile` is confined to
dev-dependencies so no consumer inherits it, and `release = false` is an answered
question rather than a deferred one.

Then follow the carried operation at low resolution through this exact path:

```text
Workspace::resolve(cwd)
→ the ancestor walk finds `.jj/`, and the root is canonicalised
→ main_repo_of: `.jj/repo` is a directory here, so no jj is spawned
→ Workspace { root, main_repo }
→ Workspace::control_dir("grove")
→ `.jj/grove` created if absent
→ Workspace::commit(&[".grove/01-…md"], message)
→ relative → fileset → `jj commit -m … root:"…"`
→ `jj log -r @- --no-graph --ignore-working-copy -T change_id`
→ Commit { change_id }
```

The trace uses a concrete starting tree, real names and values, the exact
arguments jj is spawned with, and the change id that comes back. It names the
types and functions above but copies no source owned by a later chapter; each
later-owned name receives the minimum local statement recorded in *Early uses*.

### 2 · The gate — resolution is a precondition, not a dispatch

Thesis: **there is no second lane**, so the absence of a workspace is never a
case to handle. Explain `Workspace` as a value whose existence is the proof that
the precondition passed, `resolve` as a filesystem walk that invokes no
repository discovery, canonicalisation as what makes symlink and relative aliases
resolve to one workspace, and `main_repo_of` as the one place resolution may
spawn jj.

State here, as this chapter's premise, the jj behaviour it rests on: a workspace
that borrows another's repository has `.jj/repo` as a pointer **file**, and one
that holds its own has the repository directory there — the same file-versus-
directory shape Git uses for `.git`. Explain why the test is *is there a pointer*
rather than *is the repository intact*, and what asking the second question would
have cost every resolution.

### 3 · The subprocess seam — the repository is chosen by nothing ambient

Thesis: **no ambient state selects the repository.** Own `jj.rs` entire.
Explain why the hygiene is a property of the crate rather than a habit at each
call site, the four repository selectors and why they are `env_remove`d rather
than left unset, the argued **absence** of a `JJ_*` counterpart — jj walks up
from the working directory to select a repository, and its own variables
configure the *user*, so stripping them would change who a commit is attributed
to — and why a consumer's own ambient variables are left alone.

Separate failure to *start* from failure to *succeed*, and connect each to the
refusal it becomes. Explain `produced_output` as the probe whose whole answer is
whether stdout was empty, and `rendered` as the reason a refusal can quote a
command a reader could type.

### 4 · The namespace it will not name — the crate's one addition

Thesis: **the crate has no vocabulary for its consumer**, so the consumer
supplies the name. This is the chapter that carries
[`CONTEXT-MAP.md`](../../CONTEXT-MAP.md)'s argument. Explain the postcondition
that only a named namespace makes sayable — *this directory is yours, it is
inside the workspace, and nothing tracks it* — and why it lives in the
administrative directory rather than the tracked working copy.

Own `JJ_OWNED_NAMES` and `validated_namespace`. Explain the one-directional cost
argument behind the reserved list: a name jj adds later that this list has not
heard of costs a collision, and a name listed here that jj drops costs a consumer
a different word. Explain why creation is all `control_dir` promises, and why a
writability probe would be a second answer that can already be stale.

### 5 · Scope and commit — jj already has the transaction

Thesis: **no transactions, and reads add no history.** Own `Commit`,
`is_tracked`, `commit`, `fileset` and `relative`.

State the jj behaviour this chapter rests on: the working copy is snapshotted
before every command, and the operation log *is* the transaction record — which
is why there is no witness, manifest, rollback proof, index image or quarantine
here. State the second: a change id survives `describe`, `squash` and a rebase,
which is why `Commit` holds one.

Explain the **asymmetry** as this chapter's central claim. Every probe passes
`--ignore-working-copy` except `is_tracked`, whose answer depends on the working
copy; and that exception is *measured* (jj 0.44.0), not assumed — a snapshotting
probe records an operation only when the working copy has actually changed.
Carry the path algebra as one argument rather than three functions: `resolve`
canonicalising the root is *why* `relative` must canonicalise the caller's path,
and canonicalising the **parent** is why a path the caller has just deleted is
still committable.

Explain why an empty `paths` is refused rather than widened, and why the refusal
from reading the change id afterwards means something different from the refusal
from the commit itself.

### 6 · Refusal — it speaks for jj, and never for you

Thesis: **the crate has no consumer to speak for**, so it never says what the
caller should do; no remedy it names is a statement about one consumer's
policy. Own `refusal.rs` entire. Explain why the type is opaque — every case is
a stop, so there is nothing to branch on — and what `Display` and
`Error::source` give a consumer in place of matchable variants. Explain why the
error is the crate's own type and leaks no error crate, closing the loop with
chapter 1's empty dependency table.

Walk the ten kinds at the sites that create them rather than as a catalogue, and
give the two that carry remedies their full weight: the gate's refusal, whose
two-line repair is stated unconditionally rather than chosen by probing for a
`.git`, and `CommitNotRecorded`, the only refusal that has to say something about
*state* rather than about a command.

### 7 · What jj owns — assembly and the transferable test

Own no production source. Assemble the six refusals into one table of *what was
subtracted, who owns it, and what the subtraction bought*. State the test that
separates a justified subtraction from an abdication — the underlying tool's
guarantee must be at least as strong as the one the consumer needs, and
checkable — and apply it to all six, including the one where the answer is least
comfortable. Close the early-use and source ledgers, confirm every deferral has
become an insertion, and record the final verification commands and results.

## The mapping onto the corpus

Four source roots, in this fixed order — the order the source index presents and
the manifest's `[[root]]` array follows.

| Root ID | Source path | Lines |
|---|---|---:|
| `source-crate-manifest` | `crates/jj-workspace/Cargo.toml` | 44 |
| `source-library` | `crates/jj-workspace/src/lib.rs` | 343 |
| `source-subprocess` | `crates/jj-workspace/src/jj.rs` | 81 |
| `source-refusal` | `crates/jj-workspace/src/refusal.rs` | 230 |
| **total** | | **698** |

**Every line count in this brief records the crate as it was when the book was
commissioned, and the crate has since moved twice.** `lossy-path-rendering-k66`
added sixteen lines to `lib.rs`, twenty-four to `refusal.rs` and eighty-five to
`tests/workspace.rs`; `duplicated-cause-k67` then added fourteen more to
`refusal.rs` and twenty-eight to `tests/workspace.rs`. Both are source changes
the corpus-freeze rule in `.grove/_BRIEF.md` permits, each landed in one commit
with every page and ledger row it invalidated. The current figures are 359, 268
and 752 for the corpus, and 1,012 across the two test files — the corpus is what
those two leaves left it at, and the tests have moved several times since, most
recently for the doc comment corrected at `colocated-fixture-control-claim-k204`
(*The tests*, below, says 755, which was already short of the mark before either
leaf). The
authoritative record is the book's own `walkthrough.toml` and
`source-index.md`, which `book-check --final` proves. This brief is left at the
numbers it was written with, because it is the input the book was built from
rather than a description of the book today.

**The corpus rule needs no exceptions, and this is the first book of which that
is true.** `include` is the crate's own `Cargo.toml` plus
`crates/jj-workspace/src/**/*.rs`, and that pattern pair yields exactly these
four roots: there is no `bin/`, no test-support module and no inline `tests.rs`.
The manifest carries no `[[corpus.add]]` and no `[[corpus.exclude]]`, and the
normative inventories in [`walkthrough-books.md`](walkthrough-books.md) record
that emptiness rather than omitting the book.

### Top-level ownership blocks

Eleven blocks. The State column is the required state after the orientation
slice; a later owner changes only its own rows from `deferred` to `resolved` when
it replaces the matching defer with an insert and adds the definition.

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---:|---|
| `manifest-no-dependencies` | `source-crate-manifest` | `no-dependencies` | `1-44` | 44 | `resolved` |
| `library-crate-thesis` | `source-library` | `no-dependencies` | `1-54` | 54 | `resolved` |
| `namespace-reserved-names` | `source-library` | `no-consumer-vocabulary` | `55-61` | 7 | `deferred` |
| `commit-identity` | `source-library` | `no-transactions` | `62-71` | 10 | `deferred` |
| `workspace-value-and-gate` | `source-library` | `one-lane` | `72-118` | 47 | `deferred` |
| `namespace-control-dir` | `source-library` | `no-consumer-vocabulary` | `119-145` | 27 | `deferred` |
| `scope-tracking-and-commit` | `source-library` | `no-transactions` | `146-274` | 129 | `deferred` |
| `gate-main-repo-and-canonical` | `source-library` | `one-lane` | `275-319` | 45 | `deferred` |
| `namespace-validation` | `source-library` | `no-consumer-vocabulary` | `320-343` | 24 | `deferred` |
| `subprocess-seam-source` | `source-subprocess` | `nothing-ambient` | `1-81` | 81 | `deferred` |
| `refusal-source` | `source-refusal` | `no-remedy-of-its-own` | `1-230` | 230 | `deferred` |

### Where a file's concerns split across chapters

**Exactly one root has more than one owner.** `src/lib.rs` interleaves four
chapters' concerns across seven alternating blocks, because the file is ordered
by Rust's conventions — constants, then types, then the `impl`, then free
functions — and the book is ordered by concept:

- **the gate** owns the `Workspace` value, `resolve` and the two accessors
  (`72-118`), and the free functions `main_repo_of` and `canonical` (`275-319`);
- **the namespace** owns the reserved-name constant (`55-61`), `control_dir`
  (`119-145`) and `validated_namespace` (`320-343`) — three ranges separated by
  the whole of the `impl`, and one concept;
- **scope and commit** owns `Commit` (`62-71`) and the contiguous run of
  `is_tracked`, `commit`, `fileset` and `relative` (`146-274`);
- **orientation** owns the module documentation, the module declarations, the
  re-export and the imports (`1-54`).

Whole-file ownership with no planned holes was considered and rejected: it would
put the namespace argument, the gate and the commit algebra on one page, which is
the five-page compression this brief's interview already rejected. The other
three roots are each one full-file block.

### Owned-source totals

| Slice | Page | Owned lines |
|---|---|---:|
| `no-dependencies` | `01-orientation.md` | 98 |
| `one-lane` | `02-the-gate.md` | 92 |
| `nothing-ambient` | `03-subprocess-seam.md` | 81 |
| `no-consumer-vocabulary` | `04-namespace.md` | 58 |
| `no-transactions` | `05-scope-and-commit.md` | 139 |
| `no-remedy-of-its-own` | `06-refusal.md` | 230 |
| `assembly` | `07-what-jj-owns.md` | 0 |
| **Total** | 4 source roots | **698** |

## What each chapter's prose owes

Measured before this was decided: **37% of the corpus is comment prose** —
`Cargo.toml` 54%, `lib.rs` 47%, `jj.rs` 39% — and most of `refusal.rs`'s
remainder is user-facing remedy text inside string literals. The standing risk
for this book is therefore paraphrase rather than obscurity, because the fragment
graph quotes the argued comment verbatim on the page anyway. Three things a
comment structurally cannot do are what each chapter's prose owes, and a
technical review checks for them:

1. **Adjudicate the claim.** For every argued claim, name the jj behaviour it
   rests on, the test that proves it, and the alternative rejected with what it
   would have cost. A doc comment cannot cite an integration test three files
   away; a page can.
2. **Carry the through-line.** Show where a decision in one file is only
   explicable by a decision in another. No single comment holds the argument that
   runs `resolve` → `relative` → `commit`.
3. **Supply the consumer's half.** The crate refuses to know its consumer, so its
   comments can never say what a call is *for*. The book says what grove does
   with each one.

**The third is marked, always.** Naming grove in every chapter risks blurring the
boundary the whole spine is built on, so a consumer's-half passage is set apart
from the surrounding text and never mixed into the crate's own vocabulary. The
reader must be able to see at a glance which sentences the crate could have
written and which it could not.

## Worked examples

Six chapters carry the named example section. The seventh does not: it owns no
production source and its job is synthesis.

| Chapter | Required anchor | Start and observable end |
|---|---|---|
| `01-orientation.md` | `commit-tour` | The carried operation at low resolution: one starting tree, through resolve, reservation and commit, to the change id that comes back. |
| `02-the-gate.md` | `worked-resolution` | The same tree resolved from a subdirectory; the same tree with no `.jj/`, ending in the two-line remedy with nothing created; and a secondary workspace resolving to a different main repo. |
| `03-subprocess-seam.md` | `worked-invocation` | One invocation built end to end: the exact argv, the working directory, the four removed variables, and both failure endings — jj absent, and jj declining. |
| `04-namespace.md` | `worked-reservation` | `control_dir("grove")` creating `.jj/grove`; the same call again returning the same directory with its contents kept; and a path, an empty name and a jj-owned name each refused. |
| `05-scope-and-commit.md` | `worked-commit` | The carried operation at full resolution: a caller's path through `relative` and `fileset` into the exact `jj commit` argv, the change-id read, and the `Commit` returned. |
| `06-refusal.md` | `worked-refusal` | The identical attempt where `jj commit` declines: `CommandFailed` wrapped as `CommitNotRecorded`, the rendered message, and the `source()` chain a consumer walks. |

**One command, carried through the book, with two endings.** Chapters 1 and 5 are
the same operation at low and then full resolution — the operation grove itself
performs, and the namespace string in it is the literal `"grove"` that
`crates/grove-loop/src/driver_lease.rs` passes. Chapter 6 retraces that *same*
attempt to a refusal instead of a `Commit`, so the failure path is a variation on
a trace the reader already holds rather than a second scenario to learn.

Showing the crate through its one real consumer is a deliberate tension with the
spine, and it is what makes chapter 4's thesis checkable: the reader watches
`"grove"` be passed **in** as an ordinary string argument, rather than being told
that the crate does not name its consumer.

**The example comes before the catalogue.** No section earlier than a chapter's
worked example may primarily enumerate three or more public operations, refusal
kinds, environment variables or validation rules; those catalogues follow the
complete example. A reviewer checks that the stated start, the transition steps,
and the observable end are all present.

## Early uses the order forces

Two costs are paid here. Orientation traces the carried operation through every
layer, so every type it names is owned later; and the narrative order puts the
gate ahead of the subprocess seam it calls. These eight rows are the minimum
ledger. Authors add a row before introducing any additional later-owned name.

| Symbol family | First use | Owner | Minimum local statement |
|---|---|---|---|
| `Workspace` | `01-orientation.md#public-surface` | `one-lane` | A resolved workspace is a value whose existence is the proof that the precondition passed; it carries the workspace root and the root of the workspace that holds the repository. |
| `Refusal` | `01-orientation.md#public-surface` | `no-remedy-of-its-own` | The one error type: an opaque value carrying what is wrong, where, and — where it has one — a remedy that is true whoever is calling, with no matchable variants because every case is a stop. |
| `Commit` | `01-orientation.md#public-surface` | `no-transactions` | What a taken commit returns: a change id rather than a commit id, because a change id still names the work after a rewrite. |
| `control_dir` | `01-orientation.md#public-surface` | `no-consumer-vocabulary` | A namespace is one plain directory name the consumer supplies; the directory it names is inside the workspace, untracked, never shared, and created if absent. |
| `main_repo_of` | `01-orientation.md#commit-tour` | `one-lane` | The step in resolution that decides which workspace holds the repository: `.jj/repo` is a directory in a workspace that holds its own and a pointer file in one that borrows another's, and only the borrowed case is followed. |
| `is_tracked` | `01-orientation.md#the-six-refusals` | `no-transactions` | The one probe whose answer depends on the working copy, and so the one that lets jj snapshot before answering. |
| `jj::output`, `jj::produced_output` | `02-the-gate.md#worked-resolution` | `nothing-ambient` | Every jj invocation the crate makes is built at one seam that fixes the working directory, removes the repository selectors, and separates failure to start from failure to succeed. |
| `Refusal::not_a_workspace`, `Refusal::unresolvable_path` | `02-the-gate.md#worked-resolution` | `no-remedy-of-its-own` | Refusal constructors are crate-internal; the gate's refusal names jj's two initialisation commands and states that nothing was created or changed. |

A row is `pending` while its owner is outside the current scoped prefix and
`explained` from the owner's scoped increment onward. The owning slice changes
only its own rows.

**The local statement is judged, not matched.** The validator checks row
structure, owner order, page and anchor existence, and resolution status; whether
a statement is present and sufficient is a technical-review judgement, and
keyword matching is not a substitute for it.

## Outbound links

The book is a Grove runtime crate's book, so [`walkthrough-books.md`](walkthrough-books.md)'s
*Outbound links* binds it unchanged: it declares a `[guide]` path with a non-empty
anchor array, and its `README.md` reader contract cites one of those anchors. The
`[guide] omitted` shape is not available to it — that exemption rests on
`ordinal-fs-tree` being a bounded context of its own, which
[`CONTEXT-MAP.md`](../../CONTEXT-MAP.md) explicitly denies for this crate.

| Target | Anchor | Cited from | Why this book reserves it |
|---|---|---|---|
| `docs/USAGE.md` | `usage-workspace-layouts` | `README.md` | The required reader-contract citation, and the guide section whose subject is exactly this crate's native, colocated and secondary cases. |
| `docs/USAGE.md` | `undoing-a-mistake` | `06-refusal.md` | Beside `CommitNotRecorded`, whose remedy is `jj undo` and `jj op log`. |
| `docs/USAGE.md` | `usage-driver-lease` | `04-namespace.md` | Beside `control_dir("grove")`, which is what the driver lease reserves. |
| `CONTEXT.md` | `task-commit-boundary` | `05-scope-and-commit.md` | The grove term for what a path-scoped commit is *for*. |
| `CONTEXT.md` | `driver-lease` | `04-namespace.md` | The consumer's-half answer to what lives in the reserved directory. |
| `CONTEXT.md` | `stated-vcs` | `02-the-gate.md` | The grove term the gate's one-lane refusal serves. |
| `CONTEXT.md` | `loop-control-channel` | `04-namespace.md` | The signal files the reserved directory holds. |

**One obligation follows and is placed rather than noted.** All three guide
anchors already exist in `docs/USAGE.md` in the required explicit form. `CONTEXT.md`
carries **no** explicit anchors at all, and the specification puts that obligation
on the first book to cite the glossary — which is this one. `jj-workspace-book-k25`
adds the four explicit `<a id="…"></a>` lines above to `CONTEXT.md`, before the
book can validate. Adding them is additive and breaks nothing that exists.

## What the book deliberately does not cover

### Rust and operating-system fundamentals

The book explains surprising uses of Rust or operating-system behaviour **in this
crate**, and does not teach process spawning, environment inheritance, paths and
canonicalisation, `io::Error`, trait implementation or standard test mechanics
from first principles. It does explain this crate's specific choices: why the
error type is opaque, why a namespace is validated rather than reinterpreted, why
canonicalisation is applied to a path's parent, and what `Error::source` is
expected to carry.

### Jujutsu itself

The reader knows jj (decision 7 of `plan-k1`), and the book does not teach it.
Only the behaviours a chapter's argument *rests on* are stated, once, in that
chapter, as its premise, with a link to jj's own documentation: the pointer-file
shape of a borrowed repository (chapter 2); repository selection by walking up
from the working directory, and `JJ_*` configuring the user (chapter 3); working-
copy snapshotting, the operation log as the transaction record, and change-id
survival across rewrites (chapter 5). A reader disputing a chapter's thesis finds
its premise on the same page.

### The tests

`crates/jj-workspace/tests/` — 755 lines across `workspace.rs` and
`environment.rs` — is outside the corpus: no chapter owns it and no fragment
reconstructs it. Tests remain **evidence** the book cites, and the book cites them
heavily, because *adjudicate the claim* is what each chapter's prose owes. The
only trace of them inside the corpus is the dev-dependency on `tempfile`, which
chapter 1 owns and explains.

### The consumers' own source

`crates/grove-loop`'s driver lease and `crates/grove-llm`'s CLI are named in
marked consumer's-half passages and cited by path, never reproduced. They are
other books' corpora, and reproducing them here would put one crate's bytes in
another crate's fragment graph.

### Git, beyond the boundary

A `.git` beside a `.jj` is a colocated repository and is jj's business; the crate
never reads it, spawns `git`, or branches on its presence. The book states that
once, explains why four `GIT_*` variables are nevertheless removed at the
subprocess seam — a Git-aware child following an inherited foreign repository is
a real hazard where the backend is real — and does not otherwise explore
colocation.

### Cross-layer source in orientation

Orientation owns no source belonging to a later chapter. Copying narrow fragments
from every layer into it would make its low-resolution trace visibly
source-backed, but it would violate the settled chapter boundary and create five
additional cross-slice ownership joins. Exact identifiers, concrete values, the
early-use ledger and source-backed later chapters give the same technical trace
with a smaller fragment graph.

## What this brief does not settle

Recorded as open rather than filled in, so a later reader can tell a decision
from an invention.

- **The concept-index entries.** The two lookup surfaces are required by
  [`walkthrough-books.md`](walkthrough-books.md) and their contents are the
  authoring slices' to curate, not this brief's to enumerate.
- **The fragment partition inside each block.** This brief fixes top-level
  ownership; how a block divides into intent-named literal and composite
  fragments is the owning slice's work.
- **The exact starting tree of the carried example.** The shape is settled above —
  a native workspace, a `.grove/` task file, a caller in a subdirectory — but the
  concrete names, paths and change id are the orientation slice's to fix, and
  chapters 5 and 6 then reuse them unchanged.

## An open conflict, for whoever next revises this brief

**Chapter 1's public-surface enumeration and the catalogue rule cannot both be
satisfied, and this brief is where the conflict is owned.** The chapter
description above requires the enumeration of `Workspace`, `Commit`, `Refusal`
and the four operations, and fixes `01-orientation.md#public-surface` as the
First-use anchor of two ownership-ledger rows. [`walkthrough-books.md`](walkthrough-books.md)
requires that no section earlier than its chapter's worked example primarily
enumerates three or more public operations. Both standards bind, and the book as
published satisfies the first at the cost of the second.

The editorial-pipeline pilot recorded it at every stage that met it — the
developmental edit, the copy edit and proof each declined to act, correctly,
because a stage may not revise the standard it is judged against — and the
proof-only probe found it independently at that exact anchor, which is as strong
a confirmation as that pilot could produce that it is reader-visible rather than a
scruple. The evidence is
[the pilot report](../evaluations/editorial-pipeline-pilot/README.md), which is
frozen and is not edited by this or any later session.

**It is recorded here because this brief is the artifact a correction has to move
through.** Two ways it can go, and neither is settled: move chapter 1's
enumeration behind the chapter's worked example and re-point the two ledger rows,
or state an explicit exception in `walkthrough-books.md` for an orientation
chapter whose subject *is* the public surface. A revision that changes chapter 1
without answering this has closed the section and left the conflict.
