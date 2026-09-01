# technical edit — stage record

Stage 2 of six, and the second scored one. Its charter, from the
[preregistration](../preregistration.md)'s stage table: *whether what the book
says about the code is true, complete for its scope, and not silently stale
against the frozen corpus.* It owns `T1` to `T4` and no other class.

The authority is `crates/jj-workspace/` as frozen, and `crates/jj-workspace/tests/`
as evidence — a test is legitimate reading for checking a claim and is not a root.
Where a claim is about jj rather than about the crate, jj 0.44.0 is the authority
and the measurement is recorded below with the control that makes it evidence.

<a id="provenance"></a>
## Provenance

**Stage:** technical edit (stage 2).
**Change id:** `rtpppznn`.
**Parent change id:** `qrtxukzp` — `developmental-edit-k40: run the developmental
edit over the jj-workspace book`, whose
[`1-developmental-edit.md`](1-developmental-edit.md) fixes the state this stage
starts from.

| | Book-directory digest |
|---|---|
| before | `669eee6fbaa35925c48c1c17a411dd555eb625ae36a492696c1af75ea0da4bde` |
| after | `7b5d431416c7389478fb2ca24ef0ebd2c61a1cfc2bdf5126aeafe60d18da19db` |

The before value is the developmental edit's recorded final digest, re-computed at
the start of this session and unmoved. Digests follow the preregistration's recipe
— `find <dir> -type f \( -name '*.md' -o -name '*.toml' \) | LC_ALL=C sort | xargs
shasum -a 256 | shasum -a 256`.

