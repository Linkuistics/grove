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
// `leaf` holds `Kind` and nothing else now — it is live everywhere. Its legacy
// lodgers are gone with the readers they served: the `NNN-slug` prefix parser and
// the whole `leaf_id` v1-flat module, both of which now exist only as name
// *recognisers* inside `tree_migrate`, where their one caller is. The v1 verb
// path — `leaf_read` / `leaf_grow` / `leaf_lifecycle` / `migrate` — was swept
// when `llm_cli` / `cli` / `launch` flipped to the `tree_*` modules
// (task-tree-scheme, the install-and-reflip-v2 leaf).
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
// group moved across in its own leaf and the migration planner's renderer came
// last, so there is no longer a call site whose `use` line has to be read to
// know which model it means. The two grammars differ in exactly one place that
// matters — this one is canonical where `tree_id::parse_position` is lenient —
// which is why the order mattered while both were live.
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
// owns the **write** seam every mutating verb is on, including the scaffolding
// guard `root-init` completes a formatless root through — the one writer that
// cannot require `.grove/FORMAT`, because it is the writer `FORMAT` is written
// after. `tree_read` has no production caller left at all.
pub mod task_tree;
// `test_barrier` is the publication rule the process-interruption seams share.
// It is compiled always — the seams are, too — but nothing outside them may
// reach it, so it is crate-private like the seams' own checkpoint functions.
pub(crate) mod test_barrier;
pub mod tree_access;
pub mod tree_format;
// `tree_grow` is what was left of the path-walking appender after `growing-k33`
// took the grow verbs to `task_grow`: one primitive, `leaf_add_unlocked`, and
// the destination guard it wrote through, kept alive by the lifecycle verbs that
// allocated under grove's *own* exclusive guard. `lifecycle-k35` moved the last
// of those across and the module is now test-only, awaiting `sweep-k37`.
// It is crate-private for the same reason as the modules above,
// and it is the module that settled the general rule: **a public item whose only
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
// **One** kind of surface is exempt, and every item the sweep still reports falls
// under it: **a genuine seam** — production reaches the same behaviour through a
// door a test cannot open. `tree_lifecycle::transition_to_current` keeps its
// `pub` on exactly that ground; its `pub(crate)` driver twin also reaps finish
// artifacts, so it cannot stand in for the classification.
// `methodology::markdown_files` keeps its `pub` on the same ground and is argued
// in its own doc comment: production's door onto the embed is `include_dir`,
// which a test cannot open without making a runtime dependency a dev one as well.
//
// A second exemption stood here until the legacy readers were withdrawn — **a
// frozen grammar kept whole**, which is why `leaf_id` was not trimmed to what the
// one-time migration happened to call. That rule was right while something read
// the grammar. Once nothing did, keeping a whole parser to answer a yes/no
// question was the reverse of what it argued for, and the grammar itself is in
// the VCS. What survives is the *recognition* — a private matcher per withdrawn
// layout in `tree_migrate` — because that is the part whose absence loses a
// workstream rather than a lookup (see `tree_migrate`'s `Format`).
//
// The technique that produces that list: copy `src/` to a scratch crate, make
// every module private except `cli` and `llm_cli`, and read the compiler's
// reachability warnings.
// Dead in production since `lifecycle-k35` took the last allocator across, and
// gated rather than deleted for the reason `growing-k33` gated
// `tree_read::resolve_unlocked`: the destination guard's tests are evidence
// about a hazard the library now owns, and they are worth more than the lines
// until `sweep-k37` removes both together.
#[cfg(test)]
pub(crate) mod tree_grow;
// Dead in production since `migration-k36` took the migration planner's
// recogniser and renderer across — the last production caller of any item in it.
// Unlike `tree_grow` and `tree_read` it is **not** `#[cfg(test)]`, and cannot be:
// `tests/session_kind_guidance.rs` reaches it as an integration test, through the
// public API, where that gate does not apply. So the compiler cannot hold the
// dead-module claim here the way `lifecycle-k35` had it hold theirs, and
// `sweep-k37` inherits both the deletion and the oracle that test is built on
// (see this increment's brief under `migration-k36`).
pub mod tree_id;
pub mod tree_lifecycle;
pub(crate) mod tree_migrate;
pub(crate) mod tree_migration_transaction;
// Dead in production since `lifecycle-k35` flipped the last two verbs that read
// through it, and gated rather than deleted for `growing-k33`'s reason: the
// module is the *other* implementation of a contract the library now owns, and
// `both_readers_resolve_every_reference_form_identically` is the one direct
// check the flip's *pure refactor* premise gets. Both die in `sweep-k37`.
#[cfg(test)]
pub mod tree_read;
pub mod tree_rename;
