//! Validate a policy snapshot and resolve a static route
//! (`docs/specs/harness-selection-and-execution.md`, *Policy and joint choice*).
//!
//! The worker hands over the entry's `policy` export as JSON, with any value
//! JSON cannot carry replaced by a `{"$harnessDispatch": "<type>"}` marker. This
//! module is the one validator: every refusal names the location it found, in
//! the form `policy.catalog[1].provider`, and nothing invalid is ever repaired
//! into something that passes.

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::refusal::{Refusal, Stage, EXIT_REFUSED};

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
/// Named by the spec, and refused until the handoff record that allocates the
/// run ID lands (`handoff-records-k24`).
const LATER_SLOTS: [&str; 1] = ["runId"];

#[derive(Debug)]
pub struct Policy {
    pub version: String,
    pub catalog: Vec<Candidate>,
    pub routes: BTreeMap<String, String>,
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

/// A caller input or catalog value that fills one whole argument. `runId` is
/// refused at validation, so it has no variant yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    Prompt,
    Kind,
    TaskFile,
    TaskId,
    Model,
    Effort,
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
            _ => return None,
        })
    }
}

/// The catalog index of the candidate a valid policy chose, and why.
#[derive(Debug)]
pub struct Selection {
    pub index: usize,
    pub reason: String,
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
                "use the static `routes` form in {}: an exact table from kind to candidate ID",
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
        match (present("routes"), present("select")) {
            (true, false) => {}
            (false, true) => return Err(self.unsupported("select")),
            (both, _) => {
                return Err(self.invalid(
                    "policy",
                    if both {
                        "a policy has exactly one of `routes` or `select`, and this one has both"
                    } else {
                        "a policy has exactly one of `routes` or `select`, and this one has neither"
                    },
                ))
            }
        }
        if present("loadContext") {
            return Err(self.unsupported("loadContext"));
        }

        let version = self.string(policy, "version", "policy.version")?;
        if version.trim().is_empty() {
            return Err(self.invalid("policy.version", "`version` must not be blank"));
        }
        let catalog = self.catalog(policy.get("catalog"))?;
        let routes = self.routes(&policy["routes"], &catalog)?;
        Ok(Policy {
            version,
            catalog,
            routes,
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
        let candidate = Candidate {
            id: nonempty("id")?,
            provider: nonempty("provider")?,
            model: nonempty("model")?,
            effort: nonempty("effort")?,
            program: nonempty("program")?,
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
        match slot.as_str().map(|name| (name, Slot::named(name))) {
            Some((_, Some(slot))) => Ok(Argument::Slot(slot)),
            Some((name, None)) if LATER_SLOTS.contains(&name) => Err(Refusal::new(
                "unsupported_form",
                Stage::Validation,
                EXIT_REFUSED,
                format!("the `{name}` slot is not supported by this release of harness-dispatch"),
                format!(
                    "remove the `{name}` slot from {at} in {}; the run ID arrives with the \
                     required handoff record",
                    self.source
                ),
            )
            .source(self.source)
            .location(format!("{at}.slot"))),
            _ => Err(self.invalid(
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

/// Resolve `kind` through a valid policy's routes. A kind the table does not
/// name refuses; no default candidate is ever substituted.
pub fn route(policy: &Policy, kind: &str, source: &str) -> Result<Selection, Refusal> {
    let id = policy.routes.get(kind).ok_or_else(|| {
        Refusal::new(
            "incomplete_mapping",
            Stage::Selection,
            EXIT_REFUSED,
            format!("the routes in {source} name no candidate for kind {kind:?}"),
            format!(
                "add a route {} to a candidate ID in {source}; harness-dispatch never substitutes \
                 a default candidate",
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
    })
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

    #[test]
    fn a_valid_policy_routes_its_kind_and_refuses_another() {
        let policy = Validator::new("/p.ts").validate(&valid()).unwrap();
        let selection = route(&policy, "impl", "/p.ts").unwrap();
        assert_eq!(policy.catalog[selection.index].id, "a");
        assert_eq!(selection.reason, r#"routes["impl"] names candidate "a""#);
        let refusal = route(&policy, "design", "/p.ts").unwrap_err();
        assert_eq!(refusal.code, "incomplete_mapping");
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
