# What could not move
<!-- book-page id="what-could-not-move" slice="assembly" order="21" -->
[Previous: The loop](20-the-loop.md) | [Contents](README.md)

<a id="assembly"></a>
## The page that owns no source

<!-- rollup «owned-lines-total» -->
<!-- rollup «ownership-blocks» -->
This chapter owns none of the crate's 10,685 lines. The fragment graph spans
[chapter 1](01-orientation.md) through [chapter 20](20-the-loop.md),
including the captured observer in chapter 5. Its 40 ownership blocks are
`resolved`; the [source index](source-index.md) records that graph in full and
gives this page a row whose owned-line count is zero. The row exists to be zero.

What follows from that is the reason this page reads differently from the twenty
before it. **Every sentence here is a claim about another chapter, and the
validator cannot read one.** `book-check` proves that this page declares no
fragment, that its identity matches the manifest, that its navigation is the
final-page form and that every link it makes resolves. It has nothing to say
about whether [chapter 13](13-outcomes.md) argues what this page says it argues.
On every other page the fragment graph is a second reader: a claim about the
source sits next to the bytes, and a byte out of place is `F008`. Here there are
no bytes. The check that a sentence below is true is the chapter it names, read
again — which is how each of them was written, and the reason the page is worth
the length it is.

There is no carried-example step. The example's own observable end was
[chapter 14](14-finishing.md), where the grove deletes itself, and chapter 20
took the loop that drives it to its ending; what this page starts from is the
twenty chapters, and what it ends on is the three questions answered for the
crate. The book's stated outcome is that a reader can take **any** system that
has extracted a domain-free library from underneath itself and ask of the layer
that stayed: *what did not go, and why could it not?* Twenty chapters have each
answered that for one concern. This one asks it of the crate.

<a id="twenty-chapters"></a>
## Twenty chapters, three questions

The table is the chapter's application of the stated outcome to each
source-owning page in turn. Read a row for what one chapter kept; read a column
for what the crate kept, which is what the three sections after it do. Every cell
is the chapter's own answer where the chapter states one, and is derived from
what the chapter argues where it does not — the ten pages from
[11](11-a-grove-begins.md) to [20](20-the-loop.md) close by answering all three
under their own headings, and the ten before them do not.

