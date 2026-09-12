# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove-loop/Cargo.toml` | 68 |
| `source-library-root` | `crates/grove-loop/src/lib.rs` | 377 |
| `source-task-name` | `crates/grove-loop/src/task_name.rs` | 1,675 |
| `source-task-tree` | `crates/grove-loop/src/task_tree.rs` | 1,971 |
| `source-task-grow` | `crates/grove-loop/src/task_grow.rs` | 518 |
| `source-tree-lifecycle` | `crates/grove-loop/src/tree_lifecycle.rs` | 2,734 |
| `source-verbs` | `crates/grove-loop/src/verbs.rs` | 361 |
| `source-driver` | `crates/grove-loop/src/driver.rs` | 57 |
| `source-complete` | `crates/grove-loop/src/complete.rs` | 96 |
| `source-driver-lease` | `crates/grove-loop/src/driver_lease.rs` | 1,383 |
| `source-session-config` | `crates/grove-loop/src/session_config.rs` | 358 |
| `source-prompt` | `crates/grove-loop/src/prompt.rs` | 245 |
| `source-loop-driver` | `crates/grove-loop/src/loop_driver.rs` | 615 |

<!-- source-root «source-crate-manifest» source="crates/grove-loop/Cargo.toml" lines="1-68" -->
<!-- insert «manifest-domain-bound» -->
<!-- /source-root -->
<!-- source-root «source-library-root» source="crates/grove-loop/src/lib.rs" lines="1-377" -->
<!-- insert «library-root» -->
<!-- /source-root -->
<!-- source-root «source-task-name» source="crates/grove-loop/src/task_name.rs" lines="1-1675" -->
<!-- insert «tokens-and-verdicts» -->
<!-- insert «kind-slug-and-handle» -->
<!-- insert «the-task-name» -->
<!-- insert «name-test-support-and-kit» -->
<!-- insert «classification-verdict-tests» -->
<!-- insert «grammar-and-canonicity-tests» -->
<!-- insert «shape-refusal-tests» -->
<!-- insert «slug-rule-tests» -->
<!-- insert «handle-grammar-tests» -->
<!-- /source-root -->
<!-- source-root «source-task-tree» source="crates/grove-loop/src/task_tree.rs" lines="1-1971" -->
<!-- insert «tree-opening» -->
<!-- insert «paths-and-addressing» -->
<!-- insert «walk-selection» -->
<!-- insert «kind-and-brief-chain» -->
<!-- insert «resolution» -->
<!-- insert «path-composition-tests» -->
<!-- insert «pick-tests» -->
<!-- insert «brief-chain-and-kind-tests» -->
<!-- insert «resolve-tests» -->
<!-- insert «pick-with-brief-chain-tests» -->
<!-- /source-root -->
<!-- source-root «source-task-grow» source="crates/grove-loop/src/task_grow.rs" lines="1-518" -->
<!-- insert «growing-the-tree» -->
<!-- /source-root -->
<!-- source-root «source-tree-lifecycle» source="crates/grove-loop/src/tree_lifecycle.rs" lines="1-2734" -->
<!-- insert «finish-transition» -->
<!-- insert «grove-beginning» -->
<!-- insert «decompose-production» -->
<!-- insert «outcomes-in-place» -->
<!-- insert «body-helpers» -->
<!-- insert «root-init-tests» -->
<!-- insert «finish-tests» -->
<!-- insert «decompose-tests» -->
<!-- insert «retire-and-prune-tests» -->
<!-- /source-root -->
<!-- source-root «source-verbs» source="crates/grove-loop/src/verbs.rs" lines="1-361" -->
<!-- insert «the-twelve-verbs» -->
<!-- /source-root -->
<!-- source-root «source-driver» source="crates/grove-loop/src/driver.rs" lines="1-57" -->
<!-- insert «driver-operations» -->
<!-- /source-root -->
<!-- source-root «source-complete» source="crates/grove-loop/src/complete.rs" lines="1-96" -->
<!-- insert «complete-verb» -->
<!-- /source-root -->
<!-- source-root «source-driver-lease» source="crates/grove-loop/src/driver_lease.rs" lines="1-1383" -->
<!-- insert «lease-and-epoch» -->
<!-- insert «lease-tests» -->
<!-- /source-root -->
<!-- source-root «source-session-config» source="crates/grove-loop/src/session_config.rs" lines="1-358" -->
<!-- insert «whose-file» -->
<!-- /source-root -->
<!-- source-root «source-prompt» source="crates/grove-loop/src/prompt.rs" lines="1-245" -->
<!-- insert «the-prompt-core» -->
<!-- /source-root -->
<!-- source-root «source-loop-driver» source="crates/grove-loop/src/loop_driver.rs" lines="1-615" -->
<!-- insert «loop-driver» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-domain-bound` | `source-crate-manifest` | `allowed-to-mean` | `1-68` | 68 | `resolved` |
| `library-root` | `source-library-root` | `allowed-to-mean` | `1-377` | 377 | `resolved` |
| `tokens-and-verdicts` | `source-task-name` | `four-verdicts` | `1-156` | 156 | `resolved` |
| `kind-slug-and-handle` | `source-task-name` | `the-handle-not-the-position` | `157-505` | 349 | `resolved` |
| `the-task-name` | `source-task-name` | `canonical-or-nothing` | `506-953` | 448 | `resolved` |
| `name-test-support-and-kit` | `source-task-name` | `canonical-or-nothing` | `954-1168` | 215 | `resolved` |
| `classification-verdict-tests` | `source-task-name` | `four-verdicts` | `1169-1188` | 20 | `resolved` |
| `grammar-and-canonicity-tests` | `source-task-name` | `canonical-or-nothing` | `1189-1298` | 110 | `resolved` |
| `shape-refusal-tests` | `source-task-name` | `four-verdicts` | `1299-1495` | 197 | `resolved` |
| `slug-rule-tests` | `source-task-name` | `the-handle-not-the-position` | `1496-1524` | 29 | `resolved` |
| `handle-grammar-tests` | `source-task-name` | `the-handle-not-the-position` | `1525-1675` | 151 | `resolved` |
| `tree-opening` | `source-task-tree` | `one-spelling-of-grove` | `1-290` | 290 | `resolved` |
| `paths-and-addressing` | `source-task-tree` | `paths-are-built-here` | `291-518` | 228 | `resolved` |
| `walk-selection` | `source-task-tree` | `first-live-leaf` | `519-585` | 67 | `resolved` |
| `kind-and-brief-chain` | `source-task-tree` | `root-to-leaf` | `586-688` | 103 | `resolved` |
| `resolution` | `source-task-tree` | `wider-than-a-key` | `689-971` | 283 | `resolved` |
| `path-composition-tests` | `source-task-tree` | `paths-are-built-here` | `972-1063` | 92 | `resolved` |
| `pick-tests` | `source-task-tree` | `first-live-leaf` | `1064-1315` | 252 | `resolved` |
| `brief-chain-and-kind-tests` | `source-task-tree` | `root-to-leaf` | `1316-1623` | 308 | `resolved` |
| `resolve-tests` | `source-task-tree` | `wider-than-a-key` | `1624-1944` | 321 | `resolved` |
| `pick-with-brief-chain-tests` | `source-task-tree` | `root-to-leaf` | `1945-1971` | 27 | `resolved` |
| `growing-the-tree` | `source-task-grow` | `what-the-library-cannot-see` | `1-518` | 518 | `resolved` |
| `finish-transition` | `source-tree-lifecycle` | `the-tree-deletes-itself` | `1-326` | 326 | `resolved` |
| `grove-beginning` | `source-tree-lifecycle` | `never-mistaken-for-finished` | `327-487` | 161 | `resolved` |
| `decompose-production` | `source-tree-lifecycle` | `the-key-survives` | `488-705` | 218 | `resolved` |
| `outcomes-in-place` | `source-tree-lifecycle` | `marked-in-place` | `706-1022` | 317 | `resolved` |
| `body-helpers` | `source-tree-lifecycle` | `never-mistaken-for-finished` | `1023-1086` | 64 | `resolved` |
| `root-init-tests` | `source-tree-lifecycle` | `never-mistaken-for-finished` | `1087-1502` | 416 | `resolved` |
| `finish-tests` | `source-tree-lifecycle` | `the-tree-deletes-itself` | `1503-1697` | 195 | `resolved` |
| `decompose-tests` | `source-tree-lifecycle` | `the-key-survives` | `1698-2243` | 546 | `resolved` |
| `retire-and-prune-tests` | `source-tree-lifecycle` | `marked-in-place` | `2244-2734` | 491 | `resolved` |
| `the-twelve-verbs` | `source-verbs` | `twelve-not-fourteen` | `1-361` | 361 | `resolved` |
| `driver-operations` | `source-driver` | `twelve-not-fourteen` | `1-57` | 57 | `resolved` |
| `complete-verb` | `source-complete` | `twelve-not-fourteen` | `1-96` | 96 | `resolved` |
| `lease-and-epoch` | `source-driver-lease` | `one-per-working-tree` | `1-819` | 819 | `resolved` |
| `lease-tests` | `source-driver-lease` | `which-calls-are-admitted` | `820-1383` | 564 | `resolved` |
| `whose-file` | `source-session-config` | `whose-file-and-whether` | `1-358` | 358 | `resolved` |
| `the-prompt-core` | `source-prompt` | `too-late-to-say-later` | `1-245` | 245 | `resolved` |
| `loop-driver` | `source-loop-driver` | `four-things-a-runner-cannot-choose` | `1-615` | 615 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-68` | `—` | `manifest-domain-bound` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `1-10` | `manifest-domain-bound` | `—` |
| `manifest-domain-bound` | `orientation` | `source-crate-manifest` | `composite` | `allowed-to-mean` | `1-68` | `source-crate-manifest` | `manifest-package-identity`, `manifest-dependencies`, `manifest-extracted-tree`, `manifest-dev-dependencies`, `manifest-lints`, `manifest-release` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `11-39` | `manifest-domain-bound` | `—` |
| `manifest-extracted-tree` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `40-47` | `manifest-domain-bound` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `48-50` | `manifest-domain-bound` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `51-53` | `manifest-domain-bound` | `—` |
| `manifest-release` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `54-68` | `manifest-domain-bound` | `—` |
| `source-library-root` | `source-index` | `source-library-root` | `root` | `—` | `1-377` | `—` | `library-root` |
| `library-root-thesis` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `1-10` | `library-root` | `—` |
| `library-root` | `orientation` | `source-library-root` | `composite` | `allowed-to-mean` | `1-377` | `source-library-root` | `library-root-thesis`, `library-root-and-the-driver`, `library-root-opening-mirrors`, `library-root-three-shapes`, `library-root-one-error`, `library-root-modules`, `library-root-version`, `library-root-imports-and-exports`, `library-root-tree-and-vacancy`, `library-root-reading-and-writing`, `library-root-tree-write`, `library-root-tree-write-impl`, `library-root-read-and-write`, `library-root-grove-root`, `library-root-reference`, `library-root-reference-display`, `library-root-selection`, `library-root-error`, `library-root-error-traits` |
| `library-root-and-the-driver` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `11-15` | `library-root` | `—` |
| `library-root-opening-mirrors` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `16-30` | `library-root` | `—` |
| `library-root-three-shapes` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `31-41` | `library-root` | `—` |
| `library-root-one-error` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `42-50` | `library-root` | `—` |
| `library-root-modules` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `51-63` | `library-root` | `—` |
| `library-root-version` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `64-73` | `library-root` | `—` |
| `library-root-imports-and-exports` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `74-92` | `library-root` | `—` |
| `library-root-tree-and-vacancy` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `93-107` | `library-root` | `—` |
| `library-root-reading-and-writing` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `108-124` | `library-root` | `—` |
| `library-root-tree-write` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `125-180` | `library-root` | `—` |
| `library-root-tree-write-impl` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `181-233` | `library-root` | `—` |
| `library-root-read-and-write` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `234-266` | `library-root` | `—` |
| `library-root-grove-root` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `267-271` | `library-root` | `—` |
| `library-root-reference` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `272-322` | `library-root` | `—` |
| `library-root-reference-display` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `323-328` | `library-root` | `—` |
| `library-root-selection` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `329-332` | `library-root` | `—` |
| `library-root-error` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `333-351` | `library-root` | `—` |
| `library-root-error-traits` | `orientation` | `source-library-root` | `literal` | `allowed-to-mean` | `352-377` | `library-root` | `—` |
| `source-task-name` | `source-index` | `source-task-name` | `root` | `—` | `1-1675` | `—` | `tokens-and-verdicts`, `kind-slug-and-handle`, `the-task-name`, `name-test-support-and-kit`, `classification-verdict-tests`, `grammar-and-canonicity-tests`, `shape-refusal-tests`, `slug-rule-tests`, `handle-grammar-tests` |
| `name-the-only-grammar` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1-7` | `tokens-and-verdicts` | `—` |
| `tokens-and-verdicts` | `the-tokens` | `source-task-name` | `composite` | `four-verdicts` | `1-156` | `source-task-name` | `name-the-only-grammar`, `name-canonicity-departure`, `name-classification-loses-data`, `name-handle-is-this-grammar`, `name-handle-terminal-substring`, `name-imports`, `name-brief-and-key-mark`, `name-separator`, `name-outcome`, `name-outcome-infix-and-strip`, `name-token-error`, `name-token-error-traits`, `name-refuse-token` |
| `name-canonicity-departure` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `8-8` | `tokens-and-verdicts` | `—` |
| `name-classification-loses-data` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `9-9` | `tokens-and-verdicts` | `—` |
| `name-handle-is-this-grammar` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `10-10` | `tokens-and-verdicts` | `—` |
| `name-handle-terminal-substring` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `11-12` | `tokens-and-verdicts` | `—` |
| `name-imports` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `13-18` | `tokens-and-verdicts` | `—` |
| `name-brief-and-key-mark` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `19-27` | `tokens-and-verdicts` | `—` |
| `name-separator` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `28-40` | `tokens-and-verdicts` | `—` |
| `name-outcome` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `41-56` | `tokens-and-verdicts` | `—` |
| `name-outcome-infix-and-strip` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `57-84` | `tokens-and-verdicts` | `—` |
| `name-token-error` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `85-101` | `tokens-and-verdicts` | `—` |
| `name-token-error-traits` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `102-109` | `tokens-and-verdicts` | `—` |
| `name-refuse-token` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `110-156` | `tokens-and-verdicts` | `—` |
| `name-kind` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `157-180` | `kind-slug-and-handle` | `—` |
| `kind-slug-and-handle` | `kind-slug-handle` | `source-task-name` | `composite` | `the-handle-not-the-position` | `157-505` | `source-task-name` | `name-kind`, `name-reserved-labels`, `name-kind-methods`, `name-kind-display`, `name-slug`, `name-slug-methods`, `name-slug-display`, `name-handle-error`, `name-handle-error-display`, `name-handle`, `name-handle-new-and-of`, `name-handle-parse`, `name-handle-accessors-and-render`, `name-handle-display`, `name-parts`, `name-parts-methods` |
| `name-reserved-labels` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `181-186` | `kind-slug-and-handle` | `—` |
| `name-kind-methods` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `187-236` | `kind-slug-and-handle` | `—` |
| `name-kind-display` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `237-242` | `kind-slug-and-handle` | `—` |
| `name-slug` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `243-251` | `kind-slug-and-handle` | `—` |
| `name-slug-methods` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `252-274` | `kind-slug-and-handle` | `—` |
| `name-slug-display` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `275-280` | `kind-slug-and-handle` | `—` |
| `name-handle-error` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `281-311` | `kind-slug-and-handle` | `—` |
| `name-handle-error-display` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `312-338` | `kind-slug-and-handle` | `—` |
| `name-handle` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `339-347` | `kind-slug-and-handle` | `—` |
| `name-handle-new-and-of` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `348-370` | `kind-slug-and-handle` | `—` |
| `name-handle-parse` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `371-414` | `kind-slug-and-handle` | `—` |
| `name-handle-accessors-and-render` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `415-438` | `kind-slug-and-handle` | `—` |
| `name-handle-display` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `439-444` | `kind-slug-and-handle` | `—` |
| `name-parts` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `445-466` | `kind-slug-and-handle` | `—` |
| `name-parts-methods` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `467-505` | `kind-slug-and-handle` | `—` |
| `name-task-name` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `506-534` | `the-task-name` | `—` |
| `the-task-name` | `the-name` | `source-task-name` | `composite` | `canonical-or-nothing` | `506-953` | `source-task-name` | `name-task-name`, `name-task-name-display`, `name-task-name-error`, `name-task-name-error-display`, `name-parse-charter`, `name-parse-shape`, `name-parse-parts`, `name-parse-canonicity`, `name-entry-name-rest`, `name-refusal-helpers`, `name-uncomputable-canonical`, `name-split-shape`, `name-terminal-key`, `name-peel-key` |
| `name-task-name-display` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `535-585` | `the-task-name` | `—` |
| `name-task-name-error` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `586-650` | `the-task-name` | `—` |
| `name-task-name-error-display` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `651-711` | `the-task-name` | `—` |
| `name-parse-charter` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `712-737` | `the-task-name` | `—` |
| `name-parse-shape` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `738-770` | `the-task-name` | `—` |
| `name-parse-parts` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `771-805` | `the-task-name` | `—` |
| `name-parse-canonicity` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `806-857` | `the-task-name` | `—` |
| `name-entry-name-rest` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `858-885` | `the-task-name` | `—` |
| `name-refusal-helpers` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `886-902` | `the-task-name` | `—` |
| `name-uncomputable-canonical` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `903-910` | `the-task-name` | `—` |
| `name-split-shape` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `911-923` | `the-task-name` | `—` |
| `name-terminal-key` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `924-931` | `the-task-name` | `—` |
| `name-peel-key` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `932-953` | `the-task-name` | `—` |
| `name-tests-support` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `954-1028` | `name-test-support-and-kit` | `—` |
| `name-test-support-and-kit` | `the-name` | `source-task-name` | `composite` | `canonical-or-nothing` | `954-1168` | `source-task-name` | `name-tests-support`, `name-tests-kit-fixture`, `name-tests-conforms`, `name-tests-kind-shapes`, `name-tests-undeclared-kind` |
| `name-tests-kit-fixture` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1029-1068` | `name-test-support-and-kit` | `—` |
| `name-tests-conforms` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1069-1103` | `name-test-support-and-kit` | `—` |
| `name-tests-kind-shapes` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1104-1149` | `name-test-support-and-kit` | `—` |
| `name-tests-undeclared-kind` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1150-1168` | `name-test-support-and-kit` | `—` |
| `name-tests-the-charter` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1169-1176` | `classification-verdict-tests` | `—` |
| `classification-verdict-tests` | `the-tokens` | `source-task-name` | `composite` | `four-verdicts` | `1169-1188` | `source-task-name` | `name-tests-the-charter`, `name-tests-foreign` |
| `name-tests-foreign` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1177-1188` | `classification-verdict-tests` | `—` |
| `name-tests-live-leaf` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1189-1204` | `grammar-and-canonicity-tests` | `—` |
| `grammar-and-canonicity-tests` | `the-name` | `source-task-name` | `composite` | `canonical-or-nothing` | `1189-1298` | `source-task-name` | `name-tests-live-leaf`, `name-tests-terminal-marks`, `name-tests-node-directory`, `name-tests-terminal-key-marker`, `name-tests-lenient-position`, `name-tests-unpadded-past-99`, `name-tests-unrepresentable` |
| `name-tests-terminal-marks` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1205-1220` | `grammar-and-canonicity-tests` | `—` |
| `name-tests-node-directory` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1221-1232` | `grammar-and-canonicity-tests` | `—` |
| `name-tests-terminal-key-marker` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1233-1245` | `grammar-and-canonicity-tests` | `—` |
| `name-tests-lenient-position` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1246-1278` | `grammar-and-canonicity-tests` | `—` |
| `name-tests-unpadded-past-99` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1279-1287` | `grammar-and-canonicity-tests` | `—` |
| `name-tests-unrepresentable` | `the-name` | `source-task-name` | `literal` | `canonical-or-nothing` | `1288-1298` | `grammar-and-canonicity-tests` | `—` |
| `name-tests-kind-not-a-token` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1299-1344` | `shape-refusal-tests` | `—` |
| `shape-refusal-tests` | `the-tokens` | `source-task-name` | `composite` | `four-verdicts` | `1299-1495` | `source-task-name` | `name-tests-kind-not-a-token`, `name-tests-missing-separator`, `name-tests-one-reading`, `name-tests-node-wearing-outcome`, `name-tests-bad-slug`, `name-tests-species-mismatch` |
| `name-tests-missing-separator` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1345-1381` | `shape-refusal-tests` | `—` |
| `name-tests-one-reading` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1382-1443` | `shape-refusal-tests` | `—` |
| `name-tests-node-wearing-outcome` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1444-1455` | `shape-refusal-tests` | `—` |
| `name-tests-bad-slug` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1456-1471` | `shape-refusal-tests` | `—` |
| `name-tests-species-mismatch` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1472-1495` | `shape-refusal-tests` | `—` |
| `slug-rule-tests` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `1496-1524` | `source-task-name` | `—` |
| `name-tests-ends-in-handle` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `1525-1563` | `handle-grammar-tests` | `—` |
| `handle-grammar-tests` | `kind-slug-handle` | `source-task-name` | `composite` | `the-handle-not-the-position` | `1525-1675` | `source-task-name` | `name-tests-ends-in-handle`, `name-tests-brief-no-handle`, `name-tests-handle-round-trip`, `name-tests-same-peel`, `name-tests-refused-handle`, `name-tests-lenient-strict` |
| `name-tests-brief-no-handle` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `1564-1570` | `handle-grammar-tests` | `—` |
| `name-tests-handle-round-trip` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `1571-1590` | `handle-grammar-tests` | `—` |
| `name-tests-same-peel` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `1591-1611` | `handle-grammar-tests` | `—` |
| `name-tests-refused-handle` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `1612-1645` | `handle-grammar-tests` | `—` |
| `name-tests-lenient-strict` | `kind-slug-handle` | `source-task-name` | `literal` | `the-handle-not-the-position` | `1646-1675` | `handle-grammar-tests` | `—` |
| `source-task-tree` | `source-index` | `source-task-tree` | `root` | `—` | `1-1971` | `—` | `tree-opening`, `paths-and-addressing`, `walk-selection`, `kind-and-brief-chain`, `resolution`, `path-composition-tests`, `pick-tests`, `brief-chain-and-kind-tests`, `resolve-tests`, `pick-with-brief-chain-tests` |
| `tree-header-who-owns-the-walk` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `1-13` | `tree-opening` | `—` |
| `tree-opening` | `opening` | `source-task-tree` | `composite` | `one-spelling-of-grove` | `1-290` | `source-task-tree` | `tree-header-who-owns-the-walk`, `tree-header-paths-here`, `tree-header-no-canonicalising`, `tree-header-refusal-precedence`, `tree-imports`, `tree-alias-and-read-count`, `tree-vacant-and-read-or-vacant`, `tree-guard-opening-vacancy`, `tree-read`, `tree-write`, `tree-write-or-vacancy`, `tree-reopen-write`, `tree-open-write`, `tree-absent-tree`, `tree-raised`, `tree-announce-contention`, `tree-restate` |
| `tree-header-paths-here` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `14-23` | `tree-opening` | `—` |
| `tree-header-no-canonicalising` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `24-30` | `tree-opening` | `—` |
| `tree-header-refusal-precedence` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `31-40` | `tree-opening` | `—` |
| `tree-imports` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `41-51` | `tree-opening` | `—` |
| `tree-alias-and-read-count` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `52-64` | `tree-opening` | `—` |
| `tree-vacant-and-read-or-vacant` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `65-93` | `tree-opening` | `—` |
| `tree-guard-opening-vacancy` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `94-122` | `tree-opening` | `—` |
| `tree-read` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `123-141` | `tree-opening` | `—` |
| `tree-write` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `142-154` | `tree-opening` | `—` |
| `tree-write-or-vacancy` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `155-163` | `tree-opening` | `—` |
| `tree-reopen-write` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `164-177` | `tree-opening` | `—` |
| `tree-open-write` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `178-185` | `tree-opening` | `—` |
| `tree-absent-tree` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `186-198` | `tree-opening` | `—` |
| `tree-raised` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `199-209` | `tree-opening` | `—` |
| `tree-announce-contention` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `210-259` | `tree-opening` | `—` |
| `tree-restate` | `opening` | `source-task-tree` | `literal` | `one-spelling-of-grove` | `260-290` | `tree-opening` | `—` |
| `paths-entry-path` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `291-309` | `paths-and-addressing` | `—` |
| `paths-and-addressing` | `paths` | `source-task-tree` | `composite` | `paths-are-built-here` | `291-518` | `source-task-tree` | `paths-entry-path`, `paths-target-enum`, `paths-target-fn`, `paths-unreachable-by-any-walk`, `paths-addressable-key`, `paths-interrupted-promotion`, `paths-next-key`, `paths-live-leaf`, `paths-entry-outcome` |
| `paths-target-enum` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `310-322` | `paths-and-addressing` | `—` |
| `paths-target-fn` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `323-375` | `paths-and-addressing` | `—` |
| `paths-unreachable-by-any-walk` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `376-404` | `paths-and-addressing` | `—` |
| `paths-addressable-key` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `405-455` | `paths-and-addressing` | `—` |
| `paths-interrupted-promotion` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `456-457` | `paths-and-addressing` | `—` |
| `paths-next-key` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `458-494` | `paths-and-addressing` | `—` |
| `paths-live-leaf` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `495-507` | `paths-and-addressing` | `—` |
| `paths-entry-outcome` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `508-518` | `paths-and-addressing` | `—` |
| `walk-selection-type` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `519-527` | `walk-selection` | `—` |
| `walk-selection` | `the-walk` | `source-task-tree` | `composite` | `first-live-leaf` | `519-585` | `source-task-tree` | `walk-selection-type`, `walk-pick-in`, `walk-select-in`, `walk-select-in-write`, `walk-selected` |
| `walk-pick-in` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `528-533` | `walk-selection` | `—` |
| `walk-select-in` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `534-543` | `walk-selection` | `—` |
| `walk-select-in-write` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `544-553` | `walk-selection` | `—` |
| `walk-selected` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `554-585` | `walk-selection` | `—` |
| `kind-in` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `586-604` | `kind-and-brief-chain` | `—` |
| `kind-and-brief-chain` | `kind-and-briefs` | `source-task-tree` | `composite` | `root-to-leaf` | `586-688` | `source-task-tree` | `kind-in`, `brief-chain-fn`, `leaf-entry-signature`, `leaf-entry-grammar`, `leaf-entry-compare`, `leaf-entry-walk` |
| `brief-chain-fn` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `605-615` | `kind-and-brief-chain` | `—` |
| `leaf-entry-signature` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `616-635` | `kind-and-brief-chain` | `—` |
| `leaf-entry-grammar` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `636-649` | `kind-and-brief-chain` | `—` |
| `leaf-entry-compare` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `650-671` | `kind-and-brief-chain` | `—` |
| `leaf-entry-walk` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `672-688` | `kind-and-brief-chain` | `—` |
| `resolution-outcome` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `689-707` | `resolution` | `—` |
| `resolution` | `resolve` | `source-task-tree` | `composite` | `wider-than-a-key` | `689-971` | `source-task-tree` | `resolution-outcome`, `resolution-located`, `resolution-located-fn`, `resolution-resolve-in`, `resolution-lookup-type`, `resolution-slug-match-key`, `resolution-grammar`, `resolution-reference`, `resolution-existing-path`, `resolution-ref-type`, `resolution-parse-ref`, `resolution-read-count` |
| `resolution-located` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `708-729` | `resolution` | `—` |
| `resolution-located-fn` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `730-752` | `resolution` | `—` |
| `resolution-resolve-in` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `753-793` | `resolution` | `—` |
| `resolution-lookup-type` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `794-806` | `resolution` | `—` |
| `resolution-slug-match-key` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `807-817` | `resolution` | `—` |
| `resolution-grammar` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `818-868` | `resolution` | `—` |
| `resolution-reference` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `869-912` | `resolution` | `—` |
| `resolution-existing-path` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `913-931` | `resolution` | `—` |
| `resolution-ref-type` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `932-937` | `resolution` | `—` |
| `resolution-parse-ref` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `938-960` | `resolution` | `—` |
| `resolution-read-count` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `961-971` | `resolution` | `—` |
| `paths-tests-module-open` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `972-985` | `path-composition-tests` | `—` |
| `path-composition-tests` | `paths` | `source-task-tree` | `composite` | `paths-are-built-here` | `972-1063` | `source-task-tree` | `paths-tests-module-open`, `paths-tests-composed-verbs`, `paths-tests-a-kind-and-imports`, `paths-tests-brief-chain-at`, `paths-tests-fixtures` |
| `paths-tests-composed-verbs` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `986-1013` | `path-composition-tests` | `—` |
| `paths-tests-a-kind-and-imports` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `1014-1024` | `path-composition-tests` | `—` |
| `paths-tests-brief-chain-at` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `1025-1034` | `path-composition-tests` | `—` |
| `paths-tests-fixtures` | `paths` | `source-task-tree` | `literal` | `paths-are-built-here` | `1035-1063` | `path-composition-tests` | `—` |
| `walk-tests-select-one-observation` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1064-1088` | `pick-tests` | `—` |
| `pick-tests` | `the-walk` | `source-task-tree` | `composite` | `first-live-leaf` | `1064-1315` | `source-task-tree` | `walk-tests-select-one-observation`, `walk-tests-order`, `walk-tests-terminal-leaves`, `walk-tests-descent`, `walk-tests-fall-through`, `walk-tests-none`, `walk-tests-foreign`, `walk-tests-species-mismatch`, `walk-tests-symlink`, `walk-tests-legacy-and-absent-root` |
| `walk-tests-order` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1089-1114` | `pick-tests` | `—` |
| `walk-tests-terminal-leaves` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1115-1134` | `pick-tests` | `—` |
| `walk-tests-descent` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1135-1158` | `pick-tests` | `—` |
| `walk-tests-fall-through` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1159-1197` | `pick-tests` | `—` |
| `walk-tests-none` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1198-1227` | `pick-tests` | `—` |
| `walk-tests-foreign` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1228-1245` | `pick-tests` | `—` |
| `walk-tests-species-mismatch` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1246-1274` | `pick-tests` | `—` |
| `walk-tests-symlink` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1275-1292` | `pick-tests` | `—` |
| `walk-tests-legacy-and-absent-root` | `the-walk` | `source-task-tree` | `literal` | `first-live-leaf` | `1293-1315` | `pick-tests` | `—` |
| `chain-tests-shape` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1316-1353` | `brief-chain-and-kind-tests` | `—` |
| `brief-chain-and-kind-tests` | `kind-and-briefs` | `source-task-tree` | `composite` | `root-to-leaf` | `1316-1623` | `source-task-tree` | `chain-tests-shape`, `chain-tests-siblings`, `chain-tests-skipping`, `chain-tests-done-and-relative`, `chain-tests-refusals`, `kind-tests-label-and-fixture`, `kind-tests-two-leaves`, `kind-tests-open-token`, `kind-tests-legacy-label`, `kind-tests-default-and-empty`, `kind-tests-relative-path`, `kind-tests-body-ignored`, `kind-tests-absent-root` |
| `chain-tests-siblings` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1354-1374` | `brief-chain-and-kind-tests` | `—` |
| `chain-tests-skipping` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1375-1408` | `brief-chain-and-kind-tests` | `—` |
| `chain-tests-done-and-relative` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1409-1443` | `brief-chain-and-kind-tests` | `—` |
| `chain-tests-refusals` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1444-1493` | `brief-chain-and-kind-tests` | `—` |
| `kind-tests-label-and-fixture` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1494-1503` | `brief-chain-and-kind-tests` | `—` |
| `kind-tests-two-leaves` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1504-1517` | `brief-chain-and-kind-tests` | `—` |
| `kind-tests-open-token` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1518-1545` | `brief-chain-and-kind-tests` | `—` |
| `kind-tests-legacy-label` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1546-1552` | `brief-chain-and-kind-tests` | `—` |
| `kind-tests-default-and-empty` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1553-1570` | `brief-chain-and-kind-tests` | `—` |
| `kind-tests-relative-path` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1571-1580` | `brief-chain-and-kind-tests` | `—` |
| `kind-tests-body-ignored` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1581-1612` | `brief-chain-and-kind-tests` | `—` |
| `kind-tests-absent-root` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1613-1623` | `brief-chain-and-kind-tests` | `—` |
| `resolve-tests-fixture` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1624-1662` | `resolve-tests` | `—` |
| `resolve-tests` | `resolve` | `source-task-tree` | `composite` | `wider-than-a-key` | `1624-1944` | `source-task-tree` | `resolve-tests-fixture`, `resolve-tests-bracket-key`, `resolve-tests-bare-number`, `resolve-tests-pruned`, `resolve-tests-decorative-slug`, `resolve-tests-node-by-key`, `resolve-tests-key-not-found`, `resolve-tests-slug-unique`, `resolve-tests-slug-nested`, `resolve-tests-slug-not-found`, `resolve-tests-ambiguous`, `resolve-tests-root-brief`, `resolve-tests-dot`, `resolve-tests-empty-reference`, `resolve-tests-malformed-bracket`, `resolve-tests-absent-root`, `resolve-handle-tests-full-handle`, `resolve-handle-tests-terminal-key`, `resolve-handle-tests-disambiguates`, `resolve-handle-tests-node`, `resolve-handle-tests-precedence`, `resolve-handle-tests-unmatched` |
| `resolve-tests-bracket-key` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1663-1675` | `resolve-tests` | `—` |
| `resolve-tests-bare-number` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1676-1687` | `resolve-tests` | `—` |
| `resolve-tests-pruned` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1688-1716` | `resolve-tests` | `—` |
| `resolve-tests-decorative-slug` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1717-1728` | `resolve-tests` | `—` |
| `resolve-tests-node-by-key` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1729-1743` | `resolve-tests` | `—` |
| `resolve-tests-key-not-found` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1744-1749` | `resolve-tests` | `—` |
| `resolve-tests-slug-unique` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1750-1761` | `resolve-tests` | `—` |
| `resolve-tests-slug-nested` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1762-1775` | `resolve-tests` | `—` |
| `resolve-tests-slug-not-found` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1776-1781` | `resolve-tests` | `—` |
| `resolve-tests-ambiguous` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1782-1803` | `resolve-tests` | `—` |
| `resolve-tests-root-brief` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1804-1812` | `resolve-tests` | `—` |
| `resolve-tests-dot` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1813-1822` | `resolve-tests` | `—` |
| `resolve-tests-empty-reference` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1823-1827` | `resolve-tests` | `—` |
| `resolve-tests-malformed-bracket` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1828-1834` | `resolve-tests` | `—` |
| `resolve-tests-absent-root` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1835-1845` | `resolve-tests` | `—` |
| `resolve-handle-tests-full-handle` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1846-1861` | `resolve-tests` | `—` |
| `resolve-handle-tests-terminal-key` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1862-1888` | `resolve-tests` | `—` |
| `resolve-handle-tests-disambiguates` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1889-1902` | `resolve-tests` | `—` |
| `resolve-handle-tests-node` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1903-1915` | `resolve-tests` | `—` |
| `resolve-handle-tests-precedence` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1916-1936` | `resolve-tests` | `—` |
| `resolve-handle-tests-unmatched` | `resolve` | `source-task-tree` | `literal` | `wider-than-a-key` | `1937-1944` | `resolve-tests` | `—` |
| `pick-with-brief-chain-tests` | `kind-and-briefs` | `source-task-tree` | `literal` | `root-to-leaf` | `1945-1971` | `source-task-tree` | `—` |
| `source-task-grow` | `source-index` | `source-task-grow` | `root` | `—` | `1-518` | `—` | `growing-the-tree` |
| `grow-header-the-four` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `1-24` | `growing-the-tree` | `—` |
| `growing-the-tree` | `growing` | `source-task-grow` | `composite` | `what-the-library-cannot-see` | `1-518` | `source-task-grow` | `grow-header-the-four`, `grow-header-what-went`, `grow-imports`, `grow-leaf-add-doc`, `grow-leaf-add`, `grow-inserted-and-insert-doc`, `grow-leaf-insert`, `grow-renumber`, `grow-renumbered`, `grow-lint-doc`, `grow-lint`, `grow-parent-node`, `grow-containing-level`, `grow-new-leaf`, `grow-allocated`, `grow-refuse-finish-kind`, `grow-template-and-stem`, `grow-test-module` |
| `grow-header-what-went` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `25-49` | `growing-the-tree` | `—` |
| `grow-imports` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `50-58` | `growing-the-tree` | `—` |
| `grow-leaf-add-doc` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `59-90` | `growing-the-tree` | `—` |
| `grow-leaf-add` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `91-131` | `growing-the-tree` | `—` |
| `grow-inserted-and-insert-doc` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `132-168` | `growing-the-tree` | `—` |
| `grow-leaf-insert` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `169-202` | `growing-the-tree` | `—` |
| `grow-renumber` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `203-242` | `growing-the-tree` | `—` |
| `grow-renumbered` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `243-271` | `growing-the-tree` | `—` |
| `grow-lint-doc` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `272-310` | `growing-the-tree` | `—` |
| `grow-lint` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `311-355` | `growing-the-tree` | `—` |
| `grow-parent-node` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `356-389` | `growing-the-tree` | `—` |
| `grow-containing-level` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `390-410` | `growing-the-tree` | `—` |
| `grow-new-leaf` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `411-433` | `growing-the-tree` | `—` |
| `grow-allocated` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `434-481` | `growing-the-tree` | `—` |
| `grow-refuse-finish-kind` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `482-496` | `growing-the-tree` | `—` |
| `grow-template-and-stem` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `497-516` | `growing-the-tree` | `—` |
| `grow-test-module` | `growing` | `source-task-grow` | `literal` | `what-the-library-cannot-see` | `517-518` | `growing-the-tree` | `—` |
| `source-tree-lifecycle` | `source-index` | `source-tree-lifecycle` | `root` | `—` | `1-2734` | `—` | `finish-transition`, `grove-beginning`, `decompose-production`, `outcomes-in-place`, `body-helpers`, `root-init-tests`, `finish-tests`, `decompose-tests`, `retire-and-prune-tests` |
| `finishing-module-header` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1-44` | `finish-transition` | `—` |
| `finish-transition` | `finishing` | `source-tree-lifecycle` | `composite` | `the-tree-deletes-itself` | `1-326` | `source-tree-lifecycle` | `finishing-module-header`, `finishing-imports`, `finishing-default-slug`, `finishing-current-transition`, `finishing-transition-contract`, `finishing-transition-body`, `finishing-materialize-contract`, `finishing-materialize-body`, `finishing-new-finish-leaf`, `finishing-finish-handle`, `finishing-finish-slug`, `finishing-finish-body`, `finishing-commit-contract`, `finishing-commit-classify`, `finishing-commit-revalidate`, `finishing-delete-contract`, `finishing-delete-body`, `finishing-recoverable-contract`, `finishing-recoverable-body` |
| `finishing-imports` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `45-53` | `finish-transition` | `—` |
| `finishing-default-slug` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `54-57` | `finish-transition` | `—` |
| `finishing-current-transition` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `58-63` | `finish-transition` | `—` |
| `finishing-transition-contract` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `64-74` | `finish-transition` | `—` |
| `finishing-transition-body` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `75-108` | `finish-transition` | `—` |
| `finishing-materialize-contract` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `109-112` | `finish-transition` | `—` |
| `finishing-materialize-body` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `113-139` | `finish-transition` | `—` |
| `finishing-new-finish-leaf` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `140-153` | `finish-transition` | `—` |
| `finishing-finish-handle` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `154-160` | `finish-transition` | `—` |
| `finishing-finish-slug` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `161-168` | `finish-transition` | `—` |
| `finishing-finish-body` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `169-180` | `finish-transition` | `—` |
| `finishing-commit-contract` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `181-194` | `finish-transition` | `—` |
| `finishing-commit-classify` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `195-226` | `finish-transition` | `—` |
| `finishing-commit-revalidate` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `227-254` | `finish-transition` | `—` |
| `finishing-delete-contract` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `255-259` | `finish-transition` | `—` |
| `finishing-delete-body` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `260-302` | `finish-transition` | `—` |
| `finishing-recoverable-contract` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `303-312` | `finish-transition` | `—` |
| `finishing-recoverable-body` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `313-326` | `finish-transition` | `—` |
| `grove-beginning-root-init` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `327-345` | `grove-beginning` | `—` |
| `grove-beginning` | `a-grove-begins` | `source-tree-lifecycle` | `composite` | `never-mistaken-for-finished` | `327-487` | `source-tree-lifecycle` | `grove-beginning-root-init`, `grove-beginning-default-slug`, `grove-beginning-initialize`, `grove-beginning-root-shape-type`, `grove-beginning-root-shape-fn` |
| `grove-beginning-default-slug` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `346-351` | `grove-beginning` | `—` |
| `grove-beginning-initialize` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `352-406` | `grove-beginning` | `—` |
| `grove-beginning-root-shape-type` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `407-441` | `grove-beginning` | `—` |
| `grove-beginning-root-shape-fn` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `442-487` | `grove-beginning` | `—` |
| `decompose-verb-contract` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `488-529` | `decompose-production` | `—` |
| `decompose-production` | `leaf-to-node` | `source-tree-lifecycle` | `composite` | `the-key-survives` | `488-705` | `source-tree-lifecycle` | `decompose-verb-contract`, `decompose-verb-body`, `decompose-decomposable`, `decompose-promoted-claims`, `decompose-promoted-body` |
| `decompose-verb-body` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `530-600` | `decompose-production` | `—` |
| `decompose-decomposable` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `601-638` | `decompose-production` | `—` |
| `decompose-promoted-claims` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `639-654` | `decompose-production` | `—` |
| `decompose-promoted-body` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `655-705` | `decompose-production` | `—` |
| `outcomes-retire-contract` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `706-716` | `outcomes-in-place` | `—` |
| `outcomes-in-place` | `outcomes` | `source-tree-lifecycle` | `composite` | `marked-in-place` | `706-1022` | `source-tree-lifecycle` | `outcomes-retire-contract`, `outcomes-retire-body`, `outcomes-retire-parts`, `outcomes-prune-result`, `outcomes-prune-contract`, `outcomes-prune-body`, `outcomes-planned`, `outcomes-plan-prune`, `outcomes-plan-subtree`, `outcomes-plan-leaf`, `outcomes-apply-prune`, `outcomes-stopped-partway`, `outcomes-marked-path` |
| `outcomes-retire-body` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `717-737` | `outcomes-in-place` | `—` |
| `outcomes-retire-parts` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `738-774` | `outcomes-in-place` | `—` |
| `outcomes-prune-result` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `775-785` | `outcomes-in-place` | `—` |
| `outcomes-prune-contract` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `786-815` | `outcomes-in-place` | `—` |
| `outcomes-prune-body` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `816-831` | `outcomes-in-place` | `—` |
| `outcomes-planned` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `832-838` | `outcomes-in-place` | `—` |
| `outcomes-plan-prune` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `839-861` | `outcomes-in-place` | `—` |
| `outcomes-plan-subtree` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `862-898` | `outcomes-in-place` | `—` |
| `outcomes-plan-leaf` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `899-939` | `outcomes-in-place` | `—` |
| `outcomes-apply-prune` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `940-986` | `outcomes-in-place` | `—` |
| `outcomes-stopped-partway` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `987-1006` | `outcomes-in-place` | `—` |
| `outcomes-marked-path` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `1007-1022` | `outcomes-in-place` | `—` |
| `body-helpers-grove-name` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1023-1047` | `body-helpers` | `—` |
| `body-helpers` | `a-grove-begins` | `source-tree-lifecycle` | `composite` | `never-mistaken-for-finished` | `1023-1086` | `source-tree-lifecycle` | `body-helpers-grove-name`, `body-helpers-root-brief`, `body-helpers-retitle` |
| `body-helpers-root-brief` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1048-1063` | `body-helpers` | `—` |
| `body-helpers-retitle` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1064-1086` | `body-helpers` | `—` |
| `root-init-tests-open` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1087-1122` | `root-init-tests` | `—` |
| `root-init-tests` | `a-grove-begins` | `source-tree-lifecycle` | `composite` | `never-mistaken-for-finished` | `1087-1502` | `source-tree-lifecycle` | `root-init-tests-open`, `root-init-tests-worktrees`, `root-init-tests-grow-leaf`, `root-init-tests-guards`, `root-init-tests-root-init-at`, `root-init-tests-writers`, `root-init-tests-basics`, `root-init-tests-refusals`, `root-init-tests-one-guard`, `root-init-tests-one-operation`, `root-init-tests-no-self-wait`, `root-init-tests-prediction`, `root-init-tests-refused-grove`, `root-init-tests-taskless` |
| `root-init-tests-worktrees` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1123-1176` | `root-init-tests` | `—` |
| `root-init-tests-grow-leaf` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1177-1198` | `root-init-tests` | `—` |
| `root-init-tests-guards` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1199-1230` | `root-init-tests` | `—` |
| `root-init-tests-root-init-at` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1231-1251` | `root-init-tests` | `—` |
| `root-init-tests-writers` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1252-1296` | `root-init-tests` | `—` |
| `root-init-tests-basics` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1297-1343` | `root-init-tests` | `—` |
| `root-init-tests-refusals` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1344-1369` | `root-init-tests` | `—` |
| `root-init-tests-one-guard` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1370-1388` | `root-init-tests` | `—` |
| `root-init-tests-one-operation` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1389-1411` | `root-init-tests` | `—` |
| `root-init-tests-no-self-wait` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1412-1446` | `root-init-tests` | `—` |
| `root-init-tests-prediction` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1447-1459` | `root-init-tests` | `—` |
| `root-init-tests-refused-grove` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1460-1477` | `root-init-tests` | `—` |
| `root-init-tests-taskless` | `a-grove-begins` | `source-tree-lifecycle` | `literal` | `never-mistaken-for-finished` | `1478-1502` | `root-init-tests` | `—` |
| `finishing-test-three-spellings` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1503-1531` | `finish-tests` | `—` |
| `finish-tests` | `finishing` | `source-tree-lifecycle` | `composite` | `the-tree-deletes-itself` | `1503-1697` | `source-tree-lifecycle` | `finishing-test-three-spellings`, `finishing-test-last-key`, `finishing-test-last-ordinal`, `finishing-test-reuse`, `finishing-test-already-current`, `finishing-test-malformed-name`, `finishing-test-no-grove-entries`, `finishing-test-dangling-symlink` |
| `finishing-test-last-key` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1532-1558` | `finish-tests` | `—` |
| `finishing-test-last-ordinal` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1559-1575` | `finish-tests` | `—` |
| `finishing-test-reuse` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1576-1598` | `finish-tests` | `—` |
| `finishing-test-already-current` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1599-1617` | `finish-tests` | `—` |
| `finishing-test-malformed-name` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1618-1648` | `finish-tests` | `—` |
| `finishing-test-no-grove-entries` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1649-1673` | `finish-tests` | `—` |
| `finishing-test-dangling-symlink` | `finishing` | `source-tree-lifecycle` | `literal` | `the-tree-deletes-itself` | `1674-1697` | `finish-tests` | `—` |
| `decompose-tests-opening` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `1698-1724` | `decompose-tests` | `—` |
| `decompose-tests` | `leaf-to-node` | `source-tree-lifecycle` | `composite` | `the-key-survives` | `1698-2243` | `source-tree-lifecycle` | `decompose-tests-opening`, `decompose-tests-brief-and-child`, `decompose-tests-kind`, `decompose-tests-nested`, `decompose-tests-refusals`, `decompose-tests-slug-and-path`, `decompose-tests-seam-opening`, `decompose-tests-one-guard`, `decompose-tests-twin`, `decompose-tests-destination`, `decompose-tests-interrupted`, `decompose-tests-last-key`, `decompose-tests-sweep` |
| `decompose-tests-brief-and-child` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `1725-1771` | `decompose-tests` | `—` |
| `decompose-tests-kind` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `1772-1851` | `decompose-tests` | `—` |
| `decompose-tests-nested` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `1852-1874` | `decompose-tests` | `—` |
| `decompose-tests-refusals` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `1875-1948` | `decompose-tests` | `—` |
| `decompose-tests-slug-and-path` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `1949-1995` | `decompose-tests` | `—` |
| `decompose-tests-seam-opening` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `1996-2003` | `decompose-tests` | `—` |
| `decompose-tests-one-guard` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `2004-2032` | `decompose-tests` | `—` |
| `decompose-tests-twin` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `2033-2063` | `decompose-tests` | `—` |
| `decompose-tests-destination` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `2064-2134` | `decompose-tests` | `—` |
| `decompose-tests-interrupted` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `2135-2158` | `decompose-tests` | `—` |
| `decompose-tests-last-key` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `2159-2188` | `decompose-tests` | `—` |
| `decompose-tests-sweep` | `leaf-to-node` | `source-tree-lifecycle` | `literal` | `the-key-survives` | `2189-2243` | `decompose-tests` | `—` |
| `retire-tests-opening` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2244-2261` | `retire-and-prune-tests` | `—` |
| `retire-and-prune-tests` | `outcomes` | `source-tree-lifecycle` | `composite` | `marked-in-place` | `2244-2734` | `source-tree-lifecycle` | `retire-tests-opening`, `retire-tests-body-untouched`, `retire-tests-nested`, `retire-tests-refusals`, `retire-tests-absolute`, `untracked-tests-opening`, `untracked-tests-decompose-and-prune`, `prune-leaf-tests-opening`, `prune-leaf-tests-body-and-nested`, `prune-leaf-tests-refusals`, `prune-leaf-tests-absolute`, `prune-node-tests-opening`, `prune-node-tests-done-untouched`, `prune-node-tests-grandchild`, `prune-node-tests-mixed-tracking`, `prune-node-tests-atomic`, `prune-node-tests-twin`, `prune-node-tests-guard-count`, `prune-node-tests-nothing-live`, `prune-node-tests-root-refusals` |
| `retire-tests-body-untouched` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2262-2271` | `retire-and-prune-tests` | `—` |
| `retire-tests-nested` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2272-2283` | `retire-and-prune-tests` | `—` |
| `retire-tests-refusals` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2284-2340` | `retire-and-prune-tests` | `—` |
| `retire-tests-absolute` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2341-2350` | `retire-and-prune-tests` | `—` |
| `untracked-tests-opening` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2351-2376` | `retire-and-prune-tests` | `—` |
| `untracked-tests-decompose-and-prune` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2377-2403` | `retire-and-prune-tests` | `—` |
| `prune-leaf-tests-opening` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2404-2423` | `retire-and-prune-tests` | `—` |
| `prune-leaf-tests-body-and-nested` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2424-2449` | `retire-and-prune-tests` | `—` |
| `prune-leaf-tests-refusals` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2450-2494` | `retire-and-prune-tests` | `—` |
| `prune-leaf-tests-absolute` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2495-2504` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-opening` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2505-2523` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-done-untouched` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2524-2542` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-grandchild` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2543-2560` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-mixed-tracking` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2561-2590` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-atomic` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2591-2644` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-twin` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2645-2671` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-guard-count` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2672-2701` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-nothing-live` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2702-2713` | `retire-and-prune-tests` | `—` |
| `prune-node-tests-root-refusals` | `outcomes` | `source-tree-lifecycle` | `literal` | `marked-in-place` | `2714-2734` | `retire-and-prune-tests` | `—` |
| `source-verbs` | `source-index` | `source-verbs` | `root` | `—` | `1-361` | `—` | `the-twelve-verbs` |
| `verbs-surface-header` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `1-12` | `the-twelve-verbs` | `—` |
| `the-twelve-verbs` | `the-verbs` | `source-verbs` | `composite` | `twelve-not-fourteen` | `1-361` | `source-verbs` | `verbs-surface-header`, `verbs-imports`, `verbs-root-init`, `verbs-initialized`, `verbs-pick`, `verbs-kind`, `verbs-brief-chain`, `verbs-resolve`, `verbs-leaf-add`, `verbs-leaf-insert`, `verbs-not-a-thirteenth-verb`, `verbs-leaf-decompose`, `verbs-decomposed`, `verbs-leaf-retire`, `verbs-leaf-prune`, `verbs-pruned`, `verbs-finish-commit`, `verbs-complete`, `verbs-signal-channel`, `verbs-signalled`, `verbs-sought` |
| `verbs-imports` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `13-23` | `the-twelve-verbs` | `—` |
| `verbs-root-init` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `24-49` | `the-twelve-verbs` | `—` |
| `verbs-initialized` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `50-58` | `the-twelve-verbs` | `—` |
| `verbs-pick` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `59-72` | `the-twelve-verbs` | `—` |
| `verbs-kind` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `73-85` | `the-twelve-verbs` | `—` |
| `verbs-brief-chain` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `86-95` | `the-twelve-verbs` | `—` |
| `verbs-resolve` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `96-108` | `the-twelve-verbs` | `—` |
| `verbs-leaf-add` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `109-132` | `the-twelve-verbs` | `—` |
| `verbs-leaf-insert` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `133-153` | `the-twelve-verbs` | `—` |
| `verbs-not-a-thirteenth-verb` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `154-208` | `the-twelve-verbs` | `—` |
| `verbs-leaf-decompose` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `209-229` | `the-twelve-verbs` | `—` |
| `verbs-decomposed` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `230-238` | `the-twelve-verbs` | `—` |
| `verbs-leaf-retire` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `239-247` | `the-twelve-verbs` | `—` |
| `verbs-leaf-prune` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `248-267` | `the-twelve-verbs` | `—` |
| `verbs-pruned` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `268-277` | `the-twelve-verbs` | `—` |
| `verbs-finish-commit` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `278-300` | `the-twelve-verbs` | `—` |
| `verbs-complete` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `301-325` | `the-twelve-verbs` | `—` |
| `verbs-signal-channel` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `326-339` | `the-twelve-verbs` | `—` |
| `verbs-signalled` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `340-349` | `the-twelve-verbs` | `—` |
| `verbs-sought` | `the-verbs` | `source-verbs` | `literal` | `twelve-not-fourteen` | `350-361` | `the-twelve-verbs` | `—` |
| `source-driver` | `source-index` | `source-driver` | `root` | `—` | `1-57` | `—` | `driver-operations` |
| `driver-not-fourteen-header` | `the-verbs` | `source-driver` | `literal` | `twelve-not-fourteen` | `1-26` | `driver-operations` | `—` |
| `driver-operations` | `the-verbs` | `source-driver` | `composite` | `twelve-not-fourteen` | `1-57` | `source-driver` | `driver-not-fourteen-header`, `driver-imports`, `driver-transition-to-current`, `driver-materialize-finish` |
| `driver-imports` | `the-verbs` | `source-driver` | `literal` | `twelve-not-fourteen` | `27-32` | `driver-operations` | `—` |
| `driver-transition-to-current` | `the-verbs` | `source-driver` | `literal` | `twelve-not-fourteen` | `33-42` | `driver-operations` | `—` |
| `driver-materialize-finish` | `the-verbs` | `source-driver` | `literal` | `twelve-not-fourteen` | `43-57` | `driver-operations` | `—` |
| `source-complete` | `source-index` | `source-complete` | `root` | `—` | `1-96` | `—` | `complete-verb` |
| `complete-header-in-plain-comments` | `the-verbs` | `source-complete` | `literal` | `twelve-not-fourteen` | `1-18` | `complete-verb` | `—` |
| `complete-verb` | `the-verbs` | `source-complete` | `composite` | `twelve-not-fourteen` | `1-96` | `source-complete` | `complete-header-in-plain-comments`, `complete-imports`, `complete-disposition`, `complete-disposition-token`, `complete-interpret`, `complete-signal` |
| `complete-imports` | `the-verbs` | `source-complete` | `literal` | `twelve-not-fourteen` | `19-21` | `complete-verb` | `—` |
| `complete-disposition` | `the-verbs` | `source-complete` | `literal` | `twelve-not-fourteen` | `22-38` | `complete-verb` | `—` |
| `complete-disposition-token` | `the-verbs` | `source-complete` | `literal` | `twelve-not-fourteen` | `39-52` | `complete-verb` | `—` |
| `complete-interpret` | `the-verbs` | `source-complete` | `literal` | `twelve-not-fourteen` | `53-86` | `complete-verb` | `—` |
| `complete-signal` | `the-verbs` | `source-complete` | `literal` | `twelve-not-fourteen` | `87-96` | `complete-verb` | `—` |
| `source-driver-lease` | `source-index` | `source-driver-lease` | `root` | `—` | `1-1383` | `—` | `lease-and-epoch`, `lease-tests` |
| `lease-module-header` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `1-9` | `lease-and-epoch` | `—` |
| `lease-and-epoch` | `the-lease` | `source-driver-lease` | `composite` | `one-per-working-tree` | `1-819` | `source-driver-lease` | `lease-module-header`, `lease-imports`, `lease-namespace`, `lease-names-and-bounds`, `lease-file-identity`, `lease-process-record`, `lease-epoch-record`, `lease-lock-mode`, `lease-lock-mode-impl`, `lease-file-identity-impl`, `lease-driver-lease-type`, `lease-session-epoch-guard-type`, `lease-require-signal-path`, `lease-acquire`, `lease-acquire-with`, `lease-worktree-root`, `lease-control-dir`, `lease-epoch-transitions`, `lease-revalidate`, `lease-write-epoch-record`, `lease-initialize-epoch-record`, `lease-write-epoch-contents`, `lease-acquire-lease-file`, `lease-acquire-epoch-file`, `lease-contention-diagnostic`, `lease-acquire-epoch-file-with`, `lease-acquire-lease-file-with-hook`, `lease-lock-exclusively`, `lease-close-on-exec`, `lease-random-nonce`, `lease-hex-nonce`, `lease-encode-path`, `lease-decode-path`, `lease-record-field`, `lease-parse-process-record`, `lease-read-record`, `lease-read-epoch-record`, `lease-probe-live-lease`, `lease-probe-with-hook`, `lease-admit-ambient-session`, `lease-ambient-signal-path`, `lease-signal-path-from`, `lease-admit-session`, `lease-write-record` |
| `lease-imports` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `10-21` | `lease-and-epoch` | `—` |
| `lease-namespace` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `22-30` | `lease-and-epoch` | `—` |
| `lease-names-and-bounds` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `31-36` | `lease-and-epoch` | `—` |
| `lease-file-identity` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `37-42` | `lease-and-epoch` | `—` |
| `lease-process-record` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `43-49` | `lease-and-epoch` | `—` |
| `lease-epoch-record` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `50-55` | `lease-and-epoch` | `—` |
| `lease-lock-mode` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `56-61` | `lease-and-epoch` | `—` |
| `lease-lock-mode-impl` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `62-77` | `lease-and-epoch` | `—` |
| `lease-file-identity-impl` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `78-86` | `lease-and-epoch` | `—` |
| `lease-driver-lease-type` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `87-102` | `lease-and-epoch` | `—` |
| `lease-session-epoch-guard-type` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `103-108` | `lease-and-epoch` | `—` |
| `lease-require-signal-path` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `109-133` | `lease-and-epoch` | `—` |
| `lease-acquire` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `134-145` | `lease-and-epoch` | `—` |
| `lease-acquire-with` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `146-199` | `lease-and-epoch` | `—` |
| `lease-worktree-root` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `200-206` | `lease-and-epoch` | `—` |
| `lease-control-dir` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `207-216` | `lease-and-epoch` | `—` |
| `lease-epoch-transitions` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `217-224` | `lease-and-epoch` | `—` |
| `lease-revalidate` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `225-265` | `lease-and-epoch` | `—` |
| `lease-write-epoch-record` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `266-277` | `lease-and-epoch` | `—` |
| `lease-initialize-epoch-record` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `278-300` | `lease-and-epoch` | `—` |
| `lease-write-epoch-contents` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `301-330` | `lease-and-epoch` | `—` |
| `lease-acquire-lease-file` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `331-334` | `lease-and-epoch` | `—` |
| `lease-acquire-epoch-file` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `335-350` | `lease-and-epoch` | `—` |
| `lease-contention-diagnostic` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `351-357` | `lease-and-epoch` | `—` |
| `lease-acquire-epoch-file-with` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `358-444` | `lease-and-epoch` | `—` |
| `lease-acquire-lease-file-with-hook` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `445-498` | `lease-and-epoch` | `—` |
| `lease-lock-exclusively` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `499-516` | `lease-and-epoch` | `—` |
| `lease-close-on-exec` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `517-531` | `lease-and-epoch` | `—` |
| `lease-random-nonce` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `532-540` | `lease-and-epoch` | `—` |
| `lease-hex-nonce` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `541-548` | `lease-and-epoch` | `—` |
| `lease-encode-path` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `549-556` | `lease-and-epoch` | `—` |
| `lease-decode-path` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `557-569` | `lease-and-epoch` | `—` |
| `lease-record-field` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `570-581` | `lease-and-epoch` | `—` |
| `lease-parse-process-record` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `582-604` | `lease-and-epoch` | `—` |
| `lease-read-record` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `605-613` | `lease-and-epoch` | `—` |
| `lease-read-epoch-record` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `614-635` | `lease-and-epoch` | `—` |
| `lease-probe-live-lease` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `636-639` | `lease-and-epoch` | `—` |
| `lease-probe-with-hook` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `640-703` | `lease-and-epoch` | `—` |
| `lease-admit-ambient-session` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `704-721` | `lease-and-epoch` | `—` |
| `lease-ambient-signal-path` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `722-732` | `lease-and-epoch` | `—` |
| `lease-signal-path-from` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `733-740` | `lease-and-epoch` | `—` |
| `lease-admit-session` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `741-796` | `lease-and-epoch` | `—` |
| `lease-write-record` | `the-lease` | `source-driver-lease` | `literal` | `one-per-working-tree` | `797-819` | `lease-and-epoch` | `—` |
| `epoch-tests-module-open` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `820-821` | `lease-tests` | `—` |
| `lease-tests` | `the-epoch` | `source-driver-lease` | `composite` | `which-calls-are-admitted` | `820-1383` | `source-driver-lease` | `epoch-tests-module-open`, `epoch-tests-workspace-fixture`, `epoch-tests-imports`, `epoch-tests-ambient-fixture`, `epoch-tests-fork-guard`, `epoch-tests-replace-locked`, `epoch-tests-retry-until-current`, `epoch-tests-fails-closed`, `epoch-tests-close-on-exec`, `epoch-tests-stable-record`, `epoch-tests-event-order`, `epoch-tests-orphaned-timeout`, `epoch-tests-contention-text`, `epoch-tests-manual-operations`, `epoch-tests-nonempty-ambient`, `epoch-tests-old-finishes`, `epoch-tests-record-until-handoff`, `epoch-tests-foreign-worktree`, `epoch-tests-inactive-reported`, `epoch-tests-rotated-signal`, `epoch-tests-separator-bytes`, `epoch-tests-probe-releases`, `epoch-tests-active-no-lease`, `epoch-tests-malformed` |
| `epoch-tests-workspace-fixture` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `822-828` | `lease-tests` | `—` |
| `epoch-tests-imports` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `829-838` | `lease-tests` | `—` |
| `epoch-tests-ambient-fixture` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `839-848` | `lease-tests` | `—` |
| `epoch-tests-fork-guard` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `849-884` | `lease-tests` | `—` |
| `epoch-tests-replace-locked` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `885-890` | `lease-tests` | `—` |
| `epoch-tests-retry-until-current` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `891-915` | `lease-tests` | `—` |
| `epoch-tests-fails-closed` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `916-939` | `lease-tests` | `—` |
| `epoch-tests-close-on-exec` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `940-961` | `lease-tests` | `—` |
| `epoch-tests-stable-record` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `962-1000` | `lease-tests` | `—` |
| `epoch-tests-event-order` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1001-1041` | `lease-tests` | `—` |
| `epoch-tests-orphaned-timeout` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1042-1080` | `lease-tests` | `—` |
| `epoch-tests-contention-text` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1081-1091` | `lease-tests` | `—` |
| `epoch-tests-manual-operations` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1092-1099` | `lease-tests` | `—` |
| `epoch-tests-nonempty-ambient` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1100-1117` | `lease-tests` | `—` |
| `epoch-tests-old-finishes` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1118-1177` | `lease-tests` | `—` |
| `epoch-tests-record-until-handoff` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1178-1220` | `lease-tests` | `—` |
| `epoch-tests-foreign-worktree` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1221-1245` | `lease-tests` | `—` |
| `epoch-tests-inactive-reported` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1246-1260` | `lease-tests` | `—` |
| `epoch-tests-rotated-signal` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1261-1284` | `lease-tests` | `—` |
| `epoch-tests-separator-bytes` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1285-1304` | `lease-tests` | `—` |
| `epoch-tests-probe-releases` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1305-1346` | `lease-tests` | `—` |
| `epoch-tests-active-no-lease` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1347-1366` | `lease-tests` | `—` |
| `epoch-tests-malformed` | `the-epoch` | `source-driver-lease` | `literal` | `which-calls-are-admitted` | `1367-1383` | `lease-tests` | `—` |
| `source-session-config` | `source-index` | `source-session-config` | `root` | `—` | `1-358` | `—` | `whose-file` |
| `config-header` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `1-17` | `whose-file` | `—` |
| `whose-file` | `which-files` | `source-session-config` | `composite` | `whose-file-and-whether` | `1-358` | `source-session-config` | `config-header`, `config-imports`, `config-two-paths`, `config-four-slots`, `config-vocabulary`, `config-expansion-context`, `config-delta-roots`, `config-template-source`, `config-template-source-open`, `config-from-env`, `config-personal-path`, `config-template-source-load`, `config-session-config`, `config-path-and-candidates`, `config-load`, `config-read`, `config-load-for-worktree`, `config-source-and-require`, `config-expand`, `config-find-delta`, `config-refuse-tracked`, `config-delta-is-tracked` |
| `config-imports` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `18-25` | `whose-file` | `—` |
| `config-two-paths` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `26-30` | `whose-file` | `—` |
| `config-four-slots` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `31-56` | `whose-file` | `—` |
| `config-vocabulary` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `57-63` | `whose-file` | `—` |
| `config-expansion-context` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `64-70` | `whose-file` | `—` |
| `config-delta-roots` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `71-84` | `whose-file` | `—` |
| `config-template-source` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `85-101` | `whose-file` | `—` |
| `config-template-source-open` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `102-108` | `whose-file` | `—` |
| `config-from-env` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `109-123` | `whose-file` | `—` |
| `config-personal-path` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `124-130` | `whose-file` | `—` |
| `config-template-source-load` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `131-136` | `whose-file` | `—` |
| `config-session-config` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `137-140` | `whose-file` | `—` |
| `config-path-and-candidates` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `141-155` | `whose-file` | `—` |
| `config-load` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `156-171` | `whose-file` | `—` |
| `config-read` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `172-181` | `whose-file` | `—` |
| `config-load-for-worktree` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `182-200` | `whose-file` | `—` |
| `config-source-and-require` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `201-220` | `whose-file` | `—` |
| `config-expand` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `221-260` | `whose-file` | `—` |
| `config-find-delta` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `261-293` | `whose-file` | `—` |
| `config-refuse-tracked` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `294-326` | `whose-file` | `—` |
| `config-delta-is-tracked` | `which-files` | `source-session-config` | `literal` | `whose-file-and-whether` | `327-358` | `whose-file` | `—` |
| `source-prompt` | `source-index` | `source-prompt` | `root` | `—` | `1-245` | `—` | `the-prompt-core` |
| `core-header` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `1-41` | `the-prompt-core` | `—` |
| `the-prompt-core` | `the-core` | `source-prompt` | `composite` | `too-late-to-say-later` | `1-245` | `source-prompt` | `core-header`, `core-imports`, `core-plugin`, `core-skill-name`, `core-load-instruction`, `core-runtime-facts`, `core-signalling-contract`, `core-mandate`, `core-compose`, `core-stated-vcs` |
| `core-imports` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `42-45` | `the-prompt-core` | `—` |
| `core-plugin` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `46-54` | `the-prompt-core` | `—` |
| `core-skill-name` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `55-65` | `the-prompt-core` | `—` |
| `core-load-instruction` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `66-107` | `the-prompt-core` | `—` |
| `core-runtime-facts` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `108-125` | `the-prompt-core` | `—` |
| `core-signalling-contract` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `126-178` | `the-prompt-core` | `—` |
| `core-mandate` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `179-201` | `the-prompt-core` | `—` |
| `core-compose` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `202-222` | `the-prompt-core` | `—` |
| `core-stated-vcs` | `the-core` | `source-prompt` | `literal` | `too-late-to-say-later` | `223-245` | `the-prompt-core` | `—` |
| `source-loop-driver` | `source-index` | `source-loop-driver` | `root` | `—` | `1-615` | `—` | `loop-driver` |
| `loop-header` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `1-55` | `loop-driver` | `—` |
| `loop-driver` | `the-loop` | `source-loop-driver` | `composite` | `four-things-a-runner-cannot-choose` | `1-615` | `source-loop-driver` | `loop-header`, `loop-imports`, `loop-worktree-name`, `loop-control-env`, `loop-channel-var`, `loop-scrub-list`, `loop-scrub-helper`, `loop-outcome`, `loop-run`, `loop-drive-open`, `loop-drive-interrupt`, `loop-drive-selection`, `loop-drive-expand`, `loop-drive-launch`, `loop-drive-discard`, `loop-drive-interrupted`, `loop-drive-endings`, `loop-session-prompt`, `loop-launch-contract`, `loop-launch-spawn`, `loop-handoff`, `loop-escalation`, `loop-reset-terminal`, `loop-ignore-interrupts`, `loop-picked`, `loop-tests-open`, `loop-test-handoff-preserves`, `loop-test-ordering` |
| `loop-imports` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `56-67` | `loop-driver` | `—` |
| `loop-worktree-name` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `68-75` | `loop-driver` | `—` |
| `loop-control-env` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `76-115` | `loop-driver` | `—` |
| `loop-channel-var` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `116-121` | `loop-driver` | `—` |
| `loop-scrub-list` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `122-126` | `loop-driver` | `—` |
| `loop-scrub-helper` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `127-137` | `loop-driver` | `—` |
| `loop-outcome` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `138-166` | `loop-driver` | `—` |
| `loop-run` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `167-201` | `loop-driver` | `—` |
| `loop-drive-open` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `202-225` | `loop-driver` | `—` |
| `loop-drive-interrupt` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `226-242` | `loop-driver` | `—` |
| `loop-drive-selection` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `243-259` | `loop-driver` | `—` |
| `loop-drive-expand` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `260-279` | `loop-driver` | `—` |
| `loop-drive-launch` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `280-307` | `loop-driver` | `—` |
| `loop-drive-discard` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `308-313` | `loop-driver` | `—` |
| `loop-drive-interrupted` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `314-318` | `loop-driver` | `—` |
| `loop-drive-endings` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `319-344` | `loop-driver` | `—` |
| `loop-session-prompt` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `345-377` | `loop-driver` | `—` |
| `loop-launch-contract` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `378-417` | `loop-driver` | `—` |
| `loop-launch-spawn` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `418-439` | `loop-driver` | `—` |
| `loop-handoff` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `440-457` | `loop-driver` | `—` |
| `loop-escalation` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `458-470` | `loop-driver` | `—` |
| `loop-reset-terminal` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `471-502` | `loop-driver` | `—` |
| `loop-ignore-interrupts` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `503-533` | `loop-driver` | `—` |
| `loop-picked` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `534-546` | `loop-driver` | `—` |
| `loop-tests-open` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `547-556` | `loop-driver` | `—` |
| `loop-test-handoff-preserves` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `557-592` | `loop-driver` | `—` |
| `loop-test-ordering` | `the-loop` | `source-loop-driver` | `literal` | `four-things-a-runner-cannot-choose` | `593-615` | `loop-driver` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Outcome`, `TokenError` | `01-orientation.md#the-cast` | `four-verdicts` | The terminal marks a name can carry — `DONE` and `ABANDONED` — and the refusal a token that is not well-formed produces. | `explained` |
| `Handle`, `HandleError`, `Kind`, `Parts`, `Slug` | `01-orientation.md#the-cast` | `the-handle-not-the-position` | The named parts of a task name: a kind token, a slug, and the `<slug>-k<key>` handle that is the entry's identity. `Parts` is the set of them a positioned name decomposes into. | `explained` |
| `TaskName` | `01-orientation.md#the-cast` | `canonical-or-nothing` | One parsed entry name, which renders back to the bytes it was parsed from or refuses to be computed at all. | `explained` |
| `Tree`, `Vacancy`, `task_tree::Guard`, `task_tree::write` | `01-orientation.md#the-cast` | `one-spelling-of-grove` | The tree read under the store's shared lock; the lock over a root that holds no tree; the store guard one mutation consumes; and the reopening a `TreeWrite` performs when it no longer holds one. | `explained` |
| `Selection` | `01-orientation.md#the-cast` | `first-live-leaf` | The leaf a session was launched to work: its path, its identity and its kind. | `explained` |
| `verbs::resolve`, `Resolution` | `01-orientation.md#the-cast` | `wider-than-a-key` | Resolution of one reference against the tree, whose `Ambiguous` case lists the keys of every entry a bare slug matched. | `explained` |
| `verbs::root_init` | `01-orientation.md#the-cast` | `never-mistaken-for-finished` | The verb that consumes a `Vacancy` and creates the whole grove — charter and first live leaf — as one store operation. | `explained` |
| `interpret`, `Disposition` | `01-orientation.md#the-cast` | `twelve-not-fourteen` | What the child side of the loop makes of a token written to the control channel: relaunch, or stop. | `explained` |
| `verbs`, `verbs::stale_cross_refs`, `verbs::signal_channel` | `01-orientation.md#the-cast` | `twelve-not-fourteen` | `verbs` declares fourteen public functions; twelve of them are the tree's verb surface, and `stale_cross_refs` and `signal_channel` each say in their own doc comment why they are not verbs. | `explained` |
| `admit_ambient_session`, `DriverLease`, `SessionEpochGuard` | `01-orientation.md#the-cast` | `one-per-working-tree` | The lease that keeps one live driver per working tree, the epoch that decides which calls it admits, and the check a session runs when there is no driver at all. | `explained` |
| `SessionConfig`, `TemplateSource` | `01-orientation.md#the-cast` | `whose-file-and-whether` | Whose configuration file a launch is expanded from, and whether a second one beside it is admissible. | `explained` |
| `compose`, `Mandate` | `01-orientation.md#the-cast` | `too-late-to-say-later` | The prompt a session is launched with, composed from the parts a skill cannot supply because by the time it could speak the moment has passed. | `explained` |
| `run`, `LoopOutcome` | `01-orientation.md#the-cast` | `four-things-a-runner-cannot-choose` | The loop itself, and how it ends: relaunched with fresh context, stopped resumably, or interrupted. | `explained` |
| `TaskName::Brief` | `02-the-tokens.md#the-four-verdicts` | `canonical-or-nothing` | The distinguished value supplied by the lifecycle callers for initialization and promotion. The classification test checks its species and rendered filename. | `explained` |
| `TaskName`, `TaskNameError`, `Verdict`, `verdict`, `entry`, `malformed` | `02-the-tokens.md#the-four-verdicts` | `canonical-or-nothing` | The parsed name, its refusal type, the classification a caller reads, and the three test helpers that reach a verdict: the only way to a verdict is through the `EntryName` implementation chapter 4 owns. | `explained` |
| `Kind::new`, `Slug::new` | `02-the-tokens.md#both-words-one-rule` | `the-handle-not-the-position` | The two constructors chapter 3 defines over the rule this block states: each hands its string to `refuse_token` and returns the token or the one `TokenError`, which is what makes *one rule* a fact about the code rather than an agreement between two types. | `explained` |
| `Handle::render` | `02-the-tokens.md#the-handle-in-this-grammar` | `the-handle-not-the-position` | The private handle renderer composes the title and shared key suffix. Chapter 3 defines it; `Handle`'s own `Display` and leaf-name rendering call it. Node-directory rendering shares its key-suffix renderer without carrying a title. | `explained` |
| `render_key`, `peel_key` | `02-the-tokens.md#the-handle-in-this-grammar` | `canonical-or-nothing` | Private helpers for the key suffix: `render_key` writes it, while `peel_key` separates it from the preceding text. Both belong to chapter 4; neither obtains a node title. | `explained` |
| `split_shape` | `02-the-tokens.md#the-handle-in-this-grammar` | `canonical-or-nothing` | The private free function that splits a task-shaped stem into position digits, an unexamined middle and key digits — reaching the key by calling `peel_key` rather than finding it itself, which is why the header can say there is one peel. Chapter 4 reads it. | `explained` |
| `Parts::Node` | `02-the-tokens.md#the-outcome` | `the-handle-not-the-position` | The node arm of chapter 3's `Parts`, carrying no payload. It has no outcome field at all rather than one constrained to a single value, so a node directory wearing an outcome is not a state the type can hold. | `explained` |
| `pick` | `02-the-tokens.md#the-outcome` | `first-live-leaf` | The verb that answers *what next*: a depth-first pre-order walk returning the first leaf still live, skipping briefs and the `DONE` and `ABANDONED` leaves this block's `Outcome` marks. Chapter 7 reads the walk. | `explained` |
| `Parts::leaf` | `02-the-tokens.md#refusals-inside-the-shape` | `the-handle-not-the-position` | The constructor for the leaf half of `Parts`, taking an outcome, a session kind and a slug — the named parts a positioned leaf name decomposes into. | `explained` |
| `a_kind`, `slug` | `02-the-tokens.md#refusals-inside-the-shape` | `canonical-or-nothing` | Two test helpers defined beside the conformance kit: each takes a label, builds the token type it names, and panics if the label is not well-formed, so an invalid fixture is a test bug rather than a compile error. | `explained` |
| `impl Display for TaskNameError` | `02-the-tokens.md#refusals-inside-the-shape` | `canonical-or-nothing` | The renderer chapter 4 reads, which writes each refusal's recovery advice and not merely its detection. Three of this section's six tests assert on that rendered text, so the advice is part of what they pin rather than commentary beside it. | `explained` |
| `impl Display for TaskName` | `02-the-tokens.md#refusals-inside-the-shape` | `canonical-or-nothing` | The renderer that writes a parsed name back to its filename bytes. Leaves reuse handle rendering; directories share its key-suffix renderer; node files carry no key. The round-trip test pins each of two names to its own bytes, so the reader needs the rendering direction here. | `explained` |
| `TaskName::Brief`, `TaskName::NodeFile`, `TaskName::Positioned` | `03-kind-slug-handle.md#one-place-the-grammar-is-spelled` | `canonical-or-nothing` | The parsed name distinguishes the root node file, a titled node file and a positioned entry. Node files carry no ordinal or key. `Handle::of` derives a leaf handle; node handle composition also requires the titled node file. | `explained` |
| `TaskName::parse` | `03-kind-slug-handle.md#one-place-the-grammar-is-spelled` | `canonical-or-nothing` | The one route from a filename to a parsed name, and the canonical half of the asymmetry `Handle::parse` is documented against: it refuses a name spelled any way but the one the renderer would have written. | `explained` |
| `terminal_key` | `03-kind-slug-handle.md#one-place-the-grammar-is-spelled` | `canonical-or-nothing` | The public function that answers *does this reference end in a key* and requires nothing of what precedes it, which is why `resolve`'s bare-slug fallback asks it rather than `Handle::parse`. It reaches the key through the same `peel_key`. | `explained` |
| `parse_ref` | `03-kind-slug-handle.md#one-place-the-grammar-is-spelled` | `wider-than-a-key` | The reference grammar accepts bare numeric keys independently of canonical full handles; `007` is key 7, while `a-k007` is not a canonical handle. | `explained` |
| `TaskName::compose` | `03-kind-slug-handle.md#the-handle-is-the-identity` | `canonical-or-nothing` | Composition builds a positioned name from a position, a kind, a slug and a key, so the handle's structural claim can be asserted over names built rather than parsed. | `explained` |
| `entry_path` | `05-opening.md#one-spelling-of-the-root` | `paths-are-built-here` | The one place an entry's absolute path is built, because the store returns no paths. Chapter 5 reproduces the module header that says so; chapter 6 reads the function. | `explained` |
| `target` | `05-opening.md#one-spelling-of-the-root` | `paths-are-built-here` | The other function the module header names as a place canonicalisation happens: it resolves a caller's path to a snapshot entry, canonicalising the candidate, the grove root and each walked entry's built path. Chapter 5 needs only that it does what `leaf_entry` does and returns no path, which is what makes the header's clause true of two functions rather than one. Chapter 6 reads it. | `explained` |
| `brief_chain`, `kind_in` | `05-opening.md#one-spelling-of-the-root` | `root-to-leaf` | Two of the five reading verbs the module header names in its first sentence: one answers a leaf's session kind, the other its ancestors' briefs root to leaf. Chapter 5 needs only the header's claim that all five read one snapshot taken under one lock; chapter 8 reads both functions. | `explained` |
| `leaf_entry` | `05-opening.md#one-spelling-of-the-root` | `root-to-leaf` | One of the two places canonicalisation happens, and there only to *compare* a caller's spelling of a leaf against the tree's — never to produce a path grove hands back. The module header chapter 5 reproduces names it beside `target`; chapter 6 owns `target` and explains why the clause is scoped to this module. Chapter 8 reads this function. | `explained` |
| `tree_lifecycle::leaf_prune` | `05-opening.md#the-four-openings` | `marked-in-place` | The bulk mark that carries the consequence of a mutation consuming its guard: it marks each entry under a guard of its own, so a run interrupted part way leaves some entries marked and some not. Chapter 13 reads it. | `explained` |
| `tree_lifecycle::leaf_decompose` | `06-paths.md#which-entry-a-path-names` | `the-key-survives` | The verb that turns a live leaf file into a node directory, keeping the entry's key and moving its body in as the node's `_<slug>.md`. Chapter 6 needs only that it acts on a leaf and preserves the key — which is what makes an interrupted one two halves of one entity rather than two entities. Chapter 12 reads it. | `explained` |
| `tree_lifecycle::leaf_retire` | `06-paths.md#which-entry-a-path-names` | `marked-in-place` | The verb that marks a live leaf `DONE` in place, keeping its position and its key. Chapter 6 needs only that it acts on one named leaf, because *aimed by path at one twin, it silently marks the other and reports success* is the failure `addressable_key` exists to prevent. Chapter 13 reads it. | `explained` |
| `existing_path` | `06-paths.md#canonicalise-to-compare` | `wider-than-a-key` | The function that turns a caller's argument into a path that exists — absolute, or joined onto the grove root, or onto the cwd — and it resolves no entry and canonicalises nothing: it tests `exists()` and returns the path it tried. Chapter 6 needs only that it interprets a path argument without canonicalising, which is what makes *resolved to an entry* narrower than *interprets a path*. Chapter 9 reads it. | `explained` |
| `task_grow::allocated` | `06-paths.md#predicting-the-allocation` | `what-the-library-cannot-see` | The check every grow verb runs over `next_key`'s prediction: it compares the predicted key against the one the library reports and refuses to claim success on a disagreement, which is what keeps a leaf's embedded handle from contradicting its own filename. Chapter 10 reads it. | `explained` |
| `pick_in`, `select_in` | `06-paths.md#compositions-that-are-the-tests-alone` | `first-live-leaf` | The two walk operations the test module's compositions call after opening the tree: one answers the path of the first live leaf, the other every launch fact about it. Chapter 6 needs only that each takes an already-open tree; chapter 7 reads both. | `explained` |
| `reset_read_count`, `read_count` | `07-the-walk.md#nineteen-tests` | `wider-than-a-key` | The counter's two accessors: one sets the thread-local read count to zero, the other returns it. Chapter 7's one-observation test resets before the call and asserts the count is `1`, so it needs only that the pair reads the `READ_COUNT` chapter 5 declared; chapter 9 owns the lines they sit on. | `explained` |
| `verbs::kind` | `08-kind-and-briefs.md#two-questions-one-entry` | `twelve-not-fourteen` | The public verb of the same name in `verbs.rs`, which chapter 8 names in order to say that `kind_in`'s doc comment does *not* point at it: it takes an already-open `&Tree`, calls `kind_in`, and renders the answer as a `Sought`. Chapter 8 needs only that much of it; chapter 15 reads it. | `explained` |
| `verbs::brief_chain` | `08-kind-and-briefs.md#the-chain-the-library-already-had` | `twelve-not-fourteen` | The public verb the doc comment names in its hyphenated spelling, `brief-chain`, and whose documented contract — each level requires a correctly placed node file — the comment appeals to. Chapter 8 needs only that the contract belongs to the verb and the guide rather than to this function; chapter 15 reads it. | `explained` |
| `verbs::leaf_add` | `10-growing.md#a-list-is-not-n-calls` | `twelve-not-fourteen` | The public verb this module-private function is the body of, named in the doc comment's own hyphenated spelling, `leaf-add`. Chapter 10 needs only that the verb is the public half — it takes a `TreeWrite` and hands this function the guard — and takes a validated slug, because the comment's *unreachable from the verb* is a claim about what reaches this function through it; chapter 15 reads it. | `explained` |
| `verbs::leaf_insert` | `10-growing.md#an-entry-where-the-library-names-an-ordinal` | `twelve-not-fourteen` | The public verb behind `leaf-insert`, named in the doc comment's hyphenated spelling. Chapter 10 needs only that it is the public half — it takes a `TreeWrite` and hands this function the guard — and that a second wrapper beside it performs the separate shared opening the lint requires; chapter 15 reads both. | `explained` |
| `tree_lifecycle::initialize_grove` | `10-growing.md#the-prediction-held-to-account` | `never-mistaken-for-finished` | The function `root-init` runs, which creates a grove's root, its charter and its first live leaf as one store operation. Chapter 10 needs only that its report begins with a charter carrying no key, because that is why `allocated` takes a slice of created rows rather than a whole report; chapter 11 reads it. | `explained` |
| `DEFAULT_ROOT_SLUG` | `11-a-grove-begins.md#one-operation-or-none` | `the-tree-deletes-itself` | The crate-level constant `default_root_slug` wraps, declared above the finishing code the file opens on. Chapter 11 needs only that its value is `"plan"` and that the CLI states the same default separately; chapter 14 owns the block it is declared in. | `explained` |
| `transition_to_current` | `11-a-grove-begins.md#the-value-nothing-holds` | `the-tree-deletes-itself` | The driver-facing operation that classifies an existing `.grove/` or scaffolds an absent one. Chapter 11 needs only that it is the sole caller of `default_root_slug` and `root_shape`; chapter 14 reads it. | `explained` |
| `CurrentTransition` | `11-a-grove-begins.md#what-the-tests-establish` | `the-tree-deletes-itself` | What `transition_to_current` answers: the grove was already current, or it was initialized. Chapter 11 needs only the two variants its own tests assert on; chapter 14 owns the type. | `explained` |
| `verbs::leaf_decompose` | `12-leaf-to-node.md#one-promote-that-had-to-be-one` | `twelve-not-fourteen` | The public verb behind `leaf-decompose`, named in this block's own doc comment in the hyphenated `leaf-decompose <leaf-path> <first-child-slug>` spelling. Chapter 12 needs only that it is the public half — it takes a `TreeWrite` and hands this function the guard — and that its `--kind` is documented as an override that excludes the driver-reserved `finish`; chapter 15 reads it. | `explained` |
| `verbs::leaf_retire` | `13-outcomes.md#two-verbs-one-mark` | `twelve-not-fourteen` | The public verb behind `leaf-retire`, named in this block's own doc comment in the hyphenated `leaf-retire <leaf-path>` spelling. Chapter 13 needs only that it is the public half — it takes a `TreeWrite` and hands this function the guard — and that the path it accepts is absolute or relative to the grove root; chapter 15 reads it. | `explained` |
| `verbs::leaf_prune` | `13-outcomes.md#one-guard-is-one-mark` | `twelve-not-fourteen` | The public verb behind `leaf-prune`, named in this block's doc comment in the hyphenated `leaf-prune <path>` spelling and again in the operator instruction `stopped_partway` renders. Chapter 13 needs only that it is the public half — it takes a `TreeWrite` and hands this function the guard — and that the command an operator reruns is spelled `grove-llm leaf-prune`; chapter 15 reads it. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every line of the thirteen source roots is credited once, to the slice whose
page owns it; the table shows how the 10,458 lines divide across the twenty-one
chapters, and its total is what a completed book must account for. Nine roots are
owned whole by one chapter; the four that split — `src/task_name.rs` three ways,
`src/task_tree.rs` five, `src/tree_lifecycle.rs` four and `src/driver_lease.rs`
two — are why the ownership table above has thirty-nine rows rather than thirteen.

