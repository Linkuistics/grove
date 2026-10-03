# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/keyed-launch/Cargo.toml` | 42 |
| `source-library-root` | `crates/keyed-launch/src/lib.rs` | 52 |
| `source-error-type` | `crates/keyed-launch/src/error.rs` | 56 |
| `source-argv` | `crates/keyed-launch/src/argv.rs` | 64 |
| `source-channel` | `crates/keyed-launch/src/channel.rs` | 505 |
| `source-run` | `crates/keyed-launch/src/run.rs` | 1,349 |
| `source-confinement` | `crates/keyed-launch/src/confinement.rs` | 301 |

<!-- source-root «source-crate-manifest» source="crates/keyed-launch/Cargo.toml" lines="1-42" -->
<!-- insert «manifest-one-dependency» -->
<!-- /source-root -->
<!-- source-root «source-library-root» source="crates/keyed-launch/src/lib.rs" lines="1-52" -->
<!-- insert «library-root» -->
<!-- /source-root -->
<!-- source-root «source-error-type» source="crates/keyed-launch/src/error.rs" lines="1-56" -->
<!-- insert «one-opaque-error» -->
<!-- /source-root -->
<!-- source-root «source-argv» source="crates/keyed-launch/src/argv.rs" lines="1-64" -->
<!-- insert «argv» -->
<!-- /source-root -->
<!-- source-root «source-channel» source="crates/keyed-launch/src/channel.rs" lines="1-505" -->
<!-- insert «channel-production» -->
<!-- insert «channel-inline-tests» -->
<!-- /source-root -->
<!-- source-root «source-run» source="crates/keyed-launch/src/run.rs" lines="1-1349" -->
<!-- insert «launch-shape» -->
<!-- insert «watch-and-launcher-signals» -->
<!-- insert «terminal-and-spawn» -->
<!-- insert «supervise-and-escalate» -->
<!-- /source-root -->
<!-- source-root «source-confinement» source="crates/keyed-launch/src/confinement.rs" lines="1-301" -->
<!-- insert «confinement-policy» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-one-dependency` | `source-crate-manifest` | `understands-neither` | `1-42` | 42 | `resolved` |
| `library-root` | `source-library-root` | `understands-neither` | `1-52` | 52 | `resolved` |
| `one-opaque-error` | `source-error-type` | `understands-neither` | `1-56` | 56 | `resolved` |
| `argv` | `source-argv` | `understands-neither` | `1-64` | 64 | `resolved` |
| `channel-production` | `source-channel` | `appearance-is-the-event` | `1-319` | 319 | `resolved` |
| `channel-inline-tests` | `source-channel` | `checked-without-meaning` | `320-505` | 186 | `resolved` |
| `launch-shape` | `source-run` | `nothing-else-added` | `1-202` | 202 | `resolved` |
| `watch-and-launcher-signals` | `source-run` | `the-launchers-job` | `203-391` | 189 | `resolved` |
| `terminal-and-spawn` | `source-run` | `nothing-else-added` | `392-1004` | 613 | `resolved` |
| `supervise-and-escalate` | `source-run` | `the-launchers-job` | `1005-1349` | 345 | `resolved` |
| `confinement-policy` | `source-confinement` | `confined-jobs` | `1-301` | 301 | `resolved` |

<a id="fragment-index"></a>
## Fragment index

| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `source-crate-manifest` | `source-index` | `source-crate-manifest` | `root` | `—` | `1-42` | `—` | `manifest-one-dependency` |
| `manifest-package-identity` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `1-10` | `manifest-one-dependency` | `—` |
| `manifest-one-dependency` | `orientation` | `source-crate-manifest` | `composite` | `understands-neither` | `1-42` | `source-crate-manifest` | `manifest-package-identity`, `manifest-dependencies`, `manifest-dev-dependencies`, `manifest-lints`, `manifest-release` |
| `manifest-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `11-21` | `manifest-one-dependency` | `—` |
| `manifest-dev-dependencies` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `22-24` | `manifest-one-dependency` | `—` |
| `manifest-lints` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `25-27` | `manifest-one-dependency` | `—` |
| `manifest-release` | `orientation` | `source-crate-manifest` | `literal` | `understands-neither` | `28-42` | `manifest-one-dependency` | `—` |
| `source-library-root` | `source-index` | `source-library-root` | `root` | `—` | `1-52` | `—` | `library-root` |
| `library-root-thesis` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `1-8` | `library-root` | `—` |
| `library-root` | `orientation` | `source-library-root` | `composite` | `understands-neither` | `1-52` | `source-library-root` | `library-root-thesis`, `library-root-to-a-child`, `library-root-job-and-out-of-band`, `library-root-observed`, `library-root-modules-and-exports` |
| `library-root-to-a-child` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `9-14` | `library-root` | `—` |
| `library-root-job-and-out-of-band` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `15-30` | `library-root` | `—` |
| `library-root-observed` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `31-35` | `library-root` | `—` |
| `library-root-modules-and-exports` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `36-52` | `library-root` | `—` |
| `source-error-type` | `source-index` | `source-error-type` | `root` | `—` | `1-56` | `—` | `one-opaque-error` |
| `error-import` | `orientation` | `source-error-type` | `literal` | `understands-neither` | `1-1` | `one-opaque-error` | `—` |
| `one-opaque-error` | `orientation` | `source-error-type` | `composite` | `understands-neither` | `1-56` | `source-error-type` | `error-import`, `error-launch-type`, `error-launch-traits` |
| `error-launch-type` | `orientation` | `source-error-type` | `literal` | `understands-neither` | `2-40` | `one-opaque-error` | `—` |
| `error-launch-traits` | `orientation` | `source-error-type` | `literal` | `understands-neither` | `41-56` | `one-opaque-error` | `—` |
| `source-argv` | `source-index` | `source-argv` | `root` | `—` | `1-64` | `—` | `argv` |
| `argv-type` | `orientation` | `source-argv` | `literal` | `understands-neither` | `1-12` | `argv` | `—` |
| `argv` | `orientation` | `source-argv` | `composite` | `understands-neither` | `1-64` | `source-argv` | `argv-type`, `argv-public-constructor`, `argv-program-and-args`, `argv-words` |
| `argv-public-constructor` | `orientation` | `source-argv` | `literal` | `understands-neither` | `13-36` | `argv` | `—` |
| `argv-program-and-args` | `orientation` | `source-argv` | `literal` | `understands-neither` | `37-53` | `argv` | `—` |
| `argv-words` | `orientation` | `source-argv` | `literal` | `understands-neither` | `54-64` | `argv` | `—` |
| `source-channel` | `source-index` | `source-channel` | `root` | `—` | `1-505` | `—` | `channel-production`, `channel-inline-tests` |
| `channel-thesis` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `1-9` | `channel-production` | `—` |
| `channel-production` | `the-channel` | `source-channel` | `composite` | `appearance-is-the-event` | `1-319` | `source-channel` | `channel-thesis`, `channel-prefix`, `channel-nonce-bytes`, `channel-retry-limit`, `channel-type`, `channel-allocate`, `channel-published-path`, `channel-appeared`, `channel-read`, `channel-discard`, `channel-discard-abandoned`, `channel-token`, `channel-signal`, `channel-name-grammar`, `channel-remove-if-present`, `channel-draw-nonce`, `channel-hex` |
| `channel-prefix` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `10-21` | `channel-production` | `—` |
| `channel-nonce-bytes` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `22-25` | `channel-production` | `—` |
| `channel-retry-limit` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `26-32` | `channel-production` | `—` |
| `channel-type` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `33-44` | `channel-production` | `—` |
| `channel-allocate` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `45-99` | `channel-production` | `—` |
| `channel-published-path` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `100-106` | `channel-production` | `—` |
| `channel-appeared` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `107-137` | `channel-production` | `—` |
| `channel-read` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `138-166` | `channel-production` | `—` |
| `channel-discard` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `167-176` | `channel-production` | `—` |
| `channel-discard-abandoned` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `177-232` | `channel-production` | `—` |
| `channel-token` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `233-250` | `channel-production` | `—` |
| `channel-signal` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `251-269` | `channel-production` | `—` |
| `channel-name-grammar` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `270-284` | `channel-production` | `—` |
| `channel-remove-if-present` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `285-295` | `channel-production` | `—` |
| `channel-draw-nonce` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `296-309` | `channel-production` | `—` |
| `channel-hex` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `310-319` | `channel-production` | `—` |
| `channel-tests-module` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `320-322` | `channel-inline-tests` | `—` |
| `channel-inline-tests` | `how-checked` | `source-channel` | `composite` | `checked-without-meaning` | `320-505` | `source-channel` | `channel-tests-module`, `channel-tests-allocate`, `channel-tests-read`, `channel-tests-discard`, `channel-tests-cleanup` |
| `channel-tests-allocate` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `323-360` | `channel-inline-tests` | `—` |
| `channel-tests-read` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `361-420` | `channel-inline-tests` | `—` |
| `channel-untrusted-entry-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `421-439` | `channel-tests-discard` | `—` |
| `channel-tests-discard` | `how-checked` | `source-channel` | `composite` | `checked-without-meaning` | `421-466` | `channel-inline-tests` | `channel-untrusted-entry-test`, `channel-parent-identity-test`, `channel-discard-postcondition-test` |
| `channel-parent-identity-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `440-453` | `channel-tests-discard` | `—` |
| `channel-discard-postcondition-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `454-466` | `channel-tests-discard` | `—` |
| `channel-tests-cleanup` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `467-505` | `channel-inline-tests` | `—` |
| `source-run` | `source-index` | `source-run` | `root` | `—` | `1-1349` | `—` | `launch-shape`, `watch-and-launcher-signals`, `terminal-and-spawn`, `supervise-and-escalate` |
| `run-thesis` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `1-14` | `launch-shape` | `—` |
| `launch-shape` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `1-202` | `source-run` | `run-thesis`, `run-poll-interval`, `run-escalation`, `run-launch`, `run-ended`, `run-group`, `run-end` |
| `run-poll-interval` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `15-22` | `launch-shape` | `—` |
| `run-escalation` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `23-43` | `launch-shape` | `—` |
| `run-launch` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `44-126` | `launch-shape` | `—` |
| `run-ended` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `127-156` | `launch-shape` | `—` |
| `run-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `157-169` | `launch-shape` | `—` |
| `run-end` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `170-202` | `launch-shape` | `—` |
| `run-watch-states` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `203-211` | `watch-and-launcher-signals` | `—` |
| `watch-and-launcher-signals` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `203-391` | `source-run` | `run-watch-states`, `run-interrupted-by`, `run-take-interrupt`, `run-reraise`, `run-on-terminate`, `run-install-termination-handler`, `run-disposition`, `run-sigchld-ignored-at-entry`, `run-restore-child-watching` |
| `run-interrupted-by` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `212-230` | `watch-and-launcher-signals` | `—` |
| `run-take-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `231-249` | `watch-and-launcher-signals` | `—` |
| `run-reraise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `250-292` | `watch-and-launcher-signals` | `—` |
| `run-on-terminate` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `293-299` | `watch-and-launcher-signals` | `—` |
| `run-install-termination-handler` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `300-341` | `watch-and-launcher-signals` | `—` |
| `run-disposition` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `342-352` | `watch-and-launcher-signals` | `—` |
| `run-sigchld-ignored-at-entry` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `353-359` | `watch-and-launcher-signals` | `—` |
| `run-restore-child-watching` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `360-391` | `watch-and-launcher-signals` | `—` |
| `run-default-dispositions` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `392-418` | `terminal-and-spawn` | `—` |
| `terminal-and-spawn` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `392-1004` | `source-run` | `run-default-dispositions`, `run-entry-signals`, `run-terminal-type`, `run-terminal-open`, `run-terminal-accessors`, `run-terminal-hand-to`, `run-terminal-attributes`, `run-own-group`, `run-lease-type`, `run-lease-hold`, `run-lease-lend`, `run-lease-reclaim`, `run-the-child-is-a-job`, `run-command-and-environment`, `run-terminal-handover`, `run-process-group`, `run-pre-exec`, `run-clear-and-spawn`, `run-parent-group-and-supervise` |
| `run-entry-signals` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `419-499` | `terminal-and-spawn` | `—` |
| `run-terminal-type` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `500-503` | `terminal-and-spawn` | `—` |
| `run-terminal-open` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `504-528` | `terminal-and-spawn` | `—` |
| `run-terminal-accessors` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `529-538` | `terminal-and-spawn` | `—` |
| `run-terminal-hand-to` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `539-556` | `terminal-and-spawn` | `—` |
| `run-terminal-attributes` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `557-583` | `terminal-and-spawn` | `—` |
| `run-own-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `584-589` | `terminal-and-spawn` | `—` |
| `run-lease-type` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `590-607` | `terminal-and-spawn` | `—` |
| `run-lease-hold` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `608-620` | `terminal-and-spawn` | `—` |
| `run-lease-lend` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `621-638` | `terminal-and-spawn` | `—` |
| `run-lease-reclaim` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `639-677` | `terminal-and-spawn` | `—` |
| `run-the-child-is-a-job` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `678-719` | `terminal-and-spawn` | `—` |
| `run-output-modes` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `720-782` | `run-command-and-environment` | `—` |
| `run-command-and-environment` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `720-852` | `terminal-and-spawn` | `run-output-modes`, `run-output-setup` |
| `run-output-setup` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `783-852` | `run-command-and-environment` | `—` |
| `run-terminal-handover` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `853-867` | `terminal-and-spawn` | `—` |
| `run-process-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `868-875` | `terminal-and-spawn` | `—` |
| `run-pre-exec` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `876-926` | `terminal-and-spawn` | `—` |
| `run-clear-and-spawn` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `927-939` | `terminal-and-spawn` | `—` |
| `run-spawn-handoff` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `940-966` | `run-parent-group-and-supervise` | `—` |
| `run-parent-group-and-supervise` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `940-1004` | `terminal-and-spawn` | `run-spawn-handoff`, `run-descriptor-bound` |
| `run-descriptor-bound` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `967-1004` | `run-parent-group-and-supervise` | `—` |
| `run-second-kill-pause` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1005-1014` | `supervise-and-escalate` | `—` |
| `supervise-and-escalate` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `1005-1349` | `source-run` | `run-second-kill-pause`, `run-group-confirmation`, `run-killed-poll-interval`, `run-mode`, `run-process-seam`, `run-process-for-child`, `run-confirm-gone`, `run-watched-and-failed`, `run-supervise`, `run-watch-signature`, `run-watch-lend`, `run-watch-forward-interrupt`, `run-watch-exited`, `run-watch-escalation`, `run-watch-end-the-group`, `run-kill` |
| `run-group-confirmation` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1015-1023` | `supervise-and-escalate` | `—` |
| `run-killed-poll-interval` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1024-1028` | `supervise-and-escalate` | `—` |
| `run-mode` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1029-1037` | `supervise-and-escalate` | `—` |
| `run-process-seam` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1038-1057` | `supervise-and-escalate` | `—` |
| `run-process-for-child` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1058-1113` | `supervise-and-escalate` | `—` |
| `run-confirm-gone` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1114-1137` | `supervise-and-escalate` | `—` |
| `run-watched-and-failed` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1138-1151` | `supervise-and-escalate` | `—` |
| `run-supervise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1152-1204` | `supervise-and-escalate` | `—` |
| `run-watch-signature` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1205-1217` | `supervise-and-escalate` | `—` |
| `run-watch-lend` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1218-1220` | `supervise-and-escalate` | `—` |
| `run-watch-forward-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1221-1239` | `supervise-and-escalate` | `—` |
| `run-watch-exited` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1240-1265` | `supervise-and-escalate` | `—` |
| `run-watch-escalation` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1266-1294` | `supervise-and-escalate` | `—` |
| `run-watch-end-the-group` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1295-1316` | `supervise-and-escalate` | `—` |
| `run-kill` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1317-1349` | `supervise-and-escalate` | `—` |
| `source-confinement` | `source-index` | `source-confinement` | `root` | `—` | `1-301` | `—` | `confinement-policy` |
| `held-directory-read` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `1-42` | `confinement-policy` | `—` |
| `confinement-policy` | `confined-jobs` | `source-confinement` | `composite` | `confined-jobs` | `1-301` | `source-confinement` | `held-directory-read`, `confinement-contract`, `confinement-resource-resolution`, `confinement-macos`, `confinement-linux`, `confinement-unavailable` |
| `confinement-contract` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `43-154` | `confinement-policy` | `—` |
| `confinement-resource-resolution` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `155-177` | `confinement-policy` | `—` |
| `confinement-macos` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `178-253` | `confinement-policy` | `—` |
| `confinement-linux` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `254-295` | `confinement-policy` | `—` |
| `confinement-unavailable` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `296-301` | `confinement-policy` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Channel`, `Token`, `signal` | `01-orientation.md#the-cast` | `appearance-is-the-event` | A fresh path per launch that allocation picks and writes nothing to; `signal` is what the child calls to make it appear, and `Token` is what the caller reads back. | `explained` |
| `run`, `run_observed`, `LaunchEvent`, `Launch`, `EntrySignals`, `Ended`, `End`, `Group`, `Escalation` | `01-orientation.md#the-cast` | `nothing-else-added` | `run_observed` reports successful spawn and confirmed reap synchronously; `run` uses a no-op observer. Each spawns one `Launch` — argv, channel, scrub list, granted values, a transparent caller's `EntrySignals` when there is one, working directory and the two graces of an `Escalation` — and returns an `Ended` saying which of `End`'s three cases happened, whether the channel appeared, and whether the child's `Group` was confirmed gone. | `explained` |
| `reraise`, `take_interrupt` | `01-orientation.md#the-cast` | `the-launchers-job` | The launcher's own two obligations for a termination signal: `take_interrupt` collects one that arrived between launches, and `reraise` is how a launcher dies of the same signal rather than reporting an exit code. | `explained` |
| `run_noninteractive`, `run_confined_observed`, `NoninteractiveLaunch`, `FilesystemGrants`, `confinement_available`, `confinement_system_reads`, `regular_file_at` | `01-orientation.md#the-cast` | `confined-jobs` | Detached launches with file or inherited output, mandatory filesystem grants, backend availability and system-read inventory, and stable reads of artifacts through held directories. | `explained` |
| `install_termination_handler`, `restore_child_watching`, `INTERRUPTED_BY`, `Mode`, `supervise` | `03-the-job.md#the-spawn` | `the-launchers-job` | `run`'s first and last acts: the handler that latches a cancelling signal the launcher does not ignore into the process-global `INTERRUPTED_BY`, cleared immediately before each spawn; the repair for an inherited ignored SIGCHLD; and the supervisor that, told the launch's `Mode`, watches the child, ends its group and gives the terminal back. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every source line is credited once to its owning chapter. The 7 roots
contain 2,365 lines, divided below by the manifest's ownership
blocks. `channel.rs` and `run.rs` split across chapters; the remaining roots
are owned whole.

| Slice | Page | Owned lines |
|---|---|---:|
| `understands-neither` | `01-orientation.md` | 214 |
| `appearance-is-the-event` | `02-the-channel.md` | 319 |
| `nothing-else-added` | `03-the-job.md` | 815 |
| `the-launchers-job` | `04-the-escalation.md` | 534 |
| `checked-without-meaning` | `05-how-checked.md` | 186 |
| `assembly` | `06-what-ends-a-launch.md` | 0 |
| `confined-jobs` | `07-confined-jobs.md` | 301 |
| **Total** | 7 source roots | **2,369** |
