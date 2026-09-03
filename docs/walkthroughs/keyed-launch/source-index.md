# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/keyed-launch/Cargo.toml` | 47 |
| `source-library-root` | `crates/keyed-launch/src/lib.rs` | 68 |
| `source-error-types` | `crates/keyed-launch/src/error.rs` | 81 |
| `source-vocabulary` | `crates/keyed-launch/src/vocabulary.rs` | 44 |
| `source-templates` | `crates/keyed-launch/src/templates.rs` | 670 |
| `source-argv` | `crates/keyed-launch/src/argv.rs` | 48 |
| `source-channel` | `crates/keyed-launch/src/channel.rs` | 404 |
| `source-run` | `crates/keyed-launch/src/run.rs` | 607 |
| `source-conformance` | `crates/keyed-launch/src/conformance.rs` | 104 |

<!-- source-root «source-crate-manifest» source="crates/keyed-launch/Cargo.toml" lines="1-47" -->
<!-- insert «manifest-three-dependencies» -->
<!-- /source-root -->
<!-- source-root «source-library-root» source="crates/keyed-launch/src/lib.rs" lines="1-68" -->
<!-- insert «library-root» -->
<!-- /source-root -->
<!-- source-root «source-error-types» source="crates/keyed-launch/src/error.rs" lines="1-81" -->
<!-- insert «two-opaque-errors» -->
<!-- /source-root -->
<!-- source-root «source-vocabulary» source="crates/keyed-launch/src/vocabulary.rs" lines="1-44" -->
<!-- insert «vocabulary» -->
<!-- /source-root -->
<!-- source-root «source-templates» source="crates/keyed-launch/src/templates.rs" lines="1-670" -->
<!-- insert «template-shapes» -->
<!-- insert «templates-load» -->
<!-- insert «resolution-and-expansion» -->
<!-- insert «reading-and-whole-document-validation» -->
<!-- insert «node-and-template-rules» -->
<!-- insert «word-scanning» -->
<!-- insert «diagnostics» -->
<!-- insert «templates-keys» -->
<!-- /source-root -->
<!-- source-root «source-argv» source="crates/keyed-launch/src/argv.rs" lines="1-48" -->
<!-- insert «argv» -->
<!-- /source-root -->
<!-- source-root «source-channel» source="crates/keyed-launch/src/channel.rs" lines="1-404" -->
<!-- insert «channel-production» -->
<!-- insert «channel-inline-tests» -->
<!-- /source-root -->
<!-- source-root «source-run» source="crates/keyed-launch/src/run.rs" lines="1-607" -->
<!-- insert «launch-shape» -->
<!-- insert «watch-and-launcher-signals» -->
<!-- insert «terminal-and-spawn» -->
<!-- insert «supervise-and-escalate» -->
<!-- /source-root -->
<!-- source-root «source-conformance» source="crates/keyed-launch/src/conformance.rs" lines="1-104" -->
<!-- insert «conformance» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-three-dependencies` | `source-crate-manifest` | `understands-neither` | `1-47` | 47 | `resolved` |
| `library-root` | `source-library-root` | `understands-neither` | `1-68` | 68 | `resolved` |
| `two-opaque-errors` | `source-error-types` | `understands-neither` | `1-81` | 81 | `resolved` |
| `vocabulary` | `source-vocabulary` | `rules-about-names` | `1-44` | 44 | `resolved` |
| `template-shapes` | `source-templates` | `rules-about-names` | `1-91` | 91 | `resolved` |
| `templates-load` | `source-templates` | `never-assembled` | `92-145` | 54 | `resolved` |
| `resolution-and-expansion` | `source-templates` | `whole-word-or-nothing` | `146-275` | 130 | `resolved` |
| `reading-and-whole-document-validation` | `source-templates` | `never-assembled` | `276-414` | 139 | `resolved` |
| `node-and-template-rules` | `source-templates` | `words-not-shell` | `415-534` | 120 | `resolved` |
| `word-scanning` | `source-templates` | `words-not-shell` | `535-617` | 83 | `resolved` |
| `diagnostics` | `source-templates` | `words-not-shell` | `618-660` | 43 | `resolved` |
| `templates-keys` | `source-templates` | `whole-word-or-nothing` | `661-670` | 10 | `resolved` |
| `argv` | `source-argv` | `whole-word-or-nothing` | `1-48` | 48 | `resolved` |
| `channel-production` | `source-channel` | `appearance-is-the-event` | `1-271` | 271 | `resolved` |
| `channel-inline-tests` | `source-channel` | `checked-without-meaning` | `272-404` | 133 | `resolved` |
| `launch-shape` | `source-run` | `nothing-else-added` | `1-123` | 123 | `resolved` |
| `watch-and-launcher-signals` | `source-run` | `the-launchers-job` | `124-243` | 120 | `resolved` |
| `terminal-and-spawn` | `source-run` | `nothing-else-added` | `244-448` | 205 | `resolved` |
| `supervise-and-escalate` | `source-run` | `the-launchers-job` | `449-607` | 159 | `resolved` |
| `conformance` | `source-conformance` | `checked-without-meaning` | `1-104` | 104 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-47` | `—` | `manifest-three-dependencies` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `1-10` | `manifest-three-dependencies` | `—` |
| `manifest-three-dependencies` | `orientation` | `source-crate-manifest` | `composite` | `understands-neither` | `1-47` | `source-crate-manifest` | `manifest-package-identity`, `manifest-dependencies`, `manifest-dev-dependencies`, `manifest-lints`, `manifest-release` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `11-26` | `manifest-three-dependencies` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `27-29` | `manifest-three-dependencies` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `30-32` | `manifest-three-dependencies` | `—` |
| `manifest-release` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `33-47` | `manifest-three-dependencies` | `—` |
| `source-library-root` | `source-index` | `source-library-root` | `root` | `—` | `1-68` | `—` | `library-root` |
| `library-root-thesis` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `1-9` | `library-root` | `—` |
| `library-root` | `orientation` | `source-library-root` | `composite` | `understands-neither` | `1-68` | `source-library-root` | `library-root-thesis`, `library-root-two-documents`, `library-root-vocabulary`, `library-root-to-a-child`, `library-root-job-and-out-of-band`, `library-root-conformance`, `library-root-modules-and-exports` |
| `library-root-two-documents` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `10-22` | `library-root` | `—` |
| `library-root-vocabulary` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `23-27` | `library-root` | `—` |
| `library-root-to-a-child` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `28-34` | `library-root` | `—` |
| `library-root-job-and-out-of-band` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `35-47` | `library-root` | `—` |
| `library-root-conformance` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `48-52` | `library-root` | `—` |
| `library-root-modules-and-exports` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `53-68` | `library-root` | `—` |
| `source-error-types` | `source-index` | `source-error-types` | `root` | `—` | `1-81` | `—` | `two-opaque-errors` |
| `error-import` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `1-1` | `two-opaque-errors` | `—` |
| `two-opaque-errors` | `orientation` | `source-error-types` | `composite` | `understands-neither` | `1-81` | `source-error-types` | `error-import`, `error-config-type`, `error-config-traits`, `error-launch-type`, `error-launch-traits` |
| `error-config-type` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `2-25` | `two-opaque-errors` | `—` |
| `error-config-traits` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `26-42` | `two-opaque-errors` | `—` |
| `error-launch-type` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `43-66` | `two-opaque-errors` | `—` |
| `error-launch-traits` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `67-81` | `two-opaque-errors` | `—` |
| `source-vocabulary` | `source-index` | `source-vocabulary` | `root` | `—` | `1-44` | `—` | `vocabulary` |
| `vocabulary-supplied-at-load` | `the-names` | `source-vocabulary` | `literal` | `rules-about-names` | `1-13` | `vocabulary` | `—` |
| `vocabulary` | `the-names` | `source-vocabulary` | `composite` | `rules-about-names` | `1-44` | `source-vocabulary` | `vocabulary-supplied-at-load`, `vocabulary-slot-rule`, `vocabulary-requirement`, `vocabulary-cardinality-and-message` |
| `vocabulary-slot-rule` | `the-names` | `source-vocabulary` | `literal` | `rules-about-names` | `14-20` | `vocabulary` | `—` |
| `vocabulary-requirement` | `the-names` | `source-vocabulary` | `literal` | `rules-about-names` | `21-27` | `vocabulary` | `—` |
| `vocabulary-cardinality-and-message` | `the-names` | `source-vocabulary` | `literal` | `rules-about-names` | `28-44` | `vocabulary` | `—` |
| `source-templates` | `source-index` | `source-templates` | `root` | `—` | `1-670` | `—` | `template-shapes`, `templates-load`, `resolution-and-expansion`, `reading-and-whole-document-validation`, `node-and-template-rules`, `word-scanning`, `diagnostics`, `templates-keys` |
| `template-shapes-imports` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `1-14` | `template-shapes` | `—` |
| `template-shapes` | `the-names` | `source-templates` | `composite` | `rules-about-names` | `1-91` | `source-templates` | `template-shapes-imports`, `template-shapes-templates`, `template-shapes-slot-spec`, `template-shapes-per-key-source`, `template-shapes-word`, `template-shapes-document-role`, `template-shapes-diagnostics` |
| `template-shapes-templates` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `15-28` | `template-shapes` | `—` |
| `template-shapes-slot-spec` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `29-33` | `template-shapes` | `—` |
| `template-shapes-per-key-source` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `34-45` | `template-shapes` | `—` |
| `template-shapes-word` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `46-52` | `template-shapes` | `—` |
| `template-shapes-document-role` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `53-73` | `template-shapes` | `—` |
| `template-shapes-diagnostics` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `74-91` | `template-shapes` | `—` |
| `templates-load-three-promises` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `92-108` | `templates-load` | `—` |
| `templates-load` | `two-documents` | `source-templates` | `composite` | `never-assembled` | `92-145` | `source-templates` | `templates-load-three-promises`, `templates-load-primary`, `templates-load-overlay`, `templates-load-value` |
| `templates-load-primary` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `109-114` | `templates-load` | `—` |
| `templates-load-overlay` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `115-135` | `templates-load` | `—` |
| `templates-load-value` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `136-145` | `templates-load` | `—` |
| `templates-source` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `146-154` | `resolution-and-expansion` | `—` |
| `resolution-and-expansion` | `to-an-argv` | `source-templates` | `composite` | `whole-word-or-nothing` | `146-275` | `source-templates` | `templates-source`, `templates-require`, `templates-expand`, `match-values`, `declared-slots`, `templates-unresolved` |
| `templates-require` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `155-167` | `resolution-and-expansion` | `—` |
| `templates-expand` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `168-199` | `resolution-and-expansion` | `—` |
| `match-values` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `200-243` | `resolution-and-expansion` | `—` |
| `declared-slots` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `244-251` | `resolution-and-expansion` | `—` |
| `templates-unresolved` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `252-275` | `resolution-and-expansion` | `—` |
| `compile-vocabulary` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `276-298` | `reading-and-whole-document-validation` | `—` |
| `reading-and-whole-document-validation` | `two-documents` | `source-templates` | `composite` | `never-assembled` | `276-414` | `source-templates` | `compile-vocabulary`, `read-primary`, `read-overlay`, `parse-and-validate`, `validate-document-nodes`, `validate-document-duplicates`, `validate-document-report` |
| `read-primary` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `299-312` | `reading-and-whole-document-validation` | `—` |
| `read-overlay` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `313-321` | `reading-and-whole-document-validation` | `—` |
| `parse-and-validate` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `322-341` | `reading-and-whole-document-validation` | `—` |
| `validate-document-nodes` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `342-361` | `reading-and-whole-document-validation` | `—` |
| `validate-document-duplicates` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `362-389` | `reading-and-whole-document-validation` | `—` |
| `validate-document-report` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `390-414` | `reading-and-whole-document-validation` | `—` |
| `validate-node-shape` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `415-433` | `node-and-template-rules` | `—` |
| `node-and-template-rules` | `template-law` | `source-templates` | `composite` | `words-not-shell` | `415-534` | `source-templates` | `validate-node-shape`, `validate-node-one-argument`, `validate-node-result`, `validate-template-signature`, `validate-template-comment-start`, `validate-template-split`, `validate-template-word-zero`, `validate-template-cardinality` |
| `validate-node-one-argument` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `434-460` | `node-and-template-rules` | `—` |
| `validate-node-result` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `461-468` | `node-and-template-rules` | `—` |
| `validate-template-signature` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `469-476` | `node-and-template-rules` | `—` |
| `validate-template-comment-start` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `477-484` | `node-and-template-rules` | `—` |
| `validate-template-split` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `485-496` | `node-and-template-rules` | `—` |
| `validate-template-word-zero` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `497-521` | `node-and-template-rules` | `—` |
| `validate-template-cardinality` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `522-534` | `node-and-template-rules` | `—` |
| `shell-word-scan-state` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `535-545` | `word-scanning` | `—` |
| `word-scanning` | `template-law` | `source-templates` | `composite` | `words-not-shell` | `535-617` | `source-templates` | `shell-word-scan-state`, `contains-shell-comment-start`, `parse-template-word`, `whole-substitution` |
| `contains-shell-comment-start` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `546-583` | `word-scanning` | `—` |
| `parse-template-word` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `584-611` | `word-scanning` | `—` |
| `whole-substitution` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `612-617` | `word-scanning` | `—` |
| `diagnostic-constructors` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `618-628` | `diagnostics` | `—` |
| `diagnostics` | `template-law` | `source-templates` | `composite` | `words-not-shell` | `618-660` | `source-templates` | `diagnostic-constructors`, `render-diagnostics`, `location-rendering` |
| `render-diagnostics` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `629-647` | `diagnostics` | `—` |
| `location-rendering` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `648-660` | `diagnostics` | `—` |
| `templates-keys` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `661-670` | `source-templates` | `—` |
| `source-argv` | `source-index` | `source-argv` | `root` | `—` | `1-48` | `—` | `argv` |
| `argv-slot` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `1-10` | `argv` | `—` |
| `argv` | `to-an-argv` | `source-argv` | `composite` | `whole-word-or-nothing` | `1-48` | `source-argv` | `argv-slot`, `argv-authored-only-by-expand`, `argv-no-public-constructor`, `argv-program-and-args`, `argv-words` |
| `argv-authored-only-by-expand` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `11-22` | `argv` | `—` |
| `argv-no-public-constructor` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `23-27` | `argv` | `—` |
| `argv-program-and-args` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `28-37` | `argv` | `—` |
| `argv-words` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `38-48` | `argv` | `—` |
| `source-channel` | `source-index` | `source-channel` | `root` | `—` | `1-404` | `—` | `channel-production`, `channel-inline-tests` |
| `channel-thesis` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `1-9` | `channel-production` | `—` |
| `channel-production` | `the-channel` | `source-channel` | `composite` | `appearance-is-the-event` | `1-271` | `source-channel` | `channel-thesis`, `channel-prefix`, `channel-nonce-bytes`, `channel-retry-limit`, `channel-type`, `channel-allocate`, `channel-published-path`, `channel-read`, `channel-discard`, `channel-discard-abandoned`, `channel-token`, `channel-signal`, `channel-name-grammar`, `channel-remove-if-present`, `channel-draw-nonce`, `channel-hex` |
| `channel-prefix` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `10-21` | `channel-production` | `—` |
| `channel-nonce-bytes` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `22-25` | `channel-production` | `—` |
| `channel-retry-limit` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `26-32` | `channel-production` | `—` |
| `channel-type` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `33-43` | `channel-production` | `—` |
| `channel-allocate` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `44-92` | `channel-production` | `—` |
| `channel-published-path` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `93-99` | `channel-production` | `—` |
| `channel-read` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `100-118` | `channel-production` | `—` |
| `channel-discard` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `119-128` | `channel-production` | `—` |
| `channel-discard-abandoned` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `129-184` | `channel-production` | `—` |
| `channel-token` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `185-202` | `channel-production` | `—` |
| `channel-signal` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `203-221` | `channel-production` | `—` |
| `channel-name-grammar` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `222-236` | `channel-production` | `—` |
| `channel-remove-if-present` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `237-247` | `channel-production` | `—` |
| `channel-draw-nonce` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `248-261` | `channel-production` | `—` |
| `channel-hex` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `262-271` | `channel-production` | `—` |
| `channel-tests-module` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `272-274` | `channel-inline-tests` | `—` |
| `channel-inline-tests` | `how-checked` | `source-channel` | `composite` | `checked-without-meaning` | `272-404` | `source-channel` | `channel-tests-module`, `channel-tests-allocate`, `channel-tests-read`, `channel-tests-discard`, `channel-tests-cleanup` |
| `channel-tests-allocate` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `275-312` | `channel-inline-tests` | `—` |
| `channel-tests-read` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `313-351` | `channel-inline-tests` | `—` |
| `channel-tests-discard` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `352-365` | `channel-inline-tests` | `—` |
| `channel-tests-cleanup` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `366-404` | `channel-inline-tests` | `—` |
| `source-run` | `source-index` | `source-run` | `root` | `—` | `1-607` | `—` | `launch-shape`, `watch-and-launcher-signals`, `terminal-and-spawn`, `supervise-and-escalate` |
| `run-thesis` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `1-13` | `launch-shape` | `—` |
| `launch-shape` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `1-123` | `source-run` | `run-thesis`, `run-poll-interval`, `run-escalation`, `run-launch`, `run-ended`, `run-end` |
| `run-poll-interval` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `14-21` | `launch-shape` | `—` |
| `run-escalation` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `22-42` | `launch-shape` | `—` |
| `run-launch` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `43-77` | `launch-shape` | `—` |
| `run-ended` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `78-92` | `launch-shape` | `—` |
| `run-end` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `93-123` | `launch-shape` | `—` |
| `run-watch-states` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `124-130` | `watch-and-launcher-signals` | `—` |
| `watch-and-launcher-signals` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `124-243` | `source-run` | `run-watch-states`, `run-interrupted-by`, `run-take-interrupt`, `run-reraise`, `run-on-terminate`, `run-install-termination-handler` |
| `run-interrupted-by` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `131-149` | `watch-and-launcher-signals` | `—` |
| `run-take-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `150-168` | `watch-and-launcher-signals` | `—` |
| `run-reraise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `169-211` | `watch-and-launcher-signals` | `—` |
| `run-on-terminate` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `212-218` | `watch-and-launcher-signals` | `—` |
| `run-install-termination-handler` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `219-243` | `watch-and-launcher-signals` | `—` |
| `run-default-dispositions` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `244-270` | `terminal-and-spawn` | `—` |
| `terminal-and-spawn` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `244-448` | `source-run` | `run-default-dispositions`, `run-terminal-type`, `run-terminal-open`, `run-terminal-accessors`, `run-terminal-hand-to`, `run-own-group`, `run-the-child-is-a-job`, `run-command-and-environment`, `run-terminal-handover`, `run-process-group`, `run-pre-exec`, `run-clear-and-spawn`, `run-parent-group-and-supervise` |
| `run-terminal-type` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `271-274` | `terminal-and-spawn` | `—` |
| `run-terminal-open` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `275-299` | `terminal-and-spawn` | `—` |
| `run-terminal-accessors` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `300-309` | `terminal-and-spawn` | `—` |
| `run-terminal-hand-to` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `310-328` | `terminal-and-spawn` | `—` |
| `run-own-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `329-334` | `terminal-and-spawn` | `—` |
| `run-the-child-is-a-job` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `335-366` | `terminal-and-spawn` | `—` |
| `run-command-and-environment` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `367-385` | `terminal-and-spawn` | `—` |
| `run-terminal-handover` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `386-397` | `terminal-and-spawn` | `—` |
| `run-process-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `398-403` | `terminal-and-spawn` | `—` |
| `run-pre-exec` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `404-425` | `terminal-and-spawn` | `—` |
| `run-clear-and-spawn` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `426-437` | `terminal-and-spawn` | `—` |
| `run-parent-group-and-supervise` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `438-448` | `terminal-and-spawn` | `—` |
| `run-supervise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `449-469` | `supervise-and-escalate` | `—` |
| `supervise-and-escalate` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `449-607` | `source-run` | `run-supervise`, `run-watch-signature`, `run-watch-ended`, `run-watch-terminal-recheck`, `run-watch-try-wait`, `run-watch-forward-interrupt`, `run-watch-escalation`, `run-kill` |
| `run-watch-signature` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `470-481` | `supervise-and-escalate` | `—` |
| `run-watch-ended` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `482-494` | `supervise-and-escalate` | `—` |
| `run-watch-terminal-recheck` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `495-506` | `supervise-and-escalate` | `—` |
| `run-watch-try-wait` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `507-532` | `supervise-and-escalate` | `—` |
| `run-watch-forward-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `533-553` | `supervise-and-escalate` | `—` |
| `run-watch-escalation` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `554-580` | `supervise-and-escalate` | `—` |
| `run-kill` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `581-607` | `supervise-and-escalate` | `—` |
| `source-conformance` | `source-index` | `source-conformance` | `root` | `—` | `1-104` | `—` | `conformance` |
| `conformance-thesis` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `1-31` | `conformance` | `—` |
| `conformance` | `how-checked` | `source-conformance` | `composite` | `checked-without-meaning` | `1-104` | `source-conformance` | `conformance-thesis`, `conformance-outcome`, `conformance-check-and-placeholders`, `conformance-load`, `conformance-no-keys`, `conformance-values`, `conformance-expands` |
| `conformance-outcome` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `32-43` | `conformance` | `—` |
| `conformance-check-and-placeholders` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `44-63` | `conformance` | `—` |
| `conformance-load` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `64-72` | `conformance` | `—` |
| `conformance-no-keys` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `73-82` | `conformance` | `—` |
| `conformance-values` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `83-90` | `conformance` | `—` |
| `conformance-expands` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `91-104` | `conformance` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Templates` | `01-orientation.md#the-cast` | `rules-about-names` | One loaded configuration: key to complete command template, compiled against a vocabulary and validated whole before anything is spawned. | `explained` |
| `Vocabulary`, `SlotRule`, `Requirement` | `01-orientation.md#the-cast` | `rules-about-names` | The slot names a consumer's templates are written against, each with a cardinality; supplied at load, because every template rule is a rule about a slot's name. | `explained` |
| `Argv`, `Slot` | `01-orientation.md#the-cast` | `whole-word-or-nothing` | `Argv` is a program and its arguments with no public constructor, authored only by `Templates::expand`; `Slot` is one name-and-value a caller offers to that call. | `explained` |
| `Channel`, `Token`, `signal` | `01-orientation.md#the-cast` | `appearance-is-the-event` | A fresh path per launch that allocation picks and writes nothing to; `signal` is what the child calls to make it appear, and `Token` is what the caller reads back. | `explained` |
| `run`, `Launch`, `Ended`, `End`, `Escalation` | `01-orientation.md#the-cast` | `nothing-else-added` | `run` spawns one `Launch` — argv, channel, scrub list, working directory and the two graces of an `Escalation` — and returns an `Ended` saying which of `End`'s three cases happened. | `explained` |
| `reraise`, `take_interrupt` | `01-orientation.md#the-cast` | `the-launchers-job` | The launcher's own two obligations for a termination signal: `take_interrupt` collects one that arrived between launches, and `reraise` is how a launcher dies of the same signal rather than reporting an exit code. | `explained` |
| `conformance::check` | `01-orientation.md#the-cast` | `checked-without-meaning` | The kit that holds a consumer's configuration to this crate's contract from outside the consumer's own suite. | `explained` |
| `validate_node`, `validate_template` | `03-two-documents.md#both-documents` | `words-not-shell` | The per-node and per-template rule checks `validate_document` drives over both documents; each returns diagnostics with locations rather than stopping at the first. | `explained` |
| `source_location`, `format_location`, `render_diagnostics` | `03-two-documents.md#parsed-then-validated` | `words-not-shell` | `source_location` turns a byte offset into a one-based line and column; `format_location` renders one as `path:line:column`; `render_diagnostics` assembles a document's path, role and diagnostics into one refusal. | `explained` |
| `install_termination_handler`, `INTERRUPTED_BY`, `supervise` | `07-the-job.md#the-spawn` | `the-launchers-job` | `run`'s first and last acts: the handler that latches the launcher's own SIGTERM or SIGHUP into the process-global `INTERRUPTED_BY`, cleared immediately before each spawn, and the supervisor that watches the child and takes the terminal back. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every line of the nine source roots is credited once, to the slice whose page
owns it; the table shows how the 2,073 lines divide across the ten chapters, and
its total is what a completed book must account for. Six of the nine roots are
owned whole by one chapter; the three that split — `src/templates.rs` four ways,
`src/run.rs` two ways, `src/channel.rs` at its `#[cfg(test)]` boundary — are why
the ownership table above has twenty rows rather than nine.

| Slice | Page | Owned lines |
|---|---|---:|
| `understands-neither` | `01-orientation.md` | 196 |
| `rules-about-names` | `02-the-names.md` | 135 |
| `never-assembled` | `03-two-documents.md` | 193 |
| `words-not-shell` | `04-template-law.md` | 246 |
| `whole-word-or-nothing` | `05-to-an-argv.md` | 188 |
| `appearance-is-the-event` | `06-the-channel.md` | 271 |
| `nothing-else-added` | `07-the-job.md` | 328 |
| `the-launchers-job` | `08-the-escalation.md` | 279 |
| `checked-without-meaning` | `09-how-checked.md` | 237 |
| `assembly` | `10-what-passes-through.md` | 0 |
| **Total** | 9 source roots | **2,073** |
