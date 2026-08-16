pub mod cli;
pub mod complete;
pub mod driver_lease;
pub(crate) mod finish_cleanup;
pub(crate) mod finish_transaction;
pub mod harness;
// `launch`, `loop_driver` and `tree_migrate` are crate-private because nothing
// outside the crate reaches them any more: the human verbs the suite used to
// drive through `launch` are gone, `loop_driver`'s fixtures now drive the real
// process, and `tree_migrate`'s in-place engine went with them, leaving a
// planner the migration transaction consumes internally. Narrowing them is not
// tidiness — a `pub` item in a `pub` module is reachable by definition, so
// `dead_code` says nothing about one. Crate-private, the compiler enumerates
// the next dead launch function for us instead of it needing to be hunted.
pub(crate) mod launch;
// `leaf` holds `Kind`, which is live everywhere, plus the old `NNN-slug`
// `split_prefix`; `leaf_id` is the v1-flat reader. Those two *parsers* survive
// solely as `tree_migrate`'s one-time migration inputs (task-tree-scheme). The v1
// verb path — `leaf_read` / `leaf_grow` / `leaf_lifecycle` / `migrate` — was swept
// when `llm_cli` / `cli` / `launch` flipped to the `tree_*` modules
// (task-tree-scheme, the install-and-reflip-v2 leaf).
pub mod leaf;
pub mod leaf_id;
pub mod llm_cli;
pub(crate) mod loop_driver;
// `methodology` owns the embed itself — the `include_dir!` static, the unit
// reader `build.rs` shares by `#[path]`, and the build's methodology identity.
// It is the module seam the design names, and it is `pub` on the same footing
// as `provision`: the suite drives the *real* embed through it, and the only
// alternative door is a spawned process per parser edge case, which the design
// weighed and rejected (`docs/specs/mandate-delivered-methodology.md`, *Test
// seams*). It also outlives `provision` — the embed survives the retirement of
// the directory it is currently also swept into.
pub mod methodology;
// `prompt` owns the guaranteed core — the whole of `${prompt}`, and the only
// place driver-authored prose about a session's own conduct lives. `pub` for
// the same reason `methodology` is: every interesting check runs through this
// seam against the *real* embed, and the alternative door is a spawned driver
// per claim (`docs/specs/skill-delivered-methodology.md`, *Test seams*).
pub mod prompt;
pub mod provision;
pub mod repo;
pub mod session_config;
// `test_barrier` is the publication rule the process-interruption seams share.
// It is compiled always — the seams are, too — but nothing outside them may
// reach it, so it is crate-private like the seams' own checkpoint functions.
pub(crate) mod test_barrier;
pub mod tree_access;
pub mod tree_format;
// `tree_grow` is crate-private for the same reason as the modules above, and it
// is the module that settled the general rule: **a public item whose only
// callers are tests stops being module API** — deleted where the tests can
// assert on what production reads, demoted into `mod tests` where they still
// need the convenience. Either way its coverage moves onto the interface
// production uses, and `dead_code` gets its say back.
//
// Every `pub` here was a *locked wrapper*: a guard acquisition around an
// already-resolved argument. Production calls none of them and cannot, because
// `llm_cli` must resolve the `<parent>`/`<target>` reference *inside* the same
// exclusive guard that then mutates. So a wrapper was never a spare door onto
// the operation — it was a narrower critical section wearing the operation's
// name, and the only correct use of it was the one nobody had. They now live in
// this module's `mod tests`, which is what they always were.
//
// Two kinds of surface are exempt, and after this pass they are the *only* two
// the sweep still reports, each argued where it lives:
//
//   * **a genuine seam** — production reaches the same behaviour through a door
//     a test cannot open. `tree_lifecycle::transition_to_current` keeps its
//     `pub` on exactly that ground; its `pub(crate)` driver twin also reaps
//     finish artifacts, so it cannot stand in for the classification.
//   * **a frozen grammar kept whole** — `leaf_id`, the v1-flat parser, is
//     deliberately not trimmed to what the one-time migration happens to call
//     (see its header). Trimming a parser to its current caller is how the next
//     legacy tree becomes unreadable.
//
// The technique that produces that list: copy `src/` to a scratch crate, make
// every module private except `cli` and `llm_cli`, and read the compiler's
// reachability warnings.
pub(crate) mod tree_grow;
pub mod tree_id;
pub mod tree_lifecycle;
pub(crate) mod tree_migrate;
pub(crate) mod tree_migration_transaction;
pub mod tree_read;
pub mod tree_rename;
