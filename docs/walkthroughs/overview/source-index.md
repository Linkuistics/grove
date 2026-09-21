# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 61 |
| `source-entry-point` | `crates/grove/src/main.rs` | 19 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 268 |
| `source-codex-provisioning` | `crates/grove/src/provision.rs` | 401 |
| `source-configuration-report` | `crates/grove/src/config.rs` | 160 |
| `source-configuration-json` | `crates/grove/src/config_json.rs` | 163 |
| `source-configuration-examples` | `crates/grove/src/examples.rs` | 314 |
| `source-standalone` | `crates/grove/src/standalone.rs` | 338 |
| `source-run-display` | `crates/grove/src/run_display.rs` | 119 |

<!-- source-root «source-crate-manifest» source="crates/grove/Cargo.toml" lines="1-61" -->
<!-- insert «manifest-thin-by-construction» -->
<!-- /source-root -->

<!-- source-root «source-entry-point» source="crates/grove/src/main.rs" lines="1-19" -->
<!-- insert «entry-point-three-steps» -->
<!-- /source-root -->

<!-- source-root «source-command-surface» source="crates/grove/src/cli.rs" lines="1-268" -->
<!-- insert «surface-grammar» -->
<!-- insert «surface-resolve-lease-run» -->
<!-- insert «surface-closure-tests» -->
<!-- /source-root -->

<!-- source-root «source-codex-provisioning» source="crates/grove/src/provision.rs" lines="1-401" -->
<!-- insert «codex-provisioning» -->
<!-- /source-root -->

<!-- source-root «source-configuration-report» source="crates/grove/src/config.rs" lines="1-160" -->
<!-- insert «configuration-report» -->
<!-- /source-root -->

<!-- source-root «source-configuration-json» source="crates/grove/src/config_json.rs" lines="1-163" -->
<!-- insert «configuration-json» -->
<!-- /source-root -->

<!-- source-root «source-configuration-examples» source="crates/grove/src/examples.rs" lines="1-314" -->
<!-- insert «configuration-examples» -->
<!-- /source-root -->

<!-- source-root «source-standalone» source="crates/grove/src/standalone.rs" lines="1-338" -->
<!-- insert «standalone-invocation» -->
<!-- /source-root -->

