# Source index
<!-- book-page id="source-index" role="lookup" -->

[Contents](README.md)

<a id="source-roots"></a>
## Source roots

| Root ID | Source path | Lines |
|---|---|---|
| `source-crate-manifest` | `crates/keyed-launch/Cargo.toml` | 42 |
| `source-library-root` | `crates/keyed-launch/src/lib.rs` | 47 |
| `source-error-type` | `crates/keyed-launch/src/error.rs` | 38 |
| `source-argv` | `crates/keyed-launch/src/argv.rs` | 39 |
| `source-channel` | `crates/keyed-launch/src/channel.rs` | 453 |
| `source-run` | `crates/keyed-launch/src/run.rs` | 852 |
| `source-confinement` | `crates/keyed-launch/src/confinement.rs` | 217 |

<!-- source-root «source-crate-manifest» source="crates/keyed-launch/Cargo.toml" lines="1-42" -->
<!-- insert «manifest-one-dependency» -->
<!-- /source-root -->
<!-- source-root «source-library-root» source="crates/keyed-launch/src/lib.rs" lines="1-47" -->
<!-- insert «library-root» -->
<!-- /source-root -->
<!-- source-root «source-error-type» source="crates/keyed-launch/src/error.rs" lines="1-38" -->
<!-- insert «one-opaque-error» -->
<!-- /source-root -->
<!-- source-root «source-argv» source="crates/keyed-launch/src/argv.rs" lines="1-39" -->
<!-- insert «argv» -->
<!-- /source-root -->
<!-- source-root «source-channel» source="crates/keyed-launch/src/channel.rs" lines="1-453" -->
<!-- insert «channel-production» -->
<!-- insert «channel-inline-tests» -->
<!-- /source-root -->
<!-- source-root «source-run» source="crates/keyed-launch/src/run.rs" lines="1-852" -->
<!-- insert «launch-shape» -->
<!-- insert «watch-and-launcher-signals» -->
<!-- insert «terminal-and-spawn» -->
<!-- insert «supervise-and-escalate» -->
<!-- /source-root -->
<!-- source-root «source-confinement» source="crates/keyed-launch/src/confinement.rs" lines="1-217" -->
<!-- insert «confinement-policy» -->
<!-- /source-root -->

