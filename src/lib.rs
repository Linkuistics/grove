pub mod cli;
pub mod complete;
pub mod driver_lease;
// `loop_driver` is crate-private because nothing outside the crate reaches it
// any more: the human verbs the suite used to drive through the old `launch`
// module are gone, that module has been absorbed here, and `loop_driver`'s
// fixtures now drive the real process. Narrowing it is not tidiness — a `pub`
// item in a `pub` module is reachable by definition, so `dead_code` says
// nothing about one. Crate-private, the compiler enumerates the next dead
// launch function for us instead of it needing to be hunted.
// There is no `leaf` module. It held one item, `Kind`, and its stated reason for
// sitting apart from the other three name components was that the set was closed
// and the label doubled as a configuration key. `open-kind-k20` opened the set,
// which left `Kind` as exactly what `Slug` is — a validated word of the filename
// grammar, obeying the *same* shape rule, on which the canonicity of a leaf name
// depends. It is `task_name`'s, and one function states that shape for both.
pub mod llm_cli;
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
// `task_name` is Grove's implementation of `ordinal_fs_tree::EntryName` — the
// whole seam onto the extracted tree library (gh issue #13, increment 2), and
// since `migration-k36` it is the **only** grammar production speaks. Each verb
// group moved across in its own leaf, so there is no longer a call site whose
// `use` line has to be read to know which model it means. The grammar it
// replaced was lenient on the position's padding where this one is canonical,
// which is why the order mattered while both were live; `sweep-k37` deleted it.
// Since `name-ownership-k14` it also owns the *handle* — `Handle`, the
// position-free `<slug>-k<key>` identity every other module carries — so there
// is one grammar rather than a filename grammar plus six copies of half of one.
pub mod task_name;
// `task_grow` is `leaf-add` and `leaf-insert` expressed through
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
pub mod tree_lifecycle;
