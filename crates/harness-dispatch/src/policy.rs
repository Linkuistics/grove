//! Validate a policy snapshot, then select through its static routes or the
//! caller's explicit choice, or judge what its computed `select` produced
//! (`docs/specs/harness-selection-and-execution.md`, *Policy and joint choice*).
//!
//! The worker hands over the entry's `policy` export as JSON, with any value
//! JSON cannot carry replaced by a `{"$harnessDispatch": "<type>"}` marker, and
//! later, for a `select` policy, the value `select` produced, marked the same
//! way. This module is the one validator for both: every refusal names the
//! location it found, in the form `policy.catalog[1].provider` or
//! `result.candidateId`, and nothing invalid is ever repaired into something
//! that passes. A result names a candidate of the snapshot already validated;
//! it cannot supply one of its own, or any word of argv.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::refusal::{Refusal, Stage, EXIT_REFUSED};
use crate::worker::Produced;

const TOP_LEVEL: [&str; 6] = [
    "schemaVersion",
    "version",
    "catalog",
    "routes",
    "select",
    "loadContext",
];
const CANDIDATE_FIELDS: [&str; 6] = ["id", "provider", "model", "effort", "program", "args"];
const SLOTS: [&str; 7] = [
    "prompt", "kind", "taskFile", "taskId", "model", "effort", "runId",
];

#[derive(Debug)]
pub struct Policy {
    pub version: String,
    pub catalog: Vec<Candidate>,
    pub form: Form,
}

/// How a valid policy selects: exactly one of the two.
#[derive(Debug)]
pub enum Form {
    /// The static table from kind to candidate ID, every target checked.
    Routes(BTreeMap<String, String>),
    /// A `select` callback, which only the worker can call.
    Select,
}

#[derive(Debug)]
pub struct Candidate {
    pub id: String,
    pub provider: String,
    pub model: String,
    pub effort: String,
    pub program: String,
    /// Every entry checked, with `prompt` exactly once.
    pub args: Vec<Argument>,
}

#[derive(Debug)]
pub enum Argument {
    Literal(String),
    Slot(Slot),
}

/// A caller input, catalog value or run identity that fills one whole
/// argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    Prompt,
    Kind,
    TaskFile,
    TaskId,
    Model,
    Effort,
    RunId,
}

impl Slot {
    fn named(name: &str) -> Option<Slot> {
        Some(match name {
            "prompt" => Slot::Prompt,
            "kind" => Slot::Kind,
            "taskFile" => Slot::TaskFile,
            "taskId" => Slot::TaskId,
            "model" => Slot::Model,
            "effort" => Slot::Effort,
            "runId" => Slot::RunId,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Slot::Prompt => "prompt",
            Slot::Kind => "kind",
            Slot::TaskFile => "taskFile",
            Slot::TaskId => "taskId",
            Slot::Model => "model",
            Slot::Effort => "effort",
            Slot::RunId => "runId",
        }
    }
}

impl Candidate {
    /// The catalog entry as the policy configured it, slots as slot objects:
    /// the selected catalog values a run records.
    pub fn to_json(&self) -> Value {
        let args: Vec<Value> = self
            .args
            .iter()
            .map(|argument| match argument {
                Argument::Literal(literal) => Value::String(literal.clone()),
                Argument::Slot(slot) => serde_json::json!({ "slot": slot.name() }),
            })
            .collect();
        serde_json::json!({
            "id": self.id,
            "provider": self.provider,
            "model": self.model,
            "effort": self.effort,
            "program": self.program,
            "args": args,
        })
    }
}

/// The catalog index of the candidate a valid policy chose, why, and by what.
#[derive(Debug)]
pub struct Selection {
    pub index: usize,
    pub reason: String,
    pub by: SelectedBy,
}

/// What made the selection: the routes table, the caller's explicit choice,
/// which a routes policy accepts without consulting its table, or the policy's
/// `select`, which also decides on any explicit choice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectedBy {
    Route,
    ExplicitChoice,
    Select,
}

impl SelectedBy {
    /// The stable name inspection and the run record report.
    pub fn as_str(self) -> &'static str {
        match self {
            SelectedBy::Route => "route",
            SelectedBy::ExplicitChoice => "explicit_choice",
            SelectedBy::Select => "select",
        }
    }

    /// The policy form that made it: `routes` or `select`.
    pub fn form(self) -> &'static str {
        match self {
            SelectedBy::Route | SelectedBy::ExplicitChoice => "routes",
            SelectedBy::Select => "select",
        }
    }
}

