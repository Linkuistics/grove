# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 59 |
| `source-entry-point` | `crates/grove/src/main.rs` | 17 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 209 |
| `source-codex-provisioning` | `crates/grove/src/provision.rs` | 401 |
| `source-standalone` | `crates/grove/src/standalone.rs` | 332 |
| `source-run-display` | `crates/grove/src/run_display.rs` | 119 |
| `source-dispatch-lookup` | `crates/grove/src/dispatch.rs` | 5 |
| `source-ravel-resolution` | `crates/grove/src/dispatch/ravel.rs` | 330 |

<!-- source-root «source-crate-manifest» source="crates/grove/Cargo.toml" lines="1-59" -->
<!-- insert «manifest-thin-by-construction» -->
<!-- /source-root -->

<!-- source-root «source-entry-point» source="crates/grove/src/main.rs" lines="1-17" -->
<!-- insert «entry-point-three-steps» -->
<!-- /source-root -->

<!-- source-root «source-command-surface» source="crates/grove/src/cli.rs" lines="1-209" -->
<!-- insert «surface-grammar» -->
<!-- insert «surface-resolve-lease-run» -->
<!-- insert «surface-closure-tests» -->
<!-- /source-root -->

<!-- source-root «source-codex-provisioning» source="crates/grove/src/provision.rs" lines="1-401" -->
<!-- insert «codex-provisioning» -->
<!-- /source-root -->




<!-- source-root «source-standalone» source="crates/grove/src/standalone.rs" lines="1-332" -->
<!-- insert «standalone-invocation» -->
<!-- /source-root -->

<!-- source-root «source-run-display» source="crates/grove/src/run_display.rs" lines="1-119" -->
<!-- insert «standalone-display» -->
<!-- /source-root -->

<!-- source-root «source-dispatch-lookup» source="crates/grove/src/dispatch.rs" lines="1-5" -->
<!-- insert «dispatch-lookup» -->
<!-- /source-root -->

