# Syllabus CLI
<!-- book-page id="syllabus-cli" slice="syllabus-cli-k17" order="7" -->
[Previous: Filesystem interpreter](06-filesystem-interpreter.md) | [Contents](README.md)

The `syllabus` binary is a concrete consumer of `ordinal-fs-tree`. It gives the
opaque `Parts` type the syllabus vocabulary of lessons, modules, publication
status, labels, and `OVERVIEW.md`. The library remains domain-independent: its
only consumer seam is `EntryName`, and it neither parses argv nor defines these
verbs. The person or process invoking `syllabus` is the operator; the binary is
the consumer that translates operator arguments into library values.

The binary is a demonstration rather than a generic interface. A generic
command set could obtain opaque parts only by asking for a complete filename
whose ordinal and key the library would discard, or by adding a second
consumer-supplied parts parser beside `EntryName`. The first form gives the
operator misleading arguments and the second widens the library seam. A real
consumer also names commands for its own work: `lesson-insert` and `publish`
rather than `insert` and `rewrite`.

<a id="package-boundary"></a>
## Package and consumer boundary

`clap` is optional and belongs only to the `cli` feature. The feature is enabled
by default so a plain `cargo test` builds the binary and includes its contract
tests; an external library consumer can select `default-features = false` and
retain the library's `libc`-only dependency set. This manifest fragment is
present because the package owns the feature boundary, Cargo turns that feature
selection into the optional parser dependency, the boundary prevents command
parsing from entering the reusable library, and the default makes the current
chapter's consumer part of ordinary verification.

<!-- fragment «manifest-cli-feature» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/Cargo.toml" lines="43-45" parent="source-crate-manifest" -->
````toml
[features]
default = ["cli"]
cli = ["dep:clap"]
````
<!-- /fragment -->

The binary source stays at `bin/syllabus.rs`, outside `src/`. The crate's
no-filesystem guard treats every new module under `src/` outside `src/fs/` as
algebra and rejects access to `fs`; this consumer must call `fs::read` and
`fs::write`. This manifest fragment is present because Cargo owns executable
selection, its binary declaration maps the `syllabus` name to the external
consumer source, the required feature keeps an unavailable parser from being
built, and the placement preserves the library/filesystem boundary used by the
complete insert below.

<!-- fragment «manifest-cli-binary» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/Cargo.toml" lines="62-65" parent="source-crate-manifest" -->
````toml
[[bin]]
name = "syllabus"
path = "bin/syllabus.rs"
required-features = ["cli"]
````
<!-- /fragment -->

The CLI source is one file with eight conceptual ranges. The composite below
preserves source order while the literal definitions appear beside their
explanations. The binary owns translation between argv, the syllabus domain,
the library surface, and terminal streams; its input is argv and its output is
parser text, records, advisories, or a categorized failure. Keeping the whole
file under this consumer preserves the invariant that none of those concerns
becomes a second library seam, and the fragments resolve the insert tour at the
source boundary.

<!-- fragment «syllabus-cli-source» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="1-1439" parent="source-syllabus-cli" -->
<!-- insert «cli-command-line» -->
<!-- insert «cli-parsing-and-failure» -->
<!-- insert «cli-streams-and-paths» -->
<!-- insert «cli-mutation-output» -->
<!-- insert «cli-main-dispatch» -->
<!-- insert «cli-reading» -->
<!-- insert «cli-mutations» -->
<!-- insert «cli-stream-contract-tests» -->
<!-- /fragment -->

<a id="worked-cli-insert"></a>
## One complete CLI insert

The operation begins with the same tree and command used in the orientation
tour:

```text
s/
├── OVERVIEW.md
├── 01-published-orientation-i1.md
└── 02-linear-algebra-i2/
    ├── OVERVIEW.md
    ├── 01-published-foundations-i3.md
    ├── 02-published-vectors-i5.md
    └── 03-draft-matrices-i6.md
```

```console
syllabus --root s lesson-insert 2 2 limits
```

`Cli::parse` leaves the root spelling as `PathBuf::from("s")` and produces
`Verb::LessonInsert`. The custom value parsers turn the first `2` into
`Target::Key(Key::new(2))`, the second into `Ordinal::new(2)`, and `limits`
into a validated `Label`. The absent `--status` uses `Status::Draft`. `run`
constructs `Parts::lesson(Status::Draft, label)` and dispatches those values to
the local `insert` helper.

The helper calls `fs::write::<SyllabusName>(&cli.root)`. Acquisition locks the
directory containing `s` exclusively and reads the complete tree only after the
lock succeeds. Its `WriteGuard` therefore contains the operator-supplied root
spelling, one
exclusive lock, and the snapshot used for the decision. `NewEntry::empty(parts)`
states that the new lesson has no initial bytes.

`WriteGuard::insert` consumes the guard and passes its captured `Snapshot`,
target, ordinal, and new entry to `ops::insert`. The algebra resolves key 2 to
the `02-linear-algebra-i2` level, finds ordinal 2 occupied, allocates key 7 from
the tree-wide maximum 6, and returns a guarded plan with three effects:

```text
1. MoveTo key 6:
   s/02-linear-algebra-i2/03-draft-matrices-i6.md
   → s/02-linear-algebra-i2/04-draft-matrices-i6.md
2. MoveTo key 5:
   s/02-linear-algebra-i2/02-published-vectors-i5.md
   → s/02-linear-algebra-i2/03-published-vectors-i5.md
3. Create key 7:
   s/02-linear-algebra-i2/02-draft-limits-i7.md
```

The plan moves displaced siblings highest ordinal first and preserves each
sibling's key and parts. `Plan::guarded` checks the effects sequentially against
an in-memory state, so every destination is available in the same order that
the interpreter will use. A refusal ends here with no effects.

`WriteGuard::run` maps a refusal to `Error::Refused` or gives the proceeding
plan to `fs::apply::apply`. The interpreter validates every rendered name as
one path component and calls `Run::step` in plan order. Each successful step
registers its reverse action before the next step. A forward failure calls
`Run::unwind` in reverse order and becomes exit 6 after complete restoration or
exit 7 after incomplete restoration. All three steps succeed in this example,
so the interpreter returns a `Report` containing two renames, one create, and
three landed paths.

Only after success does `report_out` write terminal output. `mutation_records`
selects `Report::created()` because it is non-empty, so stdout contains the new
subject and excludes shifted siblings:

```text
7	s/02-linear-algebra-i2/02-draft-limits-i7.md
```

Unless `--quiet` is set, `Streams::trace` walks `Report::paths()` in landing
order and correlates each destination with `Report::renamed()` to label the
advisory stderr trace:

```text
lesson-insert: 3 effects, in the order they landed:
  renamed  s/02-linear-algebra-i2/03-draft-matrices-i6.md -> s/02-linear-algebra-i2/04-draft-matrices-i6.md
  renamed  s/02-linear-algebra-i2/02-published-vectors-i5.md -> s/02-linear-algebra-i2/03-published-vectors-i5.md
  created  s/02-linear-algebra-i2/02-draft-limits-i7.md
```

`run` returns `Ok(())`, `main` returns normally, and the process exits 0. The
resulting level is:

```text
02-linear-algebra-i2/
├── OVERVIEW.md
├── 01-published-foundations-i3.md
├── 02-draft-limits-i7.md
├── 03-published-vectors-i5.md
└── 04-draft-matrices-i6.md
```

<a id="command-grammar"></a>
## Command grammar and target syntax

`Cli` owns the operator-supplied root spelling, the advisory-stream switch, and one flat
`Verb`. Existing entries are always named by stable key. The spelling `.` names
the root only where an add or insert expects a parent level. Ordinals appear
only in `lesson-insert` and `module-insert`, where the operator chooses an
occupied slot. The consumer chooses a lesson or module through the verb and
constructs the corresponding parts; the generic library continues to derive
file-versus-directory species from those parts.

The command-line range is present here because the binary owns its argument
contract and help, argv becomes typed `Cli` and `Verb` values, target syntax
keeps stable identity separate from mutable position, and these are the exact
inputs that initiated the worked insert. It also records the twelve commands
without restating clap mechanics in prose.

