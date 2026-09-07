# grove-loop — book structure brief

## Status and provenance

This is the structure brief for the book at `docs/walkthroughs/grove-loop/`,
which does not exist yet and which this document is written ahead of, in the
form [`walkthrough-books.md`](walkthrough-books.md) requires under *The
structure brief*. It settles what that specification deliberately does not: the
chapter sequence, the mapping of that sequence onto the corpus, each chapter's
worked example, the early uses the order forces, and what the book does not
cover.

**This document is authored, not recovered, and it precedes its book.** Every
decision below was settled in the `grove-loop-structure-k36` interview and is
recorded, with its rejected alternatives, in that leaf's decision log. It
follows [`jj-workspace-book-structure.md`](jj-workspace-book-structure.md),
[`overview-book-structure.md`](overview-book-structure.md),
[`grove-llm-book-structure.md`](grove-llm-book-structure.md) and
[`keyed-launch-book-structure.md`](keyed-launch-book-structure.md), the four
briefs elicited before their books, and is uniform with them.

**This brief is the human contract; the manifest is its machine-readable form.**
The chapter sequence and ownership mapping below are what
`docs/walkthroughs/grove-loop/walkthrough.toml` records as its `[[page]]` and
`[[block]]` groups. Where the two disagree, that is a defect in one of them, not
a licence to prefer either.

**The corpus has moved since this brief was written, and every line count below
is the count as it stood then.** Accepted source changes carry their ledgers and
pages with them, so the authoritative record of the corpus as it now stands is
`docs/walkthroughs/grove-loop/walkthrough.toml` and the book's own
[source index](../walkthroughs/grove-loop/source-index.md). The tables here are
left as written because they are what the chapter cut was decided against, and
rewriting them would make them false about the decision they record. As of
`manifest-dependency-clauses-k133` the corpus is 10,542 lines over the same
thirteen roots, `Cargo.toml` being 68 rather than 59 and chapter 1 owning 445
rather than 436.

**What is different about this corpus.** Thirteen roots and 10,533 lines: 72% of
the campaign's remaining corpus, and five times the largest book written so far.
Three things about it are new.

- **It is the crate that is allowed to mean something.** The four books already
  written are all, in one form or another, about refusal — `jj-workspace` opens
  each chapter on what the crate declines to own, `keyed-launch` on what a stage
  must not add or interpret, the overview on what the binary declines to do,
  `grove-llm` on what is left when a binary is thin. This crate is their mirror,
  and says so in its own first paragraph: it is *the one library crate in the
  workspace that is allowed to be domain-bound*.
- **Thirty-eight per cent of the corpus is test code, and none of it is
  excluded.** Five roots carry an inline `#[cfg(test)] mod tests` —
  `tree_lifecycle.rs` 1,649 lines, `task_tree.rs` 1,008, `task_name.rs` 694,
  `driver_lease.rs` 564, `loop_driver.rs` 69, for 3,984 lines in all. The corpus
  exception inventory in [`walkthrough-books.md`](walkthrough-books.md) carries
  exactly one `grove-loop` row, for `src/task_grow/tests.rs`, so every one of
  those 3,984 lines is owned, reconstructed and explained like any other. That
  fact drives the chapter cut rather than following from it.
- **Three claims inside the corpus were known to be false**, two of them refuted
  by other roots of the same book and the third from outside it. Each was
  adjudicated on the page while the corpus was frozen, and all three have since
  been corrected at source, every adjudicating paragraph rewritten in the same
  commit as its comment. See *Known in advance: the claims this book
  adjudicates*.

**And it is the last book.** Of `docs/ARCHITECTURE.md`'s forty-one residue
markers, **thirty-one name this crate** — thirty spelled `residue(grove-loop…)`
and one, at line 1324, spelled `residue(jj-workspace, grove-loop)`. Three of the
thirty-one are joint: line 471 with `none`, line 1189 with `keyed-launch`, and
line 1324 with `jj-workspace` — and both of those books are already written, so
when this one lands, `architecture-residue-k75` is unblocked entirely.

## Audience and intended outcome

**The audience is settled and is not re-elicited here.** Decision 7 of `plan-k1`:
a reader who knows Rust and jj, has driven a grove, and meets grove's vocabulary
as terms linked to [`CONTEXT.md`](../../CONTEXT.md) rather than re-taught. The
depth is decision 1's — complete and source-exact, every byte of the corpus
reproduced in a fragment graph. The method is
`linkuistics:writing-code-walkthroughs`, whose eight-field intake `plan-k1`
answered.

**One audience refinement this corpus forces.** The reader has met four of the
other five crates through their own books, and this book is written on that
assumption: where `grove-loop` calls `ordinal-fs-tree`, `keyed-launch` or
`jj-workspace`, the book says what grove asked for and what it got back, and
does not explain the callee. The link contract (below) means it cannot link to
those books either, so the naming is done in prose, once, at the chapter that
makes the call.

### The stated outcome: the what-could-not-move test

At the end the reader can take **any** system that has extracted a domain-free
library from underneath itself — a store, a runner, a version-control seam — and
ask of the layer that stayed: *what did not go, and why could it not?* The
answer comes in three parts, each with a cost, each with a named test, and all
three provable inside this corpus.

| | The question | The cost it carries | Pinned by | Chapters |
|---:|---|---|---|---|
| 1 | **On the way in — the names.** Does the layer own a grammar the library beneath it cannot check? | A grammar you own must be canonical — `format(parse(f)) == f` — or one entity occupies two files, sharing a key and a position. Canonicity costs a conformance kit. | the conformance kit (`task_name.rs` 1059–1177); *question 2: the grammar is canonical* (1257–1312); `pick_refuses_a_species_mismatch_at_a_task_shaped_name` | 2, 3, 4, 5 |
| 2 | **On the way through — the preconditions.** Does it check what the library cannot see, and against *which* snapshot? | The check must run against the same snapshot the operation then plans from, or it is a race with a name. A refused run must consume nothing. | `prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address`; `a_refused_run_does_not_consume_positions_or_keys`; `one_process_creating_and_reading_a_grove_never_waits_on_itself` | 6–13 |
| 3 | **On the way out — the policy.** What does it choose that nothing beneath it could have defaulted? | A chosen value must be stated where a reader can find it, and the layer must not restate what the layer above owns. | `the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one`; `the_runtime_facts_restate_no_rule_the_skill_owns`; `the_library_imposes_only_libc` | 14–20 |

The three parts are the three things that are provably hard to move — meaning,
timing and choice — and grove pays a visible price for each. Chapter 21 applies
all three to every source-owning chapter in turn; that application is the whole
of what chapter 21 is.

**Two outcomes were rejected.** The **re-derivation test** — *what do you
remember that you could re-derive?* — is sharp and explains the crate's single
strangest fact, that a crate whose thesis is *the tree is the only state*
contains 1,383 lines of untracked-file locking; but it reaches chapters 2–6 and
8–10, some 3,700 lines, only by restatement. It survives as the thesis of
chapters 13 and 16, and the contrast between those two is stated once, in
chapter 13 — the earlier of the two pages, so the book states it before either
chapter needs it, and chapter 16 inherits it rather than re-arguing it. The
**maintainer outcome** — *you can now maintain this crate* — follows free from
source-exactness, is not transferable, and leaves chapter 21 nothing to
argue; all four preceding briefs rejected the equivalent on the same ground.

## The spine: what is left here is grove's own

**Every chapter opens on what this module kept when the domain-free crates took
the rest, and why it could not move.** The spine is recovered from the source
rather than imposed on it, and it is recovered from eight places, not one.

`src/lib.rs` lines 4–10 state it for the crate: *the one library crate in the
workspace that is allowed to be domain-bound … The other three say nothing about
grove — `ordinal-fs-tree` has an ordered tree, `keyed-launch` has a key and a
template, `jj-workspace` has a workspace and a commit — and none of them has a
word for a* kind, *a* brief chain, *an* outcome, *a* handle *or* finishing. *Those
are here.* `Cargo.toml` lines 12–16 state it again for the manifest, in bold,
citing decision 1 of [`module-decomposition.md`](module-decomposition.md). And
then every large module's own header opens on exactly the same move:

| Module | Its header's own words | What did not move |
|---|---|---|
| `task_name.rs` | *the whole seam between the task tree and the library that drives it* | the grammar |
| `task_tree.rs` | *What changes is who owns the walk* | path construction, and refusal precedence |
| `task_grow.rs` | *What grove still owns, and why each piece could not move* | the reference, the preconditions, the template, the lint |
| `tree_lifecycle.rs` | *what happens to a grove that the store has no word for* | beginning, outcomes, ending |
| `session_config.rs` | *What is left here is the part that is grove's alone* | whose file, and whether it is admissible |
| `verbs.rs` | *none of which the store has a word for* | the surface, and its three shapes |
| `driver.rs` | *the reason they are not in [`crate::verbs`]* | the two operations that are not verbs |
| `driver_lease.rs` | the seam owns *where*; grove owns *whose* | the lease, and the epoch |
| `loop_driver.rs` | ***All of that is `crates/keyed-launch`'s, not this module's.*** *What stays here is the four things a loop has to choose and a runner cannot* | the four choices |

