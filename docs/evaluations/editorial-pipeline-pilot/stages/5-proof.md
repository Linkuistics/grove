# proof — stage record

Stage 5 of six, the fifth and last scored one, and the second of the two stages
in the alternative the six must beat. Its charter, from the
[preregistration](../preregistration.md)'s stage table: *the final read for errors
the earlier stages introduced or missed, and the last check that the book is
whole. Whole-book scope; every class is in charter.*

`P1` — whole-book coherence — is proof's own class and no other stage's charter
contains it. Every other class is also in proof's, so this stage cannot commit a
charter breach and its `## Out of charter` section is empty by construction.

**What this stage did not do.** It did not score anything.
`measurement-report-k45` does the reading; a proof session that started counting
would have adjusted the instrument mid-reading. The claim table below is the
input to that reading, not a verdict on it.

<a id="provenance"></a>
## Provenance

**Stage:** proof (stage 5).
**Change id:** `rlntxrkm`.
**Parent change id:** `kkktwzyx` — `art-k43: run the art stage over the
jj-workspace book`, whose [`4-art.md`](4-art.md) fixes the state this stage starts
from.

| | Book-directory digest |
|---|---|
| before | `7dc08f7e213987e651c740ee522621429e547f25db8a0c38b90cb2b39e688a18` |
| after | `1e024f25a869dfc1faa74800749782c92e0efe1954828e6e9bf61ab7c2dfb267` |

The before value is the art stage's recorded final digest, re-computed at the
start of this session and unmoved — so nothing touched the book between the four
stage commits and this one. Digests follow the preregistration's recipe —
`find <dir> -type f \( -name '*.md' -o -name '*.toml' \) | LC_ALL=C sort | xargs
shasum -a 256 | shasum -a 256`.

**Preregistration read by this session:** SHA-256
`b9ee9f8e48b69170c7ecd9787b9373dfc42242895e91d37d8a181af6f09a694e` — the same
value the draft's children and the four editorial stages before this one
recorded.

<a id="frozen-inputs"></a>
## Frozen inputs, re-checked

Re-computed at the start of this session, before any edit. All nine match the
values the art stage recorded.

| Input | SHA-256 as read | Verdict |
|---|---|---|
| `crates/jj-workspace/Cargo.toml` | `528fe93e62e0807b97a02d451ff04a86f1a1f0bd7f290de711684f3920c70936` | unmoved |
| `crates/jj-workspace/src/lib.rs` | `43ca4ad3947db34a326e5e85051617ab544bf5ee8d7df40fcc229597f7c1c9e3` | unmoved |
| `crates/jj-workspace/src/jj.rs` | `23eb9d98df0eb1906f25cfefd869c606d2750b99b703d35d0c533f5cba1f8af6` | unmoved |
| `crates/jj-workspace/src/refusal.rs` | `6ea88fc2678d7b1ad4f997adf42705699d0a9bda94b997d753902828409cdc3f` | unmoved |
| `docs/specs/walkthrough-books.md` | `d9556f93b1112fcda770aa4564c517a08b076ee241010132c7cf428abd86535d` | unmoved |
| `docs/specs/jj-workspace-book-structure.md` | `f7c9fa4ad87c79940dbb6d393d4cb03875641c17472e90f5b5eeb99327739182` | unmoved |
| `docs/USAGE.md` | `ed79be856e3427c1e56f48a33b34f9b4cd4111b7c9d9bdaa17dad9917556a59f` | unmoved |
| `CONTEXT.md` | `c0d4698440f2d925f6c67563487add708100cd8188be025267039be09b733822` | unmoved |
| `docs/walkthroughs/ordinal-fs-tree/` | `03c1fd3ebebffbf03202cfa5a597dae598293300c04a6abcaa4fb7199d5b7553` | unmoved |

<a id="what-was-read"></a>
## What was read, and how

All seven numbered pages, `README.md`, `concept-index.md`, `source-index.md` and
`walkthrough.toml` were read in full, together with the four corpus files and
both integration test files, before any edit. Proof's charter is whole-book, so
the read was organised around the things no single page's read can reveal:

- **Every count in the book was recomputed** against the corpus — owned lines per
  slice, ownership block widths, fragment line ranges, comment-versus-code splits,
  the number of variants, arms and guards, and the size of the crate's own test
  suite. One of them disagrees with another page: claim 2.
- **Every term the book uses with a number attached** was read across all pages
  for a consistent count. One drifts: claim 3.