<!-- fragment «cli-command-line» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="1-488" parent="syllabus-cli-source" -->
````rust
//! `syllabus` — the CLI `ordinal-fs-tree` ships, driving a course syllabus.
//!
//! The reference domain [`ordinal_fs_tree::reference`] is the syllabus the
//! architecture document uses for every one of its examples, and this binary is
//! the library's first end-to-end consumer and its worked example of what a
//! consumer looks like. It is a **demonstration rather than a product surface**:
//! nothing is published, and grove's own CLI in increment 2 is built from the
//! library directly rather than from anything here.
//!
//! The design of record is `docs/ordinal-fs-tree/CLI.md`, which this file does
//! not restate. Two things it settles are worth carrying at the top of the
//! source, because they are what a reader will otherwise try to "fix":
//!
//! - **The CLI is not generic, and that is the decision rather than an
//!   omission.** `EntryName::Parts` is opaque, so a generic CLI's only route to
//!   one is round-tripping a whole filename through `parse` — an argument two
//!   thirds of which the library allocates and discards. The alternative that
//!   produces good arguments parameterises the command set by a parts-parser,
//!   which is a *second* point at which the library is parameterised by its
//!   consumer and falsifies `docs/adr/entry-name-is-the-only-seam.md`. So: the
//!   library drives any conforming tree; the CLI drives the reference tree and
//!   shows a consumer how.
//! - **`main` does not return `Result`.** Rust's default reporter prints
//!   `{:?}`, and `Error<N>`'s hand-written `Debug` is a field dump — the
//!   recovery advice this design went out of its way to preserve lives in
//!   `Display`, and the one line of boilerplate everybody writes would throw it
//!   away.
//!
//! Nothing here is modelled. Neither `structure.als` nor `operations.qnt` holds
//! strings, arguments, streams or exit codes, so the tests beside this file
//! mostly say they discharge no claim — which is the honest reading of the
//! routing rule in `docs/formalism-findings.md` entry 009 and not a gap.

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusName};
use ordinal_fs_tree::{
    fs, Container, Entry, EntryNameExt, Error, Key, NewEntry, Ordinal, Refusal, Report, Target,
};

// ---------------------------------------------------------------------------
// The command line
// ---------------------------------------------------------------------------

/// Drive a course syllabus stored as a directory tree.
#[derive(Parser)]
#[command(
    name = "syllabus",
    version,
    about = "Drive a course syllabus stored as a directory tree.",
    long_about = LONG_ABOUT,
    // Twelve verbs, flat and hyphenated, so one `syllabus --help` enumerates
    // all of them: a caller that has lost its bearings recovers in one call.
    subcommand_required = true,
    arg_required_else_help = true
)]
struct Cli {
    /// The tree root. Nothing canonicalises it, so every path printed is built
    /// from this spelling verbatim.
    ///
    /// `--root syllabus` reports `syllabus/01-…`; `--root "$PWD/syllabus"`
    /// reports the absolute form. That is the library's own property made
    /// visible rather than a convenience: on macOS `/var` and `/private/var`
    /// name one inode, so canonicalising would make the mere presence of a lock
    /// observably rewrite every path a read verb returns.
    #[arg(long, global = true, default_value = ".", value_name = "PATH")]
    root: PathBuf,

    /// Suppress the advisory stream. Errors are never suppressed.
    #[arg(long, short, global = true)]
    quiet: bool,

    #[command(subcommand)]
    verb: Verb,
}

const LONG_ABOUT: &str = "\
Drive a course syllabus stored as a directory tree.

This binary is a demonstration: it is the `ordinal-fs-tree` library's worked
example of a consumer, driving the reference domain the architecture document
uses for its examples. Modules are directories, lessons are files, each lesson
carries a draft/published status, and a level's own content is its OVERVIEW.md.
There is no index, no database and no metadata file — the directory listing IS
the data structure, so `ls`, `mv` and `git diff` all work on it.

A name looks like `02-draft-limits-i9.md`: ordinal 02, status draft, label
limits, key 9. A module drops the status and the suffix: `02-linear-algebra-i2`.

NAMING A TARGET
  Every verb that names an existing entry names it BY KEY — the decimal after
  `i` — because a key is the one handle that survives insertion, reordering,
  relabelling and being moved between levels. An ordinal is stale the moment
  anything is inserted before it, and a path the moment anything is renamed.
  `.` means the tree root, and only as the <PARENT> of the four add/insert
  verbs, whose target is a level rather than an entry.

OUTPUT
  stdout is data: one record per line, `<key>` TAB `<path>`. Split on the FIRST
  tab, then percent-decode `%HH` escapes in the path. Printable ASCII is literal
  except `%`; controls and non-ASCII platform bytes are escaped, so tabs,
  newlines and non-UTF-8 names still occupy one round-trippable UTF-8 line. The
  encoded bytes may be reconstructed only on the same Rust version and target.
  The key column is the target you would pass to another verb to name what that
  line is about. `.` in the key column is the tree root.
  stderr is advisory: what else moved, and why a result was empty. `--quiet`
  suppresses it. Errors go to stderr and are never suppressed.
  A mutation prints what it created; a mutation that created nothing prints
  what it renamed. The siblings an insert shifts are on stderr, not stdout.

EXIT CODES
  0  success
  1  the environment refused (filesystem or terminal I/O, or a root with no
     containing directory) — fix the path, permissions or redirection
  2  usage: bad arguments, an unparseable label, an unknown status
  3  no entry has that key — run `syllabus list` to find the key you meant
  4  refused: a stated outcome in which nothing changed — read the message,
     it names the remedy
  5  this tree cannot be read as a syllabus — a human fixes a filename, and
     no retry helps
  6  the mutation failed and was rolled back: THE TREE IS AS IT WAS FOUND,
     so retrying is safe
  7  the mutation failed and the rollback failed too: DO NOT RETRY — the
     message says how to resolve it

  A terminal-I/O exit 1 can occur after a mutation landed. Inspect the tree
  before retrying a non-idempotent add, insert or promote.

IDEMPOTENCY
  `publish`, `unpublish` and `relabel` are idempotent: rewriting to the parts
  an entry already carries succeeds and changes nothing. The add, insert and
  promote verbs are NOT — running an add twice creates two entries. After exit
  6 a retry is safe because nothing landed; after the process is KILLED it is
  not, because what landed is unknowable.

THERE IS NO REMOVAL
  Keys are allocated as max+1 over the names in the tree, so deleting an entry
  lowers the maximum and the next add re-issues a key other entries may still
  reference. Retire a lesson with `unpublish`, which is what an attribute is
  for. Removing a file by hand damages key allocation for every later add.

OTHER THINGS WORTH KNOWING
  There is no `init`: an empty directory IS an empty tree, so `mkdir` is the
  whole of it. There is no `--dry-run`: a plan is internal by design. There are
  no lock flags: every verb takes an advisory lock on the directory containing
  the root and BLOCKS until the tree is free — a hang is a lock and not a bug.
  Nothing is staged in version control; a rename is rename(2).

EXAMPLES
  mkdir syllabus
  syllabus --root syllabus module-add . linear-algebra
  syllabus --root syllabus lesson-add 1 vectors matrices
  syllabus --root syllabus list
  syllabus --root syllabus publish 2

SEE ALSO
  `syllabus <verb> --help` for any verb below.";

#[derive(Subcommand)]
enum Verb {
    /// List entries in walk order.
    ///
    /// Depth-first, pre-order: within a level the OVERVIEW comes first, then
    /// the children by ordinal, and a module is fully explored before its next
    /// sibling. `--status` and `--label` build a predicate; `--first` decides
    /// whether that predicate short-circuits at the first match or filters the
    /// whole walk. An OVERVIEW carries no parts, so it matches no filter and is
    /// dropped whenever one is given.
    ///
    /// There is no default page and no `--limit`: the result set is bounded by
    /// the tree you named, and a silently truncated tree listing is exactly the
    /// failure this library's no-silent-skip rule exists to prevent. Narrow
    /// with the flags, or truncate with `head`.
    #[command(
        alias = "ls",
        after_help = "\
EXAMPLES
  syllabus list
  syllabus list --status draft
  syllabus list --under 2 --label vectors
  syllabus list --status draft --first

SEE ALSO
  show, ancestors, overview-chain"
    )]
    List {
        /// Keep only entries below the module with this key.
        ///
        /// A filter this CLI applies rather than a library operation — there is
        /// no subtree walk. The module itself is not in its own listing.
        #[arg(long, value_name = "KEY", value_parser = parse_key)]
        under: Option<Key>,

        /// Keep only lessons with this status: `draft` or `published`.
        #[arg(long, value_name = "STATUS", value_parser = parse_status)]
        status: Option<Status>,

        /// Keep only entries with this exact label.
        #[arg(long, value_name = "LABEL", value_parser = parse_label)]
        label: Option<Label>,

        /// Stop at the first match instead of listing every one.
        #[arg(long)]
        first: bool,
    },

    /// Show the entry with this key.
    #[command(after_help = "\
