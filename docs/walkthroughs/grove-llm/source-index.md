# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/grove-llm/Cargo.toml` | 54 |
| `source-library-root` | `crates/grove-llm/src/lib.rs` | 16 |
| `source-entry-point` | `crates/grove-llm/src/main.rs` | 3 |
| `source-command-surface` | `crates/grove-llm/src/cli.rs` | 898 |

<!-- source-root «source-crate-manifest» source="crates/grove-llm/Cargo.toml" lines="1-54" -->
<!-- insert «manifest-thin-by-crate» -->
<!-- /source-root -->
<!-- source-root «source-library-root» source="crates/grove-llm/src/lib.rs" lines="1-16" -->
<!-- insert «library-root» -->
<!-- /source-root -->
<!-- source-root «source-entry-point» source="crates/grove-llm/src/main.rs" lines="1-3" -->
<!-- insert «entry-point» -->
<!-- /source-root -->
<!-- source-root «source-command-surface» source="crates/grove-llm/src/cli.rs" lines="1-898" -->
<!-- insert «surface-thesis-and-imports» -->
<!-- insert «grammar-cli-and-enum-head» -->
<!-- insert «verb-root-init» -->
<!-- insert «verbs-reading» -->
<!-- insert «verbs-growing» -->
<!-- insert «verbs-ending» -->
<!-- insert «verbs-leaving» -->
<!-- insert «enum-close-and-operation-label» -->
<!-- insert «args-complete» -->
<!-- insert «args-root-init» -->
<!-- insert «kind-help-and-parse-kind» -->
<!-- insert «args-growing» -->
<!-- insert «args-ending» -->
<!-- insert «run-admission-and-dispatch» -->
<!-- insert «handlers-leaving» -->
<!-- insert «handler-root-init» -->
<!-- insert «handlers-reading-and-rendering» -->
<!-- insert «handlers-growing» -->
<!-- insert «handlers-ending» -->
<!-- insert «slug-argument» -->
<!-- insert «openings» -->
<!-- insert «path-and-label-helpers» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-crate` | `source-crate-manifest` | `one-call-plus-rendering` | `1-54` | 54 | `resolved` |
| `library-root` | `source-library-root` | `one-call-plus-rendering` | `1-16` | 16 | `resolved` |
| `entry-point` | `source-entry-point` | `one-call-plus-rendering` | `1-3` | 3 | `resolved` |
| `surface-thesis-and-imports` | `source-command-surface` | `one-call-plus-rendering` | `1-35` | 35 | `resolved` |
| `grammar-cli-and-enum-head` | `source-command-surface` | `admitted-before-dispatch` | `36-66` | 31 | `resolved` |
| `verb-root-init` | `source-command-surface` | `before-the-lock` | `67-75` | 9 | `resolved` |
| `verbs-reading` | `source-command-surface` | `information-not-error` | `76-125` | 50 | `resolved` |
| `verbs-growing` | `source-command-surface` | `before-the-lock` | `126-217` | 92 | `resolved` |
| `verbs-ending` | `source-command-surface` | `two-steps-remain` | `218-248` | 31 | `resolved` |
| `verbs-leaving` | `source-command-surface` | `admit-before-signal` | `249-290` | 42 | `resolved` |
| `enum-close-and-operation-label` | `source-command-surface` | `admitted-before-dispatch` | `291-311` | 21 | `resolved` |
| `args-complete` | `source-command-surface` | `admit-before-signal` | `312-323` | 12 | `resolved` |
| `args-root-init` | `source-command-surface` | `before-the-lock` | `324-331` | 8 | `resolved` |
| `kind-help-and-parse-kind` | `source-command-surface` | `before-the-lock` | `332-360` | 29 | `resolved` |
| `args-growing` | `source-command-surface` | `before-the-lock` | `361-401` | 41 | `resolved` |
| `args-ending` | `source-command-surface` | `two-steps-remain` | `402-413` | 12 | `resolved` |
| `run-admission-and-dispatch` | `source-command-surface` | `admitted-before-dispatch` | `414-461` | 48 | `resolved` |
| `handlers-leaving` | `source-command-surface` | `admit-before-signal` | `462-504` | 43 | `resolved` |
| `handler-root-init` | `source-command-surface` | `before-the-lock` | `505-532` | 28 | `resolved` |
| `handlers-reading-and-rendering` | `source-command-surface` | `information-not-error` | `533-656` | 124 | `resolved` |
| `handlers-growing` | `source-command-surface` | `before-the-lock` | `657-750` | 94 | `resolved` |
| `handlers-ending` | `source-command-surface` | `two-steps-remain` | `751-808` | 58 | `resolved` |
| `slug-argument` | `source-command-surface` | `before-the-lock` | `809-816` | 8 | `resolved` |
| `openings` | `source-command-surface` | `admitted-before-dispatch` | `817-857` | 41 | `resolved` |
| `path-and-label-helpers` | `source-command-surface` | `information-not-error` | `858-898` | 41 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-54` | `—` | `manifest-thin-by-crate` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `one-call-plus-rendering` | `1-8` | `manifest-thin-by-crate` | `—` |
| `manifest-thin-by-crate` | `orientation` | `source-crate-manifest` | `composite` | `one-call-plus-rendering` | `1-54` | `source-crate-manifest` | `manifest-package-identity`, `manifest-crate-not-a-target`, `manifest-grove-dependency-removed`, `manifest-bin-target`, `manifest-dependencies`, `manifest-dev-dependencies`, `manifest-lints`, `manifest-release` |
| `manifest-crate-not-a-target` | `orientation` | `source-crate-manifest` | `literal` | `one-call-plus-rendering` | `9-15` | `manifest-thin-by-crate` | `—` |
| `manifest-grove-dependency-removed` | `orientation` | `source-crate-manifest` | `literal` | `one-call-plus-rendering` | `16-22` | `manifest-thin-by-crate` | `—` |
| `manifest-bin-target` | `orientation` | `source-crate-manifest` | `literal` | `one-call-plus-rendering` | `23-25` | `manifest-thin-by-crate` | `—` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `one-call-plus-rendering` | `26-31` | `manifest-thin-by-crate` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `one-call-plus-rendering` | `32-44` | `manifest-thin-by-crate` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `one-call-plus-rendering` | `45-47` | `manifest-thin-by-crate` | `—` |
| `manifest-release` | `orientation` | `source-crate-manifest` | `literal` | `one-call-plus-rendering` | `48-54` | `manifest-thin-by-crate` | `—` |
| `source-library-root` | `source-index` | `source-library-root` | `root` | `—` | `1-16` | `—` | `library-root` |
| `library-root-thin` | `orientation` | `source-library-root` | `literal` | `one-call-plus-rendering` | `1-8` | `library-root` | `—` |
| `library-root` | `orientation` | `source-library-root` | `composite` | `one-call-plus-rendering` | `1-16` | `source-library-root` | `library-root-thin`, `library-root-target-and-module` |
| `library-root-target-and-module` | `orientation` | `source-library-root` | `literal` | `one-call-plus-rendering` | `9-16` | `library-root` | `—` |
| `source-entry-point` | `source-index` | `source-entry-point` | `root` | `—` | `1-3` | `—` | `entry-point` |
| `entry-point` | `orientation` | `source-entry-point` | `literal` | `one-call-plus-rendering` | `1-3` | `source-entry-point` | `—` |
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-898` | `—` | `surface-thesis-and-imports`, `grammar-cli-and-enum-head`, `verb-root-init`, `verbs-reading`, `verbs-growing`, `verbs-ending`, `verbs-leaving`, `enum-close-and-operation-label`, `args-complete`, `args-root-init`, `kind-help-and-parse-kind`, `args-growing`, `args-ending`, `run-admission-and-dispatch`, `handlers-leaving`, `handler-root-init`, `handlers-reading-and-rendering`, `handlers-growing`, `handlers-ending`, `slug-argument`, `openings`, `path-and-label-helpers` |
| `surface-header-audience` | `orientation` | `source-command-surface` | `literal` | `one-call-plus-rendering` | `1-9` | `surface-thesis-and-imports` | `—` |
| `surface-thesis-and-imports` | `orientation` | `source-command-surface` | `composite` | `one-call-plus-rendering` | `1-35` | `source-command-surface` | `surface-header-audience`, `surface-header-thin-and-order`, `surface-imports` |
| `surface-header-thin-and-order` | `orientation` | `source-command-surface` | `literal` | `one-call-plus-rendering` | `10-24` | `surface-thesis-and-imports` | `—` |
| `surface-imports` | `orientation` | `source-command-surface` | `literal` | `one-call-plus-rendering` | `25-35` | `surface-thesis-and-imports` | `—` |
| `grammar-command-attributes` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `36-51` | `grammar-cli-and-enum-head` | `—` |
| `grammar-cli-and-enum-head` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `36-66` | `source-command-surface` | `grammar-command-attributes`, `grammar-cli-struct`, `grammar-command-enum-head` |
| `grammar-cli-struct` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `52-63` | `grammar-cli-and-enum-head` | `—` |
| `grammar-command-enum-head` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `64-66` | `grammar-cli-and-enum-head` | `—` |
| `verb-root-init` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `67-75` | `source-command-surface` | `—` |
| `verbs-pick-help` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `76-83` | `verbs-reading` | `—` |
| `verbs-reading` | `reading-the-tree` | `source-command-surface` | `composite` | `information-not-error` | `76-125` | `source-command-surface` | `verbs-pick-help`, `verbs-brief-chain-help`, `verbs-kind-help`, `verbs-resolve-help` |
| `verbs-brief-chain-help` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `84-93` | `verbs-reading` | `—` |
| `verbs-kind-help` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `94-108` | `verbs-reading` | `—` |
| `verbs-resolve-help` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `109-125` | `verbs-reading` | `—` |
| `verbs-leaf-add-help` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `126-177` | `verbs-growing` | `—` |
| `verbs-growing` | `growing-the-tree` | `source-command-surface` | `composite` | `before-the-lock` | `126-217` | `source-command-surface` | `verbs-leaf-add-help`, `verbs-leaf-insert-help`, `verbs-leaf-decompose-help` |
| `verbs-leaf-insert-help` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `178-205` | `verbs-growing` | `—` |
| `verbs-leaf-decompose-help` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `206-217` | `verbs-growing` | `—` |
| `verbs-leaf-retire-help` | `ending-work` | `source-command-surface` | `literal` | `two-steps-remain` | `218-227` | `verbs-ending` | `—` |
| `verbs-ending` | `ending-work` | `source-command-surface` | `composite` | `two-steps-remain` | `218-248` | `source-command-surface` | `verbs-leaf-retire-help`, `verbs-leaf-prune-help` |
| `verbs-leaf-prune-help` | `ending-work` | `source-command-surface` | `literal` | `two-steps-remain` | `228-248` | `verbs-ending` | `—` |
| `verbs-finish-commit-help` | `leaving-the-loop` | `source-command-surface` | `literal` | `admit-before-signal` | `249-272` | `verbs-leaving` | `—` |
| `verbs-leaving` | `leaving-the-loop` | `source-command-surface` | `composite` | `admit-before-signal` | `249-290` | `source-command-surface` | `verbs-finish-commit-help`, `verbs-complete-help` |
| `verbs-complete-help` | `leaving-the-loop` | `source-command-surface` | `literal` | `admit-before-signal` | `273-290` | `verbs-leaving` | `—` |
| `grammar-command-enum-close` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `291-292` | `enum-close-and-operation-label` | `—` |
| `enum-close-and-operation-label` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `291-311` | `source-command-surface` | `grammar-command-enum-close`, `grammar-operation-label` |
| `grammar-operation-label` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `293-311` | `enum-close-and-operation-label` | `—` |
| `args-complete` | `leaving-the-loop` | `source-command-surface` | `literal` | `admit-before-signal` | `312-323` | `source-command-surface` | `—` |
| `args-root-init` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `324-331` | `source-command-surface` | `—` |
| `kind-help` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `332-346` | `kind-help-and-parse-kind` | `—` |
| `kind-help-and-parse-kind` | `growing-the-tree` | `source-command-surface` | `composite` | `before-the-lock` | `332-360` | `source-command-surface` | `kind-help`, `kind-override-help`, `parse-kind` |
| `kind-override-help` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `347-351` | `kind-help-and-parse-kind` | `—` |
| `parse-kind` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `352-360` | `kind-help-and-parse-kind` | `—` |
| `args-leaf-add` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `361-378` | `args-growing` | `—` |
| `args-growing` | `growing-the-tree` | `source-command-surface` | `composite` | `before-the-lock` | `361-401` | `source-command-surface` | `args-leaf-add`, `args-leaf-insert`, `args-leaf-decompose` |
| `args-leaf-insert` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `379-390` | `args-growing` | `—` |
| `args-leaf-decompose` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `391-401` | `args-growing` | `—` |
| `args-leaf-retire` | `ending-work` | `source-command-surface` | `literal` | `two-steps-remain` | `402-407` | `args-ending` | `—` |
| `args-ending` | `ending-work` | `source-command-surface` | `composite` | `two-steps-remain` | `402-413` | `source-command-surface` | `args-leaf-retire`, `args-leaf-prune` |
| `args-leaf-prune` | `ending-work` | `source-command-surface` | `literal` | `two-steps-remain` | `408-413` | `args-ending` | `—` |
| `run-parse-cli` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `414-421` | `run-parse-and-bare-branch` | `—` |
| `run-parse-and-bare-branch` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `414-443` | `run-admission-and-dispatch` | `run-parse-cli`, `run-standalone-completion` |
| `run-admission-and-dispatch` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `414-461` | `source-command-surface` | `run-parse-and-bare-branch`, `run-cwd-and-admission`, `run-dispatch` |
| `run-standalone-completion` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `422-443` | `run-parse-and-bare-branch` | `—` |
| `run-cwd-and-admission` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `444-445` | `run-admission-and-dispatch` | `—` |
| `run-dispatch` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `446-461` | `run-admission-and-dispatch` | `—` |
| `handler-finish-commit` | `leaving-the-loop` | `source-command-surface` | `literal` | `admit-before-signal` | `462-479` | `handlers-leaving` | `—` |
| `handlers-leaving` | `leaving-the-loop` | `source-command-surface` | `composite` | `admit-before-signal` | `462-504` | `source-command-surface` | `handler-finish-commit`, `handler-complete-admit`, `handler-complete-endings` |
| `handler-complete-admit` | `leaving-the-loop` | `source-command-surface` | `literal` | `admit-before-signal` | `480-487` | `handlers-leaving` | `—` |
| `handler-complete-endings` | `leaving-the-loop` | `source-command-surface` | `literal` | `admit-before-signal` | `488-504` | `handlers-leaving` | `—` |
| `handler-root-init-text` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `505-510` | `handler-root-init` | `—` |
| `handler-root-init` | `growing-the-tree` | `source-command-surface` | `composite` | `before-the-lock` | `505-532` | `source-command-surface` | `handler-root-init-text`, `handler-root-init-vacancy` |
| `handler-root-init-vacancy` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `511-532` | `handler-root-init` | `—` |
| `handler-pick` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `533-542` | `handlers-reading-and-rendering` | `—` |
| `handlers-reading-and-rendering` | `reading-the-tree` | `source-command-surface` | `composite` | `information-not-error` | `533-656` | `source-command-surface` | `handler-pick`, `handler-brief-chain`, `handler-kind`, `handler-leaf-in`, `handler-resolve`, `render-resolution-head`, `render-resolution-entry`, `render-resolution-absent` |
| `handler-brief-chain` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `543-555` | `handlers-reading-and-rendering` | `—` |
| `handler-kind` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `556-569` | `handlers-reading-and-rendering` | `—` |
| `handler-leaf-in` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `570-580` | `handlers-reading-and-rendering` | `—` |
| `handler-resolve` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `581-599` | `handlers-reading-and-rendering` | `—` |
| `render-resolution-head` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `600-613` | `handlers-reading-and-rendering` | `—` |
| `render-resolution-entry` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `614-630` | `handlers-reading-and-rendering` | `—` |
| `render-resolution-absent` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `631-656` | `handlers-reading-and-rendering` | `—` |
| `handler-leaf-add` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `657-670` | `handlers-growing` | `—` |
| `handlers-growing` | `growing-the-tree` | `source-command-surface` | `composite` | `before-the-lock` | `657-750` | `source-command-surface` | `handler-leaf-add`, `print-paths`, `handler-leaf-insert`, `report-insert`, `handler-leaf-decompose-head`, `handler-leaf-decompose-write` |
| `print-paths` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `671-680` | `handlers-growing` | `—` |
| `handler-leaf-insert` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `681-690` | `handlers-growing` | `—` |
| `report-insert` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `691-735` | `handlers-growing` | `—` |
| `handler-leaf-decompose-head` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `736-742` | `handlers-growing` | `—` |
| `handler-leaf-decompose-write` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `743-750` | `handlers-growing` | `—` |
| `eprint-next-steps` | `ending-work` | `source-command-surface` | `literal` | `two-steps-remain` | `751-768` | `handlers-ending` | `—` |
| `handlers-ending` | `ending-work` | `source-command-surface` | `composite` | `two-steps-remain` | `751-808` | `source-command-surface` | `eprint-next-steps`, `handler-leaf-retire`, `handler-leaf-prune-marks`, `handler-leaf-prune-reminder` |
| `handler-leaf-retire` | `ending-work` | `source-command-surface` | `literal` | `two-steps-remain` | `769-778` | `handlers-ending` | `—` |
| `handler-leaf-prune-marks` | `ending-work` | `source-command-surface` | `literal` | `two-steps-remain` | `779-799` | `handlers-ending` | `—` |
| `handler-leaf-prune-reminder` | `ending-work` | `source-command-surface` | `literal` | `two-steps-remain` | `800-808` | `handlers-ending` | `—` |
| `slug` | `growing-the-tree` | `source-command-surface` | `literal` | `before-the-lock` | `809-816` | `slug-argument` | `—` |
| `slug-argument` | `growing-the-tree` | `source-command-surface` | `composite` | `before-the-lock` | `809-816` | `source-command-surface` | `slug` |
| `openings-worktree` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `817-824` | `openings` | `—` |
| `openings` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `817-857` | `source-command-surface` | `openings-worktree`, `openings-readable`, `openings-writable`, `openings-absent` |
| `openings-readable` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `825-838` | `openings` | `—` |
| `openings-writable` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `839-847` | `openings` | `—` |
| `openings-absent` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `848-857` | `openings` | `—` |
| `helper-normalize-leaf-path` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `858-881` | `path-and-label-helpers` | `—` |
| `path-and-label-helpers` | `reading-the-tree` | `source-command-surface` | `composite` | `information-not-error` | `858-898` | `source-command-surface` | `helper-normalize-leaf-path`, `helper-label`, `helper-no-live-leaves` |
| `helper-label` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `882-890` | `path-and-label-helpers` | `—` |
| `helper-no-live-leaves` | `reading-the-tree` | `source-command-surface` | `literal` | `information-not-error` | `891-898` | `path-and-label-helpers` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Reading`, `Tree`, `Writing`, `TreeWrite` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | The two openings of a grove — a shared read and an exclusive write — each answering *vacant* as a value rather than an error. | `explained` |
| `SessionEpochGuard` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | The guard `run` obtains when it admits this process against a live session epoch — present under a driver, absent for a manual command — alive through the verb, and consulted only by `complete`. | `explained` |
| `Workspace` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | A resolved jj working tree; every verb but `complete` resolves it from the current directory, and the grove root is spelled from it in one place. | `explained` |
| `Outcome` | `01-orientation.md#the-imports` | `information-not-error` | Live, retired or abandoned — the infix a filename carries, rendered as a stderr note so a dead end never looks live. | `explained` |
| `Reference` | `01-orientation.md#the-imports` | `information-not-error` | A parsed spelling of a tree entry — key, handle or slug — read by its own type before any tree is opened. | `explained` |
| `Sought`, `Resolution` | `01-orientation.md#the-imports` | `information-not-error` | `Sought` is a found-or-nothing answer; `Resolution` is what `resolve` found — the root, one entry, or an ambiguity. | `explained` |
| `Kind`, `Slug` | `01-orientation.md#the-imports` | `before-the-lock` | The grammar's own types for a `--kind` token and a slug; malformed text is refused by them, before any lock. | `explained` |
| `Handle` | `01-orientation.md#the-imports` | `admit-before-signal` | A `<slug>-k<key>` handle, parsed with a canonical positive key. | `explained` |
| `Signalled` | `01-orientation.md#the-imports` | `admit-before-signal` | Whether `complete` wrote the disposition to a channel or found no loop to signal. | `explained` |
| `cmd_pick`, `cmd_brief_chain`, `cmd_kind`, `cmd_resolve` | `02-the-grammar.md#worked-dispatch` | `information-not-error` | One handler per reading verb: the shared opening, one `grove_loop::verbs` call, and rendering. | `explained` |
| `cmd_root_init`, `cmd_leaf_add`, `cmd_leaf_insert`, `cmd_leaf_decompose` | `02-the-grammar.md#worked-dispatch` | `before-the-lock` | One handler per growing verb: text parsed, then the exclusive opening, then one call. | `explained` |
| `cmd_leaf_retire`, `cmd_leaf_prune` | `02-the-grammar.md#worked-dispatch` | `two-steps-remain` | One handler per terminal mark: the exclusive opening, one call, the marked paths, and the two remaining steps on stderr. | `explained` |
| `cmd_finish_commit`, `cmd_complete` | `02-the-grammar.md#worked-dispatch` | `admit-before-signal` | The two handlers that open no tree: one commits through the workspace, one writes the completion channel. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every line of the four source roots is credited once, to the slice whose page
owns it; the table shows how the 971 lines divide across the seven chapters,
and its total is what a completed book must account for.

| Slice | Page | Owned lines |
|---|---|---:|
| `one-call-plus-rendering` | `01-orientation.md` | 108 |
| `admitted-before-dispatch` | `02-the-grammar.md` | 141 |
| `information-not-error` | `03-reading-the-tree.md` | 215 |
| `before-the-lock` | `04-growing-the-tree.md` | 309 |
| `two-steps-remain` | `05-ending-work.md` | 101 |
| `admit-before-signal` | `06-leaving-the-loop.md` | 97 |
| `assembly` | `07-what-order-holds.md` | 0 |
| **Total** | 4 source roots | **971** |