**Preregistration read by this session:** SHA-256
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e` — the same
value the draft's children and the developmental edit recorded.

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

**The corpus was not touched.** This commit contains no change under `crates/`;
all four roots carry the digests the preregistration froze. The jj version this
stage measured against is **0.44.0**, the version the book measures against.

<a id="what-was-checked"></a>
## What was checked, and came back true

Recorded because a technical edit that reports only what moved has not said what
it read, and the report cannot tell a page that was checked from a page that was
skipped.

- **Every test name the book cites exists.** All twenty-eight `workspace.rs` tests
  and the one `environment.rs` test were enumerated and set-differenced against the
  names the seven pages cite. No citation names a test that is not there, and every
  test the book describes does what the page says it does.
- **Every consumer-source line reference resolves.** `grove-loop/src/lib.rs:81`,
  `grove-loop/src/session_config.rs:194` and `:354`, `grove-llm/src/cli.rs:452`,
  `grove-llm/src/main.rs:1`, and `ordinal-fs-tree/src/plan.rs:255` each name the
  line the citing page says they do. So do the call-site counts: `control_dir` is
  called once (`driver_lease.rs:155`, through a constant), `commit` once
  (`tree_lifecycle.rs:294`), and `is_tracked` twice (`tree_lifecycle.rs:319`,
  `session_config.rs:357`).
- **Every line count and arithmetic claim in the pages agrees with the source.**
  The manifest's 44, `jj.rs`'s 81, `refusal.rs`'s 230; chapter 2's forty-seven and
  forty-five; chapter 3's nineteen and eleven; chapter 4's twenty-seven, nineteen
  and fifty-eight; chapter 5's hundred and thirty-nine, hundred and twenty-nine,
  sixteen, eleven, thirty-three and sixty-three; chapter 6's thirteen, eleven and
  sixteen, and its *four of the ten kinds interpolate a cause they also return from
  `source()`*; chapter 7's `98 + 92 + 81 + 58 + 139 + 230 = 698`, its eleven
  ownership blocks, its eleven early-use rows and its 7 + 4 split, and its
  twenty-nine tests.
- **Five claims about jj were re-measured**, each against the behaviour the page
  asserts. A secondary workspace's `.jj/repo` is a nineteen-byte file reading
  `../../main/.jj/repo`, exactly as chapter 2 says. `jj file list` on a fileset
  matching nothing exits 0, prints nothing on stdout and warns on stderr, exactly
  as chapter 5 says. `jj commit` on such a fileset warns, takes a commit jj labels
  `(empty)`, and exits 0, exactly as chapters 5 and 7 say. `jj git init`'s
  installation URL still answers `301 Moved Permanently` to `docs.jj-vcs.dev`,
  which is chapter 6's recorded defect. And `JJ_EMAIL` does override the configured
  `user.email` — chapter 3's claim — though only with a control: a first reading
  said it did **not**, because a `jj commit` attributes the commit to the author
  fixed when the working-copy commit was *created*, so the variable has to be set
  across the `jj new` as well. The no-`JJ_EMAIL` control returning the configured
  address is what makes the positive reading evidence rather than noise.
- **The ADR's cost figure.** Chapter 2's *roughly four thousand lines* for the Git
  lane is `docs/adr/jj-is-the-only-lane.md`'s own number.

<a id="claims"></a>
## Claims

| # | Class | Anchored subject | Before (verbatim citation) | After (verbatim citation) | What a reader gains |
|---:|---|---|---|---|---|
| 1 | `T2` | `06-refusal.md#the-namespaces-two` | see *Citation 1* | see *Citation 1* | The four reason strings the page attributes to `validated_namespace` are now the four strings `validated_namespace` contains, so the two pages that quote the same guard no longer disagree about a string chapter 4 reconstructs byte for byte. |
| 2 | `T2` | `04-namespace.md#worked-reservation` | see *Citation 2* | see *Citation 2* | The heading *in the order the checks run* is now true of the order shown, and it agrees with *Four refusals, in the order they run* two sections below instead of contradicting it. |
| 3 | `T2` | `book-wide` | see *Citation 3* | see *Citation 3* | `.jj/.gitignore` is stated as what it is — a colocated workspace's third entry — so the carried native tree is drawn the same way on all five pages that draw it, and the one worked example that depends on the entry says which workspace it is standing in. |
| 4 | `T2` | `05-scope-and-commit.md#the-path-algebra` | see *Citation 4* | see *Citation 4* | The sentence names the test that actually reaches the canonicalising fallback, so a reader chasing the coverage claim finds it rather than finding a test the same sentence says takes the other branch. |
| 5 | `T2` | `03-subprocess-seam.md#a-command-a-reader-could-type` | see *Citation 5* | see *Citation 5* | `rendered` is depended on by three refusals rather than two, and the page now says so where a reader is counting them — the third is constructed thirty lines above, in the function the page has already read. |
| 6 | `T1` | `05-scope-and-commit.md#the-path-algebra` | see *Citation 6* | see *Citation 6* | Two argued claims in this section named no test and did not say none exists; both now say so, so a reader can tell the claims this chapter closes with an assertion from the ones it closes by reading. |

Citations are carried below the table. Most are multi-line and several quote fenced
traces or tree diagrams whose pipes and box-drawing characters cannot survive a
table cell.

### Citation 1 — claim 1, `T2`

**Before**, at the anchor:

> `Namespace` carries the name and a `reason: String`, and that `String` is the one
> piece of message text in this file assembled somewhere else — `validated_namespace`
> in `lib.rs` supplies *it is empty*, *it is a path rather than one directory name*,
> *it refers to the workspace itself* or *Jujutsu owns that name inside `.jj`*.

The third is not the source's. `crates/jj-workspace/src/lib.rs:333` reads
`"it names a directory other than itself"`, which is the string chapter 4
reconstructs in `«namespace-validation-self-reference»` and discusses by name.
Confirmed by running the crate: `control_dir(".")` and `control_dir("..")` both
print *cannot reserve the control namespace `…`: it names a directory other than
itself*.

**After**, the same sentence with the source's string:

> … `validated_namespace` in `lib.rs` supplies *it is empty*, *it is a path rather
> than one directory name*, *it names a directory other than itself* or *Jujutsu
> owns that name inside `.jj`*.

