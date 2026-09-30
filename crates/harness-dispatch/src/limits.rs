//! The resource bounds a selection runs within
//! (`docs/specs/harness-selection-and-execution.md`, *Bounded context*).
//!
//! Two bounds are the caller's to adjust within a hard range: the whole-selection
//! time, with `--timeout-ms`, and the context budget, with `--context-bytes`. The
//! per-read bound follows the context budget, and a policy's read may set its own
//! up to that budget. The source count, the protocol message and the diagnostics
//! are fixed. The worker receives each effective value in its evaluate message
//! and the policy sees them in `request.limits`, but nothing a policy does can
//! raise one: the worker keeps its own copies, and the front checks what it
//! receives against its own. No bound is met by truncating: each overflow
//! refuses, naming the bound.

use std::ffi::OsStr;
use std::time::Duration;

use serde_json::{json, Value};

use crate::refusal::{Refusal, Stage, EXIT_MALFORMED};

/// The whole-selection bound, in milliseconds: its default and the range a
/// caller may choose with `--timeout-ms`.
pub const SELECTION_DEFAULT_MS: u64 = 30_000;
pub const SELECTION_MIN_MS: u64 = 1_000;
pub const SELECTION_MAX_MS: u64 = 120_000;

/// The context delivered to selection, in encoded UTF-8 JSON bytes: its default
/// and the range a caller may choose with `--context-bytes`.
pub const CONTEXT_DEFAULT_BYTES: u64 = 256 * 1024;
pub const CONTEXT_MIN_BYTES: u64 = 1;
pub const CONTEXT_MAX_BYTES: u64 = 8 * 1024 * 1024;

/// One SDK source read, by default; never more than the context budget.
pub const SOURCE_DEFAULT_BYTES: u64 = 64 * 1024;

/// Measured context sources, the `--context` document included, and the
/// records a context's `sources` array may hold.
pub const SOURCES_LIMIT: u64 = 256;

/// A policy result or catalog protocol message, in bytes.
pub const MESSAGE_BYTES: u64 = 1024 * 1024;

/// The worker's stdout and stderr together, in bytes.
pub const DIAGNOSTICS_BYTES: u64 = 256 * 1024;

/// Where a bound's value came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    /// Its default, which a caller could have changed.
    Default,
    /// Set by the named input: a flag, or a read's `maxBytes` argument.
    Set(&'static str),
    /// Fixed: nothing changes it.
    Fixed,
}

/// A bound: its name, its value, and where the value came from. Inspection
/// reports it, and a refusal it caused names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bound {
    pub name: &'static str,
    pub unit: &'static str,
    pub value: u64,
    pub origin: Origin,
}

impl Bound {
    /// The whole-selection bound as a duration.
    pub fn duration(self) -> Duration {
        Duration::from_millis(self.value)
    }

    /// The input that set it, if one did.
    pub fn flag(self) -> Option<&'static str> {
        match self.origin {
            Origin::Set(input) => Some(input),
            Origin::Default | Origin::Fixed => None,
        }
    }

    /// Where the value came from: the input that set it, `default` or `fixed`.
    pub fn from(self) -> &'static str {
        match self.origin {
            Origin::Default => "default",
            Origin::Set(input) => input,
            Origin::Fixed => "fixed",
        }
    }

    /// `{"ms": 30000, "from": "default"}`, keyed by its unit.
    pub fn to_json(self) -> Value {
        let mut value = json!({ "from": self.from() });
        value[self.unit] = self.value.into();
        value
    }

    /// `30000 ms (the default)`, `2500 ms (--timeout-ms)` or `256 sources (fixed)`.
    pub fn to_text(self) -> String {
        let from = match self.origin {
            Origin::Default => "the default",
            Origin::Set(input) => input,
            Origin::Fixed => "fixed",
        };
        format!("{} {} ({from})", self.value, self.unit)
    }
}

/// Every bound in effect for one invocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub selection: Bound,
    pub context: Bound,
    /// The default for one read; a read's `maxBytes` may set its own, up to
    /// the context budget.
    pub source: Bound,
    pub sources: Bound,
    pub message: Bound,
    pub diagnostics: Bound,
}