/// Validates against one source file, so each refusal can name it.
pub struct Validator<'a> {
    source: &'a str,
}

impl<'a> Validator<'a> {
    pub fn new(source: &'a str) -> Self {
        Validator { source }
    }

    fn invalid(&self, location: &str, message: impl Into<String>) -> Refusal {
        Refusal::new(
            "policy_invalid",
            Stage::Validation,
            EXIT_REFUSED,
            message,
            format!(
                "correct {location} in {}; the types in harness-dispatch/sdk describe a \
                 version-1 policy",
                self.source
            ),
        )
        .source(self.source)
        .location(location)
    }

    fn unsupported(&self, field: &str) -> Refusal {
        Refusal::new(
            "unsupported_form",
            Stage::Validation,
            EXIT_REFUSED,
            format!("`{field}` is not supported by this release of harness-dispatch"),
            format!(
                "remove `{field}` from {}: this release evaluates `routes` or `select` without \
                 loaded context",
                self.source
            ),
        )
        .source(self.source)
        .location(format!("policy.{field}"))
    }

    pub fn validate(&self, snapshot: &Value) -> Result<Policy, Refusal> {
        let policy = self.object(snapshot, "policy")?;

        // The version comes first: a later schema's fields are not unknown to it.
        let schema = policy
            .get("schemaVersion")
            .ok_or_else(|| self.invalid("policy.schemaVersion", "`schemaVersion` is missing"))?;
        match schema.as_u64() {
            Some(1) => {}
            Some(other) => {
                return Err(Refusal::new(
                    "unsupported_version",
                    Stage::Validation,
                    EXIT_REFUSED,
                    format!("schemaVersion {other} is not supported; this release reads schemaVersion 1"),
                    format!("write schemaVersion 1 in {}, or upgrade harness-dispatch", self.source),
                )
                .source(self.source)
                .location("policy.schemaVersion"));
            }
            None => {
                return Err(self.invalid(
                    "policy.schemaVersion",
                    format!(
                        "`schemaVersion` must be the number 1, found {}",
                        describe(schema)
                    ),
                ))
            }
        }
        if let Some(unknown) = policy.keys().find(|key| !TOP_LEVEL.contains(&key.as_str())) {
            return Err(self.invalid(
                &format!("policy.{unknown}"),
                format!("unknown field `{unknown}`"),
            ));
        }

        let present = |field: &str| policy.get(field).is_some_and(|value| !value.is_null());
        let computed =
            match (present("routes"), present("select")) {
                (true, false) => false,
                (false, true) => true,
                (both, _) => return Err(self.invalid(
                    "policy",
                    if both {
                        "a policy has exactly one of `routes` or `select`, and this one has both"
                    } else {
                        "a policy has exactly one of `routes` or `select`, and this one has neither"
                    },
                )),
            };
        if present("loadContext") {
            return Err(self.unsupported("loadContext"));
        }

        let version = self.string(policy, "version", "policy.version")?;
        if version.trim().is_empty() {
            return Err(self.invalid("policy.version", "`version` must not be blank"));
        }
        let catalog = self.catalog(policy.get("catalog"))?;
        let form = if computed {
            let select = &policy["select"];
            if !is_function(select) {
                return Err(self.invalid(
                    "policy.select",
                    format!("`select` must be a function, found {}", describe(select)),
                ));
            }
            Form::Select
        } else {
            Form::Routes(self.routes(&policy["routes"], &catalog)?)
        };
        Ok(Policy {
            version,
            catalog,
            form,
        })
    }

    fn catalog(&self, catalog: Option<&Value>) -> Result<Vec<Candidate>, Refusal> {
        let catalog =
            catalog.ok_or_else(|| self.invalid("policy.catalog", "`catalog` is missing"))?;
        let catalog = catalog.as_array().ok_or_else(|| {
            self.invalid(
                "policy.catalog",
                format!(
                    "`catalog` must be an array of candidates, found {}",
                    describe(catalog)
                ),
            )
        })?;
        let mut candidates: Vec<Candidate> = Vec::with_capacity(catalog.len());
        for (index, entry) in catalog.iter().enumerate() {
            let at = format!("policy.catalog[{index}]");
            let candidate = self.candidate(entry, &at)?;
            if let Some(first) = candidates.iter().position(|seen| seen.id == candidate.id) {
                return Err(self.invalid(
                    &format!("{at}.id"),
                    format!(
                        "candidate ID {:?} is already used by policy.catalog[{first}]",
                        candidate.id
                    ),
                ));
            }
            candidates.push(candidate);
        }
        Ok(candidates)
    }

