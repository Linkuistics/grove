# lifecycle-cutover-review-k40

**Kind:** review-impl
**Reviews:** lifecycle-cutover-k39
**Producer launch:** {"producer":"lifecycle-cutover-k39","session":"lifecycle-cutover-k39","generation":"k39","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `lifecycle-cutover-k39` and record concrete findings for its integration step.

## Context

- Review `lifecycle-cutover-k39` against the exact ordered flow in the spec and
  the complete-session-configuration ADR.
- Attack hidden launch policy, config-validation timing, double selection,
  mutation before tool/version/config checks, direct-exec argv boundaries,
  prompt authority, config reload, launch-window insertion, and status/elapsed
  diagnostics.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `lifecycle-cutover-integrate-k41` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity, observable fake-command/tree
  evidence, and the threatened contract, or an explicit no-finding result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded invalid-config and mandate-versus-insert evidence rather than
  re-running it.
- No production or test code is changed.

## Findings

Inspected: commit `lifecycle-cutover-k39: drive bare config-defined sessions`
(1415+/194-, 13 files); `src/launch.rs` :1-30 and :254-306; `src/loop_driver.rs`
:96-264, :429-446 and :676-724; `src/tree_read.rs` :46-110 and :640-665;
`src/cli.rs` :73-166; `src/provision.rs` :28-60 and :120-130;
`src/session_config.rs` :15-115; `src/tree_access.rs` :20-110;
`src/tree_lifecycle.rs` :53-85; `src/repo.rs` :168-268; `src/llm_cli.rs`
:416-420; `src/driver_lease.rs` :119-232 and :754-800; `content/prompts/`;
`tests/lifecycle_cutover.rs` in full; the `tests/driver_lease.rs`,
`tests/loop_driver.rs`, `tests/launch.rs`, `tests/jj_tree_verbs.rs` and
`tests/migrate.rs` diffs; `docs/adr/complete-session-configuration.md`;
`docs/specs/config-driven-sessions.md` §Solution (:18-59), §Configuration file
(:61-214), §Session kinds live in filenames (:216-274), §Authoritative selection
and mandate (:276-322), §Single-command lifecycle (:324-579), §Removed surfaces
(:694-740) and §Test seams (:785-875).

The producer recorded no verification prose — its commit subject is a single
line with no body — so its evidence is the committed suite itself: the
invalid-config evidence is `invalid_config_cannot_create_a_fresh_grove`
(`tests/lifecycle_cutover.rs:194`) and
`invalid_config_leaves_legacy_current_empty_and_pending_trees_byte_identical`
(:213, which byte-snapshots all four tree states); the mandate-versus-insert
evidence is `insertion_during_launch_does_not_change_the_session_mandate`
(:414); with `relaunch_reloads_config_and_uses_the_new_filename_kind` (:344),
`config_is_reloaded_after_a_completed_legacy_transition_before_launch` (:769),
`nonsignalled_nonzero_exit_reports_status_elapsed_and_launch_identity` (:511),
the three helper-check tests (:591, :623, :659) and
`select_returns_path_handle_and_kind_from_one_guarded_observation`
(`src/tree_read.rs:643`). None of it was re-run; no production or test code was
changed by this review. One independent check was made outside the repository —
`git config core.worktree` precedence in a throwaway scratch directory, for F1 —
touching no project file and running no project build, test, lint or format
command.

### F1 — medium — a deleted fixture line, not a passing assertion, is why the foreign-Git-environment test is still green

The diff removes the hostile-configuration half of
`driver_launch_uses_the_on_disk_worktree_despite_foreign_git_environment`
(`tests/driver_lease.rs:284`):

```
-    run_command("git", &intended,
-        &["config", "core.worktree", foreign.to_str().unwrap()]);
```

That line was load-bearing. The old session spawn ran
`repo::anchor_git_worktree_environment` (`src/repo.rs:191`), which *sets*
`GIT_WORK_TREE=<worktree>` precisely because, as its own doc comment states,
"Setting `GIT_WORK_TREE` also overrides a hostile `core.worktree`". The new
`launch_configured_session` (`src/loop_driver.rs:242`) calls only
`scrub_loop_control_env`, which *removes* all three selectors and sets none, so
`core.worktree` is honoured again.

Failure scenario: `intended/.git/config` carries `core.worktree = <foreign>`
(a stale clone, a submodule, a `git worktree` fixup, or a hostile checkout).
Bare `grove` leases and launches in `<intended>`, and the configured session's
first `git rev-parse --show-toplevel` reports `<foreign>` — the session then
reads, edits, and commits the wrong repository while the driver's own lease,
tree lock and migration commits stay correctly in `<intended>`. Verified in an
isolated scratch repo: with `core.worktree` set and `GIT_DIR`/`GIT_WORK_TREE`/
`GIT_COMMON_DIR` all unset, `--show-toplevel` returns the foreign path; with
`GIT_WORK_TREE` exported it returns the intended one. Restoring the deleted
fixture line therefore fails the test's surviving assertion, which still reads
`"foreground harness inherited a foreign Git worktree"`.

Threatened contract: the test's own stated guarantee, and the
`anchor_git_worktree_environment` invariant it was written to pin. Note the
spec pushes the *other* way — §Command-template grammar (:168) forbids Grove
adding "hidden harness-specific arguments or environment values", so continuing
to inject `GIT_WORK_TREE` is not obviously right either. What is not acceptable
is losing the property by editing the fixture that could detect the loss.
`lifecycle-cutover-integrate-k41` should restore the fixture and then choose
explicitly: either re-scope and rename the test to what a config-driven launch
actually guarantees and record the dropped defence in
`docs/specs/config-driven-sessions.md`, or keep the guarantee by a means that is
not environment injection.

### F2 — medium — removing the Git selectors from the configured session is undocumented, unasserted launch policy

`REPOSITORY_CONTEXT_ENV` (`src/launch.rs:278`) is chained into
`scrub_loop_control_env` (:302), so the user's opaque configured command now
runs with `GIT_DIR`, `GIT_WORK_TREE` and `GIT_COMMON_DIR` unset. The spec
scopes scrubbing narrowly: §Removed surfaces (:710) says the driver "continues
to scrub ambient control variables from **non-foreground child commands** and
grants only its own signal path to the real foreground session", and
§Command-template grammar (:171) enumerates what Grove owns as "its temporary
loop-control channel, child lifecycle, current directory, and the generated
prompt". Ambient repository context is in neither list, and
`docs/adr/complete-session-configuration.md` rejects exactly this class of
invisible target shaping.

Failure scenario: a user launches bare `grove` from a context that legitimately
exports `GIT_DIR` — a Git hook, a `git rebase --exec` step, a wrapper that
pre-resolves the repository — and their configured wrapper, written against
that contract, silently resolves a different repository than it does when run by
hand. Nothing in `config.kdl`, `--help`, or the docs mentions the removal.
It is also unasserted: `bare_grove_launches_the_selected_filename_kind_with_one_mandate_argument`
(`tests/lifecycle_cutover.rs:109`) records only the five `GROVE_*` names in the
child, so deleting `REPOSITORY_CONTEXT_ENV` again would leave every test green.

Decide with F1, since they are the two halves of one question: either state
repository context as Grove-owned in the spec and assert the child's `GIT_*`
values through the bare seam, or confine `REPOSITORY_CONTEXT_ENV` to the
driver's own non-foreground children and leave the configured command's
environment untouched apart from the loop-control channel.

### F3 — medium — the three driver-derived substitutions are never observed through the bare seam

`${session_name}`, `${worktree}` and `${repo}` are the only expansion values the
driver *derives* rather than receives: `bare_grove` (`src/launch.rs:14-24`)
resolves them from `driver_lease.worktree_root()` and `repo::main_repo_of`, and
`run_configured_loop_with_lease` (`src/loop_driver.rs:120-127`) composes
`format!("{repo_name}: {name} grove")` from the *main repo's* basename.
`tests/session_config.rs` supplies its own `ExpansionContext` literals, so it
proves the template mapping and never the derivation; no test in
`tests/lifecycle_cutover.rs` uses any of the three.

Failure scenario: in a linked Git worktree or a secondary jj workspace — the one
shape where `${worktree}` and `${repo}` genuinely differ, and the shape the spec
defines `${repo}` against (:142) — a `main_repo_of` that returned the worktree
would hand the session both a wrong `${repo}` and a wrong `<repo-basename>`
prefix in `${session_name}`, and the whole suite would stay green. Every
acceptance fixture calls `init_git_worktree` on a plain checkout, where
worktree and repo are the same path, so the two values are observationally
indistinguishable today. The spec's seam list (:794-796) asks for exactly this
through the bare process — "scalar substitution, paths with spaces, prompt in
non-final position, literal `env` as word zero, and absence of shell
evaluation"; only the middle two are covered there.

`lifecycle-cutover-integrate-k41` should add one bare-seam case in a linked Git
worktree (`tests/driver_lease.rs:353` already builds both that and the secondary
jj workspace) that pins the exact `${repo}`, `${worktree}` and `${session_name}`
argv words, plus one `env VAR=value runner … ${prompt}` word-zero case proving
the assignment is an ordinary argument and no shell evaluates it.

### F4 — low — the bare lifecycle has no jj coverage at all

Every fixture in `tests/lifecycle_cutover.rs` is `init_git_worktree`. The spec's
primary seam (:786) is "the bare `grove` process in isolated temporary Git,
native jj, and colocated jj worktrees", and the bare path takes a jj-specific
branch twice: `repo::main_repo_of` (`src/repo.rs:170`) shells out to
`jj workspace root --name default`, and `transition_to_current`
(`src/tree_lifecycle.rs:71-73`) commits migration through the jj fileset path.
Lease behaviour *is* covered for jj (`tests/driver_lease.rs:353`) and
`tests/jj_tree_verbs.rs` covers the tree verbs, so the gap is specifically the
bare lifecycle: a jj-only regression in repo resolution or the migration commit
ships green. Partly mitigated in practice because this repository dogfoods a
native jj tree.

### F5 — low — "could not spawn" exits nonzero, "spawned and failed" exits zero

`spawn_failure_names_the_kind_executable_and_config_without_retiring_the_leaf`
(`tests/lifecycle_cutover.rs:460`) asserts `!status.success()`, while
`nonsignalled_nonzero_exit_reports_status_elapsed_and_launch_identity` (:511)
asserts `status.success()` after the configured command exits 23. Both are
configured-command failures the driver diagnoses in near-identical prose
(`src/loop_driver.rs:159-178`). The spec is silent on the driver's own exit
status, so nothing is violated — but anything wrapping bare `grove` (a shell
retry loop, a launchd job, CI) cannot distinguish "the session failed" from "the
session finished without signalling", which is the distinction the diagnostic
itself draws. Worth deciding and recording rather than inheriting.

### F6 — low — a spawn failure leaks its freshly allocated signal channel

On the `launch_configured_session` error path the `?` at
`src/loop_driver.rs:170-177` leaves the loop before
`remove_signal_channel` (:179), so the `signal-*` file drawn at :161 stays in the
control directory. The epoch *is* invalidated first —
`complete_post_reap_epoch_handoff` (:429) calls `invalidate()` before returning
the launch error — so this is hygiene, not a correctness hole, and the next
driver's `cleanup_abandoned_signal_channels` (`src/driver_lease.rs:164`, from
`acquire`) removes it. The spec (:412) only requires removal *after* epoch
invalidation and signal interpretation, and says nothing about the failure path.
Flagged so k41 can decide deliberately rather than by omission.

