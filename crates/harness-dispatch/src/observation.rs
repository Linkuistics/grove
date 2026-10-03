//! Later observations of a recorded run, and `record observe`
//! (`docs/specs/harness-selection-and-execution.md`, *Records and later
//! observations*).
//!
//! An observer, never harness-dispatch, knows whether a harness ran and how its
//! work went. It writes one version-1 observation document and imports it
//! against the run. The document is read once, as data, and validated field by
//! field, so that a refusal names the exact location it found. What is
//! validated is shape and association: the source and evidence are the
//! importer's assertion, not a truth this command can check.
//!
//! Every measurement is `observed`, `unknown` or `unobserved`, and a field an
//! import does not supply stays unobserved. Nothing is defaulted, so an absent
//! measurement never reads as zero, false or accepted. A run's launch fields
//! are no part of an observation, and no import can change them.
//!
//! The export presents the measurements without aggregating them: each
//! observation with every field, and for the run, per field, the current
//! observations that supplied it, side by side.

use std::fmt::Write as _;
use std::fs;
use std::io::Read as _;
use std::os::unix::fs::OpenOptionsExt as _;
use std::path::Path;

use serde_json::{json, Map, Value};

use crate::cli::ObserveArgs;
use crate::limits::{Bound, Origin};
use crate::record;
use crate::refusal::{Refusal, Stage, EXIT_REFUSED};
use crate::run_id::{self, RunId};
use crate::store::{self, Appended, NewObservation};

/// The observation envelope's own version.
pub const OBSERVATION_VERSION: u64 = 1;

/// The fixed bound on an observation document's size.
pub const OBSERVATION_MAX_BYTES: u64 = 1_048_576;

/// The bound on an observation, finding or repair ID, as on a task ID.
const ID_MAX_BYTES: usize = 1024;

const ENVELOPE: [&str; 8] = [
    "schemaVersion",
    "observationId",
    "runId",
    "source",
    "observedAt",
    "evidence",
    "supersedes",
    "measurements",
];

/// Names of what a run records, which an observation might try to carry.
const LAUNCH: [&str; 4] = ["launch", "launchFailure", "execution", "recordedAt"];

const MEASUREMENT_FIELDS: [&str; 3] = ["state", "value", "unit"];
const FINDING_FIELDS: [&str; 3] = ["id", "summary", "repairs"];
const REPAIR_FIELDS: [&str; 4] = ["id", "summary", "findings", "runId"];
const EXIT_FIELDS: [&str; 2] = ["code", "signal"];
const ESTIMATE_FIELDS: [&str; 3] = ["probability", "calibration", "uncalibrated"];
const TIME_UNITS: [&str; 4] = ["ms", "s", "min", "h"];

/// How a run that started a harness ended, as `ending` reports it
/// (`docs/specs/harness-selection-and-execution.md`, *The run ending*).
pub const ENDINGS: [&str; 3] = ["exit_signal", "harness_exit", "cancelled"];

/// The value type of one measurement.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Type {
    /// The literal `true`: the observation is the confirmation.
    Confirmation,
    /// `exit_signal`, `harness_exit` or `cancelled`.
    Ending,
    /// `{ code }` or `{ signal }`.
    Exit,
    /// A non-negative number with a time unit.
    Time,
    /// A non-negative number with any nonblank unit.
    Usage,
    /// `accepted` or `rejected`.
    Acceptance,
    /// Findings with stable IDs.
    Findings,
    /// Repairs with stable IDs.
    Repairs,
    /// A whole number of events.
    Count,
    /// Nonblank references to evidence.
    Links,
    /// A number from 0 to 1.
    Probability,
    /// A probability that names its calibration or says it has none.
    Estimate,
}

impl Type {
    fn quantity(self) -> bool {
        matches!(self, Type::Time | Type::Usage)
    }
}

/// Every supported measurement, in the order the export lists them.
const MEASUREMENTS: [(&str, Type); 16] = [
    ("executionConfirmation", Type::Confirmation),
    ("ending", Type::Ending),
    ("exit", Type::Exit),
    ("duration", Type::Time),
    ("inputUsage", Type::Usage),
    ("outputUsage", Type::Usage),
    ("totalUsage", Type::Usage),
    ("acceptance", Type::Acceptance),
    ("missedDefects", Type::Findings),
    ("falseFindings", Type::Findings),
    ("downstreamRepair", Type::Repairs),
    ("humanTime", Type::Time),
    ("humanInterventions", Type::Count),
    ("evidenceLinks", Type::Links),
    ("choiceProbability", Type::Probability),
    ("successProbability", Type::Estimate),
];

fn names() -> String {
    MEASUREMENTS
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(", ")
}

/// A validated observation, ready to append.
#[derive(Debug)]
pub struct Observation {
    pub id: String,
    pub supersedes: Option<String>,
    pub confirms_execution: bool,
    /// The validated envelope, as stored and as a repeat is compared.
    pub document: Value,
}

/// What `record observe` did.
pub struct Receipt {
    run_id: RunId,
    observation: Observation,
    recorded_at: String,
    /// Whether this import wrote it, or found it already recorded.
    new: bool,
}