Pinned by `the_library_imposes_only_libc` and
`every_consumer_takes_the_library_with_default_features_off`
(`crates/grove-loop/tests/library_dependency.rs`), which hold the manifest's own
statement of the boundary against `cargo metadata`.

**The difference from `jj-workspace`'s spine is stated once, in chapter 1, and
no later chapter returns to it.** That book opens each chapter on what the crate
refuses to own and names who owns it instead. This book is its mirror: every
refusal in the workspace hands work *to here*, and what arrives is the part
nobody else could take, because meaning is the one thing a domain-free crate
cannot hold. `keyed-launch`'s spine — what a stage must not add or interpret —
is the same relation seen from the other end, and chapter 1 says so in the same
sentence.

**Two spines were rejected.** *The lock a verb needs is visible in its
signature* (`lib.rs` 17–41) is the crate's sharpest claim and is
compiler-enforced rather than tested — a `Vacancy` is consumed, so `root_init`
**cannot** run over a live grove — but it reaches 5,629 lines and misses the
whole driver half, which has no locks and no `Tree` in its signatures. It is
chapter 1's stated claim and chapters 5–13's proof. *The tree's shape is the only
state* reaches 76% but makes `driver_lease.rs` — whose 819-line production half
is the largest single owned block in the book — open by contradicting the spine;
it is the thesis of chapters 13 and 16 instead.
*A library of verbs and, since `loop-crate-driver-k22`, a driver* is the crate's
true structural fact and explains the only odd thing about it, but it is one
boundary and so gives the book two rules rather than a rule per chapter — the
ground on which `keyed-launch`'s structurally identical candidate was rejected.
It is stated in chapter 1 and is the Part IV/Part V boundary below.

## Chapter sequence

**Twenty-one pages: twenty owning source, and a final-only assembly page that
owns none.** The order is the seam order the spine names — grove's grammar, then
grove's walk, then what the store has no word for, then the surface, then what a
runner cannot choose. Each chapter is cut by **concept**, and the inline test
blocks travel with the concept they prove rather than being gathered into a
chapter of their own.

The five **parts** below are not pages and take no slice ID. They are the
natural session groupings for the draft stage's decomposition, and they are what
the chapter count is for: `grove-loop-book-k37` opens a node and cuts its draft
work along them.

| # | Chapter | Slice | Roots and blocks | Lines |
|---:|---|---|---|---:|
| 1 | Orientation — what this crate is allowed to mean | `allowed-to-mean` | `Cargo.toml` 1–59; `lib.rs` 1–377 | 436 |
| | **Part I — the only grammar grove has** | | | **1,714** |
| 2 | The tokens, and the four verdicts | `four-verdicts` | `task_name.rs` 1–220, 1178–1199, 1313–1521 | 451 |
| 3 | Kind, slug, handle | `the-handle-not-the-position` | `task_name.rs` 221–590, 1522–1550, 1551–1714 | 563 |
| 4 | The name, and canonicity | `canonical-or-nothing` | `task_name.rs` 591–1020, 1021–1177, 1200–1312 | 700 |
| | **Part II — who owns the walk** | | | **2,541** |
| 5 | Opening, contention and refusal | `one-spelling-of-grove` | `task_tree.rs` 1–290 | 290 |
| 6 | Paths, and addressing | `paths-are-built-here` | `task_tree.rs` 291–570, 1016–1105 | 370 |
| 7 | The walk: pick and select | `first-live-leaf` | `task_tree.rs` 571–637, 1106–1360 | 322 |
| 8 | Kind, and the brief chain | `root-to-leaf` | `task_tree.rs` 638–746, 1361–1652, 1997–2023 | 428 |
| 9 | Resolve | `wider-than-a-key` | `task_tree.rs` 747–1015, 1653–1996 | 613 |
| 10 | Growing: `leaf-add` and `leaf-insert` | `what-the-library-cannot-see` | `task_grow.rs` 1–518 | 518 |
| | **Part III — what the store has no word for** | | | **2,725** |
| 11 | A grove begins | `never-mistaken-for-finished` | `tree_lifecycle.rs` 332–489, 1013–1076, 1077–1466 | 612 |
| 12 | A leaf becomes a node | `the-key-survives` | `tree_lifecycle.rs` 490–695, 1666–2234 | 775 |
| 13 | Outcomes are marked in place | `marked-in-place` | `tree_lifecycle.rs` 696–1012, 2235–2725 | 808 |
| 14 | Finishing | `the-tree-deletes-itself` | `tree_lifecycle.rs` 1–331, 1467–1665 | 530 |
| | **Part IV — the surface** | | | **516** |
| 15 | The twelve verbs, and the two that are not | `twelve-not-fourteen` | `verbs.rs` 1–363; `driver.rs` 1–57; `complete.rs` 1–96 | 516 |
| | **Part V — what a loop must choose and a runner cannot** | | | **2,601** |
| 16 | One live driver per working tree | `one-per-working-tree` | `driver_lease.rs` 1–819 | 819 |
| 17 | Which calls the lease admits | `which-calls-are-admitted` | `driver_lease.rs` 820–1383 | 564 |
| 18 | Which files take part | `whose-file-and-whether` | `session_config.rs` 1–358 | 358 |
| 19 | The guaranteed core | `too-late-to-say-later` | `prompt.rs` 1–245 | 245 |
| 20 | The loop | `four-things-a-runner-cannot-choose` | `loop_driver.rs` 1–615 | 615 |
| 21 | What could not move | `assembly` | — (final-only; owns no source) | 0 |
| | **Total** | | **13 roots** | **10,533** |

**Slice IDs are named for the rule each chapter carries**, none equal to a page
ID and none carrying a task key, applying `jj-workspace-structure-k17`'s decision
8 rather than re-eliciting it.

**Every block boundary was verified against the exact lines before it was written
down**, and each was moved back where necessary to start at the first line of an
item's own doc comment rather than at its signature — so no chapter begins by
reproducing the tail of a comment whose subject the previous chapter owns.

**Two blocks were placed while this brief was written, not during the
interview**, and both removed an early use rather than creating one. `task_tree.rs`
1997–2023 (*pick + brief-chain together*) sits in chapter 8 rather than chapter 7,
because it exercises `brief_chain`, which chapter 8 owns. `tree_lifecycle.rs`
1467–1665 — the `materialize_finish` and `transition_to_current` tests — sits in
chapter 14 rather than chapter 11, because they exercise the finish path, which
chapter 14 owns; chapter 11 keeps 1077–1466, the shared support helpers and the
`root-init` tests.

**Three sequences were rejected.** **Fifteen pages, one or two per root, with each
big root's tests as their own chapter** gives every chapter one contiguous block
and a trivially checkable source index — but chapters 3, 5, 8 and 11 would then be
*the tests for the previous chapter*, a cut by file half rather than by concept,
with chapter 8 alone at 1,649 lines and no rule of its own to open on. **Eleven
pages, one per root, whole files** is closest to the file layout and leaves a
2,725-line chapter. **Twenty-one pages in the reader's order** — the loop first,
then a session's life, then the grammar underneath — is `docs/USAGE.md`'s order and
the way a grove actually runs, but every early chapter would use vocabulary the
book has not defined, and a seam is only visible once both of its sides are named.

**Two books were also rejected**, and on a fact rather than a preference:
[`walkthrough-books.md`](walkthrough-books.md)'s subject inventory carries exactly
one row for this crate, `grove-loop | crates/grove-loop`, and
`every_books_subject_is_exactly_the_specifications_inventory` requires the manifest
and that table to agree per book. Both halves of the crate live side by side in
`crates/grove-loop/src`, so no subject path separates them, and the test's own
narrowing attack case exists to reject exactly that move. Size is handled by the
five parts instead.

## Concept and seam responsibilities

### `README.md` — reader contract

