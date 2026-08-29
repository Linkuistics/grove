//! The conformance kit: hand it a configuration file and the slot vocabulary it
//! was written against, and learn which of this crate's obligations it violates.
//!
//! This is the **cross-crate seam**. A consumer's own suite can only assert that
//! *its* configuration works with *its* build; the kit is what holds a
//! configuration to the contract this crate states, from outside the consumer,
//! which is what keeps *reusable outside grove* true without a second
//! repository.
//!
//! ```no_run
//! use keyed_launch::{conformance, Requirement, SlotRule, Vocabulary};
//! let outcome = conformance::check(
//!     std::path::Path::new("config.kdl"),
//!     Vocabulary { slots: &[SlotRule { name: "prompt", requirement: Requirement::ExactlyOnce }] },
//! );
//! assert!(outcome.passed(), "{}", outcome.failures.join("\n"));
//! ```
//!
//! # Why an empty document fails
//!
//! A kit that only reports violations reads exactly the same when it is handed
//! nothing to check: no keys, no violations, conforming. A configuration
//! declaring no keys is therefore a failure in its own right — not because an
//! empty file is malformed, but because a suite of must-hold claims cannot
//! otherwise detect that it did not run.

use std::ffi::OsString;
use std::path::Path;

use crate::templates::Templates;
use crate::vocabulary::Vocabulary;

/// What the kit found. Empty [`failures`](Self::failures) is conformance.
pub struct Outcome {
    pub failures: Vec<String>,
}

impl Outcome {
    #[must_use]
    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }
}

/// Hold a consumer's configuration to this crate's contract.
///
/// Three obligations, in order: the document loads whole against `vocabulary`;
/// it declares at least one key; and every key it declares expands to an argv
/// with a program, given one placeholder value per declared slot. The third is
/// what stops the kit from being a second spelling of `load` — expansion is the
/// only place the compiled words are walked.
#[must_use]
pub fn check(config: &Path, vocabulary: Vocabulary<'_>) -> Outcome {
    let placeholders: Vec<(String, OsString)> = vocabulary
        .slots
        .iter()
        .map(|slot| {
            (
                slot.name.to_owned(),
                OsString::from(format!("<{}>", slot.name)),
            )
        })
        .collect();

    let templates = match Templates::load(config, None, vocabulary) {
        Ok(templates) => templates,
        Err(error) => {
            return Outcome {
                failures: vec![error.to_string()],
            }
        }
    };

    let mut failures = Vec::new();
    let keys: Vec<String> = templates.keys().into_iter().map(str::to_owned).collect();
    if keys.is_empty() {
        failures.push(format!(
            "{} declares no keys, so nothing in this kit was exercised. A configuration \
             that checks nothing passes every check.",
            config.display()
        ));
    }

    let values: Vec<crate::Slot<'_>> = placeholders
        .iter()
        .map(|(name, value)| crate::Slot {
            name,
            value: value.as_os_str(),
        })
        .collect();

    for key in &keys {
        match templates.expand(key, &values) {
            Ok(argv) => {
                if argv.program().is_empty() {
                    failures.push(format!("key `{key}` expands to an empty program"));
                }
            }
            Err(error) => failures.push(format!("key `{key}` does not expand: {error}")),
        }
    }

    Outcome { failures }
}
