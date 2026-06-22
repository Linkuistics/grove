pub mod cli;
pub mod complete;
pub mod harness;
pub mod harness_stamp;
pub mod launch;
// `leaf` (Kind + the old `NNN-slug` `split_prefix`) and `leaf_id` (the v1-flat
// reader) survive solely as `tree_migrate`'s one-time migration inputs (ADR-0034).
// The v1 verb path — `leaf_read` / `leaf_grow` / `leaf_lifecycle` / `migrate` —
// was swept when `llm_cli` / `cli` / `launch` flipped to the v2 `tree_*` modules
// (ADR-0035, the install-and-reflip-v2 leaf).
pub mod leaf;
pub mod leaf_id;
pub mod llm_cli;
pub mod loop_driver;
pub mod provision;
pub mod repo;
pub mod tree_grow;
pub mod tree_id;
pub mod tree_lifecycle;
pub mod tree_migrate;
pub mod tree_read;
