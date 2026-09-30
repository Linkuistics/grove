# ambient-authority-k30

## Goal

Prove, through the public launcher, that running dispatch in a hostile
directory or environment executes no repository code and grants no authority.
The owner's explicit choices must still be admitted.

## Context

The spec's `#policy-authority` owns the environment rules and the `--policy-env`
contract. The spec's firing-configuration table and the runtime evidence name
which probe build makes each hostile class fire. The skeleton built the
controls. This leaf proves them and adds the explicit grant.

## Done when

- `--policy-env NAME` is repeatable and grants exact names beyond the base
  environment. The names below are always excluded from grants:
  - `BUN_*`, `NODE_OPTIONS` and `NODE_PATH`;
  - dynamic-loader injection variables;
  - the private protocol variables.
  Inspection shows granted names, never values.
- Tests show the worker, and a normal child it spawns, lacking a caller's
  `GROVE_SIGNAL_FILE` and other completion values unless the value is
  explicitly granted. The usage documentation warns against granting
  `GROVE_SIGNAL_FILE`.
- A worker-shaped executable on PATH or in the cwd is never used, and
  installation control never comes from ambient input.
- Each hostile class is tested beside its firing configuration, with a probe
  build the Taskfile makes for tests only:
  - a cwd policy entry;
  - a cwd `.env` and bunfig preload;
  - a `BUN_OPTIONS` preload;
  - a `node_modules/harness-dispatch` shadow beside an admitted entry;
  - tsconfig `paths` beside an admitted entry.
  The class stays inert through the public launcher, and its firing
  configuration is seen to fire in the same run. The HOME bunfig and cwd
  tsconfig classes are reported as having no known firing configuration.
- An explicit relative `--config` and a personal policy that imports a repository
  entry are admitted. Inspection identifies the authority. Each documented
  specifier resolves to its embedded module.
- A policy that floods stdout and stderr leaves `--json` output and the private
  protocol intact.
- `runtime-evidence.md` records which controls were seen to fire against the
  shipped launcher, on which host and Bun version. The archive assertions
  confirm that no probe build ships.
- This node's `Done when` holds. Retiring this leaf closes the node. As this
  leaf's last act, cut the node's `review-impl` as the node's sibling,
  directly after it, never inside it. Run `grove-llm leaf-insert --kind
  review-impl grove-dispatch-k31 evaluation-boundary`, targeting the first root
  entry after this node (today `grove-dispatch-k31`). Give it
  `**Reviews:** evaluation-boundary-k27`, and write the node brief's review doubts
  into its body.

## Decisions (running log)

**The private protocol variables are the `HARNESS_DISPATCH_` namespace, and
loader injection is every `LD_*` and `DYLD_*` name.** The channel is named by
no variable, so the only variables dispatch owns are the two it exports to the
harness, `HARNESS_DISPATCH_RUN_ID` and `HARNESS_DISPATCH_STATE_DIR`. Granting
the second would tell the worker where the record store is, which the spec
says it is never told; granting the first hands it a stale caller value. The
whole prefix is reserved so that a later variable is excluded without an edit.
`LD_*` is ld.so's namespace (glibc reads only `LD_` names; musl reads
`LD_PRELOAD` and `LD_LIBRARY_PATH`), and `DYLD_*` is dyld's. Excluding by
prefix, with the underscore, leaves `LDFLAGS`, `BUNDLE_GEMFILE` and
`NODE_EXTRA_CA_CERTS` grantable, and the tests grant those as near misses.

**An excluded name refuses as `excluded_grant`, exit 2, stage `cli`, input
`--policy-env`.** It is a command-line input that can never be valid, like a
malformed one, but a distinct code tells an owner that the name, not its
spelling, is the problem. An empty name or one containing `=` is
`malformed_input`. A name that is not UTF-8 is `malformed_input` too, since
inspection could not show it exactly. Duplicates are one grant. A grant of a
name the caller has not set is admitted, and inspection says it is not set.

**Inspection reports `policyEnv`: each granted name, in first-seen order, with
whether the caller had it set, and never its value.** Text shows it as a
`policy env` row. The run record does not carry grants: the spec's field list
names none, and says raw environment values are not stored.

**A probe build reports the identity `probe-<name>-<source digest>`, which no
front admits, and its firing configuration drives it directly.** The three
probes are the shipped source compiled with one control removed: `autoload`
(dotenv and bunfig autoloading on), `tsconfig` (tsconfig and package.json
autoloading on) and `unregistered` (no embedded-module registration, selected
by a `HARNESS_DISPATCH_PROBE` define that the shipped build sets empty, so both
builds constant-fold it). Because a front refuses a probe's identity with exit
5, a probe can never serve as an installation's worker, and an archive whose
worker was a probe fails the installed smoke test, which inspects through the
archive's front before a release publishes. The firing configurations
therefore run the probe with a test-side protocol driver, which the spec's
table allows: none of them names the front. The probes are built into
`target/probes/harness-dispatch/<name>/`, outside the libexec layout.

**The worker's environment moved into its own module, `environment`, and
`worker::evaluate` takes it as a parameter.** The base set, the grants and the
exclusions are one rule, so one module owns it; the worker module only starts
the process. `--policy-env` was the last input refused by name, so
`SelectionArgs::refuse_unsupported` became `refuse_empty`, and `cli::unsupported`
and the `unsupported_input` code are gone.

**`BUN_BE_BUN` is proved beside `BUN_OPTIONS`, as a second `BUN_*` class.**
Seen on the pinned worker: with `BUN_BE_BUN=1` it is Bun itself and runs any
`-e` code. The spec's table named only `BUN_OPTIONS`; installation control is
the brief's words, and this variable changes what the installed worker is.

