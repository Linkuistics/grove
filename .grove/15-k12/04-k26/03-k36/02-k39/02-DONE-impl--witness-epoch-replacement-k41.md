# witness-epoch-replacement-k41


## Goal
Disprove early witness preparation while old epoch readers delay replacement.



## Context
Consume k40 concurrency controls, k38 forced-reuse release fixture and the
existing DriverLease acquire_with/prepare_launch_using seams. The required
mutation is independent of k38's directory-probe mutation.

## Done when
- A readiness/lock barrier holds an old epoch reader while replacement owns
  the lease. Old lease bytes remain intact and neither new witness is prepared
  before exclusive invalidation. Concurrent observers of released Started
  bytes remain Idle and cannot create contention.
- Model directory-first release with reused numeric identity/key as necessary
  to expose a new directory witness under the old epoch. Independently move
  preparation before epoch acquisition/invalidation in production code and
  observe the specified false attachment. The real Running positive still
  passes. Restore and rerun, preserving per-file source digests and per-case
  commands/results before and after each run.
- Preserve bounded handoff/recovery, no returned locks and read-only snapshots;
  update affected books/docs and pass focused tests and the principal gate.

## Notes
No sleeps as ordering proof, no fake public provider, no assertion of host
inode reuse. k42 reconciles k39/k36; k37 owns native platform evidence and k27
owns complete review. No competing in-session reviewer.

## Decisions (running log)

Use two complementary controls: actual replacement acquisition paused by a
readiness channel while a shared old epoch lock is held, and the existing
preparation acquisition seam with forced numeric/key reuse. The latter models
the old-record window at preparation without weakening initial acquisition.
Move only the production `launch.prepare` block ahead of epoch acquisition
for the independent mutation. Public concurrent observers supply the verdicts;
native probes and byte snapshots establish ordering and read-only behavior.

### Mutation and restoration evidence — 2026-09-16

Base revision: `e6f77952` (k40), plus this leaf's diff. Host: Darwin 25.6.0
arm64; Homebrew rustc/cargo 1.98.1. These are native-lock deterministic
interleaving controls; k37 owns cross-platform process-death evidence.