EXAMPLES
  syllabus show 9
  syllabus --root ~/course show 1

SEE ALSO
  list, ancestors")]
    Show {
        /// The key: the decimal a name carries after `i`.
        #[arg(value_parser = parse_key)]
        key: Key,
    },

    /// Show the levels containing this entry, root-first.
    ///
    /// The chain ends at the tree root, which is a level and not an entry, so
    /// its record's key column is `.` and its path is the root as you spelled
    /// it.
    #[command(after_help = "\
EXAMPLES
  syllabus ancestors 6
  syllabus ancestors 6 | head -1

SEE ALSO
  overview-chain, show")]
    Ancestors {
        /// The key of the entry whose containing levels to show.
        #[arg(value_parser = parse_key)]
        key: Key,
    },

    /// Show the OVERVIEW of each level containing this entry, root-first.
    ///
    /// Every piece of a level's own content on the path down to the entry,
    /// which is what assembling context out of a tree wants. It walks the
    /// entry's ANCESTORS, so a module's own OVERVIEW is not in its own chain —
    /// that is the one thing about this verb a reader guesses wrong. Levels
    /// with no OVERVIEW are skipped.
    #[command(after_help = "\
EXAMPLES
  syllabus overview-chain 6
  syllabus --root ~/course overview-chain 6

SEE ALSO
  ancestors, list")]
    OverviewChain {
        /// The key of the entry whose chain of OVERVIEWs to show.
        #[arg(value_parser = parse_key)]
        key: Key,
    },

    /// Add lessons at the end of a level.
    ///
    /// One per label, at consecutive ordinals with consecutive keys, planned
    /// from one reading of the tree and applied as a unit: either the whole run
    /// lands or none of it does. Lessons are created EMPTY — write their bytes
    /// afterwards with an editor or a shell redirect, which the printed path
    /// makes a one-liner.
    ///
    /// NOT idempotent: running it twice creates two sets of lessons.
    #[command(after_help = "\
EXAMPLES
  syllabus lesson-add . orientation
  syllabus lesson-add 2 vectors matrices determinants
  syllabus lesson-add 2 errata --status published
  echo '# Vectors' > \"$(syllabus lesson-add 2 vectors | cut -f2-)\"

SEE ALSO
  lesson-insert, module-add, promote")]
    LessonAdd {
        /// The level to add to: a module's key, or `.` for the tree root.
        #[arg(value_name = "PARENT", value_parser = parse_target)]
        parent: Target,

        /// One or more labels, in the order the lessons should appear.
        #[arg(required = true, num_args = 1.., value_name = "LABEL", value_parser = parse_label)]
        labels: Vec<Label>,

        /// The status every lesson in this run starts at.
        #[arg(long, value_name = "STATUS", default_value = "draft", value_parser = parse_status)]
        status: Status,
    },

    /// Add modules at the end of a level.
    ///
    /// One per label, at consecutive ordinals with consecutive keys, applied as
    /// a unit. A module is a directory and carries no status — publication is a
    /// property of a lesson here.
    ///
    /// NOT idempotent: running it twice creates two sets of modules.
    #[command(after_help = "\
EXAMPLES
  syllabus module-add . linear-algebra calculus
  syllabus module-add 2 exercises

SEE ALSO
  module-insert, lesson-add, promote")]
    ModuleAdd {
        /// The level to add to: a module's key, or `.` for the tree root.
        #[arg(value_name = "PARENT", value_parser = parse_target)]
        parent: Target,

        /// One or more labels, in the order the modules should appear.
        #[arg(required = true, num_args = 1.., value_name = "LABEL", value_parser = parse_label)]
        labels: Vec<Label>,
    },

    /// Insert a lesson at an occupied ordinal, shifting later siblings up.
    ///
    /// <AT> is an ORDINAL — the two-digit number a name starts with — and it is
    /// the one place this CLI asks you to read the grammar. Naming the entry
    /// whose slot to take would be friendlier and was rejected: it invents a
    /// key-to-ordinal lookup the library does not have, and it makes the refusal
    /// unreachable that tells you which ordinals the level actually occupies.
    ///
    /// Inserting past the last sibling is refused rather than quietly
    /// redirected — that is `lesson-add`'s job, and the two differ in what they
    /// do to every later sibling. Guess, and the refusal tells you the level's
    /// least and greatest occupied ordinals.
    ///
    /// NOT idempotent.
    #[command(after_help = "\
EXAMPLES
  syllabus lesson-insert 2 1 prerequisites
  syllabus lesson-insert . 3 interlude --status published

SEE ALSO
  lesson-add, module-insert")]
    LessonInsert {
        /// The level to insert into: a module's key, or `.` for the tree root.
        #[arg(value_name = "PARENT", value_parser = parse_target)]
        parent: Target,

        /// The ordinal to take. Its occupant and every later sibling shift up.
        #[arg(value_name = "AT", value_parser = parse_ordinal)]
        at: Ordinal,

        /// The new lesson's label.
        #[arg(value_parser = parse_label)]
        label: Label,

        /// The status the new lesson starts at.
        #[arg(long, value_name = "STATUS", default_value = "draft", value_parser = parse_status)]
        status: Status,
    },

    /// Insert a module at an occupied ordinal, shifting later siblings up.
    ///
    /// <AT> is an ORDINAL, for the reason `lesson-insert --help` gives. Each
    /// shift is a single rename, so a shifted module carries its whole subtree
    /// with it and nothing inside it is touched.
    ///
    /// NOT idempotent.
    #[command(after_help = "\
EXAMPLES
  syllabus module-insert . 2 foundations
  syllabus module-insert 3 1 warm-up

SEE ALSO
  module-add, lesson-insert")]
    ModuleInsert {
        /// The level to insert into: a module's key, or `.` for the tree root.
        #[arg(value_name = "PARENT", value_parser = parse_target)]
        parent: Target,

        /// The ordinal to take. Its occupant and every later sibling shift up.
        #[arg(value_name = "AT", value_parser = parse_ordinal)]
        at: Ordinal,

        /// The new module's label.
        #[arg(value_parser = parse_label)]
        label: Label,
    },

    /// Turn a lesson into a module, moving its bytes into the module's OVERVIEW.
    ///
    /// The module keeps the lesson's OWN ordinal and its OWN key: the entry that
    /// was a lesson IS the module, so every reference to it by key still
    /// resolves. Its bytes move verbatim into the new module's OVERVIEW.md.
    ///
    /// This is the one operation that breaks an invariant on the way through —
    /// between its two effects a file and a directory share an ordinal and a
    /// key — and therefore the one path by which this tool can damage a tree. If
    /// the rollback of a failed promotion itself fails you get exit 7, and the
    /// message says how to resolve it: a module and a lesson sharing an ordinal
    /// and a key, with the module holding no OVERVIEW, is an interrupted
    /// promotion, and removing either half resolves it.
    ///
    /// NOT idempotent, and there is nothing that turns a module back into a
    /// lesson: a lesson's content has somewhere to land and a module's children
    /// have nowhere.
    #[command(after_help = "\
EXAMPLES
  syllabus promote 9 limits
  syllabus promote 9 limits --first-lesson definition

SEE ALSO
  module-add, relabel")]
    Promote {
        /// The key of the lesson to promote.
        #[arg(value_parser = parse_key)]
        key: Key,

        /// The new module's label.
        #[arg(value_parser = parse_label)]
        label: Label,

        /// Also create one draft lesson inside the new module, in the same unit.
        #[arg(long, value_name = "LABEL", value_parser = parse_label)]
        first_lesson: Option<Label>,
    },

    /// Change an entry's label, keeping its place, its key and its kind.
    ///
    /// One rename. On a module it is the directory that is renamed, so its whole
    /// subtree comes with it untouched. A lesson keeps whatever status it has.
    ///
    /// Idempotent: relabelling to the label an entry already carries succeeds
    /// and changes nothing.
    #[command(after_help = "\
EXAMPLES
  syllabus relabel 9 limits-and-continuity
  syllabus relabel 2 linear-algebra-i

SEE ALSO
  publish, unpublish, promote")]
    Relabel {
        /// The key of the entry to relabel.
        #[arg(value_parser = parse_key)]
        key: Key,

        /// The new label.
        #[arg(value_parser = parse_label)]
        label: Label,
    },

    /// Mark a lesson published.
    ///
    /// One rename, keeping the lesson's place, its key and its label. Modules
    /// carry no status here, so publishing one is refused by this CLI before it
    /// reaches the library — use `relabel` if what you meant was the name.
    ///
    /// Idempotent: publishing a published lesson succeeds and changes nothing.
    #[command(after_help = "\
