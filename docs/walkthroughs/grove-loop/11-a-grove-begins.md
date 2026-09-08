# A grove begins
<!-- book-page id="a-grove-begins" slice="never-mistaken-for-finished" order="11" -->
[Previous: Growing: leaf-add and leaf-insert](10-growing.md) | [Contents](README.md) | [Next: A leaf becomes a node](12-leaf-to-node.md)

<a id="never-mistaken-for-finished"></a>
## The rule: a fresh grove starts with one live leaf

Every chapter so far has been handed a tree. Chapter 5 opened one, chapter 6
addressed inside one, chapters 7 to 9 read one, and chapter 10 grew one. This
chapter is where there is **no tree yet** — a worktree with no `.grove/` in it —
and the layer has to make the first one.

That sounds like the easiest thing in the file and it is the strictest, because
of what a grove *means* the moment it exists. `pick` answers the first live leaf
in walk order, and a tree with no live leaf is a search that matched nothing —
which is the same answer the driver reads as *there is nothing left to do*. So a
grove that came into existence holding only its charter would be indistinguishable
from a grove that had finished. **The first leaf is not a convenience; it is what
keeps a new grove from looking finished** — so creating it in a *second* step
would put a finished-looking grove on disk for as long as that step took, visible
to anything that read the tree in between.

> The library beneath grove can create a tree and place entries in it. It has no
> word for *the entry that must exist for the tree not to read as done*, because
> it has no word for *done*. So the obligation to create a grove **whole** is
> grove's, and the way it is discharged is a store operation grove chose rather
> than a sequence grove wrote.

This is the third part of the what-could-not-move test — *on the way out, the
policy* — met for the first time as a **creation** rather than a check. What the
layer chooses here is not a value in a config file: it is a **shape**, the
smallest tree that is not a finished one, and it chooses it by refusing to let
any smaller one exist even briefly.

The carried example reaches its eleventh step, and it is the first one with
nothing behind it.

```text
<worktree>/                             an empty worktree — no .grove at all
  └─ (nothing)

root_init(vacancy, "plan", requirements)

<worktree>/.grove/                      created, its charter and its leaf, in ONE operation
├── BRIEF.md                            "# my-grove — brief" + five empty headers
└── 01-requirements--plan-k1.md         "# plan-k1" — position 01, key 1, LIVE

  ⇒ Ok(vec![ <root>/BRIEF.md,
             <root>/01-requirements--plan-k1.md ])   charter first, then the leaf
```

The tree those two paths describe is the one chapter 7 walked and chapter 10
grew a sibling into. The slug is `plan` and the key is `1` because that is what
the code writes, not because the example chose them — and the whole of this
chapter is the argument that the intermediate state, a `.grove/` holding only
`BRIEF.md`, is a shape grove can no longer produce.

<a id="one-operation-or-none"></a>
## One operation, or none

The chapter's first ownership block is `tree_lifecycle.rs` lines 332 to 489 —
158 lines, and the file's second production concern rather than its first. The
file opens on *finishing*, which is chapter 14's; the book opens on beginning.
That inversion is deliberate and it is stated in the structure brief: the file is
ordered by Rust convention and the book by concept, and this root is the one where
the two disagree most.

<!-- fragment «grove-beginning» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="332-489" parent="source-tree-lifecycle" -->
<!-- insert «grove-beginning-root-init» -->
<!-- insert «grove-beginning-default-slug» -->
<!-- insert «grove-beginning-initialize» -->
<!-- insert «grove-beginning-root-shape-type» -->
<!-- insert «grove-beginning-root-shape-fn» -->
<!-- /fragment -->

`root_init` is three lines of body, and every one of them is a delegation.

<!-- fragment «grove-beginning-root-init» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="332-350" parent="grove-beginning" -->
````rust
/// `root-init [<slug>]`: scaffold a fresh grove under `worktree/.grove` — the root
/// `BRIEF.md` (the one unkeyed singleton) and a first **requirements** leaf
/// `01-requirements-<slug>-k1.md`. Returns the absolute paths created:
/// `BRIEF.md`, then the leaf. Refuses to clobber an existing `.grove/`.
/// Working-tree only — no commit.
///
/// The kind is fixed, with no `--kind` flag: a brand-new grove's first session
/// takes the human's own words as its only input — nothing else is on disk —
/// which is the generating rule for **HITL**, and `requirements` is the kind
/// that rule names (`docs/ARCHITECTURE.md#task-kind-taxonomy`). It
/// being fixed is also load-bearing for routing: the loop driver launches the
/// `start` session *before* this verb has run, so it can only route the
/// bootstrap by construction (`fresh-grove-start-contract`).
pub(crate) fn root_init(vacancy: TreeVacancy, slug: &Slug, kind: &Kind) -> Result<Vec<PathBuf>> {
    task_grow::refuse_finish_kind(kind, "root-init")?;
    let name = grove_name(vacancy.root());
    initialize_grove(vacancy, &name, slug, kind)
}

````
<!-- /fragment -->

**The signature is where the guarantee is.** It takes a `TreeVacancy`, not a
path and not a `Guard`. Chapter 5 read the opening that produces one: `write_or_vacancy`
answers `Opening::Tree` over a root that holds a grove and `Opening::Vacancy` over
one that does not, and the two are different types. So *`root-init` cannot clobber
a live grove* is not a check this function performs — there is no branch here that
could fail that way. A caller holding a `TreeVacancy` is a caller that already
looked and found nothing, under the store's own lock, and the lock is still held.
The refusal a reader expects to find in this function is in the type of its first
argument.

**The kind is fixed and the doc comment says why twice.** `requirements` is not a
default that a flag could override; there is no flag. The first reason is the
methodology's — a brand-new grove has nothing on disk but the human's own words,
which is the generating rule for a human-in-the-loop kind. The second is the
crate's own, and it is the load-bearing one: the loop driver launches the `start`
session **before** this verb has run, so it has no leaf to read a kind off and can
only route the bootstrap by construction. A configurable first kind would make
that routing unimplementable, and the comment names the contract
(`fresh-grove-start-contract`) rather than restating it.

The one thing `root_init` does itself is the refusal on line 346, and it is worth
naming precisely: `refuse_finish_kind` is chapter 10's function, and what it
refuses is the driver's reserved `finish` kind. **Nothing observes it here** — the
measurement is in *What the refusals are worth, measured* below, and the reason is
that the reserved kind cannot reach it. `verbs::root_init` is the only public door
to this function, and enumerating everything that reaches
`tree_lifecycle::root_init` across `crates/` finds five call sites —
`grove-llm`'s `cmd_root_init`, two integration fixtures, the excluded
`task_grow/tests.rs`, and this file's own `root_init_at` — every one of which
passes `Kind::requirements()` as a literal. The flag that would let an operator
supply `finish` does not exist, which is the same sentence the doc comment opens
with, read as a coverage fact.

<!-- fragment «grove-beginning-default-slug» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="351-356" parent="grove-beginning" -->
````rust
/// The slug the driver's own scaffold names a grove with; its one caller is
/// [`transition_to_current`]. `root-init` defaults in clap, held equal by a test.
fn default_root_slug() -> Slug {
    Slug::new(DEFAULT_ROOT_SLUG).expect("the default root slug is a valid slug")
}

````
<!-- /fragment -->

<a id="the-value-nothing-holds"></a>
### The value chosen twice, and what holds it

Five lines, and the reader should stop on them, because this is the *on the way
out — the policy* question at its smallest: **the layer chose a value, and the
book's own test asks whether a chosen value is stated where a reader can find
it.** It is stated here. It is also stated somewhere else, and what connects the
two statements is the rest of this section.

`DEFAULT_ROOT_SLUG` is `"plan"`, declared at line 56 of this file — which is
**chapter 14's block**, not this one, because the constants sit above the finishing
code the file opens on. `default_root_slug` wraps it in a `Slug`, and the `expect`
is honest: the constant is a literal this crate controls, so a failure there is a
build-time mistake rather than a runtime condition.

**The doc comment names its one caller, and until this book was finished it named
one the function does not have.** It opened *The slug `root-init` uses when nobody
supplied one* — and `root_init` takes `slug: &Slug` and never falls back.
Enumerating the call sites settles it: across `crates/`, `default_root_slug` and
`DEFAULT_ROOT_SLUG` occur at the definition, the constant, and exactly **one**
call — `transition_to_current`, at line 82, which is the driver's own scaffold and
was the comment's second clause. That clause was always exactly right; the first
was the defect, and `default-root-slug-two-spellings-k159` replaced it with the
caller the function has. `grove-llm root-init` with no argument gets its default
from a different place entirely: `#[arg(default_value = "plan")]`, in
`crates/grove-llm/src/cli.rs`.

So the value is chosen twice, in two crates, as two literals that happen to
match — and when this chapter was drafted **nothing held them to each other**.
That was not a reading of the code; it was a measurement. Changing the constant to
`"mutant"` in a copy of the workspace reddened exactly one test of the 558 the
suite then held, and it was
`transition_initializes_an_absent_grove_under_one_exclusive_guard` in this
chapter's own block, which asserts the picked leaf is `01-requirements--plan-k1.md`.
`root_init_default_slug_is_plan`, which drives the binary, stayed green throughout,
because it is pinned to the clap literal. Each spelling had an observer; their
agreement had none.

**That reading is superseded, and the re-run is a few paragraphs below.** The
suite has since grown to 560, and the *exactly one* above was taken without
relinking the mutant into the `grove` binary — so it is a lower bound rather than
a count. Read it as the finding it was for at the time, which is that the two
spellings had no shared observer, and take the numbers from the re-run.