    fn candidate(&self, entry: &Value, at: &str) -> Result<Candidate, Refusal> {
        let fields = self.object(entry, at)?;
        if let Some(unknown) = fields
            .keys()
            .find(|key| !CANDIDATE_FIELDS.contains(&key.as_str()))
        {
            return Err(self.invalid(
                &format!("{at}.{unknown}"),
                format!("unknown candidate field `{unknown}`"),
            ));
        }
        let nonempty = |field: &str| {
            let location = format!("{at}.{field}");
            let value = self.string(fields, field, &location)?;
            if value.is_empty() {
                return Err(self.invalid(&location, format!("`{field}` must not be empty")));
            }
            Ok(value)
        };
        // The program is argv[0], and the `model` and `effort` slots copy
        // their values into argv, which no NUL can cross.
        let argv_word = |field: &str| {
            let value = nonempty(field)?;
            self.no_nul(&value, &format!("{at}.{field}"), &format!("`{field}`"))?;
            Ok::<_, Refusal>(value)
        };
        let candidate = Candidate {
            id: nonempty("id")?,
            provider: nonempty("provider")?,
            model: argv_word("model")?,
            effort: argv_word("effort")?,
            program: argv_word("program")?,
            args: Vec::new(),
        };
        let args_at = format!("{at}.args");
        let args = fields
            .get("args")
            .ok_or_else(|| self.invalid(&args_at, "`args` is missing"))?;
        let args = args.as_array().ok_or_else(|| {
            self.invalid(
                &args_at,
                format!("`args` must be an array, found {}", describe(args)),
            )
        })?;
        let args: Vec<Argument> = args
            .iter()
            .enumerate()
            .map(|(index, arg)| self.argument(arg, &format!("{args_at}[{index}]")))
            .collect::<Result<_, _>>()?;
        let prompts = args
            .iter()
            .filter(|arg| matches!(arg, Argument::Slot(Slot::Prompt)))
            .count();
        if prompts != 1 {
            return Err(self.invalid(
                &args_at,
                format!(
                    "the `prompt` slot must fill exactly one argument, and `args` has it {prompts} \
                     times"
                ),
            ));
        }
        Ok(Candidate { args, ..candidate })
    }

    fn argument(&self, arg: &Value, at: &str) -> Result<Argument, Refusal> {
        if let Some(literal) = arg.as_str() {
            self.no_nul(literal, at, "a literal argument")?;
            return Ok(Argument::Literal(literal.to_owned()));
        }
        let expected =
            "an argument is a literal string or a slot object such as { slot: \"prompt\" }";
        let slot = match arg.as_object() {
            Some(object) if object.len() == 1 && object.contains_key("slot") => &object["slot"],
            Some(object) if !is_marker(arg) => {
                let extra = object.keys().find(|key| *key != "slot");
                return Err(match extra {
                    Some(extra) => self.invalid(
                        &format!("{at}.{extra}"),
                        format!("unknown argument field `{extra}`; {expected}"),
                    ),
                    None => self.invalid(
                        &format!("{at}.slot"),
                        format!("`slot` is missing; {expected}"),
                    ),
                });
            }
            _ => {
                return Err(self.invalid(at, format!("{expected}, found {}", describe(arg))));
            }
        };
        match slot.as_str().and_then(Slot::named) {
            Some(slot) => Ok(Argument::Slot(slot)),
            None => Err(self.invalid(
                &format!("{at}.slot"),
                format!(
                    "`slot` must be one of {}, found {}",
                    SLOTS.join(", "),
                    describe_value(slot)
                ),
            )),
        }
    }

    fn routes(
        &self,
        routes: &Value,
        catalog: &[Candidate],
    ) -> Result<BTreeMap<String, String>, Refusal> {
        let routes = self.object(routes, "policy.routes")?;
        let mut table = BTreeMap::new();
        for (kind, target) in routes {
            let at = format!("policy.routes[{}]", Value::String(kind.clone()));
            let id = target.as_str().ok_or_else(|| {
                self.invalid(
                    &at,
                    format!("a route names a candidate ID, found {}", describe(target)),
                )
            })?;
            if !catalog.iter().any(|candidate| candidate.id == id) {
                return Err(self.invalid(
                    &at,
                    format!(
                        "the route for kind {kind:?} names {id:?}, which is not in the catalog"
                    ),
                ));
            }
            table.insert(kind.clone(), id.to_owned());
        }
        Ok(table)
    }

