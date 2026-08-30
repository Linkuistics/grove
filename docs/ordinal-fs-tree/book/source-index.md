# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/ordinal-fs-tree/Cargo.toml` | 116 |
| `source-syllabus-cli` | `crates/ordinal-fs-tree/bin/syllabus.rs` | 1,439 |
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
<!-- insert «manifest-cli-feature» -->
<!-- insert «manifest-library-cli-boundary» -->
<!-- insert «manifest-cli-binary» -->
<!-- insert «manifest-development-and-release» -->
<!-- /source-root -->
<!-- source-root «source-syllabus-cli» source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="1-1439" -->
<!-- insert «syllabus-cli-source» -->
<!-- /source-root -->
<!-- source-root «source-library» source="crates/ordinal-fs-tree/src/lib.rs" lines="1-94" -->
<!-- insert «library-crate-surface» -->
<!-- /source-root -->
<!-- source-root «source-conformance» source="crates/ordinal-fs-tree/src/conformance.rs" lines="1-636" -->
<!-- insert «reference-conformance-source» -->
<!-- /source-root -->
<!-- source-root «source-error» source="crates/ordinal-fs-tree/src/error.rs" lines="1-342" -->
<!-- insert «filesystem-error-source» -->
<!-- /source-root -->
<!-- source-root «source-name» source="crates/ordinal-fs-tree/src/name.rs" lines="1-700" -->
<!-- insert «name-seam-source» -->
<!-- /source-root -->
<!-- source-root «source-operations» source="crates/ordinal-fs-tree/src/ops.rs" lines="1-543" -->
<!-- insert «mutation-operations-source» -->
<!-- /source-root -->
<!-- source-root «source-plan» source="crates/ordinal-fs-tree/src/plan.rs" lines="1-568" -->
<!-- insert «mutation-plan-source» -->
<!-- /source-root -->
<!-- source-root «source-reference» source="crates/ordinal-fs-tree/src/reference.rs" lines="1-555" -->
<!-- insert «reference-domain-source» -->
<!-- /source-root -->
<!-- source-root «source-report» source="crates/ordinal-fs-tree/src/report.rs" lines="1-152" -->
<!-- insert «mutation-report-source» -->
<!-- /source-root -->
<!-- source-root «source-snapshot» source="crates/ordinal-fs-tree/src/snapshot.rs" lines="1-650" -->
<!-- insert «read-snapshot-source» -->
<!-- /source-root -->
<!-- source-root «source-filesystem-module» source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="1-393" -->
<!-- insert «filesystem-read-opening» -->
<!-- insert «filesystem-write-acquire» -->
<!-- insert «filesystem-read-acquire-and-guard» -->
<!-- insert «filesystem-write-guard» -->
<!-- insert «filesystem-read-guard-api» -->
<!-- insert «filesystem-write-guard-api» -->
<!-- insert «filesystem-read-deref» -->
<!-- insert «filesystem-write-deref» -->
<!-- /source-root -->
<!-- source-root «source-filesystem-read» source="crates/ordinal-fs-tree/src/fs/read.rs" lines="1-179" -->
<!-- insert «read-filesystem-source» -->
<!-- /source-root -->
<!-- source-root «source-filesystem-apply» source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="1-471" -->
<!-- insert «filesystem-interpreter-source» -->
<!-- /source-root -->
<!-- source-root «source-filesystem-lock» source="crates/ordinal-fs-tree/src/fs/lock.rs" lines="1-91" -->
<!-- insert «filesystem-lock-source» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-package-and-library-dependency` | `source-crate-manifest` | `orientation-k11` | `1-42` | 42 | `resolved` |
| `manifest-cli-feature` | `source-crate-manifest` | `syllabus-cli-k17` | `43-45` | 3 | `resolved` |
| `manifest-library-cli-boundary` | `source-crate-manifest` | `orientation-k11` | `46-61` | 16 | `resolved` |
| `manifest-cli-binary` | `source-crate-manifest` | `syllabus-cli-k17` | `62-65` | 4 | `resolved` |
| `manifest-development-and-release` | `source-crate-manifest` | `orientation-k11` | `66-116` | 51 | `resolved` |
| `syllabus-cli-source` | `source-syllabus-cli` | `syllabus-cli-k17` | `1-1439` | 1,439 | `resolved` |
| `library-crate-surface` | `source-library` | `orientation-k11` | `1-94` | 94 | `resolved` |
| `reference-conformance-source` | `source-conformance` | `reference-domain-k13` | `1-636` | 636 | `resolved` |
| `filesystem-error-source` | `source-error` | `filesystem-interpreter-k16` | `1-342` | 342 | `resolved` |
| `name-seam-source` | `source-name` | `name-seam-k12` | `1-700` | 700 | `resolved` |
| `mutation-operations-source` | `source-operations` | `mutation-algebra-k15` | `1-543` | 543 | `resolved` |
| `mutation-plan-source` | `source-plan` | `mutation-algebra-k15` | `1-568` | 568 | `resolved` |
| `reference-domain-source` | `source-reference` | `reference-domain-k13` | `1-555` | 555 | `resolved` |
| `mutation-report-source` | `source-report` | `mutation-algebra-k15` | `1-152` | 152 | `resolved` |
| `read-snapshot-source` | `source-snapshot` | `read-path-k14` | `1-650` | 650 | `resolved` |
| `filesystem-read-opening` | `source-filesystem-module` | `read-path-k14` | `1-86` | 86 | `resolved` |
| `filesystem-write-acquire` | `source-filesystem-module` | `filesystem-interpreter-k16` | `87-105` | 19 | `resolved` |
| `filesystem-read-acquire-and-guard` | `source-filesystem-module` | `read-path-k14` | `106-131` | 26 | `resolved` |
| `filesystem-write-guard` | `source-filesystem-module` | `filesystem-interpreter-k16` | `132-154` | 23 | `resolved` |
| `filesystem-read-guard-api` | `source-filesystem-module` | `read-path-k14` | `155-168` | 14 | `resolved` |
| `filesystem-write-guard-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `169-378` | 210 | `resolved` |
| `filesystem-read-deref` | `source-filesystem-module` | `read-path-k14` | `379-386` | 8 | `resolved` |
| `filesystem-write-deref` | `source-filesystem-module` | `filesystem-interpreter-k16` | `387-393` | 7 | `resolved` |
| `read-filesystem-source` | `source-filesystem-read` | `read-path-k14` | `1-179` | 179 | `resolved` |
| `filesystem-interpreter-source` | `source-filesystem-apply` | `filesystem-interpreter-k16` | `1-471` | 471 | `resolved` |
| `filesystem-lock-source` | `source-filesystem-lock` | `filesystem-interpreter-k16` | `1-91` | 91 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-116` | `—` | `manifest-package-and-library-dependency`, `manifest-cli-feature`, `manifest-library-cli-boundary`, `manifest-cli-binary`, `manifest-development-and-release` |
| `manifest-package-and-library-dependency` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `1-42` | `source-crate-manifest` | `—` |
| `manifest-cli-feature` | `syllabus-cli` | `source-crate-manifest` | `literal` | `syllabus-cli-k17` | `43-45` | `source-crate-manifest` | `—` |
| `manifest-library-cli-boundary` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `46-61` | `source-crate-manifest` | `—` |
| `manifest-cli-binary` | `syllabus-cli` | `source-crate-manifest` | `literal` | `syllabus-cli-k17` | `62-65` | `source-crate-manifest` | `—` |
| `manifest-development-and-release` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `66-116` | `source-crate-manifest` | `—` |
| `source-syllabus-cli` | `source-index` | `source-syllabus-cli` | `root` | `—` | `1-1439` | `—` | `syllabus-cli-source` |
| `cli-command-line` | `syllabus-cli` | `source-syllabus-cli` | `literal` | `syllabus-cli-k17` | `1-488` | `syllabus-cli-source` | `—` |
| `syllabus-cli-source` | `syllabus-cli` | `source-syllabus-cli` | `composite` | `syllabus-cli-k17` | `1-1439` | `source-syllabus-cli` | `cli-command-line`, `cli-parsing-and-failure`, `cli-streams-and-paths`, `cli-mutation-output`, `cli-main-dispatch`, `cli-reading`, `cli-mutations`, `cli-stream-contract-tests` |
| `cli-parsing-and-failure` | `syllabus-cli` | `source-syllabus-cli` | `literal` | `syllabus-cli-k17` | `489-632` | `syllabus-cli-source` | `—` |
| `cli-streams-and-paths` | `syllabus-cli` | `source-syllabus-cli` | `literal` | `syllabus-cli-k17` | `633-859` | `syllabus-cli-source` | `—` |
| `cli-mutation-output` | `syllabus-cli` | `source-syllabus-cli` | `literal` | `syllabus-cli-k17` | `860-904` | `syllabus-cli-source` | `—` |
| `cli-main-dispatch` | `syllabus-cli` | `source-syllabus-cli` | `literal` | `syllabus-cli-k17` | `905-997` | `syllabus-cli-source` | `—` |
| `cli-reading` | `syllabus-cli` | `source-syllabus-cli` | `literal` | `syllabus-cli-k17` | `998-1136` | `syllabus-cli-source` | `—` |
| `cli-mutations` | `syllabus-cli` | `source-syllabus-cli` | `literal` | `syllabus-cli-k17` | `1137-1259` | `syllabus-cli-source` | `—` |
| `cli-stream-contract-tests` | `syllabus-cli` | `source-syllabus-cli` | `literal` | `syllabus-cli-k17` | `1260-1439` | `syllabus-cli-source` | `—` |
| `source-library` | `source-index` | `source-library` | `root` | `—` | `1-94` | `—` | `library-crate-surface` |
| `library-crate-surface` | `orientation` | `source-library` | `literal` | `orientation-k11` | `1-94` | `source-library` | `—` |
| `source-conformance` | `source-index` | `source-conformance` | `root` | `—` | `1-636` | `—` | `reference-conformance-source` |
| `conformance-obligations` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `1-177` | `reference-conformance-source` | `—` |
| `reference-conformance-source` | `reference-domain` | `source-conformance` | `composite` | `reference-domain-k13` | `1-636` | `source-conformance` | `conformance-obligations`, `conformance-report`, `conformance-compose-and-canonical`, `conformance-component-and-distinguished`, `conformance-found-agreement` |
| `conformance-report` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `178-282` | `reference-conformance-source` | `—` |
| `conformance-compose-and-canonical` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `283-442` | `reference-conformance-source` | `—` |
| `conformance-component-and-distinguished` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `443-541` | `reference-conformance-source` | `—` |
| `conformance-found-agreement` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `542-636` | `reference-conformance-source` | `—` |
| `source-error` | `source-index` | `source-error` | `root` | `—` | `1-342` | `—` | `filesystem-error-source` |
| `error-boundary` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `1-23` | `filesystem-error-source` | `—` |
| `filesystem-error-source` | `filesystem-interpreter` | `source-error` | `composite` | `filesystem-interpreter-k16` | `1-342` | `source-error` | `error-boundary`, `error-taxonomy`, `error-debug`, `error-display`, `error-sources` |
| `error-taxonomy` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `24-163` | `filesystem-error-source` | `—` |
| `error-debug` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `164-238` | `filesystem-error-source` | `—` |
| `error-display` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `239-322` | `filesystem-error-source` | `—` |
| `error-sources` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `323-342` | `filesystem-error-source` | `—` |
| `source-name` | `source-index` | `source-name` | `root` | `—` | `1-700` | `—` | `name-seam-source` |
| `name-identifiers` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `1-91` | `name-seam-source` | `—` |
| `name-seam-source` | `name-seam` | `source-name` | `composite` | `name-seam-k12` | `1-700` | `source-name` | `name-identifiers`, `name-classification`, `name-representation`, `entry-name-trait`, `entry-name-derived-readings`, `name-component-check` |
| `name-classification` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `92-246` | `name-seam-source` | `—` |
| `name-representation` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `247-345` | `name-seam-source` | `—` |
| `entry-name-trait` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `346-602` | `name-seam-source` | `—` |
| `entry-name-derived-readings` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `603-674` | `name-seam-source` | `—` |
| `name-component-check` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `675-700` | `name-seam-source` | `—` |
| `source-operations` | `source-index` | `source-operations` | `root` | `—` | `1-543` | `—` | `mutation-operations-source` |
| `ops-surface-and-inputs` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `1-68` | `mutation-operations-source` | `—` |
| `mutation-operations-source` | `mutation-algebra` | `source-operations` | `composite` | `mutation-algebra-k15` | `1-543` | `source-operations` | `ops-surface-and-inputs`, `ops-append`, `ops-insert`, `ops-promote`, `ops-rewrite`, `ops-resolution-and-allocation` |
| `ops-append` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `69-144` | `mutation-operations-source` | `—` |
| `ops-insert` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `145-251` | `mutation-operations-source` | `—` |
| `ops-promote` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `252-396` | `mutation-operations-source` | `—` |
| `ops-rewrite` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `397-476` | `mutation-operations-source` | `—` |
| `ops-resolution-and-allocation` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `477-543` | `mutation-operations-source` | `—` |
| `source-plan` | `source-index` | `source-plan` | `root` | `—` | `1-568` | `—` | `mutation-plan-source` |
| `plan-effects` | `mutation-algebra` | `source-plan` | `literal` | `mutation-algebra-k15` | `1-118` | `mutation-plan-source` | `—` |
| `mutation-plan-source` | `mutation-algebra` | `source-plan` | `composite` | `mutation-algebra-k15` | `1-568` | `source-plan` | `plan-effects`, `plan-guarded`, `plan-decision-and-refusals`, `plan-refusal-messages` |
| `plan-guarded` | `mutation-algebra` | `source-plan` | `literal` | `mutation-algebra-k15` | `119-229` | `mutation-plan-source` | `—` |
| `plan-decision-and-refusals` | `mutation-algebra` | `source-plan` | `literal` | `mutation-algebra-k15` | `230-419` | `mutation-plan-source` | `—` |
| `plan-refusal-messages` | `mutation-algebra` | `source-plan` | `literal` | `mutation-algebra-k15` | `420-568` | `mutation-plan-source` | `—` |
| `source-reference` | `source-index` | `source-reference` | `root` | `—` | `1-555` | `—` | `reference-domain-source` |
| `reference-vocabulary` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `1-210` | `reference-domain-source` | `—` |
| `reference-domain-source` | `reference-domain` | `source-reference` | `composite` | `reference-domain-k13` | `1-555` | `source-reference` | `reference-vocabulary`, `reference-name-and-errors`, `reference-parser`, `reference-seam-methods`, `reference-parser-helpers` |
| `reference-name-and-errors` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `211-351` | `reference-domain-source` | `—` |
| `reference-parser` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `352-473` | `reference-domain-source` | `—` |
| `reference-seam-methods` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `474-505` | `reference-domain-source` | `—` |
| `reference-parser-helpers` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `506-555` | `reference-domain-source` | `—` |
| `source-report` | `source-index` | `source-report` | `root` | `—` | `1-152` | `—` | `mutation-report-source` |
| `report-structure-and-order` | `mutation-algebra` | `source-report` | `literal` | `mutation-algebra-k15` | `1-119` | `mutation-report-source` | `—` |
| `mutation-report-source` | `mutation-algebra` | `source-report` | `composite` | `mutation-algebra-k15` | `1-152` | `source-report` | `report-structure-and-order`, `report-debug` |
| `report-debug` | `mutation-algebra` | `source-report` | `literal` | `mutation-algebra-k15` | `120-152` | `mutation-report-source` | `—` |
| `source-snapshot` | `source-index` | `source-snapshot` | `root` | `—` | `1-650` | `—` | `read-snapshot-source` |
| `snapshot-storage` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `1-104` | `read-snapshot-source` | `—` |
| `read-snapshot-source` | `read-path` | `source-snapshot` | `composite` | `read-path-k14` | `1-650` | `source-snapshot` | `snapshot-storage`, `snapshot-builder`, `snapshot-entry-views`, `snapshot-containers`, `snapshot-queries` |
| `snapshot-builder` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `105-267` | `read-snapshot-source` | `—` |
| `snapshot-entry-views` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `268-438` | `read-snapshot-source` | `—` |
| `snapshot-containers` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `439-532` | `read-snapshot-source` | `—` |
| `snapshot-queries` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `533-650` | `read-snapshot-source` | `—` |
| `source-filesystem-module` | `source-index` | `source-filesystem-module` | `root` | `—` | `1-393` | `—` | `filesystem-read-opening`, `filesystem-write-acquire`, `filesystem-read-acquire-and-guard`, `filesystem-write-guard`, `filesystem-read-guard-api`, `filesystem-write-guard-api`, `filesystem-read-deref`, `filesystem-write-deref` |
| `filesystem-read-opening` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `1-86` | `source-filesystem-module` | `—` |
| `filesystem-write-acquire` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `87-105` | `source-filesystem-module` | `—` |
| `filesystem-read-acquire-and-guard` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `106-131` | `source-filesystem-module` | `—` |
| `filesystem-write-guard` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `132-154` | `source-filesystem-module` | `—` |
| `filesystem-read-guard-api` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `155-168` | `source-filesystem-module` | `—` |
| `write-guard-accessors` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `169-181` | `filesystem-write-guard-api` | `—` |
| `filesystem-write-guard-api` | `filesystem-interpreter` | `source-filesystem-module` | `composite` | `filesystem-interpreter-k16` | `169-378` | `source-filesystem-module` | `write-guard-accessors`, `write-guard-append`, `write-guard-insert`, `write-guard-promote`, `write-guard-rewrite`, `write-guard-dispatch` |
| `write-guard-append` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `182-220` | `filesystem-write-guard-api` | `—` |
| `write-guard-insert` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `221-255` | `filesystem-write-guard-api` | `—` |
| `write-guard-promote` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `256-318` | `filesystem-write-guard-api` | `—` |
| `write-guard-rewrite` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `319-364` | `filesystem-write-guard-api` | `—` |
| `write-guard-dispatch` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `365-378` | `filesystem-write-guard-api` | `—` |
| `filesystem-read-deref` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `379-386` | `source-filesystem-module` | `—` |
| `filesystem-write-deref` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `387-393` | `source-filesystem-module` | `—` |
| `source-filesystem-read` | `source-index` | `source-filesystem-read` | `root` | `—` | `1-179` | `—` | `read-filesystem-source` |
| `read-tree-discovery` | `read-path` | `source-filesystem-read` | `literal` | `read-path-k14` | `1-81` | `read-filesystem-source` | `—` |
| `read-filesystem-source` | `read-path` | `source-filesystem-read` | `composite` | `read-path-k14` | `1-179` | `source-filesystem-read` | `read-tree-discovery`, `read-directory-listing`, `read-lock-location` |
| `read-directory-listing` | `read-path` | `source-filesystem-read` | `literal` | `read-path-k14` | `82-124` | `read-filesystem-source` | `—` |
| `read-lock-location` | `read-path` | `source-filesystem-read` | `literal` | `read-path-k14` | `125-179` | `read-filesystem-source` | `—` |
| `source-filesystem-apply` | `source-index` | `source-filesystem-apply` | `root` | `—` | `1-471` | `—` | `filesystem-interpreter-source` |
| `apply-contract` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `1-49` | `filesystem-interpreter-source` | `—` |
| `filesystem-interpreter-source` | `filesystem-interpreter` | `source-filesystem-apply` | `composite` | `filesystem-interpreter-k16` | `1-471` | `source-filesystem-apply` | `apply-contract`, `apply-plan`, `apply-run-state`, `apply-effect-step`, `apply-unwind-and-paths`, `apply-undo`, `apply-destination-claim`, `apply-fault-seam` |
| `apply-plan` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `50-87` | `filesystem-interpreter-source` | `—` |
| `apply-run-state` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `88-105` | `filesystem-interpreter-source` | `—` |
| `apply-effect-step` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `106-211` | `filesystem-interpreter-source` | `—` |
| `apply-unwind-and-paths` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `212-270` | `filesystem-interpreter-source` | `—` |
| `apply-undo` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `271-330` | `filesystem-interpreter-source` | `—` |
| `apply-destination-claim` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `331-371` | `filesystem-interpreter-source` | `—` |
| `apply-fault-seam` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `372-471` | `filesystem-interpreter-source` | `—` |
| `source-filesystem-lock` | `source-index` | `source-filesystem-lock` | `root` | `—` | `1-91` | `—` | `filesystem-lock-source` |
| `lock-contract` | `filesystem-interpreter` | `source-filesystem-lock` | `literal` | `filesystem-interpreter-k16` | `1-40` | `filesystem-lock-source` | `—` |
| `filesystem-lock-source` | `filesystem-interpreter` | `source-filesystem-lock` | `composite` | `filesystem-interpreter-k16` | `1-91` | `source-filesystem-lock` | `lock-contract`, `lock-modes`, `lock-take` |
| `lock-modes` | `filesystem-interpreter` | `source-filesystem-lock` | `literal` | `filesystem-interpreter-k16` | `41-58` | `filesystem-lock-source` | `—` |
| `lock-take` | `filesystem-interpreter` | `source-filesystem-lock` | `literal` | `filesystem-interpreter-k16` | `59-91` | `filesystem-lock-source` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Ordinal`, `Key`, `Found`, `Verdict`, `Species`, `EntryName` | `01-orientation.md#working-vocabulary` | `name-seam-k12` | Ordinal is mutable sibling position, key is stable tree identity, observed file kind is not followed, verdict separates foreign, accepted, and refused names, species controls file versus directory shape, and EntryName is the consumer parsing and composition seam. | `explained` |
| `manifest-cli-binary` | `01-orientation.md#package-contract` | `syllabus-cli-k17` | The binary declaration is CLI-owned and deferred; it maps the demonstration executable to its external consumer source and requires the CLI feature. | `explained` |
| `manifest-cli-feature` | `01-orientation.md#package-contract` | `syllabus-cli-k17` | The optional parser dependency is activated by a later CLI-owned feature range, enabled by default while library consumers may disable default features. | `explained` |
| `Label`, `Status`, `reference::Parts`, `SyllabusName` | `01-orientation.md#insert-tour` | `reference-domain-k13` | These values are the syllabus consumer's vocabulary and seam implementation, not library defaults. | `explained` |
| `Snapshot`, `Entry`, `ReadGuard` | `01-orientation.md#insert-tour` | `read-path-k14` | A snapshot is the immutable parsed tree captured under a guard, entries are borrowed views, and a read guard couples a shared lock, caller-spelled root, and snapshot. | `explained` |
| `Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, `Report` | `01-orientation.md#insert-tour` | `mutation-algebra-k15` | Target names the root or a stable key, new entry carries opaque parts and optional bytes, every input yields refusal or a guarded ordered plan, and the report records landed effects in its documented orders. | `explained` |
| `WriteGuard`, `Error`, `apply::Faults`, `apply::Run` | `01-orientation.md#insert-tour` | `filesystem-interpreter-k16` | A write guard couples an exclusive lock and snapshot and is consumed by one mutation, errors distinguish refusal, clean rollback, partial rollback, and boundary failure, Faults is a test seam, and Run owns per-plan forward and undo state. | `explained` |
| `Cli`, `Verb`, `Streams`, `Failure` | `01-orientation.md#insert-tour` | `syllabus-cli-k17` | Parsed verbs drive dispatch, stdout is result data, stderr carries advisories and errors, and failure pairs operator-facing text with an exit category. | `explained` |
| `conformance` | `02-name-seam.md#entry-name-contract` | `reference-domain-k13` | The reusable conformance kit exercises the five semantic `EntryName` obligations that Rust cannot discharge on consumer-supplied samples; the reference-domain chapter defines its scope and limits. | `explained` |
