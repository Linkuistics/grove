# Confinement withdrawal evidence

On 2026-11-03, during `stage-evidence-reconciliation-k125`, the human withdrew
operating-system confinement from Ravel and from Grove. Ravel removes
`--confine`, `--runtime-read` and both backends before the 2.0.0 cut. Grove
stops requesting confinement. After the removal, the harness's own sandbox is
the only boundary of a managed process.

`confinement-withdrawal-k142` designed the removal, and this record gives
Grove's part of that design. It covers the guarantees of a standalone run, the
release-notes route and the version consequence. It also covers the
dispositions of the acceptance criteria and the handoff to
`grove-confinement-removal-k146`.

Ravel's part is the "Withdrawal of confinement" section of Ravel's
`docs/skill-discovery-design-evidence.md`. It is in Ravel commit
`3642a5d628fdc9c53db2f46837ccf8f719242f04`. Its parent `74885e37` converts
that record to controlled English. Its child `2df9dd055824` integrates the
review, and that child is the tip of the unpushed bookmark
`skill-directory-delivery`.

## The design

The normative changes are in the specifications that own each rule:

- Ravel's decision record "Ravel does not confine a session" states the
  decision, its trade-off and the rejected alternatives.
- Ravel's session execution specification states the refusal
  `confinement_withdrawn`, stage `cli`, exit 2, for `--confine` and
  `--runtime-read`, with its remedy.
- Ravel's skill discovery specification states the read access to skill
  resources without grants.
- Ravel's configuration evaluation specification states that a harness run of
  a configuration script is never confined by Ravel. The script selects the
  sandbox of the harness through `args`.