States the audience, the spine in one sentence, the stated outcome's three
questions, and the corpus: thirteen roots, 10,533 lines, of which 3,984 are
inline test modules that are part of the corpus rather than excluded from it. It
carries the book's required guide citation,
[`usage-task-tree`](../USAGE.md#usage-task-tree) — *The task tree and its
filename grammar* — which is the guide section that states this crate's subject
in the reader's own terms, and which no other book reserves.

### 1 · Orientation — `allowed-to-mean`

**The rule: this is the one crate in the workspace that is allowed to mean
something.** `Cargo.toml` and `lib.rs` are the crate's own statement of that,
and the chapter reproduces both whole. The manifest's `[dependencies]` table
declares **five**, and four of them carry their reason in situ — `anyhow`
internal only, so a consumer takes on no error library; `libc` for the three
production modules that reach it, the `flock(2)` contention probe among them;
`keyed-launch` reached once by the verb surface and in four more places by the
crate; `ordinal-fs-tree` with `default-features = false`, so the dependency set this
crate *imposes* is exactly `libc` — and `tests/library_dependency.rs` holds that
last claim against `cargo metadata` rather than leaving it as a comment. The
fifth is `jj-workspace`, and it has no clause of its own: the comment accounts
for it only as one of *the three modules it composes*. That is a gap in the
manifest's own account rather than an incidental omission, because
`jj-workspace` is the crate's version-control seam — `tree_lifecycle.rs`,
`driver_lease.rs`, `prompt.rs`, `session_config.rs` and `loop_driver.rs` all
`use` it directly, and `lib.rs` re-exports `Commit` and `Workspace` — so the
chapter says what it is reached for where the manifest does not.

From `lib.rs`: the module list, `VERSION`, the *Opening mirrors the store's* and
*Three shapes* theses, the one-error rule, and the `<worktree>/.grove` join
`read` and `write` share. The chapter names nearly every
public type before its owner explains it — that is what a crate's own map is for,
and it is the shape all four preceding orientation chapters took.

**It carries three things no later chapter returns to:** the difference between
this book's spine and `jj-workspace`'s and `keyed-launch`'s; the *lock in the
signature* claim, which is compiler-enforced and proved in chapters 5–13; and the
`Key`/`Entry` collision — this crate takes the store's `Key` and `Entry` and adds
grove's `Kind`, `Handle` and `Outcome` beside them, which
[`CONTEXT-MAP.md`](../../CONTEXT-MAP.md) keeps apart by hand and the book must
not blur.

**It explains which crates take `version.workspace = true`, and which member
does not, and why the manifest's `libc` and `keyed-launch` clauses are worded as
they are.** See *Known in advance*.

### 2 · The tokens, and the four verdicts — `four-verdicts`

**The rule: a task-shaped name that is wrong is Malformed, never Foreign,
because skipping it is lost work.** `Outcome`, `TokenError`, `refuse_token`, and
the three constants the grammar is spelled with (`BRIEF`, `KEY_MARK`,
`SEPARATOR`). The tests are the classification tests and the refusals-inside-the-
shape tests: `the_charter_is_the_distinguished_child`,
`a_name_that_is_not_task_shaped_is_foreign`,
`a_session_kind_that_is_not_a_token_is_malformed` — the last of which is a *shape*
refusal that names the character it refused, and whose own doc comment records
that it used to be a membership refusal listing nineteen labels.

### 3 · Kind, slug, handle — `the-handle-not-the-position`

**The rule: the handle is the identity, and the position is not in it.** `Kind`
with its two reserved labels, `Slug`, `HandleError`, `Handle` and `Parts`. The
chapter's structural claim is the one `every_positioned_name_ends_in_its_own_handle`
asserts: every positioned name's rendering ends in its own handle's rendering — a
node's exactly, a leaf's followed only by the `.md` suffix — so a second spelling
of `<slug>-k<key>` anywhere in `Display` fails the moment the two disagree. That
test's own comment says why it is asserted rather than reviewed: *drift is not
expressible* has to be held by something, or it is a promise. Also
`the_slug_rule_is_the_one_grove_already_had`.

### 4 · The name, and canonicity — `canonical-or-nothing`

**The rule: `format(parse(f)) == f`, or one entity occupies two files.** `TaskName`,
`TaskNameError`, the `EntryName` implementation that is the whole seam, and the
helpers under it. The chapter carries the conformance kit — the library's own,
run against grove's grammar — and the canonicity argument, whose stakes the
module header states: the withdrawn model was lenient on padding, accepted a
hand-typed `5` and rendered `05`, so one entry could occupy two files sharing a
key and a position, which is the library's canonicity obligation broken.

### 5 · Opening, contention and refusal — `one-spelling-of-grove`

**The rule: a guard is proof the tree was there when it was opened, and no more
than that.** `Tree`, `Guard`, `Opening`, `TreeVacancy`, the four openings, and the
three error paths: `absent_tree`, `raised`, and `restate`. The chapter owns
`announce_contention`, which is the use of `libc` the manifest named alone until
`manifest-dependency-clauses-k133` — grove probes `flock(2)` non-blocking before
it announces a wait, so a caller is told it is waiting only when it really is. It
is not the whole reason `libc` is a dependency, and the chapter says so rather
than letting the probe stand for the crate: `libc` is reached from three
production modules — this probe, `driver_lease.rs` for the lease's own locking
and its close-on-exec descriptors, and `loop_driver.rs` for the terminal and
signal calls chapter 20 reads. Chapter 1 owns the manifest and explains all
three.

### 6 · Paths, and addressing — `paths-are-built-here`

**The rule: the library returns no paths, so grove builds them — in exactly one
place.** `entry_path` and why it is safe without a check; `Target`, `target` and
`unreachable_by_any_walk`; `addressable_key` and `interrupted_promotion`;
`next_key`, `live_leaf`, `entry_outcome`. The chapter also carries why nothing
here canonicalises for output: on macOS `/var` and `/private/var` name the same
inode, so canonicalising would make the mere presence of a lock rewrite every
path grove prints. Its tests are the block the source itself labels *the
path-taking compositions, which are the tests' alone*.

### 7 · The walk: pick and select — `first-live-leaf`

**The rule: the first live leaf in walk order, and position in that walk is the
only schedule there is.** `Selection`, `pick_in`, `select_in`, `select_in_write`,
`selected`. Nineteen tests — the block-opening
`select_returns_path_handle_and_kind_from_one_guarded_observation` and eighteen
named `pick_*` — and the chapter's prose owes the negative case for each:
`pick_orders_numerically_not_lexically` discriminates only where the digit
count changes, so the pair that makes it a test is `100` against `99` —
bytewise `"100" < "99"` while numerically 99 < 100. A position is zero-padded
to at least two digits and carries no other leading zero, so 9 renders `09`
and 10 renders `10`, and `"09" < "10"` lexically as well as numerically: any
pair below the 99/100 boundary separates the two orderings not at all.
`pick_returns_first_live_leaf_in_per_level_order`, `pick_skips_done_leaves`,
`pick_skips_abandoned_leaves`, `pick_descends_a_node_in_preorder`,
`pick_falls_through_an_all_done_node_to_a_later_live_leaf`,
`pick_lenient_on_foreign_files` and
`pick_refuses_a_species_mismatch_at_a_task_shaped_name` — the last being the
spine's sharpest case, since the store would have accepted the entry and the
grammar that refuses it is the thing that did not move.

### 8 · Kind, and the brief chain — `root-to-leaf`

**The rule: ancestor briefs, root to leaf, and a brief is not a leaf.** `kind_in`,
`brief_chain` and `leaf_entry`, with the `kind` and `brief-chain` test blocks and
the closing *pick + brief-chain together* block that exercises the two against one
observation.

### 9 · Resolve — `wider-than-a-key`

**The rule: grove's reference grammar is wider than a key, and has an ambiguous
outcome the library has no counterpart for.** `Resolution`, `Located`, `located`,
`resolve_in`, `Lookup`, `lookup`, `reference`, `existing_path`, `Ref` and
`parse_ref` — a path, `[n]`, `n`, `<slug>-k<key>` or a bare slug. The two resolve
test blocks, including the one the source separates out for the full
`<slug>-k<key>` handle.

### 10 · Growing: `leaf-add` and `leaf-insert` — `what-the-library-cannot-see`

**The rule: the preconditions the library cannot see, checked against the same
snapshot the operation then plans from.** This chapter is the spine at its most
explicit, because the module header is a list of exactly it: the reference
grammar, the preconditions, the task-file template, and the cross-reference lint,
each with its own paragraph saying why it could not move. The chapter carries key
*prediction* — because the template's bytes embed the key, grove predicts the
allocation and checks it against the report.

**The tests for this root are evidence, not corpus.** `src/task_grow/tests.rs`
(1,680 lines) is the book's one declared corpus exclusion, so its tests are cited
by name and never reproduced: `add_refuses_an_ambiguous_parent_slug_and_lists_the_keys`,
`add_preserves_a_gap_a_hand_edit_left_rather_than_filling_it`,
`a_refused_run_does_not_consume_positions_or_keys`,
`insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved`.
This is the only chapter in the book whose proof is entirely outside its own
pages, and it says so.

### 11 · A grove begins — `never-mistaken-for-finished`

**The rule: a fresh grove starts with one live leaf, so it is never mistaken for
finished.** `root_init`, `default_root_slug`, `initialize_grove`, `RootShape` and
`root_shape`, plus the three body-writing helpers — `grove_name`,
`root_brief_body`, `append_brief_suffix_in_file` — that are shared with chapter
12. The whole grove is created as **one store operation under one lock**, which is
what `root_init_creates_the_whole_grove_through_one_store_operation` holds, and
`a_refused_grove_leaves_no_root_behind` is its negative. The chapter also owns the
shared test-support block the next two chapters' tests use.

### 12 · A leaf becomes a node — `the-key-survives`

**The rule: the key is preserved, because the entity that was the leaf becomes
the node.** `leaf_decompose`, `decomposable` and `promoted`: the leaf *file*
`NN-<kind>--<slug>-k<key>.md` becomes the node *directory* `NN-<slug>-k<key>/`,
its body renamed in as `BRIEF.md`, and a first child grown atomically so a node
is never childless. `decompose_converts_leaf_file_to_node_dir_preserving_the_key`,
`decompose_seeds_brief_from_leaf_body_and_appends_brief_suffix`,
`decompose_creates_the_first_child_at_01_with_a_fresh_key`,
`decompose_with_no_override_inherits_the_parent_leafs_own_kind` and the four
refusals.

### 13 · Outcomes are marked in place — `marked-in-place`

**The rule: done-ness and abandonment are marked in the name, and the body is not
rewritten.** `leaf_retire` and `retire_parts`; `PruneResult`, `leaf_prune`,
`Planned`, `plan_prune`, `plan_subtree`, `plan_leaf`, `apply_prune`,
`stopped_partway` and `marked_path`. `retire_adds_done_infix_keeping_position_and_key`
and `retire_does_not_rewrite_the_header_or_body` are the pair that make *in place*
mean something; `prune_node_marks_every_live_leaf_in_the_subtree`,
`prune_node_leaves_done_leaves_untouched` and
`prune_node_is_atomic_bails_clean_on_a_leaf_it_cannot_address` are the bulk
arity, and `pruning_a_node_takes_one_guard_per_mark` is the cost the atomicity is
*not* paid with.

**This chapter states the 13/16 contrast, once, for both.** *The tree's shape is
the only state* is this chapter's thesis and chapter 16's deliberate exception;
the earlier page is where the pair is named, so chapter 16 inherits the sentence
instead of writing its own.

### 14 · Finishing — `the-tree-deletes-itself`

**The rule: the driver is the only author of the leaf that ends the grove, and
the ending deletes the tree.** `CurrentTransition`, `transition_to_current`,
`materialize_finish` and the four finish-leaf helpers; `finish_commit`,
`delete_and_commit` and `require_recoverable_grove`. The chapter's worked example
is the carried one's last step, and its observable end is the grove ceasing to
exist. `materialize_finish_writes_a_handle_that_matches_its_own_filename` holds
the three spellings agreeing — filename, handle and the `finish-commit` command
the leaf's body tells an operator to run — and
`a_tree_at_the_last_key_refuses_the_sentinel_rather_than_wrapping` is the
boundary case.

### 15 · The twelve verbs, and the two that are not — `twelve-not-fourteen`

**The rule: the surface is twelve verbs, and everything else that touches the
tree has to say why it is not one.** `verbs.rs` declares fourteen `pub fn`; two
of them — `stale_cross_refs`, a lint that follows an insert, and
`signal_channel`, a question a caller must ask *before* the write rather than
learn from it — are not verbs and each says so in its own doc comment.
`driver.rs` holds two more tree operations that are not verbs either
(`transition_to_current` runs before any session exists; `materialize_finish`
creates the one leaf `leaf-add` is forbidden to create) and states that putting
them beside the twelve *would say the surface has fourteen verbs, which it does
not*. `complete.rs` is the twelfth verb and the loop's child-side half: it writes
the disposition and nothing else, because an in-agent self-kill cannot be trusted
under every harness sandbox.

### 16 · One live driver per working tree — `one-per-working-tree`

**The rule: the seam owns *where* an untracked coordination directory may live;
grove owns *whose* it is.** The lease file and the epoch file, `FileIdentity`,
`ProcessRecord`, `EpochRecord`, `LockMode`, `DriverLease`, `SessionEpochGuard`,
the acquisition and locking paths, `ensure_close_on_exec`, the nonce and path
encodings, the record parsers, `probe_live_lease`, and `admit_ambient_session`
with `admit_session` under it.

**This is the chapter where the prose carries the load.** At 12% comment prose
over 819 lines it is the thinnest-argued root in the corpus. It is the **fourth**
largest root at 1,383 lines, behind `tree_lifecycle.rs`, `task_tree.rs` and
`task_name.rs`, and its 819-line production half is the **largest single owned
block in the book**; see *What each chapter's prose owes*.

**It is also the book's deliberate counterexample.** Everything else in the crate
is re-derived from the tree; this is the one thing that cannot be, and chapter 13
has already said so in a sentence. Inherit it rather than re-arguing it — and
read chapter 13's page for the wording, not this brief.

### 17 · Which calls the lease admits — `which-calls-are-admitted`

**The rule: the epoch decides which `grove-llm` calls a live driver admits, and a
handoff is ordered rather than timed out of trouble.** The whole inline test
module — **eighteen** `#[test]` functions, in file order:
`lease_path_replacement_retries_until_the_locked_descriptor_is_current`,
`lease_path_replacement_fails_closed_after_eight_attempts`,
`acquired_driver_descriptors_are_close_on_exec`,
`activation_and_invalidation_replace_one_stable_epoch_record`,
`epoch_acquisition_retries_open_lock_path_replacement_in_event_order`,
`an_orphaned_epoch_guard_times_out_post_reap_once_at_the_fixed_bound`,
`the_epoch_contention_diagnostic_names_the_lock_mode_and_operation`,
`manual_agent_operations_need_no_driver_epoch`,
`only_a_nonempty_loop_control_value_is_ambient_context`,
`an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls`,
`replacement_keeps_the_old_lease_record_until_it_owns_epoch_handoff`,
`ambient_context_from_another_worktree_names_both_roots`,
`an_inactive_epoch_is_reported_without_claiming_a_session_is_active`,
`a_rotated_epoch_refuses_the_old_signal_path`,
`an_epoch_signal_path_round_trips_record_separator_bytes`,
`a_successful_liveness_probe_releases_the_lease_before_validation`,
`an_active_epoch_without_a_live_lease_is_stale` and
`a_malformed_epoch_is_stale`.

At 3% comment prose this is the barest block in the corpus, and the chapter's
whole job is to say what each scenario establishes and what it would still pass
under.

### 18 · Which files take part — `whose-file-and-whether`

**The rule: everything a template *is* belongs to `keyed-launch`; what is left
here is whose file, and whether the second one is admissible.** The personal
file's path, the two roots the delta is searched at, `DeltaRoots`,
`TemplateSource`, the four slots grove's templates are written against, and the
refusal of a **tracked** delta — which could not move because it is a question
about grove's worktree, answered through grove's version-control seam, and it is
the boundary between an untrusted repository and arbitrary code execution.
Pinned by `the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one`,
`a_snapshotted_jj_delta_is_refused_in_both_jj_shapes`,
`a_trackedness_probe_that_cannot_be_completed_fails_closed` and
`a_grove_configuration_conforms_to_the_runners_own_kit`.

**It explains the corrected read count on `TemplateSource`.** See *Known in
advance*. Chapter 20 carries the other half of that explanation.

### 19 · The guaranteed core — `too-late-to-say-later`

**The rule: a sentence rides `${prompt}` only if its failure mode is one the
skill cannot repair — because by the time the skill could speak, the moment has
passed.** The three driver-authored parts and no methodology; the too-late test
and its closure on the word *fact*: a driver fact is a launch-varying value the
methodology cannot know at authoring time, and its static meaning stays in the
skill. `the_prompt_is_three_parts_in_the_sessions_own_timeline_order`,
`the_runtime_facts_restate_no_rule_the_skill_owns`,
`the_signalling_contract_states_the_mechanism_and_defers_the_ending`,
`every_kind_names_a_skill_the_plugin_ships`.

### 20 · The loop — `four-things-a-runner-cannot-choose`

**The rule: all of the spawning, watching and escalating is `keyed-launch`'s;
what stays here is the four things a loop has to choose and a runner cannot** —
which directory the channel is allocated in, which variable publishes it, which
variables are scrubbed, and how long the two graces are. `run` and `drive`,
`session_prompt`, `launch_configured_session`,
`complete_post_reap_epoch_handoff`, the `ESCALATION` constant, `reset_terminal`,
`ignore_interrupts`, `scrub_loop_control_env`, and the inline tests that hold the
ordering: `an_epoch_handoff_failure_preserves_the_launch_failure_that_preceded_it`
and `signal_interpretation_cannot_run_before_epoch_invalidation_succeeds`.

The chapter carries *restart ≡ continuation* — the loop body holds zero state and
re-derives position from the tree — and the shell sketch the header keeps, which
is still the whole loop *because a boundary is not a step*.

### 21 · What could not move — `assembly`

**Final-only, and it owns no source.** It applies the stated outcome's three
questions to each of the twenty source-owning chapters in turn, and answers them
for the crate as a whole. It is the only page whose every sentence is a claim
about another page, which is what makes it the page a reviewer should be spent on.

## The mapping onto the corpus

### Top-level ownership blocks

Thirty-nine blocks over thirteen roots. Nine roots are owned whole by one
chapter; four are split by concept, and each split is named below.

| Root | Lines | Chapters | Blocks |
|---|---:|---|---:|
| `Cargo.toml` | 59 | 1 | 1 |
| `src/lib.rs` | 377 | 1 | 1 |
| `src/task_name.rs` | 1,714 | 2, 3, 4 | 9 |
| `src/task_tree.rs` | 2,023 | 5, 6, 7, 8, 9 | 10 |
| `src/task_grow.rs` | 518 | 10 | 1 |
| `src/tree_lifecycle.rs` | 2,725 | 11, 12, 13, 14 | 9 |
| `src/verbs.rs` | 363 | 15 | 1 |
| `src/driver.rs` | 57 | 15 | 1 |
| `src/complete.rs` | 96 | 15 | 1 |
| `src/driver_lease.rs` | 1,383 | 16, 17 | 2 |
| `src/session_config.rs` | 358 | 18 | 1 |
| `src/prompt.rs` | 245 | 19 | 1 |
| `src/loop_driver.rs` | 615 | 20 | 1 |
| **13 roots** | **10,533** | | **39** |

### Where a file's concerns split across chapters

Four roots are split, and in every case for the same reason: **the file is
ordered by Rust convention — items, then `#[cfg(test)] mod tests` — and the book
is ordered by concept.** A production item and the tests that establish it are
one concept and belong on one page; the file cannot express that and the book
can.

- **`task_name.rs` three ways in nine blocks.** Production splits at the type
  boundaries — the tokens and outcomes (1–220), the named parts (221–590), the
  name itself (591–1020) — and the inline test module's own seven labelled
  sections distribute to the chapter whose concept each proves: *classification:
  the four verdicts* and *refusals inside the shape* to chapter 2; *the slug rule*
  and *the handle owns the grammar* to chapter 3; the support block, *the
  conformance kit*, *the grammar* and *question 2: the grammar is canonical* to
  chapter 4.
- **`task_tree.rs` five ways in ten blocks.** Production splits at opening
  (1–290), addressing (291–570), the walk (571–637), kind and briefs (638–746)
  and resolution (747–1015); the test module's six labelled sections follow, with
  the closing *pick + brief-chain together* block going to chapter 8 because it
  exercises `brief_chain`.
- **`tree_lifecycle.rs` four ways in nine blocks.** The file opens on finishing
  and closes on the body helpers; the book opens on beginning and closes on
  finishing, so chapter 14 owns 1–331 and chapter 11 owns 1013–1076. The test
  module's labelled sections distribute to the verb each names, with 1467–1665
  — the `materialize_finish` and `transition_to_current` tests — going to chapter
  14 rather than chapter 11.
- **`driver_lease.rs` two ways in two blocks**, at the `#[cfg(test)]` line. This
  is the one split in the book that is *not* by concept, and it is deliberate:
  the production half is 12% comment prose and the test half 3%, so the two
  halves need opposite prose treatments — one supplies an argument the source
  does not make, the other supplies claims for scenarios that state none — and a
  chapter cannot carry both instructions at once.

### Owned-source totals

| Chapter | Lines | Chapter | Lines |
|---:|---:|---:|---:|
| 1 | 436 | 12 | 775 |
| 2 | 451 | 13 | 808 |
| 3 | 563 | 14 | 530 |
| 4 | 700 | 15 | 516 |
| 5 | 290 | 16 | 819 |
| 6 | 370 | 17 | 564 |
| 7 | 322 | 18 | 358 |
| 8 | 428 | 19 | 245 |
| 9 | 613 | 20 | 615 |
| 10 | 518 | 21 | 0 |
| 11 | 612 | **Total** | **10,533** |

The mapping closes arithmetically against the frozen corpus: 12,154 lines of
`src/**/*.rs`, less the 1,680 of the excluded `src/task_grow/tests.rs`, plus the
59 of `Cargo.toml`.

## What each chapter's prose owes

Two obligations are carried unchanged from the four preceding briefs:
**adjudicate the claim** — a comment the code does not bear out is stated as
such on the page beside the fragment that reproduces it, never repeated — and
**carry the through-line**, so each chapter connects to the spine and to the
stated outcome rather than standing alone.

The third is this book's own, and it was set by measurement.

| Root | Production | prose | Tests | prose |
|---|---:|---:|---:|---:|
| `task_name.rs` | 1,020 | 42% | 694 | 20% |
| `task_tree.rs` | 1,015 | 42% | 1,008 | 15% |
| `tree_lifecycle.rs` | 1,076 | 41% | 1,649 | 15% |
| `loop_driver.rs` | 546 | 51% | 69 | 8% |
| `driver_lease.rs` | 819 | **12%** | 564 | **3%** |

Unsplit roots, for comparison: `driver.rs` 73%, `prompt.rs` 69%, `complete.rs`
65%, `Cargo.toml` 59%, `lib.rs` 58%, `verbs.rs` 57%, `task_grow.rs` 49%,
`session_config.rs` 44%. Across the corpus, 3,662 of 10,533 lines are comment
prose — 35%, close to `keyed-launch`'s 33% — but the distribution is the fact
that matters, and it splits three ways.

**Third obligation: say what each reproduced test establishes, and what it would
still pass under.** It is stated per *block*, not per chapter, so a technical
review can check it against the mapping above.

1. **Supply the claim — every inline test block** (chapters 2, 3, 4, 6, 7, 8, 9,
   11, 12, 13, 14, 17, 20; 3,984 lines, 38% of the corpus, at 14% prose). For each
   reproduced test: the property it establishes, **and what would have to be true
   for it to pass while the property was broken**. That second half is the part a
   reviewer can check and the test itself cannot state — a `#[test]` body shown
   without it teaches the mechanics and not the claim, and a test *name* is a
   label rather than an argument. **Chapter 6 is on this list vacuously**: its
   only inline-test block, `task_tree.rs` 1016–1105, is the test module's opening
   — a section comment, four *open-then-call* compositions and five fixtures —
   and carries **no** `#[test]` function, so there is no reproduced test for the
   obligation to attach to. The chapter states the vacancy on the page rather
   than leaving it silent, so a reviewer checking the list against the mapping
   can tell vacuous from omitted. No other Part II chapter is in that position:
   `task_tree.rs`'s sixty-three tests are nineteen in `pick-tests` (chapter 7),
   twenty-two in `brief-chain-and-kind-tests` and one in
   `pick-with-brief-chain-tests` (chapter 8), and twenty-one in `resolve-tests`
   (chapter 9).
2. **Supply the argument — `driver_lease.rs` 1–819** (chapter 16). Per mechanism:
   the line that enforces it, the failure it prevents, and the record clause it
   keeps. This is the same instruction `keyed-launch`'s chapters 3–5 carry for
   `templates.rs` at 13%, and for the same reason: it is where the rules bind and
   the source is silent.
3. **Do not restate — the production blocks at 41–73%** (chapters 1, 5, 10, 15,
   18, 19, and the production halves of 2, 3, 4, 6–14 and 20). The comments
   already argue and the fragment graph quotes them verbatim on the page. Prose
   there connects arguments across items, names the test, and stops.

**The risk closed** is a book that is thorough where the source already speaks
and silent across the 38% where it does not. It is the same failure
`keyed-launch`'s brief closed, in the same direction, at four times the scale.

**Two obligations were rejected.** *Name the owning side at every fragment* turns
the spine into a per-fragment duty, but the spine already does it once per
chapter and the outcome once per part; at fragment granularity it is repetition,
and it is nearly content-free across chapters 16–20 where there is no seam
underneath at all. *Hold the records to the code* would make
`architecture-residue-k75`'s deletion a check rather than a judgement, but it is
roughly half-discharged by comments that cite records already and reproduce those
citations verbatim, and the book **cannot cite** most of its records — the link
contract closes its targets to its own pages, its own roots, the guide and the
glossary — so it would be discharged in prose no link-checker can verify. The
coverage duty k75 depends on is held by the chapter sequence instead.

## Worked examples

**The carried example is one grove's whole life, in the names the code itself
writes.** Its anchor is not invented: `DEFAULT_ROOT_SLUG` is `"plan"`
(`tree_lifecycle.rs` line 56) and `requirements` is a reserved kind
(`task_name.rs` line 246), so `root_init` writes exactly

    01-requirements--plan-k1.md

That same literal appears nine times in the crate's inline tests, and it is the
first entry of the tree this book was written inside. The default the code
produces, the name the tests use and a real grove agree, which is what makes the
example checkable rather than illustrative.

The example is told **strictly from the crate's side**: the book says what a
verb was given and what it wrote, and never what the session that invoked it was
for. A reader who has driven a grove supplies that themselves; a reader who has
not is not this book's.

| Chapter | Anchor | Starts from | Observable end |
|---:|---|---|---|
| 1 | the crate's own map | `Cargo.toml` and `lib.rs` | the twelve verbs, five dependencies and one error, named |
| 2 | `01-requirements--plan-k1.md` | a directory listing | one of four verdicts, and a malformed name is not skipped |
| 3 | `plan-k1` | the name, peeled | the handle carries slug and key; `01-` is not in it |
| 4 | `01-requirements--plan-k1.md` | parsed | rendered back byte-identical, or refused as uncomputable |
| 5 | `<worktree>/.grove` | a caller with a worktree | a `Tree`, or a refusal naming what is absent |
| 6 | the leaf's path | an entry of a snapshot | one absolute path, built in exactly one place |
| 7 | the fresh grove | the tree as `root_init` left it | the first live leaf in walk order |
| 8 | the same leaf, nested | a leaf inside a node | its ancestors' briefs, root to leaf |
| 9 | `plan-k1`, `[1]`, `1`, `plan` | four spellings of one reference | one `Resolution`, or `Ambiguous` listing the keys |
| 10 | `leaf-add` | the grove after `root-init` | a sibling at the next position, its key predicted and checked |
| 11 | `root-init` | an empty worktree | `BRIEF.md` and one live leaf, as **one** store operation |
| 12 | `leaf-decompose` | a leaf that proved too big | a node directory, **key preserved**, brief seeded, first child grown |
| 13 | `leaf-retire`, `leaf-prune` | a live leaf | a `DONE-` or `ABANDONED-` infix; header and body untouched |
| 14 | `finish` | the last live leaf retired | the sentinel leaf, then no `.grove` at all |
| 15 | the twelve | a session mid-task | every verb returns the paths it wrote |
| 16 | the lease | bare `grove` in a worktree | one driver; a second exits naming the canonical worktree |
| 17 | the epoch | a driver replaced under a running one | admitted calls finish; new ones are refused |
| 18 | `config.kdl` and its delta | a repository with an untracked delta | templates for four slots — or a **tracked** delta refused |
| 19 | `${prompt}` | the selected leaf | three parts in the session's own timeline order |
| 20 | the loop | bare `grove` | relaunch with fresh context, or a stop that is resumable |
| 21 | — | the twenty chapters | the three questions, answered for the crate |

**The observable end of the carried example is that it ceases to exist.**
Chapter 14 is where the grove deletes itself, and that is what finishing *is*.
It is also why the book does not simply point the reader at `.grove/` and tell
them to look: `finish-commit` removes it, so a book illustrated from a live tree
would cite an artifact its own subject destroys.

**The second ending is a name grove refuses.** A task-*shaped* name that is not a
task name is rejected by grove, not by the store — the store would have accepted
the entry, and the grammar that refuses it is precisely the thing that did not
move. `pick_refuses_a_species_mismatch_at_a_task_shaped_name`,
`a_session_kind_that_is_not_a_token_is_malformed` and
`transition_does_not_scaffold_over_a_name_grove_refuses` are its three forms, and
it is the spine's sharpest case. As in both precedents, the refusal is the second
ending rather than the carried thread, so the happy path is not an aside.

**Three examples were rejected.** *The crate's own test fixtures verbatim* would
make every step a named passing test, but the fixtures are ad-hoc per test —
`a`/`b` for ordering, `design`/`build` for decomposition, `add` for outcomes —
built and discarded inside each test precisely because each isolates one
property, so twenty chapters would carry twenty unrelated trees. That is the
ground on which `keyed-launch`'s brief rejected the same shape; the tests remain
the book's evidence, cited chapter by chapter. *One session's life, driver-side*
is the crate's reason and carries chapters 15–20 better than anything else, but
reaches none of the 7,436 lines before them; it is chapter 20's thesis instead.
*This repository's own `.grove/`, quoted* is disqualified by chapter 14.

## Early uses the order forces

Four rows, all the price of concept order over file order, and all recorded in
the book's early-use ledger.

| Term | First used | Owned by | Why the order forces it |
|---|---:|---:|---|
| `TaskName`, `TaskNameError`, `Verdict`, and the `verdict` / `entry` / `malformed` support helpers | 2 | 4 | the classification and shape-refusal tests are *about* the four verdicts and the token rules, but the only way to reach a verdict is through the `EntryName` implementation |
| `TaskName`'s `Display` | 2 | 4 | the shape-refusal block's round-trip test asserts that each of two names renders back to its own bytes, which is a rendering two chapters before the renderer is read |
| `TaskName::compose` | 3 | 4 | `every_positioned_name_ends_in_its_own_handle` is the handle's structural claim, and it is asserted over names the test *builds* rather than parses |
| `entry_path` | 5 | 6 | `task_tree.rs`'s own module header names it as the one place paths are built, and chapter 5 reproduces that header |

**`compose` and `Display` were one row, first used in chapter 3, and that was
wrong about `Display`.** This brief and the manifest both had them as a single
family because the reasoning was about the *load-bearing* first use — the handle's
structural claim needs a rendered whole name. But chapter 2's `shape-refusal-tests`
block reproduces `crates/grove-loop/src/task_name.rs` lines 1,418 and 1,455, where
`a_multi_word_kind_beside_a_multi_word_slug_has_exactly_one_reading` calls
`to_string()` on a `TaskName` twice; in page order the first use is chapter 2.
The two symbols are not one family: `compose` is genuinely first reached in
chapter 3, at lines 1,562 to 1,579. A row exists so that **the earlier page states
the minimum locally** ([`walkthrough-books.md`](walkthrough-books.md), *Early-use
ledger*), so naming a page later than the real first use leaves the earlier page's
obligation unrecorded — which is what the split fixes. `check_early_uses` never
noticed, because it checks only that a named first use is *before* its owner, never
that it is the *earliest*; nothing was red either way.

**When a reproduced block owes a row: named or exercised, and not already
covered.** The specification's trigger has two halves — a page that *first uses*
a later-owned symbol, **or** one that *reproduces source bytes whose referent*
belongs to a later slice — so naming a symbol in a reproduced doc comment owes a
row exactly as calling it does. That is why `entry_path`'s row above is anchored
on the module header that names it rather than on the chapter that reads it. The
exception is a symbol an existing row already states the minimum for: chapter 2
reproduces `task_name.rs` 66–70, which says *both of `TaskName`'s renderings end
in a call to* `Handle::render`, and that needs no row of its own because the
chapter-1 cast row for `TaskName` already says it *renders back to the bytes it
was parsed from*. `TaskName`'s `Display` is owed a row for the `to_string()`
calls that follow, not for the header that describes it — which is why its anchor
is `#refusals-inside-the-shape` and not `#the-handle-in-this-grammar`.

**The ledger is a floor, not the set.** Any further block reproduced in one
chapter that names or calls something a later chapter owns takes a row of its
own, and the authoring leaf adds it rather than treating this table as closed.

Chapter 1 additionally names nearly every public item before its owner explains
it — `read`, `write`, `Reading`, `Writing`, `Tree`, `TreeWrite`, `Vacancy`,
`Sought`, `Selection`, `Error`, `interpret`, `Disposition`,
`admit_ambient_session`, `DriverLease`, `SessionEpochGuard`, `verbs::root_init`.
That is one block naming the cast at low resolution, not a per-term forward
reference, and it is the same shape all four preceding orientation chapters took.

## Outbound links

**The link contract closes a book's local targets to its own pages, its own
roots, [`docs/USAGE.md`](../USAGE.md) and [`CONTEXT.md`](../../CONTEXT.md).**
Decision records, `docs/specs/*` and `docs/CONFIGURATION.md` are **named in prose
and never cited**. That is stated here because a reviewer who does not know it
reads the absence of a link to
`docs/adr/one-live-driver-per-working-tree.md` from chapter 16 as an omission
rather than as the contract holding.

Twelve anchors are declared, and **every one exists today** as an explicit
`<a id="…"></a>` line immediately preceding a heading, so `book-check`'s `M201`
is green from the first slice and no promotion is owed.

**`[guide]` — `docs/USAGE.md`**

| Anchor | Section | Chapters |
|---|---|---|
| `usage-task-tree` | *The task tree and its filename grammar* | 2–4, 11–14 |
| `usage-tree-verbs` | *The `grove-llm` verbs over the tree* | 15 |
| `usage-driver-lease` | *One driver per working tree* | 16, 17 |
| `usage-finish` | *Finish* | 14 |

The `README.md`'s required guide citation is **`usage-task-tree`**: it is the
guide section that states this crate's subject in the reader's own terms, and it
is reserved by no other book.

**`[[glossary]]` — `CONTEXT.md`: all eight anchors that exist**

| Anchor | Chapters | Anchor | Chapters |
|---|---|---|---|
| `task-tree-scheme` | 2, 3, 4, 11, 12, 13 | `guaranteed-core` | 19 |
| `tree-access-lock` | 5 | `stated-vcs` | 19, 20 |
| `driver-lease` | 16, 17 | `loop-control-channel` | 15, 20 |
| `session-epoch` | 16, 17 | `task-commit-boundary` | 14, 15 |

All eight are over vocabulary this crate owns. `CONTEXT.md`'s own ownership table
assigns `grove-loop` twelve terms — *Session kind*, *Work-item handle*,
*Position*, *Permanent key*, *Leaf*, *Node directory*, *Brief chain*,
*Selection*, *Driver lease*, *Session epoch*, *Guaranteed core*, *Stated VCS* —
and `grove-llm`'s book already declares seven of these eight on weaker grounds.

**No anchor is promoted, and the reason is a fact rather than a preference.**
`task-tree-scheme` is a **grouping** heading (`CONTEXT.md` lines 769–770)
introducing a run of term entries rather than one term — `glossary-anchors-k62`'s
own context names it as one of the document's two such groupings — so a citation
of it already lands the reader in the passage that defines *Node directory*,
*Session kind*, *Work-item handle* and the rest of the tree vocabulary. Chapters
2–4 need no target that does not exist. Promoting `session-kind` and
`work-item-handle` into anchored `###` headings is permitted — `glossary-anchors-k62`
says explicitly that *a book reserving a new term simply anchors that term as
`orientation-k55` did* — but it would put a `CONTEXT.md` restructuring on the
book leaf for granularity the book does not need, and k62 is placed after every
book precisely so it can do that uniformly against the full reserved set.

Two guide anchors were considered and dropped: `usage-running-grove` and
`usage-session-lifecycle` are `keyed-launch`'s and the overview's, and they
describe the operator's experience of the loop — the overview's subject and the
guide's — rather than anything this crate owns that the four above do not already
reach.

## The book's row in the ownership table

One edit outside the book is owed, and it is `grove-loop-book-k37`'s, as each
precedent row was its own book leaf's. `docs/ARCHITECTURE.md`'s *Documentation
ownership* table carries rows for the other five books and none for this one;
`every_book_root_has_a_documentation_ownership_row` goes red until it does.

The row follows the five existing ones in form:

```text
| `grove-loop` source, read page by page | [`walkthroughs/grove-loop/README.md`](walkthroughs/grove-loop/README.md) — the code walkthrough: a book whose every chapter opens on what the crate kept when the domain-free crates took the rest, and whose fragments reconstruct every byte of the crate's frozen corpus |
```

Nothing else outside the book is owed. The subject inventory row
(`grove-loop | crates/grove-loop`) and the corpus exception row
(`crates/grove-loop/src/task_grow/tests.rs`, class `inline-test-module`) both
already exist in [`walkthrough-books.md`](walkthrough-books.md), so
`every_books_subject_is_exactly_the_specifications_inventory` and
`every_books_corpus_exceptions_are_exactly_the_specifications_inventory` are
green with **no specification edit** — provided the manifest declares that one
exclusion and no other.

**Declaring no other exclusion is what puts the corpus where it is.** The five
inline `#[cfg(test)] mod tests` blocks inside `task_name.rs`, `task_tree.rs`,
`tree_lifecycle.rs`, `driver_lease.rs` and `loop_driver.rs` are in the corpus
because the inventory has no row for them. The chapter cut follows from that
row's absence; it was not chosen around it.

## What this book makes redundant

This book absorbs no prose. What it does is make `docs/ARCHITECTURE.md`'s
crate-internal description of `grove-loop` redundant, which is the condition
`architecture-residue-k75` deletes it under. The thirty-one markers naming this
crate are the coverage obligation stated as a checklist, and every one has a
chapter:

| Markers at | Subject | Chapter |
|---|---|---:|
| 311, 340, 350, 363 | the tree grammar and its diagram; the kind token and the `--` separator; malformed against foreign; one classification shared by every verb | 2–4 |
| 333 | positions, keys, handles and the terminal infixes | 3, 13 |
| 371 | refusal by name | 4 |
| 379 | which module owns which half of the tree | 1 |
| 857, 873, 905 | the lock's scope and holders; what it does and does not promise; the contention probe and `restate` | 5 |
| 969 | `addressable_key`'s refusal, and `leaf-prune` on a node | 6, 13 |
| 433 | the walk and the finish-reservation rule | 7, 14 |
| 451 | the one pick and what it serves | 20 |
| 323 | what a node is, and how a missing `BRIEF.md` is read | 8, 12 |
| 779 | `resolve` on a chained stem, and the grow verbs' refusal | 9, 10 |
| 884, 1007, 1053 | `leaf-add`'s all-or-nothing on error; the lint's second opening; key prediction and its check | 10 |
| 1080, 1176, 1217 | the two calls of `root-init`; the one-store-operation scaffold; `root_shape` on a withdrawn layout | 11, 14 |
| 706 | the two kind tokens grove writes itself | 3, 11, 14 |
| 1163 | the transition table | 11, 14 |
| 739, 1242, 1265 | the finish reservation; the finish flow; the four teardown steps and the two undo commands | 14 |
| 461, 472, 1343 | the core's three parts; what the core reads; the stated VCS in `${prompt}` | 19 |
| 1190, 1325 | the watch and the escalation; the scrub inside the seam and the loop's complementary list | 20 |

Two of these are **joint** and their other books are written: line 1190 also
names `keyed-launch`, line 1325 also names `jj-workspace`. Line 472 is joint with
`none`. So this book is the last condition on all thirty-one.

**Cite the marker's subject, not its line.** The numbers above locate the markers
in `docs/ARCHITECTURE.md` as it stands and move whenever anything above them
does; the subject column is the stable identifier, and it is what a page should
name.

**The book neither cites nor edits `docs/ARCHITECTURE.md`.** Anchors stay where
they are; k75 does the deletion; the book's job is to make the deletion true.

## What the book deliberately does not cover

### The three domain-free crates' internals

`ordinal-fs-tree`, `keyed-launch` and `jj-workspace` each have a book of their
own, and this book cannot link to any of them. Where `grove-loop` calls one, the
book says what grove asked for and what came back, names the crate in prose, and
stops. This is the spine's own discipline: the seam is interesting from *this*
side, and the other side is another book's subject.

### `crates/grove` and `crates/grove-llm`

The binaries are the overview's and `grove-llm`'s. This book covers the library
they call and never the parsing or rendering above it — including
`report_insert`, which `verbs.rs`'s own comment names as the owner of a decision
this crate declines to make.

### The methodology

*What a `requirements` session is for*, *when a leaf should decompose*, *which
kinds are human-in-the-loop* — none of it is here. The crate knows a kind is a
token and that two are reserved; everything else about kinds is the plugin's and
`docs/USAGE.md`'s. Chapter 19 states the boundary exactly, because the too-late
test is where the crate draws it.

### The `tests/` directory

`crates/grove-loop/tests/` is 3,442 lines over five files and is **evidence, not
roots**. Its tests are cited by name throughout — `the_library_imposes_only_libc`,
`distinct_worktrees_hold_independent_leases`,
`grove_llm_admits_only_the_live_epoch_while_version_remains_exempt`,
`the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one` — and none of
it is reproduced. `src/task_grow/tests.rs` is treated the same way for the same
reason, though it reaches the book through the corpus exception inventory rather
than through the `tests/` rule.

### Rust, `libc`, and the system calls by name

`flock(2)`, `O_CLOEXEC`, process groups and signal delivery are used and named,
not taught. The reader knows Rust; where a system call's *semantics* carry an
argument — that two open file descriptions on one directory do not share a lock,
which is why grove's own guard and the library's cannot be nested — the book
states that consequence and cites the comment that makes it.

### Non-Unix platforms

The crate is Unix-only by construction and says so in its imports. The book does
not speculate about what a port would need.

### `docs/CONFIGURATION.md`'s account of the schema

Chapter 18 owns *whose file and whether*, not what a template *is*. The KDL
grammar, the slot rules and the diagnostics are `keyed-launch`'s book's, and the
operator-facing account is the configuration document's; both are named in prose
and neither is cited.

### The decision records themselves

Decisions 1 and 9 of [`module-decomposition.md`](module-decomposition.md), and
the ADRs `one-live-driver-per-working-tree`, `complete-session-configuration`,
`the-launched-child-is-a-job`, `grove-does-not-stage-its-own-renames` and
`entries-are-never-removed`, are named at the chapters that keep them and cited
nowhere, under the link contract above.

## Known in advance: the claims this book adjudicates

Five, of which two were found while this brief was written and three while the
book was drafted. Each has a leaf that fixes it, and every one of those leaves
sat **after** `grove-loop-book-k37` in the node — which is correct under the
corpus-freeze rule and meant the book had to adjudicate rather than wait.
All five have since landed and none of the five claims is left in the corpus.

**This is not the book's whole ledger of judged claims, and does not try to be.**
It counts the claims a leaf has corrected **at source**. The ones the book
adjudicates and leaves standing are recorded on their own pages instead, and
this list has never enumerated them. The exhaustiveness below is over this list,
not over those. Entry 5 was on the other side of that
line until `canonicalisation-sites-k149` landed, and joined this list when it
did.
The rule they were written under still governs any claim found later: an
adjudicating paragraph sits beside the fragment that reproduces a standing claim,
the claim is never repeated as though true, and it is never silently corrected,
since the fragment reproduces the bytes as they are. When the fix lands, that
paragraph is rewritten in the same commit as the comment — into an explanation of
the corrected wording rather than a deletion, because the reason a sentence is
phrased oddly outlives the defect that forced it.

### 1 · *Every member takes `version.workspace = true`* — chapter 1, corrected

`src/lib.rs` lines 67–68, in `VERSION`'s doc comment. It was false:
`crates/book-validation/Cargo.toml` sets `version = "0.1.0"` and is a workspace
member, so six of seven members inherit. `every-member-version-comment-k84`
landed the fix, taking the wording with no universal quantifier over a set that
is about to change — the root brief earmarks `book-validation` to leave — so the
comment now reads *every crate an operator installs takes `version.workspace =
true`*, which the manifests bear out.

**The chapter no longer adjudicates.** Its paragraph explains why the comment
quantifies over the installed crates rather than the members, names
`book-validation` as the member that does not inherit and why, and keeps the
point the adjudication carried: the invariant is what makes `VERSION` the only
version an operator can install. The identical clause is at
`crates/grove/src/cli.rs` line 12 and the overview's chapter 2 reads it, so the
two pages stay uniform. Both paragraphs, both comments and the workspace root's
own comment moved in k84's single commit.

### 2 · *The loop re-reads the configuration once per iteration* — chapters 18 and 20, corrected

`src/session_config.rs` line 89, in `TemplateSource`'s doc comment. It was false:
`src/loop_driver.rs` calls `templates.load(&delta_roots)` **twice** per iteration
— line 241 before `transition_to_current`, so the just-in-time presence rule for
the finish leaf is asked against the document as it stood *before* the tree was
mutated, and line 260 after the leaf is selected, so the launch expands the
selected kind's template from the document as it stands. The type's own doc
comment named both reads in the same sentence that said *once*, so the count was
stale rather than the design. `template-source-read-count-k86` landed the fix
inside the comment's frozen seven-line span, so no ownership range moved: the
sentence now reads *twice per iteration* and attaches each of its two clauses to
the read it belongs to.

**Neither chapter adjudicates any longer.** Chapter 18 owns the sentence and
explains why the corrected wording enumerates rather than asserts a single
consequence; chapter 20 owns the two calls and shows the count against the bytes,
which is the evidence the fix was made from. That the claim and its refutation
were **both inside this book's corpus** is why the defect could be settled
without citing another crate's page — the same property claim 3 has, and the
reason both were fixable from evidence the book itself reproduces. The outcome's
third question, *what does this layer choose that nothing beneath it could
default*, is exactly the question the two reads answer. The overview never
repeated the stale count; its ledger row and chapter 3 were amended at
`three-steps-k79`, so nothing there changed when the comment did.

### 3 · *`libc` is the probe in `task_tree`* and *`keyed-launch` is reached by exactly one verb* — chapter 1, corrected

`Cargo.toml`'s dependency comment, found at `orientation-k124` rather than while
this brief was written, which is why the brief above once recorded that no third
claim existed. Two clauses described the crate as it stood before
`loop-crate-driver-k22` moved the driver into it, and they were wrong in two
different ways.

The `libc` clause named one user where there are three: the probe in `task_tree`,
`driver_lease` (the lease's own `flock(2)` and the `fcntl(2)` close-on-exec
calls) and `loop_driver` (`isatty`, `tcgetpgrp` and `signal`). The one it named
is not the largest — outside the test modules `driver_lease` reaches `libc` on
thirteen lines, more than the other two together. The `keyed-launch` clause was **true of the verbs and false
of the crate**: `complete` is still the only one of the twelve verbs that
reaches the runner, but `driver_lease`, `loop_driver`, `session_config` and
`src/lib.rs`'s `reraise` re-export reach it too.

`manifest-dependency-clauses-k133` landed both fixes, and the distinction is what
the correction turns on: the clause now says the **verb surface** reaches the
runner once *and* that the crate reaches it in four more places, rather than
letting the first stand for the second. Deleting the clauses was ruled out —
the dependency argument is chapter 1's evidence for what the crate imposes, so
what was wrong was their scope, not their existence.

**The chapter no longer adjudicates.** Its paragraph explains why the two clauses
are not parallel, and keeps what the adjudication carried: that both attributions
were true of a smaller crate and the driver's arrival widened them. Chapter 5,
which owns the probe, cross-references chapter 1 rather than restating the
manifest.

### 4 · *`<worktree>/.grove`, spelled in exactly one place* — chapter 1, corrected

`src/lib.rs` in two places: line 268, the doc comment on `grove_root`, and lines
28–30 of the module header, which said that putting the join there *means no
caller can spell it a second way*. Found at `opening-k141`, chapter 5's draft,
whose own figure had repeated the claim; that page could stop repeating it but
owns none of these bytes, and adjudication sits beside the fragment that
reproduces a claim, so both halves fell to chapter 1.

`grep -rn 'join(".grove")' crates/*/src/` finds seven production sites. Three
besides `grove_root` are in this crate and each open the tree for themselves:
`tree_lifecycle.rs` line 76 (`transition_to_current`) and `driver.rs` line 55
(`materialize_finish`) are the driver's two tree operations, which run before it
has an opening to give them, and `tree_lifecycle.rs` line 197 is the session verb
`finish_commit`, which takes a `&Workspace` rather than a worktree. The other
three are in `crates/grove-llm/src/cli.rs`, at lines 505, 570 and 881, and are
not openings at all: they spell the root to **name** it in output, because the
library returns no path for a root it did not open. A consumer outside the crate
spelling `.grove` three times is the plainest refutation of *no caller*.

`grove-root-join-clauses-k148` landed the fix, rewording both comments inside
their own line counts so no ownership range moved and no ledger row changed.
The chapter explains a join scoped to the two public openings instead of
adjudicating one claimed for the crate, and it records what the class still
lacks: the leaf **decided against** a test over `join(".grove")` call sites,
because `the_librarys_tree_lock_is_taken_from_exactly_one_module` pins a
deadlock where a join-site count pins tidiness, and would go red on a new
opening that is correct. That decision binds this claim only;
`canonicalisation-sites-k149` judged the same question for its own and reached
the same answer by a second route — entry 5.

### 5 · *Canonicalisation appears once, in `leaf_entry`* — chapters 5, 6 and 8, corrected

`src/task_tree.rs` lines 27–29, in the module header. Found at `paths-k142`,
chapter 6's draft, which owns `target` and therefore owns the counterexample: the
claim is refuted from inside the same file, since `target`'s own doc comment says
it canonicalises *exactly as `leaf_entry` does*. Chapter 5 reproduces the header
bytes and chapter 6 adjudicated the claim beside the fragment that reproduces
`target`, which is where the account sits. Chapter 8, which owns `leaf_entry`,
carried an adjudicating paragraph of its own pointing at chapter 6's, so the fix
rewrote three pages rather than two.

`grep -rn 'canonicalize' crates/grove-loop/src/` finds eight call sites. Six are
production and all six are in this file — 346, 349 and 369 inside `target`, and
712, 715 and 734 inside `leaf_entry`, each function canonicalising the candidate,
the grove root and every walked entry's built path. The remaining two are
assertions inside `driver_lease.rs`'s `#[cfg(test)]` module, which opens at line
820. The second half of the clause named the operation both functions perform —
comparing, never reporting — and survives. Its noun did not: it compared *a
caller's spelling of a leaf*, while `target` resolves a node directory or the
grove root too, so the corrected clause says *a caller's path*.

`canonicalisation-sites-k149` landed the fix, rewording the header inside its own
line count so no ownership range moved: seven lines before and seven after, and
`task_tree.rs` still 2,023. The corrected clause is scoped to the module and
states a property rather than a count — *canonicalisation happens only where a
caller's path is resolved to an entry, in `target` and in `leaf_entry`, nowhere
else here* — because `existing_path` also interprets a caller's path argument and
deliberately canonicalises nothing, and because the grove root reaches this module
already canonical from `Workspace::resolve`. The leaf **decided against** a test
here too, for entry 4's reason and one more: the property worth pinning is
behavioural — *a reported path keeps the caller's spelling* — and no verb can
assert it, since the root is canonical before `task_tree` sees it, which is why
`crates/grove-llm/tests/resolve.rs` compares through `canonicalize` and says so.

**And `grove-llm-dependency-comments-k102` is not a sixth.** It reaches
`crates/grove-loop/src/lib.rs` line 81 only as *evidence* — the `Workspace`
re-export — and changes nothing in this corpus.

## What this brief does not settle

- **The prose itself.** This is structure and emphasis; drafting is
  `grove-loop-book-k37`'s, through `draft`, `copy-edit`, `art` and `proof`.
- **Fragment granularity inside a block.** How a 569-line test block is broken
  into fragments is the draft's, subject to the validator's reconstruction
  requirement and the early-use rule above.
- **The concept index and source index.** Both are required by
  [`walkthrough-books.md`](walkthrough-books.md) and generated against the
  finished pages.
- **Where the draft's own node boundaries fall.** The five parts are the intended
  groupings, but `grove-loop-book-k37` opens the node and decides how many
  sessions each stage takes.

### Stated limits

Nothing mechanical checks the spine, a worked example's start and end, or the
per-chapter prose obligation. `book-check` proves structure and reconstruction;
the repository tests prove links, subject, exceptions and the ownership row.
The crate's 3,442-line integration suite and the 3,984 lines of inline tests are
the book's **evidence**, not its gate — which is why the third prose obligation
exists and why chapter 21 is the page a reviewer should be spent on.