Handed forward by [`1-developmental-edit.md`](1-developmental-edit.md)'s
`## Findings not fixed`, finding 1.

### Citation 2 — claim 2, `T2`

**Before**, the heading and the order of the three cases beneath it:

> **Three refusals, in the order the checks run.** All three were observed against
> this crate on jj 0.44.0 rather than read off the source; the messages below are
> what a consumer prints.

followed by `control_dir("nested/deeper")`, then `control_dir("")`, then
`control_dir("repo")` — the path guard, the empty guard, the jj-owned guard. The
guards run empty, path, self-reference, owned (`lib.rs:320-341`), which is the
order the section below reads them in as *Four refusals, in the order they run*.

**After**, the heading kept and the cases reordered to match it, with the skipped
guard named rather than left as a gap:

> **Three refusals, in the order the checks run.** All three were observed against
> this crate on jj 0.44.0 rather than read off the source; the messages below are
> what a consumer prints. The empty name is refused first, the path second and the
> jj-owned name last, which is the order *Four refusals, in the order they run*
> below reads them in — the fourth guard, the one that catches `.` and `..`, sits
> between the second and the third and is not one of these three.

followed by `control_dir("")`, `control_dir("nested/deeper")`, `control_dir("repo")`.

Reordering rather than weakening the heading is the fix that leaves the two
sections agreeing; the structure brief fixes the *set* of three cases and not their
order, so nothing in the standard is disturbed. Handed forward by the
developmental edit's finding 2.

### Citation 3 — claim 3, `T2`, `book-wide`

**The defect.** The book presents `.jj/.gitignore` as an unconditional entry of a
jj workspace and then draws it inside a tree it has declared **native**. Measured
on jj 0.44.0:

```text
jj git init --config git.colocate=false   ->  .jj/repo  .jj/working_copy
jj git init --config git.colocate=true    ->  .jj/repo  .jj/working_copy  .jj/.gitignore
jj git init  (stock config)               ->  .jj/repo  .jj/working_copy  .jj/.gitignore
jj config get git.colocate  (stock)       ->  true
```

The two-entry shape is not exotic: it is what `crates/jj-workspace/tests/workspace.rs`'s
`native` fixture builds, for the reason its own comment gives, and it is what this
repository's `.jj/` holds. Driven through the crate from a scratch binary outside
the repository, the consequence is the one the pages turn on:
`control_dir(".gitignore")` returns the `ControlDir` refusal the book quotes in a
colocated workspace, and returns `Ok` — creating the directory — in a native one.

**Six instances, one family.** The correcting sentence is one and names no subject:
*say that `.jj/.gitignore` is a colocated workspace's entry, and do not draw it in
the carried native tree.* Above the three-instance threshold, so it is recorded as
one `book-wide` claim per *Part 1*.

1. `04-namespace.md#the-premise` — **before**: *A workspace created by `jj git init`
   and one created by `jj git init --colocate` both contain exactly three entries*,
   over a `.jj/` listing of `repo/`, `working_copy/` and `.gitignore`. True of both
   *invocations* on stock config and false of the tree the chapter goes on to use.
   **After**: two shapes, the three-entry one labelled colocated, the two-entry one
   stated, and `git.colocate`'s default named as what makes both listed invocations
   colocate.
2. `04-namespace.md#worked-reservation`, the tree after the first call — **before**
   `│   ├── working_copy/` / `│   ├── .gitignore` / `│   └── grove/`; **after** the
   `.gitignore` line removed.
3. `04-namespace.md#worked-reservation`, the fourth case — **before**: *`.gitignore`
   is a plain directory name … so `validated_namespace` accepts it, and the refusal
   comes from the filesystem instead*, in `/work/atlas`. **After**: the case is
   staged in a colocated `/work/atlas-git` and the native outcome is stated —
   *the same call there succeeds and hands the caller a directory called
   `.jj/.gitignore`* — which is the worse half of the same defect rather than its
   absence.
