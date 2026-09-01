# developmental edit — stage record

Stage 1 of six, and the first scored one. Its charter, from the
[preregistration](../preregistration.md)'s stage table: *structure, conceptual
order, what a chapter is for, and whether the book delivers the reader outcome the
structure brief asked for. Structure, not sentences.* It owns `D1` to `D5` and no
other class.

The standard it judges against is
[`jj-workspace-book-structure.md`](../../../specs/jj-workspace-book-structure.md),
which is not this stage's to revise: a disagreement with the brief is recorded
below as a finding, never acted on as a licence.

<a id="provenance"></a>
## Provenance

**Stage:** developmental edit (stage 1).
**Change id:** `qrtxukzpmuxt`.
**Parent change id:** `nurklzvurwwp` — `what-jj-owns-k61: draft chapter 7 of the
jj-workspace book and close the draft stage`, which is the last commit of the
draft range and the state
[`0-draft.md`](0-draft.md)'s `## Baseline` fixes.

| | Book-directory digest |
|---|---|
| before | `0e9d3f1e7d356c33509327dfe4df584ffa6ca531e531049098cdcf6b923aca14` |
| after | `669eee6fbaa35925c48c1c17a411dd555eb625ae36a492696c1af75ea0da4bde` |

The before value is the draft record's recorded final digest, re-computed at the
start of this session and unmoved. Digests follow the preregistration's recipe —
`find <dir> -type f \( -name '*.md' -o -name '*.toml' \) | LC_ALL=C sort | xargs
shasum -a 256 | shasum -a 256`.

**Preregistration read by this session:** SHA-256
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e` — the same
value every child of the draft range recorded.

<a id="frozen-inputs"></a>
## Frozen inputs, re-checked

Re-computed at the start of this session, before any edit. All nine match.

| Input | SHA-256 as read | Verdict |
|---|---|---|
| `crates/jj-workspace/Cargo.toml` | `528fe93e62e0807b97a02d451ff04a86f1a1f0bd7f290de711684f3920c70936` | unmoved |
| `crates/jj-workspace/src/lib.rs` | `43ca4ad3947db34a326e5e85051617ab544bf5ee8d7df40fcc229597f7c1c9e3` | unmoved |
| `crates/jj-workspace/src/jj.rs` | `23eb9d98df0eb1906f25cfefd869c606d2750b99b703d35d0c533f5cba1f8af6` | unmoved |
| `crates/jj-workspace/src/refusal.rs` | `6ea88fc2678d7b1ad4f997adf42705699d0a9bda94b997d753902828409cdc3f` | unmoved |
| `docs/specs/walkthrough-books.md` | `d9556f93b1112fcda770aa4564c517a08b076ee241010132c7cf428abd86535d` | unmoved |
| `docs/specs/jj-workspace-book-structure.md` | `f7c9fa4ad87c79940dbb6d393d4cb03875641c17472e90f5b5eeb99327739182` | unmoved |
| `docs/USAGE.md` | `ed79be856e3427c1e56f48a33b34f9b4cd4111b7c9d9bdaa17dad9917556a59f` | unmoved |
| `CONTEXT.md` | `c0d4698440f2d925f6c67563487add708100cd8188be025267039be09b733822` | unmoved, at the post-draft value the draft record explains |
| `docs/walkthroughs/ordinal-fs-tree/` | `03c1fd3ebebffbf03202cfa5a597dae598293300c04a6abcaa4fb7199d5b7553` | unmoved |

**The corpus was not touched.** This commit contains no change under
`crates/`; all four roots carry the digests the preregistration froze.

<a id="claims"></a>
## Claims

| # | Class | Anchored subject | Before (verbatim citation) | After (verbatim citation) | What a reader gains |
|---:|---|---|---|---|---|
| 1 | `D3` | `01-orientation.md#what-it-declines` | see *Citation 1* below | see *Citation 1* below | Chapter 1 now opens the way chapters 2 to 6 open — on its refusal, with the owner named — so the pattern the chapter and the `README.md` reader contract both promise is true of the chapter the reader is standing in. |
| 2 | `D1` | `01-orientation.md#commit-tour` | see *Citation 2* below | see *Citation 2* below | The one later-owned identifier chapter 1 introduced off the ledger is now on it, with its minimum local statement in prose rather than only inside a trace fence — so every name the orientation borrows from a later chapter is accounted for in the same place. |
| 3 | `D5` | `01-orientation.md#commit-tour` | see *Citation 3* below | see *Citation 3* below | The ledger is a true index of where each borrowed name is established, so a reader who follows a row's First use arrives at the statement rather than at a trace that assumes it. |

The citations are carried below the table rather than inside it. Every one of them
is multi-line, and two quote Markdown table rows whose pipes cannot survive a table
cell; a citation mangled to fit a column is not a verbatim citation, and the report
has to locate each of these in the diff.

### Citation 1 — claim 1, `D3`

**Before**, the whole of the section's opening, running from the H2 straight into
the third paragraph:

