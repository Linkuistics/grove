# The grammar and the openings
<!-- book-page id="the-grammar" slice="admitted-before-dispatch" order="2" -->
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Reading the tree](03-reading-the-tree.md)

<a id="admitted-before-dispatch"></a>
## Admitted before dispatch

Every verb enters the module through one function, `run`, and `run` does four
things in a fixed order: it parses the argument vector, it reads the current
directory, it admits this process against the live session epoch, and only then
does it dispatch to the handler the verb names. The rule this chapter opens on
is that order — **every verb is admitted before it is dispatched** — and its
second half is what the four helpers at lines 863 to 903 hold: **a grove
that is not there is not a grove that is finished**, so a verb run in a working
tree with no `.grove/` is refused, with the remedy in the refusal, rather than
told its work is done. Neither rule is one of the three orders *Orientation*
named; both are what every one of those orders stands on, because a handler
that was never dispatched takes no lock and writes no channel, and a tree that
was refused rather than reported empty is never mutated by mistake.

The chapter's premise is the session epoch, and it is stated once. A
[session epoch](../../../CONTEXT.md#session-epoch) is the record the driver
writes around each launch, binding the working tree it drives, the
[driver lease](../../../CONTEXT.md#driver-lease) it holds for the life of the
loop, and the signal path it handed the session in `GROVE_SIGNAL_FILE`; the
guide's account of the lease is
[one driver per working tree](../../USAGE.md#usage-driver-lease), and the
record's layout and the lock discipline around it are the loop's. What this
chapter needs of it is three facts. A verb run with that variable in its
environment must match the live record — the same working tree, the same
channel, a driver still alive — before it touches the tree. A verb run without
it is a manual command, and no epoch governs it. And the check is made by one
call, before dispatch, so no handler has to ask.

The chapter owns four blocks of `cli.rs` and reads them in the order the
argument takes rather than the file's: the head of the grammar, lines 35 to
65, where the version, the help-on-nothing behaviour and the `Option` are
declared; the four openings, lines 863 to 903, which every handler that
touches the tree calls and whose refusal the worked example ends on; `run`
itself, lines 412 to 437, read inside the worked example because the example
is `run` with concrete values; and the enum's close with `operation_label`,
lines 290 to 310, which is the label admission quotes and the chapter's catalogue of the
twelve labels — after the example, where a catalogue belongs.

<a id="the-grammar-head"></a>
## One version, help on nothing, and an `Option` that is never `None`

Lines 35 to 65 are the `Cli` struct with its attributes and the first line of
the `Command` enum, in three fragments: the attribute block, the struct, and
the enum's head. The block ends at line 65 because line 66 is the doc comment
of the first variant, which *Growing the tree* owns; the enum's twelve
variants are four later chapters' blocks, and its closing brace is read at the
end of this page.

The attribute block declares the surface. `#[derive(Parser)]` generates the
parser for the struct that follows, together with `Cli::parse`, which `run`
calls first, and `Cli::command`, which hands the model to whoever asks — four
files in this crate's test directory reach it, one of them through clap's
`CommandFactory` trait, because a command tree is a value a test can walk where a spawned process can only be asked what it prints. `name`
is what `--version` prints in front of the number. `version` is not this
crate's: `grove_loop::VERSION` is a constant the loop's crate root declares as
its own package version, captured at compile time, and both binaries' clap
models read it, so `grove --version` and `grove-llm --version` print one
number. At the corpus this book is frozen against that number is `20.1.0`, the
workspace's release, and the worked example below shows it.

The comment above that line argues for that mechanism, and the argument is
worth reading because a second mechanism would already do the job. Line 3 of
`crates/grove-llm/Cargo.toml` is `version.workspace = true`, as *Orientation*
read, and `crates/grove-loop/Cargo.toml` inherits the same way, so two
independent `env!("CARGO_PKG_VERSION")` reads would agree by inheritance alone.
Reading one constant makes the agreement a fact about a single definition
instead: a binary whose manifest stopped inheriting would still report the
release rather than a version of its own. Why an operator cares is the
comment's last clause, and `the_two_binaries_report_one_version` in
`crates/grove-llm/tests/llm_cli.rs` is where both halves are pinned — that the
two numbers are equal, and that `grove-llm`'s is not `0.1.0`, the number that
test's own comment records a bare `version` attribute answering when this crate
was first split into a package of its own.

`arg_required_else_help = true` is the attribute the struct's `Option` exists
for, and its effect is measured rather than described: a bare `grove-llm`
prints the short help — the `about` line, the usage line, the twelve verbs
with clap's own `help` beside them, and the options — on **stderr** and exits `2`, clap's status for a usage error, where
`grove-llm --help` prints the long form on stdout and exits `0`. `about` is the
one-line description, printed by `-h` and by the bare invocation; `long_about`
is the paragraph `--help` prints instead, and it says what *Orientation*'s
header said: these verbs are the session's, and none is meant for a human at a
terminal.

<!-- fragment «grammar-command-attributes» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="35-50" parent="grammar-cli-and-enum-head" -->
````rust

#[derive(Parser)]
#[command(
    name = "grove-llm",
    // **grove's version, not this package's.** One workspace, one release
    // version: both binaries read one constant instead of each reading its
    // own `env!("CARGO_PKG_VERSION")`, so agreement is a fact about one
    // definition rather than about two manifests staying in step — and a skew
    // between them is exactly what an operator reaches for them to diagnose.
    version = grove_loop::VERSION,
    arg_required_else_help = true,
    about = "Grove: LLM-driven verbs for mid-session use",
    long_about = "Verbs the LLM driving a grove session invokes deterministically. \
They are separated from the human-facing `grove` binary; none of these verbs \
are meant for direct human use."
)]
````
<!-- /fragment -->

The struct has one field, and the field is an `Option` whose `None` is never
observed. The comment gives the reason as clap's: the derive requires the
subcommand field to be optional for `arg_required_else_help` to print help on
a bare invocation, and this page does not re-derive clap's rule. What it can
check is the consequence. `parse` returns only when the vector named a verb —
a bare vector exits inside `parse` with the help, `--help` and `--version` exit
inside `parse` with their output, and an unknown word is a parse error, each
measured in the worked example's table — so `Cli { command: None }` is a value the type allows and the
parser never constructs. The comment's second sentence records when that
became true: the one flag that stood alone, `--content-hash`, was deleted with
the embed it named and the report that was its only caller, and since then
every invocation that parses at all names a verb. `run`'s branch for the `None`
case, read in the worked example, is unreachable for exactly this reason and
is kept so that the type's `None` still has a stated meaning.

<!-- fragment «grammar-cli-struct» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="51-62" parent="grammar-cli-and-enum-head" -->
````rust
pub struct Cli {
    /// The verb to run.
    ///
    /// Optional only because clap's derive requires it to be for
    /// `arg_required_else_help` to print help on a bare invocation; every
    /// invocation that parses at all names a verb. It stopped being genuinely
    /// optional at `delete-provisioning-k19`, which deleted `--content-hash` —
    /// the one metadata flag that stood alone — along with the embed it named
    /// and the build-pairing report that was its only caller.
    #[command(subcommand)]
    pub command: Option<Command>,
}
````
<!-- /fragment -->

The enum's head is a fragment of three lines, one of them blank, and it opens the block four chapters divide.
`#[derive(Subcommand)]` makes each variant one verb, its doc comment that
verb's `--help`, and its name the hyphenated form of the variant's; `pub enum
Command {` is the type `Cli` holds and `run` matches on. Twelve variants follow
between lines 66 and 289, and this page names none of them here: their doc
comments are the help text the guide paraphrases, and each family is read
beside its handler in the chapter that owns it.

<!-- fragment «grammar-command-enum-head» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="63-65" parent="grammar-cli-and-enum-head" -->
````rust

#[derive(Subcommand)]
pub enum Command {
````
<!-- /fragment -->

The composite that reassembles the block is stated here, and the source index
names it as one of the root's twenty-two children.

<!-- fragment «grammar-cli-and-enum-head» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="35-65" parent="source-command-surface" -->
<!-- insert «grammar-command-attributes» -->
<!-- insert «grammar-cli-struct» -->
<!-- insert «grammar-command-enum-head» -->
<!-- /fragment -->

<a id="the-openings"></a>
## One working tree, two locks, one refusal

Lines 863 to 903 are four private helpers, read here before the worked example
because its refusal ending runs through three of them. Every handler that
touches the tree begins with `worktree()`, and every one that opens a tree it
must already find there then calls `readable` or `writable`; the two exceptions
are *Growing the tree*'s — `root-init`, which takes the loop's `write` because
it wants the vacancy, and `leaf-decompose`'s inherited-kind read, which takes
the loop's `read` because *no tree* is an answer it can use. `absent` is the
refusal both helpers return for a working tree that holds no grove. The types
the four helpers pass and return — the working tree's root, `Tree`, `TreeWrite`
and the two shapes `Reading` and `Writing` — are the three rows *Orientation*'s
import table stated in minimum form, and this is the page that reads them in
full.

`worktree` resolves the working tree from the current directory and returns its
root. The resolution is `Workspace::resolve`'s: the closest ancestor of the
current directory holding a `.jj/` directory, canonicalised so that aliases of
one workspace resolve to one root, and refused — before anything is read or
changed — when no ancestor holds one. That refusal is `jj-workspace`'s wording,
not this module's, and it names the command that fixes it: measured, it is
*not a Jujutsu working tree*, then the directory it looked at and above, then
`jj git init --colocate` and `jj git init` as the two remedies, on stderr with
exit `1`. This is the fact the driver resolved before the session
existed and stated in its mandate, the
[stated VCS](../../../CONTEXT.md#stated-vcs): the binary resolves it again on
every verb rather than trusting the mandate, because a verb has no mandate,
only a working directory. The comment's claim is about what is passed to the
loop, and it holds: every verb passes the working-tree root, and the loop joins
`.grove` itself, so no call from this module spells the grove root. The module
does spell `.grove` three times for text a reader sees — in `readable`'s
refusal below, in `resolve`'s answer for the root, which *Reading the tree*
owns, and in `root-init`'s refusal of a root that already holds a grove, which
*Growing the tree* owns — and none of the three reaches a call. One consequence for the worked example:
under a driver, the working tree is resolved twice per verb, once by admission
inside the loop and once here; the two resolutions start from the same
directory and cannot disagree, and the *command resolved* clause of
admission's refusal is the first of them being reported.

<!-- fragment «openings-worktree» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="863-870" parent="openings" -->
````rust
// Resolve the worktree from the cwd. The task-tree verbs run from the worktree
// root (not from inside `.grove/`), and `grove-loop` joins `.grove` itself, so
// no caller here can spell the grove root a second way.
fn worktree() -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    Ok(Workspace::resolve(&cwd)?.root().to_path_buf())
}

````
<!-- /fragment -->

`readable` is the shared opening. `grove_loop::read` takes the shared lock,
reads the tree, and answers with a shape rather than a predicate: a `Tree`
under the lock, or `Vacant` — no tree at this working tree, holding no lock,
because a reader of a tree that is not there has nothing to hold one against.
The doc comment gives the loop's reason for answering vacancy as a value —
the driver scaffolds one — and the page checks it against the driver. The
driver scaffolds through the *exclusive* opening's vacancy, the write-side
twin of this arm, and its one call to `read` treats *vacant* as nothing to
pick; what makes vacancy a value on the read side is the library's shape — a
read of a tree that is not there is an answer, not a failure — and the
comment names the caller that wants a vacancy at all. What holds without
qualification is the rule the helper exists for: a session cannot scaffold
one, so this is where the two callers part. Without it, `pick` on a working tree with no `.grove/` would find
no live leaf and report the grove finished — which is what the loop's own
answer, taken as a tree, would say. The `?` on the call propagates the loop's
other refusals unchanged: a root that is there but unreadable, or a name in it
grove refuses, are a different category from vacancy and carry the loop's own
wording.

<!-- fragment «openings-readable» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="871-884" parent="openings" -->
````rust
/// The shared opening a read verb needs, refusing a worktree with no grove.
///
/// **A grove that is not there is not a grove that is finished**, and every verb
/// here refuses rather than conflating them: a session run one directory off
/// would otherwise be told its work was done. [`grove_loop::read`] answers the
/// vacancy because the *driver* scaffolds one; a session cannot, so this is
/// where the two callers part.
fn readable(worktree: &Path) -> Result<Tree> {
    match grove_loop::read(worktree)? {
        Reading::Tree(tree) => Ok(tree),
        Reading::Vacant => Err(absent(&worktree.join(".grove"))),
    }
}

````
<!-- /fragment -->

`writable` is the exclusive opening, and it differs from `readable` in what
each arm holds. `grove_loop::write` takes the exclusive lock either way. A
`TreeWrite` is the right to be the writer rather than the tree under a lock
held for as long as the value lives: a store mutation consumes its guard, so
the value holds the guard `write` opened with, hands it to the first mutation
a verb makes, and reopens the lock — announcing a new wait — for any second
one; a `Tree`, the shared arm's counterpart, is the tree read once under the
shared lock, and dereferences to its snapshot. A `Vacancy` is a lock over a
root that holds no tree together with the affordance to create one under it. That affordance is what `root-init`
consumes, and *Growing the tree* reads that handler calling the loop's `write`
directly for exactly that reason. Every other mutating verb calls this helper,
and this helper discards the vacancy: the `Vacancy` is dropped when the
arm's error is returned, the lock is released with it, and the verb refuses. A
grow, retire or prune verb that created a grove as a side effect of running in
the wrong directory would turn a mistyped working directory into a second
workstream, and the shape of the
return type is what makes that impossible — there is no path from `writable`
to a `TreeWrite` over a root that had no tree.

<!-- fragment «openings-writable» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="885-893" parent="openings" -->
````rust
/// The exclusive opening a mutating verb needs, refusing a worktree with no
/// grove.
fn writable(worktree: &Path) -> Result<TreeWrite> {
    match grove_loop::write(worktree)? {
        Writing::Tree(tree) => Ok(tree),
        Writing::Vacancy(vacancy) => Err(absent(vacancy.root())),
    }
}

````
<!-- /fragment -->

`absent` is one wording, and both openings call it with the same path. The
refusal names the grove root it looked for and the verb that scaffolds one, on
two lines, because the principle the comment cites by number holds that an
error which only reports detection is unfinished — the same principle
`jj-workspace`'s refusal type is built on. `readable` passes the root it joined
itself; `writable` passes the vacancy's own root, which is the path the loop
spelled from the same working tree, so the two refusals are byte-identical for
one working tree. That is the through-line: because `absent` is
one function taking a path, the shared and the exclusive opening refuse the
same way, and a reader of either refusal cannot tell which lock the verb
wanted. One test pins the first line — `errors_when_grove_root_absent` in
`crates/grove-llm/tests/pick.rs` asserts that `pick` with no `.grove/` fails
and that its stderr contains *grove root not found* — and no test asserts the
second line, so the remedy is held by this function alone.

<!-- fragment «openings-absent» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="894-903" parent="openings" -->
````rust
/// The refusal for a worktree that holds no grove — one wording, and it carries
/// the remedy, because an error that only reports detection is unfinished
/// (principle 2).
fn absent(grove_root: &Path) -> anyhow::Error {
    anyhow::anyhow!(
        "grove root not found: {}\n\nScaffold one with `grove-llm root-init`.",
        grove_root.display()
    )
}

````
<!-- /fragment -->

The composite that reassembles the four helpers is stated here.

<!-- fragment «openings» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="863-903" parent="source-command-surface" -->
<!-- insert «openings-worktree» -->
<!-- insert «openings-readable» -->
<!-- insert «openings-writable» -->
<!-- insert «openings-absent» -->
<!-- /fragment -->

<a id="worked-dispatch"></a>
## Worked example: one verb, three endings, and `--version`

The session is the one *Orientation* carries, and its first verb is this
chapter's example. The driver holds the lease over `/work/atlas/`, has written
the epoch record for this launch, and has started the session with its shell
at `/work/atlas` and `GROVE_SIGNAL_FILE` set to the signal path under
`/work/atlas/.jj/grove/`; the grove holds one live leaf, `rate-limit-k3`. The
section runs `grove-llm resolve rate-limit-k3` through `run` at full
resolution, then the same argument vector one working tree off — `/work/atlas-v2/`,
a Jujutsu workspace with no `.grove/` — in both environments a session can have,
and then `grove-llm --version`. Every line the binary prints below is what the
built binary printed at the frozen corpus, measured by running it — the
admitted line and the wrong-working-tree refusal under a live driver, the rest
without one — with the scratch tree's paths replaced by the carried tree's.

```console
$ cd /work/atlas && grove-llm resolve rate-limit-k3
/work/atlas/.grove/01-impl--rate-limit-k3.md

$ cd /work/atlas-v2 && grove-llm resolve rate-limit-k3
Error: wrong working tree for grove-llm resolve: session belongs to /work/atlas, command resolved /work/atlas-v2

$ cd /work/atlas-v2 && env -u GROVE_SIGNAL_FILE grove-llm resolve rate-limit-k3
Error: grove root not found: /work/atlas-v2/.grove

Scaffold one with `grove-llm root-init`.

$ grove-llm --version
grove-llm 20.1.0
```

The transcript is `run` rendered by its own streams, and the trace below is
what happened between each prompt and its output. The first invocation is the
carried session's, and it is admitted. `Cli::parse` returns a `Cli` whose `command` is
`Some(Command::Resolve { reference: "rate-limit-k3" })`, and the `let … else`
unwraps it; `operation_label` answers
`grove-llm resolve`; the current directory is `/work/atlas`; and
`admit_ambient_session` finds the channel in the environment, reads the epoch
record beside it under a shared lock, and makes five checks against this
process, in order — the record's working tree is the one the current directory
resolves to, by path and then by device and inode; the epoch is active; its
channel is the one in the environment; and the driver that wrote it still
holds the lease — and returns `Some(guard)`. Only then does the
`match` reach `cmd_resolve`, which *Reading the tree* owns; the path on stdout
is that handler's, and its own trace is that chapter's example.

The second invocation is the same vector with the shell one working tree off
and the driver's channel still in the environment, and it never reaches a
handler. Admission resolves the working tree the current directory belongs to,
compares it with the record's, and refuses; the refusal quotes the label and
both paths, and `?` returns it from `run` before the `match`. No lock over any
grove was taken, because `worktree`, `readable` and `writable` are called by
handlers and no handler ran. The third invocation is the same vector and the
same directory with no channel in the environment — a manual command, the
`env -u` above being the operator's spelling of that — and admission returns
`None` at once. Now the `match` runs, `cmd_resolve` calls `worktree`, which
resolves `/work/atlas-v2`, then `readable`, whose answer is *vacant*, and
`absent` refuses with the root it looked for and the verb that scaffolds one.
The fourth invocation is answered inside `parse`: clap prints the name and the
constant and exits `0`, and lines 420 and 421 are never reached. That is the
whole of the exemption — `--version` is not exempt from admission by a rule in
this module, it is exempt because `parse` is the first statement of `run` and
it does not return. The measurement behind that claim is that the same
`--version` printed the same line in a directory that is not a Jujutsu
workspace, and under a stale channel that refuses every verb.

| Argument vector | Environment | Last line of `run` reached | Stream | Exit |
|---|---|---|---|---|
| `grove-llm resolve rate-limit-k3` in `/work/atlas` | the live channel | line 427: `cmd_resolve` dispatched | stdout | `0` |
| the same, in `/work/atlas-v2` | the live channel | line 421: admission refuses | stderr | `1` |
| the same, in `/work/atlas-v2` | no channel | line 427, then `readable` refuses in `cmd_resolve` | stderr | `1` |
| `grove-llm --version`, anywhere | any | line 413: `parse` exits | stdout | `0` |
| `grove-llm --help`, anywhere | any | line 413: `parse` exits | stdout | `0` |
| `grove-llm`, anywhere | any | line 413: `parse` exits | stderr | `2` |
| `grove-llm frobnicate`, anywhere | any | line 413: `parse` exits | stderr | `2` |

The table is the example as a relation: which line of `run` each vector last
reaches, and which actor writes the exit. Four of the seven rows are clap's, two
are refusals this chapter owns, and one is a handler's. `run` itself is three
fragments, and the first is the parse and the branch the struct's `Option`
makes necessary.

<!-- fragment «run-parse-and-bare-branch» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="412-419" parent="run-admission-and-dispatch" -->
````rust
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let Some(command) = cli.command else {
        // Unreachable through clap: a bare invocation prints help
        // (`arg_required_else_help`), and any other argument is either a verb or
        // a parse error.
        bail!("no verb given; run `grove-llm --help` for the verb set");
    };
````
<!-- /fragment -->

`Cli::parse` is the first statement, and it is why `--help`, `--version` and a
bare vector stop where they do: each exits inside this call, so the current
directory is never read and admission is never asked. The `let … else` that
follows unpacks the `Option`, and its `else` branch is the one this chapter has
already shown to be unreachable — a bare invocation is answered by
`arg_required_else_help` inside `parse`, and any other vector is a verb or a
parse error — so the `bail!` exists to give the type's `None` a stated meaning
and a remedy rather than a panic, and no test drives it because no argument
vector can.

The second fragment is the two lines the chapter is named for. The current
directory is read once, with a context naming what it is for, and admission is
asked once with it and the verb's label. The call returns `Option<SessionEpochGuard>`:
`Some` under a driver, `None` for a manual command, and an error for a stale
session — an inactive epoch, another working tree, a channel that is not the
ambient one, or a driver no longer alive — each refusal quoting the label so
the operator reads the command they typed. The guard holds the epoch's shared
lock, and because the binding lives until `run` returns it is alive through
whichever handler the `match` selects; the loop's contract is that it must
remain so through the handler's separately acquired tree lock, and this binding
is what keeps it.

<!-- fragment «run-cwd-and-admission» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="420-421" parent="run-admission-and-dispatch" -->
````rust
    let cwd = std::env::current_dir().context("getting cwd for session epoch admission")?;
    let session_epoch = grove_loop::admit_ambient_session(&cwd, command.operation_label())?;
````
<!-- /fragment -->

The third fragment is the dispatch, and it is exhaustive: twelve arms for
twelve variants, one handler each, and a variant added without an arm fails to
compile. The handlers are four later chapters', and the table below states the
minimum this page owes for each family; the early-use ledger carries the same
rows. One arm is different from the rest. `Complete` alone receives the guard —
`session_epoch.as_ref()` — because `complete` is the one verb that writes to
the channel the epoch admitted, and *Leaving the loop* reads it checking the
channel it is about to write against the one it was admitted under. Every other
handler is called with its arguments and nothing else: admission is a fact
`run` established before the `match`, and no handler has to carry it.

| Handler family | What it is, for this page | Owning chapter |
|---|---|---|
| `cmd_pick`, `cmd_brief_chain`, `cmd_kind`, `cmd_resolve` | One handler per reading verb: the shared opening, one `grove_loop::verbs` call, and rendering. | Reading the tree |
| `cmd_root_init`, `cmd_leaf_add`, `cmd_leaf_insert`, `cmd_leaf_decompose` | One handler per growing verb: text parsed, presence asked, then the exclusive opening, then one call. | Growing the tree |
| `cmd_leaf_retire`, `cmd_leaf_prune` | One handler per terminal mark: the exclusive opening, one call, the marked paths, and the two remaining steps on stderr. | Ending work |
| `cmd_finish_commit`, `cmd_complete` | The two handlers that open no tree: one commits through the workspace, one writes the completion channel. | Leaving the loop |

The table is what a reader carries out of this chapter into the four that
follow: the shape each family's handler has, so that the `match` can be read
without opening any of them. Two facts about the arms are read from the
openings above rather than from the handlers. Every arm but `Complete`
resolves the working tree through `worktree`, and every arm but `RootInit`,
`FinishCommit` and `Complete` then takes one of the two openings — the first
four the shared one, the other five the exclusive one — so the refusal the
example ended on is reachable from nine of the twelve verbs, and the wrong-working-tree
refusal from all twelve.

<!-- fragment «run-dispatch» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="422-437" parent="run-admission-and-dispatch" -->
````rust
    match command {
        Command::RootInit(args) => cmd_root_init(&args),
        Command::Pick => cmd_pick(),
        Command::BriefChain { leaf_path } => cmd_brief_chain(leaf_path.as_deref()),
        Command::Kind { leaf_path } => cmd_kind(leaf_path.as_deref()),
        Command::Resolve { reference } => cmd_resolve(&reference),
        Command::LeafAdd(args) => cmd_leaf_add(&args),
        Command::LeafInsert(args) => cmd_leaf_insert(&args),
        Command::LeafDecompose(args) => cmd_leaf_decompose(&args),
        Command::LeafRetire(args) => cmd_leaf_retire(&args),
        Command::LeafPrune(args) => cmd_leaf_prune(&args),
        Command::FinishCommit { finish_handle } => cmd_finish_commit(&finish_handle),
        Command::Complete(args) => cmd_complete(&args, session_epoch.as_ref()),
    }
}

````
<!-- /fragment -->

The test that holds the order is the loop's, and it drives this binary as a
process under a real driver:
`grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` in
`crates/grove-loop/tests/driver_lease.rs`. It runs `pick` under the live
channel and requires success; `complete` with a channel other than the
admitted one, and requires refusal with no file written; `pick` under a stale
channel, and requires the *stale Grove session* refusal; and `--version` under
that same stale channel, and requires exit `0`. The last assertion is the
exemption stated as a test rather than as a reading of `run`, and it would
fail the day `parse` stopped being the first statement.

The composite that reassembles `run` is stated here.

<!-- fragment «run-admission-and-dispatch» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="412-437" parent="source-command-surface" -->
<!-- insert «run-parse-and-bare-branch» -->
<!-- insert «run-cwd-and-admission» -->
<!-- insert «run-dispatch» -->
<!-- /fragment -->

<a id="the-label"></a>
## The enum's close, and the label a verb is admitted under

The last block this chapter owns is lines 290 to 310: the enum's closing brace,
and the one method on `Command`. The brace closes a type whose body four later
chapters reproduce, and the blank line after it is the block's.

<!-- fragment «grammar-command-enum-close» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="290-291" parent="enum-close-and-operation-label" -->
````rust
}