**What k159 added is an observer of the agreement**, and the shape of it is worth
a reader's attention because it is the cheapest thing that could have worked.
`both_scaffolding_doors_name_the_first_leaf_the_same`, in
`crates/grove-llm/tests/root_init.rs`, opens both doors and compares what comes
out: it runs `root-init` with no argument against a fixture repository, calls
`grove_loop::driver::transition_to_current` against a bare directory, and asserts
the two first leaves carry the same name. It lives in `grove-llm` because that is
the only package that can reach both spellings. `DEFAULT_ROOT_SLUG` is private, so
what any test can observe is its *effect* through `driver::transition_to_current`;
the clap default is this package's own, reachable either by running the binary or
in-process through `grove_llm::cli`, which is why the library target exists at all.
`crates/grove` takes `grove-loop` as its only grove dependency and so reaches one
half; `grove-loop` cannot reach the other at all. And the test opens the driver's
door by calling it rather than through a verb, because `transition_to_current`
runs before any session exists and no verb exposes it — which is [chapter
15's](15-the-verbs.md#the-two-that-would-have-made-it-fourteen) reading of
`driver.rs`, not this chapter's.

**Re-running both mutations says the rung is no longer empty, and corrects the
first reading while it is here.** The harness is this chapter's, described under
[*What the refusals are worth,
measured*](11-a-grove-begins.md#what-the-refusals-are-worth-measured), with three
differences stated rather than smoothed. The copy carries the plugin marketplace,
so `the_namespace_is_the_shipped_plugin_entrys_declared_name` passes and the
control is ten rather than eleven — the same ten `prompt.rs` failures the section
below reports, without the eleventh that copy adds. Neither mutation touches that
test, so both counts hold against either control, and both were re-taken against
the eleven to check that they do. The two
crates' suite has grown to 560, 559 of them before k159 added its own. And **the
constant's mutant must be relinked into the `grove` binary before the suite runs**
— `cargo build -p grove --bins` after the edit, not before it — because
`crates/grove-loop/tests/driver_lease.rs` drives that binary from outside. Skip
that step and the run reuses a `grove` built from unmutated source, which is how
the *exactly one test* above was arrived at.

Against a control of 560 with those ten failures, the constant mutation reddens
**three** tests: the lock-count one above,
`both_scaffolding_doors_name_the_first_leaf_the_same`, and
`bare_scaffolding_is_anchored_before_the_configured_command_inherits_git_context`,
which scaffolds a grove by running the driver against a worktree that has none.
The clap mutation reddens **eight**, the new one among them. Their intersection is
a single test and it is the new one: the only observer in the suite that both
mutations reach, which is what *holding an agreement* means when the agreement is
between two literals nothing can merge. Take k159's test back out and the
intersection is empty, which is the finding above restated as a measurement rather
than as a reading.

The repair k159 did not make is worth naming, because a reader reaches for it
first. The CLI could have read the crate's constant instead, leaving one spelling
and nothing to keep in step — which is the preference [chapter
21](21-what-could-not-move.md#stated-twice) records this crate stating elsewhere,
about `VERSION`. It was not taken because `cli.rs` line 327 is the `grove-llm`
book's frozen corpus — it is a root of that book and of no other — so changing it
moves a reproduced fragment, that book's ledger and its validator run, plus the two
places this book quotes the literal in prose. **The route taken was cheaper, not
free**, and the difference is worth stating exactly: a test under
`crates/grove-llm/tests/` is a root of neither book, but the `grove-llm` book
*reads that directory twice* — a console transcript naming each test binary's
count, and a sentence totalling the integration tests — and both moved by one.
That is a cost argument rather than a correctness one, and naming its real price is
the honest thing a frozen corpus can do about a repair it priced out.

<a id="there-is-no-second-phase"></a>
### There is no second phase, and that is the deletion

Fifty-two lines, of which twenty-four are the doc comment. This is the densest
argument in the block, and the ratio is the point: the code is a delegation and
the comment is a history of what the delegation replaced.

<!-- fragment «grove-beginning-initialize» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="357-408" parent="grove-beginning" -->
````rust
/// Create the whole grove — the root, its `BRIEF.md` and the first
/// **requirements** leaf — as one store operation under the lock the vacancy
/// already holds. Returns the charter's path, then the leaf's.
///
/// # There is no second phase, and that is the deletion
///
/// This used to be two: grove created the root and its charter under a guard of
/// its own, released it, and appended the first leaf under the library's,
/// because the two `flock` the directory containing the root through different
/// open file descriptions and do not nest. The window between them was a real
/// tree shape — a root holding its charter and no keyed entry — and grove
/// carried a recovery for it. [`TreeVacancy::initialize`] closes the window by
/// removing the seam: the charter is the root's *distinguished child*, which the
/// store writes, and a failed `initialize` takes the root back down with it. So
/// the recovery went with the anomaly it repaired (`collapse-tree-access-k13`),
/// and what a torn tree gets now is [`RootShape::Taskless`]'s message.
///
/// # The key is predicted here, exactly as every grow verb predicts one
///
/// The leaf's body embeds its own `# <slug>-k<key>` handle, so the key has to be
/// chosen before the store allocates one. `initialize` places its entries by the
/// rule `append_many` uses over an empty tree — `Ordinal::FIRST` onward with keys
/// from 1 — so the prediction is 1, and [`task_grow::allocated`] holds the store
/// to it rather than trusting it.
fn initialize_grove(
    vacancy: TreeVacancy,
    name: &str,
    slug: &Slug,
    kind: &Kind,
) -> Result<Vec<PathBuf>> {
    let key = Some(Key::new(1));
    let leaf = task_grow::new_leaf(key, Outcome::Live, kind.clone(), slug);
    let report = vacancy
        .initialize(Some(root_brief_body(name).into_bytes()), vec![leaf])
        .map_err(task_tree::raised)?;
    // The charter first, then the leaf — `Report::created` is
    // distinguished-child-first, and the two are what `root-init` reports.
    let created = report.created();
    let Some((charter, positioned)) = created.split_first() else {
        bail!("the store initialized a grove and reported nothing created");
    };
    if !matches!(charter.name, TaskName::Brief) {
        bail!(
            "the store reported {} where the grove's charter was expected",
            charter.path.display()
        );
    }
    let mut paths = vec![charter.path.clone()];
    paths.extend(task_grow::allocated(positioned, &[key])?);
    Ok(paths)
}

````
<!-- /fragment -->

This is the function the chapter's rule lives in, and its doc comment does the
arguing, so the prose here connects rather than restates.

**What was deleted is the interesting part.** The comment describes a shape that
used to exist: grove created the root and its charter under a guard of its own,
released it, and appended the first leaf under the library's — two acquisitions,
because the two `flock` the same directory through different open file
descriptions and would deadlock nested. Between them stood a real tree on disk: a
root holding its charter and no keyed entry. That is the shape this chapter's rule
forbids, and grove used to carry a *recovery* for it, because grove itself was
what produced it.

`TreeVacancy::initialize` removes the seam rather than the recovery. The charter
is the root's **distinguished child**, which is the store's own concept, so the
store writes it; and a failed `initialize` takes the root back down with it. The
window is not narrowed, it is gone — and the repair went with the anomaly it
repaired. **A recovery deleted because its input became unproducible is a
different act from a recovery deleted because nobody hit it**, and the second half
of the file remembers which: `RootShape::Taskless` is what a torn tree gets now,
and its own doc comment says what reaching it implies.

**The prediction is the same one chapter 10 read, arriving from the other side.**
The leaf's body embeds its own `# <slug>-k<key>` handle, so the key has to be
chosen before the store allocates one; `initialize` places entries by the rule
`append_many` uses over an empty tree, so the prediction is 1. Chapter 10 met
`task_grow::allocated` as the function that holds a prediction to the store's
report, and named this function as the caller that explains why it takes a *slice*
of created rows rather than a whole report. Here is the code that clause was
about: `Report::created` here is
distinguished-child-first, so its first row is the charter — which carries no key
and could never satisfy a key prediction — and `split_first` is what separates the
charter from the positioned entries `allocated` is allowed to check.

**The two `bail!`s are grove disbelieving the store.** *Reported nothing created*
and *reported something other than a charter first* are both statements that the
library violated its own contract, and neither is an operator-facing refusal. That
is a different class from every refusal chapters 5 to 10 met, and it shows up in
the measurement below as arms no test can reach: **a test over an honest library
cannot construct a dishonest one.** Chapter 10 met the same class in `task_grow`
and drew the same line there.

<a id="three-shapes-two-refusals"></a>
## Three shapes, and two of them are refusals

The block's last two items classify a `.grove/` that already exists. They are
chapter 11's source and they have **no caller in this chapter** — `root_shape`'s
one call site is `transition_to_current`, at line 87, in chapter 14's block. The
type is here because it is *about* what a grove is, which is this chapter's
subject; the decision it feeds is chapter 14's.

<!-- fragment «grove-beginning-root-shape-type» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="409-443" parent="grove-beginning" -->
````rust
/// What an existing `.grove/` is, as far as the lifecycle transition is
/// concerned. Three shapes, and the classification is the whole of what the
/// transition decides now.
///
/// It used to be a byte-exact match against the deterministic fresh-tree content
/// under a missing `.grove/FORMAT`, because a witnessless root was *also* how a
/// legacy tree presented and the two got opposite treatment. Migration is gone
/// (`delete-migration-k6`), and the discrimination it needed went with it. Two
/// of the three shapes are now refusals rather than work: grove creates a grove
/// whole, so anything short of one is an anomaly to name (principle 2).
enum RootShape {
    /// A root holding no task — its charter and nothing else, or nothing at all.
    ///
    /// **Nothing grove does produces this any more, and that is the change.**
    /// `root-init` used to write the root and its charter under a guard of its
    /// own and append the first leaf under the library's, so a death between the
    /// phases left exactly this and grove carried a recovery for it.
    /// [`initialize_grove`] is one operation that takes the root back down if it
    /// fails, so the window is closed and the repair went with it
    /// (`collapse-tree-access-k13`). What reaches here is a tree something
    /// emptied by hand — and entries are marked, never removed
    /// (`docs/adr/entries-are-never-removed.md`), so it was not grove.
    Taskless,
    /// At least one keyed entry. A name the grammar *refuses* never reaches this
    /// classification at all: the store halts the whole tree on one, wherever it
    /// sits, and `task_tree::restate` says so in the domain's own words — which
    /// is the same answer as before by a shorter route, since a refused name is
    /// *held* and scaffolding past it would bury the real problem.
    ATree,
    /// Names grove disclaims, and nothing else. A `.grove/` in one of the
    /// layouts grove wrote before the current grammar reads exactly so, since
    /// none of those names are positioned-and-keyed.
    Unrecognised(Vec<String>),
}

````
<!-- /fragment -->

**Two of the three variants exist to be refused, and the comment says why in one
sentence:** grove creates a grove whole, so anything short of one is an anomaly to
name. That is the same principle the deleted recovery obeyed, read forward.

`Taskless` is the shape this chapter's rule made unproducible, and its doc comment
is the place the book can point at when a reader asks what the deletion bought.
Read it as a chain: nothing grove does produces this any more; `initialize_grove`
is one operation that takes the root back down if it fails; entries are marked,
never removed; **therefore** a taskless root was emptied by hand, and grove should
say so rather than quietly complete it.

`ATree` is where the interesting exclusion lives, and it is easy to read past.
*A name the grammar refuses never reaches this classification at all.* A `.grove/`
containing one malformed task name does not classify as `ATree`-with-a-problem, or
as `Unrecognised`: the store **halts the whole tree** on a refused name, wherever
in the tree it sits, and the opening fails before `root_shape` is called. That is
chapter 4's canonicity rule and chapter 5's opening observed from a third place,
and the comment draws the right conclusion — a refused name is *held*, and
scaffolding past it would bury the real problem.

`Unrecognised` carries the names rather than a count, for the reason `Ambiguous`
carried its matches in chapter 9: the caller is going to print them.

<!-- fragment «grove-beginning-root-shape-fn» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="444-489" parent="grove-beginning" -->
````rust
/// Classify a root grove has already opened.
///
/// **The snapshot answers the first question and a listing answers the second**,
/// and both are read under the guard the caller holds. Whether the tree has a
/// keyed entry is the snapshot's, exactly as [`task_tree::next_key`] reads it.
/// *Which* foreign names a taskless root is holding is not: the store skips a
/// name the domain disclaims and reports nothing about it
/// (`crates/ordinal-fs-tree/src/fs/read.rs`, the parse trichotomy), so naming
/// them in a refusal means listing the directory. That is safe here in a way it
/// was not before — grove holds the store's own lock rather than a second one
/// beside it.
fn root_shape(tree: &task_tree::Guard) -> Result<RootShape> {
    if tree.snapshot().walk().any(|entry| entry.key().is_some()) {
        return Ok(RootShape::ATree);
    }
    let grove_root = tree.root();
    let listing = fs::read_dir(grove_root)
        .with_context(|| format!("reading grove root {}", grove_root.display()))?;
    let mut disclaimed = Vec::new();
    for entry in listing {
        let entry =
            entry.with_context(|| format!("reading grove root {}", grove_root.display()))?;
        let found = if entry
            .file_type()
            .with_context(|| format!("checking {}", entry.path().display()))?
            .is_dir()
        {
            Found::Dir
        } else {
            Found::File
        };
        let name = entry.file_name();
        // A name that is not UTF-8 is not one this grammar can spell, which is
        // the same answer `TaskName::parse` reaches for anything else foreign.
        let name = name.to_string_lossy().into_owned();
        if matches!(TaskName::parse(&name, found), Verdict::Foreign) {
            disclaimed.push(name);
        }
    }
    if disclaimed.is_empty() {
        return Ok(RootShape::Taskless);
    }
    disclaimed.sort();
    Ok(RootShape::Unrecognised(disclaimed))
}

````
<!-- /fragment -->

**Two questions, two instruments, and the comment is precise about which answers
which.** *Has this tree any keyed entry?* is the snapshot's, and it is one walk
with a predicate — exactly what chapter 7 established the library can do. *Which
foreign names is this root holding?* is not the snapshot's at all, and the reason
is the store's parse trichotomy: a name the domain disclaims is **skipped**, and
the snapshot reports nothing about it. There is no way to ask a snapshot for the
things it declined to put in itself. So naming them means listing the directory
with `read_dir`, beside the store rather than through it.

That is a raw filesystem read in the middle of a chapter whose whole subject is
going through the store, and the comment defends it on exactly the ground that
makes it safe now and did not before: **grove holds the store's own lock rather
than a second one beside it**. Under the old two-guard scheme this listing would
have been a read of a directory another layer of grove's own locking was
mutating.

The UTF-8 comment is the same move chapter 2 made. A name that is not UTF-8 is
not one this grammar can spell, so `to_string_lossy` is not a compromise — the
lossy rendering is used to *report* the name, and the classification is
`Verdict::Foreign` either way. The sort at the end is there so the refusal reads
the same twice.

**`disclaimed.is_empty()` is what separates the two refusals**, and it is the one
line of real decision in the function. Note what it does *not* mean: an empty list
is not an empty directory. `disclaimed` collects only names that parse as
`Verdict::Foreign`, and `BRIEF.md` parses as `TaskName::Brief` — so a root holding
its charter and nothing else contributes no entry here and classifies as
`Taskless`, which is exactly the variant's own wording, *its charter and nothing
else, or nothing at all*. A non-empty list means grove is looking at names that
belong to something other than grove.

<a id="the-bodies-the-store-writes"></a>
## The bodies the store writes

The chapter's second ownership block is lines 1013 to 1076 — 64 lines, at the
very bottom of the production half, after everything that calls them. Three
helpers, and **each has a different consumer**, which is why they sit together at
the end rather than beside any one verb.

<!-- fragment «body-helpers» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1013-1076" parent="source-tree-lifecycle" -->
<!-- insert «body-helpers-grove-name» -->
<!-- insert «body-helpers-root-brief» -->
<!-- insert «body-helpers-retitle» -->
<!-- /fragment -->

The first is the one this chapter calls.

<!-- fragment «body-helpers-grove-name» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1013-1037" parent="body-helpers" -->
````rust
/// The grove's display name for its own charter: the **worktree** directory's
/// basename, read off the tree root the store is about to create
/// (user-owned-worktrees — grove reads no branch, ever). Both callers pass it
/// straight to [`initialize_grove`], which spends it on the root brief's
/// `# <name> — brief` title.
///
/// It takes the grove root rather than the worktree because that is what a
/// [`TreeVacancy`] carries, and `<worktree>/.grove` is the only spelling grove
/// ever opens — so the parent of the root is the worktree, and asking the
/// vacancy is one fewer argument that could disagree with the lock.
fn grove_name(grove_root: &Path) -> String {
    grove_root
        .parent()
        .and_then(Path::file_name)
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "grove".to_string())
}

/// The minimal section-header scaffold for the root `BRIEF.md` — headers only,
/// no prose (the bootstrap session fills them). The root brief is the one
/// unkeyed, position-free singleton, unchanged across schemes.
///
/// Bytes rather than a write: the charter is the root's **distinguished child**
/// and the store places it, so grove no longer opens the file itself
/// (`collapse-tree-access-k13`).
````
<!-- /fragment -->

**Two callers, one of them chapter 14's.** The *both callers* the comment names
are line 347, in `root_init` above, and line 81, in `transition_to_current`. Both
want the same string for the same reason — the root brief's `# <name> — brief`
title, which is what `initialize_grove` spends the argument on — and the argument
is the *grove root* rather than the worktree because that is what a
`TreeVacancy` carries. The comment's justification is worth keeping: `<worktree>/.grove`
is the only spelling grove ever opens, so the parent of the root is the worktree by
construction, and asking the vacancy is one fewer argument that could disagree with
the lock. This is the same discipline chapter 6 read in `entry_path` — derive the
path from the thing that holds the lock, never from a second copy of it.

**The `"grove"` fallback is unobserved**, and the measurement below says so. It
fires when `parent()` is `None`, or when the parent it returns has no file name of
its own — neither of which a path a `TreeVacancy` was opened from can present.

**This doc comment carried two summaries welded into one paragraph, and the
repair is adjudicated here rather than repeated.** Before
`welded-grove-name-summary-k160`, lines 1013 to 1017 were a single `///` run with
no blank line inside it: *The grove's name is the worktree directory's basename …
Used as the root brief's `# <name> — brief` title.* was one summary, and *The
grove's display name for its own charter: the **worktree** directory's basename,
read off the tree root the store is about to create.* was a second, written later,
describing the same function. Markdown joined them, so rustdoc's short description
— the first paragraph — was a three-sentence run whose third sentence re-described
what the first two had already said, and the module index printed the whole of it.
**The defect was the redundancy, not the length**: of the thirty-one items in this
module's index that carry a summary at all, fifteen run to more than one sentence,
so a long one is unremarkable here. A summary that says the same thing twice is
not.

**The repair spends the same five lines**, which is why nothing below line 1017
moved in a 2,725-line root. The bytes above are the fold: the later summary's
framing, the earlier one's parenthetical and its statement of what the name is
*for*, and — new, and the reason the fold does not simply drop a sentence — the
fact that both call sites hand the string to the same consumer.
`fn.grove_name.html` still renders exactly two `<p>` elements, and the first now
describes the function once.

**Nothing in this repository reported it, and nothing would have.** `cargo doc
--no-deps --document-private-items -p grove-loop` emits twenty-six warnings across
the crate and **none at all for this file** — before the repair or after — because
both paragraphs were attached to the right item; there was nothing for the
compiler to complain about. The check that finds this class is counting the
paragraphs in the *rendered* docblock and then reading the first one. That is the
sixth instrument's blind spot in a new form: chapter 10 found a `//` module header
`cargo doc` cannot see at all, and this was a correctly attached doc comment whose
*shape* was wrong. Neither sentence was false.

<!-- fragment «body-helpers-root-brief» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1038-1053" parent="body-helpers" -->
````rust
fn root_brief_body(name: &str) -> String {
    format!(
        "# {name} — brief\n\n## Goal\n\n## Done when\n\n## Decomposition\n\n## Pointers\n\n## Notes\n",
    )
}

/// Retitle a freshly-decomposed node brief's first-line handle header by appending
/// ` — brief`, rewriting the file in place. Recognises exactly the canonical
/// position-free handle `# <slug>-k<key>` (the form `leaf_add_unlocked` writes), and is
/// idempotent against an already-suffixed title; any other (hand-edited) first line
/// is left alone (conservative — never clobbers a custom title).
///
/// It takes the [`Handle`] rather than a slug and a key so the line it looks for
/// is rendered by the same code that wrote it. Recognising a title by
/// re-spelling the grammar is how this and [`task_grow::task_template_body`]
/// could once have drifted into a retitle that silently matched nothing.
````
<!-- /fragment -->

**Headers only, and no prose.** The root brief is the one unkeyed, position-free
singleton in the whole tree — chapter 3 established that the handle grammar has
nothing to say about it — and what `root-init` writes into it is five empty
section headers for the bootstrap session to fill. A scaffold that guessed at
content would be a scaffold the first session had to delete.

**Bytes rather than a write, and the comment names the reason.** `root_brief_body`
returns a `String`; it does not open a file. The charter is the root's
distinguished child and the store places it, so grove no longer opens the file
itself. That is the same deletion `initialize_grove`'s comment described, seen from
the helper's side: when the seam went, so did grove's need for a file handle here.

<!-- fragment «body-helpers-retitle» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1054-1076" parent="body-helpers" -->
````rust
fn append_brief_suffix_in_file(path: &Path, handle: &Handle) -> Result<()> {
    let body =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let title = format!("# {handle}");
    let new_first = if first.trim_end() == title {
        format!("{title} — brief")
    } else {
        return Ok(()); // already suffixed, or a custom title — leave alone
    };
    let mut out = String::with_capacity(body.len() + 8);
    out.push_str(&new_first);
    if let Some(r) = rest {
        out.push('\n');
        out.push_str(r);
    }
    fs::write(path, out.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

````
<!-- /fragment -->

**This is the one helper of the three that this chapter does not call.** Its only
call site anywhere is line 587, inside `leaf_decompose` — **chapter 12's** function
— which renames a leaf's body in as a node's `BRIEF.md` and then has to retitle its
first line. The chapter reproduces it because the manifest puts the whole
`body-helpers` block here, and the account of *why a decomposed brief is retitled
at all* is chapter 12's to give.

What this chapter owes is the shape of the function, and it is a **conservative**
one. It recognises exactly the canonical position-free handle `# <slug>-k<key>`,
and anything else — including an already-suffixed title — falls out through
`return Ok(())` and is left alone. The doc comment's argument for taking a `Handle`
rather than a slug and a key is the strongest sentence in the block: the line it
looks for is rendered by *the same code that wrote it*, so a retitle cannot drift
into silently matching nothing the way it could if the grammar were re-spelled
here. That is chapter 4's canonicity guarantee being spent rather than restated.

**Neither of the two promises in that comment is pinned by anything.** Idempotence
against an already-suffixed title, and *never clobbers a custom title*, both run
through the single `return Ok(())` on line 1065, and panicking on that branch
leaves the 560-test run no worse than its control. The measurement is below, and chapter 12 — the consumer — is
where it is worth acting on.

<a id="the-support-the-next-two-chapters-use"></a>
## The support the next two chapters use

The chapter's third ownership block is lines 1077 to 1466 — 390 lines, and it is
two things: the **whole opening of the root's inline test module**, which chapters
12, 13 and 14 all build their fixtures on, and then the `root-init` section
itself. The module is 1,649 lines in total, 15% of it comment prose, and the
book's third prose obligation attaches to every reproduced test in it: say what
the test establishes, **and what would have to be true for it to pass while the
property was broken**.

The support comes first, and it is worth reading as a design rather than as
plumbing, because half of it exists to make one distinction: **which fixtures need
a repository and which do not**.

<!-- fragment «root-init-tests» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1077-1466" parent="source-tree-lifecycle" -->
<!-- insert «root-init-tests-open» -->
<!-- insert «root-init-tests-worktrees» -->
<!-- insert «root-init-tests-grow-leaf» -->
<!-- insert «root-init-tests-guards» -->
<!-- insert «root-init-tests-root-init-at» -->
<!-- insert «root-init-tests-writers» -->
<!-- insert «root-init-tests-basics» -->
<!-- insert «root-init-tests-refusals» -->
<!-- insert «root-init-tests-one-guard» -->
<!-- insert «root-init-tests-one-operation» -->
<!-- insert «root-init-tests-no-self-wait» -->
<!-- insert «root-init-tests-prediction» -->
<!-- insert «root-init-tests-refused-grove» -->
<!-- insert «root-init-tests-taskless» -->
<!-- /fragment -->

The module opens on a kind and three imports.

<!-- fragment «root-init-tests-open» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1077-1092" parent="root-init-tests" -->
````rust
#[cfg(test)]
mod tests {
    use super::*;

    /// A [`Kind`] for a test that needs one, by its label.
    ///
    /// A kind is an **open token** since `open-kind-k20`, so a test names the token
    /// it means rather than a variant, and an invalid one is a test bug that panics
    /// here rather than a compile error somewhere else.
    fn a_kind(label: &str) -> Kind {
        Kind::new(label).expect("a test kind must be well-formed")
    }
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

````
<!-- /fragment -->

`a_kind` is the whole of `open-kind-k20` in three lines. A kind became an open
token, so a test names the token it means — `"impl"` — rather than a variant, and
a mistyped one panics here as a test bug instead of failing to compile somewhere
unhelpful. Chapter 3 established the token; this is what it costs a test.

The three `use` declarations on lines 1089 to 1091 sit **after** a function rather
than with `use super::*` at the top, which is where they would be if the block had
been written at once. It is a harmless accretion and the book notes it only
because the block is reproduced whole and a reader will see it.

<!-- fragment «root-init-tests-worktrees» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1093-1145" parent="root-init-tests" -->
````rust
    /// A bare worktree dir with **no** `.grove/` yet — for `root_init`, which
    /// creates the grove itself and needs no repository (it never renames an entry).
    fn worktree() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let wt = tmp.path().join("my-grove");
        fs::create_dir_all(&wt).unwrap();
        (tmp, wt)
    }

    /// A `.grove/` inside a real jj repo — the only kind of working tree Grove
    /// drives (`docs/adr/jj-is-the-only-lane.md`). The repository is these tests'
    /// **instrument** rather than their prerequisite: every verb below renames
    /// inside an `ordinal-fs-tree` operation, which uses `rename(2)` and records
    /// nothing of its own (`docs/adr/grove-does-not-stage-its-own-renames.md`),
    /// so nothing here needs committed files to operate on. [`commit_all`] is
    /// what makes the fixtures the ones a real session produces.
    fn jj_grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path().to_path_buf();
        run_jj(
            &repo,
            &["--config", "git.colocate=false", "git", "init", "."],
        );
        let root = repo.join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
    }

    fn run_jj(repo: &Path, args: &[&str]) {
        let out = Command::new("jj")
            .current_dir(repo)
            .args([
                "--config",
                "user.name=Test",
                "--config",
                "user.email=t@example.com",
            ])
            .args(args)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "jj {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Commit everything under the grove, putting the entries in the revision the
    /// working copy sits on — the state a real session's tree is in, and the one
    /// in which a rename that recorded anything of its own would be visible.
    fn commit_all(root: &Path) {
        run_jj(root.parent().unwrap(), &["commit", "-m", "fixture"]);
    }
````
<!-- /fragment -->

**Two fixtures, and the difference between them is the chapter's cleanest
statement of what `root-init` is.** `worktree()` is a bare directory with **no**
`.grove/` and **no repository** — and the comment gives the reason it needs
neither: `root_init` creates the grove itself, and it never renames an entry.
Every other lifecycle verb in this root does rename one, and renaming is where jj
enters.

`jj_grove()` is the other half, and its comment is the clearest argument in the
support block: **the repository is these tests' instrument rather than their
prerequisite.** Every verb below renames inside an `ordinal-fs-tree` operation,
which uses `rename(2)` and records nothing of its own, so nothing here *needs*
committed files in order to operate. The repository is there so that a rename
which *did* record something of its own would be visible. That is a fixture built
to falsify a claim rather than to satisfy a dependency, and `commit_all` is what
makes the state the one a real session's tree is in.

`run_jj` pins `user.name` and `user.email` on every invocation so the fixture does
not read the developer's own configuration, and asserts on `status.success()` with
the captured stderr in the message — a failing fixture says what jj said.

<!-- fragment «root-init-tests-grow-leaf» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1146-1167" parent="root-init-tests" -->
````rust

    /// Grow a real root-level leaf the way `llm_cli` does — the whole verb,
    /// which takes and releases the library's own write guard, so the lifecycle
    /// verb under test then takes its.
    ///
    /// It calls the verb rather than a lock-neutral primitive under a guard of
    /// its own, and since `sweep-k37` there is no other option: the primitive
    /// belonged to the withdrawn appender, and grove cannot nest its own
    /// exclusive guard inside the library's anyway. `task_grow` owns
    /// the resolve-then-mutate composition entirely, which is the composition
    /// production performs and the only one it can.
    fn grow_leaf(root: &Path, slug: &str) -> PathBuf {
        task_grow::leaf_add(guard(root), ".", &a_slug(slug), &[a_kind("impl")])
            .unwrap()
            .remove(0)
    }

    /// A [`Slug`] for a test that needs one.
    fn a_slug(text: &str) -> Slug {
        Slug::new(text).expect("a test slug must be well-formed")
    }

````
<!-- /fragment -->

**`grow_leaf` is the composition production performs, and the comment says there
is no other option.** It calls the whole `task_grow::leaf_add` verb — which takes
and releases the library's own write guard — rather than a lock-neutral primitive
under a guard of its own, because since `sweep-k37` that primitive does not exist,
and grove cannot nest its own exclusive guard inside the library's anyway. So the
lifecycle verb under test then takes *its* guard, cleanly, after the growing verb
has finished with the tree. Chapter 10 owns `leaf_add`; what this block adds is
that the test support could not have been written any other way.

**The comment names a module that does not exist**, and the book adjudicates it
rather than reproducing it as true: *the way `llm_cli` does*. There is no module, and never was a module,
called `llm_cli` in this workspace — only an integration-test file,
`crates/grove-llm/tests/llm_cli.rs`, whose name is a fossil of the same rename.
The code the sentence describes is `crates/grove-llm/src/cli.rs`. **The behaviour the comment claims is correct and
only the address is stale** — the CLI does drive the whole verb exactly as this
helper does — which is why chapter 6 adjudicated the same defect on the page
rather than cutting a source fix for it, and why this chapter does the same. The
crate carries **four** spellings in all, which is the count chapters 6 and 10 also
give: line 42 of this file, which is chapter 14's block; line 1147 here;
`task_tree.rs` line 1070, which chapter 6 owns and adjudicated; and one in the
excluded `task_grow/tests.rs`, which chapter 10 may only cite.

<!-- fragment «root-init-tests-guards» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1168-1199" parent="root-init-tests" -->
````rust
    /// The exclusive guard a lifecycle write verb now takes, opened from a grove
    /// root — the tests still name roots, and the verbs no longer open one.
    fn guard(grove_root: &Path) -> Guard {
        open(grove_root).expect("opening the tree for a lifecycle verb")
    }

    /// The same, as a `Result`, for the fixtures whose subject *is* the opening.
    fn open(grove_root: &Path) -> Result<Guard> {
        task_tree::write(grove_root)
    }

    /// The refusal an absent root gets, now that the caller does the opening.
    ///
    /// It used to be each write verb's, because each opened the tree itself.
    /// Since `loop-crate-verbs-k21` the lock is in the signature and the caller
    /// holds it, so this is the one place the wording is checked — and the verbs
    /// cannot be reached at all over a root with no tree, because
    /// [`crate::write`] answers a vacancy there and a vacancy offers only
    /// `root-init`.
    fn refusal_for_an_absent_root(grove_root: &Path) -> String {
        open(grove_root)
            .err()
            .expect("an absent root has no tree to open")
            .to_string()
    }

    /// The same, from a **worktree** — the spelling the driver-facing operations
    /// take, and the one `<worktree>/.grove` is joined in.
    fn guard_at(worktree: &Path) -> Guard {
        guard(&worktree.join(".grove"))
    }

````
<!-- /fragment -->

**Four one-line wrappers, and the doc comments carry the history.** `guard` and
`open` differ only in whether the failure is a panic or a `Result`, and the split
is deliberate: `open` is for the fixtures whose *subject is the opening*, which is
chapter 5's territory reached from here.

`refusal_for_an_absent_root` is the one to read twice. It used to be *each write
verb's* refusal, because each verb opened the tree itself; since
`loop-crate-verbs-k21` the lock is in the signature and the caller holds it, so
this is **the one place the wording is checked**. And the closing clause is the
type argument again: the verbs cannot be reached at all over a root with no tree,
because `crate::write` answers a vacancy there and a vacancy offers only
`root-init`. The same guarantee `root_init`'s signature gives, stated from the
other end.

<!-- fragment «root-init-tests-root-init-at» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1200-1220" parent="root-init-tests" -->
````rust
    /// `root-init` as its own CLI drives it: read the slug, resolve the vacancy,
    /// then scaffold — **in that order**, which is what the tests below about a
    /// refused slug are actually asserting.
    ///
    /// The verb takes the [`TreeVacancy`] since `loop-crate-verbs-k21`, which is
    /// what makes *cannot clobber a live grove* a fact about the types rather
    /// than a check — so a fixture that wants that refusal asks for the opening
    /// and finds a tree, exactly as the CLI does. The slug is read first, and
    /// before the lock, since refusing without taking an exclusive one is
    /// strictly kinder.
    fn root_init_at(worktree: &Path, slug: &str) -> Result<Vec<PathBuf>> {
        let slug = Slug::new(slug).map_err(|error| anyhow::anyhow!("slug {slug:?}: {error}"))?;
        let grove_root = worktree.join(".grove");
        match task_tree::write_or_vacancy(&grove_root)? {
            Opening::Tree(_) => {
                bail!("grove root already exists: {}", grove_root.display())
            }
            Opening::Vacancy(vacancy) => root_init(vacancy, &slug, &Kind::requirements()),
        }
    }

````
<!-- /fragment -->

**This helper is where three of the chapter's thirteen tests are actually decided,
and its doc comment says so out loud for two of them.** *Read the slug, resolve the vacancy, then
scaffold — **in that order**, which is what the tests below about a refused slug
are actually asserting.*

That sentence is doing the work of the whole *supply the claim* obligation, and it
is unusual for a fixture to state it. `Slug::new` runs on line 1211, **before**
`write_or_vacancy` on line 1213. So a test that passes a malformed slug never
reaches `root_init`, never reaches the store, and never creates a directory —
because nothing had yet asked for one. The comment's justification is that
refusing without taking an exclusive lock is strictly kinder, which is true and is
also the reason the assertion those tests make is narrower than their names
suggest. Two tests push a refused slug through it — the bad-slug test and the
reserved-slug test — and a third, `root_init_refuses_an_existing_grove`, is
decided two lines further down, in the helper's `Opening::Tree` arm. The next
section says what each consequently does not hold.

**Reading the helper is also what tells these two apart from the test that looks
like them.** `a_refused_grove_leaves_no_root_behind` ends in the same assertion —
no `.grove` exists — but is not decided here at all: its slug is one the grammar
accepts, so it goes past both lines and fails inside the store. What separates it
is its fixture and its first assertion — not the shape of the test, which is why
reading the names alone puts it in the wrong group.

<!-- fragment «root-init-tests-writers» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1221-1261" parent="root-init-tests" -->
````rust
    /// Write a leaf/brief stub with a position-free `# <handle>` header.
    fn touch(dir: &Path, name: &str, header: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, format!("# {header}\n")).unwrap();
        p
    }

    /// Write a file with an explicit body (for realistic multi-line content).
    fn touch_body(dir: &Path, name: &str, content: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    /// Create a node directory with its `BRIEF.md`, returning the directory path.
    fn mknode(dir: &Path, name: &str, handle: &str) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(&p).unwrap();
        fs::write(p.join("BRIEF.md"), format!("# {handle} — brief\n")).unwrap();
        p
    }

    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    fn body(p: &Path) -> String {
        fs::read_to_string(p).unwrap()
    }

    /// The directory's child names (files and subdirs), lexically sorted.
    fn list(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        v.sort();
        v
    }

````
<!-- /fragment -->

Six writers and readers with no argument in them. `touch`, `touch_body` and
`mknode` build the fixtures; `name_of`, `body` and `list` read them back, with
`list` sorting so an assertion does not depend on directory order.

<a id="five-of-eighteen"></a>
### Five of eighteen

That completes the support block, and the honest summary of it is a count.
**Eighteen items are declared in lines 1077 to 1261. This chapter's own tests call
five of them.**

| Called by chapter 11's tests | Called only from inside the support block | First called by a later chapter |
|---|---|---|
| `worktree`, `root_init_at`, `touch`, `name_of`, `body` | `run_jj` (by `jj_grove` and `commit_all`), `open` (by `guard` and `refusal_for_an_absent_root`) | `a_kind`, `jj_grove`, `commit_all`, `grow_leaf`, `a_slug`, `guard`, `refusal_for_an_absent_root`, `guard_at`, `touch_body`, `mknode`, `list` |

Eleven of the eighteen are never touched by this chapter's tests and are called by
chapters 12, 13 or 14. Three of those eleven — `a_kind`, `a_slug` and `guard` — do
have a caller *inside* the support block, in `grow_leaf` and `guard_at`, so the
column means *never called by chapter 11's tests* rather than *first called later*.
The split is not arbitrary. Everything chapter 11 needs is a *bare directory*:
`worktree()` and the file readers. Everything the other three need starts with
`jj_grove()` and a guard, because their verbs rename entries and chapter 11's does
not. **The support block is where *root-init needs no repository* stops being a
comment and becomes the shape of the fixtures**, and that is the second half of
why the manifest gives these 185 lines to this chapter rather than to the one that
uses most of them.

<a id="what-the-tests-establish"></a>
## What the thirteen tests establish, and what each would pass under

The `root-init` section is lines 1262 to 1466 and carries **thirteen** `#[test]`
functions. The block's prose obligation is *supply the claim*: for each, the
property it establishes **and what would have to be true for it to pass while the
property was broken**. The second half is the part a reviewer can check and the
test cannot state, and in this block it is unusually productive — four of the
thirteen turn out to pin something narrower than their names say, and all four are
narrowed by the same helper.

<!-- fragment «root-init-tests-basics» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1262-1308" parent="root-init-tests" -->
````rust
    // ---- root-init ----------------------------------------------------------

    #[test]
    fn root_init_creates_root_brief_then_first_leaf() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "plan").unwrap();
        assert_eq!(name_of(&created[0]), "BRIEF.md");
        assert_eq!(name_of(&created[1]), "01-requirements--plan-k1.md");
        assert_eq!(created.len(), 2);
        let g = wt.join(".grove");
        assert!(g.join("BRIEF.md").is_file());
        assert!(g.join("01-requirements--plan-k1.md").is_file());
    }

    // fresh-grove-start-contract: the bootstrap leaf is `requirements` — the
    // human's own words are the session's only input, which is the HITL rule —
    // and the `start` path routes on that kind without a file to peek, so this
    // assertion is the contract, not a detail of the template.
    #[test]
    fn root_init_first_leaf_kind_lives_in_its_filename() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "plan").unwrap();
        assert_eq!(name_of(&created[1]), "01-requirements--plan-k1.md");
        assert!(!body(&created[1]).contains("**Kind:**"));
    }

    #[test]
    fn root_init_first_leaf_header_is_the_position_free_handle() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "plan").unwrap();
        assert!(
            body(&created[1]).starts_with("# plan-k1\n"),
            "got {:?}",
            body(&created[1])
        );
    }

    #[test]
    fn root_init_root_brief_title_is_the_grove_name() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "plan").unwrap();
        assert_eq!(
            body(&created[0]).lines().next().unwrap(),
            "# my-grove — brief"
        );
    }