pub fn observe(args: &ObserveArgs) -> Result<Receipt, Refusal> {
    let located = record::locate(&args.run, args.state_dir.as_deref())?;
    let observation = read(&args.file, &located.cwd, &located.run_id)?;
    let document = observation.document.to_string();
    let append = NewObservation {
        observation_id: &observation.id,
        run_id: &located.run_id,
        supersedes: observation.supersedes.as_deref(),
        confirms_execution: observation.confirms_execution,
        document: &document,
    };
    let run_id = &located.run_id;
    let id = &observation.id;
    let rejected = |code: &'static str, message: String, remedy: String| {
        Refusal::new(code, Stage::Record, EXIT_REFUSED, message, remedy)
            .input("--file")
            .source(located.dir.store().to_string_lossy())
    };
    let (recorded_at, new) = match store::append_observation(&located.dir, &append)? {
        Appended::Recorded { recorded_at } => (recorded_at, true),
        Appended::AlreadyRecorded { recorded_at } => (recorded_at, false),
        Appended::RunMissing { store_exists } => {
            return Err(record::run_not_found(&located.dir, run_id, store_exists))
        }
        Appended::Conflict {
            run_id: recorded_run,
            recorded_at,
        } => {
            let against = if recorded_run == run_id.as_str() {
                "with different content".to_owned()
            } else {
                format!("against run {recorded_run}")
            };
            return Err(rejected(
                "observation_conflict",
                format!("observation {id:?} is already recorded {against} (at {recorded_at})"),
                format!(
                    "a recorded observation never changes: give a new observation a new \
                     observationId, and to correct {id:?} name it in `supersedes`; `record show \
                     --run {recorded_run}` shows what is recorded"
                ),
            )
            .location("observation.observationId"));
        }
        Appended::SupersedesUnknown { belongs_to } => {
            let target = observation.supersedes.as_deref().unwrap_or_default();
            let message = match belongs_to {
                Some(other) => format!(
                    "`supersedes` names observation {target:?}, which belongs to run {other}, not \
                     run {run_id}"
                ),
                None => format!(
                    "`supersedes` names observation {target:?}, which is not recorded for run \
                     {run_id}"
                ),
            };
            return Err(rejected(
                "supersedes_unknown",
                message,
                format!(
                    "name an observation of run {run_id} in `supersedes`, as `record show --run \
                     {run_id}` lists them, or omit `supersedes` for an observation that corrects \
                     nothing"
                ),
            )
            .location("observation.supersedes"));
        }
        Appended::AlreadySuperseded { by } => {
            let target = observation.supersedes.as_deref().unwrap_or_default();
            return Err(rejected(
                "already_superseded",
                format!(
                    "observation {target:?} is already superseded by observation {by:?}, and an \
                     observation is corrected once"
                ),
                format!("to correct it again, name the current correction {by:?} in `supersedes`"),
            )
            .location("observation.supersedes"));
        }
        Appended::ContradictsLaunchFailure { cause } => {
            let cause = cause.map(|cause| format!(" ({cause})")).unwrap_or_default();
            return Err(rejected(
                "observation_contradicts_record",
                format!(
                    "the observation confirms that run {run_id} executed, but harness-dispatch \
                     recorded a launch failure for it{cause}: its harness was never executed"
                ),
                "check that the observation names the run it observed; report \
                 executionConfirmation as unknown when the observer cannot tell"
                    .to_owned(),
            )
            .location("observation.measurements.executionConfirmation"));
        }
    };
    Ok(Receipt {
        run_id: located.run_id,
        observation,
        recorded_at,
        new,
    })
}

impl Receipt {
    fn status(&self) -> &'static str {
        if self.new {
            "recorded"
        } else {
            "already_recorded"
        }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "schemaVersion": 1,
            "observation": {
                "observationId": self.observation.id,
                "runId": self.run_id.as_str(),
                "supersedes": self.observation.supersedes,
                "recordedAt": self.recorded_at,
                "status": self.status(),
            },
        })
    }

    pub fn to_text(&self) -> String {
        let id = &self.observation.id;
        let mut text = if self.new {
            format!(
                "Recorded observation {id:?} against run {} at {}.\n",
                self.run_id, self.recorded_at
            )
        } else {
            format!(
                "Observation {id:?} is already recorded against run {} (at {}); nothing changed.\n",
                self.run_id, self.recorded_at
            )
        };
        if let Some(supersedes) = &self.observation.supersedes {
            let _ = writeln!(
                text,
                "It supersedes observation {supersedes:?}, which is kept."
            );
        }
        text
    }
}

/// Read and validate the observation document at `given`, resolved against
/// the cwd: a regular file, opened without blocking, within the fixed bound.
pub fn read(given: &Path, cwd: &Path, run_id: &RunId) -> Result<Observation, Refusal> {
    let joined = cwd.join(given);
    let shown = joined.to_string_lossy().into_owned();
    let unreadable = |what: String| {
        Refusal::new(
            "observation_unreadable",
            Stage::Observation,
            EXIT_REFUSED,
            format!("the observation {shown} {what}"),
            "name a readable version-1 observation document with --file, relative to the current \
             directory",
        )
        .input("--file")
        .source(shown.clone())
    };
    let file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(&joined)
        .map_err(|error| unreadable(format!("cannot be opened: {error}")))?;
    let metadata = file
        .metadata()
        .map_err(|error| unreadable(format!("cannot be examined: {error}")))?;
    if !metadata.is_file() {
        return Err(unreadable("is not a regular file".to_owned()));
    }
    let mut bytes = Vec::new();
    file.take(OBSERVATION_MAX_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| unreadable(format!("cannot be read: {error}")))?;
    if bytes.len() as u64 > OBSERVATION_MAX_BYTES {
        let size = metadata.len().max(bytes.len() as u64);
        return Err(Refusal::new(
            "observation_too_large",
            Stage::Observation,
            EXIT_REFUSED,
            format!(
                "the observation {shown} is {size} bytes, over the fixed bound of \
                 {OBSERVATION_MAX_BYTES} bytes"
            ),
            "split the measurements across several observations, or link bulky evidence from \
             evidenceLinks instead of including it",
        )
        .input("--file")
        .source(shown)
        .bound(Bound {
            name: "observation",
            unit: "bytes",
            value: OBSERVATION_MAX_BYTES,
            origin: Origin::Fixed,
        }));
    }
    let check = Check {
        source: &shown,
        expected: "--run names",
    };
    let value: Value = serde_json::from_slice(&bytes).map_err(|error| {
        check.invalid("observation", format!("the document is not JSON: {error}"))
    })?;
    check.envelope(value, run_id)
}

/// What dispatch observed of a run it supervised to its end.
pub struct RunEnd<'a> {
    pub run_id: &'a RunId,
    /// One of [`ENDINGS`].
    pub ending: &'a str,
    /// The harness's own exit code, when it exited.
    pub code: Option<i32>,
    /// The harness's death signal, when it has a conventional `SIG` name; a
    /// signal without one is reported as an exit nobody could name, unknown.
    pub signal: Option<&'a str>,
    /// From the harness's start to its reap.
    pub duration: std::time::Duration,
}

/// The ID of the observation `harness-dispatch` makes of `run_id`'s end, which
/// names the run so that it is one per run.
pub fn end_observation_id(run_id: &RunId) -> String {
    format!("harness-dispatch-end-{run_id}")
}

/// Dispatch's own **end observation** of a run it supervised to its end: an
/// ordinary version-1 observation, validated as an import of it would be, whose
/// `source` says who made it. The same document is what `--ending-file` holds.
pub fn end_observation(end: &RunEnd<'_>) -> Result<Observation, Refusal> {
    let exit = match (end.code, end.signal) {
        (Some(code), _) => json!({ "state": "observed", "value": { "code": code & 0xff } }),
        (None, Some(signal)) => json!({ "state": "observed", "value": { "signal": signal } }),
        (None, None) => json!({ "state": "unknown" }),
    };
    let millis = u64::try_from(end.duration.as_millis()).unwrap_or(u64::MAX);
    let document = json!({
        "schemaVersion": OBSERVATION_VERSION,
        "observationId": end_observation_id(end.run_id),
        "runId": end.run_id.as_str(),
        "source": "harness-dispatch",
        "observedAt": now(),
        "evidence": "harness-dispatch supervised the harness to its end: it spawned it as its own \
                     child, reaped it, and measured the run from the harness's start to its reap",
        "measurements": {
            "executionConfirmation": { "state": "observed", "value": true },
            "ending": { "state": "observed", "value": end.ending },
            "exit": exit,
            "duration": { "state": "observed", "value": millis, "unit": "ms" },
        },
    });
    Check {
        source: "harness-dispatch's end observation",
        expected: "harness-dispatch supervised",
    }
    .envelope(document, end.run_id)
}

