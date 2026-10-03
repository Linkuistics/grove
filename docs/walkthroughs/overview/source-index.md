# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 61 |
| `source-entry-point` | `crates/grove/src/main.rs` | 17 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 188 |
| `source-codex-provisioning` | `crates/grove/src/provision.rs` | 401 |
| `source-standalone` | `crates/grove/src/standalone.rs` | 398 |
| `source-run-display` | `crates/grove/src/run_display.rs` | 119 |
| `source-dispatch-lookup` | `crates/grove/src/dispatch.rs` | 26 |

<!-- source-root «source-crate-manifest» source="crates/grove/Cargo.toml" lines="1-61" -->
<!-- insert «manifest-thin-by-construction» -->
<!-- /source-root -->

<!-- source-root «source-entry-point» source="crates/grove/src/main.rs" lines="1-17" -->
<!-- insert «entry-point-three-steps» -->
<!-- /source-root -->

<!-- source-root «source-command-surface» source="crates/grove/src/cli.rs" lines="1-188" -->
<!-- insert «surface-grammar» -->
<!-- insert «surface-resolve-lease-run» -->
<!-- insert «surface-closure-tests» -->
<!-- /source-root -->

<!-- source-root «source-codex-provisioning» source="crates/grove/src/provision.rs" lines="1-401" -->
<!-- insert «codex-provisioning» -->
<!-- /source-root -->




<!-- source-root «source-standalone» source="crates/grove/src/standalone.rs" lines="1-398" -->
<!-- insert «standalone-invocation» -->
<!-- /source-root -->

<!-- source-root «source-run-display» source="crates/grove/src/run_display.rs" lines="1-119" -->
<!-- insert «standalone-display» -->
<!-- /source-root -->