4. `05-scope-and-commit.md#worked-commit` and `06-refusal.md#worked-refusal`, the
   carried tree — **before** both carry `│   ├── .gitignore` between `working_copy/`
   and `grove/`; **after** both drop it, so all five pages that draw `/work/atlas`
   draw it identically. Chapters 1 and 2 already drew the two-entry shape.
5. `04-namespace.md#what-this-chapter-settled` — **before** *the list is one name
   short of jj's actual contents*; **after** *one name short of what a colocated
   workspace actually contains*.
6. `07-what-jj-owns.md#the-uncomfortable-one` and the row-4 cell of
   `#applying-the-test` — **before** *On jj 0.44.0, both `jj git init` and
   `jj git init --colocate` create `.jj/.gitignore`* and *on jj 0.44.0 it is one
   name short*; **after** the colocation condition is named, `git.colocate`'s
   default is given as why the common case still hits it, and the native case is
   added as the worse outcome. The abdication verdict is unchanged and better
   evidenced: the collision is real in the default shape, and the non-colocated
   shape is where the crate hands out a directory sitting on jj's name.

### Citation 4 — claim 4, `T2`

**Before**, at the anchor:

> The crate's own suite reaches this fallback only through
> `a_deletion_is_committable_after_the_path_is_gone`, which passes a *relative* path
> and therefore takes the textual branch; the symlink-and-deletion combination above
> is covered by measurement here and by no test.

The sentence names one test as the only one that reaches the fallback and then says
that test does not reach it. It is also wrong about which test does:
`a_path_outside_the_workspace_is_refused_rather_than_answered` hands `is_tracked` an
absolute path in the temporary directory above the root, so `strip_prefix` fails,
the parent is canonicalised, and the canonical parent still does not strip — the
third `Refusal::outside_workspace` of the three the page discusses.

**After**:

> The crate's own suite reaches this fallback in one place and by its refusing end:
> `a_path_outside_the_workspace_is_refused_rather_than_answered` hands the workspace
> an absolute path in the temporary directory *above* the root, so `strip_prefix`
> fails, the parent is canonicalised, and the canonical parent is still not under
> the root — the third `outside_workspace` of the three. Its succeeding end is
> untested. `a_deletion_is_committable_after_the_path_is_gone`, the test that covers
> a path that no longer exists, passes a *relative* path and therefore never leaves
> the textual branch, so nothing in the suite exercises a canonicalised parent that
> does strip; the symlink-and-deletion combination above is covered by measurement
> here and by no test.

### Citation 5 — claim 5, `T2`

**Before**, opening the section on `rendered`:

> The last function in the file is the one both refusals depend on. Its doc comment
> makes a claim about safety, and the claim is worth more scrutiny than its eleven
> lines suggest.

Three refusals depend on it. `raw_output` renders once and hands the string to
`Refusal::not_runnable` and `Refusal::command_failed` (`jj.rs:52`, `60`, `63`), and
`output` renders again for `Refusal::output_not_text` (`jj.rs:42`) — which is the
same file's third refusal, and the one the page's own closing section counts when
it says *three refusals — one for a jj that would not start, one for a jj that
declined, and one for output that is not text*.

**After**:

> The last function in the file is the one every refusal this file constructs
> depends on: both endings above quote it, and so does the UTF-8 check in `output`
> two sections back.

### Citation 6 — claim 6, `T1`

The structure brief's *What each chapter's prose owes* requires an argued claim to
name the jj behaviour it rests on, **the test that proves it**, and the alternative
rejected. Two argued claims in this section name a behaviour and an alternative and
no test, and — unlike the neighbouring passages, which say *no test in the suite
constructs a non-UTF-8 path* and *neither is exercised by the crate's suite* — do
not say that none exists.

**Before**, the fileset-quoting argument, ending:

> One pass over the characters cannot have an order bug, and this is a crate with
> no dependencies to borrow an escaper from.

and the root-scope guard, ending:

> The reason string is written for the condition rather than for the caller's
> mistake: *the workspace root is not a scope inside itself*.

