# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/ordinal-fs-tree/Cargo.toml` | 116 |
| `source-syllabus-cli` | `crates/ordinal-fs-tree/bin/syllabus.rs` | 1,128 |
| `source-library` | `crates/ordinal-fs-tree/src/lib.rs` | 94 |
| `source-conformance` | `crates/ordinal-fs-tree/src/conformance.rs` | 636 |
| `source-error` | `crates/ordinal-fs-tree/src/error.rs` | 342 |
| `source-name` | `crates/ordinal-fs-tree/src/name.rs` | 700 |
| `source-operations` | `crates/ordinal-fs-tree/src/ops.rs` | 543 |
| `source-plan` | `crates/ordinal-fs-tree/src/plan.rs` | 568 |
| `source-reference` | `crates/ordinal-fs-tree/src/reference.rs` | 555 |
| `source-report` | `crates/ordinal-fs-tree/src/report.rs` | 152 |
| `source-snapshot` | `crates/ordinal-fs-tree/src/snapshot.rs` | 650 |
| `source-filesystem-module` | `crates/ordinal-fs-tree/src/fs/mod.rs` | 393 |
| `source-filesystem-read` | `crates/ordinal-fs-tree/src/fs/read.rs` | 179 |
| `source-filesystem-apply` | `crates/ordinal-fs-tree/src/fs/apply.rs` | 471 |
| `source-filesystem-lock` | `crates/ordinal-fs-tree/src/fs/lock.rs` | 91 |

