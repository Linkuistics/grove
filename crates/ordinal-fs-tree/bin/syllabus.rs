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
    fs, Container, Entry, EntryNameExt, Error, Key, NewEntry, Ordinal, Refusal, Report, Sought,
    Target,
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
    // Thirteen verbs, flat and hyphenated, so one `syllabus --help` enumerates
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
  tab — a `--root` you supplied may contain one. The key column is the target
  you would pass to another verb to name what that line is about, so output
  round-trips into the next call. `.` in the key column is the tree root.
  stderr is advisory: what else moved, and why a result was empty. `--quiet`
  suppresses it. Errors go to stderr and are never suppressed.
  A mutation prints what it created; a mutation that created nothing prints
  what it renamed. The siblings an insert shifts are on stderr, not stdout.

EXIT CODES
  0  success
  1  the environment refused (I/O, or a root with no containing directory) —
     fix the path or the permissions
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
  `init` creates the tree, and it is the only verb that does — every other one
  refuses a root holding no tree rather than creating one on the way past. It
  takes the lock before deciding the root is empty and still holds it while
  creating it, so two racing `init`s cannot both succeed.
  There is no `--dry-run`: a plan is internal by design. There are
  no lock flags: every verb takes an advisory lock on the directory containing
  the root and BLOCKS until the tree is free — a hang is a lock and not a bug.
  Nothing is staged in version control; a rename is rename(2).

EXAMPLES
  syllabus --root syllabus init --overview 'An introduction.'
  syllabus --root syllabus module-add . linear-algebra
  syllabus --root syllabus lesson-add 1 vectors matrices
  syllabus --root syllabus list
  syllabus --root syllabus publish 2

SEE ALSO
  `syllabus <verb> --help` for any verb below.";

#[derive(Subcommand)]
enum Verb {
    /// Create the tree: the root directory, its OVERVIEW, and any first lessons.
    ///
    /// The one verb that runs against a root holding no tree, and the only one
    /// that creates one. It takes the exclusive lock *before* deciding the tree
    /// is absent and still holds it while creating it, so two `init`s racing on
    /// one root cannot both succeed — the loser finds a tree and is refused.
    ///
    /// NOT idempotent, and deliberately not: a second `init` is refused rather
    /// than being a no-op, because the call that thinks it is creating a course
    /// and the call that finds one already there want different answers.
    #[command(after_help = "\
EXAMPLES
  syllabus --root course init
  syllabus --root course init --overview 'An introduction.'
  syllabus --root course init orientation prerequisites

SEE ALSO
  lesson-add, module-add")]
    Init {
        /// The course's own content, written into the root's OVERVIEW.md.
        ///
        /// Omit it and the root has no OVERVIEW at all, which is a different
        /// tree from one whose OVERVIEW is empty — pass `--overview ''` for
        /// that.
        #[arg(long, value_name = "TEXT")]
        overview: Option<String>,

        /// Labels for the first lessons, in the order they should appear.
        #[arg(num_args = 0.., value_name = "LABEL", value_parser = parse_label)]
        labels: Vec<Label>,

        /// The status every lesson in this run starts at.
        #[arg(long, value_name = "STATUS", default_value = "draft", value_parser = parse_status)]
        status: Status,
    },

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
    /// A search answers with a `Sought`, which is deliberately *not* a refusal —
    /// nothing was asked to change — so the read verbs are handed no refusal at
    /// all. They build one instead of wording the condition a second time:
    /// `Refusal` is a public enum with public fields and its message is already
    /// right. `docs/formalism-findings.md` entry 017 is where a second wording of
    /// one condition was measured going wrong.
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
}