No test in the suite passes a path containing `"` or `\`, and no test names the
workspace root as a scope; both were checked by enumerating every path argument the
twenty-eight interface tests pass.

**After**, a paragraph added to the first:

> **No test in the suite passes a path containing either character**, so the
> argument above rests on jj's documented literal syntax and on reading the loop,
> and not on anything that goes red. It is stated rather than left to be assumed,
> because it is the one place in this chapter where a claim about correctness has no
> assertion behind it: every path the twenty-eight interface tests name is ordinary.

and a clause added to the second:

> This guard is unasserted too: no test names the root as a scope, so the second
> route is argued from the code and closed by no assertion, while the first route —
> the empty slice — is held by `a_commit_with_no_paths_is_refused_rather_than_widened`.

Claims 4 and 6 share an anchored subject and differ in class, which *Part 1*
permits; they are not the same defect, and neither correction implies the other.

<a id="out-of-charter"></a>
## Out of charter

None. All six claims are in `T1`–`T4`, which is this stage's charter exactly.

<a id="unclaimed-changes"></a>
## Unclaimed changes

One hunk, reader-facing and deliberately not claimed, because the class the
taxonomy would need for it does not exist.

- **`06-refusal.md#the-opaque-type` and `#what-this-chapter-settled`: `grove-loop`
  does not match on `ordinal_fs_tree::Refusal`.** The page's rejected-alternative
  argument said *`ordinal_fs_tree::Refusal` … is a public enum with named variants
  carrying domain values, and `grove-loop` matches on it*, and the closing section
  said *a public enum grove matches on*. An enumerate-then-classify sweep of every
  `Refusal::` occurrence outside `crates/ordinal-fs-tree/` and
  `crates/jj-workspace/` returns nine lines: seven doc comments, one ordinary
  comment in a test module, and one test's assertion message. None is code. The
  decisive form of the same finding is shorter — **no crate outside
  `ordinal-fs-tree` imports `ordinal_fs_tree::Refusal` at all**; every
  `use ordinal_fs_tree::{…}` in `grove-loop`, `grove-llm` and `grove` names other
  types, so there is no value of that type anywhere to match on. The positive
  control — the same code-not-comment filter inside `ordinal-fs-tree/src` — returns
  the library's own construction sites, and a deliberately mutated variant name
  returns none, so the clean read is not a broken instrument. A first attempt at a
  control failed and is recorded because it is the reason the claim is stated this
  way: a filter for match-arm positions returned zero *inside* `ordinal-fs-tree`
  too, which is a filter that cannot fail rather than a tree with no matches.
  `docs/ARCHITECTURE.md`'s *How an `ordinal-fs-tree` refusal reaches an operator*
  says the same thing from the other side: Grove resolves and classifies its target
  **before** calling the library, which is why the refusals that reach an operator
  reach them unaltered rather than branched on. The passage now says that Grove
  *names* two variants, in doc comments and a test message, and why; and both
  passages now rest the contrast on the variants distinguishing cases a caller could
  act on rather than on a `match` that is not there.

  **Not claimed**, because its subject is `crates/grove-loop` and `docs/ARCHITECTURE.md`.
  `T2` tests *a statement about the corpus, about jj, or about a test*, and the
  preregistration's *Frozen inputs* fixes the corpus as the four
  `crates/jj-workspace/` roots; a false statement about a **sibling crate's** source
  is none of the three. The class it would need is something like *false claim about
  a cited non-corpus artifact* — every book in this campaign cites consumer source
  by path and line, so it is not peculiar to this page. Per the preregistration the
  taxonomy is not extended mid-campaign; this is recorded for the report to list, in
  the same shape the developmental edit used for *unnamed exception*.

