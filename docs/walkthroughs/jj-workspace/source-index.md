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
<!-- insert «namespace-reserved-names» -->
<!-- insert «commit-identity» -->
<!-- insert «workspace-value-and-gate» -->
<!-- insert «namespace-control-dir» -->
<!-- insert «scope-tracking-and-commit» -->
<!-- insert «gate-main-repo-and-canonical» -->
<!-- insert «namespace-validation» -->
<!-- /source-root -->
<!-- source-root «source-subprocess» source="crates/jj-workspace/src/jj.rs" lines="1-81" -->
<!-- insert «subprocess-seam-source» -->
<!-- /source-root -->
<!-- source-root «source-refusal» source="crates/jj-workspace/src/refusal.rs" lines="1-230" -->
<!-- insert «refusal-source» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-no-dependencies` | `source-crate-manifest` | `no-dependencies` | `1-44` | 44 | `resolved` |
| `library-crate-thesis` | `source-library` | `no-dependencies` | `1-54` | 54 | `resolved` |
| `namespace-reserved-names` | `source-library` | `no-consumer-vocabulary` | `55-61` | 7 | `resolved` |
| `commit-identity` | `source-library` | `no-transactions` | `62-71` | 10 | `resolved` |
| `workspace-value-and-gate` | `source-library` | `one-lane` | `72-118` | 47 | `resolved` |
| `namespace-control-dir` | `source-library` | `no-consumer-vocabulary` | `119-145` | 27 | `resolved` |
| `scope-tracking-and-commit` | `source-library` | `no-transactions` | `146-274` | 129 | `resolved` |
| `gate-main-repo-and-canonical` | `source-library` | `one-lane` | `275-319` | 45 | `resolved` |
| `namespace-validation` | `source-library` | `no-consumer-vocabulary` | `320-343` | 24 | `resolved` |
| `subprocess-seam-source` | `source-subprocess` | `nothing-ambient` | `1-81` | 81 | `resolved` |
| `refusal-source` | `source-refusal` | `no-remedy-of-its-own` | `1-230` | 230 | `resolved` |

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
| `namespace-owned-names-argument` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `55-59` | `namespace-reserved-names` | `—` |
| `namespace-reserved-names` | `namespace` | `source-library` | `composite` | `no-consumer-vocabulary` | `55-61` | `source-library` | `namespace-owned-names-argument`, `namespace-owned-names-list` |
| `namespace-owned-names-list` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `60-61` | `namespace-reserved-names` | `—` |
| `commit-change-id-argument` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `62-66` | `commit-identity` | `—` |
| `commit-identity` | `scope-and-commit` | `source-library` | `composite` | `no-transactions` | `62-71` | `source-library` | `commit-change-id-argument`, `commit-type` |
| `commit-type` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `67-71` | `commit-identity` | `—` |
| `gate-workspace-value` | `the-gate` | `source-library` | `literal` | `one-lane` | `72-81` | `workspace-value-and-gate` | `—` |
| `workspace-value-and-gate` | `the-gate` | `source-library` | `composite` | `one-lane` | `72-118` | `source-library` | `gate-workspace-value`, `gate-resolve-contract`, `gate-resolve-walk`, `gate-root-accessor`, `gate-main-repo-accessor` |
| `gate-resolve-contract` | `the-gate` | `source-library` | `literal` | `one-lane` | `82-96` | `workspace-value-and-gate` | `—` |
| `gate-resolve-walk` | `the-gate` | `source-library` | `literal` | `one-lane` | `97-105` | `workspace-value-and-gate` | `—` |
| `gate-root-accessor` | `the-gate` | `source-library` | `literal` | `one-lane` | `106-111` | `workspace-value-and-gate` | `—` |
| `gate-main-repo-accessor` | `the-gate` | `source-library` | `literal` | `one-lane` | `112-118` | `workspace-value-and-gate` | `—` |
| `namespace-postcondition` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `119-123` | `namespace-control-dir` | `—` |
| `namespace-control-dir` | `namespace` | `source-library` | `composite` | `no-consumer-vocabulary` | `119-145` | `source-library` | `namespace-postcondition`, `namespace-placement`, `namespace-shape`, `namespace-no-probe`, `namespace-control-dir-body` |
| `namespace-placement` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `124-129` | `namespace-control-dir` | `—` |
| `namespace-shape` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `130-134` | `namespace-control-dir` | `—` |
| `namespace-no-probe` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `135-138` | `namespace-control-dir` | `—` |
| `namespace-control-dir-body` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `139-145` | `namespace-control-dir` | `—` |
| `tracking-contract` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `146-161` | `scope-tracking-and-commit` | `—` |
| `scope-tracking-and-commit` | `scope-and-commit` | `source-library` | `composite` | `no-transactions` | `146-274` | `source-library` | `tracking-contract`, `tracking-probe`, `commit-contract`, `commit-scope-guard`, `commit-invocation`, `commit-change-id-read`, `commit-return`, `fileset-contract`, `fileset-quoting`, `relative-contract`, `relative-absolute`, `relative-strip-or-canonical-parent`, `relative-root-is-not-a-scope`, `relative-render` |
| `tracking-probe` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `162-166` | `scope-tracking-and-commit` | `—` |
| `commit-contract` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `167-177` | `scope-tracking-and-commit` | `—` |
| `commit-scope-guard` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `178-186` | `scope-tracking-and-commit` | `—` |
| `commit-invocation` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `187-191` | `scope-tracking-and-commit` | `—` |
| `commit-change-id-read` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `192-206` | `scope-tracking-and-commit` | `—` |
| `commit-return` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `207-211` | `scope-tracking-and-commit` | `—` |
| `fileset-contract` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `212-217` | `scope-tracking-and-commit` | `—` |
| `fileset-quoting` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `218-230` | `scope-tracking-and-commit` | `—` |
| `relative-contract` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `231-238` | `scope-tracking-and-commit` | `—` |
| `relative-absolute` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `239-244` | `scope-tracking-and-commit` | `—` |
| `relative-strip-or-canonical-parent` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `245-259` | `scope-tracking-and-commit` | `—` |
| `relative-root-is-not-a-scope` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `260-264` | `scope-tracking-and-commit` | `—` |
| `relative-render` | `scope-and-commit` | `source-library` | `literal` | `no-transactions` | `265-274` | `scope-tracking-and-commit` | `—` |
| `gate-main-repo-premise` | `the-gate` | `source-library` | `literal` | `one-lane` | `275-297` | `gate-main-repo-and-canonical` | `—` |
| `gate-main-repo-and-canonical` | `the-gate` | `source-library` | `composite` | `one-lane` | `275-319` | `source-library` | `gate-main-repo-premise`, `gate-main-repo-probe`, `gate-canonical` |
| `gate-main-repo-probe` | `the-gate` | `source-library` | `literal` | `one-lane` | `298-313` | `gate-main-repo-and-canonical` | `—` |
| `gate-canonical` | `the-gate` | `source-library` | `literal` | `one-lane` | `314-319` | `gate-main-repo-and-canonical` | `—` |
| `namespace-validation-empty` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `320-323` | `namespace-validation` | `—` |
| `namespace-validation` | `namespace` | `source-library` | `composite` | `no-consumer-vocabulary` | `320-343` | `source-library` | `namespace-validation-empty`, `namespace-validation-path`, `namespace-validation-self-reference`, `namespace-validation-owned`, `namespace-validation-accept` |
| `namespace-validation-path` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `324-329` | `namespace-validation` | `—` |
| `namespace-validation-self-reference` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `330-335` | `namespace-validation` | `—` |
| `namespace-validation-owned` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `336-341` | `namespace-validation` | `—` |
| `namespace-validation-accept` | `namespace` | `source-library` | `literal` | `no-consumer-vocabulary` | `342-343` | `namespace-validation` | `—` |
| `source-subprocess` | `source-index` | `source-subprocess` | `root` | `—` | `1-81` | `—` | `subprocess-seam-source` |
| `subprocess-seam-purpose` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `1-6` | `subprocess-seam-source` | `—` |
| `subprocess-seam-source` | `subprocess-seam` | `source-subprocess` | `composite` | `nothing-ambient` | `1-81` | `source-subprocess` | `subprocess-seam-purpose`, `subprocess-nothing-ambient`, `subprocess-consumer-environment`, `subprocess-imports`, `subprocess-selectors`, `subprocess-output`, `subprocess-produced-output`, `subprocess-raw-output-build`, `subprocess-raw-output-endings`, `subprocess-rendered` |
| `subprocess-nothing-ambient` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `7-15` | `subprocess-seam-source` | `—` |
| `subprocess-consumer-environment` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `16-20` | `subprocess-seam-source` | `—` |
| `subprocess-imports` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `21-24` | `subprocess-seam-source` | `—` |
| `subprocess-selectors` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `25-33` | `subprocess-seam-source` | `—` |
| `subprocess-output` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `34-43` | `subprocess-seam-source` | `—` |
| `subprocess-produced-output` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `44-49` | `subprocess-seam-source` | `—` |
| `subprocess-raw-output-build` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `50-57` | `subprocess-seam-source` | `—` |
| `subprocess-raw-output-endings` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `58-69` | `subprocess-seam-source` | `—` |
| `subprocess-rendered` | `subprocess-seam` | `source-subprocess` | `literal` | `nothing-ambient` | `70-81` | `subprocess-seam-source` | `—` |
| `source-refusal` | `source-index` | `source-refusal` | `root` | `—` | `1-230` | `—` | `refusal-source` |
| `refusal-module-thesis` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `1-13` | `refusal-source` | `—` |
| `refusal-source` | `refusal` | `source-refusal` | `composite` | `no-remedy-of-its-own` | `1-230` | `source-refusal` | `refusal-module-thesis`, `refusal-imports`, `refusal-opaque-type`, `refusal-kind-open`, `refusal-kind-gate`, `refusal-kind-namespace`, `refusal-kind-scope`, `refusal-kind-seam`, `refusal-kind-commit`, `refusal-constructors-gate`, `refusal-constructors-namespace`, `refusal-constructors-scope`, `refusal-constructors-seam`, `refusal-constructors-commit`, `refusal-display-open`, `refusal-display-gate`, `refusal-display-namespace`, `refusal-display-scope`, `refusal-display-seam`, `refusal-display-commit`, `refusal-error-source-caused`, `refusal-error-source-uncaused` |
| `refusal-imports` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `14-18` | `refusal-source` | `—` |
| `refusal-opaque-type` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `19-29` | `refusal-source` | `—` |
| `refusal-kind-open` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `30-31` | `refusal-source` | `—` |
| `refusal-kind-gate` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `32-35` | `refusal-source` | `—` |
| `refusal-kind-namespace` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `36-39` | `refusal-source` | `—` |
| `refusal-kind-scope` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `40-43` | `refusal-source` | `—` |
| `refusal-kind-seam` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `44-53` | `refusal-source` | `—` |
| `refusal-kind-commit` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `54-57` | `refusal-source` | `—` |
| `refusal-constructors-gate` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `58-70` | `refusal-source` | `—` |
| `refusal-constructors-namespace` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `71-84` | `refusal-source` | `—` |
| `refusal-constructors-scope` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `85-97` | `refusal-source` | `—` |
| `refusal-constructors-seam` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `98-118` | `refusal-source` | `—` |
| `refusal-constructors-commit` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `119-127` | `refusal-source` | `—` |
| `refusal-display-open` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `128-130` | `refusal-source` | `—` |
| `refusal-display-gate` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `131-152` | `refusal-source` | `—` |
| `refusal-display-namespace` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `153-165` | `refusal-source` | `—` |
| `refusal-display-scope` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `166-179` | `refusal-source` | `—` |
| `refusal-display-seam` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `180-194` | `refusal-source` | `—` |
| `refusal-display-commit` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `195-214` | `refusal-source` | `—` |
| `refusal-error-source-caused` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `215-221` | `refusal-source` | `—` |
| `refusal-error-source-uncaused` | `refusal` | `source-refusal` | `literal` | `no-remedy-of-its-own` | `222-230` | `refusal-source` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Workspace` | `01-orientation.md#public-surface` | `one-lane` | A resolved workspace is a value whose existence is the proof that the precondition passed; it carries the workspace root and the root of the workspace that holds the repository. | `explained` |
| `Refusal` | `01-orientation.md#public-surface` | `no-remedy-of-its-own` | The one error type: an opaque value carrying what is wrong, where, and the jj command that fixes it, with no matchable variants because every case is a stop. | `explained` |
| `control_dir` | `01-orientation.md#commit-tour` | `no-consumer-vocabulary` | A namespace is one plain directory name the consumer supplies; the directory it names is inside the workspace, untracked, never shared, and created if absent. | `explained` |
| `Commit` | `01-orientation.md#commit-tour` | `no-transactions` | What a taken commit returns: a change id rather than a commit id, because a change id still names the work after a rewrite. | `explained` |
| `is_tracked` | `01-orientation.md#the-six-refusals` | `no-transactions` | The one probe whose answer depends on the working copy, and so the one that lets jj snapshot before answering. | `explained` |
| `jj::output`, `jj::produced_output` | `02-the-gate.md#worked-resolution` | `nothing-ambient` | Every jj invocation the crate makes is built at one seam that fixes the working directory, removes the repository selectors, and separates failure to start from failure to succeed. | `explained` |
| `Refusal::not_a_workspace`, `Refusal::unresolvable_path` | `02-the-gate.md#worked-resolution` | `no-remedy-of-its-own` | Refusal constructors are crate-internal; the gate's refusal names jj's two initialisation commands and states that nothing was created or changed. | `explained` |
| `Refusal::not_runnable`, `Refusal::command_failed`, `Refusal::output_not_text` | `03-subprocess-seam.md#worked-invocation` | `no-remedy-of-its-own` | The seam's three refusals: jj could not be started and the remedy is installation; jj started and declined, and the remedy is the stderr it printed; or its output was not text and its answer cannot be read. | `explained` |
| `Refusal::namespace`, `Refusal::control_dir` | `04-namespace.md#worked-reservation` | `no-remedy-of-its-own` | The namespace's two refusals: a name the crate will not reserve, carrying the name and one short reason with a fixed remedy paragraph; and a directory the filesystem would not create, carrying the path and keeping the `io::Error` as the refusal's `source()`. | `explained` |
| `Refusal::not_scoped`, `Refusal::outside_workspace`, `Refusal::commit_not_recorded` | `05-scope-and-commit.md#worked-commit` | `no-remedy-of-its-own` | Scope and commit's three refusals: a path-scoped operation given no scope; a path the workspace does not answer for, naming the root it was compared against; and the one refusal about state rather than about a command, which means the commit is absent and names jj's operation-log repair while keeping the seam's own refusal as its cause. | `explained` |

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
