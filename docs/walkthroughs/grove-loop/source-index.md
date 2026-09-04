# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove-loop/Cargo.toml` | 59 |
| `source-library-root` | `crates/grove-loop/src/lib.rs` | 377 |
| `source-task-name` | `crates/grove-loop/src/task_name.rs` | 1,714 |
| `source-task-tree` | `crates/grove-loop/src/task_tree.rs` | 2,023 |
| `source-task-grow` | `crates/grove-loop/src/task_grow.rs` | 518 |
| `source-tree-lifecycle` | `crates/grove-loop/src/tree_lifecycle.rs` | 2,725 |
| `source-verbs` | `crates/grove-loop/src/verbs.rs` | 363 |
| `source-driver` | `crates/grove-loop/src/driver.rs` | 57 |
| `source-complete` | `crates/grove-loop/src/complete.rs` | 96 |
| `source-driver-lease` | `crates/grove-loop/src/driver_lease.rs` | 1,383 |
| `source-session-config` | `crates/grove-loop/src/session_config.rs` | 358 |
| `source-prompt` | `crates/grove-loop/src/prompt.rs` | 245 |
| `source-loop-driver` | `crates/grove-loop/src/loop_driver.rs` | 615 |

<!-- source-root «source-crate-manifest» source="crates/grove-loop/Cargo.toml" lines="1-59" -->
<!-- insert «manifest-domain-bound» -->
<!-- /source-root -->
<!-- source-root «source-library-root» source="crates/grove-loop/src/lib.rs" lines="1-377" -->
<!-- insert «library-root» -->
<!-- /source-root -->
<!-- source-root «source-task-name» source="crates/grove-loop/src/task_name.rs" lines="1-1714" -->
<!-- insert «tokens-and-verdicts» -->
<!-- defer «kind-slug-and-handle» owner="the-handle-not-the-position" lines="221-590" -->
<!-- defer «the-task-name» owner="canonical-or-nothing" lines="591-1020" -->
<!-- defer «name-test-support-and-kit» owner="canonical-or-nothing" lines="1021-1177" -->
<!-- insert «classification-verdict-tests» -->
<!-- defer «grammar-and-canonicity-tests» owner="canonical-or-nothing" lines="1200-1312" -->
<!-- insert «shape-refusal-tests» -->
<!-- defer «slug-rule-tests» owner="the-handle-not-the-position" lines="1522-1550" -->
<!-- defer «handle-grammar-tests» owner="the-handle-not-the-position" lines="1551-1714" -->
<!-- /source-root -->
<!-- source-root «source-task-tree» source="crates/grove-loop/src/task_tree.rs" lines="1-2023" -->
<!-- defer «tree-opening» owner="one-spelling-of-grove" lines="1-290" -->
<!-- defer «paths-and-addressing» owner="paths-are-built-here" lines="291-570" -->
<!-- defer «walk-selection» owner="first-live-leaf" lines="571-637" -->
<!-- defer «kind-and-brief-chain» owner="root-to-leaf" lines="638-746" -->
<!-- defer «resolution» owner="wider-than-a-key" lines="747-1015" -->
<!-- defer «path-composition-tests» owner="paths-are-built-here" lines="1016-1105" -->
<!-- defer «pick-tests» owner="first-live-leaf" lines="1106-1360" -->
<!-- defer «brief-chain-and-kind-tests» owner="root-to-leaf" lines="1361-1652" -->
<!-- defer «resolve-tests» owner="wider-than-a-key" lines="1653-1996" -->
<!-- defer «pick-with-brief-chain-tests» owner="root-to-leaf" lines="1997-2023" -->
<!-- /source-root -->
<!-- source-root «source-task-grow» source="crates/grove-loop/src/task_grow.rs" lines="1-518" -->
<!-- defer «growing-the-tree» owner="what-the-library-cannot-see" lines="1-518" -->
<!-- /source-root -->
<!-- source-root «source-tree-lifecycle» source="crates/grove-loop/src/tree_lifecycle.rs" lines="1-2725" -->
<!-- defer «finish-transition» owner="the-tree-deletes-itself" lines="1-331" -->
<!-- defer «grove-beginning» owner="never-mistaken-for-finished" lines="332-489" -->
<!-- defer «decompose-production» owner="the-key-survives" lines="490-695" -->
<!-- defer «outcomes-in-place» owner="marked-in-place" lines="696-1012" -->
<!-- defer «body-helpers» owner="never-mistaken-for-finished" lines="1013-1076" -->
<!-- defer «root-init-tests» owner="never-mistaken-for-finished" lines="1077-1466" -->
<!-- defer «finish-tests» owner="the-tree-deletes-itself" lines="1467-1665" -->
<!-- defer «decompose-tests» owner="the-key-survives" lines="1666-2234" -->
<!-- defer «retire-and-prune-tests» owner="marked-in-place" lines="2235-2725" -->
<!-- /source-root -->
<!-- source-root «source-verbs» source="crates/grove-loop/src/verbs.rs" lines="1-363" -->
<!-- defer «the-twelve-verbs» owner="twelve-not-fourteen" lines="1-363" -->
<!-- /source-root -->
<!-- source-root «source-driver» source="crates/grove-loop/src/driver.rs" lines="1-57" -->
<!-- defer «driver-operations» owner="twelve-not-fourteen" lines="1-57" -->
<!-- /source-root -->
<!-- source-root «source-complete» source="crates/grove-loop/src/complete.rs" lines="1-96" -->
<!-- defer «complete-verb» owner="twelve-not-fourteen" lines="1-96" -->
<!-- /source-root -->
<!-- source-root «source-driver-lease» source="crates/grove-loop/src/driver_lease.rs" lines="1-1383" -->
<!-- defer «lease-and-epoch» owner="one-per-working-tree" lines="1-819" -->
<!-- defer «lease-tests» owner="which-calls-are-admitted" lines="820-1383" -->
<!-- /source-root -->
<!-- source-root «source-session-config» source="crates/grove-loop/src/session_config.rs" lines="1-358" -->
<!-- defer «whose-file» owner="whose-file-and-whether" lines="1-358" -->
<!-- /source-root -->
<!-- source-root «source-prompt» source="crates/grove-loop/src/prompt.rs" lines="1-245" -->
<!-- defer «the-prompt-core» owner="too-late-to-say-later" lines="1-245" -->
<!-- /source-root -->
<!-- source-root «source-loop-driver» source="crates/grove-loop/src/loop_driver.rs" lines="1-615" -->
<!-- defer «loop-driver» owner="four-things-a-runner-cannot-choose" lines="1-615" -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-domain-bound` | `source-crate-manifest` | `allowed-to-mean` | `1-59` | 59 | `resolved` |
| `library-root` | `source-library-root` | `allowed-to-mean` | `1-377` | 377 | `resolved` |
| `tokens-and-verdicts` | `source-task-name` | `four-verdicts` | `1-220` | 220 | `resolved` |
| `kind-slug-and-handle` | `source-task-name` | `the-handle-not-the-position` | `221-590` | 370 | `deferred` |
| `the-task-name` | `source-task-name` | `canonical-or-nothing` | `591-1020` | 430 | `deferred` |
| `name-test-support-and-kit` | `source-task-name` | `canonical-or-nothing` | `1021-1177` | 157 | `deferred` |
| `classification-verdict-tests` | `source-task-name` | `four-verdicts` | `1178-1199` | 22 | `resolved` |
| `grammar-and-canonicity-tests` | `source-task-name` | `canonical-or-nothing` | `1200-1312` | 113 | `deferred` |
| `shape-refusal-tests` | `source-task-name` | `four-verdicts` | `1313-1521` | 209 | `resolved` |
| `slug-rule-tests` | `source-task-name` | `the-handle-not-the-position` | `1522-1550` | 29 | `deferred` |
| `handle-grammar-tests` | `source-task-name` | `the-handle-not-the-position` | `1551-1714` | 164 | `deferred` |
| `tree-opening` | `source-task-tree` | `one-spelling-of-grove` | `1-290` | 290 | `deferred` |
| `paths-and-addressing` | `source-task-tree` | `paths-are-built-here` | `291-570` | 280 | `deferred` |
| `walk-selection` | `source-task-tree` | `first-live-leaf` | `571-637` | 67 | `deferred` |
| `kind-and-brief-chain` | `source-task-tree` | `root-to-leaf` | `638-746` | 109 | `deferred` |
| `resolution` | `source-task-tree` | `wider-than-a-key` | `747-1015` | 269 | `deferred` |
| `path-composition-tests` | `source-task-tree` | `paths-are-built-here` | `1016-1105` | 90 | `deferred` |
| `pick-tests` | `source-task-tree` | `first-live-leaf` | `1106-1360` | 255 | `deferred` |
| `brief-chain-and-kind-tests` | `source-task-tree` | `root-to-leaf` | `1361-1652` | 292 | `deferred` |
| `resolve-tests` | `source-task-tree` | `wider-than-a-key` | `1653-1996` | 344 | `deferred` |
| `pick-with-brief-chain-tests` | `source-task-tree` | `root-to-leaf` | `1997-2023` | 27 | `deferred` |
| `growing-the-tree` | `source-task-grow` | `what-the-library-cannot-see` | `1-518` | 518 | `deferred` |
| `finish-transition` | `source-tree-lifecycle` | `the-tree-deletes-itself` | `1-331` | 331 | `deferred` |
| `grove-beginning` | `source-tree-lifecycle` | `never-mistaken-for-finished` | `332-489` | 158 | `deferred` |
| `decompose-production` | `source-tree-lifecycle` | `the-key-survives` | `490-695` | 206 | `deferred` |
| `outcomes-in-place` | `source-tree-lifecycle` | `marked-in-place` | `696-1012` | 317 | `deferred` |
| `body-helpers` | `source-tree-lifecycle` | `never-mistaken-for-finished` | `1013-1076` | 64 | `deferred` |
| `root-init-tests` | `source-tree-lifecycle` | `never-mistaken-for-finished` | `1077-1466` | 390 | `deferred` |
| `finish-tests` | `source-tree-lifecycle` | `the-tree-deletes-itself` | `1467-1665` | 199 | `deferred` |
| `decompose-tests` | `source-tree-lifecycle` | `the-key-survives` | `1666-2234` | 569 | `deferred` |
| `retire-and-prune-tests` | `source-tree-lifecycle` | `marked-in-place` | `2235-2725` | 491 | `deferred` |
| `the-twelve-verbs` | `source-verbs` | `twelve-not-fourteen` | `1-363` | 363 | `deferred` |
| `driver-operations` | `source-driver` | `twelve-not-fourteen` | `1-57` | 57 | `deferred` |
| `complete-verb` | `source-complete` | `twelve-not-fourteen` | `1-96` | 96 | `deferred` |
| `lease-and-epoch` | `source-driver-lease` | `one-per-working-tree` | `1-819` | 819 | `deferred` |
| `lease-tests` | `source-driver-lease` | `which-calls-are-admitted` | `820-1383` | 564 | `deferred` |
| `whose-file` | `source-session-config` | `whose-file-and-whether` | `1-358` | 358 | `deferred` |
| `the-prompt-core` | `source-prompt` | `too-late-to-say-later` | `1-245` | 245 | `deferred` |
| `loop-driver` | `source-loop-driver` | `four-things-a-runner-cannot-choose` | `1-615` | 615 | `deferred` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-59` | `—` | `manifest-domain-bound` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `1-10` | `manifest-domain-bound` | `—` |
| `manifest-domain-bound` | `orientation` | `source-crate-manifest` | `composite` | `allowed-to-mean` | `1-59` | `source-crate-manifest` | `manifest-package-identity`, `manifest-dependencies`, `manifest-extracted-tree`, `manifest-dev-dependencies`, `manifest-lints`, `manifest-release` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `11-30` | `manifest-domain-bound` | `—` |
| `manifest-extracted-tree` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `31-38` | `manifest-domain-bound` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `39-41` | `manifest-domain-bound` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `42-44` | `manifest-domain-bound` | `—` |
| `manifest-release` | `orientation` | `source-crate-manifest` | `literal` | `allowed-to-mean` | `45-59` | `manifest-domain-bound` | `—` |
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
| `source-task-name` | `source-index` | `source-task-name` | `root` | `—` | `1-1714` | `—` | `tokens-and-verdicts`, `kind-slug-and-handle`, `the-task-name`, `name-test-support-and-kit`, `classification-verdict-tests`, `grammar-and-canonicity-tests`, `shape-refusal-tests`, `slug-rule-tests`, `handle-grammar-tests` |
| `name-the-only-grammar` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1-12` | `tokens-and-verdicts` | `—` |
| `tokens-and-verdicts` | `the-tokens` | `source-task-name` | `composite` | `four-verdicts` | `1-220` | `source-task-name` | `name-the-only-grammar`, `name-three-on-disk-shapes`, `name-canonicity-departure`, `name-no-caller-hands-a-path`, `name-classification-loses-data`, `name-both-words-one-rule`, `name-handle-is-this-grammar`, `name-handle-terminal-substring`, `name-imports`, `name-brief-and-key-mark`, `name-separator`, `name-outcome`, `name-outcome-infix-and-strip`, `name-token-error`, `name-token-error-traits`, `name-refuse-token` |
| `name-three-on-disk-shapes` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `13-21` | `tokens-and-verdicts` | `—` |
| `name-canonicity-departure` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `22-31` | `tokens-and-verdicts` | `—` |
| `name-no-caller-hands-a-path` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `32-37` | `tokens-and-verdicts` | `—` |
| `name-classification-loses-data` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `38-47` | `tokens-and-verdicts` | `—` |
| `name-both-words-one-rule` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `48-56` | `tokens-and-verdicts` | `—` |
| `name-handle-is-this-grammar` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `57-70` | `tokens-and-verdicts` | `—` |
| `name-handle-terminal-substring` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `71-76` | `tokens-and-verdicts` | `—` |
| `name-imports` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `77-82` | `tokens-and-verdicts` | `—` |
| `name-brief-and-key-mark` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `83-91` | `tokens-and-verdicts` | `—` |
| `name-separator` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `92-104` | `tokens-and-verdicts` | `—` |
| `name-outcome` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `105-120` | `tokens-and-verdicts` | `—` |
| `name-outcome-infix-and-strip` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `121-148` | `tokens-and-verdicts` | `—` |
| `name-token-error` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `149-165` | `tokens-and-verdicts` | `—` |
| `name-token-error-traits` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `166-173` | `tokens-and-verdicts` | `—` |
| `name-refuse-token` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `174-220` | `tokens-and-verdicts` | `—` |
| `name-tests-the-charter` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1178-1185` | `classification-verdict-tests` | `—` |
| `classification-verdict-tests` | `the-tokens` | `source-task-name` | `composite` | `four-verdicts` | `1178-1199` | `source-task-name` | `name-tests-the-charter`, `name-tests-foreign` |
| `name-tests-foreign` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1186-1199` | `classification-verdict-tests` | `—` |
| `name-tests-kind-not-a-token` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1313-1358` | `shape-refusal-tests` | `—` |
| `shape-refusal-tests` | `the-tokens` | `source-task-name` | `composite` | `four-verdicts` | `1313-1521` | `source-task-name` | `name-tests-kind-not-a-token`, `name-tests-missing-separator`, `name-tests-one-reading`, `name-tests-node-wearing-outcome`, `name-tests-bad-slug`, `name-tests-species-mismatch` |
| `name-tests-missing-separator` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1359-1395` | `shape-refusal-tests` | `—` |
| `name-tests-one-reading` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1396-1457` | `shape-refusal-tests` | `—` |
| `name-tests-node-wearing-outcome` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1458-1481` | `shape-refusal-tests` | `—` |
| `name-tests-bad-slug` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1482-1497` | `shape-refusal-tests` | `—` |
| `name-tests-species-mismatch` | `the-tokens` | `source-task-name` | `literal` | `four-verdicts` | `1498-1521` | `shape-refusal-tests` | `—` |
| `source-task-tree` | `source-index` | `source-task-tree` | `root` | `—` | `1-2023` | `—` | `tree-opening`, `paths-and-addressing`, `walk-selection`, `kind-and-brief-chain`, `resolution`, `path-composition-tests`, `pick-tests`, `brief-chain-and-kind-tests`, `resolve-tests`, `pick-with-brief-chain-tests` |
| `source-task-grow` | `source-index` | `source-task-grow` | `root` | `—` | `1-518` | `—` | `growing-the-tree` |
| `source-tree-lifecycle` | `source-index` | `source-tree-lifecycle` | `root` | `—` | `1-2725` | `—` | `finish-transition`, `grove-beginning`, `decompose-production`, `outcomes-in-place`, `body-helpers`, `root-init-tests`, `finish-tests`, `decompose-tests`, `retire-and-prune-tests` |
| `source-verbs` | `source-index` | `source-verbs` | `root` | `—` | `1-363` | `—` | `the-twelve-verbs` |
| `source-driver` | `source-index` | `source-driver` | `root` | `—` | `1-57` | `—` | `driver-operations` |
| `source-complete` | `source-index` | `source-complete` | `root` | `—` | `1-96` | `—` | `complete-verb` |
| `source-driver-lease` | `source-index` | `source-driver-lease` | `root` | `—` | `1-1383` | `—` | `lease-and-epoch`, `lease-tests` |
| `source-session-config` | `source-index` | `source-session-config` | `root` | `—` | `1-358` | `—` | `whose-file` |
| `source-prompt` | `source-index` | `source-prompt` | `root` | `—` | `1-245` | `—` | `the-prompt-core` |
| `source-loop-driver` | `source-index` | `source-loop-driver` | `root` | `—` | `1-615` | `—` | `loop-driver` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Outcome`, `TokenError` | `01-orientation.md#the-cast` | `four-verdicts` | The terminal marks a name can carry — `DONE` and `ABANDONED` — and the refusal a token that is not well-formed produces. | `explained` |
| `Handle`, `HandleError`, `Kind`, `Parts`, `Slug` | `01-orientation.md#the-cast` | `the-handle-not-the-position` | The named parts of a task name: a kind token, a slug, and the `<slug>-k<key>` handle that is the entry's identity. `Parts` is the set of them a positioned name decomposes into. | `pending` |
| `TaskName` | `01-orientation.md#the-cast` | `canonical-or-nothing` | One parsed entry name, which renders back to the bytes it was parsed from or refuses to be computed at all. | `pending` |
| `Tree`, `Vacancy`, `task_tree::Guard`, `task_tree::write` | `01-orientation.md#the-cast` | `one-spelling-of-grove` | The tree read under the store's shared lock; the lock over a root that holds no tree; the store guard one mutation consumes; and the reopening a `TreeWrite` performs when it no longer holds one. | `pending` |
| `Selection` | `01-orientation.md#the-cast` | `first-live-leaf` | The leaf a session was launched to work: its path, its identity and its kind. | `pending` |
| `verbs::resolve`, `Resolution` | `01-orientation.md#the-cast` | `wider-than-a-key` | Resolution of one reference against the tree, whose `Ambiguous` case lists the keys of every entry a bare slug matched. | `pending` |
| `verbs::root_init` | `01-orientation.md#the-cast` | `never-mistaken-for-finished` | The verb that consumes a `Vacancy` and creates the whole grove — charter and first live leaf — as one store operation. | `pending` |
| `interpret`, `Disposition` | `01-orientation.md#the-cast` | `twelve-not-fourteen` | What the child side of the loop makes of a token written to the control channel: relaunch, or stop. | `pending` |
| `verbs`, `verbs::stale_cross_refs`, `verbs::signal_channel` | `01-orientation.md#the-cast` | `twelve-not-fourteen` | `verbs` declares fourteen public functions; twelve of them are the tree's verb surface, and `stale_cross_refs` and `signal_channel` each say in their own doc comment why they are not verbs. | `pending` |
| `admit_ambient_session`, `DriverLease`, `SessionEpochGuard` | `01-orientation.md#the-cast` | `one-per-working-tree` | The lease that keeps one live driver per working tree, the epoch that decides which calls it admits, and the check a session runs when there is no driver at all. | `pending` |
| `SessionConfig`, `TemplateSource` | `01-orientation.md#the-cast` | `whose-file-and-whether` | Whose configuration file a launch is expanded from, and whether a second one beside it is admissible. | `pending` |
| `compose`, `Mandate` | `01-orientation.md#the-cast` | `too-late-to-say-later` | The prompt a session is launched with, composed from the parts a skill cannot supply because by the time it could speak the moment has passed. | `pending` |
| `run`, `LoopOutcome` | `01-orientation.md#the-cast` | `four-things-a-runner-cannot-choose` | The loop itself, and how it ends: relaunched with fresh context, stopped resumably, or interrupted. | `pending` |
| `TaskName::distinguished` | `02-the-tokens.md#the-four-verdicts` | `canonical-or-nothing` | The associated function by which the domain advertises the name of a node's distinguished child. The classification test asserts that what `parse` makes of `BRIEF.md` and what the domain advertises are the same variant. | `pending` |
| `TaskName`, `TaskNameError`, `Verdict`, `verdict`, `entry`, `malformed` | `02-the-tokens.md#the-four-verdicts` | `canonical-or-nothing` | The parsed name, its refusal type, the classification a caller reads, and the three test helpers that reach a verdict: the only way to a verdict is through the `EntryName` implementation chapter 4 owns. | `pending` |
| `Parts::leaf` | `02-the-tokens.md#refusals-inside-the-shape` | `the-handle-not-the-position` | The constructor for the leaf half of `Parts`, taking an outcome, a session kind and a slug — the named parts a positioned leaf name decomposes into. | `pending` |
| `a_kind`, `slug` | `02-the-tokens.md#refusals-inside-the-shape` | `canonical-or-nothing` | Two test helpers defined beside the conformance kit: each takes a label, builds the token type it names, and panics if the label is not well-formed, so an invalid fixture is a test bug rather than a compile error. | `pending` |
| `impl Display for TaskName` | `02-the-tokens.md#refusals-inside-the-shape` | `canonical-or-nothing` | The renderer that writes a parsed name back to its filename bytes, both arms ending in the handle's own renderer. The round-trip test pins each of two names to its own bytes, so the reader needs the rendering direction here. | `pending` |
| `TaskName::compose` | `03-kind-slug-handle.md#the-handle-is-the-identity` | `canonical-or-nothing` | Composition builds a positioned name from a position, a kind, a slug and a key, so the handle's structural claim can be asserted over names built rather than parsed. | `pending` |
| `entry_path` | `05-opening.md#one-spelling-of-the-root` | `paths-are-built-here` | The one place an entry's absolute path is built, because the store returns no paths. Chapter 5 reproduces the module header that says so; chapter 6 reads the function. | `pending` |

