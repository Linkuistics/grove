# Growing the tree
<!-- book-page id="growing-the-tree" slice="before-the-lock" order="4" -->
[Previous: Reading the tree](03-reading-the-tree.md) | [Contents](README.md) | [Next: Ending work](05-ending-work.md)

<a id="before-the-lock"></a>
## Text before lock, presence before mutation

The four verbs that change the tree are `root-init`, `leaf-add`, `leaf-insert`
and `leaf-decompose`, and every one of them does the same three things in the
same order: it reads the operator's text through the grammar's own types, it
asks whether each kind it is about to write resolves to a launch template, and
only then does it open the tree for writing. The rule this chapter opens on is
that order — **text before lock, presence before mutation.** A refusal at either
of the first two steps leaves the tree byte-identical and takes no exclusive
lock on the way out, because the text is parsed and the presence rule is asked
before `writable` is ever called. This is the same discipline *Reading the
tree*'s verbs kept when they read a `Reference` before taking the shared
opening; here the opening is exclusive and the cost of getting the order wrong
is a tree changed by a command that was refused.

The exclusive opening is the [tree access lock](../../../CONTEXT.md#tree-access-lock),
and it is this chapter's premise. It is a process-scoped advisory lock every
task-tree reader and mutator takes on an open descriptor for the **working-tree
root** — shared for readers, exclusive for mutators — before it inspects names.
It is taken on the working-tree root rather than on `.grove/` because the root
exists before the grove is created and through its deletion, so root
initialization and the grow verbs all serialize against one thing. `grove_loop::read`
takes the shared lock and `grove_loop::write` the exclusive one, and *The
grammar and the openings* read the two helpers, `readable` and `writable`, that
call them. What this chapter needs of the lock is one fact about `flock(2)`,
stated once: **two open file descriptions on one directory do not share an
advisory lock.** A process that has the tree open for writing and then opens the
same directory a second time contends with *itself*, and waits forever. That
fact is behind the three passages this chapter argues hardest — `root-init`'s
choice of `match`, `leaf-insert`'s `relinquish` before the lint takes its own
reading opening, and `leaf-decompose`'s read of the inherited kind before it
opens for writing — and it is the whole of what the self-deadlock argument needs.

The chapter owns eight blocks of `cli.rs`, more than any other, and reads them
in the order a grow verb meets its work rather than the file's: the two openings
were read in *The grammar and the openings*, so this page starts at the handler.
It walks the four verbs in the order each adds one argument to the lock's scope —
`root-init`, then `leaf-add`, then `leaf-insert`, then `leaf-decompose` — reading
each handler and its argument struct beside the rule it turns on, and it reads
the shared machinery once: the presence rule `require_declared` and the `slug`
grammar, and the two `--kind` help constants with `parse_kind`. The four verbs'
doc comments, which are their `--help`, are read last, as the catalogue of
promises the handlers have just been seen to keep.

<a id="worked-leaf-add"></a>
## Worked example: one leaf added, and the refusal before the lock

The session is the one *Orientation* carries, and its writing verb is this
chapter's example. The grove holds one live leaf, `rate-limit-k3`; the session
has done its work and adds the review leaf its methodology asks for. The
configuration the driver launched it under declares two kinds, `impl` and
`review-impl`, and nothing else. Every line below is what the built binary
printed at the frozen corpus on a scratch tree of the same shape, with the
scratch paths and the scratch configuration path replaced by the carried
session's.

```console
$ cd /work/atlas && grove-llm leaf-add . rate-limit --kind review-impl
/work/atlas/.grove/02-review-impl--rate-limit-k4.md

$ grove-llm leaf-add . rate-limit --kind prototype
Error: refusing to write a leaf of kind `prototype`: no launch template resolves for it

Caused by:
    key `prototype` does not resolve: no template for it.
      Declare `prototype` in ~/.config/grove/config.kdl.
```

The first invocation is the session's, and it lands. `cmd_leaf_add` reads the
kind list — one kind, `review-impl` — through `parse_kind`, the slug
`rate-limit` through `slug`, and the parent `.` through `Reference::parse`; it
asks `require_declared` whether `review-impl` resolves, and the carried
configuration declares it, so it does; and only then does it take the exclusive
opening and make the call, which lands `02-review-impl--rate-limit-k4.md` and
prints it. The position is `02` because the grove root held one child and the
key is `k4` because a key is the maximum over the whole tree plus one; both are
the call's rules, named in *Orientation* and not this binary's.

The second invocation is the same argument vector with one word changed, and it
makes the book's second order visible. `prototype` is a well-formed kind
token, so `parse_kind` accepts it and the slug and parent parse exactly as
before; the refusal comes from `require_declared`, which loads the
configuration, finds no template for `prototype`, and returns the error above —
naming the kind and the file that must declare it. No exclusive lock was taken,
because `writable` is never reached, and the tree is byte-identical: had this
been the session's real verb, nothing would have changed on disk.
`leaf_add_refuses_a_kind_no_template_resolves_for_and_mutates_nothing` in
`crates/grove-llm/tests/session_kind_presence.rs` drives the binary against a
configuration declaring only `impl`, requires the refusal, requires that it
names the configuration file, and snapshots `.grove/` before and after to prove
the tree did not move.

The table separates the two endings by what each step did, so the order reads as
a relation rather than as two transcripts.

| Step | `--kind review-impl` | `--kind prototype` |
|---|---|---|
| parse the kinds (`parse_kind`) | `review-impl` accepted | `prototype` accepted — it is a well-formed token |
| parse the slug and parent | `rate-limit`, `.` | the same |
| presence (`require_declared`) | resolves to a template | **no template — refused here** |
| the exclusive opening (`writable`) | taken | never reached |
| the call, then stdout | `02-review-impl--rate-limit-k4.md` | nothing |
| exit, and the tree | `0`, one leaf added | `1`, byte-identical |

`root-init` is the driver's verb, not the session's — it scaffolds a grove
before any session exists — and it gets its own short trace. On a working tree
with no `.grove/`, under a configuration that declares `requirements`, it writes
the charter and the first leaf and prints both paths:

```console
$ grove-llm root-init
/work/atlas/.grove/_BRIEF.md
/work/atlas/.grove/01-requirements--plan-k1.md
```

The kind of that first leaf is fixed at `requirements`, and the presence rule
applies to it exactly as it applied to `prototype` above: measured against a
configuration that does not declare `requirements`, `root-init` prints
*refusing to write a leaf of kind `requirements`* and leaves no `.grove/` behind
at all. `root_init_asks_about_the_requirements_leaf_it_mints` in
`session_kind_presence.rs` requires both — the refusal, and that the working
tree holds no grove afterward.

<a id="the-vacancy"></a>
## `root-init`: the vacancy, and what `match` costs

`cmd_root_init` is the rule in its simplest form: one kind, fixed, and a tree
that must **not** already be there. It reads its text first — the slug, then the
requirements kind — and asks the presence rule before it opens anything, so the
first fragment is text-before-lock with nothing else in the way.

<!-- fragment «handler-root-init-text» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="481-488" parent="handler-root-init" -->
````rust
fn cmd_root_init(args: &RootInitArgs) -> Result<()> {
    let worktree = worktree()?;
    // Read before the lock: refusing a bad slug without taking an exclusive one
    // is strictly kinder, and the kind's presence rule is asked before anything
    // is written.
    let slug = slug(&args.slug)?;
    let kind = Kind::requirements();
    require_declared(&worktree, std::slice::from_ref(&kind))?;
````
<!-- /fragment -->

`slug` reads the operator's text through the grammar's `Slug` type, so a bad
slug is refused here, before any lock, with the shape refusal *Reading the
tree*'s references also used; `Kind::requirements()` is the one kind literal
grove spells for itself, because it authors this leaf before any session exists
to delegate to. `require_declared` then asks the configuration about that one
kind, and its refusal is the `requirements` one the worked example measured. All
three run before the tree is touched, which is why a refused `root-init` leaves
no `.grove/` behind:
`root_init_rejects_invalid_slug_without_creating_grove` in
`crates/grove-llm/tests/root_init.rs` pins the slug case and the empty working
tree after it.

The second fragment is where the tree is opened, and its comment makes a claim
about Rust's drop order that this page checks against the compiler.

<!-- fragment «handler-root-init-vacancy» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="489-510" parent="handler-root-init" -->
````rust
    // The refusal to clobber is the **shape** rather than a check: a live grove
    // opens as a tree, and `root-init` takes a vacancy.
    //
    // `match` holds the write open through the arm: a scrutinee is a value of
    // the enclosing `let` statement, so the live `TreeWrite` and its exclusive
    // lock are alive throughout `Writing::Tree(_)`, and drop only once that
    // statement ends. So the message stays a literal: a later improvement that
    // read the tree to name the live leaf would deadlock here, and would need
    // a `let … else`, which drops the value *before* the else block runs.
    let vacancy = match grove_loop::write(&worktree)? {
        Writing::Vacancy(vacancy) => vacancy,
        Writing::Tree(_) => bail!(
            "grove root already exists: {}",
            worktree.join(".grove").display()
        ),
    };
    let initialized = verbs::root_init(vacancy, &slug, &kind)?;
    println!("{}", initialized.brief.display());
    println!("{}", initialized.first_leaf.display());
    Ok(())
}

````
<!-- /fragment -->

`grove_loop::write` answers with a shape, not a boolean: a `Writing::Vacancy`
when the root holds no tree, or a `Writing::Tree` when it does. `root-init` wants
the vacancy — the exclusive lock over a root where a grove *can* be created,
which the loop's `verbs::root_init` consumes to write the charter, the first
leaf and nothing else, under one lock. The `Writing::Tree` arm is the refusal to
clobber, and it is the shape rather than a check: a live grove opens as a tree,
so there is no way to reach `verbs::root_init` over a root that already holds
one. Measured, that arm prints *grove root already exists* and its path;
`root_init_refuses_when_grove_already_exists` requires the *already exists*
message and the existing charter untouched, and `after_root_init_pick_returns_the_new_leaf_not_done`
holds the load-bearing half of the happy path — that `pick` answers the new leaf
rather than reporting the fresh grove finished.

The comment's drop-order claim is checkable, and this page checks it rather than
taking it on trust. Compiled and run under this workspace's toolchain, in both
the 2021 and the 2024 editions: a `match` scrutinee is a value of the enclosing
`let` statement and lives until that statement ends, so the `Writing::Tree`
value — and the exclusive lock it owns — is still alive while the
`Writing::Tree(_)` arm runs and is dropped only after it; a `let … else`
initializer's value is dropped **before** the else block runs. The comment says
that, which is why it reads as a caution about the form the code uses rather
than an argument for it. `match` is the style here; holding the write through
the failure arm is what the style costs, and the literal message is what keeps
that cost free.

The arm reads no tree: it builds a message from the worktree path the handler
already has and returns, so no second opening is taken and the self-deadlock is
unreachable. The caution is for whoever changes that — and from `cli.rs`,
dropping the value is the only release there is, because the loop's own
`relinquish`, which this chapter meets later on the lint's path, is private to
`grove-loop`. A failure arm that needed to name the live leaf would have to
become the `let … else` the comment points at.

The table sets the two forms side by side, one row each, and what a reader is to
take from it is that the safe form for a failure path that touches the tree is
the one this handler does not use.

| The form | When the value drops | A tree read added to the failure path |
|---|---|---|
| `match`, the form the code uses | after the enclosing `let` statement ends, so the `Writing::Tree` value and the exclusive lock it owns are alive throughout the arm | would deadlock |
| `let … else`, the form the comment points at | before the else block runs | would not |

Neither row is a defect today, because the arm reads no tree; the difference
matters only to whoever adds the message the comment imagines.

The self-deadlock the comment gestures at is not an ending this book traces,
because the property that rules it out is stated rather than provoked, and it is
two facts. The store's own lock is blocking, and it is the one place waiting
belongs; grove takes no blocking lock of its own, which
`no_production_lock_grove_takes_for_itself_ever_blocks` in
`crates/grove-llm/tests/tree_lock.rs` holds by scanning the production half of
every source file in the five packages grove ships — each file cut at its inline
`mod tests` — for `libc::flock` calls, and requiring every *acquisition* among
them non-blocking. A release is skipped rather than accepted, because an unlock
cannot wait and a flag saying so on one would mean nothing. That test does not
prove a verb never opens the tree twice; what does is structural, and it is the
second fact. Two of this chapter's four verbs do open the tree twice, and each
keeps its openings sequential by a different means. `leaf-decompose` takes its
reading opening *before* it asks for the exclusive one, so there is no guard yet
to hold across it. `leaf-insert` takes its second opening after the mutation,
which has consumed its guard and released the lock on return; and
`verbs::stale_cross_refs` calls `relinquish` before reading regardless, so a
guard still held there would be given up rather than carried in.
*`leaf-decompose`: two openings, in order* below reads the one handler in
`cli.rs` that takes both openings itself — `leaf-insert`'s second is taken a
layer down, inside `grove-loop` — and shows the two kept sequential.

The composite that reassembles the handler is stated here, and the source index
names it as one of the root's twenty-two children.

<!-- fragment «handler-root-init» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="481-510" parent="source-command-surface" -->
<!-- insert «handler-root-init-text» -->
<!-- insert «handler-root-init-vacancy» -->
<!-- /fragment -->

`RootInitArgs` is the one argument the verb takes, and it is where the `plan`
default the help names is declared.

<!-- fragment «args-root-init» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="323-330" parent="source-command-surface" -->
````rust
#[derive(Parser)]
pub struct RootInitArgs {
    /// Slug for the first (requirements) leaf (lowercase ASCII letters, digits,
    /// dashes). Default: `plan`. Mirrors `leaf-add . <slug>`.
    #[arg(default_value = "plan")]
    pub slug: String,
}

````
<!-- /fragment -->

The slug defaults to `plan`, so `root-init` with no argument mints
`01-requirements--plan-k1.md`; `root_init_default_slug_is_plan` and
`root_init_custom_slug_names_the_first_leaf` pin the default and an override. The
comment's *mirrors `leaf-add . <slug>`* is the through-line to the next verb: the
slug argument a session gives `leaf-add` is read by the same `slug` grammar this
default feeds, so the two verbs name a leaf the same way.

<a id="presence-before-mutation"></a>
## The presence rule, and the two names it reads

Before any grow verb opens the tree, it asks one question of the configuration,
and `require_declared` is that question. It is the second order — presence before
mutation — in one function.

<!-- fragment «require-declared» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="824-852" parent="presence-rule-and-slug" -->
````rust
/// The **just-in-time presence rule**, asked at the moment grove writes a leaf.
///
/// Before writing a leaf of kind K, K must resolve to exactly one complete
/// command composed from active personal policy and optional local overrides
/// (`docs/adr/complete-session-configuration.md`). This replaces the
/// all-nineteen completeness check, which grove can no longer make: nothing here
/// enumerates the kinds a methodology declares — since `open-kind-k20` there is
/// no enumeration to make it from — so the only honest question is about the
/// kind in hand.
///
/// It runs **before** the tree is opened, so a refusal leaves the tree
/// byte-identical and takes no exclusive lock on the way — and it loads the
/// whole configuration to ask. Both documents receive structural validation,
/// then effective bindings, routes, templates and values are checked after
/// composition. Invalid active policy fails even for an unrelated kind;
/// dormant definitions and unselected profiles need only be structurally valid.
fn require_declared(worktree: &Path, kinds: &[Kind]) -> Result<()> {
    let config = SessionConfig::load_for_worktree(worktree)?;
    for kind in kinds {
        config.require(kind.label()).with_context(|| {
            format!(
                "refusing to write a leaf of kind `{}`: no launch template resolves for it",
                kind.label()
            )
        })?;
    }
    Ok(())
}

````
<!-- /fragment -->

The rule it enforces is the just-in-time presence rule of
`docs/adr/complete-session-configuration.md`: before grove writes a leaf of kind
K, K must resolve to exactly one complete command composed from active personal
policy and optional local overrides. It replaced an older check that enumerated every kind a methodology
declared and confirmed the configuration was complete for all of them — a check
grove can no longer make, because since `docs/adr/a-kind-is-an-open-token.md`
grove holds no set of kinds to enumerate. The only honest question left is about
the kind in hand, and `config.require` asks exactly it. The refusal is the
worked example's, and its wording — the kind, and the file that must declare it —
is what `leaf_add_refuses_a_kind_no_template_resolves_for_and_mutates_nothing`
asserts.

Two facts about the fragment are easy to miss and both load-bearing. It runs
before the tree is opened — `require_declared` takes the worktree path, not a
`TreeWrite`, and every caller calls it above its `writable` — so a presence
refusal never waits on the exclusive lock and never mutates: measured under a
lock held from outside the process, an undeclared `leaf-add` returns its refusal
at once while a declared one blocks on the lock and prints *waiting for active
Grove tree operation*. And it loads the **whole** configuration to ask about one
kind. `SessionConfig::load_for_worktree` structurally validates both documents
and semantically validates their active composition before checking presence.
An invalid template reached by an effective binding fails even if the requested
kind does not use it; a dormant command definition is not compiled. Duplicate
routes in one patch fail structurally. These failures precede the requested
kind's presence check, while unfinished inactive profiles can coexist with a
working selection.

`slug` is the other name a grow verb reads, and it is read the same way, by its
own type.

<!-- fragment «slug» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="853-860" parent="presence-rule-and-slug" -->
````rust
/// A slug argument, as the grammar's own type.
///
/// One type owns the name (principle 3), so the text is read here — where an
/// operator typed it — and every verb below takes the validated [`Slug`].
fn slug(text: &str) -> Result<Slug> {
    Slug::new(text).map_err(|error| anyhow::anyhow!("slug {text:?}: {error}"))
}

````
<!-- /fragment -->

One type owns the name, so the operator's text is read here — where they typed
it — and every grow verb below takes the validated `Slug`. A malformed slug is
refused with the grammar's shape message naming the character it refused, before
any lock; `add_rejects_invalid_slug` in `crates/grove-llm/tests/leaf.rs` requires
the slug diagnostic, and the character and the order are held by the source.
That is the first order, text before lock, in the smallest possible frame: a
function that turns a `&str` into a `Slug` or an error and touches no tree.

The composite that reassembles the two helpers is stated here.

<!-- fragment «presence-rule-and-slug» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="824-860" parent="source-command-surface" -->
<!-- insert «require-declared» -->
<!-- insert «slug» -->
<!-- /fragment -->

The `--kind` token is read by `parse_kind`, and its help is one constant shared
by the two verbs that create a leaf, so the three are read together here. The
help teaches the shape of a kind and lists none, because there is no list.

<!-- fragment «kind-help» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="331-344" parent="kind-help-and-parse-kind" -->
````rust
/// `--kind` help for the two verbs that *create* a leaf. One const rather than
/// two hand-copied copies.
///
/// **It teaches the shape and lists nothing**, because there is nothing to list:
/// a kind is any well-formed token, and the set lives in the installed
/// methodology rather than in this binary
/// (`docs/adr/a-kind-is-an-open-token.md`). What the help owes instead is the
/// two facts a caller cannot guess — what the grammar accepts, and that a kind
/// no template declares is refused before the tree moves.
const KIND_HELP: &str = "Leaf kind, written into the filename: lowercase ASCII letters, \
digits and single dashes, no `--`. Grove holds no list of kinds — the installed methodology \
does — so any well-formed token is accepted here and refused later if no launch template \
declares it. `finish` is driver-reserved and refused by this verb";

````
<!-- /fragment -->

`KIND_HELP` is one constant rather than two hand-copied strings, and its job is
the two facts a caller cannot guess: what the grammar accepts, and that a kind no
template declares is refused before the tree moves. It lists nothing because a
kind is any well-formed token and the set lives in the installed methodology, not
in this binary — the point `docs/adr/a-kind-is-an-open-token.md` settles.
`leaf_add_help_exposes_no_routing_flags` in
`crates/grove-llm/tests/composition_verbs.rs` reads the rendered help and
requires that it names `--kind` and lists no kind set.

`KIND_OVERRIDE_HELP` is the same help written for the one verb whose `--kind`
overrides rather than supplies.

<!-- fragment «kind-override-help» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="345-349" parent="kind-help-and-parse-kind" -->
````rust
/// [`KIND_HELP`] for `leaf-decompose`, whose `--kind` overrides an inherited
/// kind rather than supplying a default.
const KIND_OVERRIDE_HELP: &str = "Override the first child's filename kind — any session kind \
except driver-reserved finish. Default: inherit the kind of the leaf being decomposed";

````
<!-- /fragment -->

`leaf-decompose`'s `--kind` overrides the kind its first child would otherwise
inherit, so its help says *override* and names the default as inheritance rather
than a literal; *`leaf-decompose`: two openings, in order* reads the handler that
keeps that promise. `parse_kind` is what turns the token into a `Kind`.

<!-- fragment «parse-kind» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="350-358" parent="kind-help-and-parse-kind" -->
````rust
/// A `--kind` argument, as the grammar's own type.
///
/// The refusal is the grammar's — a shape refusal naming the character it
/// refused — with the flag quoted in front of it, so an operator sees which
/// argument was rejected as well as why.
fn parse_kind(token: &str) -> Result<Kind> {
    Kind::new(token).map_err(|error| anyhow::anyhow!("--kind {token:?}: {error}"))
}

````
<!-- /fragment -->

The refusal is the grammar's — a shape refusal naming the character it refused —
with the `--kind` flag quoted in front of it, so the operator reads which
argument was rejected as well as why. Measured, `--kind Impl` is refused naming
`'I'` and `--kind a--b` is refused naming the separator `--`;
`an_ill_formed_kind_is_refused_by_shape_and_names_the_character` in
`composition_verbs.rs` requires the character and requires that no kind set is
listed. What the grammar cannot say is whether a well-formed token names a kind
that *exists*: `frobnicate` parses here and is refused, if at all, only by
`require_declared`. The `--kind` flag is required on `leaf-add` and
`leaf-insert` with no default, because a default is a kind literal under a
friendlier name — and it is the one that produced a *wrong* leaf rather than an
error, an `impl` leaf for a session that forgot the flag, before `open-kind-k20`
removed it. `leaf_add_refuses_to_guess_a_kind` requires clap's
missing-argument error and an untouched tree.

The composite that reassembles the block is stated here.

<!-- fragment «kind-help-and-parse-kind» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="331-358" parent="source-command-surface" -->
<!-- insert «kind-help» -->
<!-- insert «kind-override-help» -->
<!-- insert «parse-kind» -->
<!-- /fragment -->

<a id="the-atomic-run"></a>
## `leaf-add`: an atomic run, printed after it lands

`cmd_leaf_add` is the worked example read as source. It parses the kind list, the
slug and the parent, asks the presence rule, takes the exclusive opening, and
prints — in that order, and the order is the whole of what the handler adds to
the call.

<!-- fragment «handler-leaf-add» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="635-649" parent="handlers-growing" -->
````rust
fn cmd_leaf_add(args: &LeafAddArgs) -> Result<()> {
    let worktree = worktree()?;
    let kinds = args
        .kind
        .iter()
        .map(|token| parse_kind(token))
        .collect::<Result<Vec<_>>>()?;
    let slug = slug(&args.slug)?;
    let parent = Reference::parse(&args.parent)?;
    require_declared(&worktree, &kinds)?;
    let tree = writable(&worktree)?;
    print_paths(&verbs::leaf_add(&tree, &parent, &slug, &kinds)?);
    Ok(())
}

````
<!-- /fragment -->

The kinds are a `Vec`, because `leaf-add` writes one leaf per `--kind` in the
order given, as one unit — the shape a research vendor pair is cut in. Every
kind is parsed and then every one is passed to `require_declared`, so a list
that names one undeclared kind lands none of its leaves;
`a_kind_list_requires_every_one_of_its_kinds` in `session_kind_presence.rs`
gives it `research-a research-b combine-research` with only two declared and
requires the whole run refused. The parse-then-presence order also decides which
refusal an operator sees when two things are wrong at once: the kinds parse
first, so `leaf-add . "Bad Slug" --kind Impl` is refused for the kind, while
`leaf-add . "Bad Slug" --kind prototype` — a well-formed but undeclared kind —
is refused for the slug, which `slug` reaches before `require_declared` is
called. Both were measured. The parent is parsed by `Reference::parse` here but
resolved against the tree only inside `verbs::leaf_add`, under the lock, so
*no entry matches* for a bad parent is the call's refusal and comes after the
presence check.

`print_paths` is the last line, and its placement is a promise the help makes.

<!-- fragment «print-paths» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="650-659" parent="handlers-growing" -->
````rust
/// Print an add's paths — **after** the mutation succeeded, never as each leaf
/// lands. A run that fails is rolled back, so stdout describing a shape the
/// command reported as failed would be describing files that are no longer
/// there.
fn print_paths(paths: &[PathBuf]) {
    for p in paths {
        println!("{}", p.display());
    }
}

````
<!-- /fragment -->

The paths are printed **after** the call returned, never as each leaf lands,
because a run that fails is rolled back and stdout that described a leaf the
command then reported as failed would name a file that is no longer there.
`a_failed_run_prints_no_path_at_all` in `composition_verbs.rs` builds a run that
validates and then fails on a name obstruction — a directory wearing a leaf's
name — and requires that stdout is empty and no half-built shape is left behind.
That is the help's *nothing at all if the run could not be created*, kept by one
line's position.

`LeafAddArgs` is the argument struct, and it carries the `required` flag the help
turns on.

<!-- fragment «args-leaf-add» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="359-376" parent="args-growing" -->
````rust
#[derive(Parser)]
pub struct LeafAddArgs {
    /// Parent node — `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path.
    pub parent: String,
    /// Slug every leaf of this call carries (lowercase ASCII letters, digits,
    /// dashes).
    pub slug: String,
    /// One leaf is written per occurrence, in the order given.
    ///
    /// **Required, and it used to default to `impl`.** A default is a kind
    /// literal under a friendlier name, and it is the one that would silently
    /// produce a *wrong* leaf rather than an error — a session that forgot the
    /// flag got an `impl` leaf and no complaint (`open-kind-k20`).
    #[arg(long = "kind", required = true, help = KIND_HELP)]
    pub kind: Vec<String>,
}

````
<!-- /fragment -->

`parent` and `slug` are positional and `kind` is a repeatable required flag;
`required = true` is what makes clap refuse a bare `leaf-add . survey` with the
missing-argument error rather than default a kind. The comment records the defect
that made it required — a forgotten flag once produced a silent `impl` leaf — and
`leaf_add_refuses_to_guess_a_kind` holds the fix.

<a id="the-renumber-and-the-lint"></a>
## `leaf-insert`: the renumber, and the lint outside the lock

`cmd_leaf_insert` reads its one kind, its slug and its target the same way, asks
the presence rule, and takes the exclusive opening; what differs is the report,
because an insert shifts siblings and leaves stale position-prefixed references
behind.

<!-- fragment «handler-leaf-insert» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="660-670" parent="handlers-growing" -->
````rust
fn cmd_leaf_insert(args: &LeafInsertArgs) -> Result<()> {
    let worktree = worktree()?;
    let kind = parse_kind(&args.kind)?;
    let slug = slug(&args.slug)?;
    let target = Reference::parse(&args.target)?;
    require_declared(&worktree, std::slice::from_ref(&kind))?;
    let tree = writable(&worktree)?;
    let inserted = verbs::leaf_insert(&tree, &target, &slug, &kind)?;
    report_insert(&tree, &args.slug, &inserted)
}

````
<!-- /fragment -->

The order is `leaf-add`'s exactly — `parse_kind`, `slug`, `Reference::parse`,
`require_declared`, `writable` — so a bad kind or slug is refused before the
lock; the target is only parsed before it, since `Reference::parse` refuses a
blank reference and nothing else, and the call resolves the target against the
tree under the lock, where a `target` that names no entry fails with *no entry
matches*. The one new thing is `report_insert`, which takes the `TreeWrite` and
the insert's own report, because the report is more than a path.

<!-- fragment «report-insert» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="671-715" parent="handlers-growing" -->
````rust
/// `leaf-insert`'s output: the new leaf's path on stdout, the renumber summary
/// and the cross-reference lint on stderr.
///
/// Keep the established CLI stream semantics: the standard print macros panic on
/// a broken stdout/stderr, while the cross-reference lint is the one output that
/// is allowed to come back empty — its own failures are the tree's, never the
/// sink's, because the insert has already landed by the time it runs. So the
/// hits go out through `writeln!(…).ok()` rather than `eprintln!`, and this is
/// the site that decides it: the verb hands back a list and holds no opinion
/// about the sink (`lint-lock-scope-k32`).
///
/// **Nothing here is written under a tree lock.** The verb's shared guard is
/// consumed by its own scan, so a stderr that has stopped draining blocks this
/// process and no other — where a lint that printed under the tree's exclusive
/// lock wedged every grove on the worktree behind a stalled harness.
fn report_insert(tree: &TreeWrite, slug: &str, inserted: &verbs::Inserted) -> Result<()> {
    println!("{}", inserted.path.display());
    let renumbered = &inserted.renumbered;
    if renumbered.is_empty() {
        eprintln!("leaf-insert {slug}: no siblings to renumber");
        return Ok(());
    }
    eprintln!(
        "leaf-insert {}: renumbered {} sibling{}:",
        slug,
        renumbered.len(),
        if renumbered.len() == 1 { "" } else { "s" }
    );
    for renumber in renumbered {
        eprintln!(
            "  {:02} -> {:02}  ({})",
            renumber.from_position,
            renumber.to_position,
            renumber.to_name()
        );
    }
    let hits = verbs::stale_cross_refs(tree, renumbered)?;
    eprintln!("cross-references to review (verb does not auto-rewrite):");
    let mut stderr = std::io::stderr();
    for hit in &hits {
        writeln!(stderr, "{hit}").ok();
    }
    Ok(())
}

````
<!-- /fragment -->

Three outputs come out of one insert, across two streams. The new leaf's path
goes to stdout, the renumber summary — one line per sibling whose ordinal shifted
— goes to stderr, and a cross-reference lint goes to stderr after it. Measured,
inserting at
`rate-limit-k4` shifts that sibling from `02` to `03`, prints the summary, and
lists a stale reference the fixture planted in the root brief;
`insert_at_start_shifts_root_siblings_up_by_one` in `leaf.rs` pins the shift and
the summary, and `insert_cascades_a_node_subtree_with_position_free_headers`
pins that a shifted node drags its whole subtree with zero file contents
rewritten, because in-file headers are position-free.

None of these three prints under a tree lock — a mutation consumes its guard and
releases the exclusive lock when the verb returns, so every stdout line on this
page runs after the lock is gone. What is distinctive about the lint is the flock
fact this chapter opened on. `verbs::stale_cross_refs` needs its own reading
opening to scan the tree for stale references, and holding the write opening
while it took a shared one would be the self-deadlock, because two file
descriptions on one directory do not share a lock. So it first calls `relinquish`
on the `TreeWrite`, which drops the write guard if one is still held — on this
path the insert already spent it, so `relinquish` is the discipline that makes
the deadlock unexpressible rather than a release that happens here. It then hands
the hits back as a value rather than printing them, so `report_insert` writes
them with no tree lock held, through `writeln!(…).ok()` rather than the panicking
`eprintln!` the other lines use. A sink that has stopped draining therefore
blocks this one printing process and no other; under the shape this replaced, a
stalled harness wedged every grove process on the worktree behind the exclusive
lock. No test asserts that the lint may fail silently on a broken sink — it is a
property of the source: the hits are a returned value and the write is `.ok()`d.

`LeafInsertArgs` carries the same required `--kind` as `leaf-add`, for the same
reason.

<!-- fragment «args-leaf-insert» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="377-388" parent="args-growing" -->
````rust
#[derive(Parser)]
pub struct LeafInsertArgs {
    /// Target entry — the existing leaf or node (by key or path) whose slot the
    /// new leaf should occupy; the target and later siblings shift up by one.
    pub target: String,
    /// Slug for the new leaf (lowercase ASCII letters, digits, dashes).
    pub slug: String,
    /// Required, for the reason `leaf-add`'s is (`open-kind-k20`).
    #[arg(long = "kind", required = true, help = KIND_HELP)]
    pub kind: String,
}

````
<!-- /fragment -->

`target` is the existing entry whose slot the new leaf takes; the target and its
later siblings shift up by one. `insert_requires_an_existing_target` in `leaf.rs`
holds that inserting past the last sibling is refused — that is `leaf-add`'s job
— so the target must exist.

<a id="two-openings"></a>
## `leaf-decompose`: two openings, in order

`leaf-decompose` turns a leaf into a node, and its first child inherits the
decomposed leaf's kind unless `--kind` overrides it. To ask the presence rule
about the kind it will actually write, the handler has to read that inherited
kind — and reading it means opening the tree, which is a second opening the
first order has to keep clear of the exclusive one.

<!-- fragment «handler-leaf-decompose-head» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="716-722" parent="handlers-growing" -->
````rust
fn cmd_leaf_decompose(args: &LeafDecomposeArgs) -> Result<()> {
    let worktree = worktree()?;
    // `None` (the default) inherits the decomposed leaf's own kind; `--kind`
    // overrides it (task-kind-taxonomy).
    let kind_override = args.kind.as_deref().map(parse_kind).transpose()?;
    let leaf_path = normalize_leaf_path(&args.leaf_path);
    let first_child = slug(&args.first_child_slug)?;
````
<!-- /fragment -->

The head parses what the operator typed: the `--kind` override if present, the
leaf path, and the first child's slug — text before lock, as everywhere. The
next fragment is the one that has to read the tree before it writes, and its
comment is the flock fact applied.

<!-- fragment «handler-leaf-decompose-kind» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="723-754" parent="handlers-growing" -->
````rust
    // The first child's kind, resolved *before* the mutation so the presence
    // rule can be asked about the kind this call will actually write. With no
    // `--kind` that is the decomposed leaf's own, read off its filename — the
    // same answer `leaf_decompose` will reach for itself, and the only one it
    // ever reaches.
    //
    // A kind that cannot be read is left to the verb: `leaf-decompose` refuses a
    // brief, a retired leaf and a malformed name with its own message, and a
    // presence check that errored first would replace those refusals with a
    // complaint about configuration.
    //
    // **This read happens before the tree is opened for writing, and the order
    // is load-bearing.** Both take a lock on the same directory through their
    // own file description, and two descriptions do not share an `flock` — so
    // reading the inherited kind while holding the write opening would block
    // this process against itself, forever. `writable` is therefore the last
    // thing before the verb, here and at every other call site.
    let child_kind = match &kind_override {
        Some(kind) => Some(kind.clone()),
        None => inherited_kind(&worktree, &leaf_path),
    };
    if let Some(kind) = &child_kind {
        require_declared(&worktree, std::slice::from_ref(kind))?;
    }
    let tree = writable(&worktree)?;
    let decomposed =
        verbs::leaf_decompose(&tree, &leaf_path, &first_child, kind_override.as_ref())?;
    println!("{}", decomposed.brief.display());
    println!("{}", decomposed.first_child.display());
    Ok(())
}

````
<!-- /fragment -->

With no `--kind`, the first child's kind is the decomposed leaf's own, read off
its filename by `inherited_kind`, and that read is `require_declared`'s subject,
so it has to happen before the presence check and therefore before the tree is
opened for writing. The comment states the order and its reason: reading the
inherited kind holds a shared lock through its own file description, and if the
handler held the write opening at the same time the two descriptions would not
share the `flock`, so the read would block this process against itself forever.
`writable` is therefore the last thing before the call, here and at every other
grow verb. Measured under an external lock, `leaf-decompose` inheriting its
kind waits on the shared read and completes on release, while `leaf-decompose`
with an undeclared `--kind` override is refused at once, before either opening —
the two openings are sequential, never nested. A kind that cannot be read is left
to the verb: `leaf-decompose` refuses a node file, a retired leaf and a malformed
name with its own message, and a presence check that errored on an unreadable
kind first would replace those refusals with a complaint about configuration.
`a_verbs_own_refusal_is_not_replaced_by_a_configuration_complaint` in
`session_kind_presence.rs` decomposes a brief and requires the brief refusal to
survive, and `leaf_decompose_asks_about_the_kind_its_first_child_will_carry`
requires the inherited kind to be the one the presence rule asks about.

`inherited_kind` is the read itself, and it takes the loop's `read` opening
because *no tree* is an answer it can use.

<!-- fragment «inherited-kind» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="755-765" parent="handlers-growing" -->
````rust
/// The kind a `leaf-decompose` with no `--kind` will write, or nothing readable.
fn inherited_kind(worktree: &Path, leaf_path: &Path) -> Option<Kind> {
    let Ok(Reading::Tree(tree)) = grove_loop::read(worktree) else {
        return None;
    };
    match verbs::kind(&tree, Some(leaf_path)) {
        Ok(Sought::Match(kind)) => Some(kind),
        _ => None,
    }
}

````
<!-- /fragment -->

It reads through `grove_loop::read`, matches only `Reading::Tree`, and turns
every other answer — a vacant root, an unreadable name, a path that is not a leaf
— into `None`, because a kind it cannot read is one it leaves the verb to refuse.
`decompose_with_no_kind_flag_gives_the_first_child_the_parent_leafs_kind` in
`crates/grove-llm/tests/leaf_ops.rs` measures the inherited case landing a child
of the parent's kind, and `decompose_kind_flag_overrides_the_parent_leafs_kind`
the override; `decompose_rejects_a_brief` and `decompose_rejects_a_retired_leaf`
hold the two refusals the handler leaves to the verb.

`LeafDecomposeArgs` carries the optional `--kind`, whose help is the override
constant read above.

<!-- fragment «args-leaf-decompose» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="389-399" parent="args-growing" -->
````rust
#[derive(Parser)]
pub struct LeafDecomposeArgs {
    /// Leaf path. Absolute, or relative to the grove root (`.grove/`).
    pub leaf_path: PathBuf,
    /// Slug for the node's first child (lowercase ASCII letters, digits,
    /// dashes).
    pub first_child_slug: String,
    #[arg(long = "kind", help = KIND_OVERRIDE_HELP)]
    pub kind: Option<String>,
}

````
<!-- /fragment -->

`kind` is an `Option`, and `None` — the default — is what inherits; the handler's
`match` on `&kind_override` above is what turns that `None` into the
`inherited_kind` read.

The composite that reassembles the three grow argument structs is stated here.

<!-- fragment «args-growing» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="359-399" parent="source-command-surface" -->
<!-- insert «args-leaf-add» -->
<!-- insert «args-leaf-insert» -->
<!-- insert «args-leaf-decompose» -->
<!-- /fragment -->

The composite that reassembles the four handlers and their helpers, in source
order, is stated here.

<!-- fragment «handlers-growing» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="635-765" parent="source-command-surface" -->
<!-- insert «handler-leaf-add» -->
<!-- insert «print-paths» -->
<!-- insert «handler-leaf-insert» -->
<!-- insert «report-insert» -->
<!-- insert «handler-leaf-decompose-head» -->
<!-- insert «handler-leaf-decompose-kind» -->
<!-- insert «inherited-kind» -->
<!-- /fragment -->

<a id="the-four-contracts"></a>
## The four contracts, as `--help` states them

The four variants are the grow verbs' doc comments, which are the `--help` a
session reads and the guide paraphrases for the human; this page reproduces them
because they are corpus, and reads them for what the code keeps of each promise
and where the help says more than the code enforces. `RootInit` is the enum's
first variant and a grow verb, so its nine lines sit inside the enum ahead of
*Reading the tree*'s fifty; the comment fixes the kind at `requirements` and
promises the load-bearing thing `after_root_init_pick_returns_the_new_leaf_not_done`
holds — that `pick` returns the new leaf rather than reporting the grove
finished. The help also states that initialization creates the charter and leaf
together and that the driver refuses a charter-only root as taskless. This
lifecycle check belongs to the driver; helper selection does not perform it.
It also spells `.grove` for display, the third of the module's three
display spellings after *The grammar and the openings* and *Reading the tree*,
and none of the three reaches a call.

<!-- fragment «verb-root-init» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="66-74" parent="source-command-surface" -->
````rust
    /// Scaffold a brand-new grove's tree: create `.grove/`, write the root
    /// `_BRIEF.md` charter, and lay down a first **requirements** leaf
    /// `01-requirements--<slug>-k1.md` (default slug `plan`) — the kind is fixed, since the
    /// bootstrap session's only input is the human's own words. After this,
    /// `grove-llm pick` returns the new leaf. The charter and leaf are created
    /// together; the driver refuses a root holding only its charter as taskless.
    /// Refuses if `.grove/` already exists. Working-tree change only —
    /// no commit. Prints the root node file's path, then the leaf's.
    RootInit(RootInitArgs),
````
<!-- /fragment -->

`LeafAdd`'s comment is the longest doc comment in the file, and most of it is
methodology rather than mechanism: fifty lines on how a research vendor pair and
a review chain are cut, and where an integration is placed. That methodology is
the guide's — [Review composition and escalation](../../USAGE.md#usage-review-composition)
— and the page links it rather than explaining it, because the book's boundary
is the call. What the code keeps of it is the part the page owns: the run is
atomic, so a list lands all its leaves or none; a run that could not be created
prints nothing; and where an integration lands is the caller's to decide, which
the verb enforces nothing of.

<!-- fragment «verbs-leaf-add-help» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="125-176" parent="verbs-growing" -->
````rust
    /// Append **one leaf per `--kind`**, in the order given, under `<parent>` at
    /// the next gapless child positions with consecutive fresh permanent keys —
    /// as one unit. `<parent>` is `.` for the grove root, or a node by its key
    /// (`[n]` / `n` / `<slug>-k<key>`) or its path. `--kind` is required and
    /// repeatable; every leaf of one call carries the same slug, because the kind
    /// is what tells a shape's steps apart. Prints each new leaf's absolute path
    /// on stdout in position order — and nothing at all if the run could not be
    /// created. Working-tree change only — no commit.
    ///
    /// **A list is how a research vendor pair is cut**, and the whole reason the
    /// list exists: `leaf-add <parent> <stem> --kind research-a --kind research-b
    /// --kind combine-research` lands three flat siblings at consecutive
    /// positions, all slugged `<stem>`, or lands none of them. Three separate
    /// calls are three chances to stop half way, and a live prefix of a pair is
    /// indistinguishable from a deliberately hand-cut partial one — so the run is
    /// atomic here rather than a rule the caller keeps.
    ///
    /// **A pair is created eagerly where a review chain is created lazily, and
    /// the asymmetry is deliberate.** If `research-a` cut `research-b` at the end
    /// of its own session, `b` would inherit `a`'s framing and corpus, destroying
    /// the independence the pair is run for. Cut one when a question is
    /// load-bearing enough to pay for two independent corpora and blind spots.
    ///
    /// This is also how a **review chain** is built — one step at a time, each
    /// created only when it is required. A producer's last act is
    /// `leaf-add <parent> <stem> --kind review-<producer>` if review is
    /// warranted; the review's last act is
    /// `leaf-add <parent> <stem> --kind integrate-review-<producer>`
    /// if it found something worth acting on. Every step carries the **same
    /// bare stem**: the kind states the step's role, so the slug names only the
    /// artifact. The steps are ordinary **flat
    /// siblings** — there is no chain node — and the creating session writes the
    /// new leaf's body, which is the point of creating it late: it can put the
    /// exact uncovered case, finding, or datum into it, which is strictly more
    /// than a constructor writing up front could know. A review that finds
    /// nothing creates nothing and simply retires.
    ///
    /// **The integrate step is the one that cares where it lands.** This verb
    /// appends at the parent's *end*, which is right for it only when nothing
    /// later in that directory would run first. A review's findings are anchored
    /// to file-and-line positions an intervening leaf can move silently, so scan
    /// the review's own parent for the first sibling **entry** after it whose
    /// subtree still holds live work — a live leaf, or a node directory with one
    /// anywhere beneath it, since `pick` descends a node in place. If there is
    /// one, cut the integration with `leaf-insert <that entry> …` instead,
    /// targeting the entry itself and never a leaf inside it. Later terminal
    /// leaves, nodes whose subtree is wholly terminal, and the `finish` sentinel
    /// do not count; nor does anything outside the directory, because the walk
    /// finishes it — including this appended leaf — before any later sibling of
    /// an ancestor. A `review-*` step re-derives its citations from the
    /// producer's commit and needs none of this care.
    LeafAdd(LeafAddArgs),
````
<!-- /fragment -->

`a_review_chain_is_three_leaf_adds_landing_as_flat_siblings` in
`composition_verbs.rs` holds that the three steps are ordinary appends with a
shared stem and no chain node, and
`a_kind_list_lands_three_flat_siblings_at_consecutive_positions_and_keys` holds
the pair as one atomic run. The placement rule the help spends a paragraph on is
held by the scheduling tests in the same file, which pin what `pick` selects
after an insert rather than a filename adjacency — but those are the loop's
walk, and the book names the rule and points at the guide for it.

`LeafInsert`'s comment is the renumber contract the handler was read keeping:
the shift is one rename per sibling, position-free headers mean zero file
contents change, and the stale cross-references go to stderr.

<!-- fragment «verbs-leaf-insert-help» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="177-204" parent="verbs-growing" -->
````rust
    /// Insert a new leaf at the slot held by `<target>`, shifting `<target>` and
    /// every later sibling up one position. `<target>` is an existing leaf or
    /// node by its key or path. Because the hierarchy lives in directories, each
    /// shift is a single plain rename of one sibling directory, and the whole
    /// subtree — child names *and* keys — rides along untouched; in-file `# …`
    /// headers are position-free, so **zero file contents** are rewritten. The new leaf
    /// gets a fresh key. Prints the new leaf's absolute path on stdout; the
    /// renumber summary and stray position-prefixed cross-references go to
    /// stderr. Working-tree change only — no commit.
    ///
    /// Use it for new work that must sequence ahead of live leaves — and, as the
    /// **default**, for a **review chain's integrate step**. A review's findings
    /// are anchored to a commit and to file-and-line positions; an intervening
    /// leaf that edits a cited file moves them without erroring, and the
    /// integrating session then re-derives the reviewer's intent from a tree the
    /// reviewer never saw.
    ///
    /// The target there is **the first sibling entry after the review whose
    /// subtree still holds live work**, read in the review's own parent
    /// directory. An *entry*, not a leaf: `pick` descends a node directory in
    /// place, so a later sibling node with a live leaf anywhere beneath it
    /// blocks too — and that node is the target, never the live leaf inside it,
    /// which would insert one level down. Terminal leaves, nodes whose subtree
    /// is wholly terminal, and the driver's `finish` sentinel never block. Plain
    /// `leaf-add` is correct there only when no sibling entry blocks: the walk
    /// finishes the review's own directory, appended leaf included, before any
    /// later sibling of an ancestor.
    LeafInsert(LeafInsertArgs),
````
<!-- /fragment -->

The clause *stray position-prefixed cross-references go to stderr* is
`report_insert`'s lint, and *zero file contents are rewritten* is what
`insert_cascades_a_node_subtree_with_position_free_headers` pins. The second half
of this comment, like `leaf-add`'s, is the integration-placement methodology, and
the page links the guide for it and explains only that placement is the caller's.

`LeafDecompose`'s comment is the shortest of the four and the one whose promise
the handler's two openings exist to keep: the first child inherits the decomposed
leaf's kind unless `--kind` overrides it.

<!-- fragment «verbs-leaf-decompose-help» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="205-216" parent="verbs-growing" -->
````rust
    /// Convert a live leaf file `NN-<kind>--<slug>-k<key>.md` into a node **directory**
    /// `NN-k<key>/` (**key preserved** — the leaf that was `k<key>`
    /// becomes the node `k<key>`), moving the leaf body in as the node's
    /// `_<slug>.md` (a plain rename on every lane, staging nothing; its
    /// `# <slug>-k<key>` header retitled with ` — brief`) and
    /// atomically growing a first child
    /// `01-<kind>--<first-child-slug>-k<new>.md` so the
    /// node is never childless. The first child **inherits the decomposed
    /// leaf's own kind** unless `--kind` overrides it.
    /// Prints the node file's absolute path then the first child's, one per line.
    /// Working-tree change only — no commit.
    LeafDecompose(LeafDecomposeArgs),
````
<!-- /fragment -->

*Key preserved* is what
`decompose_converts_leaf_into_node_directory_with_first_child` pins — the leaf
that was `k<key>` becomes the node `k<key>` — and *inherits the decomposed leaf's
own kind unless `--kind` overrides it* is the promise `inherited_kind` reads the
tree to keep. The table gathers the four verbs against the two orders and the
line that keeps each promise, and the closing chapter's row for each is built
from it.

| Verb | Text it parses before the lock | Presence it asks | Kept at | Held by |
|---|---|---|---|---|
| `root-init` | slug (`slug`), kind fixed at `requirements` | `require_declared` on `requirements` | lines 486–488 before `write` | `root_init_asks_about_the_requirements_leaf_it_mints` |
| `leaf-add` | kinds (`parse_kind`), slug, parent | every kind in the list | line 644 before `writable` | `leaf_add_refuses_a_kind_no_template_resolves_for_and_mutates_nothing`, `a_kind_list_requires_every_one_of_its_kinds` |
| `leaf-insert` | kind, slug, target | the one kind | line 665 before `writable` | `leaf_insert_asks_the_same_question` |
| `leaf-decompose` | override kind, leaf path, child slug | the inherited or overridden kind | line 745 before `writable` | `leaf_decompose_asks_about_the_kind_its_first_child_will_carry` |

The composite that reassembles the three grow variants is stated here.

<!-- fragment «verbs-growing» owner="before-the-lock" source="crates/grove-llm/src/cli.rs" lines="125-216" parent="source-command-surface" -->
<!-- insert «verbs-leaf-add-help» -->
<!-- insert «verbs-leaf-insert-help» -->
<!-- insert «verbs-leaf-decompose-help» -->
<!-- /fragment -->

The four verbs that write have now been read, and each parses its text and asks
the presence rule before it takes the exclusive lock, so a refusal at either step
changes nothing on disk. What a session runs after the tree is grown is the two
terminal marks, and the next chapter reads them — the last tree verbs a session
runs, which say on stderr what remains.

[Previous: Reading the tree](03-reading-the-tree.md) | [Contents](README.md) | [Next: Ending work](05-ending-work.md)