````
<!-- /fragment -->

`operation_label` is the label each verb is admitted under, and now that the
example has shown one — `grove-llm resolve`, quoted in the wrong-working-tree
refusal — the catalogue of all twelve can follow it. The method is private with
one caller, `run`, and it returns a `&'static str` because every label is a
literal: the binary's name, a space, and the verb as the operator typed it.
That spelling is the point. Admission's refusals are the loop's wording, and
the loop does not know which verb asked; the label is the one thing this module
passes in, and it is what turns *stale Grove session* into *stale Grove session
for grove-llm resolve*. The `match` is exhaustive, so the twelve arms are the
twelve verbs, and a thirteenth variant without a label would not compile — the
same guarantee the dispatch in `run` gives, held in a second place for a
different string. The patterns also show the three shapes a variant takes,
which the four owning chapters read one family at a time: a unit variant with
no arguments, a variant with named fields the handler receives directly, and a
variant wrapping an argument struct that `derive(Parser)` fills.

<!-- fragment «grammar-operation-label» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="292-310" parent="enum-close-and-operation-label" -->
````rust
impl Command {
    fn operation_label(&self) -> &'static str {
        match self {
            Self::RootInit(_) => "grove-llm root-init",
            Self::Pick => "grove-llm pick",
            Self::BriefChain { .. } => "grove-llm brief-chain",
            Self::Kind { .. } => "grove-llm kind",
            Self::Resolve { .. } => "grove-llm resolve",
            Self::LeafAdd(_) => "grove-llm leaf-add",
            Self::LeafInsert(_) => "grove-llm leaf-insert",
            Self::LeafDecompose(_) => "grove-llm leaf-decompose",
            Self::LeafRetire(_) => "grove-llm leaf-retire",
            Self::LeafPrune(_) => "grove-llm leaf-prune",
            Self::FinishCommit { .. } => "grove-llm finish-commit",
            Self::Complete(_) => "grove-llm complete",
        }
    }
}

````
<!-- /fragment -->

The composite that reassembles the block is stated here.

<!-- fragment «enum-close-and-operation-label» owner="admitted-before-dispatch" source="crates/grove-llm/src/cli.rs" lines="290-310" parent="source-command-surface" -->
<!-- insert «grammar-command-enum-close» -->
<!-- insert «grammar-operation-label» -->
<!-- /fragment -->

The grammar has now been read up to the point where every verb looks the same:
parsed by clap, labelled, admitted, dispatched. What the handlers do after the
`match` is where the verbs differ, and the first four to read are the ones
whose absent answer is information rather than an error.

[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Reading the tree](03-reading-the-tree.md)