````
<!-- /fragment -->

**`root_init_creates_root_brief_then_first_leaf`** — the return value is
*ordered*, charter then leaf, and both files exist. It would pass while the
property was broken if `Report::created` ever stopped being
distinguished-child-first *and* something else re-sorted the vector before it was
returned; the assertion is on the order of `created`, not on the store's
guarantee. It also asserts `created.len() == 2`, which is what makes it a
statement about the *whole* grove rather than about two of its members.

**`root_init_first_leaf_kind_lives_in_its_filename`** — the kind is in the name,
and the body does not carry a `**Kind:**` line. The negative assertion is the real
one: this test would pass with the kind in **both** places if it only checked the
filename, and a template that reintroduced a body field would then be invisible.
The comment names the contract it holds — `fresh-grove-start-contract` — and the
reason: the `start` path routes on the kind without a file to peek at, so the
filename is the only place the kind may live.

**`root_init_first_leaf_header_is_the_position_free_handle`** — the body opens
`# plan-k1`, not `# 01-requirements--plan-k1`. This is chapter 3's rule spent:
`01-` is a position and positions move. It would pass while the property was
broken only if `starts_with` were satisfied by a longer header that happened to
begin the same way, which the trailing `\n` in the pattern rules out.

**`root_init_root_brief_title_is_the_grove_name`** — the charter's first line is
`# my-grove — brief`, and `my-grove` is the *worktree* directory's basename, which
the `worktree()` fixture chose. This is the only test anywhere that asserts a value `grove_name` produced —
`transition_to_current` calls it too, and nothing checks what it returned. It
would pass while the property was broken if `grove_name` read the grove root's own
basename instead of its parent's — no: that would render `# .grove — brief`. It
would pass if the fallback were wrong, because the fallback never fires here.

