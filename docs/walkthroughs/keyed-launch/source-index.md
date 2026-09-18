# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/keyed-launch/Cargo.toml` | 47 |
| `source-library-root` | `crates/keyed-launch/src/lib.rs` | 102 |
| `source-error-types` | `crates/keyed-launch/src/error.rs` | 148 |
| `source-vocabulary` | `crates/keyed-launch/src/vocabulary.rs` | 46 |
| `source-templates` | `crates/keyed-launch/src/templates.rs` | 685 |
| `source-inspection` | `crates/keyed-launch/src/inspection.rs` | 104 |
| `source-argv` | `crates/keyed-launch/src/argv.rs` | 48 |
| `source-channel` | `crates/keyed-launch/src/channel.rs` | 453 |
| `source-run` | `crates/keyed-launch/src/run.rs` | 853 |
| `source-conformance` | `crates/keyed-launch/src/conformance.rs` | 98 |
| `source-named` | `crates/keyed-launch/src/templates/named.rs` | 1,359 |
| `source-confinement` | `crates/keyed-launch/src/confinement.rs` | 219 |

<!-- source-root «source-crate-manifest» source="crates/keyed-launch/Cargo.toml" lines="1-47" -->
<!-- insert «manifest-three-dependencies» -->
<!-- /source-root -->
<!-- source-root «source-library-root» source="crates/keyed-launch/src/lib.rs" lines="1-102" -->
<!-- insert «library-root» -->
<!-- /source-root -->
<!-- source-root «source-error-types» source="crates/keyed-launch/src/error.rs" lines="1-148" -->
<!-- insert «two-opaque-errors» -->
<!-- /source-root -->
<!-- source-root «source-vocabulary» source="crates/keyed-launch/src/vocabulary.rs" lines="1-46" -->
<!-- insert «vocabulary» -->
<!-- /source-root -->
<!-- source-root «source-templates» source="crates/keyed-launch/src/templates.rs" lines="1-685" -->
<!-- insert «template-shapes» -->
<!-- insert «templates-load» -->
<!-- insert «resolution-and-expansion» -->
<!-- insert «reading-and-whole-document-validation» -->
<!-- insert «word-scanning» -->
<!-- insert «diagnostics» -->
<!-- insert «templates-keys» -->
<!-- /source-root -->
<!-- source-root «source-inspection» source="crates/keyed-launch/src/inspection.rs" lines="1-104" -->
<!-- insert «inspection-records» -->
<!-- /source-root -->
<!-- source-root «source-argv» source="crates/keyed-launch/src/argv.rs" lines="1-48" -->
<!-- insert «argv» -->
<!-- /source-root -->
<!-- source-root «source-channel» source="crates/keyed-launch/src/channel.rs" lines="1-453" -->
<!-- insert «channel-production» -->
<!-- insert «channel-inline-tests» -->
<!-- /source-root -->
<!-- source-root «source-run» source="crates/keyed-launch/src/run.rs" lines="1-853" -->
<!-- insert «launch-shape» -->
<!-- insert «watch-and-launcher-signals» -->
<!-- insert «terminal-and-spawn» -->
<!-- insert «supervise-and-escalate» -->
<!-- /source-root -->
<!-- source-root «source-conformance» source="crates/keyed-launch/src/conformance.rs" lines="1-98" -->
<!-- insert «conformance» -->
<!-- /source-root -->

<!-- source-root «source-named» source="crates/keyed-launch/src/templates/named.rs" lines="1-1359" -->
<!-- insert «named-capture» -->
<!-- insert «named-diagnostics» -->
<!-- insert «named-compile» -->
<!-- insert «named-fold» -->
<!-- insert «named-resolve» -->
<!-- insert «named-lookups» -->
<!-- /source-root -->