The claim table and this section together account for every prose hunk of this
stage's diff. The diff touches five book files — `03-subprocess-seam.md` (claim 5),
`04-namespace.md` (claims 2 and 3), `05-scope-and-commit.md` (claims 3, 4 and 6),
`06-refusal.md` (claims 1 and 3, and the hunk above), and `07-what-jj-owns.md`
(claim 3) — plus this record and `.grove/` bookkeeping. `01-orientation.md`,
`02-the-gate.md`, `README.md`, `concept-index.md`, `source-index.md` and
`walkthrough.toml` are untouched: nothing in this stage moved a fragment boundary,
a ledger row or an ownership range, which is why no manifest edit was needed.

<a id="findings-not-fixed"></a>
## Findings not fixed

Three. One is a claim this stage could not adjudicate, one belongs to a later
stage, and one is a defect in the crate that the corpus freeze forbids fixing here.

1. **An unverifiable number: `07-what-jj-owns.md#final-verification`.** *its public
   surface is fourteen lines.* The counting rule is not stated and no natural one
   produces fourteen: lines containing `pub ` across `lib.rs` and `refusal.rs` give
   eleven, adding the two structs' closing braces gives thirteen, and counting
   whole public declarations gives eleven again. It is not a claim about the corpus
   contradicted by the corpus — nothing in the source contradicts it, because
   nothing in the source can be compared with it — so it is not `T2`, and it is not
   an argued claim missing one of the brief's three elements, so it is not `T1`. It
   is a number a reader cannot check, which is a real defect with no class in the
   frozen taxonomy. Making it checkable is a sentence-level rewrite and is left
   where a stage owning sentences can take it.

2. **`02-the-gate.md#worked-resolution` over-attributes half of one claim to one
   test.** *The walk is the filesystem's, so no environment variable and no shared
   repository store can redirect it, which is asserted directly by
   `resolution_ignores_repository_selection_and_temporary_directory_environment`.*
   The test sets `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR` and `TMPDIR`; nothing
   in it constructs a shared repository store, and the secondary-workspace case that
   would is a different test. The two-clause sentence is the crate's own doc comment
   restated, so the source half is exact and only the attribution over-reaches. Left
   unfixed deliberately: chapter 3 already states the same test's scope precisely
   twice — *it sets three of the four variables and not `GIT_INDEX_FILE`*, and *even
   the mechanism is proved for three names rather than four* — so a reader who
   reaches the seam is corrected, and the repair here is a qualifier rather than a
   correction. Recorded so the proof stage can decide whether one page's precision
   should reach the other.

3. **A defect in the crate, not in the book: the `.gitignore` collision is worse in
   a native workspace than the pages describe.** The book's account — a `ControlDir`
   refusal naming the wrong remedy — is the colocated outcome. In a workspace that
   is not colocated, `control_dir(".gitignore")` **succeeds** and hands the consumer
   a directory on the exact name jj will write a file to the moment that tree is
   colocated, which turns a bad message into a latent collision. The pages now say
   so. The fix is `jj-owned-names-k65`'s, already scheduled and already placed after
   every crate book, and the corpus freeze forbids this session from touching
   `crates/jj-workspace/` for it. No new leaf: the existing one covers it, and the
   finding sharpens what it has to fix rather than adding work.

None of the three is one of the seven the draft recorded in
[`0-draft.md`](0-draft.md)'s `## Findings not fixed`, and none of the four the
developmental edit recorded — its findings 1 and 2 are claims 1 and 2 above, its
finding 3 is a specification conflict this stage has no standing to resolve, and its
finding 4 is a copy-edit-shaped hedge.

<a id="scope-and-validation"></a>
## Scope and validation

**One commit**, carrying the five book-file edits above, this record, and the
leaf's `.grove/` bookkeeping. No corpus file, no other stage's record, and nothing
else outside the preregistration's allowlist.

**Final validation, green after the stage:**

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/jj-workspace --final --check all
valid: 4 files, 698 resolved lines, 0 deferred lines, final=true
```

No edit in this stage moved a fragment boundary or a source range, so the
reconstruction the draft proved is byte-identical afterwards: every change is prose
between fragments, or a line inside a `text` trace that reconstructs nothing.
