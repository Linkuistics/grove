# Ending work
<!-- book-page id="ending-work" slice="two-steps-remain" order="5" -->
[Previous: Growing the tree](04-growing-the-tree.md) | [Contents](README.md) | [Next: Leaving the loop](06-leaving-the-loop.md)

<a id="two-steps-remain"></a>
## The last tree verbs a session runs say on stderr what remains

Two verbs end work. `leaf-retire` marks one live leaf `DONE`, and `leaf-prune`
marks a live leaf, or every live leaf under a node, `ABANDONED`. Both are one
rename per leaf — an infix added after the position in the filename, no byte
inside the file touched — and both are the last verbs a session runs against
the tree: after the mark comes the commit, which is jj's and not this binary's,
and after the commit comes `complete`, which opens no tree. The rule this
chapter opens on is what a thin binary does at that point in a session, and it
is not about the mark. The mark is one call into `grove_loop::verbs` like every
other verb's. It is about what the handler prints after the call returns: each
handler names, on stderr, **the two steps that remain** — commit this session's
work, including the rename; then run `grove-llm complete` as the last action —
at the moment of decision, in the output of the verb that ended the work,
rather than leaving them in a mandate the session read a whole context earlier.
stdout stays data: the marked paths, one per line, which callers parse.

The premise is the [task commit boundary](../../../CONTEXT.md#task-commit-boundary).
One session is one focused commit, that commit carries the artifact, whatever
the grow verbs wrote and the `DONE` rename together, and everything in that
list is written before the commit — so Retire precedes Commit as a loop step
order, not as advice. The binary enforces none of it: it has no verb that
commits a task and no check that one happened. What it can do is put the order
in front of the session at the one point the order is about to matter, and that
is the whole of what these two handlers add to their calls.

The chapter owns three blocks of `cli.rs`: the two variants (217–247), the two
argument structs (400–411), and the two handlers with `eprint_next_steps`
(768–825). It reads the reminder first, because both handlers end with it; then
`leaf-retire`, the single-leaf case, whose every refusal is the call's; then
`leaf-prune`, which adds the HITL rule the code cannot enforce, the node case,
and the reminder printed last and only when something was marked. The two doc
comments — the `--help` a session reads — are read last, as the catalogue of
promises the handlers have been seen to keep.

<a id="worked-retire"></a>
## Worked example: the session's leaf retired, and a node pruned

The session is the one *Orientation* carries. It has added its review leaf,
`02-review-impl--rate-limit-k4.md`, and now retires its own — the step *Reading
the tree* looked forward to when it resolved `rate-limit-k3` after retirement.
Every line below is what the built binary printed at the frozen corpus on a
scratch tree of the same shape, with the scratch paths replaced by the carried
session's. The verb was run without a driver, so the admission *The grammar and
the openings* read is not shown; under the driver it precedes the handler and
changes nothing the handler prints.

```console
$ cd /work/atlas && grove-llm leaf-retire /work/atlas/.grove/01-impl--rate-limit-k3.md
/work/atlas/.grove/01-DONE-impl--rate-limit-k3.md
leaf-retire: two steps remain:
  1. commit this session's work, including this rename
  2. run `grove-llm complete` as your last action
```

`cmd_leaf_retire` resolves the working tree, normalizes the path — absolute
here, so it passes through unchanged — takes the exclusive opening, and makes
the call, which renames `01-impl--rate-limit-k3.md` to
`01-DONE-impl--rate-limit-k3.md` and returns the new path. The handler prints
that path on stdout, then the two remaining steps on stderr, and exits `0`.
Nothing else under `.grove/` changed: the retired leaf's body is byte-identical
to what the session wrote, and `02-review-impl--rate-limit-k4.md` is untouched.
What the verb added to the working copy is one rename — `jj diff --summary`
shows it as `R .grove/{01-impl--rate-limit-k3.md => 01-DONE-impl--rate-limit-k3.md}`,
and on the scratch tree, committed before the verb, that line was the whole
diff. In the carried session it sits beside the new review leaf and the work in
`crates/gateway/`, all uncommitted, and the commit the first step asks for
takes all of them together, the rename recorded as a rename, because jj
snapshots the whole working copy and grove stages nothing
(`docs/adr/grove-does-not-stage-its-own-renames.md`).

The session never prunes. `leaf-prune` is run when a human has said, in so
many words, that planned work is abandoned, and the carried session met no such
decision. The example therefore steps outside the session for one command and
gives the carried grove the smallest node that shows both halves of the node
case: one leaf already retired, two still live. The tree below is the carried
tree after the retirement above, with that node added; the two root-level
leaves are as the retirement left them.

```text
/work/atlas/.grove/
├── _BRIEF.md
├── 01-DONE-impl--rate-limit-k3.md
├── 02-review-impl--rate-limit-k4.md
└── 03-k5/
    ├── _BRIEF.md
    ├── 01-DONE-impl--warm-k6.md
    ├── 02-impl--evict-k7.md
    └── 03-impl--ttl-k8.md
```

The human has confirmed that the cache work is abandoned, and the prune names
the node.

```console
$ grove-llm leaf-prune /work/atlas/.grove/03-k5
/work/atlas/.grove/03-k5/02-ABANDONED-impl--evict-k7.md
/work/atlas/.grove/03-k5/03-ABANDONED-impl--ttl-k8.md
leaf-prune: left 1 already-DONE leaf untouched:
  /work/atlas/.grove/03-k5/01-DONE-impl--warm-k6.md
leaf-prune: two steps remain:
  1. commit this session's work, including these renames
  2. run `grove-llm complete` as your last action
```

The call marks the two live leaves and leaves the retired one alone, and the
handler renders what came back in a fixed order: the marked paths on stdout,
one per line; the leaf left untouched on stderr, as an advisory; and the
reminder on stderr, last, now saying *these renames* because two leaves were
marked. The node's `_<slug>.md` is not a leaf and is neither marked nor reported.
Exit `0`; the verb added two renames to the working copy and nothing else, and
every body under the node is byte-identical. Run a second time on the same node, the
same command marks nothing, prints *leaf-prune: nothing live to mark* and the
same left-untouched line, and does **not** print the reminder — a prune that
ended no work leaves no session to close — and still exits `0`, because nothing
to mark is an answer and not a refusal, the distinction *Reading the tree*
drew.

The table separates the two invocations by stream, so what each verb prints
and where reads as one relation rather than two transcripts.

| | `leaf-retire` of the session's leaf | `leaf-prune` of `03-k5` |
|---|---|---|
| the opening | exclusive | exclusive |
| the call | `verbs::leaf_retire`: one rename | `verbs::leaf_prune`: two renames, one leaf left alone |
| stdout | the one renamed path | the two renamed paths, in the order marked |
| stderr, advisory | — | `left 1 already-DONE leaf untouched:` and its path |
| stderr, last | `two steps remain`, *this rename* | `two steps remain`, *these renames* |
| exit | `0` | `0` |
| added to the working copy | one rename | two renames |

<a id="the-reminder"></a>
## `eprint_next_steps`: two steps, on stderr

The reminder is one function both handlers call, and its comment carries the
chapter's thesis. It is read first because it is the last thing either verb
does.

<!-- fragment «eprint-next-steps» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="766-783" parent="handlers-ending" -->
````rust
// The two steps that always follow a terminal mark: the commit that carries it,
// then the completion signal. `leaf-retire` and `leaf-prune` are the
// terminal-marking pair and the **last tree verbs a session runs** — Retire
// precedes Commit, and the commit itself is jj's — so their output lands in
// the agent's context at the moment of decision, rather than only in the mandate
// a whole session earlier. **stderr**: stdout is data (callers parse the printed
// paths), and `leaf-prune`'s existing advisories already set that precedent.
fn eprint_next_steps(verb: &str, marked: usize) {
    let renames = if marked == 1 {
        "this rename"
    } else {
        "these renames"
    };
    eprintln!("{verb}: two steps remain:");
    eprintln!("  1. commit this session's work, including {renames}");
    eprintln!("  2. run `grove-llm complete` as your last action");
}

````
<!-- /fragment -->

The function takes the verb's name and the number of leaves marked, and prints
three lines on stderr: the verb and *two steps remain*, then the commit, then
`complete`. The count decides two words — *this rename* for one leaf, *these
renames* for more — and nothing else; the two steps are the same for both verbs
and for any count, because what follows a terminal mark does not depend on how
many marks there were. `leaf-retire` always passes `1`, so the singular is the
only form it prints; the plural is reachable only through a node prune that
marked at least two leaves, which is the worked example's second transcript.

The comment gives two reasons, and the page checks each. The first is timing:
these are the last tree verbs a session runs, Retire precedes Commit, and the
commit is not this binary's, so the two steps are printed where the session
will read them at the moment it has to take them. Both qualifiers in *the last
tree verbs a session runs* are load-bearing, and dropping either makes the
sentence false. **Tree verbs**, because `grove-llm complete` runs after both
and the reminder's own second line says so — it is a verb of this binary that
opens no tree, so it ends the session without ending the work on the tree.
**A session runs**, because `finish-commit` does open the tree — not in its
handler, which takes neither opening, but in the call behind it — and it opens
it exclusively; the session that retires never runs it, as
[*Leaving the loop*](06-leaving-the-loop.md#worked-complete) reads, because
that verb belongs to the finish session the driver launches once the last live
leaf is terminal. The order is the
[task commit boundary](../../../CONTEXT.md#task-commit-boundary)'s — Retire
precedes Commit — and the binary keeps none of it: nothing stops a session from
adding a leaf after retiring its own. The commit the comment calls jj's is jj's
on every tree the binary accepts. Grove drives jj and refuses a working tree
that is not jj-enabled (`docs/adr/jj-is-the-only-lane.md`) — *The grammar and
the openings* read `worktree` refusing exactly that — and admits a colocated
tree as jj's business, touching no git index on it, which
`leaf_retire_in_a_colocated_tree_leaves_the_git_index_alone` in
`crates/grove-llm/tests/jj_tree_verbs.rs` requires.

The second reason is the stream, and it is the one a test holds. stdout is
data — the marked paths, which callers parse — so the reminder goes to stderr,
where `leaf-prune`'s two advisories already were. `assert_next_steps` in
`crates/grove-llm/tests/leaf_ops.rs` is the shared assertion: it requires the
commit step and the `complete` step on stderr in that order, requires *last
action*, requires the expected singular or plural, and requires that the word
*complete* does not reach stdout. `retire_names_the_remaining_steps_on_stderr`
applies it to `leaf-retire` and additionally requires stdout to be exactly one
line; `prune_of_one_leaf_names_the_remaining_steps_on_stderr` applies it to a
single-leaf prune with *this rename*; and
`prune_of_a_node_reminds_once_for_the_whole_bulk_mark` applies it to a node
with *these renames* and counts *two steps remain* once, so one bulk mark that
ends one session earns one reminder. The wording of the three lines beyond
those phrases is held by the source and no test.

<a id="one-rename"></a>
## `leaf-retire`: one rename, and the refusals that are the call's

`cmd_leaf_retire` is the shorter of the two handlers and the shape of both:
resolve the working tree, normalize the path, open for writing, make the call,
print what it returned, then the reminder.

<!-- fragment «handler-leaf-retire» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="784-793" parent="handlers-ending" -->
````rust
fn cmd_leaf_retire(args: &LeafRetireArgs) -> Result<()> {
    let worktree = worktree()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let tree = writable(&worktree)?;
    let dst = verbs::leaf_retire(&tree, &leaf_path)?;
    println!("{}", dst.display());
    eprint_next_steps("leaf-retire", 1);
    Ok(())
}

````
<!-- /fragment -->

Set beside *Growing the tree*'s handlers, two steps are absent, and their
absence is the point. There is no text read by a grammar type and no presence
rule. The
only argument is a path; `normalize_leaf_path`, *Reading the tree*'s helper,
makes it absolute if it exists relative to the current directory and otherwise
passes it through, and the call resolves it against the tree under the lock. No
kind is written, so there is no template to ask about, and `writable` is the
first thing after the path — a bad path is refused by the call, not before it.
The order that is left is the one the chapter is about: `println!` of the
returned path, then `eprint_next_steps("leaf-retire", 1)`. The path is printed
after the call returned, as `print_paths` was in *Growing the tree*, so stdout
never names a rename that did not happen; and the count is the literal `1`,
because the verb marks exactly one leaf or refuses.

Every refusal is the call's, and the handler pre-empts none of them: it neither
reads the filename nor checks that the path names a leaf, so what comes back for
a wrong operand is `verbs::leaf_retire`'s own message, through `?`, on stderr,
with exit `1` and the tree unchanged. The table is every refusal measured
against the built binary, and it is what a reader needs to predict what a wrong
operand gets; the help text read below names the first three and not the rest.
Where a test is named it holds that the operand is refused and that the message
carries the word shown, not the wording; every wording is the source's.

| Operand | Refusal on stderr | Held by |
|---|---|---|
| the node's `_<slug>.md` | `cannot retire a brief (briefs are never done): _topic.md` | `retire_refuses_a_brief` — a refusal naming *brief* |
| a `DONE` leaf | `leaf is already retired (DONE): 01-DONE-impl--rate-limit-k3.md` | `retire_refuses_an_already_done_leaf` — a refusal naming *already retired* or *DONE* |
| an `ABANDONED` leaf | `cannot retire an abandoned (ABANDONED) leaf: …` | no test in this crate |
| a node directory | `cannot retire a node (nodes are never marked done): 03-k5` | no test in this crate |
| the grove root, `.grove` | `cannot retire the grove root (lifecycle verbs act on leaves): /work/atlas/.grove` | no test in this crate |
| a `finish` leaf | `` `finish` is driver-reserved and cannot be retired `` | `every_agent_side_mutation_refuses_the_driver_reserved_finish_kind` in `session_kind_tree.rs` — a refusal naming *finish* and *driver-reserved* |
| `.grove/99-impl--nope-k99.md`, typed at `/work/atlas`, no such leaf | `resolving path /work/atlas/.grove/.grove/99-impl--nope-k99.md`, then *No such file or directory* | no test in this crate |

The last row is the path helper's pass-through branch meeting the call's join.
A `.grove/…` spelling that does not exist relative to the current directory
passes through unchanged and is joined onto a root that already ends in
`.grove`, which is the limit *Reading the tree* measured for `kind`, and the
diagnosis is the same doubled path; the bare spelling `99-impl--nope-k99.md`
is joined once and refused with the single path. `.` is a different case: it exists relative
to the current directory, is made absolute to `/work/atlas`, and is refused as
*not under grove root*.

Two promises in the help are about what the verb does **not** change, and both
are held by tests that compare whole trees rather than assert an absence. *Task
bodies are byte-identical*: `retire_adds_done_infix_in_place` in `leaf_ops.rs`
reads the retired file back and requires its body equal to what was written.
*Retirement writes no launch-routing metadata*:
`retiring_a_reviewed_producer_changes_only_its_own_filename` in
`crates/grove-llm/tests/reviewed_producer_lifecycle.rs` snapshots every file
under `.grove/` before and after retiring a producer that a review names, and
requires that the only difference is the operand's name — so a sibling write
nobody has thought of yet would fail it, where a
`!contains("**Producer launch:**")` assertion would pass forever. That is why
the file states the claim by enumeration, and why it keeps a positive control,
`the_snapshot_comparison_rejects_a_sibling_write`, that shows the comparison
catching a planted write.
`reviewed_producer_retirement_does_not_write_body_routing_in_a_jj_native_tree`
in `jj_tree_verbs.rs` holds the same claim on a jj-native tree.

The working-copy half is the through-line to the first step of the reminder.
`retiring_a_tracked_leaf_is_one_rename_in_the_working_copy` requires that
`jj diff --summary` over `.grove` shows exactly one `R` line after the verb, and
`a_commit_after_the_verb_records_the_retire_as_a_rename` commits and requires
the recorded change to be that rename and the working copy clean afterward.
Neither is a property of this handler — the rename is the call's, and jj
snapshots it — but together they are what makes *commit this
session's work, including this rename* a step that needs no staging first.
`leaf_retire_in_a_colocated_tree_leaves_the_git_index_alone` holds a different
property on a colocated tree: the git index is unchanged after the verb.

`LeafRetireArgs` is the one argument, and it is read here because the
handler's first line after the opening consumes it: clap fills `leaf_path` from
the operator's text, `normalize_leaf_path` takes it, and the call resolves the
result against the tree. Its help is the only place the verb states which
spellings it accepts, and the worked example passed the first of them.

<!-- fragment «args-leaf-retire» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="400-405" parent="args-ending" -->
````rust
#[derive(Parser)]
pub struct LeafRetireArgs {
    /// Leaf path. Absolute, or relative to the grove root (`.grove/`).
    pub leaf_path: PathBuf,
}

````
<!-- /fragment -->

The help's *absolute, or relative to the grove root* is the call's rule for the
path, and the handler's `normalize_leaf_path` adds the working-tree-relative
case in front of it, so all three spellings *Reading the tree* tabulated reach
the leaf from `/work/atlas`. The argument is a `PathBuf` rather than a `String`
because nothing in this module reads it as text: unlike a slug or a kind there
is no grammar type that owns a path, and the tree is what decides whether it
names a leaf.

<a id="the-node-case"></a>
## `leaf-prune`: the HITL rule, the node case, and the reminder last

`cmd_leaf_prune` opens the same way and differs in what comes back: not one
path but a `Pruned` with two lists, the leaves newly marked and the retired
leaves found and left alone. The first fragment is the opening, the call, and
the rendering of both lists.

<!-- fragment «handler-leaf-prune-marks» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="794-814" parent="handlers-ending" -->
````rust
fn cmd_leaf_prune(args: &LeafPruneArgs) -> Result<()> {
    let worktree = worktree()?;
    let path = normalize_leaf_path(&args.path);
    let tree = writable(&worktree)?;
    let result = verbs::leaf_prune(&tree, &path)?;
    for p in &result.marked {
        println!("{}", p.display());
    }
    if result.marked.is_empty() {
        eprintln!("leaf-prune: nothing live to mark");
    }
    if !result.left_done.is_empty() {
        eprintln!(
            "leaf-prune: left {} already-DONE leaf{} untouched:",
            result.left_done.len(),
            if result.left_done.len() == 1 { "" } else { "s" }
        );
        for p in &result.left_done {
            eprintln!("  {}", p.display());
        }
    }
````
<!-- /fragment -->

The order of the writes is the stream contract applied. The marked paths go to
stdout first, one per line, after the call returned — the same discipline as
`print_paths`, so a prune that failed prints no path. Then the advisories, on
stderr and each conditional: *nothing live to mark* when the marked list is
empty; *left N already-DONE leaf(s) untouched:* with each path indented when
the other list is not; and, in the next fragment, the reminder. `left_done` is
the node case's report. Given a node, what comes back is every live leaf in the
subtree, now marked, in `marked`, and every `DONE` leaf found there, untouched,
in `left_done` — that work really was done, and marking it abandoned would
misreport it. A leaf already `ABANDONED` comes back in neither list, so it is
neither printed nor reported: a node whose only child is already abandoned
prints *nothing live to mark* and no second line, which was measured. Given a
single leaf, the call marks it or refuses, and `left_done` is empty by
construction.

What the handler does with `Pruned` is all it does, and what it does not do is
the sentence the help sets in bold. **HITL: only call this after explicit human
confirmation.** Nothing in this handler, and nothing in the call, asks for or
checks that confirmation — there is no flag, no prompt and no environment
variable, and the verb marks whatever it is pointed at. That is constraint 5 as
the help states it: grove guides and does not gate, so the gate is the
caller's, and the caller is the session's methodology, which tells an
unattended session that reaches the question to stop and say so rather than
run this verb. The page states the fact the code makes checkable: the HITL
rule is help text and methodology, and the binary would carry out a prune it
was never authorised to make. The prune in the worked example was run after
that confirmation, and the transcript cannot show it because the binary never
sees it.

The node case is bulk, and the call makes two promises about it that decide
what stdout can carry. A subtree the verb cannot mark in full is refused with
nothing renamed — measured with a `finish` leaf inside the node: *`finish` is
driver-reserved and cannot be pruned*, and every leaf under the node still
live. And a bulk mark is not one atomic step: the call can stop partway, with
some leaves renamed and the rest not, and
`docs/adr/bulk-marks-are-not-atomic.md` records when and why. The handler's
half of that case is the `?` on the call: it returns before the `for` loop, so
a prune that stopped partway prints **no** path on stdout even though some
renames landed, and the leaves already marked are named only in the error's
context on stderr, together with the instruction to rerun. That wording is the
loop's and this book does not trace the case; what is this handler's is that
stdout is either the complete list or empty, never a prefix.

`leaf_prune_marks_a_whole_subtree_abandoned_in_a_jj_native_tree` in
`jj_tree_verbs.rs` and `pruning_a_node_marks_every_leaf_the_same_way` in
`leaf_ops.rs` hold the bulk mark — the second requiring the working copy to
show exactly *N* plain renames, so a per-mark repository step slipping back in
would fail it. `prune_that_marks_nothing_stays_quiet` requires the *nothing
live to mark* advisory. `pruning_a_reviewed_producer_changes_only_its_own_filename`
in `reviewed_producer_lifecycle.rs` holds the help's *pruning writes no
producer receipt* by the same whole-tree enumeration the retirement test used,
and separately requires the review leaf beside the pruned producer to still
exist, live; that `pick` will then select it is the walk's, which the test does
not run, and it is what *prune the enclosing chain to close the whole reviewed
path* warns about. The left-`DONE` advisory's wording and the
three refusals of a single-leaf prune — a brief, a `DONE` leaf, an `ABANDONED`
leaf — are held by the source and no test in this crate. All three were
measured, as *cannot prune a brief (briefs are never marked)*, *cannot prune a
retired (DONE) leaf* and *leaf is already pruned (ABANDONED)*, each with the
filename, exit `1`, tree unchanged. The grove root is refused as *cannot prune
the grove root (abandoning a whole grove is a branch-delete, not a tree mark)*,
with the root's path; no test in this crate holds it either.

The second fragment is the reminder, and its comment states the two conditions
the worked example showed.

<!-- fragment «handler-leaf-prune-reminder» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="815-823" parent="handlers-ending" -->
````rust
    // Last, so the reminder is the final thing in the agent's context — and only
    // when this call actually ended some work; a no-op prune leaves no session
    // to close.
    if !result.marked.is_empty() {
        eprint_next_steps("leaf-prune", result.marked.len());
    }
    Ok(())
}

````
<!-- /fragment -->

Last, and only when something was marked. Last, so that in an agent's context
the final thing the verb said is what to do next, after the advisory about
leaves left alone; the source holds that order and no test asserts it. Only
when the marked list is not empty, because a prune that ended no work leaves no
session to close — the no-op case printed its advisory and nothing more — and
`prune_that_marks_nothing_stays_quiet` holds it by requiring the string
`grove-llm complete` absent from stderr. The count passed is the marked list's
length, which is what turns *this rename* into *these renames* for the node case
and keeps the singular for a single-leaf prune, which
`prune_of_one_leaf_names_the_remaining_steps_on_stderr` requires.

`LeafPruneArgs` mirrors `LeafRetireArgs`, with the help widened to a node and
the field renamed to match; it is read here because the operand it carries is
what makes the node case reachable at all.

<!-- fragment «args-leaf-prune» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="406-411" parent="args-ending" -->
````rust
#[derive(Parser)]
pub struct LeafPruneArgs {
    /// Leaf or node path. Absolute, or relative to the grove root (`.grove/`).
    pub path: PathBuf,
}

````
<!-- /fragment -->

*Leaf or node path*: the field is named `path` rather than `leaf_path` because
a directory is a valid operand, and `normalize_leaf_path` — despite its name —
treats one the same way, since a directory that exists relative to the current
directory passes the same `exists()` test a file does. The grove-relative
spelling works too: `leaf-prune 02-review-impl--rate-limit-k4.md` typed at
`/work/atlas` passes through and is joined by the call, and marked that leaf
when measured. The composite that reassembles the two argument structs is
stated here.

<!-- fragment «args-ending» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="400-411" parent="source-command-surface" -->
<!-- insert «args-leaf-retire» -->
<!-- insert «args-leaf-prune» -->
<!-- /fragment -->

The composite that reassembles the reminder and the two handlers, in source
order, is stated here.

<!-- fragment «handlers-ending» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="766-823" parent="source-command-surface" -->
<!-- insert «eprint-next-steps» -->
<!-- insert «handler-leaf-retire» -->
<!-- insert «handler-leaf-prune-marks» -->
<!-- insert «handler-leaf-prune-reminder» -->
<!-- /fragment -->

<a id="the-two-contracts"></a>
## The two contracts, as `--help` states them

The two variants are the terminal verbs' doc comments, which are the `--help` a
session reads and the guide paraphrases for the human; the page reproduces them
because they are corpus, and reads each for the line that keeps its promise and
the test that would catch its breach. `LeafRetire`'s comment describes the
rename exactly — the infix after the position, the position and key kept, the
body untouched — and names three refusals.

<!-- fragment «verbs-leaf-retire-help» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="217-226" parent="verbs-ending" -->
````rust
    /// Mark a live leaf retired in place by adding a `DONE` infix
    /// (`NN-<kind>--<slug>-k<key>.md` →
    /// `NN-DONE-<kind>--<slug>-k<key>.md`) — no `done/`
    /// directory; the leaf keeps its position and key in its directory, and the
    /// file's contents (its `# <slug>-k<key>` header) are untouched. Refuses a
    /// brief, an already-retired (`DONE`) leaf, and an already-abandoned
    /// (`ABANDONED`) leaf. Prints the retired file's absolute
    /// path on stdout. Task bodies are byte-identical; retirement writes no
    /// launch-routing metadata. Working-tree change only — no commit.
    LeafRetire(LeafRetireArgs),
````
<!-- /fragment -->

The three refusals it names are the call's, and it is silent on three more the
call also makes — a node, the grove root, a `finish` leaf — and on the
not-found case; the table under *`leaf-retire`: one rename* has all seven.
*Prints the retired file's absolute path on stdout* is line 789; the absolute
path is the call's, built from the caller's own spelling of the root.
*Working-tree change only — no commit* is true of every verb in this module
but `finish-commit`, and here it is the point: the commit is the first of the two steps the handler
goes on to name.

`LeafPrune`'s comment is longer because it carries a rule the code cannot
enforce, a case with two outcomes, and a warning about the methodology's
chains.

<!-- fragment «verbs-leaf-prune-help» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="227-247" parent="verbs-ending" -->
````rust
    /// Mark abandoned work `ABANDONED` in place. **HITL: only
    /// call this after explicit human confirmation** — grove never abandons
    /// planned work on its own (constraint 5: grove guides, it does not gate —
    /// the gate is yours, not the CLI's). `<path>` is a live leaf file **or** a
    /// node directory (absolute, or relative to the grove root):
    ///
    ///   * given a **leaf**, marks it directly — refuses a brief, an
    ///     already-`DONE` leaf, and an already-`ABANDONED` leaf;
    ///   * given a **node**, marks every *live* leaf in its subtree
    ///     (recursively) — leaving `DONE` leaves untouched, since that work
    ///     really was done — and refuses the grove root itself (abandoning a
    ///     whole workstream is a branch-delete, not a tree mark).
    ///
    /// The `ABANDONED` infix is filename-only — every marked leaf's
    /// `# <slug>-k<key>` header stays byte-identical. Prints each newly-marked
    /// leaf's absolute path on stdout, one per line; any already-`DONE` leaves
    /// found and left alone are reported on stderr. Pruning writes no producer
    /// receipt: pruning only a producer leaves its sibling review live and
    /// uncheckable; prune the enclosing chain to close the whole reviewed path.
    /// Working-tree change only — no commit.
    LeafPrune(LeafPruneArgs),
````
<!-- /fragment -->

Read against the handler, the comment divides into what the code keeps and
what it only says. The two cases — a leaf marked directly with its three
refusals, a node marked recursively with `DONE` leaves left alone and the root
refused — are the call's, and the rendering of the two lists is lines 799–801
and 805–814.
*Prints each newly-marked leaf's absolute path on stdout, one per line; any
already-`DONE` leaves found and left alone are reported on stderr* is exactly
those lines. *Pruning writes no producer receipt* is
`reviewed_producer_lifecycle.rs`'s enumeration. The HITL sentence and the
closing sentence about pruning a producer are methodology: the first is a rule
the binary cannot check and does not, and the second names a consequence of the
flat review chain *Growing the tree* linked to the guide — the review leaf stays
live, and `pick` will select it — that the verb neither prevents nor warns
about at run time. The table gathers both verbs' promises against the line that
keeps each and the test that holds it, in the form the closing chapter's rows
are built from.

| Promise in the help | Verb | Kept at | Held by |
|---|---|---|---|
| the `DONE` infix, position and key kept, body untouched | `leaf-retire` | the call, line 788 | `retire_adds_done_infix_in_place` |
| refuses a brief, a `DONE` leaf, an `ABANDONED` leaf | `leaf-retire` | the call, through `?` on line 788 | `retire_refuses_a_brief`, `retire_refuses_an_already_done_leaf`; the third by no test in this crate |
| the retired path on stdout | `leaf-retire` | line 789 | `retire_names_the_remaining_steps_on_stderr` — exactly one stdout line |
| no launch-routing metadata written | `leaf-retire` | nothing in the handler writes | `retiring_a_reviewed_producer_changes_only_its_own_filename` |
| a leaf marked directly; a node's live leaves marked, `DONE` left alone, the root refused | `leaf-prune` | the call, line 798 | `pruning_a_node_marks_every_leaf_the_same_way`, `leaf_prune_marks_a_whole_subtree_abandoned_in_a_jj_native_tree`; the root refusal by no test in this crate |
| marked paths on stdout, one per line; `DONE` leaves left alone reported on stderr | `leaf-prune` | lines 799–801 and 805–814 | `prune_that_marks_nothing_stays_quiet` for the empty case; the left-`DONE` wording by no test in this crate |
| no producer receipt | `leaf-prune` | nothing in the handler writes | `pruning_a_reviewed_producer_changes_only_its_own_filename` |
| HITL: only after explicit human confirmation | `leaf-prune` | nowhere — the code cannot keep it | — |
| two steps remain, on stderr | both | lines 790 and 818–820 | `assert_next_steps`, applied three ways |

The composite that reassembles the two variants is stated here.

<!-- fragment «verbs-ending» owner="two-steps-remain" source="crates/grove-llm/src/cli.rs" lines="217-247" parent="source-command-surface" -->
<!-- insert «verbs-leaf-retire-help» -->
<!-- insert «verbs-leaf-prune-help» -->
<!-- /fragment -->

The two verbs that end work have now been read, and each is one call and two
streams: the marked paths on stdout, and on stderr — last, and only when a mark
was made — the commit and the completion signal that remain. The commit is
jj's. The signal is this binary's, and it is the one verb that has to be sure
it is talking to the loop that launched it before it writes anything; the next
chapter reads it, and the other verb that opens no tree.

[Previous: Growing the tree](04-growing-the-tree.md) | [Contents](README.md) | [Next: Leaving the loop](06-leaving-the-loop.md)
