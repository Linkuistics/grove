# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 57 |
| `source-entry-point` | `crates/grove/src/main.rs` | 14 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 184 |
| `source-configuration-report` | `crates/grove/src/config.rs` | 157 |

<!-- source-root «source-crate-manifest» source="crates/grove/Cargo.toml" lines="1-57" -->
<!-- insert «manifest-thin-by-construction» -->
<!-- /source-root -->
<!-- source-root «source-entry-point» source="crates/grove/src/main.rs" lines="1-14" -->
<!-- insert «entry-point-three-steps» -->
<!-- /source-root -->
<!-- source-root «source-command-surface» source="crates/grove/src/cli.rs" lines="1-184" -->
<!-- insert «surface-grammar» -->
<!-- insert «surface-resolve-lease-run» -->
<!-- insert «surface-closure-tests» -->
<!-- /source-root -->

<!-- source-root «source-configuration-report» source="crates/grove/src/config.rs" lines="1-157" -->
<!-- insert «configuration-report» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-57` | 57 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-14` | 14 | `resolved` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-58` | 58 | `resolved` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `59-98` | 40 | `resolved` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `99-184` | 86 | `resolved` |
| `configuration-report` | `source-configuration-report` | `assembly` | `1-157` | 157 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-57` | `—` | `manifest-thin-by-construction` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `1-8` | `manifest-thin-by-construction` | `—` |
| `manifest-thin-by-construction` | `orientation` | `source-crate-manifest` | `composite` | `compiler-held` | `1-57` | `source-crate-manifest` | `manifest-package-identity`, `manifest-human-binary`, `manifest-crate-not-a-bin`, `manifest-no-lib-one-target`, `manifest-dependencies`, `manifest-tests-live-here`, `manifest-dev-dependencies`, `manifest-lints` |
| `manifest-human-binary` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `9-13` | `manifest-thin-by-construction` | `—` |
| `manifest-crate-not-a-bin` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `14-19` | `manifest-thin-by-construction` | `—` |
| `manifest-no-lib-one-target` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `20-26` | `manifest-thin-by-construction` | `—` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `27-36` | `manifest-thin-by-construction` | `—` |
| `manifest-tests-live-here` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `37-48` | `manifest-thin-by-construction` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `49-54` | `manifest-thin-by-construction` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `55-57` | `manifest-thin-by-construction` | `—` |
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-14` | `—` | `entry-point-three-steps` |
| `entry-point-module-doc` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `1-7` | `entry-point-three-steps` | `—` |
| `entry-point-three-steps` | `three-steps` | `source-entry-point` | `composite` | `one-call` | `1-14` | `source-entry-point` | `entry-point-module-doc`, `entry-point-module-and-main` |
| `entry-point-module-and-main` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `8-14` | `entry-point-three-steps` | `—` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-184` | `—` | `surface-grammar`, `surface-resolve-lease-run`, `surface-closure-tests` |
| `surface-imports` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `1-4` | `surface-grammar` | `—` |
| `surface-grammar` | `the-surface` | `source-command-surface` | `composite` | `no-arguments` | `1-58` | `source-command-surface` | `surface-imports`, `surface-doc-comment`, `surface-clap-attributes`, `surface-empty-struct`, `surface-config-command` |
| `surface-doc-comment` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `5-7` | `surface-grammar` | `—` |
| `surface-clap-attributes` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `8-19` | `surface-grammar` | `—` |
| `surface-empty-struct` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `20-44` | `surface-grammar` | `—` |
| `surface-config-command` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `45-58` | `surface-grammar` | `—` |
| `run-seam-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `59-67` | `surface-resolve-lease-run` | `—` |
| `surface-resolve-lease-run` | `three-steps` | `source-command-surface` | `composite` | `one-call` | `59-98` | `source-command-surface` | `run-seam-doc`, `run-signal-doc`, `run-errors-doc`, `run-three-steps`, `run-call-and-endings` |
| `run-signal-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `68-76` | `surface-resolve-lease-run` | `—` |
| `run-errors-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `77-80` | `surface-resolve-lease-run` | `—` |
| `run-three-steps` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `81-92` | `surface-resolve-lease-run` | `—` |
| `run-call-and-endings` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `93-98` | `surface-resolve-lease-run` | `—` |
| `tests-module-opening` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `99-102` | `surface-closure-tests` | `—` |
| `surface-closure-tests` | `proving-a-negative` | `source-command-surface` | `composite` | `closure-proved` | `99-184` | `source-command-surface` | `tests-module-opening`, `undescribed-doc-purpose`, `undescribed-doc-twice`, `undescribed-doc-empty`, `undescribed-arguments`, `undescribed-subcommands`, `describes-test-doc`, `describes-test`, `closure-test-doc`, `closure-test-subcommands`, `closure-test-arguments` |
| `undescribed-doc-purpose` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `103-106` | `surface-closure-tests` | `—` |
| `undescribed-doc-twice` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `107-116` | `surface-closure-tests` | `—` |
| `undescribed-doc-empty` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `117-119` | `surface-closure-tests` | `—` |
| `undescribed-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `120-129` | `surface-closure-tests` | `—` |
| `undescribed-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `130-140` | `surface-closure-tests` | `—` |
| `describes-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `141-146` | `surface-closure-tests` | `—` |
| `describes-test` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `147-156` | `surface-closure-tests` | `—` |
| `closure-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `157-161` | `surface-closure-tests` | `—` |
| `closure-test-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `162-172` | `surface-closure-tests` | `—` |
| `closure-test-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `173-184` | `surface-closure-tests` | `—` |
| `source-configuration-report` | `source-index` | `source-configuration-report` | `root` | `—` | `1-157` | `—` | `configuration-report` |
| `inspection-load` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `1-18` | `configuration-report` | `—` |
| `configuration-report` | `what-the-call-reaches` | `source-configuration-report` | `composite` | `assembly` | `1-157` | `source-configuration-report` | `inspection-load`, `inspection-labels`, `inspection-selection`, `inspection-words`, `inspection-histories` |
| `inspection-labels` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `19-38` | `configuration-report` | `—` |
| `inspection-selection` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `39-75` | `configuration-report` | `—` |
| `inspection-words` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `76-118` | `configuration-report` | `—` |
| `inspection-histories` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `119-157` | `configuration-report` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `grove_loop::run` | `01-orientation.md#the-binary` | `one-call` | The loop's single entry point; everything the binary does after its three steps is behind this call. | `explained` |
| `DriverLease` | `02-the-surface.md#the-imports` | `one-call` | The one-driver-per-working-tree claim, taken for the life of the process. | `explained` |
| `LoopOutcome` | `02-the-surface.md#the-imports` | `one-call` | Why the loop stopped — the value that decides whether this process exits 0 or dies of a signal. | `explained` |
| `TemplateSource` | `02-the-surface.md#the-imports` | `one-call` | Where launch policy is read from; the loop re-reads it every iteration — twice, before and after the tree transition — rather than holding a copy. | `explained` |
| `Workspace` | `02-the-surface.md#the-imports` | `one-call` | A resolved jj working tree, produced once here and handed to both the lease and the loop. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every source line is credited once to its owning chapter.

| Slice | Page | Owned lines |
|---|---|---:|
| `compiler-held` | `01-orientation.md` | 57 |
| `no-arguments` | `02-the-surface.md` | 58 |
| `one-call` | `03-three-steps.md` | 54 |
| `closure-proved` | `04-proving-a-negative.md` | 86 |
| `assembly` | `05-what-the-call-reaches.md` | 157 |
| **Total** | 4 source roots | **412** |