<a id="ownership-blocks"></a>
## Ownership blocks

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-one-dependency` | `source-crate-manifest` | `understands-neither` | `1-42` | 42 | `resolved` |
| `library-root` | `source-library-root` | `understands-neither` | `1-47` | 47 | `resolved` |
| `one-opaque-error` | `source-error-type` | `understands-neither` | `1-38` | 38 | `resolved` |
| `argv` | `source-argv` | `understands-neither` | `1-39` | 39 | `resolved` |
| `channel-production` | `source-channel` | `appearance-is-the-event` | `1-288` | 288 | `resolved` |
| `channel-inline-tests` | `source-channel` | `checked-without-meaning` | `289-453` | 165 | `resolved` |
| `launch-shape` | `source-run` | `nothing-else-added` | `1-123` | 123 | `resolved` |
| `watch-and-launcher-signals` | `source-run` | `the-launchers-job` | `124-243` | 120 | `resolved` |
| `terminal-and-spawn` | `source-run` | `nothing-else-added` | `244-678` | 435 | `resolved` |
| `supervise-and-escalate` | `source-run` | `the-launchers-job` | `679-852` | 174 | `resolved` |
| `confinement-policy` | `source-confinement` | `confined-jobs` | `1-217` | 217 | `resolved` |

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
| `source-library-root` | `source-index` | `source-library-root` | `root` | `—` | `1-47` | `—` | `library-root` |
| `library-root-thesis` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `1-8` | `library-root` | `—` |
| `library-root` | `orientation` | `source-library-root` | `composite` | `understands-neither` | `1-47` | `source-library-root` | `library-root-thesis`, `library-root-to-a-child`, `library-root-job-and-out-of-band`, `library-root-observed`, `library-root-modules-and-exports` |
| `library-root-to-a-child` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `9-14` | `library-root` | `—` |
| `library-root-job-and-out-of-band` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `15-27` | `library-root` | `—` |
| `library-root-observed` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `28-32` | `library-root` | `—` |
| `library-root-modules-and-exports` | `orientation` | `source-library-root` | `literal` | `understands-neither` | `33-47` | `library-root` | `—` |
| `source-error-type` | `source-index` | `source-error-type` | `root` | `—` | `1-38` | `—` | `one-opaque-error` |
| `error-import` | `orientation` | `source-error-type` | `literal` | `understands-neither` | `1-1` | `one-opaque-error` | `—` |
| `one-opaque-error` | `orientation` | `source-error-type` | `composite` | `understands-neither` | `1-38` | `source-error-type` | `error-import`, `error-launch-type`, `error-launch-traits` |
| `error-launch-type` | `orientation` | `source-error-type` | `literal` | `understands-neither` | `2-22` | `one-opaque-error` | `—` |
| `error-launch-traits` | `orientation` | `source-error-type` | `literal` | `understands-neither` | `23-38` | `one-opaque-error` | `—` |
| `source-argv` | `source-index` | `source-argv` | `root` | `—` | `1-39` | `—` | `argv` |
| `argv-type` | `orientation` | `source-argv` | `literal` | `understands-neither` | `1-11` | `argv` | `—` |
| `argv` | `orientation` | `source-argv` | `composite` | `understands-neither` | `1-39` | `source-argv` | `argv-type`, `argv-public-constructor`, `argv-program-and-args`, `argv-words` |
| `argv-public-constructor` | `orientation` | `source-argv` | `literal` | `understands-neither` | `12-18` | `argv` | `—` |
| `argv-program-and-args` | `orientation` | `source-argv` | `literal` | `understands-neither` | `19-28` | `argv` | `—` |
| `argv-words` | `orientation` | `source-argv` | `literal` | `understands-neither` | `29-39` | `argv` | `—` |
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
| `source-run` | `source-index` | `source-run` | `root` | `—` | `1-852` | `—` | `launch-shape`, `watch-and-launcher-signals`, `terminal-and-spawn`, `supervise-and-escalate` |
| `run-thesis` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `1-14` | `launch-shape` | `—` |
| `launch-shape` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `1-123` | `source-run` | `run-thesis`, `run-poll-interval`, `run-escalation`, `run-launch`, `run-ended`, `run-end` |
| `run-poll-interval` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `15-22` | `launch-shape` | `—` |
| `run-escalation` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `23-43` | `launch-shape` | `—` |
| `run-launch` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `44-77` | `launch-shape` | `—` |
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
| `terminal-and-spawn` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `244-678` | `source-run` | `run-default-dispositions`, `run-terminal-type`, `run-terminal-open`, `run-terminal-accessors`, `run-terminal-hand-to`, `run-own-group`, `run-the-child-is-a-job`, `run-command-and-environment`, `run-terminal-handover`, `run-process-group`, `run-pre-exec`, `run-clear-and-spawn`, `run-parent-group-and-supervise` |
| `run-terminal-type` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `271-274` | `terminal-and-spawn` | `—` |
| `run-terminal-open` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `275-299` | `terminal-and-spawn` | `—` |
| `run-terminal-accessors` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `300-309` | `terminal-and-spawn` | `—` |
| `run-terminal-hand-to` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `310-328` | `terminal-and-spawn` | `—` |
| `run-own-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `329-334` | `terminal-and-spawn` | `—` |
| `run-the-child-is-a-job` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `335-366` | `terminal-and-spawn` | `—` |
| `run-output-modes` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `367-433` | `run-command-and-environment` | `—` |
| `run-command-and-environment` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `367-472` | `terminal-and-spawn` | `run-output-modes`, `run-output-setup` |
| `run-output-setup` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `434-472` | `run-command-and-environment` | `—` |
| `run-terminal-handover` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `473-484` | `terminal-and-spawn` | `—` |
| `run-process-group` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `485-492` | `terminal-and-spawn` | `—` |
| `run-pre-exec` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `493-532` | `terminal-and-spawn` | `—` |
| `run-clear-and-spawn` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `533-544` | `terminal-and-spawn` | `—` |
| `run-spawn-handoff` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `545-575` | `run-parent-group-and-supervise` | `—` |
| `run-parent-group-and-supervise` | `the-job` | `source-run` | `composite` | `nothing-else-added` | `545-678` | `terminal-and-spawn` | `run-spawn-handoff`, `run-descriptor-bound`, `run-group-drain`, `run-process-adapter` |
| `run-descriptor-bound` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `576-613` | `run-parent-group-and-supervise` | `—` |
| `run-group-drain` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `614-636` | `run-parent-group-and-supervise` | `—` |
| `run-process-adapter` | `the-job` | `source-run` | `literal` | `nothing-else-added` | `637-678` | `run-parent-group-and-supervise` | `—` |
| `run-supervise` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `679-705` | `supervise-and-escalate` | `—` |
| `supervise-and-escalate` | `the-escalation` | `source-run` | `composite` | `the-launchers-job` | `679-852` | `source-run` | `run-supervise`, `run-watch-signature`, `run-watch-ended`, `run-watch-terminal-recheck`, `run-watch-forward-interrupt`, `run-watch-try-wait`, `run-watch-escalation`, `run-kill` |
| `run-watch-signature` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `706-719` | `supervise-and-escalate` | `—` |
| `run-watch-ended` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `720-732` | `supervise-and-escalate` | `—` |
| `run-watch-terminal-recheck` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `733-744` | `supervise-and-escalate` | `—` |
| `run-watch-forward-interrupt` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `745-758` | `supervise-and-escalate` | `—` |
| `run-watch-try-wait` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `759-790` | `supervise-and-escalate` | `—` |
| `run-watch-escalation` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `791-821` | `supervise-and-escalate` | `—` |
| `run-kill` | `the-escalation` | `source-run` | `literal` | `the-launchers-job` | `822-852` | `supervise-and-escalate` | `—` |
| `source-confinement` | `source-index` | `source-confinement` | `root` | `—` | `1-217` | `—` | `confinement-policy` |
| `held-directory-read` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `1-44` | `confinement-policy` | `—` |
| `confinement-policy` | `confined-jobs` | `source-confinement` | `composite` | `confined-jobs` | `1-217` | `source-confinement` | `held-directory-read`, `confinement-contract`, `confinement-resource-resolution`, `confinement-macos`, `confinement-linux`, `confinement-unavailable` |
| `confinement-contract` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `45-78` | `confinement-policy` | `—` |
| `confinement-resource-resolution` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `79-101` | `confinement-policy` | `—` |
| `confinement-macos` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `102-153` | `confinement-policy` | `—` |
| `confinement-linux` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `154-211` | `confinement-policy` | `—` |
| `confinement-unavailable` | `confined-jobs` | `source-confinement` | `literal` | `confined-jobs` | `212-217` | `confinement-policy` | `—` |

