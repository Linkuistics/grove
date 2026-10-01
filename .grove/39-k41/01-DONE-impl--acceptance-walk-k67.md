# acceptance-walk-k67

## Goal

Demonstrate that every acceptance case of the first release has an owner and a
passing instrument, before the documents are rewritten as current state.

## Context

The cases are the root brief's *Acceptance cases* and every row of the spec's
`#test-seams` table, the firing-configuration table and the classes with no
known firing configuration included. The instruments are the two process
seams: `crates/harness-dispatch/tests/` for the command, and
`crates/grove/tests/loop_driver.rs` for Grove's launch boundary and
controlling PTY. Per-target delivery is `task release:smoke`.

This leaf builds no missing behavior. A case with no passing instrument is
cut as a leaf ahead of `current-state-documents-k68`.

## Done when

- Each acceptance case and each clause of each seam row names its test or
  task. A clause that is a review's to judge, not a test's, says so.
- Each hostile, limit and missing-source class names its firing control.
- The named tests are shown passing by a run of `task check`, read per test
  and not by its exit status alone.
- `task release:smoke` passes on all three targets.
- The walk is in this leaf's running log, with what it found missing and the
  leaf cut for it.

## Notes

## Decisions (running log)

**`dispatch-documentation-k41` decomposed.** Its reading alone is the spec,
the dispatch README, the runtime evidence, three ADRs, two Grove references,
configure-grove and the test suites of two crates, about 17,000 lines of tests
among them. That does not fit one session. The walk runs first because the
documents rest on it. The node's brief has the split.

**How the walk was made.** Every test name of the two seams was listed from
source first, 234 in `crates/harness-dispatch/tests/` and 29 in
`crates/grove/tests/loop_driver.rs`, and each clause was then mapped to the
assertion that observes it, by `file:line`. A test's name was not taken as
evidence. Six read-only agents did the mapping, one per area, each told to
report a clause with no observing assertion as a gap. I checked a sample of
each report against the source. The passing run is one `task check` of the
working copy, read per test. Nothing in that run reads this repository's own
`.grove/`, so editing task files beside it changed none of its subjects.

**Delivery (root brief, last acceptance case).** I read these scripts myself.

| Clause | Instrument |
|---|---|
| The release route delivers the command on each supported target | `task release:smoke`: `scripts/release-smoke.sh` runs `release-smoke-target.sh` in each target's environment, which extracts the archive with that environment's `tar` and checks that `grove`, `grove-llm` and `harness-dispatch` report the version |
| Archive and install contents | `assert_archive` in `scripts/release-common.sh`, run by the build and again by the smoke; `scripts/release.test.sh` (the "release tasks" check) holds the manifest to the spec's layout, to what `dispatch.sh build` emits and to the formula |
| Firing control for the contents check | `release.test.sh`, "Positive controls: each file's omission is seen to fail, by name"; a probe build beside the worker is refused by path |
| A static TypeScript fake-harness case on each target | `case_static_typescript` in `crates/harness-dispatch/scripts/installed-smoke.sh`: `inspect`, `run` to a fake harness that exits 42, byte-exact argv, then `record show` |
| A computed TypeScript case | `case_computed_typescript`: an asynchronous `select`, a refused explicit choice that starts nothing, then a run |
| No separately installed runtime | `installed-smoke.sh` `main` refuses to start with `bun` or `node` on `PATH` |
| A package declared by `main` and one by `exports` | `case_declared_package` |
| Signal state at the installed layout | `case_signal_state`, with its own control that the script's children start with the default disposition |
| The glibc 2.17 floor, executed | `assert_glibc_floor` in `release-smoke-target.sh`: the userland reports 2.17, the 2.17 probe runs, and the 2.25 probe must be refused for its symbol version |
| The CPU floor, executed | `scripts/release-smoke-qemu.sh`: an AVX2 or Armv8.1 probe must die of SIGILL under the floor's CPU model and run under `-cpu max` |
| The kernel floor | No instrument, by the accepted trade-off: it is Bun's documented range, stated as documented |
| Package checks join the Taskfile | `dispatch:deps`, `dispatch:worker`, `dispatch:typecheck`, `dispatch:probes`, `dispatch:build`, `dispatch:check` and `dispatch:install`; `scripts/check.sh` runs the worker, probe and type checks before `cargo test` |
| Documentation explains ownership, inspection and remedies | The review `usage-agreement-k69` cuts. A test pins only the quoted command |