EXAMPLES
  syllabus publish 9
  syllabus list --status draft | cut -f1 | xargs -n1 syllabus publish

SEE ALSO
  unpublish, relabel")]
    Publish {
        /// The key of the lesson to publish.
        #[arg(value_parser = parse_key)]
        key: Key,
    },

    /// Mark a lesson draft — and how a lesson is retired here.
    ///
    /// There is no removal in this tree, deliberately: keys are allocated as
    /// max+1, so deleting an entry lowers the maximum and the next add re-issues
    /// a key other entries may still reference. Taking a lesson out of
    /// circulation is an attribute change, which is what this is.
    ///
    /// Idempotent: unpublishing a draft lesson succeeds and changes nothing.
    #[command(after_help = "\
EXAMPLES
  syllabus unpublish 9
  syllabus list --status published --under 2

SEE ALSO
  publish, relabel")]
    Unpublish {
        /// The key of the lesson to unpublish.
        #[arg(value_parser = parse_key)]
        key: Key,
    },
}

````
<!-- /fragment -->

<a id="dispatch-and-verbs"></a>
## Dispatch and verbs

`main` parses once, locks both terminal handles, creates `Streams`, and delegates
to `run`. `settle` turns the result into a process code and attempts to report a
failure through the same stderr seam; an stderr refusal becomes exit 1. `run` is
the complete mapping from the twelve domain verbs to four read helpers and five
mutation helpers. It constructs `Parts` only for commands where the operator
chooses a species. `publish` and `unpublish` are domain spellings of `rewrite`;
both add commands use `append_many`, including for one label.

This range is present because the binary owns domain dispatch, a typed verb
becomes one helper call with fully constructed consumer parts, exhaustive enum
matching keeps every command routed exactly once, and `LessonInsert` follows
the worked command into the helper rather than exposing a plan to the operator.

<!-- fragment «cli-main-dispatch» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="905-997" parent="syllabus-cli-source" -->
````rust
// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut stdout = stdout.lock();
    let mut stderr = stderr.lock();
    let mut streams = Streams::new(&mut stdout, &mut stderr, false);
    let code = match Cli::try_parse() {
        Ok(cli) => {
            streams.quiet = cli.quiet;
            let outcome = run(&cli, &mut streams);
            settle(outcome, &mut streams)
        }
        Err(error) => streams.clap(error),
    };
    if code != 0 {
        std::process::exit(i32::from(code));
    }
}

fn settle(outcome: Result<(), Failure>, streams: &mut Streams<'_>) -> u8 {
    match outcome {
        Ok(()) => 0,
        Err(failure) => match streams.failure(&failure) {
            Ok(()) => failure.code,
            Err(_) => 1,
        },
    }
}

fn run(cli: &Cli, streams: &mut Streams<'_>) -> Result<(), Failure> {
    match &cli.verb {
        Verb::List {
            under,
            status,
            label,
            first,
        } => list(cli, streams, *under, *status, label.as_ref(), *first),
        Verb::Show { key } => show(cli, streams, *key),
        Verb::Ancestors { key } => ancestors(cli, streams, *key),
        Verb::OverviewChain { key } => overview_chain(cli, streams, *key),

        Verb::LessonAdd {
            parent,
            labels,
            status,
        } => add(cli, streams, "lesson-add", *parent, {
            labels
                .iter()
                .map(|label| Parts::lesson(*status, label.clone()))
                .collect()
        }),
        Verb::ModuleAdd { parent, labels } => add(cli, streams, "module-add", *parent, {
            labels
                .iter()
                .map(|label| Parts::module(label.clone()))
                .collect()
        }),
        Verb::LessonInsert {
            parent,
            at,
            label,
            status,
        } => insert(
            cli,
            streams,
            "lesson-insert",
            *parent,
            *at,
            Parts::lesson(*status, label.clone()),
        ),
        Verb::ModuleInsert { parent, at, label } => insert(
            cli,
            streams,
            "module-insert",
            *parent,
            *at,
            Parts::module(label.clone()),
        ),
        Verb::Promote {
            key,
            label,
            first_lesson,
        } => promote(cli, streams, *key, label.clone(), first_lesson.as_ref()),
        Verb::Relabel { key, label } => relabel(cli, streams, *key, label.clone()),
        Verb::Publish { key } => set_status(cli, streams, "publish", *key, Status::Published),
        Verb::Unpublish { key } => set_status(cli, streams, "unpublish", *key, Status::Draft),
    }
}

````
<!-- /fragment -->

The read verbs and their output are:

| Verb | Library query | Result records |
|---|---|---|
| `list [--under KEY] [--status STATUS] [--label LABEL] [--first]` | `walk`, or short-circuiting `find` with `--first` | Matching entries in walk order |
| `show KEY` | `by_key` | One entry |
| `ancestors KEY` | `ancestors` | Containing levels, root-first |
| `overview-chain KEY` | `distinguished_chain` | Existing overviews in containing levels, root-first |

`list --under` is a consumer-side predicate over each entry's ancestors, not a
subtree operation. Status and label filters inspect syllabus parts; a
distinguished child has no parts and matches no such filter. An empty result is
successful: stderr distinguishes an empty tree from filters excluding every
entry, while stdout remains empty. A missing key instead constructs the public
`Refusal::TargetMissing` so all read verbs use the library's established
message and exit category.

The read-helper range is present because the consumer owns filtering and path
rendering around generic snapshot queries, a locked snapshot becomes ordered
records or one explicit missing-target refusal, filters never widen the
library's name seam, and it supplies the read half of the same consumer that
the insert example exercises.

<!-- fragment «cli-reading» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="998-1136" parent="syllabus-cli-source" -->
````rust
// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

fn list(
    cli: &Cli,
    streams: &mut Streams<'_>,
    under: Option<Key>,
    status: Option<Status>,
    label: Option<&Label>,
    first: bool,
) -> Result<(), Failure> {
    let tree = fs::read::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;

    // A `--under` naming nothing is the same condition `show` meets, and gets
    // the same refusal rather than an empty listing that looks like an answer.
    if let Some(key) = under {
        if tree.by_key(key).is_none() {
            return Err(Failure::refused(&Refusal::TargetMissing { key }));
        }
    }

    let filtered = under.is_some() || status.is_some() || label.is_some();
    let matches = |entry: &Entry<'_, SyllabusName>| {
        if let Some(key) = under {
            // `ancestors()` and not a subtree walk: there is no subtree walk in
            // the library, and this is the CLI applying a filter to a full one.
            if !entry
                .ancestors()
                .iter()
                .any(|level| level.entry().and_then(|node| node.key()) == Some(key))
            {
                return false;
            }
        }
        // An OVERVIEW carries no parts, so it matches no filter and is dropped
        // whenever one is given.
        match (status, label) {
            (None, None) => true,
            (wanted_status, wanted_label) => match entry.triple() {
                None => false,
                Some(triple) => {
                    let status_matches = match (wanted_status, triple.parts) {
                        (None, _) => true,
                        (Some(wanted), Parts::Lesson { status, .. }) => *status == wanted,
                        (Some(_), Parts::Module { .. }) => false,
                    };
                    status_matches
                        && wanted_label.is_none_or(|wanted| triple.parts.label() == wanted)
                }
            },
        }
    };

    // `--first` decides whether the predicate goes to `find`, which
    // short-circuits, or filters a full `walk`. That is the architecture's *a
    // predicate passed to `find` answers them without the library ever learning
    // what it asked*, spelled as a flag.
    let records: Vec<Record> = if first {
        tree.find(matches)
            .iter()
            .map(|entry| record_of(&cli.root, entry))
            .collect()
    } else {
        tree.walk()
            .filter(matches)
            .map(|entry| record_of(&cli.root, &entry))
            .collect()
    };

    if records.is_empty() {
        streams.note(&empty_note(tree.snapshot().is_empty(), filtered, &cli.root))?;
    }
    streams.records(&records)
}