| Ch | On the way in — the names | On the way through — the preconditions | On the way out — the policy |
|---:|---|---|---|
| [1](01-orientation.md) | The vocabulary itself — *kind*, *brief chain*, *outcome*, *handle*, *finishing* — named as the words no crate beneath has; and `VERSION`, one constant rather than several manifests kept in step | The lock is in the signature: `root_init` consumes a `Vacancy`, so it **cannot** run over a live grove. Compiler-enforced rather than checked | The permission itself: the manifest marks the crate *not domain-free* and buys five dependencies against it |
| [2](02-the-tokens.md) | The whole of it. A task-shaped name that is wrong is `Malformed`, never `Foreign` — the store sees a species, never a spelling | None. `parse` takes a string and a `Found`, and there is no tree yet | Three of the store's four verdicts, and which of the three halts a walk rather than skipping a subtree |
| [3](03-kind-slug-handle.md) | The handle is the identity and the position is not in it — a second grammar beside the filename's | None | A kind is an open token: open as to vocabulary, closed as to shape |
| [4](04-the-name.md) | `format(parse(f)) == f`, or one entity occupies two files. The cost is a conformance kit | The species the stem declares, checked against what the listing found | Refuse rather than normalise — and hand back the spelling the name should have had, or say why there is none |
| [5](05-opening.md) | Not the grammar, which was chapters 2 to 4: path construction, and refusal precedence stated in grove's own words | A guard is proof the tree was there when it was opened, and no more than that | Which condition an operator is told about first, and a contention probe that announces and never decides |
| [6](06-paths.md) | — | The library returns no paths, so grove builds them in exactly one place, and canonicalises only to compare, never to report | That a reported path is the caller's own spelling of the root, unaltered |
| [7](07-the-walk.md) | Met before this chapter's code runs: a species mismatch is chapter 4's grammar observed through chapter 7's verb | Whole-tree key and finish validation before exclusion; the first remaining ordinary live leaf in walk order, or the sole finish | One snapshot rule for driver, pick and viewer; exclusion never makes an ambiguous tree valid |
| [8](08-kind-and-briefs.md) | `leaf_entry` decides which entry a path argument meant by asking the grammar, not by re-reading the filename | Both verbs take an already-read tree, so the leaf and its context come from one snapshot | The kind is the token in the filename and never anything in the body |
| [9](09-resolve.md) | The reference grammar: five spellings, deliberately wider than the filename's and deliberately not canonical | A whole walk rather than a short-circuiting `seek`, because ambiguity is a property of the match set | `Ambiguous` is an answer and not an error — the caller re-asks with a handle |
| [10](10-growing.md) | The reference, resolved before the call rather than by the store | A precondition is worth the snapshot it is checked against: a resolution, a classification and a key prediction all off one `tree.snapshot()` | The task-file template, and the one kind an operator verb may not write |
| [11](11-a-grove-begins.md) | Almost nothing. One name grove *writes*: `01-requirements--plan-k1.md`, rendered by the grammar rather than formatted here | Moved out of the code and into the type system — `TreeVacancy` in the signature, not a check in the body | The **shape** — a charter *and* one live leaf, or nothing — and the **value** `plan`, stated twice with the agreement observed by nothing |
| [12](12-leaf-to-node.md) | Almost nothing new; one writing, the node's name composed from the promoted leaf's own triple | `decomposable`'s five arms over four conditions the store cannot see, run against the snapshot the promotion then plans from | The key survives, because the entity does |
| [13](13-outcomes.md) | The narrowest possible edit: one field of a `Parts` value, put back through a grammar proved canonical | Both verbs classify before they call; a bulk mark is planned and validated against the **first** guard's snapshot | Two terminal marks rather than one, and the arity asymmetry between `leaf-retire` and `leaf-prune` |
| [14](14-finishing.md) | A name the store would have written without complaint; the rule that only the driver may write it lives on grove's side | One opening yielding four answers; a teardown that revalidates three facts and hands its guard to the deletion unreleased | `DEFAULT_ROOT_SLUG`, the fixed first kind, and eleven lines telling an operator to do three things in an order |
| [15](15-the-verbs.md) | Every verb that names an entry takes a `Slug`, a `Kind`, a `Reference` or a `Handle`, and the store has a word for none of them | Structural rather than procedural: `root_init` takes a `Vacancy`, so the refusal to clobber is the shape and not a check | Twelve verbs and not fourteen; and that ending a session is the driver's job rather than the agent's |
| [16](16-the-lease.md) | Not a grammar but a **word**: the namespace `grove`, one string handed to a seam that could not have guessed it | The whole of the block — that the descriptor still names the path it opened, that the record belongs to the epoch it was admitted under | Six values nothing beneath could default: the namespace, two file names, a retry count, a handoff bound, a poll interval |
| [17](17-the-epoch.md) | No grammar and no parsing; one naming *decision* — the ambient context is a path, and the empty string is not a degenerate one | The admitted guard holds its shared lock for the operation's whole life rather than checking and releasing | Where its own evidence stops: four of the ladder's seven refusal arms held, and the rest named |
| [18](18-which-files.md) | Four slot names — `prompt`, `session_name`, `worktree`, `repo` — the whole of what grove tells a runner about its own domain | Find, refuse, then parse: trackedness is asked of the candidate the search already selected | The refusal of a **tracked** delta: the boundary between an untrusted repository and arbitrary code execution |
| [19](19-the-core.md) | Almost nothing. One name, `PLUGIN`, because one token appears twice in one sentence and both spellings must land on the same target | None — and it is earned: this is the only one of the crate's twelve Rust roots with no fallible construct at all | A **test** rather than a value: a sentence rides `${prompt}` only if its failure mode is one the skill cannot repair |
| [20](20-the-loop.md) | None of the grammar's, and one of its own — in the environment rather than in any grammar: `CHANNEL_VAR` | Two, and placed: the lease revalidated twice an iteration, the finish template asked of the **pre-transition** document | Four values a runner could not choose: whose directory, which variable publishes it, what a child may not inherit, how long the two graces are |

**The structure brief's bands say where a question is paid for, not where it is
asked.** The stated outcome assigns question 1 to chapters 2 to 5, question 2 to
6 to 13 and question 3 to 14 to 20, and the table above shows why that is a
centre of gravity rather than a partition: [chapter 11](11-a-grove-begins.md)
sits in question 2's band and meets question 3 *first*, as a creation rather than
a check; [chapter 8](08-kind-and-briefs.md) answers all three six chapters
before question 3's band opens, and says so — *three things stayed, and each is
one of the book's three questions*. Ten chapters, [11](11-a-grove-begins.md)
through [20](20-the-loop.md), close under the three headings verbatim. That the
bands nonetheless hold is visible in the columns rather than the rows, and the
next three sections read them.

<a id="the-names"></a>
## On the way in — the names


