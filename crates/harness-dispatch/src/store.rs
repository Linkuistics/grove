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
//! seconds. The store is opened only after the worker has been reaped, so no
//! lock is held across policy evaluation, and the wait neither extends nor
//! consumes the selection bound.
//!
//! **Versions and immutability.** The file names itself with an application ID
//! and a schema version, checked inside the transaction; a pristine file is
//! initialized in that same transaction, and anything else that does not match
//! refuses. A store is never reset or replaced. A run's launch fields are one
//! JSON document carrying its own schema version, with a field for everything
//! a later increment records, `null` until then; triggers abort any update or
//! deletion of a committed row. So a release that records something new writes
//! it into new runs' documents, at a new document version if schema 1 has no
//! field for it, and a new table arrives by a migration that only creates. No
//! committed launch field is ever rewritten.

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

/// The default state directory, relative to HOME. `XDG_STATE_HOME` is not
/// consulted: an environment-selected location is what `--state-dir` is for.
pub const DEFAULT_STATE_DIR: &str = ".local/state/harness-dispatch";
pub const STORE_FILE: &str = "records.sqlite3";

/// The fixed record-store lock wait. It is separate from the selection bound.
pub const LOCK_WAIT: Duration = Duration::from_secs(2);

/// "HDRS", at offset 68 of the database header.
const APPLICATION_ID: i32 = 0x4844_5253;
const SCHEMA_VERSION: i32 = 1;

/// Schema 1's tables and triggers. [`prepare`] writes them together with
/// the header's application ID and version, in the transaction that found the
/// file pristine, so a concurrent first use sees either nothing or all of it.
const SCHEMA: &str = "
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

/// SQLite's clock, UTC to the millisecond, evaluated in the committing
/// statement.
const NOW: &str = "strftime('%Y-%m-%dT%H:%M:%fZ', 'now')";

/// Where this invocation's records live.
#[derive(Clone, Debug)]
pub struct StateDir {
    /// Absolute: `--state-dir` joined to the original cwd, or the default
    /// under HOME. Kept as spelled; symlinks are not resolved.
    pub path: PathBuf,
    /// The flag that chose it, or `None` for the default.
    pub flag: Option<&'static str>,
}

impl StateDir {
    pub fn resolve(
        given: Option<&Path>,
        cwd: &Path,
        home: Option<&OsStr>,
    ) -> Result<StateDir, Refusal> {
        if let Some(given) = given {
            return Ok(StateDir {
                path: cwd.join(given),
                flag: Some("--state-dir"),
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
            flag: None,
        })
    }

    pub fn store(&self) -> PathBuf {
        self.path.join(STORE_FILE)
    }

    pub fn from(&self) -> &'static str {
        self.flag.unwrap_or("default")
    }

    pub fn to_json(&self) -> Value {
        json!({ "path": self.path.to_string_lossy(), "from": self.from() })
    }