/// Which emptiness it was. Exit is 0 either way — an empty tree is a tree.
fn empty_note(tree_is_empty: bool, filtered: bool, root: &Path) -> String {
    if tree_is_empty {
        format!(
            "the tree at {} holds no entries. An empty directory is an empty tree; \
             `syllabus lesson-add .` or `syllabus module-add .` puts something in it.",
            root.display()
        )
    } else if filtered {
        "no entry matched. The tree holds entries and the filters excluded all of \
         them; widen or drop `--under`, `--status` and `--label`. An OVERVIEW carries \
         no parts, so it matches no filter."
            .to_string()
    } else {
        // Unreachable while a non-empty tree with no filters lists everything;
        // stated rather than asserted, for the reason `target_of_name` gives.
        "no entry matched.".to_string()
    }
}

fn show(cli: &Cli, streams: &mut Streams<'_>, key: Key) -> Result<(), Failure> {
    let tree = fs::read::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    let entry = tree
        .by_key(key)
        .ok_or_else(|| Failure::refused(&Refusal::TargetMissing { key }))?;
    streams.records(&[record_of(&cli.root, &entry)])
}

fn ancestors(cli: &Cli, streams: &mut Streams<'_>, key: Key) -> Result<(), Failure> {
    let tree = fs::read::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    let entry = tree
        .by_key(key)
        .ok_or_else(|| Failure::refused(&Refusal::TargetMissing { key }))?;
    let records: Vec<Record> = entry
        .ancestors()
        .iter()
        .map(|level| Record {
            target: target_of_level(level),
            path: level_path(&cli.root, level),
        })
        .collect();
    streams.records(&records)
}

fn overview_chain(cli: &Cli, streams: &mut Streams<'_>, key: Key) -> Result<(), Failure> {
    let tree = fs::read::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    let entry = tree
        .by_key(key)
        .ok_or_else(|| Failure::refused(&Refusal::TargetMissing { key }))?;
    let records: Vec<Record> = entry
        .distinguished_chain()
        .iter()
        .map(|overview| record_of(&cli.root, overview))
        .collect();
    if records.is_empty() {
        streams.note(
            "no level containing that entry holds an OVERVIEW.md. A chain walks the \
             entry's ancestors, so a module's own OVERVIEW is never in its own chain.",
        )?;
    }
    streams.records(&records)
}

````
<!-- /fragment -->

The mutation verbs and their effects are:

| Verb | Library operation | Idempotent | stdout subject |
|---|---|---|---|
| `lesson-add PARENT LABEL… [--status STATUS]` | `append_many` | No | Created lessons |
| `module-add PARENT LABEL…` | `append_many` | No | Created modules |
| `lesson-insert PARENT AT LABEL [--status STATUS]` | `insert` | No | Created lesson |
| `module-insert PARENT AT LABEL` | `insert` | No | Created module |
| `promote KEY LABEL [--first-lesson LABEL]` | `promote` | No | Created module, then optional lesson |
| `relabel KEY LABEL` | `rewrite` | Yes | Renamed entry |
| `publish KEY` | `rewrite` | Yes | Renamed or unchanged lesson |
| `unpublish KEY` | `rewrite` | Yes | Renamed or unchanged lesson |

`relabel`, `publish`, and `unpublish` read existing parts from the same write
guard they consume. Relabel preserves the existing lesson/module variant;
status changes accept only lessons. This makes species-changing rewrites
unreachable from these verbs. Rewriting to equal parts is a successful no-op,
so the three rewrite-based commands remain idempotent and still report their
subject. Add, insert, and promote allocate or change structure and are not
idempotent.

The mutation-helper range is present because the consumer owns construction of
domain parts around public mutation calls, one exclusive guard plus its
snapshot becomes one `Report` or categorized `Failure`, same-guard inspection
preserves the rewrite species invariant, and the local `insert` function is the
worked operation's final consumer-to-library handoff.

<!-- fragment «cli-mutations» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="1137-1259" parent="syllabus-cli-source" -->
````rust
// ---------------------------------------------------------------------------
// Mutating
// ---------------------------------------------------------------------------

/// stdout is written only after the library mutation succeeds. Library mutation
/// failure is rolled back before this point; a terminal failure happens later
/// and cannot promise rollback.
fn report_out(
    streams: &mut Streams<'_>,
    verb: &str,
    report: &Report<SyllabusName>,
) -> Result<(), Failure> {
    streams.records(&mutation_records(report))?;
    streams.trace(verb, report)
}

fn add(
    cli: &Cli,
    streams: &mut Streams<'_>,
    verb: &str,
    parent: Target,
    parts: Vec<Parts>,
) -> Result<(), Failure> {
    let tree = fs::write::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    // Both add verbs are variadic and both call `append_many`, including for a
    // single label: the library defines `append` as one `append_many` of one
    // entry, so a CLI branch on the count would be the same arithmetic spelled
    // twice. What this exercises instead is the property only a run has —
    // either the whole run lands or none of it does.
    let entries = parts.into_iter().map(NewEntry::empty).collect();
    let report = tree
        .append_many(parent, entries)
        .map_err(|e| Failure::library(&e))?;
    report_out(streams, verb, &report)
}

fn insert(
    cli: &Cli,
    streams: &mut Streams<'_>,
    verb: &str,
    parent: Target,
    at: Ordinal,
    parts: Parts,
) -> Result<(), Failure> {
    let tree = fs::write::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    let report = tree
        .insert(parent, at, NewEntry::empty(parts))
        .map_err(|e| Failure::library(&e))?;
    report_out(streams, verb, &report)
}

fn promote(
    cli: &Cli,
    streams: &mut Streams<'_>,
    key: Key,
    label: Label,
    first_lesson: Option<&Label>,
) -> Result<(), Failure> {
    let tree = fs::write::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    // `--first-lesson` starts as a draft, and there is no flag for its status: a
    // lesson that starts published is one `publish` away, and a second flag
    // would be a second place the default lives.
    let first =
        first_lesson.map(|label| NewEntry::empty(Parts::lesson(Status::Draft, label.clone())));
    let report = tree
        .promote(key, Parts::module(label), first)
        .map_err(|e| Failure::library(&e))?;
    report_out(streams, "promote", &report)
}

fn relabel(cli: &Cli, streams: &mut Streams<'_>, key: Key, label: Label) -> Result<(), Failure> {
    let tree = fs::write::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    // Read then mutate on the *same* guard: one lock, one snapshot. A relabel
    // keeps the variant it read, which is why `RewriteSpeciesChange` is
    // unreachable from this verb.
    let parts = match parts_of(&tree, key)? {
        Parts::Lesson { status, .. } => Parts::lesson(status, label),
        Parts::Module { .. } => Parts::module(label),
    };
    let report = tree.rewrite(key, parts).map_err(|e| Failure::library(&e))?;
    report_out(streams, "relabel", &report)
}

fn set_status(
    cli: &Cli,
    streams: &mut Streams<'_>,
    verb: &str,
    key: Key,
    status: Status,
) -> Result<(), Failure> {
    let tree = fs::write::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    let parts = match parts_of(&tree, key)? {
        Parts::Lesson { label, .. } => Parts::lesson(status, label),
        // Refused by the CLI rather than by the library: modules carry no
        // publication status here, so there are no parts to compose and nothing
        // ever reaches `rewrite`. This is the CLI's own message.
        Parts::Module { label } => {
            return Err(Failure::own(format!(
                "the entry with key {key} is the module `{label}`, and a module carries \
                 no publication status in this syllabus — publication is a property of \
                 a lesson. If you meant to change its name, use \
                 `syllabus relabel {key} <label>`."
            )))
        }
    };
    let report = tree.rewrite(key, parts).map_err(|e| Failure::library(&e))?;
    report_out(streams, verb, &report)
}

/// The parts the entry with this key already carries.
///
/// Cloned out of the snapshot before the guard is consumed, which is what a
/// mutation derived from an entry's current parts needs: the guard's reading
/// surface borrows it, and every mutation takes it by value.
fn parts_of(tree: &fs::WriteGuard<SyllabusName>, key: Key) -> Result<Parts, Failure> {
    tree.by_key(key)
        .and_then(|entry| entry.triple())
        .map(|triple| triple.parts.clone())
        // An entry with a key always has a triple, so the `and_then` above
        // narrows nothing; both `None`s are the same condition.
        .ok_or_else(|| Failure::refused(&Refusal::TargetMissing { key }))
}

````
<!-- /fragment -->

<a id="parsing-and-failures"></a>
## Parsing, refusals, and exit categories

Custom parsers produce `Key`, `Target`, `Ordinal`, `Label`, and `Status`
directly, so a malformed operator value remains clap usage failure 2. Labels
and statuses are parsed by reference-domain constructors rather than by a
second grammar in the CLI. `Failure` pairs the text shown to the operator with
the numeric category returned to a calling process.

