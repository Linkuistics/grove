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
| `source-command-surface` | `crates/grove-llm/src/cli.rs` | 944 |

<!-- source-root «source-crate-manifest» source="crates/grove-llm/Cargo.toml" lines="1-54" -->
<!-- insert «manifest-thin-by-crate» -->
<!-- /source-root -->
<!-- source-root «source-library-root» source="crates/grove-llm/src/lib.rs" lines="1-16" -->
<!-- insert «library-root» -->
<!-- /source-root -->
<!-- source-root «source-entry-point» source="crates/grove-llm/src/main.rs" lines="1-3" -->
<!-- insert «entry-point» -->
<!-- /source-root -->
<!-- source-root «source-command-surface» source="crates/grove-llm/src/cli.rs" lines="1-944" -->
<!-- insert «surface-thesis-and-imports» -->
<!-- insert «grammar-cli-and-enum-head» -->
<!-- defer «verb-root-init» owner="before-the-lock" lines="66-74" -->
<!-- defer «verbs-reading» owner="information-not-error" lines="75-124" -->
<!-- defer «verbs-growing» owner="before-the-lock" lines="125-216" -->
<!-- defer «verbs-ending» owner="two-steps-remain" lines="217-247" -->
<!-- defer «verbs-leaving» owner="admit-before-signal" lines="248-289" -->
<!-- insert «enum-close-and-operation-label» -->
<!-- defer «args-complete» owner="admit-before-signal" lines="311-322" -->
<!-- defer «args-root-init» owner="before-the-lock" lines="323-330" -->
<!-- defer «kind-help-and-parse-kind» owner="before-the-lock" lines="331-358" -->
<!-- defer «args-growing» owner="before-the-lock" lines="359-399" -->
<!-- defer «args-ending» owner="two-steps-remain" lines="400-411" -->
<!-- insert «run-admission-and-dispatch» -->
<!-- defer «handlers-leaving» owner="admit-before-signal" lines="438-483" -->
<!-- defer «handler-root-init» owner="before-the-lock" lines="484-513" -->
<!-- defer «handlers-reading-and-rendering» owner="information-not-error" lines="514-636" -->
<!-- defer «handlers-growing» owner="before-the-lock" lines="637-767" -->
<!-- defer «handlers-ending» owner="two-steps-remain" lines="768-825" -->
<!-- defer «presence-rule-and-slug» owner="before-the-lock" lines="826-862" -->
<!-- insert «openings» -->
<!-- defer «path-and-label-helpers» owner="information-not-error" lines="904-944" -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-thin-by-crate` | `source-crate-manifest` | `one-call-plus-rendering` | `1-54` | 54 | `resolved` |
| `library-root` | `source-library-root` | `one-call-plus-rendering` | `1-16` | 16 | `resolved` |
| `entry-point` | `source-entry-point` | `one-call-plus-rendering` | `1-3` | 3 | `resolved` |
| `surface-thesis-and-imports` | `source-command-surface` | `one-call-plus-rendering` | `1-34` | 34 | `resolved` |
| `grammar-cli-and-enum-head` | `source-command-surface` | `admitted-before-dispatch` | `35-65` | 31 | `resolved` |
| `verb-root-init` | `source-command-surface` | `before-the-lock` | `66-74` | 9 | `deferred` |
| `verbs-reading` | `source-command-surface` | `information-not-error` | `75-124` | 50 | `deferred` |
| `verbs-growing` | `source-command-surface` | `before-the-lock` | `125-216` | 92 | `deferred` |
| `verbs-ending` | `source-command-surface` | `two-steps-remain` | `217-247` | 31 | `deferred` |
| `verbs-leaving` | `source-command-surface` | `admit-before-signal` | `248-289` | 42 | `deferred` |
| `enum-close-and-operation-label` | `source-command-surface` | `admitted-before-dispatch` | `290-310` | 21 | `resolved` |
| `args-complete` | `source-command-surface` | `admit-before-signal` | `311-322` | 12 | `deferred` |
| `args-root-init` | `source-command-surface` | `before-the-lock` | `323-330` | 8 | `deferred` |
| `kind-help-and-parse-kind` | `source-command-surface` | `before-the-lock` | `331-358` | 28 | `deferred` |
| `args-growing` | `source-command-surface` | `before-the-lock` | `359-399` | 41 | `deferred` |
| `args-ending` | `source-command-surface` | `two-steps-remain` | `400-411` | 12 | `deferred` |
| `run-admission-and-dispatch` | `source-command-surface` | `admitted-before-dispatch` | `412-437` | 26 | `resolved` |
| `handlers-leaving` | `source-command-surface` | `admit-before-signal` | `438-483` | 46 | `deferred` |
| `handler-root-init` | `source-command-surface` | `before-the-lock` | `484-513` | 30 | `deferred` |
| `handlers-reading-and-rendering` | `source-command-surface` | `information-not-error` | `514-636` | 123 | `deferred` |
| `handlers-growing` | `source-command-surface` | `before-the-lock` | `637-767` | 131 | `deferred` |
| `handlers-ending` | `source-command-surface` | `two-steps-remain` | `768-825` | 58 | `deferred` |
| `presence-rule-and-slug` | `source-command-surface` | `before-the-lock` | `826-862` | 37 | `deferred` |
| `openings` | `source-command-surface` | `admitted-before-dispatch` | `863-903` | 41 | `resolved` |
| `path-and-label-helpers` | `source-command-surface` | `information-not-error` | `904-944` | 41 | `deferred` |

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
| `source-command-surface` | `source-index` | `source-command-surface` | `root` | `—` | `1-944` | `—` | `surface-thesis-and-imports`, `grammar-cli-and-enum-head`, `verb-root-init`, `verbs-reading`, `verbs-growing`, `verbs-ending`, `verbs-leaving`, `enum-close-and-operation-label`, `args-complete`, `args-root-init`, `kind-help-and-parse-kind`, `args-growing`, `args-ending`, `run-admission-and-dispatch`, `handlers-leaving`, `handler-root-init`, `handlers-reading-and-rendering`, `handlers-growing`, `handlers-ending`, `presence-rule-and-slug`, `openings`, `path-and-label-helpers` |
| `surface-header-audience` | `orientation` | `source-command-surface` | `literal` | `one-call-plus-rendering` | `1-9` | `surface-thesis-and-imports` | `—` |
| `surface-thesis-and-imports` | `orientation` | `source-command-surface` | `composite` | `one-call-plus-rendering` | `1-34` | `source-command-surface` | `surface-header-audience`, `surface-header-thin-and-order`, `surface-imports` |
| `surface-header-thin-and-order` | `orientation` | `source-command-surface` | `literal` | `one-call-plus-rendering` | `10-23` | `surface-thesis-and-imports` | `—` |
| `surface-imports` | `orientation` | `source-command-surface` | `literal` | `one-call-plus-rendering` | `24-34` | `surface-thesis-and-imports` | `—` |
| `grammar-command-attributes` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `35-50` | `grammar-cli-and-enum-head` | `—` |
| `grammar-cli-and-enum-head` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `35-65` | `source-command-surface` | `grammar-command-attributes`, `grammar-cli-struct`, `grammar-command-enum-head` |
| `grammar-cli-struct` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `51-62` | `grammar-cli-and-enum-head` | `—` |
| `grammar-command-enum-head` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `63-65` | `grammar-cli-and-enum-head` | `—` |
| `grammar-command-enum-close` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `290-291` | `enum-close-and-operation-label` | `—` |
| `enum-close-and-operation-label` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `290-310` | `source-command-surface` | `grammar-command-enum-close`, `grammar-operation-label` |
| `grammar-operation-label` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `292-310` | `enum-close-and-operation-label` | `—` |
| `run-parse-and-bare-branch` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `412-419` | `run-admission-and-dispatch` | `—` |
| `run-admission-and-dispatch` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `412-437` | `source-command-surface` | `run-parse-and-bare-branch`, `run-cwd-and-admission`, `run-dispatch` |
| `run-cwd-and-admission` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `420-421` | `run-admission-and-dispatch` | `—` |
| `run-dispatch` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `422-437` | `run-admission-and-dispatch` | `—` |
| `openings-worktree` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `863-870` | `openings` | `—` |
| `openings` | `the-grammar` | `source-command-surface` | `composite` | `admitted-before-dispatch` | `863-903` | `source-command-surface` | `openings-worktree`, `openings-readable`, `openings-writable`, `openings-absent` |
| `openings-readable` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `871-884` | `openings` | `—` |
| `openings-writable` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `885-893` | `openings` | `—` |
| `openings-absent` | `the-grammar` | `source-command-surface` | `literal` | `admitted-before-dispatch` | `894-903` | `openings` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Reading`, `Tree`, `Writing`, `TreeWrite` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | The two openings of a grove — a shared read and an exclusive write — each answering *vacant* as a value rather than an error. | `explained` |
| `SessionEpochGuard` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | The guard `run` obtains when it admits this process against a live session epoch — present under a driver, absent for a manual command — alive through the verb, and consulted only by `complete`. | `explained` |
| `Workspace` | `01-orientation.md#the-imports` | `admitted-before-dispatch` | A resolved jj working tree; every verb but `complete` resolves it from the current directory, and the grove root is spelled from it in one place. | `explained` |
| `Outcome` | `01-orientation.md#the-imports` | `information-not-error` | Live, retired or abandoned — the infix a filename carries, rendered as a stderr note so a dead end never looks live. | `pending` |
| `Reference` | `01-orientation.md#the-imports` | `information-not-error` | A parsed spelling of a tree entry — key, handle or slug — read by its own type before any tree is opened. | `pending` |
| `Sought`, `Resolution` | `01-orientation.md#the-imports` | `information-not-error` | `Sought` is a found-or-nothing answer; `Resolution` is what `resolve` found — the root, one entry, or an ambiguity. | `pending` |
| `Kind`, `Slug` | `01-orientation.md#the-imports` | `before-the-lock` | The grammar's own types for a `--kind` token and a slug; malformed text is refused by them, before any lock. | `pending` |
| `SessionConfig` | `01-orientation.md#the-imports` | `before-the-lock` | The launch configuration, loaded whole and asked whether one kind resolves to a template. | `pending` |
| `Handle` | `01-orientation.md#the-imports` | `admit-before-signal` | A `<slug>-k<key>` handle, parsed leniently on the key and spoken canonically thereafter. | `pending` |
| `Signalled` | `01-orientation.md#the-imports` | `admit-before-signal` | Whether `complete` wrote the disposition to a channel or found no loop to signal. | `pending` |
| `cmd_pick`, `cmd_brief_chain`, `cmd_kind`, `cmd_resolve` | `02-the-grammar.md#worked-dispatch` | `information-not-error` | One handler per reading verb: the shared opening, one `grove_loop::verbs` call, and rendering. | `pending` |
| `cmd_root_init`, `cmd_leaf_add`, `cmd_leaf_insert`, `cmd_leaf_decompose` | `02-the-grammar.md#worked-dispatch` | `before-the-lock` | One handler per growing verb: text parsed, presence asked, then the exclusive opening, then one call. | `pending` |
| `cmd_leaf_retire`, `cmd_leaf_prune` | `02-the-grammar.md#worked-dispatch` | `two-steps-remain` | One handler per terminal mark: the exclusive opening, one call, the marked paths, and the two remaining steps on stderr. | `pending` |
| `cmd_finish_commit`, `cmd_complete` | `02-the-grammar.md#worked-dispatch` | `admit-before-signal` | The two handlers that open no tree: one commits through the workspace, one writes the completion channel. | `pending` |

<a id="owned-source-totals"></a>
## Owned source totals

Every line of the four source roots is credited once, to the slice whose page
owns it; the table shows how the 1,017 lines divide across the seven chapters,
and its total is what a completed book must account for.

| Slice | Page | Owned lines |
|---|---|---:|
| `one-call-plus-rendering` | `01-orientation.md` | 107 |
| `admitted-before-dispatch` | `02-the-grammar.md` | 119 |
| `information-not-error` | `03-reading-the-tree.md` | 214 |
| `before-the-lock` | `04-growing-the-tree.md` | 376 |
| `two-steps-remain` | `05-ending-work.md` | 101 |
| `admit-before-signal` | `06-leaving-the-loop.md` | 100 |
| `assembly` | `07-what-order-holds.md` | 0 |
| **Total** | 4 source roots | **1,017** |