<!-- source-root «source-run-display» source="crates/grove/src/run_display.rs" lines="1-119" -->
<!-- insert «standalone-display» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-61` | 61 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-19` | 19 | `resolved` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-119` | 119 | `resolved` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `120-168` | 49 | `resolved` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `169-268` | 100 | `resolved` |
| `codex-provisioning` | `source-codex-provisioning` | `one-call` | `1-401` | 401 | `resolved` |
| `configuration-report` | `source-configuration-report` | `assembly` | `1-160` | 160 | `resolved` |
| `configuration-json` | `source-configuration-json` | `assembly` | `1-163` | 163 | `resolved` |
| `configuration-examples` | `source-configuration-examples` | `assembly` | `1-314` | 314 | `resolved` |
| `standalone-invocation` | `source-standalone` | `isolated-invocation` | `1-338` | 338 | `resolved` |
| `standalone-display` | `source-run-display` | `isolated-invocation` | `1-119` | 119 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-61` | `—` | `manifest-thin-by-construction` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `1-8` | `manifest-thin-by-construction` | `—` |
| `manifest-thin-by-construction` | `orientation` | `source-crate-manifest` | `composite` | `compiler-held` | `1-61` | `source-crate-manifest` | `manifest-package-identity`, `manifest-human-binary`, `manifest-crate-not-a-bin`, `manifest-no-lib-one-target`, `manifest-dependencies`, `manifest-tests-live-here`, `manifest-dev-dependencies`, `manifest-lints` |
| `manifest-human-binary` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `9-13` | `manifest-thin-by-construction` | `—` |
| `manifest-crate-not-a-bin` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `14-20` | `manifest-thin-by-construction` | `—` |
| `manifest-no-lib-one-target` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `21-27` | `manifest-thin-by-construction` | `—` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `28-40` | `manifest-thin-by-construction` | `—` |
| `manifest-tests-live-here` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `41-52` | `manifest-thin-by-construction` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `53-58` | `manifest-thin-by-construction` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `59-61` | `manifest-thin-by-construction` | `—` |
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-19` | `—` | `entry-point-three-steps` |
| `entry-point-module-doc` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `1-7` | `entry-point-three-steps` | `—` |
| `entry-point-three-steps` | `three-steps` | `source-entry-point` | `composite` | `one-call` | `1-19` | `source-entry-point` | `entry-point-module-doc`, `entry-point-module-and-main` |
| `entry-point-module-and-main` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `8-19` | `entry-point-three-steps` | `—` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-268` | `—` | `surface-grammar`, `surface-resolve-lease-run`, `surface-closure-tests` |
| `surface-imports` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `1-5` | `surface-grammar` | `—` |
| `surface-grammar` | `the-surface` | `source-command-surface` | `composite` | `no-arguments` | `1-119` | `source-command-surface` | `surface-imports`, `surface-doc-comment`, `surface-clap-attributes`, `surface-empty-struct`, `surface-config-command`, `surface-process-reporting` |
| `surface-doc-comment` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `6-8` | `surface-grammar` | `—` |
| `surface-clap-attributes` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `9-20` | `surface-grammar` | `—` |
| `surface-empty-struct` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `21-58` | `surface-grammar` | `—` |
| `surface-config-command` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `59-81` | `surface-grammar` | `—` |
| `surface-process-reporting` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `82-119` | `surface-grammar` | `—` |
| `run-seam-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `120-128` | `surface-resolve-lease-run` | `—` |
| `surface-resolve-lease-run` | `three-steps` | `source-command-surface` | `composite` | `one-call` | `120-168` | `source-command-surface` | `run-seam-doc`, `run-signal-doc`, `run-errors-doc`, `run-three-steps`, `run-call-and-endings` |
| `run-signal-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `129-137` | `surface-resolve-lease-run` | `—` |
| `run-errors-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `138-141` | `surface-resolve-lease-run` | `—` |
| `run-three-steps` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `142-162` | `surface-resolve-lease-run` | `—` |
| `run-call-and-endings` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `163-168` | `surface-resolve-lease-run` | `—` |
| `tests-module-opening` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `169-172` | `surface-closure-tests` | `—` |
| `surface-closure-tests` | `proving-a-negative` | `source-command-surface` | `composite` | `closure-proved` | `169-268` | `source-command-surface` | `tests-module-opening`, `undescribed-doc-purpose`, `undescribed-doc-twice`, `undescribed-doc-empty`, `undescribed-arguments`, `undescribed-subcommands`, `describes-test-doc`, `describes-test`, `closure-test-doc`, `closure-test-subcommands`, `closure-test-arguments` |
| `undescribed-doc-purpose` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `173-176` | `surface-closure-tests` | `—` |
| `undescribed-doc-twice` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `177-186` | `surface-closure-tests` | `—` |
| `undescribed-doc-empty` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `187-189` | `surface-closure-tests` | `—` |
| `undescribed-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `190-199` | `surface-closure-tests` | `—` |
| `undescribed-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `200-210` | `surface-closure-tests` | `—` |
| `describes-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `211-216` | `surface-closure-tests` | `—` |
| `describes-test` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `217-226` | `surface-closure-tests` | `—` |
| `closure-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `227-231` | `surface-closure-tests` | `—` |
| `closure-test-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `232-250` | `surface-closure-tests` | `—` |
| `closure-test-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `251-268` | `surface-closure-tests` | `—` |
| `source-codex-provisioning` | `source-index` | `source-codex-provisioning` | `root` | `—` | `1-401` | `—` | `codex-provisioning` |
| `provision-entry-and-detection` | `three-steps` | `source-codex-provisioning` | `literal` | `one-call` | `1-41` | `codex-provisioning` | `—` |
| `codex-provisioning` | `three-steps` | `source-codex-provisioning` | `composite` | `one-call` | `1-401` | `source-codex-provisioning` | `provision-entry-and-detection`, `provision-filesystem-guards`, `provision-lock`, `provision-link-ownership`, `provision-snapshot-comparison`, `provision-link-publication`, `provision-install-flow`, `provision-unit-tests` |
| `provision-filesystem-guards` | `three-steps` | `source-codex-provisioning` | `literal` | `one-call` | `42-63` | `codex-provisioning` | `—` |
| `provision-lock` | `three-steps` | `source-codex-provisioning` | `literal` | `one-call` | `64-96` | `codex-provisioning` | `—` |
| `provision-link-ownership` | `three-steps` | `source-codex-provisioning` | `literal` | `one-call` | `97-132` | `codex-provisioning` | `—` |
| `provision-snapshot-comparison` | `three-steps` | `source-codex-provisioning` | `literal` | `one-call` | `133-178` | `codex-provisioning` | `—` |
| `provision-link-publication` | `three-steps` | `source-codex-provisioning` | `literal` | `one-call` | `179-190` | `codex-provisioning` | `—` |
| `provision-install-flow` | `three-steps` | `source-codex-provisioning` | `literal` | `one-call` | `191-307` | `codex-provisioning` | `—` |
| `provision-unit-tests` | `three-steps` | `source-codex-provisioning` | `literal` | `one-call` | `308-401` | `codex-provisioning` | `—` |
| `source-configuration-report` | `source-index` | `source-configuration-report` | `root` | `—` | `1-160` | `—` | `configuration-report` |
| `inspection-load` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `1-26` | `configuration-report` | `—` |
| `configuration-report` | `what-the-call-reaches` | `source-configuration-report` | `composite` | `assembly` | `1-160` | `source-configuration-report` | `inspection-load`, `inspection-labels`, `inspection-selection`, `inspection-words`, `inspection-histories` |
| `inspection-labels` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `27-46` | `configuration-report` | `—` |
| `inspection-selection` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `47-83` | `configuration-report` | `—` |
| `inspection-words` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `84-125` | `configuration-report` | `—` |
| `inspection-histories` | `what-the-call-reaches` | `source-configuration-report` | `literal` | `assembly` | `126-160` | `configuration-report` | `—` |
| `source-configuration-json` | `source-index` | `source-configuration-json` | `root` | `—` | `1-163` | `—` | `configuration-json` |
| `json-native-paths` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `1-25` | `configuration-json` | `—` |
| `configuration-json` | `what-the-call-reaches` | `source-configuration-json` | `composite` | `assembly` | `1-163` | `source-configuration-json` | `json-native-paths`, `json-locations`, `json-assignment-tags`, `json-inspection`, `json-diagnostics`, `json-native-tests` |
| `json-locations` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `26-43` | `configuration-json` | `—` |
| `json-assignment-tags` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `44-66` | `configuration-json` | `—` |
| `json-inspection` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `67-96` | `configuration-json` | `—` |
| `json-diagnostics` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `97-127` | `configuration-json` | `—` |
| `json-native-tests` | `what-the-call-reaches` | `source-configuration-json` | `literal` | `assembly` | `128-163` | `configuration-json` | `—` |
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
| `source-standalone` | `source-index` | `source-standalone` | `root` | `—` | `1-338` | `—` | `standalone-invocation` |
| `standalone-interface` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `1-56` | `standalone-invocation` | `—` |
| `standalone-invocation` | `standalone-invocations` | `source-standalone` | `composite` | `isolated-invocation` | `1-338` | `source-standalone` | `standalone-interface`, `standalone-policy`, `standalone-launch-context`, `standalone-supervision`, `standalone-artifact-checks`, `standalone-publication` |
| `standalone-policy` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `57-121` | `standalone-invocation` | `—` |
| `standalone-launch-context` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `122-178` | `standalone-invocation` | `—` |
| `standalone-supervision` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `179-252` | `standalone-invocation` | `—` |
| `standalone-artifact-checks` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `253-298` | `standalone-invocation` | `—` |
| `standalone-publication` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `299-338` | `standalone-invocation` | `—` |
| `source-run-display` | `source-index` | `source-run-display` | `root` | `—` | `1-119` | `—` | `standalone-display` |
| `display-relay` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `1-43` | `standalone-display` | `—` |
| `standalone-display` | `standalone-invocations` | `source-run-display` | `composite` | `isolated-invocation` | `1-119` | `source-run-display` | `display-relay`, `display-pane`, `display-control-test` |
| `display-pane` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `44-116` | `standalone-display` | `—` |
| `display-control-test` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `117-119` | `standalone-display` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Codex provisioning` | `01-orientation.md#two-products` | `one-call` | Bare startup delivers every bundled Codex-compatible skill from an embedded build-time inventory before the loop can launch a child. | `explained` |
| `grove_loop::run` | `01-orientation.md#the-binary` | `one-call` | The loop's single entry point; bare lifecycle execution after its startup checks is behind this call. | `explained` |
| `DriverLease` | `02-the-surface.md#the-imports` | `one-call` | The one-driver-per-working-tree claim, taken for the life of the process. | `explained` |
| `LoopOutcome` | `02-the-surface.md#the-imports` | `one-call` | Why the loop stopped — the value that decides whether this process exits 0 or dies of a signal. | `explained` |
| `TemplateSource` | `02-the-surface.md#the-imports` | `one-call` | Where launch policy is read from; the loop re-reads it every iteration — twice, before and after the tree transition — rather than holding a copy. | `explained` |
| `Workspace` | `02-the-surface.md#the-imports` | `one-call` | A resolved jj working tree, produced once here and handed to both the lease and the loop. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every source line is credited once to its owning chapter.

| Slice | Page | Owned lines |
|---|---|---:|
| `compiler-held` | `01-orientation.md` | 61 |
| `no-arguments` | `02-the-surface.md` | 119 |
| `one-call` | `03-three-steps.md` | 469 |
| `closure-proved` | `04-proving-a-negative.md` | 100 |
| `assembly` | `05-what-the-call-reaches.md` | 637 |
| `isolated-invocation` | `06-standalone-invocations.md` | 457 |
| **Total** | 9 source roots | **1,843** |