Filename parsing is canonical: every accepted spelling renders back to
itself. Reference lookup additionally accepts numeric and bracketed keys and
the root marker. Full handles use canonical positive keys and must agree with
the current title. Both routes share the name module’s key grammar where a
key appears in a name or handle.



Title-independent lookup is explicit in a bare key. A full handle
expresses both identity and a current title, so accepting an old title would
conceal a naming mistake. Neither lookup reads the brief body to recover a
title; the guarded snapshot’s names supply it.


**Six of the ten chapters that answer this question explicitly answer it by
naming what they do not own.** [11](11-a-grove-begins.md) and
[12](12-leaf-to-node.md) say *almost nothing*; [16](16-the-lease.md) and
[17](17-the-epoch.md) say the block owns no grammar at all;
[19](19-the-core.md) says *almost nothing, and the almost is the interesting
part*; [20](20-the-loop.md) says *none of the grammar's, and one of its own*.
That is the return on 1,743 lines rather than a gap in the book. Four of the six
take a `Handle`, a `Kind` or a `Slug` already made and none of them parses a
filename; the other two, [16](16-the-lease.md) and [17](17-the-epoch.md), do not
so much as name those types — `driver_lease.rs` contains no `Handle`, `Kind` or
`Slug` in any form, which is a stronger statement of the same fact. The grammar
was paid for once, at the seam, and eight chapters downstream of it spend
nothing.

**Part V answers question 1 four times and never with a grammar.** Where the
crate reads names it owns a grammar; where it writes names *outside* the tree it
owns single agreed strings, and the pattern is exact:
[16](16-the-lease.md) owns the namespace `grove`;
[18](18-which-files.md) owns four slot names;
[19](19-the-core.md) owns `PLUGIN`;
[20](20-the-loop.md) owns `CHANNEL_VAR`.
[Chapter 17](17-the-epoch.md) is the fifth and owns no name at all — what it
contributes is a decision *about* a name, that the ambient context is a path and
the empty string is not a degenerate one, which had to be written into a filter
because nothing else in the system would have noticed. Four strings and one
decision, across 2,601 lines that contain no parser. What a layer owns on the way
in is not always a grammar; sometimes it is a word, and the difference is whether
anything reads it back.

<a id="the-preconditions"></a>
## On the way through — the preconditions

The outcome's second cost is one sentence — *the check must run against the same
snapshot the operation then plans from, or it is a race with a name* — and the
book states it in **two idioms**, split cleanly by part.

*A race with a name* is Part II's, and four of that part's six chapters use it:
[6](06-paths.md), [7](07-the-walk.md), [9](09-resolve.md) and
[10](10-growing.md). *The same snapshot the operation then plans from* is
carried, in those words, by [12](12-leaf-to-node.md), [14](14-finishing.md) and
by **all five** of Part V — [16](16-the-lease.md), [17](17-the-epoch.md),
[18](18-which-files.md), [19](19-the-core.md) and [20](20-the-loop.md).
[Chapter 16](16-the-lease.md) is the only page in the book that uses both, which
is what a page sitting on the boundary between the tree and the untracked state
beside it would be expected to do, and it is worth checking rather than assuming.

**What the cost actually buys is visible in code shape rather than in checks.**
Chapter 10 is where it is stated most plainly, about its own block: every check
there is cheap, and what is expensive is the requirement that the check and the
operation see one tree. That is why `parent_node` takes a `&Guard` rather than a path, why
the key predictions sit inside a block whose braces end the borrow before the
mutation can consume the guard, and why `stale_cross_refs` has to say out loud
that it is looking at a *different* tree from the one the insert planned against.
A layer that checked against a fresh read would look identical on the page and be
a race.

**Three times the precondition is not a check at all.** `root_init` consumes a
`Vacancy`, so the refusal to clobber a live grove is enforced by the type system
and cannot be forgotten, cannot be reordered and cannot be tested for because it
cannot happen. [Chapter 1](01-orientation.md) states it as the crate's own claim
about itself, [chapter 5](05-opening.md) states which half of that claim is
absolute and which is weaker, [chapter 11](11-a-grove-begins.md) reads the verb
whose signature carries it, and [chapter 15](15-the-verbs.md) records it as the
clearest structural answer to question 2 in the crate. The same move appears once
more and smaller, in [chapter 13](13-outcomes.md): `DestinationOccupied` is not
guarded against but made *unreachable by arithmetic*, because an outcome infix
and a key are both parts of one name.