/// The library's outcome taxonomy, read as *what should the caller do next*.
fn exit_code(error: &Error<SyllabusName>) -> u8 {
    match error {
        Error::Io { .. } | Error::NoContainingDirectory { .. } => 1,
        // A human moves whatever is sitting on the root out of the way; no
        // retry helps, and this library will not clear it away.
        Error::RootIsNotATree { .. } => 5,
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

// ---------------------------------------------------------------------------
// The two streams
// ---------------------------------------------------------------------------

/// stdout is data, stderr is advice, and `--quiet` silences the second.
struct Streams {
    quiet: bool,
}

/// One line of stdout: the target you would pass to another verb, and the path.
struct Record {
    target: String,
    path: PathBuf,
}

impl Streams {
    /// Write the answer.
    ///
    /// Locked once and written in one pass, and a write failure ends the run
    /// quietly: `syllabus list | head -1` closes the pipe under us, and a panic
    /// there would be this tool's own noise reported as the tree's problem.
    fn records(&self, records: &[Record]) {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        for record in records {
            if writeln!(out, "{}\t{}", record.target, record.path.display()).is_err() {
                return;
            }
        }
        let _ = out.flush();
    }

    fn note(&self, message: &str) {
        if !self.quiet {
            eprintln!("{message}");
        }
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
    fn trace(&self, verb: &str, report: &Report<SyllabusName>) {
        if self.quiet {
            return;
        }
        let count = report.paths().count();
        eprintln!(
            "{verb}: {count} effect{}, in the order they landed:",
            if count == 1 { "" } else { "s" }
        );
        for path in report.paths() {
            match report.renamed().iter().find(|renamed| renamed.to == path) {
                Some(renamed) => eprintln!(
                    "  renamed  {} -> {}",
                    renamed.from.display(),
                    renamed.to.display()
                ),
                None => eprintln!("  created  {}", path.display()),
            }
        }
    }
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

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let cli = Cli::parse();
    let streams = Streams { quiet: cli.quiet };
    if let Err(failure) = run(&cli, &streams) {
        eprintln!("syllabus: {}", failure.message);
        std::process::exit(i32::from(failure.code));
    }
}

fn run(cli: &Cli, streams: &Streams) -> Result<(), Failure> {
    match &cli.verb {
        Verb::Init {
            overview,
            labels,
            status,
        } => init(cli, streams, overview.as_deref(), labels, *status),
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

// ---------------------------------------------------------------------------
// Opening: a tree, or the advice that there is none
// ---------------------------------------------------------------------------

/// The tree under a shared lock, or this CLI's refusal to invent one.
///
/// The library answers *is there a tree here* with a shape rather than a
/// predicate, so every verb below meets the vacancy in the same place and says
/// the same thing about it. Which is: nothing here creates a tree except `init`.
/// A read verb that silently printed an empty listing would be reporting a tree
/// that does not exist, and a mutation that created the tree on the way past
/// would make `mkdir` and a typo indistinguishable.
fn reading(cli: &Cli) -> Result<fs::ReadGuard<SyllabusName>, Failure> {
    match fs::read::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))? {
        fs::Reading::Tree(tree) => Ok(tree),
        fs::Reading::Vacant => Err(Failure::own(no_tree(&cli.root))),
    }
}

/// The tree under an exclusive lock, or the same refusal.
fn writing(cli: &Cli) -> Result<fs::WriteGuard<SyllabusName>, Failure> {
    match fs::write::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))? {
        fs::Writing::Tree(tree) => Ok(tree),
        fs::Writing::Vacancy(_) => Err(Failure::own(no_tree(&cli.root))),
    }
}

fn no_tree(root: &Path) -> String {
    format!(
        "there is no tree at {}. `syllabus --root {} init` creates one; no other \
         verb does, because creating a syllabus is a decision and not a fallback \
         for a mistyped --root.",
        root.display(),
        root.display()
    )
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

fn list(
    cli: &Cli,
    streams: &Streams,
    under: Option<Key>,
    status: Option<Status>,
    label: Option<&Label>,
    first: bool,
) -> Result<(), Failure> {
    let tree = reading(cli)?;

    // A `--under` naming nothing is the same condition `show` meets, and gets
    // the same refusal rather than an empty listing that looks like an answer.
    if let Some(key) = under {
        if tree.by_key(key).is_nothing() {
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

    // `--first` decides whether the predicate goes to `seek`, which
    // short-circuits, or filters a full `walk`. That is the architecture's *a
    // predicate passed to `seek` answers them without the library ever learning
    // what it asked*, spelled as a flag.
    //
    // A search matching nothing is not a refusal here either: `--first` over a
    // tree where nothing matches is the same empty listing as a full `walk` where
    // nothing matches, and gets the same note below.
    let records: Vec<Record> = if first {
        tree.seek(matches)
            .into_option()
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
        streams.note(&empty_note(tree.snapshot().is_empty(), filtered, &cli.root));
    }
    streams.records(&records);
    Ok(())
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

fn show(cli: &Cli, streams: &Streams, key: Key) -> Result<(), Failure> {
    let tree = reading(cli)?;
    let Sought::Match(entry) = tree.by_key(key) else {
        return Err(Failure::refused(&Refusal::TargetMissing { key }));
    };
    streams.records(&[record_of(&cli.root, &entry)]);
    Ok(())
}

fn ancestors(cli: &Cli, streams: &Streams, key: Key) -> Result<(), Failure> {
    let tree = reading(cli)?;
    let Sought::Match(entry) = tree.by_key(key) else {
        return Err(Failure::refused(&Refusal::TargetMissing { key }));
    };
    let records: Vec<Record> = entry
        .ancestors()
        .iter()
        .map(|level| Record {
            target: target_of_level(level),
            path: level_path(&cli.root, level),
        })
        .collect();
    streams.records(&records);
    Ok(())
}

fn overview_chain(cli: &Cli, streams: &Streams, key: Key) -> Result<(), Failure> {
    let tree = reading(cli)?;
    let Sought::Match(entry) = tree.by_key(key) else {
        return Err(Failure::refused(&Refusal::TargetMissing { key }));
    };
    let records: Vec<Record> = entry
        .distinguished_chain()
        .iter()
        .map(|overview| record_of(&cli.root, overview))
        .collect();
    if records.is_empty() {
        streams.note(
            "no level containing that entry holds an OVERVIEW.md. A chain walks the \
             entry's ancestors, so a module's own OVERVIEW is never in its own chain.",
        );
    }
    streams.records(&records);
    Ok(())
}

// ---------------------------------------------------------------------------
// Mutating
// ---------------------------------------------------------------------------

/// stdout is written only after the mutation has succeeded. A run that fails is
/// rolled back, so paths printed as effects landed would describe files that are
/// no longer there.
fn report_out(streams: &Streams, verb: &str, report: &Report<SyllabusName>) {
    streams.records(&mutation_records(report));
    streams.trace(verb, report);
}

/// `init`: the tree, from nothing, under one lock.
///
/// The whole shape of the leaf, in eight lines: `write` answers with a
/// [`Writing`](fs::Writing), the vacancy arm *is* the permission to create, and
/// there is no moment between the two in which another writer fits. The tree arm
/// is refused here rather than by the library, because *there is already a tree*
/// is not a refusal the library states — it is a call the type system does not
/// let the library be asked.
fn init(
    cli: &Cli,
    streams: &Streams,
    overview: Option<&str>,
    labels: &[Label],
    status: Status,
) -> Result<(), Failure> {
    let opened = fs::write::<SyllabusName>(&cli.root).map_err(|e| Failure::library(&e))?;
    let fs::Writing::Vacancy(vacancy) = opened else {
        return Err(Failure::own(format!(
            "there is already a tree at {}, and `init` creates one. Add to it with \
             `lesson-add` or `module-add`, or name a root that holds no tree.",
            cli.root.display()
        )));
    };
    let entries = labels
        .iter()
        .map(|label| NewEntry::empty(Parts::lesson(status, label.clone())))
        .collect();
    let report = vacancy
        .initialize(overview.map(|text| text.as_bytes().to_vec()), entries)
        .map_err(|e| Failure::library(&e))?;
    report_out(streams, "init", &report);
    Ok(())
}

fn add(
    cli: &Cli,
    streams: &Streams,
    verb: &str,
    parent: Target,
    parts: Vec<Parts>,
) -> Result<(), Failure> {
    let tree = writing(cli)?;
    // Both add verbs are variadic and both call `append_many`, including for a
    // single label: the library defines `append` as one `append_many` of one
    // entry, so a CLI branch on the count would be the same arithmetic spelled
    // twice. What this exercises instead is the property only a run has —
    // either the whole run lands or none of it does.
    let entries = parts.into_iter().map(NewEntry::empty).collect();
    let report = tree
        .append_many(parent, entries)
        .map_err(|e| Failure::library(&e))?;
    report_out(streams, verb, &report);
    Ok(())
}

fn insert(
    cli: &Cli,
    streams: &Streams,
    verb: &str,
    parent: Target,
    at: Ordinal,
    parts: Parts,
) -> Result<(), Failure> {
    let tree = writing(cli)?;
    let report = tree
        .insert(parent, at, NewEntry::empty(parts))
        .map_err(|e| Failure::library(&e))?;
    report_out(streams, verb, &report);
    Ok(())
}

fn promote(
    cli: &Cli,
    streams: &Streams,
    key: Key,
    label: Label,
    first_lesson: Option<&Label>,
) -> Result<(), Failure> {
    let tree = writing(cli)?;
    // `--first-lesson` starts as a draft, and there is no flag for its status: a
    // lesson that starts published is one `publish` away, and a second flag
    // would be a second place the default lives.
    let first =
        first_lesson.map(|label| NewEntry::empty(Parts::lesson(Status::Draft, label.clone())));
    let report = tree
        .promote(key, Parts::module(label), first)
        .map_err(|e| Failure::library(&e))?;
    report_out(streams, "promote", &report);
    Ok(())
}

fn relabel(cli: &Cli, streams: &Streams, key: Key, label: Label) -> Result<(), Failure> {
    let tree = writing(cli)?;
    // Read then mutate on the *same* guard: one lock, one snapshot. A relabel
    // keeps the variant it read, which is why `RewriteSpeciesChange` is
    // unreachable from this verb.
    let parts = match parts_of(&tree, key)? {
        Parts::Lesson { status, .. } => Parts::lesson(status, label),
        Parts::Module { .. } => Parts::module(label),
    };
    let report = tree.rewrite(key, parts).map_err(|e| Failure::library(&e))?;
    report_out(streams, "relabel", &report);
    Ok(())
}

fn set_status(
    cli: &Cli,
    streams: &Streams,
    verb: &str,
    key: Key,
    status: Status,
) -> Result<(), Failure> {
    let tree = writing(cli)?;
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
    report_out(streams, verb, &report);
    Ok(())
}

/// The parts the entry with this key already carries.
///
/// Cloned out of the snapshot before the guard is consumed, which is what a
/// mutation derived from an entry's current parts needs: the guard's reading
/// surface borrows it, and every mutation takes it by value.
fn parts_of(tree: &fs::WriteGuard<SyllabusName>, key: Key) -> Result<Parts, Failure> {
    tree.by_key(key)
        .into_option()
        .and_then(|entry| entry.triple())
        .map(|triple| triple.parts.clone())
        // An entry with a key always has a triple, so the `and_then` above
        // narrows nothing; `Sought::Nothing` and a missing triple are the same
        // condition.
        .ok_or_else(|| Failure::refused(&Refusal::TargetMissing { key }))
}