Library errors are rendered with `Display` verbatim. The binary adds only the
`syllabus: ` prefix in `main`; returning `Result` from `main` would use the
library error's diagnostic `Debug` representation and discard its recovery
advice. Read helpers construct `Refusal::TargetMissing` when an `Option` is
empty. Changing a module's publication status is the one consumer refusal: the
consumer has no module status parts to pass to `rewrite`, so both `publish` and
`unpublish` report that domain condition before the library call.

| Exit | Category | Recovery meaning |
|---|---|---|
| `0` | Success, including empty reads and idempotent rewrites | No recovery |
| `1` | I/O or no containing directory | Fix path or permissions |
| `2` | Argument parsing or usage | Fix argv |
| `3` | `TargetMissing` | List and choose an existing key |
| `4` | Every other refusal, including the CLI's module-status refusal | Follow the stated remedy; no effect landed |
| `5` | Malformed, reserved, non-UTF-8, or multi-component name | Repair the filename or domain rendering |
| `6` | Forward failure with complete unwind | The tree was restored; retry is safe |
| `7` | Forward failure with incomplete unwind | Repair from the reported intermediate state; do not retry |

The parsing and failure range is present because the binary owns usage
validation and process categories, strings become domain/library values or an
operator-facing `Failure`, the mapping preserves the library's distinctions
without rewording them, and it determines both the pre-dispatch and error exits
of the worked insert.

<!-- fragment «cli-parsing-and-failure» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="489-632" parent="syllabus-cli-source" -->
````rust
// ---------------------------------------------------------------------------
// Argument parsing: the four things argv carries
// ---------------------------------------------------------------------------

/// Every parse failure here is exit 2, because clap reports a bad value that
/// way and a hand-written usage error that exits differently is a second
/// scheme.
fn parse_key(raw: &str) -> Result<Key, String> {
    raw.parse::<u32>().map(Key::new).map_err(|_| {
        format!(
            "`{raw}` is not a key. A key is the decimal a name carries after `i` — \
             `03-draft-limits-i9.md` has key 9. Run `syllabus list` to see the keys \
             this tree holds."
        )
    })
}

fn parse_target(raw: &str) -> Result<Target, String> {
    if raw == "." {
        return Ok(Target::Root);
    }
    raw.parse::<u32>()
        .map(|key| Target::Key(Key::new(key)))
        .map_err(|_| {
            format!(
                "`{raw}` names no level. A level is a module's key — the decimal after \
                 `i` — or `.` for the tree root. Run `syllabus list` to see the keys \
                 this tree holds."
            )
        })
}

fn parse_ordinal(raw: &str) -> Result<Ordinal, String> {
    raw.parse::<u32>().map(Ordinal::new).map_err(|_| {
        format!(
            "`{raw}` is not an ordinal. An ordinal is the decimal a name starts with — \
             `03-draft-limits-i9.md` sits at ordinal 3 — and it is the position within \
             a level, not the key. Run `syllabus list` to see the level."
        )
    })
}

fn parse_label(raw: &str) -> Result<Label, String> {
    Label::new(raw).map_err(|error| {
        format!(
            "`{raw}` is not a label: {error}. A label holds lowercase ASCII letters, \
             digits and interior hyphens, and starts with a letter — `linear-algebra`, \
             `vectors`, `week-1`."
        )
    })
}

fn parse_status(raw: &str) -> Result<Status, String> {
    // `Status::from_token` rather than a mapping written a second time here.
    // That a domain can render a token it cannot read back is the failure this
    // avoids, and it is why the inverse is public — see `CLI.md`'s watch list.
    Status::from_token(raw).ok_or_else(|| {
        format!(
            "`{raw}` is not a status. A lesson is `{}` or `{}`, and a module has no \
             status at all.",
            Status::Draft.token(),
            Status::Published.token()
        )
    })
}

// ---------------------------------------------------------------------------
// Failure: a message and an exit code
// ---------------------------------------------------------------------------

/// What the operator is told, and what the caller branches on.
struct Failure {
    code: u8,
    message: String,
}

impl Failure {
    /// A library error, rendered **verbatim**.
    ///
    /// `Error`'s own `Display` deliberately refuses to put a second sentence in
    /// front of the domain's advice, because that pushes the actionable half off
    /// the end of a terminal line — a CLI that adds one has undone the decision.
    /// The `syllabus: ` prefix is the whole of what this adds.
    fn library(error: &Error<SyllabusName>) -> Self {
        Self {
            code: exit_code(error),
            message: error.to_string(),
        }
    }

    /// A refusal this CLI constructs rather than receives.
    ///
    /// `by_key` answers with an `Option`, so the read verbs are handed no
    /// refusal at all. They build one instead of wording the condition a second
    /// time: `Refusal` is a public enum with public fields and its message is
    /// already right. `docs/formalism-findings.md` entry 017 is where a second
    /// wording of one condition was measured going wrong.
    fn refused(refusal: &Refusal) -> Self {
        Self {
            code: refusal_code(refusal),
            message: refusal.to_string(),
        }
    }

    /// This CLI's own refusal, for a condition that never reaches the library.
    fn own(message: String) -> Self {
        Self { code: 4, message }
    }

    /// A terminal refused one of the process's output streams.
    fn stream(stream: &str, error: io::Error) -> Self {
        Self {
            code: 1,
            message: format!("writing {stream} failed: {error}"),
        }
    }
}

/// The library's outcome taxonomy, read as *what should the caller do next*.
fn exit_code(error: &Error<SyllabusName>) -> u8 {
    match error {
        Error::Io { .. } | Error::NoContainingDirectory { .. } => 1,
        Error::Refused(refusal) => refusal_code(refusal),
        // A human fixes a filename; no retry helps.
        Error::Malformed { .. }
        | Error::Reserved { .. }
        | Error::NonUtf8Name { .. }
        | Error::NameIsNotOneComponent { .. } => 5,
        // The single most valuable distinction the library offers, and a generic
        // `1` would throw it away.
        Error::Failed { .. } => 6,
        Error::FailedPartiallyRolledBack { .. } => 7,
    }
}

/// `TargetMissing` is its own code because it is the one refusal whose remedy is
/// *look the key up again*, which a caller can act on without reading prose.
fn refusal_code(refusal: &Refusal) -> u8 {
    match refusal {
        Refusal::TargetMissing { .. } => 3,
        _ => 4,
    }
}

````
<!-- /fragment -->

<a id="streams-and-records"></a>
## Records, advisories, and paths

Stdout is result data in the form `<target>` TAB `<encoded-path>` followed by a
newline. The target is the key that can drive a later command. A distinguished
child has no key, so its record uses the key of its containing level, or `.` for
the root. Consumers split on the first tab and percent-decode the second column.
Printable ASCII bytes other than `%` remain literal; controls, `%`, and every
non-ASCII byte in `OsStr::as_encoded_bytes()` become uppercase `%HH`. The result
is one UTF-8 physical line for tabs, newlines, and non-UTF-8 Unix path bytes.
The decoded bytes reconstruct the path on the same Rust version and target,
which is the standard library's domain for this opaque platform encoding.

`Streams` owns injected stdout and stderr writers behind a private seam. Only a
stdout `BrokenPipe` is benign, so `list | head -1` remains quiet and exits 0.
Every other stdout write or flush refusal becomes exit 1. Every stderr refusal
also becomes exit 1, including failure to report a domain failure. A mutation
has already landed before terminal output begins, so terminal exit 1 does not
promise rollback; the operator inspects the tree before retrying a
non-idempotent verb.

Clap's help, version, and usage text also passes through `Streams`: Clap renders
and classifies it, then the seam checks the selected writer and its flush. That
keeps a help pipe's stdout `BrokenPipe` benign without swallowing any other
argument-parser stream failure.

Stderr contains advisories and failures. `--quiet` suppresses landing traces
and empty-result notes but never suppresses a reportable failure. Read paths are
constructed from the root spelling, ancestor module names, and the entry name;
every name admitted to the snapshot has already passed one-component
validation, so this consumer-side join remains inside the tree.

The streams-and-paths range is present because the consumer owns terminal I/O
and reconstruction of paths absent from algebraic entries, snapshots and
reports become stable records or optional advice, validated name components
preserve tree confinement, and this range produces the stdout and stderr values
observed in the worked insert.

<!-- fragment «cli-streams-and-paths» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="633-859" parent="syllabus-cli-source" -->
````rust
// ---------------------------------------------------------------------------
// The two streams
// ---------------------------------------------------------------------------