<!-- source-root «source-crate-manifest» source="crates/ordinal-fs-tree/Cargo.toml" lines="1-116" -->
<!-- insert «manifest-package-and-library-dependency» -->
<!-- defer «manifest-cli-feature» owner="syllabus-cli-k17" lines="43-45" -->
<!-- insert «manifest-library-cli-boundary» -->
<!-- defer «manifest-cli-binary» owner="syllabus-cli-k17" lines="62-65" -->
<!-- insert «manifest-development-and-release» -->
<!-- /source-root -->
<!-- source-root «source-syllabus-cli» source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="1-1128" -->
<!-- defer «syllabus-cli-source» owner="syllabus-cli-k17" lines="1-1128" -->
<!-- /source-root -->
<!-- source-root «source-library» source="crates/ordinal-fs-tree/src/lib.rs" lines="1-94" -->
<!-- insert «library-crate-surface» -->
<!-- /source-root -->
<!-- source-root «source-conformance» source="crates/ordinal-fs-tree/src/conformance.rs" lines="1-636" -->
<!-- defer «reference-conformance-source» owner="reference-domain-k13" lines="1-636" -->
<!-- /source-root -->
<!-- source-root «source-error» source="crates/ordinal-fs-tree/src/error.rs" lines="1-342" -->
<!-- defer «filesystem-error-source» owner="filesystem-interpreter-k16" lines="1-342" -->
<!-- /source-root -->
<!-- source-root «source-name» source="crates/ordinal-fs-tree/src/name.rs" lines="1-700" -->
<!-- insert «name-seam-source» -->
<!-- /source-root -->
<!-- source-root «source-operations» source="crates/ordinal-fs-tree/src/ops.rs" lines="1-543" -->
<!-- defer «mutation-operations-source» owner="mutation-algebra-k15" lines="1-543" -->
<!-- /source-root -->
<!-- source-root «source-plan» source="crates/ordinal-fs-tree/src/plan.rs" lines="1-568" -->
<!-- defer «mutation-plan-source» owner="mutation-algebra-k15" lines="1-568" -->
<!-- /source-root -->
<!-- source-root «source-reference» source="crates/ordinal-fs-tree/src/reference.rs" lines="1-555" -->
<!-- defer «reference-domain-source» owner="reference-domain-k13" lines="1-555" -->
<!-- /source-root -->
<!-- source-root «source-report» source="crates/ordinal-fs-tree/src/report.rs" lines="1-152" -->
<!-- defer «mutation-report-source» owner="mutation-algebra-k15" lines="1-152" -->
<!-- /source-root -->
<!-- source-root «source-snapshot» source="crates/ordinal-fs-tree/src/snapshot.rs" lines="1-650" -->
<!-- defer «read-snapshot-source» owner="read-path-k14" lines="1-650" -->
<!-- /source-root -->
<!-- source-root «source-filesystem-module» source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="1-393" -->
<!-- defer «filesystem-read-opening» owner="read-path-k14" lines="1-86" -->
<!-- defer «filesystem-write-acquire» owner="filesystem-interpreter-k16" lines="87-105" -->
<!-- defer «filesystem-read-acquire-and-guard» owner="read-path-k14" lines="106-131" -->
<!-- defer «filesystem-write-guard» owner="filesystem-interpreter-k16" lines="132-154" -->
<!-- defer «filesystem-read-guard-api» owner="read-path-k14" lines="155-168" -->
<!-- defer «filesystem-write-guard-api» owner="filesystem-interpreter-k16" lines="169-378" -->
<!-- defer «filesystem-read-deref» owner="read-path-k14" lines="379-386" -->
<!-- defer «filesystem-write-deref» owner="filesystem-interpreter-k16" lines="387-393" -->
<!-- /source-root -->
<!-- source-root «source-filesystem-read» source="crates/ordinal-fs-tree/src/fs/read.rs" lines="1-179" -->
<!-- defer «read-filesystem-source» owner="read-path-k14" lines="1-179" -->
<!-- /source-root -->
<!-- source-root «source-filesystem-apply» source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="1-471" -->
<!-- defer «filesystem-interpreter-source» owner="filesystem-interpreter-k16" lines="1-471" -->
<!-- /source-root -->
<!-- source-root «source-filesystem-lock» source="crates/ordinal-fs-tree/src/fs/lock.rs" lines="1-91" -->
<!-- defer «filesystem-lock-source» owner="filesystem-interpreter-k16" lines="1-91" -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-package-and-library-dependency` | `source-crate-manifest` | `orientation-k11` | `1-42` | 42 | `resolved` |
| `manifest-cli-feature` | `source-crate-manifest` | `syllabus-cli-k17` | `43-45` | 3 | `deferred` |
| `manifest-library-cli-boundary` | `source-crate-manifest` | `orientation-k11` | `46-61` | 16 | `resolved` |
| `manifest-cli-binary` | `source-crate-manifest` | `syllabus-cli-k17` | `62-65` | 4 | `deferred` |
| `manifest-development-and-release` | `source-crate-manifest` | `orientation-k11` | `66-116` | 51 | `resolved` |
| `syllabus-cli-source` | `source-syllabus-cli` | `syllabus-cli-k17` | `1-1128` | 1,128 | `deferred` |
| `library-crate-surface` | `source-library` | `orientation-k11` | `1-94` | 94 | `resolved` |
| `reference-conformance-source` | `source-conformance` | `reference-domain-k13` | `1-636` | 636 | `deferred` |
| `filesystem-error-source` | `source-error` | `filesystem-interpreter-k16` | `1-342` | 342 | `deferred` |
| `name-seam-source` | `source-name` | `name-seam-k12` | `1-700` | 700 | `resolved` |
| `mutation-operations-source` | `source-operations` | `mutation-algebra-k15` | `1-543` | 543 | `deferred` |
| `mutation-plan-source` | `source-plan` | `mutation-algebra-k15` | `1-568` | 568 | `deferred` |
| `reference-domain-source` | `source-reference` | `reference-domain-k13` | `1-555` | 555 | `deferred` |
| `mutation-report-source` | `source-report` | `mutation-algebra-k15` | `1-152` | 152 | `deferred` |
| `read-snapshot-source` | `source-snapshot` | `read-path-k14` | `1-650` | 650 | `deferred` |
| `filesystem-read-opening` | `source-filesystem-module` | `read-path-k14` | `1-86` | 86 | `deferred` |
| `filesystem-write-acquire` | `source-filesystem-module` | `filesystem-interpreter-k16` | `87-105` | 19 | `deferred` |
| `filesystem-read-acquire-and-guard` | `source-filesystem-module` | `read-path-k14` | `106-131` | 26 | `deferred` |
| `filesystem-write-guard` | `source-filesystem-module` | `filesystem-interpreter-k16` | `132-154` | 23 | `deferred` |
| `filesystem-read-guard-api` | `source-filesystem-module` | `read-path-k14` | `155-168` | 14 | `deferred` |
| `filesystem-write-guard-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `169-378` | 210 | `deferred` |
| `filesystem-read-deref` | `source-filesystem-module` | `read-path-k14` | `379-386` | 8 | `deferred` |
| `filesystem-write-deref` | `source-filesystem-module` | `filesystem-interpreter-k16` | `387-393` | 7 | `deferred` |
| `read-filesystem-source` | `source-filesystem-read` | `read-path-k14` | `1-179` | 179 | `deferred` |
| `filesystem-interpreter-source` | `source-filesystem-apply` | `filesystem-interpreter-k16` | `1-471` | 471 | `deferred` |
| `filesystem-lock-source` | `source-filesystem-lock` | `filesystem-interpreter-k16` | `1-91` | 91 | `deferred` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-116` | `—` | `manifest-package-and-library-dependency`, `manifest-cli-feature`, `manifest-library-cli-boundary`, `manifest-cli-binary`, `manifest-development-and-release` |
| `manifest-package-and-library-dependency` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `1-42` | `source-crate-manifest` | `—` |
| `manifest-library-cli-boundary` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `46-61` | `source-crate-manifest` | `—` |
| `manifest-development-and-release` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `66-116` | `source-crate-manifest` | `—` |
| `source-syllabus-cli` | `source-index` | `source-syllabus-cli` | `root` | `—` | `1-1128` | `—` | `syllabus-cli-source` |
| `source-library` | `source-index` | `source-library` | `root` | `—` | `1-94` | `—` | `library-crate-surface` |
| `library-crate-surface` | `orientation` | `source-library` | `literal` | `orientation-k11` | `1-94` | `source-library` | `—` |
| `source-conformance` | `source-index` | `source-conformance` | `root` | `—` | `1-636` | `—` | `reference-conformance-source` |
| `source-error` | `source-index` | `source-error` | `root` | `—` | `1-342` | `—` | `filesystem-error-source` |
| `source-name` | `source-index` | `source-name` | `root` | `—` | `1-700` | `—` | `name-seam-source` |
| `name-identifiers` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `1-91` | `name-seam-source` | `—` |
| `name-seam-source` | `name-seam` | `source-name` | `composite` | `name-seam-k12` | `1-700` | `source-name` | `name-identifiers`, `name-classification`, `name-representation`, `entry-name-trait`, `entry-name-derived-readings`, `name-component-check` |
| `name-classification` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `92-246` | `name-seam-source` | `—` |
| `name-representation` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `247-345` | `name-seam-source` | `—` |
| `entry-name-trait` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `346-602` | `name-seam-source` | `—` |
| `entry-name-derived-readings` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `603-674` | `name-seam-source` | `—` |
| `name-component-check` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `675-700` | `name-seam-source` | `—` |
| `source-operations` | `source-index` | `source-operations` | `root` | `—` | `1-543` | `—` | `mutation-operations-source` |
| `source-plan` | `source-index` | `source-plan` | `root` | `—` | `1-568` | `—` | `mutation-plan-source` |
| `source-reference` | `source-index` | `source-reference` | `root` | `—` | `1-555` | `—` | `reference-domain-source` |
| `source-report` | `source-index` | `source-report` | `root` | `—` | `1-152` | `—` | `mutation-report-source` |
| `source-snapshot` | `source-index` | `source-snapshot` | `root` | `—` | `1-650` | `—` | `read-snapshot-source` |
| `source-filesystem-module` | `source-index` | `source-filesystem-module` | `root` | `—` | `1-393` | `—` | `filesystem-read-opening`, `filesystem-write-acquire`, `filesystem-read-acquire-and-guard`, `filesystem-write-guard`, `filesystem-read-guard-api`, `filesystem-write-guard-api`, `filesystem-read-deref`, `filesystem-write-deref` |
| `source-filesystem-read` | `source-index` | `source-filesystem-read` | `root` | `—` | `1-179` | `—` | `read-filesystem-source` |
| `source-filesystem-apply` | `source-index` | `source-filesystem-apply` | `root` | `—` | `1-471` | `—` | `filesystem-interpreter-source` |
| `source-filesystem-lock` | `source-index` | `source-filesystem-lock` | `root` | `—` | `1-91` | `—` | `filesystem-lock-source` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Ordinal`, `Key`, `Found`, `Verdict`, `Species`, `EntryName` | `01-orientation.md#working-vocabulary` | `name-seam-k12` | Ordinal is mutable sibling position, key is stable tree identity, observed file kind is not followed, verdict separates foreign, accepted, and refused names, species controls file versus directory shape, and EntryName is the consumer parsing and composition seam. | `explained` |
| `Label`, `Status`, `Parts`, `SyllabusName` | `01-orientation.md#insert-tour` | `reference-domain-k13` | These values are the syllabus consumer's vocabulary and seam implementation, not library defaults. | `pending` |
| `Snapshot`, `Entry`, `ReadGuard` | `01-orientation.md#insert-tour` | `read-path-k14` | A snapshot is the immutable parsed tree captured under a guard, entries are borrowed views, and a read guard couples a shared lock, caller-spelled root, and snapshot. | `pending` |
| `Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, `Report` | `01-orientation.md#insert-tour` | `mutation-algebra-k15` | Target names the root or a stable key, new entry carries opaque parts and optional bytes, every input yields refusal or a guarded ordered plan, and the report records landed effects in its documented orders. | `pending` |
| `WriteGuard`, `Error`, `apply::Faults`, `apply::Run` | `01-orientation.md#insert-tour` | `filesystem-interpreter-k16` | A write guard couples an exclusive lock and snapshot and is consumed by one mutation, errors distinguish refusal, clean rollback, partial rollback, and boundary failure, Faults is a test seam, and Run owns per-plan forward and undo state. | `pending` |
| `Cli`, `Verb`, `Streams`, `Failure` | `01-orientation.md#insert-tour` | `syllabus-cli-k17` | Parsed verbs drive dispatch, stdout is result data, stderr carries advisories and errors, and failure pairs operator-facing text with an exit category. | `pending` |