**The passing run.** `task check` on the working copy at commit `cbf96557`,
whose only difference from `e6aa7f9f` is this node's files under `.grove/`.
All 12 principal checks passed. Each of the 263 listed seam tests appears as
`test <name> ... ok` in its log, looked up by name. A name that does not
exist is found zero times, so the lookup is not vacuous. The ten ignored
tests are subprocess fixtures and credentialed smokes of `crates/grove` and
`crates/grove-loop`, none of them a seam test.

**Records and observations (seam row 4, and the brief's record case).** Test
names are relative to `crates/harness-dispatch/tests/`. I checked the
citations for `records.rs:255-262`, `observations.rs:104-236` and
`lookup.rs:748-786` against the source, and they hold.

| Clause | Test |
|---|---|
| The record is committed before the harness starts | `records::a_run_commits_its_handoff_record_before_the_harness_starts`: the fake harness copies the store as it starts, and the test finds the run in that copy |
| Each recorded launch field is read back | `records::record_show_exports_the_launch_fields_with_the_attempt_unknown_and_every_measurement_unobserved`, with `choice::run_launches_the_explicit_choice_and_records_it`, `context::run_records_the_reviewed_artifact_and_the_context_by_digest_and_size`, `grove::a_dispatched_producer_names_its_run_and_its_review_uses_the_runs_recorded_provider` for the adapter, and `lookup::host_run_returns_a_recorded_runs_immutable_launch_fields` for the creator and the source digests |
| A failed required commit prevents exec, exit 4 | `records::an_unwritable_record_directory_launches_nothing`, `a_store_that_cannot_grow_launches_nothing`, `a_lock_held_past_the_wait_launches_nothing_and_spends_no_selection_time`, `a_store_that_is_corrupt_foreign_or_newer_refuses_and_is_left_as_it_was` |
| An attempt and an exec failure stay distinct | `records::record_show_exports_…` (`handoff_attempt`, `unknown`) and `records::an_exec_failure_is_appended_to_its_attempt` (`launch_failure`, `not_executed`) |
| A failed append leaves the attempt unknown | `records::a_failed_append_leaves_the_attempt_unknown`, `handoff::a_cancelled_handoff_whose_detail_cannot_be_appended_stays_unknown` |
| Cancellation after the commit launches nothing and marks the attempt | `handoff::a_signal_between_the_commit_and_exec_launches_nothing_and_marks_the_attempt_not_executed` |
| Pre-commit refusals create no run | `records::a_refusal_before_the_commit_creates_no_run` |
| Inspection is a proposal and writes nothing | `records::inspect_proposes_a_marked_run_id_and_writes_nothing` |
| Unknown outcomes stay explicit | `observations::unsupplied_measurements_stay_unobserved_and_an_attempt_stays_unconfirmed`, and the every-measurement loop in `records::record_show_exports_…` |
| Observation round trip, every supported measurement | `observations::an_observation_round_trips_through_observe_and_show` |
| Idempotency, conflict, correction | `observations::a_repeat_is_idempotent_and_a_conflicting_repeat_refuses`, `a_correction_supersedes_its_observation_and_both_are_kept` |
| Launch fields never change | `observations::no_import_changes_the_launch_fields`, `records::committed_launch_fields_never_change` |
| Run lookup returns immutable launch fields | `lookup::host_run_returns_a_recorded_runs_immutable_launch_fields`, `a_changed_current_catalog_does_not_alter_the_returned_snapshot` |
| Run lookup reads no observation history | `lookup::a_lookup_reads_the_runs_launch_fields_never_its_observation_history`, whose control is `record show` refusing the same planted row |
| An unreadable store refuses and never reads as missing | `lookup::a_store_that_cannot_be_read_refuses_and_never_reads_as_missing`, beside `a_missing_run_is_reported_missing_whether_or_not_there_is_a_store` |
| A stored document this release cannot read refuses every read | `lookup::a_launch_record_this_release_cannot_read_refuses_every_read_of_the_run`, `observations::a_stored_observation_this_release_cannot_read_refuses_the_export` |
| The review's run records the creator provenance | `lookup::the_run_record_names_the_creator_reference_its_evidence_and_its_lookup` |
| Later observations after teardown | `observations::observations_survive_the_deletion_of_the_task_tree_that_named_the_run` |
| A retry is a new run; lookup takes only a run ID | `records::every_run_is_a_new_run_with_its_own_identity`, `lookup::run_lookup_is_the_loaders_alone_and_takes_only_a_run_id` |
| No environment value is stored or printed | `environment::inspection_and_records_name_each_grant_and_never_show_its_value` |
| A documented way to enter later observations | `observations::help_carries_an_observation_example`, which imports the help text's own example |

Every clause has a seam test. Four are held more narrowly at the seam than
the spec states them, with a unit test in `src/observation.rs` holding the
rest. The round trip imports an empty `falseFindings` list, an exit by code
and never by signal, and a `successProbability` that is uncalibrated and never
calibrated. An imported observation's unsupplied measurements are checked for
four of twelve by name, beside a count of fifteen.

**The command, independently (seam row 1, and the brief's first three
cases).** Citations checked: `authority.rs:366-399`, `review.rs:903-910`,
`support/mod.rs:194-205`.

| Clause | Test |
|---|---|
| No Grove files or binary present | By construction in `support/mod.rs`: `command_for` clears the environment, sets a fresh empty `HOME` and cwd, and a `PATH` of the sandbox's `bin` and the system directories. `review::the_generic_form_selects_with_no_task_file_or_grove` asserts it for the cwd and that `bin` |
| A task file is optional | `inspect::inspection_reports_the_routed_candidate_and_its_evidence_in_json` (`taskFile` null), `select::select_is_called_as_a_method_with_the_versioned_request_no_context_and_a_host` |
| Static selection, inspected and run | `inspect::inspection_reports_the_routed_candidate_…`, `run::run_hands_the_exact_argv_to_the_harness_in_the_callers_own_process_and_cwd` |
| Computed selection, inspected and run | `select::a_synchronous_or_asynchronous_select_chooses_a_configured_candidate`, `select::run_launches_the_computed_choice_and_records_how_it_was_made` |
| Inspection: source and authority | `inspect::inspection_reports_the_routed_candidate_…` (personal), `authority::an_explicit_relative_config_resolves_against_the_original_cwd_and_replaces_the_default` (explicit) |
| Inspection: choice, reason, expanded argv | `inspect::inspection_reports_the_routed_candidate_…`, `inspect::inspection_shows_the_unchanged_prompt_and_every_expanded_slot`, `inspect::inspection_reports_the_same_facts_as_human_text` |
| Inspection: measured sources and delivered size | `context::a_caller_context_reaches_the_policy_as_data_and_inspection_measures_it`, `context::a_loader_reads_measured_sources_against_the_callers_directory` |
| Inspection: effective limits | `deadline::inspection_reports_the_effective_selection_bound`, `bounds::context_bytes_is_one_byte_to_eight_mebibytes`; the other four are held equal to a run's record in `context::run_records_the_reviewed_artifact_…` and asserted there by `records::record_show_exports_…` |
| Inspection launches nothing | `run::inspection_reports_where_a_relative_prompt_file_was_read` |
| Literal punctuation and newlines | `run::run_hands_the_exact_argv_…`, whose prompt is `Fix the "parser"; don't `rm -rf` $HOME && echo 'done' \| tee *`, two newlines, a tab and `$(date)`, and whose task path holds a space, quotes, `;` and `$(x)`; `run::a_prompt_file_is_read_once_with_its_exact_bytes_from_the_callers_cwd` |
| Policy errors launch nothing | `inspect::an_entry_that_throws_while_loading_refuses`, `inspect::every_invalid_policy_shape_refuses_with_its_location`, `select::each_way_select_can_fail_refuses_with_its_own_code_and_launches_nothing` |
| Bad imports launch nothing | `inspect::a_missing_relative_import_refuses_and_names_the_entry`, `inspect::a_missing_package_is_never_installed_automatically` |
| Missing context and a failed loader launch nothing | `context::a_missing_required_source_refuses_and_names_it`, `context::a_failing_loader_refuses_and_launches_nothing` |
| An incomplete mapping gets no default | `inspect::an_unrouted_kind_refuses_as_an_incomplete_mapping_without_a_default`; under `run` in `refusals::every_exit_result_matches_the_spec_table` |
| An unreadable or missing selected entry names its path | `authority::a_missing_unreadable_or_unlocatable_entry_refuses_naming_the_path` |
| An unavailable program launches nothing and nothing else | `run::a_missing_program_exits_127_and_never_falls_back_to_another_candidate`, `run::an_unexecutable_program_exits_126`, `run::an_exec_error_reports_errno_with_a_remedy` |
| Explicit choice is visible, and a mismatch refuses | `select::select_sees_an_explicit_choice_and_accepting_it_selects_it`, `select::any_other_id_for_an_explicit_choice_is_a_mismatch_whatever_the_reason`, `choice::an_unknown_choice_refuses_without_selecting_anything_else` |
| Selection consumes no stdin | `run::the_harness_keeps_the_callers_stdin_stdout_and_other_descriptors`, `authority::the_worker_evaluates_policy_in_the_root_directory_with_null_stdin_and_a_fresh_environment`, `run::a_terminal_is_never_read_as_the_prompt` |
| Each documented exit: 2, 3, 4, 5, 124, 126, 127 | `refusals::every_exit_result_matches_the_spec_table`, one case each |
| The harness's own exit and signal pass through | `run::the_harness_exits_with_its_own_code_and_signal` |
| A refused run names its `inspect` invocation | `refusals::a_refused_run_names_the_equivalent_inspect_invocation_which_reproduces_the_selection` |
| Help carries independent, Grove, recovery and observation examples | `refusals::help_carries_independent_use_grove_and_refusal_recovery_examples`, `observations::help_carries_an_observation_example` |

"Launched nothing" is observed one way everywhere: the fake harness makes a
record directory as it starts, and `Sandbox::harness_ran` looks for it. Its
positive control is any run that reaches the harness, such as the last line
of `refusals::every_exit_result_matches_the_spec_table`.

Held more narrowly than stated: the import and shape refusals are driven
through `inspect` and not `run`, which evaluate one selection path; a missing
entry is the personal default and an unreadable one is a directory, with no
`--config` naming a file that does not exist; and no command-seam test gives
the front a terminal as its own stdin, which Grove's PTY suite does.

**Limits, timeouts and signals (seam rows 1 and 3).** Citations checked:
`deadline.rs:70-86` and `428-446`, `bounds.rs:480-502`, `records.rs:554-573`.

| Limit | Holds | Refuses |
|---|---|---|
| Selection time | `deadline::a_hold_that_ends_within_the_bound_reaches_the_harness` | The six `deadline::*_is_stopped_at_the_deadline` tests, exit 124; out of range in `deadline::the_bound_is_one_to_one_hundred_and_twenty_seconds_in_milliseconds` |
| Context size | `bounds::the_context_budget_holds_at_its_default_and_at_its_ceiling`, at exactly the budget | The same test at one byte more, `context_too_large`, which names the encoded size |
| One source read | `bounds::one_read_holds_to_its_limit_whatever_the_policy_does_with_the_error`, 65536 bytes | The same test at 65537, `source_too_large`; `bounds::a_read_that_already_takes_the_whole_budget_is_remedied_by_the_budget` |
| Source count | `bounds::a_context_holds_at_most_256_sources_the_document_included`, 256 | The same test at 257, `too_many_sources` |
| Protocol message | `bounds::a_catalog_snapshot_or_result_holds_to_the_message_bound`, 1 MiB | The same test at one byte more, `message_too_large` |
| Diagnostics | `bounds::diagnostics_hold_to_their_bound_across_both_streams`, 131072 on each stream | The same test with one byte more on either, `output_limit`; `bounds::a_policy_that_prints_without_end_is_stopped_for_its_output_not_its_time` |
| Prompt | `run::an_invalid_or_unreadable_prompt_is_refused_before_policy_runs`, 1 MiB through `inspect` | The same test at one byte more; `bounds::an_oversize_prompt_does_not_count_against_the_context_budget` |
| Lock wait | `records::a_lock_held_past_the_wait_launches_nothing_and_spends_no_selection_time`, after release | The same test, `record_store_locked` after at least 1.95 s, with a 1-second selection bound that is not charged |

A breach the policy catches is still reported: the `catch` arms of the read
and source-count tests, and `context::a_loader_can_refuse_as_the_policy_and_nothing_is_selected`.

| Clause | Test |
|---|---|
| A timeout at import, in the loader and in `select` launches nothing | `deadline::a_policy_that_spins_at_import_…`, `a_load_context_that_spins_…`, `a_select_that_spins_…` and their `awaiting_a_promise` twins |
| A signal in each phase cancels, launches and records nothing | `cancellation::a_signal_while_the_policy_is_evaluated_cancels_and_launches_nothing`, `a_spinning_policy_is_cancelled_and_inspect_is_cancelled_as_run_is`, `a_terminal_interrupt_to_the_whole_job_is_reported_as_the_cancellation` |
| Their controls reach the harness | `deadline::a_hold_that_ends_within_the_bound_reaches_the_harness`, `cancellation::the_same_fixtures_uninterrupted_reach_the_harness`, `handoff::the_same_stall_unsignalled_reaches_the_harness` |
| The signal is re-raised after the refusal | `Run::cancelled` in `support/mod.rs`, which every cancellation case goes through |
| A worker that ignores TERM is killed and none survives | `deadline::a_worker_that_ignores_term_is_killed_after_at_most_a_second_of_grace` |
| Cancellation reaches the policy's children | `cancellation::a_group_interrupt_reaches_the_policys_own_children_through_the_job` |
| An ignored signal cannot cancel | `cancellation::a_signal_ignored_at_entry_cannot_cancel_selection`, `handoff::a_signal_the_caller_ignored_does_not_cancel_the_handoff` |
| The mask and dispositions reach the harness, SIGPIPE included | `handoff::the_harness_inherits_the_callers_mask_and_ignored_signals`, `handoff::a_signal_the_caller_blocked_reaches_the_harness_pending_and_cancels_nothing` |
| The signal probe can see a change | `handoff::a_state_altered_between_the_front_and_the_harness_is_seen` |

Held more narrowly than stated. The 30-second default and the 120-second
ceiling are reported values: no test waits either out, which would cost that
long on every run. The one-byte context floor is accepted and reported, and
the smallest budget seen to refuse is 200 bytes. A direct read above 64 KiB
with a raised `maxBytes` is seen only through the Grove adapter. "Keeping the
first 256 KiB" is a length, over uniform bytes. The grace is bounded by the
selection bound plus four seconds, not by one second. Three controls are
missing their own fixture: the TERM-ignoring worker within its bound, the
child-spawning policy uninterrupted, and the refused append unsignalled.

**Authority and hostile classes (seam row 3 and the firing table).**
Citations checked: `hostile.rs:1319-1446`, `worker.rs:140-176` and `252-312`,
`src/worker.rs:500-516`.

| Class | Test, with its inert and firing arms |
|---|---|
| cwd policy entry | `authority::no_cwd_search_or_environment_variable_selects_an_entry`: a sentinel stays absent, then exists once `--config policy.ts` names the file |
| cwd `.env` and bunfig preload | `hostile::a_cwd_dotenv_and_bunfig_preload_stay_inert_and_fire_under_the_autoload_probe` |
| `BUN_OPTIONS` and `BUN_BE_BUN` | `hostile::bun_runtime_variables_stay_inert_through_the_front_and_fire_in_the_worker_started_directly` |
| The package shadow, three layouts | `hostile::every_documented_specifier_resolves_to_its_embedded_module_beside_a_package_shadow`, fired under the unregistered probe |
| cwd `package.json`, three forms | `hostile::a_cwd_package_json_stays_inert_and_fires_for_an_entry_admitted_there` |
| tsconfig `paths` beside an entry | `hostile::tsconfig_paths_beside_an_admitted_entry_stay_inert_and_fire_under_the_tsconfig_probe` |
| `.env` where a policy starts a `Worker` | `hostile::a_dotenv_where_a_policy_starts_a_worker_stays_inert_and_fires_under_the_autoload_probe` |
| A package above the start directory | `hostile::a_package_above_the_workers_start_directory_stays_inert_and_fires_under_the_unmoved_probe`, for `data:`, `blob:` and virtual modules |
| A `package.json` above the start directory | `hostile::a_package_json_above_the_workers_start_directory_stays_inert_and_fires_under_the_unmoved_probe`; its file-located arm is the "under no build" case |
| One that never yields | `hostile::a_package_json_that_never_yields_above_the_workers_start_directory_stays_unopened_and_stalls_the_unmoved_probe`, with its baseline |
| The transpiler cache | `hostile::the_runtime_transpiler_cache_stays_inert_through_the_front_and_fires_in_the_worker_started_directly`, under `HOME` and `XDG_CACHE_HOME` |
| `NODE_PRESERVE_SYMLINKS` and `NODE_CHANNEL_FD` | `hostile::node_resolver_and_channel_variables_stay_inert_through_the_front_and_fire_in_the_worker_started_directly`, each beside a baseline without the variable |
| The four classes with no known firing configuration | `hostile::classes_with_no_known_firing_configuration_are_reported_not_counted`, through `still_unfired`, which fails if one fires |
| A probe is never an installation's worker | `hostile::a_probe_build_is_never_accepted_as_an_installations_worker`, over `Probe::ALL`, with the shipped worker accepted as its control |

| Clause | Test |
|---|---|
| An owner-only empty start directory, removed | `worker::the_front_starts_its_worker_in_a_private_empty_directory_and_removes_it` |
| Policy code runs in `/`, with null stdin and no caller descriptor | `authority::the_worker_evaluates_policy_in_the_root_directory_…`, `authority::a_caller_descriptor_above_the_soft_descriptor_limit_never_reaches_the_worker` |
| Relative `--config` and a personal import are admitted | `authority::an_explicit_relative_config_resolves_…`, `authority::personal_policy_may_import_a_repository_entry_explicitly` |
| A package loads by `main`, `exports`, a subpath, a condition and its `imports`; `NODE_ENV` chooses none | `authority::a_package_loads_by_the_entry_point_its_package_json_declares` |
| An entry's own `package.json` | `authority::an_entrys_own_package_json_applies_its_imports_map_and_answers_its_own_name` |
| The nearest `package.json` that reads as one | `authority::the_nearest_package_json_that_reads_as_one_is_a_modules_own_and_one_that_does_not_is_passed_over` |
| A missing package is not installed | `inspect::a_missing_package_is_never_installed_automatically` |
| An `imports` alias to a registered specifier | `hostile::an_imports_alias_to_a_registered_specifier_is_a_package_lookup_and_never_the_embedded_module` |
| Worker and child lack completion values | `environment::the_worker_and_a_child_it_spawns_lack_completion_values_unless_granted`, whose control is the granted value arriving |
| Excluded grants refuse | `environment::excluded_names_are_refused_before_any_policy_runs`, every excluded class |
| Structured output stays clean | `hostile::a_policy_flooding_both_streams_leaves_json_output_and_the_protocol_intact`, `inspect::policy_output_is_captured_and_never_interleaved_with_the_report` |
| The worker is found from the front's real path | `worker::a_front_without_its_worker_refuses_and_no_ambient_decoy_substitutes`, `a_symlink_to_the_front_still_finds_its_worker`, `an_argv0_naming_another_prefix_never_relocates_the_worker` |
| A worker of another build refuses | `worker::a_worker_from_another_build_refuses_before_it_is_given_a_policy` |

Two of these can pass without observing what they claim. In the
no-known-firing test, the cwd tsconfig arm checks the probe's hello and then
only that the alias did not load, so a probe that failed before its import
would pass; the other three arms assert that the probe loaded first. And the
mismatched workers in `worker::a_worker_from_another_build_…` are shell
scripts that never read descriptor 3, so the test sees the refusal code but
not that the front withheld the policy; its own sentinel message says so.
The start directory's mode is asserted under the suite's own umask, which is
enough to tell `0o700` from the default on this host and is not "whatever the
umask".

**Grove's launch boundary and controlling PTY (seam rows 5 and 6).** Test
names are in `crates/grove/tests/loop_driver.rs` unless another file is
named. Citations checked: `loop_driver.rs:2938-2962`, `3858-3881` and
`2470-2476`, `config_show.rs:178-197`.

| Clause | Test |
|---|---|
| The prompt is unchanged and the three slots are native arguments | `the_selected_task_arrives_as_native_arguments_beside_an_unchanged_prompt`, in a worktree named `work tree 'single' "double" $(touch x) `y`; a\|b & *`, so the task path and the prompt both carry it |
| Through dispatch: prompt, task and `HARNESS_DISPATCH_RUN_ID` | `a_dispatched_session_receives_its_task_as_native_data_and_only_its_harness_the_channel` |
| The channel reaches the harness and not the worker or its child | The same test, whose policy records the worker's and a child's environment; its control grants `GROVE_SIGNAL_FILE` and the same probe then reads it |
| A literal `--choice` reaches policy | `a_literal_choice_in_the_command_definition_reaches_policy_as_the_explicit_choice` |
| Direct-harness compatibility | `a_dispatched_and_a_direct_kind_coexist_in_one_configuration` |
| Authoring succeeds and launch refuses, leaf live | `a_leaf_authored_under_a_valid_wrapper_is_refused_at_launch_and_stays_live`, which also runs the printed `inspect` line |
| The documented command is the launched one | `the_documented_command_definition_for_dispatch_is_the_one_launched_here`, over both help texts, `docs/CONFIGURATION.md`, configure-grove, the dispatch README and `docs/USAGE.md` |
| Retirement and reordering leave the creator unchanged | `retiring_and_reordering_a_producer_leaves_its_review_s_creator_unchanged`, under a remapped policy |
| A decomposed producer, closed through two levels | `a_decomposed_producer_s_review_carries_the_run_whose_retirement_closed_its_node`: the reviewed artifact is `parser-k1` and the looked-up run's task is `tokens-k5` |
| A close cascade's set | `a_close_cascade_settles_every_live_review_of_each_producer_it_finishes_and_no_other`: three reviews named, among them a nested one and one the closer cut; a `DONE`, an `ABANDONED` and a `parser-k17` review untouched |
| A direct-harness finish removes a stale line | `a_direct_harness_finish_removes_an_attempt_s_run_and_the_review_refuses_until_declared`, through to the declared review selecting another origin |
| Findings attach after `.grove/` is removed | `a_review_s_findings_attach_to_the_producer_s_run_after_the_tree_is_removed` |
| The shipped review example is what runs | The `Lifecycle` fixture's policy is the single line `export { policy } from "harness-dispatch/examples/grove-review";` |
| Standalone `grove run` refuses a lifecycle slot | `crates/grove/tests/internal/standalone.rs`, `a_standalone_template_requesting_a_task_slot_refuses_before_launch` |
| `grove config show` prints the slots symbolically, with no task | `config_show::task_slots_render_symbolically_with_no_task_selected` |
| The harness is the foreground job: PID, group, terminal, cwd | `a_dispatched_harness_is_the_foreground_job_grove_launched`, each beside the direct harness |
| The signal state matches a direct harness's | The same test, against a reference with one signal ignored and one blocked |
| The worker has null stdin and no control variable | The same test |
| The harness gets a fresh channel | The same test, which compares it with the previous launch's |
| Exit and signal death reach Grove | `a_dispatched_harness_s_exit_and_signal_death_reach_grove_as_a_direct_one_s` |
| An interrupt during selection and during execution | `an_interrupt_typed_at_the_terminal_ends_a_dispatched_job_as_it_ends_a_direct_one`: no launch, no worker, no store; then the running job |
| Descendant escalation | `the_escalation_reaps_a_dispatched_session_s_descendants_as_a_direct_one_s`, over both routes |

Held more narrowly than stated. That the slots come from the selection and
not from the prompt is observed where `SessionConfig::expand` is tested
(`crates/grove-loop/tests/session_config.rs`), since at the process seam the
two always agree. `config show` is shown to execute nothing with a plain
command, not with a dispatch wrapper. The lifecycle lines are written by the
suite's own shell procedure, as the spec says: a real session's compliance is
the methodology's to check.

**The shipped review policy (seam row 2, and the brief's creator case).**
Test names are relative to `crates/harness-dispatch/tests/`. Citations
checked: `review.rs:254-272`, `grove.rs:268-292`.

| Clause | Test |
|---|---|
| Another origin on every invocation and retry | `review::a_review_selects_a_reviewer_of_another_origin_on_every_invocation`, two launched runs with different run IDs |
| The same rule for an explicit choice | `review::an_explicit_choice_is_held_to_the_same_rule`, `grove::the_rule_holds_on_every_invocation_retry_and_explicit_choice` |
| Same origin refuses and is never replaced | `review::a_mapped_reviewer_of_the_creators_origin_refuses_and_is_never_replaced` |
| A gateway disguise refuses | `review::an_explicit_choice_is_held_to_the_same_rule`, the `gateway-careful` choice |
| A producer names its run, and its review uses the recorded provider | `grove::a_dispatched_producer_names_its_run_and_its_review_uses_the_runs_recorded_provider` |
| A changed mapping cannot rewrite it | `review::a_changed_current_mapping_does_not_change_the_creators_recorded_origin`, and the second half of the `grove` test above |
| An earlier run of the same task is no creator | `grove::an_earlier_run_of_the_same_task_does_not_stand_in_for_a_missing_creator_line` |
| An unknown run and a run never executed refuse | `review::an_unknown_run_or_a_run_that_never_executed_refuses_with_the_declaration_remedy` |
| Declaration adoption, labelled declared | `review::a_declared_creator_is_adopted_and_labelled_declared` |
| Missing, duplicate and malformed marker lines | `grove::each_grove_convention_the_adapter_interprets_has_a_fixture`, one fixture and code each; `grove::malformed_marker_lines_refuse_naming_their_line` |
| `Reviews` under an unlisted kind | The same fixture table, `review_kind_unlisted`; `review::a_custom_review_label_applies_the_rule_only_when_the_owner_lists_it` |
| A relabelled origin and a misspelt declaration | `review::a_relabelled_origin_and_a_misspelt_declaration_refuse_as_non_members` |
| The generic form, no task file, no Grove | `review::the_generic_form_selects_with_no_task_file_or_grove` |
| A run of another task identity is admitted and shown | `review::a_run_of_another_task_identity_is_admitted_and_shown` |
| The adapter reads only its task file | `grove::the_adapter_reads_only_the_task_file_it_is_given`: a file named for another kind and handle, and the measured sources listed |
| The adapter's version, exactly when imported | `grove::the_adapter_version_is_reported_exactly_when_the_policy_imports_it` |
| Tests use the shipped artifacts | Every policy imports by specifier, as `export { policy } from "harness-dispatch/examples/review"`; `examples::only_registered_example_specifiers_resolve` is the control |
| Declarations and sources ship | `examples::each_example_ships_declarations_and_a_readable_source_beside_the_worker` and its three siblings |
| The starters route exactly | `examples::the_grove_example_routes_every_grove_session_kind_exactly`, `examples::the_generic_example_routes_its_own_kinds_without_grove` |

"The generic core has no Grove review-kind list or `Reviews` parser" has no
test. I checked it by search with both controls. `Reviews` and `Creator`
occur nowhere under `crates/harness-dispatch/src`. The same search finds the
parser in `worker/grove/index.ts`, and a search for `reviewedArtifact` finds
the core's generic field, so the instrument reads that tree. Every `review-`
there is an example in `--help` text.

Held more narrowly than stated. The gateway case refuses, but nothing asserts
that the gateway candidate's program and model differ from the creator's, so
it would pass if the example lost its disguise. No case sets a candidate's
argv against its label. Marker-line refusals other than `creator_line_missing`
are driven through `inspect` only.

**What a test cannot hold.** These clauses of the acceptance cases are the
documentation-acceptance review's, which `usage-agreement-k69` cuts: that
inspection does not promise side-effect-free evaluation; that the guidance
says Grove's pre-authoring check stops at the configured command; the
personal activation instructions; which configuration owns selection and how
to remedy a refusal; and the stated floors. The methodology half of the
creator convention is held by the plugin's conformance rows, which ran in the
same `task check`: `finishing-session-names-its-run` and its three controls
in the conformance suite, and
`the_creator_reference_amendment_is_stated_where_each_rule_lives`.

**Result: every acceptance case and every seam clause has a passing
instrument, so nothing is cut ahead of the documents.** The spec may describe
the whole command as current state.

**The narrower holds are one leaf, outside this node.** None is a missing
instrument, and each behavior is held by a unit test or by a neighbouring
case. They are still claims a test's name makes and its body does not
observe, which is the obligation every implementation leaf here carried. I
cut `seam-controls` at the grove root for those a small change closes. It
sits beside the two flake leaves, because the documentation node's charter
is not test work and the documents do not wait on it.

**Accepted as visible trade-offs, with no work cut.** Waiting out the
30-second default and the 120-second ceiling costs that long on every run;
both are asserted as the effective bound. The grace is timed loosely to stay
free of flakes. The start directory's mode cannot exceed `0o700` under any
umask, since a umask only removes bits. The front's own stdin as a terminal
is Grove's PTY suite's to show. The import, shape and marker refusals run
through `inspect` because `run` evaluates the same selection, and one case of
each class does run. `config show` executes no configured command at all, so
a dispatch wrapper adds nothing to show. The adapter's "enumerates nothing"
cannot be observed against a runtime with an unrestricted file system; its
measured sources and its import lines are what the test can see.

**Installed smoke.** `task release:smoke` rebuilt the three archives from the
same working copy and passed on `aarch64-apple-darwin`,
`aarch64-unknown-linux-gnu` and `x86_64-unknown-linux-gnu`: "all 4 case(s)
passed through both fronts" on each, and twice on Linux arm64, as a native
container and under `-cpu cortex-a53`. The worker is build `721aab0848f6…`,
the one `package-json-autoloading-k66` recorded. Each control fired and
printed its observation. The glibc 2.25 probe was refused for
``version `GLIBC_2.25' not found`` in all three Linux userlands. The CPU
probe was killed with exit 132 under `-cpu cortex-a53` and under
`-cpu Nehalem`, and ran under `-cpu max`.
