//! The run record store (`docs/specs/harness-selection-and-execution.md`,
//! *Records and later observations*).
//!
//! One SQLite file, `records.sqlite3`, in the owner's state directory, written
//! only by this front process: the worker never opens it. Every `run` commits
//! one handoff attempt here before it execs, in one short exclusive
//! transaction, and a failure to commit launches nothing (exit 4).
//!
//! **Durability.** A rollback journal with `synchronous = EXTRA` and
//! `fullfsync = ON`. SQLite documents FULL as "not necessarily durable across a
//! power loss in rollback mode, so if durability is desired, it is best to set
//! the synchronous mode to EXTRA" (https://www.sqlite.org/pragma.html#pragma_synchronous),
//! and `fullfsync`, off by default, is what reaches `F_FULLFSYNC` on macOS.
//! Neither reaches above the store's own directory, so a first use also syncs
//! the parent of each directory it creates, before the commit ([`create`]).
//! WAL is not used: it needs shared memory that network filesystems, where home
//! directories often live, do not provide. The journal mode is never set, so a
//! store this command creates keeps SQLite's default.
//!
//! **Locking.** One wait of at most [`LOCK_WAIT`]. The busy timeout replaces
//! rusqlite's own 5000 ms default (rusqlite 0.40.2,
//! `src/inner_connection.rs:118`), and a write transaction starts with
//! `BEGIN EXCLUSIVE`, which in rollback mode takes the only lock its commit
//! needs. The bundled build defines `HAVE_USLEEP` (libsqlite3-sys 0.38.2,
//! `build.rs`), so SQLite's busy handler sleeps in milliseconds, not whole
//! seconds. The commit opens the store only after the worker has been reaped,
//! so its wait neither extends nor consumes the selection bound. The one
//! thing that reads the store during evaluation is a policy's run lookup: a
//! short read transaction, holding a shared lock only while it reads, whose
//! wait is cut to what is left of the selection bound ([`load`]). No lock is
//! held across policy evaluation.
//!
//! **Versions and immutability.** The file names itself with an application ID
//! and a schema version, checked inside the transaction; a pristine file is
//! initialized in that same transaction, and anything else that does not match
//! refuses. A store is never reset or replaced. A run's launch fields are one
//! JSON document carrying its own schema version, with every field present,
//! `null` where the run has no value for it; triggers abort any update or
//! deletion of a committed row. So a release that records something new writes
//! it into new runs' documents, at a new document version if schema 1 has no
//! field for it, and a new table arrives by a migration that only creates. No
//! committed launch field is ever rewritten.
//!
//! **Observations.** Schema 2 adds one table of observations: later evidence an
//! observer attaches to a run, each an immutable validated document. A version-1
//! store is migrated only by `record observe`, which creates that table inside
//! its own exclusive transaction, so a refused import leaves the store at
//! version 1. Every other operation reads and writes both versions as they are.
//!
//! **Readable documents.** The schema version says which tables there are, not
//! what the documents in them say. So every read of a run checks what it reads
//! before anything is derived from it: the launch document and any
//! launch-failure detail ([`crate::record::readable`]), and each observation it
//! reads, by the import's own validation against its row
//! ([`crate::observation::stored`]). One this release cannot read refuses like a
//! store of another version, and never reads as a run with less in it.

use std::ffi::OsStr;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io;
use std::os::unix::fs::{DirBuilderExt as _, OpenOptionsExt as _};
use std::path::{Path, PathBuf};
use std::time::Duration;

use rusqlite::{params, Connection, ErrorCode, OpenFlags, OptionalExtension as _};
use rusqlite::{Transaction, TransactionBehavior};
use serde_json::{json, Value};

use crate::refusal::{Refusal, Stage, EXIT_RECORD};
use crate::run_id::RunId;
use crate::settings::{self, Settings};

/// The default state directory, relative to HOME. `XDG_STATE_HOME` is not
/// consulted: the owner settings and `--state-dir` are what name another.
pub const DEFAULT_STATE_DIR: &str = ".local/state/harness-dispatch";
pub const STORE_FILE: &str = "records.sqlite3";

/// The fixed record-store lock wait. It is separate from the selection bound.
pub const LOCK_WAIT: Duration = Duration::from_secs(2);