<!-- fragment «root-init-tests-refusals» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1309-1333" parent="root-init-tests" -->
````rust
    #[test]
    fn root_init_refuses_an_existing_grove() {
        let (_t, wt) = worktree();
        fs::create_dir_all(wt.join(".grove")).unwrap();
        let err = root_init_at(&wt, "plan").unwrap_err();
        assert!(err.to_string().contains("already exists"), "got {err}");
    }

    #[test]
    fn root_init_rejects_a_bad_slug_without_leaving_a_grove_behind() {
        let (_t, wt) = worktree();
        assert!(root_init_at(&wt, "Bad Slug").is_err());
        assert!(
            !wt.join(".grove").exists(),
            ".grove must not be created on a bad slug"
        );
    }

    #[test]
    fn root_init_rejects_reserved_slug() {
        let (_t, wt) = worktree();
        assert!(root_init_at(&wt, "BRIEF").is_err());
        assert!(!wt.join(".grove").exists());
    }

````
<!-- /fragment -->

**`root_init_refuses_an_existing_grove`** — the refusal is the *helper's*, in the
`Opening::Tree` arm on lines 1214 to 1216, not the verb's. `root_init_at` asks for
the opening, finds a tree, and bails with *already exists*; `root_init` is never
called. That is not a defect
— it is the point the helper's doc comment makes, that a fixture wanting this
refusal asks for the opening exactly as the CLI does — but it means the test
establishes *the CLI refuses to clobber*, and the stronger property, that the verb
**cannot** be handed a live tree, is held by the type of its first argument and by
nothing executable at all.