| Slice | Page | Owned lines |
|---|---|---:|
| `allowed-to-mean` | `01-orientation.md` | 445 |
| `four-verdicts` | `02-the-tokens.md` | 373 |
| `the-handle-not-the-position` | `03-kind-slug-handle.md` | 529 |
| `canonical-or-nothing` | `04-the-name.md` | 773 |
| `one-spelling-of-grove` | `05-opening.md` | 290 |
| `paths-are-built-here` | `06-paths.md` | 320 |
| `first-live-leaf` | `07-the-walk.md` | 319 |
| `root-to-leaf` | `08-kind-and-briefs.md` | 438 |
| `wider-than-a-key` | `09-resolve.md` | 604 |
| `what-the-library-cannot-see` | `10-growing.md` | 518 |
| `never-mistaken-for-finished` | `11-a-grove-begins.md` | 641 |
| `the-key-survives` | `12-leaf-to-node.md` | 764 |
| `marked-in-place` | `13-outcomes.md` | 808 |
| `the-tree-deletes-itself` | `14-finishing.md` | 521 |
| `twelve-not-fourteen` | `15-the-verbs.md` | 514 |
| `one-per-working-tree` | `16-the-lease.md` | 819 |
| `which-calls-are-admitted` | `17-the-epoch.md` | 564 |
| `whose-file-and-whether` | `18-which-files.md` | 358 |
| `too-late-to-say-later` | `19-the-core.md` | 245 |
| `four-things-a-runner-cannot-choose` | `20-the-loop.md` | 615 |
| `assembly` | `21-what-could-not-move.md` | 0 |
| **Total** | 13 source roots | **10,458** |