/// "HDRS", at offset 68 of the database header.
const APPLICATION_ID: i32 = 0x4844_5253;
const SCHEMA_VERSION: i32 = 2;

/// Schema 1's tables and triggers. [`prepare`] writes them, and
/// [`OBSERVATIONS`], together with the header's application ID and version, in
/// the transaction that found the file pristine, so a concurrent first use sees
/// either nothing or all of it.
const SCHEMA_1: &str = "
CREATE TABLE runs (
    run_id TEXT PRIMARY KEY NOT NULL,
    recorded_at TEXT NOT NULL,
    launch TEXT NOT NULL
);
CREATE TABLE launch_failures (
    run_id TEXT PRIMARY KEY NOT NULL REFERENCES runs (run_id),
    recorded_at TEXT NOT NULL,
    detail TEXT NOT NULL
);
CREATE TRIGGER runs_never_change BEFORE UPDATE ON runs
BEGIN SELECT RAISE(ABORT, 'a committed run''s launch fields never change'); END;
CREATE TRIGGER runs_are_never_removed BEFORE DELETE ON runs
BEGIN SELECT RAISE(ABORT, 'a committed run is never removed'); END;
CREATE TRIGGER launch_failures_never_change BEFORE UPDATE ON launch_failures
BEGIN SELECT RAISE(ABORT, 'a recorded launch failure never changes'); END;
CREATE TRIGGER launch_failures_are_never_removed BEFORE DELETE ON launch_failures
BEGIN SELECT RAISE(ABORT, 'a recorded launch failure is never removed'); END;
";

/// What schema 2 adds, and all the migration from version 1 does: it only
/// creates. `supersedes` is unique, so an observation is corrected at most
/// once and corrections form a chain; `record observe` checks that first, to
/// refuse by name, and the constraint backs it.
const OBSERVATIONS: &str = "
CREATE TABLE observations (
    observation_id TEXT PRIMARY KEY NOT NULL,
    run_id TEXT NOT NULL REFERENCES runs (run_id),
    recorded_at TEXT NOT NULL,
    supersedes TEXT UNIQUE REFERENCES observations (observation_id),
    document TEXT NOT NULL
);
CREATE INDEX observations_by_run ON observations (run_id);
CREATE TRIGGER observations_never_change BEFORE UPDATE ON observations
BEGIN SELECT RAISE(ABORT, 'a recorded observation never changes'); END;
CREATE TRIGGER observations_are_never_removed BEFORE DELETE ON observations
BEGIN SELECT RAISE(ABORT, 'a recorded observation is never removed'); END;
";

/// SQLite's clock, UTC to the millisecond, evaluated in the committing
/// statement.
const NOW: &str = "strftime('%Y-%m-%dT%H:%M:%fZ', 'now')";

/// Where this invocation's records live.
#[derive(Clone, Debug)]
pub struct StateDir {
    /// Absolute: `--state-dir` joined to the original cwd, the owner
    /// settings' directory, or the default under HOME. Kept as spelled;
    /// symlinks are not resolved.
    pub path: PathBuf,
    /// The input that chose it, or `None` for the default.
    pub set_by: Option<&'static str>,
}

impl StateDir {
    /// `--state-dir` replaces the owner's setting, which replaces the default.
    pub fn resolve(
        given: Option<&Path>,
        settings: &Settings,
        cwd: &Path,
        home: Option<&OsStr>,
    ) -> Result<StateDir, Refusal> {
        if let Some(given) = given {
            return Ok(StateDir {
                path: cwd.join(given),
                set_by: Some("--state-dir"),
            });
        }
        if let Some(setting) = &settings.state_dir {
            return Ok(StateDir {
                path: setting.clone(),
                set_by: Some(settings::ORIGIN),
            });
        }
        let remedy = "set HOME to your absolute home directory, or name the record directory \
                      with --state-dir PATH";
        let unset = |message: String| {
            Refusal::new("home_unset", Stage::Record, EXIT_RECORD, message, remedy).input("HOME")
        };
        let home = home.filter(|home| !home.is_empty()).ok_or_else(|| {
            unset(format!(
                "HOME is not set, so the default record directory ~/{DEFAULT_STATE_DIR} has no \
                 location"
            ))
        })?;
        let home = Path::new(home);
        if !home.is_absolute() {
            return Err(unset(format!(
                "HOME is the relative path {}, so the default record directory would depend on \
                 the current directory",
                home.display()
            )));
        }
        Ok(StateDir {
            path: home.join(DEFAULT_STATE_DIR),
            set_by: None,
        })
    }