- **Every reproduced refusal message and every reason string** was compared with
  `refusal.rs` and `lib.rs`. All match; the developmental edit's finding 1 was
  repaired by the technical edit and stays repaired.
- **The three findings the earlier stages handed forward explicitly** were taken
  in turn: the technical edit's finding 2 and the copy edit's findings 1 and 2
  (claims 1 and 5), and the art stage's finding 4 (claim 6).
- **Every summary sentence was held against the section it summarises**, since a
  chapter's closing paragraph is where a page is most likely to contradict itself
  and is the last thing an earlier stage reads. One does: claim 4.

Three things the read found and left alone, all three deliberate and all three
recorded under `## Findings not fixed`.

<a id="claims"></a>
## Claims

Six, over four pages: two `T2`, one `P1`, one `C1`, one `C4` and one `A1`.

| # | Class | Anchored subject | Before (verbatim citation) | After (verbatim citation) | What a reader gains |
|---:|---|---|---|---|---|
| 1 | `T2` | `02-the-gate.md#worked-resolution` | *The walk is the filesystem's, so no environment variable and no shared repository store can redirect it, which is asserted directly by `resolution_ignores_repository_selection_and_temporary_directory_environment`* | the same claim, then *One half of it is asserted: … sets three of the four repository-selection variables to a foreign colocated repository and requires the intended workspace to come back. The other half is not, because no test constructs a shared repository store* — with a link to the seam's account of that test's scope | The cited test sets four environment variables and constructs no shared repository store, so *asserted directly* over-reached on half the claim. Chapter 3 already stated the scope precisely twice; a reader who stops at chapter 2 now gets the same precision rather than a stronger promise the test does not keep |
| 2 | `P1` | `05-scope-and-commit.md#the-path-algebra` | *No test in the suite passes a path containing either character … every path the **twenty-eight** interface tests name is ordinary* | *every path the suite's **twenty-nine** interface tests name is ordinary* | The suite is 28 tests in `workspace.rs` plus 1 in `environment.rs`, and `07-what-jj-owns.md#final-verification` says *Twenty-nine tests* over the same set. The undercount left one test outside the very sentence certifying the claim, and two pages gave two sizes for one suite |
| 3 | `C1` | `06-refusal.md#the-cause-chain` | *That is the only **two-link** chain the crate produces, and the worked example measured its depth: **one link**, then `None`* — and, below it, *A consumer walking `source()` from a failed commit gets **two links** and stops; the third and fourth causes exist only as bytes in a string* | *the only chain the crate produces with a `Refusal` at both ends, and the worked example measured its depth: one link, then `None`* — and *takes one link, reaches the `CommandFailed` refusal, and stops; jj's own two causes below it exist only as bytes in a string* | *Link* was counting `source()` hops on one page and chain nodes on another, and one sentence used both. `#worked-refusal`'s bold heading and the concept index's *A `source()` chain one link deep* both count hops, so the section that reads the `match` was the one out of step. The chain now has one length in the book |
| 4 | `T2` | `06-refusal.md#what-this-chapter-settled` | *the messages are structured the same way throughout: what is wrong, where, a blank line, and what jj offers — with the remedy paragraph absent in exactly **the arm** that has nothing to add* | *absent in exactly the **two** arms that have nothing to add, `CommandFailed`, whose remedy is jj's own stderr, and `OutputNotText`, which has none* | Two arms of `Display` carry no `\n\n` and so no remedy paragraph, and the page's own case-analysis table has *none* in the fifth column for both of them. The summary counted one, contradicting the frozen source and the table eight hundred lines above it |
| 5 | `C4` | `07-what-jj-owns.md#the-uncomfortable-one` | the passage restated chapter 4's mechanism nearly in full and named no page to check it against: *Chapter 4 recorded the consequence in its worked example rather than describing it: `control_dir(".gitignore")` passes `validated_namespace`, reaches `fs::create_dir_all`, and comes back as a `ControlDir` refusal telling the caller to check permissions on a path…* | *Reserving `grove`* — linked to `04-namespace.md#worked-reservation` — *traced both endings that gap has: a refusal about the wrong thing in a colocated workspace, and, in one that is not, a call that succeeds… What matters at this row is only which check produced them* | The repetition met one of the contract's five conditions — this page's goal is the verdict, not the mechanism — and then exceeded the minimum semantic contract by restating the whole earlier explanation, unlinked. The verdict now keeps only what a verdict needs and sends a reader who wants the mechanism to the page that owns it |
| 6 | `A1` | `07-what-jj-owns.md#the-closed-ledgers` | the section closed two ledgers and a line total, and the book's nine unasserted claims appeared only one at a time across chapters 3, 4 and 5 — *Both are stated here as unasserted rather than left for a reader to assume covered*, *both were checked directly instead*, *No test in the suite passes a path containing either character*, *This guard is unasserted too*, and four more | a third bolded entry, **Evidence**, carrying a nine-row table — the claim, the page and anchor that states it, and what holds it instead of a test — then a paragraph reading the fourth column down and a paragraph naming the three rows that already have leaves | Chapter 7 teaches the reader to ask *if the tool's guarantee changed tomorrow, what here would go red?* and never answered it about the book itself. The partition was carried by nine sentences in three chapters, each invisible from the others; the reader can now see that this book proves 698 lines byte for byte and argues nine claims that nothing goes red on |