/// The current UTC time as an RFC 3339 date-time with millisecond precision.
fn now() -> String {
    let since = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let (days, rest) = (since.as_secs() / 86_400, since.as_secs() % 86_400);
    // Civil date from a day count (Howard Hinnant's days_from_civil, inverted),
    // https://howardhinnant.github.io/date_algorithms.html#civil_from_days
    let z = i64::try_from(days).unwrap_or(0) + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
        rest / 3600,
        rest % 3600 / 60,
        rest % 60,
        since.subsec_millis()
    )
}

/// Check a document the store holds as observation `id` of `run_id`, which
/// supersedes `supersedes`: the validation its import passed, and that it is
/// the observation its row says it is. Why it is not one this release reads,
/// if it is not, to follow the observation's name. The store does not vouch
/// for what it holds: a later release, another writer or a damaged file can
/// put there what no import of this one would have written.
pub fn stored(
    document: &Value,
    id: &str,
    run_id: &RunId,
    supersedes: Option<&str>,
) -> Result<(), String> {
    let check = Check {
        source: "the record store",
        expected: "the store holds it for",
    };
    let unreadable = |refusal: Refusal| {
        let at = refusal.location.as_deref().unwrap_or("observation");
        let why = &refusal.message;
        format!(
            "is not a version-{OBSERVATION_VERSION} observation this release reads, at {at}: {why}"
        )
    };
    let observation = check
        .envelope(document.clone(), run_id)
        .map_err(unreadable)?;
    if observation.id != id {
        let other = observation.id;
        return Err(format!(
            "is stored with a document that names observation {other:?}"
        ));
    }
    let named = |target: Option<&str>| target.map_or("nothing".to_owned(), |t| format!("{t:?}"));
    if observation.supersedes.as_deref() != supersedes {
        return Err(format!(
            "is stored superseding {}, but its document supersedes {}",
            named(supersedes),
            named(observation.supersedes.as_deref())
        ));
    }
    Ok(())
}

/// Validates one observation document, naming the file a refusal is about.
struct Check<'a> {
    source: &'a str,
    /// What says which run the document must name, completing "but … run R".
    expected: &'a str,
}