<!-- source-root «source-ravel-resolution» source="crates/grove/src/dispatch/ravel.rs" lines="1-330" -->
<!-- insert «ravel-resolution» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-59` | 59 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-17` | 17 | `resolved` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-66` | 66 | `resolved` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `67-126` | 60 | `resolved` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `127-209` | 83 | `resolved` |
| `codex-provisioning` | `source-codex-provisioning` | `one-call` | `1-401` | 401 | `resolved` |
| `standalone-invocation` | `source-standalone` | `isolated-invocation` | `1-332` | 332 | `resolved` |
| `standalone-display` | `source-run-display` | `isolated-invocation` | `1-119` | 119 | `resolved` |
| `dispatch-lookup` | `source-dispatch-lookup` | `isolated-invocation` | `1-5` | 5 | `resolved` |
| `ravel-resolution` | `source-ravel-resolution` | `isolated-invocation` | `1-330` | 330 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-59` | `—` | `manifest-thin-by-construction` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `1-8` | `manifest-thin-by-construction` | `—` |
| `manifest-thin-by-construction` | `orientation` | `source-crate-manifest` | `composite` | `compiler-held` | `1-59` | `source-crate-manifest` | `manifest-package-identity`, `manifest-human-binary`, `manifest-crate-not-a-bin`, `manifest-no-lib-one-target`, `manifest-dependencies`, `manifest-tests-live-here`, `manifest-dev-dependencies`, `manifest-lints` |
| `manifest-human-binary` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `9-13` | `manifest-thin-by-construction` | `—` |
| `manifest-crate-not-a-bin` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `14-20` | `manifest-thin-by-construction` | `—` |
| `manifest-no-lib-one-target` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `21-27` | `manifest-thin-by-construction` | `—` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `28-38` | `manifest-thin-by-construction` | `—` |
| `manifest-tests-live-here` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `39-50` | `manifest-thin-by-construction` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `51-56` | `manifest-thin-by-construction` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `57-59` | `manifest-thin-by-construction` | `—` |
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-17` | `—` | `entry-point-three-steps` |
| `entry-point-module-doc` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `1-7` | `entry-point-three-steps` | `—` |
| `entry-point-three-steps` | `three-steps` | `source-entry-point` | `composite` | `one-call` | `1-17` | `source-entry-point` | `entry-point-module-doc`, `entry-point-module-and-main` |
| `entry-point-module-and-main` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `8-17` | `entry-point-three-steps` | `—` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-209` | `—` | `surface-grammar`, `surface-resolve-lease-run`, `surface-closure-tests` |
| `surface-imports` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `1-5` | `surface-grammar` | `—` |
| `surface-grammar` | `the-surface` | `source-command-surface` | `composite` | `no-arguments` | `1-66` | `source-command-surface` | `surface-imports`, `surface-doc-comment`, `surface-clap-attributes`, `surface-empty-struct`, `surface-process-reporting` |
| `surface-doc-comment` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `6-9` | `surface-grammar` | `—` |
| `surface-clap-attributes` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `10-21` | `surface-grammar` | `—` |
| `surface-empty-struct` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `22-51` | `surface-grammar` | `—` |
| `surface-process-reporting` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `52-66` | `surface-grammar` | `—` |
| `run-seam-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `67-75` | `surface-resolve-lease-run` | `—` |
| `surface-resolve-lease-run` | `three-steps` | `source-command-surface` | `composite` | `one-call` | `67-126` | `source-command-surface` | `run-seam-doc`, `run-signal-doc`, `run-errors-doc`, `run-three-steps`, `run-call-and-endings` |
| `run-signal-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `76-86` | `surface-resolve-lease-run` | `—` |
| `run-errors-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `87-90` | `surface-resolve-lease-run` | `—` |
| `run-three-steps` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `91-105` | `surface-resolve-lease-run` | `—` |
| `run-call-and-endings` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `106-126` | `surface-resolve-lease-run` | `—` |
| `tests-module-opening` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `127-130` | `surface-closure-tests` | `—` |
| `surface-closure-tests` | `proving-a-negative` | `source-command-surface` | `composite` | `closure-proved` | `127-209` | `source-command-surface` | `tests-module-opening`, `undescribed-doc-purpose`, `undescribed-doc-twice`, `undescribed-doc-empty`, `undescribed-arguments`, `undescribed-subcommands`, `describes-test-doc`, `describes-test`, `closure-test-doc`, `closure-test-subcommands`, `closure-test-arguments` |
| `undescribed-doc-purpose` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `131-134` | `surface-closure-tests` | `—` |
| `undescribed-doc-twice` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `135-144` | `surface-closure-tests` | `—` |
| `undescribed-doc-empty` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `145-147` | `surface-closure-tests` | `—` |
| `undescribed-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `148-157` | `surface-closure-tests` | `—` |
| `undescribed-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `158-168` | `surface-closure-tests` | `—` |
| `describes-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `169-174` | `surface-closure-tests` | `—` |
| `describes-test` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `175-184` | `surface-closure-tests` | `—` |
| `closure-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `185-189` | `surface-closure-tests` | `—` |
| `closure-test-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `190-197` | `surface-closure-tests` | `—` |
| `closure-test-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `198-209` | `surface-closure-tests` | `—` |
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
| `source-standalone` | `source-index` | `source-standalone` | `root` | `—` | `1-332` | `—` | `standalone-invocation` |
| `standalone-interface` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `1-59` | `standalone-invocation` | `—` |
| `standalone-invocation` | `standalone-invocations` | `source-standalone` | `composite` | `isolated-invocation` | `1-332` | `source-standalone` | `standalone-interface`, `standalone-staging`, `standalone-launch-context`, `standalone-supervision`, `standalone-artifact-checks`, `standalone-publication` |
| `standalone-staging` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `60-111` | `standalone-invocation` | `—` |
| `standalone-launch-context` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `112-160` | `standalone-invocation` | `—` |
| `standalone-supervision` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `161-221` | `standalone-invocation` | `—` |
| `standalone-artifact-checks` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `222-296` | `standalone-invocation` | `—` |
| `standalone-publication` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `297-332` | `standalone-invocation` | `—` |
| `source-run-display` | `source-index` | `source-run-display` | `root` | `—` | `1-119` | `—` | `standalone-display` |
| `display-relay` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `1-43` | `standalone-display` | `—` |
| `standalone-display` | `standalone-invocations` | `source-run-display` | `composite` | `isolated-invocation` | `1-119` | `source-run-display` | `display-relay`, `display-pane`, `display-control-test` |
| `display-pane` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `44-116` | `standalone-display` | `—` |
| `display-control-test` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `117-119` | `standalone-display` | `—` |
| `source-dispatch-lookup` | `source-index` | `source-dispatch-lookup` | `root` | `—` | `1-5` | `—` | `dispatch-lookup` |
| `dispatch-lookup` | `standalone-invocations` | `source-dispatch-lookup` | `literal` | `isolated-invocation` | `1-5` | `source-dispatch-lookup` | `—` |
| `source-ravel-resolution` | `source-index` | `source-ravel-resolution` | `root` | `—` | `1-330` | `—` | `ravel-resolution` |
| `ravel-resolution` | `standalone-invocations` | `source-ravel-resolution` | `literal` | `isolated-invocation` | `1-330` | `source-ravel-resolution` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Codex provisioning` | `01-orientation.md#two-products` | `one-call` | Bare startup delivers every bundled Codex-compatible skill from an embedded build-time inventory before the loop can launch a child. | `explained` |
| `grove_loop::run` | `01-orientation.md#the-binary` | `one-call` | The loop's single entry point; bare lifecycle execution after its startup checks is behind this call. | `explained` |
| `DriverLease` | `02-the-surface.md#the-imports` | `one-call` | The one-driver-per-working-tree claim, taken for the life of the process. | `explained` |
| `LoopOutcome` | `02-the-surface.md#the-imports` | `one-call` | Why the loop stopped — the value that decides whether this process exits 0 or dies of a signal. | `explained` |
| `Workspace` | `02-the-surface.md#the-imports` | `one-call` | A resolved jj working tree, produced once here and handed to both the lease and the loop. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every source line is credited once to its owning chapter.

| Slice | Page | Owned lines |
|---|---|---:|
| `compiler-held` | `01-orientation.md` | 59 |
| `no-arguments` | `02-the-surface.md` | 66 |
| `one-call` | `03-three-steps.md` | 478 |
| `closure-proved` | `04-proving-a-negative.md` | 83 |
| `assembly` | `05-what-the-call-reaches.md` | 0 |
| `isolated-invocation` | `06-standalone-invocations.md` | 786 |
| **Total** | 8 source roots | **1,472** |