**And once the cost lands on code that has nothing to do with the operation.**
[Chapter 12](12-leaf-to-node.md) is where the second question is answered least
comfortably: a promotion breaks an invariant on the way through, so the
intermediate state has to be unobservable, so every reader in the workspace has
to go through one of two functions — a constraint on code that has no interest in
decomposition, held by a test in a different crate. That is what a precondition
costs when the operation underneath it is not atomic against the thing being
preserved, and it is the only place in the book where question 2's answer reaches
outside the crate.

**One chapter answers *None* and earns it by measurement.**
[Chapter 19](19-the-core.md)'s root is the only one of the crate's twelve with no
fallible construct anywhere in it — no `Result`, no `?`, no `bail!`, no `unwrap`,
no `expect`, no `panic!`, and it is the only zero rather than merely the lowest.
Its precondition work happened upstream: the lease
admitted the driver, the walk chose the leaf, the configuration resolved a
template. Composition is what is left when nothing can fail, and a module with
nothing to check is the cleanest possible discharge of a cost about *when* to
check.

<a id="the-policy"></a>
## On the way out — the policy

The third question is the one the book accumulates rather than answers once. Part
II answers it in passing while spending itself on question 2, and one page there
is worth pausing on. [Chapter 7](07-the-walk.md)'s *position in that walk is the
only schedule there is* reads like the most total policy in the book, and no
chapter after it revisits it — but the chapter files the sentence under
**question 2**, calling it *a scheduling decision wearing a traversal's clothes*,
and answers question 3 nowhere. Taking it at its word rather than reclassifying
it is the difference between reading the columns and rewriting them. By
Part V there is nothing else left: five chapters, 2,601 lines, and every one of
them is about a value or a rule that nothing beneath could have defaulted.

The chosen values, gathered from the ten chapters that state them, are a longer
list than any single page suggests: a default root slug and a fixed first kind
([14](14-finishing.md)); the shape a new grove must have ([11](11-a-grove-begins.md));
two terminal marks rather than one, and an arity asymmetry between them
([13](13-outcomes.md)); a surface of twelve rather than fourteen
([15](15-the-verbs.md)); a namespace, two file names, a retry count, a handoff
bound and a poll interval ([16](16-the-lease.md)); four slot names and the
refusal of a tracked delta ([18](18-which-files.md)); four launch choices
([20](20-the-loop.md)). [Chapter 19](19-the-core.md) is the odd one and the
most revealing: its answer to question 3 is not a value but a **test** — a sentence
rides `${prompt}` only if its failure mode is one the skill cannot repair —
because a value can be stated and a judgement has to be bounded or it grows.

<a id="stated-twice"></a>
### The cost the crate pays four times

The outcome's third cost is that **a chosen value must be stated where a reader
can find it**. The crate has a stated preference about how to do that, and it
states it on the book's first page. [Chapter 1](01-orientation.md) reads
`VERSION`'s own comment: reading one constant makes the agreement of the two
binaries and the prompt *a fact about a single definition rather than about
several manifests staying in step*. [Chapter 14](14-finishing.md) is the same
preference honoured — the driver's own finish leaf takes its name from the same
grammar as every operator-written leaf, so there is no second spelling to keep in
step — and [chapter 18](18-which-files.md) records the same reason given for
keeping the delta roots *off* `TemplateSource`: they come from the workspace `run`
is given, so there is one derivation of them and nothing to keep in step.

Against that, four chapters record the crate doing the opposite, and putting them
in one place is the clearest thing this page can do that no page before it could.
**Each is a value stated twice, and they differ only in what holds the two copies
together.**

| Value | Stated in | And again in | What holds them in step |
|---|---|---|---|
| The default root slug `plan` ([11](11-a-grove-begins.md)) | `DEFAULT_ROOT_SLUG` | a second crate's argument default | **One test**, and it postdates the reading |
| The four slot names ([18](18-which-files.md)) | `SLOTS` | the operator-facing configuration document | A comment, checked by no one |
| `PLUGIN` ([19](19-the-core.md)) | the constant | a marketplace manifest in another directory | **One test** |
| `CHANNEL_VAR` ([20](20-the-loop.md)) | this crate | the binary that reads it back, with no shared constant | Renaming it makes **nine tests** fail or hang |

Read down the last column and the four are a ladder rather than four instances of
one lapse: a comment nobody checks, two values each held by a single test, and one
held by nine. That is a range, and the range is the finding — the crate is not
careless about restated values, it is *inconsistent* about them, and the
inconsistency is invisible from any one page because each page has exactly one
instance of it.