impl Check<'_> {
    fn invalid(&self, location: &str, message: String) -> Refusal {
        Refusal::new(
            "observation_invalid",
            Stage::Observation,
            EXIT_REFUSED,
            message,
            format!(
                "correct {location} in the observation {}; `harness-dispatch record observe \
                 --help` shows a version-1 observation",
                self.source
            ),
        )
        .input("--file")
        .source(self.source)
        .location(location)
    }

    fn envelope(&self, value: Value, run_id: &RunId) -> Result<Observation, Refusal> {
        let at = "observation";
        let envelope = self.object(&value, at)?;

        // The version comes first: a later schema's fields are not unknown to it.
        let schema = self.field(envelope, "schemaVersion", at)?;
        match schema.as_u64() {
            Some(OBSERVATION_VERSION) => {}
            Some(other) => {
                return Err(Refusal::new(
                    "unsupported_version",
                    Stage::Observation,
                    EXIT_REFUSED,
                    format!(
                        "observation schemaVersion {other} is not supported; this release reads \
                         schemaVersion {OBSERVATION_VERSION}"
                    ),
                    "write a version-1 observation, or upgrade harness-dispatch",
                )
                .input("--file")
                .source(self.source)
                .location("observation.schemaVersion"))
            }
            None => {
                return Err(self.invalid(
                    "observation.schemaVersion",
                    format!(
                        "`schemaVersion` must be the number 1, found {}",
                        describe(schema)
                    ),
                ))
            }
        }
        if let Some(unknown) = envelope
            .keys()
            .find(|key| !ENVELOPE.contains(&key.as_str()))
        {
            let message = if LAUNCH.contains(&unknown.as_str()) {
                format!(
                    "`{unknown}` is part of what harness-dispatch records for the run, which no \
                     observation can supply or change; an observation carries measurements"
                )
            } else {
                format!(
                    "unknown field `{unknown}`; an observation is {{ schemaVersion, \
                     observationId, runId, source, observedAt, evidence, supersedes?, \
                     measurements }}"
                )
            };
            return Err(self.invalid(&format!("observation.{unknown}"), message));
        }

        let id = self.identifier(
            self.field(envelope, "observationId", at)?,
            "observation.observationId",
        )?;
        let named = self.string(
            self.field(envelope, "runId", at)?,
            "observation.runId",
            true,
        )?;
        if !run_id::is_canonical(named) {
            return Err(self.invalid(
                "observation.runId",
                format!("{named:?} is not a run ID: {}", run_id_form()),
            ));
        }
        if named != run_id.as_str() {
            return Err(self.invalid(
                "observation.runId",
                format!(
                    "the observation is about run {named}, but {} run {run_id}",
                    self.expected
                ),
            ));
        }
        self.string(
            self.field(envelope, "source", at)?,
            "observation.source",
            true,
        )?;
        let observed_at = self.string(
            self.field(envelope, "observedAt", at)?,
            "observation.observedAt",
            false,
        )?;
        if !is_date_time(observed_at) {
            return Err(self.invalid(
                "observation.observedAt",
                format!(
                    "`observedAt` must be an RFC 3339 date-time with an uppercase T and Z or a \
                     numeric offset, such as 2026-10-01T09:30:00Z, found {observed_at:?}"
                ),
            ));
        }
        self.string(
            self.field(envelope, "evidence", at)?,
            "observation.evidence",
            true,
        )?;
        let supersedes = match envelope.get("supersedes") {
            None => None,
            Some(target) => {
                let target = self.identifier(target, "observation.supersedes")?;
                if target == id {
                    return Err(self.invalid(
                        "observation.supersedes",
                        "an observation cannot supersede itself".to_owned(),
                    ));
                }
                Some(target.to_owned())
            }
        };
        let measurements = self.object(
            self.field(envelope, "measurements", at)?,
            "observation.measurements",
        )?;
        for (name, measurement) in measurements {
            let location = format!("observation.measurements.{name}");
            let Some((_, kind)) = MEASUREMENTS.iter().find(|(known, _)| known == name) else {
                return Err(self.invalid(
                    &location,
                    format!(
                        "unknown measurement `{name}`; the supported measurements are {}",
                        names()
                    ),
                ));
            };
            self.measurement(name, *kind, measurement, &location)?;
        }
        let confirms_execution = confirms_execution(&value);
        let id = id.to_owned();
        Ok(Observation {
            id,
            supersedes,
            confirms_execution,
            document: value,
        })
    }

    fn measurement(&self, name: &str, kind: Type, value: &Value, at: &str) -> Result<(), Refusal> {
        let fields = self.object(value, at)?;
        self.known(
            fields,
            &MEASUREMENT_FIELDS,
            at,
            "a measurement is { state, value?, unit? }",
        )?;
        let state = self.string(
            self.field(fields, "state", at)?,
            &format!("{at}.state"),
            false,
        )?;
        match state {
            "observed" => {
                let value = fields.get("value").ok_or_else(|| {
                    self.invalid(
                        &format!("{at}.value"),
                        format!(
                            "an observed measurement has a `value`; report `{name}` as unknown \
                             when the observer could not determine it"
                        ),
                    )
                })?;
                self.value(name, kind, value, &format!("{at}.value"))?;
                let unit = fields.get("unit");
                match (kind.quantity(), unit) {
                    (true, Some(unit)) => self.unit(name, kind, unit, &format!("{at}.unit")),
                    (true, None) => Err(self.invalid(
                        &format!("{at}.unit"),
                        format!(
                            "`{name}` is a quantity, so an observed value names its `unit`{}",
                            units(kind)
                        ),
                    )),
                    (false, Some(_)) => Err(self.invalid(
                        &format!("{at}.unit"),
                        format!("`{name}` is not a quantity, and takes no `unit`"),
                    )),
                    (false, None) => Ok(()),
                }
            }
            "unknown" | "unobserved" => {
                for extra in ["value", "unit"] {
                    if fields.contains_key(extra) {
                        return Err(self.invalid(
                            &format!("{at}.{extra}"),
                            format!(
                                "an {state} measurement carries no `{extra}`: only an observed \
                                 one has a value"
                            ),
                        ));
                    }
                }
                Ok(())
            }
            other => Err(self.invalid(
                &format!("{at}.state"),
                format!("`state` must be observed, unknown or unobserved, found {other:?}"),
            )),
        }
    }

    fn value(&self, name: &str, kind: Type, value: &Value, at: &str) -> Result<(), Refusal> {
        match kind {
            Type::Confirmation => match value {
                Value::Bool(true) => Ok(()),
                Value::Bool(false) => Err(self.invalid(
                    at,
                    "an execution confirmation's value is true: it confirms that the harness \
                     ran. Report it as unknown when the observer cannot tell; a harness that \
                     never ran is harness-dispatch's own launch-failure record"
                        .to_owned(),
                )),
                other => Err(self.invalid(at, format!("expected true, found {}", describe(other)))),
            },
            Type::Ending => match self.string(value, at, false)? {
                ending if ENDINGS.contains(&ending) => Ok(()),
                other => Err(self.invalid(
                    at,
                    format!(
                        "`ending` is one of {}, found {other:?}; report it as unknown when the \
                         observer could not tell",
                        ENDINGS.join(", ")
                    ),
                )),
            },
            Type::Exit => self.exit(value, at),
            Type::Time | Type::Usage => self.amount(value, at).map(|_| ()),
            Type::Acceptance => match self.string(value, at, false)? {
                "accepted" | "rejected" => Ok(()),
                other => Err(self.invalid(
                    at,
                    format!(
                        "`acceptance` is accepted or rejected, found {other:?}; report it as \
                         unknown when the observer could not tell"
                    ),
                )),
            },
            Type::Findings => self.records(
                value,
                at,
                &FINDING_FIELDS,
                "finding",
                |check, fields, at| {
                    check.optional_string(fields, "summary", at)?;
                    check.identifiers(fields, "repairs", at)
                },
            ),
            Type::Repairs => {
                self.records(value, at, &REPAIR_FIELDS, "repair", |check, fields, at| {
                    check.optional_string(fields, "summary", at)?;
                    check.identifiers(fields, "findings", at)?;
                    if let Some(run) = fields.get("runId") {
                        let location = format!("{at}.runId");
                        let run = check.string(run, &location, true)?;
                        if !run_id::is_canonical(run) {
                            return Err(check.invalid(
                                &location,
                                format!("{run:?} is not a run ID: {}", run_id_form()),
                            ));
                        }
                    }
                    Ok(())
                })
            }
            Type::Count => match value.as_u64() {
                Some(_) => Ok(()),
                None => Err(self.invalid(
                    at,
                    format!(
                        "`{name}` is a count, a whole number of at least 0, found {}",
                        describe(value)
                    ),
                )),
            },
            Type::Links => {
                for (index, link) in self.array(value, at)?.iter().enumerate() {
                    self.string(link, &format!("{at}[{index}]"), true)?;
                }
                Ok(())
            }
            Type::Probability => self.probability(value, at),
            Type::Estimate => {
                let fields = self.object(value, at)?;
                let shape = "a success estimate is { probability, calibration } or { \
                             probability, uncalibrated: true }";
                self.known(fields, &ESTIMATE_FIELDS, at, shape)?;
                self.probability(
                    self.field(fields, "probability", at)?,
                    &format!("{at}.probability"),
                )?;
                match (fields.get("calibration"), fields.get("uncalibrated")) {
                    (Some(calibration), None) => self
                        .string(calibration, &format!("{at}.calibration"), true)
                        .map(|_| ()),
                    (None, Some(Value::Bool(true))) => Ok(()),
                    (None, Some(other)) => Err(self.invalid(
                        &format!("{at}.uncalibrated"),
                        format!(
                            "`uncalibrated` is true when present, found {}; a calibrated \
                             estimate names its data or version in `calibration` instead",
                            describe(other)
                        ),
                    )),
                    (Some(_), Some(_)) => Err(self.invalid(
                        at,
                        format!("an estimate is calibrated or uncalibrated, and this one says both; {shape}"),
                    )),
                    (None, None) => Err(self.invalid(
                        at,
                        format!(
                            "a success probability names the calibration data or version it \
                             came from, or is labelled uncalibrated; {shape}"
                        ),
                    )),
                }
            }
        }
    }

    fn exit(&self, value: &Value, at: &str) -> Result<(), Refusal> {
        let fields = self.object(value, at)?;
        let shape = "an exit is { code } or { signal }";
        self.known(fields, &EXIT_FIELDS, at, shape)?;
        match (fields.get("code"), fields.get("signal")) {
            (Some(code), None) => match code.as_u64() {
                Some(0..=255) => Ok(()),
                _ => Err(self.invalid(
                    &format!("{at}.code"),
                    format!(
                        "an exit code is a whole number from 0 to 255, found {}",
                        describe_number(code)
                    ),
                )),
            },
            (None, Some(signal)) => {
                let location = format!("{at}.signal");
                let signal = self.string(signal, &location, false)?;
                let named = signal.strip_prefix("SIG").is_some_and(|name| {
                    name.starts_with(|c: char| c.is_ascii_uppercase())
                        && name
                            .chars()
                            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                });
                if named {
                    Ok(())
                } else {
                    Err(self.invalid(
                        &location,
                        format!("a signal is named as SIGTERM or SIGKILL are, found {signal:?}"),
                    ))
                }
            }
            (Some(_), Some(_)) => Err(self.invalid(
                at,
                format!(
                    "a harness exits with a code or a signal, and this one names both; {shape}"
                ),
            )),
            (None, None) => {
                Err(self.invalid(at, format!("an exit names its code or its signal; {shape}")))
            }
        }
    }

    fn unit(&self, name: &str, kind: Type, unit: &Value, at: &str) -> Result<(), Refusal> {
        let unit = self.string(unit, at, true)?;
        if kind == Type::Time && !TIME_UNITS.contains(&unit) {
            return Err(self.invalid(
                at,
                format!("`{name}` is a time, found the unit {unit:?}{}", units(kind)),
            ));
        }
        Ok(())
    }

    /// A non-negative number. JSON has no infinity or NaN to exclude.
    fn amount(&self, value: &Value, at: &str) -> Result<f64, Refusal> {
        match value.as_f64() {
            Some(amount) if value.is_number() && amount >= 0.0 => Ok(amount),
            Some(_) if value.is_number() => Err(self.invalid(
                at,
                format!("a quantity is at least 0, found {}", describe_number(value)),
            )),
            _ => Err(self.invalid(at, format!("expected a number, found {}", describe(value)))),
        }
    }

    fn probability(&self, value: &Value, at: &str) -> Result<(), Refusal> {
        match value.as_f64() {
            Some(probability) if value.is_number() && (0.0..=1.0).contains(&probability) => Ok(()),
            _ => Err(self.invalid(
                at,
                format!(
                    "a probability is a number from 0 to 1, found {}",
                    describe_number(value)
                ),
            )),
        }
    }

    /// An array of records, each with a unique stable `id` and the fields
    /// `rest` checks.
    fn records(
        &self,
        value: &Value,
        at: &str,
        known: &[&str],
        what: &str,
        rest: impl Fn(&Self, &Map<String, Value>, &str) -> Result<(), Refusal>,
    ) -> Result<(), Refusal> {
        let shape = format!("a {what} is {{ {} }}", shape(known));
        let mut seen: Vec<&str> = Vec::new();
        for (index, record) in self.array(value, at)?.iter().enumerate() {
            let at = format!("{at}[{index}]");
            let fields = self.object(record, &at)?;
            self.known(fields, known, &at, &shape)?;
            let location = format!("{at}.id");
            let id = self.identifier(self.field(fields, "id", &at)?, &location)?;
            if seen.contains(&id) {
                return Err(self.invalid(
                    &location,
                    format!(
                        "the {what} ID {id:?} appears twice; each {what} has its own stable ID"
                    ),
                ));
            }
            seen.push(id);
            rest(self, fields, &at)?;
        }
        Ok(())
    }

    fn optional_string(
        &self,
        fields: &Map<String, Value>,
        field: &str,
        at: &str,
    ) -> Result<(), Refusal> {
        match fields.get(field) {
            Some(value) => self
                .string(value, &format!("{at}.{field}"), false)
                .map(|_| ()),
            None => Ok(()),
        }
    }

    /// An optional array of IDs, such as a finding's repair observations.
    fn identifiers(
        &self,
        fields: &Map<String, Value>,
        field: &str,
        at: &str,
    ) -> Result<(), Refusal> {
        let Some(value) = fields.get(field) else {
            return Ok(());
        };
        let at = format!("{at}.{field}");
        for (index, id) in self.array(value, &at)?.iter().enumerate() {
            self.identifier(id, &format!("{at}[{index}]"))?;
        }
        Ok(())
    }

    /// A nonblank ID of at most [`ID_MAX_BYTES`].
    fn identifier<'v>(&self, value: &'v Value, at: &str) -> Result<&'v str, Refusal> {
        let id = self.string(value, at, true)?;
        if id.len() > ID_MAX_BYTES {
            return Err(self.invalid(
                at,
                format!(
                    "an ID is at most {ID_MAX_BYTES} bytes, and this one is {} bytes",
                    id.len()
                ),
            ));
        }
        Ok(id)
    }

    fn known(
        &self,
        fields: &Map<String, Value>,
        known: &[&str],
        at: &str,
        shape: &str,
    ) -> Result<(), Refusal> {
        match fields.keys().find(|key| !known.contains(&key.as_str())) {
            Some(unknown) => Err(self.invalid(
                &format!("{at}.{unknown}"),
                format!("unknown field `{unknown}`; {shape}"),
            )),
            None => Ok(()),
        }
    }

    fn field<'v>(
        &self,
        fields: &'v Map<String, Value>,
        field: &str,
        at: &str,
    ) -> Result<&'v Value, Refusal> {
        fields
            .get(field)
            .ok_or_else(|| self.invalid(&format!("{at}.{field}"), format!("`{field}` is missing")))
    }

    fn object<'v>(&self, value: &'v Value, at: &str) -> Result<&'v Map<String, Value>, Refusal> {
        value.as_object().ok_or_else(|| {
            self.invalid(at, format!("expected an object, found {}", describe(value)))
        })
    }

    fn array<'v>(&self, value: &'v Value, at: &str) -> Result<&'v Vec<Value>, Refusal> {
        value.as_array().ok_or_else(|| {
            self.invalid(at, format!("expected an array, found {}", describe(value)))
        })
    }

    fn string<'v>(&self, value: &'v Value, at: &str, nonblank: bool) -> Result<&'v str, Refusal> {
        let text = value.as_str().ok_or_else(|| {
            self.invalid(at, format!("expected a string, found {}", describe(value)))
        })?;
        if nonblank && text.trim().is_empty() {
            return Err(self.invalid(at, "must not be blank".to_owned()));
        }
        Ok(text)
    }
}