/// stdout is data, stderr is advice, and `--quiet` silences the second.
struct Streams<'a> {
    quiet: bool,
    stdout: &'a mut dyn Write,
    stderr: &'a mut dyn Write,
}

/// One line of stdout: the target you would pass to another verb, and the path.
struct Record {
    target: String,
    path: PathBuf,
}

impl<'a> Streams<'a> {
    fn new(stdout: &'a mut dyn Write, stderr: &'a mut dyn Write, quiet: bool) -> Self {
        Self {
            quiet,
            stdout,
            stderr,
        }
    }

    /// Write the answer.
    ///
    /// A closed pipe ends the run quietly: `syllabus list | head -1` closes the
    /// pipe under us, and reporting that as the tree's problem would be noise.
    /// Every other write or flush failure is an environmental failure.
    fn records(&mut self, records: &[Record]) -> Result<(), Failure> {
        for record in records {
            if let Err(error) = writeln!(
                self.stdout,
                "{}\t{}",
                record.target,
                encode_path(&record.path)
            ) {
                return Self::stdout_result(error);
            }
        }
        match self.stdout.flush() {
            Ok(()) => Ok(()),
            Err(error) => Self::stdout_result(error),
        }
    }

    fn stdout_result(error: io::Error) -> Result<(), Failure> {
        if error.kind() == io::ErrorKind::BrokenPipe {
            Ok(())
        } else {
            Err(Failure::stream("stdout", error))
        }
    }

    fn write_stdout(&mut self, bytes: &[u8]) -> Result<(), Failure> {
        if let Err(error) = self.stdout.write_all(bytes) {
            return Self::stdout_result(error);
        }
        match self.stdout.flush() {
            Ok(()) => Ok(()),
            Err(error) => Self::stdout_result(error),
        }
    }

    fn write_stderr(&mut self, bytes: &[u8]) -> Result<(), Failure> {
        self.stderr
            .write_all(bytes)
            .map_err(|error| Failure::stream("stderr", error))?;
        self.stderr
            .flush()
            .map_err(|error| Failure::stream("stderr", error))
    }

    /// Render clap's terminal output through the same checked stream boundary
    /// as command records and diagnostics.
    fn clap(&mut self, error: clap::Error) -> u8 {
        let code = u8::try_from(error.exit_code()).unwrap_or(1);
        let rendered = error.render().to_string();
        // clap chooses the stream by error kind; rendering separately lets this
        // CLI retain every write and flush result instead of `Error::exit()`
        // discarding print failures:
        // https://docs.rs/clap/4.6.1/clap/error/struct.Error.html#method.render
        let outcome = if error.use_stderr() {
            self.write_stderr(rendered.as_bytes())
        } else {
            self.write_stdout(rendered.as_bytes())
        };
        match outcome {
            Ok(()) => code,
            Err(failure) => settle(Err(failure), self),
        }
    }

    fn note(&mut self, message: &str) -> Result<(), Failure> {
        if self.quiet {
            return Ok(());
        }
        writeln!(self.stderr, "{message}").map_err(|error| Failure::stream("stderr", error))?;
        self.stderr
            .flush()
            .map_err(|error| Failure::stream("stderr", error))
    }

    /// The landing trace: what the plan did, **in the order it landed**.
    ///
    /// `Report::paths()` is the plan's own order — which for a mixed plan is
    /// neither species' — and it yields destinations only, so the labels are
    /// reconstructed by matching each path against the renames' destinations.
    /// That is sound because a plan claims every destination exclusively, so no
    /// two effects in one plan land on one path. It is also a correlation the
    /// report could have made unnecessary; `CLI.md`'s watch list carries it.
    ///
    /// This is where the highest-ordinal-first shift rule stays observable to an
    /// operator, which is why it is a property of a *value* rather than of a
    /// loop's direction.
    fn trace(&mut self, verb: &str, report: &Report<SyllabusName>) -> Result<(), Failure> {
        if self.quiet {
            return Ok(());
        }
        let count = report.paths().count();
        writeln!(
            self.stderr,
            "{verb}: {count} effect{}, in the order they landed:",
            if count == 1 { "" } else { "s" }
        )
        .map_err(|error| Failure::stream("stderr", error))?;
        for path in report.paths() {
            match report.renamed().iter().find(|renamed| renamed.to == path) {
                Some(renamed) => writeln!(
                    self.stderr,
                    "  renamed  {} -> {}",
                    renamed.from.display(),
                    renamed.to.display()
                )
                .map_err(|error| Failure::stream("stderr", error))?,
                None => writeln!(self.stderr, "  created  {}", path.display())
                    .map_err(|error| Failure::stream("stderr", error))?,
            }
        }
        self.stderr
            .flush()
            .map_err(|error| Failure::stream("stderr", error))
    }

    fn failure(&mut self, failure: &Failure) -> io::Result<()> {
        writeln!(self.stderr, "syllabus: {}", failure.message)?;
        self.stderr.flush()
    }
}

