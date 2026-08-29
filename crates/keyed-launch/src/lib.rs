//! A configuration of **key → complete command template**, validated whole and
//! expanded into an argv.
//!
//! A consumer names a key; a template names a program. Nothing here understands
//! either: a key is an opaque string, a slot is a name the consumer declares,
//! and the words of a template are the words the file holds. What the crate owns
//! is that a launch is *one complete template string, read whole out of one
//! file* — never assembled from two — and that every rule about a template is
//! checked before anything is spawned.
//!
//! # Two documents, and what the second one may do
//!
//! [`Templates::load`] takes a primary file and an optional overlay. **A key
//! resolves only if the primary declares it**: where the overlay also declares
//! it the overlay's template is the one used, whole; where only the overlay
//! declares it the key does not resolve, and the refusal names the key and the
//! primary file that must declare it. That is what keeps a second source unable
//! to introduce a program the operator never chose, and it is checked without
//! either document knowing what a key means.
//!
//! Which files those two are, and whether the overlay is admissible at all, are
//! the consumer's questions. This crate reads the paths it is handed.
//!
//! # The vocabulary is an input to `load`, not to `expand`
//!
//! Every template rule is a rule about slot *names*, so a loader that will not
//! learn the names until expansion can check none of them. See [`Vocabulary`].
//!
//! # Testing a consumer's configuration
//!
//! [`conformance::check`] holds a configuration to this crate's contract from
//! outside the consumer's own suite.

pub mod conformance;

mod argv;
mod error;
mod templates;
mod vocabulary;

pub use argv::{Argv, Slot};
pub use error::ConfigError;
pub use templates::Templates;
pub use vocabulary::{Requirement, SlotRule, Vocabulary};