<a id="owned-source-totals"></a>
## Owned source totals

Every line of the thirteen source roots is credited once, to the slice whose
page owns it; the table shows how the 10,533 lines divide across the twenty-one
chapters, and its total is what a completed book must account for. Nine roots are
owned whole by one chapter; the four that split — `src/task_name.rs` three ways,
`src/task_tree.rs` five, `src/tree_lifecycle.rs` four and `src/driver_lease.rs`
two — are why the ownership table above has thirty-nine rows rather than thirteen.

| Slice | Page | Owned lines |
|---|---|---:|
| `allowed-to-mean` | `01-orientation.md` | 436 |
| `four-verdicts` | `02-the-tokens.md` | 451 |
| `the-handle-not-the-position` | `03-kind-slug-handle.md` | 563 |
| `canonical-or-nothing` | `04-the-name.md` | 700 |
| `one-spelling-of-grove` | `05-opening.md` | 290 |
| `paths-are-built-here` | `06-paths.md` | 370 |
| `first-live-leaf` | `07-the-walk.md` | 322 |
| `root-to-leaf` | `08-kind-and-briefs.md` | 428 |
| `wider-than-a-key` | `09-resolve.md` | 613 |
| `what-the-library-cannot-see` | `10-growing.md` | 518 |
| `never-mistaken-for-finished` | `11-a-grove-begins.md` | 612 |
| `the-key-survives` | `12-leaf-to-node.md` | 775 |
| `marked-in-place` | `13-outcomes.md` | 808 |
| `the-tree-deletes-itself` | `14-finishing.md` | 530 |
| `twelve-not-fourteen` | `15-the-verbs.md` | 516 |
| `one-per-working-tree` | `16-the-lease.md` | 819 |
| `which-calls-are-admitted` | `17-the-epoch.md` | 564 |
| `whose-file-and-whether` | `18-which-files.md` | 358 |
| `too-late-to-say-later` | `19-the-core.md` | 245 |
| `four-things-a-runner-cannot-choose` | `20-the-loop.md` | 615 |
| `assembly` | `21-what-could-not-move.md` | 0 |
| **Total** | 13 source roots | **10,533** |