<a id="early-uses"></a>
## Early uses

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Channel`, `Token`, `signal` | `01-orientation.md#the-cast` | `appearance-is-the-event` | A fresh path per launch that allocation picks and writes nothing to; `signal` is what the child calls to make it appear, and `Token` is what the caller reads back. | `explained` |
| `run`, `run_observed`, `LaunchEvent`, `Launch`, `Ended`, `End`, `Escalation` | `01-orientation.md#the-cast` | `nothing-else-added` | `run_observed` reports successful spawn and confirmed reap synchronously; `run` uses a no-op observer. Each spawns one `Launch` — argv, channel, scrub list, working directory and the two graces of an `Escalation` — and returns an `Ended` saying which of `End`'s three cases happened. | `explained` |
| `reraise`, `take_interrupt` | `01-orientation.md#the-cast` | `the-launchers-job` | The launcher's own two obligations for a termination signal: `take_interrupt` collects one that arrived between launches, and `reraise` is how a launcher dies of the same signal rather than reporting an exit code. | `explained` |
| `run_noninteractive`, `run_confined`, `Confinement`, `regular_file_at` | `01-orientation.md#the-cast` | `confined-jobs` | A launch with no terminal whose output goes to a file, the same launch under a mandatory filesystem policy, that policy's two fields, and the one read of a result that policy leaves safe. | `explained` |
| `install_termination_handler`, `INTERRUPTED_BY`, `supervise` | `03-the-job.md#the-spawn` | `the-launchers-job` | `run`'s first and last acts: the handler that latches the launcher's own SIGTERM or SIGHUP into the process-global `INTERRUPTED_BY`, cleared immediately before each spawn, and the supervisor that watches the child and takes the terminal back. | `explained` |

<a id="owned-source-totals"></a>
## Owned source totals

Every source line is credited once to its owning chapter. The 7 roots
contain 1,688 lines, divided below by the manifest's ownership
blocks. `channel.rs` and `run.rs` split across chapters; the remaining roots
are owned whole.

| Slice | Page | Owned lines |
|---|---|---:|
| `understands-neither` | `01-orientation.md` | 166 |
| `appearance-is-the-event` | `02-the-channel.md` | 288 |
| `nothing-else-added` | `03-the-job.md` | 558 |
| `the-launchers-job` | `04-the-escalation.md` | 294 |
| `checked-without-meaning` | `05-how-checked.md` | 165 |
| `assembly` | `06-what-ends-a-launch.md` | 0 |
| `confined-jobs` | `07-confined-jobs.md` | 217 |
| **Total** | 7 source roots | **1,688** |
