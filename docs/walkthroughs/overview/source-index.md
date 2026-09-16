# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 58 |
| `source-entry-point` | `crates/grove/src/main.rs` | 15 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 231 |
| `source-configuration-report` | `crates/grove/src/config.rs` | 165 |
| `source-configuration-json` | `crates/grove/src/config_json.rs` | 167 |

<!-- source-root «source-crate-manifest» source="crates/grove/Cargo.toml" lines="1-58" -->
<!-- insert «manifest-thin-by-construction» -->
<!-- /source-root -->
<!-- source-root «source-entry-point» source="crates/grove/src/main.rs" lines="1-15" -->
<!-- insert «entry-point-three-steps» -->
<!-- /source-root -->
<!-- source-root «source-command-surface» source="crates/grove/src/cli.rs" lines="1-231" -->
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

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-58` | 58 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-15` | 15 | `resolved` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-100` | 100 | `resolved` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `101-139` | 39 | `resolved` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `140-231` | 92 | `resolved` |
| `configuration-report` | `source-configuration-report` | `assembly` | `1-165` | 165 | `resolved` |
| `configuration-json` | `source-configuration-json` | `assembly` | `1-167` | 167 | `resolved` |


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
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-15` | `—` | `entry-point-three-steps` |
| `entry-point-module-doc` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `1-7` | `entry-point-three-steps` | `—` |
| `entry-point-three-steps` | `three-steps` | `source-entry-point` | `composite` | `one-call` | `1-15` | `source-entry-point` | `entry-point-module-doc`, `entry-point-module-and-main` |
| `entry-point-module-and-main` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `8-15` | `entry-point-three-steps` | `—` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-231` | `—` | `surface-grammar`, `surface-resolve-lease-run`, `surface-closure-tests` |
| `surface-imports` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `1-5` | `surface-grammar` | `—` |
| `surface-grammar` | `the-surface` | `source-command-surface` | `composite` | `no-arguments` | `1-100` | `source-command-surface` | `surface-imports`, `surface-doc-comment`, `surface-clap-attributes`, `surface-empty-struct`, `surface-config-command`, `surface-process-reporting` |
| `surface-doc-comment` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `6-8` | `surface-grammar` | `—` |
| `surface-clap-attributes` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `9-20` | `surface-grammar` | `—` |
| `surface-empty-struct` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `21-45` | `surface-grammar` | `—` |
| `surface-config-command` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `46-62` | `surface-grammar` | `—` |
| `surface-process-reporting` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `63-100` | `surface-grammar` | `—` |
| `run-seam-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `101-109` | `surface-resolve-lease-run` | `—` |
| `surface-resolve-lease-run` | `three-steps` | `source-command-surface` | `composite` | `one-call` | `101-139` | `source-command-surface` | `run-seam-doc`, `run-signal-doc`, `run-errors-doc`, `run-three-steps`, `run-call-and-endings` |
| `run-signal-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `110-118` | `surface-resolve-lease-run` | `—` |
| `run-errors-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `119-122` | `surface-resolve-lease-run` | `—` |
| `run-three-steps` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `123-133` | `surface-resolve-lease-run` | `—` |
| `run-call-and-endings` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `134-139` | `surface-resolve-lease-run` | `—` |
| `tests-module-opening` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `140-143` | `surface-closure-tests` | `—` |
| `surface-closure-tests` | `proving-a-negative` | `source-command-surface` | `composite` | `closure-proved` | `140-231` | `source-command-surface` | `tests-module-opening`, `undescribed-doc-purpose`, `undescribed-doc-twice`, `undescribed-doc-empty`, `undescribed-arguments`, `undescribed-subcommands`, `describes-test-doc`, `describes-test`, `closure-test-doc`, `closure-test-subcommands`, `closure-test-arguments` |
| `undescribed-doc-purpose` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `144-147` | `surface-closure-tests` | `—` |
| `undescribed-doc-twice` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `148-157` | `surface-closure-tests` | `—` |
| `undescribed-doc-empty` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `158-160` | `surface-closure-tests` | `—` |
| `undescribed-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `161-170` | `surface-closure-tests` | `—` |
| `undescribed-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `171-181` | `surface-closure-tests` | `—` |
| `describes-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `182-187` | `surface-closure-tests` | `—` |
| `describes-test` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `188-197` | `surface-closure-tests` | `—` |
| `closure-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `198-202` | `surface-closure-tests` | `—` |
| `closure-test-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `203-213` | `surface-closure-tests` | `—` |
| `closure-test-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `214-231` | `surface-closure-tests` | `—` |
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
| `no-arguments` | `02-the-surface.md` | 100 |
| `one-call` | `03-three-steps.md` | 54 |
| `closure-proved` | `04-proving-a-negative.md` | 92 |
| `assembly` | `05-what-the-call-reaches.md` | 332 |
| **Total** | 5 source roots | **636** |