    /// Refuse a NUL in a string that can become a word of argv. Exec cannot
    /// carry it, so it is invalid policy in every candidate, selected or not,
    /// and is refused here rather than after a handoff has been recorded.
    fn no_nul(&self, value: &str, at: &str, what: &str) -> Result<(), Refusal> {
        match value.find('\0') {
            Some(offset) => Err(self.invalid(
                at,
                format!(
                    "{what} contains a NUL character at byte {offset}, which no argument can carry"
                ),
            )),
            None => Ok(()),
        }
    }

    fn object<'v>(&self, value: &'v Value, at: &str) -> Result<&'v Map<String, Value>, Refusal> {
        match value.as_object() {
            Some(object) if !is_marker(value) => Ok(object),
            _ => Err(self.invalid(at, format!("expected an object, found {}", describe(value)))),
        }
    }

    fn string(
        &self,
        object: &Map<String, Value>,
        field: &str,
        at: &str,
    ) -> Result<String, Refusal> {
        let value = object
            .get(field)
            .ok_or_else(|| self.invalid(at, format!("`{field}` is missing")))?;
        value.as_str().map(str::to_owned).ok_or_else(|| {
            self.invalid(
                at,
                format!("`{field}` must be a string, found {}", describe(value)),
            )
        })
    }
}

/// Select from a valid routes policy: the caller's explicit choice when there
/// is one, and otherwise the route for `kind`. Either way, nothing the caller
/// or the table did not name is ever substituted.
pub fn by_routes(
    policy: &Policy,
    routes: &BTreeMap<String, String>,
    kind: &str,
    choice: Option<&str>,
    source: &str,
) -> Result<Selection, Refusal> {
    match choice {
        Some(choice) => chosen(policy, choice, source),
        None => route(policy, routes, kind, source),
    }
}

/// The catalog index of the candidate an explicit choice names. Under either
/// form, an ID the catalog lacks refuses, before any `select` runs; it is never
/// read as a request for some other candidate.
pub fn configured(policy: &Policy, choice: &str, source: &str) -> Result<usize, Refusal> {
    policy
        .catalog
        .iter()
        .position(|candidate| candidate.id == choice)
        .ok_or_else(|| {
            Refusal::new(
                "unknown_choice",
                Stage::Selection,
                EXIT_REFUSED,
                format!("--choice {choice:?} names no candidate in the catalog of {source}"),
                format!(
                    "pass --choice with one of the configured candidate IDs ({}), or omit it to \
                     select without one; harness-dispatch never substitutes another candidate",
                    ids(policy)
                ),
            )
            .input(format!("--choice {choice}"))
            .source(source)
            .location("policy.catalog")
        })
}

/// The configured candidate IDs, quoted, for a remedy to list.
fn ids(policy: &Policy) -> String {
    let ids: Vec<String> = policy
        .catalog
        .iter()
        .map(|candidate| format!("{:?}", candidate.id))
        .collect();
    ids.join(", ")
}

/// An explicit choice under routes names any configured candidate, including
/// one for a kind the table does not route, and the table cannot refuse it
/// (spec, *Policy and joint choice*).
fn chosen(policy: &Policy, choice: &str, source: &str) -> Result<Selection, Refusal> {
    let index = configured(policy, choice, source)?;
    Ok(Selection {
        index,
        reason: format!(
            "the explicit choice --choice {choice:?} names a configured candidate; routes are not \
             consulted"
        ),
        by: SelectedBy::ExplicitChoice,
    })
}

/// Resolve `kind` through a valid policy's routes. A kind the table does not
/// name refuses; no default candidate is ever substituted.
fn route(
    policy: &Policy,
    routes: &BTreeMap<String, String>,
    kind: &str,
    source: &str,
) -> Result<Selection, Refusal> {
    let id = routes.get(kind).ok_or_else(|| {
        Refusal::new(
            "incomplete_mapping",
            Stage::Selection,
            EXIT_REFUSED,
            format!("the routes in {source} name no candidate for kind {kind:?}"),
            format!(
                "add a route {} to a candidate ID in {source}, or name one configured candidate \
                 with --choice ID; harness-dispatch never substitutes a default candidate",
                Value::String(kind.to_owned())
            ),
        )
        .input(format!("--kind {kind}"))
        .source(source)
        .location("policy.routes")
    })?;
    let index = policy
        .catalog
        .iter()
        .position(|candidate| &candidate.id == id)
        .expect("validation checked every route names a catalog candidate");
    Ok(Selection {
        index,
        reason: format!(
            "routes[{}] names candidate {id:?}",
            Value::String(kind.to_owned())
        ),
        by: SelectedBy::Route,
    })
}