**The first row has moved since the chapter that found it, and the column says
so.** When chapter 11 measured the default root slug its agreement was held by
nothing, which is the reading that put a leaf against it;
`default-root-slug-two-spellings-k159` closed the gap with a single test in
`crates/grove-llm/tests/`, a directory that is neither book's corpus — but the
repair was not free of the freeze, and this page is the wrong one to pretend
otherwise. It edited two lines of frozen source, the `default_root_slug` doc
comment, which moved the fragment reproducing them; and the `grove-llm` book reads
that test directory twice without owning it, in a console transcript and in a
total, so both moved by one. The other three rows the book records and corrects
none of them, because the corpus is frozen and a book may not edit what it proves
— and the ladder is the shorter for it: what was *nothing, a comment, one test,
nine tests* is now floored at one comment.

**None of that makes the code wrong**, and it is worth saying in the same breath
as the table. Every one of the four values is correct today, and three of the
four are held by a test. What the assembly shows is the shape of the crate's
answer to its own third cost: it prefers one definition, says so, and pays for a
second statement four times where a second reader lives somewhere the first
definition cannot reach — another crate, an operator document, a marketplace
manifest, another binary. That is not an accident of implementation. It is what
happens to *a chosen value must be stated where a reader can find it* when the
readers are in four different places.

<a id="the-crates-answer"></a>
## The crate's answer

The three questions, asked of `grove-loop` rather than of one of its chapters.

**On the way in.** Yes, and once. The layer owns one grammar the library beneath
it cannot check, it is the filename grammar, and it costs 1,743 lines of which a
conformance kit and seven lines at the end of `parse` are the price of canonicity
specifically. Everything else the layer owns on the way in is a *word* rather
than a grammar — four of them, all in Part V — and none of those is
round-tripped, so none of them costs anything like as much. The test a reader can
take elsewhere is not *does the layer own names* but **does anything read its
names back off disk**; that is what turns a naming convention into a canonicity
obligation, and it is the whole difference between `task_name.rs` and
`driver_lease.rs`'s two file-name constants.

**On the way through.** Yes, everywhere, and the discipline is uniform even where
the mechanism is not. Every chapter that touches a tree checks something the
library cannot see, and the chapters that check nothing are exact rather than
lax: two of them have no tree yet, and the third has a measurement behind its
*None*.
The cost is never the check — the checks are cheap throughout — but always the
requirement that the check and the operation see one snapshot, and it shows up as
borrow scopes, as guards passed on unreleased, as a second lock stated to be a
second observation, and once as a workspace-wide constraint on how any code may
read a tree. Where the layer could make a precondition into a type it did, and
the book reads that one `Vacancy` from four angles — chapters 1, 5, 11 and 15.
Where it could not, it classified in front of the call.

**On the way out.** Yes, and increasingly: the crate's last five chapters are
policy and nothing else. What the layer chooses is a default, a shape, a
vocabulary, a count, a bound and — once — a test for admitting sentences. The
cost it pays for stating those choices is the one it pays least consistently, and
the ladder above is the evidence.

**And the thing the crate is really for.** Three domain-free crates sit
underneath this one and none of them has a word for a kind, a brief chain, an
outcome, a handle or finishing. Twenty chapters have shown what that costs, and
the sum of it is legible: **the layer that stayed is the layer that holds
meaning, and meaning is expensive in exactly three places** — where a name must
survive a round trip through a filesystem, where a check must see what the
operation will act on, and where a value must be true in two places at once. A
reader taking this test to a system of their own does not need to have read this
crate. They need to ask, of the layer that stayed: *which of your names does
something read back? which of your checks sees a different snapshot from the
operation it guards? and which of your chosen values is written down twice?* The
first two questions this crate answers well. The third is where it has four
answers and no rule.

The three questions have run down twenty-one chapters as three column headings
and three recurring subheadings, and this is the one place they can be set out
whole. It is the page's last figure because it is the only thing here a reader is
meant to take away and use on a codebase this book says nothing about: the first
column is the frame, the third is what to go and ask, and the fourth is what
happened when it was asked of `grove-loop`.

| The question | Where meaning turns out to be expensive | What to ask of the layer that stayed | This crate's answer |
|---|---|---|---|
| On the way in — the names | where a name must survive a round trip through a filesystem | which of your names does something read back? | answered well — one grammar, canonical, and a conformance kit paid for it |
| On the way through — the preconditions | where a check must see what the operation will act on | which of your checks sees a different snapshot from the operation it guards? | answered well — uniform discipline, and the cost is always the snapshot rather than the check |
| On the way out — the policy | where a value must be true in two places at once | which of your chosen values is written down twice? | **four answers and no rule** — the ladder above |

