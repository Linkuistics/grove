# What order holds
<!-- book-page id="what-order-holds" slice="assembly" order="7" -->
[Previous: Leaving the loop](06-leaving-the-loop.md) | [Contents](README.md)

<a id="assembly"></a>
## Twelve verbs, one table

<!-- rollup «owned-lines-total» -->
This chapter owns no production source. The four roots and 915 lines are
already reconstructed by the fragment graph the six chapters before it built,
and the [source index](source-index.md) records that graph in full. The
remaining work is what no single chapter could state, because each read one
family of verbs and stopped at its own rule: the twelve verbs side by side, the
two orders set against each other, and whether the test this book promised its
reader holds when it is applied back to the binary that taught it.

The chapter has no worked example. The carried session ran every verb it runs to
its ending in the five chapters that own them, and nothing here changes a value
in that trace; the catalogue below is therefore the page rather than a section
that had to wait behind an example. The page's figures are assemblies. The first
is the twelve verbs, one row each, in the order the `Command` enum declares them.
The second is the two orders, with what reversing each would cost and what
would catch it. The third is the stream contract, which is the rendering half of
the same thesis. The fourth lists the boundary: what the six chapters named and
did not explain. The two ledgers are then closed and the final
validation is recorded.

The table's five question columns are the five things this book has been asking
one family at a time. *Text* is what the handler reads with the type that owns the
grammar, before any lock; *opening* is which of the two the handler takes, or
neither; the two stream columns are what a caller parses and what a caller
reads; and the last column is the evidence, which for this crate lives outside
the corpus in a test directory ten times its size.