/// A path column that occupies one UTF-8 physical line and round-trips every
/// platform path representable by this Rust build.
fn encode_path(path: &Path) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    let mut encoded = String::new();
    // This encoding is opaque outside the same Rust version and target. That is
    // the exact reconstruction domain promised by the standard library:
    // https://doc.rust-lang.org/1.85.0/std/ffi/struct.OsStr.html#method.as_encoded_bytes
    for &byte in path.as_os_str().as_encoded_bytes() {
        if (b' '..=b'~').contains(&byte) && byte != b'%' {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
    encoded
}

// ---------------------------------------------------------------------------
// Paths, which the reading surface does not carry
// ---------------------------------------------------------------------------

/// Where an entry lives, in the caller's own spelling of the root.
///
/// The library's reading surface returns no paths — `Entry` has `name()`,
/// `depth()` and `ancestors()`, and only `Report` carries paths, built by the
/// filesystem layer. So the CLI builds them, in **one** place: the root as
/// spelled, plus each ancestor module's rendered name, plus the entry's own.
/// Every name a snapshot admits has already been checked to render as one path
/// component, so the join can only address inside the tree.
///
/// Do not answer this by adding a path to the algebra; whether the library
/// should offer one is a library question and `CLI.md`'s watch list carries it.
fn path_of(root: &Path, entry: &Entry<'_, SyllabusName>) -> PathBuf {
    let mut path = level_path(root, &entry.container());
    path.push(entry.name().to_string());
    path
}

/// Where a level lives: the root itself, or the module whose level it is.
fn level_path(root: &Path, level: &Container<'_, SyllabusName>) -> PathBuf {
    match level.entry() {
        None => root.to_path_buf(),
        Some(node) => path_of(root, &node),
    }
}

/// The target column for an entry: its own key, or — for an OVERVIEW, which
/// carries no key and which no operation can name — the level whose content it
/// is. That is the handle a caller reading `overview-chain` or `list` needs.
fn target_of(entry: &Entry<'_, SyllabusName>) -> String {
    match entry.key() {
        Some(key) => key.to_string(),
        None => target_of_level(&entry.container()),
    }
}

/// The target column for a level: a module's key, or `.` for the tree root.
fn target_of_level(level: &Container<'_, SyllabusName>) -> String {
    match level.entry().and_then(|node| node.key()) {
        Some(key) => key.to_string(),
        None => ".".to_string(),
    }
}

fn record_of(root: &Path, entry: &Entry<'_, SyllabusName>) -> Record {
    Record {
        target: target_of(entry),
        path: path_of(root, entry),
    }
}

````
<!-- /fragment -->

Mutation stdout is selected mechanically: created entries when any exist,
otherwise rename destinations. Shifted siblings are consequences rather than
the command subject and remain in the advisory landing trace. The trace follows
`Report::paths()` order and recovers rename origins by matching destinations
against `Report::renamed()`; guarded plans make destinations exclusive within
one report.

The mutation-output range is present because the consumer owns the distinction
between a command subject and its incidental moves, a `Report` becomes reusable
stdout records plus an ordered trace, exclusive destination claims make the
correlation unambiguous, and `lesson-insert` therefore prints key 7 while still
showing both highest-first shifts.

<!-- fragment «cli-mutation-output» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="860-904" parent="syllabus-cli-source" -->
````rust
// ---------------------------------------------------------------------------
// What a mutation prints
// ---------------------------------------------------------------------------

/// `created()` when it is non-empty, and otherwise `renamed()`'s destinations.
///
/// Mechanical rather than a per-verb judgement: every verb here either creates
/// something or is a pure rename, and the siblings a shift moves are the price
/// of the subject rather than the subject. `rewrite` produces exactly one
/// `MoveTo`, so the second branch yields exactly one line.
fn mutation_records(report: &Report<SyllabusName>) -> Vec<Record> {
    if report.created().is_empty() {
        report
            .renamed()
            .iter()
            .map(|renamed| Record {
                target: target_of_name(&renamed.name),
                path: renamed.to.clone(),
            })
            .collect()
    } else {
        report
            .created()
            .iter()
            .map(|created| Record {
                target: target_of_name(&created.name),
                path: created.path.clone(),
            })
            .collect()
    }
}

/// The target column for a name a mutation reports.
///
/// The `.` branch is unreachable through this CLI: an OVERVIEW reaches a report
/// only as `promote`'s rename, and `promote` creates, so the created branch is
/// what prints. It is written rather than panicked on because a total function
/// over a public enum is cheaper than a claim about which reports exist.
fn target_of_name(name: &SyllabusName) -> String {
    match name.triple() {
        Some(triple) => triple.key.to_string(),
        None => ".".to_string(),
    }
}

````
<!-- /fragment -->

<a id="omitted-features"></a>
## Deliberate omissions and retry limits

The in-file contract tests inject writers that fail on write or flush. Their
inputs include parser output, one record, or one stderr message; their outputs
are the settled exit code and categorized `Failure`. The tests distinguish a
closed stdout pipe from other stdout refusals for both help and records, cover
usage, stderr advice, and unreportable failures, and hold the invariant that all
terminal failures stay inside the documented taxonomy.
This range is included because those branches cannot be driven portably through
a real terminal without substituting the writers at the private seam.

<!-- fragment «cli-stream-contract-tests» owner="syllabus-cli-k17" source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="1260-1439" parent="syllabus-cli-source" -->
````rust
#[cfg(test)]
mod stream_contract_tests {
    use super::*;

    #[derive(Clone, Copy)]
    enum RefusalPoint {
        Write,
        Flush,
    }

    struct RefusingWriter {
        point: RefusalPoint,
        kind: io::ErrorKind,
    }

    impl Write for RefusingWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            match self.point {
                RefusalPoint::Write => Err(io::Error::new(self.kind, "controlled refusal")),
                RefusalPoint::Flush => Ok(bytes.len()),
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            match self.point {
                RefusalPoint::Write => Ok(()),
                RefusalPoint::Flush => Err(io::Error::new(self.kind, "controlled refusal")),
            }
        }
    }

    fn record() -> Record {
        Record {
            target: "1".to_string(),
            path: PathBuf::from("course/01-introduction-i1"),
        }
    }

    fn clap_error(arguments: &[&str]) -> clap::Error {
        match Cli::try_parse_from(arguments) {
            Ok(_) => panic!("arguments unexpectedly parsed"),
            Err(error) => error,
        }
    }

    #[cfg(unix)]
    #[test]
    fn path_encoding_round_trips_a_non_utf8_byte() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let path = PathBuf::from(OsString::from_vec(b"a\n%\xffcourse".to_vec()));
        let encoded = encode_path(&path);
        let mut decoded = Vec::new();
        let mut index = 0;
        while index < encoded.len() {
            if encoded.as_bytes()[index] == b'%' {
                decoded.push(
                    u8::from_str_radix(&encoded[index + 1..index + 3], 16)
                        .expect("an escape contains two hexadecimal digits"),
                );
                index += 3;
            } else {
                decoded.push(encoded.as_bytes()[index]);
                index += 1;
            }
        }

        assert_eq!(decoded, path.as_os_str().as_encoded_bytes());
    }

    #[test]
    fn help_stdout_broken_pipe_is_benign_during_write_and_flush() {
        for point in [RefusalPoint::Write, RefusalPoint::Flush] {
            let mut stdout = RefusingWriter {
                point,
                kind: io::ErrorKind::BrokenPipe,
            };
            let mut stderr = Vec::new();
            let mut streams = Streams::new(&mut stdout, &mut stderr, false);

            assert_eq!(streams.clap(clap_error(&["syllabus", "--help"])), 0);
        }
    }

    #[test]
    fn other_help_stdout_failures_are_environment_failures() {
        for point in [RefusalPoint::Write, RefusalPoint::Flush] {
            let mut stdout = RefusingWriter {
                point,
                kind: io::ErrorKind::Other,
            };
            let mut stderr = Vec::new();
            let mut streams = Streams::new(&mut stdout, &mut stderr, false);

            assert_eq!(streams.clap(clap_error(&["syllabus", "--help"])), 1);
            assert!(String::from_utf8(stderr)
                .expect("failure report is UTF-8")
                .contains("writing stdout failed"));
        }
    }

    #[test]
    fn usage_stderr_failures_are_environment_failures() {
        for point in [RefusalPoint::Write, RefusalPoint::Flush] {
            let mut stdout = Vec::new();
            let mut stderr = RefusingWriter {
                point,
                kind: io::ErrorKind::BrokenPipe,
            };
            let mut streams = Streams::new(&mut stdout, &mut stderr, false);

            assert_eq!(streams.clap(clap_error(&["syllabus", "remove"])), 1);
        }
    }

    #[test]
    fn stdout_broken_pipe_is_benign_during_write_and_flush() {
        for point in [RefusalPoint::Write, RefusalPoint::Flush] {
            let mut stdout = RefusingWriter {
                point,
                kind: io::ErrorKind::BrokenPipe,
            };
            let mut stderr = Vec::new();
            let mut streams = Streams::new(&mut stdout, &mut stderr, false);

            let outcome = streams.records(&[record()]);
            assert_eq!(settle(outcome, &mut streams), 0);
        }
    }

    #[test]
    fn other_stdout_failures_are_environment_failures() {
        for point in [RefusalPoint::Write, RefusalPoint::Flush] {
            let mut stdout = RefusingWriter {
                point,
                kind: io::ErrorKind::Other,
            };
            let mut stderr = Vec::new();
            let mut streams = Streams::new(&mut stdout, &mut stderr, false);

            let outcome = streams.records(&[record()]);
            let failure = outcome.expect_err("stdout refused");
            assert_eq!(failure.code, 1);
            assert!(failure.message.contains("stdout"));
            assert_eq!(settle(Err(failure), &mut streams), 1);
        }
    }

    #[test]
    fn stderr_advice_failures_are_environment_failures() {
        for point in [RefusalPoint::Write, RefusalPoint::Flush] {
            let mut stdout = Vec::new();
            let mut stderr = RefusingWriter {
                point,
                kind: io::ErrorKind::BrokenPipe,
            };
            let mut streams = Streams::new(&mut stdout, &mut stderr, false);

            let failure = streams.note("advice").expect_err("stderr refused");
            assert_eq!(failure.code, 1);
            assert!(failure.message.contains("stderr"));
        }
    }

    #[test]
    fn an_unreportable_failure_exits_as_an_environment_failure() {
        let mut stdout = Vec::new();
        let mut stderr = RefusingWriter {
            point: RefusalPoint::Write,
            kind: io::ErrorKind::BrokenPipe,
        };
        let mut streams = Streams::new(&mut stdout, &mut stderr, false);

        assert_eq!(
            settle(Err(Failure::own("refused".to_string())), &mut streams),
            1
        );
    }
}
````
<!-- /fragment -->

The binary has no removal command. Keys are allocated as tree-wide maximum plus
one, so deleting the maximum can reissue an identity still held by an external
reference. `unpublish` expresses retirement as an attribute rewrite. An empty
directory is already an empty tree, so initialization requires only `mkdir`.

There is no dry-run because a plan is internal and the public mutation surface
returns only a report. There are no lock options because lock scope and blocking
belong to the filesystem boundary rather than the consumer seam. There is no
label lookup in the library; `--label` is a predicate over a walk. There is no
version-control integration, migration, colour, pager, prompt, JSON mode, or
pagination. The demonstration has a bounded tree result, a tab-separated output
shape with a lossless same-platform path encoding and explicit terminal-failure
taxonomy, no destructive verb, and no second persistent representation that
would require those features.

Exit 6 is safe to retry because complete unwind restored the captured tree.
Exit 7 is not safe to retry because some effects remain. Termination by signal
has no exit-category guarantee: without a journal, the operator must inspect
the tree before deciding whether a non-idempotent add, insert, or promote can be
repeated. The advisory lock coordinates cooperating processes but does not hide
intermediate states from uncooperative writers or survive process termination.

[Previous: Filesystem interpreter](06-filesystem-interpreter.md) | [Contents](README.md)
