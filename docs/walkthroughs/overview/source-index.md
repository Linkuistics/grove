# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 58 |
| `source-entry-point` | `crates/grove/src/main.rs` | 16 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 248 |
| `source-configuration-report` | `crates/grove/src/config.rs` | 165 |
| `source-configuration-json` | `crates/grove/src/config_json.rs` | 167 |
| `source-configuration-examples` | `crates/grove/src/examples.rs` | 314 |

<!-- source-root «source-crate-manifest» source="crates/grove/Cargo.toml" lines="1-58" -->
<!-- insert «manifest-thin-by-construction» -->
<!-- /source-root -->

<!-- source-root «source-entry-point» source="crates/grove/src/main.rs" lines="1-16" -->
<!-- insert «entry-point-three-steps» -->
<!-- /source-root -->

<!-- source-root «source-command-surface» source="crates/grove/src/cli.rs" lines="1-248" -->
<!-- insert «surface-grammar» -->
<!-- insert «surface-resolve-lease-run» -->
<!-- insert «surface-closure-tests» -->
<!-- /source-root -->

<!-- source-root «source-configuration-report» source="crates/grove/src/config.rs" lines="1-165" -->
<!-- insert «configuration-report» -->
<!-- /source-root -->

<!-- source-root «source-configuration-json» source="crates/grove/src/config_json.rs" lines="1-167" -->
<!-- insert «configuration-json» -->
<!-- /source-root -->