The frozen source manifest is [Source manifest](#source-manifest).
It contains each regular file recursively under `crates/` and `.cargo/`, plus
`Cargo.toml` and `Cargo.lock`. Every run compared each subject's SHA256 before
and after; restoration compared every subject to baseline. The mutant differs
only in `crates/grove-loop/src/driver_lease.rs`: move the entire existing
`if let Some(launch) ... launch.prepare` block from after inactive-epoch writing
and cleanup to immediately before `let mut epoch = acquire(...)`. The directory
probe remains enabled. Tests and public observer are unchanged during mutation.

| Subject | SHA256 |
|---|---|
| Source manifest payload | `252be6b87ae57a16b517683a0cd50f4ef44cf3f5c0bfa68036938190c1e20e1a` |
| Baseline/restored lease | `19900ae2e3df20d951ed49d451b094e61ecc2747233a8825b33a00809045783a` |
| Mutated lease | `03cbbbb77ea58ed3f816affd9b748335dafa377fa7f68050618b370de7ef0e3f` |
| Observer including frozen tests | `8d1f0f1a717b19ba447e2508f8a8ae2d11c91a5c90ad8db580fc07b79a3a4410` |

All commands use `cargo test --locked -p grove-loop --lib <filter> -- --nocapture`.
The recorded exits and case results below apply both before and after each
run's unchanged-source comparison.

| Run | Filter | Exit / result |
|---|---|---|
| baseline | `witness_epoch_` | 0; test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 296 filtered out; finished in 0.01s |
| mutant-reuse | `witness_epoch_preparation_cannot_attach_old_mandate_to_reused_tree` | 101; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 297 filtered out; finished in 0.01s |
| mutant-positive | `witness_real_launch_started_and_reaped_reach_public_observation` | 0; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 297 filtered out; finished in 0.51s |
| restored-witnesses | `witness_` | 0; test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 276 filtered out; finished in 1.28s |
| restored-handoff | `paused_runtime_reader_allows_shared_observers_and_bounds_driver_handoff` | 0; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 297 filtered out; finished in 0.00s |
| restored-guards-exact | `capture_pause_and_returned_values_hold_no_epoch_or_tree_lock` | 0; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 297 filtered out; finished in 0.00s |

The mutant failed on the intended assertion, displaying old `work-k1` as
`Running`, relation `SameTree`, on the captured `replacement-k1` tree. The real
launch positive still passed. Restoring production made the new preparation
control return Busy before invalidation, acquire both witnesses afterwards,
and return Idle with no escaped locks after release. The actual replacement
control kept old bytes intact while three viewers remained Idle, then completed
handoff and cleanup after reader release. Snapshots were unchanged during
observation. These tests use readiness channels, lock results and bounded failure
timeouts, never sleeps as ordering proof.

The first development run found a fixture-only canonical-path mismatch in a
snapshot lookup; direct saved epoch bytes fixed it before the frozen runs.
A mistyped guard-test filter selected zero tests and supplies no evidence;
`restored-guards-exact` above names and runs the actual existing control.

Restored witness results (each selected case, all passed):

- `driver_lease::observation::tests::witness_private_open_and_identity_io_errors_release_the_epoch_guard`
- `driver_lease::observation::tests::witness_capture_uses_accepted_pin_after_root_path_replacement`
- `driver_lease::observation::tests::witness_started_public_observation_identifies_the_prepared_launch`
- `driver_lease::observation::tests::witness_release_private_first_and_both_released_are_idle`
- `driver_lease::observation::tests::witness_directory_private_and_marker_error_matrix_obeys_final_probe`
- `driver_lease::observation::tests::witness_release_after_directory_verification_obeys_final_private_probe`
- `driver_lease::observation::tests::witness_missing_replaced_and_nonregular_evidence_is_unavailable`
- `driver_lease::observation::tests::witness_released_active_epoch_is_idle_despite_leftover_bytes`
- `driver_lease::observation::tests::witness_marker_prefixes_invalid_bytes_and_release_have_exact_precedence`
- `driver_lease::observation::tests::witness_epoch_preparation_cannot_attach_old_mandate_to_reused_tree`
- `driver_lease::observation::tests::witness_tree_absence_failure_and_contention_preserve_runtime_identity`
- `driver_lease::tests::paired_witness_lease_drop_releases_both_before_replacement_cleanup`
- `driver_lease::observation::tests::witness_unlocked_matching_directory_is_busy_and_probes_escape_no_locks`
- `driver_lease::observation::tests::witness_release_directory_first_reused_identity_and_key_cannot_attach`
- `driver_lease::observation::tests::witness_epoch_replacement_waits_with_old_bytes_and_idle_viewers`
- `driver_lease::observation::tests::witness_real_launch_started_and_reaped_reach_public_observation`
- `driver_lease::observation::tests::witness_concurrent_shared_probes_of_released_started_bytes_stay_idle`
- `driver_lease::observation::tests::witness_open_and_probe_replacements_reject_stale_descriptors_and_bound_retries`
- `driver_lease::observation::tests::witness_concurrent_read_only_captures_cover_aliases_and_absent_controls`
- `driver_lease::observation::tests::witness_extension_validation_cannot_be_replaced_by_blanket_unavailability`
- `driver_lease::tests::paired_witness_failure_preserves_real_launch_and_admission`
- `driver_lease::observation::tests::witness_continuous_viewers_allow_repeated_real_launch_preparation`

The source-exact book adds separate delayed-replacement and preparation-order
fragments, with source ranges, concept lookup and totals reconciled. Final book
validation reports 16 roots / 13,910 lines / no deferred bytes. The architecture
summary states the model boundary. The ADR's ordering and native-lock contract
are unchanged. k42 still owns foreign holders and k39/k36 reconciliation; k27
owns complete review, so no in-session reviewer or parent closure is due.

Final verification: `bash scripts/check.sh` exited 0. All eight principal checks
passed, including 298 grove-loop unit tests, workspace integration/doc tests,
and final validation of all six books. All 159 frozen source subjects still
match the restored manifest after the gate. Principal output SHA256:
`d0b009d66a54ee0f4335046b2cff78ee49a2fb658a8c03690ee789a986f3cea1`.

k41's close conditions hold. k42 remains live, so k39 and k36 remain open and
no parent-chain promotion or ADR revision is due.

## Source manifest

Baseline/restored SHA256 manifest. The task records the hash of the fenced
payload, including its final newline. The mutant changes only the lease digest.

```text
19e63e397ae526e4960f53b19514a639ac80f5220da19fe7877f7868497346a1  .cargo/config.toml
225be3cfb9f8ae445592cd7586a24175272305b4cf3a843b777549c8ce52a506  Cargo.lock
842c4df2b0aecf3d2ece280bc193eb2b34d2aa3a4e5d52259a8b773b9c0c0b62  Cargo.toml
afbabe2d78ac72c76627886f82e87d2d33d507a94208f396810572d032893af1  crates/book-validation/Cargo.toml
918e9773c8fdd77dbc232c01f10a585e9b4cf02f0ec1da05444947a32d1ea890  crates/book-validation/README.md
7f1e9fe21dc0227fd80f6ab8292c17ab53a8986493e6c888f954c6ba9b8ec216  crates/book-validation/src/cli.rs
3f82d845e512eb65aed1b70e97c25728005fcfb447685dc7fb900fd88faf5156  crates/book-validation/src/corpus.rs
cdc29da1d736fcfa8e7bb7c16b622c7a64740174c79bbe9a516e661d2f331078  crates/book-validation/src/ledger.rs
687cabc955abeda130ece8739931cbfe6358fa0ac7d933f5fedc6068007b45a3  crates/book-validation/src/lib.rs
c238d03408af019e01039289c6dd2dd1cfde724d057d5484c2c17d739bf99c1d  crates/book-validation/src/main.rs
663ce23bba9e0781eaaded8e4927679d2150211131326e7136d7bd674e0b2d2f  crates/book-validation/src/manifest.rs
3bc27e0082caee39e3ba9b093bb8afd4ffb16d1cbc3e863826a45ade1e1e9058  crates/book-validation/src/markdown.rs
50d1efd6f736a68bf93eb87b8443fcbeeb46499b6de32f7e2234ca43f33656cb  crates/book-validation/src/parser.rs
8bc7b9befd837a341e9e35822d39bb5eebd69d0381e63597d5671e785b9dd32d  crates/book-validation/src/validator.rs
36116baa68b3ccb0223de4e9325433150bf99314d3c73cdc3e53458db006287c  crates/book-validation/tests/cli.rs
e9c1d90297257f4960f0047652e9e718a27d0c1a3ddc5d18d929430bf0b5ad37  crates/book-validation/tests/corpus_validation.rs
980d5e9cf8ea248fda85b74508c5abe7cf8ec2437a86d30499ff09fbb57f3749  crates/book-validation/tests/diagnostic_contract.rs
47f278da6c60de6557cd8913b951df8d140770d7d6a67cf5406074d50eb8c9d4  crates/book-validation/tests/fragment_failures.rs
4c1cb982be17dadcb2c78088118beedf204bf07ac1cd354c42bada46fc1a6bb6  crates/book-validation/tests/ledger_and_pages.rs
67112431afe818305512ddc4a57cba5a97c22ea457ab92f3f1f08b4640b9e02c  crates/book-validation/tests/lexer.rs
197bafd33b0806ef6f28d00985c8e63e528bbc51dd11345f5176f4116c0ee7d3  crates/book-validation/tests/manifest.rs
34f3b9100f8cfa9c5812c4dafb095b46f9f827ceab8e54e0d93af603f6dd0b2f  crates/book-validation/tests/markdown_validation.rs
d3290e02b19255f18e081ef688eee7a53afb26c09ab8b285b3ccff848d1be0bd  crates/book-validation/tests/rollups.rs
2fdc9845a014c12cd459bb12cc217e2148e17375bcb2d4720428689e0f17d25d  crates/book-validation/tests/scoped_slice.rs
774d306e1d35907f4aa6577fbb4f05b304472b64d5400dae3ce2c1d05f38f80a  crates/book-validation/tests/second_book.rs
5537f1a27f08405ce02caeca00c8d7588a5ece3e27cb80589fa3f88240260a88  crates/book-validation/tests/support/mod.rs
9a0a22b8469b12ec200fc8a4c7e9da490b0ec8b7e4af2d6ffcf688b721d530f4  crates/grove/Cargo.toml
42e75a04f8d00e9394db232d0cd1c570f1cc915dacbc8fc6a71f1e30f2b207a1  crates/grove/src/cli.rs
15e7a1356a9e8f8b361ac0f10d31f2c320c03063041ac7bef3354cbe603112c5  crates/grove/src/main.rs
556ddcc803d345a778fc29a604e3f26192eb32601399fb02ba1bf536ea256523  crates/grove/tests/commit_guidance.rs
b1553d9516ba1bdad858954e76bc305e6bcd206fdc2d4b3fba44290796145c9a  crates/grove/tests/corpus_exception_inventory.rs
21691a6d84059dfb46f300433cbeb18b162b3205022e385be42e432185aeb663  crates/grove/tests/env_hygiene.rs
58305700339b5fb1d9b20059d593546c0d535cb1b9c7e2c6f2296f54770d062b  crates/grove/tests/lifecycle_cutover.rs
8f90bf8c422d3077236001d57e0128f522ca7e97d38aea11182261f185f22e4f  crates/grove/tests/loop_driver.rs
9ae0928a5d416aa3709964ddeb3ac0aad76070804b46f4ad3e5dfc4453afebb3  crates/grove/tests/plugin_fallback.rs
0eba7f2640f94f5a3e5ac086ddcbc394aeea1c16132336ae933591a93593a8b9  crates/grove/tests/reference_navigation.rs
4b40dd07b62cb56265acdf347e0fba970ad430084b4f99529198d811739f21f8  crates/grove/tests/retire_guidance.rs
cad6b9172992e650747e6571ec188f83ae99ad40d69833f274b794716a7aa461  crates/grove/tests/support/mod.rs
e64bd18e81a9f3056bdfc02d4f127d8476418a8e6403f7b8942aec53a9217ca6  crates/grove/tests/user_guide_coverage.rs
2df6e7f9e595d41210568a0b5e143a053c103b0526a45568e8750bc3a158cea2  crates/grove/tests/view_command.rs
c2b0ca92e10b32dc6c2862dcd8537ef181485a0faf875a6f78333564bc113e4c  crates/grove/tests/view_terminal.rs
32cc789f43c0802926c05e37f7b49605d28cad5a4d92bce1249b6d63f14a4bcd  crates/grove-llm/Cargo.toml
26d842d627da664d6affe6fa7d3602c3653421af3f8866f907fc10e3136a7328  crates/grove-llm/src/cli.rs
ff8dc60021cba83a7ddb339b48f2951637d6b1888626978dfe2ab87a0f83a341  crates/grove-llm/src/lib.rs
1c87f2e5005bed208b29425998571f7f5e930a16e78fc0f68e58e136bb7b321e  crates/grove-llm/src/main.rs
11156ffeeb96b7189f4548e46eb8960b4132e5361cb459238e3fd724a49ea174  crates/grove-llm/tests/brief_chain.rs
05363503a49ebaa4158e01b7c168fcc14d91f349833249386c0339f42d20cade  crates/grove-llm/tests/complete.rs
a16a0a191c7dfce7f459a05958f5a2b32b74cc80e07b077fc5ba8ffe12ef1b54  crates/grove-llm/tests/composition_guidance.rs
19efcd7fa58e0c304d49cfb8c009152b52e39f75488bc2bc533d8537d7d5283d  crates/grove-llm/tests/composition_verbs.rs
ec85fad515d508ec40ccfc215a544544fc38c14fc4b59f88cef9b5c6129d8171  crates/grove-llm/tests/finish_commit.rs
370c3a9875c3f094e3dce6dc475e0e189c6c7ef24c9a6eb8117f922b5e8b5bbd  crates/grove-llm/tests/help_surfaces.rs
26d7250a4746bc04446d6c72bd1abe7788348971733ede860f8cc578bb709f40  crates/grove-llm/tests/instructed_verbs.rs
7442f9974c1253509cae5d550d52426555d8c87f2f023cf32044d738e95ff9c0  crates/grove-llm/tests/jj_tree_verbs.rs
b03c1b065db4bae17cf15499414ffa2558fe82c0bf9a92521395a0abc106e5ba  crates/grove-llm/tests/kind.rs
4f058b2f918aef21ae4d24f04ac93050ef2e6aa0abcac015cc9bbfe40d76a0bb  crates/grove-llm/tests/leaf.rs
d8cfd34e8b1284ac8d2520c42ba51433be803a69e9f36ed2b5860e0965711e13  crates/grove-llm/tests/leaf_ops.rs
90f4a11babbcc0a9a520c2048dd9d282664a56ae226b9b4e8e9a450fbb688aad  crates/grove-llm/tests/llm_cli.rs
3bb9a1e2efd3101b3fee556a81cf1925a357b0f6b20b0cd51d8547107b8aad88  crates/grove-llm/tests/node_files.rs
53191a9dbab2904e4e52fe1b04259810d812b43238b8a88a986252ce762a0687  crates/grove-llm/tests/pick.rs
bfcd4df2c0072931d4e258d1a129133a9a6d4c2d3ac25a13eea2c068ff2fd53b  crates/grove-llm/tests/removed_surface.rs
ab901b2b43fe474a26ee397d3b1900f86d2905d453d103cad33a591a0150bc5d  crates/grove-llm/tests/resolve.rs
e4489fd7c119dd802cd7130bcef8f66e0b9a85a0c8d1eacdb0ef034df74ecce0  crates/grove-llm/tests/resolve_rendering.rs
24061b7b11a241f1ed51f603bf4ab298deeb04b6238aa364d0e945bad1b3292a  crates/grove-llm/tests/reviewed_producer_lifecycle.rs
4c063773cff38085ef612586c004ecb11e128eac1db9db915046761034a17191  crates/grove-llm/tests/root_init.rs
e95999b324eb52775ba70f66e5723b8db861a0d8c8b11e70f202ca66edaeb66f  crates/grove-llm/tests/session_kind_guidance.rs
9532bb061db014312865af96ce0d811cc2a11e7782b1ec89162595e9e9f06348  crates/grove-llm/tests/session_kind_presence.rs
ef7e7ee5efdf0765d08447e23736c288da989bcbdc54887d5c36e12ccad0fd81  crates/grove-llm/tests/session_kind_tree.rs
cad6b9172992e650747e6571ec188f83ae99ad40d69833f274b794716a7aa461  crates/grove-llm/tests/support/mod.rs
61ba2b8076f5385f46cb0fe2118e17e1d80c1c7c54ed04aa7b05b6ac126e5220  crates/grove-llm/tests/tree_lock.rs
a13e57c1cbc3cd32935507b811543c9ce53d6e3b9215d97c062bdab76647205e  crates/grove-loop/Cargo.toml
b3ac3b9d554d50f80ab9b74e04b2058ab7110ecbf5b1eccb5d74872648f9e47d  crates/grove-loop/src/complete.rs
87de4980b8e45bd7d71a448dff2c67c6b33e2089081201f78c1c08c2029aa088  crates/grove-loop/src/driver.rs
8d1f0f1a717b19ba447e2508f8a8ae2d11c91a5c90ad8db580fc07b79a3a4410  crates/grove-loop/src/driver_lease/observation.rs
c96300862235ec911bbe8c11c9c0fe4e6455c675b40df2c5da34dde33c625ac1  crates/grove-loop/src/driver_lease/witnesses.rs
19900ae2e3df20d951ed49d451b094e61ecc2747233a8825b33a00809045783a  crates/grove-loop/src/driver_lease.rs
e161344012eaf5cf4974448c577f220b93eca95967f2a9107b9839dd1f8d1663  crates/grove-loop/src/lib.rs
458c7b09ac322fb55dd27b8280f0416710e04aa963c09eac8ba171d43d73f3f4  crates/grove-loop/src/loop_driver.rs
3e097914963dcc694e5c7960aa5c24736adb35c1641f4a257606116a0b19c9ec  crates/grove-loop/src/observation.rs
9f3b4acf09f319b28f44aee926d8ddcca05939ecef189a4a1157996c0e008589  crates/grove-loop/src/prompt.rs
8c1a14f7d47df36dd1a454d09a8234e781a0717b3bbb10b7247305d2d5fea89c  crates/grove-loop/src/session_config.rs
55cc75e9f1c9bc694173a50076c35b2a1d66b5fe1880d4b8bf039ef9f5aa473a  crates/grove-loop/src/task_grow/tests.rs
321aa8450d6651dfabebc3041748400fd02a03aee3ca2279864ab1a11df405ed  crates/grove-loop/src/task_grow.rs
11df8d91d4f11b52860f93db4b81da466f76609595fcaa7065d3e22f593103c4  crates/grove-loop/src/task_name.rs
6681d1ad7c27b731491755506e0bfeefebc1ef268547223383c8885ca8e35064  crates/grove-loop/src/task_tree.rs
0ec5bf961f74627ce08ec4c9064818aec7d634bcbe1c0939e7af57230ba224d3  crates/grove-loop/src/tree_lifecycle.rs
5435547ee1ca5f44aad6749879cf5f36f8e6fac015fd4d878873d9ce84502e26  crates/grove-loop/src/verbs.rs
c4d88a26beb9bd100c3ebe7b920d023ec6b1a19977e0989152b29869985ad665  crates/grove-loop/tests/driver_lease.rs
52e91dcc0be48e04e70bc2b9658e1b0a39f492a5f7470c28f9ca8c729fefd501  crates/grove-loop/tests/library_dependency.rs
59e2fb23842e07db72e323fdcef963c22e38c9b60b2b0c3049ab50ecbcce4e7b  crates/grove-loop/tests/observation.rs
2c1e87ffd57f27bfb0969a37da5d26ffbffe4fd0e14583deab272f0e5fba526e  crates/grove-loop/tests/prompt.rs
9bf87308df6fe1fd50e5aa2527b349fffbd95b714c2c28825d84b2d49da05614  crates/grove-loop/tests/selection.rs
b2a4ebe892472e2bd72d19fc45bd2af05997752de355f9c620dcc76bc93455b9  crates/grove-loop/tests/session_config.rs
cad6b9172992e650747e6571ec188f83ae99ad40d69833f274b794716a7aa461  crates/grove-loop/tests/support/mod.rs
2d7e8ced68db92c151df79899ee3d422c81e5cb9ff242c2663b022d54f5649c0  crates/grove-loop/tests/verbs.rs
ebe43e786c07e62aed565ba07f43e6d090ef6f6e9498c7e3cc885541e80bbe38  crates/grove-tui/Cargo.toml
1e67fd75d31b6405ecd6cf698048c0cc5b1ea442ff72a2374d17d55d77fd539d  crates/grove-tui/src/lib.rs
924b1ed10baa3e2011e32715da0f6438b8ca9eff98265c81a9869e6a5ff40216  crates/grove-tui/src/markdown.rs
fabc790c1621c8575c065ceb58fb3bd24413b4bcefb46d2d38fb9ab4be5e398e  crates/grove-tui/src/observation.rs
435f676052f5894ec5de3928007dcb7f5a262717225318b96c6f2341bfd275cd  crates/grove-tui/src/terminal.rs
d7912a63ef8ad867dc455c868d0dc6caa0e532550baada72a15b99c15323d3db  crates/grove-tui/src/terminal_tests.rs
c997afc279b00fa476ba79cb9cae320c5fa0c84faa3f84ed0c8322b6d6a92c47  crates/grove-tui/tests/browser.rs
d3df4572fecfa1d403a3c55f1c059d24d313da82a691b04fc4a70441bb4676ec  crates/grove-tui/tests/support/pty.rs
5b313aac5ada11f84e53ace3ebea3065a7fa30afeac7a4eb168f7cfd4258a47b  crates/jj-workspace/Cargo.toml
23eb9d98df0eb1906f25cfefd869c606d2750b99b703d35d0c533f5cba1f8af6  crates/jj-workspace/src/jj.rs
9bbf8ed311587d799463f4544371003351004f4ddb56f35319b73f814fc86d08  crates/jj-workspace/src/lib.rs
ead91bf98ff3cc063334e9620d5eff10e09e266c171d4cfbc9a35cdfd88e3e5c  crates/jj-workspace/src/refusal.rs
0ae42c4967467f4126408ef50974f8f2901b83277890c1a9146c3436c8643458  crates/jj-workspace/tests/discovery.rs
465ef2967839aa937d9aaa6b39372ce45556f17cad936b66072ccb5bd8891d61  crates/jj-workspace/tests/environment.rs
373b9f99966b44633e7075fa49d72c9bc2b80101dfebaf1ef7fcaade4248d119  crates/jj-workspace/tests/workspace.rs
4eadcd4fdf2607be6764b92ef200818f980cfd1bc05ca9319aad921660eb0bfb  crates/keyed-launch/Cargo.toml
65ceef03b120eb4327a7eee7a588fe812bb36dd69605c7f922d911b6047f14c8  crates/keyed-launch/src/argv.rs
307b9b3b3a3661b7a11c9e3e9471d07fdf88d1b4e9a06cc48374c7d6de84f947  crates/keyed-launch/src/channel.rs
14c941de15a4ee10df0fe637973e198528cc42af8a80cbe3e819cd1547743b56  crates/keyed-launch/src/conformance.rs
d3e323bc2d98b037f3d73eaca46b497e38e61a4c734b893ab2f0cfd1ce2b88da  crates/keyed-launch/src/error.rs
f50c2baa9add3a5979716a18ee68f785c714ea16de5483cb22928c1f6c74042a  crates/keyed-launch/src/lib.rs
b6e14c85f7befef9d2e01386837a4c3d45f18ce891e7dc9d003e113b0a7f5715  crates/keyed-launch/src/run.rs
2eef5c56926f1b9a2c40d301e51cbbb3dd4cf2d7c027fe0cee3afe677e732ab8  crates/keyed-launch/src/templates.rs
c279142dada37b0c72e0c40a8f7dc791f414fb5815662a9a0cf923c613009c54  crates/keyed-launch/src/vocabulary.rs
27b7760f613d45eb473c3f4aa7409339ecf5cb234328e1c1dc3a99470a5fffab  crates/keyed-launch/tests/conformance_kit.rs
a1c1be0e83b84986d62b56b6f73598749acf083eb2f45d1a19d3fa24341642af  crates/keyed-launch/tests/internal/wait_events.rs
2d0b2af84a46e458164c7a6edcdceff1be3f0c3d5a0ae261435dbaff11fab3e1  crates/keyed-launch/tests/interrupt.rs
0a8413269ba1f112db843b5ea3dfb03bc5b87102d8cf9d8fd0aac2542d1f33f5  crates/keyed-launch/tests/launch.rs
0b5c738847efd37e6cf80d89e145410e3ffecd8dba521ef782122b6562d649d8  crates/keyed-launch/tests/reraise.rs
27d8039fd7efde39ab70eacdf15944365e3f8dbe01e86653f722ef28a6085dc6  crates/keyed-launch/tests/templates.rs
bc9397fb9f2cb962d04c4b5b6dcc41ca41baaa60804718270a133de113edeb0d  crates/ordinal-fs-tree/Cargo.toml
40e83051540e5514e15f6e0f8d798aba2900353dc6171276a2c5756fb916d014  crates/ordinal-fs-tree/bin/syllabus.rs
01aa3e89da9096fcd9d861efce910a13d18fef431217c815e06641202140f5a7  crates/ordinal-fs-tree/src/conformance.rs
d94b3fbb53513f10b0fa68da7683df478d823c22cd28716316260b2bcba2cf77  crates/ordinal-fs-tree/src/error.rs
293107e01780b6c3e6392e13c69188f0dad04d4e525810e2d6d3dc59b3d57a35  crates/ordinal-fs-tree/src/fixtures.rs
157aeaf5cf1c04c0b537176246bf7e24806da993926c7a30e58d195f9cc40740  crates/ordinal-fs-tree/src/fs/apply/tests.rs
8dbfaae423e31d3cb198ebf2608baf1ac3ef0deeb4fa1a76602d4a1df847710a  crates/ordinal-fs-tree/src/fs/apply.rs
3a3efccdc19b247e8c9bef4c8bb1978f9c10f3548da3721545f6821a04787945  crates/ordinal-fs-tree/src/fs/lock.rs
c9ba2a0e34dc91f0b203af636df47f29e23beabed693321663018e4f2726e8fe  crates/ordinal-fs-tree/src/fs/mod.rs
88cf3955701ebb960a3017a6038cdfc03917996bf5aaa25d1b2ce1497a6ede7d  crates/ordinal-fs-tree/src/fs/read.rs
3838ba1e4717b4d1d7151a1d6ec116f53dd97b70f47a7028f07f2347ec37bd27  crates/ordinal-fs-tree/src/fs/remove.rs
5c77e5ab46a5639aeaa994c1ed06f11a10753bd67c82234b80ff5c05553ec458  crates/ordinal-fs-tree/src/lib.rs
1d161be9517c7551d4bdcfcf4a4a2d95d4f5b29e3997ae69f0878d4fb5afb908  crates/ordinal-fs-tree/src/name.rs
c7cdd111e3f388ff79ce3edfbb6b92cb9a23e67f10acae755fd7c57480c23fad  crates/ordinal-fs-tree/src/ops/tests.rs
4b26db98728eea84eb5afa425696b53deb7bfa6b741f682e5cad6bae5767dfcc  crates/ordinal-fs-tree/src/ops.rs
6aad8f34899dd6345bfbfc3b2279f8d3018de6d88c6e8a34d90c27ab525772ad  crates/ordinal-fs-tree/src/plan/tests.rs
b929630a50db162d20cf3db59e72cd8f3bb6b8bffe7511b26b5a8b7eae4c259a  crates/ordinal-fs-tree/src/plan.rs
bfcb9b6b4366da4ee4e455102abeb0f8621c27f7f99d03d64dd875a1fb32d9e1  crates/ordinal-fs-tree/src/reference.rs
641b2329f9ddaf1875e4766f557629c5d61c4c530d411321ac3bad526763ce0a  crates/ordinal-fs-tree/src/report.rs
6e366f978930a3ca7fe2eac2ed4569a841bf4c0e20378cef5fc37a683ffa37af  crates/ordinal-fs-tree/src/snapshot/tests.rs
ff1d796e3795fc41d8714c3c02181c40a663b38bfc4b8cb211621a129d3dc561  crates/ordinal-fs-tree/src/snapshot.rs
e90c5af64fb032553f435ffa911cbc80ea77476f772d692c7e860987c37aff8b  crates/ordinal-fs-tree/src/sought.rs
571b6f197a5892b83622f25b47b19123677bec9145d0b1a689b084414200e7a3  crates/ordinal-fs-tree/tests/algebra_has_no_filesystem.rs
dc59bcbe5034bc2b536cb4def651f164911ea7dafeb818021f666d47151fed67  crates/ordinal-fs-tree/tests/appending_on_disk.rs
6e687d6f23bcc006b889c4b40a98ce76521a4075861d157768d2b676f97ff445  crates/ordinal-fs-tree/tests/conformance_kit.rs
6c22fa05e55eb6e37f6c1e6fcd0b836a3e7c75def815f3fc1676d6a3c633d77f  crates/ordinal-fs-tree/tests/deleting_on_disk.rs
22e42a5be9c7dde2389c1003e497847ea1a7a93088507deff90826c7fc989d70  crates/ordinal-fs-tree/tests/driving_a_tree.rs
7a07ba21d28f7dd17d29e867ddb8d51934dcf1661e3c85298a71fb554464e4f0  crates/ordinal-fs-tree/tests/initializing_on_disk.rs
9dd6a878a93589023b61164268456749a4d8d47f1c22c4f13f35f0f5318cd407  crates/ordinal-fs-tree/tests/inserting_on_disk.rs
2f3885686e64f7c2562d0cc71f5070763c652f6780a4182ca0a7c9064d183a91  crates/ordinal-fs-tree/tests/names_are_confined.rs
ee3f22d9658c0ef856888ba100ce120d2e7607f46d5bb8ea7fb6506591cb5e24  crates/ordinal-fs-tree/tests/promoting_on_disk.rs
213b6a3677e5a91ec493c9a4b9bd47e52b081c1021226df3a185099294474270  crates/ordinal-fs-tree/tests/reading_on_disk.rs
e6ce6ed5170170d004b3c7ef1be8e69fb07f8e1ed77e2092276d3d7883d12fc7  crates/ordinal-fs-tree/tests/reference_domain.rs
e2b761827e0bc133e67e4a5aa95ba64f248cb62563b143ca971f0d643ec76229  crates/ordinal-fs-tree/tests/rewriting_on_disk.rs
79862e0d44848f022bf9f4189c172324ab13a13a83520ae0e0e19efbffa53f88  crates/ordinal-fs-tree/tests/valid_levels.rs
```
