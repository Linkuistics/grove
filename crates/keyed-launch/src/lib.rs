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
//! # From a template to a running child
//!
//! [`Templates::expand`] authors an [`Argv`]; [`run`] spawns it — directly,
//! with no shell — and supervises the child until it ends. The two halves meet
//! only at `Argv`, and each is usable without the other: a launcher that builds
//! its argv some other way still cannot construct one, which is the point.
//!
//! **The child is a job.** It is spawned into a process group of its own and
//! handed the launcher's controlling terminal, so a terminal signal reaches the
//! child rather than the launcher, the escalation can reap a grandchild the
//! child spawned, and a launcher's own ignored dispositions are not inherited
//! across the `exec` by every wrapper the template names. See [`run`].
//!
//! **A launch ends out of band.** An interactive child returns to its prompt
//! when it finishes rather than exiting, so its own exit is not the event
//! anyone is waiting for. [`Channel`] is: a fresh path per launch that the
//! child writes a [`Token`] to (through [`signal`]) when it is done, and whose
//! *appearance* starts the kill [`Escalation`] the child cannot perform on
//! itself. See [`Escalation`] for why that is the launcher's job.
//!
//! # Testing a consumer's configuration
//!
//! [`conformance::check`] holds a configuration to this crate's contract from
//! outside the consumer's own suite.

pub mod conformance;

mod argv;
mod channel;
mod error;
mod run;
mod templates;
mod vocabulary;

pub use argv::{Argv, Slot};
pub use channel::{signal, Channel, Token};
pub use error::{ConfigError, LaunchError};
pub use run::{reraise, run, take_interrupt, End, Ended, Escalation, Launch};
pub use templates::Templates;
pub use vocabulary::{Requirement, SlotRule, Vocabulary};