**The shipped worker reads no `package.json` at run time, so a package whose
entry `main` or `exports` declares does not load. This leaf records it and
does not decide it.** Found while building the shadow fixture: an
`exports`-only `node_modules/harness-dispatch` never loaded, under any build.
Measured through the shipped front: a package with an `index.js` loads, one
with `"main"` or only `"exports"` refuses "Cannot find package", and plain
`bun` loads all three. It is not an authority hole, since it only refuses
more. It does contradict the spec's own sentence that bare specifiers resolve
through `node_modules`. The question is precise, so it is a leaf,
`package-entry-resolution-k52`, inserted before `dispatch-documentation-k41`.
The README and the spec state the limitation as current state, and the shadow
fixture is laid out as files.

**Every new test was seen to fail against a mutated front or worker build**,
and each mutated file was restored byte for byte after each case, digest
checked (`environment.rs`, `worker.rs`, `dispatch.sh`), with the worker and
probes rebuilt afterwards. A baseline run passed all 27 tests of `environment`,
`hostile`, `worker` and `authority`. Handing the worker the caller's whole
environment failed the base-set, grant, completion and Bun-variable cases, and
the existing private-directory case. Ignoring grants failed the grant case and
the completion case's control. Disabling exclusion failed the excluded-name
case. A grant's value in `policyEnv` failed the never-shown case and the grant
case. Starting the worker in the caller's cwd failed only the existing
private-directory case: the dotenv case stayed green, because the compile
switches alone hold. Building the shipped worker with dotenv and bunfig
autoloading on failed the dotenv case only at its arm that runs the shipped
worker directly in the hostile directory: through the front it stayed inert,
because the private cwd alone holds. Both mutations together failed the
dotenv case at its public-launcher arm. A shipped build without registration
failed the specifier case. A shipped build with tsconfig and package.json
autoloading failed the tsconfig case. Probes carrying the shipped identity
failed the probe-identity case and every probe's identity check. Worker stdout
inherited rather than captured failed the flood case. A worker located from
`argv[0]` failed the argv[0] case. The two no-firing-configuration cases are
tripwires, which by definition have never been seen to fire, so they are
reported and not counted.

**The spec's acceptance rows this leaf owns map to named tests.** In the
authority and lifecycle row: hostile cwd policy is
`authority::no_cwd_search_or_environment_variable_selects_an_entry`; dotenv and
bunfig/preload `hostile::a_cwd_dotenv_and_bunfig_preload_stay_inert_and_fire_under_the_autoload_probe`;
BUN_OPTIONS `hostile::bun_runtime_variables_stay_inert_through_the_front_and_fire_in_the_worker_started_directly`;
package shadow and "a documented package specifier resolves to the embedded
module" `hostile::every_documented_specifier_resolves_to_its_embedded_module_beside_a_package_shadow`;
tsconfig `hostile::tsconfig_paths_beside_an_admitted_entry_stay_inert_and_fire_under_the_tsconfig_probe`;
the unfired classes `hostile::classes_with_no_known_firing_configuration_are_reported_not_counted`;
explicit relative config and personal import `authority::an_explicit_relative_config_resolves_against_the_original_cwd_and_replaces_the_default`
and `authority::personal_policy_may_import_a_repository_entry_explicitly`;
worker and nested child environments `environment::the_worker_and_a_child_it_spawns_lack_completion_values_unless_granted`;
structured diagnostics `hostile::a_policy_flooding_both_streams_leaves_json_output_and_the_protocol_intact`.
Worker-location spoofing is `worker::a_front_without_its_worker_refuses_and_no_ambient_decoy_substitutes`
and `worker::an_argv0_naming_another_prefix_never_relocates_the_worker`, and
that no probe can serve as the worker is
`hostile::a_probe_build_is_never_accepted_as_an_installations_worker`. The
grant contract is the rest of `environment`.

**The archive assertions refuse a probe build by path, and the front refuses
one by identity.** `scripts/release.test.sh` stages an archive with a probe
build beside the worker, under `libexec/harness-dispatch/probes/`, and the
manifest check refuses it by name. It was seen to pass the archive, and so
fail the test, with the probe left out. A probe in the worker's own place
passes the manifest but fails the installed smoke test, whose front refuses its
identity; `hostile::a_probe_build_is_never_accepted_as_an_installations_worker`
shows that refusal.

**Measured on the final source: `task check` passed 11 of 12 principal checks
in one run, and its `cargo test` passed on a rerun.** The sources it reads were
digested identical before and after. The one failure was
`grove-tui`'s `observation::tests::witnessed_launch_identity_change_is_compared_even_when_tree_capture_fails`:
its fixture's `try_observe` did not report `Running`. No change here touches
`grove-tui` or `grove-loop`. It passed five isolated reruns, and a full
`cargo test --locked --workspace` on the same source passed every suite. It is
recorded as a load-dependent flake outside this workstream, for the human,
rather than grown into this tree. A second whole `task check`, after the
release smoke, passed all 12, on the same digested sources.

**`task release:smoke` passed on the final source.** It rebuilt archives of
version 21.12.0 whose workers report build `ad1b0271bbc3…`, the build the
command-seam tests ran against, and passed the static, computed and
`signal_state` cases through both fronts on every target. macOS arm64 ran
natively. Linux arm64 ran in its native CentOS 7 container and under QEMU at
the Cortex-A53, and Linux x64 under QEMU at Nehalem. The glibc and CPU
controls fired on each Linux run. The sources it reads were digested identical
before and after. The smoke runs no hostile fixture, so the Linux targets
remain unmeasured for this leaf's controls, as the runtime evidence says.