    pub fn store(&self) -> PathBuf {
        self.path.join(STORE_FILE)
    }

    pub fn from(&self) -> &'static str {
        self.set_by.unwrap_or("default")
    }

    pub fn to_json(&self) -> Value {
        json!({ "path": self.path.to_string_lossy(), "from": self.from() })
    }

    pub fn to_text(&self) -> String {
        let from = self.set_by.unwrap_or("the default");
        format!("{} ({from})", self.path.display())
    }
}

/// What the commit wrote beside the launch document.
#[derive(Debug)]
pub struct Committed {
    pub recorded_at: String,
}

/// Commit one handoff attempt for `run_id`. The directory and file are created
/// on first use, privately. Nothing here is retried; a failure refuses.
pub fn commit(dir: &StateDir, run_id: &RunId, launch: &Value) -> Result<Committed, Refusal> {
    let file = dir.store();
    let fail = |failure: StoreFailure| {
        failure.refusal(dir, &file, "commit the run's handoff record", LOCK_WAIT)
    };
    let mut connection = create(dir, &file).map_err(fail)?;
    let transaction = exclusive(&mut connection).map_err(fail)?;
    let recorded_at = (|| {
        prepare(&transaction, Prepare::IfPristine)?;
        let recorded_at: String = transaction.query_row(
            &format!(
                "INSERT INTO runs (run_id, recorded_at, launch) VALUES (?1, {NOW}, ?2) \
                 RETURNING recorded_at"
            ),
            params![run_id.as_str(), launch.to_string()],
            |row| row.get(0),
        )?;
        transaction.commit()?;
        Ok(recorded_at)
    })()
    .map_err(fail)?;
    Ok(Committed { recorded_at })
}

/// Append a launch-failure detail to a committed attempt. The store must
/// already hold the run; nothing is created here.
pub fn append_launch_failure(
    dir: &StateDir,
    run_id: &RunId,
    detail: &Value,
) -> Result<(), Refusal> {
    let file = dir.store();
    let fail = |failure: StoreFailure| {
        failure.refusal(dir, &file, "append the run's launch failure", LOCK_WAIT)
    };
    let mut connection = open(&file, LOCK_WAIT).map_err(fail)?;
    let transaction = exclusive(&mut connection).map_err(fail)?;
    (|| {
        prepare(&transaction, Prepare::AsItIs)?;
        transaction.execute(
            &format!(
                "INSERT INTO launch_failures (run_id, recorded_at, detail) VALUES (?1, {NOW}, ?2)"
            ),
            params![run_id.as_str(), detail.to_string()],
        )?;
        transaction.commit()?;
        Ok(())
    })()
    .map_err(fail)
}

/// An observation to append. `document` is the validated envelope's compact
/// encoding, which is what a repeat is compared by.
pub struct NewObservation<'a> {
    pub observation_id: &'a str,
    pub run_id: &'a RunId,
    pub supersedes: Option<&'a str>,
    /// Whether it observes that the harness executed, which a run with a
    /// launch failure contradicts.
    pub confirms_execution: bool,
    pub document: &'a str,
}

/// What an append found. Only `Recorded` wrote anything; every other outcome
/// rolled its transaction back, a version-1 migration included.
#[derive(Debug)]
pub enum Appended {
    Recorded {
        recorded_at: String,
    },
    /// The same ID with the same document and run: nothing changed.
    AlreadyRecorded {
        recorded_at: String,
    },
    RunMissing {
        store_exists: bool,
    },
    /// The ID is recorded with another document or against another run.
    Conflict {
        run_id: String,
        recorded_at: String,
    },
    /// `supersedes` names no observation of this run: none at all, or one of
    /// the run named here.
    SupersedesUnknown {
        belongs_to: Option<String>,
    },
    /// `supersedes` names an observation another one already corrects.
    AlreadySuperseded {
        by: String,
    },
    /// An execution confirmation for a run whose launch failure is recorded,
    /// with the failure's cause.
    ContradictsLaunchFailure {
        cause: Option<String>,
    },
}

