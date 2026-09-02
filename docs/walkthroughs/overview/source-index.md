# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove/Cargo.toml` | 54 |
| `source-entry-point` | `crates/grove/src/main.rs` | 13 |
| `source-command-surface` | `crates/grove/src/cli.rs` | 137 |

<!-- source-root «source-crate-manifest» source="crates/grove/Cargo.toml" lines="1-54" -->
<!-- insert «manifest-thin-by-construction» -->
<!-- /source-root -->
<!-- source-root «source-entry-point» source="crates/grove/src/main.rs" lines="1-13" -->
<!-- defer «entry-point-three-steps» owner="one-call" lines="1-13" -->
<!-- /source-root -->
<!-- source-root «source-command-surface» source="crates/grove/src/cli.rs" lines="1-137" -->
<!-- insert «surface-grammar» -->
<!-- defer «surface-resolve-lease-run» owner="one-call" lines="20-53" -->
<!-- defer «surface-closure-tests» owner="closure-proved" lines="54-137" -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-construction` | `source-crate-manifest` | `compiler-held` | `1-54` | 54 | `resolved` |
| `entry-point-three-steps` | `source-entry-point` | `one-call` | `1-13` | 13 | `deferred` |
| `surface-grammar` | `source-command-surface` | `no-arguments` | `1-19` | 19 | `resolved` |
| `surface-resolve-lease-run` | `source-command-surface` | `one-call` | `20-53` | 34 | `deferred` |
| `surface-closure-tests` | `source-command-surface` | `closure-proved` | `54-137` | 84 | `deferred` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-54` | `—` | `manifest-thin-by-construction` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `1-8` | `manifest-thin-by-construction` | `—` |
| `manifest-thin-by-construction` | `orientation` | `source-crate-manifest` | `composite` | `compiler-held` | `1-54` | `source-crate-manifest` | `manifest-package-identity`, `manifest-human-binary`, `manifest-crate-not-a-bin`, `manifest-no-lib-one-target`, `manifest-dependencies`, `manifest-tests-live-here`, `manifest-dev-dependencies`, `manifest-lints` |
| `manifest-human-binary` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `9-13` | `manifest-thin-by-construction` | `—` |
| `manifest-crate-not-a-bin` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `14-19` | `manifest-thin-by-construction` | `—` |
| `manifest-no-lib-one-target` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `20-26` | `manifest-thin-by-construction` | `—` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `27-33` | `manifest-thin-by-construction` | `—` |
| `manifest-tests-live-here` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `34-45` | `manifest-thin-by-construction` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `46-51` | `manifest-thin-by-construction` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `compiler-held` | `52-54` | `manifest-thin-by-construction` | `—` |
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-13` | `—` | `entry-point-three-steps` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-137` | `—` | `surface-grammar`, `surface-resolve-lease-run`, `surface-closure-tests` |
| `surface-imports` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `1-2` | `surface-grammar` | `—` |
| `surface-grammar` | `the-surface` | `source-command-surface` | `composite` | `no-arguments` | `1-19` | `source-command-surface` | `surface-imports`, `surface-doc-comment`, `surface-clap-attributes`, `surface-empty-struct` |
| `surface-doc-comment` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `3-7` | `surface-grammar` | `—` |
| `surface-clap-attributes` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `8-18` | `surface-grammar` | `—` |
| `surface-empty-struct` | `the-surface` | `source-command-surface` | `literal` | `no-arguments` | `19-19` | `surface-grammar` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `grove_loop::run` | `01-orientation.md#the-binary` | `one-call` | The loop's single entry point; everything the binary does after its three steps is behind this call. | `pending` |
| `DriverLease` | `02-the-surface.md#the-imports` | `one-call` | The one-driver-per-working-tree claim, taken for the life of the process. | `pending` |
| `LoopOutcome` | `02-the-surface.md#the-imports` | `one-call` | Why the loop stopped — the value that decides whether this process exits 0 or dies of a signal. | `pending` |
| `TemplateSource` | `02-the-surface.md#the-imports` | `one-call` | Where launch policy is read from; the loop re-reads it once per iteration. | `pending` |
| `Workspace` | `02-the-surface.md#the-imports` | `one-call` | A resolved jj working tree, produced once here and handed to both the lease and the loop. | `pending` |

<a id="owned-source-totals"></a>
## Owned source totals

Every line of the three source roots is credited once, to the slice whose page
owns it; the table shows how the 204 lines divide across the five chapters, and
its total is what a completed book must account for.

| Slice | Page | Owned lines |
|---|---|---:|
| `compiler-held` | `01-orientation.md` | 54 |
| `no-arguments` | `02-the-surface.md` | 19 |
| `one-call` | `03-three-steps.md` | 47 |
| `closure-proved` | `04-proving-a-negative.md` | 84 |
| `assembly` | `05-what-the-call-reaches.md` | 0 |
| **Total** | 3 source roots | **204** |