    pub fn to_text(&self) -> String {
        let from = self.flag.unwrap_or("the default");
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
    let fail =
        |failure: StoreFailure| failure.refusal(dir, &file, "commit the run's handoff record");
    let mut connection = create(dir, &file).map_err(fail)?;
    let transaction = exclusive(&mut connection).map_err(fail)?;
    let recorded_at = (|| {
        prepare(&transaction, Initialize::IfPristine)?;
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
    let fail =
        |failure: StoreFailure| failure.refusal(dir, &file, "append the run's launch failure");
    let mut connection = open(&file).map_err(fail)?;
    let transaction = exclusive(&mut connection).map_err(fail)?;
    (|| {
        prepare(&transaction, Initialize::Never)?;
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

/// A run as the store holds it.
#[derive(Debug)]
pub struct StoredRun {
    pub recorded_at: String,
    pub launch: Value,
    /// When the detail was appended, and the detail.
    pub launch_failure: Option<(String, Value)>,
}

#[derive(Debug)]
pub enum Lookup {
    Found(StoredRun),
    /// No such run. `store_exists` says whether there was a store to look in.
    Missing {
        store_exists: bool,
    },
}

/// Read one run. A store that cannot be read, or is not a version this
/// release reads, refuses; it never reads as a store without the run.
pub fn load(dir: &StateDir, run_id: &RunId) -> Result<Lookup, Refusal> {
    let file = dir.store();
    if !file.try_exists().unwrap_or(true) {
        return Ok(Lookup::Missing {
            store_exists: false,
        });
    }
    let fail = |failure: StoreFailure| failure.refusal(dir, &file, "read the run record");
    let mut connection = open(&file).map_err(fail)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Deferred)
        .map_err(|error| fail(error.into()))?;
    (|| {
        if prepare(&transaction, Initialize::Never)? == Contents::Pristine {
            return Ok(Lookup::Missing { store_exists: true });
        }
        let run = transaction
            .query_row(
                "SELECT recorded_at, launch FROM runs WHERE run_id = ?1",
                params![run_id.as_str()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?;
        let Some((recorded_at, launch)) = run else {
            return Ok(Lookup::Missing { store_exists: true });
        };
        let failure = transaction
            .query_row(
                "SELECT recorded_at, detail FROM launch_failures WHERE run_id = ?1",
                params![run_id.as_str()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?;
        let launch = document(&launch, "runs.launch")?;
        let launch_failure = failure
            .map(|(at, detail)| {
                Ok::<_, StoreFailure>((at, document(&detail, "launch_failures.detail")?))
            })
            .transpose()?;
        Ok(Lookup::Found(StoredRun {
            recorded_at,
            launch,
            launch_failure,
        }))
    })()
    .map_err(fail)
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
    open(file)
}

/// Open an existing store file. `SQLITE_OPEN_READ_WRITE` without `CREATE`
/// never makes a file, and opens a write-protected one read-only, which is
/// enough to read and to roll back nothing. No URI interpretation: the path is
/// a path.
fn open(file: &Path) -> Result<Connection, StoreFailure> {
    let connection = Connection::open_with_flags(
        file,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    connection.busy_timeout(LOCK_WAIT)?;
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Initialize {
    IfPristine,
    Never,
}

#[derive(Debug, PartialEq, Eq)]
enum Contents {
    Pristine,
    Schema1,
}

/// Check the store's identity and version inside the open transaction, and
/// initialize a pristine one if asked. A file that is neither pristine nor
/// this store's current version refuses.
fn prepare(
    transaction: &Transaction<'_>,
    initialize: Initialize,
) -> Result<Contents, StoreFailure> {
    let application: i32 =
        transaction.pragma_query_value(None, "application_id", |row| row.get(0))?;
    let version: i32 = transaction.pragma_query_value(None, "user_version", |row| row.get(0))?;
    let objects: i64 =
        transaction.query_row("SELECT count(*) FROM sqlite_schema", [], |row| row.get(0))?;
    match (application, version) {
        (0, 0) if objects == 0 => {
            if initialize == Initialize::IfPristine {
                transaction.execute_batch(SCHEMA)?;
                // Header pragmas are journaled like any page 1 change, so they
                // roll back with the transaction: a probe with the system
                // sqlite3 saw both read 0 again after ROLLBACK.
                transaction.execute_batch(&format!(
                    "PRAGMA application_id = {APPLICATION_ID}; PRAGMA user_version = \
                     {SCHEMA_VERSION};"
                ))?;
                return Ok(Contents::Schema1);
            }
            Ok(Contents::Pristine)
        }
        (APPLICATION_ID, SCHEMA_VERSION) => Ok(Contents::Schema1),
        (APPLICATION_ID, other) => Err(StoreFailure::Invalid(format!(
            "it is record schema version {other}, and this release of harness-dispatch reads \
             version {SCHEMA_VERSION}"
        ))),
        _ => Err(StoreFailure::Invalid(format!(
            "it is not a harness-dispatch record store (application ID {application}, user \
             version {version}, {objects} schema objects)"
        ))),
    }
}

fn document(text: &str, column: &str) -> Result<Value, StoreFailure> {
    serde_json::from_str(text).map_err(|error| {
        StoreFailure::Invalid(format!("its {column} value is not valid JSON ({error})"))
    })
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
    fn refusal(self, dir: &StateDir, file: &Path, attempt: &str) -> Refusal {
        let store = file.display();
        let elsewhere = match dir.flag {
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
                            LOCK_WAIT.as_millis()
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
