pub mod cli;
pub mod complete;
pub mod driver_lease;
pub mod harness;
// `launch` and `loop_driver` are crate-private because nothing outside the
// crate reaches them any more: the human verbs the suite used to drive through
// `launch` are gone, and `loop_driver`'s fixtures now drive the real process.
// Narrowing them is not tidiness — a `pub` item in a `pub` module is reachable
// by definition, so `dead_code` says nothing about one. Crate-private, the
// compiler enumerates the next dead launch function for us instead of it
// needing to be hunted.
pub(crate) mod launch;
// `leaf` holds `Kind` and nothing else now — it is live everywhere. Its legacy
// lodgers are gone with the readers they served: the `NNN-slug` prefix parser
// and the whole `leaf_id` v1-flat module went when the v1 verb path was swept
// (task-tree-scheme, the install-and-reflip-v2 leaf), and the last of them
// followed migration itself out (`delete-migration-k6`).
pub mod leaf;
pub mod llm_cli;
pub(crate) mod loop_driver;
// `methodology` owns the embed itself — the `include_dir!` static and the
// build's methodology identity. It is `pub` on the same footing as `provision`:
// the suite asserts claims about the *real* corpus through it — that every
// kind's reference file exists, that the methodology instructs no `grove-llm`
// verb the CLI lacks — and production's own door onto that corpus is
// `include_dir`, which a test cannot open without making a runtime dependency a
// dev one as well (`docs/ARCHITECTURE.md`, *the embed test seam*).
// It also outlives `provision` — the embed survives the retirement of the
// directory it is currently also swept into.
pub mod methodology;
// `prompt` owns the guaranteed core — the whole of `${prompt}`, and the only
// place driver-authored prose about a session's own conduct lives. `pub` for
// the same reason `methodology` is: every interesting check runs through this
// seam against the *real* embed, and the alternative door is a spawned driver
// per claim (`docs/ARCHITECTURE.md`, *the embed test seam*).
pub mod prompt;
pub mod provision;
pub mod repo;
pub mod session_config;
// `task_name` is Grove's implementation of `ordinal_fs_tree::EntryName` — the
// whole seam onto the extracted tree library (gh issue #13, increment 2), and
// since `migration-k36` it is the **only** grammar production speaks. Each verb
// group moved across in its own leaf, so there is no longer a call site whose
// `use` line has to be read to know which model it means. The grammar it
// replaced was lenient on the position's padding where this one is canonical,
// which is why the order mattered while both were live; `sweep-k37` deleted it.
pub mod task_name;
// `task_grow` is `leaf-add`, `leaf-add-pair` and `leaf-insert` expressed through
// that seam — the *migrate* stage's third leaf. What is left in it is what the
// library has no counterpart for: grove's reference grammar, the preconditions
// the library cannot see, the task-file template, and the cross-reference lint.
pub mod task_grow;
// `task_tree` is grove's reading surface expressed through that seam — the
// *migrate* stage's first leaf. `pick`, `select`, `brief-chain`, `kind` and
// `resolve` read one snapshot under the library's shared lock, and it owns the
// one place grove builds a path out of the tree. Since `lifecycle-k35` it also
// owns the **write** seam every mutating verb is on. Since `sweep-k37` it is
// the only reading surface grove has.
pub mod task_tree;
pub mod tree_access;
pub mod tree_lifecycle;