### Observation — the shipped prompt contradicts itself until `lifecycle-methodology-k79`

`mandate_prompt` (`src/loop_driver.rs:216-221`) appends "do not call
`grove-llm pick`" to `content/prompts/continue.md`, which still ends with "if
`pick` is empty, propose the finish cycle instead", and the provisioned
`content/SKILL.md` still opens its loop with "**Pick.** Run `grove-llm pick`".
So from this commit forward every launched session receives an explicit mandate
wrapped in guidance telling it to do the opposite. This is *owned* — the Done
when of `lifecycle-methodology-k79` is "Bootstrap resolves the prompt-mandated
stable handle without picking", and its chain is deliberately sequenced after
`legacy-review-removal-integrate-k65`. Recorded so k41 does not duplicate that
work, and so the window is a conscious ordering choice rather than an oversight.
No action proposed here.

### Checked and clear

So `lifecycle-cutover-integrate-k41` need not re-derive these:

1. **Flow order** matches the spec diagram (:38-54) exactly — provision, lease,
   `grove-llm` version, config validate, at most one transition, one select,
   config reload, expand, spawn. `version_skew_from_path_fails_before_tree_creation`
   writes *no config at all* and still fails on skew, which is a positive proof
   that the version check precedes config load; all three helper-check tests
   additionally assert `!.grove/exists`.
