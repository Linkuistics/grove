pub mod cli;
pub mod complete;
pub mod harness;
pub mod harness_stamp;
pub mod herdr;
pub mod json;
pub mod launch;
// `leaf` holds `Kind`, which is live everywhere, plus the old `NNN-slug`
// `split_prefix`; `leaf_id` is the v1-flat reader. Those two *parsers* survive
// solely as `tree_migrate`'s one-time migration inputs (task-tree-scheme). The v1
// verb path — `leaf_read` / `leaf_grow` / `leaf_lifecycle` / `migrate` — was swept
// when `llm_cli` / `cli` / `launch` flipped to the `tree_*` modules
// (task-tree-scheme, the install-and-reflip-v2 leaf).
pub mod leaf;
pub mod leaf_id;
pub mod llm_cli;
pub mod loop_driver;
pub mod provision;
pub mod repo;
pub mod tree_access;
pub mod tree_grow;
pub mod tree_id;
pub mod tree_lifecycle;
pub mod tree_migrate;
pub mod tree_promotion;
pub mod tree_read;
pub mod tree_rename;