const SELECTED_FIELDS: [&str; 3] = ["status", "candidateId", "reason"];
const REFUSED_FIELDS: [&str; 4] = ["status", "code", "message", "remedy"];

/// Judge what a valid `select` policy's callback produced, against the catalog
/// validated before it ran. A candidate it selects must be in that catalog and
/// come with a nonblank reason; with an explicit choice it must be that choice.
/// A refusal it returns must say what and why. Every other value refuses, each
/// kind with its own code, and nothing is ever substituted for it.
pub fn computed(
    policy: &Policy,
    produced: Produced,
    kind: &str,
    choice: Option<&str>,
    source: &str,
) -> Result<Selection, Refusal> {
    let shape = format!(
        "return {{ status: \"selected\", candidateId, reason }} or {{ status: \"refused\", code, \
         message, remedy }} from select in {source}; the types in harness-dispatch/sdk describe \
         both"
    );
    let refuse = |code: &'static str, message: String, remedy: String, location: &str| {
        Refusal::new(code, Stage::Selection, EXIT_REFUSED, message, remedy)
            .source(source)
            .location(location)
    };
    let malformed = |location: &str, message: String| {
        refuse("selection_malformed", message, shape.clone(), location)
    };

    let result = match produced {
        Produced::Result(result) => result,
        Produced::Threw { name, message } => {
            return Err(refuse(
                "selection_threw",
                format!("select in {source} threw, or its promise rejected: {name}: {message}"),
                format!(
                    "fix select in {source} so that it returns a result on every path; to \
                     decline, return {{ status: \"refused\", code, message, remedy }} rather \
                     than throwing"
                ),
                "policy.select",
            ))
        }
        Produced::Unsettled => {
            return Err(refuse(
                "selection_unsettled",
                format!(
                    "select in {source} returned a promise that never settled: it was still \
                     pending when nothing was left running that could settle it"
                ),
                "make every path through select resolve or reject its promise; an await on \
                 something that will never complete leaves it pending"
                    .to_owned(),
                "policy.select",
            ))
        }
        Produced::Unserializable { name, message } => {
            return Err(malformed(
                "result",
                format!("the result of select cannot be serialized: {name}: {message}"),
            ))
        }
    };

    if result.is_null() {
        return Err(refuse(
            "selection_abstained",
            format!(
                "select in {source} returned no result (undefined or null), so it selected \
                 nothing"
            ),
            format!("{shape}; harness-dispatch never picks a candidate for a policy that abstains"),
            "result",
        ));
    }
    let fields = match result.as_object() {
        Some(fields) if !is_marker(&result) => fields,
        _ => {
            return Err(malformed(
                "result",
                format!(
                    "a selection result must be an object, found {}",
                    describe(&result)
                ),
            ))
        }
    };
    let status = fields
        .get("status")
        .ok_or_else(|| malformed("result.status", "`status` is missing".to_owned()))?;
    let allowed: &[&str] = match status.as_str() {
        Some("selected") => &SELECTED_FIELDS,
        Some("refused") => &REFUSED_FIELDS,
        _ => {
            return Err(malformed(
                "result.status",
                format!(
                    "`status` must be \"selected\" or \"refused\", found {}",
                    describe_value(status)
                ),
            ))
        }
    };
    if let Some(unknown) = fields.keys().find(|key| !allowed.contains(&key.as_str())) {
        return Err(malformed(
            &format!("result.{unknown}"),
            format!(
                "unknown field `{unknown}`: a {} result has only {}, and a result cannot supply a \
                 program or arguments",
                status.as_str().unwrap_or_default(),
                allowed.join(", ")
            ),
        ));
    }
    let text = |field: &str| {
        let location = format!("result.{field}");
        let value = fields
            .get(field)
            .ok_or_else(|| malformed(&location, format!("`{field}` is missing")))?;
        let text = value.as_str().ok_or_else(|| {
            malformed(
                &location,
                format!("`{field}` must be a string, found {}", describe(value)),
            )
        })?;
        Ok::<_, Refusal>((text.to_owned(), location))
    };
    let nonblank = |field: &str| {
        let (text, location) = text(field)?;
        if text.trim().is_empty() {
            return Err(malformed(&location, format!("`{field}` must not be blank")));
        }
        Ok(text)
    };

    if status == "refused" {
        let (code, message, remedy) =
            (nonblank("code")?, nonblank("message")?, nonblank("remedy")?);
        let input = match choice {
            Some(choice) => format!("--choice {choice}"),
            None => format!("--kind {kind}"),
        };
        return Err(Refusal::new(
            "policy_refused",
            Stage::Selection,
            EXIT_REFUSED,
            format!("the policy {source} refused the selection: {message}"),
            remedy,
        )
        .policy_code(code)
        .input(input)
        .source(source));
    }

    let (id, _) = text("candidateId")?;
    let reason = nonblank("reason")?;
    if let Some(choice) = choice.filter(|choice| *choice != id) {
        return Err(refuse(
            "explicit_choice_mismatch",
            format!(
                "--choice {choice:?} was given, and select in {source} selected {id:?} instead \
                 (its reason: {reason})"
            ),
            format!(
                "a policy accepts an explicit choice by selecting that same ID, or refuses it with \
                 {{ status: \"refused\", code, message, remedy }}; harness-dispatch never runs \
                 another candidate in its place. Omit --choice to let {source} choose"
            ),
            "result.candidateId",
        )
        .input(format!("--choice {choice}")));
    }
    let index = policy
        .catalog
        .iter()
        .position(|candidate| candidate.id == id)
        .ok_or_else(|| {
            refuse(
                "unknown_candidate",
                format!("select in {source} selected {id:?}, which is not in its catalog"),
                format!(
                    "select a configured candidate ID ({}), or add {id:?} to the catalog; a \
                     result names a candidate and cannot supply one",
                    ids(policy)
                ),
                "result.candidateId",
            )
        })?;
    Ok(Selection {
        index,
        reason,
        by: SelectedBy::Select,
    })
}