impl Limits {
    /// The caller's `--timeout-ms` and `--context-bytes`, checked, with every
    /// other bound at its fixed or derived value.
    pub fn read(
        timeout_ms: Option<&OsStr>,
        context_bytes: Option<&OsStr>,
    ) -> Result<Limits, Refusal> {
        let selection = digits(
            timeout_ms,
            "--timeout-ms",
            "milliseconds",
            SELECTION_MIN_MS..=SELECTION_MAX_MS,
            "give the whole-selection bound in milliseconds, from 1000 (1 second) to 120000 \
             (120 seconds), or omit --timeout-ms for the 30-second default",
        )?;
        let context = digits(
            context_bytes,
            "--context-bytes",
            "bytes",
            CONTEXT_MIN_BYTES..=CONTEXT_MAX_BYTES,
            "give the context budget in bytes, from 1 to 8388608 (8 MiB), or omit \
             --context-bytes for the 262144-byte (256 KiB) default",
        )?;
        let selection = Bound {
            name: "selection",
            unit: "ms",
            value: selection.unwrap_or(SELECTION_DEFAULT_MS),
            origin: origin(selection, "--timeout-ms"),
        };
        let context = Bound {
            name: "context",
            unit: "bytes",
            value: context.unwrap_or(CONTEXT_DEFAULT_BYTES),
            origin: origin(context, "--context-bytes"),
        };
        // A read's default never exceeds the budget the read must fit in, and
        // when the budget is what lowers it, the budget's input is its origin.
        let source = if context.value < SOURCE_DEFAULT_BYTES {
            Bound {
                name: "source",
                ..context
            }
        } else {
            Bound {
                name: "source",
                unit: "bytes",
                value: SOURCE_DEFAULT_BYTES,
                origin: Origin::Default,
            }
        };
        let fixed = |name, unit, value| Bound {
            name,
            unit,
            value,
            origin: Origin::Fixed,
        };
        Ok(Limits {
            selection,
            context,
            source,
            sources: fixed("sources", "sources", SOURCES_LIMIT),
            message: fixed("message", "bytes", MESSAGE_BYTES),
            diagnostics: fixed("diagnostics", "bytes", DIAGNOSTICS_BYTES),
        })
    }

    fn all(self) -> [Bound; 6] {
        [
            self.selection,
            self.context,
            self.source,
            self.sources,
            self.message,
            self.diagnostics,
        ]
    }

    /// Each bound by name, as inspection and the run record report them.
    pub fn to_json(self) -> Value {
        let mut bounds = serde_json::Map::new();
        for bound in self.all() {
            bounds.insert(bound.name.into(), bound.to_json());
        }
        Value::Object(bounds)
    }

    /// One line per bound, for inspection's text.
    pub fn to_text(self) -> Vec<String> {
        self.all()
            .iter()
            .map(|bound| {
                let what = match bound.name {
                    "selection" => "selection within",
                    "context" => "context at most",
                    "source" => "one read at most",
                    "sources" => "at most",
                    "message" => "protocol message at most",
                    _ => "policy output at most",
                };
                format!("{what} {}", bound.to_text())
            })
            .collect()
    }

    /// What the policy sees as `request.limits`: each effective value, in the
    /// units its name says.
    pub fn to_request(self) -> Value {
        json!({
            "selectionMs": self.selection.value,
            "contextBytes": self.context.value,
            "sourceBytes": self.source.value,
            "sources": self.sources.value,
            "messageBytes": self.message.value,
            "diagnosticsBytes": self.diagnostics.value,
        })
    }
}

fn origin(given: Option<u64>, flag: &'static str) -> Origin {
    match given {
        Some(_) => Origin::Set(flag),
        None => Origin::Default,
    }
}

/// A flag's value as plain digits within `range`. Anything else, a sign or an
/// exponent included, is malformed rather than read generously, and nothing
/// out of range is clamped.
fn digits(
    given: Option<&OsStr>,
    flag: &str,
    unit: &str,
    range: std::ops::RangeInclusive<u64>,
    remedy: &str,
) -> Result<Option<u64>, Refusal> {
    let Some(given) = given else {
        return Ok(None);
    };
    let shown = given.to_string_lossy();
    given
        .to_str()
        .filter(|text| !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
        .and_then(|digits| digits.parse::<u64>().ok())
        .filter(|value| range.contains(value))
        .map(Some)
        .ok_or_else(|| {
            Refusal::new(
                "malformed_input",
                Stage::Cli,
                EXIT_MALFORMED,
                format!(
                    "{flag} {shown:?} is not a whole number of {unit} from {} to {}",
                    range.start(),
                    range.end()
                ),
                remedy,
            )
            .input(flag)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_and_their_origins() {
        let limits = Limits::read(None, None).unwrap();
        assert_eq!(
            limits.to_json(),
            json!({
                "selection": { "ms": 30_000, "from": "default" },
                "context": { "bytes": 262_144, "from": "default" },
                "source": { "bytes": 65_536, "from": "default" },
                "sources": { "sources": 256, "from": "fixed" },
                "message": { "bytes": 1_048_576, "from": "fixed" },
                "diagnostics": { "bytes": 262_144, "from": "fixed" },
            })
        );
    }

    #[test]
    fn a_context_budget_below_the_read_default_caps_each_read() {
        let limits = Limits::read(None, Some(OsStr::new("1024"))).unwrap();
        assert_eq!(
            limits.source.to_json(),
            json!({ "bytes": 1024, "from": "--context-bytes" })
        );
        let limits = Limits::read(None, Some(OsStr::new("8388608"))).unwrap();
        assert_eq!(
            limits.source.to_json(),
            json!({ "bytes": 65_536, "from": "default" })
        );
    }

    #[test]
    fn a_bound_outside_its_range_or_not_plain_digits_is_malformed() {
        for bad in ["0", "8388609", "1e3", "+5", " 5", "", "5.0"] {
            let refusal = Limits::read(None, Some(OsStr::new(bad))).unwrap_err();
            assert_eq!(refusal.code, "malformed_input", "{bad:?}");
            assert_eq!(refusal.input.as_deref(), Some("--context-bytes"), "{bad:?}");
        }
    }
}