<!-- source-root «source-confinement» source="crates/keyed-launch/src/confinement.rs" lines="1-219" -->
<!-- insert «confinement-policy» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-three-dependencies` | `source-crate-manifest` | `understands-neither` | `1-47` | 47 | `resolved` |
| `library-root` | `source-library-root` | `understands-neither` | `1-102` | 102 | `resolved` |
| `two-opaque-errors` | `source-error-types` | `understands-neither` | `1-148` | 148 | `resolved` |
| `vocabulary` | `source-vocabulary` | `rules-about-names` | `1-46` | 46 | `resolved` |
| `template-shapes` | `source-templates` | `rules-about-names` | `1-150` | 150 | `resolved` |
| `templates-load` | `source-templates` | `never-assembled` | `151-248` | 98 | `resolved` |
| `resolution-and-expansion` | `source-templates` | `whole-word-or-nothing` | `249-425` | 177 | `resolved` |
| `reading-and-whole-document-validation` | `source-templates` | `never-assembled` | `426-554` | 129 | `resolved` |
| `word-scanning` | `source-templates` | `words-not-shell` | `555-604` | 50 | `resolved` |
| `diagnostics` | `source-templates` | `words-not-shell` | `605-675` | 71 | `resolved` |
| `templates-keys` | `source-templates` | `whole-word-or-nothing` | `676-685` | 10 | `resolved` |
| `inspection-records` | `source-inspection` | `rules-about-names` | `1-104` | 104 | `resolved` |
| `argv` | `source-argv` | `whole-word-or-nothing` | `1-48` | 48 | `resolved` |
| `channel-production` | `source-channel` | `appearance-is-the-event` | `1-288` | 288 | `resolved` |
| `channel-inline-tests` | `source-channel` | `checked-without-meaning` | `289-453` | 165 | `resolved` |
| `launch-shape` | `source-run` | `nothing-else-added` | `1-124` | 124 | `resolved` |
| `watch-and-launcher-signals` | `source-run` | `the-launchers-job` | `125-244` | 120 | `resolved` |
| `terminal-and-spawn` | `source-run` | `nothing-else-added` | `245-679` | 435 | `resolved` |
| `supervise-and-escalate` | `source-run` | `the-launchers-job` | `680-853` | 174 | `resolved` |
| `conformance` | `source-conformance` | `checked-without-meaning` | `1-98` | 98 | `resolved` |
| `named-capture` | `source-named` | `never-assembled` | `1-533` | 533 | `resolved` |
| `named-diagnostics` | `source-named` | `never-assembled` | `534-550` | 17 | `resolved` |
| `named-compile` | `source-named` | `words-not-shell` | `551-688` | 138 | `resolved` |
| `named-fold` | `source-named` | `never-assembled` | `689-1027` | 339 | `resolved` |
| `named-resolve` | `source-named` | `never-assembled` | `1028-1324` | 297 | `resolved` |
| `named-lookups` | `source-named` | `never-assembled` | `1325-1359` | 35 | `resolved` |
| `confinement-policy` | `source-confinement` | `confined-jobs` | `1-219` | 219 | `resolved` |

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
| `source-library-root` | `source-index` | `source-library-root` | `root` | `—` | `1-102` | `—` | `library-root` |
| `library-root-thesis` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `1-8` | `library-root` | `—` |
| `library-root` | `orientation` | `source-library-root` | `composite` | `understands-neither` | `1-102` | `source-library-root` | `library-root-thesis`, `library-root-two-documents`, `library-root-vocabulary`, `library-root-to-a-child`, `library-root-job-and-out-of-band`, `library-root-conformance`, `library-root-modules-and-exports` |
| `library-root-two-documents` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `9-21` | `library-root` | `—` |
| `library-root-vocabulary` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `22-43` | `library-root` | `—` |
| `library-root-to-a-child` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `44-53` | `library-root` | `—` |
| `library-root-job-and-out-of-band` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `54-66` | `library-root` | `—` |
| `library-root-conformance` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `67-76` | `library-root` | `—` |
| `library-root-modules-and-exports` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `77-102` | `library-root` | `—` |
| `source-error-types` | `source-index` | `source-error-types` | `root` | `—` | `1-148` | `—` | `two-opaque-errors` |
| `error-import` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `1-1` | `two-opaque-errors` | `—` |
| `two-opaque-errors` | `orientation` | `source-error-types` | `composite` | `understands-neither` | `1-148` | `source-error-types` | `error-import`, `error-config-type`, `error-config-traits`, `error-launch-type`, `error-launch-traits` |
| `error-config-type` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `2-93` | `two-opaque-errors` | `—` |
| `error-config-traits` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `94-109` | `two-opaque-errors` | `—` |
| `error-launch-type` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `110-133` | `two-opaque-errors` | `—` |
| `error-launch-traits` | `orientation` | `source-error-types` | `literal` | `understands-neither` | `134-148` | `two-opaque-errors` | `—` |
| `source-vocabulary` | `source-index` | `source-vocabulary` | `root` | `—` | `1-46` | `—` | `vocabulary` |
| `vocabulary-supplied-at-load` | `the-names` | `source-vocabulary` | `literal` | `rules-about-names` | `1-15` | `vocabulary` | `—` |
| `vocabulary` | `the-names` | `source-vocabulary` | `composite` | `rules-about-names` | `1-46` | `source-vocabulary` | `vocabulary-supplied-at-load`, `vocabulary-slot-rule`, `vocabulary-requirement`, `vocabulary-cardinality-and-message` |
| `vocabulary-slot-rule` | `the-names` | `source-vocabulary` | `literal` | `rules-about-names` | `16-22` | `vocabulary` | `—` |
| `vocabulary-requirement` | `the-names` | `source-vocabulary` | `literal` | `rules-about-names` | `23-29` | `vocabulary` | `—` |
| `vocabulary-cardinality-and-message` | `the-names` | `source-vocabulary` | `literal` | `rules-about-names` | `30-46` | `vocabulary` | `—` |
| `source-templates` | `source-index` | `source-templates` | `root` | `—` | `1-685` | `—` | `template-shapes`, `templates-load`, `resolution-and-expansion`, `reading-and-whole-document-validation`, `word-scanning`, `diagnostics`, `templates-keys` |
| `template-shapes-imports` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `1-19` | `template-shapes` | `—` |
| `template-shapes` | `the-names` | `source-templates` | `composite` | `rules-about-names` | `1-150` | `source-templates` | `template-shapes-imports`, `template-shapes-templates`, `template-shapes-slot-spec`, `template-shapes-per-key-source`, `template-shapes-word`, `template-shapes-document-role`, `template-shapes-diagnostics` |
| `template-shapes-templates` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `20-86` | `template-shapes` | `—` |
| `template-shapes-slot-spec` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `87-92` | `template-shapes` | `—` |
| `template-shapes-per-key-source` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `93-101` | `template-shapes` | `—` |
| `template-shapes-word` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `102-104` | `template-shapes` | `—` |
| `template-shapes-document-role` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `105-133` | `template-shapes` | `—` |
| `template-shapes-diagnostics` | `the-names` | `source-templates` | `literal` | `rules-about-names` | `134-150` | `template-shapes` | `—` |
| `templates-load-three-promises` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `151-160` | `templates-load` | `—` |
| `templates-load` | `two-documents` | `source-templates` | `composite` | `never-assembled` | `151-248` | `source-templates` | `templates-load-three-promises`, `templates-load-primary`, `templates-load-overlay`, `templates-load-value` |
| `templates-load-primary` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `161-209` | `templates-load` | `—` |
| `templates-load-overlay` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `210-210` | `templates-load` | `—` |
| `templates-load-value` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `211-248` | `templates-load` | `—` |
| `templates-source` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `249-257` | `resolution-and-expansion` | `—` |
| `resolution-and-expansion` | `to-an-argv` | `source-templates` | `composite` | `whole-word-or-nothing` | `249-425` | `source-templates` | `templates-source`, `templates-require`, `templates-expand`, `match-values`, `declared-slots`, `templates-unresolved` |
| `templates-require` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `258-283` | `resolution-and-expansion` | `—` |
| `templates-expand` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `284-330` | `resolution-and-expansion` | `—` |
| `match-values` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `331-393` | `resolution-and-expansion` | `—` |
| `declared-slots` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `394-401` | `resolution-and-expansion` | `—` |
| `templates-unresolved` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `402-425` | `resolution-and-expansion` | `—` |
| `compile-vocabulary` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `426-463` | `reading-and-whole-document-validation` | `—` |
| `reading-and-whole-document-validation` | `two-documents` | `source-templates` | `composite` | `never-assembled` | `426-554` | `source-templates` | `compile-vocabulary`, `read-primary`, `read-overlay`, `parse-and-validate` |
| `read-primary` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `464-488` | `reading-and-whole-document-validation` | `—` |
| `read-overlay` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `489-508` | `reading-and-whole-document-validation` | `—` |
| `parse-and-validate` | `two-documents` | `source-templates` | `literal` | `never-assembled` | `509-554` | `reading-and-whole-document-validation` | `—` |
| `word-scanning` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `555-604` | `source-templates` | `—` |
| `diagnostics` | `template-law` | `source-templates` | `literal` | `words-not-shell` | `605-675` | `source-templates` | `—` |
| `templates-keys` | `to-an-argv` | `source-templates` | `literal` | `whole-word-or-nothing` | `676-685` | `source-templates` | `—` |
| `source-inspection` | `source-index` | `source-inspection` | `root` | `—` | `1-104` | `—` | `inspection-records` |
| `inspection-assignments` | `the-names` | `source-inspection` | `literal` | `rules-about-names` | `1-44` | `inspection-records` | `—` |
| `inspection-records` | `the-names` | `source-inspection` | `composite` | `rules-about-names` | `1-104` | `source-inspection` | `inspection-assignments`, `inspection-words`, `inspection-commands`, `inspection-snapshot` |
| `inspection-words` | `the-names` | `source-inspection` | `literal` | `rules-about-names` | `45-59` | `inspection-records` | `—` |
| `inspection-commands` | `the-names` | `source-inspection` | `literal` | `rules-about-names` | `60-88` | `inspection-records` | `—` |
| `inspection-snapshot` | `the-names` | `source-inspection` | `literal` | `rules-about-names` | `89-104` | `inspection-records` | `—` |
| `source-argv` | `source-index` | `source-argv` | `root` | `—` | `1-48` | `—` | `argv` |
| `argv-slot` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `1-10` | `argv` | `—` |
| `argv` | `to-an-argv` | `source-argv` | `composite` | `whole-word-or-nothing` | `1-48` | `source-argv` | `argv-slot`, `argv-authored-only-by-expand`, `argv-no-public-constructor`, `argv-program-and-args`, `argv-words` |
| `argv-authored-only-by-expand` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `11-22` | `argv` | `—` |
| `argv-no-public-constructor` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `23-27` | `argv` | `—` |
| `argv-program-and-args` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `28-37` | `argv` | `—` |
| `argv-words` | `to-an-argv` | `source-argv` | `literal` | `whole-word-or-nothing` | `38-48` | `argv` | `—` |
| `source-channel` | `source-index` | `source-channel` | `root` | `—` | `1-453` | `—` | `channel-production`, `channel-inline-tests` |
| `channel-thesis` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `1-9` | `channel-production` | `—` |
| `channel-production` | `the-channel` | `source-channel` | `composite` | `appearance-is-the-event` | `1-288` | `source-channel` | `channel-thesis`, `channel-prefix`, `channel-nonce-bytes`, `channel-retry-limit`, `channel-type`, `channel-allocate`, `channel-published-path`, `channel-read`, `channel-discard`, `channel-discard-abandoned`, `channel-token`, `channel-signal`, `channel-name-grammar`, `channel-remove-if-present`, `channel-draw-nonce`, `channel-hex` |
| `channel-prefix` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `10-21` | `channel-production` | `—` |
| `channel-nonce-bytes` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `22-25` | `channel-production` | `—` |
| `channel-retry-limit` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `26-32` | `channel-production` | `—` |
| `channel-type` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `33-44` | `channel-production` | `—` |
| `channel-allocate` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `45-99` | `channel-production` | `—` |
| `channel-published-path` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `100-106` | `channel-production` | `—` |
| `channel-read` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `107-135` | `channel-production` | `—` |
| `channel-discard` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `136-145` | `channel-production` | `—` |
| `channel-discard-abandoned` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `146-201` | `channel-production` | `—` |
| `channel-token` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `202-219` | `channel-production` | `—` |
| `channel-signal` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `220-238` | `channel-production` | `—` |
| `channel-name-grammar` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `239-253` | `channel-production` | `—` |
| `channel-remove-if-present` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `254-264` | `channel-production` | `—` |
| `channel-draw-nonce` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `265-278` | `channel-production` | `—` |
| `channel-hex` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `279-288` | `channel-production` | `—` |
| `channel-tests-module` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `289-291` | `channel-inline-tests` | `—` |
| `channel-inline-tests` | `how-checked` | `source-channel` | `composite` | `checked-without-meaning` | `289-453` | `source-channel` | `channel-tests-module`, `channel-tests-allocate`, `channel-tests-read`, `channel-tests-discard`, `channel-tests-cleanup` |
| `channel-tests-allocate` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `292-329` | `channel-inline-tests` | `—` |
| `channel-tests-read` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `330-368` | `channel-inline-tests` | `—` |
| `channel-untrusted-entry-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `369-387` | `channel-tests-discard` | `—` |
| `channel-tests-discard` | `how-checked` | `source-channel` | `composite` | `checked-without-meaning` | `369-414` | `channel-inline-tests` | `channel-untrusted-entry-test`, `channel-parent-identity-test`, `channel-discard-postcondition-test` |
| `channel-parent-identity-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `388-401` | `channel-tests-discard` | `—` |
| `channel-discard-postcondition-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `402-414` | `channel-tests-discard` | `—` |
| `channel-tests-cleanup` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `415-453` | `channel-inline-tests` | `—` |
| `source-run` | `source-index` | `source-run` | `root` | `—` | `1-853` | `—` | `launch-shape`, `watch-and-launcher-signals`, `terminal-and-spawn`, `supervise-and-escalate` |
| `run-thesis` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `1-14` | `launch-shape` | `—` |
| `launch-shape` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `1-124` | `source-run` | `run-thesis`, `run-poll-interval`, `run-escalation`, `run-launch`, `run-ended`, `run-end` |
| `run-poll-interval` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `15-22` | `launch-shape` | `—` |
| `run-escalation` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `23-43` | `launch-shape` | `—` |
| `run-launch` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `44-78` | `launch-shape` | `—` |
| `run-ended` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `79-93` | `launch-shape` | `—` |
| `run-end` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `94-124` | `launch-shape` | `—` |
| `run-watch-states` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `125-131` | `watch-and-launcher-signals` | `—` |
| `watch-and-launcher-signals` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `125-244` | `source-run` | `run-watch-states`, `run-interrupted-by`, `run-take-interrupt`, `run-reraise`, `run-on-terminate`, `run-install-termination-handler` |
| `run-interrupted-by` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `132-150` | `watch-and-launcher-signals` | `—` |
| `run-take-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `151-169` | `watch-and-launcher-signals` | `—` |
| `run-reraise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `170-212` | `watch-and-launcher-signals` | `—` |
| `run-on-terminate` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `213-219` | `watch-and-launcher-signals` | `—` |
| `run-install-termination-handler` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `220-244` | `watch-and-launcher-signals` | `—` |
| `run-default-dispositions` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `245-271` | `terminal-and-spawn` | `—` |
| `terminal-and-spawn` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `245-679` | `source-run` | `run-default-dispositions`, `run-terminal-type`, `run-terminal-open`, `run-terminal-accessors`, `run-terminal-hand-to`, `run-own-group`, `run-the-child-is-a-job`, `run-command-and-environment`, `run-terminal-handover`, `run-process-group`, `run-pre-exec`, `run-clear-and-spawn`, `run-parent-group-and-supervise` |
| `run-terminal-type` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `272-275` | `terminal-and-spawn` | `—` |
| `run-terminal-open` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `276-300` | `terminal-and-spawn` | `—` |
| `run-terminal-accessors` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `301-310` | `terminal-and-spawn` | `—` |
| `run-terminal-hand-to` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `311-329` | `terminal-and-spawn` | `—` |
| `run-own-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `330-335` | `terminal-and-spawn` | `—` |
| `run-the-child-is-a-job` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `336-367` | `terminal-and-spawn` | `—` |
| `run-output-modes` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `368-434` | `run-command-and-environment` | `—` |
| `run-command-and-environment` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `368-473` | `terminal-and-spawn` | `run-output-modes`, `run-output-setup` |
| `run-output-setup` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `435-473` | `run-command-and-environment` | `—` |
| `run-terminal-handover` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `474-485` | `terminal-and-spawn` | `—` |
| `run-process-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `486-493` | `terminal-and-spawn` | `—` |
| `run-pre-exec` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `494-533` | `terminal-and-spawn` | `—` |
| `run-clear-and-spawn` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `534-545` | `terminal-and-spawn` | `—` |
| `run-spawn-handoff` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `546-576` | `run-parent-group-and-supervise` | `—` |
| `run-parent-group-and-supervise` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `546-679` | `terminal-and-spawn` | `run-spawn-handoff`, `run-descriptor-bound`, `run-group-drain`, `run-process-adapter` |
| `run-descriptor-bound` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `577-614` | `run-parent-group-and-supervise` | `—` |
| `run-group-drain` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `615-637` | `run-parent-group-and-supervise` | `—` |
| `run-process-adapter` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `638-679` | `run-parent-group-and-supervise` | `—` |
| `run-supervise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `680-706` | `supervise-and-escalate` | `—` |
| `supervise-and-escalate` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `680-853` | `source-run` | `run-supervise`, `run-watch-signature`, `run-watch-ended`, `run-watch-terminal-recheck`, `run-watch-forward-interrupt`, `run-watch-try-wait`, `run-watch-escalation`, `run-kill` |
| `run-watch-signature` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `707-720` | `supervise-and-escalate` | `—` |
| `run-watch-ended` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `721-733` | `supervise-and-escalate` | `—` |
| `run-watch-terminal-recheck` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `734-745` | `supervise-and-escalate` | `—` |
| `run-watch-forward-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `746-759` | `supervise-and-escalate` | `—` |
| `run-watch-try-wait` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `760-791` | `supervise-and-escalate` | `—` |
| `run-watch-escalation` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `792-822` | `supervise-and-escalate` | `—` |
| `run-kill` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `823-853` | `supervise-and-escalate` | `—` |
| `source-conformance` | `source-index` | `source-conformance` | `root` | `—` | `1-98` | `—` | `conformance` |
| `conformance-thesis` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `1-31` | `conformance` | `—` |
| `conformance` | `how-checked` | `source-conformance` | `composite` | `checked-without-meaning` | `1-98` | `source-conformance` | `conformance-thesis`, `conformance-outcome`, `conformance-check-and-placeholders`, `conformance-load`, `conformance-no-keys`, `conformance-values`, `conformance-expands` |
| `conformance-outcome` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `32-43` | `conformance` | `—` |
| `conformance-check-and-placeholders` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `44-57` | `conformance` | `—` |
| `conformance-load` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `58-66` | `conformance` | `—` |
| `conformance-no-keys` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `67-76` | `conformance` | `—` |
| `conformance-values` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `77-84` | `conformance` | `—` |
| `conformance-expands` | `how-checked` | `source-conformance` | `literal` | `checked-without-meaning` | `85-98` | `conformance` | `—` |
| `source-named` | `source-index` | `source-named` | `root` | `—` | `1-1359` | `—` | `named-capture`, `named-diagnostics`, `named-compile`, `named-fold`, `named-resolve`, `named-lookups` |
| `named-capture` | `two-documents` | `source-named` | `literal` | `never-assembled` | `1-533` | `source-named` | `—` |
| `named-diagnostics` | `two-documents` | `source-named` | `literal` | `never-assembled` | `534-550` | `source-named` | `—` |
| `named-compile` | `template-law` | `source-named` | `literal` | `words-not-shell` | `551-688` | `source-named` | `—` |
| `named-fold` | `two-documents` | `source-named` | `literal` | `never-assembled` | `689-1027` | `source-named` | `—` |
| `named-resolve` | `two-documents` | `source-named` | `literal` | `never-assembled` | `1028-1324` | `source-named` | `—` |
| `named-lookups` | `two-documents` | `source-named` | `literal` | `never-assembled` | `1325-1359` | `source-named` | `—` |
| `source-confinement` | `source-index` | `source-confinement` | `root` | `—` | `1-219` | `—` | `confinement-policy` |
| `held-directory-read` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `1-44` | `confinement-policy` | `—` |
| `confinement-policy` | `confined-jobs` | `source-confinement` | `composite` | `confined-jobs` | `1-219` | `source-confinement` | `held-directory-read`, `confinement-contract`, `confinement-resource-resolution`, `confinement-macos`, `confinement-linux`, `confinement-unavailable` |
| `confinement-contract` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `45-78` | `confinement-policy` | `—` |
| `confinement-resource-resolution` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `79-103` | `confinement-policy` | `—` |
| `confinement-macos` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `104-155` | `confinement-policy` | `—` |
| `confinement-linux` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `156-213` | `confinement-policy` | `—` |
| `confinement-unavailable` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `214-219` | `confinement-policy` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Catalog`, `Selection`, `SourceRole`, `Source`, `SourceSpan` | `01-orientation.md#the-cast` | `rules-about-names` | Catalog owns captured documents and vocabulary; Selection supplies the explicit profile list and optional source origin; SourceRole, Source and SourceSpan identify that origin. | `explained` |
| `Inspection`, `CompiledWord`, `Origin`, `AssignmentHistory` | `01-orientation.md#the-cast` | `rules-about-names` | Inspection explains captured commands and replaced assignments through response-local origins and histories; CompiledWord is the literal/slot representation also used by expansion. | `explained` |
| `Templates` | `01-orientation.md#the-cast` | `rules-about-names` | One loaded configuration: key to complete command template, compiled against a vocabulary and validated whole before anything is spawned. | `explained` |
| `Vocabulary`, `SlotRule`, `Requirement` | `01-orientation.md#the-cast` | `rules-about-names` | The slot names a consumer's templates are written against, each with a cardinality; supplied at load, because every template rule is a rule about a slot's name. | `explained` |
| `Argv`, `Slot` | `01-orientation.md#the-cast` | `whole-word-or-nothing` | `Argv` is a program and its arguments with no public constructor, authored only by `Templates::expand`; `Slot` is one name-and-value a caller offers to that call. | `explained` |
| `Channel`, `Token`, `signal` | `01-orientation.md#the-cast` | `appearance-is-the-event` | A fresh path per launch that allocation picks and writes nothing to; `signal` is what the child calls to make it appear, and `Token` is what the caller reads back. | `explained` |
| `run`, `run_observed`, `LaunchEvent`, `Launch`, `Ended`, `End`, `Escalation` | `01-orientation.md#the-cast` | `nothing-else-added` | `run_observed` reports successful spawn and confirmed reap synchronously; `run` uses a no-op observer. Each spawns one `Launch` — argv, channel, scrub list, working directory and the two graces of an `Escalation` — and returns an `Ended` saying which of `End`'s three cases happened. | `explained` |
| `reraise`, `take_interrupt` | `01-orientation.md#the-cast` | `the-launchers-job` | The launcher's own two obligations for a termination signal: `take_interrupt` collects one that arrived between launches, and `reraise` is how a launcher dies of the same signal rather than reporting an exit code. | `explained` |
| `conformance::check` | `01-orientation.md#the-cast` | `checked-without-meaning` | The kit that holds a consumer's configuration to this crate's contract from outside the consumer's own suite. | `explained` |
| `named::compile` | `03-two-documents.md#both-documents` | `words-not-shell` | The compiler turns an effective command definition into argument fragments and runtime slots, checking the captured vocabulary before expansion. | `explained` |
| `source_location`, `format_location`, `render_diagnostics` | `03-two-documents.md#parsed-then-validated` | `words-not-shell` | `source_location` turns a byte offset into a one-based line and column; `format_location` renders one as `path:line:column`; `render_diagnostics` assembles a document's path, role and diagnostics into one refusal. | `explained` |
| `install_termination_handler`, `INTERRUPTED_BY`, `supervise` | `07-the-job.md#the-spawn` | `the-launchers-job` | `run`'s first and last acts: the handler that latches the launcher's own SIGTERM or SIGHUP into the process-global `INTERRUPTED_BY`, cleared immediately before each spawn, and the supervisor that watches the child and takes the terminal back. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every source line is credited once to its owning chapter. The 11 roots
contain 3,710 lines, divided below by the manifest's ownership
blocks. Templates, run and channel split across chapters; the remaining roots
are owned whole.

| Slice | Page | Owned lines |
|---|---|---:|
| `understands-neither` | `01-orientation.md` | 297 |
| `rules-about-names` | `02-the-names.md` | 300 |
| `never-assembled` | `03-two-documents.md` | 1,448 |
| `words-not-shell` | `04-template-law.md` | 259 |
| `whole-word-or-nothing` | `05-to-an-argv.md` | 235 |
| `appearance-is-the-event` | `06-the-channel.md` | 288 |
| `nothing-else-added` | `07-the-job.md` | 559 |
| `the-launchers-job` | `08-the-escalation.md` | 294 |
| `checked-without-meaning` | `09-how-checked.md` | 263 |
| `assembly` | `10-what-passes-through.md` | 0 |
| `confined-jobs` | `11-confined-jobs.md` | 219 |
| **Total** | 12 source roots | **4,162** |
