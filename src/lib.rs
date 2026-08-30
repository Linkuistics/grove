//! What is left of the `grove` package: the human-facing binary's CLI, the loop
//! driver, its lease, the prompt it composes, and the per-kind launch
//! configuration it reads.
//!
//! **The tree and its verbs are not here.** `loop-crate-verbs-k21` moved
//! `task_name`, `task_tree`, `task_grow`, `tree_lifecycle` and the twelve verbs
//! into `crates/grove-loop`, and the `grove-llm` binary that dispatches them
//! into `crates/grove-llm`. What is below is the half of decision 9 that
//! `loop-crate-driver-k22` moves next — the driver, the lease and the loop —
//! plus `session_config`, which is the driver's reading of the runner's
//! configuration.

/// The version this repository ships, and the only one.
///
/// **One workspace, one release version** (`docs/specs/module-decomposition.md`,
/// decision 1): the `grove` package is what `cargo release` cuts and what the
/// tag names, and every other member is excluded from that cut. So the two
/// binaries and the prompt's published version all read this rather than their
/// own package's — `crates/grove-llm` would otherwise answer `--version` with
/// its own `0.1.0`, which names nothing an operator can install.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod cli;
pub mod driver_lease;
// `loop_driver` is crate-private because nothing outside the crate reaches it
// any more: the human verbs the suite used to drive through the old `launch`
// module are gone, that module has been absorbed here, and `loop_driver`'s
// fixtures now drive the real process. Narrowing it is not tidiness — a `pub`
// item in a `pub` module is reachable by definition, so `dead_code` says
// nothing about one. Crate-private, the compiler enumerates the next dead
// launch function for us instead of it needing to be hunted.
pub(crate) mod loop_driver;
// `prompt` owns the whole of `${prompt}` — every part of it, and the only place
// driver-authored prose about a session's own conduct lives. `pub` because the
// suite checks the core through this seam rather than by spawning a driver per
// claim; there is no longer an embed behind it to be the *real* subject, since
// `delete-provisioning-k19` deleted `content/` and the module that carried it.
// There is no `provision` module and no `harness` registry. Grove writes no
// skill directory: the methodology installs as a plugin like every other skill
// this repo ships (`plugins/grove/`), so the delivery question grove used to
// answer by sweeping an embed into three personal skill directories is now
// answered by the same install route the other plugins use.
pub mod prompt;
// There is no `repo` module. The VCS seam is `crates/jj-workspace`
// (`docs/specs/module-decomposition.md`, decision 8): resolving a workspace,
// refusing a working tree that is not one, and taking a path-scoped commit are
// now behind a crate that has never heard of grove, and the sites that used to
// call the adapter call `Workspace` directly. What stayed here is what only
// grove can say — the name of its control namespace, why a tracked
// configuration delta is refused, and what `.grove/` is.
pub mod session_config;
