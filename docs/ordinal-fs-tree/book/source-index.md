# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/ordinal-fs-tree/Cargo.toml` | 112 |
| `source-syllabus-cli` | `crates/ordinal-fs-tree/bin/syllabus.rs` | 1,439 |
| `source-library` | `crates/ordinal-fs-tree/src/lib.rs` | 103 |
| `source-conformance` | `crates/ordinal-fs-tree/src/conformance.rs` | 667 |
| `source-error` | `crates/ordinal-fs-tree/src/error.rs` | 510 |
| `source-name` | `crates/ordinal-fs-tree/src/name.rs` | 717 |
| `source-operations` | `crates/ordinal-fs-tree/src/ops.rs` | 634 |
| `source-plan` | `crates/ordinal-fs-tree/src/plan.rs` | 597 |
| `source-reference` | `crates/ordinal-fs-tree/src/reference.rs` | 559 |
| `source-report` | `crates/ordinal-fs-tree/src/report.rs` | 186 |
| `source-snapshot` | `crates/ordinal-fs-tree/src/snapshot.rs` | 677 |
| `source-sought` | `crates/ordinal-fs-tree/src/sought.rs` | 132 |
| `source-filesystem-module` | `crates/ordinal-fs-tree/src/fs/mod.rs` | 827 |
| `source-filesystem-read` | `crates/ordinal-fs-tree/src/fs/read.rs` | 407 |
| `source-filesystem-apply` | `crates/ordinal-fs-tree/src/fs/apply.rs` | 488 |
| `source-filesystem-remove` | `crates/ordinal-fs-tree/src/fs/remove.rs` | 275 |
| `source-filesystem-lock` | `crates/ordinal-fs-tree/src/fs/lock.rs` | 91 |