2. **No double selection.** One `tree_read::select` per iteration
   (`src/loop_driver.rs:137`); the `grove-llm kind --with-harness --json` peek
   (:1440) is unreachable from the bare path; `select_unlocked`
   (`src/tree_read.rs:70`) copies path, handle and kind under a single guard and
   `select_returns_path_handle_and_kind_from_one_guarded_observation` (:643)
   pins `acquisition_count() == 1`.
3. **Guard lifetimes and lock order.** `select` returns owned data so the shared
   tree guard is dropped before the second config load and the spawn, as
   §Single-command lifecycle (:503-506) requires; `activate_session_epoch`
   (`src/loop_driver.rs:246`) is taken after `select` has returned, so no epoch
   guard is ever acquired while a tree guard is held.
4. **Pre-mutation config failure is byte-safe** for rootless, legacy, current,
   empty and pending-migration trees — the load at :134 precedes
   `transition_to_current` at :136, and the two invalid-config tests snapshot
   and compare every one of the five states.
5. **Config reload** happens unconditionally at :144 after any transition and on
   every iteration; the relaunch test proves the *kind* is re-derived from the
   new filename too, not just the template.
6. **Direct execution, no shell.** `Command::new(executable).args(arguments)`
   (:243) with the worktree as cwd; the argv test pins `argc=<3>` around a
   non-final `${prompt}` and uses spaces in the home, worktree, executable and
   log paths.