/// Whether a snapshot value is the marker for a JavaScript function.
fn is_function(value: &Value) -> bool {
    is_marker(value) && value["$harnessDispatch"] == "function"
}

fn is_marker(value: &Value) -> bool {
    value
        .as_object()
        .is_some_and(|object| object.len() == 1 && object.contains_key("$harnessDispatch"))
}

/// A short description of a JSON value's type, for "found …" messages. A marker
/// describes the JavaScript value the worker could not serialize.
fn describe(value: &Value) -> String {
    match value {
        Value::Null => "null".to_owned(),
        Value::Bool(_) => "a boolean".to_owned(),
        Value::Number(_) => "a number".to_owned(),
        Value::String(_) => "a string".to_owned(),
        Value::Array(_) => "an array".to_owned(),
        Value::Object(object) => match object.get("$harnessDispatch").and_then(Value::as_str) {
            Some(kind) if object.len() == 1 => format!("a {kind}"),
            _ => "an object".to_owned(),
        },
    }
}

/// `describe`, but quoting a string so a wrong slot name is visible.
fn describe_value(value: &Value) -> String {
    match value {
        Value::String(text) => format!("{text:?}"),
        other => describe(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid() -> Value {
        json!({
            "schemaVersion": 1,
            "version": "v1",
            "catalog": [
                {"id": "a", "provider": "p", "model": "m", "effort": "e", "program": "x", "args": [{"slot": "prompt"}]},
            ],
            "routes": {"impl": "a"},
        })
    }

    /// Select through a valid routes policy's own table.
    fn select(
        policy: &Policy,
        kind: &str,
        choice: Option<&str>,
        source: &str,
    ) -> Result<Selection, Refusal> {
        let Form::Routes(routes) = &policy.form else {
            panic!("a routes policy");
        };
        by_routes(policy, routes, kind, choice, source)
    }

    /// `valid()` with `select` in place of its routes, and a second candidate.
    fn computing() -> Policy {
        let mut policy = valid();
        policy.as_object_mut().unwrap().remove("routes");
        policy["select"] = json!({"$harnessDispatch": "function"});
        policy["catalog"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id": "b", "provider": "q", "model": "m", "effort": "e", "program": "x", "args": [{"slot": "prompt"}]}));
        Validator::new("/p.ts").validate(&policy).unwrap()
    }

    fn judged(result: Value, choice: Option<&str>) -> Result<Selection, Refusal> {
        computed(
            &computing(),
            Produced::Result(result),
            "impl",
            choice,
            "/p.ts",
        )
    }

    #[test]
    fn a_select_policy_is_valid_only_with_a_function() {
        assert!(matches!(computing().form, Form::Select));
        let mut policy = valid();
        policy.as_object_mut().unwrap().remove("routes");
        for (select, found) in [
            (json!("choose"), "found a string"),
            (json!({"$harnessDispatch": "symbol"}), "found a symbol"),
            (json!({}), "found an object"),
        ] {
            policy["select"] = select;
            let refusal = Validator::new("/p.ts").validate(&policy).unwrap_err();
            assert_eq!(refusal.code, "policy_invalid");
            assert_eq!(refusal.location.as_deref(), Some("policy.select"));
            assert!(refusal.message.contains(found), "{}", refusal.message);
        }
    }

    #[test]
    fn a_selected_result_names_a_catalog_candidate_with_a_reason() {
        let selection = judged(
            json!({"status": "selected", "candidateId": "b", "reason": "b fits"}),
            None,
        )
        .unwrap();
        assert_eq!(selection.index, 1);
        assert_eq!(selection.reason, "b fits");
        assert_eq!(selection.by, SelectedBy::Select);
        // The same ID accepts an explicit choice.
        let accepted = judged(
            json!({"status": "selected", "candidateId": "b", "reason": "b fits"}),
            Some("b"),
        )
        .unwrap();
        assert_eq!(accepted.index, 1);
    }

    #[test]
    fn every_malformed_result_names_where_it_is_wrong() {
        let selected = |extra: Value| {
            let mut result = json!({"status": "selected", "candidateId": "a", "reason": "r"});
            for (key, value) in extra.as_object().unwrap() {
                if value.is_null() {
                    result.as_object_mut().unwrap().remove(key);
                } else {
                    result[key] = value.clone();
                }
            }
            result
        };
        let cases = [
            (json!("a"), "result", "found a string"),
            (json!(["a"]), "result", "found an array"),
            (
                json!({"$harnessDispatch": "function"}),
                "result",
                "found a function",
            ),
            (
                json!({"candidateId": "a", "reason": "r"}),
                "result.status",
                "missing",
            ),
            (
                selected(json!({"status": "chosen"})),
                "result.status",
                "found \"chosen\"",
            ),
            (
                selected(json!({"status": 1})),
                "result.status",
                "found a number",
            ),
            (
                selected(json!({"reason": null})),
                "result.reason",
                "missing",
            ),
            (selected(json!({"reason": " \n"})), "result.reason", "blank"),
            (
                selected(json!({"reason": 7})),
                "result.reason",
                "found a number",
            ),
            (
                selected(json!({"candidateId": null})),
                "result.candidateId",
                "missing",
            ),
            (
                selected(json!({"candidateId": {"$harnessDispatch": "function"}})),
                "result.candidateId",
                "found a function",
            ),
            (
                selected(json!({"args": ["--yolo"]})),
                "result.args",
                "cannot supply a program or arguments",
            ),
            (
                selected(json!({"program": "/bin/sh"})),
                "result.program",
                "cannot supply",
            ),
            (
                selected(json!({"code": "c"})),
                "result.code",
                "unknown field",
            ),
            (
                json!({"status": "refused", "code": "c", "message": "m"}),
                "result.remedy",
                "missing",
            ),
            (
                json!({"status": "refused", "code": "", "message": "m", "remedy": "r"}),
                "result.code",
                "blank",
            ),
            (
                json!({"status": "refused", "code": "c", "message": "m", "remedy": "r", "candidateId": "a"}),
                "result.candidateId",
                "unknown field",
            ),
        ];
        for (result, location, found) in cases {
            let refusal = judged(result.clone(), None).unwrap_err();
            assert_eq!(refusal.code, "selection_malformed", "{result}");
            assert_eq!(refusal.location.as_deref(), Some(location), "{result}");
            assert!(
                refusal.message.contains(found),
                "{result}: {}",
                refusal.message
            );
            assert_eq!(refusal.source.as_deref(), Some("/p.ts"));
        }
    }

    #[test]
    fn each_other_failure_of_select_has_its_own_code() {
        let policy = computing();
        let refused = |produced| computed(&policy, produced, "impl", None, "/p.ts").unwrap_err();
        let threw = refused(Produced::Threw {
            name: "TypeError".into(),
            message: "x is undefined".into(),
        });
        assert_eq!(threw.code, "selection_threw");
        assert!(threw.message.contains("TypeError: x is undefined"));
        assert_eq!(refused(Produced::Unsettled).code, "selection_unsettled");
        let unserializable = refused(Produced::Unserializable {
            name: "TypeError".into(),
            message: "cyclic".into(),
        });
        assert_eq!(unserializable.code, "selection_malformed");
        assert_eq!(
            refused(Produced::Result(Value::Null)).code,
            "selection_abstained"
        );

        let unknown = judged(
            json!({"status": "selected", "candidateId": "z", "reason": "r"}),
            None,
        )
        .unwrap_err();
        assert_eq!(unknown.code, "unknown_candidate");
        assert_eq!(unknown.location.as_deref(), Some("result.candidateId"));
        assert!(
            unknown.remedy.contains(r#"("a", "b")"#),
            "{}",
            unknown.remedy
        );
    }

    #[test]
    fn with_a_choice_any_other_id_is_a_mismatch_known_or_not() {
        for other in ["a", "z"] {
            let refusal = judged(
                json!({"status": "selected", "candidateId": other, "reason": "a safe fallback"}),
                Some("b"),
            )
            .unwrap_err();
            assert_eq!(refusal.code, "explicit_choice_mismatch", "{other}");
            assert_eq!(refusal.input.as_deref(), Some("--choice b"));
            assert!(
                refusal.message.contains("a safe fallback"),
                "{}",
                refusal.message
            );
        }
    }

    #[test]
    fn a_policys_refusal_keeps_its_code_beside_the_stable_one() {
        let refusal = judged(
            json!({"status": "refused", "code": "no_reviewer", "message": "nobody fits", "remedy": "declare one"}),
            Some("b"),
        )
        .unwrap_err();
        assert_eq!(refusal.code, "policy_refused");
        assert_eq!(refusal.policy_code.as_deref(), Some("no_reviewer"));
        assert_eq!(refusal.stage, Stage::Selection);
        assert!(
            refusal.message.ends_with("nobody fits"),
            "{}",
            refusal.message
        );
        assert_eq!(refusal.remedy, "declare one");
        assert_eq!(refusal.input.as_deref(), Some("--choice b"));
        let unchosen = judged(
            json!({"status": "refused", "code": "c", "message": "m", "remedy": "r"}),
            None,
        )
        .unwrap_err();
        assert_eq!(unchosen.input.as_deref(), Some("--kind impl"));
    }

    #[test]
    fn a_valid_policy_routes_its_kind_and_refuses_another() {
        let policy = Validator::new("/p.ts").validate(&valid()).unwrap();
        let selection = select(&policy, "impl", None, "/p.ts").unwrap();
        assert_eq!(policy.catalog[selection.index].id, "a");
        assert_eq!(selection.reason, r#"routes["impl"] names candidate "a""#);
        assert_eq!(selection.by, SelectedBy::Route);
        let refusal = select(&policy, "design", None, "/p.ts").unwrap_err();
        assert_eq!(refusal.code, "incomplete_mapping");
    }

    #[test]
    fn an_explicit_choice_bypasses_the_routes_and_an_unknown_one_refuses() {
        let mut policy = valid();
        policy["catalog"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id": "b", "provider": "q", "model": "m", "effort": "e", "program": "x", "args": [{"slot": "prompt"}]}));
        let policy = Validator::new("/p.ts").validate(&policy).unwrap();
        // "impl" routes to "a", and "design" is not routed at all.
        for kind in ["impl", "design"] {
            let selection = select(&policy, kind, Some("b"), "/p.ts").unwrap();
            assert_eq!(policy.catalog[selection.index].id, "b");
            assert_eq!(selection.by, SelectedBy::ExplicitChoice);
        }
        let refusal = select(&policy, "impl", Some("c"), "/p.ts").unwrap_err();
        assert_eq!(refusal.code, "unknown_choice");
        assert_eq!(refusal.input.as_deref(), Some("--choice c"));
        assert!(
            refusal.remedy.contains(r#"("a", "b")"#),
            "{}",
            refusal.remedy
        );
    }

    #[test]
    fn a_marker_is_described_as_the_value_it_replaced() {
        let mut policy = valid();
        policy["catalog"][0]["model"] = json!({"$harnessDispatch": "function"});
        let refusal = Validator::new("/p.ts").validate(&policy).unwrap_err();
        assert_eq!(refusal.location.as_deref(), Some("policy.catalog[0].model"));
        assert!(
            refusal.message.contains("found a function"),
            "{}",
            refusal.message
        );
    }

    #[test]
    fn a_non_array_catalog_names_what_it_found() {
        let mut policy = valid();
        policy["catalog"] = json!({"a": 1});
        let refusal = Validator::new("/p.ts").validate(&policy).unwrap_err();
        assert!(
            refusal.message.contains("found an object"),
            "{}",
            refusal.message
        );
    }
}