The asymmetry in the last column is the book's result. Two of the three questions
this crate can be said to have settled; the third it answers four times over,
differently each time, and no page before this one could have seen that because
each of the four is alone on its own page.

<a id="what-the-book-made-redundant"></a>
## Thirty-one markers, and the one whose map was corrected

The book neither cites nor edits `docs/ARCHITECTURE.md`. What it does is make
that document's crate-internal description of `grove-loop` redundant, which is
the condition under which a later leaf deletes it. Thirty-one of the document's
forty-one residue markers name this crate — twenty-eight naming it alone, one
joint with `keyed-launch`, one with `jj-workspace`, one with `none` — and every
one of the thirty-one has been checked against the chapter the structure brief
maps it to, by the marker's own subject rather than by its line number.

**All thirty-one are covered by the chapters the map names.** The grammar and
its diagram, the kind token and the separator, malformed against foreign, and
one classification shared by every verb are chapters 2 to 4's; positions, keys,
handles and the terminal infixes are chapters 3 and 13's; refusal by name is
chapter 4's; which module owns which half of the tree is chapter 1's; the lock's
scope, what it does and does not promise, and the contention probe are chapter
5's; `addressable_key`'s refusal and `leaf-prune` on a node are chapters 6 and
13's; the walk and the finish-reservation rule are chapters 7 and 14's; the one
pick and what it serves is chapter 20's; what a node is and how a missing
`_BRIEF.md` is read are chapters 8 and 12's; `resolve` on a stem several leaves
share, and the grow verbs' refusal on the same reference, are chapters 9 and
10's; `leaf-add`'s unwinding, the lint's second opening and key prediction are
chapter 10's; the two calls of `root-init`, the one-store-operation scaffold,
`root_shape` on a withdrawn layout and `finish-commit`'s symlink gate are
chapters 11 and 14's; the transition table is chapters 11 and 14's; the two kind
tokens grove writes itself are chapters 3, 11 and 14's; the finish reservation,
the finish flow and the four teardown steps with their two undo commands are
chapter 14's; the core's three parts, what the core reads and the stated VCS in
`${prompt}` are chapter 19's; and the watch and the escalation, and the scrub
inside the seam with the loop's complementary list, are chapter 20's.

**One of those thirty-one reads that way only because the map was corrected, and
how it got there is the finding.** As drafted, the map carried *the walk and the
finish-reservation rule* and *the one pick and what it serves* as a single joint
row against chapters 7 and 14. That pairing is right for the first marker and
was wrong for the second. The one pick — the driver's single authoritative pick
per iteration, one guarded read copying the selected leaf's path, handle and
kind, the guard released before the second configuration load and the spawn, and
that one value serving readiness, the launch diagnostic, template selection and
the mandate with no second tree read — is covered by neither of those chapters.
**[Chapter 20](20-the-loop.md) is where it is covered**, and all but one clause
of it: that page reproduces the block which performs the pick and argues *the
kind is passed, never re-read* against it — one guarded selection reaches four
consumers, so the prompt and the command a session receives cannot disagree
about what kind it is. The one clause it does not argue is the marker's *the
read guard is released before the second configuration load and the spawn*:
chapter 20 reads the two configuration loads and the spawn in order, but its
guard passages are the terminal's and the signal's rather than the tree read's,
so the release point is shown and not argued. The map now splits the joint row
and names chapter 20 for the second marker alone, so the brief and this page say
the same thing.

The split mattered rather than being cosmetic, and for the reason a previous
correction to the same map recorded: the map **is** the deletion's coverage
obligation, so a marker attributed to a chapter that does not cover it is a
passage deleted on the strength of a page that never made it redundant — or,
read the other way, a passage left standing because the named chapters do not
carry it. The subject was covered either way, so the deletion was never unsafe;
what was wrong was the checklist, and the checklist is the only instrument the
deleting session has.

**So the condition holds.** Every one of the thirty-one markers naming this crate
has a chapter that makes its subject redundant, the two joint markers' other
books were written before this one, and this book is the last condition on all
thirty-one.

<a id="the-closed-ledgers"></a>
## The closed ledgers

The book's two ledgers are complete, and the first of the two closes mechanically
rather than by this page's say-so.

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
<!-- rollup «ownership-blocks-owned-by» of="allowed-to-mean" -->
<!-- rollup «ownership-blocks-not-owned-by» of="allowed-to-mean" -->
**Ownership.** 40 top-level blocks over 14 source roots, every one
`resolved`. [Chapter 1](01-orientation.md) owns 2 blocks; the other 38 belong to
later chapters. The captured observer joins chapter 5 as a resolved block. No `defer` remains anywhere in the book, and none may — `F003`
reports any defer at all in final mode, so *every reservation has become an
insertion* is something the validator refuses to let be false rather than
something this page asserts.