/// Append one observation to a recorded run, in one exclusive transaction that
/// migrates a version-1 store first. Nothing is created: a store that does not
/// exist, or holds nothing yet, holds no run to observe.
pub fn append_observation(dir: &StateDir, new: &NewObservation<'_>) -> Result<Appended, Refusal> {
    let file = dir.store();
    if !file.try_exists().unwrap_or(true) {
        return Ok(Appended::RunMissing {
            store_exists: false,
        });
    }
    let fail =
        |failure: StoreFailure| failure.refusal(dir, &file, "record the observation", LOCK_WAIT);
    let mut connection = open(&file, LOCK_WAIT).map_err(fail)?;
    let transaction = exclusive(&mut connection).map_err(fail)?;
    (|| {
        if prepare(&transaction, Prepare::Migrate)? == Contents::Pristine {
            return Ok(Appended::RunMissing { store_exists: true });
        }
        let run = new.run_id.as_str();
        let Some(stored) = run_fields(&transaction, new.run_id)? else {
            return Ok(Appended::RunMissing { store_exists: true });
        };
        // A repeat first, so that a correction repeated after it took effect
        // is still the same import rather than a second correction.
        let recorded = transaction
            .query_row(
                "SELECT run_id, recorded_at, document FROM observations WHERE observation_id = ?1",
                params![new.observation_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .optional()?;
        if let Some((run_id, recorded_at, document)) = recorded {
            return Ok(if run_id == run && document == new.document {
                Appended::AlreadyRecorded { recorded_at }
            } else {
                Appended::Conflict {
                    run_id,
                    recorded_at,
                }
            });
        }
        if let Some(target) = new.supersedes {
            let belongs_to: Option<String> = transaction
                .query_row(
                    "SELECT run_id FROM observations WHERE observation_id = ?1",
                    params![target],
                    |row| row.get(0),
                )
                .optional()?;
            if belongs_to.as_deref() != Some(run) {
                return Ok(Appended::SupersedesUnknown { belongs_to });
            }
            let by: Option<String> = transaction
                .query_row(
                    "SELECT observation_id FROM observations WHERE supersedes = ?1",
                    params![target],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(by) = by {
                return Ok(Appended::AlreadySuperseded { by });
            }
        }
        if new.confirms_execution {
            if let Some((_, detail)) = &stored.launch_failure {
                let cause = detail["cause"].as_str().map(str::to_owned);
                return Ok(Appended::ContradictsLaunchFailure { cause });
            }
        }
        let recorded_at: String = transaction.query_row(
            &format!(
                "INSERT INTO observations (observation_id, run_id, recorded_at, supersedes, \
                 document) VALUES (?1, ?2, {NOW}, ?3, ?4) RETURNING recorded_at"
            ),
            params![new.observation_id, run, new.supersedes, new.document],
            |row| row.get(0),
        )?;
        transaction.commit()?;
        Ok(Appended::Recorded { recorded_at })
    })()
    .map_err(fail)
}

/// A run as the store holds it, checked readable.
#[derive(Debug)]
pub struct StoredRun {
    pub recorded_at: String,
    pub launch: Value,
    /// When the detail was appended, and the detail.
    pub launch_failure: Option<(String, Value)>,
    /// In the order they were recorded; empty unless [`load`] was asked to
    /// read them.
    pub observations: Vec<StoredObservation>,
}

/// One recorded observation of a run.
#[derive(Debug)]
pub struct StoredObservation {
    pub observation_id: String,
    pub recorded_at: String,
    /// The validated envelope as it was imported.
    pub document: Value,
    /// The observation that corrects this one, if any.
    pub superseded_by: Option<String>,
}

/// Whether [`load`] reads a run's observations. A run lookup does not: its
/// read is the run's launch fields and any launch failure, bounded by what
/// the commit wrote, however long the run's later history grows.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Observations {
    Read,
    Skip,
}

#[derive(Debug)]
pub enum Lookup {
    Found(StoredRun),
    /// No such run. `store_exists` says whether there was a store to look in.
    Missing {
        store_exists: bool,
    },
}

/// Read one run, and its `observations` if asked, waiting at most `wait` for a
/// writer's lock. A store that cannot be read, is not a version this release
/// reads, or holds a document of the run this release cannot read, refuses,
/// naming the `attempt`; it never reads as a store without the run. Only a
/// store file known to be absent is one: a directory that cannot be searched
/// says nothing either way, and refuses.
pub fn load(
    dir: &StateDir,
    run_id: &RunId,
    wait: Duration,
    attempt: &str,
    observations: Observations,
) -> Result<Lookup, Refusal> {
    let file = dir.store();
    if !file.try_exists().unwrap_or(true) {
        return Ok(Lookup::Missing {
            store_exists: false,
        });
    }
    let fail = |failure: StoreFailure| failure.refusal(dir, &file, attempt, wait);
    let mut connection = open(&file, wait).map_err(fail)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Deferred)
        .map_err(|error| fail(error.into()))?;
    (|| {
        let contents = prepare(&transaction, Prepare::AsItIs)?;
        if contents == Contents::Pristine {
            return Ok(Lookup::Missing { store_exists: true });
        }
        let Some(mut stored) = run_fields(&transaction, run_id)? else {
            return Ok(Lookup::Missing { store_exists: true });
        };
        if observations == Observations::Read && contents == Contents::Version(2) {
            let mut statement = transaction.prepare(
                "SELECT observed.observation_id, observed.recorded_at, observed.supersedes, \
                 observed.document, (SELECT correction.observation_id FROM observations AS \
                 correction WHERE correction.supersedes = observed.observation_id) FROM \
                 observations AS observed WHERE observed.run_id = ?1 ORDER BY observed.rowid",
            )?;
            let rows = statement.query_map(params![run_id.as_str()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            })?;
            for row in rows {
                let (observation_id, recorded_at, supersedes, text, superseded_by) = row?;
                let what = format!("run {run_id}'s observation {observation_id:?}");
                let document = document(&text, &what)?;
                crate::observation::stored(
                    &document,
                    &observation_id,
                    run_id,
                    supersedes.as_deref(),
                )
                .map_err(|why| StoreFailure::Invalid(format!("{what} {why}")))?;
                stored.observations.push(StoredObservation {
                    observation_id,
                    recorded_at,
                    document,
                    superseded_by,
                });
            }
        }
        Ok(Lookup::Found(stored))
    })()
    .map_err(fail)
}

/// Read a run's launch fields and any launch failure inside `transaction`,
/// and check that this release can read them; `None` if the store does not
/// hold the run.
fn run_fields(
    transaction: &Transaction<'_>,
    run_id: &RunId,
) -> Result<Option<StoredRun>, StoreFailure> {
    let run = transaction
        .query_row(
            "SELECT recorded_at, launch FROM runs WHERE run_id = ?1",
            params![run_id.as_str()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()?;
    let Some((recorded_at, launch)) = run else {
        return Ok(None);
    };
    let failure = transaction
        .query_row(
            "SELECT recorded_at, detail FROM launch_failures WHERE run_id = ?1",
            params![run_id.as_str()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()?;
    let launch = document(&launch, &format!("run {run_id}'s launch record"))?;
    let launch_failure = failure
        .map(|(at, detail)| {
            let detail = document(&detail, &format!("run {run_id}'s launch failure"))?;
            Ok::<_, StoreFailure>((at, detail))
        })
        .transpose()?;
    crate::record::readable(&launch, launch_failure.as_ref().map(|(_, detail)| detail))
        .map_err(|why| StoreFailure::Invalid(format!("run {run_id}'s {why}")))?;
    Ok(Some(StoredRun {
        recorded_at,
        launch,
        launch_failure,
        observations: Vec::new(),
    }))
}

/// Create the directory and file privately if absent, then open the file.
/// Pre-creating the file with mode 0600 gives SQLite's journal the same mode,
/// which SQLite copies from the database file.
///
/// SQLite's EXTRA sync reaches the store's own directory, never the entries
/// that lead to it. So the parent of every directory found missing here is
/// synced before the commit, and a directory that cannot be synced refuses
/// the run like the commit itself: otherwise a power loss after exec could
/// leave a committed attempt with no path to it. A concurrent first use may
/// create one first; it is synced all the same, since its creator may not have
/// finished doing so.
fn create(dir: &StateDir, file: &Path) -> Result<Connection, StoreFailure> {
    let missing: Vec<&Path> = dir
        .path
        .ancestors()
        .take_while(|ancestor| !ancestor.exists())
        .collect();
    DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&dir.path)
        .map_err(|error| StoreFailure::Io {
            what: format!(
                "the record directory {} cannot be created",
                dir.path.display()
            ),
            error,
        })?;
    for parent in missing.iter().filter_map(|created| created.parent()) {
        File::open(parent)
            .and_then(|parent| parent.sync_all())
            .map_err(|error| StoreFailure::Io {
                what: format!(
                    "the directory {} cannot be synced after the record directory was created \
                     in it",
                    parent.display()
                ),
                error,
            })?;
    }
    // Never truncate: an existing store is opened as it is, and one this
    // command cannot use refuses rather than being replaced.
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .open(file)
        .map_err(|error| StoreFailure::Io {
            what: format!(
                "the record store {} cannot be opened for writing",
                file.display()
            ),
            error,
        })?;
    open(file, LOCK_WAIT)
}

/// Open an existing store file, to wait at most `wait` for a lock.
/// `SQLITE_OPEN_READ_WRITE` without `CREATE` never makes a file, and opens a
/// write-protected one read-only, which is enough to read and to roll back
/// nothing. No URI interpretation: the path is a path.
fn open(file: &Path, wait: Duration) -> Result<Connection, StoreFailure> {
    let connection = Connection::open_with_flags(
        file,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    connection.busy_timeout(wait)?;
    // Connection settings, which take no lock and read nothing. `foreign_keys`
    // is a no-op inside a transaction, so it is set before one begins.
    connection.execute_batch(
        "PRAGMA synchronous = EXTRA; PRAGMA fullfsync = ON; PRAGMA foreign_keys = ON;",
    )?;
    Ok(connection)
}

fn exclusive(connection: &mut Connection) -> Result<Transaction<'_>, StoreFailure> {
    Ok(connection.transaction_with_behavior(TransactionBehavior::Exclusive)?)
}

/// What [`prepare`] may change.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Prepare {
    /// Initialize a pristine file at the current version.
    IfPristine,
    /// Migrate a version-1 store to the current version.
    Migrate,
    /// Neither: take the store as it is.
    AsItIs,
}

#[derive(Debug, PartialEq, Eq)]
enum Contents {
    Pristine,
    Version(i32),
}

/// Check the store's identity and version inside the open transaction, and
/// initialize or migrate it if asked. A file that is neither pristine nor a
/// version this release reads refuses.
fn prepare(transaction: &Transaction<'_>, prepare: Prepare) -> Result<Contents, StoreFailure> {
    let application: i32 =
        transaction.pragma_query_value(None, "application_id", |row| row.get(0))?;
    let version: i32 = transaction.pragma_query_value(None, "user_version", |row| row.get(0))?;
    let objects: i64 =
        transaction.query_row("SELECT count(*) FROM sqlite_schema", [], |row| row.get(0))?;
    match (application, version) {
        (0, 0) if objects == 0 => {
            if prepare == Prepare::IfPristine {
                transaction.execute_batch(SCHEMA_1)?;
                transaction.execute_batch(OBSERVATIONS)?;
                // Header pragmas are journaled like any page 1 change, so they
                // roll back with the transaction: a probe with the system
                // sqlite3 saw both read 0 again after ROLLBACK.
                transaction.execute_batch(&format!(
                    "PRAGMA application_id = {APPLICATION_ID}; PRAGMA user_version = \
                     {SCHEMA_VERSION};"
                ))?;
                return Ok(Contents::Version(SCHEMA_VERSION));
            }
            Ok(Contents::Pristine)
        }
        (APPLICATION_ID, 1) if prepare == Prepare::Migrate => {
            transaction.execute_batch(OBSERVATIONS)?;
            transaction.execute_batch(&format!("PRAGMA user_version = {SCHEMA_VERSION};"))?;
            Ok(Contents::Version(SCHEMA_VERSION))
        }
        (APPLICATION_ID, version @ (1 | SCHEMA_VERSION)) => Ok(Contents::Version(version)),
        (APPLICATION_ID, other) => Err(StoreFailure::Invalid(format!(
            "it is record schema version {other}, and this release of harness-dispatch reads \
             versions 1 and {SCHEMA_VERSION}"
        ))),
        _ => Err(StoreFailure::Invalid(format!(
            "it is not a harness-dispatch record store (application ID {application}, user \
             version {version}, {objects} schema objects)"
        ))),
    }
}

/// The refusal for a record the store holds but this release cannot read as
/// one: a launch document of an unknown version, or missing a field.
pub fn unreadable_record(dir: &StateDir, attempt: &str, why: String) -> Refusal {
    StoreFailure::Invalid(why).refusal(dir, &dir.store(), attempt, LOCK_WAIT)
}

/// Parse the stored document `what` names.
fn document(text: &str, what: &str) -> Result<Value, StoreFailure> {
    serde_json::from_str(text)
        .map_err(|error| StoreFailure::Invalid(format!("{what} is not valid JSON ({error})")))
}

/// Why the store could not be used, before it becomes a refusal naming the
/// store and what was being attempted.
#[derive(Debug)]
enum StoreFailure {
    Io { what: String, error: io::Error },
    Sqlite(rusqlite::Error),
    Invalid(String),
}

impl From<rusqlite::Error> for StoreFailure {
    fn from(error: rusqlite::Error) -> Self {
        StoreFailure::Sqlite(error)
    }
}

impl StoreFailure {
    /// `wait` is the lock wait the attempt had, which a lock refusal reports.
    fn refusal(self, dir: &StateDir, file: &Path, attempt: &str, wait: Duration) -> Refusal {
        let store = file.display();
        let elsewhere = match dir.set_by {
            Some(_) => "or name another record directory with --state-dir",
            None => "or name a record directory with --state-dir",
        };
        let never = "harness-dispatch never launches without its record";
        let (code, message, remedy) = match self {
            StoreFailure::Io { what, error } => {
                let full = matches!(error.raw_os_error(), Some(libc::ENOSPC | libc::EDQUOT));
                if full {
                    (
                        "record_store_full",
                        format!("could not {attempt}: {what}: {error}"),
                        format!(
                            "free space on the filesystem holding {store}, {elsewhere}; {never}"
                        ),
                    )
                } else {
                    (
                        "record_store_unwritable",
                        format!("could not {attempt}: {what}: {error}"),
                        format!(
                            "make {} a directory you can write, {elsewhere}; {never}",
                            dir.path.display()
                        ),
                    )
                }
            }
            StoreFailure::Invalid(why) => (
                "record_store_invalid",
                format!("could not {attempt}: the record store {store} cannot be used: {why}"),
                format!(
                    "upgrade harness-dispatch if a newer release wrote the store, or move {store} \
                     aside and keep it; harness-dispatch never resets or replaces a store, \
                     {elsewhere}"
                ),
            ),
            StoreFailure::Sqlite(error) => {
                let shown = format!("could not {attempt} in {store}: {error}");
                match error.sqlite_error_code() {
                    Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked) => (
                        "record_store_locked",
                        format!(
                            "{shown}; another process held the store for longer than the {} ms \
                             lock wait",
                            wait.as_millis()
                        ),
                        format!(
                            "run again once the other harness-dispatch or reader has finished; \
                             the wait is fixed, {elsewhere}"
                        ),
                    ),
                    Some(ErrorCode::DiskFull) => (
                        "record_store_full",
                        shown,
                        format!(
                            "free space on the filesystem holding {store}, {elsewhere}; {never}"
                        ),
                    ),
                    Some(
                        ErrorCode::ReadOnly | ErrorCode::PermissionDenied | ErrorCode::CannotOpen,
                    ) => (
                        "record_store_unwritable",
                        shown,
                        format!(
                            "make {} and {store} writable, {elsewhere}; {never}",
                            dir.path.display()
                        ),
                    ),
                    Some(ErrorCode::NotADatabase | ErrorCode::DatabaseCorrupt) => (
                        "record_store_invalid",
                        shown,
                        format!(
                            "move {store} aside and keep it; harness-dispatch never resets or \
                             replaces a store, {elsewhere}"
                        ),
                    ),
                    _ => (
                        "record_commit_failed",
                        shown,
                        format!(
                            "check the filesystem holding {store} and run again, {elsewhere}; \
                             {never}"
                        ),
                    ),
                }
            }
        };
        Refusal::new(code, Stage::Record, EXIT_RECORD, message, remedy).source(store.to_string())
    }
}