fn shape(fields: &[&str]) -> String {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            if index == 0 {
                (*field).to_owned()
            } else {
                format!("{field}?")
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn units(kind: Type) -> String {
    match kind {
        Type::Time => format!(", one of {}", TIME_UNITS.join(", ")),
        _ => ", such as tokens".to_owned(),
    }
}

fn run_id_form() -> &'static str {
    "36 characters of lowercase hexadecimal digits and hyphens, as harness-dispatch reported it"
}

/// A value's type, for "found …" messages.
fn describe(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(_) => "a boolean".to_owned(),
        Value::Number(_) => "a number".to_owned(),
        Value::String(_) => "a string".to_owned(),
        Value::Array(_) => "an array".to_owned(),
        Value::Object(_) => "an object".to_owned(),
    }
}

/// A number as written, or the type of anything else.
fn describe_number(value: &Value) -> String {
    match value {
        Value::Number(number) => number.to_string(),
        other => describe(other),
    }
}

/// Whether `text` is an RFC 3339 date-time (https://www.rfc-editor.org/rfc/rfc3339#section-5.6):
/// `YYYY-MM-DDTHH:MM:SS`, an optional fraction, then `Z` or `±HH:MM`. Only the
/// uppercase `T` and `Z` are taken, the day must exist in its month, and the
/// second may be 60, for a leap second.
fn is_date_time(text: &str) -> bool {
    let bytes = text.as_bytes();
    let number = |from: usize, to: usize| -> Option<u32> {
        let part = bytes.get(from..to)?;
        if !part.iter().all(u8::is_ascii_digit) {
            return None;
        }
        std::str::from_utf8(part).ok()?.parse().ok()
    };
    let at = |index: usize, byte: u8| bytes.get(index) == Some(&byte);
    let separated = at(4, b'-') && at(7, b'-') && at(10, b'T') && at(13, b':') && at(16, b':');
    let (Some(year), Some(month), Some(day), Some(hour), Some(minute), Some(second)) = (
        number(0, 4),
        number(5, 7),
        number(8, 10),
        number(11, 13),
        number(14, 16),
        number(17, 19),
    ) else {
        return false;
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    if !separated || day == 0 || day > days || hour > 23 || minute > 59 || second > 60 {
        return false;
    }
    let mut rest = &bytes[19..];
    if let Some(fraction) = rest.strip_prefix(b".") {
        let digits = fraction
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        if digits == 0 {
            return false;
        }
        rest = &fraction[digits..];
    }
    match rest {
        b"Z" => true,
        [b'+' | b'-', h1, h2, b':', m1, m2] => {
            let digits = [h1, h2, m1, m2].iter().all(|byte| byte.is_ascii_digit());
            let value = |tens: u8, ones: u8| u32::from(tens - b'0') * 10 + u32::from(ones - b'0');
            digits && value(*h1, *h2) <= 23 && value(*m1, *m2) <= 59
        }
        _ => false,
    }
}

/// Every supported measurement of one observation: what it supplied, and
/// `{"state": "unobserved"}` for the rest.
pub fn expanded(document: &Value) -> Map<String, Value> {
    MEASUREMENTS
        .iter()
        .map(|(name, _)| {
            let given = document["measurements"]
                .get(*name)
                .cloned()
                .unwrap_or_else(|| json!({ "state": "unobserved" }));
            ((*name).to_owned(), given)
        })
        .collect()
}

/// Whether an observation observes that the harness executed: an observed
/// confirmation, whose value is `true`.
pub fn confirms_execution(document: &Value) -> bool {
    let confirmation = &document["measurements"]["executionConfirmation"];
    confirmation["state"] == "observed" && confirmation["value"] == true
}

/// Every supported measurement of a run, from its current observations
/// (`(observationId, document)`, in recorded order): per field, a `state` and
/// the `current` entries that supplied it, each with its observation's ID.
/// The state is `observed` if any current observation observed the field, else
/// `unknown` if one reported that, else `unobserved`. Several values are listed
/// side by side; nothing chooses between them.
pub fn summary<'a>(
    current: impl Iterator<Item = (&'a str, &'a Value)> + Clone,
) -> Map<String, Value> {
    MEASUREMENTS
        .iter()
        .map(|(name, _)| {
            let entries: Vec<Value> = current
                .clone()
                .filter_map(|(id, document)| {
                    let measurement = document["measurements"].get(*name)?;
                    if measurement["state"] == "unobserved" {
                        return None;
                    }
                    let mut entry = Map::new();
                    entry.insert("observationId".into(), id.into());
                    entry.extend(measurement.as_object()?.clone());
                    Some(Value::Object(entry))
                })
                .collect();
            let has = |state: &str| entries.iter().any(|entry| entry["state"] == state);
            let state = if has("observed") {
                "observed"
            } else if has("unknown") {
                "unknown"
            } else {
                "unobserved"
            };
            (
                (*name).to_owned(),
                json!({ "state": state, "current": entries }),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUN: &str = "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34";

    fn run() -> RunId {
        RunId::parse(RUN, "--run").unwrap()
    }

    fn envelope(measurements: Value) -> Value {
        json!({
            "schemaVersion": 1,
            "observationId": "o-1",
            "runId": RUN,
            "source": "an observer",
            "observedAt": "2026-10-01T09:30:00Z",
            "evidence": "what the observer saw",
            "measurements": measurements,
        })
    }

    fn check(value: Value) -> Result<Observation, Refusal> {
        Check {
            source: "/work/observation.json",
            expected: "--run names",
        }
        .envelope(value, &run())
    }

    #[test]
    fn every_supported_measurement_in_every_state_is_accepted() {
        let observed = json!({
            "executionConfirmation": { "state": "observed", "value": true },
            "ending": { "state": "observed", "value": "harness_exit" },
            "exit": { "state": "observed", "value": { "code": 0 } },
            "duration": { "state": "observed", "value": 12.5, "unit": "s" },
            "inputUsage": { "state": "observed", "value": 1200, "unit": "tokens" },
            "outputUsage": { "state": "observed", "value": 0, "unit": "tokens" },
            "totalUsage": { "state": "observed", "value": 0.42, "unit": "USD" },
            "acceptance": { "state": "observed", "value": "rejected" },
            "missedDefects": { "state": "observed", "value": [] },
            "falseFindings": { "state": "observed", "value": [
                { "id": "F1", "summary": "not a defect", "repairs": ["o-9"] },
                { "id": "F2" },
            ] },
            "downstreamRepair": { "state": "observed", "value": [
                { "id": "R1", "summary": "fixed the parser", "findings": ["F7"], "runId": RUN },
            ] },
            "humanTime": { "state": "observed", "value": 20, "unit": "min" },
            "humanInterventions": { "state": "observed", "value": 2 },
            "evidenceLinks": { "state": "observed", "value": ["https://example.com/pr/12"] },
            "choiceProbability": { "state": "observed", "value": 1 },
            "successProbability": { "state": "observed", "value": { "probability": 0.7, "calibration": "pilot-2026-10" } },
        });
        assert_eq!(
            observed.as_object().unwrap().len(),
            MEASUREMENTS.len(),
            "the fixture covers every field"
        );
        let observation = check(envelope(observed)).unwrap();
        assert!(observation.confirms_execution);
        assert_eq!(observation.id, "o-1");

        for state in ["unknown", "unobserved"] {
            let all: Map<String, Value> = MEASUREMENTS
                .iter()
                .map(|(name, _)| ((*name).to_owned(), json!({ "state": state })))
                .collect();
            let observation = check(envelope(Value::Object(all))).unwrap();
            assert!(!observation.confirms_execution, "{state}");
        }
        // Only the value `true` confirms, even in a document no import accepts.
        for value in [json!(false), Value::Null, json!("true")] {
            let document = envelope(
                json!({ "executionConfirmation": { "state": "observed", "value": value } }),
            );
            assert!(!confirms_execution(&document), "{value}");
        }
        check(envelope(json!({}))).unwrap();
        check(envelope(json!({
            "exit": { "state": "observed", "value": { "signal": "SIGKILL" } },
            "successProbability": { "state": "observed", "value": { "probability": 0, "uncalibrated": true } },
        })))
        .unwrap();
        let mut corrected = envelope(json!({}));
        corrected["supersedes"] = json!("o-0");
        assert_eq!(check(corrected).unwrap().supersedes.as_deref(), Some("o-0"));
    }

    #[test]
    fn each_malformed_observation_is_refused_at_its_location() {
        let base = envelope(json!({}));
        let with = |field: &str, value: Value| {
            let mut document = base.clone();
            document[field] = value;
            document
        };
        let without = |field: &str| {
            let mut document = base.clone();
            document.as_object_mut().unwrap().remove(field);
            document
        };
        let measured = |name: &str, measurement: Value| envelope(json!({ name: measurement }));
        let cases = [
            (json!([]), "observation"),
            (without("schemaVersion"), "observation.schemaVersion"),
            (
                with("schemaVersion", json!("1")),
                "observation.schemaVersion",
            ),
            (with("extra", json!(1)), "observation.extra"),
            (with("launch", json!({})), "observation.launch"),
            (without("observationId"), "observation.observationId"),
            (
                with("observationId", json!(" ")),
                "observation.observationId",
            ),
            (
                with("observationId", json!("x".repeat(1025))),
                "observation.observationId",
            ),
            (
                with("runId", json!(RUN.to_uppercase())),
                "observation.runId",
            ),
            (
                with("runId", json!("0192f0c4-7a1e-4b2c-9d3e-4f5a6b7c8d9e")),
                "observation.runId",
            ),
            (without("source"), "observation.source"),
            (with("source", json!("")), "observation.source"),
            (
                with("observedAt", json!("2026-10-01 09:30:00Z")),
                "observation.observedAt",
            ),
            (
                with("observedAt", json!("2026-02-29T09:30:00Z")),
                "observation.observedAt",
            ),
            (
                with("observedAt", json!("2026-10-01T24:00:00Z")),
                "observation.observedAt",
            ),
            (
                with("observedAt", json!("2026-10-01T09:30:00z")),
                "observation.observedAt",
            ),
            (
                with("observedAt", json!("2026-10-01T09:30:00")),
                "observation.observedAt",
            ),
            (
                with("observedAt", json!("2026-10-01T09:30:00.Z")),
                "observation.observedAt",
            ),
            (with("evidence", json!(3)), "observation.evidence"),
            (with("supersedes", json!("o-1")), "observation.supersedes"),
            (with("supersedes", json!(null)), "observation.supersedes"),
            (without("measurements"), "observation.measurements"),
            (with("measurements", json!([])), "observation.measurements"),
            (
                measured("humanWork", json!({ "state": "unknown" })),
                "observation.measurements.humanWork",
            ),
            (
                measured(
                    "ending",
                    json!({ "state": "observed", "value": "finished" }),
                ),
                "observation.measurements.ending.value",
            ),
            (
                measured("ending", json!({ "state": "observed", "value": 1 })),
                "observation.measurements.ending.value",
            ),
            (
                measured("duration", json!("12s")),
                "observation.measurements.duration",
            ),
            (
                measured("duration", json!({ "value": 1, "unit": "s" })),
                "observation.measurements.duration.state",
            ),
            (
                measured(
                    "duration",
                    json!({ "state": "seen", "value": 1, "unit": "s" }),
                ),
                "observation.measurements.duration.state",
            ),
            (
                measured("duration", json!({ "state": "observed", "unit": "s" })),
                "observation.measurements.duration.value",
            ),
            (
                measured("duration", json!({ "state": "observed", "value": 1 })),
                "observation.measurements.duration.unit",
            ),
            (
                measured(
                    "duration",
                    json!({ "state": "observed", "value": 1, "unit": "days" }),
                ),
                "observation.measurements.duration.unit",
            ),
            (
                measured(
                    "duration",
                    json!({ "state": "observed", "value": -1, "unit": "s" }),
                ),
                "observation.measurements.duration.value",
            ),
            (
                measured("duration", json!({ "state": "unknown", "value": 0 })),
                "observation.measurements.duration.value",
            ),
            (
                measured("duration", json!({ "state": "unobserved", "unit": "s" })),
                "observation.measurements.duration.unit",
            ),
            (
                measured(
                    "duration",
                    json!({ "state": "observed", "value": 1, "unit": "s", "note": "" }),
                ),
                "observation.measurements.duration.note",
            ),
            (
                measured(
                    "totalUsage",
                    json!({ "state": "observed", "value": 1, "unit": " " }),
                ),
                "observation.measurements.totalUsage.unit",
            ),
            (
                measured(
                    "acceptance",
                    json!({ "state": "observed", "value": "accepted", "unit": "votes" }),
                ),
                "observation.measurements.acceptance.unit",
            ),
            (
                measured(
                    "acceptance",
                    json!({ "state": "observed", "value": "unknown" }),
                ),
                "observation.measurements.acceptance.value",
            ),
            (
                measured(
                    "executionConfirmation",
                    json!({ "state": "observed", "value": false }),
                ),
                "observation.measurements.executionConfirmation.value",
            ),
            (
                measured(
                    "exit",
                    json!({ "state": "observed", "value": { "code": 256 } }),
                ),
                "observation.measurements.exit.value.code",
            ),
            (
                measured(
                    "exit",
                    json!({ "state": "observed", "value": { "code": 1, "signal": "SIGTERM" } }),
                ),
                "observation.measurements.exit.value",
            ),
            (
                measured(
                    "exit",
                    json!({ "state": "observed", "value": { "signal": "15" } }),
                ),
                "observation.measurements.exit.value.signal",
            ),
            (
                measured(
                    "humanInterventions",
                    json!({ "state": "observed", "value": 1.5 }),
                ),
                "observation.measurements.humanInterventions.value",
            ),
            (
                measured(
                    "missedDefects",
                    json!({ "state": "observed", "value": [{ "summary": "x" }] }),
                ),
                "observation.measurements.missedDefects.value[0].id",
            ),
            (
                measured(
                    "missedDefects",
                    json!({ "state": "observed", "value": [{ "id": "F1" }, { "id": "F1" }] }),
                ),
                "observation.measurements.missedDefects.value[1].id",
            ),
            (
                measured(
                    "falseFindings",
                    json!({ "state": "observed", "value": [{ "id": "F1", "repairs": [""] }] }),
                ),
                "observation.measurements.falseFindings.value[0].repairs[0]",
            ),
            (
                measured(
                    "downstreamRepair",
                    json!({ "state": "observed", "value": [{ "id": "R1", "runId": "r" }] }),
                ),
                "observation.measurements.downstreamRepair.value[0].runId",
            ),
            (
                measured(
                    "evidenceLinks",
                    json!({ "state": "observed", "value": ["a", ""] }),
                ),
                "observation.measurements.evidenceLinks.value[1]",
            ),
            (
                measured(
                    "choiceProbability",
                    json!({ "state": "observed", "value": 1.5 }),
                ),
                "observation.measurements.choiceProbability.value",
            ),
            (
                measured(
                    "successProbability",
                    json!({ "state": "observed", "value": { "probability": 0.5 } }),
                ),
                "observation.measurements.successProbability.value",
            ),
            (
                measured(
                    "successProbability",
                    json!({ "state": "observed", "value": { "probability": 0.5, "uncalibrated": false } }),
                ),
                "observation.measurements.successProbability.value.uncalibrated",
            ),
            (
                measured(
                    "successProbability",
                    json!({ "state": "observed", "value": { "probability": 0.5, "calibration": "c", "uncalibrated": true } }),
                ),
                "observation.measurements.successProbability.value",
            ),
        ];
        for (value, location) in cases {
            let refusal = check(value.clone()).unwrap_err();
            assert_eq!(
                refusal.code, "observation_invalid",
                "{value}: {}",
                refusal.message
            );
            assert_eq!(
                refusal.location.as_deref(),
                Some(location),
                "{value}: {}",
                refusal.message
            );
            assert_eq!(refusal.input.as_deref(), Some("--file"), "{value}");
        }
    }

    #[test]
    fn a_later_version_is_unsupported_and_a_launch_field_says_why() {
        let refusal = check(json!({ "schemaVersion": 2, "launch": {} })).unwrap_err();
        assert_eq!(refusal.code, "unsupported_version");
        assert_eq!(
            refusal.location.as_deref(),
            Some("observation.schemaVersion")
        );

        let mut document = envelope(json!({}));
        document["launch"] = json!({ "kind": "other" });
        let refusal = check(document).unwrap_err();
        assert!(
            refusal
                .message
                .contains("no observation can supply or change"),
            "{}",
            refusal.message
        );
    }

    #[test]
    fn only_real_rfc_3339_date_times_are_taken() {
        for good in [
            "2026-10-01T09:30:00Z",
            "2024-02-29T23:59:60Z",
            "2026-10-01T09:30:00.123456Z",
            "2026-10-01T19:30:00+10:00",
            "2026-10-01T00:00:00-23:59",
        ] {
            assert!(is_date_time(good), "{good}");
        }
        for bad in [
            "",
            "2026-10-01",
            "2026-10-01T09:30Z",
            "2026-13-01T09:30:00Z",
            "2026-00-01T09:30:00Z",
            "2026-04-31T09:30:00Z",
            "1900-02-29T09:30:00Z",
            "2026-10-01T09:60:00Z",
            "2026-10-01T09:30:61Z",
            "2026-10-01T09:30:00+24:00",
            "2026-10-01T09:30:00+1000",
            "2026-10-01T09:30:00Zjunk",
            "٢٠٢٦-10-01T09:30:00Z",
        ] {
            assert!(!is_date_time(bad), "{bad}");
        }
    }

    #[test]
    fn the_summary_lists_current_values_side_by_side_and_never_zero_fills() {
        let first = json!({ "measurements": {
            "acceptance": { "state": "observed", "value": "accepted" },
            "duration": { "state": "unknown" },
            "exit": { "state": "unobserved" },
        } });
        let second = json!({ "measurements": {
            "acceptance": { "state": "observed", "value": "rejected" },
            "duration": { "state": "observed", "value": 3, "unit": "s" },
        } });
        let current = [("a", &first), ("b", &second)];
        let both = summary(current.iter().map(|(id, doc)| (*id, *doc)));
        assert_eq!(both.len(), MEASUREMENTS.len());
        assert_eq!(
            both["acceptance"],
            json!({ "state": "observed", "current": [
                { "observationId": "a", "state": "observed", "value": "accepted" },
                { "observationId": "b", "state": "observed", "value": "rejected" },
            ] })
        );
        assert_eq!(both["duration"]["state"], "observed");
        assert_eq!(both["duration"]["current"].as_array().unwrap().len(), 2);
        assert_eq!(
            both["exit"],
            json!({ "state": "unobserved", "current": [] })
        );
        assert_eq!(
            both["totalUsage"],
            json!({ "state": "unobserved", "current": [] })
        );

        let only_unknown = [("a", &first)];
        let one = summary(only_unknown.iter().map(|(id, doc)| (*id, *doc)));
        assert_eq!(one["duration"]["state"], "unknown");

        let expanded = expanded(&second);
        assert_eq!(expanded.len(), MEASUREMENTS.len());
        assert_eq!(expanded["exit"], json!({ "state": "unobserved" }));
        assert_eq!(expanded["duration"]["value"], 3);
    }

    fn ended<'a>(ending: &'a str, code: Option<i32>, signal: Option<&'a str>) -> Observation {
        end_observation(&RunEnd {
            run_id: &run(),
            ending,
            code,
            signal,
            duration: std::time::Duration::from_millis(1500),
        })
        .unwrap()
    }

    #[test]
    fn the_end_observation_is_an_ordinary_validated_observation() {
        let observation = ended("exit_signal", Some(0), None);
        assert!(observation.confirms_execution);
        assert_eq!(observation.id, format!("harness-dispatch-end-{RUN}"));
        let document = &observation.document;
        assert_eq!(document["source"], "harness-dispatch");
        let measured = &document["measurements"];
        assert_eq!(measured.as_object().unwrap().len(), 4);
        assert_eq!(measured["ending"]["value"], "exit_signal");
        assert_eq!(measured["exit"]["value"], json!({ "code": 0 }));
        assert_eq!(
            measured["duration"],
            json!({ "state": "observed", "value": 1500, "unit": "ms" })
        );
        // What the store holds must read back as the observation it is.
        stored(document, &observation.id, &run(), None).unwrap();
    }

    #[test]
    fn the_end_observation_reports_a_signal_and_what_it_could_not_name() {
        let died = ended("harness_exit", None, Some("SIGKILL"));
        assert_eq!(
            died.document["measurements"]["exit"]["value"],
            json!({ "signal": "SIGKILL" })
        );
        let neither = ended("cancelled", None, None);
        assert_eq!(
            neither.document["measurements"]["exit"],
            json!({ "state": "unknown" })
        );
        for ending in ENDINGS {
            assert_eq!(
                ended(ending, Some(3), None).document["measurements"]["ending"]["value"],
                ending
            );
        }
    }

    #[test]
    fn now_is_an_rfc_3339_date_time() {
        let stamp = now();
        assert!(is_date_time(&stamp), "{stamp}");
        assert_eq!(stamp.len(), "2026-10-01T09:30:00.000Z".len(), "{stamp}");
    }
}
