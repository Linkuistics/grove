pub mod cli;
pub mod complete;
pub mod driver_lease;
pub(crate) mod finish_cleanup;
pub(crate) mod finish_transaction;
pub mod harness;
pub mod json;
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
pub mod provision;
pub mod repo;
pub mod session_config;
pub mod task_relationship;
pub mod tree_access;
pub mod tree_format;
pub mod tree_grow;
pub mod tree_id;
pub mod tree_lifecycle;
pub(crate) mod tree_migrate;
pub(crate) mod tree_migration_transaction;
pub mod tree_promotion;
pub mod tree_read;
pub mod tree_rename;