**`root_init_rejects_a_bad_slug_without_leaving_a_grove_behind`** and
**`root_init_rejects_reserved_slug`** are the two that pin less than their names
promise, and the helper is where it hides. `Slug::new` runs before
`write_or_vacancy`, so both refuse at chapter 3's grammar without the store being
opened at all. What they establish is that **grove's front door creates no
directory**; what they do *not* establish is that anything unwinds, because
nothing was wound. Neither is refused for the reason a reader
would guess. `refuse_token` tests the reserved set, then the dashes, then the `--`
separator, and only then the character class — and that last arm reports the
**first** offending character it finds. So `"Bad Slug"` is refused for its capital
`B`, and its space is never reached; `"BRIEF"` never reaches the character class at
all, because it is caught as a **reserved** token and the message names the
grammar's own markers rather than its alphabet. Both refusals are chapter 3's,
observed through chapter 11's verb.

The test that *does* wind something and then unwind it is a hundred and twelve
lines further down. What carries it past this helper is the one thing these two
lack: a slug the grammar accepts.

<!-- fragment «root-init-tests-one-guard» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1334-1352" parent="root-init-tests" -->
````rust
    #[test]
    fn transition_initializes_an_absent_grove_under_one_exclusive_guard() {
        let (_temporary, worktree) = worktree();
        crate::task_tree::reset_read_count();

        let outcome = transition_to_current(&worktree).unwrap();

        let grove_root = worktree.join(".grove");
        assert_eq!(outcome, CurrentTransition::RootInitialized);
        // **One, and it used to be two.** Classifying the root and creating it
        // were separate acquisitions because grove's guard and the library's
        // could not nest; the vacancy is one opening that answers both.
        assert_eq!(crate::task_tree::read_count(), 1);
        assert_eq!(
            name_of(&crate::task_tree::tests::pick(&grove_root).unwrap().unwrap()),
            "01-requirements--plan-k1.md"
        );
    }