<!-- source-root «source-configuration-examples» source="crates/grove/src/examples.rs" lines="1-314" -->
<!-- insert «configuration-examples» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-58` | 58 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-16` | 16 | `resolved` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-106` | 106 | `resolved` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `107-148` | 42 | `resolved` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `149-248` | 100 | `resolved` |
| `configuration-report` | `source-configuration-report` | `assembly` | `1-165` | 165 | `resolved` |
| `configuration-json` | `source-configuration-json` | `assembly` | `1-167` | 167 | `resolved` |
| `configuration-examples` | `source-configuration-examples` | `assembly` | `1-314` | 314 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-58` | `—` | `manifest-thin-by-construction` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `1-8` | `manifest-thin-by-construction` | `—` |
| `manifest-thin-by-construction` | `orientation` | `source-crate-manifest` | `composite` | `compiler-held` | `1-58` | `source-crate-manifest` | `manifest-package-identity`, `manifest-human-binary`, `manifest-crate-not-a-bin`, `manifest-no-lib-one-target`, `manifest-dependencies`, `manifest-tests-live-here`, `manifest-dev-dependencies`, `manifest-lints` |
| `manifest-human-binary` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `9-13` | `manifest-thin-by-construction` | `—` |
| `manifest-crate-not-a-bin` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `14-19` | `manifest-thin-by-construction` | `—` |
| `manifest-no-lib-one-target` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `20-26` | `manifest-thin-by-construction` | `—` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `27-36` | `manifest-thin-by-construction` | `—` |
| `manifest-tests-live-here` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `37-49` | `manifest-thin-by-construction` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `50-55` | `manifest-thin-by-construction` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `56-58` | `manifest-thin-by-construction` | `—` |
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-16` | `—` | `entry-point-three-steps` |
| `entry-point-module-doc` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `1-7` | `entry-point-three-steps` | `—` |
| `entry-point-three-steps` | `three-steps` | `source-entry-point` | `composite` | `one-call` | `1-16` | `source-entry-point` | `entry-point-module-doc`, `entry-point-module-and-main` |
| `entry-point-module-and-main` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `8-16` | `entry-point-three-steps` | `—` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-248` | `—` | `surface-grammar`, `surface-resolve-lease-run`, `surface-closure-tests` |
| `surface-imports` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `1-5` | `surface-grammar` | `—` |
| `surface-grammar` | `the-surface` | `source-command-surface` | `composite` | `no-arguments` | `1-106` | `source-command-surface` | `surface-imports`, `surface-doc-comment`, `surface-clap-attributes`, `surface-empty-struct`, `surface-config-command`, `surface-process-reporting` |
| `surface-doc-comment` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `6-8` | `surface-grammar` | `—` |
| `surface-clap-attributes` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `9-20` | `surface-grammar` | `—` |
| `surface-empty-struct` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `21-45` | `surface-grammar` | `—` |
| `surface-config-command` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `46-68` | `surface-grammar` | `—` |
| `surface-process-reporting` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `69-106` | `surface-grammar` | `—` |
| `run-seam-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `107-115` | `surface-resolve-lease-run` | `—` |
| `surface-resolve-lease-run` | `three-steps` | `source-command-surface` | `composite` | `one-call` | `107-148` | `source-command-surface` | `run-seam-doc`, `run-signal-doc`, `run-errors-doc`, `run-three-steps`, `run-call-and-endings` |
| `run-signal-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `116-124` | `surface-resolve-lease-run` | `—` |
| `run-errors-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `125-128` | `surface-resolve-lease-run` | `—` |
| `run-three-steps` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `129-142` | `surface-resolve-lease-run` | `—` |
| `run-call-and-endings` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `143-148` | `surface-resolve-lease-run` | `—` |
| `tests-module-opening` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `149-152` | `surface-closure-tests` | `—` |
| `surface-closure-tests` | `proving-a-negative` | `source-command-surface` | `composite` | `closure-proved` | `149-248` | `source-command-surface` | `tests-module-opening`, `undescribed-doc-purpose`, `undescribed-doc-twice`, `undescribed-doc-empty`, `undescribed-arguments`, `undescribed-subcommands`, `describes-test-doc`, `describes-test`, `closure-test-doc`, `closure-test-subcommands`, `closure-test-arguments` |
| `undescribed-doc-purpose` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `153-156` | `surface-closure-tests` | `—` |
| `undescribed-doc-twice` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `157-166` | `surface-closure-tests` | `—` |
| `undescribed-doc-empty` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `167-169` | `surface-closure-tests` | `—` |
| `undescribed-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `170-179` | `surface-closure-tests` | `—` |
| `undescribed-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `180-190` | `surface-closure-tests` | `—` |
| `describes-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `191-196` | `surface-closure-tests` | `—` |
| `describes-test` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `197-206` | `surface-closure-tests` | `—` |
| `closure-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `207-211` | `surface-closure-tests` | `—` |
| `closure-test-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `212-230` | `surface-closure-tests` | `—` |
| `closure-test-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `231-248` | `surface-closure-tests` | `—` |
| `source-configuration-report` | `source-index` | `source-configuration-report` | `root` | `—` | `1-165` | `—` | `configuration-report` |
| `inspection-load` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `1-26` | `configuration-report` | `—` |
| `configuration-report` | `what-the-call-reaches` | `source-configuration-report` | `composite` | `assembly` | `1-165` | `source-configuration-report` | `inspection-load`, `inspection-labels`, `inspection-selection`, `inspection-words`, `inspection-histories` |
| `inspection-labels` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `27-46` | `configuration-report` | `—` |
| `inspection-selection` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `47-83` | `configuration-report` | `—` |
| `inspection-words` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `84-126` | `configuration-report` | `—` |
| `inspection-histories` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `127-165` | `configuration-report` | `—` |
| `source-configuration-json` | `source-index` | `source-configuration-json` | `root` | `—` | `1-167` | `—` | `configuration-json` |
| `json-native-paths` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `1-25` | `configuration-json` | `—` |
| `configuration-json` | `what-the-call-reaches` | `source-configuration-json` | `composite` | `assembly` | `1-167` | `source-configuration-json` | `json-native-paths`, `json-locations`, `json-assignment-tags`, `json-inspection`, `json-diagnostics`, `json-native-tests` |
| `json-locations` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `26-43` | `configuration-json` | `—` |
| `json-assignment-tags` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `44-70` | `configuration-json` | `—` |
| `json-inspection` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `71-100` | `configuration-json` | `—` |
| `json-diagnostics` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `101-131` | `configuration-json` | `—` |
| `json-native-tests` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `132-167` | `configuration-json` | `—` |
| `source-configuration-examples` | `source-index` | `source-configuration-examples` | `root` | `—` | `1-314` | `—` | `configuration-examples` |
| `examples-package` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `1-39` | `configuration-examples` | `—` |
| `configuration-examples` | `what-the-call-reaches` | `source-configuration-examples` | `composite` | `assembly` | `1-314` | `source-configuration-examples` | `examples-package`, `examples-storage`, `examples-report`, `examples-install`, `examples-run`, `examples-faults`, `examples-race-test`, `examples-write-test`, `examples-refusal-test` |
| `examples-storage` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `40-75` | `configuration-examples` | `—` |
| `examples-report` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `76-96` | `configuration-examples` | `—` |
| `examples-install` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `97-154` | `configuration-examples` | `—` |
| `examples-run` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `155-167` | `configuration-examples` | `—` |
| `examples-faults` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `168-241` | `configuration-examples` | `—` |
| `examples-race-test` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `242-264` | `configuration-examples` | `—` |
| `examples-write-test` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `265-296` | `configuration-examples` | `—` |
| `examples-refusal-test` | `what-the-call-reaches` | `source-configuration-examples` | `literal` | `assembly` | `297-314` | `configuration-examples` | `—` |

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
| `compiler-held` | `01-orientation.md` | 58 |
| `no-arguments` | `02-the-surface.md` | 106 |
| `one-call` | `03-three-steps.md` | 58 |
| `closure-proved` | `04-proving-a-negative.md` | 100 |
| `assembly` | `05-what-the-call-reaches.md` | 646 |
| **Total** | 6 source roots | **968** |