> `jj-workspace` resolves a Jujutsu workspace, refuses a working tree that is not
> one, and takes a path-scoped commit. That is the whole crate: four operations, a
> value, and one error type, over roughly seven hundred lines with no dependencies.
>
> A crate that small is normally explained by listing what it does.

Neither refusal 1 nor `std` appears anywhere in the section, while the paragraph
below it tells the reader: *Each chapter of this book opens on something this crate
declines to own and names who owns it instead.*

**After**, inserted between those two paragraphs:

> The first of those refusals is this chapter's, and it is declared rather than
> argued: **no dependencies**. `std` owns what was subtracted — it spawns the child
> process, it reads the directory, and it supplies the `Error` trait the crate's one
> error type implements — so nothing else is taken, and nothing is imposed on a
> consumer that takes this crate. The manifest is where that refusal is written
> down, and it is read first below.

### Citation 2 — claim 2, `D1`

**Before**, inside the carried trace at the anchor, with no prose and no ledger row
anywhere in the book naming it:

> ```text
>   main_repo_of("/work/atlas"):
>     `.jj/repo` is a directory, not a pointer file
>     -> the workspace holds its own repository; no jj is spawned
> ```

`main_repo_of` is owned by the `one-lane` slice — `02-the-gate.md`, two chapters
later — and the structure brief's early-use table is *the minimum ledger. Authors
add a row before introducing any additional later-owned name.*

**After**, a prose paragraph immediately below the trace fence:

> `main_repo_of` is the one step in that trace that could have spawned jj and did
> not, and it is named here because the reader meets it two chapters before the page
> that owns it. It decides which workspace holds the repository: `.jj/repo` is a
> directory in a workspace that holds its own and a pointer file in one that borrows
> another's, so only the borrowed case has a pointer to follow — and following it is
> jj's job rather than this crate's. *The gate* owns it.

together with a new row in both the manifest and the ledger, whose Minimum local
statement reads *The step in resolution that decides which workspace holds the
repository: `.jj/repo` is a directory in a workspace that holds its own and a
pointer file in one that borrows another's, and only the borrowed case is followed.*

### Citation 3 — claim 3, `D5`