**Claim 6 is the art stage's finding 4, fixed here rather than there.** Art
recorded it as an `A1` it declined to draw because the relation is over the whole
book rather than over any one page, and handed it to proof. The attribution rule
credits the stage that fixes a finding, not the stage that found it. It is stated
under `A1` rather than `P1` because its before-text exhibits `A1`'s observable
test directly — a partition carried only by running prose — and the
preregistration's tie-break gives the earlier code in taxonomy order.

<a id="out-of-charter"></a>
## Out of charter

None, and none is possible. Proof's charter is the whole book and every taxonomy
class, so the preregistration states outright that proof cannot commit a charter
breach.

<a id="unclaimed-changes"></a>
## Unclaimed changes

None. This stage's diff is nine hunks over five files, and every one is part of a
claim above — in file order:

| Hunk | File | Claim |
|---|---|---|
| 1 | `02-the-gate.md` | 1 |
| 2 | `05-scope-and-commit.md` | 2 |
| 3 | `06-refusal.md` | 3 |
| 4 | `06-refusal.md` | 3 |
| 5 | `06-refusal.md` | 4 |
| 6 | `07-what-jj-owns.md` | 5 |
| 7 | `07-what-jj-owns.md` | 6 |
| 8 | `07-what-jj-owns.md` | 6 |
| 9 | `concept-index.md` | 6 |

Hunks 3 and 4 are one claim because they are one terminology decision applied in
the two places that used the term differently; a fix to only one of them would
have left the drift in place. Hunks 7, 8 and 9 are one claim because the new
ledger needs a lead-in sentence in the section that introduces the other two, the
table itself, and one navigation row in the concept index — the index being
*curated navigation into the arguments*, so an argument added without a row would
be unreachable from it.

No hunk touches a fragment directive, a fence, a source range, a heading, an
anchor, a ledger row or the manifest. Every change is prose, a Markdown table, or
one list item.

<a id="findings-not-fixed"></a>
## Findings not fixed

Three. Two are classes the frozen taxonomy does not carry and were already
recorded by earlier stages; one is a conflict inside a frozen standard that no
stage may resolve.

1. **`01-orientation.md#public-surface` is still a `D2` by one standard and a
   requirement of the other.** Carried forward unchanged from
   [`1-developmental-edit.md`](1-developmental-edit.md)'s finding 3 and
   [`3-copy-edit.md`](3-copy-edit.md)'s finding 3. The structure brief's chapter-1
   description requires the section, and the brief's own catalogue rule together
   with the book contract's forbids it. Nothing in proof's charter resolves a
   conflict between two clauses of a frozen standard either — proof is a read of
   the book against the standards, not a revision of the standards — and the draft
   followed the specific clause over the general one, which is the reading the
   book contract's own carve-out supports for most of the section. Recorded a
   third time so the report sees it survived every stage rather than being
   silently dropped at the last one.

2. **A class the taxonomy does not carry: an unnamed exception.** Carried forward
   from [`1-developmental-edit.md`](1-developmental-edit.md)'s finding 4.
   `06-refusal.md#the-case-analysis` is headed *Ten kinds, in nearly the order the
   reader met them* and hedges with *nearly* and *close to*. **This one has since
   been cashed rather than left hanging** — the section now names the displacement
   outright: *The one displacement is the seam's three, which the reader met in
   chapter 3 and the file lists fourth of the five groups…* The hedge is therefore
   no longer uncashed, and what remains is only that the heading still says
   *nearly* where the body says exactly which exception it means. That is a
   sentence-level judgement with no class in the taxonomy, and it is now cosmetic
   rather than a gap; it is recorded so the report can see the finding was
   resolved by the developmental edit's own later work rather than by this stage.

