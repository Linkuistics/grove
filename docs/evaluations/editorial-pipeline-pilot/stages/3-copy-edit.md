# copy edit — stage record

Stage 3 of six, and the third scored one. Its charter, from the
[preregistration](../preregistration.md)'s stage table: *sentences, terminology,
consistency with the book contract's prose rules, and uniformity with the
`ordinal-fs-tree` precedent. Does not restructure.* It owns `C1` to `C5` and no
other class.

The standards are `docs/specs/walkthrough-books.md`'s
[*Prose contract*](../../../specs/walkthrough-books.md#prose-contract), the
structure brief `docs/specs/jj-workspace-book-structure.md`, `CONTEXT.md` as the
glossary a grove term is linked to rather than re-taught, `docs/USAGE.md` as the
guide whose vocabulary the book's own must not contradict, and the relocated
`ordinal-fs-tree` book as the uniformity precedent. Nothing was restructured: no
chapter, section, worked example, fragment or ledger row moved, and every edit
below is a sentence, a term, a link or a line fill.

<a id="provenance"></a>
## Provenance

**Stage:** copy edit (stage 3).
**Change id:** `yxktpovq`.
**Parent change id:** `rtpppznn` — `technical-edit-k41: run the technical edit
over the jj-workspace book`, whose
[`2-technical-edit.md`](2-technical-edit.md) fixes the state this stage starts
from.

| | Book-directory digest |
|---|---|
| before | `7b5d431416c7389478fb2ca24ef0ebd2c61a1cfc2bdf5126aeafe60d18da19db` |
| after | `ca79d04d3f691bc3c48717e33b0469bc41da5d76996852d92586a1b169d8ee52` |

The before value is the technical edit's recorded final digest, re-computed at
the start of this session and unmoved. Digests follow the preregistration's
recipe — `find <dir> -type f \( -name '*.md' -o -name '*.toml' \) | LC_ALL=C sort
| xargs shasum -a 256 | shasum -a 256`.

**Preregistration read by this session:** SHA-256
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e` — the same
value the draft's children, the developmental edit and the technical edit
recorded.

<a id="frozen-inputs"></a>
## Frozen inputs, re-checked

Re-computed at the start of this session, before any edit. All nine match the
values the technical edit recorded.

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

**The corpus was not touched.** This commit contains no change under `crates/`.
Neither `CONTEXT.md` nor `docs/USAGE.md` was edited: claim 2 below adds citations
of glossary anchors that already exist, and both are declared in the book's own
`walkthrough.toml` `[[glossary]]` block.

<a id="what-was-checked"></a>
## What was checked, and came back clean

A copy edit that reports only what it changed has not said what it read, and the
five classes it owns are not equally represented below. Each sweep is recorded
with its result so the report can tell a class that was checked from one that was
skipped.

- **Every prose sentence of all ten pages was read**, in reading order, against
  the contract's *Direct declarative style*, *Local context and repetition*,
  *Source-fragment introductions* and *Audience* rules.
- **`C2`, rhetorical questions and narrative suspense.** Every `?` outside a
  fenced block was enumerated: two occurrences, both in prose, and neither is a
  rhetorical question — `07-what-jj-owns.md#the-test`'s *if the tool's guarantee
  changed tomorrow, what here would go red?* is the stated form of the test the
  chapter is about, and `06-refusal.md#scopes-two`'s *did you mean to pass the
  files you changed?* is a quoted example of a message the crate does **not**
  write. Both are kept. Six passages that flag a judgement in advance — *the most
  carefully argued paragraph in the crate*, *worth more scrutiny than its eleven
  lines suggest*, *load-bearing in a way a reader who skims it will miss* — were
  considered as narrative suspense and not claimed: each states what the reader is
  about to be shown rather than withholding it, which is the distinction the rule
  draws.
- **`C1`, jj's own vocabulary.** *working copy*, *working tree*, *repository*,
  *operation log*, *change id*, *commit id*, *revision*, *snapshot*, *fileset*,
  *template*, *default workspace*, *secondary workspace* and *colocated* were each
  checked against jj's glossary and against the book's own earlier use. One
  drifted — *native*, claim 3. The `change id` / `change ID` split is jj's own
  spelling inside a quotation and the book's outside it, which is correct rather
  than inconsistent.
- **`C3`, failure categories.** Every passage that distinguishes a modelled
  refusal, an environmental failure, a rollback failure and an implementation
  defect was located and read: chapter 2 on `not_a_workspace` against
  `unresolvable_path`, chapter 3 on failure to start against failure to succeed,
  chapter 5 on `CommitNotRecorded` against the unwrapped identity read, chapter 6
  on the `.gitignore` remedy and the stale installation URL, and chapter 7's
  verdict table. All four categories are labelled where the text separates them.
  **No `C3` defect was found**, and the class is reported empty rather than
  omitted.
- **`C4`, links.** All eighty-one Markdown links were enumerated and each label
  read against its destination. Every label names its destination; no paragraph
  loses its claim when its link is removed. The weakest is
  `[anyhow](https://docs.rs/anyhow/1.0.102/anyhow/struct.Error.html)` in
  `06-refusal.md#the-premise`, whose label names the crate rather than the struct
  page — the sentence around it supplies the purpose, so it is recorded here and
  not claimed.
- **`C5`, fragment introductions.** All seventy-eight literal fragments were
  checked: for each, the nearest preceding nonblank block was extracted and read
  against the contract's five editorial questions. Seventy-seven answer them.
  The one that does not is claim 6.
- **Mechanical sweeps.** Doubled words and article agreement across all ten pages:
  one hit, recorded under *Unclaimed changes*. Chapter-reference capitalisation
  (`Chapter 4` against `chapter 4`): twenty-four occurrences, every capitalised
  one sentence-initial, so there is no defect. Line fill: three prose lines exceeded
  the book's own fill by more than twenty columns, all three left by the previous
  stage, and are rewrapped under *Unclaimed changes*.

<a id="claims"></a>
## Claims

| # | Class | Anchored subject | Before (verbatim citation) | After (verbatim citation) | What a reader gains |
|---:|---|---|---|---|---|
| 1 | `C1` | `book-wide` | see *Citation 1* | see *Citation 1* | The book's one consumer is spelled the same way everywhere, so a reader no longer meets *Grove's loop driver* on one page and *grove's loop driver* on another and has to decide whether they are the same thing. |
| 2 | `C1` | `01-orientation.md#commit-tour` | see *Citation 2* | see *Citation 2* | The three grove terms the book's first consumer's-half block uses are links into the glossary rather than a definition written here, which is the contract's rule and what the other four blocks already do. |
| 3 | `C1` | `02-the-gate.md#worked-resolution` | see *Citation 3* | see *Citation 3* | *Native* names the same layout on every page it appears on. A reader who took chapter 1's gloss could not follow chapter 4's `.gitignore` argument, which turns on native meaning *not colocated*. |
| 4 | `C2` | `book-wide` | see *Citation 4* | see *Citation 4* | Five figures of speech are replaced by the mechanism each stood for, so a claim that was carried by an idiom is now carried by something a reader can check. |
| 5 | `C2` | `07-what-jj-owns.md#final-verification` | see *Citation 5* | see *Citation 5* | The reason the crate has no unit tests rests on a measure of the public surface a reader can count, rather than on a line count with no stated counting rule. |
| 6 | `C5` | `04-namespace.md#the-reservation` | see *Citation 6* | see *Citation 6* | The introducing paragraph of `«namespace-control-dir-body»` now answers the contract's five questions, so the fragment arrives with its actor, its input, its output, its invariant and its place in the worked example stated. |

Citations are carried below the table; most are multi-line.

### Citation 1 — claim 1, `C1`, `book-wide`

**The defect.** The book writes its one consumer both as `Grove` and as `grove`,
for the same referent and sometimes in the same construction. The clearest pair is
one phrase spelled two ways: `README.md` has *grove's loop driver and its CLI*
and `01-orientation.md#what-it-declines` has *Grove's loop driver reserves a
control namespace through it*. Seven of the eleven marked consumer's-half blocks open
with the consumer's name: four capitalise it and three do not —
`05-scope-and-commit.md#the-commit` has *grove calls `Workspace::commit` at
exactly one site* against `01-orientation.md#what-it-declines`'s *Grove calls
`control_dir("grove")` once per loop*.

**The direction, and why it is not a preference.** The book's own structure brief
— the standard this book is authored to — spells the consumer lowercase
throughout: *The book says what grove does*, *Naming grove in every chapter risks
blurring the boundary*, *inside grove's cut*, *the grove term for what a
path-scoped commit is for*. The book's own majority does too, at more than twenty
occurrences against six. Lowercase is also what keeps the instance sense readable:
*a grove*, *a grove session*, *a grove user* are already lowercase everywhere and
are unaffected.

**Six instances, one family.** The correcting sentence is one and names no
subject: *spell the consumer `grove`.* Above the three-instance threshold, so it
is recorded as one `book-wide` claim per *Part 1*.

1. `README.md#contents` — **before** *Grove's own vocabulary is linked to the
   glossary rather than re-taught*; **after** *grove's own vocabulary …*.
2. `01-orientation.md#what-it-declines` — **before** *Grove's loop driver reserves
   a control namespace through it and grove's task sessions commit through it*
   (both spellings in one sentence); **after** *grove's loop driver …*.
3. `01-orientation.md#what-it-declines`, the specimen consumer's-half block —
   **before** ***The consumer's half.** Grove calls `control_dir("grove")` once per
   loop*; **after** *grove calls …*.
4. `02-the-gate.md#one-lane` — **before** ***The consumer's half.** Grove resolves
   the [stated VCS]*; **after** *grove resolves …*.
5. `03-subprocess-seam.md#worked-invocation` — **before** ***The consumer's
   half.** Grove runs its sessions under a harness*; **after** *grove runs …*.
6. `07-what-jj-owns.md#applying-the-test` — **before** ***The consumer's half.**
   Grove commits at a [task commit boundary]*; **after** *grove commits at …*.

`enum Namespace { Grove, … }` in `04-namespace.md#no-consumer-vocabulary` is a
Rust identifier in an illustrative type and is deliberately untouched.

### Citation 2 — claim 2, `C1`

The contract's *Audience* rule: grove's vocabulary is **linked to the glossary,
never re-taught**. The book's first consumer's-half block does the opposite — it
defines one glossary term inline and names two more with no link, while the blocks
in chapters 4, 5 and 7 link all three.

**Before**, at the anchor:

> **The consumer's half.** The commit above is a task commit boundary: one
> session's whole task, sealed in one commit named by its work-item handle. The
> reserved directory `.jj/grove` holds the driver lease and the loop control
> channel. None of those three words exists in this crate; all three are what its
> one consumer does with what it returns.

*one session's whole task, sealed in one commit named by its work-item handle* is
[`CONTEXT.md`](../../../../CONTEXT.md#task-commit-boundary)'s definition restated on
the page, and the chapter is the one place in the book where a reader meets these
terms first.

**After**, the same three terms as citations — quoted as source, so the relative
paths are the book page's own rather than this record's:

```markdown
> **The consumer's half.** The commit above is a
> [task commit boundary](../../../CONTEXT.md#task-commit-boundary), and the
> reserved directory `.jj/grove` holds the
> [driver lease](../../../CONTEXT.md#driver-lease) and the
> [loop control channel](../../../CONTEXT.md#loop-control-channel). None of those
> three terms exists in this crate; all three are what its one consumer does with
> what it returns.
```

All three anchors are declared in this book's `walkthrough.toml` `[[glossary]]`
block and exist in `CONTEXT.md` as explicit `<a id="…"></a>` lines, so the
citations are inside the manifest's declared set rather than new outbound links.
*words* becomes *terms* because two of the three are not one word.

### Citation 3 — claim 3, `C1`

**The defect.** *Native* is glossed against one distinction and used against
another. The guide the book cites for workspace layouts partitions three ways —
*Native (`jj git init`), colocated (`jj git init --colocate`, where `.git` and
`.jj` sit side by side), and secondary workspaces created with `jj workspace add`*
([`docs/USAGE.md#usage-workspace-layouts`](../../../USAGE.md#usage-workspace-layouts),
which `README.md` names as this crate's entry point) — and chapters 4, 6 and 7 use
*native* in exactly that sense: *A workspace that is not colocated holds the first
two and no third*, *`/work/atlas` is native and has no `.jj/.gitignore`*, *In a
workspace that is not colocated the same call succeeds*. Chapters 1 and 2 gloss it
as the opposite of *secondary* instead, which is a different axis: a colocated
workspace also holds its own repository.

**Before**, `02-the-gate.md#worked-resolution`:

> It is a native workspace at `/work/atlas` holding its own repository, with a
> caller four directories down:

and `01-orientation.md#commit-tour`, where the term is introduced:

> The starting tree is a native jj workspace — one that holds its own repository
> rather than borrowing another's — with a grove task tree inside it:

**After**, both naming the layout rather than one of its two properties:

> It is a native workspace at `/work/atlas` — not colocated with Git, and holding
> its own repository — with a caller four directories down:

> The starting tree is a native jj workspace — one that is not colocated with Git
> and holds its own repository rather than borrowing another's — with a grove task
> tree inside it:

The correction is one sentence — *a native workspace is one that is neither
colocated nor secondary* — applied at the two subjects that carry the gloss. Two
instances, below the three-instance threshold, so it stays one claim anchored at
the second rather than becoming a `book-wide` one; the chapter 1 instance is part
of the same correction and is not recorded separately.

### Citation 4 — claim 4, `C2`, `book-wide`

The contract's *Direct declarative style*: prose does not use metaphors, idioms,
emotive language or persuasive framing, and identifies the actor, input,
behaviour, result and invariant where those distinctions matter. Five passages
carry a claim on a figure of speech instead.

**Five instances, one family.** The correcting sentence is one and names no
subject: *replace the figure of speech with the mechanism it stands for.* Above
the three-instance threshold, so it is recorded as one `book-wide` claim.

1. `01-orientation.md#crate-thesis` — **before** *… straight into a namespace jj
   owns and may extend, which is a collision waiting for a jj release*; **after**
   *… which is a collision the next jj release can create*. The idiom is *an
   accident waiting to happen*; the replacement names what would have to happen.
2. `03-subprocess-seam.md#the-file-and-its-claims` — **before** *in a colocated
   tree it points at a repository a Git-aware child will happily use*; **after**
   *… a Git-aware child will use in place of the intended one*. The
   anthropomorphism goes and the actual hazard — the wrong repository, not an
   eager child — is stated.
3. `04-namespace.md#no-consumer-vocabulary` — **before** *The `&str` costs a
   validation function, which is the last section of this chapter. The enum would
   have cost the spine.* **After** *… The enum would have cost the fourth refusal
   itself: a crate holding the list of who may coordinate through a jj workspace
   has a vocabulary for its consumers.* *The spine* is a metaphor the book never
   defines — the phrase appears in the concept index as a label and nowhere in the
   prose — so the sentence's whole content was carried by an undefined figure.
4. `06-refusal.md#the-gates-two` — **before** *a gate that says nothing about its
   own effects invites the reader to check by hand*; **after** *… leaves the
   reader to check the tree by hand*. The personification goes and what would be
   checked is named.
5. `07-what-jj-owns.md#the-uncomfortable-one` — **before** *The guarantee holds by
   luck, in the sense that the collision is refused*; **after** *The collision is
   refused, but by `create_dir_all` failing on an entry that already exists rather
   than by the check meant to catch it*. This is the one of the five where the
   idiom was also doing analytic work: *by luck* was the chapter's verdict on the
   crate's fourth refusal, and the mechanism it stands for — a filesystem error
   standing in for a validation the list did not perform — is the evidence for
   that verdict.

### Citation 5 — claim 5, `C2`

Handed forward by [`2-technical-edit.md`](2-technical-edit.md)'s
`## Findings not fixed`, finding 1: a number a reader cannot check, which that
stage placed with the stage that owns sentences.

**Before**, at the anchor:

> Twenty-nine tests, all of them integration tests: the crate carries no unit
> tests and no documentation tests, because its public surface is fourteen lines
> and every claim worth making about it is a claim about a real workspace on disk.

The counting rule is not stated and no natural one produces fourteen: lines
carrying `pub ` outside `pub(crate)` across `lib.rs` and `refusal.rs` give eleven,
adding the two structs' closing braces gives thirteen, and whole public
declarations give eleven again. The sentence rests a design claim — why there are
no unit tests — on a measure the reader is invited to check and cannot.

**After**, the same claim on the measure chapter 1 already gave — quoted as
source, so the link is the book page's own:

```markdown
Twenty-nine tests, all of them integration tests: the crate carries no unit
tests and no documentation tests, because its public surface is the three types
and six functions [*Orientation*](01-orientation.md#public-surface) listed —
`Workspace`, `Commit` and `Refusal`, with `resolve` the only constructor — and
every claim worth making about it is a claim about a real workspace on disk.
```

Countable in the source (`Workspace`, `Commit`, `Refusal`; `resolve`, `root`,
`main_repo`, `control_dir`, `is_tracked`, `commit`) and identical to
`01-orientation.md#public-surface`'s *three public types* and *four operations …
plus two accessors*, which is where the claim's `C1` half sits: the book measured
its own public surface two ways.

### Citation 6 — claim 6, `C5`

The contract's *Source-fragment introductions*: the introducing paragraph answers
five editorial questions — why the fragment is present here, which actor owns the
behaviour, what relevant input becomes what output, which invariant it establishes
or uses, and what role it has in the page's current example. Seventy-seven of the
book's seventy-eight literal fragments answer them. This one answers the first.

**Before**, the whole introducing block of `«namespace-control-dir-body»`:

> The body is four lines, and their order is the whole of what they do.

Fifteen words, and four of the five questions unanswered: the actor is not named,
no input or output is stated, the invariant the ordering establishes is not given,
and the fragment is not tied to the worked reservation the same page traced. It is
the shortest fragment introduction in the book; the next shortest,
`«subprocess-raw-output-endings»`, states its input–output pair — the child's two
ways of going wrong becoming two refusals — in eighteen.

**After**:

> The body is four lines, and their order is the whole of what they do:
> `control_dir` turns the caller's namespace string into a path under the
> workspace's `.jj/` that exists by the time it is returned, validating before it
> joins and joining before it creates, which is what keeps a refused name from
> reaching the filesystem at all. It is the first call of the worked example
> above, at the resolution the trace showed.

The analysis below the fragment is untouched: it reads the shadowing, the `join`,
`create_dir_all`'s idempotence and the *never shared* clause, and none of that is
restated here.

<a id="out-of-charter"></a>
## Out of charter

None. All six claims are in `C1`–`C5`, which is this stage's charter exactly.

<a id="unclaimed-changes"></a>
## Unclaimed changes

Three, and together with the claim table they account for every prose hunk in
this stage's diff.

- **`04-namespace.md#the-reserved-list`: an article.** *only the second is
  achievable without a interface the foreign system does not offer* → *without an
  interface*. Mechanical, and no taxonomy class covers agreement: `C2` tests style
  and unnamed distinctions, not grammar. Found by an enumerate-then-classify sweep
  over every `a` + vowel-initial word in all ten pages, which returned this and no
  other.

- **`06-refusal.md#the-case-analysis`: the hedge is cashed.** The section is headed
  *Ten kinds, in nearly the order the reader met them* and says the file's order is
  *close to the order this book met them*, without ever saying where the divergence
  is. A sentence now names it — the seam's three, met in chapter 3 and listed
  fourth of the five groups, because the enum follows `lib.rs`'s operations and the
  seam is the file those operations call rather than one of them. **Reader-facing
  and deliberately not claimed**: this is
  [`1-developmental-edit.md`](1-developmental-edit.md)'s finding 4, which that
  stage recorded precisely because the taxonomy carries no class for *a qualifier
  that concedes a divergence without saying where it is* — `D1` is about a concept
  used before its owner establishes it, `T2` about a statement the source
  contradicts, and `C2` about a figure of speech or an unnamed actor, input,
  behaviour, result or invariant. A true hedge that is never cashed is none of
  those. Fixed here because it is one sentence and this stage owns sentences;
  scored as nothing, and recorded for the report to list beside the developmental
  edit's own account of the same gap.

- **Line fill, seven paragraphs.** Three prose lines ran to 102, 104 and 111
  columns against the book's own fill of about 82 — `01-orientation.md#commit-tour`
  (where *Five things in that trace are the whole book* had also been run into the
  end of the preceding paragraph, and is restored to its own),
  `03-subprocess-seam.md#nothing-ambient` and
  `04-namespace.md#no-consumer-vocabulary`. All three were left by the previous
  stage. Rewrapping them, and reflowing the paragraphs the six claims above edited
  so that no line exceeds the fill, is formatting with no reader-facing effect: the
  words are those the claim citations quote.

The claim table and this section together account for every prose hunk of this
stage's diff. The diff touches seven book files — `README.md` (claim 1),
`01-orientation.md` (claims 1, 2, 3 and 4, and the rewrap),
`02-the-gate.md` (claims 1 and 3), `03-subprocess-seam.md` (claims 1 and 4, and
the rewrap), `04-namespace.md` (claims 4 and 6, the article, and the rewrap),
`06-refusal.md` (claim 4 and the hedge) and `07-what-jj-owns.md` (claims 1, 4 and
5) — plus this record and the leaf's `.grove/` bookkeeping. `05-scope-and-commit.md`,
`concept-index.md`, `source-index.md` and `walkthrough.toml` are untouched:
nothing in this stage moved a fragment boundary, an ownership range, a ledger row
or a page's anchors, which is why no manifest or index edit was needed and why the
concept index's labels still name the headings they point at.

<a id="findings-not-fixed"></a>
## Findings not fixed

Four. Two belong to other stages, one is a repetition judgement this stage
declined to act on for a stated reason, and one is a class the taxonomy does not
carry.

1. **`C4`, `07-what-jj-owns.md#the-uncomfortable-one`: chapter 4's `.gitignore`
   mechanism is restated nearly in full.** The section repeats the colocated and
   native shapes, `git.colocate`'s default, the path through `validated_namespace`
   into `fs::create_dir_all`, and the `ControlDir`-rather-than-`Namespace` outcome
   — all of which `04-namespace.md#worked-reservation` and `#the-reserved-list`
   establish. The repetition meets one of the contract's five conditions (*the
   current page has a different immediate goal*, here the verdict rather than the
   mechanism), so what is at issue is only whether it exceeds the minimum semantic
   contract. **Not fixed deliberately**: the technical edit's claim 3 added the
   native-case sentences to this exact passage one commit ago, and trimming what
   the previous stage has just landed would be a regression against a verified
   claim rather than a copy edit. Left for proof, whose charter is whole-book and
   which reads the two passages together.

2. **`02-the-gate.md#worked-resolution` still over-attributes half of one claim to
   one test.** Carried forward unchanged from
   [`2-technical-edit.md`](2-technical-edit.md)'s finding 2: *no environment
   variable and no shared repository store can redirect it, which is asserted
   directly by `resolution_ignores_repository_selection_and_temporary_directory_environment`*,
   where the test sets four environment variables and constructs no shared
   repository store. The repair is a qualifier about what a test asserts, which is
   `T1`/`T2` territory and not this stage's to make; a copy edit rewriting a claim
   about test coverage would be crediting itself with the technical edit's class.

3. **`01-orientation.md#public-surface` remains a `D2` by one standard and a
   requirement of the other.** [`1-developmental-edit.md`](1-developmental-edit.md)'s
   finding 3 records the conflict between the structure brief's chapter-1
   description and the two catalogue rules. Nothing in this stage's charter can
   resolve a conflict between two frozen standards, and no sentence-level edit
   closes it.

4. **A class the taxonomy does not carry: a link label that names its destination
   but not its purpose.** `06-refusal.md#the-premise` cites
   `[anyhow](https://docs.rs/anyhow/1.0.102/anyhow/struct.Error.html)`, whose label
   names the crate rather than the page — the `Error` struct's documentation for
   the locked version, cited for its blanket `From` impl. `C4`'s observable test is
   scoped past exactly this: it fires on a label `M201` accepts that *still* does
   not name destination **and** purpose, and the surrounding sentence supplies the
   purpose here, so a stricter reading would make every crate-named link in the
   book a defect. Recorded rather than claimed, and rather than silently
   normalised.

None of the four is one of the seven the draft recorded in
[`0-draft.md`](0-draft.md)'s `## Findings not fixed`. Of the two findings the
technical edit handed forward that were this stage's, one is claim 5 and one is
finding 2 above, which it explicitly left for proof.

<a id="scope-and-validation"></a>
## Scope and validation

**One commit**, carrying the seven book-file edits above, this record, and the
leaf's `.grove/` bookkeeping. No corpus file, no other stage's record, no
`CONTEXT.md`, no `docs/USAGE.md`, and nothing else outside the preregistration's
allowlist.

**Final validation, green after the stage:**

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/jj-workspace --final --check all
valid: 4 files, 698 resolved lines, 0 deferred lines, final=true
```

```console
$ bash scripts/check.sh
...
=== book-check
  book-check docs/walkthroughs/jj-workspace
valid: 4 files, 698 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/ordinal-fs-tree
valid: 17 files, 8720 resolved lines, 0 deferred lines, final=true
  2 book(s) checked, 0 failing
  ✓ book-check

check: all 8 principal checks pass
```

No edit in this stage moved a fragment boundary or a source range, so the
reconstruction the draft proved is byte-identical afterwards: every change is
prose between fragments.