<!-- source-root «source-dispatch-lookup» source="crates/grove/src/dispatch.rs" lines="1-26" -->
<!-- insert «dispatch-lookup» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-61` | 61 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-17` | 17 | `resolved` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-62` | 62 | `resolved` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `63-105` | 43 | `resolved` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `106-188` | 83 | `resolved` |
| `codex-provisioning` | `source-codex-provisioning` | `one-call` | `1-401` | 401 | `resolved` |
| `standalone-invocation` | `source-standalone` | `isolated-invocation` | `1-398` | 398 | `resolved` |
| `standalone-display` | `source-run-display` | `isolated-invocation` | `1-119` | 119 | `resolved` |
| `dispatch-lookup` | `source-dispatch-lookup` | `isolated-invocation` | `1-26` | 26 | `resolved` |

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
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-17` | `—` | `entry-point-three-steps` |
| `entry-point-module-doc` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `1-7` | `entry-point-three-steps` | `—` |
| `entry-point-three-steps` | `three-steps` | `source-entry-point` | `composite` | `one-call` | `1-17` | `source-entry-point` | `entry-point-module-doc`, `entry-point-module-and-main` |
| `entry-point-module-and-main` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `8-17` | `entry-point-three-steps` | `—` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-188` | `—` | `surface-grammar`, `surface-resolve-lease-run`, `surface-closure-tests` |
| `surface-imports` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `1-5` | `surface-grammar` | `—` |
| `surface-grammar` | `the-surface` | `source-command-surface` | `composite` | `no-arguments` | `1-62` | `source-command-surface` | `surface-imports`, `surface-doc-comment`, `surface-clap-attributes`, `surface-empty-struct`, `surface-process-reporting` |
| `surface-doc-comment` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `6-9` | `surface-grammar` | `—` |
| `surface-clap-attributes` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `10-21` | `surface-grammar` | `—` |
| `surface-empty-struct` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `22-51` | `surface-grammar` | `—` |
| `surface-process-reporting` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `52-62` | `surface-grammar` | `—` |
| `run-seam-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `63-71` | `surface-resolve-lease-run` | `—` |
| `surface-resolve-lease-run` | `three-steps` | `source-command-surface` | `composite` | `one-call` | `63-105` | `source-command-surface` | `run-seam-doc`, `run-signal-doc`, `run-errors-doc`, `run-three-steps`, `run-call-and-endings` |
| `run-signal-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `72-80` | `surface-resolve-lease-run` | `—` |
| `run-errors-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `81-84` | `surface-resolve-lease-run` | `—` |
| `run-three-steps` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `85-99` | `surface-resolve-lease-run` | `—` |
| `run-call-and-endings` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `100-105` | `surface-resolve-lease-run` | `—` |
| `tests-module-opening` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `106-109` | `surface-closure-tests` | `—` |
| `surface-closure-tests` | `proving-a-negative` | `source-command-surface` | `composite` | `closure-proved` | `106-188` | `source-command-surface` | `tests-module-opening`, `undescribed-doc-purpose`, `undescribed-doc-twice`, `undescribed-doc-empty`, `undescribed-arguments`, `undescribed-subcommands`, `describes-test-doc`, `describes-test`, `closure-test-doc`, `closure-test-subcommands`, `closure-test-arguments` |
| `undescribed-doc-purpose` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `110-113` | `surface-closure-tests` | `—` |
| `undescribed-doc-twice` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `114-123` | `surface-closure-tests` | `—` |
| `undescribed-doc-empty` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `124-126` | `surface-closure-tests` | `—` |
| `undescribed-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `127-136` | `surface-closure-tests` | `—` |
| `undescribed-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `137-147` | `surface-closure-tests` | `—` |
| `describes-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `148-153` | `surface-closure-tests` | `—` |
| `describes-test` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `154-163` | `surface-closure-tests` | `—` |
| `closure-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `164-168` | `surface-closure-tests` | `—` |
| `closure-test-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `169-176` | `surface-closure-tests` | `—` |
| `closure-test-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `177-188` | `surface-closure-tests` | `—` |
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
| `source-standalone` | `source-index` | `source-standalone` | `root` | `—` | `1-398` | `—` | `standalone-invocation` |
| `standalone-interface` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `1-64` | `standalone-invocation` | `—` |
| `standalone-invocation` | `standalone-invocations` | `source-standalone` | `composite` | `isolated-invocation` | `1-398` | `source-standalone` | `standalone-interface`, `standalone-staging`, `standalone-launch-context`, `standalone-supervision`, `standalone-selection`, `standalone-artifact-checks`, `standalone-publication` |
| `standalone-staging` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `65-124` | `standalone-invocation` | `—` |
| `standalone-launch-context` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `125-160` | `standalone-invocation` | `—` |
| `standalone-supervision` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `161-242` | `standalone-invocation` | `—` |
| `standalone-selection` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `243-316` | `standalone-invocation` | `—` |
| `standalone-artifact-checks` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `317-362` | `standalone-invocation` | `—` |
| `standalone-publication` | `standalone-invocations` | `source-standalone` | `literal` | `isolated-invocation` | `363-398` | `standalone-invocation` | `—` |
| `source-run-display` | `source-index` | `source-run-display` | `root` | `—` | `1-119` | `—` | `standalone-display` |
| `display-relay` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `1-43` | `standalone-display` | `—` |
| `standalone-display` | `standalone-invocations` | `source-run-display` | `composite` | `isolated-invocation` | `1-119` | `source-run-display` | `display-relay`, `display-pane`, `display-control-test` |
| `display-pane` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `44-116` | `standalone-display` | `—` |
| `display-control-test` | `standalone-invocations` | `source-run-display` | `literal` | `isolated-invocation` | `117-119` | `standalone-display` | `—` |
| `source-dispatch-lookup` | `source-index` | `source-dispatch-lookup` | `root` | `—` | `1-26` | `—` | `dispatch-lookup` |
| `dispatch-lookup` | `standalone-invocations` | `source-dispatch-lookup` | `literal` | `isolated-invocation` | `1-26` | `source-dispatch-lookup` | `—` |

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
| `compiler-held` | `01-orientation.md` | 61 |
| `no-arguments` | `02-the-surface.md` | 62 |
| `one-call` | `03-three-steps.md` | 461 |
| `closure-proved` | `04-proving-a-negative.md` | 83 |
| `assembly` | `05-what-the-call-reaches.md` | 0 |
| `isolated-invocation` | `06-standalone-invocations.md` | 543 |
| **Total** | 7 source roots | **1,210** |