- The [standalone invocation contract](specs/standalone-invocations.md) states
  Grove's request, the effect of `--runtime-read`, the environment of the job,
  the cancellation and [the harness sandbox](specs/standalone-invocations.md#the-harness-sandbox).
- The [caller contract](specs/harness-selection-and-execution.md) and the
  [delivery design](specs/codex-skill-provisioning.md) drop the confined
  standalone caller.

Grove's request is the same for Ravel 1.1 and 2.0:
`start-session --kind=KIND --prompt=PROMPT --exit-dir=DIR`. Ravel 1.1 keeps
`--confine`, and Grove does not send it. The lifecycle request does not
change, because it never held `--confine`.

## Guarantees of a standalone run

Each guarantee that a confined `grove run` gave has one disposition.

| Guarantee of a confined `grove run` | Disposition | Reason |
| --- | --- | --- |
| A private staged directory, copied inputs and an export of only the declared outputs after a successful end | Kept | Grove owns them, independent of any sandbox |
| A held directory defends the export against a renamed working directory | Kept | The held descriptor does not depend on a sandbox |
| No launch directory or control variable of an enclosing session reaches the job | Kept | Grove's scrub and Ravel's six launch controls remove them |
| No repository selector reaches the harness | Moved to Grove | Ravel's confined environment kept seven names only. Grove now removes `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR` and `GIT_INDEX_FILE` |
| No multiplexer handle reaches the harness | Weakened, and moved to Grove | Grove removes `TMUX`, `TMUX_PANE`, `ZELLIJ`, `ZELLIJ_SESSION_NAME` and `ZELLIJ_PANE_ID`. A socket stays reachable at its default path unless the harness sandbox denies it |
| No terminal, and standard input at end of file | Kept | Grove runs Ravel as a detached job in its own POSIX session |
| No read of the owner's Ravel configuration and settings | Withdrawn | A harness reads them unless its own sandbox denies reads. Codex in `--sandbox workspace-write` reads every path |
| No read of an unrelated project or of the parent project | Withdrawn | The same reason. The prompt still tells the agent to work only on the staged files |
| Writes only in the working directory and the private run directory | Moved to the harness | The harness sandbox that the configuration selects limits writes |
| Native guidance under HOME stays unreadable without an explicit grant | Withdrawn | The harness reads its native roots, as in a lifecycle session |
| Snapshot read grants on the directory route | Moved to the harness | Ravel publishes readable paths, and the configuration gives the harness read access to them |
| Explicit read-only `--runtime-read` grants | Withdrawn | Every file that the owner can read is readable. The flag stays and has no effect |
| A nested cancellation ends at once, inside the grace of an outer supervisor | Withdrawn | Ravel now forwards the signal and sends KILL after its kill-grace of 5 seconds. A nested run can outlast a shorter outer grace, and Grove then publishes nothing |
| A missing sandbox refuses the launch, with no unconfined fallback | Removed | No launch asks for a sandbox |

The change from the seven names of Ravel's confined environment to the caller's
environment has one more effect. The harness receives every other variable of
the caller, such as an API key, as a lifecycle session does.

## The harness sandbox of a standalone kind

The configuration script of the owner selects the harness sandbox of each
standalone kind, as it selects the harness, the model and the effort. Grove
builds no harness argument. Grove's sample routes only one standalone kind,
`release-notes`, through the Codex helper below. The lifecycle routes of the
sample do not change, because no lifecycle launch was confined.

The ending command of the agent writes the result channel beneath the private
`control` directory, inside the system temporary directory. Codex in
`--sandbox workspace-write` writes in `TMPDIR`. The trial record shows that a
lifecycle Codex session in that mode reaches its channel. A standalone channel
has a different parent, so the release-notes check below measures it once.

## The release-notes route

`scripts/release-notes/codex-headless.sh` changes in four ways:

1. It runs Codex with `--sandbox workspace-write` in place of `--sandbox
   danger-full-access`. Codex's own sandbox could not run inside Ravel's.
2. It uses the owner's Codex home. It copies no `auth.json` into the staged
   directory, so a token refresh changes the owner's own file.
3. It sets no `SSL_CERT_FILE`. That bundle replaced a Keychain service that
   only Ravel's profile denied.
4. Its sign-in check gives the remedy `codex login`. It names no grant.

It keeps `--ephemeral`, `--ignore-user-config`, `--ignore-rules` and
`--skip-git-repo-check`. `scripts/release-notes.sh` stops reading
`GROVE_RELEASE_RUNTIME_READ` and passes no `--runtime-read`. The variable has
no effect after the change.

## The version consequence for Grove

The change fits the project's default minor release. `grove run` still accepts
`--runtime-read`, so no command that worked before fails after the change. A
command that failed before only because its sandbox failed now runs. The
release changes no output format, exit status or other flag.

The withdrawal weakens documented guarantees, and the changelog entry must
state that plainly. A standalone harness can now read the owner's
configuration and the parent project unless its own sandbox denies the read.
The human decided that trade-off. No major release is necessary, so this
design asks the human no version question.

A removal of the flag would break each command that passes it. That removal
needs a major release, and this design does not propose it.

## Dispositions of the acceptance criteria

| Criterion | Where | Disposition |
| --- | --- | --- |
| "Confined standalone" | Delivery specification | Replaced by the row "Standalone". The confined results of 2026-11-03 and earlier are historical |
| "Native-only bootstrap on 1.1", its sentence on standalone proof under actual confinement | Delivery specification | Withdrawn. The standalone caller reads the same native roots |
| The trial record names the confinement mode | Delivery specification | Changed to the harness sandbox mode |
| The Linux floor runs the directory part of "Standalone" on the 2.0 lane | Delivery specification | Kept on the general floor route, without the confinement probes. See [Linux floor evidence](#linux-floor-evidence) |
| The Linux floor runs the native part of "Standalone" on each runtime | Delivery specification | Withdrawn. Its floor case needed the sandbox to make the native copies unreadable |
| "Real confinement" | Ravel's acceptance table | Replaced by "Read access" and "Removed confinement flags" |
| "Harness runs", under `--confine` | Ravel's acceptance table | The fake harness reads the resources outside its working directory |
| "Nested sessions", confinement refusals | Ravel's acceptance table | Withdrawn |
| Readable resources "including confined launches" | Brief of `common-skill-loading-k3` | Withdrawn by the human's decision. The brief states it |
| Fixture tests of "confinement" | Brief of `common-skill-loading-k3` | Withdrawn by the human's decision. The brief states it |
| Evidence of "actual supported confinement/installed layouts" | Brief of `common-loading-acceptance-k29` | Narrowed to the installed layouts. The brief states it |
| `discovery-confined` and `missing-kind-confined`, in both harnesses | Brief of `common-loading-acceptance-k29` and the trial record | Historical. No build after the removal can run them, and the runner loses both cases |
| The record marks each confinement result as historical | Brief of `acceptance-handoff-k106` | Unchanged. `stage-evidence-reconciliation-k125` does it |
| "Design must settle … confinement/delegation" and the seam "lifetime and confinement" | Root brief | Settled by the withdrawal. The root brief keeps the human's words, and this record holds the disposition |

## Handoff to `grove-confinement-removal-k146`

The lists below name each dependent at the design commit. The book and
walkthrough work is a natural second child, as the leaf's notes say.

### Code

- `crates/grove-loop/src/session_command.rs`: replace `confined` with an
  infallible standalone builder that returns the common `start` arguments.
  `PathBuf` leaves the imports.
- `crates/grove/src/standalone.rs`: keep `runtime_read` as an accepted flag
  with no effect. Print one notice on stderr when it is present, and give it
  the help "No effect: Grove no longer confines a standalone run". Call the new
  builder. Remove the four repository selectors and the five multiplexer
  variables from the job environment. Change the comment on Ravel's confined
  environment.
- `crates/grove/src/cli.rs`: describe `grove run` as a run in a private
  temporary directory. Replace "Requires OS confinement" in its help with the
  harness sandbox as the boundary, and remove the sentence on `--runtime-read`
  grants.
- `assets/ravel-policy/provenance.ts`, `provenance.rhai` and `README.md`:
  remove the grant of the original executable for confined callers.
- `assets/ravel-policy/sample/config.rhai` and `sample/policy.ts`: the notes
  writer runs Codex with `--sandbox workspace-write`, and the comments say so.
  The lifecycle Codex arguments stay.

### Tests

In `crates/grove/tests/standalone.rs`:

- `Fixture::run_kind` and `Fixture::run_bare` stop adding `--runtime-read`.
- `standalone_scrubs_enclosing_ravel_and_retired_controls_before_selection`
  drops its `--runtime-read` and keeps its name, which `task test:authority`
  filters on.
- `the_policy_selects_outside_the_sandbox_and_the_harness_runs_inside_it`
  takes a new name. It loses the failed reads of the owner configuration and
  the `confinement_overlap` case, and it checks the unconfined environment.
- `confined_harness_cannot_consume_stdin_or_parent_handles_but_can_read_runtime_grants`
  takes a new name and loses the credential grant. Its `/dev/tty` probe must
  open the device, because `test -r` passes on mode 0666 without a sandbox. It
  checks that the four selectors and the five multiplexer variables are absent.
- `native_copies_outside_every_grant_stay_unreadable_and_a_force_stop_publishes_nothing`
  goes, with `READS_NATIVE_COPIES`, `Fixture::native_copies`,
  `Fixture::run_guided`, `Fixture::transcript_text` and
  `force_stopped_after_unreadable_copies`. A scripted agent that finds no
  native copy and force-stops replaces its force-stop part.
- `ravel_is_found_on_path_independently_of_groves_real_path`,
  `nested_exit_and_cancellation_leave_the_enclosing_dispatched_run_intact` and
  `acknowledgement_names_the_canonical_dispatch_path_when_the_sibling_is_a_symlink`
  drop their `--runtime-read`.
- A new test runs `grove run --runtime-read FILE` and checks the notice and an
  unchanged Ravel request.
- The module documentation and four comments stop naming confinement.

Elsewhere:

- `crates/grove-loop/tests/session_command.rs`:
  `confined_request_grants_only_the_named_runtime_file_and_isolates_the_result`
  goes. A test that the standalone request holds `--exit-dir` and neither
  removed flag replaces it.
- `crates/grove/tests/loop_driver.rs`:
  `installed_ravel_help_exposes_only_the_public_launch_boundary` stops
  requiring `--runtime-read` in the help of `start-session`.
- `crates/grove/tests/directory_route.rs`:
  `a_confined_standalone_run_reads_injected_skills_but_cannot_write_them_or_read_configuration`
  takes a new name. It keeps its delivery assertions and the snapshot write
  probes, and it loses the other probes, the grants and the unconfined
  control. `Lane::standalone` loses its `grants` parameter.
- `crates/grove-llm/tests/removed_surface.rs`: the role of
  `GROVE_RELEASE_RUNTIME_READ` goes with the variable.

### Scripts and Taskfile

- The release-notes route changes as [above](#the-release-notes-route), with
  `codex-headless.test.sh` and `scripts/release-prepare.test.sh`. The fake
  `grove-fixture` stops parsing `--runtime-read`, and the real run drops it.
- `scripts/templates/grove.rb.tmpl` and the `release` comment of
  `Taskfile.yml`: Homebrew's test sandbox no longer meets a nested Ravel
  sandbox. Change both comments.
- The harness trial runner `scripts/harness_trial.py` loses its confined
  set-up: `confined_setup`, `confined_reads`, `CODEX_CONFINED_SANDBOX`,
  `CONFINED_CODEX_HOME`, the certificate bundle, the confined probes and the
  `--runtime-read` of its standalone branch. Codex keeps `--sandbox
  workspace-write` on every route. `scripts/test_harness_trial.py` loses its
  confined cases and checks.
- `scripts/harness-trials/cases/discovery-confined/` and
  `missing-kind-confined/` go. `trial-launch.sh` loses its options for a
  confined launch.
- `scripts/harness-trials/profile-copy.py` and `descriptor-probe.py` stay.
  They are the instruments of the record's measurements, and each reads the
  Ravel source of a recorded commit. This follows the decision of
  `ravel-token-handoff-k141` for `descriptor-probe.py`.
- `Taskfile.yml`: `ravel:commands` and `ravel:directory` change their
  descriptions, and `ravel:directory` loses the run of the removed native
  test. The Linux floor tasks change as [below](#linux-floor-evidence).

### Documents

- `docs/USAGE.md` and `docs/RELEASING.md`: the standalone run, its harness
  sandbox and the release-notes route, without confinement. The section
  "Runtime access" of `RELEASING.md` goes, with every link to it.
- `docs/specs/user-guide-coverage.md`, row G9: remove the runtime grants and
  the mandatory confinement. The row is in a `pending` region of the
  controlled-English conversion, so convert that region first.
- `docs/ARCHITECTURE.md`, `docs/grove-provider-evidence.md`,
  `docs/ravel-target-inputs.md`, `docs/grove-delivery-contraction.md` and
  `docs/adr/a-review-carries-its-creator-reference.md`: remove the confined
  standalone request and the grant of the original executable. Convert the
  decision record first, because the checker finds ten problems in it.
- `docs/design/harness-selection-and-execution/`: the launch and delivery
  views drop the confined standalone caller.
- `plugins/grove/skills/configure-grove/references/policy.md` and
  `plugins/grove/skills/grove/references/execute.md`: the standalone request
  without confinement. The rule against an inherited terminal, multiplexer
  connection or completion authority stays. Grove's scrub and detached job now
  enforce it.
- The walkthrough books `docs/walkthroughs/overview/` and
  `docs/walkthroughs/grove-loop/` reproduce `standalone.rs`, `cli.rs` and
  `session_command.rs` exactly. Regenerate their fragments, and change the
  prose that names confinement.
- `CHANGELOG.md`: change the unreleased entries that name the confined `grove
  run` or the confined directory case. Add an entry for the withdrawal that
  states the weakened guarantees.
- `docs/common-loading-acceptance-record.md`: the runner sections
  "Conventions", "What one run does", the two adapters and "The controls"
  describe the confined mode of the runner. Change them with the runner.
  `stage-evidence-reconciliation-k125` marks the confined results as
  historical.

### Checks

- `bash scripts/check.sh` with the 1.1 keg and with the new 2.0 build, as the
  "HOME of each run" recipe of `docs/grove-dual-ravel-stage-record.md` states.
- One real `grove run release-notes` through the changed helper. It spends one
  Codex call. It records that Codex runs under `workspace-write` and that the
  ending command reaches Ravel from the staged directory.
- One run of `task release:smoke` with `TARGETS=aarch64-unknown-linux-gnu`.
  It runs the directory case on the general route, in the floor container and
  under QEMU. The record names the 2.0 archive that the run used.

## Linux floor evidence

The proof of the glibc 2.17 link floor and the CPU floor stays. It lives in
the general release route: `stage_linux_inputs`, `smoke_in_floor_container`
and `smoke_under_qemu` of `scripts/release-smoke.sh`,
`release-smoke-target.sh`, `release-smoke-environment.sh` and
`release-smoke-qemu.sh`. The QEMU helper keeps its own private namespaces,
its `pivot_root` and its seccomp profile, which do not depend on bubblewrap.

The proof of skill delivery at the floor stays too. The directory case is the
only check that publishes skill snapshots from the release archive at the
glibc 2.17 floor. Ravel's release smoke publishes no snapshot.
`grove-floor-directory-case.sh` therefore becomes a delivery case on the 2.0
lane of the general route, on each Linux target. Both general routes run their
cases as uid 1000, so a snapshot still refuses a write by its file modes.

The case keeps these checks:

- The run completes. Grove reports the completion and publishes the evidence
  files.
- The first read is the mandatory spine in a snapshot. Every read is a snapshot
  path, and the spine, the resource and the kind have their expected contents.
- `request.skills` names the mandatory file. The caller block keeps the
  standalone instructions, and no native copy exists under HOME.
- The probes `descriptor-read` and `cwd-write` succeed. The probes
  `snapshot-write` and `snapshot-append` fail.

The case loses the probes `source-read`, `source-write`, `outside-write`,
`config-read` and `settings-read`. It also loses its two `--runtime-read`
grants and its unconfined control. Its comments and messages stop naming
confinement. The general route stages the script with the cases, in the floor
container and under QEMU. The implementation chooses the call site, such as
`release-smoke-target.sh` or the 2.0 lane of `grove-installed-smoke.sh`.

The native part of the Standalone row has no floor case. The earlier native
case needed the sandbox to make the native copies unreadable, so that the
agent force-stopped. Without a sandbox the copies are readable.
`grove-installed-smoke.sh` still runs the standalone caller and its output
publication on each lane. The scripted agent of
`crates/grove/tests/standalone.rs` checks the interpretation of a force-stop,
as [Tests](#tests) states.

The proof of bubblewrap and confinement goes:

- The scripts `release-floor-backend.sh`, `release-floor-qemu-backend.sh`,
  `release-floor-qemu.sh`, `release-floor-native.sh`,
  `grove-floor-native-cases.sh`, `release-floor-backend-probe.sh` and
  `release-floor-backend-probe-host.sh`, with their tests.
  `atomic_directory_publish.py` and its test go too, because only the backend
  preparers use them.
- `scripts/floor-native-security.c`, and the two bubblewrap rules of
  `scripts/floor_seccomp.py`, with the matching assertions of
  `floor-seccomp.test.sh`.
- The block of `release-smoke-qemu.sh` that stages the backend inputs. These
  are `bwrap`, its libraries, the security probe and the CPU probe of the
  sandbox.
- The Taskfile tasks `release:floor:qemu`, `release:floor:qemu:prepare`,
  `release:floor:qemu:controls`, `release:floor:native`,
  `release:floor:native:run`, `release:floor:prepare`,
  `release:floor:controls` and `release:floor:probe`. The task
  `release:floor:seccomp:controls` stays for the QEMU helper.
- `docs/linux-floor-backend.md` and `docs/linux-floor-native.md`. The general
  QEMU route of `docs/linux-floor-qemu.md` stays under a new title, and its
  backend cases go. A dated handoff section that another document cites stays
  as a historical record.

`smoke_linux` of `release-smoke.sh` then runs `grove-installed-smoke.sh` and
the directory case through the general route, as the Ravel consumers do. Check
once whether the QEMU route still needs `systempaths=unconfined`.

These consumers of the removed route change in the same commit. The test
`every_repository_markdown_reference_resolves` reads every Markdown file, so a
link to a removed document fails `cargo nextest`.

- `scripts/check.sh`: remove the scripts above and their tests from the
  ShellCheck list, and keep `floor-seccomp.test.sh`. Remove the runs of
  `release:floor:controls` and `release:floor:qemu:controls`. Keep the run of
  `release:floor:seccomp:controls`, because `smoke_under_qemu` uses the
  `--qemu` profile.
- `docs/RELEASING.md`, "Installed smoke test": the general route replaces the
  native floor environment and the QEMU confinement environment. The denial
  task and the native guidance case go, and the directory case stays on the 2.0
  lane. The item on each Linux target loses the sandbox descendant of its CPU
  control.
- `docs/ravel-target-inputs.md`: the item on the 2.0 lane names the directory
  case of the general route. "Reused execution seams" loses its paragraph on
  the bubblewrap backend.
- `docs/linux-floor-qemu.md`: its links to `linux-floor-native.md` and
  `linux-floor-backend.md` go with its backend cases.
- `scripts/floor-seccomp/README.md`: it names `docs/linux-floor-native.md` and
  the native profile. Name the QEMU route and its profile instead.

`docs/ravel-caller-surface-inventory.tsv` and
`docs/grove-ravel-docs-evidence.md` are dated audits. They stay, as the dated
floor records do.

The dated floor records stay as records of earlier runs. They include the
handoffs of 2026-10-13 and 2026-10-14 and the floor rows of
`docs/grove-dual-ravel-stage-record.md`. No build after the removal can
repeat a bubblewrap result.

## Review integration

`confinement-withdrawal-k143` reviewed this design and recorded three
findings. `confinement-withdrawal-k144` verified each one against the source
and accepted each one as a real issue:

- **F1, the check script.** The handoff omitted `scripts/check.sh`. An
  enumeration of the removed floor names found four more consumers. The list
  in [Linux floor evidence](#linux-floor-evidence) now names each one.
- **F2, directory delivery at the floor.** The handoff removed the only floor
  check of snapshot publication. The directory case now stays on the general
  route, and the delivery specification states that route.
- **F3, write protection of the configuration.** Ravel's table gave a reason
  that does not always hold. Ravel's evidence record now states the
  protection as conditional on the harness sandbox. Ravel commit
  `2df9dd0558248404da48842499b9dcd03a605fec` holds that change.

## Limits

This record states a design. No product code changed, and no test, build or
harness ran in its leaf. The behaviour of Codex in `--sandbox workspace-write`
with a standalone channel is a measurement for the removal leaf. The sandbox
settings of Claude Code come from the names in the 2.1.317 executable, such as
`sandbox.enabled` and `sandbox.failIfUnavailable`. No run measured them.
