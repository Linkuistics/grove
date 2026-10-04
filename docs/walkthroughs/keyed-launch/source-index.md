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
| `source-channel` | `crates/keyed-launch/src/channel.rs` | 429 |
| `source-run` | `crates/keyed-launch/src/run.rs` | 1,340 |
| `source-confinement` | `crates/keyed-launch/src/confinement.rs` | 326 |

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
<!-- source-root «source-channel» source="crates/keyed-launch/src/channel.rs" lines="1-429" -->
<!-- insert «channel-production» -->
<!-- insert «channel-inline-tests» -->
<!-- /source-root -->
<!-- source-root «source-run» source="crates/keyed-launch/src/run.rs" lines="1-1340" -->
<!-- insert «launch-shape» -->
<!-- insert «watch-and-launcher-signals» -->
<!-- insert «terminal-and-spawn» -->
<!-- insert «supervise-and-escalate» -->
<!-- /source-root -->
<!-- source-root «source-confinement» source="crates/keyed-launch/src/confinement.rs" lines="1-326" -->
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
| `channel-production` | `source-channel` | `appearance-is-the-event` | `1-268` | 268 | `resolved` |
| `channel-inline-tests` | `source-channel` | `checked-without-meaning` | `269-429` | 161 | `resolved` |
| `launch-shape` | `source-run` | `nothing-else-added` | `1-196` | 196 | `resolved` |
| `watch-and-launcher-signals` | `source-run` | `the-launchers-job` | `197-385` | 189 | `resolved` |
| `terminal-and-spawn` | `source-run` | `nothing-else-added` | `386-998` | 613 | `resolved` |
| `supervise-and-escalate` | `source-run` | `the-launchers-job` | `999-1340` | 342 | `resolved` |
| `confinement-policy` | `source-confinement` | `confined-jobs` | `1-326` | 326 | `resolved` |

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
| `source-channel` | `source-index` | `source-channel` | `root` | `—` | `1-429` | `—` | `channel-production`, `channel-inline-tests` |
| `channel-thesis` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `1-9` | `channel-production` | `—` |
| `channel-production` | `the-channel` | `source-channel` | `composite` | `appearance-is-the-event` | `1-268` | `source-channel` | `channel-thesis`, `channel-prefix`, `channel-nonce-bytes`, `channel-retry-limit`, `channel-type`, `channel-allocate`, `channel-published-path`, `channel-appeared`, `channel-discard`, `channel-discard-abandoned`, `channel-signal`, `channel-name-grammar`, `channel-remove-if-present`, `channel-draw-nonce`, `channel-hex` |
| `channel-prefix` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `10-21` | `channel-production` | `—` |
| `channel-nonce-bytes` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `22-25` | `channel-production` | `—` |
| `channel-retry-limit` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `26-32` | `channel-production` | `—` |
| `channel-type` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `33-44` | `channel-production` | `—` |
| `channel-allocate` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `45-99` | `channel-production` | `—` |
| `channel-published-path` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `100-106` | `channel-production` | `—` |
| `channel-appeared` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `107-136` | `channel-production` | `—` |
| `channel-discard` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `137-146` | `channel-production` | `—` |
| `channel-discard-abandoned` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `147-202` | `channel-production` | `—` |
| `channel-signal` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `203-218` | `channel-production` | `—` |
| `channel-name-grammar` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `219-233` | `channel-production` | `—` |
| `channel-remove-if-present` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `234-244` | `channel-production` | `—` |
| `channel-draw-nonce` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `245-258` | `channel-production` | `—` |
| `channel-hex` | `the-channel` | `source-channel` | `literal` | `appearance-is-the-event` | `259-268` | `channel-production` | `—` |
| `channel-tests-module` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `269-271` | `channel-inline-tests` | `—` |
| `channel-inline-tests` | `how-checked` | `source-channel` | `composite` | `checked-without-meaning` | `269-429` | `source-channel` | `channel-tests-module`, `channel-tests-allocate`, `channel-tests-read`, `channel-tests-discard`, `channel-tests-cleanup` |
| `channel-tests-allocate` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `272-309` | `channel-inline-tests` | `—` |
| `channel-tests-read` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `310-340` | `channel-inline-tests` | `—` |
| `channel-untrusted-entry-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `341-360` | `channel-tests-discard` | `—` |
| `channel-tests-discard` | `how-checked` | `source-channel` | `composite` | `checked-without-meaning` | `341-390` | `channel-inline-tests` | `channel-untrusted-entry-test`, `channel-parent-identity-test`, `channel-discard-postcondition-test` |
| `channel-parent-identity-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `361-377` | `channel-tests-discard` | `—` |
| `channel-discard-postcondition-test` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `378-390` | `channel-tests-discard` | `—` |
| `channel-tests-cleanup` | `how-checked` | `source-channel` | `literal` | `checked-without-meaning` | `391-429` | `channel-inline-tests` | `—` |
| `source-run` | `source-index` | `source-run` | `root` | `—` | `1-1340` | `—` | `launch-shape`, `watch-and-launcher-signals`, `terminal-and-spawn`, `supervise-and-escalate` |
| `run-thesis` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `1-14` | `launch-shape` | `—` |
| `launch-shape` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `1-196` | `source-run` | `run-thesis`, `run-poll-interval`, `run-escalation`, `run-launch`, `run-ended`, `run-group`, `run-end` |
| `run-poll-interval` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `15-22` | `launch-shape` | `—` |
| `run-escalation` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `23-43` | `launch-shape` | `—` |
| `run-launch` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `44-122` | `launch-shape` | `—` |
| `run-ended` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `123-150` | `launch-shape` | `—` |
| `run-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `151-163` | `launch-shape` | `—` |
| `run-end` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `164-196` | `launch-shape` | `—` |
| `run-watch-states` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `197-205` | `watch-and-launcher-signals` | `—` |
| `watch-and-launcher-signals` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `197-385` | `source-run` | `run-watch-states`, `run-interrupted-by`, `run-take-interrupt`, `run-reraise`, `run-on-terminate`, `run-install-termination-handler`, `run-disposition`, `run-sigchld-ignored-at-entry`, `run-restore-child-watching` |
| `run-interrupted-by` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `206-224` | `watch-and-launcher-signals` | `—` |
| `run-take-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `225-243` | `watch-and-launcher-signals` | `—` |
| `run-reraise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `244-286` | `watch-and-launcher-signals` | `—` |
| `run-on-terminate` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `287-293` | `watch-and-launcher-signals` | `—` |
| `run-install-termination-handler` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `294-335` | `watch-and-launcher-signals` | `—` |
| `run-disposition` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `336-346` | `watch-and-launcher-signals` | `—` |
| `run-sigchld-ignored-at-entry` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `347-353` | `watch-and-launcher-signals` | `—` |
| `run-restore-child-watching` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `354-385` | `watch-and-launcher-signals` | `—` |
| `run-default-dispositions` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `386-412` | `terminal-and-spawn` | `—` |
| `terminal-and-spawn` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `386-998` | `source-run` | `run-default-dispositions`, `run-entry-signals`, `run-terminal-type`, `run-terminal-open`, `run-terminal-accessors`, `run-terminal-hand-to`, `run-terminal-attributes`, `run-own-group`, `run-lease-type`, `run-lease-hold`, `run-lease-lend`, `run-lease-reclaim`, `run-the-child-is-a-job`, `run-command-and-environment`, `run-terminal-handover`, `run-process-group`, `run-pre-exec`, `run-clear-and-spawn`, `run-parent-group-and-supervise` |
| `run-entry-signals` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `413-493` | `terminal-and-spawn` | `—` |
| `run-terminal-type` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `494-497` | `terminal-and-spawn` | `—` |
| `run-terminal-open` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `498-522` | `terminal-and-spawn` | `—` |
| `run-terminal-accessors` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `523-532` | `terminal-and-spawn` | `—` |
| `run-terminal-hand-to` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `533-550` | `terminal-and-spawn` | `—` |
| `run-terminal-attributes` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `551-577` | `terminal-and-spawn` | `—` |
| `run-own-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `578-583` | `terminal-and-spawn` | `—` |
| `run-lease-type` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `584-601` | `terminal-and-spawn` | `—` |
| `run-lease-hold` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `602-614` | `terminal-and-spawn` | `—` |
| `run-lease-lend` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `615-632` | `terminal-and-spawn` | `—` |
| `run-lease-reclaim` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `633-671` | `terminal-and-spawn` | `—` |
| `run-the-child-is-a-job` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `672-713` | `terminal-and-spawn` | `—` |
| `run-output-modes` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `714-776` | `run-command-and-environment` | `—` |
| `run-command-and-environment` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `714-846` | `terminal-and-spawn` | `run-output-modes`, `run-output-setup` |
| `run-output-setup` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `777-846` | `run-command-and-environment` | `—` |
| `run-terminal-handover` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `847-861` | `terminal-and-spawn` | `—` |
| `run-process-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `862-869` | `terminal-and-spawn` | `—` |
| `run-pre-exec` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `870-920` | `terminal-and-spawn` | `—` |
| `run-clear-and-spawn` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `921-933` | `terminal-and-spawn` | `—` |
| `run-spawn-handoff` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `934-960` | `run-parent-group-and-supervise` | `—` |
| `run-parent-group-and-supervise` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `934-998` | `terminal-and-spawn` | `run-spawn-handoff`, `run-descriptor-bound` |
| `run-descriptor-bound` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `961-998` | `run-parent-group-and-supervise` | `—` |
| `run-second-kill-pause` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `999-1008` | `supervise-and-escalate` | `—` |
| `supervise-and-escalate` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `999-1340` | `source-run` | `run-second-kill-pause`, `run-group-confirmation`, `run-killed-poll-interval`, `run-mode`, `run-process-seam`, `run-process-for-child`, `run-confirm-gone`, `run-watched-and-failed`, `run-supervise`, `run-watch-signature`, `run-watch-lend`, `run-watch-forward-interrupt`, `run-watch-exited`, `run-watch-escalation`, `run-watch-end-the-group`, `run-kill` |
| `run-group-confirmation` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1009-1017` | `supervise-and-escalate` | `—` |
| `run-killed-poll-interval` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1018-1022` | `supervise-and-escalate` | `—` |
| `run-mode` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1023-1031` | `supervise-and-escalate` | `—` |
| `run-process-seam` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1032-1051` | `supervise-and-escalate` | `—` |
| `run-process-for-child` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1052-1107` | `supervise-and-escalate` | `—` |
| `run-confirm-gone` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1108-1131` | `supervise-and-escalate` | `—` |
| `run-watched-and-failed` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1132-1145` | `supervise-and-escalate` | `—` |
| `run-supervise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1146-1195` | `supervise-and-escalate` | `—` |
| `run-watch-signature` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1196-1208` | `supervise-and-escalate` | `—` |
| `run-watch-lend` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1209-1211` | `supervise-and-escalate` | `—` |
| `run-watch-forward-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1212-1230` | `supervise-and-escalate` | `—` |
| `run-watch-exited` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1231-1256` | `supervise-and-escalate` | `—` |
| `run-watch-escalation` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1257-1285` | `supervise-and-escalate` | `—` |
| `run-watch-end-the-group` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1286-1307` | `supervise-and-escalate` | `—` |
| `run-kill` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `1308-1340` | `supervise-and-escalate` | `—` |
| `source-confinement` | `source-index` | `source-confinement` | `root` | `—` | `1-326` | `—` | `confinement-policy` |
| `held-directory-read` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `1-42` | `confinement-policy` | `—` |
| `confinement-policy` | `confined-jobs` | `source-confinement` | `composite` | `confined-jobs` | `1-326` | `source-confinement` | `held-directory-read`, `confinement-contract`, `confinement-resource-resolution`, `confinement-macos`, `confinement-linux`, `confinement-unavailable` |
| `confinement-contract` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `43-179` | `confinement-policy` | `—` |
| `confinement-resource-resolution` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `180-202` | `confinement-policy` | `—` |
| `confinement-macos` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `203-278` | `confinement-policy` | `—` |
| `confinement-linux` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `279-320` | `confinement-policy` | `—` |
| `confinement-unavailable` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `321-326` | `confinement-policy` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Channel`, `signal` | `01-orientation.md#the-cast` | `appearance-is-the-event` | A fresh path per launch that allocation picks and writes nothing to; `signal` is what the child calls to make it appear, and the runner reports appearance through `Ended.signalled`. | `explained` |
| `run`, `run_observed`, `LaunchEvent`, `Launch`, `EntrySignals`, `Ended`, `End`, `Group`, `Escalation` | `01-orientation.md#the-cast` | `nothing-else-added` | `run_observed` reports successful spawn and confirmed reap synchronously; `run` uses a no-op observer. Each spawns one `Launch` — argv, channel, scrub list, granted values, a transparent caller's `EntrySignals` when there is one, working directory and the two graces of an `Escalation` — and returns an `Ended` saying which of `End`'s three cases happened, whether the channel appeared, and whether the child's `Group` was confirmed gone. | `explained` |
| `reraise`, `take_interrupt` | `01-orientation.md#the-cast` | `the-launchers-job` | The launcher's own two obligations for a termination signal: `take_interrupt` collects one that arrived between launches, and `reraise` is how a launcher dies of the same signal rather than reporting an exit code. | `explained` |
| `run_noninteractive`, `run_confined_observed`, `NoninteractiveLaunch`, `FilesystemGrants`, `confinement_available`, `confinement_system_reads`, `regular_file_at` | `01-orientation.md#the-cast` | `confined-jobs` | Detached launches with file or inherited output, mandatory filesystem grants, backend availability and system-read inventory, and stable reads of artifacts through held directories. | `explained` |
| `install_termination_handler`, `restore_child_watching`, `INTERRUPTED_BY`, `Mode`, `supervise` | `03-the-job.md#the-spawn` | `the-launchers-job` | `run`'s first and last acts: the handler that latches a cancelling signal the launcher does not ignore into the process-global `INTERRUPTED_BY`, cleared immediately before each spawn; the repair for an inherited ignored SIGCHLD; and the supervisor that, told the launch's `Mode`, watches the child, ends its group and gives the terminal back. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every source line is credited once to its owning chapter. The 7 roots
contain 2,309 lines, divided below by the manifest's ownership
blocks. `channel.rs` and `run.rs` split across chapters; the remaining roots
are owned whole.

| Slice | Page | Owned lines |
|---|---|---:|
| `understands-neither` | `01-orientation.md` | 214 |
| `appearance-is-the-event` | `02-the-channel.md` | 268 |
| `nothing-else-added` | `03-the-job.md` | 809 |
| `the-launchers-job` | `04-the-escalation.md` | 531 |
| `checked-without-meaning` | `05-how-checked.md` | 161 |
| `assembly` | `06-what-ends-a-launch.md` | 0 |
| `confined-jobs` | `07-confined-jobs.md` | 326 |
| **Total** | 7 source roots | **2,309** |