| Verb | Text read by its own type | Opening | stdout | stderr | Evidence |
|---|---|---|---|---|---|
| `root-init` | `Slug`; the kind is fixed at `requirements` and read from no argument | exclusive, and only over a **vacancy** — a live grove is refused by the shape, not by a check | the charter's path, then the first leaf's | nothing; *grove root already exists* is a refusal, not an answer | `root_init_creates_root_brief_and_first_requirements_leaf`, `root_init_refuses_when_grove_already_exists`, `root_init_rejects_invalid_slug_without_creating_grove` |
| `pick` | none | shared | the next live leaf's path | the *no live leaves* line, when there is none | `picks_first_live_leaf_in_numeric_order`, `fully_retired_grove_prints_diagnostic_and_exits_zero`; the `ABANDONED` skip is measured, with no test in this crate |
| `brief-chain` | none; the optional path is normalised, not parsed | shared | one absolute node-file path per line, root first | the same *no live leaves* line, in the no-argument form | `leaf_two_levels_deep_returns_root_and_ancestor_node_briefs`, `missing_intermediate_node_file_refuses`, `missing_root_node_file_refuses`; the diagnostic is established by the handler alone |
| `kind` | none; the optional path is normalised | shared | the kind label, one lowercase token | the same line again | `every_shipped_kind_round_trips_through_the_verb`, `empty_grove_prints_no_live_leaves_on_stderr_and_exits_zero` |
| `resolve` | `Reference` | shared | the entry's path, or the grove root for `.` | the retired or abandoned note, the not-found line, or the ambiguity listing | `resolve_by_key_bracketed_and_bare`, `resolve_not_found_exits_zero_with_diagnostic`, `resolve_ambiguous_slug_lists_keys_on_stderr`, `resolve_dot_prints_the_grove_root`; six tests in `resolve_rendering.rs` establish the rendering directly through `render_resolution` |
| `leaf-add` | `Kind` per `--kind`, then `Slug`, then `Reference` | exclusive | each leaf's path in position order, printed after the run landed | nothing | `a_kind_list_lands_three_flat_siblings_at_consecutive_positions_and_keys`, `a_failed_run_prints_no_path_at_all`, `add_rejects_a_kind_that_is_not_a_token_and_names_the_character` |
| `leaf-insert` | `Kind`, then `Slug`, then `Reference` | exclusive | the new leaf's path | the renumber summary and then the cross-reference lint — or, with no sibling to renumber, *no siblings to renumber* alone and neither | `insert_at_start_shifts_root_siblings_up_by_one`, `insert_cascades_a_node_subtree_with_position_free_headers`, `insert_requires_an_existing_target` |
| `leaf-decompose` | `Kind` when `--kind` is given, then `Slug`; the leaf path is normalised | exclusive | the node's `_<slug>.md`, then the first child's path | nothing | `decompose_with_no_kind_flag_gives_the_first_child_the_parent_leafs_kind`, `decompose_kind_flag_overrides_the_parent_leafs_kind`, `decompose_rejects_a_brief` |
| `leaf-retire` | none; the path is normalised | exclusive | the renamed path | the two steps that remain | `retire_adds_done_infix_in_place`, `retire_names_the_remaining_steps_on_stderr`, `retiring_a_reviewed_producer_changes_only_its_own_filename` |
| `leaf-prune` | none; the path is normalised | exclusive | every marked path, one per line | *nothing live to mark* when it marked none, the untouched `DONE` leaves when there were any — the two are independent — and the two steps last when something was marked | `pruning_a_node_marks_every_leaf_the_same_way`, `prune_of_a_node_reminds_once_for_the_whole_bulk_mark`, `prune_that_marks_nothing_stays_quiet` |
| `finish-commit` | `Handle`, with a canonical positive key | none in the handler; the call takes the exclusive opening and holds it through the deletion | nothing | `finish-commit <handle>: committed as <change id>` | `a_noncanonical_key_spelling_refuses_without_mutation`, `finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, `native_jj_finish_commit_records_only_the_teardown`; the stderr line is established by source alone, with no test in this crate |
| `record-teardown` | none; ambient launch directory | none; checks the exact admitted directory before recording an absent tree | nothing | recorded path, or explicit no-loop message | `teardown_refuses_every_existing_grove_shape_and_is_idempotent_after_removal`, `no_launch_records_nothing_even_outside_a_workspace`, `a_record_cannot_follow_a_link_or_replace_a_directory` |

Four things the table says that no chapter could, because each is a statement
about the twelve rather than about a family. They are read down the columns.

**Six verbs read operator text with a type, and six read none.** `resolve`,
`leaf-add`, `leaf-insert`, `leaf-decompose` and `finish-commit` hand a string to
`Reference::parse`, `Kind::new`, `Slug::new` or `Handle::parse` and get a refusal
or a value; `root-init` is the sixth and reads one `Slug` and nothing else,
because its kind is a literal in the handler. The other six take a path or
nothing, and a path is *normalised* rather than parsed — `normalize_leaf_path`
makes a cwd-relative spelling absolute and passes anything else through for the
verb to join, which is not a grammar decision and refuses nothing. That is why
the first order is stated on one chapter's page and not on six: only where there
is text with a grammar is there an order to get wrong.

**Two openings, three ways of taking them, and two verbs that take neither.**
Four reading verbs take the shared opening; five mutating verbs take the
exclusive one; and `root-init` takes the exclusive opening and requires that it
come back a *vacancy*, so the refusal to clobber a live grove is the shape of
the answer rather than a check the handler makes. No handler takes both.
`leaf-decompose` did, while it had a kind to ask about before it wrote, and
*Growing the tree* records what went with that read.
`finish-commit` takes neither and leaves the opening to the call, which needs it
for a deletion the handler never sees. `record-teardown` also takes no tree
opening: it resolves the workspace only when the ambient launch directory is
present, checks that directory against the admitted guard, and delegates the
absent-root check and record creation.

**Ten verbs put something on stdout and two put nothing there.**
`finish-commit` and `record-teardown` print only advice on stderr. Dispatch exit
is a separate command and does not contribute a Grove verb to this count.

**Three of the six verbs that write to the tree are silent on stderr when they
succeed** — `root-init`, `leaf-add` and `leaf-decompose`. The paths each wrote
are its whole report. `leaf-insert` is not silent, because its renumber has consequences elsewhere in
the tree the caller has to be told about, and the two terminal marks are not
silent because a session still has two steps to run; the difference is what each
verb does to the tree. The four reading verbs are silent on stderr as well
whenever they find what was asked for — for them an absent answer is the only
thing there is to report.

<a id="the-two-orders"></a>
## The two orders, and what reversing each would cost

The book's promised outcome is a test, and its first half is the question this
page's table answers for one binary: of everything this command surface does,
what is left that is not rendering? The answer is two orders, and the second
half of the test is the part a reader can only do with the table in front of
them — for each order, say what reversing it costs, and name the check that
would catch the reversal. A cost with no check behind it is a belief; a check
with no stated cost is a test nobody can grade.

| # | The order | Where it happens | What reversing it would cost | What would catch it |
|---:|---|---|---|---|
| 1 | the operator's text is read by the type that owns it, before a lock is taken | the six handlers that read text with a type: `cmd_root_init`, `cmd_resolve`, `cmd_leaf_add`, `cmd_leaf_insert`, `cmd_leaf_decompose`, `cmd_finish_commit` | a lock over the whole grove taken, and waited for, in order to refuse a typo — the exclusive one for five of the six, the shared one for `resolve`. Nothing is corrupted, because the refusal still comes before the call. But a refusal that needs no tree queues behind whatever holds it, and a stalled process on the working tree turns a malformed command into a hang | **no test does.** The refusal tests — `root_init_rejects_invalid_slug_without_creating_grove`, `add_rejects_a_kind_that_is_not_a_token_and_names_the_character`, `a_noncanonical_key_spelling_refuses_without_mutation` — require the tree untouched after a refusal, and that holds in either order. What holds this one is the source, where every parse stands above the opening, and the measurement below |
| 2 | the session is admitted against the launch directory before recording teardown | `cmd_record_teardown` | a teardown record written outside the launch this session was admitted for | source order and the shared guard held through the write; filesystem effects covered in `record_teardown.rs` |

The measurement is one run of the built binary on a scratch grove, while
another process held the exclusive lock on the working-tree root. A malformed
argument went to each of the six: a kind and then a slug that are not tokens to
`leaf-add`, a kind to `leaf-insert`, a slug to `leaf-decompose` and to
`root-init`, an empty reference to `resolve`, and a handle that is not one to
`finish-commit`. Every one was refused at once, exit `1`, with the grammar's own
message. The same `leaf-add` and `resolve` with well-formed arguments waited for
the lock until they were killed. So the refusals reached no opening, and the
run could tell a command that waits from one that does not.

The first order avoids waiting for a tree lock to refuse malformed text. The
second binds the teardown write to this admitted launch. Epoch invalidation
requires the exclusive lock, so the shared guard held across the handler orders
the record write before replacement. The lifecycle verb checks absent `.grove/`
independently of admission; either check alone would leave the other fact unknown.

**There were three orders until `dispatch-launch-k19`.** Between these two stood
a just-in-time presence rule: each of the four verbs that write a kind asked,
before the tree was opened for writing, whether that kind resolved to a launch
template in Grove's configuration, and refused if it did not. Grove has no
launch configuration now, and the header says so in the sentence after the one
that names the two orders left. A kind is written if it is a well-formed token.
Whether a session of that kind can run is decided by the owner's
harness-dispatch policy when the driver launches the leaf, and a refused launch
leaves the leaf live. The deletion moved a cost and did not remove it: a leaf of
a kind nothing routes now lands on disk and is picked, and the refusal arrives
at launch where it used to arrive before a byte moved.
`every_leaf_writing_verb_writes_any_kind_with_no_policy_installed` and
`old_configuration_files_refuse_no_tree_verb`
(`crates/grove-llm/tests/no_kind_admission.rs`) hold this binary's half, that
nothing is asked. The launch's half is the driver's, and outside this book.

Applied to a thin command surface of your own, the test has three steps.

1. **Ask what is left that is not rendering.** List everything the surface does
   between parsing and its call into the library. If the list is empty the
   surface is rendering and nothing else, which is a real answer and the one
   this crate's header claims for itself before qualifying it.
2. **Classify each survivor as an order, and name the two things it orders.**
   Text before a lock, a check before a mutation, an admission before a write:
   each is a pair, and naming both halves is what turns *it validates input*
   into a claim that can be reversed and therefore tested.
3. **For each order, name the cost of reversing it and the check that fails.**
   A compile error, a failing test, or a measurement written beside the code —
   and *nothing* is an answer, which says review alone enforces the order. Say
   which of the three you have; they are not interchangeable. A surface where
   review alone enforces every order is one refactor away from having none.

<a id="the-stream-contract"></a>
## The rendering half

The orders are what remains beyond rendering, and rendering is substantive.
It has a contract of its own, stated in the reader contract as four rules —
data on stdout, advice on stderr, exit zero for information, every refusal
carrying its remedy — and the six chapters met each of them one verb at a time.
The table is the contract as a partition of everything this binary writes. Its
last column is where the class was read in full. It shows that the exit status
is a function of the class and never of the verb.

| Class | Stream | Exit | What it is | Read in |
|---|---|---:|---|---|
| data | stdout | `0` | a path, a list of paths, or one kind token — the ten verbs that answer a caller | every chapter that owns a verb |
| an absent answer | stderr | `0` | the grove holds nothing to report: no live leaf, no entry matching a reference, or several entries where one was wanted | *Reading the tree* |
| advice beside a success | stderr | `0` | the two remaining steps, the renumber summary, the cross-reference lint, the untouched `DONE` leaves, the change id, the disposition the loop will act on | *Growing the tree*, *Ending work*, *Leaving the loop* |
| a refusal | stderr, behind `Error:` | `1` | a grove that is not there, a grove that already is, a token that is not one, a session in the wrong working tree | *The grammar and the openings*, *Growing the tree* |
| a usage error | stderr | `2` | clap's, before `run`'s first statement returns: a bare invocation, an unknown verb, a missing required argument | *The grammar and the openings* |

The line the whole contract turns on is the third row against the fourth, and it
is the one this binary's audience makes load-bearing. A session reading its own
stderr cannot afford *nothing found* and *I refuse* to look alike, because the
first is a fact about the grove and the second is a fact about the call — and the
binary keeps them apart with the exit status rather than with wording, which is
the only half of the distinction a caller can act on without parsing prose. Two
of the four rules are therefore about the same thing from opposite sides: *exit
zero for information* is what makes the absent answer safe to ignore, and *every
refusal carries its remedy* is what makes the refusal worth reading. `absent` is
the one wording behind the fourth row's first case, and it carries
`grove-llm root-init` in its second line for exactly that reason.


Missing node files are refusals, including a missing root file. The guarded
opening returns an error before `brief-chain` can print an incomplete result.
An empty live-work selection remains a normal answer on a valid tree.


The fifth row is clap's and not this module's, and it is in the table because a
caller cannot tell from the stream alone that it is: a usage error and a refusal
both arrive on stderr and both fail. The status separates them, and it does so
without a line of this module's help: `cli.rs` writes no exit status anywhere —
`main` returns a `Result`, the standard library turns an `Err` into `1`, and clap
exits `2` on its own before `run` ever returns. That is the whole of the
difference a script can see.

<a id="three-holders"></a>
## What the compiler holds, what order holds, what tests hold

*Thin* is the claim the file's header makes about itself, and the book has now
read every line behind it. Three different things hold it, they are not equally
strong, and the difference is the second half of what a reader takes away.

**The compiler enforces the boundary.** `grove-llm` is a package of its own rather
than a `[[bin]]` target declared inside `grove-loop`'s manifest, and Rust privacy
is drawn at the crate, so no line of this module can name a `pub(crate)` item of
either library it depends on. *Orientation* read the alternative that clause
holds for and the shape it does not — a binary target that compiles the library's
modules as its own can name them, one that merely depends on the library beside
it cannot — and what rules the first shape out here is the package boundary,
which none of this crate's three Rust files reaches past. The check is not a
discipline anyone keeps: an unpublished item is `E0603` at build time, before a
test is compiled, let alone run. Every `grove_loop::` name the module uses is a
`pub` item of that crate, and the one type it imports from `jj_workspace` is
published by both.

The package's own library target does not weaken that. `src/lib.rs` says why it
exists — a clap command tree is not something a spawned process can be asked
about, so the surface lives where this crate's tests can inspect it — and says
what it costs, which is nothing, because the code the binary must not reimplement
is in a different crate either way. The binary target and the library beside it
are one package, and the guarantee was never about that edge.

**Two orders govern what remains beyond rendering**, and the table above is
the whole of it. One test and one measurement support them. The test drives the
binary under a real driver and holds the second order. The first rests on where
the parses stand in six handlers, and on a measurement this page took once and
no suite repeats. That is weaker support than the boundary above, which the
compiler settles. For the first order it is also weaker than the conventions
below, each of which has a test that can fail.

**Tests verify three properties of the surface**, and each is a property no
compiler can state.

| Property | Asserted by | The change that breaks it | What fails |
|---|---|---|---|
| the verb surface is flat | `the_grove_llm_verb_surface_is_flat` | a subcommand nested under a verb | the test, naming the verb that grew children — and it exists to protect the next row's comparison, which compares bare names across two grains |
| every help surface describes everything it lists | `the_llm_facing_binary_describes_every_option_it_lists` | an argument or subcommand with no doc comment, or with an empty one | the test, printing the command path and the id; it walks clap's model rather than scraping rendered text, because a parser for two help layouts is likelier to be wrong than the thing it checks |
| the shipped methodology instructs no verb the CLI lacks | `the_shipped_methodology_instructs_no_verb_the_cli_lacks` | a verb removed from the CLI while the methodology still names it, or a new instruction for a verb that does not exist | the test, listing the offending mentions with their files and line numbers |

The third is the one worth reading closely, because of how its authors bounded
it. Its corpus is every Markdown file under every shipped
`plugins/*/skills` tree, including Linkuistics: narrowing it to Grove alone
misses instructions a sibling skill gives a Grove session. Widening the reader
caught the wrapped retired completion command in the doubt skill before that
instruction was repaired. A universal claim — *nothing instructed is missing* — is satisfied forever by
a scanner that has quietly stopped matching anything, so the test pins the
instructed set as an **equality** against a written list of ten rather than as a
floor: losing any instructed verb fails here, and so does gaining one nobody has
confirmed. It then runs its own classifier against a synthetic line naming a
constructor that was removed, and requires the classifier to see it. The cost is
a list to maintain and the purchase is that the check can fail; the file says so
in those terms.

Ten is not twelve, and the gap is the fact the row does not state. `root-init`
and `kind` are the two verbs the shipped methodology never instructs, and each
for its own reason. `root-init` creates a grove, which no session inside a loop
does: the driver scaffolds one for itself through the vacancy *The grammar and
the openings* read, so the verb's caller is whoever starts a grove rather than
anyone following the methodology. `kind` reports a leaf's kind, and its own help
says why nobody in the loop needs to ask — *the loop driver selects its own leaf
in-process, so nothing here routes a launch* — while a session was told its kind
in the mandate that launched it. Both are exposed and both are `--help`-visible;
neither appears in a sentence telling a session to run it. That is a property of
the shipped methodology rather than of this binary, and the test would fail if it
changed in either direction.

<a id="where-the-book-stops"></a>
## Where this book stops

**Everything behind `grove_loop::verbs` is named in this book and explained
nowhere in it.** That sentence is the boundary, and it is stated here once rather
than beside each thing a chapter had to name. A page could not be followed
otherwise — *Reading the tree* cannot read `cmd_pick` without the walk, and
*Growing the tree* cannot read `cmd_leaf_add` without the atomic run — so each
chapter named what it needed, gave the minimum a reader must hold, and explained
none of it. The table lists that boundary. Every row was needed for a claim
about *this* crate, and no row is a claim about the loop.

| Named as | Where it is named | What the page needed it for |
|---|---|---|
| The two openings of a grove, and the vacancy each answers | [*One working tree, two locks, one refusal*](02-the-grammar.md#the-openings) | Why `readable` and `writable` can refuse the same way, and why `root-init` refuses by shape |
| The pre-order walk that selects the first live leaf | [*The absent answer*](03-reading-the-tree.md#the-absent-answer) | What `pick` renders, and what `Nothing` means to a driver that never runs the verb |
| What a reference resolves to — an entry, the root, or an ambiguity | [*Worked example: one reference, three renderings*](03-reading-the-tree.md#worked-resolve) | The three shapes `render_resolution` is exhaustive over |
| The atomic run a `--kind` list lands or rolls back | [*`leaf-add`: an atomic run*](04-growing-the-tree.md#the-atomic-run) | Why `print_paths` runs after the call and never during it |
| The renumber, and the cross-reference scan that follows it | [*`leaf-insert`: the renumber, and the lint*](04-growing-the-tree.md#the-renumber-and-the-lint) | Why the hits come back as a value, and where the second opening would have deadlocked |
| The terminal marks, and what a node's prune leaves alone | [*`leaf-prune`: the HITL rule, the node case*](05-ending-work.md#the-node-case) | What the marked paths on stdout are, and which leaves the advisory reports |
| The [session epoch](../../../CONTEXT.md#session-epoch), the [driver lease](../../../CONTEXT.md#driver-lease), and admission | [*Admitted before dispatch*](02-the-grammar.md#admitted-before-dispatch) | Why `run` can refuse at the epoch-admission call, before any handler is chosen |
| The launch directory and dispatch job context | [*Two separate effects*](06-leaving-the-loop.md#admit-before-signal) | Grove admission and teardown recording; dispatch owns harness exit |
| The deletion and the path-scoped commit behind the finish sentinel | [*`cmd_finish_commit`*](06-leaving-the-loop.md#the-teardown) | What the change id on stderr records |
| Whether a kind can be launched at all | [*Text before lock*](04-growing-the-tree.md#before-the-lock) | Why no verb asks, and where the answer is given instead |

Each row but the last is a subject of `crates/grove-loop`, or of a crate the
loop reaches on this crate's behalf — the path-scoped commit is the
version-control seam's — and the account of each is that crate's own. The last
row is no Grove crate's: whether a kind can be launched is the owner's
harness-dispatch policy's to decide, when the driver launches a leaf of it. The
methodology those verbs serve is in
neither: it is the
guide's, at [the account of the verbs over the tree](../../USAGE.md#usage-tree-verbs),
and the review-chain and vendor-pair methodology in `leaf-add`'s doc comment is
reproduced here as corpus and explained only as far as the code keeps it.

One more thing lives in this crate's test directory and is named here once,
because a reader who has followed the last column of every table in this book
will otherwise conclude that the directory holds only evidence about the CLI. It
does not. `instructed_verbs.rs`, `composition_guidance.rs`,
`session_kind_guidance.rs` and `removed_surface.rs` compare the **shipped
methodology** against this binary — the verbs it instructs against the verbs that
exist, the kinds its examples spell against the grammar that parses them, the
flags its prose names against the flags the real verb carries, and the names it
still uses against the ones the codebase removed. Forty-three of the suite's tests
are in those four files. They are cited in this book wherever one holds a claim
about `cli.rs`, and they are not explained: their subject is the plugin and the
documentation, which are in no crate and therefore in no book.

<a id="the-closed-ledgers"></a>
## The closed ledgers

The book's two ledgers are complete, and for the first of them the closure is
mechanical rather than a claim this page makes.

<!-- rollup «ownership-blocks» -->
<!-- rollup «source-roots» -->
<!-- rollup «ownership-blocks-owned-by» of="one-call-plus-rendering" -->
<!-- rollup «ownership-blocks-not-owned-by» of="one-call-plus-rendering" -->
**Ownership.** 24 top-level blocks over 4 source roots, every one of
them `resolved`. *Orientation* created the whole ownership table at the start,
with its own 4 blocks resolved and the other 20 reserved by `defer`
directives; each later chapter replaced its own defers with inserts and turned
its own rows. No `defer` directive remains anywhere in the book, and none may:
`F003` reports any defer at all in final mode, so *every reservation has become
an insertion* is a statement the validator refuses to let be false rather than
one this page asserts.

<!-- rollup «early-use-rows» -->
<!-- rollup «early-use-rows-declared» -->
<!-- rollup «early-use-rows-at» of="01-orientation.md#the-imports" -->
<!-- rollup «early-use-rows-not-at» of="01-orientation.md#the-imports" -->
**Early use.** 13 rows, every one `explained`, and every one declared in
the manifest as well as in the ledger. The structure brief fixes all 13 and
the order forced no more. 9 have their first use at
[*The imports*](01-orientation.md#the-imports), where the import block names
every `grove-loop` type the binary reaches before any chapter says what it does
with one; the other 4 are the handler families, first named at
[*Worked example: one verb, three endings*](02-the-grammar.md#worked-dispatch)
by `run`'s exhaustive `match`, four chapters before the last of them is read.
Both costs are the price of two decisions the structure brief made — orientation
owns the import block, and chapter 2 owns `run` — and each row turned
`explained` in its owning chapter and in no other.

<!-- rollup «owned-lines-sequence» -->
<!-- rollup «source-owning-chapters» -->
**Owned source.** 105 + 119 + 215 + 309 + 101 + 66 = 915 lines across 6
chapters, and 0 for this one. The seventh row of that table exists to be zero:
a chapter that owns no source is the shape the structure brief chose for the
assembly, and the total is the 915 lines in the current authoritative corpus.

**The corpus's own claims.** No chapter corrected a comment it found wanting:
as these seven chapters were drafted, each was reproduced as written and
judged beside its fragment, because the corpus is frozen and a book may not
edit what it proves. Five were judged worth a source change, and four leaves
carried them. Two of the five were known before drafting began — the
manifest's
*everything this binary can reach is something `grove-loop` chose to publish*,
which holds for what is reached and not for what is reachable through the
direct `jj-workspace` dependency; and `lib.rs`'s *or `grove`*, naming a
dependency the manifest's own comment records as removed;
`grove-llm-dependency-comments-k102` carried both, together with a third
clause it rewrote in the same seven lines — the manifest's *a binary target
can reach its own library's private items*, which *Orientation* had read
against the one shape it holds for and which `bin-target-privacy-claim-k87`
had corrected everywhere else. The later source cutover moves completion out of this binary: the final
reminder names `harness-dispatch exit`, while `record-teardown` records Grove's
absent-root fact. Historical measurements below describe the earlier corpus;
the final checks at the end of this chapter verify the current four roots.

<!-- rollup «source-roots» -->
The [concept index](concept-index.md) and the [source index](source-index.md)
are the two lookup surfaces, and neither is part of the reading order. The source
index is the authoritative record of how the fragment graph reconstructs each of
the 4 files; the concept index is curated navigation into the arguments, and
makes no completeness claim.

<a id="final-verification"></a>
## Final verification

Three commands prove the book, and they prove different things. The first is the
only one that reads the corpus byte for byte.

```console
$ cargo run --quiet -p book-validation --bin book-check -- \
    --repo . --book docs/walkthroughs/grove-llm --final --check all
valid: 4 files, 915 resolved lines, 0 deferred lines, final=true
```

`--final` is what makes this different from every scoped run the six drafting
sessions made. In scoped mode a later chapter's range may be reserved by a defer
and counted as deferred rather than resolved; in final mode a defer is an error,
every source root must expand to its complete file, and the page inventory must
match the manifest exactly. 915 resolved and 0 deferred is the whole corpus
reconstructed.

The following gate and crate-suite transcripts are historical authoring evidence;
they precede the dispatch cutover. The current equality result above was rerun
at signal-contract-k18. Repository-wide verification is owned by its parent session.

```console
$ bash scripts/check.sh
...
=== book-check
  book-check docs/walkthroughs/grove-llm
valid: 4 files, 949 resolved lines, 0 deferred lines, final=true
...
  6 book(s) checked, 0 failing
  ✓ book-check

check: all 12 principal checks pass
```

The second command is the repository-wide gate, shown with the other five
books' lines elided. It runs
`book-check --final --check all` over every book root under `docs/walkthroughs/`
by discovery rather than from a list, which is why this book has been inside the
gate since *Orientation* created its directory, and why every drafting session
but this one left the script red on `book-check` alone: a prefix deliberately
leaves later blocks deferred, and `--final` will not have it. The script also
runs the repository's own tests, and three of those cover this book without
naming it — `every_repository_markdown_reference_resolves` sweeps every Markdown
file in the repository, so this page's links are checked with the rest;
`every_book_root_has_a_documentation_ownership_row` fails a book root with no row
in the *Documentation ownership* table of `docs/ARCHITECTURE.md`; and the
corpus-exception inventory in
`crates/grove/tests/corpus_exception_inventory.rs` requires this manifest's empty
exception set to match the specification's tables. All three are in the `grove`
crate's tests, not this one's.

```console
$ cargo test --locked -p grove-llm
     Running unittests src/lib.rs (target/debug/deps/grove_llm-166367deada00dd9)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running unittests src/main.rs (target/debug/deps/grove_llm-6a52075a099c4f73)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/brief_chain.rs (target/debug/deps/brief_chain-776efa9c74d5f807)
running 7 tests
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Historical completion tests (removed at signal-contract-k18)
running 7 tests
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/composition_guidance.rs (target/debug/deps/composition_guidance-fad4f6a14b20751f)
running 17 tests
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/composition_verbs.rs (target/debug/deps/composition_verbs-6bcc55dc9a61c686)
running 20 tests
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/finish_commit.rs (target/debug/deps/finish_commit-5888287447fe4f7c)
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/help_surfaces.rs (target/debug/deps/help_surfaces-6bdc56f8a62a50f4)
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/instructed_verbs.rs (target/debug/deps/instructed_verbs-bc793173f8fb184e)
running 2 tests
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/jj_tree_verbs.rs (target/debug/deps/jj_tree_verbs-a4fcd7ff9fc7e59c)
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/kind.rs (target/debug/deps/kind-aff3fa40f6d9c4b9)
running 16 tests
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/leaf.rs (target/debug/deps/leaf-47fbb5763c735ab8)
running 21 tests
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/leaf_ops.rs (target/debug/deps/leaf_ops-9c03767985ee85d5)
running 20 tests
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/llm_cli.rs (target/debug/deps/llm_cli-b5c737bbda819ecb)
running 2 tests
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/no_kind_admission.rs (target/debug/deps/no_kind_admission-d5a2fb45c273ffff)
running 2 tests
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/node_files.rs (target/debug/deps/node_files-4affaa3778f74de2)
running 3 tests
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/pick.rs (target/debug/deps/pick-6c8c2c7a70abc999)
running 9 tests
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/removed_surface.rs (target/debug/deps/removed_surface-9c885c4dcb258213)
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/resolve.rs (target/debug/deps/resolve-a484a54a5d220eb0)
running 13 tests
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/resolve_rendering.rs (target/debug/deps/resolve_rendering-c0a9a265558978d3)
running 6 tests
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/reviewed_producer_lifecycle.rs (target/debug/deps/reviewed_producer_lifecycle-4eca9a7d57068467)
running 3 tests
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/root_init.rs (target/debug/deps/root_init-2801800809bf44d7)
running 10 tests
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/session_kind_guidance.rs (target/debug/deps/session_kind_guidance-fe8dabc8964954ae)
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/session_kind_tree.rs (target/debug/deps/session_kind_tree-ef3ca8fbc0b13a96)
running 17 tests
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
     Running tests/tree_lock.rs (target/debug/deps/tree_lock-7afdfb47b5dce6d8)
running 6 tests
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   Doc-tests grove_llm
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The third command runs the crate's own suite, whose tests supply the evidence
this book cites and does not reproduce. The per-test `ok` lines, the `finished
in` clauses and the blank lines between blocks are elided; every test binary is
shown. What the transcript makes visible, and no chapter could, is the shape of
the evidence: **both unit-test targets run zero tests, and the doc-tests run
none.** There is no test inside this corpus at all. Every test named in this
book's last columns is one of the two hundred and thirty-three integration tests
under `crates/grove-llm/tests/`, or one in another crate the citing page names —
all of them outside the four roots this book reconstructs, which is what the
manifest's dev-dependencies were the only trace of, back in *Orientation*, and
what the library target exists to make possible.

<!-- rollup «source-roots» -->
<!-- rollup «owned-lines-total» -->
<!-- rollup «chapters» -->
The book reconstructs 4 roots, 915 lines, 7 chapters, two lookup
surfaces, zero deferred ranges. What it argued is that a thin command surface
over a library has exactly one thing left to get right, and that the thing is
order — two of them, each stated where it happens and each with a different
cost for being reversed. What it leaves the reader with is the question to take
to a surface of their own: what is left here that is not rendering, and for each
survivor, what fails if the order is reversed?

[Previous: Leaving the loop](06-leaving-the-loop.md) | [Contents](README.md)
