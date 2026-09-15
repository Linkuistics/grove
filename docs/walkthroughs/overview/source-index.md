# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 55 |
| `source-entry-point` | `crates/grove/src/main.rs` | 13 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 156 |

<!-- source-root «source-crate-manifest» source="crates/grove/Cargo.toml" lines="1-55" -->
<!-- insert «manifest-thin-by-construction» -->
<!-- /source-root -->
<!-- source-root «source-entry-point» source="crates/grove/src/main.rs" lines="1-13" -->
<!-- insert «entry-point-three-steps» -->
<!-- /source-root -->
<!-- source-root «source-command-surface» source="crates/grove/src/cli.rs" lines="1-156" -->
<!-- insert «surface-grammar» -->
<!-- insert «surface-resolve-lease-run» -->
<!-- insert «surface-closure-tests» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-55` | 55 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-13` | 13 | `resolved` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-36` | 36 | `resolved` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `37-73` | 37 | `resolved` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `74-156` | 83 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-55` | `—` | `manifest-thin-by-construction` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `1-8` | `manifest-thin-by-construction` | `—` |
| `manifest-thin-by-construction` | `orientation` | `source-crate-manifest` | `composite` | `compiler-held` | `1-55` | `source-crate-manifest` | `manifest-package-identity`, `manifest-human-binary`, `manifest-crate-not-a-bin`, `manifest-no-lib-one-target`, `manifest-dependencies`, `manifest-tests-live-here`, `manifest-dev-dependencies`, `manifest-lints` |
| `manifest-human-binary` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `9-13` | `manifest-thin-by-construction` | `—` |
| `manifest-crate-not-a-bin` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `14-19` | `manifest-thin-by-construction` | `—` |
| `manifest-no-lib-one-target` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `20-26` | `manifest-thin-by-construction` | `—` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `27-34` | `manifest-thin-by-construction` | `—` |
| `manifest-tests-live-here` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `35-46` | `manifest-thin-by-construction` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `47-52` | `manifest-thin-by-construction` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `53-55` | `manifest-thin-by-construction` | `—` |
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-13` | `—` | `entry-point-three-steps` |
| `entry-point-module-doc` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `1-7` | `entry-point-three-steps` | `—` |
| `entry-point-three-steps` | `three-steps` | `source-entry-point` | `composite` | `one-call` | `1-13` | `source-entry-point` | `entry-point-module-doc`, `entry-point-module-and-main` |
| `entry-point-module-and-main` | `three-steps` | `source-entry-point` | `literal` | `one-call` | `8-13` | `entry-point-three-steps` | `—` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-156` | `—` | `surface-grammar`, `surface-resolve-lease-run`, `surface-closure-tests` |
| `surface-imports` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `1-4` | `surface-grammar` | `—` |
| `surface-grammar` | `the-surface` | `source-command-surface` | `composite` | `no-arguments` | `1-36` | `source-command-surface` | `surface-imports`, `surface-doc-comment`, `surface-clap-attributes`, `surface-empty-struct` |
| `surface-doc-comment` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `5-7` | `surface-grammar` | `—` |
| `surface-clap-attributes` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `8-19` | `surface-grammar` | `—` |
| `surface-empty-struct` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `20-36` | `surface-grammar` | `—` |
| `run-seam-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `37-45` | `surface-resolve-lease-run` | `—` |
| `surface-resolve-lease-run` | `three-steps` | `source-command-surface` | `composite` | `one-call` | `37-73` | `source-command-surface` | `run-seam-doc`, `run-signal-doc`, `run-errors-doc`, `run-three-steps`, `run-call-and-endings` |
| `run-signal-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `46-54` | `surface-resolve-lease-run` | `—` |
| `run-errors-doc` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `55-58` | `surface-resolve-lease-run` | `—` |
| `run-three-steps` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `59-67` | `surface-resolve-lease-run` | `—` |
| `run-call-and-endings` | `three-steps` | `source-command-surface` | `literal` | `one-call` | `68-73` | `surface-resolve-lease-run` | `—` |
| `tests-module-opening` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `74-77` | `surface-closure-tests` | `—` |
| `surface-closure-tests` | `proving-a-negative` | `source-command-surface` | `composite` | `closure-proved` | `74-156` | `source-command-surface` | `tests-module-opening`, `undescribed-doc-purpose`, `undescribed-doc-twice`, `undescribed-doc-empty`, `undescribed-arguments`, `undescribed-subcommands`, `describes-test-doc`, `describes-test`, `closure-test-doc`, `closure-test-subcommands`, `closure-test-arguments` |
| `undescribed-doc-purpose` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `78-81` | `surface-closure-tests` | `—` |
| `undescribed-doc-twice` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `82-91` | `surface-closure-tests` | `—` |
| `undescribed-doc-empty` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `92-94` | `surface-closure-tests` | `—` |
| `undescribed-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `95-104` | `surface-closure-tests` | `—` |
| `undescribed-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `105-115` | `surface-closure-tests` | `—` |
| `describes-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `116-121` | `surface-closure-tests` | `—` |
| `describes-test` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `122-131` | `surface-closure-tests` | `—` |
| `closure-test-doc` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `132-136` | `surface-closure-tests` | `—` |
| `closure-test-subcommands` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `137-144` | `surface-closure-tests` | `—` |
| `closure-test-arguments` | `proving-a-negative` | `source-command-surface` | `literal` | `closure-proved` | `145-156` | `surface-closure-tests` | `—` |

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

Every line of the three source roots is credited once, to the slice whose page
owns it; the table shows how the 224 lines divide across the five chapters, and
its total is what a completed book must account for.

| Slice | Page | Owned lines |
|---|---|---:|
| `compiler-held` | `01-orientation.md` | 55 |
| `no-arguments` | `02-the-surface.md` | 36 |
| `one-call` | `03-three-steps.md` | 50 |
| `closure-proved` | `04-proving-a-negative.md` | 83 |
| `assembly` | `05-what-the-call-reaches.md` | 0 |
| **Total** | 3 source roots | **224** |
