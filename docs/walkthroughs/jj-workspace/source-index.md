# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/jj-workspace/Cargo.toml` | 44 |
| `source-library` | `crates/jj-workspace/src/lib.rs` | 343 |
| `source-subprocess` | `crates/jj-workspace/src/jj.rs` | 81 |
| `source-refusal` | `crates/jj-workspace/src/refusal.rs` | 230 |

<!-- source-root «source-crate-manifest» source="crates/jj-workspace/Cargo.toml" lines="1-44" -->
<!-- insert «manifest-no-dependencies» -->
<!-- /source-root -->
<!-- source-root «source-library» source="crates/jj-workspace/src/lib.rs" lines="1-343" -->
<!-- insert «library-crate-thesis» -->
<!-- defer «namespace-reserved-names» owner="no-consumer-vocabulary" lines="55-61" -->
<!-- defer «commit-identity» owner="no-transactions" lines="62-71" -->
<!-- insert «workspace-value-and-gate» -->
<!-- defer «namespace-control-dir» owner="no-consumer-vocabulary" lines="119-145" -->
<!-- defer «scope-tracking-and-commit» owner="no-transactions" lines="146-274" -->
<!-- insert «gate-main-repo-and-canonical» -->
<!-- defer «namespace-validation» owner="no-consumer-vocabulary" lines="320-343" -->
<!-- /source-root -->
<!-- source-root «source-subprocess» source="crates/jj-workspace/src/jj.rs" lines="1-81" -->
<!-- defer «subprocess-seam-source» owner="nothing-ambient" lines="1-81" -->
<!-- /source-root -->
<!-- source-root «source-refusal» source="crates/jj-workspace/src/refusal.rs" lines="1-230" -->
<!-- defer «refusal-source» owner="no-remedy-of-its-own" lines="1-230" -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-no-dependencies` | `source-crate-manifest` | `no-dependencies` | `1-44` | 44 | `resolved` |
| `library-crate-thesis` | `source-library` | `no-dependencies` | `1-54` | 54 | `resolved` |
| `namespace-reserved-names` | `source-library` | `no-consumer-vocabulary` | `55-61` | 7 | `deferred` |
| `commit-identity` | `source-library` | `no-transactions` | `62-71` | 10 | `deferred` |
| `workspace-value-and-gate` | `source-library` | `one-lane` | `72-118` | 47 | `resolved` |
| `namespace-control-dir` | `source-library` | `no-consumer-vocabulary` | `119-145` | 27 | `deferred` |
| `scope-tracking-and-commit` | `source-library` | `no-transactions` | `146-274` | 129 | `deferred` |
| `gate-main-repo-and-canonical` | `source-library` | `one-lane` | `275-319` | 45 | `resolved` |
| `namespace-validation` | `source-library` | `no-consumer-vocabulary` | `320-343` | 24 | `deferred` |
| `subprocess-seam-source` | `source-subprocess` | `nothing-ambient` | `1-81` | 81 | `deferred` |
| `refusal-source` | `source-refusal` | `no-remedy-of-its-own` | `1-230` | 230 | `deferred` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-44` | `—` | `manifest-no-dependencies` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `no-dependencies` | `1-12` | `manifest-no-dependencies` | `—` |
| `manifest-no-dependencies` | `orientation` | `source-crate-manifest` | `composite` | `no-dependencies` | `1-44` | `source-crate-manifest` | `manifest-package-identity`, `manifest-empty-dependencies`, `manifest-dev-dependency`, `manifest-lints`, `manifest-release-lane` |
| `manifest-empty-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `no-dependencies` | `13-19` | `manifest-no-dependencies` | `—` |
| `manifest-dev-dependency` | `orientation` | `source-crate-manifest` | `literal` | `no-dependencies` | `20-26` | `manifest-no-dependencies` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `no-dependencies` | `27-29` | `manifest-no-dependencies` | `—` |
| `manifest-release-lane` | `orientation` | `source-crate-manifest` | `literal` | `no-dependencies` | `30-44` | `manifest-no-dependencies` | `—` |
| `source-library` | `source-index` | `source-library` | `root` | `—` | `1-343` | `—` | `library-crate-thesis`, `namespace-reserved-names`, `commit-identity`, `workspace-value-and-gate`, `namespace-control-dir`, `scope-tracking-and-commit`, `gate-main-repo-and-canonical`, `namespace-validation` |
| `library-purpose-sentence` | `orientation` | `source-library` | `literal` | `no-dependencies` | `1-9` | `library-crate-thesis` | `—` |
| `library-crate-thesis` | `orientation` | `source-library` | `composite` | `no-dependencies` | `1-54` | `source-library` | `library-purpose-sentence`, `library-thesis-no-consumer`, `library-thesis-no-transactions`, `library-thesis-reads-add-no-history`, `library-module-surface` |
| `library-thesis-no-consumer` | `orientation` | `source-library` | `literal` | `no-dependencies` | `10-20` | `library-crate-thesis` | `—` |
| `library-thesis-no-transactions` | `orientation` | `source-library` | `literal` | `no-dependencies` | `21-30` | `library-crate-thesis` | `—` |
| `library-thesis-reads-add-no-history` | `orientation` | `source-library` | `literal` | `no-dependencies` | `31-45` | `library-crate-thesis` | `—` |
| `library-module-surface` | `orientation` | `source-library` | `literal` | `no-dependencies` | `46-54` | `library-crate-thesis` | `—` |
| `gate-workspace-value` | `the-gate` | `source-library` | `literal` | `one-lane` | `72-81` | `workspace-value-and-gate` | `—` |
| `workspace-value-and-gate` | `the-gate` | `source-library` | `composite` | `one-lane` | `72-118` | `source-library` | `gate-workspace-value`, `gate-resolve-contract`, `gate-resolve-walk`, `gate-root-accessor`, `gate-main-repo-accessor` |
| `gate-resolve-contract` | `the-gate` | `source-library` | `literal` | `one-lane` | `82-96` | `workspace-value-and-gate` | `—` |
| `gate-resolve-walk` | `the-gate` | `source-library` | `literal` | `one-lane` | `97-105` | `workspace-value-and-gate` | `—` |
| `gate-root-accessor` | `the-gate` | `source-library` | `literal` | `one-lane` | `106-111` | `workspace-value-and-gate` | `—` |
| `gate-main-repo-accessor` | `the-gate` | `source-library` | `literal` | `one-lane` | `112-118` | `workspace-value-and-gate` | `—` |
| `gate-main-repo-premise` | `the-gate` | `source-library` | `literal` | `one-lane` | `275-297` | `gate-main-repo-and-canonical` | `—` |
| `gate-main-repo-and-canonical` | `the-gate` | `source-library` | `composite` | `one-lane` | `275-319` | `source-library` | `gate-main-repo-premise`, `gate-main-repo-probe`, `gate-canonical` |
| `gate-main-repo-probe` | `the-gate` | `source-library` | `literal` | `one-lane` | `298-313` | `gate-main-repo-and-canonical` | `—` |
| `gate-canonical` | `the-gate` | `source-library` | `literal` | `one-lane` | `314-319` | `gate-main-repo-and-canonical` | `—` |
| `source-subprocess` | `source-index` | `source-subprocess` | `root` | `—` | `1-81` | `—` | `subprocess-seam-source` |
| `source-refusal` | `source-index` | `source-refusal` | `root` | `—` | `1-230` | `—` | `refusal-source` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Workspace` | `01-orientation.md#public-surface` | `one-lane` | A resolved workspace is a value whose existence is the proof that the precondition passed; it carries the workspace root and the root of the workspace that holds the repository. | `explained` |
| `Refusal` | `01-orientation.md#public-surface` | `no-remedy-of-its-own` | The one error type: an opaque value carrying what is wrong, where, and the jj command that fixes it, with no matchable variants because every case is a stop. | `pending` |
| `control_dir` | `01-orientation.md#commit-tour` | `no-consumer-vocabulary` | A namespace is one plain directory name the consumer supplies; the directory it names is inside the workspace, untracked, never shared, and created if absent. | `pending` |
| `Commit` | `01-orientation.md#commit-tour` | `no-transactions` | What a taken commit returns: a change id rather than a commit id, because a change id still names the work after a rewrite. | `pending` |
| `is_tracked` | `01-orientation.md#the-six-refusals` | `no-transactions` | The one probe whose answer depends on the working copy, and so the one that lets jj snapshot before answering. | `pending` |
| `jj::output`, `jj::produced_output` | `02-the-gate.md#worked-resolution` | `nothing-ambient` | Every jj invocation the crate makes is built at one seam that fixes the working directory, removes the repository selectors, and separates failure to start from failure to succeed. | `pending` |
| `Refusal::not_a_workspace`, `Refusal::unresolvable_path` | `02-the-gate.md#worked-resolution` | `no-remedy-of-its-own` | Refusal constructors are crate-internal; the gate's refusal names jj's two initialisation commands and states that nothing was created or changed. | `pending` |

<a id="owned-source-totals"></a>
## Owned source totals

| Slice | Page | Owned lines |
|---|---|---:|
| `no-dependencies` | `01-orientation.md` | 98 |
| `one-lane` | `02-the-gate.md` | 92 |
| `nothing-ambient` | `03-subprocess-seam.md` | 81 |
| `no-consumer-vocabulary` | `04-namespace.md` | 58 |
| `no-transactions` | `05-scope-and-commit.md` | 139 |
| `no-remedy-of-its-own` | `06-refusal.md` | 230 |
| `assembly` | `07-what-jj-owns.md` | 0 |
| **Total** | 4 source roots | **698** |