<!-- source-root «source-crate-manifest» source="crates/ordinal-fs-tree/Cargo.toml" lines="1-112" -->
<!-- insert «manifest-package-and-library-dependency» -->
<!-- insert «manifest-cli-feature» -->
<!-- insert «manifest-library-cli-boundary» -->
<!-- insert «manifest-cli-binary» -->
<!-- insert «manifest-development-and-release» -->
<!-- /source-root -->
<!-- source-root «source-syllabus-cli» source="crates/ordinal-fs-tree/bin/syllabus.rs" lines="1-1439" -->
<!-- insert «syllabus-cli-source» -->
<!-- /source-root -->
<!-- source-root «source-library» source="crates/ordinal-fs-tree/src/lib.rs" lines="1-103" -->
<!-- insert «library-crate-surface» -->
<!-- /source-root -->
<!-- source-root «source-conformance» source="crates/ordinal-fs-tree/src/conformance.rs" lines="1-667" -->
<!-- insert «reference-conformance-source» -->
<!-- /source-root -->
<!-- source-root «source-error» source="crates/ordinal-fs-tree/src/error.rs" lines="1-510" -->
<!-- insert «filesystem-error-source» -->
<!-- /source-root -->
<!-- source-root «source-name» source="crates/ordinal-fs-tree/src/name.rs" lines="1-717" -->
<!-- insert «name-seam-source» -->
<!-- /source-root -->
<!-- source-root «source-operations» source="crates/ordinal-fs-tree/src/ops.rs" lines="1-634" -->
<!-- insert «mutation-operations-source» -->
<!-- /source-root -->
<!-- source-root «source-plan» source="crates/ordinal-fs-tree/src/plan.rs" lines="1-597" -->
<!-- insert «mutation-plan-source» -->
<!-- /source-root -->
<!-- source-root «source-reference» source="crates/ordinal-fs-tree/src/reference.rs" lines="1-559" -->
<!-- insert «reference-domain-source» -->
<!-- /source-root -->
<!-- source-root «source-report» source="crates/ordinal-fs-tree/src/report.rs" lines="1-186" -->
<!-- insert «mutation-report-source» -->
<!-- /source-root -->
<!-- source-root «source-snapshot» source="crates/ordinal-fs-tree/src/snapshot.rs" lines="1-677" -->
<!-- insert «read-snapshot-source» -->
<!-- /source-root -->
<!-- source-root «source-sought» source="crates/ordinal-fs-tree/src/sought.rs" lines="1-132" -->
<!-- insert «sought-object-answer» -->
<!-- /source-root -->
<!-- source-root «source-filesystem-module» source="crates/ordinal-fs-tree/src/fs/mod.rs" lines="1-827" -->
<!-- insert «filesystem-read-opening» -->
<!-- insert «filesystem-write-acquire» -->
<!-- insert «filesystem-read-acquire-and-guard» -->
<!-- insert «filesystem-writing-shape» -->
<!-- insert «filesystem-reading-api» -->
<!-- insert «filesystem-writing-api» -->
<!-- insert «filesystem-read-guard» -->
<!-- insert «filesystem-write-guard» -->
<!-- insert «filesystem-vacancy-api» -->
<!-- insert «filesystem-read-guard-api» -->
<!-- insert «filesystem-write-guard-api» -->
<!-- insert «filesystem-read-deref» -->
<!-- insert «filesystem-write-deref» -->
<!-- /source-root -->
<!-- source-root «source-filesystem-read» source="crates/ordinal-fs-tree/src/fs/read.rs" lines="1-407" -->
<!-- insert «read-filesystem-source» -->
<!-- /source-root -->
<!-- source-root «source-filesystem-apply» source="crates/ordinal-fs-tree/src/fs/apply.rs" lines="1-488" -->
<!-- insert «filesystem-interpreter-source» -->
<!-- /source-root -->
<!-- source-root «source-filesystem-remove» source="crates/ordinal-fs-tree/src/fs/remove.rs" lines="1-275" -->
<!-- insert «filesystem-removal-source» -->
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
| `manifest-development-and-release` | `source-crate-manifest` | `orientation-k11` | `66-112` | 47 | `resolved` |
| `syllabus-cli-source` | `source-syllabus-cli` | `syllabus-cli-k17` | `1-1439` | 1,439 | `resolved` |
| `library-crate-surface` | `source-library` | `orientation-k11` | `1-103` | 103 | `resolved` |
| `reference-conformance-source` | `source-conformance` | `reference-domain-k13` | `1-667` | 667 | `resolved` |
| `filesystem-error-source` | `source-error` | `filesystem-interpreter-k16` | `1-510` | 510 | `resolved` |
| `name-seam-source` | `source-name` | `name-seam-k12` | `1-717` | 717 | `resolved` |
| `mutation-operations-source` | `source-operations` | `mutation-algebra-k15` | `1-634` | 634 | `resolved` |
| `mutation-plan-source` | `source-plan` | `mutation-algebra-k15` | `1-597` | 597 | `resolved` |
| `reference-domain-source` | `source-reference` | `reference-domain-k13` | `1-559` | 559 | `resolved` |
| `mutation-report-source` | `source-report` | `mutation-algebra-k15` | `1-186` | 186 | `resolved` |
| `read-snapshot-source` | `source-snapshot` | `read-path-k14` | `1-677` | 677 | `resolved` |
| `sought-object-answer` | `source-sought` | `name-seam-k12` | `1-132` | 132 | `resolved` |
| `filesystem-read-opening` | `source-filesystem-module` | `read-path-k14` | `1-128` | 128 | `resolved` |
| `filesystem-write-acquire` | `source-filesystem-module` | `filesystem-interpreter-k16` | `129-155` | 27 | `resolved` |
| `filesystem-read-acquire-and-guard` | `source-filesystem-module` | `read-path-k14` | `156-202` | 47 | `resolved` |
| `filesystem-writing-shape` | `source-filesystem-module` | `filesystem-interpreter-k16` | `203-215` | 13 | `resolved` |
| `filesystem-reading-api` | `source-filesystem-module` | `read-path-k14` | `216-248` | 33 | `resolved` |
| `filesystem-writing-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `249-290` | 42 | `resolved` |
| `filesystem-read-guard` | `source-filesystem-module` | `read-path-k14` | `291-304` | 14 | `resolved` |
| `filesystem-write-guard` | `source-filesystem-module` | `filesystem-interpreter-k16` | `305-388` | 84 | `resolved` |
| `filesystem-vacancy-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `389-520` | 132 | `resolved` |
| `filesystem-read-guard-api` | `source-filesystem-module` | `read-path-k14` | `521-534` | 14 | `resolved` |
| `filesystem-write-guard-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `535-812` | 278 | `resolved` |
| `filesystem-read-deref` | `source-filesystem-module` | `read-path-k14` | `813-820` | 8 | `resolved` |
| `filesystem-write-deref` | `source-filesystem-module` | `filesystem-interpreter-k16` | `821-827` | 7 | `resolved` |
| `read-filesystem-source` | `source-filesystem-read` | `read-path-k14` | `1-407` | 407 | `resolved` |
| `filesystem-interpreter-source` | `source-filesystem-apply` | `filesystem-interpreter-k16` | `1-488` | 488 | `resolved` |
| `filesystem-removal-source` | `source-filesystem-remove` | `filesystem-interpreter-k16` | `1-275` | 275 | `resolved` |
| `filesystem-lock-source` | `source-filesystem-lock` | `filesystem-interpreter-k16` | `1-91` | 91 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-112` | `—` | `manifest-package-and-library-dependency`, `manifest-cli-feature`, `manifest-library-cli-boundary`, `manifest-cli-binary`, `manifest-development-and-release` |
| `manifest-package-and-library-dependency` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `1-42` | `source-crate-manifest` | `—` |
| `manifest-cli-feature` | `syllabus-cli` | `source-crate-manifest` | `literal` | `syllabus-cli-k17` | `43-45` | `source-crate-manifest` | `—` |
| `manifest-library-cli-boundary` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `46-61` | `source-crate-manifest` | `—` |
| `manifest-cli-binary` | `syllabus-cli` | `source-crate-manifest` | `literal` | `syllabus-cli-k17` | `62-65` | `source-crate-manifest` | `—` |
| `manifest-development-and-release` | `orientation` | `source-crate-manifest` | `literal` | `orientation-k11` | `66-112` | `source-crate-manifest` | `—` |
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
| `source-library` | `source-index` | `source-library` | `root` | `—` | `1-103` | `—` | `library-crate-surface` |
| `library-crate-surface` | `orientation` | `source-library` | `literal` | `orientation-k11` | `1-103` | `source-library` | `—` |
| `source-conformance` | `source-index` | `source-conformance` | `root` | `—` | `1-667` | `—` | `reference-conformance-source` |
| `conformance-obligations` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `1-208` | `reference-conformance-source` | `—` |
| `reference-conformance-source` | `reference-domain` | `source-conformance` | `composite` | `reference-domain-k13` | `1-667` | `source-conformance` | `conformance-obligations`, `conformance-report`, `conformance-compose-and-canonical`, `conformance-component-and-distinguished`, `conformance-found-agreement` |
| `conformance-report` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `209-313` | `reference-conformance-source` | `—` |
| `conformance-compose-and-canonical` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `314-473` | `reference-conformance-source` | `—` |
| `conformance-component-and-distinguished` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `474-572` | `reference-conformance-source` | `—` |
| `conformance-found-agreement` | `reference-domain` | `source-conformance` | `literal` | `reference-domain-k13` | `573-667` | `reference-conformance-source` | `—` |
| `source-error` | `source-index` | `source-error` | `root` | `—` | `1-510` | `—` | `filesystem-error-source` |
| `error-boundary` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `1-23` | `filesystem-error-source` | `—` |
| `filesystem-error-source` | `filesystem-interpreter` | `source-error` | `composite` | `filesystem-interpreter-k16` | `1-510` | `source-error` | `error-boundary`, `error-taxonomy`, `error-debug`, `error-display`, `error-sources` |
| `error-taxonomy` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `24-259` | `filesystem-error-source` | `—` |
| `error-debug` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `260-354` | `filesystem-error-source` | `—` |
| `error-display` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `355-489` | `filesystem-error-source` | `—` |
| `error-sources` | `filesystem-interpreter` | `source-error` | `literal` | `filesystem-interpreter-k16` | `490-510` | `filesystem-error-source` | `—` |
| `source-name` | `source-index` | `source-name` | `root` | `—` | `1-717` | `—` | `name-seam-source` |
| `name-identifiers` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `1-92` | `name-seam-source` | `—` |
| `name-seam-source` | `name-seam` | `source-name` | `composite` | `name-seam-k12` | `1-717` | `source-name` | `name-identifiers`, `name-classification`, `name-representation`, `entry-name-trait`, `entry-name-derived-readings`, `name-component-check` |
| `name-classification` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `93-247` | `name-seam-source` | `—` |
| `name-representation` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `248-346` | `name-seam-source` | `—` |
| `entry-name-trait` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `347-616` | `name-seam-source` | `—` |
| `entry-name-derived-readings` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `617-691` | `name-seam-source` | `—` |
| `name-component-check` | `name-seam` | `source-name` | `literal` | `name-seam-k12` | `692-717` | `name-seam-source` | `—` |
| `source-operations` | `source-index` | `source-operations` | `root` | `—` | `1-634` | `—` | `mutation-operations-source` |
| `ops-surface-and-inputs` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `1-70` | `mutation-operations-source` | `—` |
| `mutation-operations-source` | `mutation-algebra` | `source-operations` | `composite` | `mutation-algebra-k15` | `1-634` | `source-operations` | `ops-surface-and-inputs`, `ops-append`, `ops-initialize`, `ops-insert`, `ops-promote`, `ops-rewrite`, `ops-resolution-and-allocation` |
| `ops-append` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `71-119` | `mutation-operations-source` | `—` |
| `ops-initialize` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `120-183` | `mutation-operations-source` | `—` |
| `ops-insert` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `184-290` | `mutation-operations-source` | `—` |
| `ops-promote` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `291-437` | `mutation-operations-source` | `—` |
| `ops-rewrite` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `438-517` | `mutation-operations-source` | `—` |
| `ops-resolution-and-allocation` | `mutation-algebra` | `source-operations` | `literal` | `mutation-algebra-k15` | `518-634` | `mutation-operations-source` | `—` |
| `source-plan` | `source-index` | `source-plan` | `root` | `—` | `1-597` | `—` | `mutation-plan-source` |
| `plan-effects` | `mutation-algebra` | `source-plan` | `literal` | `mutation-algebra-k15` | `1-118` | `mutation-plan-source` | `—` |
| `mutation-plan-source` | `mutation-algebra` | `source-plan` | `composite` | `mutation-algebra-k15` | `1-597` | `source-plan` | `plan-effects`, `plan-guarded`, `plan-decision-and-refusals`, `plan-refusal-messages` |
| `plan-guarded` | `mutation-algebra` | `source-plan` | `literal` | `mutation-algebra-k15` | `119-229` | `mutation-plan-source` | `—` |
| `plan-decision-and-refusals` | `mutation-algebra` | `source-plan` | `literal` | `mutation-algebra-k15` | `230-433` | `mutation-plan-source` | `—` |
| `plan-refusal-messages` | `mutation-algebra` | `source-plan` | `literal` | `mutation-algebra-k15` | `434-597` | `mutation-plan-source` | `—` |
| `source-reference` | `source-index` | `source-reference` | `root` | `—` | `1-559` | `—` | `reference-domain-source` |
| `reference-vocabulary` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `1-214` | `reference-domain-source` | `—` |
| `reference-domain-source` | `reference-domain` | `source-reference` | `composite` | `reference-domain-k13` | `1-559` | `source-reference` | `reference-vocabulary`, `reference-name-and-errors`, `reference-parser`, `reference-seam-methods`, `reference-parser-helpers` |
| `reference-name-and-errors` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `215-355` | `reference-domain-source` | `—` |
| `reference-parser` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `356-477` | `reference-domain-source` | `—` |
| `reference-seam-methods` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `478-509` | `reference-domain-source` | `—` |
| `reference-parser-helpers` | `reference-domain` | `source-reference` | `literal` | `reference-domain-k13` | `510-559` | `reference-domain-source` | `—` |
| `source-report` | `source-index` | `source-report` | `root` | `—` | `1-186` | `—` | `mutation-report-source` |
| `report-structure-and-order` | `mutation-algebra` | `source-report` | `literal` | `mutation-algebra-k15` | `1-153` | `mutation-report-source` | `—` |
| `mutation-report-source` | `mutation-algebra` | `source-report` | `composite` | `mutation-algebra-k15` | `1-186` | `source-report` | `report-structure-and-order`, `report-debug` |
| `report-debug` | `mutation-algebra` | `source-report` | `literal` | `mutation-algebra-k15` | `154-186` | `mutation-report-source` | `—` |
| `source-snapshot` | `source-index` | `source-snapshot` | `root` | `—` | `1-677` | `—` | `read-snapshot-source` |
| `snapshot-storage` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `1-104` | `read-snapshot-source` | `—` |
| `read-snapshot-source` | `read-path` | `source-snapshot` | `composite` | `read-path-k14` | `1-677` | `source-snapshot` | `snapshot-storage`, `snapshot-builder`, `snapshot-entry-views`, `snapshot-containers`, `snapshot-queries` |
| `snapshot-builder` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `105-267` | `read-snapshot-source` | `—` |
| `snapshot-entry-views` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `268-438` | `read-snapshot-source` | `—` |
| `snapshot-containers` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `439-540` | `read-snapshot-source` | `—` |
| `snapshot-queries` | `read-path` | `source-snapshot` | `literal` | `read-path-k14` | `541-677` | `read-snapshot-source` | `—` |
| `source-sought` | `source-index` | `source-sought` | `root` | `—` | `1-132` | `—` | `sought-object-answer` |
| `sought-object-answer` | `name-seam` | `source-sought` | `literal` | `name-seam-k12` | `1-132` | `source-sought` | `—` |
| `source-filesystem-module` | `source-index` | `source-filesystem-module` | `root` | `—` | `1-827` | `—` | `filesystem-read-opening`, `filesystem-write-acquire`, `filesystem-read-acquire-and-guard`, `filesystem-writing-shape`, `filesystem-reading-api`, `filesystem-writing-api`, `filesystem-read-guard`, `filesystem-write-guard`, `filesystem-vacancy-api`, `filesystem-read-guard-api`, `filesystem-write-guard-api`, `filesystem-read-deref`, `filesystem-write-deref` |
| `filesystem-read-opening` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `1-128` | `source-filesystem-module` | `—` |
| `filesystem-write-acquire` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `129-155` | `source-filesystem-module` | `—` |
| `filesystem-read-acquire-and-guard` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `156-202` | `source-filesystem-module` | `—` |
| `filesystem-writing-shape` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `203-215` | `source-filesystem-module` | `—` |
| `filesystem-reading-api` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `216-248` | `source-filesystem-module` | `—` |
| `filesystem-writing-api` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `249-290` | `source-filesystem-module` | `—` |
| `filesystem-read-guard` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `291-304` | `source-filesystem-module` | `—` |
| `filesystem-write-guard` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `305-388` | `source-filesystem-module` | `—` |
| `filesystem-vacancy-api` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `389-520` | `source-filesystem-module` | `—` |
| `filesystem-read-guard-api` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `521-534` | `source-filesystem-module` | `—` |
| `write-guard-accessors` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `535-547` | `filesystem-write-guard-api` | `—` |
| `filesystem-write-guard-api` | `filesystem-interpreter` | `source-filesystem-module` | `composite` | `filesystem-interpreter-k16` | `535-812` | `source-filesystem-module` | `write-guard-accessors`, `write-guard-append`, `write-guard-insert`, `write-guard-promote`, `write-guard-rewrite`, `write-guard-delete`, `write-guard-dispatch` |
| `write-guard-append` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `548-586` | `filesystem-write-guard-api` | `—` |
| `write-guard-insert` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `587-621` | `filesystem-write-guard-api` | `—` |
| `write-guard-promote` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `622-684` | `filesystem-write-guard-api` | `—` |
| `write-guard-rewrite` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `685-730` | `filesystem-write-guard-api` | `—` |
| `write-guard-delete` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `731-798` | `filesystem-write-guard-api` | `—` |
| `write-guard-dispatch` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `799-812` | `filesystem-write-guard-api` | `—` |
| `filesystem-read-deref` | `read-path` | `source-filesystem-module` | `literal` | `read-path-k14` | `813-820` | `source-filesystem-module` | `—` |
| `filesystem-write-deref` | `filesystem-interpreter` | `source-filesystem-module` | `literal` | `filesystem-interpreter-k16` | `821-827` | `source-filesystem-module` | `—` |
| `source-filesystem-read` | `source-index` | `source-filesystem-read` | `root` | `—` | `1-407` | `—` | `read-filesystem-source` |
| `read-tree-discovery` | `read-path` | `source-filesystem-read` | `literal` | `read-path-k14` | `1-81` | `read-filesystem-source` | `—` |
| `read-filesystem-source` | `read-path` | `source-filesystem-read` | `composite` | `read-path-k14` | `1-407` | `source-filesystem-read` | `read-tree-discovery`, `read-directory-listing`, `read-lock-location` |
| `read-directory-listing` | `read-path` | `source-filesystem-read` | `literal` | `read-path-k14` | `82-155` | `read-filesystem-source` | `—` |
| `read-lock-location` | `read-path` | `source-filesystem-read` | `literal` | `read-path-k14` | `156-407` | `read-filesystem-source` | `—` |
| `source-filesystem-apply` | `source-index` | `source-filesystem-apply` | `root` | `—` | `1-488` | `—` | `filesystem-interpreter-source` |
| `apply-contract` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `1-49` | `filesystem-interpreter-source` | `—` |
| `filesystem-interpreter-source` | `filesystem-interpreter` | `source-filesystem-apply` | `composite` | `filesystem-interpreter-k16` | `1-488` | `source-filesystem-apply` | `apply-contract`, `apply-plan`, `apply-run-state`, `apply-effect-step`, `apply-unwind-and-paths`, `apply-undo`, `apply-destination-claim`, `apply-fault-seam` |
| `apply-plan` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `50-105` | `filesystem-interpreter-source` | `—` |
| `apply-run-state` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `106-123` | `filesystem-interpreter-source` | `—` |
| `apply-effect-step` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `124-228` | `filesystem-interpreter-source` | `—` |
| `apply-unwind-and-paths` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `229-288` | `filesystem-interpreter-source` | `—` |
| `apply-undo` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `289-348` | `filesystem-interpreter-source` | `—` |
| `apply-destination-claim` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `349-382` | `filesystem-interpreter-source` | `—` |
| `apply-fault-seam` | `filesystem-interpreter` | `source-filesystem-apply` | `literal` | `filesystem-interpreter-k16` | `383-488` | `filesystem-interpreter-source` | `—` |
| `source-filesystem-remove` | `source-index` | `source-filesystem-remove` | `root` | `—` | `1-275` | `—` | `filesystem-removal-source` |
| `remove-contract` | `filesystem-interpreter` | `source-filesystem-remove` | `literal` | `filesystem-interpreter-k16` | `1-64` | `filesystem-removal-source` | `—` |
| `filesystem-removal-source` | `filesystem-interpreter` | `source-filesystem-remove` | `composite` | `filesystem-interpreter-k16` | `1-275` | `source-filesystem-remove` | `remove-contract`, `remove-tree`, `remove-spelling-guard`, `remove-worklist-and-failure` |
| `remove-tree` | `filesystem-interpreter` | `source-filesystem-remove` | `literal` | `filesystem-interpreter-k16` | `65-106` | `filesystem-removal-source` | `—` |
| `remove-spelling-guard` | `filesystem-interpreter` | `source-filesystem-remove` | `literal` | `filesystem-interpreter-k16` | `107-188` | `filesystem-removal-source` | `—` |
| `remove-worklist-and-failure` | `filesystem-interpreter` | `source-filesystem-remove` | `literal` | `filesystem-interpreter-k16` | `189-275` | `filesystem-removal-source` | `—` |
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
| `Sought` | `01-orientation.md#public-surface` | `name-seam-k12` | Sought distinguishes a search match from a completed search that matched nothing; nothing is neither a mutation refusal nor an error, while accessors retain Option. | `explained` |
| `Label`, `Status`, `reference::Parts`, `SyllabusName` | `01-orientation.md#insert-tour` | `reference-domain-k13` | These values are the syllabus consumer's vocabulary and seam implementation, not library defaults. | `explained` |
| `Snapshot`, `Entry`, `ReadGuard` | `01-orientation.md#insert-tour` | `read-path-k14` | A snapshot is the immutable parsed tree captured under a guard, entries are borrowed views, and a read guard couples a shared lock, caller-spelled root, and snapshot. | `explained` |
| `Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, `Report` | `01-orientation.md#insert-tour` | `mutation-algebra-k15` | Target names the root or a stable key, new entry carries opaque parts and bytes that may be empty, every input yields refusal or a guarded ordered plan, and the report records landed effects in its documented orders. | `explained` |
| `WriteGuard`, `Error`, `apply::Faults`, `apply::Run` | `01-orientation.md#insert-tour` | `filesystem-interpreter-k16` | A write guard couples an exclusive lock and snapshot and is consumed by one mutation, errors distinguish refusal, clean rollback, partial rollback, and boundary failure, Faults is a test seam, and Run owns per-plan forward and undo state. | `explained` |
| `Cli`, `Verb`, `Streams`, `Failure` | `01-orientation.md#insert-tour` | `syllabus-cli-k17` | Parsed verbs drive dispatch, stdout is result data, stderr carries advisories and errors, and failure pairs operator-facing text with an exit category. | `explained` |
| `conformance` | `02-name-seam.md#entry-name-contract` | `reference-domain-k13` | The reusable conformance kit samples five semantic `EntryName` obligations and publishes the two type-shape constraints with their remaining deterministic-call assumptions; the reference-domain chapter defines its scope and limits. | `explained` |