3. **A class the taxonomy does not carry: a link label that names its destination
   but not its purpose.** Carried forward unchanged from
   [`3-copy-edit.md`](3-copy-edit.md)'s finding 4 — `06-refusal.md#the-premise`
   cites `[anyhow](https://docs.rs/anyhow/1.0.102/anyhow/struct.Error.html)`,
   whose label names the crate rather than the page. `C4`'s observable test is
   scoped past exactly this and the surrounding sentence supplies the purpose, so
   normalising it here would be a stage acting outside the frozen taxonomy on the
   last read. Left as the copy edit left it.

**And one the technical edit left that this stage judged not to be a defect.**
[`2-technical-edit.md`](2-technical-edit.md)'s finding 1 — *its public surface is
fourteen lines* at `07-what-jj-owns.md#final-verification` — is gone: the copy
edit replaced it with a phrase that names them — *the three types and six
functions* the chapter links as `01-orientation.md#public-surface` — which is
checkable against that section (three types, four operations, two accessors) and
against the source. Recorded because a finding handed forward and then repaired
by an intervening stage would otherwise look unaddressed at the end of the chain.

None of the three is one of the seven the draft recorded in
[`0-draft.md`](0-draft.md)'s `## Findings not fixed`, and this stage found no new
defect in the crate: the four corpus defects the draft cut leaves for —
`jj-docs-url-k64`, `jj-owned-names-k65`, `lossy-path-rendering-k66` and
`duplicated-cause-k67` — are all still outstanding, all still stated on the pages
that meet them. Two of the four now carry a row in the book's new evidence ledger
— `jj-owned-names-k65` at row 5 and `lossy-path-rendering-k66` at rows 7 and 8 —
because those two are gaps in what the book can *prove*; `jj-docs-url-k64` and
`duplicated-cause-k67` are defects the book states and measures rather than claims
it cannot support, so they belong to their pages and not to that ledger.

<a id="scope-and-validation"></a>
## Scope and validation

**One commit**, carrying the nine book-file hunks above, this record, and the
leaf's `.grove/` bookkeeping. No corpus file, no other stage's record, no
`CONTEXT.md`, no `docs/USAGE.md`, no manifest, and nothing else outside the
preregistration's allowlist.

**Final validation, green after the stage:**

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/jj-workspace --final --check all
valid: 4 files, 698 resolved lines, 0 deferred lines, final=true
```

No edit in this stage touched a fragment directive, a fence, or a source range,
so the reconstruction the draft proved is byte-identical afterwards. `M105`'s
rule that a prose paragraph must be the nearest preceding nonblank block before a
fragment's opening directive constrained claim 6 the way it constrained every
table the art stage placed: the evidence table sits in `07-what-jj-owns.md`,
which owns no source and contains no fragment directive at all, so the rule was
satisfied without a placement decision.

**`bash scripts/check.sh` passes**, which is the obligation the task file names
beyond final validation: it runs `book-check --final --check all` over every book
root under `docs/walkthroughs/` by discovery, and the repository's own tests
alongside — including `every_repository_markdown_reference_resolves`, which is
what proves the intra-book links claims 1, 5 and 6 introduced — thirteen of them,
nine in the evidence table alone — resolve to anchors that exist.

<a id="the-book-is-whole"></a>
## The book is whole

The second half of proof's charter, checked rather than asserted.

- **Seven chapters, two lookup surfaces, one contents page**, and the manifest's
  page inventory matches exactly — `F0`-class inventory errors are what `--final`
  reports, and it reports none.
- **No deferred hole survives in prose or in a directive.** `F003` reports any
  `defer` at all in final mode, and `0 deferred lines` is the validator saying
  there are none.
- **Both ledgers resolve.** Eleven ownership blocks, every one `resolved`; eleven
  early-use rows, every one `explained`; and the owned-source totals sum to the
  698 the root brief froze.
- **Every cross-page promise this stage could find is kept.** The three that were
  not are claims 1, 4 and 6 above: a page promising more of a test than the test
  asserts, a page contradicting its own table, and a chapter teaching a question
  it never applied to itself.

The book is complete with no deferred holes, and this is the state
`measurement-report-k45` is written against.