7. **Mandate authority.** `${prompt}` is one argument carrying the launcher plus
   the stable handle; a leaf inserted during the launch window neither changes
   the mandate nor is lost (:414 asserts both).
8. **No hidden target/harness/model injection.** The argv test exports a stale
   `GROVE_SESSION_TARGET` on the driver and observes `target=<unset>` in the
   child, with `GROVE_HARNESS_BIN`, `GROVE_IMPL_MODEL`, `GROVE_SKILL_DIR` and
   `GROVE_LLM_BIN` likewise unset; no stamp is written
   (`tests/driver_lease.rs:327`); no pre-flight or codex sandbox probe runs on
   the bare path.
9. **Status/elapsed diagnostics** name status, elapsed, kind, word zero and
   config path (:531-538). Generalising `complete_post_reap_epoch_handoff` over
   `E` left its ordering intact: invalidation precedes signal interpretation,
   and a launch error survives an invalidation failure.
10. **Metadata-only front door.** `--help`/`--version` exit inside `Cli::parse()`
    (`src/cli.rs:164`) before `bare_grove`, provisioning nothing; `grove do` and
    `grove retire` are rejected. Legacy `do_grove` survives only through a
    test-only re-entry harness (`legacy_grove_do_command`), off the bare path as
    the task requires.
11. **`grove-llm --version` really is epoch-exempt**, so `checked_grove_llm`
    (`src/loop_driver.rs:681`) spawning it unscrubbed cannot trip admission:
    `admit_ambient_session` is called at `src/llm_cli.rs:419`, *after*
    `Cli::parse()`, and clap exits during parse for `--version`.
12. **Kind/target totality.** `Kind::label()` (`src/leaf.rs:293`) and
    `session_config::REQUIRED_KINDS` (`src/session_config.rs:15`) are the same
    nineteen spellings, so every selectable kind resolves a target and
    `expand`'s "no target for session kind" arm is unreachable from `select`.

## Notes

The reviewer produces findings only; `lifecycle-cutover-integrate-k41` owns
fixes. Finish behavior remains explicitly out of this review.