**Before**, two ledger rows declaring `#commit-tour` as their First use (pipes are
the ledger's own column separators):

```text
| `control_dir` | `01-orientation.md#commit-tour` | `no-consumer-vocabulary` | A namespace is one plain directory name the consumer supplies; …
| `Commit` | `01-orientation.md#commit-tour` | `no-transactions` | What a taken commit returns: a change id rather than a commit id, …
```

Neither statement is at `#commit-tour`. Both are at `#public-surface`, the section
above it, which reads *A namespace, in this crate's vocabulary, is one plain
directory name the consumer supplies. The directory it names is inside the
workspace, untracked, never shared with another namespace, and created if absent.*
and *`Commit` is what a taken commit returns. It carries a change id rather than a
commit id, because a change id still names the work after a rewrite.*

**After**, both rows point at the anchor that states them:

```text
| `control_dir` | `01-orientation.md#public-surface` | `no-consumer-vocabulary` | …
| `Commit` | `01-orientation.md#public-surface` | `no-transactions` | …
```

Claim 3 is one claim at one anchored subject rather than two: the defect is the
anchor, and both rows exhibit it there. It is below the three-instance threshold
that would force the `book-wide` collapse.

<a id="out-of-charter"></a>
## Out of charter

None. All three claims are in `D1`–`D5`, which is this stage's charter exactly.

<a id="unclaimed-changes"></a>
## Unclaimed changes

One hunk, and it is consequential rather than independent.

- **`07-what-jj-owns.md`, `#the-closed-ledgers`.** The *Early use* paragraph
  counted `Ten rows`, attributed `Three more` to the refusal-constructor chapters,
  and said the manifest declares `only the five whose first use is in chapter 1`.
  Claim 2 adds a row, so those three counts become eleven, four and six. This
  closes no taxonomy defect of its own: before claim 2 the counts were correct, and
  leaving them would have made the closing chapter contradict the ledger it
  closes. It is recorded here rather than claimed because a stage may not score a
  number it made wrong itself.

The claim table and this section together account for every hunk of this stage's
diff. The diff touches four files — `01-orientation.md` (two hunks, claims 1 and
2), `walkthrough.toml` and `source-index.md` (claims 2 and 3, which are the same
edit in the manifest and in the ledger), and `07-what-jj-owns.md` (the hunk above).
Nothing outside `docs/walkthroughs/jj-workspace/` is touched except this record and
`.grove/` bookkeeping.

<a id="findings-not-fixed"></a>
## Findings not fixed

Four. Two are technical-edit classes and are left for `technical-edit-k41`; one is
a conflict inside the structure brief that this stage may not resolve; one is a
class the taxonomy does not carry.

A fifth was drafted and **withdrawn on checking**, and the withdrawal is recorded
because a finding this stage hands forward is work it is asking another session to
do. `06-refusal.md#the-case-analysis` says the enum's order *is the order the
crate's own `lib.rs` introduces the operations*, which looked false against the
file's declaration order — `JJ_OWNED_NAMES`, `Commit`, `Workspace`, then the free
functions. It is not false. The sentence says *operations*, and `lib.rs` introduces
those in the order `resolve`, `root`, `main_repo`, `control_dir`, `is_tracked`,
`commit` — gate, namespace, scope, which is exactly the first six variants' order.
The seam's three come from `jj.rs` and are appended; the commit's one wraps them.

1. **`T2`, `06-refusal.md#the-namespaces-two`.** The page lists
   `validated_namespace`'s four reason strings as *it is empty*, *it is a path
   rather than one directory name*, **it refers to the workspace itself** and
   *Jujutsu owns that name inside `.jj`*. The third is not the source's:
   `crates/jj-workspace/src/lib.rs:333` reads `"it names a directory other than
   itself"`, which is what chapter 4 quotes in the fragment and discusses by name.
   A reader comparing the two pages finds them disagreeing about a string the book
   reconstructs byte for byte on one of them. Not fixed here: a statement about the
   corpus contradicted by the frozen source is `T2`, and correcting it inside a
   developmental diff would credit this stage for the technical edit's work.

2. **`T2`, `04-namespace.md#worked-reservation`.** The worked example is headed
   `**Three refusals, in the order the checks run.**` and then presents
   `control_dir("nested/deeper")`, `control_dir("")` and `control_dir("repo")` — the
   path guard, the empty guard and the jj-owned guard. The checks run in the order
   empty, path, self-reference, owned (`lib.rs:320-341`, and the page's own
   *Four refusals, in the order they run* reads them that way), so the example's
   stated ordering is not the ordering it shows. The order of the three cases is
   also the one thing this section claims about the code that the section below it
   contradicts.

3. **The structure brief's chapter-1 description and its own catalogue rule cannot
   both be satisfied, and the book cannot resolve it.** The brief's *Worked
   examples* section says `No section earlier than a chapter's worked example may
   primarily enumerate three or more public operations, refusal kinds, environment
   variables or validation rules`, and `walkthrough-books.md`'s *Worked examples*
   says the same of `three or more public queries, operations, verbs, error
   variants, or conformance obligations` — the union of which is `D2`'s observable
   test. The brief's *1 · Orientation* description nevertheless requires the chapter
   to *establish the crate's purpose in its own one sentence, the six refusals as
   the book's map, and the public surface: `Workspace`, `Commit`, `Refusal`, and the
   four operations*, and *then* follow the carried operation; and its early-use
   table fixes `01-orientation.md#public-surface` as the First use of `Workspace`
   and `Refusal`, which places that section before the example by construction.
   `01-orientation.md#public-surface` therefore enumerates three types, four
   operations and two accessors ahead of `#commit-tour`, and is a `D2` by the
   general clause and mandated by the specific one. **Not fixed, and deliberately
   not claimed either way**: the brief is the standard this stage is judged against
   and not this stage's to revise, the specific clause is the one the draft
   followed, and the contract's own carve-out — `Vocabulary needed for the trace may
   precede it` — covers most though not all of the section. The residue is
   `is_tracked` and the two accessors, which the trace does not use. Recorded so the
   report can see a conflict between two clauses of one frozen standard rather than
   a stage's silence.

4. **A defect class the taxonomy does not carry: an unnamed exception.**
   `06-refusal.md#the-case-analysis` is headed `Ten kinds, in nearly the order the
   reader met them` and says the file's order is `close to the order this book met
   them: the gate's two, the namespace's two, scope's two, the seam's three, and
   the commit's one`. The reader met the seam's three in chapter 3, before the
   namespace's in chapter 4 and scope's in chapter 5, so the hedges *nearly* and
   *close to* are carrying a real two-position displacement that the page never
   names. Nothing in `D1`–`D5`, `T1`–`T4` or `C1`–`C5` tests a hedge that is true
   but uncashed: `D1` is about a concept used before its owner establishes it,
   `T2` about a statement contradicted by the source, and this is neither. The
   class it would need is something like *unnamed exception* — a qualifier that
   concedes a divergence without saying where it is. Per the preregistration the
   taxonomy is not extended mid-campaign; this is recorded for the report to list.

None of the four is one of the seven the draft already recorded in
[`0-draft.md`](0-draft.md)'s `## Findings not fixed`, and none is a corpus defect:
findings 1, 2 and 4 are page prose, and finding 3 is a specification conflict.

<a id="scope-and-validation"></a>
## Scope and validation

**One commit**, carrying the four book-file edits above, this record, and the
leaf's `.grove/` bookkeeping. No corpus file, no other stage's record, and nothing
else outside the preregistration's allowlist.

**Final validation, green after the stage:**

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/jj-workspace --final --check all
valid: 4 files, 698 resolved lines, 0 deferred lines, final=true
```

The reordering in claim 3 is not cosmetic to the validator: `check_early_uses`
(`crates/book-validation/src/ledger.rs`) keys every ledger row on
`(chapter number, anchor byte offset, owner slice order, symbols)` and rejects a
table whose rows are not ascending in that key, so moving two rows to an earlier
anchor and inserting a third forced the whole chapter-1 block into a new canonical
order. `F009` was the check that would have caught a wrong one.