````
<!-- /fragment -->

**`transition_initializes_an_absent_grove_under_one_exclusive_guard`** — the
driver's own scaffold path, and the first of two tests that assert a **count of
lock acquisitions**. `reset_read_count` and `read_count` are chapter 9's
instrumentation, met in chapter 7's tests; here the assertion is `1`, and the
inline comment says what it replaced: classifying the root and creating it were
separate acquisitions because grove's guard and the library's could not nest.

It is also one of this crate's **two** observers of `DEFAULT_ROOT_SLUG`, by way of
the `01-requirements--plan-k1.md` it expects `pick` to answer; the other is
`bare_scaffolding_is_anchored_before_the_configured_command_inherits_git_context`
in `tests/driver_lease.rs`, which reaches the same constant from the far side, by
running the `grove` binary against a worktree that has no grove in it. This one
would pass while the lock property was broken if `read_count` were incremented
somewhere other than the opening it is meant to count — which is why chapter 9 put
the counter where it did — and **both** pass while the *slug* property is broken in
the CLI, because the CLI's default is a second literal neither can see. That is the
gap [*The value chosen twice*](11-a-grove-begins.md#the-value-nothing-holds)
measures, and the test `default-root-slug-two-spellings-k159` put in `grove-llm`'s
fixtures is the one that closes it.

<!-- fragment «root-init-tests-one-operation» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1353-1375" parent="root-init-tests" -->
````rust
    /// **The whole grove is one store operation, and that is the deletion.**
    /// `root-init` used to take a guard of grove's own for the root and its
    /// charter — neither of which the library could create, since it has to
    /// reach the root in order to snapshot it — release it, and append the first
    /// leaf under the library's, because the two `flock` one directory through
    /// different open file descriptions and would deadlock nested. The store's
    /// vacancy creates all three under the lock it already holds, so the count
    /// this asserts is the second lock layer's absence.
    #[test]
    fn root_init_creates_the_whole_grove_through_one_store_operation() {
        let (_t, wt) = worktree();
        crate::task_tree::reset_read_count();

        let created = root_init_at(&wt, "plan").unwrap();

        assert_eq!(
            crate::task_tree::read_count(),
            1,
            "the root, its charter and the first leaf are created under exactly one lock"
        );
        assert_eq!(name_of(&created[0]), "BRIEF.md");
        assert_eq!(name_of(&created[1]), "01-requirements--plan-k1.md");
    }
````
<!-- /fragment -->

**`root_init_creates_the_whole_grove_through_one_store_operation`** is the test
the structure brief names as this chapter's rule, and its doc comment is the
clearest statement of the deletion in the file: *the count this asserts is the
second lock layer's absence*.

**What would have to be true for it to pass while the property was broken.** The
assertion is `read_count() == 1`, so it is a claim about how many times the tree
was *opened*, not about how many filesystem operations happened underneath. A
`root_init` that opened once and then wrote the charter and the leaf in two
separate non-atomic store calls would satisfy this test exactly, and would
reintroduce the window. What forecloses that is not this assertion but
`TreeVacancy::initialize`'s own contract — one call, and the root comes back down
if it fails — which lives in another crate and which this book may name but not
reproduce. **The test pins the lock count; the atomicity is the store's promise.**
That distinction is the honest form of this chapter's rule, and it is worth
stating plainly rather than letting the test name carry it.

<!-- fragment «root-init-tests-no-self-wait» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1376-1410" parent="root-init-tests" -->
````rust

    /// **Grove no longer waits on itself.** The failure mode the second lock
    /// layer existed to paper over is a single process taking two `flock`s on
    /// one directory through two open file descriptions, which blocks forever —
    /// so the direct assertion is that one process runs the tree-creating verb
    /// and a tree-reading one back to back and finishes. A regression that put a
    /// guard of grove's own back around either would hang here rather than fail,
    /// so the wait is bounded and the timeout *is* the failure.
    #[test]
    fn one_process_creating_and_reading_a_grove_never_waits_on_itself() {
        let (temporary, worktree) = worktree();
        let (sender, receiver) = std::sync::mpsc::channel();
        // Detached deliberately: a thread deadlocked on `flock` cannot be
        // joined, and the point of the timeout is to report that rather than
        // inherit the hang.
        std::thread::spawn(move || {
            let created = root_init_at(&worktree, "plan").unwrap();
            let grove_root = worktree.join(".grove");
            let picked = crate::task_tree::tests::pick(&grove_root).unwrap().unwrap();
            let again = transition_to_current(&worktree).unwrap();
            sender.send((created, picked, again)).unwrap();
        });

        let (created, picked, again) = receiver
            .recv_timeout(std::time::Duration::from_secs(10))
            .expect(
                "grove waited on a lock of its own: root-init and pick deadlocked in one process",
            );

        assert_eq!(name_of(&created[1]), "01-requirements--plan-k1.md");
        assert_eq!(picked, created[1]);
        assert_eq!(again, CurrentTransition::AlreadyCurrent);
        drop(temporary);
    }

````
<!-- /fragment -->

**`one_process_creating_and_reading_a_grove_never_waits_on_itself`** — the third
of the block's lock tests and the only one that asserts no count at all, because
what it is testing cannot be counted: it is the only test in the block whose
failure mode is a **timeout rather than an assertion**. Its doc comment is explicit about that: a regression
that put a guard of grove's own back around either verb would *hang* here rather
than fail, so the wait is bounded and the timeout is the failure.

The detached thread is deliberate and the comment says why — a thread deadlocked
on `flock` cannot be joined, so joining it would inherit the hang instead of
reporting it. `recv_timeout` at ten seconds is the whole mechanism.

**What it would pass under.** Ten seconds is a wall-clock number, so a machine
slow enough, or a future implementation that serialised legitimately for nine
seconds, passes. And it asserts `AlreadyCurrent` on the second transition, which
is chapter 14's variant reached from here: the value proves the second call
*classified* rather than scaffolded, and it is reachable only through
`root_shape`'s `ATree` arm. That makes this test one of the two places in
**chapter 11's own block** where a `RootShape` arm is pinned at all — the other is
the taskless test that closes the section.

<!-- fragment «root-init-tests-prediction» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1411-1423" parent="root-init-tests" -->
````rust
    /// The leaf's `# <slug>-k<key>` handle is rendered before the library
    /// allocates the key, so `root-init` predicts the allocation exactly as every
    /// grow verb does — and `task_grow::allocated` refuses to report success on a
    /// disagreement. This is the assertion that would fail if the prediction and
    /// the library's `max + 1` ever parted.
    #[test]
    fn root_inits_first_leaf_handle_matches_the_key_the_library_allocated() {
        let (_t, wt) = worktree();
        let created = root_init_at(&wt, "custom-plan").unwrap();
        assert_eq!(name_of(&created[1]), "01-requirements--custom-plan-k1.md");
        assert!(body(&created[1]).starts_with("# custom-plan-k1\n"));
    }

````
<!-- /fragment -->

**`root_inits_first_leaf_handle_matches_the_key_the_library_allocated`** — the
prediction chapter 10 explained, checked at the one place a grove has no existing
keys to predict from. The slug is `custom-plan` rather than `plan`, which is what
makes it a test of the *rendering* rather than a second copy of the default.

**Its doc comment overstates by one step, and the measurement below is what
settles it.** *`task_grow::allocated` refuses to report success on a disagreement.
This is the assertion that would fail if the prediction and the library's `max + 1`
ever parted.* The first sentence is true. The second describes this test's
assertions, which are on the **filename** and the **body** — so if the prediction
and the allocation parted, this test would indeed fail, but it would fail on the
name, and `allocated`'s refusal would never be read. Panicking on `allocated`'s
error arm in a copy of the workspace leaves the 560-test run no worse than its
control: **nothing in this crate or in `grove-llm` ever makes the prediction
disagree**, so the guard the
comment credits is unexercised on this path. The test pins the agreement; it does
not exercise the check.

<!-- fragment «root-init-tests-refused-grove» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1424-1441" parent="root-init-tests" -->
````rust
    /// **A grove that fails to initialize leaves no root at all**, which is what
    /// closed the window the deleted recovery existed for. The store creates the
    /// root, places the charter and the first leaf, and takes the root back down if
    /// any of it fails — so a slug the grammar accepts and the filesystem will not
    /// reaches past the lock, fails *inside* the store, and still leaves nothing.
    #[test]
    fn a_refused_grove_leaves_no_root_behind() {
        let (_t, wt) = worktree();
        // Long enough that the leaf's own filename is what the placement fails on,
        // and asserted: a slug refused earlier holds nothing about the unwind.
        let error = root_init_at(&wt, &"a".repeat(300)).unwrap_err().to_string();
        assert!(error.contains("creating the leaf"), "got {error}");
        assert!(
            !wt.join(".grove").exists(),
            "a refused root-init must leave no root"
        );
    }

````
<!-- /fragment -->

**`a_refused_grove_leaves_no_root_behind`** is the test the structure brief names
as the negative of *one store operation*, and at a glance it holds what
`root_init_rejects_a_bad_slug_without_leaving_a_grove_behind` holds a hundred and
twelve lines earlier: the call failed, and no `.grove` exists. Its second
assertion is indeed that test's second assertion. The **slug** and the **first**
assertion are where the two part company, and between them they are the whole of
what this test is.

`"Bad Slug"` is refused by the grammar — for its capital `B`, as the section
above works out — so `root_init_at` returns from `Slug::new` on line 1211 and
`write_or_vacancy` on 1213 is never reached. Three hundred `a`s are a slug the
grammar *accepts*: `refuse_token` checks the reserved set, the ends, the `--`
separator and the character class, and not one of its checks is a length. So this
call goes the whole way.
The vacancy is handed out, `initialize` creates the root and writes `BRIEF.md`
into it, and then placing `01-requirements--aaa…-k1.md` fails, because that name
is longer than the longest single name the filesystem will take — 255 bytes on
the machine this was run on. The store unwinds its own effects and removes the
root it had made, and the error grove reports says so: *creating the leaf …:
File name too long (os error 63). Nothing was changed — every effect this
operation had applied was undone.*

**That is why the first assertion is on the error's text and not only on its
existence.** `is_err()` alone cannot tell the two fixtures apart, and a length
bound added to `Slug` later would quietly turn this test back into a second copy
of the bad-slug one — still passing, and holding nothing about the unwind.
Asserting `creating the leaf` pins the *lateness* of the failure, and the
lateness is the whole of what makes this a test of taking the root back down.

So **the unwinding is asserted here**, and the doc comment's larger property is
the property the assertions hold. The measurement below is the evidence rather
than this reading: a panic on `initialize`'s error arm — row 2 — reddens this
test, and this test is the only observer that arm has. What it reaches is the
store failing *honestly* — which is the one failure a fixture on this side of the
boundary can construct, and why the three arms beside it in `initialize_grove` are
reached by nothing at all. Constructing even this one cost a fixture rather than a
seam, and had to: `ordinal-fs-tree` carries a fault-injection seam, but `Faults` is
`pub(crate)` to that crate and the word appears nowhere in `grove-loop`, so an
operating-system limit on a filename is the only lever there is.

<!-- fragment «root-init-tests-taskless» owner="never-mistaken-for-finished" source="crates/grove-loop/src/tree_lifecycle.rs" lines="1442-1466" parent="root-init-tests" -->
````rust
    /// **A root holding its charter and no task is refused, not repaired.**
    /// Grove used to complete it, because grove itself produced it; it does not
    /// any more (`collapse-tree-access-k13`), so what is left is an anomaly, and
    /// principle 2 says an anomaly gets a sentence naming the fix.
    #[test]
    fn a_taskless_root_is_refused_with_advice_rather_than_completed() {
        let (_t, wt) = worktree();
        let grove_root = wt.join(".grove");
        fs::create_dir(&grove_root).unwrap();
        touch(&grove_root, "BRIEF.md", "my-grove — brief");

        let error = transition_to_current(&wt).unwrap_err().to_string();

        assert!(error.contains("holds no task"), "{error}");
        assert!(
            error.contains("jj undo"),
            "the refusal must name the fix: {error}"
        );
        assert_eq!(
            fs::read_dir(&grove_root).unwrap().count(),
            1,
            "a refusal repairs nothing"
        );
    }

````
<!-- /fragment -->

**`a_taskless_root_is_refused_with_advice_rather_than_completed`** closes the
block and closes the argument the deleted recovery opened. Grove used to complete
a taskless root because grove itself produced it; it does not any more, so what is
left is an anomaly, and principle 2 says an anomaly gets a sentence naming the fix.
All three assertions are load-bearing: *holds no task* is the classification,
*jj undo* is the fix named, and `read_dir(...).count() == 1` is **a refusal
repairs nothing** — the charter the fixture wrote is still the only thing there.

**What it would pass under.** All three assertions are substring or count checks
on the refusal grove produced, so the test is satisfied by *any* code path that
declines and writes nothing while saying those two things. It does not reach the
classification: a `root_shape` that answered `Unrecognised` with a message
carrying the same two substrings would pass it unchanged. What attributes this
test to the `Taskless` arm specifically is not the assertion but the mutation
below — panicking on line 484 reddens this test and nothing else.

This is the second of the two `RootShape` arms chapter 11's own tests observe.
The other is `ATree`, held by
`one_process_creating_and_reading_a_grove_never_waits_on_itself` through its
`AlreadyCurrent` assertion; only `Unrecognised` is left to chapter 14 alone.

<a id="what-the-refusals-are-worth-measured"></a>
## What the refusals are worth, measured

Three claims above rest on a measurement rather than on a reading, and this is it.
The procedure is the one chapters 8, 9 and 10 established: replace each refusal or
fallback arm with a **panic** — not a reworded message, because a test asserting a
bare `is_err()` survives any wording — in a copy of the workspace, run the whole
of `grove-loop` and `grove-llm`, and diff the per-test results against an
unmutated control run of the same copy. The copy is neither a jj repository nor a
checkout of the plugin marketplace, so **eleven of its 560 tests fail before any
mutation** — ten in `crates/grove-loop/tests/prompt.rs` with `NotAWorkspace`, and
one, `the_namespace_is_the_shipped_plugin_entrys_declared_name`, on a manifest the
copy does not carry —; a mutant that reads as clean
against zero would be meaningless, and every row below is a difference against
that control. What makes that a control rather than a count is the **set**: two
copies can agree on eleven and disagree about which eleven, so each mutant is
read as the `comm` difference against those names.

**And the mutant has to be relinked into the `grove` binary before the suite
runs** — `cargo build -p grove --bins` after the edit, not before it. Skip it and
cargo happily runs a `grove` built from unmutated source, every out-of-process
observer in `crates/grove-loop/tests/driver_lease.rs` and
`crates/grove-llm/tests/removed_surface.rs` stays green, and the table reads as a
suite with fewer observers than it has. The step is stated again where [this
chapter re-runs k159's own
mutations](11-a-grove-begins.md#the-value-nothing-holds), because that is where
its cost was first paid; the table below is the re-run that step buys, and row 6 is
where it shows.

**Two of the ten arms make the reading a lie if you skip a check.** A mutant that
fails to compile prints no per-test lines at all and reads exactly like a clean
result, so each run below was confirmed to have executed all 560. And a panic at
the *call site* measures reachability rather than observation: panicking
unconditionally on entry to `root_init` reddens 43 tests and says nothing about
whether its refusal is held.

| # | Arm | Line | Observed by |
|---:|---|---:|---|
| 1 | `refuse_finish_kind` — the driver's reserved kind | 346 | **nothing** |
| 2 | `initialize` failed — `task_tree::raised` | 391 | `a_refused_grove_leaves_no_root_behind` |
| 3 | `bail!` the store reported nothing created | 396 | **nothing** |
| 4 | `bail!` the store reported a non-charter first | 399 | **nothing** |
| 5 | `allocated` — the key prediction disagreed | 405 | **nothing** |
| 6 | `RootShape::ATree` | 457 | `transition_leaves_a_current_grove_unchanged_and_ready_for_pick`, `one_process_creating_and_reading_a_grove_never_waits_on_itself` — and eight more that reach it without observing it (below) |
| 7 | `RootShape::Taskless` | 484 | `a_taskless_root_is_refused_with_advice_rather_than_completed` |
| 8 | `RootShape::Unrecognised` | 487 | `transition_refuses_a_root_holding_no_grove_entry_at_all` |
| 9 | `grove_name` — the `"grove"` fallback | 1028 | **nothing** |
| 10 | `append_brief_suffix_in_file` — the conservative return | 1065 | **nothing** |

Rows 2, 6, 7 and 8 are the control that makes the other six readable: each reddens
an attributable set, so the instrument demonstrably fails when it should.

**Row 6 is the one that reddens more than its observers, and the difference is
the same one the entry probe makes.** Panicking on line 457 turns **ten** tests
red, not two. Five are in `crates/grove-loop/tests/driver_lease.rs` and three in
`crates/grove-llm/tests/removed_surface.rs`, and every one of the eight drives
the loop against a worktree that *already holds* a grove — so
`transition_to_current` classifies it `ATree` before any launch happens, and each
dies in its fixture rather than in an assertion. That is the shape chapter 19
reads in `prompt.rs`'s ten: a test refused entry to what it came to check says
nothing about the arm it died on. The two named in the table are the two that
assert on what the arm *returns* — `AlreadyCurrent`, and that a current grove is
left unchanged and ready for `pick` — so the arm is **reached ten ways and
observed two**. Neither file is a root of any book, so none of the eight belongs
to a chapter's block; both are `tests/`, which this book takes as evidence.

**Six of ten arms are held by nothing, and they split cleanly — but not the way
chapter 10's did.** Rows 3, 4 and 5 are the class chapter 10 named: *the
library did something its contract forbids*, which no test over an honest library
can construct. Row 1 is different — `refuse_finish_kind` is an ordinary
operator-facing refusal, and it is unobserved for a smaller reason: all five call
sites that reach `root_init`, through `verbs::root_init` or directly, pass
`Kind::requirements()` as a literal, so the reserved kind cannot reach it. Rows 9 and 10 are unexercised **fallbacks** rather than
refusals, and row 10 is the one that matters downstream, because both promises in
`append_brief_suffix_in_file`'s doc comment run through it.

**Row 2 is the line between those three and the rest, and it is a thin one.**
`initialize` failing is not the library breaking its contract — it is the library
keeping it, reporting an honest filesystem failure and unwinding — so unlike rows
3, 4 and 5 a test *can* construct it, and
`a_refused_grove_leaves_no_root_behind` does, with a slug too long to place. The
three arms beside it stay unreachable because each one asserts the store lied.

**Four arms are observed, by five tests, and three of the five are this
chapter's.** Five, not thirteen: the eight above reach row 6 without asserting
anything about it, and an arm's observers are the tests that would notice it
returning something else. Row 2's single observer is at 1430, here. Row 6 has two —
`one_process_creating_and_reading_a_grove_never_waits_on_itself` at line 1385,
inside this chapter's own block, and
`transition_leaves_a_current_grove_unchanged_and_ready_for_pick` at 1564, inside
`finish-tests` — so the `ATree` arm is pinned from both sides. Row 7's single
observer is at 1447, here. Row 8's is at 1621, in chapter 14's block, and
`Unrecognised` is the one arm this chapter's 390 lines do not reach at all.

So `RootShape` is chapter 11's source, `transition_to_current` is `root_shape`'s
only caller, and the evidence is **split** rather than exported: the type is
declared where its meaning is, and two of its three arms are exercised by tests on
either side of the 1466/1467 boundary. Chapter 14 owes the account of
`Unrecognised`, which is the arm it alone holds.

<a id="what-could-not-move"></a>
## What could not move

The block's answer to the book's three questions is unusually lopsided, and that
is the chapter's own result.

**On the way in — the names.** Almost nothing. `root_init` takes a `Slug` and a
`Kind` already parsed, and the two tests that look like grammar tests are chapter
3's refusals observed through this verb's helper. What this chapter contributes is
the one name grove *writes* rather than reads: `01-requirements--plan-k1.md`, and
the `# plan-k1` handle inside it, both rendered by the grammar rather than
formatted here.

**On the way through — the preconditions.** This is where the chapter's weight is,
and the answer is that the precondition was **removed from the code and put into
the type system**. *You may not scaffold over a live grove* is not a check in
`root_init`; it is `TreeVacancy` in its signature. And the check that used to
exist — the recovery for a root holding a charter and no task — was deleted along
with the two-phase creation that produced its input. What is left of it is
`RootShape::Taskless`, which is not a repair but a refusal with the fix named in
it.

**On the way out — the policy.** Two choices, and they were of different quality.
The **shape** grove chose — a grove is a charter *and* one live leaf, or it is
nothing — is stated in `initialize_grove`, enforced by one store operation and
observed by two lock-count tests. The **value** grove chose — the slug `plan` — is
stated twice, in two crates, and the agreement between the two statements was
observed by nothing until `default-root-slug-two-spellings-k159` gave it a test.
The layer that stayed got the first one right by construction and the second one
right only after the book read it, and the difference between *enforced by the
shape* and *held by one test* is a fair summary of what this chapter costs to
maintain.

The grove now exists, and it holds one leaf that `pick` will answer. Chapter 12
takes that leaf, finds it proved bigger than its brief assumed, and turns it into
a node without letting its key change.

[Previous: Growing: leaf-add and leaf-insert](10-growing.md) | [Contents](README.md) | [Next: A leaf becomes a node](12-leaf-to-node.md)