<!-- rollup «early-use-rows» -->
<!-- rollup «early-use-rows-at» of="01-orientation.md#the-cast" -->
<!-- rollup «early-use-rows-not-at» of="01-orientation.md#the-cast" -->
**Early use.** 52 rows, every one `explained`. 13 of them have their
first use at [*The cast*](01-orientation.md#the-cast), where `lib.rs`'s export
list names types belonging to twelve later chapters before any of them has said
what one is; the other 39 are the price of concept order over file
order inside the later parts. The structure brief fixed a smaller set in advance
and the drafting added to it, because **an early-use row is owed by naming or
exercising** and the manifest's rows are a floor rather than the set — a rule
this book established in its own chapter 2 and applied in every chapter after it.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
**Owned source.** 486 + 373 + 529 + 773 + 459 + 320 + 336 + 438 + 604 + 518 + 641 + 764 + 808 + 521 + 514 + 819 + 564 + 358 + 245 + 615 = 10,685 lines across
20 source-owning chapters. This closing chapter owns zero lines. The source
index records the root sizes, block ranges and chapter totals.

**The corpus's own claims.** Two were known false before drafting began, and
neither still is: each has been corrected at source, in one commit with the
paragraph that judged it. `session_config.rs`'s *the loop re-reads the
configuration once per iteration*, which `loop_driver.rs` refutes twice inside
this same corpus, went at `template-source-read-count-k86`, so chapters 18 and
20 now explain a comment that counts both of its reads rather than adjudicating
one that counts neither. `lib.rs`'s claim about which crates take
`version.workspace = true` went at `every-member-version-comment-k84`, so
chapter 1 now explains the quantifier the comment carries rather than
adjudicating one it does not. Many more were found while drafting — stale
enumerations, a miscounted helper list naming a function that never existed,
four comments addressing a module by a name this workspace does not have, five
unresolved intra-doc links, a citation to a requirement number no document in
this repository carries, a parenthesised citation naming no anchor at all — and
each was adjudicated beside its fragment, because the corpus was frozen while
this book was written and a book may not edit what it is proving. Where a page
judged a source change worth making, a leaf carries it; where it did not, the
bytes stand as written and the page says so. **How many of that second group have
since landed is not counted here**, and the omission is deliberate. They were
landing before this chapter was written and have gone on landing since, each
carried by a session with no reason to open this page — so a number on this line
is false again shortly after it is true. This line
has carried five of them, one through five in turn, and not one was true on the
day it was written. What stands in its place is structural and cannot go stale: a
repair travels in one commit with every page that reproduces the changed bytes,
which is the freeze rule the whole campaign runs on, so no page in this book
reproduces a byte a later commit changed without having changed beside it — and
`book-check` over the whole book is what refuses to let that be false, rather than
anything asserted here. Which claims have since been corrected is therefore
something the chapters say one by one, in the paragraphs that once judged them.
The repairs set out below are examples of the kinds rather than the set of them.
`unresolved-doc-links-k151` is one: all five broken intra-doc links were
repaired in one commit with the four pages that reproduce them. Chapters 7, 8
and 19 now explain the corrected spelling rather than adjudicating the broken
one; chapter 1 carries the fifth line's new bytes and explains nothing, because
it never adjudicated that link and owes no paragraph for one it did not judge.
The five did not share a cause, and the leaf landed two kinds of repair rather
than one. Four were unlinked: three named `task_tree.rs`'s own `#[cfg(test)]`
compositions, which `cargo doc` compiles away — the blind spot
[chapter 17](17-the-epoch.md#three-per-cent-and-a-blind-instrument) measures
from the inside — so no path could have resolved them, and the fourth named a
module `prompt-names-the-kind-k18` had deleted. Only the fifth took a path:
`Resolution` is public inside a private module and reaches the API as
`verbs::Resolution`, so the re-export is the only spelling that resolves.
`Cargo.toml`'s clauses locating `libc` in `task_tree` alone and `keyed-launch`
behind one verb were found while chapter 1 was drafted and corrected at
`manifest-dependency-clauses-k133`. `lib.rs`'s two claims that
`<worktree>/.grove` is spelled in exactly one place were found at
[chapter 5](05-opening.md)'s drafting, which could stop repeating them but owned
none of the bytes, and were corrected at `grove-root-join-clauses-k148` against
an enumeration of seven production sites. And `task_tree.rs`'s header claim that
canonicalisation appears once was found at [chapter 6](06-paths.md)'s drafting,
reproduced by [chapter 5](05-opening.md) and restated by
[chapter 8](08-kind-and-briefs.md), and corrected at
`canonicalisation-sites-k149`. Last, [chapter 18](18-which-files.md)'s two
broken addresses in `session_config.rs` — a citation to a *requirement 6* that
no document in this repository has, and a Markdown link to
`../docs/adr/untracked-configuration-delta.md` resolving from neither the
rendered page nor the source tree — were corrected at
`requirement-six-citation-k189`, both onto the same decision record and both
inside that root's frozen 358 lines, so the chapter now explains two repaired
citations rather than adjudicating two broken ones. Neither was found by an
instrument: the first string was unique in the repository, the second sat in the
one class nothing here reads, and both surfaced from enumerating the block's
citations by hand while the chapter was drafted. Chapter 1 now explains
corrected wording in every place it once judged a standing claim, and
adjudicates none. The structure brief's *Known in advance* counts five claims in
total, against the two this paragraph knew before drafting began, and none of
the five now stands in the corpus. The fifth is chapter 6's *canonicalisation
appears once*, which this book adjudicated and left standing because no leaf had
yet corrected it; `canonicalisation-sites-k149` did, and it joined the list on
the same terms as the other four. That list counts only claims a leaf has
corrected at source, so the ones this book judged and left standing are recorded
on their own pages and are not in it, wherever they fall.

<!-- rollup «source-roots» -->
The [concept index](concept-index.md) and the [source index](source-index.md) are
the two lookup surfaces and neither is part of the reading order. The source index
is the authoritative record of how the fragment graph reconstructs each of the
14 files; the concept index is curated navigation into the arguments and
makes no completeness claim.

<a id="final-verification"></a>
## Final verification

Two commands prove the book, and they prove different things. The first is the
only one that reads the corpus byte for byte.

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/grove-loop --final --check all
valid: 14 files, 10685 resolved lines, 0 deferred lines, final=true
```

`--final` is what separates this from every scoped run the twenty drafting
sessions made. In scoped mode a later chapter's range may be reserved by a defer
and counted as deferred rather than resolved, and each of this book's first five
children ended with thousands of lines legitimately deferred; in final mode a
defer is an error, every source root must expand to its complete file, and the
page inventory must match the manifest exactly. Fourteen files, 10,685 resolved
and 0 deferred is the whole frozen corpus reconstructed from explained fragments.

```console
$ bash scripts/check.sh
...
=== book-check
  book-check docs/walkthroughs/grove-llm
valid: 4 files, 1017 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/grove-loop
valid: 14 files, 10685 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/jj-workspace
valid: 4 files, 752 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/keyed-launch
valid: 9 files, 2073 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/ordinal-fs-tree
valid: 17 files, 8905 resolved lines, 0 deferred lines, final=true
  book-check docs/walkthroughs/overview
valid: 3 files, 204 resolved lines, 0 deferred lines, final=true
  6 book(s) checked, 0 failing
  ✓ book-check

check: all 8 principal checks pass
```

The second command is the repository-wide gate, and this is the first point in
this book's drafting at which it passes. It runs `book-check --final --check
all` over every book root under `docs/walkthroughs/` by discovery rather than
from a list, which is why this book has been inside the gate since chapter 1
created its directory and why every drafting session before this one left the
script red on `book-check` alone: a prefix deliberately leaves later blocks
deferred, and `--final` will not have it. Five of the script's tests cover this
book without naming it — `every_repository_markdown_reference_resolves` sweeps
every Markdown file in the repository, so this page's links are checked with the
rest; `every_book_root_has_a_documentation_ownership_row` fails a book root with
no row in the *Documentation ownership* table of `docs/ARCHITECTURE.md`, which
chapter 1's session added; and the three tests of
`crates/grove/tests/corpus_exception_inventory.rs` require this manifest's single
exclusion and its subject to equal the tables in
`docs/specs/walkthrough-books.md`, the third by attacking a narrowed subject. All of those live in the `grove` crate's
tests, not this crate's.

<!-- rollup «source-roots» -->
<!-- rollup «owned-lines-total» -->
<!-- rollup «chapters» -->
The book is complete: 14 roots, 10,685 lines, 21 chapters, two
lookup surfaces, zero deferred ranges. What it argued is that a layer which
extracts a domain-free library from underneath itself keeps exactly what carries
meaning, and that meaning is expensive in three measurable places. What it leaves
the reader with is the question to take to a layer of their own: **what did not
go, and why could it not?**

[Previous: The loop](20-the-loop.md) | [Contents](README.md)
